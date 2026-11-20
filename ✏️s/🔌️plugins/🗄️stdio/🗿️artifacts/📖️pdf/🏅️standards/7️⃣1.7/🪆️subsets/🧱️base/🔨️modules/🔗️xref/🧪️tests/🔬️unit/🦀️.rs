#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::encryption::Decryptor;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::filters::decode_stream;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::lexer::{brute_force_scan, dict_i64, find_last_subslice, parse_indirect_at, Lexer};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::io::error::{malformed, PResult, PdfEngineError};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use crate::standards::v1_7::subsets::base::modules::xref::*;

#[test]
fn incremental_revisions_keep_freed_objects_out_of_the_retained_graph() {
    for pdf in [include_bytes!("../../../../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf").as_slice(), include_bytes!("../../../../🧪️tests/🧫️fixtures/🔗️incremental-free/💧️stream.pdf").as_slice(), include_bytes!("../../../../🧪️tests/🧫️fixtures/🔗️incremental-free/🌀️flate.pdf").as_slice()] {
        let index = build_xref(pdf).unwrap();
        assert!(!index.brute_forced);
        assert!(!index.entries.contains_key(&3));
        assert!(index.entries.contains_key(&9));
        let mut resolver = Resolver::new(pdf, index.entries);
        assert!(resolver.resolve(3).is_none());
        assert!(resolver.resolve(9).is_some());
    }
}

#[test]
fn undecodable_retained_streams_keep_their_encoding_declaration_and_bytes() {
    use crate::standards::v1_7::subsets::base::modules::{lexer::object_bytes, writer::materialize_streams};
    let input = b"8 0 obj << /Length 3 /Filter /VendorFilter /DecodeParms << /Mode 2 >> >>\nstream\nabc\nendstream\nendobj";
    let (_, source) = parse_indirect_at(input, 0).unwrap();
    let object = normalize_pdf_object(source).unwrap();
    let raw = object_bytes(&materialize_streams(&object));
    let mut reader = Lexer::new(&raw);
    let restored = reader.parse_object().unwrap();
    assert_eq!(restored.dict_get("Filter").and_then(PdfObject::as_name), Some("VendorFilter"));
    assert_eq!(restored.dict_get("DecodeParms").and_then(|p| p.dict_get("Mode")).and_then(PdfObject::as_i64), Some(2));
    let PdfObject::Stream { data, .. } = restored else { panic!("expected retained stream") };
    assert_eq!(data, b"abc");
}

#[test]
fn latest_classic_section_yields_free_entry_and_trailer_using_single_work_grants() {
    use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosStatus};
    let source = include_bytes!("../../../../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf");
    let mut cursor = ClassicSectionCursor::new(startxref_offset(source).unwrap());
    let mut rows = Vec::new();
    let grant = CosGrant { work: 1, allocation_bytes: 8192, release_bytes: 8192 };
    loop {
        let turn = cursor.advance(source, grant, false);
        assert!(turn.work <= 1);
        match turn.event.unwrap() {
            ClassicSectionEvent::Row { number, live, .. } => rows.push((number, live)),
            ClassicSectionEvent::Progress(CosStatus::Complete { .. }) => break,
            ClassicSectionEvent::Progress(CosStatus::More) => {}
            _ => panic!("expected xref progress"),
        }
    }
    assert_eq!(rows, [(0, false), (3, false), (9, true)]);
    let trailer = PdfObject::Dict(cursor.take_trailer().unwrap());
    assert_eq!(trailer.dict_get("Info").and_then(PdfObject::as_ref), Some(ObjRef { num: 9, gen: 0 }));
}

#[test]
fn prefixed_pdf_preserves_structured_free_entries_under_either_offset_convention() {
    use crate::standards::v1_7::subsets::base::io::import::decode_pdf;
    let source = include_bytes!("../../../../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf");
    let prefix = b"\n% document preamble\n";
    let delta = prefix.len();
    let mut absolute = source.to_vec();
    let mut next_footer = false;
    for line in absolute.split_mut(|&byte| byte == b'\n') {
        if next_footer {
            shift_decimal(line, delta);
            next_footer = false;
        }
        if line == b"startxref" { next_footer = true; }
        if line.get(16..18) == Some(b" n") { shift_decimal(&mut line[..10], delta); }
        if let Some(prev) = line.windows(6).position(|word| word == b"/Prev ") {
            let digits = &mut line[prev + 6..];
            let end = digits.iter().position(|b| !b.is_ascii_digit()).unwrap_or(digits.len());
            shift_decimal(&mut digits[..end], delta);
        }
    }
    for source in [source.as_slice(), &absolute] {
        let pdf = [prefix.as_slice(), source].concat();
        let doc = decode_pdf(&pdf).unwrap();
        assert_eq!(doc.pages.len(), 2);
        assert_eq!(doc.info.title.as_deref(), Some("Updated"));
        assert!(doc.objects.iter().all(|object| object.id.num != 3));
    }

    fn shift_decimal(field: &mut [u8], delta: usize) {
        let value = std::str::from_utf8(field).unwrap().parse::<usize>().unwrap() + delta;
        field.copy_from_slice(format!("{value:0width$}", width = field.len()).as_bytes());
    }
}
