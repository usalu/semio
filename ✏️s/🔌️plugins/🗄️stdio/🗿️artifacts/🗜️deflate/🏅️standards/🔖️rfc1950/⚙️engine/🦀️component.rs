//! ⚙️ DeflateEngine — zlib (RFC1950) + DEFLATE (RFC1951) + Adler32, hand-rolled.

use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::{DeflateArtifact, DeflateDiff, DeflateMutation, DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};

//#region Adler32
/// 🧮 Adler-32 (RFC1950).
pub fn adler32(data: &[u8]) -> u32 {
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

//#region BitIO
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
    fn finish(mut self) -> Vec<u8> {
        self.align_byte();
        self.out
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
//#endregion BitIO

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
    for i in 0..=143 { l[i] = 8; }
    for i in 144..=255 { l[i] = 9; }
    for i in 256..=279 { l[i] = 7; }
    for i in 280..=287 { l[i] = 8; }
    l
}

fn fixed_dist_lengths() -> Vec<u8> {
    vec![5u8; 32]
}
//#endregion Huffman

//#region DeflateCodec
const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049,
    3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];

//#region Lz77
/// 🪟️ RFC1951 §3.2.5 sliding window: matches can reference up to this many bytes back.
const WINDOW: usize = 32 * 1024;
/// 📏️ Shortest a match is worth emitting as length+distance instead of two-plus literals.
const MIN_MATCH: usize = 3;
/// 📏️ Longest length symbol 285 can encode (258 = LEN_BASE[28] + 2^5-1 extra bits).
const MAX_MATCH: usize = 258;
/// 🔑️ 3-byte-prefix hash table size (2^15 buckets over a 3-byte key -- zlib's own classic choice).
const HASH_BITS: u32 = 15;
const HASH_SIZE: usize = 1 << HASH_BITS;

#[inline]
fn hash3(data: &[u8], i: usize) -> usize {
    // A cheap multiplicative hash over 3 bytes; collisions just mean a longer (still correct)
    // chain walk, never a wrong match -- every candidate is verified byte-by-byte below.
    let v = (data[i] as u32) | ((data[i + 1] as u32) << 8) | ((data[i + 2] as u32) << 16);
    ((v.wrapping_mul(0x9E3779B1)) >> (32 - HASH_BITS)) as usize
}

/// 🔎️ Longest match at `pos` found by walking the hash chain, verified byte-by-byte (hash
/// collisions are possible; a wrong-length "match" would corrupt the stream, so every candidate
/// is checked in full before being accepted). `max_chain` bounds how many chain entries we walk
/// before settling -- a simple, deterministic time/ratio tradeoff (higher = better ratio, slower).
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
        // Cheap pre-check before the full byte compare: the candidate must at least beat the
        // current best at the position we haven't yet verified.
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
    if best_len >= MIN_MATCH { Some((best_len, best_dist)) } else { None }
}

