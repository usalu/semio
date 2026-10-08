use super::*;

/// 🧬️ Registers the document schema tiff's declaration contributes — the contract every snapshot edit validates against;
/// a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::v6_0::subsets::document::schema::tiff_artifact_schema_descriptor()]).expect("the tiff document schema registers");
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_tiff_baseline_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, TIFF_BASELINE_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TiffBaselineEditor as ArtifactEditor>::DIALECT, TIFF_BASELINE_DIALECT);
}

#[test]
fn payload_detail_edits_publish_the_exact_requested_value() {
    register_document_schema();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
    let mut snapshot = crate::schema::blank_tiff_snapshot();
    snapshot.ifds[0].storage.chunks[0] = vec![7, 9];
    let base: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("/ifds/0/storage/chunks/0{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") { *from = format!("/ifds/0/storage/chunks/0{}", from.as_str().unwrap()).into(); }
        let event: editing::SnapshotEditEvent = semio_framework_pack_json::from_json_str(&event.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let emitted = <TiffBaselineEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::apply_diff(<TiffBaselineMutation as protocol::Mutation<TiffSnapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut("/ifds/0/storage/chunks/0").unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::TiffBaselineEditor, || semio_framework_plugin::App { definition: super::create_tiff_baseline_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline");
