
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn set_visibility_relays_the_directory_command() {
    let mut app = artifact_app_laws::new_app().await;
    let result = app.dispatch_typed(SpaceIndexCommand::SetVisibility(SetVisibility { visibility: "public".into() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("set visibility");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.directory.set-visibility");
            let args = pack::json_from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("visibility").and_then(|v| v.as_str()), Some("public"));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
