//! 🛤️ Locates and validates a structured PDF cross-reference table under a single pooled turn budget.

use semio_framework_value::ByteReadSource;
use super::{FooterCursor, FooterStep, XrefEntry, StructuredTrailer, XrefTrailerCursor, XrefAssemblyCursor, XrefAssemblyEvent, XrefIndex, XrefIndexStatus, XrefOffsetCursor, XrefOffsetStatus, XrefStreamError};
use semio_s_artifact_stdio_pdf_lexer::{CosGrant, CosLimits, CosReceipt, CosStatus};
use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error::PResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructuredXrefEvent {
    More,
    RecoveryRequired,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete { offsets_valid: bool },
    Cancelled,
    Closed,
}

pub struct StructuredXrefReceipt {
    pub event: Result<StructuredXrefEvent, XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum Phase {
    Footer(FooterCursor),
    Revisions,
    FoldTrailer,
    Validate(XrefOffsetCursor),
    Complete { offsets_valid: bool },
    CloseRevisions,
    CloseIndex,
    CloseTrailers,
    Closed,
}

/// 🧭️ Holds the authoritative directory until every live normal offset agrees with its generation.
pub struct StructuredXrefCursor {
    phase: Phase,
    revisions: XrefAssemblyCursor,
    index: Option<XrefIndex>,
    trailers: XrefTrailerCursor,
    limits: CosLimits,
    maximum_stream_bytes: usize,
}

impl StructuredXrefCursor {
    pub fn new(source_len: usize, limits: CosLimits, maximum_stream_bytes: usize) -> Self {
        Self {
            phase: Phase::Footer(FooterCursor::new(source_len)),
            revisions: XrefAssemblyCursor::new(None, limits, maximum_stream_bytes),
            index: None, trailers: XrefTrailerCursor::default(), limits, maximum_stream_bytes,
        }
    }

