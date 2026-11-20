//! 🌊 Reads one PDF xref stream section in admitted turns.

use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::{decode_stream, PdfDecodedStreamCursor, StreamDataGrant, StreamDataStatus};
use semio_s_artifact_stdio_pdf_lexer::{CosGrant, CosLimits, IndirectFrameCursor, IndirectFrameStatus, StreamSpan, StreamSpanCursor};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfObject};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error::{malformed, PResult};
use crate::{XrefEntry, XrefStreamRowCursor};

/// 🎫 The result of driving one xref stream section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefStreamEvent {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Row { number: u32, entry: Option<XrefEntry> },
    Complete,
    Cancelled,
    Closed,
}

/// ⚠️ Distinguishes decoder refusals from xref structure failures.
#[derive(Debug)]
pub enum XrefStreamError {
    Structure(&'static str),
    Decode(ValueError),
}

/// 🧾 Observed work and physical buffer effects.
#[derive(Debug)]
#[must_use]
pub struct XrefStreamReceipt {
    pub event: Result<XrefStreamEvent, XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum Phase {
    Frame,
    Fields,
    Span(StreamSpanCursor),
    Decode(StreamSpan),
    Rows,
    ReleaseBody,
    Complete,
    Failed,
    Cancelled,
    Closing,
    Closed,
}

/// 🧭 Resumes framing, stream decoding and row parsing without copying the encoded span.
#[must_use]
pub struct XrefStreamSectionCursor {
    phase: Phase,
    frame: IndirectFrameCursor,
    decoder: PdfDecodedStreamCursor,
    decoded: Vec<u8>,
    dict_position: usize,
    fields: [Option<usize>; 4],
    ranges_position: usize,
    object: u32,
    remaining: u32,
    body_position: usize,
    reader: Option<XrefStreamRowCursor>,
    decoder_closed: bool,
    recoverable_span: Option<StreamSpan>,
}

impl XrefStreamSectionCursor {
    /// 🌱 Opens at the original startxref offset with explicit COS and decoded output ceilings.
    pub fn new(offset: usize, limits: CosLimits, maximum_inflated: usize, maximum_output: usize) -> Self {
        Self {
            phase: Phase::Frame, frame: IndirectFrameCursor::new(offset, limits),
            decoder: PdfDecodedStreamCursor::new(maximum_inflated, maximum_output), decoded: Vec::new(),
            dict_position: 0, fields: [None; 4], ranges_position: 0, object: 0, remaining: 0,
            body_position: 0, reader: None, decoder_closed: false, recoverable_span: None,
        }
    }

    /// ▶️ Advances at most one child stage and publishes at most one xref row.
    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> XrefStreamReceipt {
        if cancelled && matches!(self.phase, Phase::Frame | Phase::Fields | Phase::Span(_) | Phase::Decode(_) | Phase::Rows | Phase::ReleaseBody) {
            self.phase = Phase::Cancelled;
        }
        let mut receipt = empty();
        receipt.event = match self.phase {
            Phase::Complete => Ok(XrefStreamEvent::Complete),
            Phase::Cancelled => Ok(XrefStreamEvent::Cancelled),
            Phase::Closed => Ok(XrefStreamEvent::Closed),
            Phase::Closing => Ok(XrefStreamEvent::More),
            Phase::Failed => Err(XrefStreamError::Structure("xref stream section already failed")),
            _ if grant.work == 0 => Ok(XrefStreamEvent::More),
            _ => self.advance_stage(source, grant, &mut receipt),
        };
        if receipt.event.is_err() { self.phase = Phase::Failed; }
        receipt
    }

