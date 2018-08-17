extern crate hush;
extern crate luminance_glfw;

use hush::note;
use hush::{Instrument, SampleTime, SineSynth};
use luminance_glfw::surface::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

fn main() {
  let mut surface = GlfwSurface::new(WindowDim::Windowed(940, 560), "hush piano", WindowOpt::default()).expect("GLFW surface");

  // for now we’ll be testing the SineSynth only
  let mut synth = SineSynth::new();

  'app: loop {
    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
          break 'app;
        }

        // key on
        WindowEvent::Key(key, _, Action::Press, _) => {
          match key {
            Key::Q => {
              synth.note_on(note::C4, SampleTime(0));
            }

            Key::W => {
              synth.note_on(note::DB4, SampleTime(0));
            }

            Key::E => {
              synth.note_on(note::D4, SampleTime(0));
            }

            Key::R => {
              synth.note_on(note::EB4, SampleTime(0));
            }

            Key::T => {
              synth.note_on(note::E4, SampleTime(0));
            }

            Key::Y => {
              synth.note_on(note::F4, SampleTime(0));
            }

            Key::U => {
              synth.note_on(note::GB4, SampleTime(0));
            }

            Key::I => {
              synth.note_on(note::G4, SampleTime(0));
            }

            Key::O => {
              synth.note_on(note::AB4, SampleTime(0));
            }

            Key::P => {
              synth.note_on(note::A5, SampleTime(0));
            }

            Key::LeftBracket => {
              synth.note_on(note::BB5, SampleTime(0));
            }

            Key::RightBracket => {
              synth.note_on(note::B5, SampleTime(0));
            }

            _ => ()
          }
        }

        // key off
        WindowEvent::Key(key, _, Action::Release, _) => {
          match key {
            Key::Q | Key::W | Key::E | Key::R | Key::T | Key::Y | Key::U | Key::I | Key::O | Key::P | Key::LeftBracket | Key::RightBracket => {
              synth.note_off();
            }

            _ => ()
          }
        }

        _ => ()
      }
    }

    // ask for a few samples to start with
    {
      let samples = synth.get_samples(SampleTime(0), SampleTime(100));

      if !samples.is_empty() {
        println!("samples\n{:?}", samples);
      }
    }

    // automatically toggle the note off for now
    synth.note_off();
  }
}
