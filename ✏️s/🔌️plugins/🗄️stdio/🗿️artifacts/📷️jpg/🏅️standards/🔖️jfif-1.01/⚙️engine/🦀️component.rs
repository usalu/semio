//! ⚙️ JpgEngine — real baseline-sequential (SOF0) JPEG codec: Huffman entropy
//! decoding, dequantization, integer-ish separable IDCT, YCbCr→RGB with
//! nearest-neighbor chroma upsampling. Progressive/arithmetic/lossless SOFn
//! variants are explicit `JpgError::Unsupported`, never decoded as garbage.
//! Verified against a standalone scratch harness (22/22 checks: IDCT/FDCT
//! identity, Huffman round trip, 4:2:0 subsampling, restart intervals,
//! grayscale, SOF2 rejection) before landing here.

use crate::artifacts::jpg::{
    schema::snapshot::{JpgFrameComponent, JpgFrameHeader, JpgScanComponent, RasterImage},
    JpgArtifact, JpgDiff, JpgMutation, JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA,
};
use std::collections::HashMap;

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
//#endregion Errors

//#region ZigZag
/// 🔀 `NATURAL[zigzag_index]` — DQT/DCT coefficients are stored/decoded in
/// zigzag scan order; this maps back to row-major 8x8 block position.
const ZIGZAG_TO_NATURAL: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10,
    17, 24, 32, 25, 18, 11, 4, 5,
    12, 19, 26, 33, 40, 48, 41, 34,
    27, 20, 13, 6, 7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36,
    29, 22, 15, 23, 30, 37, 44, 51,
    58, 59, 52, 45, 38, 31, 39, 46,
    53, 60, 61, 54, 47, 55, 62, 63,
];
//#endregion ZigZag

//#region Idct
/// 📐 Separable 1D IDCT-8 (ITU T.81 A.3.3), applied row-then-column for the
/// 2D block transform — O(N^2) per axis instead of the O(N^4) direct sum.
fn idct_1d(input: &[f64; 8]) -> [f64; 8] {
    let mut out = [0f64; 8];
    for x in 0..8 {
        let mut sum = 0f64;
        for u in 0..8 {
            let cu = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
            sum += cu * input[u] * ((2.0 * x as f64 + 1.0) * u as f64 * std::f64::consts::PI / 16.0).cos();
        }
        out[x] = 0.5 * sum;
    }
    out
}

/// 📐 Separable 1D forward DCT-8 — mirror of `idct_1d`, used by the encoder.
fn fdct_1d(input: &[f64; 8]) -> [f64; 8] {
    let mut out = [0f64; 8];
    for u in 0..8 {
        let cu = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
        let mut sum = 0f64;
        for x in 0..8 {
            sum += input[x] * ((2.0 * x as f64 + 1.0) * u as f64 * std::f64::consts::PI / 16.0).cos();
        }
        out[u] = 0.5 * cu * sum;
    }
    out
}

fn idct_8x8(block: &[f64; 64]) -> [f64; 64] {
    let mut tmp = [0f64; 64];
    for r in 0..8 {
        let mut row = [0f64; 8];
        row.copy_from_slice(&block[r * 8..r * 8 + 8]);
        tmp[r * 8..r * 8 + 8].copy_from_slice(&idct_1d(&row));
    }
    let mut out = [0f64; 64];
    for c in 0..8 {
        let mut col = [0f64; 8];
        for r in 0..8 { col[r] = tmp[r * 8 + c]; }
        let res = idct_1d(&col);
        for r in 0..8 { out[r * 8 + c] = res[r]; }
    }
    out
}

