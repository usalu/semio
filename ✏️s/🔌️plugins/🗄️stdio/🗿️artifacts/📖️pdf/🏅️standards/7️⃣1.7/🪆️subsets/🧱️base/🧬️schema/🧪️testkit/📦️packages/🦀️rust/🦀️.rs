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
mod document_contracts {
    use crate::standards::v1_7::subsets::base::{
        io::text::snapshot::retained_text::admit_retained_text,
        modules::{
            lexer::{parse_indirect_at, CosGrant},
            lift::{lift_document, ControlledDocumentLiftCursor, ControlledLiftBarrier, ControlledLiftEvent,
                DocumentLiftAction, DocumentLiftCursor, DocumentLiftGrant, DocumentLiftStatus, PageContentBarrier, PageContentEvent},
        },
        schema::{
            graph_source::{GraphSource, ObjectSource}, snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfOp, PdfTextString},
            stream_roles::PdfStreamRoleValue,
        },
    };

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
            PdfObject::Stream { dict: Vec::new(), data: first.as_bytes().to_vec(), filters: Vec::new() },
            PdfObject::Stream { dict: Vec::new(), data: second.as_bytes().to_vec(), filters: Vec::new() },
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
    fn controlled_annotations_match_cold_lift_and_retain_partial_pages() {
        use crate::standards::v1_7::subsets::base::schema::snapshot::PdfAnnotation;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/📌️controlled-annotations/🔣️.json")).unwrap();
        let mut objects: Vec<_> = fixture["objects"].as_array().unwrap().iter().map(|row| {
            let authored = format!("{} {} obj\n{}\nendobj\n", row[0], row[1], row[2].as_str().unwrap());
            let (id, value) = parse_indirect_at(authored.as_bytes(), 0).unwrap();
            PdfIndirectObject { id, value }
        }).collect();
        let mut root = [PdfDictEntry::new("Root", PdfObject::Ref(objects[0].id))];
        admit_retained_text(&mut objects, &mut root, |id| (1..=8).contains(&id.num).then(|| id.num as usize - 1)).unwrap();
        let cold = lift_document(&root, "1.7", &mut GraphSource::new(&objects)).unwrap();
        assert_eq!(cold.pages.iter().map(|page| page.annotations.len()).collect::<Vec<_>>(), [2, 1]);
        assert_eq!(cold.pages[0].annotations[0].contents.as_deref(), fixture["expected"]["note"]["contents"].as_str());
        assert_eq!(cold.pages[0].annotations[0].name.as_deref(), fixture["expected"]["note"]["name"].as_str());

        for mode in ["complete", "cancel", "appearance"] {
            let mut graph = objects.clone();
            if mode == "appearance" {
                let PdfObject::Dict(entries) = &mut graph[5].value else { panic!("link dictionary"); };
                entries.push(PdfDictEntry::new("AP", PdfObject::Dict(Vec::new())));
            }
            let expected = graph.clone();
            let mut graph = BorrowedAnnotationGraph(graph);
            let mut cursor = DocumentLiftCursor::controlled();
            let mut grant = CosGrant { work: 1, ..Default::default() };
            let mut events = 0;
            let mut note_granted = false;
            let mut slots_granted = false;
            let mut ended = false;
            for _ in 0..2048 {
                let cancelled = mode == "cancel" && note_granted && grant.allocation_bytes == 0;
                let receipt = cursor.advance(&root, "1.7", &mut graph, grant, cancelled);
                assert!(receipt.work <= 1 && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
                match receipt.event.unwrap() {
                    ControlledLiftEvent::NeedsAllocation { bytes } => {
                        slots_granted |= bytes == 2 * std::mem::size_of::<PdfAnnotation>();
                        grant.allocation_bytes = bytes;
                        assert_eq!(receipt.work, 0);
                    }
                    ControlledLiftEvent::NeedsRelease { bytes } => grant.release_bytes = bytes,
                    ControlledLiftEvent::More => {
                        note_granted |= receipt.allocated_bytes == ["Review note", "review-1", "Comment", "Reviewer", "Plan"].iter().map(|s| s.len()).sum::<usize>() + 3 * std::mem::size_of::<f64>();
                        grant.allocation_bytes = 0;
                        grant.release_bytes = 0;
                    }
                    ControlledLiftEvent::Complete => {
                        assert_eq!(mode, "complete");
                        assert_eq!(cursor.take_result().unwrap(), cold);
                        ended = true;
                        break;
                    }
                    event @ (ControlledLiftEvent::Cancelled | ControlledLiftEvent::Deferred(ControlledLiftBarrier::Page)) => {
                        assert_eq!(event, if mode == "cancel" { ControlledLiftEvent::Cancelled } else { ControlledLiftEvent::Deferred(ControlledLiftBarrier::Page) });
                        assert!(cursor.take_result().is_none());
                        let inspected = graph.0.clone();
                        cursor.begin_close();
                        let close = CosGrant { work: 1, allocation_bytes: 0, release_bytes: usize::MAX };
                        while cursor.close_step(close).event != Ok(ControlledLiftEvent::Closed) {}
                        assert_eq!(graph.0, inspected);
                        let partial = cursor.take_partial();
                        assert!(partial.snapshot.unwrap().pages.is_empty());
                        assert_eq!(partial.page.unwrap().annotations, cold.pages[0].annotations[..1]);
                        ended = true;
                        break;
                    }
                    event => panic!("unexpected controlled annotation state {event:?}"),
                }
                events += receipt.work;
            }
            assert!(ended && slots_granted && note_granted && events > 10);
            assert_eq!(graph.0, expected);
        }
    }

    struct BorrowedAnnotationGraph(Vec<PdfIndirectObject>);

    impl ObjectSource for BorrowedAnnotationGraph {
        fn get(&mut self, _: ObjRef) -> Option<PdfObject> { panic!("controlled annotations must borrow PDF originals") }

        fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
            read(self.0.iter().find(|object| object.id == reference).map(|object| &object.value));
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
                6..=8 => PdfObject::Stream { dict: Vec::new(), data: [a, b, "BT /F1 12 Tf (Inner) Tj ET"][(id.num - 6) as usize].as_bytes().to_vec(), filters: Vec::new() },
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
