//! 🧪️ testkit of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

#[cfg(test)]
#[path = "../../../../🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content/🧪️tests/🔬️unit/🦀️.rs"]
mod page_content_contracts;

#[cfg(test)]
#[path = "../../../../🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🧪️tests/🔬️unit/🦀️.rs"]
mod controlled_real_world;

#[cfg(test)]
mod document_contracts {
    use crate::standards::v1_7::subsets::base::{
        io::text::snapshot::retained_text::admit_retained_text,
        modules::{
            lexer::{parse_indirect_at, CosGrant},
            lift::{lift_document, lift_document_with, ControlledDocumentLiftCursor, ControlledLiftBarrier, ControlledLiftEvent,
                DocumentLiftAction, DocumentLiftCursor, DocumentLiftGrant, DocumentLiftStatus, PageContentBarrier, PageContentEvent},
        },
        schema::{
            graph_source::{GraphSource, ObjectSource}, snapshot::{stream_bytes, ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfOp, PdfTextString},
            stream_roles::PdfStreamRoleValue,
        },
    };

    fn inherited_fixture_objects() -> Vec<PdfIndirectObject> {
        let input = include_bytes!("../../../../🧪️tests/🧫️fixtures/🌳️page-tree/🧪️nested-inheritance.pdf");
        (1..=13).map(|number| {
            let header = format!("\n{number} 0 obj\n");
            let offset = input.windows(header.len()).position(|bytes| bytes == header.as_bytes()).unwrap() + 1;
            let (id, value) = parse_indirect_at(input, offset).unwrap();
            PdfIndirectObject { id, value }
        }).collect()
    }