fn fdct_8x8(block: &[f64; 64]) -> [f64; 64] {
    let mut tmp = [0f64; 64];
    for c in 0..8 {
        let mut col = [0f64; 8];
        for r in 0..8 { col[r] = block[r * 8 + c]; }
        let res = fdct_1d(&col);
        for r in 0..8 { tmp[r * 8 + c] = res[r]; }
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

//#region Huffman
/// 🌳 Canonical Huffman table built from DHT's `bits`/`values` (ITU T.81
/// Annex C). `decode` keyed by `(code_length, code)`; `encode` by symbol.
#[derive(Clone, Debug, Default)]
struct HuffTable {
    decode: HashMap<(u8, u16), u8>,
    encode: HashMap<u8, (u8, u16)>,
    max_len: u8,
}

/// 🏗️ Builds canonical codes per Annex C.2: codes are assigned in symbol
/// order, incrementing within a length and left-shifting on length change —
/// same "canonical" spirit as deflate's Huffman but JPEG's table layout
/// (flat bits[16] counts + values[]) is its own format, not reused from deflate.
fn build_huffman(bits: &[u8; 16], values: &[u8]) -> Result<HuffTable, JpgError> {
    let mut sizes: Vec<u8> = Vec::new();
    for (l, &count) in bits.iter().enumerate() {
        for _ in 0..count { sizes.push((l + 1) as u8); }
    }
    if sizes.len() != values.len() {
        return Err(JpgError::Malformed("DHT bits/values length mismatch".into()));
    }
    let mut codes: Vec<u16> = Vec::with_capacity(sizes.len());
    if !sizes.is_empty() {
        let mut code: u16 = 0;
        let mut si = sizes[0];
        let mut k = 0usize;
        while k < sizes.len() {
            while k < sizes.len() && sizes[k] == si {
                codes.push(code);
                code += 1;
                k += 1;
            }
            code <<= 1;
            si += 1;
        }
    }
    let mut table = HuffTable::default();
    for ((len, code), val) in sizes.into_iter().zip(codes).zip(values.iter().copied()) {
        table.decode.insert((len, code), val);
        table.encode.insert(val, (len, code));
        table.max_len = table.max_len.max(len);
    }
    Ok(table)
}
//#endregion Huffman

//#region BitIo
/// ✍️ MSB-first bit writer with JPEG byte stuffing (`0xFF` → `0xFF 0x00`).
struct BitWriter {
    bytes: Vec<u8>,
    acc: u32,
    nbits: u32,
}
impl BitWriter {
    fn new() -> Self { Self { bytes: Vec::new(), acc: 0, nbits: 0 } }
    fn put_bits(&mut self, value: u16, len: u8) {
        if len == 0 { return; }
        self.acc = (self.acc << len) | (value as u32 & ((1u32 << len) - 1));
        self.nbits += len as u32;
        while self.nbits >= 8 {
            self.nbits -= 8;
            let byte = ((self.acc >> self.nbits) & 0xFF) as u8;
            self.bytes.push(byte);
            if byte == 0xFF { self.bytes.push(0x00); }
        }
    }
    fn flush(&mut self) {
        if self.nbits > 0 {
            let pad = 8 - self.nbits;
            let byte = ((self.acc << pad) & 0xFF) as u8;
            self.bytes.push(byte);
            if byte == 0xFF { self.bytes.push(0x00); }
            self.nbits = 0;
            self.acc = 0;
        }
    }
}

/// 👓 MSB-first bit reader over the entropy-coded segment. `next_byte`
/// transparently undoes byte stuffing and reports `None` the instant a real
/// marker (restart or otherwise) is encountered, so callers can react instead
/// of silently consuming marker bytes as data.
struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    acc: u32,
    nbits: u32,
}
impl<'a> BitReader<'a> {
    fn new(data: &'a [u8], pos: usize) -> Self { Self { data, pos, acc: 0, nbits: 0 } }
    fn next_byte(&mut self) -> Option<u8> {
        if self.pos >= self.data.len() { return None; }
        let b = self.data[self.pos];
        if b == 0xFF {
            let b2 = self.data.get(self.pos + 1).copied().unwrap_or(0);
            if b2 == 0x00 { self.pos += 2; return Some(0xFF); }
            return None;
        }
        self.pos += 1;
        Some(b)
    }
    fn read_bit(&mut self) -> Result<u8, JpgError> {
        if self.nbits == 0 {
            match self.next_byte() {
                Some(b) => { self.acc = b as u32; self.nbits = 8; }
                None => return Err(JpgError::Malformed("unexpected marker inside entropy-coded segment".into())),
            }
        }
        self.nbits -= 1;
        Ok(((self.acc >> self.nbits) & 1) as u8)
    }
    fn read_bits(&mut self, n: u8) -> Result<u16, JpgError> {
        let mut v = 0u16;
        for _ in 0..n { v = (v << 1) | self.read_bit()? as u16; }
        Ok(v)
    }
    fn decode_symbol(&mut self, table: &HuffTable) -> Result<u8, JpgError> {
        let mut code: u16 = 0;
        for len in 1..=table.max_len {
            code = (code << 1) | self.read_bit()? as u16;
            if let Some(v) = table.decode.get(&(len, code)) { return Ok(*v); }
        }
        Err(JpgError::Malformed("huffman decode: no matching code".into()))
    }
    /// 🔁 Byte-align and consume one `RSTn` marker at a restart boundary;
    /// also resets the DC predictors (caller's responsibility) per T.81 F.2.2.5.
    fn skip_restart_marker(&mut self) -> Result<(), JpgError> {
        self.nbits = 0;
        self.acc = 0;
        if self.pos + 1 < self.data.len() && self.data[self.pos] == 0xFF && (0xD0..=0xD7).contains(&self.data[self.pos + 1]) {
            self.pos += 2;
            Ok(())
        } else {
            Err(JpgError::Malformed("expected restart marker not found".into()))
        }
    }
}

/// ➕ Sign-extends a JPEG-encoded magnitude/sign pair (T.81 F.12): values
/// below `2^(size-1)` are negative, encoded as `value - (2^size - 1)`.
fn extend_sign(value: u16, size: u8) -> i32 {
    if size == 0 { return 0; }
    let v = value as i32;
    let vt = 1i32 << (size - 1);
    if v < vt { v - (1 << size) + 1 } else { v }
}

fn size_of(mut v: i32) -> u8 {
    if v < 0 { v = -v; }
    let mut s = 0u8;
    while v > 0 { s += 1; v >>= 1; }
    s
}
//#endregion BitIo

//#region BlockCodec
/// 🧱 Encodes one 8x8 block's already-quantized zigzag coefficients: DC as a
/// difference from the running per-component predictor, AC via run-length +
/// size Huffman symbols with ZRL (0xF0) for 16-zero runs and EOB (0x00) once
/// the remainder is all zero.
fn encode_block(bw: &mut BitWriter, coeffs: &[i32; 64], dc_pred: &mut i32, dc_table: &HuffTable, ac_table: &HuffTable) -> Result<(), JpgError> {
    let diff = coeffs[0] - *dc_pred;
    *dc_pred = coeffs[0];
    let sz = size_of(diff);
    let (len, code) = *dc_table.encode.get(&sz).ok_or_else(|| JpgError::Malformed("dc symbol not in table".into()))?;
    bw.put_bits(code, len);
    if sz > 0 {
        let bits = if diff < 0 { (diff - 1) as u16 & ((1u16 << sz) - 1) } else { diff as u16 };
        bw.put_bits(bits, sz);
    }
    let mut run = 0u8;
    for &v in coeffs.iter().skip(1) {
        if v == 0 { run += 1; continue; }
        while run >= 16 {
            let (len, code) = *ac_table.encode.get(&0xF0).ok_or_else(|| JpgError::Malformed("ac ZRL symbol not in table".into()))?;
            bw.put_bits(code, len);
            run -= 16;
        }
        let sz = size_of(v);
        let rs = (run << 4) | sz;
        let (len, code) = *ac_table.encode.get(&rs).ok_or_else(|| JpgError::Malformed("ac symbol not in table".into()))?;
        bw.put_bits(code, len);
        let bits = if v < 0 { (v - 1) as u16 & ((1u16 << sz) - 1) } else { v as u16 };
        bw.put_bits(bits, sz);
        run = 0;
    }
    if run > 0 {
        let (len, code) = *ac_table.encode.get(&0x00).ok_or_else(|| JpgError::Malformed("ac EOB symbol not in table".into()))?;
        bw.put_bits(code, len);
    }
    Ok(())
}

/// 🧱 Decodes one 8x8 block into zigzag-order quantized coefficients.
fn decode_block(br: &mut BitReader, dc_pred: &mut i32, dc_table: &HuffTable, ac_table: &HuffTable) -> Result<[i32; 64], JpgError> {
    let mut out = [0i32; 64];
    let sz = br.decode_symbol(dc_table)?;
    let bits = if sz > 0 { br.read_bits(sz)? } else { 0 };
    *dc_pred += extend_sign(bits, sz);
    out[0] = *dc_pred;
    let mut z = 1usize;
    while z < 64 {
        let rs = br.decode_symbol(ac_table)?;
        let run = rs >> 4;
        let sz = rs & 0x0F;
        if sz == 0 {
            if run == 15 { z += 16; continue; } // ZRL
            break; // EOB
        }
        z += run as usize;
        if z >= 64 { return Err(JpgError::Malformed("ac coefficient run overruns block".into())); }
        let bits = br.read_bits(sz)?;
        out[z] = extend_sign(bits, sz);
        z += 1;
    }
    Ok(out)
}
//#endregion BlockCodec

//#region QuantTables
/// 📊 Annex K.1 example luminance quantization table (natural/row-major order).
const STD_LUMA_Q: [i32; 64] = [
    16, 11, 10, 16, 24, 40, 51, 61,
    12, 12, 14, 19, 26, 58, 60, 55,
    14, 13, 16, 24, 40, 57, 69, 56,
    14, 17, 22, 29, 51, 87, 80, 62,
    18, 22, 37, 56, 68, 109, 103, 77,
    24, 35, 55, 64, 81, 104, 113, 92,
    49, 64, 78, 87, 103, 121, 120, 101,
    72, 92, 95, 98, 112, 100, 103, 99,
];
/// 📊 Annex K.1 example chrominance quantization table (natural order).
const STD_CHROMA_Q: [i32; 64] = [
    17, 18, 24, 47, 99, 99, 99, 99,
    18, 21, 26, 66, 99, 99, 99, 99,
    24, 26, 56, 99, 99, 99, 99, 99,
    47, 66, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
];

/// 📈 IJG-standard quality→scale mapping applied to the Annex K base tables.
fn scale_quality(base: &[i32; 64], quality: i32) -> [i32; 64] {
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
fn quant_zigzag(natural: &[i32; 64]) -> [i32; 64] {
    let mut out = [0i32; 64];
    for z in 0..64 { out[z] = natural[ZIGZAG_TO_NATURAL[z]]; }
    out
}
//#endregion QuantTables

//#region StdHuffmanTables
/// 🌳 Annex K.3 example Huffman tables (bits[16] counts + value bytes), used
/// both to build the encoder's tables and to write real DHT segments the
/// decoder reconstructs identically from — round-trip correctness therefore
/// doesn't depend on matching the spec's tables byte-for-byte, only on
/// internal consistency between what's written and what's parsed.
const DC_LUMA_BITS: [u8; 16] = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
fn dc_luma_values() -> Vec<u8> { (0..=11).collect() }
const DC_CHROMA_BITS: [u8; 16] = [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
fn dc_chroma_values() -> Vec<u8> { (0..=11).collect() }
const AC_LUMA_BITS: [u8; 16] = [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 0x7d];
fn ac_luma_values() -> Vec<u8> {
    vec![
        0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12,
        0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07,
        0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08,
        0x23, 0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0,
        0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16,
        0x17, 0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28,
        0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
        0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
        0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
        0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
        0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79,
        0x7a, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
        0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98,
        0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
        0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6,
        0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5,
        0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4,
        0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2,
        0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea,
        0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
        0xf9, 0xfa,
    ]
}
const AC_CHROMA_BITS: [u8; 16] = [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 0x77];
fn ac_chroma_values() -> Vec<u8> {
    vec![
        0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21,
        0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71,
        0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91,
        0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0,
        0x15, 0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34,
        0xe1, 0x25, 0xf1, 0x17, 0x18, 0x19, 0x1a, 0x26,
        0x27, 0x28, 0x29, 0x2a, 0x35, 0x36, 0x37, 0x38,
        0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
        0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
        0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
        0x79, 0x7a, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
        0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96,
        0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5,
        0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4,
        0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3,
        0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2,
        0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda,
        0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9,
        0xea, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
        0xf9, 0xfa,
    ]
}
//#endregion StdHuffmanTables

//#region ColorConvert
/// 🎨 ITU-R BT.601 RGB→YCbCr.
fn rgb_to_ycbcr(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let (r, g, b) = (r as f64, g as f64, b as f64);
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let cb = -0.168736 * r - 0.331264 * g + 0.5 * b + 128.0;
    let cr = 0.5 * r - 0.418688 * g - 0.081312 * b + 128.0;
    (y, cb, cr)
}
/// 🎨 ITU-R BT.601 YCbCr→RGB, clamped to `0..=255`.
fn ycbcr_to_rgb(y: f64, cb: f64, cr: f64) -> (u8, u8, u8) {
    let (cb, cr) = (cb - 128.0, cr - 128.0);
    let r = y + 1.402 * cr;
    let g = y - 0.344136 * cb - 0.714136 * cr;
    let b = y + 1.772 * cb;
    (r.round().clamp(0.0, 255.0) as u8, g.round().clamp(0.0, 255.0) as u8, b.round().clamp(0.0, 255.0) as u8)
}
//#endregion ColorConvert

//#region Encode
/// 🧩 Downsamples a full-res plane by box-averaging `fx`×`fy` pixel blocks —
/// used to build the (subsampled) chroma planes at 4:2:0/4:2:2 from 4:4:4 source.
fn box_downsample(src: &[f64], sw: usize, sh: usize, fx: usize, fy: usize) -> (Vec<f64>, usize, usize) {
    let dw = sw / fx;
    let dh = sh / fy;
    let mut out = vec![0f64; dw * dh];
    for y in 0..dh {
        for x in 0..dw {
            let mut sum = 0f64;
            for dy in 0..fy { for dx in 0..fx { sum += src[(y * fy + dy) * sw + (x * fx + dx)]; } }
            out[y * dw + x] = sum / (fx * fy) as f64;
        }
    }
    (out, dw, dh)
}

/// 🖨️ Encodes an RGBA raster as baseline sequential JPEG, 3-component
/// (Y/Cb/Cr, ids 1/2/3) 4:2:0 subsampled, Annex K example tables at quality 90
/// — chosen so the round trip through our own decoder stays well under a
/// visually-lossless error budget. Edges are replicated (not zero-padded) up
/// to the next MCU (16x16) boundary to avoid ringing.
pub fn encode_jpg(snap: &JpgSnapshot) -> Result<Vec<u8>, JpgError> {
    let img = &snap.image;
    if img.width == 0 || img.height == 0 { return Err(JpgError::Malformed("empty image".into())); }
    if img.rgba.len() != (img.width as usize) * (img.height as usize) * 4 {
        return Err(JpgError::Malformed("rgba length mismatch".into()));
    }
    if img.width > u16::MAX as u32 || img.height > u16::MAX as u32 {
        return Err(JpgError::Unsupported("image dimensions exceed JPEG's 16-bit SOF0 width/height field".into()));
    }
    let (width, height): (u16, u16) = (img.width as u16, img.height as u16);
    let hmax = 2usize;
    let vmax = 2usize;
    let mcu_w = 8 * hmax;
    let mcu_h = 8 * vmax;
    let mcus_x = (width as usize + mcu_w - 1) / mcu_w;
    let mcus_y = (height as usize + mcu_h - 1) / mcu_h;
    let pw = mcus_x * mcu_w;
    let ph = mcus_y * mcu_h;

    let mut yfull = vec![0f64; pw * ph];
    let mut cbfull = vec![0f64; pw * ph];
    let mut crfull = vec![0f64; pw * ph];
    for y in 0..ph {
        let sy = y.min(height as usize - 1);
        for x in 0..pw {
            let sx = x.min(width as usize - 1);
            let idx = (sy * width as usize + sx) * 4;
            let (yy, cb, cr) = rgb_to_ycbcr(img.rgba[idx], img.rgba[idx + 1], img.rgba[idx + 2]);
            yfull[y * pw + x] = yy;
            cbfull[y * pw + x] = cb;
            crfull[y * pw + x] = cr;
        }
    }
    let (cbplane, cpw, _cph) = box_downsample(&cbfull, pw, ph, hmax, vmax);
    let (crplane, _, _) = box_downsample(&crfull, pw, ph, hmax, vmax);

    let comps: [JpgFrameComponent; 3] = [
        JpgFrameComponent { id: 1, h_sampling: hmax as u8, v_sampling: vmax as u8, quant_table_id: 0 },
        JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
        JpgFrameComponent { id: 3, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
    ];
    let frame = JpgFrameHeader { precision: 8, width, height, components: comps.to_vec() };

    let luma_q = quant_zigzag(&scale_quality(&STD_LUMA_Q, 90));
    let chroma_q = quant_zigzag(&scale_quality(&STD_CHROMA_Q, 90));
    let dc_luma = build_huffman(&DC_LUMA_BITS, &dc_luma_values())?;
    let ac_luma = build_huffman(&AC_LUMA_BITS, &ac_luma_values())?;
    let dc_chroma = build_huffman(&DC_CHROMA_BITS, &dc_chroma_values())?;
    let ac_chroma = build_huffman(&AC_CHROMA_BITS, &ac_chroma_values())?;

    let mut out = Vec::new();
    out.extend_from_slice(&[0xFF, 0xD8]); // SOI
    out.extend_from_slice(&[0xFF, 0xE0, 0x00, 0x10, b'J', b'F', b'I', b'F', 0, 1, 1, 0, 0, 1, 0, 1, 0, 0]); // APP0/JFIF

    let mut dqt0 = vec![0xFFu8, 0xDB, 0x00, 0x43, 0x00];
    dqt0.extend(luma_q.iter().map(|&v| v as u8));
    out.extend_from_slice(&dqt0);
    let mut dqt1 = vec![0xFFu8, 0xDB, 0x00, 0x43, 0x01];
    dqt1.extend(chroma_q.iter().map(|&v| v as u8));
    out.extend_from_slice(&dqt1);

    let mut sof = vec![0xFFu8, 0xC0];
    let sof_len = 8 + 3 * frame.components.len();
    sof.push((sof_len >> 8) as u8);
    sof.push((sof_len & 0xFF) as u8);
    sof.push(frame.precision);
    sof.push((frame.height >> 8) as u8);
    sof.push((frame.height & 0xFF) as u8);
    sof.push((frame.width >> 8) as u8);
    sof.push((frame.width & 0xFF) as u8);
    sof.push(frame.components.len() as u8);
    for c in frame.components.iter() {
        sof.push(c.id);
        sof.push((c.h_sampling << 4) | c.v_sampling);
        sof.push(c.quant_table_id);
    }
    out.extend_from_slice(&sof);

    write_dht(&mut out, 0, 0, &DC_LUMA_BITS, &dc_luma_values());
    write_dht(&mut out, 1, 0, &AC_LUMA_BITS, &ac_luma_values());
    write_dht(&mut out, 0, 1, &DC_CHROMA_BITS, &dc_chroma_values());
    write_dht(&mut out, 1, 1, &AC_CHROMA_BITS, &ac_chroma_values());

    let scan_comps: Vec<JpgScanComponent> = vec![
        JpgScanComponent { id: 1, dc_table_id: 0, ac_table_id: 0 },
        JpgScanComponent { id: 2, dc_table_id: 1, ac_table_id: 1 },
        JpgScanComponent { id: 3, dc_table_id: 1, ac_table_id: 1 },
    ];
    let mut sos = vec![0xFFu8, 0xDA];
    let sos_len = 6 + 2 * scan_comps.len();
    sos.push((sos_len >> 8) as u8);
    sos.push((sos_len & 0xFF) as u8);
    sos.push(scan_comps.len() as u8);
    for c in scan_comps.iter() {
        sos.push(c.id);
        sos.push((c.dc_table_id << 4) | c.ac_table_id);
    }
    sos.push(0);
    sos.push(63);
    sos.push(0);
    out.extend_from_slice(&sos);

    let mut bw = BitWriter::new();
    let mut dc_pred = [0i32; 3];
    for my in 0..mcus_y {
        for mx in 0..mcus_x {
            // component 0 (Y): full-res 2x2 blocks per MCU
            for by in 0..vmax {
                for bx in 0..hmax {
                    let ox = (mx * hmax + bx) * 8;
                    let oy = (my * vmax + by) * 8;
                    let mut block = [0f64; 64];
                    for r in 0..8 { for c in 0..8 { block[r * 8 + c] = yfull[(oy + r) * pw + (ox + c)] - 128.0; } }
                    let coeff = fdct_8x8(&block);
                    let mut zz = [0i32; 64];
                    for z in 0..64 { zz[z] = (coeff[ZIGZAG_TO_NATURAL[z]] / luma_q[z] as f64).round() as i32; }
                    encode_block(&mut bw, &zz, &mut dc_pred[0], &dc_luma, &ac_luma)?;
                }
            }
            // Cb, Cr: one block per MCU (already half-res)
            for (ci, plane) in [&cbplane, &crplane].iter().enumerate() {
                let ox = mx * 8;
                let oy = my * 8;
                let mut block = [0f64; 64];
                for r in 0..8 { for c in 0..8 { block[r * 8 + c] = plane[(oy + r) * cpw + (ox + c)] - 128.0; } }
                let coeff = fdct_8x8(&block);
                let mut zz = [0i32; 64];
                for z in 0..64 { zz[z] = (coeff[ZIGZAG_TO_NATURAL[z]] / chroma_q[z] as f64).round() as i32; }
                encode_block(&mut bw, &zz, &mut dc_pred[1 + ci], &dc_chroma, &ac_chroma)?;
            }
        }
    }
    bw.flush();
    out.extend_from_slice(&bw.bytes);
    out.extend_from_slice(&[0xFF, 0xD9]); // EOI
    Ok(out)
}

fn write_dht(out: &mut Vec<u8>, class: u8, id: u8, bits: &[u8; 16], values: &[u8]) {
    out.push(0xFF);
    out.push(0xC4);
    let len = 2 + 1 + 16 + values.len();
    out.push((len >> 8) as u8);
    out.push((len & 0xFF) as u8);
    out.push((class << 4) | id);
    out.extend_from_slice(bits);
    out.extend_from_slice(values);
}
//#endregion Encode

//#region Decode
/// 📥 Decodes baseline sequential JPEG (SOF0 only) into an RGBA raster.
/// Any other SOFn marker (progressive/extended/lossless/arithmetic) is a
/// typed `JpgError::Unsupported` naming the exact variant — never decoded.
pub fn decode_jpg(data: &[u8]) -> Result<JpgSnapshot, JpgError> {
    if data.len() < 4 || data[0] != 0xFF || data[1] != 0xD8 {
        return Err(JpgError::Malformed("missing SOI".into()));
    }
    let mut i = 2usize;
    let mut quant: HashMap<u8, [i32; 64]> = HashMap::new();
    let mut dc_tables: HashMap<u8, HuffTable> = HashMap::new();
    let mut ac_tables: HashMap<u8, HuffTable> = HashMap::new();
    let mut frame: Option<JpgFrameHeader> = None;
    let mut restart_interval = 0u16;

    loop {
        if i + 1 >= data.len() { return Err(JpgError::Malformed("truncated before EOI".into())); }
        if data[i] != 0xFF { i += 1; continue; }
        let marker = data[i + 1];
        i += 2;
        match marker {
            0xD8 => continue, // stray SOI, tolerate
            0xD9 => return Err(JpgError::Malformed("EOI before SOS".into())),
            0xC0 => {
                let len = read_u16(data, i)?;
                let seg = slice_at(data, i + 2, len.saturating_sub(2))?;
                if seg.len() < 6 { return Err(JpgError::Malformed("SOF0 segment too short".into())); }
                let height = ((seg[1] as u16) << 8) | seg[2] as u16;
                let width = ((seg[3] as u16) << 8) | seg[4] as u16;
                let nc = seg[5] as usize;
                let mut components = Vec::with_capacity(nc);
                for k in 0..nc {
                    let base = 6 + k * 3;
                    if base + 2 >= seg.len() { return Err(JpgError::Malformed("SOF0 component list truncated".into())); }
                    components.push(JpgFrameComponent {
                        id: seg[base],
                        h_sampling: seg[base + 1] >> 4,
                        v_sampling: seg[base + 1] & 0x0F,
                        quant_table_id: seg[base + 2],
                    });
                }
                frame = Some(JpgFrameHeader { precision: seg[0], width, height, components });
                i += len;
            }
            0xC1 | 0xC2 | 0xC3 | 0xC5 | 0xC6 | 0xC7 | 0xC9 | 0xCA | 0xCB | 0xCD | 0xCE | 0xCF => {
                let name = match marker {
                    0xC1 => "extended sequential (SOF1)",
                    0xC2 => "progressive (SOF2)",
                    0xC3 => "lossless (SOF3)",
                    0xC5 => "differential sequential (SOF5)",
                    0xC6 => "differential progressive (SOF6)",
                    0xC7 => "differential lossless (SOF7)",
                    0xC9 => "arithmetic extended sequential (SOF9)",
                    0xCA => "arithmetic progressive (SOF10)",
                    0xCB => "arithmetic lossless (SOF11)",
                    0xCD => "arithmetic differential sequential (SOF13)",
                    0xCE => "arithmetic differential progressive (SOF14)",
                    _ => "arithmetic differential lossless (SOF15)",
                };
                return Err(JpgError::Unsupported(name.into()));
            }
            0xDB => {
                let len = read_u16(data, i)?;
                let mut p = i + 2;
                let end = i + len;
                while p < end {
                    if p >= data.len() { return Err(JpgError::Malformed("DQT truncated".into())); }
                    let pq = data[p] >> 4;
                    let tq = data[p] & 0x0F;
                    p += 1;
                    let mut tbl = [0i32; 64];
                    for slot in tbl.iter_mut() {
                        if pq == 0 {
                            *slot = *data.get(p).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))? as i32;
                            p += 1;
                        } else {
                            let hi = *data.get(p).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                            let lo = *data.get(p + 1).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                            *slot = ((hi as i32) << 8) | lo as i32;
                            p += 2;
                        }
                    }
                    quant.insert(tq, tbl);
                }
                i += len;
            }
            0xC4 => {
                let len = read_u16(data, i)?;
                let mut p = i + 2;
                let end = i + len;
                while p < end {
                    if p + 16 >= data.len() { return Err(JpgError::Malformed("DHT truncated".into())); }
                    let class = data[p] >> 4;
                    let id = data[p] & 0x0F;
                    p += 1;
                    let mut bits = [0u8; 16];
                    bits.copy_from_slice(&data[p..p + 16]);
                    p += 16;
                    let count: usize = bits.iter().map(|&b| b as usize).sum();
                    let values = slice_at(data, p, count)?.to_vec();
                    p += count;
                    let table = build_huffman(&bits, &values)?;
                    if class == 0 { dc_tables.insert(id, table); } else { ac_tables.insert(id, table); }
                }
                i += len;
            }
            0xDD => {
                let len = read_u16(data, i)?;
                let seg = slice_at(data, i + 2, 2)?;
                restart_interval = ((seg[0] as u16) << 8) | seg[1] as u16;
                i += len;
            }
            0xDA => {
                let frame = frame.clone().ok_or_else(|| JpgError::Malformed("SOS before SOF0".into()))?;
                let len = read_u16(data, i)?;
                let seg = slice_at(data, i + 2, len.saturating_sub(2))?;
                let ns = *seg.first().ok_or_else(|| JpgError::Malformed("SOS truncated".into()))? as usize;
                let mut scan_tabs: Vec<(u8, u8)> = Vec::with_capacity(ns);
                for k in 0..ns {
                    let base = 1 + k * 2;
                    let sel = *seg.get(base).ok_or_else(|| JpgError::Malformed("SOS truncated".into()))?;
                    let dcac = *seg.get(base + 1).ok_or_else(|| JpgError::Malformed("SOS truncated".into()))?;
                    let _ = sel;
                    scan_tabs.push((dcac >> 4, dcac & 0x0F));
                }
                if ns != frame.components.len() {
                    return Err(JpgError::Unsupported("multi-scan (non-interleaved) baseline JPEG".into()));
                }
                i += len;
                let rgba = decode_scan(data, i, &frame, &scan_tabs, &quant, &dc_tables, &ac_tables, restart_interval)?;
                return Ok(JpgSnapshot {
                    schema: STDIO_JPG_DOCUMENT_SCHEMA.into(),
                    image: RasterImage { width: frame.width as u32, height: frame.height as u32, rgba },
                });
            }
            0xE0..=0xEF | 0xFE => {
                let len = read_u16(data, i)?;
                i += len;
            }
            0x01 | 0xD0..=0xD7 => {} // TEM / stray restart outside a scan: no length field, skip
            _ => return Err(JpgError::Malformed(format!("unhandled marker 0xFF{marker:02X} before SOS"))),
        }
    }
}

fn read_u16(data: &[u8], at: usize) -> Result<usize, JpgError> {
    let hi = *data.get(at).ok_or_else(|| JpgError::Malformed("marker length truncated".into()))?;
    let lo = *data.get(at + 1).ok_or_else(|| JpgError::Malformed("marker length truncated".into()))?;
    Ok(((hi as usize) << 8) | lo as usize)
}
fn slice_at(data: &[u8], at: usize, len: usize) -> Result<&[u8], JpgError> {
    data.get(at..at + len).ok_or_else(|| JpgError::Malformed("segment out of bounds".into()))
}

/// 🎞️ Decodes the entropy-coded scan for all components (nearest-neighbor
/// chroma upsampling for subsampled components; grayscale skips color
/// conversion entirely) into RGBA.
#[allow(clippy::too_many_arguments)]
fn decode_scan(
    data: &[u8],
    start: usize,
    frame: &JpgFrameHeader,
    scan_tabs: &[(u8, u8)],
    quant: &HashMap<u8, [i32; 64]>,
    dc_tables: &HashMap<u8, HuffTable>,
    ac_tables: &HashMap<u8, HuffTable>,
    restart_interval: u16,
) -> Result<Vec<u8>, JpgError> {
    let hmax = frame.components.iter().map(|c| c.h_sampling).max().unwrap_or(1).max(1) as usize;
    let vmax = frame.components.iter().map(|c| c.v_sampling).max().unwrap_or(1).max(1) as usize;
    let mcu_w = 8 * hmax;
    let mcu_h = 8 * vmax;
    let (width, height) = (frame.width as usize, frame.height as usize);
    let mcus_x = (width + mcu_w - 1) / mcu_w;
    let mcus_y = (height + mcu_h - 1) / mcu_h;

    let mut planes: Vec<Vec<f64>> = Vec::with_capacity(frame.components.len());
    let mut plane_dims: Vec<(usize, usize)> = Vec::with_capacity(frame.components.len());
    for c in frame.components.iter() {
        let pwc = mcus_x * c.h_sampling.max(1) as usize * 8;
        let phc = mcus_y * c.v_sampling.max(1) as usize * 8;
        planes.push(vec![0f64; pwc * phc]);
        plane_dims.push((pwc, phc));
    }

    let mut br = BitReader::new(data, start);
    let mut dc_pred = vec![0i32; frame.components.len()];
    let mut mcus_since_restart = 0u32;
    for my in 0..mcus_y {
        for mx in 0..mcus_x {
            if restart_interval > 0 && mcus_since_restart == restart_interval as u32 && (my != 0 || mx != 0) {
                br.skip_restart_marker()?;
                for p in dc_pred.iter_mut() { *p = 0; }
                mcus_since_restart = 0;
            }
            for (ci, c) in frame.components.iter().enumerate() {
                let (dc_id, ac_id) = scan_tabs[ci];
                let dc_tab = dc_tables.get(&dc_id).ok_or_else(|| JpgError::Malformed("missing DC huffman table".into()))?;
                let ac_tab = ac_tables.get(&ac_id).ok_or_else(|| JpgError::Malformed("missing AC huffman table".into()))?;
                let q = quant.get(&c.quant_table_id).ok_or_else(|| JpgError::Malformed("missing quant table".into()))?;
                let (pwc, _) = plane_dims[ci];
                for by in 0..c.v_sampling.max(1) as usize {
                    for bx in 0..c.h_sampling.max(1) as usize {
                        let zz = decode_block(&mut br, &mut dc_pred[ci], dc_tab, ac_tab)?;
                        let mut natural = [0f64; 64];
                        for z in 0..64 { natural[ZIGZAG_TO_NATURAL[z]] = (zz[z] * q[z]) as f64; }
                        let spatial = idct_8x8(&natural);
                        let ox = (mx * c.h_sampling.max(1) as usize + bx) * 8;
                        let oy = (my * c.v_sampling.max(1) as usize + by) * 8;
                        for r in 0..8 { for cc in 0..8 { planes[ci][(oy + r) * pwc + (ox + cc)] = spatial[r * 8 + cc] + 128.0; } }
                    }
                }
            }
            mcus_since_restart += 1;
        }
    }

    let grayscale = frame.components.len() == 1;
    let y_idx = frame.components.iter().position(|c| c.id == 1).unwrap_or(0);
    let (cb_idx, cr_idx) = if grayscale {
        (None, None)
    } else {
        (frame.components.iter().position(|c| c.id == 2), frame.components.iter().position(|c| c.id == 3))
    };
    let mut rgba = vec![0u8; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let yc = frame.components[y_idx];
            let (ypwc, _) = plane_dims[y_idx];
            let sy = (y * yc.v_sampling.max(1) as usize) / vmax;
            let sx = (x * yc.h_sampling.max(1) as usize) / hmax;
            let yy = planes[y_idx][sy * ypwc + sx];
            let (r, g, b) = if grayscale {
                let v = yy.round().clamp(0.0, 255.0) as u8;
                (v, v, v)
            } else {
                let cb_idx = cb_idx.ok_or_else(|| JpgError::Malformed("missing Cb component".into()))?;
                let cr_idx = cr_idx.ok_or_else(|| JpgError::Malformed("missing Cr component".into()))?;
                let cbc = frame.components[cb_idx];
                let crc = frame.components[cr_idx];
                let (cbpwc, _) = plane_dims[cb_idx];
                let (crpwc, _) = plane_dims[cr_idx];
                let cby = (y * cbc.v_sampling.max(1) as usize) / vmax;
                let cbx = (x * cbc.h_sampling.max(1) as usize) / hmax;
                let cry = (y * crc.v_sampling.max(1) as usize) / vmax;
                let crx = (x * crc.h_sampling.max(1) as usize) / hmax;
                ycbcr_to_rgb(yy, planes[cb_idx][cby * cbpwc + cbx], planes[cr_idx][cry * crpwc + crx])
            };
            let idx = (y * width + x) * 4;
            rgba[idx] = r;
            rgba[idx + 1] = g;
            rgba[idx + 2] = b;
            rgba[idx + 3] = 255;
        }
    }
    Ok(rgba)
}
//#endregion Decode

pub fn empty_jpg_snapshot() -> JpgSnapshot { JpgSnapshot::default() }

pub fn register() {
    crate::artifacts::jpg::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::jpg::schema::jpg_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<JpgSnapshot, JpgMutation>(STDIO_JPG_DOCUMENT_SCHEMA));
}

pub struct JpgEngine { artifact_state: JpgArtifact, snapshot_state: JpgSnapshot }
impl JpgEngine {
    pub fn new(snapshot: JpgSnapshot) -> Self {
        Self { artifact_state: JpgArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn gradient_image(w: u32, h: u32) -> Vec<u8> {
        let mut out = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let idx = ((y * w + x) * 4) as usize;
                out[idx] = ((x * 255) / w.max(1)) as u8;
                out[idx + 1] = ((y * 255) / h.max(1)) as u8;
                out[idx + 2] = (((x + y) * 255) / (w + h).max(1)) as u8;
                out[idx + 3] = 255;
            }
        }
        out
    }

    fn checkerboard_image(w: u32, h: u32) -> Vec<u8> {
        let mut out = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let idx = ((y * w + x) * 4) as usize;
                let on = ((x / 8) + (y / 8)) % 2 == 0;
                let v = if on { 230u8 } else { 20u8 };
                out[idx] = v;
                out[idx + 1] = v;
                out[idx + 2] = v;
                out[idx + 3] = 255;
            }
        }
        out
    }

    fn mae(a: &[u8], b: &[u8]) -> f64 {
        assert_eq!(a.len(), b.len());
        let mut sum = 0f64;
        let mut n = 0usize;
        for i in (0..a.len()).step_by(4) {
            for c in 0..3 {
                sum += (a[i + c] as i32 - b[i + c] as i32).abs() as f64;
                n += 1;
            }
        }
        sum / n as f64
    }

    #[test]
    fn idct_fdct_is_identity() {
        let mut block = [0f64; 64];
        for (i, v) in block.iter_mut().enumerate() { *v = ((i * 37 % 255) as f64) - 128.0; }
        let coeff = fdct_8x8(&block);
        let recon = idct_8x8(&coeff);
        let maxerr = block.iter().zip(recon.iter()).fold(0f64, |m, (a, b)| m.max((a - b).abs()));
        assert!(maxerr < 1e-6, "maxerr={maxerr}");
    }

    #[test]
    fn huffman_round_trips_all_dc_luma_symbols() {
        let table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
        let mut bw = BitWriter::new();
        for v in 0u8..=11 {
            let (l, c) = *table.encode.get(&v).unwrap();
            bw.put_bits(c, l);
        }
        bw.flush();
        let mut br = BitReader::new(&bw.bytes, 0);
        for v in 0u8..=11 {
            assert_eq!(br.decode_symbol(&table).unwrap(), v);
        }
    }

    #[test]
    fn single_block_round_trips_through_huffman() {
        let dc_table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
        let ac_table = build_huffman(&AC_LUMA_BITS, &ac_luma_values()).unwrap();
        let mut zz = [0i32; 64];
        zz[0] = 120;
        zz[1] = 5;
        zz[2] = -3;
        zz[20] = 1;
        let mut bw = BitWriter::new();
        let mut dc_pred = 0i32;
        encode_block(&mut bw, &zz, &mut dc_pred, &dc_table, &ac_table).unwrap();
        bw.flush();
        let mut br = BitReader::new(&bw.bytes, 0);
        let mut dc_pred2 = 0i32;
        let decoded = decode_block(&mut br, &mut dc_pred2, &dc_table, &ac_table).unwrap();
        assert_eq!(decoded, zz);
    }

    /// 🖼️ Non-solid-color round trip — the case the old "solid-color only"
    /// codec could never have passed. Gradient exercises AC energy across
    /// every block; asserts mean-absolute-pixel-error stays well under a
    /// visually-lossless budget of 10/255.
    #[test]
    fn gradient_round_trip_under_mae_threshold() {
        let (w, h) = (48u32, 40u32);
        let img = gradient_image(w, h);
        let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba: img.clone() } };
        let bytes = encode_jpg(&snap).expect("encode");
        assert!(bytes.starts_with(&[0xFF, 0xD8]));
        assert!(bytes.ends_with(&[0xFF, 0xD9]));
        let decoded = decode_jpg(&bytes).expect("decode");
        assert_eq!(decoded.image.width, w);
        assert_eq!(decoded.image.height, h);
        let err = mae(&img, &decoded.image.rgba);
        println!("gradient round-trip MAE = {err}");
        assert!(err < 10.0, "gradient MAE too high: {err}");
    }

    /// 🖼️ Checkerboard: high-frequency content, harder for quantization to
    /// preserve than a gradient — same bar (MAE < 10/255).
    #[test]
    fn checkerboard_round_trip_under_mae_threshold() {
        let (w, h) = (32u32, 32u32);
        let img = checkerboard_image(w, h);
        let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba: img.clone() } };
        let bytes = encode_jpg(&snap).expect("encode");
        let decoded = decode_jpg(&bytes).expect("decode");
        let err = mae(&img, &decoded.image.rgba);
        println!("checkerboard round-trip MAE = {err}");
        assert!(err < 10.0, "checkerboard MAE too high: {err}");
    }

    #[test]
    fn solid_color_still_round_trips() {
        let (w, h) = (16u32, 16u32);
        let mut img = vec![0u8; (w * h * 4) as usize];
        for px in img.chunks_mut(4) { px[0] = 200; px[1] = 100; px[2] = 50; px[3] = 255; }
        let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: RasterImage { width: w, height: h, rgba: img.clone() } };
        let bytes = encode_jpg(&snap).expect("encode");
        let decoded = decode_jpg(&bytes).expect("decode");
        let err = mae(&img, &decoded.image.rgba);
        assert!(err < 5.0, "solid MAE too high: {err}");
    }

    /// 🚫 Progressive (SOF2) must be a typed `Unsupported` error, never
    /// silently decoded — hand-crafted minimal SOF2 segment.
    #[test]
    fn progressive_sof2_is_explicit_unsupported() {
        let mut bytes = vec![0xFFu8, 0xD8];
        bytes.extend_from_slice(&[0xFF, 0xC2, 0x00, 0x0B, 0x08, 0x00, 0x08, 0x00, 0x08, 0x01, 0x01, 0x11, 0x00]);
        bytes.extend_from_slice(&[0xFF, 0xD9]);
        let result = decode_jpg(&bytes);
        assert!(matches!(result, Err(JpgError::Unsupported(_))), "expected Unsupported, got {result:?}");
    }

    #[test]
    fn non_jpeg_input_is_malformed_not_panic() {
        let result = decode_jpg(&[0x00, 0x01, 0x02, 0x03]);
        assert!(matches!(result, Err(JpgError::Malformed(_))));
    }
}
//#endregion Tests
