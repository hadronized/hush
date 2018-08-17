#![no_std]
#![feature(alloc)]
#![feature(core_intrinsics)]

extern crate alloc;

pub mod hertz;
pub mod note;

use alloc::vec::Vec;
use core::intrinsics::{fabsf64, floorf64, powf64, sinf64};
use core::f64::consts::PI;

use hertz::Hertz;
use note::Note;

const TWICE_PI: f64 = 2. * PI;

pub type Time = f64;
pub type Sample = f64;

/// The core sine wave (normalized).
#[inline(always)]
pub fn sine_wave(t: Time) -> Sample {
  unsafe { sinf64(t * TWICE_PI) }
}

/// The square wave (normalized).
#[inline(always)]
pub fn square_wave(t: Time) -> Sample {
  unsafe { powf64(-1., floorf64(2. * t)) }
}

/// The triangle wave (normalized).
#[inline(always)]
pub fn triangle_wave(t: Time) -> Sample {
  unsafe { fabsf64(4. * (t - 0.5) % 1. - 2.) - 1. }
}

/// The sawtooth wave (normalized).
#[inline(always)]
pub fn sawtooth_wave(t: Time) -> Sample {
  2. * ((t - 0.5) % 1.) - 1.
}

/// Oscillator.
pub struct Oscillator<F> where F: Fn(Time) -> Sample {
  sampling_buffer: Vec<Sample>,
  wave: F,
}

// Step between two sampling points when sampling at 44.1 kHz.
const SAMPLING_STEP: f64 = 1. / 44100.;

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
      let t = SAMPLING_STEP * i as f64;
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

/// A sine instrument.
///
/// This instrument will play notes over a sine oscillator.
pub struct SineSynth {
  pressed: Option<PressedNote>,
  oscillator: Oscillator<fn(Time) -> Sample>
}

impl SineSynth {
  pub fn new() -> Self {
    let oscillator = Oscillator::new(sine_wave as fn(_) -> _);

    SineSynth {
      pressed: None,
      oscillator
    }
  }
}

impl Instrument for SineSynth {
  fn note_on(&mut self, note: Note, time: SampleTime) {
    self.pressed = Some(PressedNote { note, time });
  }

  fn note_off(&mut self) {
    self.pressed = None;
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
