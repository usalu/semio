//! 🖼️ Image containers and classical image processing: grayscale pyramids, gradients, filtering, patch correlation and PNG codec behind an interface.

// #region 🔖️Types
/// 🌫️ Row-major single-channel image with luma values in `[0, 1]`; pixel `(x, y)` lives at `data[y * width + x]`.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageGray {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
}

impl ImageGray {
    /// 🌫️ Zero-filled grayscale image of the given size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, data: vec![0.0; (width as usize) * (height as usize)] }
    }

    /// 🔍️ Pixel value at integer coordinates `(x, y)`.
    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[(y * self.width + x) as usize]
    }

    /// ✏️ Writes the pixel value at integer coordinates `(x, y)`.
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        self.data[(y * self.width + x) as usize] = value;
    }

    /// 🎚️ Grayscale conversion of an RGBA image via BT.601 luma weights `0.299 R + 0.587 G + 0.114 B`, normalized to `[0, 1]`.
    /// <https://en.wikipedia.org/wiki/Rec._601>
    pub fn from_rgba8_luma(src: &ImageRgba8) -> Self {
        let mut out = Self::new(src.width, src.height);
        for (dst, px) in out.data.iter_mut().zip(src.data.as_chunks::<4>().0.iter()) {
            *dst = (0.299 * f32::from(px[0]) + 0.587 * f32::from(px[1]) + 0.114 * f32::from(px[2])) / 255.0;
        }
        out
    }

    /// 📐️ Bilinear sample at subpixel `(x, y)`; coordinates are clamped to the image border, and an empty image samples to `0`.
    pub fn sample(&self, x: f32, y: f32) -> f32 {
        if self.width == 0 || self.height == 0 {
            return 0.0;
        }
        let xf = x.clamp(0.0, (self.width - 1) as f32);
        let yf = y.clamp(0.0, (self.height - 1) as f32);
        let x0 = xf.floor() as u32;
        let y0 = yf.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);
        let fx = xf - x0 as f32;
        let fy = yf - y0 as f32;
        let top = self.get(x0, y0) * (1.0 - fx) + self.get(x1, y0) * fx;
        let bottom = self.get(x0, y1) * (1.0 - fx) + self.get(x1, y1) * fx;
        top * (1.0 - fy) + bottom * fy
    }
}

/// 🎨️ Row-major 8-bit RGBA image with interleaved channels; pixel `(x, y)` occupies `data[(y * width + x) * 4 ..][..4]`.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageRgba8 {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl ImageRgba8 {
    /// 🎨️ Zero-filled (transparent black) RGBA image of the given size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, data: vec![0; (width as usize) * (height as usize) * 4] }
    }

    /// 🔍️ RGBA pixel at integer coordinates `(x, y)`.
    pub fn get_rgba(&self, x: u32, y: u32) -> [u8; 4] {
        let idx = ((y * self.width + x) * 4) as usize;
        [self.data[idx], self.data[idx + 1], self.data[idx + 2], self.data[idx + 3]]
    }

    /// 📐️ Bilinear RGB sample at subpixel `(x, y)`, normalized to `[0, 1]` per channel; coordinates are clamped to the image border, and an empty image samples to black.
    pub fn sample_rgb(&self, x: f32, y: f32) -> [f32; 3] {
        if self.width == 0 || self.height == 0 {
            return [0.0; 3];
        }
        let xf = x.clamp(0.0, (self.width - 1) as f32);
        let yf = y.clamp(0.0, (self.height - 1) as f32);
        let x0 = xf.floor() as u32;
        let y0 = yf.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);
        let fx = xf - x0 as f32;
        let fy = yf - y0 as f32;
        let p00 = self.get_rgba(x0, y0);
        let p10 = self.get_rgba(x1, y0);
        let p01 = self.get_rgba(x0, y1);
        let p11 = self.get_rgba(x1, y1);
        let mut out = [0.0f32; 3];
        for (channel, slot) in out.iter_mut().enumerate() {
            let top = f32::from(p00[channel]) * (1.0 - fx) + f32::from(p10[channel]) * fx;
            let bottom = f32::from(p01[channel]) * (1.0 - fx) + f32::from(p11[channel]) * fx;
            *slot = (top * (1.0 - fy) + bottom * fy) / 255.0;
        }
        out
    }
}
// #endregion 🔖️Types

// #region 🔖️Codec
/// ⚠️ Error type for image codec operations (PNG, JPEG); `Dimensions` signals a size/buffer mismatch before any encoding is attempted, and `UnsupportedJpeg` flags progressive/arithmetic/non-baseline JPEG variants this decoder deliberately does not attempt.
#[derive(Clone, Debug, PartialEq)]
pub enum ImageError {
    Decode(String),
    Encode(String),
    Dimensions,
    UnsupportedJpeg(String),
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(msg) => write!(f, "image decode failed: {msg}"),
            Self::Encode(msg) => write!(f, "image encode failed: {msg}"),
            Self::Dimensions => write!(f, "image dimensions do not match the pixel buffer"),
            Self::UnsupportedJpeg(msg) => write!(f, "unsupported jpeg variant: {msg}"),
        }
    }
}

impl std::error::Error for ImageError {}

/// 📥️ Decodes a PNG byte stream into RGBA; 8-bit gray, gray-alpha, RGB and RGBA (plus paletted) inputs are expanded to RGBA, and 16-bit inputs are downconverted to 8-bit.
/// <https://www.w3.org/TR/png-3/>
pub fn decode_png(bytes: &[u8]) -> Result<ImageRgba8, ImageError> {
    let mut decoder = png::Decoder::new(bytes);
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().map_err(|e| ImageError::Decode(e.to_string()))?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).map_err(|e| ImageError::Decode(e.to_string()))?;
    if info.bit_depth != png::BitDepth::Eight {
        return Err(ImageError::Decode(format!("unsupported bit depth after normalization: {:?}", info.bit_depth)));
    }
    let pixels = &buf[..info.buffer_size()];
    let mut out = ImageRgba8::new(info.width, info.height);
    match info.color_type {
        png::ColorType::Rgba => out.data.copy_from_slice(pixels),
        png::ColorType::Rgb => {
            for (dst, src) in out.data.as_chunks_mut::<4>().0.iter_mut().zip(pixels.as_chunks::<3>().0.iter()) {
                dst[0] = src[0];
                dst[1] = src[1];
                dst[2] = src[2];
                dst[3] = 255;
            }
        }
        png::ColorType::Grayscale => {
            for (dst, &luma) in out.data.as_chunks_mut::<4>().0.iter_mut().zip(pixels.iter()) {
                dst[0] = luma;
                dst[1] = luma;
                dst[2] = luma;
                dst[3] = 255;
            }
        }
        png::ColorType::GrayscaleAlpha => {
            for (dst, src) in out.data.as_chunks_mut::<4>().0.iter_mut().zip(pixels.as_chunks::<2>().0.iter()) {
                dst[0] = src[0];
                dst[1] = src[0];
                dst[2] = src[0];
                dst[3] = src[1];
            }
        }
        other => return Err(ImageError::Decode(format!("unsupported color type: {other:?}"))),
    }
    Ok(out)
}

/// 📤️ Encodes an RGBA image as an 8-bit RGBA PNG byte stream.
pub fn encode_png(img: &ImageRgba8) -> Result<Vec<u8>, ImageError> {
    if img.width == 0 || img.height == 0 || img.data.len() != (img.width as usize) * (img.height as usize) * 4 {
        return Err(ImageError::Dimensions);
    }
    let mut out = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut out, img.width, img.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|e| ImageError::Encode(e.to_string()))?;
        writer.write_image_data(&img.data).map_err(|e| ImageError::Encode(e.to_string()))?;
    }
    Ok(out)
}

/// 📤️ Encodes row-major 16-bit grayscale samples as a 16-bit grayscale PNG byte stream (big-endian per the PNG spec), for lossless DSM/heightfield export.
pub fn encode_png_gray16(data: &[u16], width: u32, height: u32) -> Result<Vec<u8>, ImageError> {
    if width == 0 || height == 0 || data.len() != (width as usize) * (height as usize) {
        return Err(ImageError::Dimensions);
    }
    let mut bytes = Vec::with_capacity(data.len() * 2);
    for &sample in data {
        bytes.extend_from_slice(&sample.to_be_bytes());
    }
    let mut out = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut out, width, height);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Sixteen);
        let mut writer = encoder.write_header().map_err(|e| ImageError::Encode(e.to_string()))?;
        writer.write_image_data(&bytes).map_err(|e| ImageError::Encode(e.to_string()))?;
    }
    Ok(out)
}

// #region 🔖️JpegTables
/// 🔀️ Zig-zag scan position `k` (0..64) to natural row-major block index `y * 8 + x`, per JPEG's DCT coefficient serialization order.
/// <https://www.w3.org/Graphics/JPEG/itu-t81.pdf> (Annex A, Figure A.6)
const JPEG_ZIGZAG: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

/// 🧮️ Annex K.1 luminance quantization base table, natural (row-major) order.
const JPEG_LUMA_QUANT_BASE: [u16; 64] = [
    16, 11, 10, 16, 24, 40, 51, 61, 12, 12, 14, 19, 26, 58, 60, 55, 14, 13, 16, 24, 40, 57, 69, 56, 14, 17, 22, 29, 51, 87, 80, 62, 18, 22, 37, 56, 68, 109, 103, 77, 24, 35, 55, 64, 81, 104, 113, 92, 49, 64, 78, 87, 103, 121, 120, 101, 72, 92, 95,
    98, 112, 100, 103, 99,
];

/// 🧮️ Annex K.2 chrominance quantization base table, natural (row-major) order.
const JPEG_CHROMA_QUANT_BASE: [u16; 64] = [
    17, 18, 24, 47, 99, 99, 99, 99, 18, 21, 26, 66, 99, 99, 99, 99, 24, 26, 56, 99, 99, 99, 99, 99, 47, 66, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99,
];

/// 🌳️ Canonical Huffman spec as (code-length counts for lengths 1..=16, symbols in code order); shared wire format for DHT segments and the Annex K.3 recommended tables.
struct JpegHuffSpec {
    counts: [u8; 16],
    symbols: &'static [u8],
}

const JPEG_STD_DC_LUMA: JpegHuffSpec = JpegHuffSpec { counts: [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0], symbols: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] };
const JPEG_STD_DC_CHROMA: JpegHuffSpec = JpegHuffSpec { counts: [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0], symbols: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] };

const JPEG_STD_AC_LUMA: JpegHuffSpec = JpegHuffSpec {
    counts: [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 0x7D],
    symbols: &[
        0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xA1, 0x08, 0x23, 0x42, 0xB1, 0xC1, 0x15, 0x52, 0xD1, 0xF0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0A, 0x16,
        0x17, 0x18, 0x19, 0x1A, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
        0x6A, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6,
        0xB7, 0xB8, 0xB9, 0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8,
        0xF9, 0xFA,
    ],
};