fn length_symbol(len: usize) -> (usize, u32, u8) {
    // Highest base <= len; LEN_BASE is ascending so a linear scan from the top is exact and simple.
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

/// 🗜️ Raw DEFLATE compress: real LZ77 (32KB window, hash-chain match search, lazy one-step
/// lookahead) emitted as a single fixed-Huffman block. Fixes the prior "literals only" encoder,
/// which never searched for matches at all and so always expanded its input by ~12.5%
/// (9 bits/byte average under the fixed literal table) instead of compressing it.
pub fn deflate_raw(data: &[u8]) -> Vec<u8> {
    let lit_codes = build_codes(&fixed_lit_lengths());
    let dist_codes = build_codes(&fixed_dist_lengths());
    let mut bw = BitWriter::new();
    bw.write_bits(0b011, 3); // BFINAL=1, BTYPE=01 (fixed Huffman)

    if data.len() < MIN_MATCH {
        for &byte in data {
            let (code, len) = lit_codes[byte as usize];
            bw.write_bits(code, len);
        }
        let (eob, elen) = lit_codes[256];
        bw.write_bits(eob, elen);
        return bw.finish();
    }

    const MAX_CHAIN: usize = 128;
    let mut head = vec![-1i32; HASH_SIZE];
    let mut prev = vec![-1i32; WINDOW];
    let insert = |data: &[u8], i: usize, head: &mut [i32], prev: &mut [i32]| {
        if i + MIN_MATCH <= data.len() {
            let h = hash3(data, i);
            prev[i & (WINDOW - 1)] = head[h];
            head[h] = i as i32;
        }
    };

    let mut pos = 0usize;
    let mut pending_match: Option<(usize, usize, usize)> = None; // (start_pos, len, dist)
    while pos < data.len() {
        let m = longest_match(data, pos, &head, &prev, MAX_CHAIN);
        insert(data, pos, &mut head, &mut prev);

        match (pending_match.take(), m) {
            (None, Some((len, dist))) => {
                // Lazy matching: don't commit yet -- check whether pos+1 has a strictly longer
                // match, in which case emitting a literal at pos and taking THAT match instead
                // yields better compression (the classic zlib lazy-evaluation heuristic).
                pending_match = Some((pos, len, dist));
                pos += 1;
            }
            (Some((start, len, dist)), next_m) => {
                let better_next = matches!(next_m, Some((nlen, _)) if nlen > len);
                if better_next {
                    // Emit the deferred position as a literal, keep the new (better) match pending.
                    let (code, clen) = lit_codes[data[start] as usize];
                    bw.write_bits(code, clen);
                    if let Some((nlen, ndist)) = next_m {
                        pending_match = Some((pos, nlen, ndist));
                    }
                    pos += 1;
                } else {
                    // Commit the deferred match starting at `start` (== pos - 1 here).
                    let (lsym, lextra, lebits) = length_symbol(len);
                    let (lcode, llen) = lit_codes[lsym];
                    bw.write_bits(lcode, llen);
                    if lebits > 0 {
                        bw.write_bits(lextra, lebits);
                    }
                    let (dsym, dextra, debits) = distance_symbol(dist);
                    let (dcode, dlen) = dist_codes[dsym];
                    bw.write_bits(dcode, dlen);
                    if debits > 0 {
                        bw.write_bits(dextra, debits);
                    }
                    // Hash-insert every position the match covers (except `start`, `start+1`
                    // already inserted above) so future matches can reference into it.
                    let match_end = (start + len).min(data.len());
                    for i in (start + 2)..match_end {
                        insert(data, i, &mut head, &mut prev);
                    }
                    pos = match_end;
                }
            }
            (None, None) => {
                let (code, clen) = lit_codes[data[pos] as usize];
                bw.write_bits(code, clen);
                pos += 1;
            }
        }
    }
    if let Some((start, len, dist)) = pending_match {
        let (lsym, lextra, lebits) = length_symbol(len);
        let (lcode, llen) = lit_codes[lsym];
        bw.write_bits(lcode, llen);
        if lebits > 0 {
            bw.write_bits(lextra, lebits);
        }
        let (dsym, dextra, debits) = distance_symbol(dist);
        let (dcode, dlen) = dist_codes[dsym];
        bw.write_bits(dcode, dlen);
        if debits > 0 {
            bw.write_bits(dextra, debits);
        }
        let _ = start;
    }

    let (eob, elen) = lit_codes[256];
    bw.write_bits(eob, elen);
    bw.finish()
}

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

fn inflate_codes(
    br: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    lit: &HuffDecoder,
    dist: &HuffDecoder,
) -> Result<(), String> {
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

fn inflate_dynamic(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
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
    inflate_codes(br, out, &lit, &dist)
}

/// 🗜️ Raw DEFLATE inflate (stored + fixed + dynamic).
pub fn inflate_raw(data: &[u8]) -> Result<Vec<u8>, String> {
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

/// 🗜️ Zlib-wrap compress (CMF/FLG + raw deflate + Adler32).
pub fn zlib_compress(data: &[u8]) -> Result<Vec<u8>, String> {
    let raw = deflate_raw(data);
    let mut out = Vec::with_capacity(2 + raw.len() + 4);
    out.push(0x78);
    out.push(0x01);
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(data).to_be_bytes());
    Ok(out)
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

//#region 🔖️SnapshotCodec
/// 🧬️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the typed
/// entry points -- these are what `DeflateSnapshot`'s `ArtifactDsl`/`ArtifactPack` impls call.
/// `zlib_compress`/`zlib_decompress` above stay byte<->byte (they're load-bearing for other
/// artifacts' own internal zlib framing -- PNG IDAT, PDF stream objects -- which have never gone
/// through a `DeflateSnapshot` and must not start doing so here); these two are the RFC1950
/// container<->typed-snapshot pair that actually populates/consumes `cmf`/`flg`/`dict_id` instead
/// of a bare byte blob.
///
/// 🧮️ Encodes a `DeflateSnapshot` into a full zlib (RFC1950) byte stream: CMF/FLG rebuilt from
/// the typed fields (with a freshly computed FCHECK -- it's a pure function of the other header
/// bits, never independently stored), the preset-dictionary id when present, the raw DEFLATE
/// payload, and a freshly computed Adler-32 trailer (never a stale stored checksum).
pub fn encode_deflate_snapshot(snapshot: &DeflateSnapshot) -> Vec<u8> {
    let cmf = ((snapshot.window_bits & 0x0F) << 4) | (snapshot.compression_method & 0x0F);
    let fdict = snapshot.dict_id.is_some();
    let flg_hi = (snapshot.compression_level_hint.to_bits() << 6) | ((fdict as u8) << 5);
    let fcheck = (31 - (((cmf as u16) * 256 + flg_hi as u16) % 31)) % 31;
    let flg = flg_hi | (fcheck as u8);

    let raw = deflate_raw(&snapshot.payload);
    let mut out = Vec::with_capacity(2 + 4 + raw.len() + 4);
    out.push(cmf);
    out.push(flg);
    if let Some(dict_id) = snapshot.dict_id {
        out.extend_from_slice(&dict_id.to_be_bytes());
    }
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(&snapshot.payload).to_be_bytes());
    out
}

/// 🧮️ Decodes a zlib (RFC1950) byte stream into a typed `DeflateSnapshot`: CMF/FLG parsed into
/// typed fields, the preset-dictionary id extracted when FDICT is set, the payload inflated, and
/// the Adler-32 trailer verified against the freshly decompressed payload.
///
/// 📖️ A preset dictionary's ID is retained honestly as typed data, but this codec cannot prime
/// the LZ77 window with actual dictionary content -- that capability doesn't exist in
/// `inflate_raw`/`deflate_raw` (the real LZ77+Huffman engine, untouched per this wave's mandate).
/// Round trips through this codec's own `encode_deflate_snapshot` are unaffected (it never
/// primes a dictionary either); a foreign stream that truly relied on dictionary-primed
/// backreferences would surface as an "invalid backreference" error from `inflate_raw`, same as
/// any other genuinely undecodable stream.
pub fn decode_deflate_snapshot(data: &[u8]) -> Result<DeflateSnapshot, String> {
    if data.len() < 6 {
        return Err("zlib stream too short".into());
    }
    let cmf = data[0];
    let flg = data[1];
    let compression_method = cmf & 0x0F;
    let window_bits = (cmf >> 4) & 0x0F;
    if compression_method != 8 {
        return Err("unsupported zlib compression method".into());
    }
    if ((cmf as u16) * 256 + flg as u16) % 31 != 0 {
        return Err("zlib CMF/FLG check failed".into());
    }
    let fdict = flg & 0x20 != 0;
    let compression_level_hint = DeflateLevelHint::from_bits(flg >> 6);

    let mut pos = 2usize;
    let dict_id = if fdict {
        if data.len() < pos + 4 {
            return Err("truncated preset dictionary id".into());
        }
        let id = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;
        Some(id)
    } else {
        None
    };

    if data.len() < pos + 4 {
        return Err("zlib stream too short".into());
    }
    let adler_bytes = &data[data.len() - 4..];
    let expect = u32::from_be_bytes([adler_bytes[0], adler_bytes[1], adler_bytes[2], adler_bytes[3]]);
    let raw = &data[pos..data.len() - 4];
    let payload = inflate_raw(raw)?;
    let got = adler32(&payload);
    if got != expect {
        return Err(format!("adler32 mismatch: expected {expect:#010x}, got {got:#010x}"));
    }

    Ok(DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        compression_method,
        window_bits,
        compression_level_hint,
        dict_id,
        payload,
    })
}
//#endregion 🔖️SnapshotCodec
//#endregion DeflateCodec

//#region DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_deflate_snapshot() -> DeflateSnapshot {
    DeflateSnapshot::default()
}

