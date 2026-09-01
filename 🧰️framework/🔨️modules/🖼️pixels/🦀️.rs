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
#[derive(Clone, Debug, PartialEq, Eq, Default)]
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
/// 🧮️ ISO 3309 / ITU-T V.42 CRC-32 (the PNG §5.4 checksum), table-built once per call — small
/// inputs (chunk headers/data) dominate call sites here, so a lazily-built table isn't worth the
/// extra machinery.
fn crc32(data: &[u8]) -> u32 {
    fn table_entry(mut value: u32) -> u32 {
        for _ in 0..8 {
            value = if value & 1 != 0 { 0xEDB88320 ^ (value >> 1) } else { value >> 1 };
        }
        value
    }
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = table_entry(index as u32) ^ (crc >> 8);
    }
    crc ^ 0xFFFF_FFFF
}
//#endregion Crc32

//#region Deflate
mod deflate {
    //! 🗜️ RFC1950/RFC1951 zlib/DEFLATE — Adler32, bit I/O, canonical Huffman, greedy hash-chain
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
        data: &'a [u8],
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
        for i in 0..=143 {
            l[i] = 8;
        }
        for i in 144..=255 {
            l[i] = 9;
        }
        for i in 256..=279 {
            l[i] = 7;
        }
        for i in 280..=287 {
            l[i] = 8;
        }
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
    /// 🗜️ Batch fixed-Huffman DEFLATE compressor: greedy hash-chain LZ77 with lazy one-step
    /// lookahead, emitted as a single final fixed-Huffman block (RFC1951 BTYPE=1).
    fn deflate_raw(data: &[u8]) -> Vec<u8> {
        let mut writer = BitWriter::new();
        writer.write_bits(0b011, 3);
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
        writer.align_byte();
        writer.out
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
                    lens.extend(std::iter::repeat(prev).take(rep));
                }
                17 => {
                    let rep = br.read_bits(3)? as usize + 3;
                    lens.extend(std::iter::repeat(0u8).take(rep));
                }
                18 => {
                    let rep = br.read_bits(7)? as usize + 11;
                    lens.extend(std::iter::repeat(0u8).take(rep));
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

    /// 🗜️ Zlib unwrap + inflate + Adler32 verify.
    pub fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        if data.len() < 6 {
            return Err("zlib stream too short".into());
        }
        let cmf = data[0];
        let flg = data[1];
        if (cmf & 0x0F) != 8 {
            return Err("unsupported zlib compression method".into());
        }
        if ((cmf as u16) * 256 + flg as u16) % 31 != 0 {
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
}
//#endregion Deflate

//#region PngChunkIo
const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

fn write_chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(ty);
    out.extend_from_slice(data);
    let mut crc_in = Vec::with_capacity(4 + data.len());
    crc_in.extend_from_slice(ty);
    crc_in.extend_from_slice(data);
    out.extend_from_slice(&crc32(&crc_in).to_be_bytes());
}

/// 📖️ Splits a PNG byte stream into `(type, data)` chunks, rejecting CRC mismatches and
/// truncation up front so downstream decode logic never has to re-check framing.
fn read_chunks(data: &[u8]) -> Result<Vec<([u8; 4], &[u8])>, String> {
    if data.len() < 8 || data[0..8] != PNG_SIGNATURE {
        return Err("png: bad signature".into());
    }
    let mut pos = 8usize;
    let mut chunks = Vec::new();
    loop {
        if pos + 8 > data.len() {
            return Err("png: truncated chunk header".into());
        }
        let len = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        let ty: [u8; 4] = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
        let start = pos + 8;
        let end = start.checked_add(len).ok_or("png: chunk length overflow")?;
        if end + 4 > data.len() {
            return Err("png: truncated chunk data or crc".into());
        }
        let chunk_data = &data[start..end];
        let stored_crc = u32::from_be_bytes([data[end], data[end + 1], data[end + 2], data[end + 3]]);
        let mut crc_in = Vec::with_capacity(4 + len);
        crc_in.extend_from_slice(&ty);
        crc_in.extend_from_slice(chunk_data);
        if crc32(&crc_in) != stored_crc {
            return Err(format!("png: chunk CRC mismatch ({})", String::from_utf8_lossy(&ty)));
        }
        chunks.push((ty, chunk_data));
        pos = end + 4;
        if ty == *b"IEND" {
            break;
        }
        if pos >= data.len() {
            return Err("png: missing IEND".into());
        }
    }
    Ok(chunks)
}
//#endregion PngChunkIo

//#region PngIhdr
struct Ihdr {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    interlace: u8,
}

fn parse_ihdr(data: &[u8]) -> Result<Ihdr, String> {
    if data.len() != 13 {
        return Err("png IHDR: expected 13 bytes".into());
    }
    let width = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let height = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    let bit_depth = data[8];
    let color_type = data[9];
    let compression = data[10];
    let filter_method = data[11];
    let interlace = data[12];
    if width == 0 || height == 0 {
        return Err("png IHDR: zero dimension".into());
    }
    if compression != 0 {
        return Err("png IHDR: unsupported compression method".into());
    }
    if filter_method != 0 {
        return Err("png IHDR: unsupported filter method".into());
    }
    if interlace > 1 {
        return Err("png IHDR: unsupported interlace method".into());
    }
    let valid = match color_type {
        0 => matches!(bit_depth, 1 | 2 | 4 | 8 | 16),
        2 => matches!(bit_depth, 8 | 16),
        3 => matches!(bit_depth, 1 | 2 | 4 | 8),
        4 => matches!(bit_depth, 8 | 16),
        6 => matches!(bit_depth, 8 | 16),
        _ => false,
    };
    if !valid {
        return Err(format!("png IHDR: unsupported color type {color_type} / bit depth {bit_depth}"));
    }
    Ok(Ihdr { width, height, bit_depth, color_type, interlace })
}

fn samples_per_pixel(color_type: u8) -> usize {
    match color_type {
        0 => 1,
        2 => 3,
        3 => 1,
        4 => 2,
        6 => 4,
        _ => unreachable!("validated in parse_ihdr"),
    }
}

fn bpp_bytes(ihdr: &Ihdr) -> usize {
    ((samples_per_pixel(ihdr.color_type) * ihdr.bit_depth as usize + 7) / 8).max(1)
}

fn packed_row_bytes(width: u32, color_type: u8, bit_depth: u8) -> usize {
    let bits = width as usize * samples_per_pixel(color_type) * bit_depth as usize;
    (bits + 7) / 8
}
//#endregion PngIhdr

//#region PngFilter
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (a, b, c) = (a as i32, b as i32, c as i32);
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}

