fn main() {
  for octave in (-2)..9 {
    let halves = (octave - 4) * 12;
    println!("const A{}: Note = Note({:.5})", octave, pitch(halves));
    println!("const B{}: Note = Note({:.5})", octave, pitch(halves + 2));
    println!("const C{}: Note = Note({:.5})", octave + 1, pitch(halves + 3));
    println!("const D{}: Note = Note({:.5})", octave + 1, pitch(halves + 5));
    println!("const E{}: Note = Note({:.5})", octave + 1, pitch(halves + 7));
    println!("const F{}: Note = Note({:.5})", octave + 1, pitch(halves + 8));
    println!("const G{}: Note = Note({:.5})", octave + 1, pitch(halves + 10));
  }
}

// Generate the pitch of note distant to C4 by n half-tones.
fn pitch(n: isize) -> f64 {
  2.0f64.powf(n as f64 / 12.) * 440.
}
