//! 📚️ Enumerates the live objects in authored file order under bounded heap work.

use super::{XrefEntry, XrefIndex, XrefIndexScan, XrefRow};
use semio_s_artifact_stdio_pdf_lexer::CosGrant;
use std::mem::size_of;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefFileOrderEvent {
    More,
    Object(u32),
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete,
    Cancelled,
    Closed,
}

pub struct XrefFileOrderReceipt {
    pub event: Result<XrefFileOrderEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

#[derive(Default)]
enum Phase { #[default] Reserve, Scan, Heapify, Drain, Complete, Cancelled, Closing, Closed }

/// 🧮️ Builds a min-heap using one comparison/swap level per work unit.
#[derive(Default)]
pub struct XrefFileOrderCursor {
    phase: Phase,
    rows: XrefIndexScan,
    numbers: Vec<u32>,
    next_parent: usize,
    sift: Option<usize>,
}

impl XrefFileOrderCursor {
    pub fn advance(&mut self, source: &XrefIndex, grant: CosGrant, cancelled: bool) -> XrefFileOrderReceipt {
        use XrefFileOrderEvent as Event;
        if cancelled && !matches!(self.phase, Phase::Closing | Phase::Closed) { self.phase = Phase::Cancelled; }
        let mut receipt = step(Ok(Event::More));
        loop {
            receipt.event = match self.phase {
                Phase::Cancelled => Ok(Event::Cancelled),
                Phase::Closing => Ok(Event::More),
                Phase::Closed => Ok(Event::Closed),
                Phase::Complete => Ok(Event::Complete),
                _ if receipt.work == grant.work => Ok(Event::More),
                Phase::Reserve => {
                    receipt.work += 1;
                    let Some(bytes) = source.len().checked_mul(size_of::<u32>()) else {
                        receipt.event = Err("PDF object order capacity overflow");
                        return receipt;
                    };
                    if bytes > grant.allocation_bytes {
                        Ok(Event::NeedsAllocation { bytes })
                    } else if self.numbers.try_reserve_exact(source.len()).is_err() {
                        Err("unable to allocate the PDF object order")
                    } else {
                        self.phase = Phase::Scan;
                        receipt.allocated_bytes = self.numbers.capacity() * size_of::<u32>();
                        Ok(Event::More)
                    }
                }
                Phase::Scan => {
                    let budget = grant.work.saturating_sub(receipt.work);
                    let (work, done, row) = self.rows.advance(source, budget);
                    receipt.work += work;
                    if let Some(XrefRow { number, entry: Some(_) }) = row {
                        if self.numbers.len() == self.numbers.capacity() {
                            receipt.event = Err("PDF cross-reference source changed during ordering");
                            return receipt;
                        }
                        self.numbers.push(number);
                    }
                    if done {
                        self.next_parent = self.numbers.len() / 2;
                        self.phase = Phase::Heapify;
                    } else if work == 0 {
                        receipt.event = Ok(Event::More);
                        return receipt;
                    }
                    continue;
                }
                Phase::Heapify | Phase::Drain if self.sift.is_some() => {
                    self.sift = self.sift_level(source, self.sift.expect("active xref sift"));
                    receipt.work += 1;
                    continue;
                }
                Phase::Heapify => {
                    receipt.work += 1;
                    if let Some(parent) = self.next_parent.checked_sub(1) {
                        self.next_parent = parent;
                        self.sift = Some(parent);
                    } else {
                        self.phase = Phase::Drain;
                    }
                    continue;
                }
                Phase::Drain => {
                    receipt.work += 1;
                    if self.numbers.is_empty() {
                        self.phase = Phase::Complete;
                        Ok(Event::Complete)
                    } else {
                        let first = self.numbers.swap_remove(0);
                        if self.numbers.len() > 1 { self.sift = Some(0); }
                        Ok(Event::Object(first))
                    }
                }
            };
            return receipt;
        }
    }

    fn sift_level(&mut self, source: &XrefIndex, root: usize) -> Option<usize> {
        let left = root * 2 + 1;
        let left_id = *self.numbers.get(left)?;
        let mut child = left;
        let mut child_key = position(source, left_id);
        if let Some(right_id) = self.numbers.get(left + 1) {
            let right_key = position(source, *right_id);
            if right_key < child_key { child = left + 1; child_key = right_key; }
        }
        if child_key >= position(source, self.numbers[root]) { return None; }
        self.numbers.swap(root, child);
        Some(child)
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::Closed) { self.phase = Phase::Closing; }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> XrefFileOrderReceipt {
        use XrefFileOrderEvent as Event;
        if matches!(self.phase, Phase::Closed) { return step(Ok(Event::Closed)); }
        if !matches!(self.phase, Phase::Closing) || grant.work == 0 { return step(Ok(Event::More)); }
        let bytes = self.numbers.capacity() * size_of::<u32>();
        if bytes > grant.release_bytes { return step(Ok(Event::NeedsRelease { bytes })); }
        self.numbers = Vec::new();
        self.phase = Phase::Closed;
        XrefFileOrderReceipt { work: 1, released_bytes: bytes, ..step(Ok(Event::Closed)) }
    }
}

fn position(source: &XrefIndex, number: u32) -> (usize, u32, u32) {
    match source.get(&number) {
        Some(XrefEntry::Normal { offset, .. }) => (*offset, 0, number),
        Some(XrefEntry::Compressed { stream_num, index }) => {
            let offset = match source.get(stream_num) {
                Some(XrefEntry::Normal { offset, .. }) => *offset,
                _ => usize::MAX,
            };
            (offset, index.saturating_add(1), number)
        }
        None => (usize::MAX, u32::MAX, number),
    }
}

fn step(event: Result<XrefFileOrderEvent, &'static str>) -> XrefFileOrderReceipt {
    XrefFileOrderReceipt { event, work: 0, allocated_bytes: 0, released_bytes: 0 }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