fn filter_row(filter_type: u8, cur: &[u8], prev: Option<&[u8]>, bpp: usize) -> Vec<u8> {
    let mut out = vec![0u8; cur.len()];
    for x in 0..cur.len() {
        let a = if x >= bpp { cur[x - bpp] } else { 0 };
        let b = prev.map(|p| p[x]).unwrap_or(0);
        let c = if x >= bpp { prev.map(|p| p[x - bpp]).unwrap_or(0) } else { 0 };
        out[x] = match filter_type {
            0 => cur[x],
            1 => cur[x].wrapping_sub(a),
            2 => cur[x].wrapping_sub(b),
            3 => cur[x].wrapping_sub(((a as u16 + b as u16) / 2) as u8),
            4 => cur[x].wrapping_sub(paeth(a, b, c)),
            _ => unreachable!("caller only passes 0..=4"),
        };
    }
    out
}

fn defilter_row(filter_type: u8, filt: &[u8], prev: Option<&[u8]>, bpp: usize) -> Result<Vec<u8>, String> {
    if filter_type > 4 {
        return Err(format!("png: unsupported filter type {filter_type}"));
    }
    let mut out = vec![0u8; filt.len()];
    for x in 0..filt.len() {
        let a = if x >= bpp { out[x - bpp] } else { 0 };
        let b = prev.map(|p| p[x]).unwrap_or(0);
        let c = if x >= bpp { prev.map(|p| p[x - bpp]).unwrap_or(0) } else { 0 };
        out[x] = match filter_type {
            0 => filt[x],
            1 => filt[x].wrapping_add(a),
            2 => filt[x].wrapping_add(b),
            3 => filt[x].wrapping_add(((a as u16 + b as u16) / 2) as u8),
            4 => filt[x].wrapping_add(paeth(a, b, c)),
            _ => unreachable!("checked above"),
        };
    }
    Ok(out)
}

/// 🧮️ Minimum-sum-of-absolute-values heuristic (bytes read as signed), the common real-world
/// choice per PNG spec §9.8 — not optimal, but genuinely per-scanline-adaptive.
fn choose_filter(cur: &[u8], prev: Option<&[u8]>, bpp: usize) -> (u8, Vec<u8>) {
    let mut best_ft = 0u8;
    let mut best_sum = i64::MAX;
    let mut best = Vec::new();
    for ft in 0u8..=4 {
        let f = filter_row(ft, cur, prev, bpp);
        let sum: i64 = f.iter().map(|&b| (b as i8).unsigned_abs() as i64).sum();
        if sum < best_sum {
            best_sum = sum;
            best_ft = ft;
            best = f;
        }
    }
    (best_ft, best)
}

