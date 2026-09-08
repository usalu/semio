
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, testkit};

#[semio_framework_async_macros::async_test]
async fn request_delete_opens_the_confirm_dialog_without_mutating() {
    let (mut app, id) = testkit::new_app_with_artifact().await;
    let result = app.dispatch_typed(SpaceIndexCommand::RequestDeleteArtifact(RequestDeleteArtifact { id: id.clone() }), &semio_framework_plugin::testkit::meta("local")).await.expect("request delete");
    assert!(result.mutations.is_empty(), "requesting delete never mutates the document directly");
    assert_eq!(app.snapshot().unwrap().artifacts.len(), 1, "the row survives until the dialog is confirmed");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::OpenDialog { dialog_id, args, .. } => {
            assert_eq!(dialog_id, "deleteArtifact");
            let args = pack::json_from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("id").and_then(|v| v.as_str()), Some(id.as_str()));
        }
        other => panic!("expected OpenDialog, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn request_delete_of_a_missing_row_faults() {
    let mut app = testkit::new_app().await;
    let error = app.dispatch_typed(SpaceIndexCommand::RequestDeleteArtifact(RequestDeleteArtifact { id: "ghost".into() }), &semio_framework_plugin::testkit::meta("local")).await.expect_err("missing row must fault");
    assert_eq!(error.code.0, "s.space.mutation.target-missing");
}
