#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;

pub mod channel;
pub mod chorus;
mod conv;
pub mod gen;
pub mod modulator;
pub mod reverb;
pub mod settings;
pub mod soundfont;
pub mod synth;
pub mod tuning;
pub mod voice;
mod voice_pool;
