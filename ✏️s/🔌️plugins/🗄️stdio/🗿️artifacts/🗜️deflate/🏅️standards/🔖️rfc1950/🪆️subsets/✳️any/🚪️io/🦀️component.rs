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
#[derive(serde::Serialize, serde::Deserialize)]
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

#[path = "🗜️dynamic/🦀️component.rs"]
mod dynamic;

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
    if best_len >= MIN_MATCH {
        Some((best_len, best_dist))
    } else {
        None
    }
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

//#region 🧵️StreamingEncode
const DEFLATE_MAX_CHAIN: usize = 128;
const DEFLATE_CHECKPOINT_MAGIC: &[u8; 4] = b"SDJ1";

#[derive(Clone, Copy)]
struct PendingMatch {
    start: usize,
    length: usize,
    distance: usize,
}

/// 🧵️ Persistent fixed-Huffman DEFLATE encoder with bounded LZ77 transitions and portable checkpoints.
pub struct DeflateEncodeJob {
    input: Vec<u8>,
    writer: BitWriter,
    head: Vec<i32>,
    previous: Vec<i32>,
    position: usize,
    pending: Option<PendingMatch>,
    checkpoint_interval: usize,
    next_checkpoint: usize,
    complete: bool,
}

impl DeflateEncodeJob {
    /// 🌱️ Creates a deterministic encoder; `checkpoint_interval` is measured in applied input bytes.
    pub fn new(input: Vec<u8>, checkpoint_interval: usize) -> Self {
        let mut writer = BitWriter::new();
        writer.write_bits(0b011, 3);
        let checkpoint_interval = checkpoint_interval.max(1);
        Self { input, writer, head: vec![-1; HASH_SIZE], previous: vec![-1; WINDOW], position: 0, pending: None, checkpoint_interval, next_checkpoint: checkpoint_interval, complete: false }
    }

    /// 📈 Returns applied input bytes and total input bytes.
    pub fn progress(&self) -> (usize, usize) {
        (self.position, self.input.len())
    }

