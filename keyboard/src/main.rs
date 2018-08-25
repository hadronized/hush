extern crate alto;
extern crate hush;
extern crate luminance_glfw;

// test only
extern crate hound;

mod streaming;

use alto::Source;
use hush::instrument::{Instrument, NoteChannel, Synth};
use hush::note;
use hush::time::SampleTime;
use luminance_glfw::surface::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

fn main() {
  let mut surface = GlfwSurface::new(WindowDim::Windowed(940, 560), "hush piano", WindowOpt::default()).expect("GLFW surface");

  let mut synth = Synth::sine();

  // backend stuff: OpenAL here
  let alto = alto::Alto::load_default().unwrap();
  let al_device = alto.open(None).unwrap();
  let mut al_ctx = al_device.new_context(None).unwrap();

  // for streaming
  let mut streamer = streaming::Streamer::new(&mut al_ctx);

  'app: loop {
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
              synth.note_on(note::C4, SampleTime(0));
            }

            Key::W => {
              println!("on DB4");
              synth.note_on(note::DB4, SampleTime(0));
            }

            Key::E => {
              println!("on D4");
              synth.note_on(note::D4, SampleTime(0));
            }

            Key::R => {
              println!("on EB4");
              synth.note_on(note::EB4, SampleTime(0));
            }

            Key::T => {
              println!("on E4");
              synth.note_on(note::E4, SampleTime(0));
            }

            Key::Y => {
              println!("on F4");
              synth.note_on(note::F4, SampleTime(0));
            }

            Key::U => {
              println!("on GB4");
              synth.note_on(note::GB4, SampleTime(0));
            }

            Key::I => {
              println!("on G4");
              synth.note_on(note::G4, SampleTime(0));
            }

            Key::O => {
              println!("on AB4");
              synth.note_on(note::AB4, SampleTime(0));
            }

            Key::P => {
              println!("on A4");
              synth.note_on(note::A4, SampleTime(0));
            }

            Key::LeftBracket => {
              println!("on BB4");
              synth.note_on(note::BB4, SampleTime(0));
            }

            Key::RightBracket => {
              println!("on B4");
              synth.note_on(note::B4, SampleTime(0));
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
    streamer.refresh(&mut synth);
  }
}
