use super::*;
use crate::editor::rewriting::TRINITY_REWRITING_PLAY_BODY_ARTIFACT;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};
use semio_s_artifact_trinity_jack::{Camera, JackSnapshot, JackWorkingScene, Manifest, Node, PropertyBag};

//#region 🪟️WindowLaws
/// 🪟️ A before-fixture an order of magnitude past one viewport — the subject of every window law below.
fn oversized_state(nodes: usize) -> RewritingSnapshot {
    let nodes: Vec<Node> = (0..nodes)
        .map(|index| Node { id: format!("node-{index}"), kind: "Piece".into(), name: format!("Piece {index}"), x: index as f64, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: Vec::new() })
        .collect();
    let fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "oversized".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes, edges: Vec::new() }, None);
    RewritingSnapshot { before_fixture_json: fixture.to_json().expect("the oversized before-fixture serializes"), ..Default::default() }
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(state: &RewritingSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let node = render(state, &NoConfig::default(), &TrinityRewritingLabels::NATIVE_EN, &TreeWindows::for_body(&view, TRINITY_REWRITING_PLAY_BODY_ARTIFACT)).expect("render the rewriting document tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the rewriting document tree")
}

/// 🪟️ Law (a): the nodes container stamps its FULL extent and materialises at most its slice — no `+N`.
#[test]
fn oversized_document_stamps_totals_and_never_a_continuation_row() {
    let state = oversized_state(240);
    let json = window_body(&state, Vec::new());
    assert!(json.contains("\"total\":240"), "the nodes section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"node-").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing.
#[test]
fn closed_section_stamps_total_and_materialises_no_children() {
    let state = oversized_state(240);
    let json = window_body(&state, vec![TreeWindowRequest { body_key: TRINITY_REWRITING_PLAY_BODY_ARTIFACT.into(), node_key: "trinity-document.nodes".into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":240"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("\"node-0\""), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw node id.
#[test]
fn host_window_materialises_exactly_its_slice() {
    let state = oversized_state(240);
    let json = window_body(&state, vec![TreeWindowRequest { body_key: TRINITY_REWRITING_PLAY_BODY_ARTIFACT.into(), node_key: "trinity-document.nodes".into(), open: Some(true), offset: 100, rows: 8 }]);
    assert!(json.contains("\"offset\":100"), "the section reports its offset: {json}");
    for index in 100..108 {
        assert!(json.contains(&format!("\"node-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"node-99\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"node-108\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d): the `"graph"`-domain rows carry a granularity and no binding of their own, while the
/// tree root carries exactly one `interactionSelect`.
#[test]
fn domain_rows_are_granular_and_the_tree_binds_one_interaction_select() {
    let state = oversized_state(4);
    let json = window_body(&state, Vec::new());
    assert!(json.contains("\"interactionDomain\":\"graph\""), "the tree declares its domain: {json}");
    assert_eq!(json.matches("interactionSelect").count(), 1, "exactly one tree-level pick binding: {json}");
    assert_eq!(json.matches("\"granularity\":\"node\"").count(), 4, "every node row is a pick target: {json}");
}
//#endregion 🪟️WindowLaws