const JPEG_STD_AC_CHROMA: JpegHuffSpec = JpegHuffSpec {
    counts: [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 0x77],
    symbols: &[
        0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71, 0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xA1, 0xB1, 0xC1, 0x09, 0x23, 0x33, 0x52, 0xF0, 0x15, 0x62, 0x72, 0xD1, 0x0A, 0x16, 0x24, 0x34,
        0xE1, 0x25, 0xF1, 0x17, 0x18, 0x19, 0x1A, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6A, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2, 0xB3, 0xB4,
        0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8,
        0xF9, 0xFA,
    ],
};

/// 🧮️ IJG Annex K quality scaling: `quality < 50` scales coarser than 100%, `quality > 50` scales finer, `quality == 50` is identity; each base entry is scaled and clamped to the 8-bit DQT range `[1, 255]`.
fn jpeg_scale_quant_table(base: &[u16; 64], quality: u8) -> [u16; 64] {
    let q = i32::from(quality.clamp(1, 100));
    let scale = if q < 50 { 5000 / q } else { 200 - 2 * q };
    let mut out = [0u16; 64];
    for (dst, &src) in out.iter_mut().zip(base.iter()) {
        *dst = ((i32::from(src) * scale + 50) / 100).clamp(1, 255) as u16;
    }
    out
}
// #endregion 🔖️JpegTables

// #region 🔖️JpegHuffman
/// 🌳️ Decode-side canonical Huffman table built via the JPEG spec's `mincode`/`maxcode`/`valptr` algorithm (Annex F, Figure F.16 / F.22).
struct JpegHuffDecodeTable {
    mincode: [i32; 17],
    maxcode: [i32; 17],
    valptr: [i32; 17],
    symbols: Vec<u8>,
}

impl JpegHuffDecodeTable {
    /// 🌳️ Assigns canonical codes to `symbols` in bit-length order (owned, so DHT-parsed tables never leak memory) and records the per-length decode boundaries.
    fn build(counts: &[u8; 16], symbols: Vec<u8>) -> Self {
        let mut mincode = [0i32; 17];
        let mut maxcode = [-1i32; 17];
        let mut valptr = [0i32; 17];
        let mut code = 0i32;
        let mut k = 0i32;
        for len in 1..=16usize {
            let count = i32::from(counts[len - 1]);
            if count > 0 {
                valptr[len] = k;
                mincode[len] = code;
                code += count;
                k += count;
                maxcode[len] = code - 1;
            }
            code <<= 1;
        }
        Self { mincode, maxcode, valptr, symbols }
    }

    /// 🔍️ Symbol at zero-based `valptr`-relative `index`, bounds-checked against corrupt/truncated-derived indices.
    fn symbol_at(&self, index: i32) -> Option<u8> {
        usize::try_from(index).ok().and_then(|i| self.symbols.get(i).copied())
    }
}

/// 🌳️ Encode-side canonical code table: symbol byte value → `(code, bit length)`, built with the same canonical assignment as the decode table.
fn jpeg_build_encode_table(spec: &JpegHuffSpec) -> [Option<(u16, u8)>; 256] {
    let mut table: [Option<(u16, u8)>; 256] = [None; 256];
    let mut code = 0u16;
    let mut k = 0usize;
    for len in 1..=16u8 {
        let count = spec.counts[len as usize - 1] as usize;
        for _ in 0..count {
            table[spec.symbols[k] as usize] = Some((code, len));
            code += 1;
            k += 1;
        }
        code <<= 1;
    }
    table
}

/// 🧮️ JPEG magnitude category (`SSSS`): the number of bits needed to represent `abs(value)`, `0` for `value == 0`.
fn jpeg_category(value: i32) -> u8 {
    (32 - value.unsigned_abs().leading_zeros()) as u8
}

/// 🧮️ EXTEND procedure (Annex F, Figure F.12): reconstructs a signed coefficient from its `size`-bit unsigned code word.
fn jpeg_extend(bits: u32, size: u8) -> i32 {
    if size == 0 {
        return 0;
    }
    let bits = bits as i32;
    let half = 1i32 << (size - 1);
    if bits < half {
        bits - (1 << size) + 1
    } else {
        bits
    }
}

/// 🧮️ Inverse of `jpeg_extend`: the `size`-bit code word for a signed coefficient already known to fit its category.
fn jpeg_signed_bits(value: i32, size: u8) -> u16 {
    if value < 0 {
        (value + (1 << size) - 1) as u16
    } else {
        value as u16
    }
}
// #endregion 🔖️JpegHuffman

// #region 🔖️JpegBits
/// 📖️ MSB-first bit reader over an entropy-coded JPEG scan; transparently undoes byte stuffing (`0xFF 0x00` → literal `0xFF`) and stops (without consuming) at any other marker so restart markers can be handled by the caller.
struct JpegBitReader<'a> {
    data: &'a [u8],
    pos: usize,
    acc: u32,
    nbits: u32,
}

impl<'a> JpegBitReader<'a> {
    /// 📖️ Reader starting at byte offset `pos` in `data`.
    fn new(data: &'a [u8], pos: usize) -> Self {
        Self { data, pos, acc: 0, nbits: 0 }
    }

    /// 📖️ Next destuffed entropy byte, or `Ok(None)` when the stream is sitting at an unconsumed marker.
    fn fill_byte(&mut self) -> Result<Option<u8>, ImageError> {
        let &byte = self.data.get(self.pos).ok_or_else(|| ImageError::Decode("truncated jpeg entropy stream".to_string()))?;
        if byte != 0xFF {
            self.pos += 1;
            return Ok(Some(byte));
        }
        let &next = self.data.get(self.pos + 1).ok_or_else(|| ImageError::Decode("truncated jpeg entropy stream".to_string()))?;
        if next == 0x00 {
            self.pos += 2;
            Ok(Some(0xFF))
        } else {
            Ok(None)
        }
    }

    /// 📖️ Single next bit, or `Ok(None)` at a marker boundary.
    fn next_bit(&mut self) -> Result<Option<u32>, ImageError> {
        if self.nbits == 0 {
            match self.fill_byte()? {
                Some(byte) => {
                    self.acc = u32::from(byte);
                    self.nbits = 8;
                }
                None => return Ok(None),
            }
        }
        self.nbits -= 1;
        Ok(Some((self.acc >> self.nbits) & 1))
    }

    /// 📖️ `n` bits as an unsigned value, MSB first; a marker boundary before `n` bits are collected is a decode error.
    fn receive_bits(&mut self, n: u8) -> Result<u32, ImageError> {
        let mut value = 0u32;
        for _ in 0..n {
            let bit = self.next_bit()?.ok_or_else(|| ImageError::Decode("unexpected marker inside jpeg entropy stream".to_string()))?;
            value = (value << 1) | bit;
        }
        Ok(value)
    }

    /// 📖️ Decodes one Huffman symbol by walking bit-by-bit through `table`'s canonical code ranges.
    fn decode_huff(&mut self, table: &JpegHuffDecodeTable) -> Result<u8, ImageError> {
        let mut code = 0i32;
        for len in 1..=16usize {
            let bit = self.next_bit()?.ok_or_else(|| ImageError::Decode("unexpected marker inside jpeg entropy stream".to_string()))?;
            code = (code << 1) | bit as i32;
            if table.maxcode[len] != -1 && code <= table.maxcode[len] {
                return table.symbol_at(table.valptr[len] + (code - table.mincode[len])).ok_or_else(|| ImageError::Decode("huffman symbol out of range".to_string()));
            }
        }
        Err(ImageError::Decode("invalid huffman code in jpeg entropy stream".to_string()))
    }

    /// 🔁️ Discards any partially-consumed byte and reads a `0xFFDn` restart marker at the now byte-aligned position.
    fn consume_restart_marker(&mut self) -> Result<(), ImageError> {
        self.acc = 0;
        self.nbits = 0;
        let &marker = self.data.get(self.pos).ok_or_else(|| ImageError::Decode("truncated jpeg stream before restart marker".to_string()))?;
        let &kind = self.data.get(self.pos + 1).ok_or_else(|| ImageError::Decode("truncated jpeg stream before restart marker".to_string()))?;
        if marker != 0xFF || !(0xD0..=0xD7).contains(&kind) {
            return Err(ImageError::Decode("expected jpeg restart marker".to_string()));
        }
        self.pos += 2;
        Ok(())
    }
}

/// ✍️ MSB-first bit writer that applies JPEG byte stuffing (`0xFF` → `0xFF 0x00`) as bytes are emitted.
#[derive(Default)]
struct JpegBitWriter {
    bytes: Vec<u8>,
    acc: u32,
    nbits: u32,
}

impl JpegBitWriter {
    /// ✍️ Appends the low `len` bits of `code`, most significant bit first.
    fn put_bits(&mut self, code: u16, len: u8) {
        if len == 0 {
            return;
        }
        self.acc = (self.acc << len) | (u32::from(code) & ((1u32 << len) - 1));
        self.nbits += u32::from(len);
        while self.nbits >= 8 {
            self.nbits -= 8;
            let byte = ((self.acc >> self.nbits) & 0xFF) as u8;
            self.bytes.push(byte);
            if byte == 0xFF {
                self.bytes.push(0x00);
            }
        }
    }

    /// ✍️ Pads any partial final byte with low zero bits and flushes it.
    fn flush(&mut self) {
        if self.nbits > 0 {
            let byte = ((self.acc << (8 - self.nbits)) & 0xFF) as u8;
            self.bytes.push(byte);
            if byte == 0xFF {
                self.bytes.push(0x00);
            }
            self.acc = 0;
            self.nbits = 0;
        }
    }
}
// #endregion 🔖️JpegBits

// #region 🔖️JpegDct
/// 🌊️ 8-point orthonormal DCT-II basis value `cos[(2x+1) u π / 16]`.
fn jpeg_dct_basis(x: usize, u: usize) -> f64 {
    (((2 * x + 1) as f64) * (u as f64) * std::f64::consts::PI / 16.0).cos()
}

/// 🌊️ 8-point forward DCT-II, orthonormally scaled (`C(0) = 1/√2`, `C(u > 0) = 1`, overall factor `1/2`) so it is its own exact matrix inverse via `jpeg_idct_1d`.
fn jpeg_dct_1d(input: &[f64; 8]) -> [f64; 8] {
    let mut out = [0.0f64; 8];
    for (u, slot) in out.iter_mut().enumerate() {
        let cu = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
        let sum: f64 = (0..8).map(|x| input[x] * jpeg_dct_basis(x, u)).sum();
        *slot = 0.5 * cu * sum;
    }
    out
}

/// 🌊️ 8-point inverse DCT-III, the exact transpose of `jpeg_dct_1d`'s orthonormal matrix.
fn jpeg_idct_1d(input: &[f64; 8]) -> [f64; 8] {
    let mut out = [0.0f64; 8];
    for (x, slot) in out.iter_mut().enumerate() {
        let sum: f64 = (0..8)
            .map(|u| {
                let cu = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
                cu * input[u] * jpeg_dct_basis(x, u)
            })
            .sum();
        *slot = 0.5 * sum;
    }
    out
}

