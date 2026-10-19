//! 🧪️ Aggregated unit tests of the `panels` leaves, one module per leaf.

mod catalogue {
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use super::super::catalogue::*;
use crate::editor::dag::unit_tests::context::{close, new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_every_node_kind() {
    let mut app = new_app().await;
    let json = render_body(&mut app, DAG_PLAY_BODY_CATALOGUE).await;
    close(&mut app);
    for kind in ["computation", "slider", "select", "screen", "note", "preview"] {
        assert!(json.contains(kind), "catalogue must list the {kind} kind: {json}");
    }
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_CATALOGUE));
}
}

mod document {
use crate::DagScene;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use super::super::document::*;
use crate::editor::dag::unit_tests::context::{close, new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn dag_play_labels_resolve_native_by_default() {
    let mut app = new_app().await;
    let json = render_body(&mut app, DAG_PLAY_BODY_ARTIFACT).await;
    close(&mut app);
    assert!(json.contains("Nodes"));
    assert!(json.contains("Edges"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the four window laws that replace the
// old `+N` paging laws. (a) an oversized document stamps every container's real `total` and
// materialises at most its slice, (b) a closed container stamps `total` and builds no children,
// (c) a host `TreeWindowRequest{offset, rows}` materialises exactly `[offset, offset + rows)` keyed by
// the row's own target id, (d) pick rows carry a granularity and no binding of their own while the
// tree root carries exactly one `interactionSelect`.
use semio_framework_plugin::{BuiltNode, Component, TreeWindowRequest, TreeWindows, ViewModel, INTERACTION_SELECT_ACTION_ID};

const NODES_SECTION: &str = "dag-play-document.nodes";
const OVERSIZED: usize = 200;

fn oversized_dag_document(count: usize) -> crate::DagScene {
    crate::DagScene { nodes: (0..count).map(|index| crate::DagNodeSpec { id: format!("node-{index:03}"), name: format!("Node {index}"), ..crate::DagNodeSpec::default() }).collect(), edges: Vec::new() }
}

fn window_law_request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: DAG_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

fn window_law_node<'a>(root: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    fn walk<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
        if node.key.as_str() == key {
            return Some(node);
        }
        node.children.iter().find_map(|child| walk(child, key))
    }
    walk(root, key).unwrap_or_else(|| panic!("no node keyed {key}"))
}

fn window_law_extent(node: &BuiltNode) -> (u32, u32) {
    let window = match &node.component {
        Component::TreeSection(props) => props.window,
        Component::TreeItem(props) => props.window,
        _ => None,
    };
    let window = window.unwrap_or_else(|| panic!("{} stamps no window", node.key.as_str()));
    (window.total, window.offset)
}

fn window_law_keys(node: &BuiltNode) -> Vec<String> {
    node.children.iter().map(|child| child.key.as_str().to_string()).collect()
}

fn window_law_no_continuation(node: &BuiltNode) {
    assert!(!node.key.as_str().ends_with(".more"), "{} is a continuation row", node.key.as_str());
    if let Component::TreeItem(props) = &node.component {
        assert!(!props.label.0.as_str().starts_with('+'), "{} carries a +N label", node.key.as_str());
    }
    for child in node.children.iter() {
        window_law_no_continuation(child);
    }
}

fn window_law_view(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }
}

#[semio_framework_async_macros::async_test]
async fn an_oversized_document_stamps_the_full_total_and_materialises_at_most_its_slice() {
    let document = oversized_dag_document(OVERSIZED);
    let tree = render(&document, crate::editor::dag::terminology::dag_play_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::unhosted()).expect("the document tree builds");
    let nodes = window_law_node(&tree, NODES_SECTION);
    assert_eq!(window_law_extent(nodes), (OVERSIZED as u32, 0));
    assert!(nodes.children.len() < OVERSIZED, "only the first-paint slice is materialised: {}", nodes.children.len());
    window_law_no_continuation(&tree);
}

#[semio_framework_async_macros::async_test]
async fn a_closed_section_stamps_its_total_and_builds_no_children() {
    let document = oversized_dag_document(OVERSIZED);
    let view = window_law_view(vec![window_law_request(NODES_SECTION, Some(false), 0, 32)]);
    let tree = render(&document, crate::editor::dag::terminology::dag_play_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::for_body(&view, DAG_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let nodes = window_law_node(&tree, NODES_SECTION);
    assert_eq!(window_law_extent(nodes), (OVERSIZED as u32, 0));
    assert_eq!(nodes.children.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_node_id() {
    let document = oversized_dag_document(OVERSIZED);
    let view = window_law_view(vec![window_law_request(NODES_SECTION, Some(true), 120, 5)]);
    let tree = render(&document, crate::editor::dag::terminology::dag_play_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::for_body(&view, DAG_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let nodes = window_law_node(&tree, NODES_SECTION);
    assert_eq!(window_law_extent(nodes), (OVERSIZED as u32, 120));
    assert_eq!(window_law_keys(nodes), (120..125).map(|index| format!("node-{index:03}")).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn pick_rows_carry_a_granularity_and_the_tree_root_carries_the_one_interaction_select() {
    let document = oversized_dag_document(4);
    let tree = render(&document, crate::editor::dag::terminology::dag_play_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::unhosted()).expect("the document tree builds");
    assert_eq!(tree.bindings.len(), 1);
    assert_eq!(tree.bindings.iter().next().expect("the tree binding").action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    for row in window_law_node(&tree, NODES_SECTION).children.iter() {
        let Component::TreeItem(props) = &row.component else { panic!("{} is not a tree row", row.key.as_str()) };
        assert_eq!(props.granularity.as_ref().map(semio_framework_plugin::UiText::as_str), Some("node"));
        assert!(row.bindings.is_empty(), "{} must not pay for its own activate binding", row.key.as_str());
    }
}

//#endregion 🪟️WindowLaws
}

mod inspection {
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use super::super::inspection::*;
use crate::editor::dag::unit_tests::context::{close, new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_INSPECTOR));
}

#[semio_framework_async_macros::async_test]
async fn renders_the_select_a_node_placeholder_when_nothing_is_selected() {
    let mut app = new_app().await;
    let json = render_body(&mut app, DAG_PLAY_BODY_INSPECTOR).await;
    close(&mut app);
    assert!(json.contains("Select a node"));
}

/// 🕹️ `render` carries no `InteractionView` (`DagPlayApp::render` always calls this panel's own
/// `render` with an empty selection now — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM,
/// matching `space`'s identical gap), so this exercises the panel's OWN rendering logic directly
/// with an explicit selection, the way `space`'s inspector test does, rather than driving it
/// end-to-end through a (now selection-blind) app dispatch.
#[semio_framework_async_macros::async_test]
async fn renders_id_name_and_kind_fields_for_a_single_selected_node() {
    let document = crate::examples::demo::scene();
    let node_id = document.nodes.first().map(|node| node.id.clone()).expect("node");
    let labels = crate::editor::dag::terminology::dag_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let node = render(&document, &[node_id.clone()], labels).expect("inspector component tree");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("inspector tree projection");
    assert!(json.contains(&node_id));
    assert!(json.contains("Name") || json.contains("Kind"));
}


/// 🏗 DSL txt carrier: serialize then deserialize must restore the snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn txt_dsl_carrier_round_trips_exactly() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    use semio_framework::io_schema::IoPayload;
    let snapshot = crate::empty_snapshot();
    let exported = export::txt::v_utf_8::any::DagIntoTxt::serialize(&snapshot,&semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("dsl txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = import::txt::v_utf_8::any::TxtIntoDag::deserialize(&IoPayload::Text(text)).await.expect("dsl txt import");
    assert_eq!(back.value, snapshot);
}
}
