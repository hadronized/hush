#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::{fabsf64, floorf64, powf64, sinf64};
use core::f64::EPSILON;
use core::f64::consts::PI;

const TWICE_PI: f64 = 2. * PI;

type Time = f64;
type PCMOutput = f64;
type Herts = f64;

/// The core sine wave (normalized).
#[inline(always)]
fn sine_wave(t: Time) -> PCMOutput {
  unsafe { sinf64(t * TWICE_PI) }
}

/// The square wave (normalized).
#[inline(always)]
fn square_wave(t: Time) -> PCMOutput {
  unsafe { powf64(-1., floorf64(2. * t)) }
}

// The triangle wave (normalized).
#[inline(always)]
fn triangle_wave(t: Time) -> PCMOutput {
  unsafe { fabsf64(4. * (t - 0.5) % 1. - 2.) - 1. }
}

mod tests {
  use super::*;

  #[inline(always)]
  fn abs(a: f64) -> f64 {
    if a < 0. { -a } else { a }
  }

  #[inline(always)]
  fn eqf(a: f64, b: f64) -> bool {
    abs(a - b) <= EPSILON
  }

  #[test]
  fn sine_wave_remarkable_values() {
    assert!(eqf(sine_wave(0.), 0.));
    assert!(eqf(sine_wave(0.5), 0.));
    //assert!(eqf(sine_wave(1.), 0.)); // FIXME
    assert!(eqf(sine_wave(0.25), 1.));
    assert!(eqf(sine_wave(0.75), -1.));
  }

  #[test]
  fn square_wave_remarkable_values() {
    assert!(eqf(square_wave(0.), 1.));
    assert!(eqf(square_wave(0.6), -1.));
    assert!(eqf(square_wave(0.9), -1.));
  }
}
