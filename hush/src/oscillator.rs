//! Basic oscillators.

use alloc::vec::Vec;
use core::intrinsics::{fabsf32, floorf32, powf32, sinf32};
use core::f32::consts::PI;

use hertz::Hertz;
use sample::Sample;
use time::SampleTime;

const TWICE_PI: f32 = 2. * PI;

/// The core sine wave (normalized).
#[inline(always)]
pub fn sine_wave(t: Hertz) -> Sample {
  unsafe { sinf32(t * TWICE_PI) }
}

/// The square wave (normalized).
#[inline(always)]
pub fn square_wave(t: Hertz) -> Sample {
  unsafe { powf32(-1., floorf32(2. * t)) }
}

/// The triangle wave (normalized).
#[inline(always)]
pub fn triangle_wave(t: Hertz) -> Sample {
  unsafe { fabsf32((t + 1.5) % 2. - 1.) * 2. - 1. }
}

/// The sawtooth wave (normalized).
#[inline(always)]
pub fn sawtooth_wave(t: Hertz) -> Sample {
  2. * (-(t % 1.)) + 1.
}

/// Oscillator.
pub struct Oscillator<F> where F: Fn(Hertz) -> Sample {
  sampling_buffer: Vec<Sample>,
  wave: F,
}

// Step between two sampling points when sampling at 44.1 kHz.
const SAMPLING_STEP: f32 = 1. / 44100.;

impl<F> Oscillator<F> where F: Fn(Hertz) -> Sample {
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
