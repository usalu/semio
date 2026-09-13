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
    symbols: [u16; 288],
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
        if lengths.len() > 288 {
            return Err(DeflateError::BadHuffmanCode);
        }
        let mut symbols = [0u16; 288];
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
    DynamicCodeLengths { clc: Huffman, hlit: usize, hdist: usize, lengths: [u8; 318], len: usize },
    DynamicRepeatPrev { clc: Huffman, hlit: usize, hdist: usize, lengths: [u8; 318], len: usize, prev: u8 },
    DynamicRepeatZero { clc: Huffman, hlit: usize, hdist: usize, lengths: [u8; 318], len: usize, bits: u32, base: u32 },
    DecodeSymbol { lit_len: Huffman, dist: Huffman },
    LengthExtra { lit_len: Huffman, dist: Huffman, base_len: u16, extra: u8 },
    DecodeDistanceSymbol { lit_len: Huffman, dist: Huffman, length: u16 },
    DistanceExtra { lit_len: Huffman, dist: Huffman, length: u16, base_dist: u32, extra: u8 },
    CopyMatch { lit_len: Huffman, dist: Huffman, distance: u32, remaining: u16 },
    Done,
}

pub const RETAINED_INFLATE_WINDOW_BYTES: usize = 32 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedInflateAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedInflateAllocationError {
    pub allocated_bytes: usize,
    pub reason: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedInflateCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedInflateProgress {
    pub history_bytes: usize,
    pub allocated_bytes: usize,
    pub maximum_distance: usize,
    pub observed_distance_one: bool,
    pub dynamic_repeat_codes: u8,
    pub allocation_ready: bool,
    pub terminal: bool,
}

struct RetainedInflateHistory {
    bytes: std::mem::ManuallyDrop<Vec<u8>>,
    target: usize,
    maximum_allocation_bytes: usize,
    length: usize,
    write: usize,
    allocation_fault: Option<RetainedInflateAllocationError>,
    physical_closed: bool,
}

impl RetainedInflateHistory {
    fn new(target: usize, maximum_allocation_bytes: usize) -> Result<Self, DeflateError> {
        if target > maximum_allocation_bytes || maximum_allocation_bytes > isize::MAX as usize {
            return Err(DeflateError::OutputLimitExceeded);
        }
        Ok(Self {
            bytes: std::mem::ManuallyDrop::new(Vec::new()),
            target,
            maximum_allocation_bytes,
            length: 0,
            write: 0,
            allocation_fault: None,
            physical_closed: false,
        })
    }

    fn allocated_bytes(&self) -> usize {
        self.bytes.capacity()
    }

    fn ready(&self) -> bool {
        self.allocation_fault.is_none() && (self.target == 0 || self.allocated_bytes() >= self.target) && self.allocated_bytes() <= self.maximum_allocation_bytes && !self.physical_closed
    }

    fn next_allocation_bytes(&self) -> Option<usize> {
        (!self.ready() && self.allocation_fault.is_none() && self.target != 0).then_some(self.target)
    }

    fn reserve(&mut self, maximum_bytes: usize) -> Result<RetainedInflateAllocationStep, RetainedInflateAllocationError> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        let Some(exact) = self.next_allocation_bytes() else { return Ok(RetainedInflateAllocationStep::default()) };
        if maximum_bytes < exact {
            return Ok(RetainedInflateAllocationStep::default());
        }
        if self.bytes.try_reserve_exact(exact).is_err() {
            let fault = RetainedInflateAllocationError { allocated_bytes: self.allocated_bytes(), reason: "retained inflate history allocation failed" };
            self.allocation_fault = Some(fault);
            return Err(fault);
        }
        let allocated_bytes = self.allocated_bytes();
        if allocated_bytes < exact || allocated_bytes > maximum_bytes || allocated_bytes > self.maximum_allocation_bytes {
            let fault = RetainedInflateAllocationError { allocated_bytes, reason: "retained inflate history allocation exceeded physical ceiling" };
            self.allocation_fault = Some(fault);
            return Err(fault);
        }
        Ok(RetainedInflateAllocationStep { progressed: true, allocated_bytes })
    }

    fn push(&mut self, byte: u8) -> Result<(), DeflateError> {
        if !self.ready() || self.target == 0 {
            return Err(DeflateError::OutputLimitExceeded);
        }
        if self.length < self.target {
            self.bytes.push(byte);
            self.length += 1;
            self.write = self.length % self.target;
        } else {
            self.bytes[self.write] = byte;
            self.write = (self.write + 1) % self.target;
        }
        Ok(())
    }

    fn read_back(&self, distance: usize) -> Result<u8, DeflateError> {
        if distance == 0 || distance > self.length {
            return Err(DeflateError::BadDistance);
        }
        let index = if self.length < self.target { self.length - distance } else { (self.write + self.target - distance) % self.target };
        Ok(self.bytes[index])
    }

    fn reset(&mut self) {
        self.bytes.clear();
        self.length = 0;
        self.write = 0;
    }

    fn release(&mut self, maximum_bytes: usize) -> RetainedInflateCloseStep {
        let allocated_bytes = self.allocated_bytes();
        if allocated_bytes > maximum_bytes {
            return RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let bytes = std::mem::replace(&mut self.bytes, std::mem::ManuallyDrop::new(Vec::new()));
        drop(std::mem::ManuallyDrop::into_inner(bytes));
        self.physical_closed = true;
        if allocated_bytes == 0 {
            RetainedInflateCloseStep::Complete
        } else {
            RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: allocated_bytes }
        }
    }
}

