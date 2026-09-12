
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn create_artifact_relays_only_the_catalog_choice_and_name_without_local_publication() {
    let mut app = artifact_app_laws::new_app().await;
    let kind_choice = "{\"kindId\":\"s.gis.gismap\",\"schema\":\"gis.map\"}";
    let result = app.dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: " First ".into(), kind_choice: kind_choice.into() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("create artifact");
    let snapshot = app.snapshot().expect("projection");
    assert!(snapshot.artifacts.is_empty(), "the guest must not mint or publish a document identity");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.create-space-artifact");
            let args = pack::json_from_dsl_value(&args.clone().expect("args"));
            assert_eq!(args.get("kindChoice").and_then(|v| v.as_str()), Some(kind_choice));
            assert_eq!(args.get("name").and_then(|v| v.as_str()), Some("First"));
            assert_eq!(args.as_object().map(|value| value.len()), Some(2));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_name_and_kind_open_the_dialog_instead_of_failing() {
    let mut app = artifact_app_laws::new_app().await;
    let result = app.dispatch_typed(SpaceIndexCommand::CreateArtifact(CreateArtifact { name: String::new(), kind_choice: String::new() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("empty args must open the dialog, not fail");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::OpenDialog { dialog_id, args, .. } => {
            assert_eq!(dialog_id, "createArtifact");
            assert!(args.is_none());
        }
        other => panic!("expected OpenDialog, got {other:?}"),
    }
    let snapshot = app.snapshot().expect("projection");
    assert!(snapshot.artifacts.is_empty(), "opening the dialog must not create a row");
}
