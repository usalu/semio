//! 🧩️ Merges completed section indexes into the authoritative PDF object directory.

use super::{XrefIndex, XrefIndexReceipt, XrefIndexScan, XrefIndexStatus, XrefRow, receipt};
use semio_s_artifact_stdio_pdf_lexer::CosGrant;

#[derive(Clone, Copy)]
enum Phase { Collect, MergePrimary, MergeCompanion, ReleasePrimary, ReleaseCompanion, Closed }

pub struct RevisionIndex {
    result: XrefIndex,
    primary: XrefIndex,
    companion: XrefIndex,
    scan: XrefIndexScan,
    destination_pending: bool,
    phase: Phase,
    closing: bool,
}

impl Default for RevisionIndex {
    fn default() -> Self {
        Self {
            result: Default::default(), primary: Default::default(), companion: Default::default(),
            scan: Default::default(), destination_pending: false, phase: Phase::Collect, closing: false,
        }
    }
}

impl RevisionIndex {
    /// 📥️ Queues one row in its provisional section without publishing it after an incomplete parse.
    pub fn offer(&mut self, row: XrefRow, companion: bool) -> bool {
        if !matches!(self.phase, Phase::Collect) || self.closing { return false; }
        if companion { self.companion.offer(row) } else { self.primary.offer(row) }
    }

    pub fn insert_step(&mut self, companion: bool, grant: CosGrant) -> XrefIndexReceipt {
        if !matches!(self.phase, Phase::Collect) || self.closing { return receipt(XrefIndexStatus::Cancelled); }
        if companion { self.companion.advance(grant, false) } else { self.primary.advance(grant, false) }
    }

    /// 🧾️ Starts merging a complete revision, allowing its companion to override primary entries.
    pub fn begin_merge(&mut self, companion_is_valid: bool) -> bool {
        if !matches!(self.phase, Phase::Collect) || self.closing || self.primary.pending.is_some() || self.companion.pending.is_some() { return false; }
        self.phase = if companion_is_valid { Phase::MergeCompanion } else {
            self.companion.begin_close();
            Phase::ReleaseCompanion
        };
        true
    }

    pub fn discard_primary(&mut self) {
        self.primary.begin_close();
        self.phase = Phase::ReleasePrimary;
    }

    /// ▶️ Advances index merge and backing retirement within a single caller grant.
    pub fn merge_step(&mut self, grant: CosGrant) -> XrefIndexReceipt {
        if self.closing { return receipt(XrefIndexStatus::Cancelled); }
        match self.phase {
            Phase::Collect => return receipt(XrefIndexStatus::Inserted),
            Phase::Closed => return receipt(XrefIndexStatus::Closed),
            _ => {}
        }
        if self.destination_pending {
            let mut turn = self.result.advance(grant, false);
            if matches!(turn.status, Ok(XrefIndexStatus::Inserted | XrefIndexStatus::AlreadyKnown)) {
                self.destination_pending = false;
                turn.status = Ok(XrefIndexStatus::More);
            }
            return turn;
        }
        let (section, release, following) = match self.phase {
            Phase::MergePrimary => (&mut self.primary, Phase::ReleasePrimary, Phase::Collect),
            Phase::MergeCompanion => (&mut self.companion, Phase::ReleaseCompanion, Phase::MergePrimary),
            Phase::ReleasePrimary => (&mut self.primary, Phase::ReleasePrimary, Phase::Collect),
            Phase::ReleaseCompanion => (&mut self.companion, Phase::ReleaseCompanion, Phase::MergePrimary),
            Phase::Collect | Phase::Closed => unreachable!(),
        };
        if matches!(self.phase, Phase::ReleasePrimary | Phase::ReleaseCompanion) {
            let mut turn = section.close_step(grant);
            if matches!(turn.status, Ok(XrefIndexStatus::Closed)) {
                *section = XrefIndex::default();
                self.phase = following;
                self.scan = XrefIndexScan::default();
                turn.status = Ok(XrefIndexStatus::More);
            }
            return turn;
        }
        let (work, complete, row) = self.scan.advance(section, grant.work);
        if complete {
            section.begin_close();
            self.phase = release;
        }
        if let Some(row) = row {
            assert!(self.result.offer(row));
            self.destination_pending = true;
        }
        XrefIndexReceipt { work, ..receipt(XrefIndexStatus::More) }
    }

    /// 📤️ Moves the stable directory after every completed revision settles.
    pub fn take(&mut self) -> Option<XrefIndex> {
        if !matches!(self.phase, Phase::Collect) || self.closing || self.primary.entries != 0 || self.companion.entries != 0 || self.primary.pending.is_some() || self.companion.pending.is_some() { return None; }
        self.phase = Phase::Closed;
        Some(std::mem::take(&mut self.result))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        self.primary.begin_close();
        self.companion.begin_close();
        self.result.begin_close();
    }

    pub fn close_step(&mut self, grant: CosGrant) -> XrefIndexReceipt {
        if !self.closing { return receipt(XrefIndexStatus::More); }
        for table in [&mut self.primary, &mut self.companion, &mut self.result] {
            let turn = table.close_step(grant);
            if turn.work != 0 || !matches!(turn.status, Ok(XrefIndexStatus::Closed)) {
                return XrefIndexReceipt { status: turn.status.map(|status| {
                    if status == XrefIndexStatus::Closed { XrefIndexStatus::More } else { status }
                }), ..turn };
            }
        }
        self.phase = Phase::Closed;
        receipt(XrefIndexStatus::Closed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::XrefEntry;

    #[test]
    fn companion_and_newest_tombstone_win_under_single_unit_merge() {
        let mut merged = RevisionIndex::default();
        let grant = CosGrant { work: 1, allocation_bytes: 16 * 1024, release_bytes: 16 * 1024 };
        for revision in [
            [(true, 3, None), (false, 3, Some(XrefEntry::Normal { offset: 1174, gen: 0 }))],
            [(false, 3, Some(XrefEntry::Normal { offset: 269, gen: 0 })), (false, 4, Some(XrefEntry::Normal { offset: 400, gen: 0 }))],
        ] {
            for (companion, number, entry) in revision {
                assert!(merged.offer(XrefRow { number, entry }, companion));
                loop {
                    let turn = merged.insert_step(companion, grant);
                    assert!(turn.work <= 1);
                    if turn.status.unwrap() == XrefIndexStatus::Inserted { break; }
                }
            }
            assert!(merged.begin_merge(revision.iter().any(|(companion, _, _)| *companion)));
            loop {
                let turn = merged.merge_step(grant);
                assert!(turn.work <= 1);
                if turn.status.unwrap() == XrefIndexStatus::Inserted { break; }
            }
        }
        let result = merged.take().unwrap();
        assert!(result.is_free(3));
        assert_eq!(result.len(), 1);
        assert_eq!(result.get(&4), Some(&XrefEntry::Normal { offset: 400, gen: 0 }));
    }
}
