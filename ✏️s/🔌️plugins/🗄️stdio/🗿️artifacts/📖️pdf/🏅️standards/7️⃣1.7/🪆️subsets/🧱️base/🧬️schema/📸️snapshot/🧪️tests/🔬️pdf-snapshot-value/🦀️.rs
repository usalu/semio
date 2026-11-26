#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::stream_roles::PdfAdmittedStreamRole;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use pack::value::{DslValue, FromValue, ToValue};

fn populated_snapshot() -> PdfSnapshot {
    let root = ObjRef { num: 1, gen: 0 };
    let mut page = PdfPage::new(612.0, 792.0);
    page.crop_box = Some([12.0, 12.0, 600.0, 780.0]);
    page.rotate = 90;
    page.content = vec![PdfOp::BeginText, PdfOp::SetFont { name: "F1".into(), size: 12.0 }, PdfOp::MoveText { tx: 72.0, ty: 720.0 }, PdfOp::ShowText { text: PdfTextString::text("Semio") }, PdfOp::EndText, PdfOp::Rectangle { x: 1.0, y: 2.0, width: 3.0, height: 4.5 }, PdfOp::Fill].into();
    page.annotations.push(PdfAnnotation::link([0.0, 0.0, 10.0, 10.0], "https://semio-tech.com"));
    PdfSnapshot {
        pages: vec![page],
        fonts: vec![PdfFont::standard("F1", "Helvetica")],
        images: vec![PdfImage::gray8("Im1", 2, 1, vec![0, 255])],
        outlines: vec![PdfOutlineItem::to_page("Start", 0)],
        info: PdfInfo { title: Some("Value path".to_string()), producer: Some("semio".to_string()), creation_date: Some(PdfDate { year: 2026, month: 9, day: 18, hour: 12, minute: 0, second: 0, offset_minutes: Some(120) }), ..PdfInfo::default() },
        objects: vec![PdfIndirectObject { id: root, value: PdfObject::Name("Catalog".to_string()) }],
        trailer: vec![PdfDictEntry { key: "Root".to_string(), value: PdfObject::Ref(root) }],
        ..PdfSnapshot::default()
    }
}

#[test]
fn populated_snapshot_round_trips_through_value() {
    let snapshot = populated_snapshot();
    assert_eq!(PdfSnapshot::from_value(snapshot.to_value()), Ok(snapshot));
}

#[test]
fn owned_reference_snapshot_reuses_document_id_storage() {
    use semio_framework_dsl_record::FieldValue;
    let snapshot = populated_snapshot();
    let mut record = crate::standards::v1_7::subsets::base::io::text::snapshot::to_record(&snapshot);
    let bytes = vec![0, 20, 127, 255];
    let backing = bytes.as_ptr();
    record.fields.insert(26, FieldValue::Value(DslValue::Array(vec![DslValue::Bytes(bytes), DslValue::Bytes(vec![99])])));
    let snapshot = crate::standards::v1_7::subsets::base::io::text::snapshot::from_record(record).unwrap();
    let id = snapshot.document_id.unwrap();
    assert_eq!(id, [vec![0, 20, 127, 255], vec![99]]);
    assert_eq!(id[0].as_ptr(), backing);
}

#[test]
fn owned_cos_stream_pages_decoded_octets() {
    let bytes = vec![0, 1, 127, 255];
    let value = DslValue::Object(vec![
        ("kind".into(), DslValue::String("stream".into())),
        ("dict".into(), DslValue::Array(Vec::new())),
        ("data".into(), DslValue::Bytes(bytes)),
        ("filters".into(), DslValue::Array(Vec::new())),
    ]);
    let PdfObject::Stream { data, .. } = PdfObject::from_value(value).unwrap() else { panic!("expected stream") };
    assert_eq!(data.to_vec_owner(), [0, 1, 127, 255]);
    assert_eq!(data.retained_bytes().len(), 4);
}

#[test]
fn missing_optional_fields_keep_the_derived_defaults() {
    let value = DslValue::object([("schema".to_string(), DslValue::String("stdio.pdf.1.7".to_string()))]);
    assert_eq!(PdfSnapshot::from_value(value), Ok(PdfSnapshot { declared_version: String::new(), ..PdfSnapshot::default() }));
}

