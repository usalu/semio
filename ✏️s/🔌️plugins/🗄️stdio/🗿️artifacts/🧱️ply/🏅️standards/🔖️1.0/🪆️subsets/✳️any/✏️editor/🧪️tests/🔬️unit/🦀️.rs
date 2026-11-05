use crate::standards::v1_0::subsets::any::schema::mutations::PlyMutation;
use crate::standards::v1_0::subsets::any::schema::snapshot::PlySnapshot;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
use semio_s_artifact_stdio_contract::editing;
use crate::apply_mutation;
use crate::editor::ply::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_ply_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PLY_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<PlyAnyEditor as ArtifactEditor>::DIALECT, PLY_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<PlyAnyEditor, crate::viewer::ply::PlyAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", crate::editor::ply::PlyAnyEditor, || semio_framework_plugin::App { definition: crate::editor::ply::create_ply_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.0/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use crate::standards::v1_0::subsets::any::schema::snapshot::{PlyElement, PlyProperty, PlyRow, PlyScalarType, PlyValue};
    let base = PlySnapshot {
        comments: vec!["first".into(), "second".into()],
        elements: vec![PlyElement { name: "vertex".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }], rows: vec![PlyRow { values: vec![PlyValue::Float(1.0)] }] }],
        ..PlySnapshot::default()
    };
    let emit = |event: editing::SnapshotEditEvent| <PlyAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let moved = emit(editing::SnapshotEditEvent::SetValue { path: "/elements/0/rows/0/values/0/value".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(2.5)) }).expect("a value edit resolves");
    let [mutation @ PlyMutation::SetRowProperty(_)] = moved.artifact_mutations.as_slice() else { panic!("a row value edit raises the row-property kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.elements[0].rows[0].values[0], PlyValue::Float(2.5));
    let reworded = emit(editing::SnapshotEditEvent::SetValue { path: "/comments/1".into(), value: semio_framework_value::DslValue::String("changed".into()) }).expect("a comment replacement resolves");
    let mut state = base.clone();
    reworded.artifact_mutations.iter().for_each(|mutation| {
        apply_mutation(&mut state, mutation);
    });
    assert_eq!(state.comments, vec!["first".to_string(), "changed".to_string()]);
    let dropped = emit(editing::SnapshotEditEvent::RemoveValue { path: "/elements/0/rows/0".into() }).expect("a row removal resolves");
    assert!(matches!(dropped.artifact_mutations.as_slice(), [PlyMutation::RemoveRow(_)]));
    let unnamed = emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("the schema stamp has no kind");
    assert_eq!(unnamed.code.0, "snapshot-edit.unsupported-path");
}

#[semio_framework_async_macros::async_test]
async fn a_transform_gesture_moves_vertex_columns_in_their_declared_type_and_keeps_other_columns() {
    use crate::standards::v1_0::subsets::any::schema::snapshot::{PlyElement, PlyProperty, PlyRow, PlyScalarType, PlyValue};
    let scalar = |name: &str, kind| PlyProperty::Scalar { name: name.into(), kind };
    let vertex = PlyElement { name: "vertex".into(), count: 1, properties: vec![scalar("x", PlyScalarType::Float), scalar("y", PlyScalarType::Float), scalar("z", PlyScalarType::Double), scalar("red", PlyScalarType::UChar)], rows: vec![PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(2.0), PlyValue::Double(3.0), PlyValue::UChar(200)] }] };
    let base = PlySnapshot { elements: vec![vertex], ..PlySnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[1,1,1],"scale":[2,2,2]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <PlyAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect("the transform resolves");
    assert_eq!(emit.artifact_mutations.len(), 3);
    let mut state = base.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut state, mutation); });
    assert_eq!(state.elements[0].rows[0].values, vec![PlyValue::Float(3.0), PlyValue::Float(5.0), PlyValue::Double(7.0), PlyValue::UChar(200)]);
    let refused = <PlyAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &PlySnapshot::default()).err().expect("no vertex element");
    assert!(refused.message.contains("vertex"));
}

#[test]
fn natural_file_codec_declares_the_exact_kind_extension_and_media_type() {
    let codec = <PlyAnyEditor as ArtifactEditor>::natural_file_codec().expect("natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.ply@1.0", ".ply", "model/ply", true));
}

#[test]
fn natural_ply_round_trips_edits_agrees_with_an_independent_text_reader_and_refuses_invalid_files() {
    let source = include_bytes!("../🧫️fixtures/📂️natural/sample.ply");
    let snapshot = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(source).expect("ply opens");
    assert_eq!(snapshot.elements.iter().map(|element| element.name.as_str()).collect::<Vec<_>>(), ["vertex", "face"]);
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"scale":[2,2,2]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <PlyAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut edited, mutation); });
    let saved = <PlyAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("ply saves");
    let text = String::from_utf8(saved.clone()).expect("an ascii ply is text");
    let body: Vec<&str> = text.lines().skip_while(|line| *line != "end_header").skip(1).collect();
    let second: Vec<f64> = body[1].split_whitespace().map(|token| token.parse().expect("a decimal coordinate")).collect();
    assert_eq!((body.len(), second), (4, vec![2.0, 0.0, 0.0]));
    assert!(text.starts_with("ply\nformat ascii 1.0\n") && text.contains("comment one triangle"));
    let reopened = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    assert_eq!(reopened.elements[0].rows[1], edited.elements[0].rows[1]);
    assert_eq!(edited.elements[0].rows[1].values[0], crate::standards::v1_0::subsets::any::schema::snapshot::PlyValue::Float(2.0));
    assert_eq!(reopened.comments, snapshot.comments);
    assert!(<PlyAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/invalid.ply")).is_err());
}