    fn advance_stage(&mut self, source: &[u8], grant: CosGrant, receipt: &mut XrefStreamReceipt) -> Result<XrefStreamEvent, XrefStreamError> {
        match &mut self.phase {
            Phase::Frame => {
                let result = self.frame.advance(source, grant, false);
                receipt.work = result.work;
                receipt.allocated_bytes = result.allocated_bytes;
                receipt.released_bytes = result.released_bytes;
                let status = result.status.map_err(XrefStreamError::Structure)?;
                if status == IndirectFrameStatus::Complete {
                    self.phase = Phase::Fields;
                    return Ok(XrefStreamEvent::More);
                }
                Ok(frame_status(status))
            }
            Phase::Fields => self.plan_fields(source.len(), grant.work, receipt).map_err(XrefStreamError::Structure),
            Phase::Span(cursor) => {
                let result = cursor.advance(source, grant.work);
                receipt.work = result.inspected;
                if let Some(span) = result.span { self.phase = Phase::Decode(span); }
                Ok(XrefStreamEvent::More)
            }
            Phase::Decode(span) => {
                let span = *span;
                let dict = dictionary(&self.frame).map_err(XrefStreamError::Structure)?;
                let raw = source.get(span.start..span.end).ok_or(XrefStreamError::Structure("xref stream source changed"))?;
                let result = self.decoder.advance(dict, raw, stream_grant(grant), false);
                receipt.work = result.work;
                receipt.allocated_bytes = result.allocated_bytes;
                receipt.released_bytes = result.released_bytes;
                let status = result.status.map_err(|error| {
                    if matches!(error.kind, ValueRefusalKind::InvalidValue | ValueRefusalKind::UnsupportedOwner) {
                        self.recoverable_span = Some(span);
                    }
                    XrefStreamError::Decode(error)
                })?;
                if status == StreamDataStatus::Complete {
                    let bytes = self.decoder.take_bytes().ok_or(XrefStreamError::Structure("xref stream body is unavailable"))?;
                    self.decoded = bytes;
                    self.phase = Phase::Rows;
                    return Ok(XrefStreamEvent::More);
                }
                Ok(stream_status(status))
            }
            Phase::Rows => self.next_row(grant.work, receipt).map_err(XrefStreamError::Structure),
            Phase::ReleaseBody => match self.retire_body(grant, receipt)? {
                XrefStreamEvent::Closed => {
                    self.phase = Phase::Complete;
                    Ok(XrefStreamEvent::Complete)
                }
                progress => Ok(progress),
            },
            _ => unreachable!("xref stage dispatch requires an active phase"),
        }
    }

    fn plan_fields(&mut self, source_len: usize, work: usize, receipt: &mut XrefStreamReceipt) -> Result<XrefStreamEvent, &'static str> {
        let dict = dictionary(&self.frame)?;
        let start = self.frame.stream_start().ok_or("xref stream object is not a stream")?;
        while receipt.work < work {
            receipt.work += 1;
            let Some(entry) = dict.get(self.dict_position) else {
                let value = |index: usize| self.fields[index].map(|field| &dict[field].value);
                let Some(PdfObject::Array(widths)) = value(1) else { return Err("xref stream missing /W"); };
                let [a, b, c] = widths.as_slice() else { return Err("invalid xref stream /W field count"); };
                let width = |value: &PdfObject| value.as_i64().and_then(|n| usize::try_from(n).ok()).ok_or("invalid xref stream /W field");
                self.reader = Some(XrefStreamRowCursor::new([width(a)?, width(b)?, width(c)?])?);
                match value(2) {
                    Some(PdfObject::Array(ranges)) if ranges.len() % 2 == 0 => {}
                    None => {
                        self.remaining = unsigned(value(3).ok_or("xref stream missing /Size")?).ok_or("invalid xref stream /Size")?;
                    }
                    _ => return Err("invalid xref stream /Index"),
                }
                let length = value(0).and_then(PdfObject::as_i64).and_then(|length| usize::try_from(length).ok());
                self.phase = Phase::Span(StreamSpanCursor::new(start, length, source_len));
                return Ok(XrefStreamEvent::More);
            };
            if let Some(slot) = ["Length", "W", "Index", "Size"].iter().position(|&key| key == entry.key) {
                self.fields[slot].get_or_insert(self.dict_position);
            }
            self.dict_position += 1;
        }
        Ok(XrefStreamEvent::More)
    }