fn defilter_pass(raw: &[u8], mut pos: usize, height: u32, row_bytes: usize, bpp: usize) -> Result<(Vec<Vec<u8>>, usize), String> {
    let mut rows = Vec::with_capacity(height as usize);
    let mut prev: Option<Vec<u8>> = None;
    for _ in 0..height {
        if pos >= raw.len() {
            return Err("png: truncated scanline data".into());
        }
        let ft = raw[pos];
        pos += 1;
        if pos + row_bytes > raw.len() {
            return Err("png: truncated scanline data".into());
        }
        let filt = &raw[pos..pos + row_bytes];
        pos += row_bytes;
        let recon = defilter_row(ft, filt, prev.as_deref(), bpp)?;
        prev = Some(recon.clone());
        rows.push(recon);
    }
    Ok((rows, pos))
}
//#endregion PngFilter

//#region PngAdam7
/// 🪜️ Pass geometry `(start_x, start_y, step_x, step_y)`, PNG spec §8.2.
const ADAM7: [(u32, u32, u32, u32); 7] = [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)];

fn adam7_pass_dims(width: u32, height: u32, pass: usize) -> (u32, u32) {
    let (sx, sy, stx, sty) = ADAM7[pass];
    let w = if width > sx { (width - sx + stx - 1) / stx } else { 0 };
    let h = if height > sy { (height - sy + sty - 1) / sty } else { 0 };
    (w, h)
}
//#endregion PngAdam7

//#region PngUnpack
fn unpack_samples(row: &[u8], width: usize, spp: usize, bit_depth: u8) -> Vec<u32> {
    let count = width * spp;
    let mut out = Vec::with_capacity(count);
    if bit_depth == 16 {
        for i in 0..count {
            out.push(((row[i * 2] as u32) << 8) | row[i * 2 + 1] as u32);
        }
    } else if bit_depth == 8 {
        for i in 0..count {
            out.push(row[i] as u32);
        }
    } else {
        let mut bitpos = 0usize;
        for _ in 0..count {
            let mut v = 0u32;
            for _ in 0..bit_depth {
                let byte = row[bitpos / 8];
                let bit = (byte >> (7 - (bitpos % 8))) & 1;
                v = (v << 1) | bit as u32;
                bitpos += 1;
            }
            out.push(v);
        }
    }
    out
}

fn scale_to_8(sample: u32, bit_depth: u8) -> u8 {
    match bit_depth {
        8 => sample as u8,
        16 => (sample >> 8) as u8,
        _ => {
            let maxval = (1u32 << bit_depth) - 1;
            ((sample * 255 + maxval / 2) / maxval) as u8
        }
    }
}

/// 🎨️ Converts one pixel's raw (unscaled) samples to 8-bit RGBA using PLTE/tRNS as needed.
fn pixel_to_rgba(samples: &[u32], ihdr: &Ihdr, palette: &[[u8; 3]], palette_alpha: &[u8], gray_trans: Option<u32>, rgb_trans: Option<(u32, u32, u32)>) -> Result<[u8; 4], String> {
    match ihdr.color_type {
        0 => {
            let g = samples[0];
            let a = if gray_trans == Some(g) { 0 } else { 255 };
            let g8 = scale_to_8(g, ihdr.bit_depth);
            Ok([g8, g8, g8, a])
        }
        2 => {
            let (r, g, b) = (samples[0], samples[1], samples[2]);
            let a = if rgb_trans == Some((r, g, b)) { 0 } else { 255 };
            Ok([scale_to_8(r, ihdr.bit_depth), scale_to_8(g, ihdr.bit_depth), scale_to_8(b, ihdr.bit_depth), a])
        }
        3 => {
            let idx = samples[0] as usize;
            let rgb = palette.get(idx).ok_or_else(|| format!("png: palette index {idx} out of range"))?;
            let a = palette_alpha.get(idx).copied().unwrap_or(255);
            Ok([rgb[0], rgb[1], rgb[2], a])
        }
        4 => {
            let g8 = scale_to_8(samples[0], ihdr.bit_depth);
            let a8 = scale_to_8(samples[1], ihdr.bit_depth);
            Ok([g8, g8, g8, a8])
        }
        6 => Ok([scale_to_8(samples[0], ihdr.bit_depth), scale_to_8(samples[1], ihdr.bit_depth), scale_to_8(samples[2], ihdr.bit_depth), scale_to_8(samples[3], ihdr.bit_depth)]),
        _ => unreachable!("validated in parse_ihdr"),
    }
}
//#endregion PngUnpack

