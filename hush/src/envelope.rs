//! ADSR envelopes and related types.

use time::Time;

/// State of an ADSR.  pub enum ADSRState {
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSRState {
  /// The ADSR was enabled.
  ///
  /// Also contains the time at which it was switched on.
  On(Time),
  /// The ADSR was disabled.
  ///
  /// Also contains th etime at which it was switched off.
  Off(Time)
}

/// A normalized ADSR (Attack–Decay–Sustain–Release) envelope.
///
/// ADSR envelopes can be used to implement various of effect: amplitude modulation, pitch
/// modulation, etc.
///
/// The minimal value an ADSR gives you is 0. The maximal value an ADSR envelope gives you is 1.
pub struct ADSR {
  attack: Time,
  decay: Time,
  sustain: f32,
  release: Time,
  state: ADSRState
}

impl ADSR {
  pub fn new(attack: Time, decay: Time, sustain: f32, release: Time) -> Option<Self> {
    if attack <= 0. || decay <= 0. || sustain < 0. || release < 0. {
      return None;
    }

    Some(Self {
      attack,
      decay,
      sustain,
      release,
      state: ADSRState::Off(-1. / 0.) // -Inf should be enough for anyone trying to release :D
    })
  }

  // Switch on.
  pub fn on(&mut self, t: Time) {
    self.state = ADSRState::On(t);
  }

  // Switch off.
  pub fn off(&mut self, t: Time) {
    self.state = ADSRState::Off(t);
  }

  // Get the state of the envelope.
  pub fn state(&self) -> ADSRState {
    self.state
  }

  // Get the current value based on the current time.
  pub fn get(&self, t: Time) -> f32 {
    match self.state {
      ADSRState::On(t_0) => {
        let attack_ = t_0 + self.attack;

        if t <= attack_ { // attacking
          return (t - t_0) / self.attack;
        }

        let decay_ = attack_ + self.decay;
        
        if t <= t_0 + decay_{ // decaying
          let nt = (t - attack_) / self.decay;
          1. + nt * (self.sustain - 1.)
        } else { // sustaining
          self.sustain
        }
      }

      ADSRState::Off(t_0) => {
        // release only possible here
        let nt = (t - t_0) / self.release;
        let q = (1. - nt) * self.sustain;
        q.max(0.) // ensure we don’t get weird negative values if we forget to switch the ADSR off
      }
    }
  }
}
