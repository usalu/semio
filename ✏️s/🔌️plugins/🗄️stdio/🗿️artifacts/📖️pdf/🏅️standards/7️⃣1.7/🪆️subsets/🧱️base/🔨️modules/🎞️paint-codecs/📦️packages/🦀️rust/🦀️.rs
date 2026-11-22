//! 🎞️ In-house PDF image and paint codecs. Every decoder reports progress and stops when `Control` returns false.

#[path = "../../../🖼️images/🧮️arithmetic/🦀️.rs"]
mod arithmetic;

#[path = "../../../🗜️filters/🧾️ccitt/🦀️.rs"]
pub mod ccitt;

#[path = "../../../🗜️filters/🧩️jbig2/🦀️.rs"]
pub mod jbig2;

#[path = "../../../🖼️images/🌅️jpx/🦀️.rs"]
pub mod jpx;

#[path = "../../../🌅️mesh-shading/🦀️.rs"]
pub mod mesh;

#[path = "../../../🗜️filters/🧮️postscript/🦀️.rs"]
pub mod postscript;

/// 🎛️ Progress and cancellation hook: receives completion in `0.0..=1.0` and returns `false` to cancel.
pub type Control<'a> = &'a mut dyn FnMut(f32) -> bool;

/// 🚫️ Why a codec stopped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RasterError {
    Malformed(&'static str),
    Unsupported(&'static str),
    TooLarge,
    Cancelled,
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️codecs/🦀️.rs"]
mod codec_tests;
