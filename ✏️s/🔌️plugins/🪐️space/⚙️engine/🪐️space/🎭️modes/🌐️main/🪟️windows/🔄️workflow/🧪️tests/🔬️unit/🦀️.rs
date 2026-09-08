
use super::*;

#[semio_framework_async_macros::async_test]
async fn renders_workflow_scene() {
    use semio_framework_plugin::{PluginApp, VcsArtifactApp, ViewModel};
    let mut app = VcsArtifactApp::<crate::engine::space::SpaceApp>::new(crate::engine::space::SpaceApp::default()).await;
    let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewModel::default()).await.expect("render");
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(node).expect("workflow tree projection");
    assert!(json.contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn workflow_scene_uses_flow_engine_with_fixture() {
    use semio_framework_plugin::{PluginApp, VcsArtifactApp, ViewModel};
    let mut app = VcsArtifactApp::<crate::engine::space::SpaceApp>::new(crate::engine::space::SpaceApp::default()).await;
    let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewModel::default()).await.expect("render");
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(node).expect("workflow tree projection");
    assert!(json.contains(r#"\"engine\":\"flow\""#));
    assert!(json.contains("fixtureJson"));
    assert!(json.contains(r#"\"schema\":\"flow.fixture\""#));
}
