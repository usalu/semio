use crate::standards::v3_0::subsets::any::schema::mutations::ObjMutation;
use crate::standards::v3_0::subsets::any::schema::snapshot::ObjSnapshot;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
use semio_s_artifact_stdio_contract::editing;
use crate::apply_mutation;
use crate::editor::obj::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_obj_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, OBJ_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ObjAnyEditor as ArtifactEditor>::DIALECT, OBJ_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<ObjAnyEditor, crate::viewer::obj::ObjAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", crate::editor::obj::ObjAnyEditor, || semio_framework_plugin::App { definition: crate::editor::obj::create_obj_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️3.0/🪆️subsets/📐️geometry");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_row() {
    
    use crate::standards::v3_0::subsets::any::schema::snapshot::{ObjGroup, ObjVertex};
    let base = ObjSnapshot { vertices: vec![ObjVertex::default(), ObjVertex::default()], groups: vec![ObjGroup { name: "walls".into(), faces: vec![0] }], ..ObjSnapshot::default() };
    let emit = |event: editing::SnapshotEditEvent| <ObjAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let moved = emit(editing::SnapshotEditEvent::SetValue { path: "/vertices/1/x".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(2.0)) }).expect("a vertex edit resolves");
    let [mutation @ ObjMutation::SetVertex(_)] = moved.artifact_mutations.as_slice() else { panic!("a vertex coordinate raises the vertex kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.vertices[1].x, 2.0);
    let renamed = emit(editing::SnapshotEditEvent::SetValue { path: "/groups/0/name".into(), value: semio_framework_value::DslValue::String("floors".into()) }).expect("a group rename resolves");
    let mut state = base.clone();
    renamed.artifact_mutations.iter().for_each(|mutation| {
        apply_mutation(&mut state, mutation);
    });
    assert_eq!(state.groups[0].name, "floors");
    let dropped = emit(editing::SnapshotEditEvent::RemoveValue { path: "/vertices/0".into() }).expect("a vertex removal resolves");
    assert!(matches!(dropped.artifact_mutations.as_slice(), [ObjMutation::RemoveVertex(_)]));
    assert_eq!(emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("no kind").code.0, "snapshot-edit.unsupported-path");
}

#[semio_framework_async_macros::async_test]
async fn a_transform_gesture_moves_every_vertex_rotates_normals_and_keeps_the_rest() {
    use crate::standards::v3_0::subsets::any::schema::snapshot::{ObjFace, ObjFaceVertex, ObjNormal, ObjVertex};
    let face = ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: Some(0) }] };
    let base = ObjSnapshot { vertices: vec![ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: Some(1.0) }, ObjVertex { x: 0.0, y: 1.0, z: 0.0, w: None }], normals: vec![ObjNormal { x: 1.0, y: 0.0, z: 0.0 }], faces: vec![face.clone()], ..ObjSnapshot::default() };
    let gesture = |source: &str| editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <ObjAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&gesture(r#"{"translate":[10,0,0],"rotate":[0,0,90]}"#), &base).expect("the transform resolves");
    assert_eq!(emit.artifact_mutations.len(), 3);
    let mut state = base.clone();
    emit.artifact_mutations.iter().for_each(|mutation| apply_mutation(&mut state, mutation));
    assert!((state.vertices[0].x - 10.0).abs() < 1e-9 && (state.vertices[0].y - 1.0).abs() < 1e-9);
    assert!((state.vertices[1].x - 9.0).abs() < 1e-9 && state.vertices[1].y.abs() < 1e-9);
    assert_eq!(state.vertices[0].w, Some(1.0));
    assert!((state.normals[0].y - 1.0).abs() < 1e-9);
    assert_eq!(state.faces, vec![face]);
    let refused = <ObjAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&gesture(r#"{"scale":[0,1,1]}"#), &base).err().expect("zero scale is refused");
    assert_eq!(refused.code.0, "snapshot-edit.transform-invalid");
}

#[test]
fn natural_file_codec_declares_the_exact_kind_extension_and_media_type() {
    let codec = <ObjAnyEditor as ArtifactEditor>::natural_file_codec().expect("natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.obj@3.0", ".obj", "model/obj", false));
}

#[test]
fn natural_obj_round_trips_edits_and_refuses_invalid_files() {
    let source = include_bytes!("../🧫️fixtures/📂️natural/sample.obj");
    let snapshot = <ObjAnyEditor as ArtifactEditor>::decode_natural_file(source).expect("obj opens");
    assert_eq!((snapshot.vertices.len(), snapshot.faces.len(), snapshot.normals.len()), (3, 1, 1));
    let again = <ObjAnyEditor as ArtifactEditor>::decode_natural_file(&<ObjAnyEditor as ArtifactEditor>::encode_natural_file(&snapshot).expect("obj saves")).expect("obj reopens");
    assert_eq!(again, snapshot);
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[0,0,2]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let emit = <ObjAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    emit.artifact_mutations.iter().for_each(|mutation| apply_mutation(&mut edited, mutation));
    let saved = <ObjAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("edited obj saves");
    let text = String::from_utf8(saved.clone()).unwrap();
    assert!(text.lines().any(|line| line.starts_with("v 0 0 2") || line.starts_with("v 0.0 0.0 2")), "{text}");
    assert!(text.contains("mtllib part.mtl") && text.contains("usemtl red") && text.contains("# one triangle"));
    let reopened = <ObjAnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("edited obj reopens");
    assert_eq!(reopened.vertices[2].z, 2.0);
    assert_eq!(reopened.faces, snapshot.faces);
    assert!(<ObjAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/invalid.obj")).is_err());
    assert!(<ObjAnyEditor as ArtifactEditor>::decode_natural_file(&[0xff, 0xfe]).is_err());
}

#[test]
fn the_scene_draws_the_files_own_triangles() {
    let snapshot = <ObjAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.obj")).expect("obj opens");
    let (positions, triangles, markers) = semio_s_artifact_stdio_obj_inferences::schema::inferences::scene_geometry(&snapshot);
    assert_eq!((positions, triangles, markers.len()), (vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], vec![0, 1, 2], 0));
    assert!(main::render(&snapshot).is_ok());
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
fn natural_obj_decodes_to_what_the_independent_reference_parser_read() {
    let expected = reference();
    let snapshot = <ObjAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/sample.obj")).expect("obj opens");
    assert_eq!(snapshot.vertices.iter().map(|vertex| vec![vertex.x, vertex.y, vertex.z]).collect::<Vec<_>>(), rows(expected.get("vertices").unwrap()));
    assert_eq!(snapshot.normals.iter().map(|normal| vec![normal.x, normal.y, normal.z]).collect::<Vec<_>>(), rows(expected.get("normals").unwrap()));
    assert_eq!(snapshot.faces.iter().map(|face| face.vertices.iter().map(|corner| corner.vertex as f64).collect::<Vec<_>>()).collect::<Vec<_>>(), rows(expected.get("faces").unwrap()));
    assert_eq!(snapshot.objects.iter().map(|object| object.name.as_str()).collect::<Vec<_>>(), ["tri"]);
}
