//! 🧩️ Compositing, frame and layer laws retain their original shared physical-allocation witnesses.
#![cfg(test)]
pub use semio_framework_pixels::*;
use semio_framework_test_fixtures::fixture_path;
use std::{path::{Path,PathBuf},sync::OnceLock};
pub fn product_directory()->&'static Path{static ROOT:OnceLock<PathBuf>=OnceLock::new();ROOT.get_or_init(||fixture_path!("../../../../.."))}
#[path="../../../🧪️testing/📖️fixtures/🦀️.rs"]
mod fixture_reads;
pub use fixture_reads::{text,bytes};
#[path="../../../🧪️tests/🥣️companion/♻️retirement/🦀️.rs"]
pub mod retirement;
#[path="../../../🧪️tests/🥣️companion/🧩️compositing/🦀️.rs"]
pub mod compositing;
#[path="../../../🧪️tests/🥣️companion/📐️frames/🦀️.rs"]
pub mod frames;
#[path="../../../🧪️tests/🥣️companion/🗂️layers/🦀️.rs"]
pub mod layers;
