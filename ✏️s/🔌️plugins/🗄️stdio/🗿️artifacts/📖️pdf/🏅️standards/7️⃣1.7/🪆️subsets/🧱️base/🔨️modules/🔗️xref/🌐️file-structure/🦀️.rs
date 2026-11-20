//! 🌐️ Admits the PDF header and selects the intact native cross-reference coordinate system.

use super::{StructuredXrefCursor, StructuredXrefEvent, XrefIndex, XrefStreamError};
use semio_s_artifact_stdio_pdf_lexer::{CosGrant, CosLimits, PdfHeader, PdfHeaderCursor, PdfHeaderStep};
use semio_s_artifact_stdio_pdf_xref_stream::StructuredTrailer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfFileStructureEvent {
    More, NotPdf, RecoveryRequired, NeedsAllocation { bytes: usize }, NeedsRelease { bytes: usize },
    Complete, Cancelled, Closed,
}

pub struct PdfFileStructureReceipt {
    pub event: Result<PdfFileStructureEvent, XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

pub struct PdfFileStructure {
    pub header: PdfHeader,
    pub origin: usize,
    pub index: XrefIndex,
    pub trailer: StructuredTrailer,
    pub intact: bool,
    pub root_resolves: bool,
}

enum Phase { Header, Xref, RetryRelative, NotPdf, Complete { intact: bool, root_resolves: bool }, Closing, Closed }

/// 🧭️ Retires an incorrect absolute index before retrying a preambled file at its PDF origin.
pub struct PdfFileStructureCursor {
    probe: PdfHeaderCursor,
    header: Option<PdfHeader>,
    xref: Option<StructuredXrefCursor>,
    phase: Phase,
    origin: usize,
    maximum_stream_bytes: usize,
}

impl PdfFileStructureCursor {
    pub fn new(maximum_stream_bytes: usize) -> Self {
        Self { probe: PdfHeaderCursor::default(), header: None, xref: None, phase: Phase::Header, origin: 0, maximum_stream_bytes }
    }

    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> PdfFileStructureReceipt {
        use PdfFileStructureEvent as Event;
        if cancelled { self.begin_close(); }
        match self.phase {
            Phase::NotPdf => result(Event::NotPdf, 0),
            Phase::Complete { .. } => result(Event::Complete, 0),
            Phase::Closing => result(Event::Cancelled, 0),
            Phase::Closed => result(Event::Closed, 0),
            _ if grant.work == 0 => result(Event::More, 0),
            Phase::Header => {
                let (work, step) = self.probe.advance(source, grant.work);
                match step {
                    PdfHeaderStep::More => result(Event::More, work),
                    PdfHeaderStep::Complete(Some(header)) => {
                        self.header = Some(header);
                        self.xref = Some(StructuredXrefCursor::new(source.len(), CosLimits::default(), self.maximum_stream_bytes));
                        self.phase = Phase::Xref;
                        result(Event::More, work)
                    }
                    PdfHeaderStep::Complete(None) => { self.phase = Phase::NotPdf; result(Event::NotPdf, work) }
                }
            }
            Phase::Xref => {
                let cursor = self.xref.as_mut().expect("PDF source structure requires xref ownership");
                let step = cursor.advance(&source[self.origin..], grant, false);
                let event = match step.event {
                    Ok(StructuredXrefEvent::Complete { offsets_valid }) => {
                        let root_resolves = cursor.root_resolves();
                        let intact = root_resolves && offsets_valid;
                        if !intact && self.origin == 0 && self.header.is_some_and(|header| header.offset() != 0) {
                            cursor.begin_close();
                            self.phase = Phase::RetryRelative;
                            Ok(Event::More)
                        } else {
                            self.phase = Phase::Complete { intact, root_resolves };
                            Ok(Event::Complete)
                        }
                    }
                    status => status.map(map_event),
                };
                PdfFileStructureReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
            }
            Phase::RetryRelative => {
                let step = self.retire_xref(grant);
                if self.xref.is_none() {
                    self.origin = self.header.expect("header-relative structure has a header").offset();
                    self.xref = Some(StructuredXrefCursor::new(source.len() - self.origin, CosLimits::default(), self.maximum_stream_bytes));
                    self.phase = Phase::Xref;
                }
                step
            }
        }
    }

    /// 🛟️ Uses the reference decoder when the enclosing caller permits compatibility work.
    pub fn recover_compatible(&mut self, source: &[u8]) -> bool {
        let Some(cursor) = self.xref.as_mut() else { return false; };
        cursor.recover_compatible(&source[self.origin..]).ok() == Some(true) || cursor.skip_failed()
    }

    pub fn take(&mut self) -> Option<PdfFileStructure> {
        let Phase::Complete { intact, root_resolves } = self.phase else { return None; };
        let (index, trailer) = self.xref.as_mut()?.take()?;
        Some(PdfFileStructure { header: self.header?, origin: self.origin, index, trailer, intact, root_resolves })
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::Closing | Phase::Closed) {
            if let Some(cursor) = self.xref.as_mut() { cursor.begin_close(); }
            self.phase = Phase::Closing;
        }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> PdfFileStructureReceipt {
        match self.phase {
            Phase::Closed => result(PdfFileStructureEvent::Closed, 0),
            Phase::Closing if self.xref.is_none() => {
                self.phase = Phase::Closed;
                result(PdfFileStructureEvent::Closed, 0)
            }
            Phase::Closing => self.retire_xref(grant),
            _ => result(PdfFileStructureEvent::More, 0),
        }
    }

    fn retire_xref(&mut self, grant: CosGrant) -> PdfFileStructureReceipt {
        let step = self.xref.as_mut().expect("xref physical retirement").close_step(grant);
        let event = match step.event {
            Ok(StructuredXrefEvent::Closed) => { self.xref = None; Ok(PdfFileStructureEvent::More) }
            status => status.map(map_event),
        };
        PdfFileStructureReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
    }
}

fn result(event: PdfFileStructureEvent, work: usize) -> PdfFileStructureReceipt {
    PdfFileStructureReceipt { event: Ok(event), work, allocated_bytes: 0, released_bytes: 0 }
}

fn map_event(event: StructuredXrefEvent) -> PdfFileStructureEvent {
    use PdfFileStructureEvent as Event;
    match event {
        StructuredXrefEvent::More | StructuredXrefEvent::Complete { .. } => Event::More,
        StructuredXrefEvent::RecoveryRequired => Event::RecoveryRequired,
        StructuredXrefEvent::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        StructuredXrefEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        StructuredXrefEvent::Cancelled => Event::Cancelled,
        StructuredXrefEvent::Closed => Event::Closed,
    }
}
