use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use crate::editor::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_semio_image_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, SEMIO_IMAGE_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SemioImageEditor as ArtifactEditor>::DIALECT, SEMIO_IMAGE_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<SemioImageEditor, crate::viewer::SemioImageViewer>().await;
}

semio_plugin::history_edit_acceptance_law!("stdio/SemioImageEditor", SemioImageEditor, || semio_plugin::App { definition: create_semio_image_editor(), examples: Vec::new() }, "../..");

#[test]
fn natural_file_codec_belongs_to_the_image_editor_and_round_trips_the_portable_snapshot() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️natural-file.json")).expect("portable natural image fixture");
    let codec = <SemioImageEditor as ArtifactEditor>::natural_file_codec().expect("image editor owns its natural codec");
    let descriptor = serde_json::json!({"formatKind":codec.format_kind,"extension":codec.extension,"mediaType":codec.media_type,"binary":codec.binary});
    assert_eq!(descriptor, fixture["codec"]);
    let oracle = serde_json::to_string(&fixture["snapshot"]).expect("independent JSON fixture");
    let snapshot: crate::schema::snapshot::SemioImageSnapshot = semio_framework_pack_json::from_json_str(&oracle, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("image snapshot");
    let bytes = <SemioImageEditor as ArtifactEditor>::encode_natural_file(&snapshot).expect("natural image encode");
    assert!(!bytes.is_empty());
    let decoded = <SemioImageEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("natural image decode");
    assert_eq!(decoded, snapshot);
    let actual = semio_framework_pack_json::to_json_string(&decoded);
    let independent: serde_json::Value = serde_json::from_str(&actual).expect("independent round-trip JSON");
    assert_eq!(independent, fixture["snapshot"]);
}
