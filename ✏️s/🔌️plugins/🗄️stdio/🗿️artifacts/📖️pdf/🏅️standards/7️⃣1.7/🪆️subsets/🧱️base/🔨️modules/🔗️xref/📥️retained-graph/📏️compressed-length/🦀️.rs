//! 📏️ Resolves a stream's compressed integer length through its exact live object-stream entry.

use semio_framework_value::ByteReadSource;

use super::{
    admit_box, failed, more, object_stream, projection, release, released, report, retire_object_stream_header, retire_projection,
    CosGrant, CosLimits, GraphImportCursor, GraphImportEvent, GraphImportReceipt, NormalProjection, NormalProjectionCursor,
    NormalProjectionEvent, ObjRef, ObjectStreamCursor, ObjectStreamEvent, ObjectStreamHeader, PdfObject,
    RetiringValue, XrefEntry, XrefIndex, XrefIndexLookup,
};

pub(super) struct CompressedLength {
    reference: ObjRef,
    stream_number: u32,
    stream_lookup: XrefIndexLookup,
    member_index: u32,
    header: Option<ObjectStreamHeader>,
    data: Vec<u8>,
    member: Option<Box<ObjectStreamCursor>>,
    result: Option<Option<usize>>,
    pub(super) projection: Option<Box<NormalProjectionCursor>>,
}

impl CompressedLength {
    pub(super) fn new(reference: ObjRef, stream_number: u32, member_index: u32) -> Self {
        Self { reference, stream_number, stream_lookup: XrefIndexLookup::new(stream_number), member_index, header: None, data: Vec::new(), member: None, result: None, projection: None }
    }

    pub(super) fn stream_number(&self) -> u32 { self.stream_number }

    pub(super) fn begin_close(&mut self) {
        if let Some(projection) = self.projection.as_mut() { projection.begin_close(); }
        if let Some(member) = self.member.as_mut() { member.begin_close(); }
    }
}

impl GraphImportCursor {
    pub(super) fn advance_compressed_length(&mut self, source: &(impl ByteReadSource + ?Sized), index: &XrefIndex, grant: CosGrant) -> GraphImportReceipt {
        let lookup = self.compressed_length.as_mut().expect("compressed PDF length lookup");
        if let Some(length) = lookup.result {
            let reference = lookup.reference;
            let step = self.close_compressed_length(grant);
            if self.compressed_length.is_none() {
                let waiting = self.normal.as_mut().expect("PDF stream awaiting compressed length");
                if let Err(reason) = waiting.provide_compressed_length(reference, length) { return failed(reason); }
            }
            return step;
        }
        if let Some(member) = lookup.member.as_mut() {
            let step = member.advance(&lookup.data, grant, false);
            if matches!(step.event, Ok(ObjectStreamEvent::Complete { .. })) {
                let Some(value) = member.take_value() else { return failed("compressed PDF length was unavailable"); };
                let length = match value {
                    PdfObject::Int(length) => usize::try_from(length).ok(),
                    value => { self.retiring = Some(RetiringValue::new(value)); None }
                };
                lookup.result = Some(length);
                lookup.begin_close();
            }
            return object_stream(step);
        }
        if let Some(header) = lookup.header.as_mut() {
            if lookup.projection.is_some() { return retire_projection(&mut lookup.projection, grant); }
            if header.advance() { return more(1); }
            let Some((count, first)) = header.fields() else { return failed("PDF object stream is missing N or First"); };
            let bytes = std::mem::size_of::<ObjectStreamCursor>();
            if bytes > grant.allocation_bytes { return report(Ok(GraphImportEvent::NeedsAllocation { bytes }), 0); }
            let mut member = ObjectStreamCursor::new(count, first, CosLimits::default());
            if let Err(reason) = member.select(lookup.reference.num, lookup.member_index) { return failed(reason); }
            let Some(member) = admit_box(member) else { return failed("unable to allocate compressed PDF length member"); };
            lookup.member = Some(member);
            self.retiring = retire_object_stream_header(header);
            return GraphImportReceipt { allocated_bytes: bytes, ..more(1) };
        }
        self.advance_length_projection(source, index, grant)
    }

