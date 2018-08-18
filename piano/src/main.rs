extern crate alto;
extern crate hush;
extern crate luminance_glfw;

use hush::note;
use hush::{Instrument, SampleTime, SineSynth};
use luminance_glfw::surface::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

fn main() {
  let mut surface = GlfwSurface::new(WindowDim::Windowed(940, 560), "hush piano", WindowOpt::default()).expect("GLFW surface");

  // for now we’ll be testing the SineSynth only
  let mut synth = SineSynth::new();

  // streaming stuff; we’ll just load data into two buffers to start with
  let alto = alto::Alto::load_default().unwrap();
  let al_device = alto.open(None).unwrap();
  let al_ctx = al_device.new_context(None).unwrap();
  let mut source = al_ctx.new_streaming_source().unwrap(); // the thing that can play buffers
  let al_buffers = (0..2).into_iter().map(|_| {
    al_ctx.new_buffer::<alto::Stereo<f32>, _>(&vec![0; 44100], 44100).unwrap()
  }).collect::<Vec<_>>();
  let idle_buffer = None;

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

    // check whether a buffer has completely been processed (i.e. we need to load more data) and
    // perform buffers swapping if needed
    if source.buffers_processed() > 0 {
      // unqueue the currently playing buffer
      let mut unqueued = source.unqueue_buffer().expect("unqueued buffer");

      // FIXME: we typically want to do this in a thread
      // ask for the next chunks of samples
      let samples = synth.get_samples(SampleTime(0), SampleTime(44100));

      if !samples.is_empty() {
        unqueued.set_data(samples, 44100);
        source.queue_buffer(unqueued);
      } else {
        // no data to queue the buffer with, let’s just put it away
        idle_buffer = Some(unqueued);
      }
    }

    // automatically toggle the note off for now
    synth.note_off();
  }
}
