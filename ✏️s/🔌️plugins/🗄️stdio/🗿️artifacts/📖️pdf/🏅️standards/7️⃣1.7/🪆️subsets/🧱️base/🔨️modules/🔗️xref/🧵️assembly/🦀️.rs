//! 🧵️ Assembles authoritative xref rows one paid revision action at a time.

use semio_framework_value::ByteReadSource;
use super::{
    RevisionIndex, XrefIndex, XrefIndexReceipt, XrefIndexStatus, XrefRevisionCursor, XrefRevisionEvent,
    XrefRevisionReceipt, XrefStreamError,
};
use semio_s_artifact_stdio_pdf_lexer::{CosGrant, CosLimits, XrefPrimaryCycle, XrefTarget};
use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error::PResult;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefAssemblyEvent {
    More,
    PrimaryTrailer,
    RecoveryRequired,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete,
    Cancelled,
    Closed,
}

pub struct XrefAssemblyReceipt {
    pub event: Result<XrefAssemblyEvent, XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum Phase { Read, Visit(XrefTarget), Insert { companion: bool }, Trailer, Merge, Recover, Complete, CloseRevision, CloseIndex, Closed }

/// 🧩️ Publishes only complete revision rows while lending every primary trailer to its enclosing importer.
pub struct XrefAssemblyCursor {
    revisions: XrefRevisionCursor,
    index: RevisionIndex,
    cycle: XrefPrimaryCycle,
    phase: Phase,
    primary_pending: bool,
}

impl XrefAssemblyCursor {
    pub fn new(start: Option<usize>, limits: CosLimits, maximum_stream_bytes: usize) -> Self {
        Self {
            revisions: XrefRevisionCursor::new(start, limits, maximum_stream_bytes),
            index: RevisionIndex::default(), cycle: XrefPrimaryCycle::default(),
            phase: Phase::Read, primary_pending: false,
        }
    }

