//! Instruments.

use hertz::Hertz;
use note::Note;
use oscillator::{Oscillator, sine_wave, square_wave, triangle_wave, sawtooth_wave};
use time::SampleTime;
use sample::Sample;

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

/// A note pressed at a given time.
pub struct PressedNote {
  note: Note,
  time: SampleTime
}

/// A synth.
pub struct Synth {
  pressed: Option<PressedNote>,
  oscillator: Oscillator<fn(Hertz) -> Sample>
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