#[test]
fn missing_schema_reports_the_exact_field() {
    let error = PdfSnapshot::from_value(DslValue::object([])).unwrap_err();
    assert_eq!(error.to_string(), "missing field `schema`");
}

#[test]
fn camel_case_json_shape_agrees_with_serde_json_oracle() {
    let snapshot = populated_snapshot();
    let actual: serde_json::Value = snapshot.to_value().into();
    assert_eq!(actual["pages"][0]["mediaBox"], serde_json::json!([0.0, 0.0, 612.0, 792.0]));
    assert_eq!(actual["pages"][0]["content"][1], serde_json::json!({"op": "setFont", "name": "F1", "size": 12.0}));
    assert_eq!(actual["pages"][0]["content"][3], serde_json::json!({"op": "showText", "text": {"kind": "text", "text": "Semio"}}));
    assert_eq!(actual["fonts"][0]["kind"]["kind"], serde_json::json!("type1"));
    assert_eq!(actual["fonts"][0]["kind"]["encoding"]["base"], serde_json::json!("winAnsi"));
    assert_eq!(actual["info"]["creationDate"]["offsetMinutes"], serde_json::json!(120));
    assert_eq!(actual["pages"][0]["annotations"][0]["kind"]["action"]["kind"], serde_json::json!({"kind": "uri", "uri": "https://semio-tech.com", "isMap": false}));
}

#[test]
fn page_text_joins_shown_runs() {
    let snapshot = populated_snapshot();
    assert_eq!(snapshot.pages[0].text(), "Semio");
}


#[test]
fn fresh_ids_avoid_every_collection() {
    let snapshot = populated_snapshot();
    assert_eq!(snapshot.fresh_id("F"), "F2");
    assert_eq!(snapshot.fresh_id("Im"), "Im2");
}

#[test]
fn ordinary_and_controlled_initial_record_pack_body_diagnostic() {
    use pack::record as pack_rt;
    let owner = <crate::editor::pdf17::Pdf17Editor as semio_plugin::ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let original_spec = crate::standards::v1_7::subsets::base::io::text::snapshot::spec();
    let original_record = crate::standards::v1_7::subsets::base::io::text::snapshot::to_record(&owner);
    let maximum = semio_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default().max_allocation_bytes;
    let mut observer = |_| true;
    let mut native = semio_framework_value::NativeEncodeControl::new(maximum, &mut observer);
    let paid_spec = crate::standards::v1_7::subsets::base::io::text::snapshot::spec_producer().encode(&mut native).unwrap();
    let paid_record = crate::standards::v1_7::subsets::base::io::text::snapshot::to_record_controlled(&owner, &mut native).unwrap();
    let options = pack_rt::EncodeOptions::default();
    let original = pack_rt::encode_document(&original_spec, &original_record, &options).unwrap();
    let record_join = pack_rt::encode_document(&original_spec, &paid_record, &options).unwrap();
    let spec_join = pack_rt::encode_document(&paid_spec, &original_record, &options).unwrap();
    let controlled = pack_rt::encode_document_controlled(&paid_spec, &paid_record, &options, &mut native).unwrap();
    for (label,bytes) in [("record", &record_join),("spec", &spec_join),("controlled", &controlled)] {
        let first = original.iter().zip(bytes.iter()).position(|(left,right)|left != right);
    }
    let original_decoded = pack_rt::decode_document(&original,&original_spec,&pack_rt::DecodeOptions::default()).unwrap().0;
    let controlled_decoded = pack_rt::decode_document(&controlled,&original_spec,&pack_rt::DecodeOptions::default()).unwrap().0;
    assert_eq!(original_record,paid_record,"real initial owner RecordValue changed");
    assert_eq!(original,record_join,"paid RecordValue changes ordinary body");
    assert_eq!(original,spec_join,"paid RecordSpec changes ordinary body");
    assert_eq!(original,controlled,"controlled encoder changes exact body");
    let shipped = semio_os_kernel::pack_rt::encode_document(&original_spec,&original_record,&semio_os_kernel::PackEncodeOptions::default()).unwrap();
    assert_eq!(shipped,original,"actual shipped ArtifactPack runtime must use the same intrinsic Record authority as SQLite");
}

