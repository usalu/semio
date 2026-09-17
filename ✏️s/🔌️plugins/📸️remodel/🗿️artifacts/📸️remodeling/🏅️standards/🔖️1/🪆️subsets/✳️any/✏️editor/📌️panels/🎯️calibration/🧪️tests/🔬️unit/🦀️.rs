use super::*;
use crate::editor::remodeling::commands::add_gcp::AddGcp;
use crate::editor::remodeling::unit_tests::context::{app, dispatch, render as render_body};
use crate::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn the_calibration_panel_lists_added_ground_control_points() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::AddGcp(AddGcp { name: "Corner".into(), world_x: 1.0, world_y: 2.0, world_z: 3.0 })).await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_CALIBRATION).await.contains("Corner"));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the four window laws that replace the
// old `+N` paging laws. (a) an oversized document stamps every container's real `total` and
// materialises at most its slice, (b) a closed container stamps `total` and builds no children,
// (c) a host `TreeWindowRequest{offset, rows}` materialises exactly `[offset, offset + rows)` keyed by
// the row's own target id, (d) pick rows carry a granularity and no binding of their own while the
// tree root carries exactly one `interactionSelect`.
use crate::editor::remodeling::terminology::remodeling_labels;
use semio_framework_plugin::{BuiltNode, Component, TreeWindowRequest, TreeWindows, ViewModel};

const GCPS_SECTION: &str = "remodeling-calibration.gcps";
const OVERSIZED: usize = 200;

/// 📸️ A photogrammetry project with far more ground control points than one node admits. Law (d) does
/// not apply here: `📸️remodel` declares no `InteractionDefinition`, so this panel has no pick rows.
fn oversized_calibration(count: usize) -> crate::RemodelingSnapshot {
    let mut scene = crate::default_remodeling_scene();
    scene.gcps = (0..count).map(|index| crate::GroundControlPoint { id: format!("gcp-{index:03}"), name: format!("GCP {index}"), world_position: [0.0, 0.0, 0.0], observations: Vec::new() }).collect();
    scene
}

fn window_law_request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: REMODELING_PLAY_BODY_CALIBRATION.into(), node_key: node_key.into(), open, offset, rows }
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
    ViewModel { tree_windows: requests, ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn an_oversized_rig_stamps_the_full_total_and_materialises_at_most_its_slice() {
    let scene = oversized_calibration(OVERSIZED);
    let tree = render(&scene, remodeling_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the calibration panel builds");
    let gcps = window_law_node(&tree, GCPS_SECTION);
    assert_eq!(window_law_extent(gcps), (OVERSIZED as u32, 0));
    assert!(gcps.children.len() < OVERSIZED, "only the first-paint slice is materialised: {}", gcps.children.len());
    window_law_no_continuation(&tree);
}

#[semio_framework_async_macros::async_test]
async fn a_closed_section_stamps_its_total_and_builds_no_children() {
    let scene = oversized_calibration(OVERSIZED);
    let view = window_law_view(vec![window_law_request(GCPS_SECTION, Some(false), 0, 32)]);
    let tree = render(&scene, remodeling_labels(&ViewModel::default()), &TreeWindows::for_body(&view, REMODELING_PLAY_BODY_CALIBRATION)).expect("the calibration panel builds");
    let gcps = window_law_node(&tree, GCPS_SECTION);
    assert_eq!(window_law_extent(gcps), (OVERSIZED as u32, 0));
    assert_eq!(gcps.children.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_gcp_id() {
    let scene = oversized_calibration(OVERSIZED);
    let view = window_law_view(vec![window_law_request(GCPS_SECTION, Some(true), 100, 4)]);
    let tree = render(&scene, remodeling_labels(&ViewModel::default()), &TreeWindows::for_body(&view, REMODELING_PLAY_BODY_CALIBRATION)).expect("the calibration panel builds");
    let gcps = window_law_node(&tree, GCPS_SECTION);
    assert_eq!(window_law_extent(gcps), (OVERSIZED as u32, 100));
    assert_eq!(window_law_keys(gcps), (100..104).map(|index| format!("remodeling-calibration.gcp.gcp-{index:03}")).collect::<Vec<_>>());
}

/// 🧰️ The two count lines are panel chrome in their own fixed section, so they never cost the
/// windowed camera/GCP sections a slot.
#[semio_framework_async_macros::async_test]
async fn the_count_lines_live_in_their_own_fixed_section() {
    let scene = oversized_calibration(4);
    let tree = render(&scene, remodeling_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the calibration panel builds");
    assert_eq!(window_law_keys(window_law_node(&tree, "remodeling-calibration.summary")), vec!["remodeling-calibration.summary".to_string(), "remodeling-calibration.gcp-count".to_string()]);
}

//#endregion 🪟️WindowLaws
