//! 🗃️ Publishes ordered native PDF objects and their exact generation-aware graph positions.

use super::{XrefEntry, XrefIndex, XrefIndexReceipt, XrefIndexStatus, XrefRow};
use semio_s_artifact_stdio_pdf_lexer::CosGrant;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject;
use std::mem::size_of;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphMaterializationEvent {
    More,
    Ready,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    RetireObject,
    Cancelled,
    Closed,
}

pub struct GraphMaterializationReceipt {
    pub event: Result<GraphMaterializationEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum Phase { Reserve, Ready, Insert, Commit, CloseObjects, CloseIndex, ReleaseVector, Closed }

/// 🪪️ Moves each original object only after its retained position is complete.
pub struct RetainedGraphBuilder {
    phase: Phase,
    maximum: usize,
    objects: Vec<PdfIndirectObject>,
    positions: XrefIndex,
    pending: Option<PdfIndirectObject>,
}

impl RetainedGraphBuilder {
    pub fn new(maximum_objects: usize) -> Self {
        Self { phase: Phase::Reserve, maximum: maximum_objects, objects: Vec::new(), positions: XrefIndex::default(), pending: None }
    }

    /// 📥️ Offers the original object. A refused move leaves it in the caller's ownership.
    pub fn offer(&mut self, object: PdfIndirectObject) -> Result<(), PdfIndirectObject> {
        if !matches!(self.phase, Phase::Ready) || self.objects.len() == self.maximum { return Err(object); }
        let row = XrefRow {
            number: object.id.num,
            entry: Some(XrefEntry::Normal { offset: self.objects.len(), gen: object.id.gen }),
        };
        if !self.positions.offer(row) { return Err(object); }
        self.pending = Some(object);
        self.phase = Phase::Insert;
        Ok(())
    }

    pub fn advance(&mut self, grant: CosGrant, cancelled: bool) -> GraphMaterializationReceipt {
        use GraphMaterializationEvent as Event;
        if cancelled { self.begin_close(); }
        match self.phase {
            Phase::Ready => report(Ok(Event::Ready), 0),
            Phase::Closed => report(Ok(Event::Closed), 0),
            Phase::CloseObjects | Phase::CloseIndex | Phase::ReleaseVector => report(Ok(Event::Cancelled), 0),
            _ if grant.work == 0 => report(Ok(Event::More), 0),
            Phase::Reserve => {
                let Some(bytes) = self.maximum.checked_mul(size_of::<PdfIndirectObject>()) else { return report(Err("PDF graph backing capacity overflow"), 1); };
                if bytes > grant.allocation_bytes { return report(Ok(Event::NeedsAllocation { bytes }), 0); }
                if self.objects.try_reserve_exact(self.maximum).is_err() { return report(Err("unable to allocate PDF retained graph"), 1); }
                self.phase = Phase::Ready;
                GraphMaterializationReceipt { allocated_bytes: self.objects.capacity() * size_of::<PdfIndirectObject>(), ..report(Ok(Event::Ready), 1) }
            }
            Phase::Insert => {
                let step = self.positions.advance(grant, false);
                if step.status == Ok(XrefIndexStatus::Inserted) { self.phase = Phase::Commit; }
                index_report(step)
            }
            Phase::Commit => {
                self.objects.push(self.pending.take().expect("indexed graph object remains in custody"));
                self.phase = Phase::Ready;
                report(Ok(Event::Ready), 1)
            }
        }
    }

    /// 📤️ Transfers the ordered graph and its original allocation only from a settled boundary.
    pub fn finish(&mut self) -> Option<(Vec<PdfIndirectObject>, XrefIndex)> {
        if !matches!(self.phase, Phase::Ready) { return None; }
        self.phase = Phase::Closed;
        Some((std::mem::take(&mut self.objects), std::mem::take(&mut self.positions)))
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::CloseObjects | Phase::CloseIndex | Phase::ReleaseVector | Phase::Closed) {
            self.phase = Phase::CloseObjects;
        }
    }

    /// 🧹️ Lends one exact object to the enclosing typed retirement before releasing its vector backing.
    pub fn take_retired_object(&mut self) -> Option<PdfIndirectObject> {
        if !matches!(self.phase, Phase::CloseObjects) { return None; }
        self.pending.take().or_else(|| self.objects.pop())
    }

    pub fn close_step(&mut self, grant: CosGrant) -> GraphMaterializationReceipt {
        use GraphMaterializationEvent as Event;
        match self.phase {
            Phase::Closed => report(Ok(Event::Closed), 0),
            _ if grant.work == 0 => report(Ok(Event::More), 0),
            Phase::CloseObjects => {
                if self.pending.is_some() || !self.objects.is_empty() { return report(Ok(Event::RetireObject), 0); }
                self.positions.begin_close();
                self.phase = Phase::CloseIndex;
                report(Ok(Event::More), 1)
            }
            Phase::CloseIndex => {
                let step = self.positions.close_step(grant);
                if step.status == Ok(XrefIndexStatus::Closed) { self.phase = Phase::ReleaseVector; }
                index_report(step)
            }
            Phase::ReleaseVector => {
                let bytes = self.objects.capacity() * size_of::<PdfIndirectObject>();
                if bytes > grant.release_bytes { return report(Ok(Event::NeedsRelease { bytes }), 0); }
                self.objects = Vec::new();
                self.phase = Phase::Closed;
                GraphMaterializationReceipt { released_bytes: bytes, ..report(Ok(Event::Closed), 1) }
            }
            _ => report(Ok(Event::More), 0),
        }
    }
}

fn report(event: Result<GraphMaterializationEvent, &'static str>, work: usize) -> GraphMaterializationReceipt {
    GraphMaterializationReceipt { event, work, allocated_bytes: 0, released_bytes: 0 }
}

fn index_report(turn: XrefIndexReceipt) -> GraphMaterializationReceipt {
    use GraphMaterializationEvent as Event;
    let event = match turn.status {
        Ok(XrefIndexStatus::AlreadyKnown) => Err("PDF graph has two live entries for one object number"),
        status => status.map(|status| match status {
            XrefIndexStatus::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
            XrefIndexStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
            _ => Event::More,
        }),
    };
    GraphMaterializationReceipt { event, work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