    /// ▶️ Drives one child action under the source, allocation, and release grant.
    pub fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), grant: CosGrant, cancelled: bool) -> XrefAssemblyReceipt {
        use XrefAssemblyEvent as Event;
        if cancelled { self.begin_close(); }
        let event = match self.phase {
            Phase::Closed => Event::Closed,
            Phase::CloseRevision | Phase::CloseIndex => Event::Cancelled,
            Phase::Complete => Event::Complete,
            Phase::Trailer => Event::PrimaryTrailer,
            Phase::Recover => Event::RecoveryRequired,
            _ if grant.work == 0 => Event::More,
            Phase::Visit(target) => {
                let admitted = match target {
                    XrefTarget::Primary(offset) => offset < source.byte_len() && !self.cycle.revisits(offset),
                    XrefTarget::Companion(offset) => offset < source.byte_len(),
                };
                if !self.revisions.admit_visit(target, admitted) { return fault("xref revision lost its requested target", 1); }
                self.phase = Phase::Read;
                if matches!(target, XrefTarget::Companion(_)) && !admitted && !self.merge(false) {
                    return fault("xref section could not merge its primary index", 1);
                }
                return response(Ok(Event::More), 1, 0, 0);
            }
            Phase::Insert { companion } => {
                let step = self.index.insert_step(companion, grant);
                if matches!(step.status, Ok(XrefIndexStatus::Inserted | XrefIndexStatus::AlreadyKnown)) {
                    self.phase = Phase::Read;
                }
                return index_response(step);
            }
            Phase::Merge => {
                let step = self.index.merge_step(grant);
                if matches!(step.status, Ok(XrefIndexStatus::Inserted)) {
                    self.primary_pending = false;
                    self.phase = Phase::Read;
                }
                return index_response(step);
            }
            Phase::Read => {
                let step = self.revisions.advance(source, grant, false);
                return self.read(step);
            }
        };
        response(Ok(event), 0, 0, 0)
    }

    fn read(&mut self, step: XrefRevisionReceipt) -> XrefAssemblyReceipt {
        use XrefAssemblyEvent as Event;
        let event = match step.event {
            Ok(XrefRevisionEvent::Visit(target)) => { self.phase = Phase::Visit(target); Ok(Event::More) }
            Ok(XrefRevisionEvent::Row { target, row }) => {
                let companion = matches!(target, XrefTarget::Companion(_));
                if self.index.offer(row, companion) {
                    self.phase = Phase::Insert { companion };
                    Ok(Event::More)
                } else { Err(XrefStreamError::Structure("xref section already owns an uncommitted row")) }
            }
            Ok(XrefRevisionEvent::Trailer(XrefTarget::Primary(_))) => { self.phase = Phase::Trailer; Ok(Event::PrimaryTrailer) }
            Ok(XrefRevisionEvent::Trailer(XrefTarget::Companion(_))) => {
                self.revisions.skip_target();
                if self.merge(true) { Ok(Event::More) } else { Err(XrefStreamError::Structure("xref companion arrived without an eligible primary")) }
            }
            Ok(XrefRevisionEvent::Complete) => { self.phase = Phase::Complete; Ok(Event::Complete) }
            Ok(XrefRevisionEvent::NeedsAllocation { bytes }) => Ok(Event::NeedsAllocation { bytes }),
            Ok(XrefRevisionEvent::NeedsRelease { bytes }) => Ok(Event::NeedsRelease { bytes }),
            Ok(XrefRevisionEvent::More) => Ok(Event::More),
            Err(XrefStreamError::Decode(_)) => { self.phase = Phase::Recover; Ok(Event::RecoveryRequired) }
            Err(_) => {
                if self.skip_failed() { Ok(Event::More) } else { Err(XrefStreamError::Structure("invalid xref revision could not be discarded")) }
            }
            Ok(_) => Err(XrefStreamError::Structure("xref revision stopped before completing its index")),
        };
        response(event, step.work, step.allocated_bytes, step.released_bytes)
    }

    /// 📂️ Moves the latest validated primary trailer while retaining the original index rows.
    pub fn take_primary_trailer(&mut self) -> Option<Vec<PdfDictEntry>> {
        if !matches!(self.phase, Phase::Trailer) { return None; }
        let companion = self.revisions.awaits_companion();
        if !companion && !self.index.begin_merge(false) { return None; }
        let trailer = self.revisions.take_trailer()?;
        self.primary_pending = true;
        self.phase = if companion { Phase::Read } else { Phase::Merge };
        Some(trailer)
    }

    fn merge(&mut self, companion: bool) -> bool {
        if !self.primary_pending || !self.index.begin_merge(companion) { return false; }
        self.phase = Phase::Merge;
        true
    }

    /// 🩹️ Allows a reference caller to try the compatibility decoder after a bounded refusal.
    pub fn recover_compatible(&mut self, source: &[u8]) -> PResult<bool> {
        if !matches!(self.phase, Phase::Recover) { return Ok(false); }
        let recovered = self.revisions.recover_compatible(source)?;
        if recovered { self.phase = Phase::Read; }
        Ok(recovered)
    }

    /// ⏭️ Drops provisional state for an undecodable section under the original grants.
    pub fn skip_failed(&mut self) -> bool {
        if !matches!(self.phase, Phase::Read | Phase::Recover) { return false; }
        self.revisions.skip_target();
        if self.primary_pending { self.merge(false) } else {
            self.index.discard_primary();
            self.phase = Phase::Merge;
            true
        }
    }

    /// 📤️ Moves the authoritative default-free and live index after all primary rows settle.
    pub fn take_index(&mut self) -> Option<XrefIndex> {
        if !matches!(self.phase, Phase::Complete) { return None; }
        self.index.take()
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::CloseRevision | Phase::CloseIndex | Phase::Closed) {
            self.revisions.begin_close();
            self.index.begin_close();
            self.phase = Phase::CloseRevision;
        }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> XrefAssemblyReceipt {
        use XrefAssemblyEvent as Event;
        match self.phase {
            Phase::CloseRevision => {
                let step = self.revisions.close_step(grant);
                let event = match step.event {
                    Ok(XrefRevisionEvent::Closed) => { self.phase = Phase::CloseIndex; Ok(Event::More) }
                    Ok(XrefRevisionEvent::NeedsRelease { bytes }) => Ok(Event::NeedsRelease { bytes }),
                    Ok(_) => Ok(Event::More),
                    Err(error) => Err(error),
                };
                response(event, step.work, step.allocated_bytes, step.released_bytes)
            }
            Phase::CloseIndex => {
                let result = index_response(self.index.close_step(grant));
                if matches!(result.event, Ok(Event::Closed)) { self.phase = Phase::Closed; }
                result
            }
            Phase::Closed => response(Ok(Event::Closed), 0, 0, 0),
            _ => response(Ok(Event::More), 0, 0, 0),
        }
    }
}

