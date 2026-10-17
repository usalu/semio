//! 🖼️ Owned raster image codecs (PNG, RFC1950 zlib/DEFLATE) and pixel-buffer utilities — no
//! third-party runtime dependency. Ports the pure algorithmic core already proven inside
//! `semio-s-plugin-stdio`'s own `📷️png`/`🗜️deflate` artifacts (ticket
//! 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, wave 1) down to the
//! framework tier so `✏️s/🔌️plugins/**` crates never need the third-party `png`/`image` crates
//! at runtime. Ancillary PNG metadata (gAMA/cHRM/sRGB/pHYs/tIME/bKGD/text chunks) is parsed only
//! insofar as it affects pixel reconstruction — this module always canonicalizes into flat RGBA8.

use std::fmt;

//#region Error
/// ⚠️ Raster codec failure — a malformed/unsupported input, never a panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RasterError {
    Codec(String),
}

impl fmt::Display for RasterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(message) => write!(formatter, "raster codec error: {message}"),
        }
    }
}

impl std::error::Error for RasterError {}

impl From<String> for RasterError {
    fn from(message: String) -> Self {
        Self::Codec(message)
    }
}
//#endregion Error

//#region RasterImage
/// 🎨️ Row-major 8-bit RGBA image with interleaved channels; pixel `(x, y)` occupies
/// `pixels[(y * width + x) * 4 ..][..4]`.
#[derive(Clone, Debug, PartialEq, Eq, Default, semio_framework_value::RetireOwned)]
pub struct RasterImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl RasterImage {
    /// 🎨️ Zero-filled (transparent black) RGBA image of the given size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, pixels: vec![0u8; (width as usize) * (height as usize) * 4] }
    }
}
//#endregion RasterImage

//#region Crc32
/// 🧮️ ISO 3309 / ITU-T V.42 CRC-32 (the PNG §5.4 checksum).
fn crc32(data: &[u8]) -> u32 {
    crc32_update(0xFFFF_FFFF, data) ^ 0xFFFF_FFFF
}

/// 🧮️ The 256-entry CRC-32 table, built once at compile time — the per-byte cost of
/// [`crc32_update`] is one lookup, which keeps a streamed IDAT check proportional to the bytes a
/// decoder step actually consumes.
const CRC32_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut index = 0usize;
    while index < 256 {
        let mut value = index as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 != 0 { 0xEDB88320 ^ (value >> 1) } else { value >> 1 };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
};

/// ➕️ Folds `data` into a running (pre-inversion) CRC-32 register.
fn crc32_update(mut crc: u32, data: &[u8]) -> u32 {
    for &byte in data {
        crc = CRC32_TABLE[((crc ^ byte as u32) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc
}
//#endregion Crc32

//#region Deflate
#[path = "🗜️deflate/🦀️.rs"]
mod deflate;
//#endregion Deflate

#[path = "🧱️png-chunks-and-header/🦀️.rs"]
mod png_chunks_and_header;
use png_chunks_and_header::*;

#[path = "🧮️png-filters-and-samples/🦀️.rs"]
mod png_filters_and_samples;
use png_filters_and_samples::*;

#[path = "📤️png-encoding-core/🦀️.rs"]
mod png_encoding_core;
pub use png_encoding_core::*;

#[path = "🔄️scanline-decoder/🦀️.rs"]
mod scanline_decoder;
pub use scanline_decoder::*;

//#region PngCodec

#[path = "📷️png/✍️encode/🦀️.rs"]
pub mod png_encoding;
#[path = "📷️png/📥️decode/🦀️.rs"]
pub mod png_decoding;
#[path = "📸️jpeg/📥️decode/🦀️.rs"]
pub mod jpeg_decoding;
#[path = "🖼️image/📥️decode/🦀️.rs"]
pub mod image_decoding;

//#endregion PngCodec

//#region Resize
/// 📐️ Bilinear-resamples an RGBA8 image to `(dst_width, dst_height)`; degenerates to a copy when
/// the target size matches the source, and to transparent black when either target dimension is
/// zero.
pub fn resize_bilinear(image: &RasterImage, dst_width: u32, dst_height: u32) -> RasterImage {
    if dst_width == image.width && dst_height == image.height {
        return image.clone();
    }
    if dst_width == 0 || dst_height == 0 || image.width == 0 || image.height == 0 {
        return RasterImage::new(dst_width, dst_height);
    }
    let mut out = RasterImage::new(dst_width, dst_height);
    let scale_x = image.width as f64 / dst_width as f64;
    let scale_y = image.height as f64 / dst_height as f64;
    let get = |x: u32, y: u32, channel: usize| -> f64 {
        let idx = ((y * image.width + x) * 4) as usize + channel;
        image.pixels[idx] as f64
    };
    for dy in 0..dst_height {
        let sy = ((dy as f64 + 0.5) * scale_y - 0.5).max(0.0);
        let y0 = sy.floor() as u32;
        let y1 = (y0 + 1).min(image.height - 1);
        let fy = sy - y0 as f64;
        for dx in 0..dst_width {
            let sx = ((dx as f64 + 0.5) * scale_x - 0.5).max(0.0);
            let x0 = sx.floor() as u32;
            let x1 = (x0 + 1).min(image.width - 1);
            let fx = sx - x0 as f64;
            let out_idx = ((dy * dst_width + dx) * 4) as usize;
            for channel in 0..4 {
                let top = get(x0, y0, channel) * (1.0 - fx) + get(x1, y0, channel) * fx;
                let bottom = get(x0, y1, channel) * (1.0 - fx) + get(x1, y1, channel) * fx;
                let value = top * (1.0 - fy) + bottom * fy;
                out.pixels[out_idx + channel] = value.round().clamp(0.0, 255.0) as u8;
            }
        }
    }
    out
}
//#endregion Resize

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod deflate {
        use super::super::deflate as inner;
        pub fn zlib_compress(data: &[u8]) -> Vec<u8> {
            inner::zlib_compress(data)
        }
        pub fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
            inner::zlib_decompress(data)
        }
    }
}
