//! 🗜️ First-party raw DEFLATE (RFC 1951) codec. Zero runtime dependency; `miniz_oxide` survives
//! only as the `[dev-dependencies]` differential oracle, see the `🧪️Oracle` test region.
//! `inflate`/`Inflater` decompress anything `miniz_oxide::deflate::compress_to_vec` already
//! produced (persisted `.spk`/`.spr` payloads must keep decoding). `compress` is a real LZ77 match
//! finder over a single fixed-Huffman block (`BTYPE=1`) — not a stored-block fallback — so segment
//! payloads still shrink. Only raw DEFLATE is implemented (no zlib/RFC 1950 wrapper): every call
//! site (`📡️replication/⚙️codec`) already used `miniz_oxide::DataFormat::Raw`. See
//! <https://www.rfc-editor.org/rfc/rfc1951>.

//#region 🔖️Errors
/// 🚨️ Every fallible entry point in this module returns this — deliberately small since callers
/// (`📡️replication`'s `PackError`) fold it into their own richer error type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeflateError {
    BadBlockType,
    BadStoredLength,
    BadHuffmanCode,
    BadDistance,
    UnexpectedEnd,
    OutputLimitExceeded,
}
//#endregion 🔖️Errors

//#region 🔖️Tables
const LENGTH_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LENGTH_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE: [u32; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
const CLC_ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

fn length_index_for(len: u16) -> usize {
    for index in (0..LENGTH_BASE.len()).rev() {
        if len >= LENGTH_BASE[index] {
            return index;
        }
    }
    0
}

fn distance_index_for(distance: u32) -> usize {
    for index in (0..DIST_BASE.len()).rev() {
        if distance >= DIST_BASE[index] {
            return index;
        }
    }
    0
}

fn fixed_literal_length_lengths() -> [u8; 288] {
    let mut lengths = [0u8; 288];
    lengths[0..144].fill(8);
    lengths[144..256].fill(9);
    lengths[256..280].fill(7);
    lengths[280..288].fill(8);
    lengths
}

fn fixed_distance_lengths() -> [u8; 30] {
    [5u8; 30]
}

/// 🔢️ RFC 1951 §3.2.2's canonical-code assignment: codes ordered first by length then by symbol,
/// consecutive within a length. Used by the encoder; the decoder's `Huffman::build` derives the
/// same assignment implicitly from `counts`/`symbols` without ever materializing the bit patterns.
fn canonical_codes(lengths: &[u8]) -> Vec<u16> {
    let max_len = lengths.iter().copied().max().unwrap_or(0) as usize;
    let mut bl_count = vec![0u16; max_len + 1];
    for &len in lengths {
        if len > 0 {
            bl_count[len as usize] += 1;
        }
    }
    let mut code = 0u16;
    let mut next_code = vec![0u16; max_len + 1];
    for bits in 1..=max_len {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }
    let mut codes = vec![0u16; lengths.len()];
    for (symbol, &len) in lengths.iter().enumerate() {
        if len > 0 {
            codes[symbol] = next_code[len as usize];
            next_code[len as usize] += 1;
        }
    }
    codes
}
//#endregion 🔖️Tables

//#region 🔖️BitIo
struct BitReader {
    buffer: u32,
    bits: u32,
}

impl BitReader {
    fn new() -> Self {
        Self { buffer: 0, bits: 0 }
    }

    fn push_byte(&mut self, byte: u8) {
        self.buffer |= (byte as u32) << self.bits;
        self.bits += 8;
    }

    fn ensure(&mut self, want: u32, pending: &mut Option<u8>, input_complete: bool) -> Result<bool, DeflateError> {
        while self.bits < want {
            if let Some(byte) = pending.take() {
                self.push_byte(byte);
                continue;
            }
            return if input_complete { Err(DeflateError::UnexpectedEnd) } else { Ok(false) };
        }
        Ok(true)
    }

    fn ensure_huffman(&mut self, pending: &mut Option<u8>, input_complete: bool) -> Result<bool, DeflateError> {
        match self.ensure(15, pending, input_complete) {
            Ok(ready) => Ok(ready),
            Err(DeflateError::UnexpectedEnd) => Ok(true),
            Err(error) => Err(error),
        }
    }

    fn peek(&self, n: u32) -> u32 {
        if n == 0 {
            0
        } else {
            self.buffer & ((1u32 << n) - 1)
        }
    }

    fn drop_bits(&mut self, n: u32) {
        self.buffer >>= n;
        self.bits = self.bits.saturating_sub(n);
    }

    fn take(&mut self, n: u32) -> u32 {
        let value = self.peek(n);
        self.drop_bits(n);
        value
    }

    fn take_bit(&mut self) -> Option<u32> {
        (self.bits > 0).then(|| self.take(1))
    }

    fn align_byte(&mut self) {
        let remainder = self.bits % 8;
        self.drop_bits(remainder);
    }
}

struct BitWriter {
    bytes: Vec<u8>,
    buffer: u32,
    bits: u32,
}

impl BitWriter {
    fn new() -> Self {
        Self { bytes: Vec::new(), buffer: 0, bits: 0 }
    }

    fn write_bits(&mut self, value: u32, n: u32) {
        if n == 0 {
            return;
        }
        self.buffer |= (value & ((1u32 << n) - 1)) << self.bits;
        self.bits += n;
        while self.bits >= 8 {
            self.bytes.push((self.buffer & 0xff) as u8);
            self.buffer >>= 8;
            self.bits -= 8;
        }
    }

    fn write_huffman_code(&mut self, code: u16, len: u8) {
        for shift in (0..len).rev() {
            self.write_bits(((code >> shift) & 1) as u32, 1);
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.bits > 0 {
            self.bytes.push((self.buffer & 0xff) as u8);
        }
        self.bytes
    }
}
//#endregion 🔖️BitIo

//#region 🔖️Huffman
/// 🌲️ A canonical Huffman decode table built the same way `puff.c` (Mark Adler's public-domain
/// reference `inflate`) does: `counts[len]` plus `symbols` sorted by `(len, code)`, decoded
/// bit-by-bit by tracking the running `(code, first, index)` triple per length — no bit-reversed
/// lookup table needed.
struct Huffman {
    counts: [u16; 16],
    symbols: Vec<u16>,
}

impl Huffman {
    fn build(lengths: &[u8]) -> Result<Self, DeflateError> {
        let mut counts = [0u16; 16];
        for &len in lengths {
            if len as usize >= counts.len() {
                return Err(DeflateError::BadHuffmanCode);
            }
            counts[len as usize] += 1;
        }
        counts[0] = 0;
        let mut offsets = [0u16; 16];
        for len in 1..16 {
            offsets[len] = offsets[len - 1] + counts[len - 1];
        }
        let mut symbols = vec![0u16; lengths.len()];
        for (symbol, &len) in lengths.iter().enumerate() {
            if len != 0 {
                symbols[offsets[len as usize] as usize] = symbol as u16;
                offsets[len as usize] += 1;
            }
        }
        Ok(Self { counts, symbols })
    }

    fn decode(&self, reader: &mut BitReader) -> Result<u16, DeflateError> {
        let mut code: i32 = 0;
        let mut first: i32 = 0;
        let mut index: i32 = 0;
        for len in 1..16 {
            code |= reader.take_bit().ok_or(DeflateError::UnexpectedEnd)? as i32;
            let count = self.counts[len] as i32;
            if code - first < count {
                return Ok(self.symbols[(index + (code - first)) as usize]);
            }
            index += count;
            first += count;
            first <<= 1;
            code <<= 1;
        }
        Err(DeflateError::BadHuffmanCode)
    }
}
//#endregion 🔖️Huffman

//#region 🔖️Inflate
/// 🚦️ What one `Inflater::advance` call produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InflateOutcome {
    NeedInput,
    Wrote(u8),
    Done,
}

enum Phase {
    BlockHeader,
    StoredLen,
    StoredCopy { remaining: u16 },
    DynamicCounts,
    DynamicClcLengths { read: usize, hclen: usize, hlit: usize, hdist: usize, clc_lengths: [u8; 19] },
    DynamicCodeLengths { clc: Huffman, hlit: usize, hdist: usize, lengths: Vec<u8> },
    DynamicRepeatPrev { clc: Huffman, hlit: usize, hdist: usize, lengths: Vec<u8>, prev: u8 },
    DynamicRepeatZero { clc: Huffman, hlit: usize, hdist: usize, lengths: Vec<u8>, bits: u32, base: u32 },
    DecodeSymbol { lit_len: Huffman, dist: Huffman },
    LengthExtra { lit_len: Huffman, dist: Huffman, base_len: u16, extra: u8 },
    DecodeDistanceSymbol { lit_len: Huffman, dist: Huffman, length: u16 },
    DistanceExtra { lit_len: Huffman, dist: Huffman, length: u16, base_dist: u32, extra: u8 },
    CopyMatch { lit_len: Huffman, dist: Huffman, distance: u32, remaining: u16 },
    Done,
}

/// 🌊️ Resumable raw-DEFLATE decoder that yields exactly one output byte (or `NeedInput`/`Done`)
/// per `advance` call, driven by admitting at most one pending input byte at a time — the exact
/// shape `📡️replication`'s mounted pack reader needs to hand back control between I/O grants.
/// Keeps every byte it has produced so far (`output`) so LZ77 back-references can index into it;
/// callers that only need a one-shot `Vec<u8>` should use `inflate` instead.
pub struct Inflater {
    reader: BitReader,
    output: Vec<u8>,
    phase: Phase,
    final_block: bool,
}

impl Default for Inflater {
    fn default() -> Self {
        Self::new()
    }
}

impl Inflater {
    pub fn new() -> Self {
        Self { reader: BitReader::new(), output: Vec::new(), phase: Phase::BlockHeader, final_block: false }
    }

    /// ▶️ Advances the state machine by at most one admitted input byte, producing at most one
    /// output byte. `pending` is taken (set to `None`) exactly when this call consumed it.
    pub fn advance(&mut self, pending: &mut Option<u8>, input_complete: bool) -> Result<InflateOutcome, DeflateError> {
        loop {
            let phase = std::mem::replace(&mut self.phase, Phase::Done);
            match phase {
                Phase::Done => {
                    self.phase = Phase::Done;
                    return Ok(InflateOutcome::Done);
                }
                Phase::BlockHeader => {
                    if !self.reader.ensure(3, pending, input_complete)? {
                        self.phase = Phase::BlockHeader;
                        return Ok(InflateOutcome::NeedInput);
                    }
                    self.final_block = self.reader.take(1) == 1;
                    let btype = self.reader.take(2);
                    match btype {
                        0 => {
                            self.reader.align_byte();
                            self.phase = Phase::StoredLen;
                        }
                        1 => {
                            let lit_len = Huffman::build(&fixed_literal_length_lengths())?;
                            let dist = Huffman::build(&fixed_distance_lengths())?;
                            self.phase = Phase::DecodeSymbol { lit_len, dist };
                        }
                        2 => self.phase = Phase::DynamicCounts,
                        _ => return Err(DeflateError::BadBlockType),
                    }
                }
                Phase::StoredLen => {
                    if !self.reader.ensure(32, pending, input_complete)? {
                        self.phase = Phase::StoredLen;
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let len = self.reader.take(16) as u16;
                    let nlen = self.reader.take(16) as u16;
                    if len != !nlen {
                        return Err(DeflateError::BadStoredLength);
                    }
                    self.phase = Phase::StoredCopy { remaining: len };
                }
                Phase::StoredCopy { remaining } => {
                    if remaining == 0 {
                        self.phase = if self.final_block { Phase::Done } else { Phase::BlockHeader };
                        continue;
                    }
                    if !self.reader.ensure(8, pending, input_complete)? {
                        self.phase = Phase::StoredCopy { remaining };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let byte = self.reader.take(8) as u8;
                    self.output.push(byte);
                    self.phase = Phase::StoredCopy { remaining: remaining - 1 };
                    return Ok(InflateOutcome::Wrote(byte));
                }
                Phase::DynamicCounts => {
                    if !self.reader.ensure(14, pending, input_complete)? {
                        self.phase = Phase::DynamicCounts;
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let hlit = self.reader.take(5) as usize + 257;
                    let hdist = self.reader.take(5) as usize + 1;
                    let hclen = self.reader.take(4) as usize + 4;
                    self.phase = Phase::DynamicClcLengths { read: 0, hclen, hlit, hdist, clc_lengths: [0u8; 19] };
                }
                Phase::DynamicClcLengths { read, hclen, hlit, hdist, mut clc_lengths } => {
                    if read == hclen {
                        let clc = Huffman::build(&clc_lengths)?;
                        self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths: Vec::with_capacity(hlit + hdist) };
                        continue;
                    }
                    if !self.reader.ensure(3, pending, input_complete)? {
                        self.phase = Phase::DynamicClcLengths { read, hclen, hlit, hdist, clc_lengths };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    clc_lengths[CLC_ORDER[read]] = self.reader.take(3) as u8;
                    self.phase = Phase::DynamicClcLengths { read: read + 1, hclen, hlit, hdist, clc_lengths };
                }
                Phase::DynamicCodeLengths { clc, hlit, hdist, mut lengths } => {
                    if lengths.len() >= hlit + hdist {
                        let lit_len = Huffman::build(&lengths[..hlit])?;
                        let dist = Huffman::build(&lengths[hlit..hlit + hdist])?;
                        self.phase = Phase::DecodeSymbol { lit_len, dist };
                        continue;
                    }
                    if !self.reader.ensure_huffman(pending, input_complete)? {
                        self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let symbol = clc.decode(&mut self.reader)?;
                    match symbol {
                        0..=15 => {
                            lengths.push(symbol as u8);
                            self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths };
                        }
                        16 => {
                            let prev = *lengths.last().ok_or(DeflateError::BadHuffmanCode)?;
                            self.phase = Phase::DynamicRepeatPrev { clc, hlit, hdist, lengths, prev };
                        }
                        17 => self.phase = Phase::DynamicRepeatZero { clc, hlit, hdist, lengths, bits: 3, base: 3 },
                        18 => self.phase = Phase::DynamicRepeatZero { clc, hlit, hdist, lengths, bits: 7, base: 11 },
                        _ => return Err(DeflateError::BadHuffmanCode),
                    }
                }
                Phase::DynamicRepeatPrev { clc, hlit, hdist, mut lengths, prev } => {
                    if !self.reader.ensure(2, pending, input_complete)? {
                        self.phase = Phase::DynamicRepeatPrev { clc, hlit, hdist, lengths, prev };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let repeat = self.reader.take(2) + 3;
                    for _ in 0..repeat {
                        lengths.push(prev);
                    }
                    self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths };
                }
                Phase::DynamicRepeatZero { clc, hlit, hdist, mut lengths, bits, base } => {
                    if !self.reader.ensure(bits, pending, input_complete)? {
                        self.phase = Phase::DynamicRepeatZero { clc, hlit, hdist, lengths, bits, base };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let repeat = self.reader.take(bits) + base;
                    for _ in 0..repeat {
                        lengths.push(0);
                    }
                    self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths };
                }
                Phase::DecodeSymbol { lit_len, dist } => {
                    if !self.reader.ensure_huffman(pending, input_complete)? {
                        self.phase = Phase::DecodeSymbol { lit_len, dist };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let symbol = lit_len.decode(&mut self.reader)?;
                    if symbol < 256 {
                        let byte = symbol as u8;
                        self.output.push(byte);
                        self.phase = Phase::DecodeSymbol { lit_len, dist };
                        return Ok(InflateOutcome::Wrote(byte));
                    } else if symbol == 256 {
                        self.phase = if self.final_block { Phase::Done } else { Phase::BlockHeader };
                    } else {
                        let index = (symbol - 257) as usize;
                        if index >= LENGTH_BASE.len() {
                            return Err(DeflateError::BadHuffmanCode);
                        }
                        let extra = LENGTH_EXTRA[index];
                        let base_len = LENGTH_BASE[index];
                        self.phase = if extra == 0 { Phase::DecodeDistanceSymbol { lit_len, dist, length: base_len } } else { Phase::LengthExtra { lit_len, dist, base_len, extra } };
                    }
                }
                Phase::LengthExtra { lit_len, dist, base_len, extra } => {
                    if !self.reader.ensure(extra as u32, pending, input_complete)? {
                        self.phase = Phase::LengthExtra { lit_len, dist, base_len, extra };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let length = base_len + self.reader.take(extra as u32) as u16;
                    self.phase = Phase::DecodeDistanceSymbol { lit_len, dist, length };
                }
                Phase::DecodeDistanceSymbol { lit_len, dist, length } => {
                    if !self.reader.ensure_huffman(pending, input_complete)? {
                        self.phase = Phase::DecodeDistanceSymbol { lit_len, dist, length };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let symbol = dist.decode(&mut self.reader)? as usize;
                    if symbol >= DIST_BASE.len() {
                        return Err(DeflateError::BadDistance);
                    }
                    let extra = DIST_EXTRA[symbol];
                    let base_dist = DIST_BASE[symbol];
                    self.phase = if extra == 0 { Phase::CopyMatch { lit_len, dist, distance: base_dist, remaining: length } } else { Phase::DistanceExtra { lit_len, dist, length, base_dist, extra } };
                }
                Phase::DistanceExtra { lit_len, dist, length, base_dist, extra } => {
                    if !self.reader.ensure(extra as u32, pending, input_complete)? {
                        self.phase = Phase::DistanceExtra { lit_len, dist, length, base_dist, extra };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let distance = base_dist + self.reader.take(extra as u32);
                    self.phase = Phase::CopyMatch { lit_len, dist, distance, remaining: length };
                }
                Phase::CopyMatch { lit_len, dist, distance, remaining } => {
                    if remaining == 0 {
                        self.phase = Phase::DecodeSymbol { lit_len, dist };
                        continue;
                    }
                    let back = distance as usize;
                    if back == 0 || back > self.output.len() {
                        return Err(DeflateError::BadDistance);
                    }
                    let byte = self.output[self.output.len() - back];
                    self.output.push(byte);
                    self.phase = Phase::CopyMatch { lit_len, dist, distance, remaining: remaining - 1 };
                    return Ok(InflateOutcome::Wrote(byte));
                }
            }
        }
    }
}

/// #️⃣ One-shot raw-DEFLATE decompression, bounded by `max_output_len` so a corrupt/hostile
/// `stored` slice cannot force an unbounded allocation.
pub fn inflate(stored: &[u8], max_output_len: usize) -> Result<Vec<u8>, DeflateError> {
    let mut inflater = Inflater::new();
    let mut output = Vec::new();
    let mut index = 0usize;
    let mut pending: Option<u8> = None;
    loop {
        if pending.is_none() && index < stored.len() {
            pending = Some(stored[index]);
            index += 1;
        }
        let input_complete = index >= stored.len();
        match inflater.advance(&mut pending, input_complete)? {
            InflateOutcome::NeedInput => {}
            InflateOutcome::Wrote(byte) => {
                if output.len() >= max_output_len {
                    return Err(DeflateError::OutputLimitExceeded);
                }
                output.push(byte);
            }
            InflateOutcome::Done => return Ok(output),
        }
    }
}
//#endregion 🔖️Inflate

//#region 🔖️Deflate
const WINDOW_SIZE: usize = 32768;
const MIN_MATCH: usize = 3;
const MAX_MATCH: usize = 258;
const HASH_BITS: u32 = 15;
const HASH_SIZE: usize = 1 << HASH_BITS;
const MAX_CHAIN: usize = 128;

fn hash3(a: u8, b: u8, c: u8) -> usize {
    let value = (a as u32) | (b as u32) << 8 | (c as u32) << 16;
    ((value.wrapping_mul(2654435761)) >> (32 - HASH_BITS)) as usize
}

/// 🗜️ One fixed-Huffman (`BTYPE=1`) block over the whole input, found via a greedy hash-chain
/// LZ77 match finder (min match 3, max 258, 32 KiB window, `MAX_CHAIN` probes) — a real matcher,
/// not a stored-block passthrough, so segment payloads still shrink.
pub fn deflate(raw: &[u8]) -> Vec<u8> {
    let lit_len_lengths = fixed_literal_length_lengths();
    let dist_lengths = fixed_distance_lengths();
    let lit_len_codes = canonical_codes(&lit_len_lengths);
    let dist_codes = canonical_codes(&dist_lengths);
    let mut writer = BitWriter::new();
    writer.write_bits(1, 1);
    writer.write_bits(1, 2);
    let mut head = vec![u32::MAX; HASH_SIZE];
    let mut prev = vec![u32::MAX; raw.len()];
    let insert = |position: usize, raw: &[u8], head: &mut [u32], prev: &mut [u32]| {
        if position + MIN_MATCH <= raw.len() {
            let h = hash3(raw[position], raw[position + 1], raw[position + 2]);
            prev[position] = head[h];
            head[h] = position as u32;
        }
    };
    let mut i = 0usize;
    while i < raw.len() {
        let mut best_len = 0usize;
        let mut best_dist = 0usize;
        if i + MIN_MATCH <= raw.len() {
            let h = hash3(raw[i], raw[i + 1], raw[i + 2]);
            let mut candidate = head[h];
            let mut chain = 0usize;
            while candidate != u32::MAX && chain < MAX_CHAIN {
                let cpos = candidate as usize;
                if i - cpos > WINDOW_SIZE {
                    break;
                }
                let max_possible = (raw.len() - i).min(MAX_MATCH);
                let mut len = 0usize;
                while len < max_possible && raw[cpos + len] == raw[i + len] {
                    len += 1;
                }
                if len > best_len {
                    best_len = len;
                    best_dist = i - cpos;
                }
                candidate = prev[cpos];
                chain += 1;
            }
        }
        if best_len >= MIN_MATCH {
            let length_index = length_index_for(best_len as u16);
            let length_symbol = 257 + length_index;
            writer.write_huffman_code(lit_len_codes[length_symbol], lit_len_lengths[length_symbol]);
            let extra_len = LENGTH_EXTRA[length_index];
            if extra_len > 0 {
                writer.write_bits((best_len as u16 - LENGTH_BASE[length_index]) as u32, extra_len as u32);
            }
            let dist_index = distance_index_for(best_dist as u32);
            writer.write_huffman_code(dist_codes[dist_index], dist_lengths[dist_index]);
            let extra_dist = DIST_EXTRA[dist_index];
            if extra_dist > 0 {
                writer.write_bits(best_dist as u32 - DIST_BASE[dist_index], extra_dist as u32);
            }
            let end = i + best_len;
            while i < end {
                insert(i, raw, &mut head, &mut prev);
                i += 1;
            }
        } else {
            writer.write_huffman_code(lit_len_codes[raw[i] as usize], lit_len_lengths[raw[i] as usize]);
            insert(i, raw, &mut head, &mut prev);
            i += 1;
        }
    }
    writer.write_huffman_code(lit_len_codes[256], lit_len_lengths[256]);
    writer.finish()
}
//#endregion 🔖️Deflate

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_bytes(seed: u64, len: usize) -> Vec<u8> {
        let mut state = seed | 1;
        (0..len)
            .map(|_| {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                (state >> 33) as u8
            })
            .collect()
    }

    #[test]
    fn round_trips_empty_and_small_inputs() {
        for sample in [b"".as_slice(), b"a", b"ab", b"hello, world!"] {
            let compressed = deflate(sample);
            let decompressed = inflate(&compressed, sample.len()).expect("inflate ours");
            assert_eq!(decompressed, sample);
        }
    }

    #[test]
    fn round_trips_repetitive_input_that_forces_long_matches() {
        let sample: Vec<u8> = std::iter::repeat(b'x').take(5000).collect();
        let compressed = deflate(&sample);
        assert!(compressed.len() < sample.len(), "a real LZ77 matcher must shrink a run of one byte");
        let decompressed = inflate(&compressed, sample.len()).expect("inflate ours");
        assert_eq!(decompressed, sample);
    }

    #[test]
    fn round_trips_pseudo_random_lcg_input_across_sizes() {
        for (seed, len) in [(1u64, 0usize), (2, 1), (3, 63), (4, 64), (5, 1023), (6, 4097), (7, 40000)] {
            let sample = lcg_bytes(seed, len);
            let compressed = deflate(&sample);
            let decompressed = inflate(&compressed, sample.len()).expect("inflate ours");
            assert_eq!(decompressed, sample, "mismatch at seed={seed} len={len}");
        }
    }

    /// 🌊️ Drives `Inflater::advance` one admitted byte at a time — the exact granularity
    /// `DeflateRetainedCursor` uses in production — and returns everything it wrote.
    fn streamed_inflate_result(compressed: &[u8]) -> Result<Vec<u8>, DeflateError> {
        let mut inflater = Inflater::new();
        let mut output = Vec::new();
        let mut index = 0usize;
        let mut pending: Option<u8> = None;
        loop {
            if pending.is_none() && index < compressed.len() {
                pending = Some(compressed[index]);
                index += 1;
            }
            let input_complete = index >= compressed.len();
            match inflater.advance(&mut pending, input_complete)? {
                InflateOutcome::NeedInput if input_complete => return Err(DeflateError::UnexpectedEnd),
                InflateOutcome::NeedInput => {}
                InflateOutcome::Wrote(byte) => output.push(byte),
                InflateOutcome::Done => return Ok(output),
            }
        }
    }

    fn streamed_inflate(compressed: &[u8]) -> Vec<u8> {
        streamed_inflate_result(compressed).expect("advance")
    }

    fn btype_of(compressed: &[u8]) -> u8 {
        (compressed[0] >> 1) & 0b11
    }

    #[test]
    fn stream_produces_the_same_bytes_as_one_shot_inflate() {
        let sample = lcg_bytes(9, 5000);
        let compressed = deflate(&sample);
        assert_eq!(streamed_inflate(&compressed), sample);
        assert_eq!(inflate(&compressed, sample.len()).expect("inflate ours"), sample);
    }

    #[test]
    fn accepts_a_short_final_huffman_tail() {
        let fixture = include_str!("🧪️tests/🔣️deflate-tail-cases.json");
        let file: TailCorpusFile = serde_json::from_str(fixture).expect("valid DEFLATE tail fixture");
        assert_eq!(file.cases.len(), 1, "expected one fixed-EOB tail vector");
        for case in file.cases {
            assert_eq!(miniz_oxide::inflate::decompress_to_vec_with_limit(&case.stored, 1).expect("miniz inflates fixed EOB"), case.raw, "miniz mismatch for {}", case.name);
            assert_eq!(inflate(&case.stored, 0).expect("inflate fixed EOB"), case.raw, "one-shot mismatch for {}", case.name);
            assert_eq!(streamed_inflate_result(&case.stored).expect("streamed fixed EOB"), case.raw, "streamed mismatch for {}", case.name);
        }
    }

    #[test]
    fn rejects_truncation_before_the_final_huffman_symbol() {
        let sample = "the quick brown fox jumps over the lazy dog. ".repeat(500).into_bytes();
        let compressed = deflate(&sample);
        let truncated = &compressed[..1];
        assert_eq!(inflate(truncated, sample.len()), Err(DeflateError::UnexpectedEnd));
        assert_eq!(streamed_inflate_result(truncated), Err(DeflateError::UnexpectedEnd));
    }

    #[test]
    fn reads_miniz_oxide_dynamic_huffman_blocks() {
        let sample = "the quick brown fox jumps over the lazy dog. ".repeat(2000).into_bytes();
        let theirs = miniz_oxide::deflate::compress_to_vec(&sample, 6);
        assert_eq!(btype_of(&theirs), 2, "expected miniz_oxide to pick BTYPE=10 dynamic Huffman here");
        assert_eq!(inflate(&theirs, sample.len()).expect("inflate dynamic huffman"), sample);
        assert_eq!(streamed_inflate(&theirs), sample);
    }

    #[test]
    fn reads_a_stored_block() {
        let raw: [u8; 5] = [0, 0, 0, 0xff, 0xff];
        let mut stored = vec![0b001u8];
        let len = raw.len() as u16;
        stored.extend_from_slice(&len.to_le_bytes());
        stored.extend_from_slice(&(!len).to_le_bytes());
        stored.extend_from_slice(&raw);
        assert_eq!(btype_of(&stored), 0, "expected BTYPE=00 stored block");
        assert_eq!(inflate(&stored, raw.len()).expect("inflate stored block"), raw);
        assert_eq!(streamed_inflate(&stored), raw);
    }

    #[test]
    fn round_trips_multi_block_input_spanning_the_window() {
        let sample: Vec<u8> = (0..3u64).flat_map(|block| lcg_bytes(100 + block, 40_000)).collect();
        assert!(sample.len() > 32768 * 3, "must exceed three window sizes to span block boundaries");
        let theirs = miniz_oxide::deflate::compress_to_vec(&sample, 6);
        assert_eq!(inflate(&theirs, sample.len()).expect("inflate large miniz output"), sample);
        assert_eq!(streamed_inflate(&theirs), sample);
        let ours = deflate(&sample);
        assert_eq!(inflate(&ours, sample.len()).expect("inflate our own large output"), sample);
        assert_eq!(streamed_inflate(&ours), sample);
    }

    //#region 🧪️Oracle
    /// 🧪️ `miniz_oxide` lives ONLY in `[dev-dependencies]` here — the differential oracle proving
    /// round-trip compatibility in both directions with what is already persisted.
    #[derive(serde::Deserialize)]
    struct CorpusFile {
        cases: Vec<CorpusCase>,
    }

    #[derive(serde::Deserialize)]
    struct CorpusCase {
        seed: u64,
        len: usize,
    }

    #[derive(serde::Deserialize)]
    struct TailCorpusFile {
        cases: Vec<TailCorpusCase>,
    }

    #[derive(serde::Deserialize)]
    struct TailCorpusCase {
        name: String,
        stored: Vec<u8>,
        raw: Vec<u8>,
    }

    #[test]
    fn ours_inflates_miniz_oxide_output_and_vice_versa() {
        let raw = include_str!("🧪️tests/🔣️deflate-corpus.json");
        let file: CorpusFile = serde_json::from_str(raw).expect("valid deflate corpus fixture");
        assert!(file.cases.len() >= 8, "expected a real length sweep");
        for case in &file.cases {
            let sample = lcg_bytes(case.seed, case.len);
            let theirs = miniz_oxide::deflate::compress_to_vec(&sample, 6);
            let ours_from_theirs = inflate(&theirs, sample.len()).unwrap_or_else(|error| panic!("ours failed to inflate miniz_oxide output at seed={} len={}: {error:?}", case.seed, case.len));
            assert_eq!(ours_from_theirs, sample, "ours-from-theirs mismatch at seed={} len={}", case.seed, case.len);

            let ours = deflate(&sample);
            let theirs_from_ours = miniz_oxide::inflate::decompress_to_vec_with_limit(&ours, sample.len().max(1)).unwrap_or_else(|error| panic!("miniz_oxide failed to inflate our output at seed={} len={}: {error:?}", case.seed, case.len));
            assert_eq!(theirs_from_ours, sample, "theirs-from-ours mismatch at seed={} len={}", case.seed, case.len);
        }
    }
    //#endregion 🧪️Oracle
}
