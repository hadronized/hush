/// Regular time.
pub type Time = f32;

/// Sample time.
///
/// A sample time is a discretized time used to sample an oscillator. When a DSP asks for signal
/// samples, it will use that kind of discretized time (or indirectly). What is interesting is that
/// a number of frames is such a time (it’s a difference of sample time), so it’s very easy to
/// convert from that measure to an actual time that can be used to sample from.
pub struct SampleTime(pub usize);
