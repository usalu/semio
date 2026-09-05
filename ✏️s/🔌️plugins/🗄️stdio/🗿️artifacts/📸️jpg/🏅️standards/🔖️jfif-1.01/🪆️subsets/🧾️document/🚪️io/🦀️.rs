//! 🚪️ IO stdio.jpg (jfif-1.01/🧾️document) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::document::schema::JpgAnalyzer;
    use crate::artifacts::jpg::JpgSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct JpgComposerComposition;

    impl ArtifactComposition for JpgComposerComposition {
        type Snapshot = JpgSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "JpgComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = JpgAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "JpgComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the
// real baseline-sequential (SOF0) JPEG codec — Huffman entropy decoding, dequantization,
// integer-ish separable IDCT, YCbCr→RGB with nearest-neighbor chroma upsampling — relocated here
// verbatim (destination rule 2: codecs → `🚪️io/`; rule 6: pure format algorithms with no
// snapshot dependency stay WITH the codec here rather than promoting to a module engine, since
// they're JPEG-specific, not artifact-independent). Progressive/arithmetic/lossless SOFn variants
// are explicit `JpgError::Unsupported`, never decoded as garbage. `JpgEngine` (zero construction
// sites) and the dead `register`/`register_pilot_languages`/`register_artifact_inferences`/
// `register_schema_specs` cluster (superseded by `declaration()` in the artifact root, zero real
// callers) were deleted outright, not relocated. `empty_jpg_snapshot`/`demo_jpg_snapshot` moved to
// `../🧬️schema` (pure helpers over the document type).
use crate::artifacts::jpg::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass, JpgHuffmanTable, JpgQuantTable, JpgScanComponent, JpgSegment};
use crate::artifacts::jpg::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};
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
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];
//#endregion ZigZag

