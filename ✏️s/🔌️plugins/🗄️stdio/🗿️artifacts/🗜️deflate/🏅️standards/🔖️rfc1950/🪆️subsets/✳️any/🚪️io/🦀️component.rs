//! 🚪️ IO stdio.deflate (rfc1950/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::DeflateAnalyzer;
    use crate::artifacts::deflate::DeflateSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct DeflateComposerComposition;

    impl ArtifactComposition for DeflateComposerComposition {
        type Snapshot = DeflateSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
                return Err(ComposeError { message: "DeflateComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = DeflateAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "DeflateComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🦑️DissolvedEngineCodec
// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
// MACHINES). Pure format algorithms — Adler32, bit IO, Huffman, LZ77 (~1,080 LOC) — kept HERE
// with the codec that is their only caller, per that ticket's rule 6 ("deflate's Huffman/LZ77 is
// the clearest case"): they have no `DeflateSnapshot` dependency of their own, but they are not
// genuinely artifact-independent either (no other stdio artifact shares this exact bitstream), so
// a module-engine one level up would be a distinction without a difference. `zlib_compress`/
// `zlib_decompress` stay byte<->byte (load-bearing for other artifacts' own internal zlib framing
// — PNG IDAT, PDF stream objects — which have never gone through a `DeflateSnapshot` and must not
// start doing so here); `encode_deflate_snapshot`/`decode_deflate_snapshot` are the RFC1950
// container<->typed-snapshot pair `DeflateSnapshot`'s `ArtifactDsl`/`ArtifactPack` impls call.
use crate::artifacts::deflate::schema::snapshot::{DeflateLevelHint, DeflateSnapshot};
use crate::artifacts::deflate::STDIO_DEFLATE_DOCUMENT_SCHEMA;

//#region Adler32
/// 🧮 Adler-32 (RFC1950).
pub async fn adler32(data: &[u8]) -> u32 {
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
    async fn new() -> Self {
        Self { out: Vec::new(), cur: 0, nbits: 0 }
    }
    async fn write_bits(&mut self, mut value: u32, mut count: u8) {
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
    async fn align_byte(&mut self) {
        if self.nbits > 0 {
            self.out.push(self.cur);
            self.cur = 0;
            self.nbits = 0;
        }
    }
    async fn finish(mut self) -> Vec<u8> {
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
    async fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0, cur: 0, nbits: 0 }
    }
    async fn read_bits(&mut self, count: u8) -> Result<u32, String> {
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
    async fn align_byte(&mut self) {
        self.nbits = 0;
        self.cur = 0;
    }
}
//#endregion BitIO

//#region Huffman
async fn reverse_bits(mut v: u32, len: u8) -> u32 {
    let mut r = 0u32;
    for _ in 0..len {
        r = (r << 1) | (v & 1);
        v >>= 1;
    }
    r
}

async fn build_codes(lengths: &[u8]) -> Vec<(u32, u8)> {
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
            codes[i] = (reverse_bits(c, len).await, len);
        }
    }
    codes
}

struct HuffDecoder {
    table: Vec<Option<(u16, u8)>>,
    max_bits: u8,
}

impl HuffDecoder {
    async fn from_lengths(lengths: &[u8]) -> Result<Self, String> {
        let max_bits = lengths.iter().copied().max().unwrap_or(0);
        if max_bits > 15 {
            return Err("invalid huffman length".into());
        }
        let size = 1usize << max_bits;
        let mut table = vec![None; size.max(1)];
        let codes = build_codes(lengths).await;
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

    async fn decode(&self, br: &mut BitReader<'_>) -> Result<u16, String> {
        if self.max_bits == 0 {
            return Err("empty huffman alphabet".into());
        }
        let mut acc = 0u32;
        for len in 1..=self.max_bits {
            let bit = br.read_bits(1).await?;
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

async fn fixed_lit_lengths() -> Vec<u8> {
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

async fn fixed_dist_lengths() -> Vec<u8> {
    vec![5u8; 32]
}
//#endregion Huffman

//#region DeflateCodec
const LEN_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LEN_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

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
async fn hash3(data: &[u8], i: usize) -> usize {
    // A cheap multiplicative hash over 3 bytes; collisions just mean a longer (still correct)
    // chain walk, never a wrong match -- every candidate is verified byte-by-byte below.
    let v = (data[i] as u32) | ((data[i + 1] as u32) << 8) | ((data[i + 2] as u32) << 16);
    ((v.wrapping_mul(0x9E3779B1)) >> (32 - HASH_BITS)) as usize
}

/// 🔎️ Longest match at `pos` found by walking the hash chain, verified byte-by-byte (hash
/// collisions are possible; a wrong-length "match" would corrupt the stream, so every candidate
/// is checked in full before being accepted). `max_chain` bounds how many chain entries we walk
/// before settling -- a simple, deterministic time/ratio tradeoff (higher = better ratio, slower).
async fn longest_match(data: &[u8], pos: usize, head: &[i32], prev: &[i32], max_chain: usize) -> Option<(usize, usize)> {
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
    if best_len >= MIN_MATCH {
        Some((best_len, best_dist))
    } else {
        None
    }
}

async fn length_symbol(len: usize) -> (usize, u32, u8) {
    // Highest base <= len; LEN_BASE is ascending so a linear scan from the top is exact and simple.
    for (idx, &base) in LEN_BASE.iter().enumerate().rev() {
        if len >= base as usize {
            return (257 + idx, (len - base as usize) as u32, LEN_EXTRA[idx]);
        }
    }
    unreachable!("length_symbol called with len < MIN_MATCH")
}

async fn distance_symbol(dist: usize) -> (usize, u32, u8) {
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
pub async fn deflate_raw(data: &[u8]) -> Vec<u8> {
    let lit_codes = build_codes(&fixed_lit_lengths());
    let dist_codes = build_codes(&fixed_dist_lengths());
    let mut bw = BitWriter::new().await;
    bw.write_bits(0b011, 3).await; // BFINAL=1, BTYPE=01 (fixed Huffman)

    if data.len() < MIN_MATCH {
        for &byte in data {
            let (code, len) = lit_codes[byte as usize];
            bw.write_bits(code, len).await;
        }
        let (eob, elen) = lit_codes[256];
        bw.write_bits(eob, elen).await;
        return bw.finish().await;
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
                let better_next = matches!(next_m.await, Some((nlen, _)) if nlen > len);
                if better_next {
                    // Emit the deferred position as a literal, keep the new (better) match pending.
                    let (code, clen) = lit_codes[data[start] as usize];
                    bw.write_bits(code, clen).await;
                    if let Some((nlen, ndist)) = next_m.await {
                        pending_match = Some((pos, nlen, ndist));
                    }
                    pos += 1;
                } else {
                    // Commit the deferred match starting at `start` (== pos - 1 here).
                    let (lsym, lextra, lebits) = length_symbol(len).await;
                    let (lcode, llen) = lit_codes[lsym];
                    bw.write_bits(lcode, llen).await;
                    if lebits > 0 {
                        bw.write_bits(lextra, lebits).await;
                    }
                    let (dsym, dextra, debits) = distance_symbol(dist).await;
                    let (dcode, dlen) = dist_codes[dsym];
                    bw.write_bits(dcode, dlen).await;
                    if debits > 0 {
                        bw.write_bits(dextra, debits).await;
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
                bw.write_bits(code, clen).await;
                pos += 1;
            }
        }
    }
    if let Some((start, len, dist)) = pending_match {
        let (lsym, lextra, lebits) = length_symbol(len).await;
        let (lcode, llen) = lit_codes[lsym];
        bw.write_bits(lcode, llen).await;
        if lebits > 0 {
            bw.write_bits(lextra, lebits).await;
        }
        let (dsym, dextra, debits) = distance_symbol(dist).await;
        let (dcode, dlen) = dist_codes[dsym];
        bw.write_bits(dcode, dlen).await;
        if debits > 0 {
            bw.write_bits(dextra, debits).await;
        }
        let _ = start;
    }

    let (eob, elen) = lit_codes[256];
    bw.write_bits(eob, elen).await;
    bw.finish().await
}

#[cfg(not(target_arch = "wasm32"))]
async fn deflate_raw_tuned(data: &[u8], memory: i32, good: i32, lazy: i32, nice: i32, chain: i32, sync: bool) -> Result<Vec<u8>, String> {
    let input_len = u32::try_from(data.len()).map_err(|_| "raw DEFLATE input exceeds 4 GiB".to_string())?;
    let mut stream = libz_sys::z_stream {
        next_in: std::ptr::null_mut(),
        avail_in: 0,
        total_in: 0,
        next_out: std::ptr::null_mut(),
        avail_out: 0,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: illustrator_zlib_allocate,
        zfree: illustrator_zlib_free,
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    unsafe {
        let initialized = libz_sys::deflateInit2_(&mut stream, 4, libz_sys::Z_DEFLATED, -15, memory, libz_sys::Z_DEFAULT_STRATEGY, libz_sys::zlibVersion(), size_of::<libz_sys::z_stream>() as i32);
        if initialized != libz_sys::Z_OK {
            return Err(format!("raw DEFLATE initialization failed with status {initialized}"));
        }
        let tuned = libz_sys::deflateTune(&mut stream, good, lazy, nice, chain);
        if tuned != libz_sys::Z_OK {
            let _ = libz_sys::deflateEnd(&mut stream);
            return Err(format!("raw DEFLATE tuning failed with status {tuned}"));
        }
        let capacity = (libz_sys::deflateBound(&mut stream, data.len() as libz_sys::uLong) as usize).checked_add(32).ok_or("raw DEFLATE output capacity overflow")?;
        let output_len = u32::try_from(capacity).map_err(|_| "raw DEFLATE output exceeds 4 GiB".to_string())?;
        let mut output = vec![0u8; capacity];
        stream.next_in = data.as_ptr().cast_mut();
        stream.avail_in = input_len;
        stream.next_out = output.as_mut_ptr();
        stream.avail_out = output_len;
        let flushed = if sync { libz_sys::deflate(&mut stream, libz_sys::Z_SYNC_FLUSH) } else { libz_sys::Z_OK };
        let finished = if flushed == libz_sys::Z_OK && (!sync || stream.avail_in == 0) {
            if sync {
                stream.next_in = std::ptr::null_mut();
            }
            libz_sys::deflate(&mut stream, libz_sys::Z_FINISH)
        } else {
            libz_sys::Z_STREAM_ERROR
        };
        let written = stream.total_out as usize;
        let ended = libz_sys::deflateEnd(&mut stream);
        if flushed != libz_sys::Z_OK || finished != libz_sys::Z_STREAM_END || ended != libz_sys::Z_OK {
            return Err(format!("raw DEFLATE failed with statuses {flushed}/{finished}/{ended}"));
        }
        output.truncate(written);
        Ok(output)
    }
}

#[cfg(target_arch = "wasm32")]
async fn deflate_raw_tuned(data: &[u8], _memory: i32, _good: i32, _lazy: i32, _nice: i32, _chain: i32, _sync: bool) -> Result<Vec<u8>, String> {
    use std::io::Write;
    let mut encoder = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(data).map_err(|error| error.to_string())?;
    encoder.finish().map_err(|error| error.to_string())
}

/// 🎯 Deterministic Office-compatible raw DEFLATE materialization for container formats.
pub async fn deflate_raw_deterministic(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 8, 1, 4, 258, 1024, true).await
}

/// 🖼️ Deterministic high-search raw DEFLATE materialization for vector-media payloads.
pub async fn deflate_raw_deterministic_high_search(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 8, 4, 4, 258, 4096, true).await
}

/// 🧳 Deterministic compact-block high-search raw DEFLATE for embedded binary payloads.
pub async fn deflate_raw_deterministic_compact_high_search(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 7, 4, 4, 258, 4096, true).await
}

async fn inflate_block_stored(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
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

async fn inflate_codes(br: &mut BitReader<'_>, out: &mut Vec<u8>, lit: &HuffDecoder, dist: &HuffDecoder) -> Result<(), String> {
    loop {
        let sym = lit.decode(br).await? as usize;
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
                length += br.read_bits(extra).await? as usize;
            }
            let dsym = dist.decode(br).await? as usize;
            if dsym >= DIST_BASE.len() {
                return Err("invalid distance symbol".into());
            }
            let mut distance = DIST_BASE[dsym] as usize;
            let dextra = DIST_EXTRA[dsym];
            if dextra > 0 {
                distance += br.read_bits(dextra).await? as usize;
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

async fn inflate_dynamic(br: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
    let hlit = br.read_bits(5).await? as usize + 257;
    let hdist = br.read_bits(5).await? as usize + 1;
    let hclen = br.read_bits(4).await? as usize + 4;
    const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    let mut cl_lens = vec![0u8; 19];
    for i in 0..hclen {
        cl_lens[ORDER[i]] = br.read_bits(3).await? as u8;
    }
    let cl_dec = HuffDecoder::from_lengths(&cl_lens).await?;
    let mut lens = Vec::with_capacity(hlit + hdist);
    while lens.len() < hlit + hdist {
        let sym = cl_dec.decode(br).await? as usize;
        match sym {
            0..=15 => lens.push(sym as u8),
            16 => {
                let rep = br.read_bits(2).await? as usize + 3;
                let prev = *lens.last().ok_or("bad repeat")?;
                lens.extend(std::iter::repeat(prev).take(rep));
            }
            17 => {
                let rep = br.read_bits(3).await? as usize + 3;
                lens.extend(std::iter::repeat(0u8).take(rep));
            }
            18 => {
                let rep = br.read_bits(7).await? as usize + 11;
                lens.extend(std::iter::repeat(0u8).take(rep));
            }
            _ => return Err("bad code-length symbol".into()),
        }
    }
    if lens.len() < hlit + hdist {
        return Err("incomplete dynamic trees".into());
    }
    let lit = HuffDecoder::from_lengths(&lens[..hlit]).await?;
    let dist = HuffDecoder::from_lengths(&lens[hlit..hlit + hdist]).await?;
    inflate_codes(br, out, &lit, &dist).await
}

/// 🗜️ Raw DEFLATE inflate (stored + fixed + dynamic).
pub async fn inflate_raw(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut br = BitReader::new(data).await;
    let mut out = Vec::new();
    loop {
        let bfinal = br.read_bits(1).await?;
        let btype = br.read_bits(2).await?;
        match btype {
            0 => inflate_block_stored(&mut br, &mut out).await?,
            1 => {
                let lit = HuffDecoder::from_lengths(&fixed_lit_lengths()).await?;
                let dist = HuffDecoder::from_lengths(&fixed_dist_lengths()).await?;
                inflate_codes(&mut br, &mut out, &lit, &dist).await?;
            }
            2 => inflate_dynamic(&mut br, &mut out).await?,
            _ => return Err("reserved BTYPE".into()),
        }
        if bfinal == 1 {
            break;
        }
    }
    Ok(out)
}

/// 🗜️ Zlib-wrap compress (CMF/FLG + raw deflate + Adler32).
pub async fn zlib_compress(data: &[u8]) -> Result<Vec<u8>, String> {
    let raw = deflate_raw(data).await;
    let mut out = Vec::with_capacity(2 + raw.len() + 4);
    out.push(0x78);
    out.push(0x01);
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(data).await.to_be_bytes());
    Ok(out)
}

/// 🎯 Deterministic maximum-compression RFC 1950 materialization for formats whose native
/// canonical writer requires dynamic-Huffman zlib output.
pub async fn zlib_compress_deterministic(data: &[u8]) -> Result<Vec<u8>, String> {
    use std::io::Write;
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::new(9));
    encoder.write_all(data).map_err(|error| error.to_string())?;
    encoder.finish().map_err(|error| error.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async unsafe fn illustrator_zlib_allocate(_opaque: *mut std::ffi::c_void, items: libz_sys::uInt, item_size: libz_sys::uInt) -> *mut std::ffi::c_void {
    let Some(size) = (items as usize).checked_mul(item_size as usize) else {
        return std::ptr::null_mut();
    };
    let header = size_of::<usize>();
    let align = align_of::<usize>();
    let Some(total) = size.checked_add(header) else {
        return std::ptr::null_mut();
    };
    let Ok(layout) = std::alloc::Layout::from_size_align(total, align) else {
        return std::ptr::null_mut();
    };
    let allocation = unsafe { std::alloc::alloc(layout) } as *mut usize;
    if allocation.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        allocation.write(total);
        allocation.add(1).cast()
    }
}

#[cfg(not(target_arch = "wasm32"))]
async unsafe fn illustrator_zlib_free(_opaque: *mut std::ffi::c_void, address: *mut std::ffi::c_void) {
    if address.is_null() {
        return;
    }
    unsafe {
        let allocation = address.cast::<usize>().sub(1);
        let total = allocation.read();
        let layout = std::alloc::Layout::from_size_align_unchecked(total, align_of::<usize>());
        std::alloc::dealloc(allocation.cast(), layout);
    }
}

/// 🎨 Deterministic Adobe Illustrator Flate materialization: its PDF producer uses a 4 KiB
/// window and closes a level-six stream with a partial flush before the final block.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn zlib_compress_illustrator(data: &[u8]) -> Result<Vec<u8>, String> {
    let input_length = libz_sys::uInt::try_from(data.len()).map_err(|_| "zlib input exceeds uInt".to_string())?;
    let mut stream = libz_sys::z_stream {
        next_in: std::ptr::null_mut(),
        avail_in: 0,
        total_in: 0,
        next_out: std::ptr::null_mut(),
        avail_out: 0,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: illustrator_zlib_allocate,
        zfree: illustrator_zlib_free,
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let initialized = unsafe { libz_sys::deflateInit2_(&mut stream, 6, libz_sys::Z_DEFLATED, 12, 5, libz_sys::Z_DEFAULT_STRATEGY, libz_sys::zlibVersion(), size_of::<libz_sys::z_stream>() as std::ffi::c_int) };
    if initialized != libz_sys::Z_OK {
        return Err(format!("zlib deflateInit2 failed with {initialized}"));
    }
    let bound = unsafe { libz_sys::deflateBound(&mut stream, data.len() as libz_sys::uLong) } as usize;
    let mut output = vec![0_u8; bound.saturating_add(64)];
    let output_length = libz_sys::uInt::try_from(output.len()).map_err(|_| "zlib output bound exceeds uInt".to_string());
    let result = output_length.and_then(|output_length| {
        stream.next_in = data.as_ptr().cast_mut();
        stream.avail_in = input_length;
        stream.next_out = output.as_mut_ptr();
        stream.avail_out = output_length;
        let partial = unsafe { libz_sys::deflate(&mut stream, libz_sys::Z_PARTIAL_FLUSH) };
        if partial != libz_sys::Z_OK || stream.avail_in != 0 {
            return Err(format!("zlib partial flush failed with {partial}"));
        }
        let finish = unsafe { libz_sys::deflate(&mut stream, libz_sys::Z_FINISH) };
        if finish != libz_sys::Z_STREAM_END {
            return Err(format!("zlib finish failed with {finish}"));
        }
        Ok(stream.total_out as usize)
    });
    let ended = unsafe { libz_sys::deflateEnd(&mut stream) };
    if ended != libz_sys::Z_OK {
        return Err(format!("zlib deflateEnd failed with {ended}"));
    }
    output.truncate(result?);
    Ok(output)
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn zlib_compress_illustrator(data: &[u8]) -> Result<Vec<u8>, String> {
    zlib_compress_deterministic(data)
}

/// 🗜️ Zlib unwrap + inflate + Adler32 verify.
pub async fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
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
    let out = inflate_raw(raw).await?;
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
pub async fn encode_deflate_snapshot(snapshot: &DeflateSnapshot) -> Vec<u8> {
    let cmf = ((snapshot.window_bits & 0x0F) << 4) | (snapshot.compression_method & 0x0F);
    let fdict = snapshot.dict_id.is_some();
    let flg_hi = (snapshot.compression_level_hint.to_bits() << 6) | ((fdict as u8) << 5);
    let fcheck = (31 - (((cmf as u16) * 256 + flg_hi as u16) % 31)) % 31;
    let flg = flg_hi | (fcheck as u8);

    let raw = deflate_raw(&snapshot.payload).await;
    let mut out = Vec::with_capacity(2 + 4 + raw.len() + 4);
    out.push(cmf);
    out.push(flg);
    if let Some(dict_id) = snapshot.dict_id {
        out.extend_from_slice(&dict_id.to_be_bytes());
    }
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(&snapshot.payload).await.to_be_bytes());
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
pub async fn decode_deflate_snapshot(data: &[u8]) -> Result<DeflateSnapshot, String> {
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
    let payload = inflate_raw(raw).await?;
    let got = adler32(&payload);
    if got != expect {
        return Err(format!("adler32 mismatch: expected {expect:#010x}, got {got:#010x}"));
    }

    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method, window_bits, compression_level_hint, dict_id, payload })
}
//#endregion 🔖️SnapshotCodec
//#endregion DeflateCodec

//#region 🧪️CodecTests
#[cfg(test)]
mod codec_tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum TraceToken {
        Literal { output: usize, byte: u8 },
        Match { output: usize, length: usize, distance: usize },
    }

    async fn trace_codes(br: &mut BitReader<'_>, out: &mut Vec<u8>, lit: &HuffDecoder, dist: &HuffDecoder, tokens: &mut Vec<TraceToken>) -> Result<(), String> {
        loop {
            let sym = lit.decode(br)? as usize;
            if sym < 256 {
                tokens.push(TraceToken::Literal { output: out.len(), byte: sym as u8 });
                out.push(sym as u8);
            } else if sym == 256 {
                return Ok(());
            } else if sym <= 285 {
                let idx = sym - 257;
                let mut length = LEN_BASE[idx] as usize;
                if LEN_EXTRA[idx] > 0 {
                    length += br.read_bits(LEN_EXTRA[idx])? as usize;
                }
                let dsym = dist.decode(br)? as usize;
                let mut distance = DIST_BASE[dsym] as usize;
                if DIST_EXTRA[dsym] > 0 {
                    distance += br.read_bits(DIST_EXTRA[dsym])? as usize;
                }
                tokens.push(TraceToken::Match { output: out.len(), length, distance });
                for _ in 0..length {
                    out.push(out[out.len() - distance]);
                }
            } else {
                return Err("invalid trace symbol".into());
            }
        }
    }

    async fn trace_first_dynamic_block_details(data: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<TraceToken>), String> {
        let mut br = BitReader::new(data);
        let _final = br.read_bits(1)?;
        if br.read_bits(2)? != 2 {
            return Err("first block is not dynamic".into());
        }
        let hlit = br.read_bits(5)? as usize + 257;
        let hdist = br.read_bits(5)? as usize + 1;
        let hclen = br.read_bits(4)? as usize + 4;
        const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
        let mut cl_lens = vec![0u8; 19];
        for index in 0..hclen {
            cl_lens[ORDER[index]] = br.read_bits(3)? as u8;
        }
        let cl = HuffDecoder::from_lengths(&cl_lens)?;
        let mut lens = Vec::with_capacity(hlit + hdist);
        while lens.len() < hlit + hdist {
            match cl.decode(&mut br)? as usize {
                symbol @ 0..=15 => lens.push(symbol as u8),
                16 => {
                    let repeat = br.read_bits(2)? as usize + 3;
                    let previous = *lens.last().ok_or("trace repeat without previous length")?;
                    lens.extend(std::iter::repeat(previous).take(repeat));
                }
                17 => {
                    let repeat = br.read_bits(3)? as usize + 3;
                    lens.extend(std::iter::repeat(0).take(repeat));
                }
                18 => {
                    let repeat = br.read_bits(7)? as usize + 11;
                    lens.extend(std::iter::repeat(0).take(repeat));
                }
                _ => return Err("invalid trace code-length symbol".into()),
            }
        }
        let lit_lengths = lens[..hlit].to_vec();
        let dist_lengths = lens[hlit..hlit + hdist].to_vec();
        let lit = HuffDecoder::from_lengths(&lit_lengths)?;
        let dist = HuffDecoder::from_lengths(&dist_lengths)?;
        let mut out = Vec::new();
        let mut tokens = Vec::new();
        trace_codes(&mut br, &mut out, &lit, &dist, &mut tokens)?;
        Ok((lit_lengths, dist_lengths, tokens))
    }

    async fn trace_first_dynamic_block(data: &[u8]) -> Result<Vec<TraceToken>, String> {
        Ok(trace_first_dynamic_block_details(data)?.2)
    }

    async fn raw_zip_member<'a>(archive: &'a [u8], wanted: &str) -> Option<&'a [u8]> {
        let mut offset = 0usize;
        while archive.get(offset..offset + 4) == Some(b"PK\x03\x04") {
            let compressed = u32::from_le_bytes(archive[offset + 18..offset + 22].try_into().ok()?) as usize;
            let name_len = u16::from_le_bytes(archive[offset + 26..offset + 28].try_into().ok()?) as usize;
            let extra_len = u16::from_le_bytes(archive[offset + 28..offset + 30].try_into().ok()?) as usize;
            let name_start = offset + 30;
            let payload_start = name_start + name_len + extra_len;
            if std::str::from_utf8(&archive[name_start..name_start + name_len]).ok()? == wanted {
                return Some(&archive[payload_start..payload_start + compressed]);
            }
            offset = payload_start + compressed;
        }
        None
    }

    async fn miniz_probe_sync(input: &[u8], probes: u32) -> Vec<u8> {
        let mut compressor = miniz_oxide::deflate::core::CompressorOxide::new(probes);
        let mut output = Vec::new();
        let _ = miniz_oxide::deflate::core::compress_to_output(&mut compressor, input, miniz_oxide::deflate::core::TDEFLFlush::Sync, |chunk| {
            output.extend_from_slice(chunk);
            true
        });
        let _ = miniz_oxide::deflate::core::compress_to_output(&mut compressor, &[], miniz_oxide::deflate::core::TDEFLFlush::Finish, |chunk| {
            output.extend_from_slice(chunk);
            true
        });
        output
    }

    async fn zlib_level(input: &[u8], level: u32) -> Vec<u8> {
        use std::io::Write;
        let mut encoder = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::new(level));
        encoder.write_all(input).expect("zlib input");
        encoder.finish().expect("zlib output")
    }

    async fn zlib_tuned_sync(input: &[u8], good: i32, lazy: i32, nice: i32, chain: i32) -> Vec<u8> {
        deflate_raw_tuned(input, 8, good, lazy, nice, chain, true).expect("tuned raw DEFLATE")
    }

    async fn zlib_tuned_finish(input: &[u8], good: i32, lazy: i32, nice: i32, chain: i32) -> Vec<u8> {
        deflate_raw_tuned(input, 8, good, lazy, nice, chain, false).expect("tuned raw DEFLATE")
    }

    async fn zlib_tuned_finish_memory(input: &[u8], memory: i32, good: i32, lazy: i32, nice: i32, chain: i32) -> Vec<u8> {
        deflate_raw_tuned(input, memory, good, lazy, nice, chain, false).expect("tuned raw DEFLATE")
    }

    async fn token_divergence(expected: &[TraceToken], candidate: &[TraceToken]) -> usize {
        expected.iter().zip(candidate).position(|(left, right)| left != right).unwrap_or(expected.len().min(candidate.len()))
    }

    async fn token_at_output(tokens: &[TraceToken], output: usize) -> usize {
        tokens
            .iter()
            .position(|token| match token {
                TraceToken::Literal { output: position, .. } | TraceToken::Match { output: position, .. } => *position >= output,
            })
            .unwrap_or(tokens.len())
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_pptx_token_divergence_matrix() {
        let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
        for path in ["[Content_Types].xml", "ppt/presentation.xml", "ppt/slides/slide1.xml", "ppt/slides/slide39.xml"] {
            let expected = raw_zip_member(&archive, path).expect("raw fixture member");
            let input = inflate_raw(expected).expect("inflate fixture member");
            let candidate = miniz_probe_sync(&input, 100);
            let expected_tokens = trace_first_dynamic_block(expected).expect("expected token trace");
            let candidate_tokens = trace_first_dynamic_block(&candidate).expect("candidate token trace");
            let first = token_divergence(&expected_tokens, &candidate_tokens);
            eprintln!(
                "[DEBUG] token_trace path={path} expected_bytes={} candidate_bytes={} expected_tokens={} candidate_tokens={} first={first} expected={:?} candidate={:?}",
                expected.len(),
                candidate.len(),
                expected_tokens.len(),
                candidate_tokens.len(),
                expected_tokens.get(first),
                candidate_tokens.get(first)
            );
            let output = match expected_tokens.get(first) {
                Some(TraceToken::Literal { output, .. }) | Some(TraceToken::Match { output, .. }) => *output,
                None => input.len(),
            };
            let candidate_first = token_at_output(&candidate_tokens, output);
            eprintln!(
                "[DEBUG] token_context path={path} output={output} expected={:?} candidate={:?}",
                &expected_tokens[first.saturating_sub(4)..(first + 5).min(expected_tokens.len())],
                &candidate_tokens[candidate_first.saturating_sub(4)..(candidate_first + 5).min(candidate_tokens.len())]
            );
            for level in 1..=9 {
                let zlib = zlib_level(&input, level);
                let zlib_tokens = trace_first_dynamic_block(&zlib).expect("zlib token trace");
                let divergence = token_divergence(&expected_tokens, &zlib_tokens);
                eprintln!("[DEBUG] token_zlib path={path} level={level} bytes={} tokens={} first={divergence} expected={:?} candidate={:?}", zlib.len(), zlib_tokens.len(), expected_tokens.get(divergence), zlib_tokens.get(divergence));
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_pptx_zlib_tune_matrix() {
        let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
        let members: Vec<_> = ["[Content_Types].xml", "ppt/presentation.xml", "ppt/slides/slide1.xml", "ppt/slides/slide39.xml"]
            .into_iter()
            .map(|path| {
                let expected = raw_zip_member(&archive, path).expect("raw fixture member");
                let input = inflate_raw(expected).expect("inflate fixture member");
                let tokens = trace_first_dynamic_block(expected).expect("fixture token trace");
                (path, input, tokens)
            })
            .collect();
        let mut best = (0usize, 0i32, 0i32, 0i32, 0i32, Vec::new());
        for good in [1, 2, 4, 8, 16, 32] {
            for lazy in [1, 2, 4, 8, 16, 32, 64, 128] {
                for nice in [8, 16, 32, 64, 128, 258] {
                    for chain in [16, 32, 64, 128, 256, 512, 1024, 2048, 4096] {
                        let mut score = 0usize;
                        let mut divergences = Vec::new();
                        for (_, input, expected) in &members {
                            let candidate = zlib_tuned_sync(input, good, lazy, nice, chain);
                            let tokens = trace_first_dynamic_block(&candidate).expect("tuned token trace");
                            let divergence = token_divergence(expected, &tokens);
                            score += divergence;
                            divergences.push(divergence);
                        }
                        if score > best.0 {
                            best = (score, good, lazy, nice, chain, divergences);
                        }
                    }
                }
            }
        }
        eprintln!("[DEBUG] zlib_tune best={best:?} paths={:?}", members.iter().map(|member| member.0).collect::<Vec<_>>());
        for (path, input, _) in &members {
            let expected = raw_zip_member(&archive, path).expect("raw fixture member");
            let candidate = zlib_tuned_sync(input, best.1, best.2, best.3, best.4);
            let prefix = expected.iter().zip(&candidate).position(|(left, right)| left != right).unwrap_or(expected.len().min(candidate.len()));
            eprintln!("[DEBUG] zlib_tune_exact path={path} expected={} candidate={} prefix={prefix} exact={}", expected.len(), candidate.len(), expected == candidate);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_pptx_emf_tune_matrix() {
        let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
        let members: Vec<_> = ["ppt/media/image9.emf", "ppt/media/image10.emf", "ppt/media/image11.emf"]
            .into_iter()
            .map(|path| {
                let expected = raw_zip_member(&archive, path).expect("fixture EMF");
                (path, inflate_raw(expected).expect("inflate fixture EMF"), expected)
            })
            .collect();
        let mut exact = None;
        let mut best = (0usize, 0i32, 0i32, 0i32, 0i32, Vec::new());
        for good in [1, 2, 4, 8, 16, 32] {
            for lazy in [1, 2, 4, 8, 16, 32, 64, 128, 258] {
                for nice in [8, 16, 32, 64, 128, 258] {
                    for chain in [16, 32, 64, 128, 256, 512, 1024, 2048, 4096] {
                        let mut score = 0usize;
                        let mut prefixes = Vec::new();
                        let mut all_exact = true;
                        for (_, input, expected) in &members {
                            let candidate = zlib_tuned_sync(input, good, lazy, nice, chain);
                            let prefix = expected.iter().zip(&candidate).position(|(left, right)| left != right).unwrap_or(expected.len().min(candidate.len()));
                            score += prefix;
                            prefixes.push((prefix, candidate.len(), expected.len()));
                            all_exact &= candidate == *expected;
                        }
                        if score > best.0 {
                            best = (score, good, lazy, nice, chain, prefixes);
                        }
                        if all_exact {
                            exact = Some((good, lazy, nice, chain));
                        }
                    }
                }
            }
        }
        eprintln!("[DEBUG] emf_tune paths={:?} exact={exact:?} best={best:?}", members.iter().map(|member| member.0).collect::<Vec<_>>());
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_pptx_bin_tune_matrix() {
        let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
        let expected = raw_zip_member(&archive, "ppt/embeddings/oleObject1.bin").expect("fixture OLE");
        let input = inflate_raw(expected).expect("inflate fixture OLE");
        let expected_tokens = trace_first_dynamic_block(expected).expect("fixture OLE token trace");
        let mut exact = None;
        let mut best = (0usize, 0i32, 0i32, 0i32, 0i32, 0usize);
        let mut closest = (usize::MAX, 0i32, 0i32, 0i32, 0i32, 0usize);
        for good in [1, 2, 4, 8, 16, 32] {
            for lazy in [1, 2, 4, 8] {
                for nice in [8, 16, 32, 64, 128, 258] {
                    for chain in [16, 32, 64, 128, 256, 512, 1024, 2048, 4096] {
                        let candidate = zlib_tuned_finish(&input, good, lazy, nice, chain);
                        let tokens = trace_first_dynamic_block(&candidate).expect("candidate OLE token trace");
                        let divergence = token_divergence(&expected_tokens, &tokens);
                        if divergence > best.0 {
                            best = (divergence, good, lazy, nice, chain, candidate.len());
                        }
                        let size_difference = candidate.len().abs_diff(expected.len());
                        if size_difference < closest.0 {
                            closest = (size_difference, good, lazy, nice, chain, candidate.len());
                        }
                        if candidate == expected {
                            exact = Some((good, lazy, nice, chain));
                        }
                    }
                }
            }
        }
        eprintln!("[DEBUG] bin_tune input={} expected={} expected_tokens={} exact={exact:?} best_tokens={best:?} closest_size={closest:?}", input.len(), expected.len(), expected_tokens.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_pptx_bin_token_lineage() {
        let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
        let expected = raw_zip_member(&archive, "ppt/embeddings/oleObject1.bin").expect("fixture OLE");
        let input = inflate_raw(expected).expect("inflate fixture OLE");
        let expected_tokens = trace_first_dynamic_block(expected).expect("fixture OLE token trace");
        for level in 1..=9 {
            let candidate = zlib_level(&input, level);
            let tokens = trace_first_dynamic_block(&candidate).expect("zlib OLE token trace");
            let divergence = token_divergence(&expected_tokens, &tokens);
            eprintln!("[DEBUG] bin_lineage backend=zlib level={level} bytes={} first={divergence} expected={:?} candidate={:?}", candidate.len(), expected_tokens.get(divergence), tokens.get(divergence));
        }
        for probes in [1, 4, 16, 64, 100, 256, 1024, 4095] {
            let candidate = miniz_probe_sync(&input, probes);
            let tokens = trace_first_dynamic_block(&candidate).expect("miniz OLE token trace");
            let divergence = token_divergence(&expected_tokens, &tokens);
            eprintln!("[DEBUG] bin_lineage backend=miniz probes={probes} bytes={} first={divergence} expected={:?} candidate={:?}", candidate.len(), expected_tokens.get(divergence), tokens.get(divergence));
        }
        let candidate = zlib_tuned_finish(&input, 4, 4, 258, 4096);
        let (expected_lit, expected_dist, _) = trace_first_dynamic_block_details(expected).expect("fixture OLE details");
        let (candidate_lit, candidate_dist, _) = trace_first_dynamic_block_details(&candidate).expect("candidate OLE details");
        let lit_differences = expected_lit.iter().zip(&candidate_lit).enumerate().filter(|(_, (left, right))| left != right).map(|(index, (left, right))| (index, *left, *right)).collect::<Vec<_>>();
        let dist_differences = expected_dist.iter().zip(&candidate_dist).enumerate().filter(|(_, (left, right))| left != right).map(|(index, (left, right))| (index, *left, *right)).collect::<Vec<_>>();
        eprintln!(
            "[DEBUG] bin_huffman expected_lit={} candidate_lit={} lit_differences={lit_differences:?} expected_dist={} candidate_dist={} dist_differences={dist_differences:?}",
            expected_lit.len(),
            candidate_lit.len(),
            expected_dist.len(),
            candidate_dist.len()
        );
        for memory in 1..=9 {
            let candidate = zlib_tuned_finish_memory(&input, memory, 4, 4, 258, 4096);
            let tokens = trace_first_dynamic_block(&candidate).expect("memory OLE token trace");
            let divergence = token_divergence(&expected_tokens, &tokens);
            let prefix = expected.iter().zip(&candidate).position(|(left, right)| left != right).unwrap_or(expected.len().min(candidate.len()));
            eprintln!("[DEBUG] bin_memory memory={memory} bytes={} tokens={} first_token={divergence} prefix={prefix} exact={}", candidate.len(), tokens.len(), candidate == expected);
        }
        let candidate = deflate_raw_tuned(&input, 7, 4, 4, 258, 4096, true).expect("memory-seven sync OLE");
        let prefix = expected.iter().zip(&candidate).position(|(left, right)| left != right).unwrap_or(expected.len().min(candidate.len()));
        eprintln!("[DEBUG] bin_memory_sync bytes={} prefix={prefix} exact={}", candidate.len(), candidate == expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_pptx_bin_policy() {
        let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
        for path in ["ppt/embeddings/oleObject1.bin", "ppt/embeddings/oleObject2.bin", "ppt/embeddings/oleObject3.bin"] {
            let expected = raw_zip_member(&archive, path).expect("fixture OLE");
            let input = inflate_raw(expected).expect("inflate fixture OLE");
            let candidate = deflate_raw_deterministic_compact_high_search(&input).expect("compress fixture OLE");
            assert_eq!(candidate, expected, "embedded binary policy must reproduce {path}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_pptx_first_member_backend_matrix() {
        let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
        let name_len = u16::from_le_bytes([archive[26], archive[27]]) as usize;
        let extra_len = u16::from_le_bytes([archive[28], archive[29]]) as usize;
        let compressed_len = u32::from_le_bytes([archive[18], archive[19], archive[20], archive[21]]) as usize;
        let start = 30 + name_len + extra_len;
        let expected = &archive[start..start + compressed_len];
        let input = inflate_raw(expected).expect("inflate fixture member");
        for level in 0..=10 {
            let candidate = miniz_oxide::deflate::compress_to_vec(&input, level);
            let prefix = candidate.iter().zip(expected).position(|(left, right)| left != right).unwrap_or(candidate.len().min(expected.len()));
            eprintln!("[DEBUG] backend=miniz_oxide level={level} len={} exact={} prefix={prefix} head={:02x?}", candidate.len(), candidate == expected, &candidate[..candidate.len().min(8)]);
            for (schedule, flush) in [("sync", miniz_oxide::deflate::core::TDEFLFlush::Sync), ("full", miniz_oxide::deflate::core::TDEFLFlush::Full)] {
                let flags = miniz_oxide::deflate::core::create_comp_flags_from_zip_params(level.into(), 0, 0);
                let mut compressor = miniz_oxide::deflate::core::CompressorOxide::new(flags);
                let mut scheduled = Vec::new();
                let _ = miniz_oxide::deflate::core::compress_to_output(&mut compressor, &input, flush, |chunk| {
                    scheduled.extend_from_slice(chunk);
                    true
                });
                let _ = miniz_oxide::deflate::core::compress_to_output(&mut compressor, &[], miniz_oxide::deflate::core::TDEFLFlush::Finish, |chunk| {
                    scheduled.extend_from_slice(chunk);
                    true
                });
                let prefix = scheduled.iter().zip(expected).position(|(left, right)| left != right).unwrap_or(scheduled.len().min(expected.len()));
                eprintln!("[DEBUG] backend=miniz_oxide level={level} schedule={schedule} len={} exact={} prefix={prefix} head={:02x?}", scheduled.len(), scheduled == expected, &scheduled[..scheduled.len().min(8)]);
            }
        }
        for level in 0..=9 {
            let mut output = vec![0u8; input.len() * 2 + 1024];
            let config = zlib_rs::DeflateConfig { level, window_bits: -15, ..zlib_rs::DeflateConfig::default() };
            let (candidate, status) = zlib_rs::compress_slice(&mut output, &input, config);
            let prefix = candidate.iter().zip(expected).position(|(left, right)| left != right).unwrap_or(candidate.len().min(expected.len()));
            eprintln!("[DEBUG] backend=zlib-rs level={level} len={} exact={} prefix={prefix} head={:02x?} status={status:?}", candidate.len(), candidate == expected, &candidate[..candidate.len().min(8)]);
        }
        let mut candidate = Vec::new();
        zopfli::compress(zopfli::Options::default(), zopfli::Format::Deflate, input.as_slice(), &mut candidate).expect("zopfli");
        eprintln!("[DEBUG] backend=zopfli len={} exact={}", candidate.len(), candidate == expected);
        let mut best = (0usize, 0u32, 0usize, false, false);
        let mut exact = None;
        let mut same_length = Vec::new();
        for probes in 1..=0x0fff_u32 {
            for greedy in [false, true] {
                for filtered in [false, true] {
                    let mut flags = probes;
                    if greedy {
                        flags |= miniz_oxide::deflate::core::deflate_flags::TDEFL_GREEDY_PARSING_FLAG;
                    }
                    if filtered {
                        flags |= miniz_oxide::deflate::core::deflate_flags::TDEFL_FILTER_MATCHES;
                    }
                    let mut compressor = miniz_oxide::deflate::core::CompressorOxide::new(flags);
                    let mut scheduled = Vec::new();
                    let _ = miniz_oxide::deflate::core::compress_to_output(&mut compressor, &input, miniz_oxide::deflate::core::TDEFLFlush::Sync, |chunk| {
                        scheduled.extend_from_slice(chunk);
                        true
                    });
                    let _ = miniz_oxide::deflate::core::compress_to_output(&mut compressor, &[], miniz_oxide::deflate::core::TDEFLFlush::Finish, |chunk| {
                        scheduled.extend_from_slice(chunk);
                        true
                    });
                    let prefix = scheduled.iter().zip(expected).position(|(left, right)| left != right).unwrap_or(scheduled.len().min(expected.len()));
                    if prefix > best.0 {
                        best = (prefix, probes, scheduled.len(), greedy, filtered);
                    }
                    if scheduled.len() == expected.len() {
                        same_length.push((probes, greedy, filtered, prefix));
                    }
                    if scheduled == expected {
                        exact = Some((probes, greedy, filtered));
                    }
                }
            }
        }
        eprintln!("[DEBUG] backend=miniz_oxide probe_matrix exact={exact:?} best={best:?} same_length_count={} same_length_best={:?}", same_length.len(), same_length.iter().max_by_key(|candidate| candidate.3));
    }

    #[semio_framework_async_macros::async_test]
    async fn adler32_empty_is_one() {
        assert_eq!(adler32(b""), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn zlib_round_trip() {
        let payloads: &[&[u8]] = &[b"", b"a", b"hello zlib", &[0u8; 64], b"abracadabra abracadabra"];
        for p in payloads {
            let enc = zlib_compress(p).expect("compress");
            let dec = zlib_decompress(&enc).expect("decompress");
            assert_eq!(&dec, p);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn illustrator_partial_flush_materialization_matches_fixture_stream() {
        let fixture = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/📄️bachelor-thesis.pdf")).expect("fixture");
        let marker = b"/Length 3362\n/Filter /FlateDecode\n>>\nstream\n";
        let start = fixture.windows(marker.len()).position(|window| window == marker).expect("Illustrator stream") + marker.len();
        let expected = &fixture[start..start + 3362];
        let decoded = zlib_decompress(expected).expect("decode Illustrator stream");
        let actual = zlib_compress_illustrator(&decoded).expect("encode Illustrator stream");
        assert_eq!(actual, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn raw_deflate_round_trip() {
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
    #[semio_framework_async_macros::async_test]
    async fn raw_deflate_compresses_repetitive_text() {
        let text = "the quick brown fox jumps over the lazy dog. ".repeat(200);
        let p = text.as_bytes();
        let enc = deflate_raw(p);
        assert!(enc.len() < p.len(), "compressed ({}) should be smaller than input ({}) for highly repetitive text", enc.len(), p.len());
        let dec = inflate_raw(&enc).expect("inflate");
        assert_eq!(dec, p);
    }

    #[semio_framework_async_macros::async_test]
    async fn raw_deflate_round_trips_binary_with_long_range_matches() {
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

    #[semio_framework_async_macros::async_test]
    async fn raw_deflate_round_trips_random_incompressible_data() {
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

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let payload = b"pack-envelope-payload".to_vec();
        let snap = DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Default, dict_id: None, payload: payload.clone() };
        let pack = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DeflateSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded, snap);
        assert_eq!(decoded.payload, payload);
    }

    /// 🧪️ `encode_deflate_snapshot`/`decode_deflate_snapshot` round-trip every typed header field,
    /// including a preset-dictionary id.
    #[semio_framework_async_macros::async_test]
    async fn snapshot_codec_round_trip_with_preset_dictionary() {
        let snap =
            DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 5, compression_level_hint: DeflateLevelHint::Maximum, dict_id: Some(0x1234_5678), payload: b"preset-dictionary-id-round-trip".to_vec() };
        let bytes = encode_deflate_snapshot(&snap);
        // 🪆️ FDICT set + DICTID present between CMF/FLG and the deflate body.
        assert_eq!(bytes[1] & 0x20, 0x20);
        let decoded = decode_deflate_snapshot(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    /// 🧪️ Ticket 26/08/10/…: `decode_deflate_snapshot` rejects a CMF/FLG check failure --
    /// FCHECK is derived, not fabricated, so a corrupted header must not silently decode.
    #[semio_framework_async_macros::async_test]
    async fn snapshot_codec_rejects_bad_check_bits() {
        let mut bytes =
            encode_deflate_snapshot(&DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Default, dict_id: None, payload: b"corrupt-me".to_vec() });
        bytes[1] ^= 0x01; // flip a FCHECK bit
        assert!(decode_deflate_snapshot(&bytes).is_err());
    }
}
//#endregion 🧪️CodecTests
//#endregion 🦑️DissolvedEngineCodec

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — pure `ComposerEntry` aggregation, no engine needed. NOTE: always reach this via a
/// fully-qualified path (`standards::v_rfc1950::subsets::any::io::io_registry::entries()`) — the
/// artifact root's OWN `io_registry` (`🗿️artifacts/🗜️deflate/🦀️component.rs`) shadows this name
/// with a DIFFERENT return type (`&'static [&'static ComposerEntry]` vs this module's
/// `&'static [ComposerEntry]`); a bare `io_registry::entries()` silently rebinds to the wrong one.
pub mod io_registry {
    use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::DeflateComposer as DeflateRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<DeflateRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🔖️RegisterSchemaSpecs
/// 📇️ P2-FG2: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) —
/// real, non-fabricated call: `DeflateSnapshot` derives `#[derive(dsl::DslRecord)]`, so
/// `__dsl_spec` genuinely exists (`../🧬️schema/📸️snapshot/🦀️component.rs`). Only the snapshot
/// schema id is registered — `DeflateDiff` has NO derivable `RecordSpec` (its `dict_id:
/// Option<Option<u32>>` tri-state field blocks the derive), so `"stdio.deflate#diff"` is
/// deliberately not called here, matching the recipe's own "skip rather than fabricate" rule.
/// Dissolved out of the former `⚙️engine::register_schema_specs` — this is NOT one of stdio's 10
/// protected imperative plugin-root calls; it is a narrow `.setup(...)` gap-filler survivor of
/// `deflate::declaration()`'s own conversion, repointed directly at this new location by stdio's
/// plugin root (`🗄️stdio/🦀️component.rs`).
#[cfg(not(target_arch = "wasm32"))]
pub async fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.deflate", DeflateSnapshot::__dsl_spec);
}

#[cfg(target_arch = "wasm32")]
pub async fn register_schema_specs() {}
//#endregion 🔖️RegisterSchemaSpecs
