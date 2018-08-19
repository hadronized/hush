use alto::{Buffer, Context, Mono, Source, SourceState, StreamingSource};
use hush::instrument::Instrument;

// Handle audio streaming.
//
// An object of this type can be used to stream an instrument activity to the DSP device in full
// transparency.
pub struct Streamer {
  source: StreamingSource,
  buffers: Vec<Buffer>
}

impl Streamer {
  pub fn new(al_ctx: &mut Context) -> Self {
    let source = al_ctx.new_streaming_source().expect("OpenAL source");

    let buffers = (0..2).into_iter().map(|_| {
      al_ctx.new_buffer::<Mono<f32>, _>(&vec![0.; 44100], 44100).unwrap()
    }).collect::<Vec<_>>();

    Self { source, buffers }
  }

  /// Refresh the streaming process to check whether the DSP and/or streaming buffers should be
  /// updated.
  pub fn refresh<I>(&mut self, instrument: &mut I) where I: Instrument {
    // first thing first: we check the state of the DSP
    if self.source.state() == SourceState::Playing {
      if instrument.is_active() {
        // DSP playing and instrument is still active: we need to check whether some more data is
        // needed
        unimplemented!();
      } else {
        // DSP is playing but the instrument is not active anymore: release phase
        unimplemented!();
      }
    } else if instrument.is_active() {
        // the DSP is not playing but the instrument is active, so we need to queue at
        // least one buffer in
        self.queue_one_buffer(instrument); 
    }
  }

  /// Queue a buffer of data.
  fn queue_one_buffer<I>(&mut self, instrument: &mut I) where I: Instrument {
    unimplemented!()
  }
}