    #[test]
    fn document_lifting_preserves_inherited_page_geometry_and_text_resources() {
        let objects = inherited_fixture_objects();
        let mut source = GraphSource::new(&objects);
        let trailer = [PdfDictEntry::new("Root", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))];
        let lifted = lift_document_with(&trailer, "1.7", &mut source);
        assert!(lifted.admission_error.is_none());
        assert_eq!(lifted.page_refs.iter().map(|reference| reference.num).collect::<Vec<_>>(), [6, 5, 3]);
        let pages = &lifted.snapshot.pages;
        let probes = [
            ([0.0, 0.0, 200.0, 250.0], Some([10.0, 20.0, 190.0, 230.0]), 180, "FInner", "Override"),
            ([0.0, 0.0, 300.0, 400.0], Some([10.0, 20.0, 190.0, 230.0]), 90, "FInner", "Inner"),
            ([0.0, 0.0, 300.0, 400.0], None, 0, "FRoot", "Root"),
        ];
        assert_eq!(pages.len(), probes.len());
        for (page, (media, crop, rotation, font, label)) in pages.iter().zip(probes) {
            assert_eq!((page.media_box, page.crop_box, page.rotate), (media, crop, rotation));
            assert!(page.content.iter().any(|op| matches!(op, PdfOp::SetFont { name, .. } if name == font)));
            assert!(page.content.iter().any(|op| matches!(op, PdfOp::ShowText { text: PdfTextString::Text { text } } if text == label)));
        }
    }

    #[test]
    fn semantic_lift_stages_pages_and_transfers_cancelled_or_failed_results() {
        let graph = inherited_fixture_objects();
        let trailer = [PdfDictEntry::new("Root", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))];
        let mut cursor = DocumentLiftCursor::new();
        let traversal = CosGrant { work: 1, allocation_bytes: 64 * 1024, release_bytes: 64 * 1024 };
        let planned = DocumentLiftGrant { traversal, semantic: false };
        {
            let mut first_borrow = GraphSource::new(&graph);
            let first = cursor.advance(&trailer, "1.7", &mut first_borrow, planned, false);
            assert_eq!(first.status, DocumentLiftStatus::NeedsSemantic(DocumentLiftAction::Header));
        }
        let mut source = GraphSource::new(&graph);
        for _ in 0..2048 {
            let plan = cursor.advance(&trailer, "1.7", &mut source, planned, false);
            assert!(plan.semantic.is_none());
            if let Some(turn) = plan.traversal {
                assert!(turn.work <= traversal.work);
            }
            if let DocumentLiftStatus::NeedsSemantic(action) = plan.status {
                let turn = cursor.advance(&trailer, "1.7", &mut source, DocumentLiftGrant { traversal: CosGrant::default(), semantic: true }, false);
                assert_eq!(turn.semantic, Some(action));
                assert!(turn.traversal.is_none());
                if action == (DocumentLiftAction::ProjectPage { index: 0 }) {
                    assert_eq!(cursor.page_progress(), (3, 1));
                    assert_eq!(cursor.advance(&trailer, "1.7", &mut source, DocumentLiftGrant::default(), true).status, DocumentLiftStatus::Cancelled);
                    assert!(cursor.take_result().is_none());
                    let retained = cursor.take_partial().expect("partial semantic result");
                    assert_eq!(retained.lifter.snapshot.pages.len(), 1);
                    assert_eq!(retained.pages.len(), 3);
                    break;
                }
            }
        }
        assert_eq!(cursor.page_progress().1, 1);
        drop(cursor);
        drop(source);

        let mut graph = graph;
        let PdfObject::Stream { data, .. } = &mut graph[12].value else { panic!("fixture content stream"); };
        *data = stream_bytes(b"BI /W 1 /H 1 /BPC 8 /CS /G /F /VendorFilter ID x EI\n");
        let mut source = GraphSource::new(&graph);
        let mut cursor = DocumentLiftCursor::new();
        let grant = DocumentLiftGrant { traversal, semantic: true };
        for _ in 0..2048 {
            let turn = cursor.advance(&trailer, "1.7", &mut source, grant, false);
            if turn.status == DocumentLiftStatus::Failed {
                assert!(cursor.take_result().is_none());
                let retained = cursor.take_partial().expect("failed semantic result");
                assert!(retained.lifter.admission_error.is_some());
                assert!(retained.failed_page.is_some());
                assert!(retained.lifter.snapshot.pages.is_empty());
                return;
            }
        }
        panic!("malformed page did not stop the semantic lift");
    }

    #[test]
    fn resource_free_document_projects_identically_without_cold_semantic_grants() {
        assert!(std::mem::size_of::<ControlledDocumentLiftCursor>() <= 32 * 1024);
        let reference = |num| ObjRef { num, gen: 0 };
        let link = |num| PdfObject::Ref(reference(num));
        let name = |value: &str| PdfObject::Name(value.into());
        let dict = |pairs: Vec<(&str, PdfObject)>| PdfObject::Dict(pairs.into_iter().map(|(key, value)| PdfDictEntry::new(key, value)).collect());
        let rect = |limits: [i64; 4]| PdfObject::Array(limits.into_iter().map(PdfObject::Int).collect());
        let (first, second) = include_str!("../../../../🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content/🧪️tests/🔬️unit/🧪️page-projection.txt").split_once("\n%% STREAM\n").unwrap();
        let values = vec![
            dict(vec![("Type", name("Catalog")), ("Pages", link(2)), ("Version", name("1.7")), ("PageLayout", name("SinglePage")), ("PageMode", name("UseNone")), ("Lang", PdfObject::Text("en".into()))]),
            dict(vec![("Type", name("Pages")), ("Count", PdfObject::Int(1)), ("MediaBox", rect([0, 0, 100, 90])), ("Kids", PdfObject::Array(vec![link(3)])), ("Resources", link(8))]),
            dict(vec![("Type", name("Page")), ("Parent", link(2)), ("CropBox", rect([5, 2, 98, 89])), ("Rotate", PdfObject::Int(90)), ("Contents", link(4)), ("Annots", PdfObject::Array(Vec::new())), ("AA", dict(Vec::new()))]),
            PdfObject::Array(vec![link(5), link(6)]),
            PdfObject::Stream { dict: Vec::new(), data: stream_bytes(&(first.as_bytes())), filters: Vec::new() },
            PdfObject::Stream { dict: Vec::new(), data: stream_bytes(&(second.as_bytes())), filters: Vec::new() },
            dict(vec![("Title", PdfObject::Text("Controlled vector".into()))]),
            dict(Vec::new()),
        ];
        let mut objects = values.into_iter().enumerate().map(|(index, value)| PdfIndirectObject { id: reference(index as u32 + 1), value }).collect::<Vec<_>>();
        let trailer = [PdfDictEntry::new("Root", link(1)), PdfDictEntry::new("Info", link(7)),
            PdfDictEntry::new("ID", PdfObject::Array(vec![PdfObject::Str(vec![1, 2]), PdfObject::Str(vec![3, 4])]))];
        let reference_result = lift_document(&trailer, "1.6", &mut GraphSource::new(&objects)).unwrap();
        for expect_barrier in [false, true] {
            if expect_barrier {
                let PdfObject::Dict(page) = &mut objects[2].value else { unreachable!() };
                page.push(PdfDictEntry::new("Thumb", link(5)));
            }
            let mut source = GraphSource::new(&objects);
            let mut cursor = DocumentLiftCursor::controlled();
            let mut grant = CosGrant { work: 1, allocation_bytes: 0, release_bytes: 0 };
            let mut ended = false;
            for _ in 0..8192 {
                let turn = cursor.advance(&trailer, "1.6", &mut source, grant, false);
                assert!(turn.work <= 1 && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
                match turn.event.unwrap() {
                    ControlledLiftEvent::NeedsAllocation { bytes } => grant.allocation_bytes = bytes,
                    ControlledLiftEvent::NeedsRelease { bytes } => grant.release_bytes = bytes,
                    ControlledLiftEvent::More => { grant.allocation_bytes = 0; grant.release_bytes = 0; }
                    ControlledLiftEvent::Complete => {
                        assert!(!expect_barrier);
                        let snapshot = cursor.take_result().unwrap();
                        assert_eq!(snapshot, reference_result);
                        ended = true;
                        break;
                    }
                    ControlledLiftEvent::Deferred(ControlledLiftBarrier::Page) => {
                        assert!(expect_barrier && cursor.take_result().is_none());
                        cursor.begin_close();
                        let close = CosGrant { work: 1, allocation_bytes: 0, release_bytes: usize::MAX };
                        while cursor.close_step(close).event != Ok(ControlledLiftEvent::Closed) {}
                        assert!(cursor.take_partial().snapshot.is_some());
                        ended = true;
                        break;
                    }
                    event => panic!("unexpected controlled PDF lift state {event:?}"),
                }
            }
            assert!(ended);
        }
    }

    #[test]
    fn document_lift_consumes_the_granted_vector_program_before_cold_page_finalization() {
        let (a, b) = include_str!("../../../../🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content/🧪️tests/🔬️unit/🧪️page-projection.txt").split_once("\n%% STREAM\n").unwrap();
        let reference = |num| ObjRef { num, gen: 0 };
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>",
            "<< /Type /Pages /Count 2 /Kids [3 0 R 4 0 R] /MediaBox [0 0 100 100] >>",
            "<< /Type /Page /Parent 2 0 R /Contents 5 0 R >>",
            "<< /Type /Page /Parent 2 0 R /Contents 8 0 R /Resources << /Font << /F1 9 0 R >> >> >>",
            "[6 0 R 7 0 R]",
            "",
            "",
            "",
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
        ];
        let graph: Vec<_> = objects.iter().enumerate().map(|(index, authored)| {
            let id = reference(index as u32 + 1);
            let value = match id.num {
                6..=8 => PdfObject::Stream { dict: Vec::new(), data: stream_bytes(&([a, b, "BT /F1 12 Tf (Inner) Tj ET"][(id.num - 6) as usize].as_bytes())), filters: Vec::new() },
                _ => parse_indirect_at(format!("{} 0 obj\n{authored}\nendobj\n", id.num).as_bytes(), 0).unwrap().1,
            };
            PdfIndirectObject { id, value }
        }).collect();
        let mut graph = GraphSource::new(&graph);
        let trailer = [PdfDictEntry::new("Root", PdfObject::Ref(reference(1)))];
        let mut cursor = DocumentLiftCursor::new();
        let traversal = CosGrant { work: 1, allocation_bytes: 64 * 1024, release_bytes: 64 * 1024 };
        let mut prepared = false;
        let mut fallback = false;
        for _ in 0..8192 {
            let turn = cursor.advance(&trailer, "1.7", &mut graph, DocumentLiftGrant { traversal, semantic: false }, false);
            assert!(turn.semantic.is_none());
            if let Some(content) = turn.content {
                assert!(content.work <= traversal.work);
                match content.event.unwrap() {
                    PageContentEvent::Complete => prepared = true,
                    PageContentEvent::Deferred(PageContentBarrier::ResourceContext) => fallback = true,
                    _ => {}
                }
            }
            if let DocumentLiftStatus::NeedsSemantic(action) = turn.status {
                if action == (DocumentLiftAction::ProjectPage { index: 0 }) { assert!(prepared); }
                let receipt = cursor.advance(&trailer, "1.7", &mut graph, DocumentLiftGrant { traversal: CosGrant::default(), semantic: true }, false);
                assert_eq!(receipt.semantic, Some(action));
                assert!(receipt.content.is_none());
                if receipt.status == DocumentLiftStatus::Complete {
                    let state = cursor.take_result().unwrap();
                    let page = &state.snapshot.pages[0];
                    assert!(fallback);
                    assert!(page.content.iter().any(|op| matches!(op, PdfOp::MoveTo { x: 20.0, y: 10.0 })));
                    assert!(state.snapshot.admitted_stream_roles.iter().any(|role| matches!(&role.value, PdfStreamRoleValue::Operators { content } if content == &page.content)));
                    assert!(state.snapshot.pages[1].content.iter().any(|op| matches!(op, PdfOp::ShowText { text: PdfTextString::Text { text } } if text == "Inner")));
                    return;
                }
            }
        }
        panic!("document did not assemble both vector and font-dependent pages");
    }
}

