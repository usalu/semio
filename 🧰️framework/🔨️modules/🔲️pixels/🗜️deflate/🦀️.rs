//! 🔢️ RFC1950/RFC1951 zlib/DEFLATE — Adler32, bit I/O, canonical Huffman, greedy hash-chain
//! LZ77, and the stored/fixed/dynamic inflate trio. Ported verbatim (algorithm-for-algorithm)
//! from `semio-s-plugin-stdio`'s own `🗜️deflate` artifact codec, stripped of that artifact's
//! interactive-job checkpointing and `DeflateSnapshot` typed-container framing — this module
//! only ever needs the byte<->byte zlib wrapper PNG's IDAT stream is built on.

//#region Adler32
fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}
//#endregion Adler32

//#region BitIo
#[derive(semio_framework_value::RetireOwned)]
struct BitWriter {
    out: Vec<u8>,
    cur: u8,
    nbits: u8,
}

impl BitWriter {
    fn new() -> Self {
        Self { out: Vec::new(), cur: 0, nbits: 0 }
    }
    fn write_bits(&mut self, mut value: u32, mut count: u8) {
        while count > 0 {
            let take = (8 - self.nbits).min(count);
            let mask = (1u32 << take) - 1;
            self.cur |= ((value & mask) as u8) << self.nbits;
            self.nbits += take;
            value >>= take;
            count -= take;
            if self.nbits == 8 {
                self.out.push(self.cur);
                self.cur = 0;
                self.nbits = 0;
            }
        }
    }
    fn align_byte(&mut self) {
        if self.nbits > 0 {
            self.out.push(self.cur);
            self.cur = 0;
            self.nbits = 0;
        }
    }
}

struct BitReader<'a> {
    pub data: &'a [u8],
    pos: usize,
    cur: u8,
    nbits: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0, cur: 0, nbits: 0 }
    }
    fn read_bits(&mut self, count: u8) -> Result<u32, String> {
        let mut out = 0u32;
        for i in 0..count {
            if self.nbits == 0 {
                if self.pos >= self.data.len() {
                    return Err("unexpected end of deflate stream".into());
                }
                self.cur = self.data[self.pos];
                self.pos += 1;
                self.nbits = 8;
            }
            let bit = (self.cur & 1) as u32;
            self.cur >>= 1;
            self.nbits -= 1;
            out |= bit << i;
        }
        Ok(out)
    }
    fn align_byte(&mut self) {
        self.nbits = 0;
        self.cur = 0;
    }
}
//#endregion BitIo

//#region Huffman
fn reverse_bits(mut v: u32, len: u8) -> u32 {
    let mut r = 0u32;
    for _ in 0..len {
        r = (r << 1) | (v & 1);
        v >>= 1;
    }
    r
}

fn build_codes(lengths: &[u8]) -> Vec<(u32, u8)> {
    let mut bl_count = [0u32; 16];
    for &l in lengths {
        if l > 0 {
            bl_count[l as usize] += 1;
        }
    }
    let mut next_code = [0u32; 16];
    let mut code = 0u32;
    for bits in 1..=15 {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }
    let mut codes = vec![(0u32, 0u8); lengths.len()];
    for (i, &len) in lengths.iter().enumerate() {
        if len != 0 {
            let c = next_code[len as usize];
            next_code[len as usize] += 1;
            codes[i] = (reverse_bits(c, len), len);
        }
    }
    codes
}

struct HuffDecoder {
    table: Vec<Option<(u16, u8)>>,
    max_bits: u8,
}

impl HuffDecoder {
    fn from_lengths(lengths: &[u8]) -> Result<Self, String> {
        let max_bits = lengths.iter().copied().max().unwrap_or(0);
        if max_bits > 15 {
            return Err("invalid huffman length".into());
        }
        let size = 1usize << max_bits;
        let mut table = vec![None; size.max(1)];
        let codes = build_codes(lengths);
        for (sym, &(code, len)) in codes.iter().enumerate() {
            if len == 0 {
                continue;
            }
            let step = 1usize << len;
            let mut fill = code as usize;
            while fill < size {
                table[fill] = Some((sym as u16, len));
                fill += step;
            }
        }
        Ok(Self { table, max_bits })
    }