/// 🌊️ Separable 8×8 transform: applies `pass` to each row then to each column of the resulting rows.
fn jpeg_transform_2d(block: &[f64; 64], pass: fn(&[f64; 8]) -> [f64; 8]) -> [f64; 64] {
    let mut rows = [0.0f64; 64];
    for r in 0..8 {
        let mut line = [0.0f64; 8];
        line.copy_from_slice(&block[r * 8..r * 8 + 8]);
        rows[r * 8..r * 8 + 8].copy_from_slice(&pass(&line));
    }
    let mut out = [0.0f64; 64];
    for c in 0..8 {
        let mut line = [0.0f64; 8];
        for r in 0..8 {
            line[r] = rows[r * 8 + c];
        }
        let transformed = pass(&line);
        for r in 0..8 {
            out[r * 8 + c] = transformed[r];
        }
    }
    out
}

/// 🌊️ Forward 2D DCT of an 8×8, level-shifted spatial block in natural row-major order.
fn jpeg_forward_dct_block(block: &[f64; 64]) -> [f64; 64] {
    jpeg_transform_2d(block, jpeg_dct_1d)
}

/// 🌊️ Inverse 2D DCT back to an 8×8 spatial block in natural row-major order.
fn jpeg_inverse_dct_block(block: &[f64; 64]) -> [f64; 64] {
    jpeg_transform_2d(block, jpeg_idct_1d)
}
// #endregion 🔖️JpegDct

// #region 🔖️JpegDecode
/// 📖️ Bounds-checked byte cursor over a JPEG marker stream; every read fails cleanly on truncation instead of panicking.
struct JpegCursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> JpegCursor<'a> {
    fn read_u8(&mut self) -> Result<u8, ImageError> {
        let &byte = self.data.get(self.pos).ok_or_else(|| ImageError::Decode("truncated jpeg stream".to_string()))?;
        self.pos += 1;
        Ok(byte)
    }

    fn read_u16(&mut self) -> Result<u16, ImageError> {
        let hi = self.read_u8()?;
        let lo = self.read_u8()?;
        Ok(u16::from_be_bytes([hi, lo]))
    }

    fn read_bytes(&mut self, n: usize) -> Result<&'a [u8], ImageError> {
        let end = self.pos.checked_add(n).ok_or_else(|| ImageError::Decode("jpeg segment length overflow".to_string()))?;
        let slice = self.data.get(self.pos..end).ok_or_else(|| ImageError::Decode("truncated jpeg stream".to_string()))?;
        self.pos = end;
        Ok(slice)
    }

    /// ➡️ Byte offset just past a length-prefixed segment whose 2-byte `len` (self-inclusive, just read) starts at the current position; rejects the malformed `len < 2` case instead of underflowing.
    fn segment_end(&self, len: u16) -> Result<usize, ImageError> {
        let payload = len.checked_sub(2).ok_or_else(|| ImageError::Decode("jpeg segment length shorter than its own length field".to_string()))?;
        self.pos.checked_add(usize::from(payload)).ok_or_else(|| ImageError::Decode("jpeg segment length overflow".to_string()))
    }

    /// ➡️ Scans forward to the next marker byte pair, skipping `0xFF` fill bytes (`0xFF00` is only meaningful inside entropy data, never here).
    fn next_marker(&mut self) -> Result<u8, ImageError> {
        loop {
            let mut byte = self.read_u8()?;
            if byte != 0xFF {
                return Err(ImageError::Decode(format!("expected jpeg marker, found {byte:#04x}")));
            }
            loop {
                byte = self.read_u8()?;
                if byte != 0xFF {
                    break;
                }
            }
            if byte != 0x00 {
                return Ok(byte);
            }
        }
    }
}

/// 🖼️ One SOF component: stream id, horizontal/vertical sampling factors, and quantization table id.
#[derive(Clone, Copy)]
struct JpegFrameComponent {
    id: u8,
    h: u8,
    v: u8,
    q_id: u8,
    dc_id: u8,
    ac_id: u8,
}

/// 🖼️ Parsed SOF0/SOF1 frame header.
struct JpegFrameInfo {
    width: u32,
    height: u32,
    components: Vec<JpegFrameComponent>,
}

/// 🖼️ Per-component decoded sample plane at that component's own (possibly subsampled) resolution, padded up to a whole number of MCUs.
struct JpegPlane {
    width: u32,
    h: u8,
    v: u8,
    data: Vec<u8>,
}

/// 📥️ Parses an SOF0/SOF1 segment (length already positioned after the marker) into dimensions and component sampling/quantization ids; rejects non-8-bit precision and component counts other than 1 (grayscale) or 3 (YCbCr).
fn jpeg_parse_sof(cursor: &mut JpegCursor<'_>) -> Result<JpegFrameInfo, ImageError> {
    let len = cursor.read_u16()?;
    if len < 8 {
        return Err(ImageError::Decode("jpeg SOF segment too short".to_string()));
    }
    let precision = cursor.read_u8()?;
    if precision != 8 {
        return Err(ImageError::Decode(format!("unsupported jpeg sample precision {precision}")));
    }
    let height = u32::from(cursor.read_u16()?);
    let width = u32::from(cursor.read_u16()?);
    let n = cursor.read_u8()?;
    if n != 1 && n != 3 {
        return Err(ImageError::UnsupportedJpeg(format!("unsupported component count {n}")));
    }
    let mut components = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let id = cursor.read_u8()?;
        let sampling = cursor.read_u8()?;
        let q_id = cursor.read_u8()?;
        if q_id > 3 {
            return Err(ImageError::Decode("jpeg quantization table id out of range".to_string()));
        }
        let h = sampling >> 4;
        let v = sampling & 0x0F;
        if h == 0 || v == 0 || h > 4 || v > 4 {
            return Err(ImageError::Decode("jpeg sampling factor out of range".to_string()));
        }
        components.push(JpegFrameComponent { id, h, v, q_id, dc_id: 0, ac_id: 0 });
    }
    if width == 0 || height == 0 {
        return Err(ImageError::Decode("jpeg frame has zero width or height".to_string()));
    }
    Ok(JpegFrameInfo { width, height, components })
}

/// 📥️ Parses a DQT segment (length already positioned after the marker) into `[natural-order table; up to 4]`, converting from the wire's zig-zag serialization.
fn jpeg_parse_dqt(cursor: &mut JpegCursor<'_>, tables: &mut [Option<[u16; 64]>; 4]) -> Result<(), ImageError> {
    let len = cursor.read_u16()?;
    let end = cursor.segment_end(len)?;
    while cursor.pos < end {
        let pq_tq = cursor.read_u8()?;
        let precision = pq_tq >> 4;
        let id = usize::from(pq_tq & 0x0F);
        if id > 3 {
            return Err(ImageError::Decode("jpeg quantization table id out of range".to_string()));
        }
        let mut natural = [0u16; 64];
        for &nat in &JPEG_ZIGZAG {
            let value = if precision == 0 { u16::from(cursor.read_u8()?) } else { cursor.read_u16()? };
            natural[nat] = value;
        }
        tables[id] = Some(natural);
    }
    Ok(())
}

/// 📥️ Parses a DHT segment (length already positioned after the marker), which may carry multiple tables back to back, into `dc`/`ac` table slots by id.
fn jpeg_parse_dht(cursor: &mut JpegCursor<'_>, dc: &mut [Option<JpegHuffDecodeTable>; 4], ac: &mut [Option<JpegHuffDecodeTable>; 4]) -> Result<(), ImageError> {
    let len = cursor.read_u16()?;
    let end = cursor.segment_end(len)?;
    while cursor.pos < end {
        let tc_th = cursor.read_u8()?;
        let class = tc_th >> 4;
        let id = usize::from(tc_th & 0x0F);
        if id > 3 {
            return Err(ImageError::Decode("jpeg huffman table id out of range".to_string()));
        }
        let mut counts = [0u8; 16];
        counts.copy_from_slice(cursor.read_bytes(16)?);
        let total = counts.iter().map(|&c| c as usize).sum::<usize>();
        let symbols = cursor.read_bytes(total)?.to_vec();
        let table = JpegHuffDecodeTable::build(&counts, symbols);
        if class == 0 {
            dc[id] = Some(table)
        } else {
            ac[id] = Some(table)
        };
    }
    Ok(())
}

/// 📥️ Decodes one dequantized, natural-order spatial-domain 8×8 block from the entropy stream, updating `dc_pred` in place.
fn jpeg_decode_block(bits: &mut JpegBitReader<'_>, dc_table: &JpegHuffDecodeTable, ac_table: &JpegHuffDecodeTable, quant: &[u16; 64], dc_pred: &mut i32) -> Result<[f64; 64], ImageError> {
    let mut zigzag = [0i32; 64];
    let dc_size = bits.decode_huff(dc_table)?;
    if dc_size > 11 {
        return Err(ImageError::Decode("jpeg dc category out of range".to_string()));
    }
    let diff = if dc_size == 0 { 0 } else { jpeg_extend(bits.receive_bits(dc_size)?, dc_size) };
    *dc_pred += diff;
    zigzag[0] = *dc_pred;
    let mut k = 1usize;
    while k < 64 {
        let rs = bits.decode_huff(ac_table)?;
        let run = rs >> 4;
        let size = rs & 0x0F;
        if size == 0 {
            if run == 15 {
                k += 16;
                continue;
            }
            break;
        }
        k += usize::from(run);
        if k >= 64 || size > 10 {
            return Err(ImageError::Decode("jpeg ac coefficient index out of range".to_string()));
        }
        zigzag[k] = jpeg_extend(bits.receive_bits(size)?, size);
        k += 1;
    }
    let mut block = [0.0f64; 64];
    for (zz, &nat) in JPEG_ZIGZAG.iter().enumerate() {
        block[nat] = f64::from(zigzag[zz] * i32::from(quant[nat]));
    }
    Ok(jpeg_inverse_dct_block(&block))
}

