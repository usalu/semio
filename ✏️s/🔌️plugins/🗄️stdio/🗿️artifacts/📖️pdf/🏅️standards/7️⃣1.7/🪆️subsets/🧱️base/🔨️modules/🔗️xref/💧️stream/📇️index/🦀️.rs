//! 📇️ A resumable sparse cross-reference index, preserving the newest live or free row.

use crate::XrefEntry;
use semio_s_artifact_stdio_pdf_lexer::CosGrant;
use std::mem::size_of;

#[path = "🧩️revision-merge/🦀️.rs"]
mod revision_merge;
pub use revision_merge::RevisionIndex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XrefRow {
    pub number: u32,
    pub entry: Option<XrefEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefIndexStatus {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Inserted,
    AlreadyKnown,
    Cancelled,
    Closed,
}

#[derive(Debug, PartialEq, Eq)]
pub struct XrefIndexReceipt {
    pub status: Result<XrefIndexStatus, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

/// 🧭️ Visits a physically immutable sparse index with one slot inspection per work unit.
#[derive(Default)]
pub struct XrefIndexScan { position: usize }

impl XrefIndexScan {
    pub fn advance(&mut self, index: &XrefIndex, grant: usize) -> (usize, bool, Option<XrefRow>) {
        let end = self.position.saturating_add(grant).min(index.slots.len());
        let start = self.position;
        while self.position < end {
            let row = index.slots[self.position];
            self.position += 1;
            if row.is_some() { return (self.position - start, false, row); }
        }
        (end - start, end == index.slots.len(), None)
    }
}

/// 🧷️ Finds a live row in the immutable index, accounting for each collision slot inspected.
pub struct XrefIndexLookup {
    number: u32,
    offset: Option<usize>,
    result: Option<Option<XrefEntry>>,
}

impl XrefIndexLookup {
    pub fn new(number: u32) -> Self { Self { number, offset: None, result: None } }

    pub fn advance(&mut self, index: &XrefIndex, grant: usize) -> (usize, Option<Option<XrefEntry>>) {
        if self.result.is_some() { return (0, self.result); }
        let mut offset = self.offset.unwrap_or_else(|| position(self.number, index.slots.len()));
        for work in 1..=grant {
            self.result = match index.slots.get(offset).and_then(Option::as_ref) {
                Some(row) if row.number != self.number => None,
                row => Some(row.and_then(|row| row.entry)),
            };
            if self.result.is_some() { return (work, self.result); }
            offset = next(offset, index.slots.len());
        }
        self.offset = Some(offset);
        (grant, None)
    }
}

type Slots = Vec<Option<XrefRow>>;

struct Growth {
    slots: Slots,
    target: usize,
    next_old: usize,
    position: Option<usize>,
}

#[derive(Default)]
pub struct XrefIndex {
    slots: Slots,
    growth: Option<Growth>,
    pending: Option<(XrefRow, usize, bool)>,
    live: usize,
    entries: usize,
    closing: bool,
}

impl XrefIndex {
    /// 📥️ Accepts one row from the newest section or its older revisions in precedence order.
    pub fn offer(&mut self, row: XrefRow) -> bool {
        self.offer_ordered(row, false)
    }

    fn offer_ordered(&mut self, row: XrefRow, replace_live: bool) -> bool {
        if self.pending.is_some() || self.closing { return false; }
        self.pending = Some((row, position(row.number, self.slots.len()), replace_live));
        true
    }

    pub fn rows(&self) -> impl Iterator<Item = XrefRow> + '_ {
        self.slots.iter().flatten().copied()
    }

    pub fn len(&self) -> usize { self.live }
    pub fn is_empty(&self) -> bool { self.live == 0 }

    /// 📏️ Bytes of slot backing the index holds, the release its close credits.
    pub fn held_bytes(&self) -> usize { (self.slots.capacity() + self.growth.as_ref().map_or(0, |growth| growth.slots.capacity())) * std::mem::size_of::<Option<XrefRow>>() }

    pub fn get(&self, number: &u32) -> Option<&XrefEntry> { self.find(*number)?.entry.as_ref() }
    pub fn contains_key(&self, number: &u32) -> bool { self.get(number).is_some() }
    pub fn is_free(&self, number: u32) -> bool { self.find(number).is_some_and(|row| row.entry.is_none()) }

    pub fn iter(&self) -> impl Iterator<Item = (&u32, &XrefEntry)> + '_ {
        self.slots.iter().flatten().filter_map(|row| row.entry.as_ref().map(|entry| (&row.number, entry)))
    }

    fn find(&self, number: u32) -> Option<&XrefRow> {
        let mut at = position(number, self.slots.len());
        while let Some(row) = self.slots.get(at)?.as_ref() {
            if row.number == number { return Some(row); }
            at = next(at, self.slots.len());
        }
        None
    }

    /// 🧾️ Reference drain for a completed row, keeping free entries authoritative during repair.
    pub fn insert_row(&mut self, row: XrefRow, replace_live: bool) -> Result<(), &'static str> {
        if !self.offer_ordered(row, replace_live) { return Err("xref index already owns a pending row"); }
        let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
        loop {
            match self.advance(grant, false).status? {
                XrefIndexStatus::Inserted | XrefIndexStatus::AlreadyKnown => return Ok(()),
                XrefIndexStatus::More | XrefIndexStatus::NeedsAllocation { .. } | XrefIndexStatus::NeedsRelease { .. } => {}
                _ => return Err("xref index insertion stopped"),
            }
        }
    }

    /// ▶️ Fills or rehashes at most one slot per work unit and acquires only admitted capacity.
    pub fn advance(&mut self, grant: CosGrant, cancelled: bool) -> XrefIndexReceipt {
        let mut result = receipt(XrefIndexStatus::More);
        if cancelled { self.begin_close(); }
        if self.closing { result.status = Ok(XrefIndexStatus::Cancelled); return result; }
        while result.work < grant.work {
            result.work += 1;
            if let Some(growth) = self.growth.as_mut() {
                if growth.slots.len() < growth.target {
                    growth.slots.push(None);
                } else if let Some(slot) = self.slots.get(growth.next_old) {
                    if let Some(row) = *slot {
                        let index = growth.position.get_or_insert_with(|| position(row.number, growth.slots.len()));
                        if growth.slots[*index].is_some() {
                            *index = next(*index, growth.slots.len());
                            continue;
                        }
                        growth.slots[*index] = Some(row);
                    }
                    growth.position = None;
                    growth.next_old += 1;
                } else {
                    let bytes = self.slots.capacity() * size_of::<Option<XrefRow>>();
                    if bytes > grant.release_bytes - result.released_bytes {
                        result.status = Ok(XrefIndexStatus::NeedsRelease { bytes });
                        return result;
                    }
                    self.slots = self.growth.take().expect("growth backing").slots;
                    result.released_bytes += bytes;
                    if let Some((row, index, _)) = &mut self.pending { *index = position(row.number, self.slots.len()); }
                }
                continue;
            }
            let Some((row, index, replace_live)) = self.pending.as_mut() else { return result };
            if let Some(Some(current)) = self.slots.get_mut(*index) {
                if current.number == row.number {
                    let replace = *replace_live && current.entry.is_some() && row.entry.is_some();
                    if replace { *current = *row; }
                    self.pending = None;
                    result.status = Ok(if replace { XrefIndexStatus::Inserted } else { XrefIndexStatus::AlreadyKnown });
                    return result;
                }
                *index = next(*index, self.slots.len());
            } else if self.entries < self.slots.len() / 2 {
                self.slots[*index] = Some(*row);
                self.entries += 1;
                self.live += usize::from(row.entry.is_some());
                self.pending = None;
                result.status = Ok(XrefIndexStatus::Inserted);
                return result;
            } else {
                let target = self.slots.len().checked_mul(2).unwrap_or(usize::MAX).max(8);
                let Some(bytes) = target.checked_mul(size_of::<Option<XrefRow>>()) else {
                    result.status = Err("xref index capacity overflow");
                    return result;
                };
                if bytes > grant.allocation_bytes - result.allocated_bytes {
                    result.status = Ok(XrefIndexStatus::NeedsAllocation { bytes });
                    return result;
                }
                let mut slots = Vec::new();
                if slots.try_reserve_exact(target).is_err() {
                    result.status = Err("xref index allocation refused");
                    return result;
                }
                result.allocated_bytes += bytes;
                self.growth = Some(Growth { slots, target, next_old: 0, position: None });
            }
        }
        result
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        self.pending = None;
    }

    /// ♻️ Returns one resident table backing per turn.
    pub fn close_step(&mut self, grant: CosGrant) -> XrefIndexReceipt {
        if !self.closing || grant.work == 0 { return receipt(XrefIndexStatus::More); }
        let slots = self.growth.as_mut().map_or(&mut self.slots, |growth| &mut growth.slots);
        let bytes = slots.capacity() * size_of::<Option<XrefRow>>();
        if bytes > grant.release_bytes { return receipt(XrefIndexStatus::NeedsRelease { bytes }); }
        *slots = Vec::new();
        if self.growth.take().is_none() {
            self.entries = 0;
            self.live = 0;
            return XrefIndexReceipt { released_bytes: bytes, work: usize::from(bytes != 0), ..receipt(XrefIndexStatus::Closed) };
        }
        XrefIndexReceipt { released_bytes: bytes, work: 1, ..receipt(XrefIndexStatus::More) }
    }
}

