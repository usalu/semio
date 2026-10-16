//! 🥣️ Companion tests of pixels: PNG and JPEG codecs, editing, sampling, coverage, compositing and the retirement of raster leases.
//!
//! 🧭️ The root laws are mounted flat; each module's laws sit below a wrapper of the same name that restates the module's imports, so the laws that share witnesses through `crate::retirement::tests` keep their paths.
#![cfg(test)]

pub use semio_framework_pixels::*;
pub use semio_framework_pixels::testkit::deflate;

use std::{collections::HashMap, path::{Path, PathBuf}, sync::{Mutex, OnceLock}};

pub fn product_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.."))
}

fn leaked(relative: &str) -> &'static [u8] {
    static READ: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let mut read = READ.get_or_init(Default::default).lock().unwrap();
    read.entry(relative.to_owned()).or_insert_with(|| {
        let path = product_directory().join(relative);
        Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_slice())
    })
}

pub fn text(relative: &str) -> &'static str {
    std::str::from_utf8(leaked(relative)).unwrap_or_else(|error| panic!("test input {relative} is not UTF-8: {error}"))
}

pub fn bytes(relative: &str) -> &'static [u8] {
    leaked(relative)
}

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

#[path = "🧩️compositing/🦀️.rs"]
pub mod compositing;

#[path = "📐️frames/🦀️.rs"]
pub mod frames;

#[path = "🗂️layers/🦀️.rs"]
pub mod layers;
