use semio_framework_ui_contract::SurfaceKind;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::NodeGraphEdgeRecord;
#[allow(unused_imports)]
use semio_framework_plugin::NodeGraphNodeRecord;
#[allow(unused_imports)]
use semio_framework_plugin::NodeGraphPortRecord;
#[allow(unused_imports)]
use semio_framework_plugin::NodeGraphScene;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_os_kernel::Viewport2d;
use crate::viewer::jack::modes::view::windows::graph::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_node_graph_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, semio_framework_plugin::SurfaceKind::NodeGraph);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::schema::snapshot::empty_trinity_graph_snapshot::<crate::test_support::FixtureComposition>();
    let node = render(&document, crate::standards::v1::subsets::any::schema::content::jack_content_for_handle(&document.content).expect("retained content child").read()).expect("node graph surface");
    assert!(semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project semantic UI test tree").contains("node-graph"));
}
