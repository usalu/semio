
use super::*;

#[semio_framework_async_macros::async_test]
async fn renders_compiled_dag_editor() {
    use semio_framework_plugin::{PluginApp, ViewModel};
    // 🧬️ Registry-backed: `SpaceApp` publishes a `bounded_first_step_tool_proofs!` roster, and an
    // empty `AppActionRegistry` declares none of them as `Migrated`, so the registry-LESS
    // `VcsArtifactApp::new` fails construction with `interactive-job.catalog-authority`.
    let mut app = crate::engine::space::unit_tests::context::app_with_registry().await;
    let node = app.render(S_PLAY_BODY_COMPILED_DAG, None, &ViewModel::default()).await.expect("render");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("compiled DAG tree projection");
    assert!(json.contains("text-editor"));
    let wire = compiled_dag_wire_literal(&demo_space_projection().await).await;
    assert!(wire.contains("appInstance") || wire.contains("draw"));
}
