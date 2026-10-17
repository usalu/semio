//! 🚰️ Inflater of the deflate codec: the resumable byte-granular inflate state machine and the one-shot inflate function. See [`super`].

use super::*;

//#region 🔖️Inflate
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

    /// 🧮️ Whole prefetched bytes remaining after the final end-of-block code.
    pub fn terminal_unused_bytes(&self) -> Option<usize> {
        matches!(self.phase, Phase::Done).then_some(self.reader.bits as usize / 8)
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

    /// 📏️ Quotes the retained logical frame separately from its original history release.
    pub fn retained_retirement_demands(&self) -> semio_framework_value::RetirementDemand {
        use semio_framework_value::RetirementDemand;
        match self.retained_close_phase {
            RetainedInflateClosePhase::Open | RetainedInflateClosePhase::DecoderLogical => RetirementDemand { depth: 1, ..Default::default() },
            RetainedInflateClosePhase::HistoryPhysical => RetirementDemand { release_bytes: self.next_retained_release_allocation_bytes().unwrap_or(0), depth: 1, ..Default::default() },
            RetainedInflateClosePhase::Closed => Default::default(),
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
                    if hlit > 286 {
                        return Err(DeflateError::BadHuffmanCode);
                    }
                    self.phase = Phase::DynamicClcLengths { read: 0, hclen, hlit, hdist, clc_lengths: [0u8; 19] };
                }
                Phase::DynamicClcLengths { read, hclen, hlit, hdist, mut clc_lengths } => {
                    if read == hclen {
                        let clc = Huffman::build(&clc_lengths)?;
                        if !clc.complete {
                            return Err(DeflateError::BadHuffmanCode);
                        }
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
                        if lengths[256] == 0 {
                            return Err(DeflateError::BadHuffmanCode);
                        }
                        let lit_len = Huffman::build(&lengths[..hlit])?;
                        let dist = Huffman::build(&lengths[hlit..hlit + hdist])?;
                        if !lit_len.valid_alphabet(false) || !dist.valid_alphabet(true) {
                            return Err(DeflateError::BadHuffmanCode);
                        }
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
