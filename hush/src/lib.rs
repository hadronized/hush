#![no_std]
#![feature(alloc)]
#![feature(core_intrinsics)]

extern crate alloc;

pub mod hertz;
pub mod notes;

use alloc::vec::Vec;
use core::intrinsics::{fabsf64, floorf64, powf64, sinf64};
use core::f64::consts::PI;

const TWICE_PI: f64 = 2. * PI;

pub type Time = f64;
pub type PCMOutput = f64;

/// The core sine wave (normalized).
#[inline(always)]
pub fn sine_wave(t: Time) -> PCMOutput {
  unsafe { sinf64(t * TWICE_PI) }
}

/// The square wave (normalized).
#[inline(always)]
pub fn square_wave(t: Time) -> PCMOutput {
  unsafe { powf64(-1., floorf64(2. * t)) }
}

/// The triangle wave (normalized).
#[inline(always)]
pub fn triangle_wave(t: Time) -> PCMOutput {
  unsafe { fabsf64(4. * (t - 0.5) % 1. - 2.) - 1. }
}

/// The sawtooth wave (normalized).
#[inline(always)]
pub fn sawtooth_wave(t: Time) -> PCMOutput {
  2. * ((t - 0.5) % 1.) - 1.
}

/// Oscillator.
pub struct Oscillator<F> where F: Fn(Time) -> PCMOutput {
  sampling_buffer: Vec<PCMOutput>,
  wave: F,
}

// Step between two sampling points when sampling at 44.1 kHz.
const SAMPLING_STEP: f64 = 1. / 44100.;

impl<F> Oscillator<F> where F: Fn(Time) -> PCMOutput {
  pub fn new(f: F) -> Self {
    Oscillator {
      sampling_buffer: Vec::with_capacity(44100),
      wave: f
    }
  }

  /// Sample from sample `start` to `end`.
  pub fn sample(&mut self, start: usize, end: usize) -> &[PCMOutput] {
    assert!(end >= start);

    // generate the samples
    for i in start..end {
      let t = SAMPLING_STEP * i as f64;
      let signal = (self.wave)(t);

      self.sampling_buffer.push(signal);
    }

    // return the samples we just generated
    &self.sampling_buffer[0..end - start]
  }
}