    /// 💾 Encodes all persistent state required for byte-identical continuation.
    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.input.len() + self.writer.out.len() + (self.head.len() + self.previous.len()) * 4 + 96);
        bytes.extend_from_slice(DEFLATE_CHECKPOINT_MAGIC);
        write_bytes(&mut bytes, &self.input);
        write_bytes(&mut bytes, &self.writer.out);
        bytes.push(self.writer.cur);
        bytes.push(self.writer.nbits);
        write_i32s(&mut bytes, &self.head);
        write_i32s(&mut bytes, &self.previous);
        write_usize(&mut bytes, self.position);
        match self.pending {
            Some(pending) => {
                bytes.push(1);
                write_usize(&mut bytes, pending.start);
                write_usize(&mut bytes, pending.length);
                write_usize(&mut bytes, pending.distance);
            }
            None => bytes.push(0),
        }
        write_usize(&mut bytes, self.checkpoint_interval);
        write_usize(&mut bytes, self.next_checkpoint);
        bytes.push(u8::from(self.complete));
        bytes
    }

    /// ♻️ Restores a job checkpoint without replaying already-applied input.
    pub fn from_checkpoint(bytes: &[u8]) -> Result<Self, String> {
        let mut reader = CheckpointReader::new(bytes);
        if reader.take(4)? != DEFLATE_CHECKPOINT_MAGIC {
            return Err("invalid DEFLATE job checkpoint magic".into());
        }
        let input = reader.read_bytes()?;
        let output = reader.read_bytes()?;
        let cur = reader.read_u8()?;
        let nbits = reader.read_u8()?;
        if nbits > 7 {
            return Err("invalid DEFLATE job bit cursor".into());
        }
        let head = reader.read_i32s()?;
        let previous = reader.read_i32s()?;
        if head.len() != HASH_SIZE || previous.len() != WINDOW {
            return Err("invalid DEFLATE job index dimensions".into());
        }
        let position = reader.read_usize()?;
        let pending = match reader.read_u8()? {
            0 => None,
            1 => Some(PendingMatch { start: reader.read_usize()?, length: reader.read_usize()?, distance: reader.read_usize()? }),
            _ => return Err("invalid DEFLATE job pending-match tag".into()),
        };
        let checkpoint_interval = reader.read_usize()?.max(1);
        let next_checkpoint = reader.read_usize()?;
        let complete = reader.read_u8()? != 0;
        if position > input.len() || !reader.is_empty() {
            return Err("invalid DEFLATE job checkpoint cursor".into());
        }
        Ok(Self { input, writer: BitWriter { out: output, cur, nbits }, head, previous, position, pending, checkpoint_interval, next_checkpoint, complete })
    }

    fn insert(&mut self, position: usize) {
        if position + MIN_MATCH <= self.input.len() {
            let hash = hash3(&self.input, position);
            self.previous[position & (WINDOW - 1)] = self.head[hash];
            self.head[hash] = position as i32;
        }
    }

    fn emit_literal(&mut self, position: usize, codes: &[(u32, u8)]) {
        let (code, length) = codes[self.input[position] as usize];
        self.writer.write_bits(code, length);
    }

    fn emit_match(&mut self, pending: PendingMatch, literal_codes: &[(u32, u8)], distance_codes: &[(u32, u8)]) {
        let (symbol, extra, extra_bits) = length_symbol(pending.length);
        let (code, length) = literal_codes[symbol];
        self.writer.write_bits(code, length);
        self.writer.write_bits(extra, extra_bits);
        let (symbol, extra, extra_bits) = distance_symbol(pending.distance);
        let (code, length) = distance_codes[symbol];
        self.writer.write_bits(code, length);
        self.writer.write_bits(extra, extra_bits);
    }

    fn process_transition(&mut self, literal_codes: &[(u32, u8)], distance_codes: &[(u32, u8)]) -> usize {
        let before = self.position;
        if self.position >= self.input.len() {
            if let Some(pending) = self.pending.take() {
                self.emit_match(pending, literal_codes, distance_codes);
            }
            let (code, length) = literal_codes[256];
            self.writer.write_bits(code, length);
            self.writer.align_byte();
            self.complete = true;
            return 1;
        }
        let next = longest_match(&self.input, self.position, &self.head, &self.previous, DEFLATE_MAX_CHAIN);
        self.insert(self.position);
        match (self.pending.take(), next) {
            (None, Some((length, distance))) => {
                self.pending = Some(PendingMatch { start: self.position, length, distance });
                self.position += 1;
            }
            (Some(pending), next) if matches!(next, Some((length, _)) if length > pending.length) => {
                self.emit_literal(pending.start, literal_codes);
                let (length, distance) = next.expect("guarded match");
                self.pending = Some(PendingMatch { start: self.position, length, distance });
                self.position += 1;
            }
            (Some(pending), _) => {
                self.emit_match(pending, literal_codes, distance_codes);
                let end = (pending.start + pending.length).min(self.input.len());
                for position in (pending.start + 2)..end {
                    self.insert(position);
                }
                self.position = end;
            }
            (None, None) => {
                self.emit_literal(self.position, literal_codes);
                self.position += 1;
            }
        }
        self.position.saturating_sub(before).max(1)
    }

    fn finish(mut self) -> Vec<u8> {
        let literal_codes = build_codes(&fixed_lit_lengths());
        let distance_codes = build_codes(&fixed_dist_lengths());
        while !self.complete {
            self.process_transition(&literal_codes, &distance_codes);
        }
        self.writer.out
    }

    fn checkpoint(&self) -> semio_framework_job::Checkpoint {
        semio_framework_job::Checkpoint { state: self.checkpoint_bytes(), applied_progress: self.position as u64 }
    }

    fn commit(&self) -> semio_framework_job::CommitCandidate {
        semio_framework_job::CommitCandidate { state: self.checkpoint_bytes(), output: self.writer.out.clone() }
    }
}