impl Drop for RetainedInflateHistory {
    fn drop(&mut self) {
        assert!(self.physical_closed && self.allocated_bytes() == 0, "retained inflate history reached Drop before exact physical release");
    }
}

enum InflateHistory {
    Cold(Vec<u8>),
    Retained(RetainedInflateHistory),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RetainedInflateClosePhase {
    Open,
    DecoderLogical,
    HistoryPhysical,
    Closed,
}

/// 🌊️ Resumable raw-DEFLATE decoder that yields exactly one output byte (or `NeedInput`/`Done`)
/// per `advance` call, driven by admitting at most one pending input byte at a time — the exact
/// shape `📡️replication`'s mounted pack reader needs to hand back control between I/O grants.
/// Keeps every byte it has produced so far (`output`) so LZ77 back-references can index into it;
/// callers that only need a one-shot `Vec<u8>` should use `inflate` instead.
pub struct Inflater {
    reader: BitReader,
    history: InflateHistory,
    phase: Phase,
    final_block: bool,
    maximum_distance: usize,
    observed_distance_one: bool,
    dynamic_repeat_codes: u8,
    retained_close_phase: RetainedInflateClosePhase,
}

impl Default for Inflater {
    fn default() -> Self {
        Self::new()
    }
}

impl Inflater {
    pub fn new() -> Self {
        Self {
            reader: BitReader::new(),
            history: InflateHistory::Cold(Vec::new()),
            phase: Phase::BlockHeader,
            final_block: false,
            maximum_distance: 0,
            observed_distance_one: false,
            dynamic_repeat_codes: 0,
            retained_close_phase: RetainedInflateClosePhase::Open,
        }
    }

    pub fn try_new_retained(maximum_history_bytes: usize, maximum_allocation_bytes: usize) -> Result<Self, DeflateError> {
        let target = maximum_history_bytes.min(RETAINED_INFLATE_WINDOW_BYTES);
        Ok(Self {
            reader: BitReader::new(),
            history: InflateHistory::Retained(RetainedInflateHistory::new(target, maximum_allocation_bytes)?),
            phase: Phase::BlockHeader,
            final_block: false,
            maximum_distance: 0,
            observed_distance_one: false,
            dynamic_repeat_codes: 0,
            retained_close_phase: RetainedInflateClosePhase::Open,
        })
    }

    pub fn next_retained_allocation_bytes(&self) -> Option<usize> {
        match &self.history {
            InflateHistory::Cold(_) => None,
            InflateHistory::Retained(history) => history.next_allocation_bytes(),
        }
    }

    pub fn reserve_retained_history(&mut self, maximum_bytes: usize) -> Result<RetainedInflateAllocationStep, RetainedInflateAllocationError> {
        match &mut self.history {
            InflateHistory::Cold(_) => Ok(RetainedInflateAllocationStep::default()),
            InflateHistory::Retained(history) => history.reserve(maximum_bytes),
        }
    }

    pub fn retained_allocated_bytes(&self) -> usize {
        match &self.history {
            InflateHistory::Cold(_) => 0,
            InflateHistory::Retained(history) => history.allocated_bytes(),
        }
    }