fn index_response(step: XrefIndexReceipt) -> XrefAssemblyReceipt {
    use XrefAssemblyEvent as Event;
    let event = step.status.map_err(XrefStreamError::Structure).map(|status| match status {
        XrefIndexStatus::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        XrefIndexStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        XrefIndexStatus::Closed => Event::Closed,
        XrefIndexStatus::Cancelled => Event::Cancelled,
        _ => Event::More,
    });
    response(event, step.work, step.allocated_bytes, step.released_bytes)
}

fn response(event: Result<XrefAssemblyEvent, XrefStreamError>, work: usize, allocated_bytes: usize, released_bytes: usize) -> XrefAssemblyReceipt {
    XrefAssemblyReceipt { event, work, allocated_bytes, released_bytes }
}

fn fault(reason: &'static str, work: usize) -> XrefAssemblyReceipt {
    response(Err(XrefStreamError::Structure(reason)), work, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FooterCursor, FooterStep};

    #[test]
    fn assembled_revisions_publish_newest_rows_and_close_partial_ownership() {
        let source = include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️incremental-free/🌀️flate.pdf");
        let mut footer = FooterCursor::new(source.len());
        let offset = loop {
            if let FooterStep::Complete(offset) = footer.advance(|position| source.get(position).copied(), 1) { break offset; }
        };
        for cancelled in [false, true] {
            let mut cursor = XrefAssemblyCursor::new(offset, CosLimits::default(), 4096);
            let cap = CosGrant { work: 1, allocation_bytes: 256 * 1024, release_bytes: 256 * 1024 };
            let mut allocated = 0;
            let mut released = 0;
            let mut primary = 0;
            let mut done = false;
            for _ in 0..50_000 {
                let receipt = cursor.advance(source, cap, cancelled && allocated > 0);
                assert!(receipt.work <= cap.work && receipt.allocated_bytes <= cap.allocation_bytes && receipt.released_bytes <= cap.release_bytes);
                allocated += receipt.allocated_bytes;
                released += receipt.released_bytes;
                match receipt.event.unwrap() {
                    XrefAssemblyEvent::PrimaryTrailer => {
                        let trailer = cursor.take_primary_trailer().unwrap();
                        if primary == 0 {
                            let info = trailer.iter().find(|entry| entry.key == "Info").unwrap().value.as_ref().unwrap();
                            assert_eq!(info.num, 9);
                        }
                        primary += 1;
                    }
                    XrefAssemblyEvent::More => {}
                    XrefAssemblyEvent::Complete => {
                        assert!(!cancelled);
                        let mut index = cursor.take_index().unwrap();
                        assert!(index.is_free(3));
                        assert!(index.contains_key(&9));
                        index.begin_close();
                        loop {
                            let step = index.close_step(cap);
                            released += step.released_bytes;
                            if step.status.unwrap() == XrefIndexStatus::Closed { break; }
                        }
                        done = true;
                        break;
                    }
                    XrefAssemblyEvent::Cancelled => {
                        assert!(cancelled);
                        assert!(cursor.take_index().is_none());
                        done = true;
                        break;
                    }
                    event => panic!("unexpected assembly event {event:?}"),
                }
            }
            assert!(done);
            assert_eq!(primary, if cancelled { 0 } else { 2 });
            cursor.begin_close();
            loop {
                let step = cursor.close_step(cap);
                assert!(step.work <= cap.work);
                released += step.released_bytes;
                if step.event.unwrap() == XrefAssemblyEvent::Closed { break; }
            }
            if cancelled { assert_eq!(released, allocated); }
            else { assert!(released <= allocated); }
        }
    }
}
