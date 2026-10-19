//! 🧵️ Byte-granular canonical compression with independently admitted matching-window pages.
use super::*;
use semio_framework_value::list::{PagedList, PagedListError, PagedListRefusalKind};
pub use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand};

const HISTORY: usize = WINDOW_SIZE + MAX_MATCH + 2;
#[derive(Clone,Copy)]
struct HeadSlot{position:u32,generation:u32}

/// 🚦️ The exact retained work frontier, including compressed output custody.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedDeflatePhase { Initialize, Input, Search, Compare, Token, Insert, Output, Finish, Complete, Closing, Closed }
impl RetainedDeflatePhase {
    pub fn as_str(self) -> &'static str { match self { Self::Initialize => "initialize", Self::Input => "input", Self::Search => "search", Self::Compare => "compare", Self::Token => "token", Self::Insert => "insert", Self::Output => "output", Self::Finish => "finish", Self::Complete => "complete", Self::Closing => "closing", Self::Closed => "closed" } }
}

/// 🎟️ One paid source/structure event and the initialized output prefix it lends.
#[derive(Default, Debug)]
pub struct RetainedDeflateProgress { pub progress: RetainedCloneProgress, pub written_bytes: usize, pub need_input: bool, pub complete: bool }

/// 🗜️ Greedy fixed-Huffman state owns only bounded window pages, never input-sized vectors.
pub struct RetainedDeflateEncoder {
    head: PagedList<HeadSlot, HASH_SIZE>, previous: PagedList<u32, WINDOW_SIZE>, bytes: PagedList<u8, HISTORY>,generation:u32,
    phase: RetainedDeflatePhase, initialize: u8, initialized: usize,
    received: usize, position: usize, finished_input: bool,
    candidate: u32, chain: usize, matched: usize, best: usize, distance: usize, insert_end: usize,
    buffer: u64, bits: u32, written: usize,
}

impl Default for RetainedDeflateEncoder {
    fn default() -> Self { Self { head: PagedList::empty(), previous: PagedList::empty(), bytes: PagedList::empty(),generation:1, phase: RetainedDeflatePhase::Initialize, initialize: 0, initialized: 0, received: 0, position: 0, finished_input: false, candidate: u32::MAX, chain: 0, matched: 0, best: 0, distance: 0, insert_end: 0, buffer: 3, bits: 3, written: 0 } }
}

fn page_error(error: PagedListError) -> ValueError { ValueError::literal(match error.kind { PagedListRefusalKind::AllocationFailed => ValueRefusalKind::AllocationFailed, PagedListRefusalKind::OwnershipLimit => ValueRefusalKind::OwnershipLimit, PagedListRefusalKind::InvariantViolated => ValueRefusalKind::InvariantViolated }, error.reason) }
fn invalid(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, reason) }

fn initialize_demands<T, const N: usize>(owner: &PagedList<T, N>, index: usize) -> Result<(usize, usize, usize), ValueError> {
    if index == owner.len() && !owner.has_reserved_slot() { Ok((owner.next_allocation_bytes().map_err(page_error)?, owner.next_reserve_copy_byte_demand().map_err(page_error)?, owner.next_reserve_depth_demand().map_err(page_error)? + 1)) }
    else { Ok((0, size_of::<T>(), owner.next_pop_depth_demand().map_err(page_error)?.max(1) + 1)) }
}

fn initialize_one<T: Copy, const N: usize>(owner: &mut PagedList<T, N>, index: usize, value: T, grant: RetainedCloneGrant) -> Result<(RetainedCloneProgress, bool), ValueError> {
    if index == owner.len() && !owner.has_reserved_slot() {
        return owner.reserve_one_funded(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }).map(|receipt| (receipt, false)).map_err(|error| page_error(error.refusal()).with_retained_progress(RetainedCloneProgress { retained_capacity_bytes: error.allocated_bytes, ..Default::default() }));
    }
    if index == owner.len() { owner.push_reserved(value).map_err(|_| invalid("Deflate initialized slot lacks admitted backing"))?; }
    else { owner[index] = value; }
    Ok((RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), ..Default::default() }, true))
}