/// 📥️ Decodes the entropy-coded scan into one padded sample plane per frame component, honoring restart markers every `restart_interval` MCUs (`0` disables restarts).
fn jpeg_decode_scan(
    cursor: &mut JpegCursor<'_>,
    frame: &JpegFrameInfo,
    quant_tables: &[Option<[u16; 64]>; 4],
    dc_tables: &[Option<JpegHuffDecodeTable>; 4],
    ac_tables: &[Option<JpegHuffDecodeTable>; 4],
    restart_interval: u16,
) -> Result<Vec<JpegPlane>, ImageError> {
    let max_h = frame.components.iter().map(|c| c.h).max().unwrap_or(1);
    let max_v = frame.components.iter().map(|c| c.v).max().unwrap_or(1);
    let mcu_w = 8 * u32::from(max_h);
    let mcu_h = 8 * u32::from(max_v);
    let mcus_x = frame.width.div_ceil(mcu_w);
    let mcus_y = frame.height.div_ceil(mcu_h);
    let mut planes: Vec<JpegPlane> = frame.components.iter().map(|c| JpegPlane { width: mcus_x * u32::from(c.h) * 8, h: c.h, v: c.v, data: vec![0u8; (mcus_x * u32::from(c.h) * 8 * mcus_y * u32::from(c.v) * 8) as usize] }).collect();
    let mut dc_pred = vec![0i32; frame.components.len()];
    let mut bits = JpegBitReader::new(cursor.data, cursor.pos);
    let mut mcus_since_restart = 0u32;
    for my in 0..mcus_y {
        for mx in 0..mcus_x {
            if restart_interval > 0 && mcus_since_restart == u32::from(restart_interval) {
                bits.consume_restart_marker()?;
                dc_pred.iter_mut().for_each(|p| *p = 0);
                mcus_since_restart = 0;
            }
            for (ci, comp) in frame.components.iter().enumerate() {
                let quant = quant_tables[comp.q_id as usize].as_ref().ok_or_else(|| ImageError::Decode("jpeg scan references undefined quantization table".to_string()))?;
                let dc_table = dc_tables[comp.dc_id as usize].as_ref().ok_or_else(|| ImageError::Decode("jpeg scan references undefined dc huffman table".to_string()))?;
                let ac_table = ac_tables[comp.ac_id as usize].as_ref().ok_or_else(|| ImageError::Decode("jpeg scan references undefined ac huffman table".to_string()))?;
                let plane = &mut planes[ci];
                for dy in 0..u32::from(comp.v) {
                    for dx in 0..u32::from(comp.h) {
                        let spatial = jpeg_decode_block(&mut bits, dc_table, ac_table, quant, &mut dc_pred[ci])?;
                        let base_x = (mx * u32::from(comp.h) + dx) * 8;
                        let base_y = (my * u32::from(comp.v) + dy) * 8;
                        for row in 0..8u32 {
                            for col in 0..8u32 {
                                let sample = (spatial[(row * 8 + col) as usize] + 128.0).round().clamp(0.0, 255.0) as u8;
                                let idx = ((base_y + row) * plane.width + base_x + col) as usize;
                                plane.data[idx] = sample;
                            }
                        }
                    }
                }
            }
            mcus_since_restart += 1;
        }
    }
    cursor.pos = bits.pos;
    Ok(planes)
}

/// 📥️ Decodes a baseline (SOF0/SOF1) JFIF byte stream into RGBA; chroma is nearest/box-upsampled per its sampling factors and combined via BT.601 full-range JFIF coefficients. Progressive (SOF2), lossless, arithmetic-coded and other non-baseline markers return `ImageError::UnsupportedJpeg`.
/// <https://www.w3.org/Graphics/JPEG/itu-t81.pdf>
pub fn decode_jpeg(bytes: &[u8]) -> Result<ImageRgba8, ImageError> {
    let mut cursor = JpegCursor { data: bytes, pos: 0 };
    if cursor.next_marker()? != 0xD8 {
        return Err(ImageError::Decode("jpeg stream does not start with SOI".to_string()));
    }
    let mut quant_tables: [Option<[u16; 64]>; 4] = [None, None, None, None];
    let mut dc_tables: [Option<JpegHuffDecodeTable>; 4] = [None, None, None, None];
    let mut ac_tables: [Option<JpegHuffDecodeTable>; 4] = [None, None, None, None];
    let mut restart_interval = 0u16;
    let mut frame: Option<JpegFrameInfo> = None;
    let mut planes: Option<Vec<JpegPlane>> = None;
    loop {
        let marker = cursor.next_marker()?;
        match marker {
            0xD9 => break,
            0xC0 | 0xC1 => frame = Some(jpeg_parse_sof(&mut cursor)?),
            0xC2 => return Err(ImageError::UnsupportedJpeg("progressive (SOF2) jpeg is not supported".to_string())),
            0xC3 | 0xC5..=0xC7 | 0xC9..=0xCF => return Err(ImageError::UnsupportedJpeg(format!("non-baseline SOF marker {marker:#04x}"))),
            0xC4 => jpeg_parse_dht(&mut cursor, &mut dc_tables, &mut ac_tables)?,
            0xDB => jpeg_parse_dqt(&mut cursor, &mut quant_tables)?,
            0xDD => {
                let len = cursor.read_u16()?;
                if len != 4 {
                    return Err(ImageError::Decode("jpeg DRI segment has unexpected length".to_string()));
                }
                restart_interval = cursor.read_u16()?;
            }
            0xDA => {
                let mut info = frame.take().ok_or_else(|| ImageError::Decode("jpeg SOS segment before SOF".to_string()))?;
                cursor.read_u16()?;
                let n = cursor.read_u8()?;
                if usize::from(n) != info.components.len() {
                    return Err(ImageError::Decode("jpeg SOS component count does not match SOF".to_string()));
                }
                for _ in 0..n {
                    let selector = cursor.read_u8()?;
                    let tables = cursor.read_u8()?;
                    let comp = info.components.iter_mut().find(|c| c.id == selector).ok_or_else(|| ImageError::Decode("jpeg SOS references unknown component".to_string()))?;
                    comp.dc_id = tables >> 4;
                    comp.ac_id = tables & 0x0F;
                }
                cursor.read_bytes(3)?;
                planes = Some(jpeg_decode_scan(&mut cursor, &info, &quant_tables, &dc_tables, &ac_tables, restart_interval)?);
                frame = Some(info);
            }
            0xE0..=0xEF | 0xFE | 0x01 | 0xD0..=0xD7 => {
                if marker != 0x01 && !(0xD0..=0xD7).contains(&marker) {
                    let len = cursor.read_u16()?;
                    let end = cursor.segment_end(len)?;
                    cursor.read_bytes(end - cursor.pos)?;
                }
            }
            _ => {
                let len = cursor.read_u16()?;
                let end = cursor.segment_end(len)?;
                cursor.read_bytes(end - cursor.pos)?;
            }
        }
    }
    let frame = frame.ok_or_else(|| ImageError::Decode("jpeg stream has no frame header".to_string()))?;
    let planes = planes.ok_or_else(|| ImageError::Decode("jpeg stream has no scan data".to_string()))?;
    let max_h = frame.components.iter().map(|c| c.h).max().unwrap_or(1);
    let max_v = frame.components.iter().map(|c| c.v).max().unwrap_or(1);
    let mut out = ImageRgba8::new(frame.width, frame.height);
    let sample_at = |plane: &JpegPlane, x: u32, y: u32| -> u8 {
        let px = x * u32::from(plane.h) / u32::from(max_h);
        let py = y * u32::from(plane.v) / u32::from(max_v);
        plane.data[(py * plane.width + px) as usize]
    };
    for y in 0..frame.height {
        for x in 0..frame.width {
            let idx = ((y * frame.width + x) * 4) as usize;
            if planes.len() == 1 {
                let luma = sample_at(&planes[0], x, y);
                out.data[idx] = luma;
                out.data[idx + 1] = luma;
                out.data[idx + 2] = luma;
            } else {
                let yy = f32::from(sample_at(&planes[0], x, y));
                let cb = f32::from(sample_at(&planes[1], x, y)) - 128.0;
                let cr = f32::from(sample_at(&planes[2], x, y)) - 128.0;
                out.data[idx] = (yy + 1.402 * cr).round().clamp(0.0, 255.0) as u8;
                out.data[idx + 1] = (yy - 0.344_136 * cb - 0.714_136 * cr).round().clamp(0.0, 255.0) as u8;
                out.data[idx + 2] = (yy + 1.772 * cb).round().clamp(0.0, 255.0) as u8;
            }
            out.data[idx + 3] = 255;
        }
    }
    Ok(out)
}
// #endregion 🔖️JpegDecode

// #region 🔖️JpegEncode
/// 🎨️ Edge-clamped RGBA sample, defensive against a mismatched `data` buffer so `encode_jpeg` never panics on a malformed `ImageRgba8`.
fn jpeg_get_px(image: &ImageRgba8, x: u32, y: u32) -> [u8; 4] {
    if image.width == 0 || image.height == 0 {
        return [0, 0, 0, 255];
    }
    let cx = x.min(image.width - 1);
    let cy = y.min(image.height - 1);
    let idx = ((cy * image.width + cx) * 4) as usize;
    match image.data.get(idx..idx + 4) {
        Some(px) => [px[0], px[1], px[2], px[3]],
        None => [0, 0, 0, 255],
    }
}

/// 🎨️ Builds full-resolution (padded to a multiple of 16) `Y`, `Cb`, `Cr` planes via BT.601 full-range JFIF coefficients, then box-downsamples `Cb`/`Cr` by 2× for 4:2:0.
fn jpeg_to_ycbcr_420(image: &ImageRgba8) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let pw = image.width.max(1).next_multiple_of(16);
    let ph = image.height.max(1).next_multiple_of(16);
    let mut y_plane = vec![0.0f32; (pw * ph) as usize];
    let mut cb_full = vec![0.0f32; (pw * ph) as usize];
    let mut cr_full = vec![0.0f32; (pw * ph) as usize];
    for py in 0..ph {
        for px in 0..pw {
            let [r, g, b, _] = jpeg_get_px(image, px, py);
            let (r, g, b) = (f32::from(r), f32::from(g), f32::from(b));
            let idx = (py * pw + px) as usize;
            y_plane[idx] = 0.299 * r + 0.587 * g + 0.114 * b;
            cb_full[idx] = 128.0 - 0.168_736 * r - 0.331_264 * g + 0.5 * b;
            cr_full[idx] = 128.0 + 0.5 * r - 0.418_688 * g - 0.081_312 * b;
        }
    }
    let (cw, ch) = (pw / 2, ph / 2);
    let mut cb = vec![0.0f32; (cw * ch) as usize];
    let mut cr = vec![0.0f32; (cw * ch) as usize];
    for cy in 0..ch {
        for cx in 0..cw {
            let (x0, y0) = (cx * 2, cy * 2);
            let avg = |plane: &[f32]| (plane[(y0 * pw + x0) as usize] + plane[(y0 * pw + x0 + 1) as usize] + plane[((y0 + 1) * pw + x0) as usize] + plane[((y0 + 1) * pw + x0 + 1) as usize]) * 0.25;
            cb[(cy * cw + cx) as usize] = avg(&cb_full);
            cr[(cy * cw + cx) as usize] = avg(&cr_full);
        }
    }
    (y_plane, cb, cr)
}

/// 🎨️ Level-shifted (`- 128`) natural-order 8×8 block read from a plane at `(bx, by)`.
fn jpeg_read_block(plane: &[f32], plane_w: u32, bx: u32, by: u32) -> [f64; 64] {
    let mut block = [0.0f64; 64];
    for row in 0..8u32 {
        for col in 0..8u32 {
            block[(row * 8 + col) as usize] = f64::from(plane[((by + row) * plane_w + bx + col) as usize]) - 128.0;
        }
    }
    block
}

