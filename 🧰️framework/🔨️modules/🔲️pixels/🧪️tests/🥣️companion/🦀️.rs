//! 🥣️ Companion tests of pixels: PNG and JPEG codecs, editing, sampling, coverage, compositing and the retirement of raster leases.
//!
//! 🧭️ The root laws are mounted flat; each module's laws sit below a wrapper of the same name that restates the module's imports, so the laws that share witnesses through `crate::retirement::tests` keep their paths.
#![cfg(test)]

use semio_framework_test_fixtures::fixture_path;

pub use semio_framework_pixels::*;
pub use semio_framework_pixels::testkit::deflate;

use std::{path::{Path, PathBuf}, sync::OnceLock};

pub fn product_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| fixture_path!("../../../.."))
}

#[path = "../../🧪️testing/📖️fixtures/🦀️.rs"]
mod fixture_reads;
pub use fixture_reads::{text,bytes};

//#region Tests
#[path = "../🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

#[path = "✍️editing/🦀️.rs"]
pub mod editing;

#[path = "↗️affine-sampling/🦀️.rs"]
pub mod affine_sampling;

#[path = "📷️png-encoding/🦀️.rs"]
pub mod png_encoding;

#[path = "🧱️png-decoding/🦀️.rs"]
pub mod png_decoding;

#[path = "📸️jpeg-decoding/🦀️.rs"]
pub mod jpeg_decoding;

#[path = "🖼️image-decoding/🦀️.rs"]
pub mod image_decoding;

#[path = "🖊️coverage/🦀️.rs"]
pub mod coverage;