#[test]
fn owned_pdf_snapshot_borrowed_projection_matches_reference_pack_fields() {
    use semio_framework_dsl_record::{FieldValue, native_encoding::{FieldProjectionView, RetainedFieldProjection}};
    use semio_framework_value::ByteReadSource;
    use store::os_store::serialization::ArtifactStoreSnapshot;
    use crate::standards::v1_7::subsets::base::io::{import::decode_pdf, text::snapshot as snapshot_text};

    let pdf = include_bytes!("../../../../🔨️modules/🗜️filters/🪣️stream-data/🧪️tests/🔬️unit/🧪️streams.pdf");
    let mut snapshot = decode_pdf(pdf).unwrap();
    snapshot.document_id = Some([vec![0, 255, 9], vec![]]);
    let stream = snapshot.objects.iter().position(|object| object.id.num == 4).unwrap();
    let PdfObject::Stream { data, .. } = &snapshot.objects[stream].value else { panic!("expected content stream") };
    let view = snapshot.native_pack_source().unwrap();
    let FieldProjectionView::IntrinsicBytesSource(original) = view.source.projection_view(&[29, stream, 1, 2]).unwrap() else { panic!("expected original paged octets") };
    let mut copied = vec![0u8; data.len()];
    assert_eq!((original.byte_len(), original.read_bytes(0, &mut copied)), (data.len(), data.len()));
    assert_eq!(copied, data.to_vec_owner());

    let expected = snapshot_text::to_record(&snapshot);
    let mut projection = RetainedFieldProjection::new(view.source);
    let mut accept = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(1024 * 1024, &mut accept);
    let mut actual = None;
    for _ in 0..200_000 {
        actual = projection.step(view.source, 1, &mut control).unwrap();
        if actual.is_some() { break; }
    }
    let FieldValue::Record(actual) = actual.expect("projection finished") else { panic!("expected Pack record") };
    assert_eq!(actual, expected);
    let expected_bytes = store::pack_rt::encode_document(&snapshot_text::spec(), &expected, &Default::default()).unwrap();
    let actual_bytes = store::pack_rt::encode_document(&snapshot_text::spec(), &actual, &Default::default()).unwrap();
    assert_eq!(actual_bytes, expected_bytes);
}