fn receipt(status: XrefIndexStatus) -> XrefIndexReceipt {
    XrefIndexReceipt { status: Ok(status), work: 0, allocated_bytes: 0, released_bytes: 0 }
}

fn position(number: u32, slots: usize) -> usize {
    let mixed = (number ^ (number >> 16)).wrapping_mul(0x7feb_352d);
    let mixed = (mixed ^ (mixed >> 15)).wrapping_mul(0x846c_a68b);
    (mixed ^ (mixed >> 16)) as usize & slots.saturating_sub(1)
}

fn next(position: usize, slots: usize) -> usize { (position + 1) & (slots - 1) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_spends_one_unit_per_collision_and_stops_on_tombstones() {
        let mut index = XrefIndex::default();
        index.slots = vec![None; 8];
        let start = position(17, index.slots.len());
        index.slots[start] = Some(XrefRow { number: 20, entry: None });
        let after_collision = next(start, index.slots.len());
        index.slots[after_collision] = Some(XrefRow { number: 17, entry: None });
        let mut lookup = XrefIndexLookup::new(17);
        assert_eq!(lookup.advance(&index, 0), (0, None));
        assert_eq!(lookup.advance(&index, 1), (1, None));
        assert_eq!(lookup.advance(&index, 1), (1, Some(None)));
        assert_eq!(lookup.advance(&index, 1), (0, Some(None)));
    }
}