fn retirement<T, const N: usize>(owner: &PagedList<T, N>) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { release_bytes: if owner.is_empty() { owner.next_release_allocation_bytes().map_err(page_error)? } else { 0 }, depth: (if owner.is_empty() { owner.next_release_depth_demand() } else { owner.next_pop_depth_demand() }).map_err(page_error)? + 1, ..Default::default() })
}

fn retire<T, const N: usize>(owner: &mut PagedList<T, N>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
    if owner.pop().is_some() { Ok(RetainedCloneProgress { copied_items: 1, ..Default::default() }) }
    else { let step = owner.release_empty_page(grant.maximum_release_bytes).map_err(page_error)?; Ok(RetainedCloneProgress { copied_items: usize::from(step.progressed), released_bytes: step.released_allocation_bytes, ..Default::default() }) }
}

impl RetainedDeflateEncoder {
    pub fn phase(&self) -> RetainedDeflatePhase { if !matches!(self.phase, RetainedDeflatePhase::Closing | RetainedDeflatePhase::Closed) && (self.bits >= 8 || self.phase == RetainedDeflatePhase::Complete && self.bits != 0) { RetainedDeflatePhase::Output } else { self.phase } }
    pub fn input_len(&self) -> usize { self.received }
    pub fn output_len(&self) -> usize { self.written }
    pub fn is_complete(&self) -> bool { self.phase == RetainedDeflatePhase::Complete && self.bits == 0 }
    pub fn terminal_is_empty(&self) -> bool { self.phase == RetainedDeflatePhase::Closed && self.head.terminal_is_empty() && self.previous.terminal_is_empty() && self.bytes.terminal_is_empty() }
    pub fn physical_backing_bytes(&self) -> usize { self.head.allocated_bytes() + self.previous.allocated_bytes() + self.bytes.allocated_bytes() }
    fn demands(&self) -> Result<(usize, usize, usize), ValueError> {
        if self.phase() == RetainedDeflatePhase::Output { return Ok((0, 1, 1)); }
        if self.phase == RetainedDeflatePhase::Initialize { return match self.initialize { 0 => initialize_demands(&self.head, self.initialized), 1 => initialize_demands(&self.previous, self.initialized), 2 => initialize_demands(&self.bytes, self.initialized), _ => Err(invalid("Deflate initialization frontier exceeded owners")) }; }
        let depth = self.head.next_pop_depth_demand().map_err(page_error)?.max(self.previous.next_pop_depth_demand().map_err(page_error)?).max(self.bytes.next_pop_depth_demand().map_err(page_error)?) + 1;
        Ok((0, match self.phase { RetainedDeflatePhase::Input => 1, RetainedDeflatePhase::Search => 11, RetainedDeflatePhase::Compare => 6, RetainedDeflatePhase::Token => 1, RetainedDeflatePhase::Insert => 23, _ => 0 }, depth))
    }
    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { self.demands().map(|demand| demand.0) }
    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demands().map(|demand| demand.1) }
    pub fn next_depth_demand(&self) -> Result<usize, ValueError> { self.demands().map(|demand| demand.2) }
    fn at(&self, position: usize) -> u8 { self.bytes[position % HISTORY] }
    fn bits(&mut self, value: u32, count: u32) { self.buffer |= u64::from(value) << self.bits; self.bits += count; }
    fn literal(&mut self, symbol: usize) {
        let (code, length) = match symbol { 0..=143 => (48 + symbol as u32, 8), 144..=255 => (400 + symbol as u32 - 144, 9), 256..=279 => (symbol as u32 - 256, 7), _ => (192 + symbol as u32 - 280, 8) };
        self.bits(code.reverse_bits() >> (32 - length), length);
    }
    /// 🔁️ Reuses paid matching pages for a fresh independent segment without a whole-buffer reset.
    pub fn restart(&mut self) -> Result<(), ValueError> {
        if !self.is_complete() { return Err(invalid("Deflate restart requires complete output custody")); }
        self.generation=self.generation.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Deflate matching generation exceeds retained width"))?;self.phase = RetainedDeflatePhase::Input; self.received = 0; self.position = 0; self.finished_input = false; self.candidate = u32::MAX; self.chain = 0; self.matched = 0; self.best = 0; self.distance = 0; self.insert_end = 0; self.buffer = 3; self.bits = 3; self.written = 0; Ok(())
    }
    /// ✍️ Advances one funded frontier and leaves unaccepted input with its original owner.
    pub fn advance(&mut self, pending: &mut Option<u8>, input_complete: bool, output: &mut [u8], grant: RetainedCloneGrant) -> Result<RetainedDeflateProgress, ValueError> {
        if matches!(self.phase, RetainedDeflatePhase::Closing | RetainedDeflatePhase::Closed) { return Err(invalid("Deflate producer is closing")); }
        if self.is_complete() { return Ok(RetainedDeflateProgress { complete: true, ..Default::default() }); }
        let (capacity, copy, depth) = self.demands()?;
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < capacity || grant.maximum_copy_bytes < copy || grant.maximum_depth < depth { return Ok(Default::default()); }
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.phase() == RetainedDeflatePhase::Output {
            if output.is_empty() { return Ok(Default::default()); }
            output[0] = self.buffer as u8; self.buffer >>= 8; self.bits = self.bits.saturating_sub(8); self.written += 1; progress.copied_bytes = 1;
            return Ok(RetainedDeflateProgress { progress, written_bytes: 1, complete: self.is_complete(), ..Default::default() });
        }
        match self.phase {
            RetainedDeflatePhase::Initialize => {
                let (receipt, placed) = match self.initialize { 0 => initialize_one(&mut self.head, self.initialized,HeadSlot{position:u32::MAX,generation:0}, grant)?, 1 => initialize_one(&mut self.previous, self.initialized, u32::MAX, grant)?, 2 => initialize_one(&mut self.bytes, self.initialized, 0, grant)?, _ => return Err(invalid("Deflate initialization owner disappeared")) };
                progress = receipt;
                if placed { self.initialized += 1; let count = match self.initialize { 0 => HASH_SIZE, 1 => WINDOW_SIZE, _ => HISTORY }; if self.initialized == count { self.initialized = 0; self.initialize += 1; if self.initialize == 3 { self.phase = RetainedDeflatePhase::Input; } } }
            }
            RetainedDeflatePhase::Input => {
                if self.received - self.position >= MAX_MATCH + 2 || self.finished_input { self.best = 0; self.distance = 0; self.chain = 0; self.candidate = u32::MAX; self.phase = if self.position == self.received { RetainedDeflatePhase::Finish } else { RetainedDeflatePhase::Search }; }
                else if let Some(byte) = *pending { if self.received >= u32::MAX as usize { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "Deflate input exceeds position width")); } self.bytes[self.received % HISTORY] = byte; self.received += 1; *pending = None; progress.copied_bytes = 1; }
                else if input_complete { self.finished_input = true; }
                else { return Ok(RetainedDeflateProgress { need_input: true, ..Default::default() }); }
            }
            RetainedDeflatePhase::Search => {
                if self.received - self.position < MIN_MATCH { self.phase = RetainedDeflatePhase::Token; }
                else if self.candidate == u32::MAX && self.chain == 0 { let hash = hash3(self.at(self.position), self.at(self.position + 1), self.at(self.position + 2));let head=self.head[hash]; self.candidate =if head.generation==self.generation{head.position}else{u32::MAX}; progress.copied_bytes = 11; self.chain = 1; if self.candidate == u32::MAX { self.phase = RetainedDeflatePhase::Token; } }
                else if self.candidate == u32::MAX || self.chain > MAX_CHAIN || self.position - self.candidate as usize > WINDOW_SIZE { self.phase = RetainedDeflatePhase::Token; }
                else { self.matched = 0; self.phase = RetainedDeflatePhase::Compare; }
            }
            RetainedDeflatePhase::Compare => {
                let maximum = (self.received - self.position).min(MAX_MATCH);
                let equal = self.matched < maximum && self.at(self.candidate as usize + self.matched) == self.at(self.position + self.matched);
                progress.copied_bytes = if self.matched < maximum { 2 } else { 0 };
                if equal { self.matched += 1; }
                if !equal || self.matched == maximum { if self.matched > self.best { self.best = self.matched; self.distance = self.position - self.candidate as usize; } if self.matched==maximum{self.phase=RetainedDeflatePhase::Token;}else{self.candidate = self.previous[self.candidate as usize % WINDOW_SIZE]; self.chain += 1; self.phase = RetainedDeflatePhase::Search; progress.copied_bytes += 4;} }
            }
            RetainedDeflatePhase::Token => {
                let consumed = if self.best >= MIN_MATCH { let length = length_index_for(self.best as u16); self.literal(257 + length); self.bits((self.best as u16 - LENGTH_BASE[length]) as u32, u32::from(LENGTH_EXTRA[length])); let distance = distance_index_for(self.distance as u32); self.bits((distance as u32).reverse_bits() >> 27, 5); self.bits(self.distance as u32 - DIST_BASE[distance], u32::from(DIST_EXTRA[distance])); self.best }
                else { self.literal(usize::from(self.at(self.position))); progress.copied_bytes = 1; 1 };
                self.insert_end = self.position + consumed; self.phase = RetainedDeflatePhase::Insert;
            }
            RetainedDeflatePhase::Insert => {
                if self.received - self.position >= MIN_MATCH { let hash = hash3(self.at(self.position), self.at(self.position + 1), self.at(self.position + 2));let head=self.head[hash]; self.previous[self.position % WINDOW_SIZE] =if head.generation==self.generation{head.position}else{u32::MAX}; self.head[hash] =HeadSlot{position:self.position as u32,generation:self.generation}; progress.copied_bytes = 23; }
                self.position += 1; if self.position == self.insert_end { self.phase = RetainedDeflatePhase::Input; }
            }
            RetainedDeflatePhase::Finish => { self.literal(256); self.phase = RetainedDeflatePhase::Complete; }
            _ => return Err(invalid("Deflate retained work frontier is invalid")),
        }
        Ok(RetainedDeflateProgress { progress, complete: self.is_complete(), ..Default::default() })
    }
    pub fn begin_close(&mut self) { if self.phase != RetainedDeflatePhase::Closed { self.phase = RetainedDeflatePhase::Closing; } }
    /// 📏️ Quotes one exact payload pop or whole original backing retirement.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.head.terminal_is_empty() { retirement(&self.head) } else if !self.previous.terminal_is_empty() { retirement(&self.previous) } else if !self.bytes.terminal_is_empty() { retirement(&self.bytes) } else { Ok(RetirementDemand { depth: 1, ..Default::default() }) }
    }
    /// 🍂️ Releases one scalar or original physical allocation with no copied compression buffers.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands()?;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.begin_close();
        let receipt = if !self.head.terminal_is_empty() { retire(&mut self.head, grant)? } else if !self.previous.terminal_is_empty() { retire(&mut self.previous, grant)? } else if !self.bytes.terminal_is_empty() { retire(&mut self.bytes, grant)? } else { self.phase = RetainedDeflatePhase::Closed; RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) })
    }
}

impl Drop for RetainedDeflateEncoder { fn drop(&mut self) { assert!(self.head.terminal_is_empty() && self.previous.terminal_is_empty() && self.bytes.terminal_is_empty(), "retained Deflate matching pages require a terminal-empty close witness"); } }