/// 🎨️ Forward-DCTs, quantizes (round-to-nearest) and zig-zag reorders one 8×8 block.
fn jpeg_quantize_block(spatial: &[f64; 64], quant: &[u16; 64]) -> [i32; 64] {
    let transformed = jpeg_forward_dct_block(spatial);
    let mut zigzag = [0i32; 64];
    for (zz, &nat) in JPEG_ZIGZAG.iter().enumerate() {
        zigzag[zz] = (transformed[nat] / f64::from(quant[nat])).round() as i32;
    }
    zigzag
}

/// ✍️ Entropy-encodes one quantized zig-zag block's DC (differential) and AC (run-length) coefficients into `bits`, updating `dc_pred` in place.
fn jpeg_encode_block(bits: &mut JpegBitWriter, zigzag: &[i32; 64], dc_table: &[Option<(u16, u8)>; 256], ac_table: &[Option<(u16, u8)>; 256], dc_pred: &mut i32) {
    let diff = zigzag[0] - *dc_pred;
    *dc_pred = zigzag[0];
    let dc_size = jpeg_category(diff);
    if let Some((code, len)) = dc_table[usize::from(dc_size)] {
        bits.put_bits(code, len);
    }
    if dc_size > 0 {
        bits.put_bits(jpeg_signed_bits(diff, dc_size), dc_size);
    }
    let mut run = 0u8;
    for &value in &zigzag[1..64] {
        if value == 0 {
            run += 1;
            continue;
        }
        while run >= 16 {
            if let Some((code, len)) = ac_table[0xF0] {
                bits.put_bits(code, len);
            }
            run -= 16;
        }
        let size = jpeg_category(value);
        let symbol = (run << 4) | size;
        if let Some((code, len)) = ac_table[usize::from(symbol)] {
            bits.put_bits(code, len);
        }
        bits.put_bits(jpeg_signed_bits(value, size), size);
        run = 0;
    }
    if run > 0 {
        if let Some((code, len)) = ac_table[0x00] {
            bits.put_bits(code, len);
        }
    }
}

/// ✍️ Appends a length-prefixed segment (`marker`, big-endian length covering itself, `payload`) to `out`.
fn jpeg_write_segment(out: &mut Vec<u8>, marker: u8, payload: &[u8]) {
    out.push(0xFF);
    out.push(marker);
    out.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
    out.extend_from_slice(payload);
}

/// ✍️ DQT payload for one 8-bit-precision table, values serialized in zig-zag order.
fn jpeg_dqt_payload(id: u8, table: &[u16; 64]) -> Vec<u8> {
    let mut payload = vec![id & 0x0F];
    payload.extend(JPEG_ZIGZAG.iter().map(|&nat| table[nat] as u8));
    payload
}

/// ✍️ DHT payload for one Annex K.3 table.
fn jpeg_dht_payload(class: u8, id: u8, spec: &JpegHuffSpec) -> Vec<u8> {
    let mut payload = vec![(class << 4) | (id & 0x0F)];
    payload.extend_from_slice(&spec.counts);
    payload.extend_from_slice(spec.symbols);
    payload
}

/// 📤️ Encodes an RGBA image as a baseline (SOF0) JFIF byte stream: 4:2:0 chroma subsampling, the Annex K.3 recommended Huffman tables, and Annex K.1/K.2 quantization tables scaled by `quality` (`1..=100`, clamped). Degenerate `image` buffers (zero-sized or length-mismatched) never panic — out-of-range pixels read as opaque black.
/// <https://www.w3.org/Graphics/JPEG/itu-t81.pdf>
pub fn encode_jpeg(image: &ImageRgba8, quality: u8) -> Vec<u8> {
    let (y_plane, cb_plane, cr_plane) = jpeg_to_ycbcr_420(image);
    let pw = image.width.max(1).next_multiple_of(16);
    let ph = image.height.max(1).next_multiple_of(16);
    let (cw, _ch) = (pw / 2, ph / 2);
    let luma_quant = jpeg_scale_quant_table(&JPEG_LUMA_QUANT_BASE, quality);
    let chroma_quant = jpeg_scale_quant_table(&JPEG_CHROMA_QUANT_BASE, quality);
    let dc_luma = jpeg_build_encode_table(&JPEG_STD_DC_LUMA);
    let ac_luma = jpeg_build_encode_table(&JPEG_STD_AC_LUMA);
    let dc_chroma = jpeg_build_encode_table(&JPEG_STD_DC_CHROMA);
    let ac_chroma = jpeg_build_encode_table(&JPEG_STD_AC_CHROMA);

    let mut out = Vec::new();
    out.extend_from_slice(&[0xFF, 0xD8]);
    jpeg_write_segment(&mut out, 0xE0, b"JFIF\0\x01\x01\x00\x00\x01\x00\x01\x00\x00");
    jpeg_write_segment(&mut out, 0xDB, &jpeg_dqt_payload(0, &luma_quant));
    jpeg_write_segment(&mut out, 0xDB, &jpeg_dqt_payload(1, &chroma_quant));
    let mut sof = vec![8];
    sof.extend_from_slice(&(image.height as u16).to_be_bytes());
    sof.extend_from_slice(&(image.width as u16).to_be_bytes());
    sof.push(3);
    sof.extend_from_slice(&[1, 0x22, 0, 2, 0x11, 1, 3, 0x11, 1]);
    jpeg_write_segment(&mut out, 0xC0, &sof);
    jpeg_write_segment(&mut out, 0xC4, &jpeg_dht_payload(0, 0, &JPEG_STD_DC_LUMA));
    jpeg_write_segment(&mut out, 0xC4, &jpeg_dht_payload(1, 0, &JPEG_STD_AC_LUMA));
    jpeg_write_segment(&mut out, 0xC4, &jpeg_dht_payload(0, 1, &JPEG_STD_DC_CHROMA));
    jpeg_write_segment(&mut out, 0xC4, &jpeg_dht_payload(1, 1, &JPEG_STD_AC_CHROMA));
    jpeg_write_segment(&mut out, 0xDA, &[3, 1, 0x00, 2, 0x11, 3, 0x11, 0, 63, 0]);

    let mut bits = JpegBitWriter::default();
    let (mut dc_y, mut dc_cb, mut dc_cr) = (0i32, 0i32, 0i32);
    let mcus_x = pw / 16;
    let mcus_y = ph / 16;
    for my in 0..mcus_y {
        for mx in 0..mcus_x {
            for dy in 0..2u32 {
                for dx in 0..2u32 {
                    let block = jpeg_read_block(&y_plane, pw, mx * 16 + dx * 8, my * 16 + dy * 8);
                    let zz = jpeg_quantize_block(&block, &luma_quant);
                    jpeg_encode_block(&mut bits, &zz, &dc_luma, &ac_luma, &mut dc_y);
                }
            }
            let cb_block = jpeg_read_block(&cb_plane, cw, mx * 8, my * 8);
            let cb_zz = jpeg_quantize_block(&cb_block, &chroma_quant);
            jpeg_encode_block(&mut bits, &cb_zz, &dc_chroma, &ac_chroma, &mut dc_cb);
            let cr_block = jpeg_read_block(&cr_plane, cw, mx * 8, my * 8);
            let cr_zz = jpeg_quantize_block(&cr_block, &chroma_quant);
            jpeg_encode_block(&mut bits, &cr_zz, &dc_chroma, &ac_chroma, &mut dc_cr);
        }
    }
    bits.flush();
    out.extend_from_slice(&bits.bytes);
    out.extend_from_slice(&[0xFF, 0xD9]);
    out
}
// #endregion 🔖️JpegEncode
// #endregion 🔖️Codec

// #region 🔖️Filter
fn mirror_index(i: i64, n: i64) -> i64 {
    let mut i = i;
    loop {
        if i < 0 {
            i = -1 - i;
        } else if i >= n {
            i = 2 * n - 1 - i;
        } else {
            return i;
        }
    }
}

/// 🌀️ Separable Gaussian blur with kernel radius `ceil(3 sigma)` and half-sample mirror padding (which preserves the image mean exactly for symmetric kernels); `sigma <= 0` returns the input unchanged.
/// <https://en.wikipedia.org/wiki/Gaussian_blur>
pub fn gaussian_blur(img: &ImageGray, sigma: f32) -> ImageGray {
    if sigma <= 0.0 || img.width == 0 || img.height == 0 {
        return img.clone();
    }
    let radius = (3.0 * sigma).ceil() as i64;
    let mut kernel = Vec::with_capacity((2 * radius + 1) as usize);
    let inv_two_sigma_sq = -0.5 / (sigma * sigma);
    for k in -radius..=radius {
        kernel.push(((k * k) as f32 * inv_two_sigma_sq).exp());
    }
    let total: f32 = kernel.iter().sum();
    for weight in kernel.iter_mut() {
        *weight /= total;
    }
    let (w, h) = (img.width as i64, img.height as i64);
    let mut horizontal = ImageGray::new(img.width, img.height);
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0;
            for (k, weight) in kernel.iter().enumerate() {
                let sx = mirror_index(x + k as i64 - radius, w);
                acc += weight * img.data[(y * w + sx) as usize];
            }
            horizontal.data[(y * w + x) as usize] = acc;
        }
    }
    let mut out = ImageGray::new(img.width, img.height);
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0;
            for (k, weight) in kernel.iter().enumerate() {
                let sy = mirror_index(y + k as i64 - radius, h);
                acc += weight * horizontal.data[(sy * w + x) as usize];
            }
            out.data[(y * w + x) as usize] = acc;
        }
    }
    out
}

/// 📦️ Box blur over a `(2 radius + 1)²` window via an integral image; the window is clamped at the borders and the average divides by the number of covered pixels.
pub fn box_blur_integral(img: &ImageGray, radius: u32) -> ImageGray {
    if img.width == 0 || img.height == 0 {
        return img.clone();
    }
    let integral = IntegralImage::build(img);
    let mut out = ImageGray::new(img.width, img.height);
    for y in 0..img.height {
        let y0 = y.saturating_sub(radius);
        let y1 = (y + radius + 1).min(img.height);
        for x in 0..img.width {
            let x0 = x.saturating_sub(radius);
            let x1 = (x + radius + 1).min(img.width);
            let count = f64::from((x1 - x0) * (y1 - y0));
            out.set(x, y, (integral.rect_sum(x0, y0, x1, y1) / count) as f32);
        }
    }
    out
}

/// ➕️ Summed-area table over pixel values and squared values, with a zero top row and left column: `sum`/`sq_sum` have `(width + 1) * (height + 1)` entries and entry `(x, y)` holds the sum over pixels `[0, x) × [0, y)`.
/// <https://en.wikipedia.org/wiki/Summed-area_table>
#[derive(Clone, Debug, PartialEq)]
pub struct IntegralImage {
    pub width: u32,
    pub height: u32,
    pub sum: Vec<f64>,
    pub sq_sum: Vec<f64>,
}

