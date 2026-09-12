
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn open_artifact_relays_with_document_and_space_ids() {
    let (mut app, id) = context::new_app_with_indexed_artifact().await;
    let result = app.dispatch_typed(SpaceIndexCommand::OpenArtifact(OpenArtifact { id: id.clone() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("open indexed artifact");
    assert!(result.mutations.is_empty());
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.open-artifact");
            let args = pack::json_from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("documentId").and_then(|v| v.as_str()), Some(id.as_str()));
            assert_eq!(args.get("schema").and_then(|v| v.as_str()), Some("s.draw.draw"));
            assert!(args.get("role").is_none(), "role is omitted so the shell resolves OpeningPreferences");
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn open_artifact_of_a_missing_row_faults() {
    let mut app = artifact_app_laws::new_app().await;
    let error = app.dispatch_typed(SpaceIndexCommand::OpenArtifact(OpenArtifact { id: "ghost".into() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect_err("missing row must fault");
    assert_eq!(error.code.0, "s.space.index.target-missing");
}