    pub fn retained_history_ptr(&self) -> Option<usize> {
        match &self.history {
            InflateHistory::Cold(_) => None,
            InflateHistory::Retained(history) if history.allocated_bytes() != 0 => Some(history.bytes.as_ptr() as usize),
            InflateHistory::Retained(_) => None,
        }
    }

    pub fn retained_progress(&self) -> Option<RetainedInflateProgress> {
        match &self.history {
            InflateHistory::Cold(_) => None,
            InflateHistory::Retained(history) => Some(RetainedInflateProgress {
                history_bytes: history.length,
                allocated_bytes: history.allocated_bytes(),
                maximum_distance: self.maximum_distance,
                observed_distance_one: self.observed_distance_one,
                dynamic_repeat_codes: self.dynamic_repeat_codes,
                allocation_ready: history.ready(),
                terminal: self.retained_terminal_is_empty(),
            }),
        }
    }

    pub fn reset_retained(&mut self) -> Result<(), DeflateError> {
        let InflateHistory::Retained(history) = &mut self.history else { return Err(DeflateError::BadBlockType) };
        if self.retained_close_phase != RetainedInflateClosePhase::Open || !history.ready() {
            return Err(DeflateError::OutputLimitExceeded);
        }
        history.reset();
        self.reader = BitReader::new();
        self.phase = Phase::BlockHeader;
        self.final_block = false;
        Ok(())
    }

    pub fn next_retained_release_allocation_bytes(&self) -> Option<usize> {
        if self.retained_close_phase != RetainedInflateClosePhase::HistoryPhysical {
            return None;
        }
        match &self.history {
            InflateHistory::Retained(history) if history.allocated_bytes() != 0 => Some(history.allocated_bytes()),
            _ => None,
        }
    }

