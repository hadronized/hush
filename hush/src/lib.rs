#![no_std]
#![feature(alloc)]
#![feature(core_intrinsics)]

extern crate alloc;

pub mod hertz;
pub mod note;

use alloc::vec::Vec;
use core::intrinsics::{fabsf32, floorf32, powf32, sinf32};
use core::f32::consts::PI;

use hertz::Hertz;
use note::Note;

const TWICE_PI: f32 = 2. * PI;

pub type Time = f32;
pub type Sample = f32;

/// The core sine wave (normalized).
#[inline(always)]
pub fn sine_wave(t: Time) -> Sample {
  unsafe { sinf32(t * TWICE_PI) }
}

/// The square wave (normalized).
#[inline(always)]
pub fn square_wave(t: Time) -> Sample {
  unsafe { powf32(-1., floorf32(2. * t)) }
}

/// The triangle wave (normalized).
#[inline(always)]
pub fn triangle_wave(t: Time) -> Sample {
  unsafe { fabsf32(4. * (t - 0.5) % 1. - 2.) - 1. }
}

/// The sawtooth wave (normalized).
#[inline(always)]
pub fn sawtooth_wave(t: Time) -> Sample {
  2. * (-t % 1.) - 1.
}

/// Oscillator.
pub struct Oscillator<F> where F: Fn(Time) -> Sample {
  sampling_buffer: Vec<Sample>,
  wave: F,
}

// Step between two sampling points when sampling at 44.1 kHz.
const SAMPLING_STEP: f32 = 1. / 44100.;

impl<F> Oscillator<F> where F: Fn(Time) -> Sample {
  pub fn new(f: F) -> Self {
    Oscillator {
      sampling_buffer: Vec::with_capacity(44100),
      wave: f
    }
  }

  /// Sample from sample `start` to `end` with the given frequency.
  pub fn sample(&mut self, start: SampleTime, end: SampleTime, freq: Hertz) -> &[Sample] {
    let s = start.0;
    let e = end.0;

    assert!(e >= s);

    // clear the buffer
    self.sampling_buffer.clear();

    // generate the samples
    for i in s..e {
      let t = SAMPLING_STEP * i as f32;
      let signal = (self.wave)(t * freq);

      self.sampling_buffer.push(signal);
    }

    // return the samples we just generated
    &self.sampling_buffer[0 .. e - s]
  }
}

/// A channel instrument.
///
/// A channel instrument can play notes.
pub trait Instrument {
  /// Trigger a note at a given time.
  fn note_on(&mut self, note: Note, time: SampleTime);

  // Release a note.
  fn note_off(&mut self);

  // Is the instrument currently active / playing?
  fn is_active(&self) -> bool;

  // Get a few samples from this instrument.
  fn get_samples(&mut self, start: SampleTime, end: SampleTime) -> &[Sample];
}

/// Sample time.
///
/// A sample time is a discretized time used to sample an oscillator. When a DSP asks for signal
/// samples, it will use that kind of discretized time (or indirectly). What is interesting is that
/// a number of frames is such a time (it’s a difference of sample time), so it’s very easy to
/// convert from that measure to an actual time that can be used to sample from.
pub struct SampleTime(pub usize);

/// A note pressed at a given time.
pub struct PressedNote {
  note: Note,
  time: SampleTime
}

/// A synth.
pub struct Synth {
  pressed: Option<PressedNote>,
  oscillator: Oscillator<fn(Time) -> Sample>
}

impl Synth {
  pub fn sine() -> Self {
    let oscillator = Oscillator::new(sine_wave as fn(_) -> _);

    Synth {
      pressed: None,
      oscillator
    }
  }

  pub fn square() -> Self {
    let oscillator = Oscillator::new(square_wave as fn(_) -> _);

    Synth {
      pressed: None,
      oscillator
    }
  }

  pub fn triangle() -> Self {
    let oscillator = Oscillator::new(triangle_wave as fn(_) -> _);

    Synth {
      pressed: None,
      oscillator
    }
  }

  pub fn sawtooth() -> Self {
    let oscillator = Oscillator::new(sawtooth_wave as fn(_) -> _);

    Synth {
      pressed: None,
      oscillator
    }
  }
}

impl Instrument for Synth {
  fn note_on(&mut self, note: Note, time: SampleTime) {
    self.pressed = Some(PressedNote { note, time });
  }

  fn note_off(&mut self) {
    self.pressed = None;
  }

  fn is_active(&self) -> bool {
    self.pressed.is_some()
  }

  fn get_samples(&mut self, start: SampleTime, end: SampleTime) -> &[Sample] {
    match self.pressed {
      None => &[],

      Some(ref pressed) => {
        self.oscillator.sample(start, end, pressed.note.frequency())
      }
    }
  }
}

/// State of an ADSR.
pub enum ADSRState {
  /// The ADSR was enabled.
  ///
  /// Also contains the time at which it was switched on.
  On(Time),
  /// The ADSR was disabled.
  ///
  /// Also contains th etime at which it was switched off.
  Off(Time)
}

/// A normalized ADSR (Attack–Decay–Sustain–Release) envelope.
///
/// ADSR envelopes can be used to implement various of effect: amplitude modulation, pitch
/// modulation, etc.
///
/// The minimal value an ADSR gives you is 0. The maximal value an ADSR envelope gives you is 1.
pub struct ADSR {
  attack: Time,
  decay: Time,
  sustain: f32,
  release: Time,
  state: ADSRState
}

impl ADSR {
  fn new(attack: Time, decay: Time, sustain: f32, release: Time) -> Option<Self> {
    if attack <= 0. || decay <= 0. || sustain < 0. || release < 0. {
      return None;
    }

    Some(Self {
      attack,
      decay,
      sustain,
      release,
      state: ADSRState::Off(-1. / 0.) // -Inf should be enough for anyone trying to release :D
    })
  }

  // Switch on.
  fn on(&mut self, t: Time) {
    self.state = ADSRState::On(t);
  }

  // Switch off.
  fn off(&mut self, t: Time) {
    self.state = ADSRState::Off(t);
  }

  // Get the current value based on the current time.
  fn get(&self, t: Time) -> f32 {
    match self.state {
      ADSRState::On(t_0) => {
        let attack_ = t_0 + self.attack;

        if t <= attack_ { // attacking
          return (t - t_0) / self.attack;
        }

        let decay_ = attack_ + self.decay;
        
        if t <= t_0 + decay_{ // decaying
          let nt = (t - attack_) / self.decay;
          1. + nt * (self.sustain - 1.)
        } else { // sustaining
          self.sustain
        }
      }

      ADSRState::Off(t_0) => {
        // release only possible here
        let nt = (t - t_0) / self.release;
        let q = (1. - nt) * self.sustain;
        q.max(0.) // ensure we don’t get weird negative values if we forget to switch the ADSR off
      }
    }
  }
}
