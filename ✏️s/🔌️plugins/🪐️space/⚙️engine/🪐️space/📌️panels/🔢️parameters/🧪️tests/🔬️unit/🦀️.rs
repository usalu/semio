
use super::*;

#[semio_framework_async_macros::async_test]
async fn render_produces_the_add_parameter_header() {
    let projection = semio_framework_os::empty_workflow_snapshot().await;
    let config = crate::engine::space::config::SpaceConfig::default();
    let labels = semio_framework_plugin::resolve_labels::<SStudioLabels>(&semio_framework_plugin::ViewModel::default());
    let node = render(&projection, labels).expect("render");
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("parameters tree projection");
    assert!(json.contains("addParameter"), "header must carry the add-parameter action: {json}");
    assert!(json.contains("parameter"), "empty parameter count copy must render: {json}");
}
