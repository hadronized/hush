//! Instruments.

use hertz::Hertz;
use note::Note;
use oscillator::{Oscillator, sine_wave, square_wave, triangle_wave, sawtooth_wave};
use time::SampleTime;
use sample::Sample;

/// An instrument.
///
/// An instrument can play notes by pressing and releasing them. Notes can be played independently
/// from each other, allowing for a rich and mixed audio signal. This is done through “note channels”.
pub trait Instrument {
  /// Trigger a note at a given time on a given note channel.
  fn note_on(&mut self, note: Note, time: SampleTime) -> NoteChannel;

  /// Release a note.
  fn note_off(&mut self, note_channel: NoteChannel);

  /// Is the instrument currently active / playing?
  fn is_active(&self) -> bool;

  /// Get a few samples from this instrument.
  fn get_samples(&mut self, start: SampleTime, end: SampleTime) -> &[Sample];
}

/// A note channel.
///
/// When an instrument is asked to play a note, it does it on a “note channel”, allowing for multiple
/// notes to be played at the same time. The number of available channels is instrument-specific.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoteChannel(usize);

impl NoteChannel {
  pub fn new(i: usize) -> Self {
    NoteChannel(i)
  }

  pub fn default() -> Self {
    Self::new(0)
  }
}

/// A note pressed at a given time.
pub struct PressedNote {
  note: Note,
  channel: NoteChannel,
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
  fn note_on(&mut self, note: Note, time: SampleTime) -> NoteChannel {
    let channel = NoteChannel::default();

    self.pressed = Some(PressedNote { note, channel, time });

    channel
  }

  fn note_off(&mut self, _: NoteChannel) {
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