    /// ▶️ Accepts the same stable source slice from the footer probe through the last object offset.
    pub fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), grant: CosGrant, cancelled: bool) -> StructuredXrefReceipt {
        use StructuredXrefEvent as Event;
        if cancelled { self.begin_close(); }
        match &mut self.phase {
            Phase::Closed => step(Event::Closed, 0),
            Phase::CloseRevisions | Phase::CloseIndex | Phase::CloseTrailers => step(Event::Cancelled, 0),
            Phase::Complete { offsets_valid } => step(Event::Complete { offsets_valid: *offsets_valid }, 0),
            _ if grant.work == 0 => step(Event::More, 0),
            Phase::Footer(cursor) => {
                let (work, result) = cursor.advance_counted(|position| source.byte_at(position), grant.work);
                if let FooterStep::Complete(offset) = result {
                    self.revisions = XrefAssemblyCursor::new(offset, self.limits, self.maximum_stream_bytes);
                    self.phase = Phase::Revisions;
                }
                step(Event::More, work)
            }
            Phase::Revisions => {
                let result = self.revisions.advance(source, grant, false);
                let event = match result.event {
                    Ok(XrefAssemblyEvent::PrimaryTrailer) => {
                        let trailer = self.revisions.take_primary_trailer().expect("xref primary trailer event owns a trailer");
                        assert!(self.trailers.offer(trailer).is_ok(), "xref trailer cursor already owns an unfinished revision");
                        self.phase = Phase::FoldTrailer;
                        Ok(Event::More)
                    }
                    Ok(XrefAssemblyEvent::Complete) => {
                        self.index = self.revisions.take_index();
                        if self.index.is_some() {
                            self.phase = Phase::Validate(XrefOffsetCursor::default());
                            Ok(Event::More)
                        } else {
                            Err(XrefStreamError::Structure("xref assembly lost its authoritative directory"))
                        }
                    }
                    status => status.map(map_event),
                };
                StructuredXrefReceipt { event, work: result.work, allocated_bytes: result.allocated_bytes, released_bytes: result.released_bytes }
            }
            Phase::FoldTrailer => {
                let result = self.trailers.advance(grant);
                if matches!(result.status, Ok(CosStatus::Complete { .. })) { self.phase = Phase::Revisions; }
                trailer_receipt(result)
            }
            Phase::Validate(cursor) => {
                let index = self.index.as_ref().expect("structured xref validation owns an index");
                let (work, result) = cursor.advance(|position| source.byte_at(position), index, grant.work, false);
                let offsets_valid = match result {
                    XrefOffsetStatus::More => return step(Event::More, work),
                    XrefOffsetStatus::Cancelled => return step(Event::Cancelled, work),
                    XrefOffsetStatus::Complete => !index.is_empty(),
                    XrefOffsetStatus::Invalid { .. } => false,
                };
                self.phase = Phase::Complete { offsets_valid };
                step(Event::Complete { offsets_valid }, work)
            }
        }
    }

    /// 🏃️ Bytes the next unit writes when it is a run unit: a footer, table or object-header byte, or one index slot; `None` for trailers and every other action.
    pub fn run_write_bytes(&self) -> Option<usize> {
        match self.phase {
            Phase::Footer(_) | Phase::Validate(_) => Some(1),
            Phase::Revisions => self.revisions.run_write_bytes(),
            _ => None,
        }
    }

    /// 🗝️ Checks the original trailer root before choosing between absolute and header-relative offsets.
    pub fn root_resolves(&self) -> bool {
        let Some(root) = self.trailers.ready_trailer().and_then(|trailer| trailer.get_inheritable("Root")).and_then(|value| value.as_ref()) else { return false; };
        matches!(self.index.as_ref().and_then(|index| index.get(&root.num)), Some(XrefEntry::Normal { gen, .. }) if *gen == root.gen)
            || root.gen == 0 && matches!(self.index.as_ref().and_then(|index| index.get(&root.num)), Some(XrefEntry::Compressed { .. }))
    }

    /// 📤️ Transfers the directory and merged trailer to the importer or repair path.
    pub fn take(&mut self) -> Option<(XrefIndex, StructuredTrailer)> {
        let index = matches!(self.phase, Phase::Complete { .. }).then(|| self.index.take()).flatten()?;
        let trailer = self.trailers.take().expect("finished xref owns its settled primary trailers");
        Some((index, trailer))
    }

    /// 🛟️ Uses the reference decoder at the request of a synchronous importer.
    pub fn recover_compatible(&mut self, source: &[u8]) -> PResult<bool> { self.revisions.recover_compatible(source) }

    /// ⏩️ Abandons a failed provisional section while retaining earlier admitted revisions.
    pub fn skip_failed(&mut self) -> bool { self.revisions.skip_failed() }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::CloseRevisions | Phase::CloseIndex | Phase::CloseTrailers | Phase::Closed) {
            self.revisions.begin_close();
            self.trailers.begin_close();
            if let Some(index) = self.index.as_mut() { index.begin_close(); }
            self.phase = Phase::CloseRevisions;
        }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> StructuredXrefReceipt {
        use StructuredXrefEvent as Event;
        match self.phase {
            Phase::Closed => step(Event::Closed, 0),
            Phase::CloseRevisions => {
                let result = self.revisions.close_step(grant);
                let event = result.event.map(|event| {
                    if event == XrefAssemblyEvent::Closed {
                        self.phase = Phase::CloseIndex;
                        Event::More
                    } else { map_event(event) }
                });
                StructuredXrefReceipt { event, work: result.work, allocated_bytes: result.allocated_bytes, released_bytes: result.released_bytes }
            }
            Phase::CloseIndex => {
                let Some(index) = self.index.as_mut() else {
                    self.phase = Phase::CloseTrailers;
                    return step(Event::More, 0);
                };
                let result = index.close_step(grant);
                let event = result.status.map_err(XrefStreamError::Structure).map(|status| match status {
                    XrefIndexStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
                    XrefIndexStatus::Closed => {
                        self.index = None;
                        self.phase = Phase::CloseTrailers;
                        Event::More
                    }
                    _ => Event::More,
                });
                StructuredXrefReceipt { event, work: result.work, allocated_bytes: result.allocated_bytes, released_bytes: result.released_bytes }
            }
            Phase::CloseTrailers => {
                let result = self.trailers.advance(grant);
                if result.status == Ok(CosStatus::Closed) { self.phase = Phase::Closed; }
                trailer_receipt(result)
            }
            _ => step(Event::More, 0),
        }
    }
}

fn step(event: StructuredXrefEvent, work: usize) -> StructuredXrefReceipt {
    StructuredXrefReceipt { event: Ok(event), work, allocated_bytes: 0, released_bytes: 0 }
}

fn trailer_receipt(result: CosReceipt) -> StructuredXrefReceipt {
    use StructuredXrefEvent as Event;
    let event = result.status.map_err(XrefStreamError::Structure).map(|state| match state {
        CosStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        CosStatus::Closed => Event::Closed,
        _ => Event::More,
    });
    StructuredXrefReceipt { event, work: result.work, allocated_bytes: result.allocated_bytes, released_bytes: result.released_bytes }
}

fn map_event(event: XrefAssemblyEvent) -> StructuredXrefEvent {
    use StructuredXrefEvent as Event;
    match event {
        XrefAssemblyEvent::More | XrefAssemblyEvent::Complete => Event::More,
        XrefAssemblyEvent::PrimaryTrailer => Event::More,
        XrefAssemblyEvent::RecoveryRequired => Event::RecoveryRequired,
        XrefAssemblyEvent::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        XrefAssemblyEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        XrefAssemblyEvent::Cancelled => Event::Cancelled,
        XrefAssemblyEvent::Closed => Event::Closed,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