impl semio_framework_job::InteractiveJob for DeflateEncodeJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        use semio_framework_job::StepOutcome;
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        context.set_stage("deflate:encode");
        let literal_codes = build_codes(&fixed_lit_lengths());
        let distance_codes = build_codes(&fixed_dist_lengths());
        loop {
            if self.complete {
                return StepOutcome::Complete(self.commit());
            }
            let work = self.process_transition(&literal_codes, &distance_codes);
            context.consume_fuel(work as u64);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if self.complete {
                return StepOutcome::Complete(self.commit());
            }
            if self.position >= self.next_checkpoint {
                while self.next_checkpoint <= self.position {
                    self.next_checkpoint = self.next_checkpoint.saturating_add(self.checkpoint_interval);
                }
                return StepOutcome::CheckpointReady(self.checkpoint());
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
        }
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if let Some((released_items, released_bytes)) = retire_deflate_vec_step(&mut self.input, maximum_items, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = retire_deflate_vec_step(&mut self.writer.out, maximum_items, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = retire_deflate_vec_step(&mut self.head, maximum_items, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = retire_deflate_vec_step(&mut self.previous, maximum_items, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        self.pending = None;
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.input.capacity() == 0 && self.writer.out.capacity() == 0 && self.head.capacity() == 0 && self.previous.capacity() == 0 && self.pending.is_none()
    }
}

fn retire_deflate_vec_step<T>(values: &mut Vec<T>, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
    let item_bytes = std::mem::size_of::<T>();
    if !values.is_empty() {
        if maximum_items == 0 || maximum_bytes < item_bytes {
            return Some((0, 0));
        }
        drop(values.pop());
        return Some((1, item_bytes));
    }
    if values.capacity() == 0 {
        return None;
    }
    let backing_bytes = values.capacity().saturating_mul(item_bytes);
    if maximum_items == 0 || maximum_bytes < backing_bytes {
        return Some((0, 0));
    }
    drop(std::mem::take(values));
    Some((1, backing_bytes))
}

fn write_usize(bytes: &mut Vec<u8>, value: usize) {
    bytes.extend_from_slice(&(value as u64).to_le_bytes());
}

fn write_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    write_usize(bytes, value.len());
    bytes.extend_from_slice(value);
}

fn write_i32s(bytes: &mut Vec<u8>, values: &[i32]) {
    write_usize(bytes, values.len());
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
}

struct CheckpointReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> CheckpointReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], String> {
        let end = self.cursor.checked_add(length).ok_or("DEFLATE checkpoint length overflow")?;
        let value = self.bytes.get(self.cursor..end).ok_or("truncated DEFLATE job checkpoint")?;
        self.cursor = end;
        Ok(value)
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn read_usize(&mut self) -> Result<usize, String> {
        let value = u64::from_le_bytes(self.take(8)?.try_into().expect("fixed checkpoint width"));
        usize::try_from(value).map_err(|_| "DEFLATE checkpoint value exceeds usize".into())
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, String> {
        let length = self.read_usize()?;
        Ok(self.take(length)?.to_vec())
    }

    fn read_i32s(&mut self) -> Result<Vec<i32>, String> {
        let length = self.read_usize()?;
        let mut values = Vec::with_capacity(length);
        for _ in 0..length {
            values.push(i32::from_le_bytes(self.take(4)?.try_into().expect("fixed checkpoint width")));
        }
        Ok(values)
    }

    fn is_empty(&self) -> bool {
        self.cursor == self.bytes.len()
    }
}

/// 🗜️ Batch adapter over [`DeflateEncodeJob`] for non-interactive artifact codecs.
pub fn deflate_raw(data: &[u8]) -> Vec<u8> {
    DeflateEncodeJob::new(data.to_vec(), usize::MAX).finish()
}
//#endregion 🧵️StreamingEncode

//#region 🧵️TunedEncode
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
enum TunedFrame {
    Raw,
    Zlib { header: u16, adler: u32 },
}

/// 🎛️ Persistent bounded encoder for the selected Office, Illustrator, and level-nine policies.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TunedDeflateEncodeJob {
    engine: dynamic::Job,
    frame: TunedFrame,
}

impl TunedDeflateEncodeJob {
    fn classic(input: Vec<u8>, policy: dynamic::Policy, frame: TunedFrame) -> Self {
        Self { engine: dynamic::Job::classic(input, policy), frame }
    }

    /// 🗜️ Creates the deterministic Office-compatible raw encoder.
    pub fn office(input: Vec<u8>) -> Self {
        Self::classic(input, dynamic::Policy { window_bits: 15, memory_level: 8, good: 1, lazy: 4, nice: 258, chain: 1024, finish: dynamic::Finish::Sync }, TunedFrame::Raw)
    }

    /// 🔎 Creates the deterministic high-search raw encoder.
    pub fn office_high_search(input: Vec<u8>) -> Self {
        Self::classic(input, dynamic::Policy { window_bits: 15, memory_level: 8, good: 4, lazy: 4, nice: 258, chain: 4096, finish: dynamic::Finish::Sync }, TunedFrame::Raw)
    }

    /// 🧳 Creates the deterministic compact high-search raw encoder.
    pub fn office_compact_high_search(input: Vec<u8>) -> Self {
        Self::classic(input, dynamic::Policy { window_bits: 15, memory_level: 7, good: 4, lazy: 4, nice: 258, chain: 4096, finish: dynamic::Finish::Sync }, TunedFrame::Raw)
    }

    /// 🎨 Creates the deterministic Illustrator partial-flush RFC 1950 encoder.
    pub fn illustrator(input: Vec<u8>) -> Self {
        let frame = TunedFrame::Zlib { header: 0x4889, adler: adler32(&input) };
        Self::classic(input, dynamic::Policy { window_bits: 12, memory_level: 5, good: 8, lazy: 16, nice: 128, chain: 128, finish: dynamic::Finish::Partial }, frame)
    }

    /// 🏁 Creates the deterministic miniz-compatible level-nine RFC 1950 encoder.
    pub fn level_nine(input: Vec<u8>) -> Self {
        let frame = TunedFrame::Zlib { header: 0x78da, adler: adler32(&input) };
        Self { engine: dynamic::Job::miniz(input), frame }
    }

    /// 📈 Returns applied transition progress and its deterministic upper bound.
    pub fn progress(&self) -> (usize, usize) {
        self.engine.progress()
    }

    /// 💾 Captures all encoder cursors, indices, tokens, and partial output without replay.
    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("tuned DEFLATE job state is serializable")
    }

    /// ♻️ Restores a tuned encoder from [`Self::checkpoint_bytes`].
    pub fn from_checkpoint(bytes: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(bytes).map_err(|error| format!("invalid tuned DEFLATE checkpoint: {error}"))
    }

    fn output(&self) -> Vec<u8> {
        match self.frame {
            TunedFrame::Raw => self.engine.output().to_vec(),
            TunedFrame::Zlib { header, adler } => {
                let mut output = Vec::with_capacity(self.engine.output().len() + 6);
                output.extend_from_slice(&header.to_be_bytes());
                output.extend_from_slice(self.engine.output());
                output.extend_from_slice(&adler.to_be_bytes());
                output
            }
        }
    }

    fn finish(mut self) -> Vec<u8> {
        while !self.engine.step() {}
        self.output()
    }
}

