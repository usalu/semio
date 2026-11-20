//! 📨️ Transfers inherited PDF trailer fields and retires older dictionaries under source grants.

use semio_s_artifact_stdio_pdf_lexer::{CosGrant, CosReceipt, CosStatus, PdfCosCursor};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfObject};

const INHERITABLE: [&str; 4] = ["Root", "Info", "ID", "Encrypt"];

/// 🧾️ Keeps the newest trailer in its original backing with at most four inherited entries.
#[derive(Default)]
pub struct StructuredTrailer {
    latest: Vec<PdfDictEntry>,
    positions: [Option<usize>; INHERITABLE.len()],
    inherited: [Option<PdfDictEntry>; INHERITABLE.len()],
}

impl StructuredTrailer {
    /// 🧹️ Re-adopts the transferred parts for physically bounded retirement.
    pub fn into_retirement(self) -> XrefTrailerCursor { XrefTrailerCursor { result: Some(self), phase: Phase::Closing, ..XrefTrailerCursor::default() } }

    pub fn get_inheritable(&self, key: &str) -> Option<&PdfObject> {
        let slot = inheritable(key)?;
        if let Some(index) = self.positions[slot] { return Some(&self.latest[index].value); }
        self.inherited.iter().flatten().find(|entry| entry.key == key).map(|entry| &entry.value)
    }

    /// 🫴️ Transfers original trailer fields for bounded compaction by the importer.
    pub fn into_parts(self) -> (Vec<PdfDictEntry>, [Option<PdfDictEntry>; 4]) { (self.latest, self.inherited) }

    /// 📦️ Materializes the complete ordered dictionary for a reference caller.
    pub fn into_entries(mut self) -> Vec<PdfDictEntry> {
        self.latest.extend(self.inherited.into_iter().flatten());
        self.latest
    }
}

#[derive(Default)]
enum Phase { #[default] Idle, Latest, Earlier, RetireEarlier, Closing, Closed }

#[derive(Default)]
pub struct XrefTrailerCursor {
    result: Option<StructuredTrailer>,
    earlier: Option<Vec<PdfDictEntry>>,
    retirement: Option<PdfCosCursor>,
    seen: [bool; INHERITABLE.len()],
    inherited_count: usize,
    position: usize,
    phase: Phase,
}

impl XrefTrailerCursor {
    /// 📥️ Receives a validated primary trailer without copying its source-owned fields.
    pub fn offer(&mut self, entries: Vec<PdfDictEntry>) -> Result<(), Vec<PdfDictEntry>> {
        if !matches!(self.phase, Phase::Idle) { return Err(entries); }
        self.position = 0;
        if self.result.is_some() {
            self.earlier = Some(entries);
            self.phase = Phase::Earlier;
        } else {
            self.result = Some(StructuredTrailer { latest: entries, ..StructuredTrailer::default() });
            self.phase = Phase::Latest;
        }
        Ok(())
    }

    /// ▶️ Visits one trailer entry or retires one partial backing action per unit.
    pub fn advance(&mut self, grant: CosGrant) -> CosReceipt {
        if matches!(self.phase, Phase::Closed) { return receipt(CosStatus::Closed, 0); }
        if matches!(self.phase, Phase::Idle) { return receipt(CosStatus::Complete { end: 0 }, 0); }
        if grant.work == 0 { return receipt(CosStatus::More, 0); }
        if let Some(cursor) = self.retirement.as_mut() {
            let mut result = cursor.close_step(grant);
            if result.status == Ok(CosStatus::Closed) {
                self.retirement = None;
                result.status = Ok(CosStatus::More);
                if matches!(self.phase, Phase::RetireEarlier) { self.phase = Phase::Idle; }
            }
            return result;
        }
        match self.phase {
            Phase::Latest => self.scan_latest(),
            Phase::Earlier => self.scan_earlier(),
            Phase::Closing => self.retire_next(),
            _ => unreachable!("xref trailer phase lacks its expected work owner"),
        }
        receipt(CosStatus::More, 1)
    }

    fn scan_latest(&mut self) {
        let latest = self.result.as_mut().expect("initial trailer");
        let Some(entry) = latest.latest.get(self.position) else { self.phase = Phase::Idle; return; };
        if let Some(slot) = inheritable(&entry.key) {
            latest.positions[slot].get_or_insert(self.position);
            self.seen[slot] = true;
        }
        self.position += 1;
    }

    fn scan_earlier(&mut self) {
        let earlier = self.earlier.as_mut().expect("earlier trailer");
        let Some(entry) = earlier.get_mut(self.position) else {
            let entries = self.earlier.take().expect("earlier trailer");
            self.retirement = Some(PdfCosCursor::retire_dictionary(entries));
            self.phase = Phase::RetireEarlier;
            return;
        };
        if let Some(slot) = inheritable(&entry.key).filter(|&slot| !self.seen[slot]) {
            let moved = std::mem::replace(entry, PdfDictEntry::new(String::new(), PdfObject::Null));
            self.result.as_mut().expect("latest trailer").inherited[self.inherited_count] = Some(moved);
            self.inherited_count += 1;
            self.seen[slot] = true;
        }
        self.position += 1;
    }

    pub(crate) fn ready_trailer(&self) -> Option<&StructuredTrailer> {
        self.result.as_ref().filter(|_| matches!(self.phase, Phase::Idle))
    }

    /// 📤️ Transfers the newest and inherited entries in their original custody.
    pub fn take(&mut self) -> Option<StructuredTrailer> {
        if matches!(self.phase, Phase::Idle) {
            self.phase = Phase::Closed;
            Some(self.result.take().unwrap_or_default())
        } else { None }
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::Closed) { self.phase = Phase::Closing; }
    }

    fn retire_next(&mut self) {
        if let Some(entries) = self.earlier.take() {
            self.retirement = Some(PdfCosCursor::retire_dictionary(entries));
        } else if let Some(result) = self.result.as_mut() {
            if result.latest.capacity() != 0 {
                self.retirement = Some(PdfCosCursor::retire_dictionary(std::mem::take(&mut result.latest)));
            } else if let Some(entry) = result.inherited.iter_mut().find_map(Option::take) {
                self.retirement = Some(PdfCosCursor::retire_dictionary_entry(entry));
            } else {
                self.result = None;
                self.phase = Phase::Closed;
            }
        } else { self.phase = Phase::Closed; }
    }
}

fn inheritable(key: &str) -> Option<usize> { INHERITABLE.iter().position(|&name| name == key) }

fn receipt(status: CosStatus, work: usize) -> CosReceipt {
    CosReceipt { status: Ok(status), work, allocated_bytes: 0, released_bytes: 0 }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
