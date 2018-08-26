//! # Demoscene-oriented software synthesizer.
//!
//! This crate exposes a full-featured audio synthesizer that supports multi-channel instruments,
//! envelopes, audio effects and is backend agnostic, which means you can use this crate with pretty
//! much any kind of technology (OpenAL, ALSA/Pulseaudio, Core Audio APIs, etc.). This crate doesn’t
//! provide per-se a way to actually play anything on your hardware. Please head over to the
//! appropriate crate.
//!
//! ## About `no_std`
//!
//! Because this crate is built for demoscene purposes first, it supports building without the Rust
//! standard library (aka. `std`). This is extremely important and won’t ever be removed.
//!
//! # Features
//!
//! ## Oscillators
//!
//! The most simple form of audio signal this crate gives you is an `Oscillator`. An `Oscillator` is
//! a continuous representation of a very simple wave sound – e.g. a *sine wave*, a *sawtooth* wave,
//! etc.
//!
//! In this crate, the sound waves held by oscillators are normalized: the full period must be fully
//! represented in `[0; 1]`. For instance, for a *sine wave* to be used as an oscillator, it’s
//! period must be fully represented from 0 to 1, mapping 0 to 0 and 2π to 1 (since the *sine*
//! function is 2π-periodic).
//!
//! This rule of normalization is used pretty much everywhere in the crate, so ensure you are
//! completely comfortable with the idea.
//!
//! ## Instruments
//!
//! The basic and primitive block of this crate is an instrument. An instrument is an audio signal
//! producer. It can produce an audio signal via `Note`s. Instruments have the concept of “holding”
//! and “releasing” notes. This enables to build interesting effects and audio signatures, such as
//! sustaine, repeating / cycling sound, etc.
//!
//! The audio signal output from instruments can then be taken out and passed to other audio blocks
//! for further audio processing.
//!
//! ## Multi-channel instruments
//!
//! By default, all instruments support the concept of multi-channeling. This allows for holding
//! more than one note at a time. Imagine playing the guitar: you typically very often have chords
//! or play several strings at once, yielding several notes at the same time. Multi-channeling
//! introduces this simple concept.
//!
//! When asking an instrument to play a note, you can optionally ask the instrument to play the note
//! on a given `NoteChannel`, allowing to play several notes at the same time.
//!
//! ## Envelopes
//!
//! Envelopes are typically used to modify the volume of an audio signal on the fly. This crate
//! gives you
//! [ADSR](https://en.wikipedia.org/wiki/Synthesizer#Attack_Decay_Sustain_Release_(ADSR)_envelope)
//! envelopes, that can be parametered to achieve interesting sound effects.
//!
//! # Special thanks
//!
//! I’d like to give a big *thank you* to Alkama (a.k.a. Mathieu Moncharmont) for all of his
//! theoretical teaching about audio signal processing and how to yield a clear and proper signal.
//! **Thank you!**

#![no_std]
#![feature(alloc)]
#![feature(core_intrinsics)]

extern crate alloc;

pub mod envelope;
pub mod instrument;
pub mod hertz;
pub mod note;
pub mod oscillator;
pub mod sample;
pub mod time;