impl semio_framework_job::InteractiveJob for TunedDeflateEncodeJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        use semio_framework_job::StepOutcome;
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        context.set_stage("deflate:tuned-encode");
        loop {
            if self.engine.step() {
                return StepOutcome::Complete(semio_framework_job::CommitCandidate { state: Vec::new(), output: self.output() });
            }
            context.consume_fuel(1);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
        }
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        let (complete, released_items, released_bytes) = self.engine.close_step(maximum_items, maximum_bytes);
        if complete {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.engine.terminal_is_empty()
    }
}
//#endregion 🧵️TunedEncode

fn deflate_raw_tuned(data: &[u8], memory: i32, good: i32, lazy: i32, nice: i32, chain: i32, sync: bool) -> Result<Vec<u8>, String> {
    let value = |name: &str, value: i32| usize::try_from(value).map_err(|_| format!("raw DEFLATE {name} must be non-negative"));
    Ok(TunedDeflateEncodeJob::classic(
        data.to_vec(),
        dynamic::Policy {
            window_bits: 15,
            memory_level: value("memory level", memory)?,
            good: value("good length", good)?,
            lazy: value("lazy length", lazy)?,
            nice: value("nice length", nice)?,
            chain: value("chain length", chain)?,
            finish: if sync { dynamic::Finish::Sync } else { dynamic::Finish::Raw },
        },
        TunedFrame::Raw,
    )
    .finish())
}

/// 🎯 Deterministic Office-compatible raw DEFLATE materialization for container formats.
pub fn deflate_raw_deterministic(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 8, 1, 4, 258, 1024, true)
}

