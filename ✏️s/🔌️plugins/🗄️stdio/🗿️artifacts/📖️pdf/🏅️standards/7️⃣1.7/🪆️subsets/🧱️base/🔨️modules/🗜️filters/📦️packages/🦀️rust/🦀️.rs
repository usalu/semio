//! 🗜️ Bounded PDF stream decoding and predictors.

#[path = "../../🌀️bounded-flate/🦀️.rs"]
mod bounded_flate;
pub use bounded_flate::*;

#[path = "../../🧬️bounded-lzw/🦀️.rs"]
mod bounded_lzw;
pub use bounded_lzw::PdfLzwCursor;

#[path = "../../🪣️stream-data/🦀️.rs"]
mod stream_data;
pub use stream_data::{PdfStreamBytes, PdfStreamDataCursor, StreamDataGrant, StreamDataReceipt, StreamDataStatus};

#[cfg(test)]
#[path = "../../🧪️tests/🔬️flate/🦀️.rs"]
mod flate_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🧮️bounded-predictor/🦀️.rs"]
mod predictor_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🧬️lzw/🦀️.rs"]
mod lzw_tests;