    fn decode(&self, br: &mut BitReader<'_>) -> Result<u16, String> {
        if self.max_bits == 0 {
            return Err("empty huffman alphabet".into());
        }
        let mut acc = 0u32;
        for len in 1..=self.max_bits {
            let bit = br.read_bits(1)?;
            acc |= bit << (len - 1);
            if let Some(Some((sym, l))) = self.table.get(acc as usize) {
                if *l == len {
                    return Ok(*sym);
                }
            }
        }
        Err("invalid huffman symbol".into())
    }
}

fn fixed_lit_lengths() -> Vec<u8> {
    let mut l = vec![0u8; 288];
    l[..144].fill(8);
    l[144..256].fill(9);
    l[256..280].fill(7);
    l[280..].fill(8);
    l
}

fn fixed_dist_lengths() -> Vec<u8> {
    vec![5u8; 32]
}
//#endregion Huffman

//#region Lz77
const LEN_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LEN_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

const WINDOW: usize = 32 * 1024;
const MIN_MATCH: usize = 3;
const MAX_MATCH: usize = 258;
const HASH_BITS: u32 = 15;
const HASH_SIZE: usize = 1 << HASH_BITS;
const MAX_CHAIN: usize = 128;

#[inline]
fn hash3(data: &[u8], i: usize) -> usize {
    let v = (data[i] as u32) | ((data[i + 1] as u32) << 8) | ((data[i + 2] as u32) << 16);
    ((v.wrapping_mul(0x9E3779B1)) >> (32 - HASH_BITS)) as usize
}

fn longest_match(data: &[u8], pos: usize, head: &[i32], prev: &[i32], max_chain: usize) -> Option<(usize, usize)> {
    if pos + MIN_MATCH > data.len() {
        return None;
    }
    let limit = (data.len() - pos).min(MAX_MATCH);
    let mut best_len = 0usize;
    let mut best_dist = 0usize;
    let mut candidate = head[hash3(data, pos)];
    let min_candidate = pos.saturating_sub(WINDOW - 1) as i32;
    let mut chain = 0usize;
    while candidate >= min_candidate && candidate >= 0 && chain < max_chain {
        let cpos = candidate as usize;
        if best_len == 0 || (cpos + best_len < data.len() && data[cpos + best_len] == data[pos + best_len]) {
            let mut len = 0usize;
            while len < limit && data[cpos + len] == data[pos + len] {
                len += 1;
            }
            if len > best_len {
                best_len = len;
                best_dist = pos - cpos;
                if len >= limit {
                    break;
                }
            }
        }
        candidate = prev[cpos & (WINDOW - 1)];
        chain += 1;
    }
    if best_len >= MIN_MATCH {
        Some((best_len, best_dist))
    } else {
        None
    }
}

fn length_symbol(len: usize) -> (usize, u32, u8) {
    for (idx, &base) in LEN_BASE.iter().enumerate().rev() {
        if len >= base as usize {
            return (257 + idx, (len - base as usize) as u32, LEN_EXTRA[idx]);
        }
    }
    unreachable!("length_symbol called with len < MIN_MATCH")
}

fn distance_symbol(dist: usize) -> (usize, u32, u8) {
    for (idx, &base) in DIST_BASE.iter().enumerate().rev() {
        if dist >= base as usize {
            return (idx, (dist - base as usize) as u32, DIST_EXTRA[idx]);
        }
    }
    unreachable!("distance_symbol called with dist == 0")
}
//#endregion Lz77

//#region Compress
/// 🎞️ Batch fixed-Huffman DEFLATE compressor: greedy hash-chain LZ77 with lazy one-step
/// lookahead, emitted as a single final fixed-Huffman block (RFC1951 BTYPE=1).
fn deflate_raw(data: &[u8]) -> Vec<u8> {
    let mut writer = BitWriter::new();
    write_fixed_block(&mut writer, data, true);
    writer.align_byte();
    writer.out
}

