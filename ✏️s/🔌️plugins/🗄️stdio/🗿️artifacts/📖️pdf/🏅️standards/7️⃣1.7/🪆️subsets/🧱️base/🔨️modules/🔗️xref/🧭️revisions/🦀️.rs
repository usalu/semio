//! 🗂️ Resumable traversal of the newest PDF cross-reference revision and its ancestors.

use semio_framework_value::ByteReadSource;
use super::{ClassicSectionCursor, ClassicSectionEvent, ClassicSectionReceipt, XrefEntry, XrefRow, XrefStreamError, XrefStreamEvent, XrefStreamReceipt, XrefStreamSectionCursor};
use semio_s_artifact_stdio_pdf_lexer::{
    CosGrant, CosLimits, CosStatus, XrefLinkCursor, XrefRevisionSchedule, XrefSectionKind, XrefSectionProbe, XrefTarget,
};
use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error::PResult;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefRevisionEvent {
    More,
    Visit(XrefTarget),
    Row { target: XrefTarget, row: XrefRow },
    Trailer(XrefTarget),
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete,
    Cancelled,
    Closed,
}

pub struct XrefRevisionReceipt {
    pub event: Result<XrefRevisionEvent, XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum Stage { Next, Visit, Probe(XrefSectionProbe), Section, Links(XrefLinkCursor), Trailer, Retire, Complete, Cancelled, Closed }
enum Section { Table(ClassicSectionCursor), Stream(XrefStreamSectionCursor) }

pub struct XrefRevisionCursor {
    stage: Stage,
    schedule: XrefRevisionSchedule,
    target: Option<XrefTarget>,
    section: Option<Section>,
    limits: CosLimits,
    maximum_stream_bytes: usize,
}

impl XrefRevisionCursor {
    /// 🌱️ Starts from the admitted footer offset and emits rows with their exact revision ownership.
    pub fn new(start: Option<usize>, limits: CosLimits, maximum_stream_bytes: usize) -> Self {
        Self { stage: Stage::Next, schedule: XrefRevisionSchedule::new(start), target: None, section: None, limits, maximum_stream_bytes }
    }

    /// 🪪️ Continues only once the caller has checked this target against its original offset index.
    pub fn admit_visit(&mut self, target: XrefTarget, allowed: bool) -> bool {
        if !matches!(self.stage, Stage::Visit) || self.target != Some(target) { return false; }
        self.stage = match target {
            XrefTarget::Primary(offset) if allowed => Stage::Probe(XrefSectionProbe::new(offset)),
            XrefTarget::Companion(offset) if allowed => {
                self.section = Some(Section::Stream(XrefStreamSectionCursor::new(offset, self.limits, self.maximum_stream_bytes, self.maximum_stream_bytes)));
                Stage::Section
            }
            XrefTarget::Companion(_) => Stage::Next,
            XrefTarget::Primary(_) => Stage::Complete,
        };
        true
    }

