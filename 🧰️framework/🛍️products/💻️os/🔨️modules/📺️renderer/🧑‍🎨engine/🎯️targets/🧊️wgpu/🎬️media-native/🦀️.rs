//! 🎬️ Native media host: WAV and first-party MP4 decoding, the media transport and the native player.

#[path = "../../../🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native/🦀️.rs"]
mod media_native;
pub use media_native::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🖼️presenter.rs"]
mod presenter_tests;