    fn advance_length_projection(&mut self, source: &(impl ByteReadSource + ?Sized), index: &XrefIndex, grant: CosGrant) -> GraphImportReceipt {
        let lookup = self.compressed_length.as_mut().expect("compressed PDF length carrier");
        let Some(cursor) = lookup.projection.as_mut() else {
            let (work, entry) = lookup.stream_lookup.advance(index, grant.work);
            if work != 0 { return more(work); }
            let Some(Some(XrefEntry::Normal { offset, gen })) = entry else {
                return failed("compressed PDF length carrier is not an indexed normal object");
            };
            let bytes = std::mem::size_of::<NormalProjectionCursor>();
            if bytes > grant.allocation_bytes { return report(Ok(GraphImportEvent::NeedsAllocation { bytes }), 0); }
            let id = ObjRef { num: lookup.stream_number, gen };
            let cursor = NormalProjectionCursor::new(id, offset, CosLimits::default(), self.maximum_stream_bytes).keep_object_streams();
            let Some(cursor) = admit_box(cursor) else { return failed("unable to allocate compressed PDF length carrier"); };
            lookup.projection = Some(cursor);
            return GraphImportReceipt { allocated_bytes: bytes, ..more(1) };
        };

        let step = cursor.advance(source, grant, false);
        match step.event {
            Ok(NormalProjectionEvent::NeedsLength(reference)) => self.request_length(reference),
            Ok(NormalProjectionEvent::Complete) => {
                match cursor.take() {
                    Some(NormalProjection::ObjectStream(PdfObject::Stream { dict, data, filters })) => {
                        lookup.header = Some(ObjectStreamHeader::new(dict));
                        lookup.data = data;
                        self.retiring = Some(RetiringValue::new(PdfObject::Stream { dict: Vec::new(), data: Vec::new(), filters }));
                    }
                    Some(NormalProjection::Value(value) | NormalProjection::ObjectStream(value)) => {
                        self.retiring = Some(RetiringValue::new(value));
                        return failed("compressed PDF length carrier is not a decoded object stream");
                    }
                    Some(NormalProjection::ReferenceStream { dict, .. }) => {
                        self.retiring = Some(RetiringValue::new(PdfObject::Dict(dict)));
                        return failed("compressed PDF length carrier is not a decoded object stream");
                    }
                    _ => return failed("compressed PDF length carrier is not a decoded object stream"),
                }
                cursor.begin_close();
            }
            _ => {}
        }
        projection(step)
    }

