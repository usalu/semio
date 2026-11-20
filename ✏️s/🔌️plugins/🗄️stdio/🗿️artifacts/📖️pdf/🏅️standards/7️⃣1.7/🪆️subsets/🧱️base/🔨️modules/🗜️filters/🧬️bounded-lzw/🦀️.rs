//! 🧬 Rust-owned bounded PDF LZW dictionary and phrase expansion.

use crate::{FlateCloseStep, FlateGrant, FlateStatus, FlateStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

const FIRST: u16 = 258;
const LIMIT: u16 = 4096;
const ENTRIES: usize = (LIMIT - FIRST) as usize;
const PHRASE_BYTES: usize = LIMIT as usize;

#[derive(Clone, Copy)]
struct Entry { prefix: u16, suffix: u8 }

#[derive(Clone, Copy)]
struct Expansion { code: u16, next: u16, append_first: bool }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Life { Decode, Complete, Cancelled, Failed, Closing, Closed }

/// 🧵 Resumes PDF LZWDecode through admitted input, dictionary expansion and byte output.
#[must_use]
pub struct PdfLzwCursor {
    life: Life,
    table: Vec<Entry>,
    phrase: Vec<u8>,
    prefix: Option<u16>,
    expansion: Option<Expansion>,
    first: Option<u8>,
    last: Option<u8>,
    bits: u32,
    available: u8,
    width: u8,
    early: u16,
    input: usize,
    output: usize,
    limit: usize,
    source_len: Option<usize>,
}

impl PdfLzwCursor {
    /// 🌱 Creates a cursor without claiming dictionary or phrase buffers.
    pub fn new(maximum_output_bytes: usize, early_change: bool) -> Self {
        Self {
            life: Life::Decode, table: Vec::new(), phrase: Vec::new(), prefix: None, expansion: None, first: None, last: None,
            bits: 0, available: 0, width: 9, early: u16::from(early_change), input: 0, output: 0,
            limit: maximum_output_bytes, source_len: None,
        }
    }

    /// 📊 Counts compressed source bytes read and decoded output bytes staged.
    pub fn progress(&self) -> (usize, usize) { (self.input, self.output) }

    /// 🧳 Counts physical dictionary and phrase buffer backing.
    pub fn owned_bytes(&self) -> usize { self.table.capacity() * size_of::<Entry>() + self.phrase.capacity() }

    /// ▶️ Charges one unit per input byte, extracted code, expanded prefix edge, or output byte.
    pub fn advance(&mut self, source: &[u8], output: &mut [u8], grant: FlateGrant, cancelled: bool) -> FlateStep {
        if cancelled && self.life == Life::Decode { self.life = Life::Cancelled; }
        let mut step = FlateStep { status: Ok(FlateStatus::Yield), work: 0, allocated_bytes: 0, written_bytes: 0 };
        step.status = match self.life {
            Life::Decode => self.decode(source, output, grant, &mut step),
            Life::Complete => Ok(FlateStatus::Complete),
            Life::Cancelled => Ok(FlateStatus::Cancelled),
            _ => Err(ValueError::invariant("PDF LZW cursor is unavailable")),
        };
        if self.life == Life::Decode && step.status.is_err() { self.life = Life::Failed; }
        step
    }

    fn decode(&mut self, source: &[u8], output: &mut [u8], grant: FlateGrant, step: &mut FlateStep) -> Result<FlateStatus, ValueError> {
        if *self.source_len.get_or_insert(source.len()) != source.len() {
            return Err(ValueError::invariant("PDF LZW source length changed"));
        }
        while step.work < grant.work {
            if let Some(expansion) = self.expansion {
                let allocation = grant.allocation_bytes - step.allocated_bytes;
                if self.prefix.is_some() && self.table.capacity() == 0 {
                    if let Some(status) = reserve(&mut self.table, ENTRIES, allocation, step)? { return Ok(status); }
                    continue;
                }
                if expansion.next >= FIRST && self.phrase.capacity() == 0 {
                    if let Some(status) = reserve(&mut self.phrase, PHRASE_BYTES, allocation, step)? { return Ok(status); }
                    continue;
                }
                step.work += 1;
                self.expand(expansion)?;
            } else if self.first.is_some() || !self.phrase.is_empty() || self.last.is_some() {
                if self.output == self.limit {
                    return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF LZW output exceeds the admitted extent"));
                }
                if step.written_bytes == output.len() { return Ok(FlateStatus::NeedsOutput); }
                output[step.written_bytes] = self.first.take().or_else(|| self.phrase.pop()).or_else(|| self.last.take()).expect("staged LZW byte");
                step.written_bytes += 1;
                self.output += 1;
                step.work += 1;
            } else if self.available < self.width {
                step.work += 1;
                let byte = *source.get(self.input).ok_or_else(|| format_error("truncated PDF LZW stream before EOD"))?;
                self.input += 1;
                self.bits = (self.bits << 8) | u32::from(byte);
                self.available += 8;
            } else {
                step.work += 1;
                self.available -= self.width;
                let code = (self.bits >> self.available) as u16;
                self.bits &= (1 << self.available) - 1;
                match code {
                    256 => { self.table.clear(); self.prefix = None; self.width = 9; }
                    257 => { self.life = Life::Complete; return Ok(FlateStatus::Complete); }
                    _ => {
                        let next_code = FIRST + self.table.len() as u16;
                        if next_code == LIMIT { return Err(format_error("PDF LZW dictionary requires a clear code")); }
                        if code < next_code {
                            self.expansion = Some(Expansion { code, next: code, append_first: false });
                        } else if code == next_code {
                            let next = self.prefix.ok_or_else(|| format_error("PDF LZW code has no previous phrase"))?;
                            self.expansion = Some(Expansion { code, next, append_first: true });
                        } else {
                            return Err(format_error("PDF LZW code exceeds its dictionary"));
                        }
                    }
                }
            }
        }
        Ok(FlateStatus::Yield)
    }

    fn expand(&mut self, expansion: Expansion) -> Result<(), ValueError> {
        if expansion.next >= FIRST {
            let entry = self.table.get((expansion.next - FIRST) as usize).ok_or_else(|| format_error("invalid PDF LZW dictionary link"))?;
            if self.phrase.len() == self.phrase.capacity() || entry.prefix >= expansion.next {
                return Err(format_error("invalid PDF LZW phrase"));
            }
            self.phrase.push(entry.suffix);
            self.expansion = Some(Expansion { next: entry.prefix, ..expansion });
            return Ok(());
        }
        let byte = expansion.next as u8;
        if let Some(prefix) = self.prefix {
            self.table.push(Entry { prefix, suffix: byte });
            let next = FIRST + self.table.len() as u16;
            if self.width < 12 && next + self.early == (1 << self.width) { self.width += 1; }
        }
        self.prefix = Some(expansion.code);
        self.first = Some(byte);
        self.last = expansion.append_first.then_some(byte);
        self.expansion = None;
        Ok(())
    }

    /// ♻️ Stops decode and retains buffers for measured retirement.
    pub fn begin_close(&mut self) {
        if self.life != Life::Closed { self.life = Life::Closing; }
    }

    /// 📐 Quotes the next physical backing release.
    pub fn retirement_demand(&self) -> RetirementDemand {
        let release_bytes = if self.phrase.capacity() != 0 { self.phrase.capacity() } else { self.table.capacity() * size_of::<Entry>() };
        RetirementDemand { release_bytes, depth: usize::from(!self.terminal_is_empty()), ..RetirementDemand::default() }
    }

    /// 🧹 Releases at most one backing buffer under the current grant.
    pub fn close_step(&mut self, maximum_items: usize, maximum_release_bytes: usize) -> FlateCloseStep {
        let mut step = FlateCloseStep { complete: self.terminal_is_empty(), ..FlateCloseStep::default() };
        if self.life != Life::Closing || maximum_items == 0 { return step; }
        let bytes = self.retirement_demand().release_bytes;
        if bytes > maximum_release_bytes { return step; }
        if self.phrase.capacity() != 0 { self.phrase = Vec::new(); } else { self.table = Vec::new(); }
        step.work = 1;
        step.released_items = usize::from(bytes != 0);
        step.released_bytes = bytes;
        step.complete = self.owned_bytes() == 0;
        if step.complete { self.life = Life::Closed; }
        step
    }

    /// 🏁 Confirms terminal retirement.
    pub fn terminal_is_empty(&self) -> bool { self.life == Life::Closed }
}

fn reserve<T>(storage: &mut Vec<T>, count: usize, allocation: usize, step: &mut FlateStep) -> Result<Option<FlateStatus>, ValueError> {
    let bytes = count * size_of::<T>();
    if bytes > allocation { return Ok(Some(FlateStatus::NeedsHistory { bytes })); }
    step.work += 1;
    storage.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "PDF LZW allocation failed"))?;
    step.allocated_bytes += storage.capacity() * size_of::<T>();
    if storage.capacity() * size_of::<T>() > allocation {
        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "PDF LZW allocation exceeded the granted bytes"));
    }
    Ok(None)
}

fn format_error(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, reason) }
