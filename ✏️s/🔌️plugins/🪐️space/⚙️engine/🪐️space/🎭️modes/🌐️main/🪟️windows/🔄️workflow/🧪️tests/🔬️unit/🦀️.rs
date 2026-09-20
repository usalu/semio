
use super::*;

#[semio_framework_async_macros::async_test]
async fn renders_workflow_scene() {
    use semio_framework_plugin::{PluginApp, ViewModel};
    // 🧬️ Registry-backed: `SpaceApp` publishes a `bounded_first_step_tool_proofs!` roster, and an
    // empty `AppActionRegistry` declares none of them as `Migrated`, so the registry-LESS
    // `VcsArtifactApp::new` fails construction with `interactive-job.catalog-authority`.
    let mut app = crate::engine::space::unit_tests::context::app_with_registry().await;
    let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewModel::default()).await.expect("render");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("workflow tree projection");
    assert!(json.contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn workflow_scene_uses_flow_engine_with_fixture() {
    use semio_framework_plugin::{PluginApp, ViewModel};
    let mut app = crate::engine::space::unit_tests::context::app_with_registry().await;
    let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewModel::default()).await.expect("render");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("workflow tree projection");
    assert!(json.contains(r#"\"engine\":\"flow\""#));
    assert!(json.contains("fixtureJson"));
    assert!(json.contains(r#"\"schema\":\"flow.host_snapshot\""#));
}
