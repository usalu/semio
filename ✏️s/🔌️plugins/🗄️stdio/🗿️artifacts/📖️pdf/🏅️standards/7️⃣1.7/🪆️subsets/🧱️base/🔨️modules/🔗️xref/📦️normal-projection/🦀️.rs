//! 📦️ Projects a normal indirect object from stable authored bytes into retained data.

use super::XrefStreamError;
use semio_framework_value::{ByteReadRange, ByteReadSource, ValueRefusalKind};
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::{DecodedStreamData, PdfDecodedStreamCursor, StreamDataGrant, StreamDataReceipt, StreamDataStatus};
use semio_s_artifact_stdio_pdf_lexer::{CosGrant, CosLimits, CosReceipt, CosStatus, NormalObject, NormalObjectStatus, PdfCosCursor, PdfNormalObjectCursor, PdfStreamDictionaryCursor, StreamSpan};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfObject, PdfStreamFilter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NormalProjectionEvent {
    More,
    NeedsLength(ObjRef),
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete,
    Cancelled,
    Closed,
}

pub enum NormalProjection {
    Value(PdfObject),
    ObjectStream(PdfObject),
    ReferenceStream { dict: Vec<PdfDictEntry>, span: StreamSpan },
    Omitted,
}

pub struct NormalProjectionReceipt {
    pub event: Result<NormalProjectionEvent, XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum Shape { Classify, Decode, CloseDecoder { decoded: bool }, Sanitize, Ready, Reference, Omitted }
enum Phase { Frame, CloseFrame, Shape(Shape), Cancelled, Closing, Closed }

/// 🎯️ Retains exact generations, indirect length requests and decoded stream custody between turns.
pub struct NormalProjectionCursor {
    id: ObjRef,
    frame: PdfNormalObjectCursor,
    frame_closed: bool,
    current: Option<NormalObject>,
    decoder: Option<PdfDecodedStreamCursor>,
    data: Option<DecodedStreamData>,
    dictionary: Option<PdfStreamDictionaryCursor>,
    retiring: Option<PdfCosCursor>,
    phase: Phase,
    type_position: usize,
    keep_object_streams: bool,
    object_stream: bool,
    compressed_length: bool,
    maximum_stream_bytes: usize,
}

impl NormalProjectionCursor {
    pub fn new(id: ObjRef, offset: usize, limits: CosLimits, maximum_stream_bytes: usize) -> Self {
        Self {
            id, frame: PdfNormalObjectCursor::new(offset, limits), frame_closed: false, current: None,
            decoder: None, data: None, dictionary: None, retiring: None, phase: Phase::Frame,
            type_position: 0, keep_object_streams: false, object_stream: false, compressed_length: false, maximum_stream_bytes,
        }
    }

    pub fn keep_object_streams(mut self) -> Self { self.keep_object_streams = true; self }

    pub fn provide_length(&mut self, reference: ObjRef, length: Option<usize>) -> Result<(), &'static str> {
        self.frame.provide_length(reference, length)
    }