#[test]
fn the_scene_draws_triangles_and_falls_back_to_points_without_faces() {
    let snapshot = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.ply")).expect("ply opens");
    let (positions, triangles, markers) = semio_s_artifact_stdio_ply_inferences::schema::inferences::scene_geometry(&snapshot);
    assert_eq!((positions.len(), triangles, markers.len()), (3, vec![0, 1, 2], 0));
    let mut points = snapshot.clone();
    points.elements.retain(|element| element.name == "vertex");
    let (positions, triangles, markers) = semio_s_artifact_stdio_ply_inferences::schema::inferences::scene_geometry(&points);
    assert_eq!((positions.len(), triangles.len(), markers.len()), (0, 0, 3));
}

#[semio_framework_async_macros::async_test]
async fn a_details_edit_of_one_vertex_coordinate_changes_only_that_cell() {
    use crate::standards::v1_0::subsets::any::schema::snapshot::PlyValue;
    use semio_framework_value::ToValue;
    let snapshot = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.ply")).expect("ply opens");
    let event = editing::SnapshotEditEvent::SetValue { path: "/elements/0/rows/1/values/0".into(), value: PlyValue::Float(5.0).to_value() };
    let emit = <PlyAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("the coordinate edit resolves");
    assert_eq!(emit.artifact_mutations.len(), 1);
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut edited, mutation); });
    assert_eq!(edited.elements[0].rows[1].values, vec![PlyValue::Float(5.0), PlyValue::Float(0.0), PlyValue::Float(0.0)]);
    assert_eq!(edited.elements[0].rows[0], snapshot.elements[0].rows[0]);
    assert_eq!(edited.elements[1], snapshot.elements[1]);
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
fn natural_ply_decodes_to_what_the_independent_reference_parser_read() {
    use crate::standards::v1_0::subsets::any::schema::snapshot::PlyValue;
    let expected = reference();
    let snapshot = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.ply")).expect("ply opens");
    let float = |value: &PlyValue| match value { PlyValue::Float(float) => f64::from(*float), other => panic!("{other:?}") };
    assert_eq!(snapshot.elements[0].rows.iter().map(|row| row.values.iter().map(float).collect::<Vec<_>>()).collect::<Vec<_>>(), rows(expected.get("vertex").unwrap()));
    let faces: Vec<Vec<f64>> = snapshot.elements[1].rows.iter().map(|row| match &row.values[0] { PlyValue::List(items) => items.iter().map(|item| match item { PlyValue::Int(index) => f64::from(*index), other => panic!("{other:?}") }).collect(), other => panic!("{other:?}") }).collect();
    assert_eq!(faces, rows(expected.get("faces").unwrap()));
    assert_eq!(snapshot.comments, ["one triangle"]);
}

#[semio_framework_async_macros::async_test]
async fn the_binary_encode_path_round_trips_and_the_snapshot_keeps_the_opened_form() {
    use crate::standards::v1_0::subsets::any::schema::snapshot::PlyFormat;
    let ascii = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.ply")).expect("ply opens");
    assert_eq!(ascii.format, PlyFormat::Ascii);
    let binary = semio_s_artifact_stdio_ply_inferences::natural_file::encode_ply_with_format(&ascii, PlyFormat::BinaryLittleEndian).expect("binary ply encodes");
    assert!(binary.starts_with(b"ply\nformat binary_little_endian 1.0\n"));
    let reopened = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(&binary).expect("binary ply reopens");
    assert_eq!(reopened.format, PlyFormat::BinaryLittleEndian);
    assert_eq!(reopened.elements, ascii.elements);
    let saved_again = <PlyAnyEditor as ArtifactEditor>::encode_natural_file(&reopened).expect("the opened form is kept on save");
    assert!(saved_again.starts_with(b"ply\nformat binary_little_endian 1.0\n"));
}

#[test]
fn ply_rs_reads_the_saved_edit_back_to_the_edited_model_in_ascii_and_binary() {
    use crate::standards::v1_0::subsets::any::schema::snapshot::PlyFormat;
    use semio_s_artifact_stdio_ply_test_oracle::standards::v1_0::subsets::any::oracle_round_trip;
    let snapshot = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.ply")).expect("ply opens");
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"scale":[2,2,2]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <PlyAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut edited, mutation); });
    for saved in [<PlyAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("ply saves"), semio_s_artifact_stdio_ply_inferences::natural_file::encode_ply_with_format(&edited, PlyFormat::BinaryLittleEndian).expect("binary ply saves")] {
        let by_ply_rs = oracle_round_trip(&saved).expect("ply-rs parses the saved file");
        let reopened = <PlyAnyEditor as ArtifactEditor>::decode_natural_file(&by_ply_rs).expect("the ply-rs output reopens");
        assert_eq!(reopened.elements[0].rows, edited.elements[0].rows);
        assert_eq!(reopened.elements[1].rows, edited.elements[1].rows);
    }
}
