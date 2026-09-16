use super::*;
use crate::editor::jack::TRINITY_JACK_PLAY_BODY_ARTIFACT;
use crate::{Camera, Edge, JackWorkingScene, Manifest, Node, PropertyBag};
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

//#region 🪟️WindowLaws
/// 🪟️ A graph an order of magnitude past one viewport — the subject of every window law below.
fn oversized_snapshot(nodes: usize, edges: usize) -> JackSnapshot {
    let nodes: Vec<Node> = (0..nodes)
        .map(|index| Node { id: format!("node-{index}"), kind: "Piece".into(), name: format!("Piece {index}"), x: index as f64, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: Vec::new() })
        .collect();
    let edges: Vec<Edge> = (0..edges).map(|index| Edge { id: format!("edge-{index}"), kind: "Connection".into(), source: format!("node-{index}"), target: format!("node-{}", index + 1), properties: PropertyBag::new() }).collect();
    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "oversized".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes, edges }, None)
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(snapshot: &JackSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let node = render(snapshot, &semio_framework_plugin::NoConfig::default(), &TrinityJackLabels::NATIVE_EN, &TreeWindows::for_body(&view, TRINITY_JACK_PLAY_BODY_ARTIFACT)).expect("render the jack document tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the jack document tree")
}

/// 🪟️ Law (a): every container stamps its FULL extent and materialises at most its slice — no `+N`.
#[test]
fn oversized_document_stamps_totals_and_never_a_continuation_row() {
    let snapshot = oversized_snapshot(240, 180);
    let json = window_body(&snapshot, Vec::new());
    assert!(json.contains("\"total\":240"), "the nodes section stamps its full extent: {json}");
    assert!(json.contains("\"total\":180"), "the edges section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"node-").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a closed container stamps its total and materialises nothing — `edges` is authored
/// closed, so it is lazy even without a host request.
#[test]
fn closed_section_stamps_total_and_materialises_no_children() {
    let snapshot = oversized_snapshot(240, 180);
    let json = window_body(&snapshot, vec![TreeWindowRequest { body_key: TRINITY_JACK_PLAY_BODY_ARTIFACT.into(), node_key: "trinity-document.nodes".into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":240"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("\"node-0\""), "a closed section materialises no rows: {json}");
    assert!(!json.contains("trinity-document.edge."), "the authored-closed edges section materialises no rows either: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw node id.
#[test]
fn host_window_materialises_exactly_its_slice() {
    let snapshot = oversized_snapshot(240, 180);
    let json = window_body(&snapshot, vec![TreeWindowRequest { body_key: TRINITY_JACK_PLAY_BODY_ARTIFACT.into(), node_key: "trinity-document.nodes".into(), open: Some(true), offset: 60, rows: 10 }]);
    assert!(json.contains("\"offset\":60"), "the section reports its offset: {json}");
    for index in 60..70 {
        assert!(json.contains(&format!("\"node-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"node-59\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"node-70\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d): the `"ast"`-domain rows carry a granularity and no binding of their own, while the tree
/// root carries exactly one `interactionSelect`.
#[test]
fn domain_rows_are_granular_and_the_tree_binds_one_interaction_select() {
    let snapshot = oversized_snapshot(4, 2);
    let json = window_body(&snapshot, Vec::new());
    assert!(json.contains("\"interactionDomain\":\"ast\""), "the tree declares its domain: {json}");
    assert_eq!(json.matches("interactionSelect").count(), 1, "exactly one tree-level pick binding: {json}");
    assert_eq!(json.matches("\"granularity\":\"node\"").count(), 4, "every node row is a pick target: {json}");
}
//#endregion 🪟️WindowLaws
