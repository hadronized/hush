#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sinf64;
use core::f64::EPSILON;
use core::f64::consts::PI;

const TWICE_PI: f64 = 2. * PI;

type Time = f64;
type PCMOutput = f64;

/// The core sine wave (normalized).
fn sine_wave(t: Time) -> PCMOutput {
  unsafe { sinf64(t * TWICE_PI) }
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
}