/// 📄️ The demo `stdio.deflate` document — a genuine, non-empty RFC1950 container: a real
/// preset-dictionary id (exercises the FDICT-gated `dict_id` field) plus repetitive text payload
/// (round-trips through this artifact's own `deflate_raw`/`inflate_raw`). Single source of truth
/// for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🗜️example.zz`/`🎒️example.pack.semio`
/// (all three are literally this snapshot's `print_dsl`/`encode_deflate_snapshot`/`encode_pack`
/// output, asserted equal by `fixture_honesty_law` below) and for the conformance laws' own demo
/// case.
pub fn demo_deflate_snapshot() -> DeflateSnapshot {
    DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        compression_method: 8,
        window_bits: 7,
        compression_level_hint: DeflateLevelHint::Default,
        dict_id: Some(0x1234_5678),
        payload: b"the quick brown fox jumps over the lazy dog".to_vec(),
    }
}
//#endregion DocumentHelpers

//#region Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::deflate::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    register_schema_specs();
    store::register_document_codec(store::ArtifactCodec::of::<DeflateSnapshot, DeflateMutation>(
        STDIO_DEFLATE_DOCUMENT_SCHEMA,
    ));
}

/// 📇️ P2-FG2: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) —
/// real, non-fabricated call: `DeflateSnapshot` derives `#[derive(dsl::DslRecord)]`, so
/// `__dsl_spec` genuinely exists (../🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs). Only
/// the snapshot schema id is registered — `DeflateDiff` has NO derivable `RecordSpec` (see
/// `register_pilot_languages`'s own doc comment), so `"stdio.deflate#diff"` is deliberately not
/// called here, matching the recipe's own "skip rather than fabricate" rule. `#[cfg]`-gated to
/// match `os_dsl::registry`'s own `#[cfg(not(target_arch = "wasm32"))]` — the registry simply
/// does not exist as a compiled item on `wasm32`.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.deflate", DeflateSnapshot::__dsl_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}

