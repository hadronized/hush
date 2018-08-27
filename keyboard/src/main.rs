extern crate alto;
extern crate hush;
extern crate luminance_glfw;

// test only
extern crate hound;

mod streaming;

use alto::Source;
use hush::envelope::{ADSR, ADSRState};
use hush::instrument::{Instrument, NoteChannel, Synth};
use hush::note::{self, Note};
use hush::sample::Sample;
use hush::time::{SampleTime, Time};
use luminance_glfw::surface::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};
use std::time::Instant;

// Our instrument is a Synth with an ADSR envelope.
struct SynthADSR {
  synth: Synth,
  envelope: ADSR,
  source_time: Instant
}

impl SynthADSR {
  fn new(synth: Synth) -> Self {
    let envelope = ADSR::new(0.2, 0.1, 0.9, 1.).expect("ADSR envelope");
    let source_time = Instant::now();

    SynthADSR { synth, envelope, source_time }
  }

  pub fn sine() -> Self {
    Self::new(Synth::sine())
  }

  pub fn triangle() -> Self {
    Self::new(Synth::triangle())
  }

  pub fn square() -> Self {
    Self::new(Synth::square())
  }

  pub fn sawtooth() -> Self {
    Self::new(Synth::sawtooth())
  }
}

impl Instrument for SynthADSR {
  fn note_on(&mut self, note: Note, channel: NoteChannel) {
    self.synth.note_on(note, channel);
    self.envelope.on(time_from_instant(&self.source_time));
  }

  fn note_off(&mut self, _: NoteChannel) {
    self.envelope.off(time_from_instant(&self.source_time));
  }

  fn is_active(&self, t: Time) -> bool {
    match self.envelope.state() {
      ADSRState::On(_) => true,
      ADSRState::Off(_) => {
        // when the signal hits 0, it’s officially unactive
        self.envelope.get(t) > 0.
      }
    }
  }

  fn get_samples(&mut self, start: SampleTime, end: SampleTime) -> &[Sample] {
    unimplemented!()
  }
}

fn time_from_instant(instant: &Instant) -> f32 {
  let duration = instant.elapsed();
  let secs = duration.as_secs() as f32;
  let millis = duration.subsec_millis() as f32;

  secs + millis * 1e-3
}

fn main() {
  let mut surface = GlfwSurface::new(WindowDim::Windowed(940, 560), "hush piano", WindowOpt::default()).expect("GLFW surface");

  let mut synth = Synth::sine();

  // backend stuff: OpenAL here
  let alto = alto::Alto::load_default().unwrap();
  let al_device = alto.open(None).unwrap();
  let mut al_ctx = al_device.new_context(None).unwrap();

  // for streaming
  let mut streamer = streaming::Streamer::new(&mut al_ctx);

  // for timing
  let now = Instant::now();

  'app: loop {
    let t = time_from_instant(&now);

    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
          break 'app;
        }

        // key on
        WindowEvent::Key(key, _, Action::Press, _) => {
          match key {
            Key::F1 => {
              synth = Synth::sine();
            }

            Key::F2 => {
              synth = Synth::square();
            }

            Key::F3 => {
              synth = Synth::triangle();
            }

            Key::F4 => {
              synth = Synth::sawtooth();
            }

            Key::Q => {
              println!("on C4");
              synth.note_on(note::C4, NoteChannel::default());
            }

            Key::W => {
              println!("on DB4");
              synth.note_on(note::DB4, NoteChannel::default());
            }

            Key::E => {
              println!("on D4");
              synth.note_on(note::D4, NoteChannel::default());
            }

            Key::R => {
              println!("on EB4");
              synth.note_on(note::EB4, NoteChannel::default());
            }

            Key::T => {
              println!("on E4");
              synth.note_on(note::E4, NoteChannel::default());
            }

            Key::Y => {
              println!("on F4");
              synth.note_on(note::F4, NoteChannel::default());
            }

            Key::U => {
              println!("on GB4");
              synth.note_on(note::GB4, NoteChannel::default());
            }

            Key::I => {
              println!("on G4");
              synth.note_on(note::G4, NoteChannel::default());
            }

            Key::O => {
              println!("on AB4");
              synth.note_on(note::AB4, NoteChannel::default());
            }

            Key::P => {
              println!("on A4");
              synth.note_on(note::A4, NoteChannel::default());
            }

            Key::LeftBracket => {
              println!("on BB4");
              synth.note_on(note::BB4, NoteChannel::default());
            }

            Key::RightBracket => {
              println!("on B4");
              synth.note_on(note::B4, NoteChannel::default());
            }

            _ => ()
          }
        }

        // key off
        WindowEvent::Key(key, _, Action::Release, _) => {
          match key {
            Key::Q | Key::W | Key::E | Key::R | Key::T | Key::Y | Key::U | Key::I | Key::O | Key::P | Key::LeftBracket | Key::RightBracket => {
              println!("off");
              synth.note_off(NoteChannel::default());
            }

            _ => ()
          }
        }

        _ => ()
      }
    }

    // handle streaming
    streamer.refresh(&mut synth, t);
  }
}
