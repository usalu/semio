use crate::standards::v_ascii::subsets::any::schema::mutations::StlMutation;
use crate::standards::v_ascii::subsets::any::schema::snapshot::StlSnapshot;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
use semio_s_artifact_stdio_contract::editing;
use crate::apply_mutation;
use crate::editor::stl::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_stl_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, STL_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<StlAnyEditor as ArtifactEditor>::DIALECT, STL_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<StlAnyEditor, crate::viewer::stl::StlAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", crate::editor::stl::StlAnyEditor, || semio_framework_plugin::App { definition: crate::editor::stl::create_stl_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️ascii/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use crate::standards::v_ascii::subsets::any::schema::snapshot::StlTriangle;
    let base = StlSnapshot { triangles: vec![StlTriangle::default(), StlTriangle::default()], ..StlSnapshot::default() };
    let emit = |event: editing::SnapshotEditEvent| <StlAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let renamed = emit(editing::SnapshotEditEvent::SetValue { path: "/solidName".into(), value: semio_framework_value::DslValue::String("part".into()) }).expect("a name edit resolves");
    assert!(matches!(renamed.artifact_mutations.as_slice(), [StlMutation::SetSolidName(_)]));
    let nudged = emit(editing::SnapshotEditEvent::SetValue { path: "/triangles/1/vertices/0/2".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(2.5)) }).expect("a vertex edit resolves");
    let [mutation @ StlMutation::SetTriangleVertices(_)] = nudged.artifact_mutations.as_slice() else { panic!("a vertex edit raises the vertices kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.triangles[1].vertices[0][2], 2.5);
    let whole = emit(editing::SnapshotEditEvent::SetValue { path: "/triangles/0".into(), value: semio_framework_value::ToValue::to_value(&StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[1.0, 0.0, 0.0]; 3] }) }).expect("a whole-triangle set resolves");
    assert!(matches!(whole.artifact_mutations.as_slice(), [StlMutation::SetTriangleNormal(_), StlMutation::SetTriangleVertices(_)]));
    let unnamed = emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("the schema stamp has no kind");
    assert_eq!(unnamed.code.0, "snapshot-edit.unsupported-path");
}

#[semio_framework_async_macros::async_test]
async fn a_transform_gesture_moves_every_triangle_and_rotates_its_normal() {
    use crate::standards::v_ascii::subsets::any::schema::snapshot::StlTriangle;
    let base = StlSnapshot { solid_name: "part".into(), triangles: vec![StlTriangle { normal: [1.0, 0.0, 0.0], vertices: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }], ..StlSnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[0,0,5],"scale":[2,2,2]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <StlAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect("the transform resolves");
    let mut state = base.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut state, mutation); });
    assert_eq!(state.triangles[0].vertices, [[0.0, 0.0, 5.0], [2.0, 0.0, 5.0], [0.0, 2.0, 5.0]]);
    assert_eq!(state.triangles[0].normal, [1.0, 0.0, 0.0]);
    assert_eq!(state.solid_name, "part");
}

#[test]
fn natural_file_codec_declares_the_exact_kind_extension_and_media_type() {
    let codec = <StlAnyEditor as ArtifactEditor>::natural_file_codec().expect("natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.stl@ascii", ".stl", "model/stl", false));
}

#[test]
fn natural_stl_round_trips_edits_agrees_with_an_independent_text_reader_and_reads_binary_files() {
    let source = include_bytes!("../🧫️fixtures/📂️natural/sample.stl");
    let snapshot = <StlAnyEditor as ArtifactEditor>::decode_natural_file(source).expect("stl opens");
    assert_eq!((snapshot.solid_name.as_str(), snapshot.triangles.len()), ("part", 1));
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[5,0,0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <StlAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut edited, mutation); });
    let saved = <StlAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("stl saves");
    let numbers = |line: &str| line.split_whitespace().skip(1).map(|token| token.parse::<f64>().expect("a decimal coordinate")).collect::<Vec<f64>>();
    let text = String::from_utf8(saved.clone()).expect("ascii stl is text");
    let independent: Vec<Vec<f64>> = text.lines().map(str::trim).filter(|line| line.starts_with("vertex ")).map(numbers).collect();
    assert_eq!(independent, vec![vec![5.0, 0.0, 0.0], vec![6.0, 0.0, 0.0], vec![5.0, 1.0, 0.0]]);
    assert!(text.starts_with("solid part") && text.trim_end().ends_with("endsolid part"));
    let reopened = <StlAnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("the saved file reopens");
    assert_eq!(reopened.triangles[0].vertices, [[5.0, 0.0, 0.0], [6.0, 0.0, 0.0], [5.0, 1.0, 0.0]]);
    let binary = semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::encode_stl_binary(&edited);
    assert_eq!(<StlAnyEditor as ArtifactEditor>::decode_natural_file(&binary).expect("binary stl opens").triangles[0].vertices, reopened.triangles[0].vertices);
    assert!(<StlAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/invalid.stl")).is_err());
}