    fn next_row(&mut self, work: usize, receipt: &mut XrefStreamReceipt) -> Result<XrefStreamEvent, &'static str> {
        while receipt.work < work {
            receipt.work += 1;
            if self.remaining == 0 {
                if let Some(field) = self.fields[2] {
                    let PdfObject::Array(ranges) = &dictionary(&self.frame)?[field].value else { return Err("invalid xref stream /Index"); };
                    if let Some(pair) = ranges.get(self.ranges_position..self.ranges_position.saturating_add(2)) {
                        self.object = unsigned(&pair[0]).ok_or("invalid first xref stream object")?;
                        self.remaining = unsigned(&pair[1]).ok_or("invalid xref stream subsection count")?;
                        self.ranges_position += 2;
                        continue;
                    }
                }
                self.phase = Phase::ReleaseBody;
                return Ok(XrefStreamEvent::More);
            }
            let byte = self.decoded.get(self.body_position).ok_or("truncated xref stream row")?;
            let (used, row) = self.reader.as_mut().expect("xref row reader").advance(std::slice::from_ref(byte))?;
            self.body_position += used;
            if let Some([kind, first, second]) = row {
                let entry = match kind {
                    1 => Some(XrefEntry::Normal { offset: usize::try_from(first).map_err(|_| "invalid xref byte offset")?, gen: u16::try_from(second).map_err(|_| "invalid xref generation")? }),
                    2 => Some(XrefEntry::Compressed { stream_num: u32::try_from(first).map_err(|_| "invalid object stream number")?, index: u32::try_from(second).map_err(|_| "invalid object stream index")? }),
                    _ => None,
                };
                let number = self.object;
                self.remaining -= 1;
                if self.remaining != 0 { self.object = self.object.checked_add(1).ok_or("xref stream object number overflow")?; }
                return Ok(XrefStreamEvent::Row { number, entry });
            }
        }
        Ok(XrefStreamEvent::More)
    }

    /// 🛟 Recovers an earlier decoder format refusal through the synchronous PDF compatibility path.
    pub fn recover_compatible(&mut self, source: &[u8]) -> PResult<bool> {
        if !matches!(self.phase, Phase::Failed) { return Ok(false); }
        let Some(span) = self.recoverable_span.take() else { return Ok(false); };
        self.decoder.begin_close();
        let grant = StreamDataGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
        loop {
            match self.decoder.close_step(grant).status {
                Ok(StreamDataStatus::Closed) => break,
                Ok(StreamDataStatus::More) => {}
                _ => return malformed("could not retire rejected xref decoder"),
            }
        }
        self.decoder_closed = true;
        let Ok(dict) = dictionary(&self.frame) else { return malformed("xref stream object is not a dictionary"); };
        let Some(raw) = source.get(span.start..span.end) else { return malformed("xref stream source changed"); };
        self.decoded = decode_stream(dict, raw)?.0;
        self.phase = Phase::Rows;
        Ok(true)
    }

    pub fn ready_trailer(&self) -> Option<&[PdfDictEntry]> {
        if !matches!(self.phase, Phase::Complete) { return None; }
        self.frame.ready_value()?.as_dict()
    }

    /// 📤 Transfers the exact typed trailer after all rows have been emitted.
    pub fn take_trailer(&mut self) -> Option<Vec<PdfDictEntry>> {
        if !matches!(self.phase, Phase::Complete) { return None; }
        let PdfObject::Dict(dict) = self.frame.take_frame()?.value else { return None; };
        Some(dict)
    }

    /// ♻️ Cancels further row publication and begins bounded backing retirement.
    pub fn begin_close(&mut self) {
        if matches!(self.phase, Phase::Closing | Phase::Closed) { return; }
        self.phase = Phase::Closing;
        self.decoder.begin_close();
        self.frame.begin_close();
    }

