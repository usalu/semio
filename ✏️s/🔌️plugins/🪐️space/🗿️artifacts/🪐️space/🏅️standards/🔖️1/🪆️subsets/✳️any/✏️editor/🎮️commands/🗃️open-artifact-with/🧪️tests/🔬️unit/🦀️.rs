
use super::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn open_artifact_with_relays_the_explicit_choice() {
    let (mut app, id) = context::new_app_with_indexed_artifact().await;
    let result = app
        .dispatch_typed(SpaceIndexCommand::OpenArtifactWith(OpenArtifactWith { id: id.clone(), role: "viewer".into(), plugin_id: "draw".into(), app_id: "draw-play".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"))
        .await
        .expect("open with");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.open-artifact-with");
            let args = pack::json_from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("viewer"));
            assert_eq!(args.get("pluginId").and_then(|v| v.as_str()), Some("draw"));
            assert_eq!(args.get("appId").and_then(|v| v.as_str()), Some("draw-play"));
            assert_eq!(args.get("schema").and_then(|v| v.as_str()), Some("s.draw.draw"));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
