use crate::standards::v1_0::subsets::any::schema::mutations::LasMutation;
use crate::standards::v1_0::subsets::any::schema::snapshot::LasSnapshot;
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use semio_s_artifact_stdio_contract::editing;
use crate::apply_mutation;
use crate::editor::las::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_las_any_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, LAS_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<LasAnyEditor as ArtifactEditor>::DIALECT, LAS_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<LasAnyEditor, crate::viewer::las::LasAnyViewer>().await;
}

semio_plugin::history_edit_acceptance_law!("stdio", crate::editor::las::LasAnyEditor, || semio_plugin::App { definition: crate::editor::las::create_las_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.0/🪆️subsets/🎩️header");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use crate::standards::v1_0::subsets::any::schema::snapshot::{LasPoint, LasVlr};
    let base = LasSnapshot {
        vlrs: vec![LasVlr { user_id: "u".into(), record_id: 1, description: "d".into(), data: vec![1, 2] }],
        points: vec![LasPoint::default(), LasPoint::default()],
        ..LasSnapshot::default()
    };
    let emit = |event: editing::SnapshotEditEvent| <LasAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let renamed = emit(editing::SnapshotEditEvent::SetValue { path: "/header/systemIdentifier".into(), value: semio_framework_value::DslValue::String("scanner".into()) }).expect("a header edit resolves");
    let [mutation @ LasMutation::SetSystemIdentifier(_)] = renamed.artifact_mutations.as_slice() else { panic!("a header field edit raises the kind of that group") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.header.system_identifier, "scanner");
    let scaled = emit(editing::SnapshotEditEvent::SetValue { path: "/header/yScale".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(0.5)) }).expect("a scale edit resolves");
    assert!(matches!(scaled.artifact_mutations.as_slice(), [LasMutation::SetScaleAndOffset(_)]));
    let structural = emit(editing::SnapshotEditEvent::SetValue { path: "/header/headerSize".into(), value: semio_framework_value::DslValue::uint(300) });
    assert!(structural.is_err(), "a structural header field has no kind");
    let moved = emit(editing::SnapshotEditEvent::SetValue { path: "/points/1/classification".into(), value: semio_framework_value::DslValue::uint(2) }).expect("a point field edit resolves");
    assert!(matches!(moved.artifact_mutations.as_slice(), [LasMutation::SetPoint(_)]));
    let data = emit(editing::SnapshotEditEvent::SetValue { path: "/vlrs/0/data".into(), value: semio_framework_value::ToValue::to_value(&vec![9u8]) }).expect("a VLR data edit resolves");
    assert!(matches!(data.artifact_mutations.as_slice(), [LasMutation::SetVlrData(_)]));
    let identity = emit(editing::SnapshotEditEvent::SetValue { path: "/vlrs/0/recordId".into(), value: semio_framework_value::DslValue::uint(7) }).expect("a VLR identity edit resolves");
    assert!(matches!(identity.artifact_mutations.as_slice(), [LasMutation::RemoveVlr(_), LasMutation::InsertVlr(_)]));
}

#[semio_framework_async_macros::async_test]
async fn a_transform_gesture_moves_points_updates_bounds_and_keeps_classification_and_units() {
    use crate::standards::v1_0::subsets::any::schema::snapshot::{LasHeader, LasPoint};
    let point = |x: f64, y: f64, z: f64, classification: u8| LasPoint { x, y, z, classification, intensity: 7, ..LasPoint::default() };
    let base = LasSnapshot { header: LasHeader { x_scale: 0.001, ..LasHeader::default() }, points: vec![point(0.0, 0.0, 0.0, 2), point(1.0, 2.0, 3.0, 5)], ..LasSnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[10,20,30]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <LasAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect("the transform resolves");
    let mut state = base.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut state, mutation); });
    assert_eq!((state.points[1].x, state.points[1].y, state.points[1].z), (11.0, 22.0, 33.0));
    assert_eq!((state.points[1].classification, state.points[1].intensity), (5, 7));
    assert_eq!((state.header.min_x, state.header.max_z), (10.0, 33.0));
    assert_eq!(state.header.x_scale, 0.001);
    assert!(<LasAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &LasSnapshot::default()).is_err());
}

#[test]
fn natural_file_codec_declares_the_exact_kind_extension_and_media_type() {
    let codec = <LasAnyEditor as ArtifactEditor>::natural_file_codec().expect("natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.las@1.0", ".las", "application/vnd.las", true));
}