//#region PngCodec
/// 📤️ Encodes an 8-bit RGBA image as a minimal, spec-conformant PNG: IHDR (color type 6 / bit
/// depth 8 / interlace 0), one adaptively-filtered IDAT, IEND. No ancillary chunks — none of this
/// crate's callers need gAMA/cHRM/sRGB/pHYs/tIME/bKGD/text metadata round-tripped.
pub fn encode_png(image: &RasterImage) -> Result<Vec<u8>, RasterError> {
    let expected_len = (image.width as usize).checked_mul(image.height as usize).and_then(|p| p.checked_mul(4)).ok_or_else(|| RasterError::Codec("dimensions overflow".into()))?;
    if image.pixels.len() != expected_len {
        return Err(RasterError::Codec("pixels length mismatch".into()));
    }
    let idat = pack_idat(image.width, image.height, 6, 8, &image.pixels, 4);
    let compressed = deflate::zlib_compress(&idat);
    Ok(assemble_png(image.width, image.height, 6, 8, &compressed))
}

/// 📤️ Encodes row-major big-endian 16-bit grayscale samples as a 16-bit grayscale PNG (color
/// type 0), for lossless heightfield/DSM-style export.
pub fn encode_png_gray16(width: u32, height: u32, samples: &[u16]) -> Result<Vec<u8>, RasterError> {
    let expected_len = (width as usize).checked_mul(height as usize).ok_or_else(|| RasterError::Codec("dimensions overflow".into()))?;
    if width == 0 || height == 0 || samples.len() != expected_len {
        return Err(RasterError::Codec("samples length mismatch".into()));
    }
    let mut packed = Vec::with_capacity(samples.len() * 2);
    for &sample in samples {
        packed.extend_from_slice(&sample.to_be_bytes());
    }
    let idat = pack_idat(width, height, 0, 16, &packed, 2);
    let compressed = deflate::zlib_compress(&idat);
    Ok(assemble_png(width, height, 0, 16, &compressed))
}

fn pack_idat(width: u32, height: u32, _color_type: u8, _bit_depth: u8, packed: &[u8], bpp: usize) -> Vec<u8> {
    let row_bytes = width as usize * bpp;
    let mut idat = Vec::with_capacity((row_bytes + 1) * height as usize);
    let mut prev: Option<Vec<u8>> = None;
    for y in 0..height as usize {
        let row = &packed[y * row_bytes..(y + 1) * row_bytes];
        let (ft, filtered) = choose_filter(row, prev.as_deref(), bpp);
        idat.push(ft);
        idat.extend_from_slice(&filtered);
        prev = Some(row.to_vec());
    }
    idat
}

fn assemble_png(width: u32, height: u32, color_type: u8, bit_depth: u8, compressed_idat: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&PNG_SIGNATURE);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[bit_depth, color_type, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"IDAT", compressed_idat);
    write_chunk(&mut out, b"IEND", &[]);
    out
}