    pub fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), grant: CosGrant, cancelled: bool) -> XrefRevisionReceipt {
        use XrefRevisionEvent as Event;
        let mut receipt = response(Event::More);
        if cancelled { self.begin_close(); }
        receipt.event = match &mut self.stage {
            Stage::Cancelled => Ok(Event::Cancelled),
            Stage::Complete => Ok(Event::Complete),
            Stage::Closed => Ok(Event::Closed),
            Stage::Visit => Ok(Event::Visit(self.target.expect("pending xref target"))),
            Stage::Trailer => Ok(Event::Trailer(self.target.expect("completed xref target"))),
            _ if grant.work == 0 => Ok(Event::More),
            Stage::Next => {
                receipt.work = 1;
                self.target = self.schedule.next_target();
                self.stage = if self.target.is_some() { Stage::Visit } else { Stage::Complete };
                Ok(self.target.map_or(Event::Complete, Event::Visit))
            }
            Stage::Probe(probe) => {
                let (work, kind) = probe.advance(|position| source.byte_at(position), grant.work);
                receipt.work = work;
                if let Some(kind) = kind {
                    let Some(XrefTarget::Primary(offset)) = self.target else { unreachable!() };
                    self.section = Some(match kind {
                        XrefSectionKind::Table => Section::Table(ClassicSectionCursor::new(offset)),
                        XrefSectionKind::Stream => Section::Stream(XrefStreamSectionCursor::new(offset, self.limits, self.maximum_stream_bytes, self.maximum_stream_bytes)),
                    });
                    self.stage = Stage::Section;
                }
                Ok(Event::More)
            }
            Stage::Section => {
                let target = self.target.expect("reading xref target");
                let result = self.section.as_mut().expect("reading xref section").advance(source, grant, target);
                if matches!(result.event, Ok(Event::Trailer(_))) {
                    self.stage = if matches!(target, XrefTarget::Primary(_)) { Stage::Links(XrefLinkCursor::default()) } else { Stage::Trailer };
                    return XrefRevisionReceipt { event: Ok(Event::More), ..result };
                }
                return result;
            }
            Stage::Links(cursor) => {
                let trailer = self.section.as_ref().and_then(Section::trailer).expect("typed xref trailer");
                let (work, links) = cursor.advance(trailer, grant.work);
                receipt.work = work;
                if let Some(links) = links { self.schedule.finish_primary(links); self.stage = Stage::Trailer; }
                Ok(Event::More)
            }
            Stage::Retire => {
                let mut result = self.close_section(grant);
                if matches!(self.stage, Stage::Closed) { self.stage = Stage::Next; result.event = Ok(Event::More); }
                return result;
            }
        };
        receipt
    }

    pub fn awaits_companion(&self) -> bool { self.schedule.awaits_companion() }

    /// 🏃️ Whether the next unit reads one byte: the section probe or a classic table row.
    pub fn scans_bytes(&self) -> bool {
        match self.stage {
            Stage::Probe(_) => true,
            Stage::Section => matches!(&self.section, Some(Section::Table(cursor)) if cursor.scans_rows()),
            _ => false,
        }
    }

    pub fn recover_compatible(&mut self, source: &[u8]) -> PResult<bool> {
        match self.section.as_mut() { Some(Section::Stream(cursor)) => cursor.recover_compatible(source), _ => Ok(false) }
    }

    /// ↪️ Discards an unusable section before visiting the next retained target.
    pub fn skip_target(&mut self) {
        if let Some(section) = self.section.as_mut() { section.begin_close(); }
        self.stage = Stage::Retire;
    }

    /// 📤️ Transfers the section’s complete typed trailer and then retires the remaining parser state.
    pub fn take_trailer(&mut self) -> Option<Vec<PdfDictEntry>> {
        if !matches!(self.stage, Stage::Trailer) { return None; }
        let section = self.section.as_mut()?;
        let trailer = match section { Section::Table(cursor) => cursor.take_trailer(), Section::Stream(cursor) => cursor.take_trailer() }?;
        section.begin_close();
        self.stage = Stage::Retire;
        Some(trailer)
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.stage, Stage::Closed) { self.stage = Stage::Cancelled; }
        if let Some(section) = self.section.as_mut() { section.begin_close(); }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> XrefRevisionReceipt {
        if !matches!(self.stage, Stage::Cancelled | Stage::Closed) { return response(XrefRevisionEvent::More); }
        self.close_section(grant)
    }

    fn close_section(&mut self, grant: CosGrant) -> XrefRevisionReceipt {
        let result = self.section.as_mut().map_or_else(|| response(XrefRevisionEvent::Closed), |section| section.close(grant));
        if matches!(result.event, Ok(XrefRevisionEvent::Closed)) { self.section = None; self.stage = Stage::Closed; }
        result
    }
}

impl Section {
    fn trailer(&self) -> Option<&[PdfDictEntry]> {
        match self { Self::Table(cursor) => cursor.ready_trailer(), Self::Stream(cursor) => cursor.ready_trailer() }
    }

    fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), grant: CosGrant, target: XrefTarget) -> XrefRevisionReceipt {
        match self {
            Self::Table(cursor) => table(cursor.advance(|position| source.byte_at(position), grant, false), target),
            Self::Stream(cursor) => stream(cursor.advance(source, grant, false), target),
        }
    }

    fn begin_close(&mut self) {
        match self { Self::Table(cursor) => cursor.begin_close(), Self::Stream(cursor) => cursor.begin_close() }
    }

    fn close(&mut self, grant: CosGrant) -> XrefRevisionReceipt {
        match self {
            Self::Table(cursor) => table(cursor.close_step(grant), XrefTarget::Primary(0)),
            Self::Stream(cursor) => stream(cursor.close_step(grant), XrefTarget::Primary(0)),
        }
    }
}

