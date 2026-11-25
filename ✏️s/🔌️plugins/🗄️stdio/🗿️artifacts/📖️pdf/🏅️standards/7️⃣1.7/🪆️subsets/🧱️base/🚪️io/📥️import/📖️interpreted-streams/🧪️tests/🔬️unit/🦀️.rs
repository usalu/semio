//! 🧪️ The interpreted-stream walk: what an import decodes in place, what it keeps as the file's encoded bytes, and its bounded, cancellable units.

use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::{stream_bytes, PdfStreamFilter};
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::encode_stream;

const CONTENT: &[u8] = b"0 0 m 10 10 l S\n";

fn private_data() -> Vec<u8> { b"application private data that no PDF reader interprets ".repeat(64) }

fn encoded(data: &[u8]) -> (Vec<u8>, Vec<PdfDictEntry>) { encode_stream(data, &[PdfStreamFilter::Flate { predictor: None }]) }

fn reference(num: u32) -> PdfObject { PdfObject::Ref(ObjRef { num, gen: 0 }) }

/// 🧱️ A one-page graph as an import retains it: the page content and the page's private application data both still encoded.
fn retained_graph() -> (Vec<PdfIndirectObject>, Vec<PdfDictEntry>) {
    let object = |num, value| PdfIndirectObject { id: ObjRef { num, gen: 0 }, value };
    let stream = |data: &[u8]| { let (body, entries) = encoded(data); PdfObject::Stream { dict: entries, data: stream_bytes(&body), filters: Vec::new() } };
    let objects = vec![
        object(1, PdfObject::Dict(vec![PdfDictEntry::new("Type", PdfObject::name("Catalog")), PdfDictEntry::new("Pages", reference(2))])),
        object(2, PdfObject::Dict(vec![PdfDictEntry::new("Type", PdfObject::name("Pages")), PdfDictEntry::new("Kids", PdfObject::Array(vec![reference(3)])), PdfDictEntry::new("Count", PdfObject::Int(1))])),
        object(3, PdfObject::Dict(vec![
            PdfDictEntry::new("Type", PdfObject::name("Page")), PdfDictEntry::new("Parent", reference(2)), PdfDictEntry::new("Contents", reference(4)),
            PdfDictEntry::new("PieceInfo", PdfObject::Dict(vec![PdfDictEntry::new("Application", PdfObject::Dict(vec![PdfDictEntry::new("Private", reference(5))]))])),
        ])),
        object(4, stream(CONTENT)),
        object(5, stream(&private_data())),
    ];
    (objects, vec![PdfDictEntry::new("Root", reference(1))])
}

fn position(objects: &[PdfIndirectObject]) -> impl Fn(ObjRef) -> Option<usize> + '_ { move |id| objects.iter().position(|object| object.id == id) }

#[test]
fn the_walk_decodes_page_content_in_place_and_keeps_private_application_data_as_the_files_bytes() {
    let (mut objects, trailer) = retained_graph();
    let private = objects[4].value.clone();
    let positions: Vec<ObjRef> = objects.iter().map(|object| object.id).collect();
    let counts = admit_interpreted_streams(&mut objects, &trailer, |id| positions.iter().position(|candidate| *candidate == id)).unwrap();
    assert_eq!(counts, (1, 0));
    let PdfObject::Stream { dict, data, filters } = &objects[3].value else { panic!("page content stream") };
    assert_eq!(data.to_vec_owner(), CONTENT);
    assert_eq!(filters, &vec![PdfStreamFilter::Flate { predictor: None }]);
    assert!(dict.iter().all(|entry| entry.key != "Filter"), "the decoded stream's chain moved into its typed filters");
    assert_eq!(objects[4].value, private, "the private stream keeps its encoded body and its declared chain");
}

#[test]
fn every_walk_unit_stays_inside_its_grant_and_cancellation_mid_decode_retires_everything_it_allocated() {
    for cancel in [false, true] {
        let (mut objects, trailer) = retained_graph();
        let original = objects.clone();
        let mut cursor = InterpretedStreamCursor::new(1 << 20);
        let mut grant = CosGrant { work: 1, allocation_bytes: 0, release_bytes: 0 };
        let (mut allocated, mut released) = (0usize, 0usize);
        loop {
            if cancel && cursor.decodes_body_bytes() { cursor.begin_close(); break; }
            let turn = cursor.advance(&mut objects, &trailer, position(&original), grant);
            assert!(turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes, "a unit stays inside its grant");
            allocated += turn.allocated_bytes;
            released += turn.released_bytes;
            match turn.status.unwrap() {
                CosStatus::NeedsAllocation { bytes } => grant.allocation_bytes = bytes,
                CosStatus::NeedsRelease { bytes } => grant.release_bytes = bytes,
                CosStatus::Complete { .. } => { cursor.begin_close(); break; }
                _ => { grant.allocation_bytes = 0; grant.release_bytes = 0; }
            }
        }
        let close = CosGrant { work: 1, allocation_bytes: 0, release_bytes: usize::MAX };
        loop {
            let turn = cursor.close_step(close);
            released += turn.released_bytes;
            if turn.status.unwrap() == CosStatus::Closed { break; }
        }
        if cancel {
            assert_eq!(objects, original, "a cancelled walk leaves every stream as the import retained it");
            assert_eq!(allocated, released, "a cancelled walk retires everything it allocated");
        } else {
            let PdfObject::Stream { data, .. } = &objects[3].value else { panic!("page content stream") };
            assert_eq!(data.to_vec_owner(), CONTENT);
            assert_eq!(objects[4].value, original[4].value);
        }
    }
}