#[test]
fn genuine_pdf_paged_snapshot_export_matches_reference_pack_and_digest() {
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
    use semio_plugin::artifact_app_laws::OriginalWorkFixture;
    use store::os_store::serialization::{ArtifactStoreSnapshot,OriginalSnapshotPackExport as Export};

    let pdf = include_bytes!("../../../../🔨️modules/🗜️filters/🪣️stream-data/🧪️tests/🔬️unit/🧪️streams.pdf");
    let mut snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(pdf).unwrap();
    snapshot.document_id = Some([vec![0,255,9],Vec::new()]);
    let view = || snapshot.native_pack_source().unwrap();
    let original_identity = view().source.projection_identity();
    let reference = (PdfSnapshot::reference_snapshot_codec().unwrap().encode)(&snapshot).unwrap();
    const MAX_TURNS: usize = 2_000_000;
    let stock = RetainedCloneGrant { maximum_items: 2 * MAX_TURNS, ..OriginalWorkFixture::stock() };
    let mut caller = OriginalWorkFixture::with_source(
        semio_plugin::__job::OperationId(991), semio_plugin::__job::Generation(17),
        stock, || Some(1), u64::MAX, 4 * MAX_TURNS as u64,
    );
    let mut decoder_ready = false;
    for _ in 0..semio_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        decoder_ready = caller.turn(stock, |cx| {
            let mut outcome = None;
            if !cx.with_native_receiving(&mut outcome, |_, _| Ok(()))? { return Ok(false) }
            let outcome = outcome.expect("snapshot export receives its original decoder");
            assert!(outcome.source_refusal.is_none() && outcome.call.is_ok());
            cx.settle_receiving_scope()?;
            Ok(cx.native_receiving_identity().is_some())
        }).unwrap();
        if decoder_ready { break }
    }
    assert!(decoder_ready, "snapshot export requires a settled decoder");
    let grant = RetainedCloneGrant { maximum_items: 1, ..stock };
    let mut export = None;
    assert!(caller.turn(grant,|cx|Export::admit(view(),17,reference.len(),cx,&mut export)).unwrap());

    for turn in 0..MAX_TURNS {
        let exporter = export.as_mut().unwrap();
        if exporter.ready() { break; }
        let demand = exporter.advance_demands().unwrap();
        caller.turn(grant,|cx| {
            let loan = cx.retained_grant();
            let fuel = cx.fuel_remaining();
            let before = cx.retained_progress();
            let ready = exporter.advance(view(),17,cx)?;
            assert!(ready || cx.retained_progress() != before || cx.fuel_remaining() < fuel,
                "PDF Pack export stalled on turn {turn} with {fuel} fuel, demand {demand:?}, and loan {loan:?}");
            Ok(())
        }).unwrap();
    }
    assert_eq!(view().source.projection_identity(),original_identity);
    let exporter = export.as_mut().unwrap();
    assert!(exporter.ready(),"genuine PDF export did not reach its sealed paged result");
    let mut output = None;
    assert!(caller.turn(grant,|cx|exporter.take(cx,&mut output)).unwrap());
    assert!(exporter.terminal_is_empty());
    let output = output.unwrap();
    assert_eq!(output.bytes().to_vec_owner(),reference);
    assert_eq!(output.digest(),*semio_framework_hash::hash(&reference).as_bytes());
    caller.retire_owned(output,grant);
    caller.turn(grant,|cx|{
        let demand=Export::boxed_release_demands();
        semio_plugin::__job::close_original_retained_step(cx,demand,|_|{
            drop(export.take());
            RetainedWorkOutcome{
                work:1,
                result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})),
            }
        })?;
        Ok(())
    }).unwrap();
    assert!(export.is_none());
    caller.finish();
}

#[test]
fn controlled_annotations_match_cold_lift_and_retain_partial_pages() {
    use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::{
        io::text::snapshot::retained_text::admit_retained_text,
        modules::{
            lexer::{parse_indirect_at, CosGrant},
            lift::{lift_document, ControlledLiftBarrier, ControlledLiftEvent, DocumentLiftCursor},
        },
        schema::graph_source::GraphSource,
    };
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
                    assert_eq!(receipt.allocated_bytes, 0);
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

impl semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::schema::graph_source::ObjectSource for BorrowedAnnotationGraph {
    fn get(&mut self, _: ObjRef) -> Option<PdfObject> { panic!("controlled annotations must borrow PDF originals") }

    fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
        read(self.0.iter().find(|object| object.id == reference).map(|object| &object.value));
    }
}