/// 📥️ Decodes a PNG byte stream into canonical 8-bit RGBA — every color type (grayscale,
/// truecolor, indexed, grayscale+alpha, truecolor+alpha), every bit depth (1/2/4/8/16), and
/// Adam7 interlacing.
pub fn decode_png(data: &[u8]) -> Result<RasterImage, RasterError> {
    let chunks = read_chunks(data).map_err(RasterError::Codec)?;
    let mut ihdr: Option<Ihdr> = None;
    let mut palette: Vec<[u8; 3]> = Vec::new();
    let mut palette_alpha: Vec<u8> = Vec::new();
    let mut gray_trans: Option<u32> = None;
    let mut rgb_trans: Option<(u32, u32, u32)> = None;
    let mut idat = Vec::new();
    let mut seen_idat = false;

    for &(ty, chunk) in &chunks {
        if ty == *b"IHDR" {
            ihdr = Some(parse_ihdr(chunk).map_err(RasterError::Codec)?);
        } else if ty == *b"PLTE" {
            if chunk.len() % 3 != 0 {
                return Err(RasterError::Codec("png PLTE: length not a multiple of 3".into()));
            }
            palette = chunk.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
        } else if ty == *b"tRNS" {
            let color_type = ihdr.as_ref().ok_or_else(|| RasterError::Codec("png: tRNS before IHDR".into()))?.color_type;
            match color_type {
                0 => {
                    if chunk.len() != 2 {
                        return Err(RasterError::Codec("png tRNS: expected 2 bytes for grayscale".into()));
                    }
                    gray_trans = Some(u16::from_be_bytes([chunk[0], chunk[1]]) as u32);
                }
                2 => {
                    if chunk.len() != 6 {
                        return Err(RasterError::Codec("png tRNS: expected 6 bytes for truecolor".into()));
                    }
                    let r = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
                    let g = u16::from_be_bytes([chunk[2], chunk[3]]) as u32;
                    let b = u16::from_be_bytes([chunk[4], chunk[5]]) as u32;
                    rgb_trans = Some((r, g, b));
                }
                3 => {
                    palette_alpha = chunk.to_vec();
                }
                _ => {}
            }
        } else if ty == *b"IDAT" {
            idat.extend_from_slice(chunk);
            seen_idat = true;
        } else if ty == *b"IEND" {
            // 🚫️ terminal marker, no payload to fold in
        } else if ty[0].is_ascii_uppercase() {
            return Err(RasterError::Codec(format!("png: unsupported critical chunk {}", String::from_utf8_lossy(&ty))));
        }
    }

    let ihdr = ihdr.ok_or_else(|| RasterError::Codec("png: missing IHDR".into()))?;
    if !seen_idat {
        return Err(RasterError::Codec("png: missing IDAT".into()));
    }
    if ihdr.color_type == 3 && palette.is_empty() {
        return Err(RasterError::Codec("png: color type 3 requires PLTE".into()));
    }

    let raw = deflate::zlib_decompress(&idat).map_err(RasterError::Codec)?;
    let spp = samples_per_pixel(ihdr.color_type);
    let bpp = bpp_bytes(&ihdr);
    let mut rgba = vec![0u8; ihdr.width as usize * ihdr.height as usize * 4];

    let mut put_row = |samples: &[u32], row_width: usize, base_x: u32, base_y: u32, step_x: u32| -> Result<(), String> {
        for i in 0..row_width {
            let px = pixel_to_rgba(&samples[i * spp..i * spp + spp], &ihdr, &palette, &palette_alpha, gray_trans, rgb_trans)?;
            let x = base_x + i as u32 * step_x;
            let idx = (base_y as usize * ihdr.width as usize + x as usize) * 4;
            rgba[idx..idx + 4].copy_from_slice(&px);
        }
        Ok(())
    };

    if ihdr.interlace == 0 {
        let row_bytes = packed_row_bytes(ihdr.width, ihdr.color_type, ihdr.bit_depth);
        let (rows, _) = defilter_pass(&raw, 0, ihdr.height, row_bytes, bpp).map_err(RasterError::Codec)?;
        for (y, row) in rows.iter().enumerate() {
            let samples = unpack_samples(row, ihdr.width as usize, spp, ihdr.bit_depth);
            put_row(&samples, ihdr.width as usize, 0, y as u32, 1).map_err(RasterError::Codec)?;
        }
    } else {
        let mut pos = 0usize;
        for pass in 0..7 {
            let (pw, ph) = adam7_pass_dims(ihdr.width, ihdr.height, pass);
            if pw == 0 || ph == 0 {
                continue;
            }
            let row_bytes = packed_row_bytes(pw, ihdr.color_type, ihdr.bit_depth);
            let (rows, new_pos) = defilter_pass(&raw, pos, ph, row_bytes, bpp).map_err(RasterError::Codec)?;
            pos = new_pos;
            let (sx, sy, stx, sty) = ADAM7[pass];
            for (j, row) in rows.iter().enumerate() {
                let samples = unpack_samples(row, pw as usize, spp, ihdr.bit_depth);
                put_row(&samples, pw as usize, sx, sy + j as u32 * sty, stx).map_err(RasterError::Codec)?;
            }
        }
    }

    Ok(RasterImage { width: ihdr.width, height: ihdr.height, pixels: rgba })
}
//#endregion PngCodec

//#region PngScanlineDecoder
/// ⏱️ Incremental, one-row-per-call PNG decoder — bounds per-step CPU cost for callers that must
/// stay interaction-friendly (progress/cancellation) while decoding a still image. Non-interlaced
/// images decode genuinely row-by-row (the deflate decompress of the already-length-bounded IDAT
/// is the one eager step; unfiltering + sample unpacking + RGBA canonicalization happen one row
/// per [`Self::next_row`] call). Interlaced images decode all Adam7 passes eagerly on
/// construction (rare in practice for camera/photogrammetry input) and then simply drain a
/// pre-built row queue — see the module docs for the tradeoff this accepts.
pub struct PngScanlineDecoder {
    width: u32,
    height: u32,
    rows: PngScanlineSource,
}

enum PngScanlineSource {
    NonInterlaced { ihdr: Box<Ihdr>, palette: Vec<[u8; 3]>, palette_alpha: Vec<u8>, gray_trans: Option<u32>, rgb_trans: Option<(u32, u32, u32)>, raw: Vec<u8>, pos: usize, prev: Option<Vec<u8>>, row_bytes: usize, bpp: usize, next_y: u32 },
    Queued(std::collections::VecDeque<Vec<u8>>),
}

