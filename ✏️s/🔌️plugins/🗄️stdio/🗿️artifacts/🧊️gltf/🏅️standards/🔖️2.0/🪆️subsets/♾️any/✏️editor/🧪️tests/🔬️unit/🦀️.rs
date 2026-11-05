use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_gltf_any_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, GLTF_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<GltfAnyEditor as ArtifactEditor>::DIALECT, GLTF_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<GltfAnyEditor, crate::viewer::gltf::GltfAnyViewer>().await;
}

semio_plugin::history_edit_acceptance_law!("stdio", super::GltfAnyEditor, || semio_plugin::App { definition: super::create_gltf_any_editor(), examples: Vec::new() }, "../../..");

#[test]
fn natural_file_codec_declares_the_exact_kind_extension_and_media_type() {
    let codec = <GltfAnyEditor as ArtifactEditor>::natural_file_codec().expect("natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.gltf@2.0", ".gltf", "model/gltf+json", false));
}

#[test]
fn natural_gltf_opens_json_and_glb_agrees_with_an_independent_container_header_check_and_keeps_edits() {
    let source = include_bytes!("../🧫️fixtures/📂️natural/sample.gltf");
    let snapshot = <GltfAnyEditor as ArtifactEditor>::decode_natural_file(source).expect("gltf opens");
    assert_eq!(snapshot.document.nodes[0].translation, Some([1.0, 2.0, 3.0]));
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[0,10,0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let rows = <GltfAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    assert_eq!(rows.artifact_mutations.len(), 1);
    let mut moved = snapshot.clone();
    rows.artifact_mutations.iter().for_each(|mutation| assert!(semio_s_artifact_stdio_contract::apply_mutation(&mut moved, mutation).messages().is_empty()));
    assert_eq!(moved.document.nodes[0].translation, Some([1.0, 12.0, 3.0]));
    assert_eq!(moved.document.nodes[0].name, snapshot.document.nodes[0].name);
    let moved_saved = <GltfAnyEditor as ArtifactEditor>::encode_natural_file(&moved).expect("moved gltf saves");
    assert_eq!(<GltfAnyEditor as ArtifactEditor>::decode_natural_file(&moved_saved).expect("moved gltf reopens").document.nodes[0].translation, Some([1.0, 12.0, 3.0]));
    let model = semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::encode_glb;
    let glb = model(&snapshot).expect("glb encodes");
    assert_eq!((&glb[0..4], u32::from_le_bytes(glb[4..8].try_into().unwrap()), u32::from_le_bytes(glb[8..12].try_into().unwrap()) as usize), (&b"glTF"[..], 2, glb.len()));
    assert_eq!(&glb[16..20], b"JSON");
    let from_glb = <GltfAnyEditor as ArtifactEditor>::decode_natural_file(&glb).expect("glb opens");
    assert_eq!(from_glb.document.nodes, snapshot.document.nodes);
    let saved = <GltfAnyEditor as ArtifactEditor>::encode_natural_file(&from_glb).expect("gltf saves");
    assert_eq!(<GltfAnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("gltf reopens").document.nodes, snapshot.document.nodes);
    let parsed = semio_framework_pack_json::parse(std::str::from_utf8(&moved_saved).expect("gltf is text"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the saved gltf is valid JSON");
    assert_eq!(semio_framework_pack_json::to_dsl_value(&parsed).get("nodes").and_then(|nodes| nodes.as_array()).map(|nodes| nodes.len()), Some(1));
    assert!(<GltfAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/invalid.gltf")).is_err());
    assert!(<GltfAnyEditor as ArtifactEditor>::decode_natural_file(b"glTF\x02\0\0\0").is_err());
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
fn natural_gltf_and_glb_decode_to_what_the_independent_reference_writer_produced() {
    let expected = reference();
    let reference_nodes = expected.get("nodes").unwrap().as_array().unwrap();
    for source in [&include_bytes!("../🧫️fixtures/📂️natural/sample.gltf")[..], &include_bytes!("../🧫️fixtures/📂️natural/sample.glb")[..]] {
        let snapshot = <GltfAnyEditor as ArtifactEditor>::decode_natural_file(source).expect("the file opens");
        assert_eq!(snapshot.document.nodes.len(), reference_nodes.len());
        snapshot.document.nodes.iter().zip(reference_nodes).for_each(|(node, reference)| {
            assert_eq!(node.name.as_deref(), reference.get("name").and_then(|name| name.as_str()));
            assert_eq!(node.translation.map(|translation| translation.to_vec()), Some(numbers(reference.get("translation").unwrap())));
        });
    }
}

#[test]
fn the_scene_places_accessor_triangles_by_the_node_transform() {
    let snapshot = <GltfAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/triangle.gltf")).expect("the triangle opens");
    let (positions, triangles, markers) = semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::scene_geometry(&snapshot.document, &snapshot.buffers);
    assert_eq!((triangles, markers.len()), (vec![0, 1, 2], 0));
    assert_eq!(positions.iter().map(|point| point.to_vec()).collect::<Vec<_>>(), rows(reference().get("triangleWorld").unwrap()));
    assert!(main::render(&snapshot).is_ok());
}

#[test]
fn the_glb_encode_path_round_trips_through_the_natural_decoder() {
    let snapshot = <GltfAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/triangle.gltf")).expect("the triangle opens");
    let glb = semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::encode_glb(&snapshot).expect("glb encodes");
    assert_eq!(&glb[0..4], b"glTF");
    let reopened = <GltfAnyEditor as ArtifactEditor>::decode_natural_file(&glb).expect("glb reopens");
    assert_eq!(reopened.document.nodes, snapshot.document.nodes);
    assert_eq!(reopened.document.meshes, snapshot.document.meshes);
    assert_eq!(reopened.buffers[0], snapshot.buffers[0]);
}

#[test]
fn the_independent_container_reader_accepts_the_edited_glb_and_returns_the_edited_nodes() {
    let snapshot = <GltfAnyEditor as ArtifactEditor>::decode_natural_file(include_bytes!("../🧫️fixtures/📂️natural/triangle.gltf")).expect("the triangle opens");
    let event = editing::SnapshotEditEvent::SetValue { path: editing::TRANSFORM_PATH.into(), value: semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"translate":[0,10,0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()) };
    let rows = <GltfAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("transform resolves");
    let mut edited = snapshot.clone();
    rows.artifact_mutations.iter().for_each(|mutation| assert!(semio_s_artifact_stdio_contract::apply_mutation(&mut edited, mutation).messages().is_empty()));
    let glb = semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::encode_glb(&edited).expect("glb encodes");
    let by_oracle = semio_s_artifact_stdio_gltf_test_oracle::standards::v2_0::subsets::any::round_trip(&glb).expect("the independent reader accepts the edited container");
    let reopened = <GltfAnyEditor as ArtifactEditor>::decode_natural_file(&by_oracle).expect("the independent output reopens");
    assert_eq!(reopened.document.nodes, edited.document.nodes);
    assert_eq!(reopened.document.nodes[0].translation, Some([1.0, 12.0, 3.0]));
    assert_eq!(reopened.buffers[0], edited.buffers[0]);
    let saved = <GltfAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("gltf saves");
    let projected = semio_s_artifact_stdio_gltf_test_oracle::standards::v2_0::subsets::any::project_gltf(&glb);
    assert!(projected.is_ok() && !saved.is_empty());
}