#[cfg(test)]
mod xref_index_contracts {
    use crate::standards::v1_7::subsets::base::modules::{lexer::CosGrant, xref::{XrefEntry, XrefIndex, XrefIndexScan, XrefIndexStatus, XrefRow}};

    #[test]
    fn sparse_newest_entries_survive_growth_and_older_rows_cannot_revive_tombstones() {
        let mut index = XrefIndex::default();
        let mut allocated = 0;
        let mut released = 0;
        let funded = CosGrant { work: 1, allocation_bytes: 64 * 1024, release_bytes: 64 * 1024 };
        for (number, freed) in [(3, true), (3, false)].into_iter().chain((10..150).map(|i| (i * 101, false))) {
            let entry = (!freed).then_some(XrefEntry::Normal { offset: number as usize * 20, gen: 0 });
            assert!(index.offer(XrefRow { number, entry }));
            loop {
                let turn = index.advance(funded, false);
                assert!(turn.work <= 1);
                allocated += turn.allocated_bytes;
                released += turn.released_bytes;
                match turn.status.unwrap() {
                    XrefIndexStatus::More => {}
                    XrefIndexStatus::Inserted => break,
                    XrefIndexStatus::AlreadyKnown => { assert_eq!(number, 3); break }
                    _ => panic!("fully funded xref insertion"),
                }
            }
        }
        let mut scan = XrefIndexScan::default();
        let mut total_rows = 0;
        loop {
            let (work, complete, row) = scan.advance(&index, 1);
            assert!(work <= 1);
            total_rows += usize::from(row.is_some());
            if complete { break; }
        }
        assert_eq!(total_rows, 141);
        assert!(index.is_free(3));
        assert!(!index.contains_key(&3));
        assert_eq!(index.len(), 140);
        assert_eq!(index.get(&14_140), Some(&XrefEntry::Normal { offset: 282_800, gen: 0 }));
        index.begin_close();
        let turn = index.close_step(funded);
        assert_eq!(turn.status, Ok(XrefIndexStatus::Closed));
        assert_eq!(released + turn.released_bytes, allocated);
    }

    #[test]
    fn cancel_during_migration_closes_old_and_new_tables_separately() {
        let mut index = XrefIndex::default();
        let funded = CosGrant { work: 1, allocation_bytes: 8192, release_bytes: 8192 };
        for number in 0..4 { index.insert_row(XrefRow { number, entry: None }, false).unwrap(); }
        assert!(index.offer(XrefRow { number: 4, entry: None }));
        let allocated = (0..8).any(|_| index.advance(funded, false).allocated_bytes > 0);
        assert!(allocated);
        assert_eq!(index.advance(funded, true).status, Ok(XrefIndexStatus::Cancelled));
        let denied = CosGrant { release_bytes: 0, ..funded };
        assert!(matches!(index.close_step(denied).status, Ok(XrefIndexStatus::NeedsRelease { .. })));
        assert_eq!(index.close_step(funded).status, Ok(XrefIndexStatus::More));
        assert_eq!(index.close_step(funded).status, Ok(XrefIndexStatus::Closed));
    }
}
