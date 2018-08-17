fn main() {
  for octave in (-2)..9 {
    let halves = (octave - 4) * 12;
    println!("const A{}: Note = Note({:.5});", octave, pitch(halves));
    println!("const BB{}: Note = Note({:.5});", octave, pitch(halves + 1));
    println!("const B{}: Note = Note({:.5});", octave, pitch(halves + 2));
    println!("const C{}: Note = Note({:.5});", octave + 1, pitch(halves + 3));
    println!("const DB{}: Note = Note({:.5});", octave + 1, pitch(halves + 4));
    println!("const D{}: Note = Note({:.5});", octave + 1, pitch(halves + 5));
    println!("const EB{}: Note = Note({:.5});", octave + 1, pitch(halves + 6));
    println!("const E{}: Note = Note({:.5});", octave + 1, pitch(halves + 7));
    println!("const F{}: Note = Note({:.5});", octave + 1, pitch(halves + 8));
    println!("const GB{}: Note = Note({:.5});", octave + 1, pitch(halves + 9));
    println!("const G{}: Note = Note({:.5});", octave + 1, pitch(halves + 10));
    println!("const AB{}: Note = Note({:.5});", octave + 1, pitch(halves + 11));
  }
}

// Generate the pitch of note distant to C4 by n half-tones.
fn pitch(n: isize) -> f64 {
  2.0f64.powf(n as f64 / 12.) * 440.
}
