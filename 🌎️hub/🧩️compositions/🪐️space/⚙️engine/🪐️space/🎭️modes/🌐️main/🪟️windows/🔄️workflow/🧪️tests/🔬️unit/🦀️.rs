
use super::*;
use semio_hub_space_foundation::S_PLAY_BODY_WORKFLOW;

#[semio_framework_async_macros::async_test]
async fn renders_workflow_scene() {
    use semio_framework_plugin::{PluginApp, ViewModel};
    let mut app = crate::unit_tests::context::app_with_registry().await;
    let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("workflow tree projection");
    assert!(json.contains("node-graph"));
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn workflow_scene_uses_flow_engine_with_fixture() {
    use semio_framework_plugin::{PluginApp, ViewModel};
    let mut app = crate::unit_tests::context::app_with_registry().await;
    let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("workflow tree projection");
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_ui_scene::NodeGraphScene>(&json).expect("node-graph scene");
    assert!(scene.capabilities_json.as_deref().is_some_and(|capabilities| capabilities.contains(r#""engine":"flow""#)));
    assert!(scene.host_snapshot_json.as_deref().is_some_and(|snapshot| snapshot.contains(r#""schema":"flow.host_snapshot""#)));
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