impl IntegralImage {
    /// ➕️ Builds the summed-area tables in one row-prefix pass.
    pub fn build(img: &ImageGray) -> Self {
        let w = img.width as usize;
        let h = img.height as usize;
        let stride = w + 1;
        let mut sum = vec![0.0f64; stride * (h + 1)];
        let mut sq_sum = vec![0.0f64; stride * (h + 1)];
        for y in 0..h {
            let mut row = 0.0f64;
            let mut row_sq = 0.0f64;
            for x in 0..w {
                let v = f64::from(img.data[y * w + x]);
                row += v;
                row_sq += v * v;
                sum[(y + 1) * stride + x + 1] = sum[y * stride + x + 1] + row;
                sq_sum[(y + 1) * stride + x + 1] = sq_sum[y * stride + x + 1] + row_sq;
            }
        }
        Self { width: img.width, height: img.height, sum, sq_sum }
    }

    /// ➕️ Sum of pixel values over the inclusive-exclusive rectangle `x in [x0, x1), y in [y0, y1)`.
    pub fn rect_sum(&self, x0: u32, y0: u32, x1: u32, y1: u32) -> f64 {
        let stride = self.width as usize + 1;
        let (x0, y0, x1, y1) = (x0 as usize, y0 as usize, x1 as usize, y1 as usize);
        self.sum[y1 * stride + x1] - self.sum[y0 * stride + x1] - self.sum[y1 * stride + x0] + self.sum[y0 * stride + x0]
    }

    /// ➕️ Sum of squared pixel values over the inclusive-exclusive rectangle `x in [x0, x1), y in [y0, y1)`.
    pub fn rect_sq_sum(&self, x0: u32, y0: u32, x1: u32, y1: u32) -> f64 {
        let stride = self.width as usize + 1;
        let (x0, y0, x1, y1) = (x0 as usize, y0 as usize, x1 as usize, y1 as usize);
        self.sq_sum[y1 * stride + x1] - self.sq_sum[y0 * stride + x1] - self.sq_sum[y1 * stride + x0] + self.sq_sum[y0 * stride + x0]
    }
}

/// 🔻️ Half-resolution downsample by 2×2 averaging; odd dimensions round up (`div_ceil`) and edge blocks clamp to the last row/column.
pub fn downsample_half(img: &ImageGray) -> ImageGray {
    if img.width == 0 || img.height == 0 {
        return img.clone();
    }
    let out_w = img.width.div_ceil(2);
    let out_h = img.height.div_ceil(2);
    let mut out = ImageGray::new(out_w, out_h);
    for y in 0..out_h {
        let y0 = 2 * y;
        let y1 = (2 * y + 1).min(img.height - 1);
        for x in 0..out_w {
            let x0 = 2 * x;
            let x1 = (2 * x + 1).min(img.width - 1);
            let avg = 0.25 * (img.get(x0, y0) + img.get(x1, y0) + img.get(x0, y1) + img.get(x1, y1));
            out.set(x, y, avg);
        }
    }
    out
}
// #endregion 🔖️Filter

// #region 🔖️Gradient
/// 🧭️ Per-pixel image gradients, row-major with `gx`/`gy` of length `width * height`.
#[derive(Clone, Debug, PartialEq)]
pub struct GradientField {
    pub gx: Vec<f32>,
    pub gy: Vec<f32>,
    pub width: u32,
    pub height: u32,
}

/// 🧭️ 3×3 Scharr gradients with mirror padding, normalized by 1/32 so a unit-slope ramp yields a gradient of exactly 1 per pixel.
/// <https://en.wikipedia.org/wiki/Sobel_operator#Alternative_operators>
pub fn scharr_gradients(img: &ImageGray) -> GradientField {
    let (w, h) = (img.width as i64, img.height as i64);
    let mut gx = vec![0.0f32; (w * h) as usize];
    let mut gy = vec![0.0f32; (w * h) as usize];
    for y in 0..h {
        for x in 0..w {
            let p = |dx: i64, dy: i64| img.data[(mirror_index(y + dy, h) * w + mirror_index(x + dx, w)) as usize];
            let idx = (y * w + x) as usize;
            gx[idx] = (3.0 * (p(1, -1) - p(-1, -1)) + 10.0 * (p(1, 0) - p(-1, 0)) + 3.0 * (p(1, 1) - p(-1, 1))) / 32.0;
            gy[idx] = (3.0 * (p(-1, 1) - p(-1, -1)) + 10.0 * (p(0, 1) - p(0, -1)) + 3.0 * (p(1, 1) - p(1, -1))) / 32.0;
        }
    }
    GradientField { gx, gy, width: img.width, height: img.height }
}

/// 🧭️ Per-pixel gradient magnitude `√(gx² + gy²)` and orientation `atan2(gy, gx)` in radians, both row-major.
pub fn gradient_magnitude_orientation(g: &GradientField) -> (Vec<f32>, Vec<f32>) {
    let mut magnitude = Vec::with_capacity(g.gx.len());
    let mut orientation = Vec::with_capacity(g.gx.len());
    for (&gx, &gy) in g.gx.iter().zip(g.gy.iter()) {
        magnitude.push(gx.hypot(gy));
        orientation.push(gy.atan2(gx));
    }
    (magnitude, orientation)
}
// #endregion 🔖️Gradient

// #region 🔖️Pyramid
/// 🗻️ Coarse-to-fine image pyramid; `levels[0]` is the original and each next level halves the resolution (`scale = 0.5`).
#[derive(Clone, Debug, PartialEq)]
pub struct Pyramid {
    pub levels: Vec<ImageGray>,
    pub scale: f32,
}

/// 🗻️ Builds up to `n_levels` pyramid levels: each level is the previous one Gaussian-blurred with `sigma = 1` then 2×2-downsampled; stops early once a level reaches 1×1.
/// <https://en.wikipedia.org/wiki/Pyramid_(image_processing)>
pub fn build_pyramid(img: &ImageGray, n_levels: usize) -> Pyramid {
    let mut levels = Vec::with_capacity(n_levels);
    if n_levels == 0 {
        return Pyramid { levels, scale: 0.5 };
    }
    levels.push(img.clone());
    while levels.len() < n_levels {
        let prev = levels.last().expect("levels is non-empty after the initial push");
        if prev.width <= 1 && prev.height <= 1 {
            break;
        }
        let next = downsample_half(&gaussian_blur(prev, 1.0));
        levels.push(next);
    }
    Pyramid { levels, scale: 0.5 }
}
// #endregion 🔖️Pyramid

// #region 🔖️Patch
/// 🧩️ Square intensity patch of side `2 radius + 1`, row-major in `data`.
#[derive(Clone, Debug, PartialEq)]
pub struct Patch {
    pub radius: u32,
    pub data: Vec<f32>,
}

/// 🧩️ Bilinearly samples a square patch centered at `(cx, cy)`, with the sampling grid rotated by `rotation` radians (counter-clockwise offset `(dx, dy)` maps to `(cx + dx cos − dy sin, cy + dx sin + dy cos)`).
pub fn extract_patch(img: &ImageGray, cx: f32, cy: f32, radius: u32, rotation: f32) -> Patch {
    let side = (2 * radius + 1) as usize;
    let (sin, cos) = rotation.sin_cos();
    let r = radius as i64;
    let mut data = Vec::with_capacity(side * side);
    for dy in -r..=r {
        for dx in -r..=r {
            let fx = dx as f32;
            let fy = dy as f32;
            data.push(img.sample(cx + fx * cos - fy * sin, cy + fx * sin + fy * cos));
        }
    }
    Patch { radius, data }
}

/// 🎯️ Zero-mean normalized cross correlation in `[-1, 1]`; returns `0` for mismatched sizes or degenerate (near-zero) variance.
/// <https://en.wikipedia.org/wiki/Cross-correlation#Zero-normalized_cross-correlation_(ZNCC)>
pub fn zncc(a: &Patch, b: &Patch) -> f32 {
    if a.data.len() != b.data.len() || a.data.is_empty() {
        return 0.0;
    }
    let n = a.data.len() as f32;
    let mean_a = a.data.iter().sum::<f32>() / n;
    let mean_b = b.data.iter().sum::<f32>() / n;
    let mut numerator = 0.0f32;
    let mut var_a = 0.0f32;
    let mut var_b = 0.0f32;
    for (&pa, &pb) in a.data.iter().zip(b.data.iter()) {
        let da = pa - mean_a;
        let db = pb - mean_b;
        numerator += da * db;
        var_a += da * da;
        var_b += db * db;
    }
    let denominator = (var_a * var_b).sqrt();
    if denominator < 1e-12 {
        return 0.0;
    }
    numerator / denominator
}

/// 📏️ Sum of squared differences between two patches (mismatched sizes compare only the overlapping prefix).
pub fn ssd(a: &Patch, b: &Patch) -> f32 {
    a.data.iter().zip(b.data.iter()).map(|(&pa, &pb)| (pa - pb) * (pa - pb)).sum()
}

/// 🧮️ Census transform per pixel: bit `(dy + r)(2r + 1) + (dx + r)` is set when the neighbor at offset `(dx, dy)` is strictly darker than the center. `radius` is clamped to 3 (7×7 window, 49 bits) to fit `u64`; pixels whose window leaves the image get code `0`.
/// <https://en.wikipedia.org/wiki/Census_transform>
pub fn census_transform(img: &ImageGray, radius: u32) -> Vec<u64> {
    let r = radius.min(3) as i64;
    let (w, h) = (img.width as i64, img.height as i64);
    let side = 2 * r + 1;
    let mut out = vec![0u64; (w * h) as usize];
    for y in r..(h - r) {
        for x in r..(w - r) {
            let center = img.data[(y * w + x) as usize];
            let mut code = 0u64;
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if img.data[((y + dy) * w + x + dx) as usize] < center {
                        code |= 1 << ((dy + r) * side + (dx + r));
                    }
                }
            }
            out[(y * w + x) as usize] = code;
        }
    }
    out
}

/// 🧮️ Hamming distance between two census codes.
pub fn hamming_cost_census(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}
// #endregion 🔖️Patch

// #region 🔖️Warp
/// 🔀️ Affine warp by inverse mapping: `m` maps OUTPUT pixel coordinates to SOURCE coordinates (`sx = m[0]·[x, y, 1]`, `sy = m[1]·[x, y, 1]`), sampled bilinearly with clamped borders.
pub fn warp_affine(img: &ImageGray, m: &[[f32; 3]; 2], out_w: u32, out_h: u32) -> ImageGray {
    let mut out = ImageGray::new(out_w, out_h);
    for y in 0..out_h {
        for x in 0..out_w {
            let fx = x as f32;
            let fy = y as f32;
            let sx = m[0][0] * fx + m[0][1] * fy + m[0][2];
            let sy = m[1][0] * fx + m[1][1] * fy + m[1][2];
            out.set(x, y, img.sample(sx, sy));
        }
    }
    out
}