fn write_fixed_block(writer: &mut BitWriter, data: &[u8], final_block: bool) {
    writer.write_bits(if final_block { 0b011 } else { 0b010 }, 3);
    let literal_codes = build_codes(&fixed_lit_lengths());
    let distance_codes = build_codes(&fixed_dist_lengths());
    let mut head = vec![-1i32; HASH_SIZE];
    let mut previous = vec![-1i32; WINDOW];
    let insert = |position: usize, head: &mut [i32], previous: &mut [i32]| {
        if position + MIN_MATCH <= data.len() {
            let hash = hash3(data, position);
            previous[position & (WINDOW - 1)] = head[hash];
            head[hash] = position as i32;
        }
    };
    let mut position = 0usize;
    let mut pending: Option<(usize, usize, usize)> = None;
    while position < data.len() {
        let next = longest_match(data, position, &head, &previous, MAX_CHAIN);
        insert(position, &mut head, &mut previous);
        match (pending.take(), next) {
            (None, Some((length, distance))) => {
                pending = Some((position, length, distance));
                position += 1;
            }
            (Some((start, plen, _)), Some((length, distance))) if length > plen => {
                let (code, bits) = literal_codes[data[start] as usize];
                writer.write_bits(code, bits);
                pending = Some((position, length, distance));
                position += 1;
            }
            (Some((start, length, distance)), _) => {
                let (symbol, extra, extra_bits) = length_symbol(length);
                let (code, bits) = literal_codes[symbol];
                writer.write_bits(code, bits);
                writer.write_bits(extra, extra_bits);
                let (symbol, extra, extra_bits) = distance_symbol(distance);
                let (code, bits) = distance_codes[symbol];
                writer.write_bits(code, bits);
                writer.write_bits(extra, extra_bits);
                let end = (start + length).min(data.len());
                for p in (start + 2)..end {
                    insert(p, &mut head, &mut previous);
                }
                position = end;
            }
            (None, None) => {
                let (code, bits) = literal_codes[data[position] as usize];
                writer.write_bits(code, bits);
                position += 1;
            }
        }
    }
    if let Some((start, length, distance)) = pending.take() {
        let (symbol, extra, extra_bits) = length_symbol(length);
        let (code, bits) = literal_codes[symbol];
        writer.write_bits(code, bits);
        writer.write_bits(extra, extra_bits);
        let (symbol, extra, extra_bits) = distance_symbol(distance);
        let (code, bits) = distance_codes[symbol];
        writer.write_bits(code, bits);
        writer.write_bits(extra, extra_bits);
        let _ = start;
    }
    let (code, bits) = literal_codes[256];
    writer.write_bits(code, bits);
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct ZlibEncodeCursor {
    writer: BitWriter,
    adler_a: u32,
    adler_b: u32,
    started: bool,
}

impl ZlibEncodeCursor {
    pub(super) fn new() -> Self {
        Self { writer: BitWriter::new(), adler_a: 1, adler_b: 0, started: false }
    }

    pub(super) fn push(&mut self, data: &[u8], final_block: bool) -> Vec<u8> {
        if !self.started {
            self.writer.out.extend_from_slice(&[0x78, 0x01]);
            self.started = true;
        }
        for &byte in data {
            self.adler_a = (self.adler_a + u32::from(byte)) % 65521;
            self.adler_b = (self.adler_b + self.adler_a) % 65521;
        }
        write_fixed_block(&mut self.writer, data, final_block);
        if final_block {
            self.writer.align_byte();
            self.writer.out.extend_from_slice(&((self.adler_b << 16) | self.adler_a).to_be_bytes());
        }
        std::mem::take(&mut self.writer.out)
    }
}
//#endregion Compress

//#region Decompress
fn inflate_block_stored(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
    br.align_byte();
    if br.pos + 4 > br.data.len() {
        return Err("truncated stored block".into());
    }
    let len = u16::from_le_bytes([br.data[br.pos], br.data[br.pos + 1]]);
    let nlen = u16::from_le_bytes([br.data[br.pos + 2], br.data[br.pos + 3]]);
    br.pos += 4;
    if len ^ 0xFFFF != nlen {
        return Err("stored block LEN/NLEN mismatch".into());
    }
    let end = br.pos + len as usize;
    if end > br.data.len() {
        return Err("truncated stored payload".into());
    }
    out.extend_from_slice(&br.data[br.pos..end]);
    br.pos = end;
    Ok(())
}

fn inflate_codes(br: &mut BitReader<'_>, out: &mut Vec<u8>, lit: &HuffDecoder, dist: &HuffDecoder) -> Result<(), String> {
    loop {
        let sym = lit.decode(br)? as usize;
        if sym < 256 {
            out.push(sym as u8);
        } else if sym == 256 {
            break;
        } else if sym <= 285 {
            let idx = sym - 257;
            if idx >= LEN_BASE.len() {
                return Err("invalid length symbol".into());
            }
            let mut length = LEN_BASE[idx] as usize;
            let extra = LEN_EXTRA[idx];
            if extra > 0 {
                length += br.read_bits(extra)? as usize;
            }
            let dsym = dist.decode(br)? as usize;
            if dsym >= DIST_BASE.len() {
                return Err("invalid distance symbol".into());
            }
            let mut distance = DIST_BASE[dsym] as usize;
            let dextra = DIST_EXTRA[dsym];
            if dextra > 0 {
                distance += br.read_bits(dextra)? as usize;
            }
            if distance == 0 || distance > out.len() {
                return Err("invalid backreference".into());
            }
            for _ in 0..length {
                let b = out[out.len() - distance];
                out.push(b);
            }
        } else {
            return Err("invalid lit/len symbol".into());
        }
    }
    Ok(())
}

fn dynamic_decoders(br: &mut BitReader<'_>) -> Result<(HuffDecoder, HuffDecoder), String> {
    let hlit = br.read_bits(5)? as usize + 257;
    let hdist = br.read_bits(5)? as usize + 1;
    let hclen = br.read_bits(4)? as usize + 4;
    const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    let mut cl_lens = vec![0u8; 19];
    for i in 0..hclen {
        cl_lens[ORDER[i]] = br.read_bits(3)? as u8;
    }
    let cl_dec = HuffDecoder::from_lengths(&cl_lens)?;
    let mut lens = Vec::with_capacity(hlit + hdist);
    while lens.len() < hlit + hdist {
        let sym = cl_dec.decode(br)? as usize;
        match sym {
            0..=15 => lens.push(sym as u8),
            16 => {
                let rep = br.read_bits(2)? as usize + 3;
                let prev = *lens.last().ok_or("bad repeat")?;
                lens.extend(std::iter::repeat_n(prev, rep));
            }
            17 => {
                let rep = br.read_bits(3)? as usize + 3;
                lens.extend(std::iter::repeat_n(0u8, rep));
            }
            18 => {
                let rep = br.read_bits(7)? as usize + 11;
                lens.extend(std::iter::repeat_n(0u8, rep));
            }
            _ => return Err("bad code-length symbol".into()),
        }
    }
    if lens.len() < hlit + hdist {
        return Err("incomplete dynamic trees".into());
    }
    let lit = HuffDecoder::from_lengths(&lens[..hlit])?;
    let dist = HuffDecoder::from_lengths(&lens[hlit..hlit + hdist])?;
    Ok((lit, dist))
}

fn inflate_dynamic(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
    let (lit, dist) = dynamic_decoders(br)?;
    inflate_codes(br, out, &lit, &dist)
}

fn inflate_raw(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut br = BitReader::new(data);
    let mut out = Vec::new();
    loop {
        let bfinal = br.read_bits(1)?;
        let btype = br.read_bits(2)?;
        match btype {
            0 => inflate_block_stored(&mut br, &mut out)?,
            1 => {
                let lit = HuffDecoder::from_lengths(&fixed_lit_lengths())?;
                let dist = HuffDecoder::from_lengths(&fixed_dist_lengths())?;
                inflate_codes(&mut br, &mut out, &lit, &dist)?;
            }
            2 => inflate_dynamic(&mut br, &mut out)?,
            _ => return Err("reserved BTYPE".into()),
        }
        if bfinal == 1 {
            break;
        }
    }
    Ok(out)
}
//#endregion Decompress

//#region Zlib
/// 🗜️ Zlib-wrap compress (CMF/FLG + raw deflate + Adler32).
pub fn zlib_compress(data: &[u8]) -> Vec<u8> {
    let raw = deflate_raw(data);
    let mut out = Vec::with_capacity(2 + raw.len() + 4);
    out.push(0x78);
    out.push(0x01);
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

/// ✅️ Zlib unwrap + inflate + Adler32 verify.
pub fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 6 {
        return Err("zlib stream too short".into());
    }
    let cmf = data[0];
    let flg = data[1];
    if (cmf & 0x0F) != 8 {
        return Err("unsupported zlib compression method".into());
    }
    if !((cmf as u16) * 256 + flg as u16).is_multiple_of(31) {
        return Err("zlib CMF/FLG check failed".into());
    }
    if flg & 0x20 != 0 {
        return Err("zlib preset dictionary not supported".into());
    }
    let adler_bytes = &data[data.len() - 4..];
    let expect = u32::from_be_bytes([adler_bytes[0], adler_bytes[1], adler_bytes[2], adler_bytes[3]]);
    let raw = &data[2..data.len() - 4];
    let out = inflate_raw(raw)?;
    let got = adler32(&out);
    if got != expect {
        return Err(format!("adler32 mismatch: expected {expect:#010x}, got {got:#010x}"));
    }
    Ok(out)
}
//#endregion Zlib
