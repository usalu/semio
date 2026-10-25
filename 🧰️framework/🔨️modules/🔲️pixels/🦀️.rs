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
    /// 🔲️ Zero-filled (transparent black) RGBA image of the given size.
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

/// 📋️ The 256-entry CRC-32 table, built once at compile time — the per-byte cost of
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

/// 📉️ Resamples an RGBA8 image to `(dst_width, dst_height)` with the separable triangle filter whose support widens with the
/// reduction ratio, so a large downscale averages every covered source pixel instead of aliasing; vertical pass first into
/// unrounded `f32` rows, horizontal pass second, one rounding at the end. Degenerates exactly like [`resize_bilinear`].
pub fn resize_triangle(image: &RasterImage, dst_width: u32, dst_height: u32) -> RasterImage {
    if dst_width == image.width && dst_height == image.height {
        return image.clone();
    }
    if dst_width == 0 || dst_height == 0 || image.width == 0 || image.height == 0 {
        return RasterImage::new(dst_width, dst_height);
    }
    let width = image.width as usize;
    let height = image.height as usize;
    let weights = |source: usize, target: u32| -> Vec<(usize, Vec<f32>)> {
        let ratio = source as f32 / target as f32;
        let scale = ratio.max(1.0);
        (0..target)
            .map(|out| {
                let center = (out as f32 + 0.5) * ratio;
                let left = ((center - scale).floor() as i64).clamp(0, source as i64 - 1) as usize;
                let right = ((center + scale).ceil() as i64).clamp(left as i64 + 1, source as i64) as usize;
                let center = center - 0.5;
                let mut values: Vec<f32> = (left..right)
                    .map(|index| {
                        let distance = ((index as f32 - center) / scale).abs();
                        if distance < 1.0 { 1.0 - distance } else { 0.0 }
                    })
                    .collect();
                let sum: f32 = values.iter().sum();
                values.iter_mut().for_each(|value| *value /= sum);
                (left, values)
            })
            .collect()
    };
    let vertical = weights(height, dst_height);
    let horizontal = weights(width, dst_width);
    let target_height = dst_height as usize;
    let mut middle = vec![0f32; width * target_height * 4];
    for (row, (top, rows)) in vertical.iter().enumerate() {
        for column in 0..width {
            let mut sum = [0f32; 4];
            for (offset, weight) in rows.iter().enumerate() {
                let at = ((top + offset) * width + column) * 4;
                for channel in 0..4 {
                    sum[channel] += image.pixels[at + channel] as f32 * weight;
                }
            }
            middle[(row * width + column) * 4..][..4].copy_from_slice(&sum);
        }
    }
    let mut out = RasterImage::new(dst_width, dst_height);
    for row in 0..target_height {
        for (column, (left, columns)) in horizontal.iter().enumerate() {
            let mut sum = [0f32; 4];
            for (offset, weight) in columns.iter().enumerate() {
                let at = (row * width + left + offset) * 4;
                for channel in 0..4 {
                    sum[channel] += middle[at + channel] * weight;
                }
            }
            let at = (row * dst_width as usize + column) * 4;
            for channel in 0..4 {
                out.pixels[at + channel] = sum[channel].clamp(0.0, 255.0).round() as u8;
            }
        }
    }
    out
}
//#endregion Resize

//#region Orientation
/// 🧭️ The eight EXIF orientations (tag 0x0112): the transform that turns the stored pixels upright for display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    NoTransforms,
    FlipHorizontal,
    Rotate180,
    FlipVertical,
    Rotate90FlipHorizontal,
    Rotate90,
    Rotate270FlipHorizontal,
    Rotate270,
}

impl Orientation {
    /// 🔢️ Maps the EXIF value `1..=8`; any other value names no orientation.
    pub fn from_exif(value: u16) -> Option<Self> {
        Some(match value {
            1 => Self::NoTransforms,
            2 => Self::FlipHorizontal,
            3 => Self::Rotate180,
            4 => Self::FlipVertical,
            5 => Self::Rotate90FlipHorizontal,
            6 => Self::Rotate90,
            7 => Self::Rotate270FlipHorizontal,
            8 => Self::Rotate270,
            _ => return None,
        })
    }
}