//#region Idct
/// 📐 Separable 1D IDCT-8 (ITU T.81 A.3.3), applied row-then-column for the
/// 2D block transform — O(N^2) per axis instead of the O(N^4) direct sum.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
fn fdct_8x8(block: &[f64; 64]) -> [f64; 64] {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_huffman(bits: &[u8; 16], values: &[u8]) -> Result<HuffTable, JpgError> {
    let mut sizes: Vec<u8> = Vec::new();
    for (l, &count) in bits.iter().enumerate() {
        for _ in 0..count {
            sizes.push((l + 1) as u8);
        }
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new() -> Self {
        Self { bytes: Vec::new(), acc: 0, nbits: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn put_bits(&mut self, value: u16, len: u8) {
        if len == 0 {
            return;
        }
        self.acc = (self.acc << len) | (value as u32 & ((1u32 << len) - 1));
        self.nbits += len as u32;
        while self.nbits >= 8 {
            self.nbits -= 8;
            let byte = ((self.acc >> self.nbits) & 0xFF) as u8;
            self.bytes.push(byte);
            if byte == 0xFF {
                self.bytes.push(0x00);
            }
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn flush(&mut self) {
        if self.nbits > 0 {
            let pad = 8 - self.nbits;
            let byte = ((self.acc << pad) & 0xFF) as u8;
            self.bytes.push(byte);
            if byte == 0xFF {
                self.bytes.push(0x00);
            }
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
    data: &'a dyn JpgByteSource,
    pos: usize,
    acc: u32,
    nbits: u32,
}
impl<'a> BitReader<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(data: &'a dyn JpgByteSource, pos: usize) -> Self {
        Self { data, pos, acc: 0, nbits: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn next_byte(&mut self) -> Option<u8> {
        if self.pos >= self.data.len() {
            return None;
        }
        let b = self.data.byte(self.pos)?;
        if b == 0xFF {
            let b2 = self.data.byte(self.pos.checked_add(1)?).unwrap_or(0);
            if b2 == 0x00 {
                self.pos += 2;
                return Some(0xFF);
            }
            return None;
        }
        self.pos += 1;
        Some(b)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_bit(&mut self) -> Result<u8, JpgError> {
        if self.nbits == 0 {
            match self.next_byte() {
                Some(b) => {
                    self.acc = b as u32;
                    self.nbits = 8;
                }
                None => return Err(JpgError::Malformed("unexpected marker inside entropy-coded segment".into())),
            }
        }
        self.nbits -= 1;
        Ok(((self.acc >> self.nbits) & 1) as u8)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_bits(&mut self, n: u8) -> Result<u16, JpgError> {
        let mut v = 0u16;
        for _ in 0..n {
            v = (v << 1) | self.read_bit()? as u16;
        }
        Ok(v)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn decode_symbol(&mut self, table: &HuffTable) -> Result<u8, JpgError> {
        let mut code: u16 = 0;
        for len in 1..=table.max_len {
            code = (code << 1) | self.read_bit()? as u16;
            if let Some(v) = table.decode.get(&(len, code)) {
                return Ok(*v);
            }
        }
        Err(JpgError::Malformed("huffman decode: no matching code".into()))
    }
    /// 🔁 Byte-align and consume one `RSTn` marker at a restart boundary;
    /// also resets the DC predictors (caller's responsibility) per T.81 F.2.2.5.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn skip_restart_marker(&mut self) -> Result<(), JpgError> {
        self.nbits = 0;
        self.acc = 0;
        let Some(next) = self.pos.checked_add(1) else { return Err(JpgError::Malformed("restart cursor overflow".into())) };
        if next < self.data.len() && self.data.byte(self.pos) == Some(0xFF) && self.data.byte(next).is_some_and(|marker| (0xD0..=0xD7).contains(&marker)) {
            self.pos += 2;
            Ok(())
        } else {
            Err(JpgError::Malformed("expected restart marker not found".into()))
        }
    }
}

/// ➕ Sign-extends a JPEG-encoded magnitude/sign pair (T.81 F.12): values
/// below `2^(size-1)` are negative, encoded as `value - (2^size - 1)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn extend_sign(value: u16, size: u8) -> i32 {
    if size == 0 {
        return 0;
    }
    let v = value as i32;
    let vt = 1i32 << (size - 1);
    if v < vt {
        v - (1 << size) + 1
    } else {
        v
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn size_of(mut v: i32) -> u8 {
    if v < 0 {
        v = -v;
    }
    let mut s = 0u8;
    while v > 0 {
        s += 1;
        v >>= 1;
    }
    s
}
//#endregion BitIo

//#region BlockCodec
/// 🧱 Encodes one 8x8 block's already-quantized zigzag coefficients: DC as a
/// difference from the running per-component predictor, AC via run-length +
/// size Huffman symbols with ZRL (0xF0) for 16-zero runs and EOB (0x00) once
/// the remainder is all zero.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
        if v == 0 {
            run += 1;
            continue;
        }
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_block(br: &mut BitReader<'_>, dc_pred: &mut i32, dc_table: &HuffTable, ac_table: &HuffTable) -> Result<[i32; 64], JpgError> {
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
            if run == 15 {
                z += 16;
                continue;
            } // ZRL
            break; // EOB
        }
        z += run as usize;
        if z >= 64 {
            return Err(JpgError::Malformed("ac coefficient run overruns block".into()));
        }
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
    16, 11, 10, 16, 24, 40, 51, 61, 12, 12, 14, 19, 26, 58, 60, 55, 14, 13, 16, 24, 40, 57, 69, 56, 14, 17, 22, 29, 51, 87, 80, 62, 18, 22, 37, 56, 68, 109, 103, 77, 24, 35, 55, 64, 81, 104, 113, 92, 49, 64, 78, 87, 103, 121, 120, 101, 72, 92, 95,
    98, 112, 100, 103, 99,
];
/// 📊 Annex K.1 example chrominance quantization table (natural order).
const STD_CHROMA_Q: [i32; 64] = [
    17, 18, 24, 47, 99, 99, 99, 99, 18, 21, 26, 66, 99, 99, 99, 99, 24, 26, 56, 99, 99, 99, 99, 99, 47, 66, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99,
];

/// 📈 IJG-standard quality→scale mapping applied to the Annex K base tables.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn quant_zigzag(natural: &[i32; 64]) -> [i32; 64] {
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
const DC_LUMA_BITS: [u8; 16] = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dc_luma_values() -> Vec<u8> {
    (0..=11).collect()
}
const DC_CHROMA_BITS: [u8; 16] = [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dc_chroma_values() -> Vec<u8> {
    (0..=11).collect()
}
const AC_LUMA_BITS: [u8; 16] = [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 0x7d];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ac_luma_values() -> Vec<u8> {
    vec![
        0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08, 0x23, 0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16,
        0x17, 0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
        0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6,
        0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
        0xf9, 0xfa,
    ]
}
const AC_CHROMA_BITS: [u8; 16] = [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 0x77];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ac_chroma_values() -> Vec<u8> {
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
fn rgb_to_ycbcr(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let (r, g, b) = (r as f64, g as f64, b as f64);
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let cb = -0.168736 * r - 0.331264 * g + 0.5 * b + 128.0;
    let cr = 0.5 * r - 0.418688 * g - 0.081312 * b + 128.0;
    (y, cb, cr)
}
/// 🎨 ITU-R BT.601 YCbCr→RGB, clamped to `0..=255`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn box_downsample(src: &[f64], sw: usize, sh: usize, fx: usize, fy: usize) -> (Vec<f64>, usize, usize) {
    let dw = sw / fx;
    let dh = sh / fy;
    let mut out = vec![0f64; dw * dh];
    for y in 0..dh {
        for x in 0..dw {
            let mut sum = 0f64;
            for dy in 0..fy {
                for dx in 0..fx {
                    sum += src[(y * fy + dy) * sw + (x * fx + dx)];
                }
            }
            out[y * dw + x] = sum / (fx * fy) as f64;
        }
    }
    (out, dw, dh)
}

/// 🏷️ Builds a real `APP0`/`JFIF\0` segment (ITU-T T.871 §) from `snap.jfif_*`, including an
/// embedded thumbnail when present.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_jfif_app0(snap: &JpgSnapshot) -> Vec<u8> {
    let thumb = snap.jfif_thumbnail.as_ref();
    let (tw, th, tdata): (u8, u8, &[u8]) = match thumb {
        Some(t) => (t.width, t.height, &t.rgb_data),
        None => (0, 0, &[]),
    };
    let len = 2 + 5 + 2 + 1 + 2 + 2 + 1 + 1 + tdata.len();
    let mut out = vec![0xFFu8, 0xE0, (len >> 8) as u8, (len & 0xFF) as u8];
    out.extend_from_slice(b"JFIF\0");
    out.push(snap.jfif_version.0);
    out.push(snap.jfif_version.1);
    out.push(snap.jfif_density_units.to_u8());
    out.push((snap.jfif_x_density >> 8) as u8);
    out.push((snap.jfif_x_density & 0xFF) as u8);
    out.push((snap.jfif_y_density >> 8) as u8);
    out.push((snap.jfif_y_density & 0xFF) as u8);
    out.push(tw);
    out.push(th);
    out.extend_from_slice(tdata);
    out
}

/// 📐️ The SOF0 component list this encoder writes: the decoded frame's OWN ids and sampling
/// factors when the snapshot carries a frame, so a 4:4:4 document stays 4:4:4 across a
/// decode/re-encode, and the historical `1:2x2, 2:1x1, 3:1x1` default when it does not.
///
/// The quantization-table selector is NOT carried through: this encoder always emits exactly two
/// fresh Annex K DQT tables (0 luma, 1 chroma), so the first component is bound to 0 and every
/// other to 1 — the same canonicalization `encode_jpg`'s own doc comment already declares for the
/// DQT/DHT tables themselves. A sampling factor outside T.81 §B.2.2's 1..=4, or one that does not
/// divide the frame's maximum, is refused rather than silently rounded: both would make the MCU
/// geometry unrepresentable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frame_components_of(snap: &JpgSnapshot) -> Result<Vec<JpgFrameComponent>, JpgError> {
    let declared = snap.frame.as_ref().map(|frame| frame.components.clone()).unwrap_or_default();
    if declared.is_empty() {
        return Ok(vec![
            JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
            JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
            JpgFrameComponent { id: 3, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
        ]);
    }
    if declared.len() != 1 && declared.len() != 3 {
        return Err(JpgError::Unsupported(format!("this encoder writes a single-component grayscale frame or a three-component Y/Cb/Cr frame; the snapshot declares {}", declared.len())));
    }
    for component in &declared {
        if !(1..=4).contains(&component.h_sampling) || !(1..=4).contains(&component.v_sampling) {
            return Err(JpgError::Unsupported(format!("T.81 B.2.2 confines a component's sampling factors to 1..=4; component {} declares {}x{}", component.id, component.h_sampling, component.v_sampling)));
        }
    }
    let hmax = declared.iter().map(|component| component.h_sampling).max().unwrap_or(1);
    let vmax = declared.iter().map(|component| component.v_sampling).max().unwrap_or(1);
    for component in &declared {
        if hmax % component.h_sampling != 0 || vmax % component.v_sampling != 0 {
            return Err(JpgError::Unsupported(format!("component {}'s {}x{} sampling does not divide the frame maximum {hmax}x{vmax}, so its plane has no integral resolution", component.id, component.h_sampling, component.v_sampling)));
        }
    }
    Ok(declared
        .into_iter()
        .enumerate()
        .map(|(index, component)| JpgFrameComponent { quant_table_id: if index == 0 { 0 } else { 1 }, ..component })
        .collect())
}

/// 🖨️ Encodes an RGBA raster as baseline sequential JPEG (Y/Cb/Cr, ids 1/2/3, or a single Y
/// component for a grayscale frame), Annex K example tables scaled by `snap.re_encode_quality`
/// (IJG convention, default 90) — chosen so the round trip through our own decoder stays well
/// under a visually-lossless error budget. Edges are replicated (not zero-padded) up to the next
/// MCU boundary to avoid ringing. Writes a real JFIF APP0 from `snap.jfif_*` and re-emits
/// `snap.other_segments` verbatim right after it — always canonicalizes to fresh Annex K DQT/DHT
/// tables at the chosen quality (documented normal form, matches png's pixel-canonicalization
/// precedent: `quant_tables`/`huffman_tables` are typed RETENTION of a decoded file's actual
/// tables, not necessarily what a subsequent re-encode emits) — `restart_interval` is retained but
/// this encoder never emits `DRI`/restart markers (documented deviation, `## deviations`).
///
/// 📐️ The SOF0 SAMPLING FACTORS come from `snap.frame`, not from a fixed 4:2:0 choice. T.81 §B.2.2
/// makes `H`/`V` per-component frame parameters in 1..=4, and the 🧱️baseline subset's
/// `check_baseline_conformance` reads them as one of its five class axes — so an encoder that
/// stamped every frame 4:2:0 was silently resampling the chroma of every 4:4:4 document it
/// re-serialized and moving a conformance axis while doing it. Each component's plane is box-
/// filtered by `(hmax / h, vmax / v)` and emitted as `h * v` blocks per MCU, which reduces to the
/// previous behaviour exactly when the frame really is `1:2x2, 2:1x1, 3:1x1`. A frame with no
/// components at all (a snapshot that was never decoded from a real file) keeps that 4:2:0 default,
/// since there is nothing to honour.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_jpg(snap: &JpgSnapshot) -> Result<Vec<u8>, JpgError> {
    if snap.width == 0 || snap.height == 0 {
        return Err(JpgError::Malformed("empty image".into()));
    }
    if snap.pixels.len() != (snap.width as usize) * (snap.height as usize) * 4 {
        return Err(JpgError::Malformed("pixels length mismatch".into()));
    }
    if snap.width > u16::MAX as u32 || snap.height > u16::MAX as u32 {
        return Err(JpgError::Unsupported("image dimensions exceed JPEG's 16-bit SOF0 width/height field".into()));
    }
    let (width, height): (u16, u16) = (snap.width as u16, snap.height as u16);
    let quality = snap.re_encode_quality.map(|q| q as i32).unwrap_or(90);
    let comps = frame_components_of(snap)?;
    let hmax = comps.iter().map(|c| c.h_sampling as usize).max().unwrap_or(1);
    let vmax = comps.iter().map(|c| c.v_sampling as usize).max().unwrap_or(1);
    let mcu_w = 8 * hmax;
    let mcu_h = 8 * vmax;
    let mcus_x = (width as usize + mcu_w - 1) / mcu_w;
    let mcus_y = (height as usize + mcu_h - 1) / mcu_h;
    let pw = mcus_x * mcu_w;
    let ph = mcus_y * mcu_h;

    let mut full: Vec<Vec<f64>> = vec![vec![0f64; pw * ph]; comps.len()];
    for y in 0..ph {
        let sy = y.min(height as usize - 1);
        for x in 0..pw {
            let sx = x.min(width as usize - 1);
            let idx = (sy * width as usize + sx) * 4;
            let (yy, cb, cr) = rgb_to_ycbcr(snap.pixels[idx], snap.pixels[idx + 1], snap.pixels[idx + 2]);
            let channels = [yy, cb, cr];
            for (plane, value) in full.iter_mut().zip(channels) {
                plane[y * pw + x] = value;
            }
        }
    }
    // 📐️ One plane per component at that component's own resolution: `hmax / h` by `vmax / v`
    // box-filtered. A component already at the frame maximum keeps its full-resolution buffer by
    // MOVE rather than being box-filtered 1:1 into a second copy of it — on a 2275x2560 scan that
    // is 46 MB of `f64` per component not allocated twice.
    let planes: Vec<(Vec<f64>, usize)> = comps
        .iter()
        .zip(full)
        .map(|(component, source)| {
            let (fx, fy) = (hmax / component.h_sampling as usize, vmax / component.v_sampling as usize);
            if fx == 1 && fy == 1 {
                return (source, pw);
            }
            let (plane, plane_width, _) = box_downsample(&source, pw, ph, fx, fy);
            (plane, plane_width)
        })
        .collect();

    let frame = JpgFrameHeader { precision: 8, width, height, components: comps.clone() };

    let luma_q = quant_zigzag(&scale_quality(&STD_LUMA_Q, quality));
    let chroma_q = quant_zigzag(&scale_quality(&STD_CHROMA_Q, quality));
    let dc_luma = build_huffman(&DC_LUMA_BITS, &dc_luma_values())?;
    let ac_luma = build_huffman(&AC_LUMA_BITS, &ac_luma_values())?;
    let dc_chroma = build_huffman(&DC_CHROMA_BITS, &dc_chroma_values())?;
    let ac_chroma = build_huffman(&AC_CHROMA_BITS, &ac_chroma_values())?;

    let mut out = Vec::new();
    out.extend_from_slice(&[0xFF, 0xD8]); // SOI
    out.extend_from_slice(&encode_jfif_app0(snap));
    for seg in &snap.other_segments {
        out.push(0xFF);
        out.push(seg.marker);
        let len = seg.data.len() + 2;
        out.push((len >> 8) as u8);
        out.push((len & 0xFF) as u8);
        out.extend_from_slice(&seg.data);
    }

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

    let scan_comps: Vec<JpgScanComponent> =
        comps.iter().enumerate().map(|(index, component)| JpgScanComponent { id: component.id, dc_table_id: if index == 0 { 0 } else { 1 }, ac_table_id: if index == 0 { 0 } else { 1 } }).collect();
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
    let mut dc_pred = vec![0i32; comps.len()];
    for my in 0..mcus_y {
        for mx in 0..mcus_x {
            // 🧩️ T.81 §A.2.3 MCU interleave: every component contributes `h * v` blocks per MCU, in
            // component order, row-major within the component — which is one block for a 1x1
            // component and four for a 2x2 one.
            for (index, component) in comps.iter().enumerate() {
                let (plane, plane_width) = &planes[index];
                let (table_q, dc_table, ac_table) = if index == 0 { (&luma_q, &dc_luma, &ac_luma) } else { (&chroma_q, &dc_chroma, &ac_chroma) };
                for by in 0..component.v_sampling as usize {
                    for bx in 0..component.h_sampling as usize {
                        let ox = (mx * component.h_sampling as usize + bx) * 8;
                        let oy = (my * component.v_sampling as usize + by) * 8;
                        let mut block = [0f64; 64];
                        for r in 0..8 {
                            for c in 0..8 {
                                block[r * 8 + c] = plane[(oy + r) * plane_width + (ox + c)] - 128.0;
                            }
                        }
                        let coeff = fdct_8x8(&block);
                        let mut zz = [0i32; 64];
                        for z in 0..64 {
                            zz[z] = (coeff[ZIGZAG_TO_NATURAL[z]] / table_q[z] as f64).round() as i32;
                        }
                        encode_block(&mut bw, &zz, &mut dc_pred[index], dc_table, ac_table)?;
                    }
                }
            }
        }
    }
    bw.flush();
    out.extend_from_slice(&bw.bytes);
    out.extend_from_slice(&[0xFF, 0xD9]); // EOI
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
/// 🏷️ Attempts to parse `seg` (the APP0 segment body, i.e. everything after the 2-byte length
/// field) as a real JFIF header (ITU-T T.871 §: 5-byte `"JFIF\0"` identifier + version + units +
/// x/y density + thumbnail dims + optional embedded RGB thumbnail). `None` if the identifier
/// doesn't match — a non-JFIF APP0 (e.g. a bare Exif/other APP0) is retained verbatim in
/// `other_segments` instead by the caller.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_jfif_app0(seg: &[u8]) -> Option<(JfifVersion, JfifDensityUnits, u16, u16, Option<JfifThumbnail>)> {
    if seg.len() < 14 || &seg[0..5] != b"JFIF\0" {
        return None;
    }
    let version = (seg[5], seg[6]);
    let units = JfifDensityUnits::from_u8(seg[7]).ok()?;
    let x_density = ((seg[8] as u16) << 8) | seg[9] as u16;
    let y_density = ((seg[10] as u16) << 8) | seg[11] as u16;
    let tw = seg[12];
    let th = seg[13];
    let need = 3 * tw as usize * th as usize;
    let thumbnail = if tw > 0 && th > 0 && seg.len() >= 14 + need { Some(JfifThumbnail { width: tw, height: th, rgb_data: seg[14..14 + need].to_vec() }) } else { None };
    Some((version, units, x_density, y_density, thumbnail))
}
type JfifVersion = (u8, u8);

/// 🧩️ Random-access compressed JPEG source; implementations may be chunk ropes and need
/// not join the complete compressed input into a contiguous allocation.
pub trait JpgByteSource {
    fn len(&self) -> usize;
    fn byte(&self, index: usize) -> Option<u8>;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl JpgByteSource for [u8] {
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    fn byte(&self, index: usize) -> Option<u8> {
        self.get(index).copied()
    }
}

impl JpgByteSource for &[u8] {
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    fn byte(&self, index: usize) -> Option<u8> {
        self.get(index).copied()
    }
}

fn source_range(data: &dyn JpgByteSource, at: usize, len: usize) -> Result<Vec<u8>, JpgError> {
    let end = at.checked_add(len).ok_or_else(|| JpgError::Malformed("segment range overflow".into()))?;
    if end > data.len() {
        return Err(JpgError::Malformed("segment out of bounds".into()));
    }
    (at..end).map(|index| data.byte(index).ok_or_else(|| JpgError::Malformed("segment out of bounds".into()))).collect()
}

/// 📥 Decodes baseline sequential JPEG (SOF0 only) into an RGBA raster.
/// Any other SOFn marker (progressive/extended/lossless/arithmetic) is a
/// typed `JpgError::Unsupported` naming the exact variant — never decoded.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_jpg(data: &[u8]) -> Result<JpgSnapshot, JpgError> {
    decode_jpg_source(&data)
}

/// 🧩️ Decodes a baseline JPEG directly from a bounded random-access chunk source.
pub fn decode_jpg_source(data: &dyn JpgByteSource) -> Result<JpgSnapshot, JpgError> {
    if data.len() < 4 || data.byte(0) != Some(0xFF) || data.byte(1) != Some(0xD8) {
        return Err(JpgError::Malformed("missing SOI".into()));
    }
    let mut i = 2usize;
    let mut quant: HashMap<u8, [i32; 64]> = HashMap::new();
    let mut dc_tables: HashMap<u8, HuffTable> = HashMap::new();
    let mut ac_tables: HashMap<u8, HuffTable> = HashMap::new();
    let mut quant_tables: Vec<JpgQuantTable> = Vec::new();
    let mut huffman_tables: Vec<JpgHuffmanTable> = Vec::new();
    let mut other_segments: Vec<JpgSegment> = Vec::new();
    let mut frame: Option<JpgFrameHeader> = None;
    let mut sof_marker: u8 = 0;
    let mut restart_interval_raw = 0u16;
    let mut restart_interval: Option<u16> = None;
    let mut jfif_version: JfifVersion = (1, 1);
    let mut jfif_density_units = JfifDensityUnits::Aspect;
    let mut jfif_x_density: u16 = 1;
    let mut jfif_y_density: u16 = 1;
    let mut jfif_thumbnail: Option<JfifThumbnail> = None;

    loop {
        if i + 1 >= data.len() {
            return Err(JpgError::Malformed("truncated before EOI".into()));
        }
        if data.byte(i) != Some(0xFF) {
            i += 1;
            continue;
        }
        let marker = data.byte(i.checked_add(1).ok_or_else(|| JpgError::Malformed("marker cursor overflow".into()))?).ok_or_else(|| JpgError::Malformed("truncated marker".into()))?;
        i += 2;
        match marker {
            0xD8 => continue, // stray SOI, tolerate
            0xD9 => return Err(JpgError::Malformed("EOI before SOS".into())),
            0xC0 => {
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, len.saturating_sub(2))?;
                if seg.len() < 6 {
                    return Err(JpgError::Malformed("SOF0 segment too short".into()));
                }
                let height = ((seg[1] as u16) << 8) | seg[2] as u16;
                let width = ((seg[3] as u16) << 8) | seg[4] as u16;
                let nc = seg[5] as usize;
                let mut components = Vec::with_capacity(nc);
                for k in 0..nc {
                    let base = 6 + k * 3;
                    if base + 2 >= seg.len() {
                        return Err(JpgError::Malformed("SOF0 component list truncated".into()));
                    }
                    components.push(JpgFrameComponent { id: seg[base], h_sampling: seg[base + 1] >> 4, v_sampling: seg[base + 1] & 0x0F, quant_table_id: seg[base + 2] });
                }
                frame = Some(JpgFrameHeader { precision: seg[0], width, height, components });
                sof_marker = marker;
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
                    if p >= data.len() {
                        return Err(JpgError::Malformed("DQT truncated".into()));
                    }
                    let table_info = data.byte(p).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                    let pq = table_info >> 4;
                    let tq = table_info & 0x0F;
                    p += 1;
                    let mut tbl = [0i32; 64];
                    let mut tbl_u16 = [0u16; 64];
                    for (z, slot) in tbl.iter_mut().enumerate() {
                        if pq == 0 {
                            let v = data.byte(p).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                            *slot = v as i32;
                            tbl_u16[z] = v as u16;
                            p += 1;
                        } else {
                            let hi = data.byte(p).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                            let lo = data.byte(p.checked_add(1).ok_or_else(|| JpgError::Malformed("DQT cursor overflow".into()))?).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                            let v = ((hi as u16) << 8) | lo as u16;
                            *slot = v as i32;
                            tbl_u16[z] = v;
                            p += 2;
                        }
                    }
                    quant.insert(tq, tbl);
                    quant_tables.retain(|t| t.id != tq);
                    quant_tables.push(JpgQuantTable { id: tq, precision: pq, values: tbl_u16 });
                }
                i += len;
            }
            0xC4 => {
                let len = read_u16(data, i)?;
                let mut p = i + 2;
                let end = i + len;
                while p < end {
                    if p + 16 >= data.len() {
                        return Err(JpgError::Malformed("DHT truncated".into()));
                    }
                    let table_info = data.byte(p).ok_or_else(|| JpgError::Malformed("DHT truncated".into()))?;
                    let class = table_info >> 4;
                    let id = table_info & 0x0F;
                    p += 1;
                    let mut bits = [0u8; 16];
                    bits.copy_from_slice(&source_range(data, p, 16)?);
                    p += 16;
                    let count: usize = bits.iter().map(|&b| b as usize).sum();
                    let values = source_range(data, p, count)?;
                    p += count;
                    let table = build_huffman(&bits, &values)?;
                    let huffman_class = JpgHuffmanClass::from_u8(class).map_err(JpgError::Malformed)?;
                    if class == 0 {
                        dc_tables.insert(id, table);
                    } else {
                        ac_tables.insert(id, table);
                    }
                    huffman_tables.retain(|t| !(t.class == huffman_class && t.id == id));
                    huffman_tables.push(JpgHuffmanTable { id, class: huffman_class, bits, values });
                }
                i += len;
            }
            0xCC => {
                // 🚫 DAC (Define Arithmetic Coding conditioning) — its presence means the
                // entropy-coded scan needs arithmetic decoding, which this Huffman-only decoder
                // never implements (T.81 baseline sequential DCT is Huffman-only, Annex F). An
                // explicit `Unsupported` here (rather than falling through to the generic
                // "unhandled marker" `Malformed` case below) preserves the module's "never decode
                // arithmetic-coded data as Huffman garbage" invariant with a precise error, and
                // means `JpgSnapshot.arithmetic` is genuinely `false` for every snapshot this
                // engine ever returns `Ok` for (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
                return Err(JpgError::Unsupported("arithmetic coding conditioning (DAC present)".into()));
            }
            0xDD => {
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, 2)?;
                restart_interval_raw = ((seg[0] as u16) << 8) | seg[1] as u16;
                restart_interval = Some(restart_interval_raw);
                i += len;
            }
            0xDA => {
                let frame = frame.clone().ok_or_else(|| JpgError::Malformed("SOS before SOF0".into()))?;
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, len.saturating_sub(2))?;
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
                let rgba = decode_scan(data, i, &frame, &scan_tabs, &quant, &dc_tables, &ac_tables, restart_interval_raw)?;
                let (width, height) = (frame.width as u32, frame.height as u32);
                // 🏅️ sof_marker/arithmetic: real data the decode loop above already computed
                // transiently (the SOF0 marker byte, the DAC rejection above) — persisted here so
                // `subsets::baseline::analyzer::check_baseline_conformance`
                // (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES) has real fields
                // to check instead of an unmodeled gap. `dc_huffman_table_count`/
                // `ac_huffman_table_count` are now DERIVED from `huffman_tables` by the analyzer
                // (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION) —
                // no longer separately persisted, one source of truth.
                return Ok(JpgSnapshot {
                    schema: STDIO_JPG_DOCUMENT_SCHEMA.into(),
                    width,
                    height,
                    pixels: rgba,
                    re_encode_quality: None,
                    jfif_version,
                    jfif_density_units,
                    jfif_x_density,
                    jfif_y_density,
                    jfif_thumbnail,
                    frame: Some(frame),
                    sof_marker,
                    arithmetic: false,
                    quant_tables,
                    huffman_tables,
                    restart_interval,
                    other_segments,
                });
            }
            0xE0 => {
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, len.saturating_sub(2))?;
                match parse_jfif_app0(&seg) {
                    Some((version, units, xd, yd, thumb)) => {
                        jfif_version = version;
                        jfif_density_units = units;
                        jfif_x_density = xd;
                        jfif_y_density = yd;
                        jfif_thumbnail = thumb;
                    }
                    None => other_segments.push(JpgSegment { marker, data: seg }),
                }
                i += len;
            }
            0xE1..=0xEF | 0xFE => {
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, len.saturating_sub(2))?;
                other_segments.push(JpgSegment { marker, data: seg });
                i += len;
            }
            0x01 | 0xD0..=0xD7 => {} // TEM / stray restart outside a scan: no length field, skip
            _ => return Err(JpgError::Malformed(format!("unhandled marker 0xFF{marker:02X} before SOS"))),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_u16(data: &dyn JpgByteSource, at: usize) -> Result<usize, JpgError> {
    let hi = data.byte(at).ok_or_else(|| JpgError::Malformed("marker length truncated".into()))?;
    let lo = data.byte(at.checked_add(1).ok_or_else(|| JpgError::Malformed("marker length cursor overflow".into()))?).ok_or_else(|| JpgError::Malformed("marker length truncated".into()))?;
    Ok(((hi as usize) << 8) | lo as usize)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
/// 🎞️ Decodes the entropy-coded scan for all components (nearest-neighbor
/// chroma upsampling for subsampled components; grayscale skips color
/// conversion entirely) into RGBA.
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_scan(
    data: &dyn JpgByteSource,
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
                for p in dc_pred.iter_mut() {
                    *p = 0;
                }
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
                        for z in 0..64 {
                            natural[ZIGZAG_TO_NATURAL[z]] = (zz[z] * q[z]) as f64;
                        }
                        let spatial = idct_8x8(&natural);
                        let ox = (mx * c.h_sampling.max(1) as usize + bx) * 8;
                        let oy = (my * c.v_sampling.max(1) as usize + by) * 8;
                        for r in 0..8 {
                            for cc in 0..8 {
                                planes[ci][(oy + r) * pwc + (ox + cc)] = spatial[r * 8 + cc] + 128.0;
                            }
                        }
                    }
                }
            }
            mcus_since_restart += 1;
        }
    }

    let grayscale = frame.components.len() == 1;
    let y_idx = frame.components.iter().position(|c| c.id == 1).unwrap_or(0);
    let (cb_idx, cr_idx) = if grayscale { (None, None) } else { (frame.components.iter().position(|c| c.id == 2), frame.components.iter().position(|c| c.id == 3)) };
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

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jpg::schema::demo_jpg_snapshot;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    #[semio_framework_async_macros::async_test]
    async fn idct_fdct_is_identity() {
        let mut block = [0f64; 64];
        for (i, v) in block.iter_mut().enumerate() {
            *v = ((i * 37 % 255) as f64) - 128.0;
        }
        let coeff = fdct_8x8(&block);
        let recon = idct_8x8(&coeff);
        let maxerr = block.iter().zip(recon.iter()).fold(0f64, |m, (a, b)| m.max((a - b).abs()));
        assert!(maxerr < 1e-6, "maxerr={maxerr}");
    }

    #[semio_framework_async_macros::async_test]
    async fn huffman_round_trips_all_dc_luma_symbols() {
        let table = build_huffman(&DC_LUMA_BITS, &dc_luma_values()).unwrap();
        let mut bw = BitWriter::new();
        for v in 0u8..=11 {
            let (l, c) = *table.encode.get(&v).unwrap();
            bw.put_bits(c, l);
        }
        bw.flush();
        let source: &[u8] = &bw.bytes;
        let mut br = BitReader::new(&source, 0);
        for v in 0u8..=11 {
            assert_eq!(br.decode_symbol(&table).unwrap(), v);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn single_block_round_trips_through_huffman() {
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
        let source: &[u8] = &bw.bytes;
        let mut br = BitReader::new(&source, 0);
        let mut dc_pred2 = 0i32;
        let decoded = decode_block(&mut br, &mut dc_pred2, &dc_table, &ac_table).unwrap();
        assert_eq!(decoded, zz);
    }

    /// 🖼️ Non-solid-color round trip — the case the old "solid-color only"
    /// codec could never have passed. Gradient exercises AC energy across
    /// every block; asserts mean-absolute-pixel-error stays well under a
    /// visually-lossless budget of 10/255.
    #[semio_framework_async_macros::async_test]
    async fn gradient_round_trip_under_mae_threshold() {
        let (w, h) = (48u32, 40u32);
        let img = gradient_image(w, h);
        let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels: img.clone(), ..JpgSnapshot::default() };
        let bytes = encode_jpg(&snap).expect("encode");
        assert!(bytes.starts_with(&[0xFF, 0xD8]));
        assert!(bytes.ends_with(&[0xFF, 0xD9]));
        let decoded = decode_jpg(&bytes).expect("decode");
        assert_eq!(decoded.width, w);
        assert_eq!(decoded.height, h);
        let err = mae(&img, &decoded.pixels);
        println!("gradient round-trip MAE = {err}");
        assert!(err < 10.0, "gradient MAE too high: {err}");
    }

    /// 🖼️ Checkerboard: high-frequency content, harder for quantization to
    /// preserve than a gradient — same bar (MAE < 10/255).
    #[semio_framework_async_macros::async_test]
    async fn checkerboard_round_trip_under_mae_threshold() {
        let (w, h) = (32u32, 32u32);
        let img = checkerboard_image(w, h);
        let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels: img.clone(), ..JpgSnapshot::default() };
        let bytes = encode_jpg(&snap).expect("encode");
        let decoded = decode_jpg(&bytes).expect("decode");
        let err = mae(&img, &decoded.pixels);
        println!("checkerboard round-trip MAE = {err}");
        assert!(err < 10.0, "checkerboard MAE too high: {err}");
    }

    #[semio_framework_async_macros::async_test]
    async fn solid_color_still_round_trips() {
        let (w, h) = (16u32, 16u32);
        let mut img = vec![0u8; (w * h * 4) as usize];
        for px in img.chunks_mut(4) {
            px[0] = 200;
            px[1] = 100;
            px[2] = 50;
            px[3] = 255;
        }
        let snap = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels: img.clone(), ..JpgSnapshot::default() };
        let bytes = encode_jpg(&snap).expect("encode");
        let decoded = decode_jpg(&bytes).expect("decode");
        let err = mae(&img, &decoded.pixels);
        assert!(err < 5.0, "solid MAE too high: {err}");
    }

    /// 🚫 Progressive (SOF2) must be a typed `Unsupported` error, never
    /// silently decoded — hand-crafted minimal SOF2 segment.
    #[semio_framework_async_macros::async_test]
    async fn progressive_sof2_is_explicit_unsupported() {
        let mut bytes = vec![0xFFu8, 0xD8];
        bytes.extend_from_slice(&[0xFF, 0xC2, 0x00, 0x0B, 0x08, 0x00, 0x08, 0x00, 0x08, 0x01, 0x01, 0x11, 0x00]);
        bytes.extend_from_slice(&[0xFF, 0xD9]);
        let result = decode_jpg(&bytes);
        assert!(matches!(result, Err(JpgError::Unsupported(_))), "expected Unsupported, got {result:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn non_jpeg_input_is_malformed_not_panic() {
        let result = decode_jpg(&[0x00, 0x01, 0x02, 0x03]);
        assert!(matches!(result, Err(JpgError::Malformed(_))));
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG2: per-artifact conformance laws (the recipe's §4 deliverable checklist item 6) —
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Relocated verbatim from `⚙️engine`'s own test
    /// region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — mirrors png's own
    /// identically-named module exactly (same six laws, same structure, only the demo-case helpers
    /// differ per the recipe's own note that every pilot's `conformance_laws` module is
    /// near-identical).
    mod conformance_laws {
        use super::*;
        use crate::artifacts::jpg::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
        /// message).
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar (hex-dump grammar of the TEXT DSL
        /// form — jpg's real internal marker structure is `../💾️binary/📡️.protocol.semio`'s
        /// job, not this leaf's, per the recipe's own png precedent) recognizes real `print_dsl`
        /// output for the demo snapshot — same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_jpg_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `JpgMutation` variant (`mutations::demo_mutation_cases()`).
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `JpgDiff` (`diff::demo_diff_cases()`), incl. the empty diff and
        /// every tri-state/`JpgFrameChange`/collection-triple shape.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_jpg_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_jpg_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        ///
        /// 🧪️ P2-FG2 deviation from png's own verbatim `fixture_honesty_law` shape (documented,
        /// not a mistake): jpg is a LOSSY lifecycle format whose `parse_dsl`/`decode_pack` genuinely
        /// `decode_jpg`-round-trip through real DCT/quantization/Huffman compression, then
        /// canonicalize a FRESH `frame`/`quant_tables`/`huffman_tables`/`sof_marker` on re-decode
        /// (matching `codec_retention_law`'s own already-established precedent above, and the
        /// engine's own documented `EncodeScopeNote`) — a hand-authored `demo_jpg_snapshot()` (never
        /// itself decoded) can therefore NEVER equal `parse_dsl(print_dsl(demo))` field-for-field
        /// (confirmed live: a real `cargo test` run showed exactly this — decoded `frame`/
        /// `quant_tables`/`huffman_tables` populated, `re_encode_quality` reset to `None`, pixels
        /// DCT-lossy-shifted). The FORWARD direction (`print_dsl(demo) == FIXTURE_DSL`,
        /// `encode_pack(demo) == FIXTURE_PACK`, byte-for-byte) still asserts the strong "fixture is
        /// GENUINE encoder output" guarantee the recipe's law is really about; the REVERSE direction
        /// asserts the same width/height/pixel-length invariant `codec_retention_law` already
        /// establishes as this artifact's own honest lossy-round-trip contract, plus the ACTUAL
        /// dimension bytes on wire (SOF0 width/height) matching, rather than asserting the
        /// impossible byte-exact struct equality.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

            let demo = demo_jpg_snapshot();

            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_jpg_snapshot()) drifted from the shipped .dsl.semio fixture");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_jpg_snapshot()) drifted from the shipped .pack.semio fixture");

            let parsed = <JpgSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed.width, demo.width, "shipped .dsl.semio fixture decodes to a different width than demo_jpg_snapshot()");
            assert_eq!(parsed.height, demo.height, "shipped .dsl.semio fixture decodes to a different height than demo_jpg_snapshot()");
            assert_eq!(parsed.pixels.len(), demo.pixels.len(), "shipped .dsl.semio fixture decodes to a different pixel buffer length than demo_jpg_snapshot()");

            let decoded = <JpgSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, parsed, "shipped .pack.semio fixture must decode identically to the shipped .dsl.semio fixture (same real JFIF bytes, two envelope shapes)");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::document::schema::JpgComposer as JpgRawAnyComposer;
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::JpgBaselineComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<JpgRawAnyComposer>(), composer_entry_of::<JpgBaselineComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
