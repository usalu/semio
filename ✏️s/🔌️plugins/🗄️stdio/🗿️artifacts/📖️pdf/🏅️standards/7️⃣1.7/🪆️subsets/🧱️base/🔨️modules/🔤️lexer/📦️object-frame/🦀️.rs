//! 📦️ Frames an indirect object in the original PDF, leaving any stream bytes in their source.

use super::{
    CosGrant, CosLimits, CosReceipt, CosStatus, HeaderStep, IndirectHeaderCursor, PdfCosCursor,
    StreamPrefixCursor, StreamPrefixStep,
};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndirectFrameStatus {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete,
    Cancelled,
    Closed,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IndirectFrameReceipt {
    pub status: Result<IndirectFrameStatus, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

pub struct IndirectFrame {
    pub id: ObjRef,
    pub value: PdfObject,
    pub value_end: usize,
    pub stream_start: Option<usize>,
}

enum Stage {
    Header(IndirectHeaderCursor),
    Value,
    StreamPrefix(StreamPrefixCursor),
    Complete,
    Failed(&'static str),
    Cancelled,
    Closing,
    Closed,
}

pub struct IndirectFrameCursor {
    stage: Stage,
    limits: CosLimits,
    id: Option<ObjRef>,
    value: Option<PdfCosCursor>,
    value_end: usize,
    stream_start: Option<usize>,
}

impl IndirectFrameCursor {
    pub fn new(offset: usize, limits: CosLimits) -> Self {
        Self {
            stage: Stage::Header(IndirectHeaderCursor::new(offset)), limits, id: None,
            value: None, value_end: 0, stream_start: None,
        }
    }

    /// 📥️ Inspects the original header and value, then identifies where a declared stream begins.
    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> IndirectFrameReceipt {
        let mut receipt = IndirectFrameReceipt { status: Ok(IndirectFrameStatus::More), work: 0, allocated_bytes: 0, released_bytes: 0 };
        if cancelled && matches!(self.stage, Stage::Header(_) | Stage::Value | Stage::StreamPrefix(_)) {
            self.stage = Stage::Cancelled;
        }
        match &mut self.stage {
            Stage::Header(header) => {
                let start = header.position();
                match header.advance(source, grant.work) {
                    Ok((work, step)) => {
                        receipt.work = work;
                        if let HeaderStep::Complete { id, body_offset } = step {
                            self.id = Some(id);
                            self.value = Some(PdfCosCursor::new(body_offset, self.limits));
                            self.stage = Stage::Value;
                        }
                    }
                    Err(reason) => {
                        receipt.work = (header.position() - start).saturating_add(1).min(grant.work);
                        self.stage = Stage::Failed(reason);
                        receipt.status = Err(reason);
                    }
                }
            }
            Stage::Value => {
                let cursor = self.value.as_mut().expect("framed value owner");
                let CosReceipt { status, work, allocated_bytes, released_bytes } = cursor.advance(source, grant, false);
                receipt.work = work;
                receipt.allocated_bytes = allocated_bytes;
                receipt.released_bytes = released_bytes;
                receipt.status = match status {
                    Ok(CosStatus::Complete { end }) => {
                        self.value_end = end;
                        if cursor.ready_value().is_some_and(|value| matches!(value, PdfObject::Dict(_))) {
                            self.stage = Stage::StreamPrefix(StreamPrefixCursor::new(end));
                            Ok(IndirectFrameStatus::More)
                        } else {
                            self.stage = Stage::Complete;
                            Ok(IndirectFrameStatus::Complete)
                        }
                    }
                    Ok(CosStatus::More) => Ok(IndirectFrameStatus::More),
                    Ok(CosStatus::NeedsAllocation { bytes }) => Ok(IndirectFrameStatus::NeedsAllocation { bytes }),
                    Ok(CosStatus::NeedsRelease { bytes }) => Ok(IndirectFrameStatus::NeedsRelease { bytes }),
                    Ok(CosStatus::Cancelled | CosStatus::Closed) => Err("indirect object value owner stopped unexpectedly"),
                    Err(reason) => { self.stage = Stage::Failed(reason); Err(reason) }
                };
            }
            Stage::StreamPrefix(prefix) => {
                let (work, step) = prefix.advance(source, grant.work);
                receipt.work = work;
                if let StreamPrefixStep::Complete(start) = step {
                    self.stream_start = start;
                    self.stage = Stage::Complete;
                    receipt.status = Ok(IndirectFrameStatus::Complete);
                }
            }
            Stage::Complete => receipt.status = Ok(IndirectFrameStatus::Complete),
            Stage::Failed(reason) => receipt.status = Err(reason),
            Stage::Cancelled => receipt.status = Ok(IndirectFrameStatus::Cancelled),
            Stage::Closing => {}
            Stage::Closed => receipt.status = Ok(IndirectFrameStatus::Closed),
        }
        receipt
    }

    /// 📤️ Transfers the completed value and original source offsets without copying stream data.
    pub fn take_frame(&mut self) -> Option<IndirectFrame> {
        if !matches!(self.stage, Stage::Complete) { return None; }
        Some(IndirectFrame {
            id: self.id?, value: self.value.as_mut()?.take_value()?,
            value_end: self.value_end, stream_start: self.stream_start,
        })
    }

    /// 🔎️ Borrows the completed COS value while the frame retains retirement custody.
    pub fn ready_value(&self) -> Option<&PdfObject> {
        if !matches!(self.stage, Stage::Complete) { return None; }
        self.value.as_ref()?.ready_value()
    }

    /// 📍 Returns the source offset of a completed frame's stream body.
    pub fn stream_start(&self) -> Option<usize> {
        if !matches!(self.stage, Stage::Complete) { return None; }
        self.stream_start
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.stage, Stage::Closed) { self.stage = Stage::Closing; }
        if let Some(value) = self.value.as_mut() { value.begin_close(); }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> IndirectFrameReceipt {
        let mut result = IndirectFrameReceipt { status: Ok(IndirectFrameStatus::More), work: 0, allocated_bytes: 0, released_bytes: 0 };
        if matches!(self.stage, Stage::Closing) {
            if let Some(value) = self.value.as_mut() {
                let receipt = value.close_step(grant);
                result.work = receipt.work;
                result.released_bytes = receipt.released_bytes;
                match receipt.status {
                    Ok(CosStatus::Closed) => self.value = None,
                    Ok(CosStatus::NeedsRelease { bytes }) => result.status = Ok(IndirectFrameStatus::NeedsRelease { bytes }),
                    Ok(_) => {}
                    Err(reason) => result.status = Err(reason),
                }
            }
            if self.value.is_none() { self.stage = Stage::Closed; }
        }
        if matches!(self.stage, Stage::Closed) { result.status = Ok(IndirectFrameStatus::Closed); }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_real_stream_without_copying_payload() {
        let pdf = include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️indirect-length.pdf");
        let start = pdf.windows(7).position(|window| window == b"4 0 obj").unwrap();
        let mut cursor = IndirectFrameCursor::new(start, CosLimits::default());
        let grant = CosGrant { work: 1, allocation_bytes: 8192, release_bytes: 8192 };
        loop {
            let turn = cursor.advance(pdf, grant, false);
            assert!(turn.work <= 1);
            if turn.status.unwrap() == IndirectFrameStatus::Complete { break; }
        }
        let frame = cursor.take_frame().unwrap();
        assert_eq!(frame.id, ObjRef { num: 4, gen: 0 });
        assert_eq!(frame.value.dict_get("Length").and_then(PdfObject::as_ref), Some(ObjRef { num: 5, gen: 0 }));
        assert!(pdf[frame.value_end..].starts_with(b"\nstream"));
        assert!(pdf[frame.stream_start.unwrap()..].starts_with(b"BT /F1"));
        assert!(cursor.take_frame().is_none());
        cursor.begin_close();
        while cursor.close_step(grant).status.unwrap() != IndirectFrameStatus::Closed {}
    }

    #[test]
    fn cancellation_in_stream_prefix_retires_the_dict() {
        let source = b"1 0 obj << /Length 14 >> % notice\nstream\n1234";
        let mut cursor = IndirectFrameCursor::new(0, CosLimits::default());
        let grant = CosGrant { work: 1, allocation_bytes: 8192, release_bytes: 8192 };
        let mut allocated = 0;
        let mut released = 0;
        while !matches!(cursor.stage, Stage::StreamPrefix(_)) {
            let turn = cursor.advance(source, grant, false);
            assert!(turn.status.is_ok());
            allocated += turn.allocated_bytes;
            released += turn.released_bytes;
        }
        assert_eq!(cursor.advance(source, grant, true).status, Ok(IndirectFrameStatus::Cancelled));
        assert!(cursor.take_frame().is_none());
        cursor.begin_close();
        loop {
            let turn = cursor.close_step(grant);
            released += turn.released_bytes;
            if turn.status.unwrap() == IndirectFrameStatus::Closed { break; }
        }
        assert_eq!(allocated, released);
    }
}
