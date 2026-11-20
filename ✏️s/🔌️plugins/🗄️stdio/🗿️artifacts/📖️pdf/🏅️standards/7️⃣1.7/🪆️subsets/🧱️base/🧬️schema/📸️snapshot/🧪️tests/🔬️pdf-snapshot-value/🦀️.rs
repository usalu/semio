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
    page.content = vec![PdfOp::BeginText, PdfOp::SetFont { name: "F1".into(), size: 12.0 }, PdfOp::MoveText { tx: 72.0, ty: 720.0 }, PdfOp::ShowText { text: PdfTextString::text("Semio") }, PdfOp::EndText, PdfOp::Rectangle { x: 1.0, y: 2.0, width: 3.0, height: 4.5 }, PdfOp::Fill];
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
fn owned_cos_stream_reuses_decoded_octet_storage() {
    let bytes = vec![0, 1, 127, 255];
    let backing = bytes.as_ptr();
    let value = DslValue::Object(vec![
        ("kind".into(), DslValue::String("stream".into())),
        ("dict".into(), DslValue::Array(Vec::new())),
        ("data".into(), DslValue::Bytes(bytes)),
        ("filters".into(), DslValue::Array(Vec::new())),
    ]);
    let PdfObject::Stream { data, .. } = PdfObject::from_value(value).unwrap() else { panic!("expected stream") };
    assert_eq!(data, [0, 1, 127, 255]);
    assert_eq!(data.as_ptr(), backing);
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
    use store::os_store::serialization::ArtifactStoreSnapshot;
    use crate::standards::v1_7::subsets::base::io::{import::decode_pdf, text::snapshot as snapshot_text};

    let pdf = include_bytes!("../../../../🔨️modules/🗜️filters/🪣️stream-data/🧪️tests/🔬️unit/🧪️streams.pdf");
    let mut snapshot = decode_pdf(pdf).unwrap();
    snapshot.document_id = Some([vec![0, 255, 9], vec![]]);
    let stream = snapshot.objects.iter().position(|object| object.id.num == 4).unwrap();
    let PdfObject::Stream { data, .. } = &snapshot.objects[stream].value else { panic!("expected content stream") };
    let view = snapshot.native_pack_source().unwrap();
    let FieldProjectionView::IntrinsicBytes(original) = view.source.projection_view(&[29, stream, 1, 2]).unwrap() else { panic!("expected original octets") };
    assert_eq!(original.as_ptr(), data.as_ptr());
    assert_eq!(original, data);

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
    let mut caller = OriginalWorkFixture::new();
    let grant = RetainedCloneGrant { maximum_items: 1,..OriginalWorkFixture::stock() };
    let mut export = None;
    assert!(caller.turn(grant,|cx|Export::admit(view(),17,reference.len(),cx,&mut export)).unwrap());

    for _ in 0..1_000_000 {
        let exporter = export.as_mut().unwrap();
        if exporter.ready() { break; }
        caller.turn(grant,|cx|exporter.advance(view(),17,cx)).unwrap();
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
}
