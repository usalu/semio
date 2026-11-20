//! 🧭️ Frames an indirect object and negotiates its stream length without allocating a raw body.

use crate::{CosGrant, CosLimits, IndirectFrameCursor, IndirectFrameStatus, StreamSpan, StreamSpanCursor};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};

/// 🎫 The next normal-object action or terminal state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NormalObjectStatus {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    NeedsLength(ObjRef),
    AwaitingLength,
    Complete,
    Cancelled,
    Closed,
}

/// 🧾 The work and backing effects of one normal-object turn.
#[derive(Debug, PartialEq, Eq)]
#[must_use]
pub struct NormalObjectReceipt {
    pub status: Result<NormalObjectStatus, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

/// 📦 A typed value and its original, unowned stream body if present.
pub struct NormalObject {
    pub id: ObjRef,
    pub value: PdfObject,
    pub value_end: usize,
    pub stream: Option<StreamSpan>,
}

enum Phase {
    Frame,
    Length { start: usize, position: usize },
    Wait { reference: ObjRef, start: usize, source_len: usize },
    Span(StreamSpanCursor),
    Complete(Option<StreamSpan>),
    Failed(&'static str),
    Cancelled,
    Closing,
    Closed,
}

/// 🧮 Resolves one normal indirect object against a stable source slice.
#[must_use]
pub struct PdfNormalObjectCursor {
    frame: IndirectFrameCursor,
    phase: Phase,
}

impl PdfNormalObjectCursor {
    /// 🌱 Opens a normal object at a known cross-reference offset.
    pub fn new(offset: usize, limits: CosLimits) -> Self {
        Self { frame: IndirectFrameCursor::new(offset, limits), phase: Phase::Frame }
    }

    /// ▶️ Drives a single stage with the caller's independent work and capacity grants.
    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> NormalObjectReceipt {
        if cancelled && matches!(self.phase, Phase::Frame | Phase::Length { .. } | Phase::Wait { .. } | Phase::Span(_)) {
            self.phase = Phase::Cancelled;
        }
        let mut receipt = empty();
        receipt.status = match &mut self.phase {
            Phase::Complete(_) => Ok(NormalObjectStatus::Complete),
            Phase::Cancelled => Ok(NormalObjectStatus::Cancelled),
            Phase::Closed => Ok(NormalObjectStatus::Closed),
            Phase::Closing => Ok(NormalObjectStatus::More),
            Phase::Failed(error) => Err(error),
            Phase::Wait { .. } => Ok(NormalObjectStatus::AwaitingLength),
            _ if grant.work == 0 => Ok(NormalObjectStatus::More),
            Phase::Frame => {
                let framed = self.frame.advance(source, grant, false);
                receipt.work = framed.work;
                receipt.allocated_bytes = framed.allocated_bytes;
                receipt.released_bytes = framed.released_bytes;
                match framed.status {
                    Ok(IndirectFrameStatus::Complete) => match self.frame.stream_start() {
                        Some(start) => { self.phase = Phase::Length { start, position: 0 }; Ok(NormalObjectStatus::More) }
                        None => { self.phase = Phase::Complete(None); Ok(NormalObjectStatus::Complete) }
                    },
                    status => status.map(frame_status),
                }
            }
            Phase::Length { start, position } => {
                let (start, position) = (*start, *position);
                self.plan_length(start, position, source.len(), grant.work, &mut receipt)
            }
            Phase::Span(cursor) => {
                let result = cursor.advance(source, grant.work);
                receipt.work = result.inspected;
                if let Some(span) = result.span {
                    self.phase = Phase::Complete(Some(span));
                    Ok(NormalObjectStatus::Complete)
                } else {
                    Ok(NormalObjectStatus::More)
                }
            }
        };
        if let Err(error) = receipt.status { self.phase = Phase::Failed(error); }
        receipt
    }

    fn plan_length(&mut self, start: usize, mut position: usize, source_len: usize, work: usize, receipt: &mut NormalObjectReceipt) -> Result<NormalObjectStatus, &'static str> {
        let Some(PdfObject::Dict(dict)) = self.frame.ready_value() else { return Err("normal stream lacks its dictionary"); };
        while receipt.work < work {
            receipt.work += 1;
            let Some(entry) = dict.get(position) else {
                self.phase = Phase::Span(StreamSpanCursor::new(start, None, source_len));
                return Ok(NormalObjectStatus::More);
            };
            if entry.key == "Length" {
                if let PdfObject::Ref(reference) = entry.value {
                    self.phase = Phase::Wait { reference, start, source_len };
                    return Ok(NormalObjectStatus::NeedsLength(reference));
                }
                let length = entry.value.as_i64().and_then(|number| usize::try_from(number).ok());
                self.phase = Phase::Span(StreamSpanCursor::new(start, length, source_len));
                return Ok(NormalObjectStatus::More);
            }
            position += 1;
        }
        self.phase = Phase::Length { start, position };
        Ok(NormalObjectStatus::More)
    }

    /// 📮 Delivers the requested reference length or selects the bounded marker search.
    pub fn provide_length(&mut self, reference: ObjRef, length: Option<usize>) -> Result<(), &'static str> {
        let Phase::Wait { reference: expected, start, source_len } = self.phase else { return Err("normal object has no outstanding length request"); };
        if expected != reference { return Err("normal object length reference does not match its request"); }
        self.phase = Phase::Span(StreamSpanCursor::new(start, length, source_len));
        Ok(())
    }

    /// 📤 Transfers the completed value and original body span, retaining no payload copy.
    pub fn take_object(&mut self) -> Option<NormalObject> {
        let Phase::Complete(stream) = self.phase else { return None; };
        let frame = self.frame.take_frame()?;
        Some(NormalObject { id: frame.id, value: frame.value, value_end: frame.value_end, stream })
    }

    /// ♻️ Begins closing partial frame custody from any phase.
    pub fn begin_close(&mut self) {
        if matches!(self.phase, Phase::Closing | Phase::Closed) { return; }
        self.phase = Phase::Closing;
        self.frame.begin_close();
    }

    /// 🧹 Retires native ownership under the caller's work and release grants.
    pub fn close_step(&mut self, grant: CosGrant) -> NormalObjectReceipt {
        let mut receipt = empty();
        if matches!(self.phase, Phase::Closed) { receipt.status = Ok(NormalObjectStatus::Closed); return receipt; }
        if !matches!(self.phase, Phase::Closing) { return receipt; }
        let result = self.frame.close_step(grant);
        if result.status == Ok(IndirectFrameStatus::Closed) { self.phase = Phase::Closed; }
        NormalObjectReceipt {
            status: result.status.map(frame_status), work: result.work,
            allocated_bytes: result.allocated_bytes, released_bytes: result.released_bytes,
        }
    }
}

fn empty() -> NormalObjectReceipt {
    NormalObjectReceipt { status: Ok(NormalObjectStatus::More), work: 0, allocated_bytes: 0, released_bytes: 0 }
}

fn frame_status(status: IndirectFrameStatus) -> NormalObjectStatus {
    match status {
        IndirectFrameStatus::More => NormalObjectStatus::More,
        IndirectFrameStatus::NeedsAllocation { bytes } => NormalObjectStatus::NeedsAllocation { bytes },
        IndirectFrameStatus::NeedsRelease { bytes } => NormalObjectStatus::NeedsRelease { bytes },
        IndirectFrameStatus::Complete => NormalObjectStatus::Complete,
        IndirectFrameStatus::Cancelled => NormalObjectStatus::Cancelled,
        IndirectFrameStatus::Closed => NormalObjectStatus::Closed,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