#[test]
fn natural_las_round_trips_edits_agrees_with_an_independent_byte_reader_and_refuses_invalid_files() {
    use crate::standards::v1_0::subsets::any::schema::snapshot::LasPoint;
    let mut base = LasSnapshot::default();
    base.points = vec![LasPoint { x: 1.0, y: 2.0, z: 3.0, classification: 2, intensity: 9, ..LasPoint::default() }, LasPoint { x: 4.0, y: 5.0, z: 6.0, classification: 5, ..LasPoint::default() }];
    base.header.number_of_point_records = 2;
    let bytes = <LasAnyEditor as ArtifactEditor>::encode_natural_file(&base).expect("las encodes");
    let snapshot = <LasAnyEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("las opens");
    assert_eq!(snapshot.points.len(), 2);
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[10,0,0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <LasAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut edited, mutation); });
    let saved = <LasAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("las saves");
    let le32 = |at: usize| u32::from_le_bytes(saved[at..at + 4].try_into().unwrap());
    let le_f64 = |at: usize| f64::from_le_bytes(saved[at..at + 8].try_into().unwrap());
    assert_eq!(&saved[0..4], b"LASF");
    let (offset_to_points, records, scale_x, offset_x) = (le32(96) as usize, le32(107), le_f64(131), le_f64(155));
    assert_eq!(records, 2);
    let first_x = i32::from_le_bytes(saved[offset_to_points..offset_to_points + 4].try_into().unwrap());
    assert!((f64::from(first_x) * scale_x + offset_x - 11.0).abs() < scale_x);
    let reopened = <LasAnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    assert_eq!((reopened.points[0].x, reopened.points[1].classification, reopened.points[0].intensity), (11.0, 5, 9));
    assert_eq!(reopened.header.x_scale, snapshot.header.x_scale);
    assert!(<LasAnyEditor as ArtifactEditor>::decode_natural_file(b"not a las file").is_err());
    assert!(<LasAnyEditor as ArtifactEditor>::decode_natural_file(&bytes[..40]).is_err());
}

#[test]
fn the_scene_draws_every_point_as_a_marker() {
    use crate::standards::v1_0::subsets::any::schema::snapshot::LasPoint;
    let mut snapshot = LasSnapshot::default();
    snapshot.points = vec![LasPoint { x: 1.0, y: 2.0, z: 3.0, ..LasPoint::default() }];
    let (positions, triangles, markers) = semio_s_artifact_stdio_las_schema::schema::inferences::scene_geometry(&snapshot);
    assert_eq!((positions.len(), triangles.len(), markers), (0, 0, vec![[1.0, 2.0, 3.0]]));
}

fn reference() -> semio_framework_value::DslValue {
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(include_str!("../🧫️fixtures/📂️natural/expected.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the reference fixture parses"))
}

fn numbers(value: &semio_framework_value::DslValue) -> Vec<f64> {
    value.as_array().expect("an array").iter().map(|item| item.as_f64().expect("a number")).collect()
}

fn rows(value: &semio_framework_value::DslValue) -> Vec<Vec<f64>> {
    value.as_array().expect("an array").iter().map(numbers).collect()
}

#[test]
fn natural_las_decodes_a_file_written_by_the_independent_reference_writer() {
    let expected = reference();
    let snapshot = <LasAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.las")).expect("las opens");
    assert_eq!(vec![snapshot.header.x_scale, snapshot.header.y_scale, snapshot.header.z_scale], numbers(expected.get("scale").unwrap()));
    assert_eq!(vec![snapshot.header.x_offset, snapshot.header.y_offset, snapshot.header.z_offset], numbers(expected.get("offset").unwrap()));
    let points = expected.get("points").unwrap().as_array().unwrap();
    assert_eq!(snapshot.points.len(), points.len());
    snapshot.points.iter().zip(points).for_each(|(point, reference)| {
        let field = |key: &str| reference.get(key).and_then(|value| value.as_f64()).unwrap();
        assert!((point.x - field("x")).abs() < 1e-9 && (point.y - field("y")).abs() < 1e-9 && (point.z - field("z")).abs() < 1e-9);
        assert_eq!((f64::from(point.intensity), f64::from(point.classification), f64::from(point.point_source_id)), (field("intensity"), field("classification"), field("pointSourceId")));
    });
    let saved = <LasAnyEditor as ArtifactEditor>::encode_natural_file(&snapshot).expect("las saves");
    assert_eq!(<LasAnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("las reopens").points, snapshot.points);
}

#[test]
fn the_las_crate_reads_the_saved_edit_back_to_the_edited_model() {
    use semio_s_artifact_stdio_las_test_oracle::standards::v1_0::subsets::header::oracle_round_trip;
    let snapshot = <LasAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.las")).expect("las opens");
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[10,0,0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <LasAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut edited, mutation); });
    let saved = <LasAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("las saves");
    let by_las_crate = oracle_round_trip(&saved).expect("the las crate parses the saved file");
    let reopened = <LasAnyEditor as ArtifactEditor>::decode_natural_file(&by_las_crate).expect("the las crate output reopens");
    assert_eq!(reopened.points.len(), edited.points.len());
    reopened.points.iter().zip(&edited.points).for_each(|(read, written)| {
        assert!((read.x - written.x).abs() <= snapshot.header.x_scale && (read.y - written.y).abs() <= snapshot.header.y_scale && (read.z - written.z).abs() <= snapshot.header.z_scale);
        assert_eq!((read.classification, read.intensity, read.point_source_id), (written.classification, written.intensity, written.point_source_id));
    });
    assert!((reopened.points[0].x - (snapshot.points[0].x + 10.0)).abs() <= snapshot.header.x_scale);
}