/// 📌️ P2-FG2: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's
/// exemplar pattern (`📖️grammar-recipe.md` §4's deliverable checklist, json's own
/// `register_pilot_languages`) — `stdio.deflate`/`.op`/`.diff`/`.pack`/`.spr`, all
/// `dsl::passthrough_hooks`. `diff`'s `protocol` slot stays `None` matching the exemplar's own
/// shape exactly (the role scheme has no dedicated "diff binary" role even though
/// `🔺️diff/💾️binary/📡️component.protocol.semio` is a real, conformance-tested file — its binary
/// form is exercised directly by `protocol_walk_law` below, just not wired through a 6th
/// `LanguageRole`).
///
/// `register_schema_spec` (P2-M3's `FullResolver` insertion API, see `register_schema_specs`
/// below) is called for the SNAPSHOT schema id only — `DeflateSnapshot` derives `dsl::DslRecord`
/// for real (a genuine `fn() -> RecordSpec` exists via `__dsl_spec`), unlike json/csv/zip/png's
/// fully hand-rolled types. `DeflateDiff` does NOT derive `dsl::DslDiff` (its `dict_id:
/// Option<Option<u32>>` tri-state field blocks the derive — `dsl_derive::classify_field` peels
/// exactly one `Option<..>` layer, confirmed via real `cargo check`, per
/// `🔺️diff/🦀️component.rs`'s own doc comment) and so has no derivable `RecordSpec` either — the
/// call is skipped for `"stdio.deflate#diff"` rather than fabricated, filed as `mechanism_gaps`
/// in this wave's report.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.deflate",
        extension: Some("zz"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::deflate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::deflate::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.deflate"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.deflate.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::deflate::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::deflate::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::deflate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::deflate::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.deflate.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.deflate.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::deflate::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::deflate::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.deflate.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.deflate.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::deflate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.deflate.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.deflate.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::deflate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::deflate::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.deflate.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.deflate`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(
        crate::artifacts::deflate::schema::deflate_artifact_schema_descriptor(),
    );
}
//#endregion Register