/// 🔎️ Reads the orientation entry of IFD0 in a raw TIFF/EXIF block (`II*\0` or `MM\0*` header); `None` when the block is
/// malformed or carries no orientation entry of format SHORT with count 1 and a value in `1..=8`.
pub fn tiff_orientation(block: &[u8]) -> Option<Orientation> {
    let little = match block.get(..4)? {
        [0x49, 0x49, 42, 0] => true,
        [0x4d, 0x4d, 0, 42] => false,
        _ => return None,
    };
    let short = |at: usize| -> Option<u16> {
        let bytes: [u8; 2] = block.get(at..at + 2)?.try_into().ok()?;
        Some(if little { u16::from_le_bytes(bytes) } else { u16::from_be_bytes(bytes) })
    };
    let long = |at: usize| -> Option<u32> {
        let bytes: [u8; 4] = block.get(at..at + 4)?.try_into().ok()?;
        Some(if little { u32::from_le_bytes(bytes) } else { u32::from_be_bytes(bytes) })
    };
    let directory = long(4)? as usize;
    let entries = short(directory)? as usize;
    (0..entries).find_map(|entry| {
        let at = directory.checked_add(2)?.checked_add(entry.checked_mul(12)?)?;
        (short(at)? == 0x0112 && short(at + 2)? == 3 && long(at + 4)? == 1).then(|| Orientation::from_exif(short(at + 8)?.min(255)))?
    })
}

/// ⏮️ The orientation an encoded JPEG (first `Exif` APP1 segment) or PNG (`eXIf` chunk) declares; `NoTransforms` for every
/// other format and for a missing, truncated or invalid declaration.
pub fn encoded_orientation(bytes: &[u8]) -> Orientation {
    let found = if bytes.starts_with(&[0xff, 0xd8]) {
        let mut at = 2usize;
        loop {
            while bytes.get(at) == Some(&0xff) && bytes.get(at + 1) == Some(&0xff) {
                at += 1;
            }
            let (Some(0xff), Some(&marker)) = (bytes.get(at), bytes.get(at + 1)) else { break None };
            if marker == 0xd9 || marker == 0xda {
                break None;
            }
            if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
                at += 2;
                continue;
            }
            let Some(length) = bytes.get(at + 2..at + 4).map(|pair| u16::from_be_bytes([pair[0], pair[1]]) as usize) else { break None };
            let Some(segment) = bytes.get(at + 4..at + 2 + length) else { break None };
            if marker == 0xe1 && segment.starts_with(b"Exif\0\0") {
                break tiff_orientation(&segment[6..]);
            }
            at += 2 + length;
        }
    } else if bytes.starts_with(&[0x89, b'P', b'N', b'G', 13, 10, 26, 10]) {
        let mut at = 8usize;
        loop {
            let Some(header) = bytes.get(at..at + 8) else { break None };
            let length = u32::from_be_bytes([header[0], header[1], header[2], header[3]]) as usize;
            let Some(end) = at.checked_add(8).and_then(|start| start.checked_add(length)) else { break None };
            if &header[4..8] == b"eXIf" {
                break bytes.get(at + 8..end).and_then(tiff_orientation);
            }
            if &header[4..8] == b"IEND" {
                break None;
            }
            at = end.saturating_add(4);
        }
    } else {
        None
    };
    found.unwrap_or_default()
}

/// 🔄️ Applies `orientation` to `image`, returning the upright pixels (width and height swap for the four quarter-turn cases).
pub fn apply_orientation(image: &RasterImage, orientation: Orientation) -> RasterImage {
    let (width, height) = (image.width as usize, image.height as usize);
    let (out_width, out_height) = match orientation {
        Orientation::Rotate90 | Orientation::Rotate270 | Orientation::Rotate90FlipHorizontal | Orientation::Rotate270FlipHorizontal => (height, width),
        _ => (width, height),
    };
    let mut out = RasterImage::new(out_width as u32, out_height as u32);
    for y in 0..out_height {
        for x in 0..out_width {
            let (source_x, source_y) = match orientation {
                Orientation::NoTransforms => (x, y),
                Orientation::FlipHorizontal => (width - 1 - x, y),
                Orientation::Rotate180 => (width - 1 - x, height - 1 - y),
                Orientation::FlipVertical => (x, height - 1 - y),
                Orientation::Rotate90 => (y, height - 1 - x),
                Orientation::Rotate270 => (width - 1 - y, x),
                Orientation::Rotate90FlipHorizontal => (y, x),
                Orientation::Rotate270FlipHorizontal => (width - 1 - y, out_width - 1 - x),
            };
            out.pixels[(y * out_width + x) * 4..][..4].copy_from_slice(&image.pixels[(source_y * width + source_x) * 4..][..4]);
        }
    }
    out
}
//#endregion Orientation

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