/// 🖼️ Deterministic high-search raw DEFLATE materialization for vector-media payloads.
pub fn deflate_raw_deterministic_high_search(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 8, 4, 4, 258, 4096, true)
}

/// 🧳 Deterministic compact-block high-search raw DEFLATE for embedded binary payloads.
pub fn deflate_raw_deterministic_compact_high_search(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 7, 4, 4, 258, 4096, true)
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

/// 🎯 Deterministic maximum-compression RFC 1950 materialization for formats whose native
/// canonical writer requires dynamic-Huffman zlib output.
pub fn zlib_compress_deterministic(data: &[u8]) -> Result<Vec<u8>, String> {
    Ok(TunedDeflateEncodeJob::level_nine(data.to_vec()).finish())
}

/// 🎨 Deterministic Adobe Illustrator Flate materialization: its PDF producer uses a 4 KiB
/// window and closes a level-six stream with a partial flush before the final block.
pub fn zlib_compress_illustrator(data: &[u8]) -> Result<Vec<u8>, String> {
    Ok(TunedDeflateEncodeJob::illustrator(data.to_vec()).finish())
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

    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method, window_bits, compression_level_hint, dict_id, payload })
}
//#endregion 🔖️SnapshotCodec
//#endregion DeflateCodec

//#region 🧪️CodecTests
#[cfg(test)]
mod codec_tests {
    use super::*;