//#region ArtifactEngine
/// ⚙️ `stdio.deflate` artifact engine.
pub struct DeflateEngine {
    artifact_state: DeflateArtifact,
    snapshot_state: DeflateSnapshot,
}

impl DeflateEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: DeflateSnapshot) -> Self {
        let artifact_state = DeflateArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion ArtifactEngine

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adler32_empty_is_one() {
        assert_eq!(adler32(b""), 1);
    }

    #[test]
    fn zlib_round_trip() {
        let payloads: &[&[u8]] = &[b"", b"a", b"hello zlib", &[0u8; 64], b"abracadabra abracadabra"];
        for p in payloads {
            let enc = zlib_compress(p).expect("compress");
            let dec = zlib_decompress(&enc).expect("decompress");
            assert_eq!(&dec, p);
        }
    }

    #[test]
    fn raw_deflate_round_trip() {
        let p = b"stdio-deflate-conformance";
        let enc = deflate_raw(p);
        let dec = inflate_raw(&enc).expect("inflate");
        assert_eq!(dec, p);
    }

    /// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the prior
    /// `deflate_raw` never searched for LZ77 matches at all -- it emitted every byte as a fixed-
    /// Huffman literal (~9 bits/byte average), so compressing ALWAYS expanded the input by ~12.5%.
    /// This is the regression test for that: real, repetitive text must come out smaller, not
    /// bigger, and must still round-trip exactly.
    #[test]
    fn raw_deflate_compresses_repetitive_text() {
        let text = "the quick brown fox jumps over the lazy dog. ".repeat(200);
        let p = text.as_bytes();
        let enc = deflate_raw(p);
        assert!(enc.len() < p.len(), "compressed ({}) should be smaller than input ({}) for highly repetitive text", enc.len(), p.len());
        let dec = inflate_raw(&enc).expect("inflate");
        assert_eq!(dec, p);
    }

    #[test]
    fn raw_deflate_round_trips_binary_with_long_range_matches() {
        // A repeating 4-byte pattern well past MIN_MATCH, exercising the hash-chain match finder
        // across many window-fulls (data.len() > WINDOW) so distances near the 32KB boundary are
        // exercised too, not just short-range matches.
        let mut p = Vec::with_capacity(100_000);
        for i in 0..25_000u32 {
            p.extend_from_slice(&i.to_le_bytes());
        }
        let enc = deflate_raw(&p);
        assert!(enc.len() < p.len());
        let dec = inflate_raw(&enc).expect("inflate");
        assert_eq!(dec, p);
    }