impl PngScanlineDecoder {
    /// 🌱️ Parses IHDR/PLTE/tRNS and decompresses the IDAT stream (the only eager, non-bounded
    /// step — callers that need a hard CPU/byte cap on THIS step too should keep bounding total
    /// PNG byte size before construction, exactly as `MAX_STILL_PIXELS`-style guards already do).
    pub fn new(data: &[u8]) -> Result<Self, RasterError> {
        let chunks = read_chunks(data).map_err(RasterError::Codec)?;
        let mut ihdr: Option<Ihdr> = None;
        let mut palette: Vec<[u8; 3]> = Vec::new();
        let mut palette_alpha: Vec<u8> = Vec::new();
        let mut gray_trans: Option<u32> = None;
        let mut rgb_trans: Option<(u32, u32, u32)> = None;
        let mut idat = Vec::new();
        let mut seen_idat = false;
        for &(ty, chunk) in &chunks {
            if ty == *b"IHDR" {
                ihdr = Some(parse_ihdr(chunk).map_err(RasterError::Codec)?);
            } else if ty == *b"PLTE" {
                palette = chunk.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
            } else if ty == *b"tRNS" {
                let color_type = ihdr.as_ref().ok_or_else(|| RasterError::Codec("png: tRNS before IHDR".into()))?.color_type;
                match color_type {
                    0 if chunk.len() == 2 => gray_trans = Some(u16::from_be_bytes([chunk[0], chunk[1]]) as u32),
                    2 if chunk.len() == 6 => {
                        let r = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
                        let g = u16::from_be_bytes([chunk[2], chunk[3]]) as u32;
                        let b = u16::from_be_bytes([chunk[4], chunk[5]]) as u32;
                        rgb_trans = Some((r, g, b));
                    }
                    3 => palette_alpha = chunk.to_vec(),
                    _ => {}
                }
            } else if ty == *b"IDAT" {
                idat.extend_from_slice(chunk);
                seen_idat = true;
            }
        }
        let ihdr = ihdr.ok_or_else(|| RasterError::Codec("png: missing IHDR".into()))?;
        if !seen_idat {
            return Err(RasterError::Codec("png: missing IDAT".into()));
        }
        if ihdr.color_type == 3 && palette.is_empty() {
            return Err(RasterError::Codec("png: color type 3 requires PLTE".into()));
        }
        let raw = deflate::zlib_decompress(&idat).map_err(RasterError::Codec)?;
        let width = ihdr.width;
        let height = ihdr.height;
        if ihdr.interlace == 0 {
            let row_bytes = packed_row_bytes(width, ihdr.color_type, ihdr.bit_depth);
            let bpp = bpp_bytes(&ihdr);
            Ok(Self { width, height, rows: PngScanlineSource::NonInterlaced { ihdr: Box::new(ihdr), palette, palette_alpha, gray_trans, rgb_trans, raw, pos: 0, prev: None, row_bytes, bpp, next_y: 0 } })
        } else {
            let spp = samples_per_pixel(ihdr.color_type);
            let bpp = bpp_bytes(&ihdr);
            let mut rgba_rows: Vec<Vec<u8>> = (0..height).map(|_| vec![0u8; width as usize * 4]).collect();
            let mut pos = 0usize;
            for pass in 0..7 {
                let (pw, ph) = adam7_pass_dims(width, height, pass);
                if pw == 0 || ph == 0 {
                    continue;
                }
                let row_bytes = packed_row_bytes(pw, ihdr.color_type, ihdr.bit_depth);
                let (rows, new_pos) = defilter_pass(&raw, pos, ph, row_bytes, bpp).map_err(RasterError::Codec)?;
                pos = new_pos;
                let (sx, sy, stx, sty) = ADAM7[pass];
                for (j, row) in rows.iter().enumerate() {
                    let samples = unpack_samples(row, pw as usize, spp, ihdr.bit_depth);
                    let y = sy + j as u32 * sty;
                    for i in 0..pw as usize {
                        let px = pixel_to_rgba(&samples[i * spp..i * spp + spp], &ihdr, &palette, &palette_alpha, gray_trans, rgb_trans).map_err(RasterError::Codec)?;
                        let x = (sx + i as u32 * stx) as usize;
                        rgba_rows[y as usize][x * 4..x * 4 + 4].copy_from_slice(&px);
                    }
                }
            }
            Ok(Self { width, height, rows: PngScanlineSource::Queued(rgba_rows.into()) })
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// ⏭️ Advances by exactly one scanline, returning canonical RGBA8 bytes (`width * 4`), or
    /// `None` once every row has been yielded.
    pub fn next_row(&mut self) -> Result<Option<Vec<u8>>, RasterError> {
        match &mut self.rows {
            PngScanlineSource::Queued(queue) => Ok(queue.pop_front()),
            PngScanlineSource::NonInterlaced { ihdr, palette, palette_alpha, gray_trans, rgb_trans, raw, pos, prev, row_bytes, bpp, next_y } => {
                if *next_y >= self.height {
                    return Ok(None);
                }
                if *pos >= raw.len() {
                    return Err(RasterError::Codec("png: truncated scanline data".into()));
                }
                let ft = raw[*pos];
                *pos += 1;
                if *pos + *row_bytes > raw.len() {
                    return Err(RasterError::Codec("png: truncated scanline data".into()));
                }
                let filt = &raw[*pos..*pos + *row_bytes];
                *pos += *row_bytes;
                let recon = defilter_row(ft, filt, prev.as_deref(), *bpp).map_err(RasterError::Codec)?;
                let spp = samples_per_pixel(ihdr.color_type);
                let samples = unpack_samples(&recon, self.width as usize, spp, ihdr.bit_depth);
                let mut row = vec![0u8; self.width as usize * 4];
                for i in 0..self.width as usize {
                    let px = pixel_to_rgba(&samples[i * spp..i * spp + spp], &**ihdr, &palette[..], &palette_alpha[..], *gray_trans, *rgb_trans).map_err(RasterError::Codec)?;
                    row[i * 4..i * 4 + 4].copy_from_slice(&px);
                }
                *prev = Some(recon);
                *next_y += 1;
                Ok(Some(row))
            }
        }
    }
}
//#endregion PngScanlineDecoder

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

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn gradient_checkerboard(w: u32, h: u32) -> RasterImage {
        let mut pixels = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            for x in 0..w {
                let checker = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
                pixels.extend_from_slice(&[checker, ((x * 37) % 256) as u8, ((y * 53) % 256) as u8, 255]);
            }
        }
        RasterImage { width: w, height: h, pixels }
    }

