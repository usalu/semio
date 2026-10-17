//! 🧮️ Errors, transforms and constant tables of the JPEG codec: the typed error, the zigzag order, the 8x8 DCT and inverse DCT, the quantization and standard Huffman tables and the colour conversion. See [`super`].


//#region Errors
/// 🚧 Typed decode/encode failure — `Unsupported` names the exact JPEG
/// variant so callers never mistake "we chose not to decode this" for
/// "this decoded to garbage".
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JpgError {
    Unsupported(String),
    Malformed(String),
}

impl std::fmt::Display for JpgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JpgError::Unsupported(what) => write!(f, "jpg: unsupported: {what}"),
            JpgError::Malformed(what) => write!(f, "jpg: malformed: {what}"),
        }
    }
}

impl std::error::Error for JpgError {}

/// 🧭️ Carries the codec's own unsupported/malformed authority into the canonical Value refusal kinds.
impl From<JpgError> for semio_framework_value::ValueError {
    fn from(error: JpgError) -> Self {
        let kind = match &error { JpgError::Unsupported(_) => semio_framework_value::ValueRefusalKind::UnsupportedOwner, JpgError::Malformed(_) => semio_framework_value::ValueRefusalKind::InvalidValue };
        Self::new(kind, error.to_string())
    }
}
//#endregion Errors

//#region ZigZag
/// 🔀 `NATURAL[zigzag_index]` — DQT/DCT coefficients are stored/decoded in
/// zigzag scan order; this maps back to row-major 8x8 block position.
pub(crate) const ZIGZAG_TO_NATURAL: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];
//#endregion ZigZag

//#region Idct
/// 📐 Separable 1D IDCT-8 (ITU T.81 A.3.3), applied row-then-column for the
/// 2D block transform — O(N^2) per axis instead of the O(N^4) direct sum.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn idct_1d(input: &[f64; 8]) -> [f64; 8] {
    let mut out = [0f64; 8];
    for (x, value) in out.iter_mut().enumerate() {
        let mut sum = 0f64;
        for (u, &sample) in input.iter().enumerate() {
            let cu = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
            sum += cu * sample * ((2.0 * x as f64 + 1.0) * u as f64 * std::f64::consts::PI / 16.0).cos();
        }
        *value = 0.5 * sum;
    }
    out
}

