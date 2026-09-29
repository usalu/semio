use super::*;

/// 🧬️ Registers the document schema jpg's declaration contributes — the contract every snapshot edit validates against;
/// a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::v_jfif_1_01::subsets::document::schema::jpg_artifact_schema_descriptor()]).expect("the jpg document schema registers");
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
    let base: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("/pixels{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") { *from = format!("/pixels{}", from.as_str().unwrap()).into(); }
        let event: editing::SnapshotEditEvent = pack::json::from_json_str(&event.to_string()).unwrap();
        let emitted = <JpgBaselineEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::MutationDiff::apply(<JpgBaselineMutation as protocol::Mutation<JpgSnapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut("/pixels").unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}
