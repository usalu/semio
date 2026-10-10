use super::*;
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

#[cfg(feature = "component-app-assembly")]
#[test]
fn ordinary_and_controlled_initial_record_pack_body_diagnostic() {
    use pack::record as pack_rt;
    let owner = <crate::editor::pdf17::Pdf17Editor as semio_framework_plugin::ArtifactEditor>::initial_snapshot();
    let original_spec = crate::standards::v1_7::subsets::base::io::text::snapshot::spec();
    let original_record = crate::standards::v1_7::subsets::base::io::text::snapshot::to_record(&owner);
    let maximum = semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default().max_allocation_bytes;
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
        eprintln!("[DEBUG] pdf17 body {label} original={} candidate={} first={first:?}",original.len(),bytes.len());
    }
    let original_decoded = pack_rt::decode_document(&original,&original_spec,&pack_rt::DecodeOptions::default()).unwrap().0;
    let controlled_decoded = pack_rt::decode_document(&controlled,&original_spec,&pack_rt::DecodeOptions::default()).unwrap().0;
    eprintln!("[DEBUG] pdf17 record equal={} decoded equal={} original={:?} controlled={:?}",original_record == paid_record,original_decoded == controlled_decoded,original_record,paid_record);
    assert_eq!(original_record,paid_record,"real initial owner RecordValue changed");
    assert_eq!(original,record_join,"paid RecordValue changes ordinary body");
    assert_eq!(original,spec_join,"paid RecordSpec changes ordinary body");
    assert_eq!(original,controlled,"controlled encoder changes exact body");
    let shipped = semio_framework_os_kernel::pack_rt::encode_document(&original_spec,&original_record,&semio_framework_os_kernel::PackEncodeOptions::default()).unwrap();
    eprintln!("[DEBUG] actual shipped Record authority original={} core={} first={:?}",shipped.len(),original.len(),shipped.iter().zip(&original).position(|(left,right)|left!=right));
    assert_eq!(shipped,original,"actual shipped ArtifactPack runtime must use the same intrinsic Record authority as SQLite");
}
