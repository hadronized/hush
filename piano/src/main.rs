extern crate hush;
extern crate luminance_glfw;

use hush::note;
use luminance_glfw::surface::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

fn main() {
  let mut surface = GlfwSurface::new(WindowDim::Windowed(940, 560), "hush piano", WindowOpt::default()).expect("GLFW surface");

  'app: loop {
    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
          break 'app;
        }

        // key on
        WindowEvent::Key(key, _, Action::Press, _) => {
          println!("key: {:?}", key);

          match key {
            Key::Q => {
              println!("{:?} on", note::C4);
            }

            Key::W => {
              println!("{:?} on", note::DB4);
            }

            Key::E => {
              println!("{:?} on", note::D4);
            }

            Key::R => {
              println!("{:?} on", note::EB4);
            }

            Key::T => {
              println!("{:?} on", note::E4);
            }

            Key::Y => {
              println!("{:?} on", note::F4);
            }

            Key::U => {
              println!("{:?} on", note::GB4);
            }

            Key::I => {
              println!("{:?} on", note::G4);
            }

            Key::O => {
              println!("{:?} on", note::AB4);
            }

            Key::P => {
              println!("{:?} on", note::A5);
            }

            Key::LeftBracket => {
              println!("{:?} on", note::BB5);
            }

            Key::RightBracket => {
              println!("{:?} on", note::B5);
            }

            _ => ()
          }
        }

        // key off
        WindowEvent::Key(key, _, Action::Release, _) => {
        }

        _ => ()
      }
    }
  }
}