#[test]
fn controlled_shapes_match_the_independent_reader_and_the_cold_lift() {
    use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::{
        io::text::snapshot::retained_text::admit_retained_text,
        modules::{
            lexer::{parse_indirect_at, CosGrant},
            lift::{lift_document, ControlledLiftEvent, DocumentLiftCursor},
        },
        schema::graph_source::GraphSource,
    };
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/📌️controlled-shapes/🔣️.json")).unwrap();
    for (shape, case) in fixture.as_object().unwrap() {
        let mut objects: Vec<_> = case["objects"].as_array().unwrap().iter().map(|row| {
            let authored = format!("{} {} obj\n{}\nendobj\n", row[0], row[1], row[2].as_str().unwrap());
            let (id, value) = parse_indirect_at(authored.as_bytes(), 0).unwrap();
            PdfIndirectObject { id, value }
        }).collect();
        let mut root = [PdfDictEntry::new("Root", PdfObject::Ref(objects[0].id))];
        let count = objects.len();
        admit_retained_text(&mut objects, &mut root, |id| ((id.num as usize) <= count).then(|| id.num as usize - 1)).unwrap();
        let cold = lift_document(&root, "1.7", &mut GraphSource::new(&objects)).unwrap();
        let expected = &case["projection"]["pages"];
        assert_eq!(cold.pages.len(), expected.as_array().unwrap().len(), "{shape}: page count");
        for (page, reference) in cold.pages.iter().zip(expected.as_array().unwrap()) {
            let media: Vec<f64> = reference["mediaBox"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect();
            assert_eq!(page.media_box.to_vec(), media, "{shape}: media box");
            assert_eq!(i64::from(page.rotate), reference["rotate"].as_i64().unwrap(), "{shape}: rotation");
            assert_eq!(page.annotations.len(), reference["annotations"].as_array().unwrap().len(), "{shape}: annotations");
        }
        let mut graph = BorrowedAnnotationGraph(objects.clone());
        let mut cursor = DocumentLiftCursor::controlled();
        let mut grant = CosGrant { work: 1, ..Default::default() };
        let mut finished = false;
        for _ in 0..8192 {
            let receipt = cursor.advance(&root, "1.7", &mut graph, grant, false);
            match receipt.event.unwrap() {
                ControlledLiftEvent::NeedsAllocation { bytes } => grant.allocation_bytes = bytes,
                ControlledLiftEvent::NeedsRelease { bytes } => grant.release_bytes = bytes,
                ControlledLiftEvent::More => { grant.allocation_bytes = 0; grant.release_bytes = 0; }
                ControlledLiftEvent::Complete => { assert_eq!(cursor.take_result().unwrap(), cold, "{shape}: controlled lift differs from the cold lift"); finished = true; break; }
                ControlledLiftEvent::Deferred(barrier) => panic!("{shape}: the governed lifter still defers at {barrier:?}"),
                event => panic!("{shape}: unexpected controlled event {event:?}"),
            }
        }
        assert!(finished, "{shape}: the controlled lift did not finish");
    }
}

#[test]
fn paged_page_program_keeps_the_wire_shape_of_the_vector_it_replaced() {
    let operators: Vec<PdfOp> = populated_snapshot().pages[0].content.iter().cloned().collect();
    let paged: PdfContent = operators.iter().cloned().collect();
    assert_eq!(paged.to_value(), operators.to_value());
    let (paged_json, vector_json): (serde_json::Value, serde_json::Value) = (paged.to_value().into(), operators.to_value().into());
    assert_eq!(paged_json, vector_json);
    assert!(paged_json.as_array().is_some_and(|rows| rows.len() == operators.len()));
    assert_eq!(PdfContent::from_value(operators.to_value()), Ok(paged));
}

/// 📜️ Retiring a page program of N operators never needs one release larger than one page (fixture `cases`), and every page it released was acquired.
#[test]
fn retiring_a_page_program_never_needs_a_release_larger_than_one_page() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let page_bytes = fixture["pageBytes"].as_u64().unwrap() as usize;
    let per_page = page_bytes / std::mem::size_of::<PdfOp>();
    for case in fixture["cases"].as_array().unwrap() {
        let count = case["operators"].as_u64().unwrap() as usize;
        let mut program: PdfContent = (0..count).map(|index| PdfOp::MoveTo { x: index as f64, y: 0.0 }).collect();
        assert_eq!(program.len(), count);
        let acquired = program.allocated_bytes();
        let payload_pages = count.div_ceil(per_page);
        assert!(acquired >= payload_pages * per_page * std::mem::size_of::<PdfOp>().min(page_bytes));
        let (mut released, mut largest) = (0, 0);
        while program.pop().is_some() {}
        while !program.terminal_is_empty() {
            let quote = program.next_release_allocation_bytes().unwrap();
            largest = largest.max(quote);
            assert!(quote <= page_bytes, "a release of {quote} bytes exceeds one {page_bytes}-byte page for {count} operators");
            released += program.release_empty_page_leased(page_bytes).unwrap().released_allocation_bytes;
        }
        assert_eq!(released, acquired);
        assert!(count == 0 || largest > 0);
    }
}