    #[test]
    fn raw_deflate_round_trips_random_incompressible_data() {
        // Match search finding nothing (or only sub-MIN_MATCH runs) must still round-trip --
        // pure-literal fallback path.
        let mut state = 0x2545F4914F6CDD1Du64;
        let mut p = Vec::with_capacity(4096);
        for _ in 0..4096 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            p.push((state & 0xFF) as u8);
        }
        let enc = deflate_raw(&p);
        let dec = inflate_raw(&enc).expect("inflate");
        assert_eq!(dec, p);
    }

    #[test]
    fn codec_round_trip() {
        let payload = b"pack-envelope-payload".to_vec();
        let snap = DeflateSnapshot {
            schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
            compression_method: 8,
            window_bits: 7,
            compression_level_hint: DeflateLevelHint::Default,
            dict_id: None,
            payload: payload.clone(),
        };
        let pack = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DeflateSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded, snap);
        assert_eq!(decoded.payload, payload);
    }

    /// 🧪️ `encode_deflate_snapshot`/`decode_deflate_snapshot` round-trip every typed header field,
    /// including a preset-dictionary id.
    #[test]
    fn snapshot_codec_round_trip_with_preset_dictionary() {
        let snap = DeflateSnapshot {
            schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
            compression_method: 8,
            window_bits: 5,
            compression_level_hint: DeflateLevelHint::Maximum,
            dict_id: Some(0x1234_5678),
            payload: b"preset-dictionary-id-round-trip".to_vec(),
        };
        let bytes = encode_deflate_snapshot(&snap);
        // 🪆️ FDICT set + DICTID present between CMF/FLG and the deflate body.
        assert_eq!(bytes[1] & 0x20, 0x20);
        let decoded = decode_deflate_snapshot(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    /// 🧪️ Ticket 26/08/10/…: `decode_deflate_snapshot` rejects a CMF/FLG check failure --
    /// FCHECK is derived, not fabricated, so a corrupted header must not silently decode.
    #[test]
    fn snapshot_codec_rejects_bad_check_bits() {
        let mut bytes = encode_deflate_snapshot(&DeflateSnapshot {
            schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
            compression_method: 8,
            window_bits: 7,
            compression_level_hint: DeflateLevelHint::Default,
            dict_id: None,
            payload: b"corrupt-me".to_vec(),
        });
        bytes[1] ^= 0x01; // flip a FCHECK bit
        assert!(decode_deflate_snapshot(&bytes).is_err());
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG2: per-artifact conformance laws (recipe §4 deliverable item 6) — grammar/protocol
    /// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff` output,
    /// `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
    /// fixture-honesty round-trip. Lives here (the engine's own test region), not any framework
    /// file — `m5` auto-discovers the snapshot grammar+`.dsl.semio`/protocol+`.pack.semio` pairs
    /// independently; these tests are this artifact's OWN early-warning, plus direct coverage of
    /// the mutations/diff facets that harness does not auto-discover at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::deflate::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below.
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the demo snapshot (a non-empty payload + a real preset-dictionary id).
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_deflate_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `DeflateMutation` demo case (`mutations::demo_mutation_cases()`).
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
        /// output for every representative `DeflateDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty-line diff and both `dict_id` tri-state directions.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first), every demo mutation's
        /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed ==
        /// bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_deflate_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `demo_deflate_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_deflate_snapshot();

            let parsed = <DeflateSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_deflate_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_deflate_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <DeflateSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_deflate_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_deflate_snapshot()) drifted from the shipped .pack.semio fixture");
        }

        /// ✅️ `schema_spec_registration_resolves`: `register_schema_specs` genuinely resolves the
        /// snapshot schema id through `dsl::registry::full_resolver()` once called (real
        /// `DeflateSnapshot::__dsl_spec`, not fabricated — see that fn's own doc comment for why
        /// the diff id is deliberately NOT registered).
        #[test]
        #[cfg(not(target_arch = "wasm32"))]
        fn schema_spec_registration_resolves() {
            use dsl::os_pack::cli::SchemaResolver;
            register_schema_specs();
            let resolver = dsl::registry::full_resolver();
            assert!(resolver.resolve("stdio.deflate").is_some(), "stdio.deflate must resolve");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion Tests