    fn lcg_bytes(seed: u64, count: usize) -> Vec<u8> {
        let mut state = seed;
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            out.push((state >> 56) as u8);
        }
        out
    }

    //#region OwnRoundTrip
    #[test]
    fn gradient_checkerboard_round_trip() {
        let image = gradient_checkerboard(17, 13);
        let encoded = encode_png(&image).expect("encode");
        let decoded = decode_png(&encoded).expect("decode");
        assert_eq!(decoded, image);
    }

    #[test]
    fn solid_color_round_trip() {
        let (w, h) = (4u32, 4u32);
        let pixels: Vec<u8> = (0..w * h).flat_map(|_| [10u8, 20, 30, 255]).collect();
        let image = RasterImage { width: w, height: h, pixels };
        let encoded = encode_png(&image).expect("encode");
        let decoded = decode_png(&encoded).expect("decode");
        assert_eq!(decoded, image);
    }

    #[test]
    fn random_rgba_round_trip() {
        let (w, h) = (23u32, 19u32);
        let pixels = lcg_bytes(0xC0FFEE, (w * h * 4) as usize);
        let image = RasterImage { width: w, height: h, pixels };
        let encoded = encode_png(&image).expect("encode");
        let decoded = decode_png(&encoded).expect("decode");
        assert_eq!(decoded, image);
    }

    #[test]
    fn gray16_round_trip_via_decode() {
        let (w, h) = (5u32, 3u32);
        let samples: Vec<u16> = (0..w * h).map(|i| (i as u16).wrapping_mul(4111)).collect();
        let encoded = encode_png_gray16(w, h, &samples).expect("encode");
        let decoded = decode_png(&encoded).expect("decode");
        assert_eq!(decoded.width, w);
        assert_eq!(decoded.height, h);
        for (i, &sample) in samples.iter().enumerate() {
            let hi = (sample >> 8) as u8;
            assert_eq!(decoded.pixels[i * 4], hi);
            assert_eq!(decoded.pixels[i * 4 + 1], hi);
            assert_eq!(decoded.pixels[i * 4 + 2], hi);
            assert_eq!(decoded.pixels[i * 4 + 3], 255);
        }
    }

    #[test]
    fn scanline_decoder_matches_batch_decode() {
        let image = gradient_checkerboard(31, 11);
        let encoded = encode_png(&image).expect("encode");
        let batch = decode_png(&encoded).expect("batch decode");
        let mut scanline = PngScanlineDecoder::new(&encoded).expect("scanline decoder");
        assert_eq!(scanline.width(), image.width);
        assert_eq!(scanline.height(), image.height);
        let mut rows = Vec::new();
        while let Some(row) = scanline.next_row().expect("next row") {
            rows.push(row);
        }
        assert_eq!(rows.len(), image.height as usize);
        let flattened: Vec<u8> = rows.into_iter().flatten().collect();
        assert_eq!(flattened, batch.pixels);
    }

    #[test]
    fn crc_mismatch_is_rejected() {
        let image = gradient_checkerboard(2, 2);
        let mut encoded = encode_png(&image).expect("encode");
        let last = encoded.len() - 1;
        encoded[last] ^= 0xFF;
        assert!(decode_png(&encoded).is_err());
    }

    #[test]
    fn resize_bilinear_identity_is_copy() {
        let image = gradient_checkerboard(6, 6);
        let resized = resize_bilinear(&image, 6, 6);
        assert_eq!(resized, image);
    }

    #[test]
    fn resize_bilinear_solid_color_stays_solid() {
        let (w, h) = (8u32, 8u32);
        let pixels: Vec<u8> = (0..w * h).flat_map(|_| [200u8, 100, 50, 255]).collect();
        let image = RasterImage { width: w, height: h, pixels };
        let resized = resize_bilinear(&image, 3, 5);
        for chunk in resized.pixels.chunks_exact(4) {
            assert_eq!(chunk, &[200, 100, 50, 255]);
        }
    }
    //#endregion OwnRoundTrip

    //#region OracleDifferential
    /// 🔬️ Differential oracle: encode with OUR codec, decode with the third-party `png` crate
    /// (dev-dependency only — never a runtime dependency of this crate or any plugin), and vice
    /// versa. Deterministic LCG-seeded pixels, no `rand` crate.
    #[test]
    fn oracle_decodes_our_encode() {
        let (w, h) = (29u32, 17u32);
        let pixels = lcg_bytes(0xA5A5_1234, (w * h * 4) as usize);
        let image = RasterImage { width: w, height: h, pixels: pixels.clone() };
        let encoded = encode_png(&image).expect("our encode");

        let decoder = png::Decoder::new(encoded.as_slice());
        let mut reader = decoder.read_info().expect("oracle read_info");
        let mut buffer = vec![0u8; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buffer).expect("oracle next_frame");
        assert_eq!(info.width, w);
        assert_eq!(info.height, h);
        assert_eq!(info.color_type, png::ColorType::Rgba);
        assert_eq!(info.bit_depth, png::BitDepth::Eight);
        assert_eq!(&buffer[..info.buffer_size()], pixels.as_slice());
    }

    #[test]
    fn our_decode_reads_oracle_encode() {
        let (w, h) = (21u32, 25u32);
        let pixels = lcg_bytes(0xFEED_BEEF, (w * h * 4) as usize);
        let mut encoded = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut encoded, w, h);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().expect("oracle write_header");
            writer.write_image_data(&pixels).expect("oracle write_image_data");
        }
        let decoded = decode_png(&encoded).expect("our decode");
        assert_eq!(decoded.width, w);
        assert_eq!(decoded.height, h);
        assert_eq!(decoded.pixels, pixels);
    }

    #[test]
    fn our_decode_reads_oracle_palette_encode() {
        let (w, h) = (6u32, 4u32);
        let palette: Vec<u8> = vec![10, 20, 30, 200, 100, 50, 0, 0, 0, 255, 255, 255];
        let indices: Vec<u8> = (0..w * h).map(|i| (i % 4) as u8).collect();
        let mut encoded = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut encoded, w, h);
            encoder.set_color(png::ColorType::Indexed);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_palette(palette.clone());
            let mut writer = encoder.write_header().expect("oracle write_header");
            writer.write_image_data(&indices).expect("oracle write_image_data");
        }
        let decoded = decode_png(&encoded).expect("our decode");
        assert_eq!(decoded.width, w);
        assert_eq!(decoded.height, h);
        for (i, &index) in indices.iter().enumerate() {
            let base = index as usize * 3;
            assert_eq!(&decoded.pixels[i * 4..i * 4 + 3], &palette[base..base + 3]);
            assert_eq!(decoded.pixels[i * 4 + 3], 255);
        }
    }

    #[test]
    fn zlib_compress_decompress_round_trip() {
        let payload = lcg_bytes(0x1357_9BDF, 4096);
        let compressed = deflate::zlib_compress(&payload);
        let decompressed = deflate::zlib_decompress(&compressed).expect("valid zlib stream");
        assert_eq!(decompressed, payload);
    }
    //#endregion OracleDifferential
}
//#endregion Tests