    /// 🧹 Releases the decoded body, decoder history and retained trailer separately.
    pub fn close_step(&mut self, grant: CosGrant) -> XrefStreamReceipt {
        let mut receipt = empty();
        if matches!(self.phase, Phase::Closed) { receipt.event = Ok(XrefStreamEvent::Closed); return receipt; }
        if !matches!(self.phase, Phase::Closing) || grant.work == 0 { return receipt; }
        match self.retire_body(grant, &mut receipt) {
            Ok(XrefStreamEvent::Closed) => {}
            progress => { receipt.event = progress; return receipt; }
        }
        let result = self.frame.close_step(grant);
        receipt.work = result.work;
        receipt.allocated_bytes = result.allocated_bytes;
        receipt.released_bytes = result.released_bytes;
        receipt.event = result.status.map_err(XrefStreamError::Structure).map(|status| {
            if status == IndirectFrameStatus::Closed { self.phase = Phase::Closed; }
            frame_status(status)
        });
        receipt
    }
    fn retire_body(&mut self, grant: CosGrant, receipt: &mut XrefStreamReceipt) -> Result<XrefStreamEvent, XrefStreamError> {
        let bytes = self.decoded.capacity();
        if bytes > 0 {
            if bytes > grant.release_bytes { return Ok(XrefStreamEvent::NeedsRelease { bytes }); }
            self.decoded = Vec::new();
            receipt.work = 1;
            receipt.released_bytes = bytes;
            return Ok(XrefStreamEvent::More);
        }
        if self.decoder_closed { return Ok(XrefStreamEvent::Closed); }
        let result = self.decoder.close_step(stream_grant(grant));
        receipt.work = result.work;
        receipt.allocated_bytes = result.allocated_bytes;
        receipt.released_bytes = result.released_bytes;
        result.status.map_err(XrefStreamError::Decode).map(|status| {
            if status == StreamDataStatus::Closed {
                self.decoder_closed = true;
                XrefStreamEvent::More
            } else { stream_status(status) }
        })
    }

}

fn dictionary(frame: &IndirectFrameCursor) -> Result<&[PdfDictEntry], &'static str> {
    match frame.ready_value() {
        Some(PdfObject::Dict(dict)) => Ok(dict),
        _ => Err("xref stream object is not a dictionary"),
    }
}

fn unsigned(value: &PdfObject) -> Option<u32> { value.as_i64().and_then(|value| u32::try_from(value).ok()) }

fn empty() -> XrefStreamReceipt {
    XrefStreamReceipt { event: Ok(XrefStreamEvent::More), work: 0, allocated_bytes: 0, released_bytes: 0 }
}

fn stream_grant(grant: CosGrant) -> StreamDataGrant {
    StreamDataGrant { work: grant.work, allocation_bytes: grant.allocation_bytes, release_bytes: grant.release_bytes }
}

fn frame_status(status: IndirectFrameStatus) -> XrefStreamEvent {
    match status {
        IndirectFrameStatus::More => XrefStreamEvent::More,
        IndirectFrameStatus::NeedsAllocation { bytes } => XrefStreamEvent::NeedsAllocation { bytes },
        IndirectFrameStatus::NeedsRelease { bytes } => XrefStreamEvent::NeedsRelease { bytes },
        IndirectFrameStatus::Complete => XrefStreamEvent::Complete,
        IndirectFrameStatus::Cancelled => XrefStreamEvent::Cancelled,
        IndirectFrameStatus::Closed => XrefStreamEvent::Closed,
    }
}

fn stream_status(status: StreamDataStatus) -> XrefStreamEvent {
    match status {
        StreamDataStatus::More => XrefStreamEvent::More,
        StreamDataStatus::NeedsAllocation { bytes } => XrefStreamEvent::NeedsAllocation { bytes },
        StreamDataStatus::NeedsRelease { bytes } => XrefStreamEvent::NeedsRelease { bytes },
        StreamDataStatus::Complete => XrefStreamEvent::Complete,
        StreamDataStatus::Cancelled => XrefStreamEvent::Cancelled,
        StreamDataStatus::Closed => XrefStreamEvent::Closed,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