/// 🔀️ Homography warp by inverse mapping with perspective divide: `h` maps OUTPUT pixel coordinates to SOURCE coordinates in homogeneous form; outputs with a near-zero denominator are set to `0`.
/// <https://en.wikipedia.org/wiki/Homography_(computer_vision)>
pub fn warp_homography(img: &ImageGray, h: &[[f64; 3]; 3], out_w: u32, out_h: u32) -> ImageGray {
    let mut out = ImageGray::new(out_w, out_h);
    for y in 0..out_h {
        for x in 0..out_w {
            let fx = f64::from(x);
            let fy = f64::from(y);
            let denom = h[2][0] * fx + h[2][1] * fy + h[2][2];
            if denom.abs() < 1e-12 {
                continue;
            }
            let sx = (h[0][0] * fx + h[0][1] * fy + h[0][2]) / denom;
            let sy = (h[1][0] * fx + h[1][1] * fy + h[1][2]) / denom;
            out.set(x, y, img.sample(sx as f32, sy as f32));
        }
    }
    out
}

/// 🔀️ Generic remap: output pixel `i` (row-major over `out_w × out_h`) samples the source bilinearly at `(map_x[i], map_y[i])`.
pub fn remap(img: &ImageGray, map_x: &[f32], map_y: &[f32], out_w: u32, out_h: u32) -> ImageGray {
    let mut out = ImageGray::new(out_w, out_h);
    for ((dst, &mx), &my) in out.data.iter_mut().zip(map_x.iter()).zip(map_y.iter()) {
        *dst = img.sample(mx, my);
    }
    out
}
// #endregion 🔖️Warp