    pub fn close_retained_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> RetainedInflateCloseStep {
        if self.retained_close_phase == RetainedInflateClosePhase::Closed {
            return RetainedInflateCloseStep::Complete;
        }
        if maximum_items == 0 && maximum_bytes == 0 {
            return RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        match self.retained_close_phase {
            RetainedInflateClosePhase::Open => {
                if maximum_items == 0 {
                    return RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                if let InflateHistory::Retained(history) = &mut self.history {
                    history.reset();
                }
                self.retained_close_phase = RetainedInflateClosePhase::DecoderLogical;
                RetainedInflateCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            RetainedInflateClosePhase::DecoderLogical => {
                if maximum_items == 0 {
                    return RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                self.reader = BitReader::new();
                self.phase = Phase::Done;
                self.final_block = false;
                self.retained_close_phase = RetainedInflateClosePhase::HistoryPhysical;
                RetainedInflateCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            RetainedInflateClosePhase::HistoryPhysical => {
                let step = match &mut self.history {
                    InflateHistory::Cold(_) => RetainedInflateCloseStep::Complete,
                    InflateHistory::Retained(history) => history.release(maximum_bytes),
                };
                if matches!(step, RetainedInflateCloseStep::Complete) || matches!(step, RetainedInflateCloseStep::Pending { released_bytes, .. } if released_bytes != 0) {
                    self.retained_close_phase = RetainedInflateClosePhase::Closed;
                }
                step
            }
            RetainedInflateClosePhase::Closed => RetainedInflateCloseStep::Complete,
        }
    }

    pub fn retained_terminal_is_empty(&self) -> bool {
        self.retained_close_phase == RetainedInflateClosePhase::Closed
            && matches!(&self.history, InflateHistory::Retained(history) if history.length == 0 && history.allocated_bytes() == 0 && history.physical_closed)
    }

    fn history_ready(&self) -> bool {
        match &self.history {
            InflateHistory::Cold(_) => true,
            InflateHistory::Retained(history) => history.ready() && self.retained_close_phase == RetainedInflateClosePhase::Open,
        }
    }

    fn push_history(&mut self, byte: u8) -> Result<(), DeflateError> {
        match &mut self.history {
            InflateHistory::Cold(history) => {
                history.push(byte);
                Ok(())
            }
            InflateHistory::Retained(history) => history.push(byte),
        }
    }

    fn read_history(&self, distance: usize) -> Result<u8, DeflateError> {
        match &self.history {
            InflateHistory::Cold(history) => distance
                .checked_sub(1)
                .and_then(|_| history.len().checked_sub(distance))
                .and_then(|index| history.get(index).copied())
                .ok_or(DeflateError::BadDistance),
            InflateHistory::Retained(history) => history.read_back(distance),
        }
    }

    /// ▶️ Advances the state machine by at most one admitted input byte, producing at most one
    /// output byte. `pending` is taken (set to `None`) exactly when this call consumed it.
    pub fn advance(&mut self, pending: &mut Option<u8>, input_complete: bool) -> Result<InflateOutcome, DeflateError> {
        if !self.history_ready() {
            return Ok(InflateOutcome::NeedInput);
        }
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
                    self.push_history(byte)?;
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
                        self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths: [0; 318], len: 0 };
                        continue;
                    }
                    if !self.reader.ensure(3, pending, input_complete)? {
                        self.phase = Phase::DynamicClcLengths { read, hclen, hlit, hdist, clc_lengths };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    clc_lengths[CLC_ORDER[read]] = self.reader.take(3) as u8;
                    self.phase = Phase::DynamicClcLengths { read: read + 1, hclen, hlit, hdist, clc_lengths };
                }
                Phase::DynamicCodeLengths { clc, hlit, hdist, mut lengths, len } => {
                    if len == hlit + hdist {
                        let lit_len = Huffman::build(&lengths[..hlit])?;
                        let dist = Huffman::build(&lengths[hlit..hlit + hdist])?;
                        self.phase = Phase::DecodeSymbol { lit_len, dist };
                        continue;
                    }
                    if len > hlit + hdist || hlit + hdist > lengths.len() {
                        return Err(DeflateError::BadHuffmanCode);
                    }
                    if !self.reader.ensure_huffman(pending, input_complete)? {
                        self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths, len };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let symbol = clc.decode(&mut self.reader)?;
                    match symbol {
                        0..=15 => {
                            lengths[len] = symbol as u8;
                            self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths, len: len + 1 };
                        }
                        16 => {
                            if len == 0 {
                                return Err(DeflateError::BadHuffmanCode);
                            }
                            self.dynamic_repeat_codes |= 0b001;
                            let prev = lengths[len - 1];
                            self.phase = Phase::DynamicRepeatPrev { clc, hlit, hdist, lengths, len, prev };
                        }
                        17 => {
                            self.dynamic_repeat_codes |= 0b010;
                            self.phase = Phase::DynamicRepeatZero { clc, hlit, hdist, lengths, len, bits: 3, base: 3 };
                        }
                        18 => {
                            self.dynamic_repeat_codes |= 0b100;
                            self.phase = Phase::DynamicRepeatZero { clc, hlit, hdist, lengths, len, bits: 7, base: 11 };
                        }
                        _ => return Err(DeflateError::BadHuffmanCode),
                    }
                }
                Phase::DynamicRepeatPrev { clc, hlit, hdist, mut lengths, len, prev } => {
                    if !self.reader.ensure(2, pending, input_complete)? {
                        self.phase = Phase::DynamicRepeatPrev { clc, hlit, hdist, lengths, len, prev };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let repeat = self.reader.take(2) as usize + 3;
                    let next = len.checked_add(repeat).filter(|next| *next <= hlit + hdist && *next <= lengths.len()).ok_or(DeflateError::BadHuffmanCode)?;
                    for entry in &mut lengths[len..next] {
                        *entry = prev;
                    }
                    self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths, len: next };
                }
                Phase::DynamicRepeatZero { clc, hlit, hdist, mut lengths, len, bits, base } => {
                    if !self.reader.ensure(bits, pending, input_complete)? {
                        self.phase = Phase::DynamicRepeatZero { clc, hlit, hdist, lengths, len, bits, base };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let repeat = (self.reader.take(bits) + base) as usize;
                    let next = len.checked_add(repeat).filter(|next| *next <= hlit + hdist && *next <= lengths.len()).ok_or(DeflateError::BadHuffmanCode)?;
                    lengths[len..next].fill(0);
                    self.phase = Phase::DynamicCodeLengths { clc, hlit, hdist, lengths, len: next };
                }
                Phase::DecodeSymbol { lit_len, dist } => {
                    if !self.reader.ensure_huffman(pending, input_complete)? {
                        self.phase = Phase::DecodeSymbol { lit_len, dist };
                        return Ok(InflateOutcome::NeedInput);
                    }
                    let symbol = lit_len.decode(&mut self.reader)?;
                    if symbol < 256 {
                        let byte = symbol as u8;
                        self.push_history(byte)?;
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
                    let byte = self.read_history(back)?;
                    self.maximum_distance = self.maximum_distance.max(back);
                    self.observed_distance_one |= back == 1;
                    self.push_history(byte)?;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