    pub(super) fn close_compressed_length(&mut self, grant: CosGrant) -> GraphImportReceipt {
        let lookup = self.compressed_length.as_mut().expect("closing compressed PDF length");
        if lookup.projection.is_some() { return retire_projection(&mut lookup.projection, grant); }
        if let Some(member) = lookup.member.as_mut() {
            let step = member.close_step(grant);
            if step.event != Ok(ObjectStreamEvent::Closed) || step.work != 0 { return object_stream(step); }
            let bytes = std::mem::size_of::<ObjectStreamCursor>();
            if bytes > grant.release_bytes { return report(Ok(GraphImportEvent::NeedsRelease { bytes }), 0); }
            lookup.member = None;
            return released(bytes);
        }
        if let Some(header) = lookup.header.take() {
            let mut header = header;
            self.retiring = retire_object_stream_header(&mut header);
            return more(1);
        }
        if lookup.data.capacity() != 0 { return release(&mut lookup.data, grant); }
        self.compressed_length = None;
        more(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{PdfByteGraphEvent, PdfFilePreparationCursor, PreparedPdfFile, XrefStreamError};
    use crate::standards::v1_7::subsets::base::modules::xref::read_file_xref;

    const INPUT: &[u8] = include_bytes!("../../../../🧪️tests/🧫️fixtures/📦️object-stream/📏️length.pdf");
    const CONTENT: &[u8] = b"BT /F1 12 Tf 10 70 Td (prefix\nendstream\nsuffix) Tj ET";
    const GRANT: CosGrant = CosGrant { work: 1, allocation_bytes: 32 * 1024, release_bytes: 32 * 1024 };

    fn prepare(input: &[u8]) -> Result<PreparedPdfFile, XrefStreamError> {
        let mut cursor = PdfFilePreparationCursor::new(100, 1_000_000);
        (0..100_000).find_map(|_| {
            let step = cursor.advance(input, GRANT, false);
            assert!(step.work <= GRANT.work);
            assert!(step.allocated_bytes <= GRANT.allocation_bytes && step.released_bytes <= GRANT.release_bytes);
            match step.event {
                Ok(PdfByteGraphEvent::More | PdfByteGraphEvent::Object) => None,
                Ok(PdfByteGraphEvent::Complete) => cursor.take().map(Ok),
                Err(error) => Some(Err(error)),
                event => panic!("unexpected graph event {event:?}"),
            }
        }).expect("one-unit compressed length completion")
    }

    #[test]
    fn exact_compressed_length_preserves_the_first_and_subsequent_stream_bytes() {
        let file = prepare(INPUT).unwrap();
        for number in [4, 10] {
            let object = file.graph.objects.iter().find(|object| object.id == ObjRef { num: number, gen: 0 }).unwrap();
            let PdfObject::Stream { data, .. } = &object.value else { panic!("expected content stream"); };
            assert_eq!(data.as_slice(), CONTENT);
        }
        for (number, value) in [(6, 42), (7, CONTENT.len() as i64)] {
            let object = file.graph.objects.iter().find(|object| object.id == ObjRef { num: number, gen: 0 }).unwrap();
            assert_eq!(object.value, PdfObject::Int(value));
        }
    }

    #[test]
    fn compressed_length_cancellation_retires_the_carrier_and_member_admissions() {
        let (_, offset, file) = read_file_xref(INPUT).unwrap();
        let input = &INPUT[offset..];
        for member_admitted in [false, true] {
            let mut cursor = GraphImportCursor::new(file.entries.len(), 1_000_000);
            let mut held = 0usize;
            let advance = |cursor: &mut GraphImportCursor, held: &mut usize| {
                let step = cursor.advance(input, &file.entries, GRANT, false);
                assert!(step.work <= 1);
                assert!(step.allocated_bytes <= GRANT.allocation_bytes && step.released_bytes <= GRANT.release_bytes);
                *held = (*held + step.allocated_bytes).checked_sub(step.released_bytes).unwrap();
                assert!(matches!(step.event, Ok(GraphImportEvent::Object | GraphImportEvent::More)), "{:?}", step.event);
            };

            assert!((0..100_000).any(|_| {
                advance(&mut cursor, &mut held);
                cursor.compressed_length.as_ref().is_some_and(|lookup| {
                    if member_admitted { lookup.member.is_some() } else { lookup.projection.is_some() }
                })
            }));
            if member_admitted {
                for _ in 0..12 { advance(&mut cursor, &mut held); }
                let lookup = cursor.compressed_length.as_ref().unwrap();
                assert!(lookup.member.is_some() && !lookup.data.is_empty());
            }
            assert_eq!(cursor.advance(input, &file.entries, GRANT, true).event.unwrap(), GraphImportEvent::Cancelled);
            let denied = CosGrant { release_bytes: 0, ..GRANT };
            assert!((0..100).any(|_| {
                let step = cursor.close_step(denied);
                assert_eq!(step.allocated_bytes + step.released_bytes, 0);
                matches!(step.event, Ok(GraphImportEvent::NeedsRelease { bytes }) if bytes > 0)
            }));
            assert!((0..100_000).any(|_| {
                let step = cursor.close_step(GRANT);
                assert!(step.work <= GRANT.work);
                assert_eq!(step.allocated_bytes, 0);
                held = held.checked_sub(step.released_bytes).unwrap();
                step.event.unwrap() == GraphImportEvent::Closed
            }));
            assert_eq!(held, 0);
            assert!(cursor.take().is_none());
        }
    }

    #[test]
    fn compressed_length_carrier_cannot_depend_on_its_own_member() {
        let mut input = INPUT.to_vec();
        let marker = b"/Length 9 2 R";
        let start = INPUT.windows(marker.len()).position(|window| window == marker).unwrap();
        input[start..start + marker.len()].copy_from_slice(b"/Length 7 0 R");
        assert!(matches!(prepare(&input), Err(XrefStreamError::Structure("PDF object stream length cannot be compressed"))));
    }

    #[test]
    fn compressed_length_can_only_belong_to_generation_zero() {
        let mut input = INPUT.to_vec();
        let marker = b"/Length 7 0 R";
        let start = INPUT.windows(marker.len()).position(|window| window == marker).unwrap();
        input[start..start + marker.len()].copy_from_slice(b"/Length 7 1 R");
        let file = prepare(&input).unwrap();
        let first = file.graph.objects.iter().find(|object| object.id.num == 4).unwrap();
        let PdfObject::Stream { data, .. } = &first.value else { panic!("expected content stream"); };
        assert_eq!(data.as_slice(), b"BT /F1 12 Tf 10 70 Td (prefix");
    }
}
