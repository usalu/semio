//! 🌈 MuPDF 1.26.7 DeviceCMYK image conversion as a 17 x 17 x 17 x 17 sRGB grid; its own crate keeps the 250 KB table out of the page editor's source boundary (see `📜️cmyk-image-clut.py`).

pub const GRID: usize = 17;

pub static TABLE: &[u8; 3 * GRID * GRID * GRID * GRID] = include_bytes!("🌈️cmyk-image-clut.bin");