#[test]
fn the_scene_draws_the_files_own_triangles() {
    let snapshot = <StlAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.stl")).expect("stl opens");
    let (positions, triangles, markers) = semio_s_artifact_stdio_stl_inferences::schema::inferences::scene_geometry(&snapshot);
    assert_eq!((positions, triangles, markers.len()), (vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], vec![0, 1, 2], 0));
}

#[semio_framework_async_macros::async_test]
async fn a_details_edit_of_one_vertex_coordinate_changes_only_that_vertex() {
    let snapshot = <StlAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.stl")).expect("stl opens");
    let value = semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(7.5));
    let event = editing::SnapshotEditEvent::SetValue { path: "/triangles/0/vertices/1/2".into(), value };
    let emit = <StlAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("the coordinate edit resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut edited, mutation); });
    assert_eq!(edited.triangles[0].vertices, [[0.0, 0.0, 0.0], [1.0, 0.0, 7.5], [0.0, 1.0, 0.0]]);
    assert_eq!(edited.triangles[0].normal, snapshot.triangles[0].normal);
    assert_eq!(edited.solid_name, snapshot.solid_name);
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
fn natural_stl_decodes_to_what_the_independent_reference_parser_read() {
    let expected = reference();
    let snapshot = <StlAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.stl")).expect("stl opens");
    assert_eq!(expected.get("solid").and_then(|solid| solid.as_str()), Some(snapshot.solid_name.as_str()));
    let triangles = expected.get("triangles").unwrap().as_array().unwrap();
    assert_eq!(snapshot.triangles.len(), triangles.len());
    snapshot.triangles.iter().zip(triangles).for_each(|(triangle, reference)| {
        assert_eq!(triangle.normal.to_vec(), numbers(reference.get("normal").unwrap()));
        assert_eq!(triangle.vertices.iter().map(|vertex| vertex.to_vec()).collect::<Vec<_>>(), rows(reference.get("vertices").unwrap()));
    });
}

#[semio_framework_async_macros::async_test]
async fn the_binary_encode_path_round_trips_through_the_natural_decoder() {
    let snapshot = <StlAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.stl")).expect("stl opens");
    let binary = semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::encode_stl_binary(&snapshot);
    assert_eq!(binary.len(), 84 + 50 * snapshot.triangles.len());
    assert_eq!(u32::from_le_bytes(binary[80..84].try_into().unwrap()) as usize, snapshot.triangles.len());
    let reopened = <StlAnyEditor as ArtifactEditor>::decode_natural_file(&binary).expect("binary stl reopens");
    assert_eq!(reopened.triangles, snapshot.triangles);
}

#[test]
fn stl_io_reads_the_saved_edit_and_the_binary_encoding_back_to_the_edited_model() {
    use semio_s_artifact_stdio_stl_test_oracle::standards::v_ascii::subsets::any::oracle_round_trip;
    let snapshot = <StlAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.stl")).expect("stl opens");
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[5,0,0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <StlAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| { apply_mutation(&mut edited, mutation); });
    for saved in [<StlAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("stl saves"), semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::encode_stl_binary(&edited)] {
        let by_stl_io = oracle_round_trip(&saved).expect("stl_io parses the saved file");
        let reopened = <StlAnyEditor as ArtifactEditor>::decode_natural_file(&by_stl_io).expect("the stl_io output reopens");
        assert_eq!(reopened.triangles.len(), edited.triangles.len());
        assert_eq!(reopened.triangles[0].vertices, [[5.0, 0.0, 0.0], [6.0, 0.0, 0.0], [5.0, 1.0, 0.0]]);
    }
}