    #[test]
    fn deflate_job_zero_grant_preserves_input_and_one_opportunity_close_is_exact() {
        let mut job = DeflateEncodeJob::new(vec![1, 2, 3], 1);
        let pointer = job.input.as_ptr();
        semio_framework_job::InteractiveJob::begin_close(&mut job);
        assert_eq!(
            semio_framework_job::InteractiveJob::close_step(&mut job, 0, usize::MAX),
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }
        );
        assert_eq!(job.input.as_ptr(), pointer);
        let mut opportunities = 0usize;
        while !semio_framework_job::InteractiveJob::terminal_is_empty(&job) {
            let step = semio_framework_job::InteractiveJob::close_step(&mut job, 1, usize::MAX);
            if let semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } = step {
                assert!(released_items <= 1);
            }
            opportunities += 1;
            assert!(opportunities < HASH_SIZE + WINDOW + 64);
        }
    }

    fn raw_zip_member<'a>(archive: &'a [u8], wanted: &str) -> Option<&'a [u8]> {
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

    #[test]
    fn exact_pptx_bin_policy() {
        let archive = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("fixture");
        for path in ["ppt/embeddings/oleObject1.bin", "ppt/embeddings/oleObject2.bin", "ppt/embeddings/oleObject3.bin"] {
            let expected = raw_zip_member(&archive, path).expect("fixture OLE");
            let input = inflate_raw(expected).expect("inflate fixture OLE");
            let candidate = deflate_raw_deterministic_compact_high_search(&input).expect("compress fixture OLE");
            assert_eq!(candidate, expected, "embedded binary policy must reproduce {path}");
        }
    }

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
    fn illustrator_partial_flush_materialization_matches_fixture_stream() {
        let fixture = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/📄️bachelor-thesis.pdf")).expect("fixture");
        let marker = b"/Length 3362\n/Filter /FlateDecode\n>>\nstream\n";
        let start = fixture.windows(marker.len()).position(|window| window == marker).expect("Illustrator stream") + marker.len();
        let expected = &fixture[start..start + 3362];
        let decoded = zlib_decompress(expected).expect("decode Illustrator stream");
        let actual = zlib_compress_illustrator(&decoded).expect("encode Illustrator stream");
        assert_eq!(actual, expected);
    }

    #[test]
    fn raw_deflate_round_trip() {
        let p = b"stdio-deflate-conformance";
        let enc = deflate_raw(p);
        let dec = inflate_raw(&enc).expect("inflate");
        assert_eq!(dec, p);
    }

    fn drive_encode_job(mut job: DeflateEncodeJob, fuel: u64) -> Vec<u8> {
        use semio_framework_job::{root_cancel_token, Generation, InteractiveJob, OperationId, StepBudget, StepContext, StepOutcome};
        let cancel = root_cancel_token();
        let mut sequence = 0;
        loop {
            let mut context = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, u64::MAX), cancel.clone(), || 0, &mut sequence);
            match job.step(&mut context) {
                StepOutcome::Complete(commit) => return commit.output,
                StepOutcome::Yield | StepOutcome::CheckpointReady(_) => {}
                outcome => panic!("unexpected DEFLATE job outcome: {outcome:?}"),
            }
        }
    }

    #[test]
    fn streaming_encode_is_byte_identical_across_batch_sizes() {
        let payload = b"streaming DEFLATE streaming DEFLATE streaming DEFLATE".repeat(64);
        let expected = deflate_raw(&payload);
        for fuel in [1, 2, 7, 64, 1024] {
            assert_eq!(drive_encode_job(DeflateEncodeJob::new(payload.clone(), 29), fuel), expected, "fuel={fuel}");
        }
    }

    #[test]
    fn streaming_encode_matches_pre_refactor_golden_bytes() {
        assert_eq!(deflate_raw(b"stdio-deflate-stream-golden-stdio-deflate-stream-golden"), [43, 46, 73, 201, 204, 215, 77, 73, 77, 203, 73, 44, 73, 213, 45, 46, 41, 74, 77, 204, 213, 77, 207, 207, 73, 73, 205, 211, 197, 35, 7, 0]);
    }

    #[test]
    fn streaming_checkpoint_restore_is_byte_identical() {
        use semio_framework_job::{root_cancel_token, Generation, InteractiveJob, OperationId, StepBudget, StepContext, StepOutcome};
        let payload = b"checkpointed owned compression ".repeat(256);
        let expected = deflate_raw(&payload);
        let mut job = DeflateEncodeJob::new(payload, 31);
        let mut sequence = 0;
        let checkpoint = loop {
            let mut context = StepContext::new(OperationId(2), Generation(1), StepBudget::new(5, u64::MAX), root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                break checkpoint;
            }
        };
        let restored = DeflateEncodeJob::from_checkpoint(&checkpoint.state).expect("restore checkpoint");
        assert_eq!(checkpoint.applied_progress as usize, restored.progress().0);
        assert_eq!(drive_encode_job(restored, 3), expected);
    }

    #[test]
    fn streaming_encode_observes_cancellation_without_progress() {
        use semio_framework_job::{root_cancel_token, Generation, InteractiveJob, OperationId, StepBudget, StepContext, StepOutcome};
        let mut job = DeflateEncodeJob::new(vec![7; 4096], 64);
        let before = job.checkpoint_bytes();
        let cancel = root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = StepContext::new(OperationId(3), Generation(1), StepBudget::new(1, u64::MAX), cancel, || 0, &mut sequence);
        assert_eq!(job.step(&mut context), StepOutcome::Cancelled);
        assert_eq!(job.checkpoint_bytes(), before);
    }

    #[test]
    fn adversarial_streaming_transition_stays_below_watchdog_ceiling() {
        use semio_framework_job::{root_cancel_token, Generation, InteractiveJob, OperationId, StepBudget, StepContext};
        let mut input = Vec::with_capacity(256 * 1024);
        for index in 0..256 * 1024 {
            input.push(((index * 31) ^ (index >> 5)) as u8);
        }
        let mut job = DeflateEncodeJob::new(input, usize::MAX);
        let mut sequence = 0;
        let mut context = StepContext::new(OperationId(4), Generation(1), StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
        let started = std::time::Instant::now();
        let _ = job.step(&mut context);
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
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
        let snap = DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Default, dict_id: None, payload: payload.clone() };
        let pack = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DeflateSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded, snap);
        assert_eq!(decoded.payload, payload);
    }

    /// 🧪️ `encode_deflate_snapshot`/`decode_deflate_snapshot` round-trip every typed header field,
    /// including a preset-dictionary id.
    #[test]
    fn snapshot_codec_round_trip_with_preset_dictionary() {
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
    #[test]
    fn snapshot_codec_rejects_bad_check_bits() {
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
pub fn register_schema_specs() {
    semio_framework_plugin::resolve_ready(dsl::registry::register_schema_spec("stdio.deflate", DeflateSnapshot::__dsl_spec));
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}
//#endregion 🔖️RegisterSchemaSpecs