/// 📐 Separable 1D forward DCT-8 — mirror of `idct_1d`, used by the encoder.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fdct_1d(input: &[f64; 8]) -> [f64; 8] {
    let mut out = [0f64; 8];
    for (u, value) in out.iter_mut().enumerate() {
        let cu = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
        let mut sum = 0f64;
        for (x, &sample) in input.iter().enumerate() {
            sum += sample * ((2.0 * x as f64 + 1.0) * u as f64 * std::f64::consts::PI / 16.0).cos();
        }
        *value = 0.5 * cu * sum;
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn idct_8x8(block: &[f64; 64]) -> [f64; 64] {
    let mut tmp = [0f64; 64];
    for r in 0..8 {
        let mut row = [0f64; 8];
        row.copy_from_slice(&block[r * 8..r * 8 + 8]);
        tmp[r * 8..r * 8 + 8].copy_from_slice(&idct_1d(&row));
    }
    let mut out = [0f64; 64];
    for c in 0..8 {
        let mut col = [0f64; 8];
        for r in 0..8 {
            col[r] = tmp[r * 8 + c];
        }
        let res = idct_1d(&col);
        for r in 0..8 {
            out[r * 8 + c] = res[r];
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn fdct_8x8(block: &[f64; 64]) -> [f64; 64] {
    let mut tmp = [0f64; 64];
    for c in 0..8 {
        let mut col = [0f64; 8];
        for r in 0..8 {
            col[r] = block[r * 8 + c];
        }
        let res = fdct_1d(&col);
        for r in 0..8 {
            tmp[r * 8 + c] = res[r];
        }
    }
    let mut out = [0f64; 64];
    for r in 0..8 {
        let mut row = [0f64; 8];
        row.copy_from_slice(&tmp[r * 8..r * 8 + 8]);
        out[r * 8..r * 8 + 8].copy_from_slice(&fdct_1d(&row));
    }
    out
}
//#endregion Idct

//#region QuantTables
/// 📊 Annex K.1 example luminance quantization table (natural/row-major order).
pub(crate) const STD_LUMA_Q: [i32; 64] = [
    16, 11, 10, 16, 24, 40, 51, 61, 12, 12, 14, 19, 26, 58, 60, 55, 14, 13, 16, 24, 40, 57, 69, 56, 14, 17, 22, 29, 51, 87, 80, 62, 18, 22, 37, 56, 68, 109, 103, 77, 24, 35, 55, 64, 81, 104, 113, 92, 49, 64, 78, 87, 103, 121, 120, 101, 72, 92, 95,
    98, 112, 100, 103, 99,
];

/// 📊 Annex K.1 example chrominance quantization table (natural order).
pub(crate) const STD_CHROMA_Q: [i32; 64] = [
    17, 18, 24, 47, 99, 99, 99, 99, 18, 21, 26, 66, 99, 99, 99, 99, 24, 26, 56, 99, 99, 99, 99, 99, 47, 66, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99,
];

/// 📈 IJG-standard quality→scale mapping applied to the Annex K base tables.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn scale_quality(base: &[i32; 64], quality: i32) -> [i32; 64] {
    let quality = quality.clamp(1, 100);
    let scale = if quality < 50 { 5000 / quality } else { 200 - quality * 2 };
    let mut out = [0i32; 64];
    for i in 0..64 {
        out[i] = ((base[i] * scale + 50) / 100).clamp(1, 255);
    }
    out
}

/// 🔀 Reindexes a natural-order table into zigzag order — DQT stores entries
/// in the same scan order the entropy coder emits, so `table[z]` lines up
/// directly with a zigzag-order coefficient at position `z`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn quant_zigzag(natural: &[i32; 64]) -> [i32; 64] {
    let mut out = [0i32; 64];
    for z in 0..64 {
        out[z] = natural[ZIGZAG_TO_NATURAL[z]];
    }
    out
}
//#endregion QuantTables

//#region StdHuffmanTables
/// 🌳 Annex K.3 example Huffman tables (bits[16] counts + value bytes), used
/// both to build the encoder's tables and to write real DHT segments the
/// decoder reconstructs identically from — round-trip correctness therefore
/// doesn't depend on matching the spec's tables byte-for-byte, only on
/// internal consistency between what's written and what's parsed.
pub(crate) const DC_LUMA_BITS: [u8; 16] = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dc_luma_values() -> Vec<u8> {
    (0..=11).collect()
}

pub(crate) const DC_CHROMA_BITS: [u8; 16] = [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dc_chroma_values() -> Vec<u8> {
    (0..=11).collect()
}

pub(crate) const AC_LUMA_BITS: [u8; 16] = [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 0x7d];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn ac_luma_values() -> Vec<u8> {
    vec![
        0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08, 0x23, 0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16,
        0x17, 0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
        0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6,
        0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
        0xf9, 0xfa,
    ]
}

pub(crate) const AC_CHROMA_BITS: [u8; 16] = [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 0x77];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn ac_chroma_values() -> Vec<u8> {
    vec![
        0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71, 0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0, 0x15, 0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34,
        0xe1, 0x25, 0xf1, 0x17, 0x18, 0x19, 0x1a, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4,
        0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
        0xf9, 0xfa,
    ]
}
//#endregion StdHuffmanTables

//#region ColorConvert
/// 🎨 ITU-R BT.601 RGB→YCbCr.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn rgb_to_ycbcr(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let (r, g, b) = (r as f64, g as f64, b as f64);
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let cb = -0.168736 * r - 0.331264 * g + 0.5 * b + 128.0;
    let cr = 0.5 * r - 0.418688 * g - 0.081312 * b + 128.0;
    (y, cb, cr)
}

/// 🎨 ITU-R BT.601 YCbCr→RGB, clamped to `0..=255`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn ycbcr_to_rgb(y: f64, cb: f64, cr: f64) -> (u8, u8, u8) {
    let (cb, cr) = (cb - 128.0, cr - 128.0);
    let r = y + 1.402 * cr;
    let g = y - 0.344136 * cb - 0.714136 * cr;
    let b = y + 1.772 * cb;
    (r.round().clamp(0.0, 255.0) as u8, g.round().clamp(0.0, 255.0) as u8, b.round().clamp(0.0, 255.0) as u8)
}
//#endregion ColorConvert