// #region 🔖️Nms
/// 📌️ Strict local maxima of a row-major score map: keeps pixels strictly greater than every other score in their `(2 radius + 1)²` window (clamped at borders) and strictly above `threshold`, returned as `(x, y, score)` sorted by descending score, then row-major position, for determinism.
pub fn non_max_suppression(scores: &[f32], width: u32, height: u32, radius: u32, threshold: f32) -> Vec<(u32, u32, f32)> {
    use std::cmp::Ordering;
    let (w, h) = (width as i64, height as i64);
    let r = radius as i64;
    let mut peaks = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let score = scores[(y * w + x) as usize];
            if score.partial_cmp(&threshold) != Some(Ordering::Greater) {
                continue;
            }
            let mut is_max = true;
            'window: for ny in (y - r).max(0)..=(y + r).min(h - 1) {
                for nx in (x - r).max(0)..=(x + r).min(w - 1) {
                    if nx == x && ny == y {
                        continue;
                    }
                    if scores[(ny * w + nx) as usize] >= score {
                        is_max = false;
                        break 'window;
                    }
                }
            }
            if is_max {
                peaks.push((x as u32, y as u32, score));
            }
        }
    }
    peaks.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(Ordering::Equal).then_with(|| (a.1, a.0).cmp(&(b.1, b.0))));
    peaks
}
// #endregion 🔖️Nms

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_next(state: &mut u32) -> f32 {
        *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (*state >> 8) as f32 / 16_777_216.0
    }

    fn lcg_image(width: u32, height: u32, seed: u32) -> ImageGray {
        let mut img = ImageGray::new(width, height);
        let mut state = seed;
        for value in img.data.iter_mut() {
            *value = lcg_next(&mut state);
        }
        img
    }

    fn smooth_image(width: u32, height: u32) -> ImageGray {
        let mut img = ImageGray::new(width, height);
        for y in 0..height {
            for x in 0..width {
                img.set(x, y, 0.5 + 0.25 * (x as f32 * 0.23).sin() + 0.2 * (y as f32 * 0.17).cos());
            }
        }
        img
    }

    fn mean_and_variance(img: &ImageGray) -> (f64, f64) {
        let n = img.data.len() as f64;
        let mean = img.data.iter().map(|&v| f64::from(v)).sum::<f64>() / n;
        let variance = img.data.iter().map(|&v| (f64::from(v) - mean).powi(2)).sum::<f64>() / n;
        (mean, variance)
    }

    fn invert3(h: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let det = h[0][0] * (h[1][1] * h[2][2] - h[1][2] * h[2][1]) - h[0][1] * (h[1][0] * h[2][2] - h[1][2] * h[2][0]) + h[0][2] * (h[1][0] * h[2][1] - h[1][1] * h[2][0]);
        let inv_det = 1.0 / det;
        [
            [(h[1][1] * h[2][2] - h[1][2] * h[2][1]) * inv_det, (h[0][2] * h[2][1] - h[0][1] * h[2][2]) * inv_det, (h[0][1] * h[1][2] - h[0][2] * h[1][1]) * inv_det],
            [(h[1][2] * h[2][0] - h[1][0] * h[2][2]) * inv_det, (h[0][0] * h[2][2] - h[0][2] * h[2][0]) * inv_det, (h[0][2] * h[1][0] - h[0][0] * h[1][2]) * inv_det],
            [(h[1][0] * h[2][1] - h[1][1] * h[2][0]) * inv_det, (h[0][1] * h[2][0] - h[0][0] * h[2][1]) * inv_det, (h[0][0] * h[1][1] - h[0][1] * h[1][0]) * inv_det],
        ]
    }

    #[test]
    fn luma_conversion_matches_bt601_weights() {
        let mut rgba = ImageRgba8::new(3, 1);
        rgba.data.copy_from_slice(&[255, 0, 0, 255, 255, 255, 255, 255, 10, 20, 30, 255]);
        let gray = ImageGray::from_rgba8_luma(&rgba);
        assert!((gray.get(0, 0) - 0.299).abs() < 1e-4);
        assert!((gray.get(1, 0) - 1.0).abs() < 1e-4);
        let expected = (0.299 * 10.0 + 0.587 * 20.0 + 0.114 * 30.0) / 255.0;
        assert!((gray.get(2, 0) - expected).abs() < 1e-6);
    }

    #[test]
    fn gray_sample_bilinear_interpolates_between_pixels() {
        let mut img = ImageGray::new(2, 2);
        img.data.copy_from_slice(&[0.0, 1.0, 0.0, 1.0]);
        assert!((img.sample(0.5, 0.5) - 0.5).abs() < 1e-6);
        assert!((img.sample(-5.0, -5.0) - 0.0).abs() < 1e-6);
        assert!((img.sample(10.0, 10.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rgba_sample_rgb_normalizes_and_interpolates() {
        let mut img = ImageRgba8::new(2, 1);
        img.data.copy_from_slice(&[0, 0, 0, 255, 255, 102, 0, 255]);
        let mid = img.sample_rgb(0.5, 0.0);
        assert!((mid[0] - 0.5).abs() < 1e-4);
        assert!((mid[1] - 0.2).abs() < 1e-4);
        assert!(mid[2].abs() < 1e-6);
    }

    #[test]
    fn png_rgba_round_trip_is_lossless() {
        let mut img = ImageRgba8::new(4, 3);
        for (i, value) in img.data.iter_mut().enumerate() {
            *value = (i * 17 % 251) as u8;
        }
        let bytes = encode_png(&img).expect("encode succeeds");
        let decoded = decode_png(&bytes).expect("decode succeeds");
        assert_eq!(decoded, img);
    }

    #[test]
    fn png_gray16_encode_decodes_back_losslessly() {
        let data: Vec<u16> = vec![0, 1, 500, 40_000, 65_535, 12_345];
        let bytes = encode_png_gray16(&data, 3, 2).expect("encode succeeds");
        let mut decoder = png::Decoder::new(bytes.as_slice());
        decoder.set_transformations(png::Transformations::IDENTITY);
        let mut reader = decoder.read_info().expect("readable png");
        let mut buf = vec![0u8; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).expect("decodable frame");
        assert_eq!(info.bit_depth, png::BitDepth::Sixteen);
        assert_eq!(info.color_type, png::ColorType::Grayscale);
        let decoded: Vec<u16> = buf[..info.buffer_size()].as_chunks::<2>().0.iter().map(|&pair| u16::from_be_bytes(pair)).collect();
        assert_eq!(decoded, data);
    }

    #[test]
    fn png_codec_rejects_dimension_mismatch() {
        let img = ImageRgba8 { width: 2, height: 2, data: vec![0; 3] };
        assert_eq!(encode_png(&img), Err(ImageError::Dimensions));
        assert_eq!(encode_png_gray16(&[1, 2, 3], 2, 2), Err(ImageError::Dimensions));
        assert!(matches!(decode_png(&[1, 2, 3]), Err(ImageError::Decode(_))));
    }

    #[test]
    fn gaussian_blur_preserves_mean_and_reduces_variance() {
        let img = lcg_image(32, 32, 7);
        let blurred = gaussian_blur(&img, 1.5);
        let (mean_before, var_before) = mean_and_variance(&img);
        let (mean_after, var_after) = mean_and_variance(&blurred);
        assert!((mean_before - mean_after).abs() < 1e-4);
        assert!(var_after < var_before);
    }

    #[test]
    fn box_blur_integral_matches_brute_force_average() {
        let img = lcg_image(9, 7, 21);
        let blurred = box_blur_integral(&img, 2);
        for y in 0..7u32 {
            for x in 0..9u32 {
                let (x0, x1) = (x.saturating_sub(2), (x + 3).min(9));
                let (y0, y1) = (y.saturating_sub(2), (y + 3).min(7));
                let mut total = 0.0f64;
                for sy in y0..y1 {
                    for sx in x0..x1 {
                        total += f64::from(img.get(sx, sy));
                    }
                }
                let expected = total / f64::from((x1 - x0) * (y1 - y0));
                assert!((f64::from(blurred.get(x, y)) - expected).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn integral_image_rect_sums_match_brute_force() {
        let img = lcg_image(16, 16, 42);
        let integral = IntegralImage::build(&img);
        for &(x0, y0, x1, y1) in &[(0u32, 0u32, 16u32, 16u32), (2, 3, 10, 11), (5, 5, 6, 6), (0, 7, 16, 8), (4, 0, 5, 16)] {
            let mut brute_sum = 0.0f64;
            let mut brute_sq = 0.0f64;
            for y in y0..y1 {
                for x in x0..x1 {
                    let v = f64::from(img.get(x, y));
                    brute_sum += v;
                    brute_sq += v * v;
                }
            }
            assert!((integral.rect_sum(x0, y0, x1, y1) - brute_sum).abs() < 1e-8);
            assert!((integral.rect_sq_sum(x0, y0, x1, y1) - brute_sq).abs() < 1e-8);
        }
    }

    #[test]
    fn downsample_half_averages_two_by_two_blocks() {
        let mut img = ImageGray::new(4, 2);
        img.data.copy_from_slice(&[0.0, 1.0, 0.2, 0.4, 1.0, 0.0, 0.6, 0.8]);
        let half = downsample_half(&img);
        assert_eq!((half.width, half.height), (2, 1));
        assert!((half.get(0, 0) - 0.5).abs() < 1e-6);
        assert!((half.get(1, 0) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn scharr_gradients_recover_ramp_slopes() {
        let mut img = ImageGray::new(8, 8);
        for y in 0..8 {
            for x in 0..8 {
                img.set(x, y, 0.03 * x as f32 + 0.07 * y as f32);
            }
        }
        let g = scharr_gradients(&img);
        let idx = (3 * 8 + 3) as usize;
        assert!((g.gx[idx] - 0.03).abs() < 1e-5);
        assert!((g.gy[idx] - 0.07).abs() < 1e-5);
        let (magnitude, orientation) = gradient_magnitude_orientation(&g);
        assert!((magnitude[idx] - (0.03f32 * 0.03 + 0.07 * 0.07).sqrt()).abs() < 1e-5);
        assert!((orientation[idx] - 0.07f32.atan2(0.03)).abs() < 1e-4);
    }

    #[test]
    fn build_pyramid_halves_resolution_per_level() {
        let img = lcg_image(64, 48, 3);
        let pyramid = build_pyramid(&img, 4);
        assert_eq!(pyramid.levels.len(), 4);
        assert_eq!(pyramid.scale, 0.5);
        assert_eq!((pyramid.levels[0].width, pyramid.levels[0].height), (64, 48));
        assert_eq!((pyramid.levels[1].width, pyramid.levels[1].height), (32, 24));
        assert_eq!((pyramid.levels[3].width, pyramid.levels[3].height), (8, 6));
        assert_eq!(pyramid.levels[0], img);
    }

    #[test]
    fn zncc_is_invariant_to_gain_and_bias_and_flips_sign() {
        let img = lcg_image(16, 16, 99);
        let a = extract_patch(&img, 8.0, 8.0, 3, 0.0);
        let gained = Patch { radius: 3, data: a.data.iter().map(|&v| v * 1.7 + 0.2).collect() };
        let negated = Patch { radius: 3, data: a.data.iter().map(|&v| -v).collect() };
        assert!((zncc(&a, &gained) - 1.0).abs() < 1e-4);
        assert!((zncc(&a, &negated) + 1.0).abs() < 1e-4);
        let flat = Patch { radius: 3, data: vec![0.5; a.data.len()] };
        assert_eq!(zncc(&a, &flat), 0.0);
        assert_eq!(ssd(&a, &a), 0.0);
        assert!(ssd(&a, &gained) > 0.0);
    }

    #[test]
    fn extract_patch_quarter_turn_matches_transpose_relation() {
        let img = smooth_image(32, 32);
        let radius = 3u32;
        let side = (2 * radius + 1) as usize;
        let straight = extract_patch(&img, 15.0, 14.0, radius, 0.0);
        let rotated = extract_patch(&img, 15.0, 14.0, radius, std::f32::consts::FRAC_PI_2);
        for row in 0..side {
            for col in 0..side {
                let expected = straight.data[col * side + (side - 1 - row)];
                assert!((rotated.data[row * side + col] - expected).abs() < 1e-3);
            }
        }
    }

    #[test]
    fn census_transform_and_hamming_behave_on_known_pattern() {
        let mut img = ImageGray::new(5, 5);
        for (i, value) in img.data.iter_mut().enumerate() {
            *value = i as f32 / 25.0;
        }
        let codes = census_transform(&img, 1);
        assert_eq!(codes[2 * 5 + 2], 0b1111);
        assert_eq!(codes[0], 0);
        assert_eq!(hamming_cost_census(0b1011, 0b0010), 2);
        assert_eq!(hamming_cost_census(u64::MAX, 0), 64);
    }

    #[test]
    fn warp_affine_identity_returns_original() {
        let img = lcg_image(16, 16, 5);
        let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let warped = warp_affine(&img, &identity, 16, 16);
        for (&out, &original) in warped.data.iter().zip(img.data.iter()) {
            assert!((out - original).abs() < 1e-6);
        }
    }

    #[test]
    fn warp_homography_inverse_round_trips_away_from_borders() {
        let img = smooth_image(48, 48);
        let h = [[1.0, 0.03, 1.5], [0.02, 1.0, -1.0], [2e-4, 1e-4, 1.0]];
        let h_inv = invert3(&h);
        let forward = warp_homography(&img, &h, 48, 48);
        let round = warp_homography(&forward, &h_inv, 48, 48);
        let margin = 8u32;
        let mut total = 0.0f64;
        let mut count = 0u32;
        for y in margin..(48 - margin) {
            for x in margin..(48 - margin) {
                total += f64::from((round.get(x, y) - img.get(x, y)).abs());
                count += 1;
            }
        }
        assert!(total / f64::from(count) < 2e-2);
    }

    #[test]
    fn remap_identity_returns_original() {
        let img = lcg_image(12, 10, 11);
        let mut map_x = vec![0.0f32; 120];
        let mut map_y = vec![0.0f32; 120];
        for y in 0..10u32 {
            for x in 0..12u32 {
                map_x[(y * 12 + x) as usize] = x as f32;
                map_y[(y * 12 + x) as usize] = y as f32;
            }
        }
        let out = remap(&img, &map_x, &map_y, 12, 10);
        assert_eq!(out, img);
    }

    #[test]
    fn non_max_suppression_keeps_planted_peaks_sorted() {
        let mut scores = vec![0.0f32; 81];
        scores[3 * 9 + 4] = 1.0;
        scores[3 * 9 + 5] = 0.9;
        scores[9 + 1] = 0.5;
        let peaks = non_max_suppression(&scores, 9, 9, 2, 0.1);
        assert_eq!(peaks, vec![(4, 3, 1.0), (1, 1, 0.5)]);
        assert!(non_max_suppression(&scores, 9, 9, 2, 2.0).is_empty());
    }

    fn gradient_rgba8(width: u32, height: u32) -> ImageRgba8 {
        let mut img = ImageRgba8::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                img.data[idx] = (x * 255 / width.max(1)) as u8;
                img.data[idx + 1] = (y * 255 / height.max(1)) as u8;
                img.data[idx + 2] = (((x + y) * 255) / (width + height).max(1)) as u8;
                img.data[idx + 3] = 255;
            }
        }
        img
    }

    fn checkerboard_rgba8(width: u32, height: u32, cell: u32) -> ImageRgba8 {
        let mut img = ImageRgba8::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let on = ((x / cell.max(1)) + (y / cell.max(1))) % 2 == 0;
                let v = if on { 235 } else { 20 };
                img.data[idx] = v;
                img.data[idx + 1] = v;
                img.data[idx + 2] = v;
                img.data[idx + 3] = 255;
            }
        }
        img
    }

    fn flat_rgba8(width: u32, height: u32, rgb: [u8; 3]) -> ImageRgba8 {
        let mut img = ImageRgba8::new(width, height);
        for px in img.data.as_chunks_mut::<4>().0.iter_mut() {
            *px = [rgb[0], rgb[1], rgb[2], 255];
        }
        img
    }

    fn psnr_rgba8(a: &ImageRgba8, b: &ImageRgba8) -> f64 {
        let mse = a.data.iter().zip(b.data.iter()).map(|(&x, &y)| (f64::from(x) - f64::from(y)).powi(2)).sum::<f64>() / a.data.len() as f64;
        if mse <= 0.0 {
            f64::INFINITY
        } else {
            20.0 * 255.0f64.log10() - 10.0 * mse.log10()
        }
    }

    #[test]
    fn jpeg_round_trip_gradient_meets_psnr_floor() {
        let img = gradient_rgba8(64, 48);
        let bytes = encode_jpeg(&img, 90);
        let decoded = decode_jpeg(&bytes).expect("decode succeeds");
        assert_eq!((decoded.width, decoded.height), (img.width, img.height));
        assert!(psnr_rgba8(&img, &decoded) > 35.0);
    }

    #[test]
    fn jpeg_round_trip_checkerboard_meets_psnr_floor() {
        let img = checkerboard_rgba8(64, 32, 6);
        let bytes = encode_jpeg(&img, 90);
        let decoded = decode_jpeg(&bytes).expect("decode succeeds");
        assert_eq!((decoded.width, decoded.height), (img.width, img.height));
        assert!(psnr_rgba8(&img, &decoded) > 35.0);
    }

    #[test]
    fn jpeg_flat_color_round_trips_with_near_zero_error() {
        let img = flat_rgba8(32, 32, [200, 100, 50]);
        let bytes = encode_jpeg(&img, 90);
        let decoded = decode_jpeg(&bytes).expect("decode succeeds");
        assert_eq!((decoded.width, decoded.height), (img.width, img.height));
        assert!(psnr_rgba8(&img, &decoded) > 45.0);
    }

    #[test]
    fn jpeg_higher_quality_yields_lower_error() {
        let img = gradient_rgba8(48, 48);
        let psnr_high = psnr_rgba8(&img, &decode_jpeg(&encode_jpeg(&img, 100)).expect("decode succeeds"));
        let psnr_low = psnr_rgba8(&img, &decode_jpeg(&encode_jpeg(&img, 10)).expect("decode succeeds"));
        assert!(psnr_high > psnr_low);
    }

    #[test]
    fn jpeg_decode_never_panics_on_truncated_input() {
        let img = gradient_rgba8(32, 32);
        let bytes = encode_jpeg(&img, 80);
        for cut in [0usize, 1, 2, 4, 10, bytes.len() / 4, bytes.len() / 2, (3 * bytes.len()) / 4, bytes.len() - 5, bytes.len() - 1] {
            let result = decode_jpeg(&bytes[..cut]);
            assert!(result.is_err(), "truncation at {cut} bytes should error, not decode");
        }
    }

    #[test]
    fn jpeg_single_mcu_cosine_pattern_recovers_within_quantization_error() {
        let mut img = ImageRgba8::new(16, 16);
        for y in 0..16u32 {
            for x in 0..16u32 {
                let idx = ((y * 16 + x) * 4) as usize;
                let value = (128.0 + 60.0 * (x as f32 * std::f32::consts::PI / 8.0).cos()).round().clamp(0.0, 255.0) as u8;
                img.data[idx] = value;
                img.data[idx + 1] = value;
                img.data[idx + 2] = value;
                img.data[idx + 3] = 255;
            }
        }
        let bytes = encode_jpeg(&img, 95);
        let decoded = decode_jpeg(&bytes).expect("decode succeeds");
        assert_eq!((decoded.width, decoded.height), (16, 16));
        let mut max_abs_error = 0i32;
        for (&expected, &actual) in img.data.iter().zip(decoded.data.iter()) {
            max_abs_error = max_abs_error.max((i32::from(expected) - i32::from(actual)).abs());
        }
        assert!(max_abs_error < 20, "max abs error {max_abs_error} exceeds quantization tolerance");
    }

    #[test]
    fn jpeg_progressive_marker_is_unsupported() {
        let bytes = [0xFFu8, 0xD8, 0xFF, 0xC2, 0x00, 0x0B, 0x08, 0x00, 0x10, 0x00, 0x10, 0x03, 0x01, 0x22, 0x00, 0x02, 0x11, 0x01];
        assert!(matches!(decode_jpeg(&bytes), Err(ImageError::UnsupportedJpeg(_))));
    }

    #[test]
    fn jpeg_decode_rejects_missing_soi() {
        assert!(matches!(decode_jpeg(&[0x00, 0x01, 0x02]), Err(ImageError::Decode(_))));
        assert!(matches!(decode_jpeg(&[]), Err(ImageError::Decode(_))));
    }
}
// #endregion 🔖️Tests
