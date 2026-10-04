use super::*;

/// 🧬️ Registers the document schema jpg's declaration contributes — the contract every snapshot edit validates against;
/// a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::v_jfif_1_01::subsets::document::schema::jpg_artifact_schema_descriptor()]).expect("the jpg document schema registers");
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_jpg_baseline_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, JPG_BASELINE_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<JpgBaselineEditor as ArtifactEditor>::DIALECT, JPG_BASELINE_DIALECT);
}

#[test]
fn payload_detail_edits_publish_the_exact_requested_value() {
    register_document_schema();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
    let mut snapshot = JpgSnapshot::default();
    snapshot.pixels = vec![7, 9];
    let base: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("/pixels{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") {
            *from = format!("/pixels{}", from.as_str().unwrap()).into();
        }
        let event: editing::SnapshotEditEvent = semio_framework_pack_json::from_json_str(&event.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let emitted = <JpgBaselineEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::MutationDiff::apply(<JpgBaselineMutation as protocol::Mutation<JpgSnapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut("/pixels").unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::JpgBaselineEditor, || semio_framework_plugin::App { definition: super::create_jpg_baseline_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline");

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_plugin_media_and_isolates_fresh_owner_history() {
    use semio_framework_plugin::plugin_app_close_prelude::{MediaArtifact, MediaArtifactDescriptor};
    use semio_framework_plugin::{artifact_app_laws, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
    let source = include_bytes!("../../../../🧾️document/📚️examples/🎬️demo/🖼️assets/🖼️.jpg");
    let codec = <JpgBaselineEditor as ArtifactEditor>::natural_file_codec().expect("paired natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.jpg@jfif-1.01", ".jpg", "image/jpeg", true));
    let artifact = MediaArtifact {
        descriptor: MediaArtifactDescriptor {
            edge_id: None,
            port_id: Some(NATURAL_FILE_PORT.into()),
            kind_id: Some(codec.format_kind.into()),
            media_type: None,
            wire: MediaWireFormat::Binary { format_kind: codec.format_kind.into() },
            blob_hash: None,
        },
        data: source.to_vec(),
    };
    let initial = <JpgBaselineEditor as ArtifactEditor>::initial_snapshot();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<JpgBaselineEditor>, _>(async { semio_framework_plugin::App { definition: create_jpg_baseline_editor(), examples: Vec::new() } }).await;
    let mut outside = artifact.clone();
    outside.data.push(0x7f);
    app.consume_media(NATURAL_FILE_PORT, artifact).await.expect("registered natural import");
    artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.expect("natural import publishes");
    let opened = app.snapshot().expect("opened snapshot").clone();
    let refused = match app.consume_media(NATURAL_FILE_PORT, outside).await {
        Err(_) => true,
        Ok(_) => artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.is_err(),
    };
    assert!(refused);
    assert_eq!(app.snapshot().expect("refused baseline JPEG preserves the owner"), opened);
    let saved = app.produce_media(NATURAL_FILE_PORT).await.expect("registered natural save");
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    assert_eq!(saved.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let oracle = semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::oracle_identity_round_trip(&saved.data).expect("image independently reopens and writes the baseline JPEG export");
    let observed = semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::project_jpg_mutation(&saved.data).expect("project baseline JPEG export");
    let expected = semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::project_jpg_mutation(&oracle).expect("project independent baseline JPEG output");
    assert_eq!(observed.get("width"), expected.get("width"));
    assert_eq!(observed.get("height"), expected.get("height"));
    let mut reopened = artifact_app_laws::new_registered_app::<EditorApp<JpgBaselineEditor>, _>(async { semio_framework_plugin::App { definition: create_jpg_baseline_editor(), examples: Vec::new() } }).await;
    reopened.bind_instance_id(2).await;
    reopened.consume_media(NATURAL_FILE_PORT, saved).await.expect("fresh owner imports exported bytes");
    artifact_app_laws::settle_registered_typed_operation(&mut reopened, 2).await.expect("fresh owner import publishes");
    let reopened_snapshot = reopened.snapshot().expect("reopened snapshot").clone();
    assert_eq!(reopened_snapshot, opened);
    artifact_app_laws::settle_history_verb(&mut app, "undo", 1).await;
    assert_eq!(app.snapshot().expect("source undo snapshot"), initial);
    assert_eq!(reopened.snapshot().expect("reopened snapshot remains isolated"), reopened_snapshot);
    artifact_app_laws::settle_history_verb(&mut app, "redo", 1).await;
    assert_eq!(app.snapshot().expect("source redo snapshot"), opened);
    artifact_app_laws::settle_history_verb(&mut reopened, "undo", 2).await;
    assert_eq!(reopened.snapshot().expect("reopened undo snapshot"), initial);
    assert_eq!(app.snapshot().expect("source remains isolated"), opened);
    artifact_app_laws::settle_history_verb(&mut reopened, "redo", 2).await;
    assert_eq!(reopened.snapshot().expect("reopened redo snapshot"), reopened_snapshot);
    artifact_app_laws::close_registered_fixture_app(&mut reopened);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}