    /// 🔒️ Records compressed length custody so object streams cannot depend on a compressed length.
    pub fn provide_compressed_length(&mut self, reference: ObjRef, length: Option<usize>) -> Result<(), &'static str> {
        self.provide_length(reference, length)?;
        self.compressed_length = true;
        Ok(())
    }

    /// 🛂️ Quotes an empty history allocation before decoding normal stream data.
    pub fn next_reserves_history(&self) -> bool {
        matches!(self.phase, Phase::Shape(Shape::Decode)) && self.decoder.as_ref().is_some_and(PdfDecodedStreamCursor::next_reserves_history)
    }

    /// ▶️ Drives one object, one stream and its dictionary under the original stable source and grants.
    pub fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), grant: CosGrant, cancelled: bool) -> NormalProjectionReceipt {
        use NormalProjectionEvent as Event;
        if cancelled && !matches!(self.phase, Phase::Closing | Phase::Closed) { self.phase = Phase::Cancelled; }
        match self.phase {
            Phase::Cancelled => receipt(Ok(Event::Cancelled), 0),
            Phase::Closed => receipt(Ok(Event::Closed), 0),
            Phase::Closing => receipt(Ok(Event::More), 0),
            Phase::Shape(Shape::Ready | Shape::Reference | Shape::Omitted) => receipt(Ok(Event::Complete), 0),
            _ if grant.work == 0 => receipt(Ok(Event::More), 0),
            Phase::Frame => self.advance_frame(source, grant),
            Phase::CloseFrame => {
                let step = self.close_frame(grant);
                if self.frame_closed {
                    self.phase = Phase::Shape(match self.current.as_ref() {
                        Some(object) if object.id == self.id && object.stream.is_some() => Shape::Classify,
                        Some(object) if object.id == self.id => Shape::Ready,
                        _ => Shape::Omitted,
                    });
                }
                step
            }
            Phase::Shape(Shape::Classify) => self.classify(),
            Phase::Shape(Shape::Decode) => self.decode(source, grant),
            Phase::Shape(Shape::CloseDecoder { decoded }) => {
                let step = self.close_decoder(grant);
                if self.decoder.is_none() {
                    self.phase = Phase::Shape(if decoded { Shape::Sanitize } else if self.object_stream { Shape::Omitted } else { Shape::Reference });
                }
                step
            }
            Phase::Shape(Shape::Sanitize) => self.sanitize(grant),
        }
    }

    fn advance_frame(&mut self, source: &(impl ByteReadSource + ?Sized), grant: CosGrant) -> NormalProjectionReceipt {
        use NormalObjectStatus as Status;
        use NormalProjectionEvent as Event;
        let step = self.frame.advance_bytes(source.byte_len(), |position| source.byte_at(position), grant, false);
        let event = match step.status {
            Ok(Status::Complete) => {
                self.current = self.frame.take_object();
                self.frame.begin_close();
                self.phase = Phase::CloseFrame;
                Event::More
            }
            Err(_) => {
                self.frame.begin_close();
                self.phase = Phase::CloseFrame;
                Event::More
            }
            Ok(Status::NeedsLength(reference)) => Event::NeedsLength(reference),
            Ok(Status::NeedsAllocation { bytes }) => Event::NeedsAllocation { bytes },
            Ok(Status::NeedsRelease { bytes }) => Event::NeedsRelease { bytes },
            _ => Event::More,
        };
        NormalProjectionReceipt { event: Ok(event), work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
    }

    fn classify(&mut self) -> NormalProjectionReceipt {
        let Some(PdfObject::Dict(dict)) = self.current.as_ref().map(|object| &object.value) else {
            return receipt(Err(XrefStreamError::Structure("framed PDF stream has no dictionary")), 1);
        };
        if let Some(entry) = dict.get(self.type_position) {
            self.type_position += 1;
            if entry.key != "Type" { return receipt(Ok(NormalProjectionEvent::More), 1); }
            let kind = entry.value.as_name();
            self.object_stream = kind == Some("ObjStm");
            if self.object_stream && self.compressed_length {
                return receipt(Err(XrefStreamError::Structure("PDF object stream length cannot be compressed")), 1);
            }
            if kind == Some("XRef") || (self.object_stream && !self.keep_object_streams) {
                self.phase = Phase::Shape(Shape::Omitted);
                return receipt(Ok(NormalProjectionEvent::More), 1);
            }
        }
        self.decoder = Some(PdfDecodedStreamCursor::new(self.maximum_stream_bytes, self.maximum_stream_bytes));
        self.phase = Phase::Shape(Shape::Decode);
        receipt(Ok(NormalProjectionEvent::More), 1)
    }

    fn decode(&mut self, source: &(impl ByteReadSource + ?Sized), grant: CosGrant) -> NormalProjectionReceipt {
        let Some(NormalObject { value: PdfObject::Dict(dict), stream: Some(span), .. }) = self.current.as_ref() else {
            return receipt(Err(XrefStreamError::Structure("framed PDF stream lost its dictionary")), 0);
        };
        let Some(raw) = span.end.checked_sub(span.start).and_then(|length| ByteReadRange::new(source, span.start, length)) else {
            return receipt(Err(XrefStreamError::Structure("retained PDF stream source changed")), 0);
        };
        let decoder = self.decoder.as_mut().expect("active PDF stream decoder");
        let mut step = decoder.advance(dict, &raw, stream_grant(grant), false);
        if matches!(step.status, Ok(StreamDataStatus::Complete)) {
            self.data = decoder.take_output();
            if self.data.is_none() { return receipt(Err(XrefStreamError::Structure("retained PDF stream lost its decoded output")), step.work); }
            decoder.begin_close();
            self.phase = Phase::Shape(Shape::CloseDecoder { decoded: true });
            step.status = Ok(StreamDataStatus::More);
        } else if step.status.as_ref().is_err_and(|error| matches!(error.kind, ValueRefusalKind::InvalidValue | ValueRefusalKind::UnsupportedOwner)) {
            decoder.begin_close();
            self.phase = Phase::Shape(Shape::CloseDecoder { decoded: false });
            step.status = Ok(StreamDataStatus::More);
        }
        stream_receipt(step)
    }

    fn sanitize(&mut self, grant: CosGrant) -> NormalProjectionReceipt {
        if let Some(cursor) = self.dictionary.as_mut() {
            let step = cursor.advance(grant, false);
            if matches!(step.status, Ok(CosStatus::Complete { .. })) { self.phase = Phase::Shape(Shape::Ready); }
            return cos_receipt(step);
        }
        let object = self.current.as_mut().expect("decoded PDF stream owner");
        let PdfObject::Dict(dict) = std::mem::replace(&mut object.value, PdfObject::Null) else {
            return receipt(Err(XrefStreamError::Structure("decoded PDF stream dictionary disappeared")), 1);
        };
        self.dictionary = Some(PdfStreamDictionaryCursor::new(dict, true));
        receipt(Ok(NormalProjectionEvent::More), 1)
    }

    /// 📤️ Moves a typed value or the exact source to the reference codec.
    pub fn take(&mut self) -> Option<NormalProjection> {
        match self.phase {
            Phase::Shape(Shape::Ready) => {
                let NormalObject { value, stream, .. } = self.current.take()?;
                self.phase = Phase::Closing;
                let value = if stream.is_some() {
                    let dict = self.dictionary.as_mut()?.take_dictionary()?;
                    let DecodedStreamData { bytes: data, filters } = self.data.take()?;
                    PdfObject::Stream { dict, data, filters }
                } else { value };
                Some(if self.object_stream { NormalProjection::ObjectStream(value) } else { NormalProjection::Value(value) })
            }
            Phase::Shape(Shape::Reference) => {
                let NormalObject { value: PdfObject::Dict(dict), stream: Some(span), .. } = self.current.take()? else { return None; };
                self.phase = Phase::Closing;
                Some(NormalProjection::ReferenceStream { dict, span })
            }
            Phase::Shape(Shape::Omitted) => { self.begin_close(); Some(NormalProjection::Omitted) }
            _ => None,
        }
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::Closed | Phase::Closing) {
            self.frame.begin_close();
            if let Some(decoder) = self.decoder.as_mut() { decoder.begin_close(); }
            self.phase = Phase::Closing;
        }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> NormalProjectionReceipt {
        use NormalProjectionEvent as Event;
        if matches!(self.phase, Phase::Closed) { return receipt(Ok(Event::Closed), 0); }
        if !matches!(self.phase, Phase::Closing) || grant.work == 0 { return receipt(Ok(Event::More), 0); }
        if !self.frame_closed { return self.close_frame(grant); }
        if self.decoder.is_some() { return self.close_decoder(grant); }
        if let Some(cursor) = self.dictionary.as_mut() {
            let mut step = cursor.close_step(grant);
            if step.status == Ok(CosStatus::Closed) { self.dictionary = None; step.status = Ok(CosStatus::More); }
            return cos_receipt(step);
        }
        if let Some(cursor) = self.retiring.as_mut() {
            let mut step = cursor.close_step(grant);
            if step.status == Ok(CosStatus::Closed) { self.retiring = None; step.status = Ok(CosStatus::More); }
            return cos_receipt(step);
        }
        if let Some(object) = self.current.take() {
            self.retiring = Some(PdfCosCursor::retire_parsed_value(object.value));
            return receipt(Ok(Event::More), 1);
        }
        if let Some(data) = self.data.as_mut() {
            let bytes = data.bytes.capacity();
            if bytes != 0 {
                if bytes > grant.release_bytes { return receipt(Ok(Event::NeedsRelease { bytes }), 0); }
                data.bytes = Vec::new();
                return released(bytes);
            }
            let count = data.filters.len().min(grant.work);
            if count != 0 {
                data.filters.truncate(data.filters.len() - count);
                return receipt(Ok(Event::More), count);
            }
            let bytes = data.filters.capacity() * std::mem::size_of::<PdfStreamFilter>();
            if bytes > grant.release_bytes { return receipt(Ok(Event::NeedsRelease { bytes }), 0); }
            self.data = None;
            return released(bytes);
        }
        self.phase = Phase::Closed;
        receipt(Ok(Event::Closed), 0)
    }

    fn close_frame(&mut self, grant: CosGrant) -> NormalProjectionReceipt {
        let step = self.frame.close_step(grant);
        let event = match step.status {
            Ok(NormalObjectStatus::NeedsRelease { bytes }) => Ok(NormalProjectionEvent::NeedsRelease { bytes }),
            Ok(NormalObjectStatus::Closed) => { self.frame_closed = true; Ok(NormalProjectionEvent::More) }
            Err(error) => Err(XrefStreamError::Structure(error)),
            _ => Ok(NormalProjectionEvent::More),
        };
        NormalProjectionReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
    }

    fn close_decoder(&mut self, grant: CosGrant) -> NormalProjectionReceipt {
        let mut step = self.decoder.as_mut().expect("PDF stream decoder closes once").close_step(stream_grant(grant));
        if matches!(step.status, Ok(StreamDataStatus::Closed)) { self.decoder = None; step.status = Ok(StreamDataStatus::More); }
        stream_receipt(step)
    }
}

fn stream_grant(grant: CosGrant) -> StreamDataGrant {
    StreamDataGrant { work: grant.work, allocation_bytes: grant.allocation_bytes, release_bytes: grant.release_bytes }
}

fn stream_receipt(step: StreamDataReceipt) -> NormalProjectionReceipt {
    use NormalProjectionEvent as Event;
    let event = step.status.map_err(XrefStreamError::Decode).map(|status| match status {
        StreamDataStatus::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        StreamDataStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        _ => Event::More,
    });
    NormalProjectionReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
}

fn cos_receipt(step: CosReceipt) -> NormalProjectionReceipt {
    use NormalProjectionEvent as Event;
    let event = step.status.map_err(XrefStreamError::Structure).map(|status| match status {
        CosStatus::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        CosStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        _ => Event::More,
    });
    NormalProjectionReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
}

fn receipt(event: Result<NormalProjectionEvent, XrefStreamError>, work: usize) -> NormalProjectionReceipt {
    NormalProjectionReceipt { event, work, allocated_bytes: 0, released_bytes: 0 }
}

fn released(bytes: usize) -> NormalProjectionReceipt {
    NormalProjectionReceipt { released_bytes: bytes, ..receipt(Ok(NormalProjectionEvent::More), 1) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
