extern crate alto;
extern crate hush;
extern crate luminance_glfw;

// test only
extern crate hound;

mod streaming;

use alto::Source;
use hush::instrument::{Instrument, Synth};
use hush::note;
use hush::time::SampleTime;
use luminance_glfw::surface::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

fn main() {
  let mut surface = GlfwSurface::new(WindowDim::Windowed(940, 560), "hush piano", WindowOpt::default()).expect("GLFW surface");

  let mut synth = Synth::sine();

  // streaming stuff; we’ll just load data into two buffers to start with
  let alto = alto::Alto::load_default().unwrap();
  let al_device = alto.open(None).unwrap();
  let al_ctx = al_device.new_context(None).unwrap();

  let mut source = al_ctx.new_streaming_source().unwrap(); // the thing that can play buffers

  // create blank buffers; this array represents idle buffers (i.e. blank buffers that have not been
  // queued on a source yet)
  let mut buffers = (0..2).into_iter().map(|_| {
    al_ctx.new_buffer::<alto::Mono<f32>, _>(&vec![0.; 44100], 44100).unwrap()
  }).collect::<Vec<_>>();

  //// test only: output a A4 sawtooth into a wav file
  //{
  //  let spec = hound::WavSpec {
  //    channels: 1,
  //    sample_rate: 44100,
  //    bits_per_sample: 16,
  //    sample_format: hound::SampleFormat::Int,
  //  };

  //  let mut writer = hound::WavWriter::create("/tmp/sawtooth.wav", spec).unwrap();

  //  synth = Synth::sine();
  //  synth.note_on(note::A4, SampleTime(0));
  //  {
  //    let samples = synth.get_samples(SampleTime(0), SampleTime(44100));

  //    for sample in samples {
  //      let amplitude = std::i16::MAX as f32;
  //      writer.write_sample((sample * amplitude) as i16).unwrap();
  //    }
  //  }

  //  synth = Synth::square();
  //  synth.note_on(note::A4, SampleTime(0));
  //  {
  //    let samples = synth.get_samples(SampleTime(0), SampleTime(44100));

  //    for sample in samples {
  //      let amplitude = std::i16::MAX as f32;
  //      writer.write_sample((sample * amplitude) as i16).unwrap();
  //    }
  //  }

  //  synth = Synth::triangle();
  //  synth.note_on(note::A4, SampleTime(0));
  //  {
  //    let samples = synth.get_samples(SampleTime(0), SampleTime(44100));

  //    for sample in samples {
  //      let amplitude = std::i16::MAX as f32;
  //      writer.write_sample((sample * amplitude) as i16).unwrap();
  //    }
  //  }

  //  synth = Synth::sawtooth();
  //  synth.note_on(note::A4, SampleTime(0));
  //  {
  //    let samples = synth.get_samples(SampleTime(0), SampleTime(44100));

  //    for sample in samples {
  //      let amplitude = std::i16::MAX as f32;
  //      writer.write_sample((sample * amplitude) as i16).unwrap();
  //    }
  //  }
  //}

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
              synth.note_off();
            }

            _ => ()
          }
        }

        _ => ()
      }
    }

    if source.state() != alto::SourceState::Playing {
      // get one second of samples into a buffer
      let samples = synth.get_samples(SampleTime(0), SampleTime(44100));

      if samples.is_empty() {
        // nothing to play, let’s just unqueue the buffers from the DSP
        loop {
          if buffers.len() == 2 {
            break;
          }

          buffers.push(source.unqueue_buffer().expect("unqueue buffer"));
        }
      } else {
        let mut buffer = buffers.swap_remove(0);

        // load the fresh samples into the buffer
        buffer.set_data::<alto::Mono<f32>, _>(samples, 44100);
        // enqueue the buffer
        source.queue_buffer(buffer);
        source.play();
      }
    } else {
      // we are currently playing something
      if synth.is_active() {
        // active and playing
      } else {
        // not active but still playing
      }
    }
  }
}
