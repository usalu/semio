//! 🧹️ Projects parsed stream dictionaries without copying retained fields or dropping rejected trees in a turn.

use super::{CosGrant, CosReceipt, CosStatus, PdfCosCursor, PdfDictEntry, PdfObject};

enum Phase { Scan, ClearTail, Complete, Cancelled, Closing, Closed }

/// 🗂️ Compacts a parsed stream dictionary and preserves its original field order.
pub struct PdfStreamDictionaryCursor {
    phase: Phase,
    dict: Vec<PdfDictEntry>,
    removed: Option<PdfCosCursor>,
    read: usize,
    keep: usize,
    normalized: bool,
    retain: fn(&PdfDictEntry, bool) -> bool,
}

impl PdfStreamDictionaryCursor {
    pub fn new(dict: Vec<PdfDictEntry>, normalized: bool) -> Self {
        Self { phase: Phase::Scan, dict, removed: None, read: 0, keep: 0, normalized, retain: retain_stream_entry }
    }

    /// ♻️ Uses the same bounded in-place compaction for another PDF dictionary role.
    pub fn with_retention(dict: Vec<PdfDictEntry>, retain: fn(&PdfDictEntry, bool) -> bool) -> Self {
        Self { retain, ..Self::new(dict, false) }
    }

    pub fn advance(&mut self, grant: CosGrant, cancelled: bool) -> CosReceipt {
        if cancelled && matches!(self.phase, Phase::Scan | Phase::ClearTail | Phase::Complete) { self.phase = Phase::Cancelled; }
        match self.phase {
            Phase::Cancelled => return report(CosStatus::Cancelled, 0),
            Phase::Complete => return report(CosStatus::Complete { end: self.keep }, 0),
            Phase::Closed => return report(CosStatus::Closed, 0),
            Phase::Closing => return report(CosStatus::More, 0),
            _ if grant.work == 0 => return report(CosStatus::More, 0),
            _ => {}
        }
        if self.removed.is_some() { return self.retire_turn(grant); }

        if matches!(self.phase, Phase::Scan) {
            let Some(entry) = self.dict.get_mut(self.read) else {
                self.phase = Phase::ClearTail;
                return report(CosStatus::More, 1);
            };
            if (self.retain)(entry, self.normalized) {
                self.dict.swap(self.keep, self.read);
                self.keep += 1;
            } else {
                let detached = std::mem::replace(entry, PdfDictEntry::new(String::new(), PdfObject::Null));
                self.removed = Some(PdfCosCursor::retire_dictionary_entry(detached));
            }
            self.read += 1;
            return report(CosStatus::More, 1);
        }

        let remove = (self.dict.len() - self.keep).min(grant.work);
        self.dict.truncate(self.dict.len() - remove);
        if self.dict.len() == self.keep {
            self.phase = Phase::Complete;
            report(CosStatus::Complete { end: self.keep }, remove)
        } else { report(CosStatus::More, remove) }
    }

    pub fn take_dictionary(&mut self) -> Option<Vec<PdfDictEntry>> {
        if !matches!(self.phase, Phase::Complete) { return None; }
        self.phase = Phase::Closed;
        Some(std::mem::take(&mut self.dict))
    }

    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        if matches!(self.phase, Phase::Closed) { return report(CosStatus::Closed, 0); }
        self.phase = Phase::Closing;
        if grant.work == 0 { return report(CosStatus::More, 0); }
        if self.removed.is_some() { return self.retire_turn(grant); }
        if self.dict.capacity() != 0 {
            self.removed = Some(PdfCosCursor::retire_dictionary(std::mem::take(&mut self.dict)));
            return report(CosStatus::More, 1);
        }
        self.phase = Phase::Closed;
        report(CosStatus::Closed, 0)
    }

    fn retire_turn(&mut self, grant: CosGrant) -> CosReceipt {
        let mut step = self.removed.as_mut().expect("detached PDF dictionary owner").close_step(grant);
        if step.status == Ok(CosStatus::Closed) {
            self.removed = None;
            step.status = Ok(CosStatus::More);
        }
        step
    }
}

/// 🧾️ Keeps external-file declarations and preserves filter declarations when decoding was unavailable.
pub fn retain_stream_entry(entry: &PdfDictEntry, normalized: bool) -> bool {
    entry.key != "Length" && (!normalized || match entry.key.as_str() {
        "Filter" | "DecodeParms" | "DP" => false,
        "F" => !matches!(entry.value, PdfObject::Name(_) | PdfObject::Array(_)),
        _ => true,
    })
}

fn report(status: CosStatus, work: usize) -> CosReceipt {
    CosReceipt { status: Ok(status), work, allocated_bytes: 0, released_bytes: 0 }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