fn response(event: XrefRevisionEvent) -> XrefRevisionReceipt {
    XrefRevisionReceipt { event: Ok(event), work: 0, allocated_bytes: 0, released_bytes: 0 }
}

fn table(result: ClassicSectionReceipt, target: XrefTarget) -> XrefRevisionReceipt {
    use XrefRevisionEvent as Event;
    let event = result.event.map_err(XrefStreamError::Structure).map(|event| match event {
        ClassicSectionEvent::Row { number, offset, generation, live } => Event::Row { target, row: XrefRow { number, entry: live.then_some(XrefEntry::Normal { offset, gen: generation }) } },
        ClassicSectionEvent::Progress(status) => match status {
            CosStatus::Complete { .. } => Event::Trailer(target),
            CosStatus::More => Event::More,
            CosStatus::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
            CosStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
            CosStatus::Cancelled => Event::Cancelled,
            CosStatus::Closed => Event::Closed,
        }
    });
    XrefRevisionReceipt { event, work: result.work, allocated_bytes: result.allocated_bytes, released_bytes: result.released_bytes }
}

fn stream(result: XrefStreamReceipt, target: XrefTarget) -> XrefRevisionReceipt {
    use XrefRevisionEvent as Event;
    let event = result.event.map(|event| match event {
        XrefStreamEvent::Row { number, entry } => Event::Row { target, row: XrefRow { number, entry } },
        XrefStreamEvent::Complete => Event::Trailer(target),
        XrefStreamEvent::More => Event::More,
        XrefStreamEvent::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        XrefStreamEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        XrefStreamEvent::Cancelled => Event::Cancelled,
        XrefStreamEvent::Closed => Event::Closed,
    });
    XrefRevisionReceipt { event, work: result.work, allocated_bytes: result.allocated_bytes, released_bytes: result.released_bytes }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drives_mixed_revisions_and_retires_a_partial_trailer() {
        let pdf = include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️incremental-free/🌀️flate.pdf");
        let grant = CosGrant { work: 1, allocation_bytes: 256 * 1024, release_bytes: 256 * 1024 };
        let mut footer = crate::FooterCursor::new(pdf.len());
        let offset = loop {
            if let crate::FooterStep::Complete(offset) = footer.advance(|position| pdf.get(position).copied(), 1) { break offset; }
        };
        let mut cursor = XrefRevisionCursor::new(offset, CosLimits::default(), 4096);
        let mut orders = Vec::new();
        let mut tombstone = false;
        let mut allocated = 0;
        let mut released = 0;
        loop {
            let receipt = cursor.advance(pdf, grant, false);
            assert!(receipt.work <= 1);
            allocated += receipt.allocated_bytes;
            released += receipt.released_bytes;
            match receipt.event.unwrap() {
                XrefRevisionEvent::Visit(target) => { assert!(cursor.admit_visit(target, true)); orders.push(target); }
                XrefRevisionEvent::Row { row, .. } => { tombstone |= row.number == 3 && row.entry.is_none(); }
                XrefRevisionEvent::Trailer(_) if orders.len() == 2 => break,
                XrefRevisionEvent::Trailer(_) => { assert!(cursor.take_trailer().is_some()); }
                XrefRevisionEvent::More => {}
                event => panic!("unexpected xref revision event {event:?}"),
            }
        }
        assert_eq!(orders, [XrefTarget::Primary(offset.unwrap()), XrefTarget::Primary(703)]);
        assert!(tombstone);
        assert!(matches!(cursor.advance(pdf, grant, true).event.unwrap(), XrefRevisionEvent::Cancelled));
        loop {
            let receipt = cursor.close_step(grant);
            released += receipt.released_bytes;
            if matches!(receipt.event.unwrap(), XrefRevisionEvent::Closed) { break; }
        }
        assert!(released <= allocated, "the first trailer's backing moved to the caller");
        assert!(cursor.take_trailer().is_none());
    }
}
