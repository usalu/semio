//! 🔡️ Entropy coding of the JPEG codec: the Huffman table builder, the bit writer and reader, the sign extension and the block encoder and decoder. See [`super`].

use super::*;

//#region Huffman
/// 🌳 Canonical Huffman table built from DHT's `bits`/`values` (ITU T.81
/// Annex C). `decode` keyed by `(code_length, code)`; `encode` by symbol.
#[derive(Clone, Debug, Default)]
pub(crate) struct HuffTable {
    pub(crate) decode: HashMap<(u8, u16), u8>,
    pub(crate) encode: HashMap<u8, (u8, u16)>,
    max_len: u8,
}

/// 🏗️ Builds canonical codes per Annex C.2: codes are assigned in symbol
/// order, incrementing within a length and left-shifting on length change —
/// same "canonical" spirit as deflate's Huffman but JPEG's table layout
/// (flat bits[16] counts + values[]) is its own format, not reused from deflate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn build_huffman(bits: &[u8; 16], values: &[u8]) -> Result<HuffTable, JpgError> {
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
pub(crate) struct BitWriter {
    pub(crate) bytes: Vec<u8>,
    pub(crate) acc: u32,
    pub(crate) nbits: u32,
}

impl BitWriter {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn new() -> Self {
        Self { bytes: Vec::new(), acc: 0, nbits: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn put_bits(&mut self, value: u16, len: u8) {
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
    pub(crate) fn flush(&mut self) {
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
pub(crate) struct BitReader<'a> {
    pub(crate) data: &'a dyn JpgByteSource,
    pub(crate) pos: usize,
    pub(crate) acc: u32,
    pub(crate) nbits: u32,
}

impl<'a> BitReader<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn new(data: &'a dyn JpgByteSource, pos: usize) -> Self {
        Self { data, pos, acc: 0, nbits: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn next_byte(&mut self) -> Option<u8> {
        if self.pos >= self.data.len() {
            return None;
        }
        let b = self.data.byte(self.pos)?;
        if b == 0xFF {
            // A lone 0xFF as the final byte of the source is a truncated stuffing pair or
            // marker, never data — reporting it as a stuffed 0xFF decoded bits the input lacks.
            let b2 = self.data.byte(self.pos.checked_add(1)?)?;
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
    pub(crate) fn decode_symbol(&mut self, table: &HuffTable) -> Result<u8, JpgError> {
        let mut code: u16 = 0;
        for len in 1..=table.max_len {
            code = (code << 1) | self.read_bit()? as u16;
            if let Some(v) = table.decode.get(&(len, code)) {
                return Ok(*v);
            }
        }
        Err(JpgError::Malformed("huffman decode: no matching code".into()))
    }
    /// 🏁️ T.81 §B.2.1/§E.2.4: the entropy-coded segment of the (single, interleaved) baseline
    /// scan ends at a marker, which for a one-scan image is `EOI` — optionally preceded by `0xFF`
    /// fill bytes (§B.1.1.2). Any other state after the last MCU (source exhausted, a missing or
    /// partial `EOI`, trailing entropy bytes) means the scan was truncated or malformed, and
    /// decoding it anyway would hand out pixels the input never carried.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn expect_end_of_image(&self) -> Result<(), JpgError> {
        let mut pos = self.pos;
        while self.data.byte(pos) == Some(0xFF) && pos.checked_add(1).and_then(|next| self.data.byte(next)) == Some(0xFF) {
            pos += 1;
        }
        let marker = pos.checked_add(1).and_then(|next| self.data.byte(next));
        if self.data.byte(pos) == Some(0xFF) && marker == Some(0xD9) {
            Ok(())
        } else {
            Err(JpgError::Malformed("entropy-coded segment not terminated by EOI (truncated or trailing scan data)".into()))
        }
    }
    /// 🔁 Byte-align and consume one `RSTn` marker at a restart boundary;
    /// also resets the DC predictors (caller's responsibility) per T.81 F.2.2.5.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn skip_restart_marker(&mut self) -> Result<(), JpgError> {
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
pub(crate) fn size_of(mut v: i32) -> u8 {
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
pub(crate) fn encode_block(bw: &mut BitWriter, coeffs: &[i32; 64], dc_pred: &mut i32, dc_table: &HuffTable, ac_table: &HuffTable) -> Result<(), JpgError> {
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
pub(crate) fn decode_block(br: &mut BitReader<'_>, dc_pred: &mut i32, dc_table: &HuffTable, ac_table: &HuffTable) -> Result<[i32; 64], JpgError> {
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
