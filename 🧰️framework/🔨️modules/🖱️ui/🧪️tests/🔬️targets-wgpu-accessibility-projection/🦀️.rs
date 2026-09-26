//! ♿️ LAW: the wgpu target PUBLISHES an accessibility tree, and it is the same tree React turns into
//! ARIA attributes.
//!
//! `AccessibilitySpec` is a field on every `UiNodeRecord` and React's Interpreter consumes it
//! directly (`accessibilityAriaProps`), but this target's `reconcile`/`paint` had zero references to
//! it — a GPU canvas carries no elements, so every label, description, live region and shortcut the
//! producer authored was read off the wire and then dropped on the floor
//! (`📓️audit-wgpu-parity-2026-09-13.md` gap #3). These laws pin the projection that closes it, over
//! the SHARED `🧬️contract/🧫️fixtures/♿️accessibility-projection.json` corpus the contract's own Rust
//! laws and the TypeScript `uiAccessibilityProjectionNodeV1` twin both answer.
//!
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

use super::*;
use crate::wgpu::reconcile::{UiDocumentReconcileCursor, UiDocumentReconcileStep};
use crate::wgpu::tree::UiDocumentTree;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeRecord, UiRevision};

const PROJECTION_FIXTURE: &str = include_str!("../../🧬️contract/🧫️fixtures/♿️accessibility-projection.json");
const FIXTURE_GENERATION: u64 = 1;

fn law() -> serde_json::Value {
    serde_json::from_str(PROJECTION_FIXTURE).expect("♿️ the shared accessibility-projection fixture parses")
}

/// 🌳️ The fixture's flat snapshot, admitted as one published document lease.
fn published_document(law: &serde_json::Value) -> UiDocumentTree {
    let source = &law["document"];
    let nodes = source["nodes"].as_array().expect("fixture nodes");
    let header = UiDocumentLeaseHeader {
        generation: FIXTURE_GENERATION,
        surface: SurfaceId::try_from(source["surface"].as_str().expect("fixture surface")).expect("fixture surface id"),
        revision: UiRevision(source["revision"].as_u64().expect("fixture revision")),
        root: UiNodeId(source["root"].as_u64().expect("fixture root")),
        layout_epoch: source["layoutEpoch"].as_u64().expect("fixture layout epoch"),
        node_count: nodes.len(),
    };
    let mut document = UiDocumentTree::new(header).expect("fixture header admits");
    for node in nodes {
        let record: UiNodeRecord = serde_json::from_value(node.clone()).expect("fixture record deserializes against the contract");
        document.try_upsert_record(record).expect("fixture record admits");
    }
    document
}

fn mounted_tree(law: &serde_json::Value) -> UiTree {
    let mut tree = UiTree::new();
    tree.publish_document(published_document(law));
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(FIXTURE_GENERATION);
    for _ in 0..4096 {
        match tree.step_document_reconcile(&mut cursor, "procedural-main", "generation3d") {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => return tree,
            UiDocumentReconcileStep::Fault(fault) => panic!("the fixture document must mount cleanly, got {fault:?}"),
        }
    }
    panic!("document reconcile did not terminate inside its own node budget");
}

/// 🔀️ Each retained Toggle appearance publishes the live state channel of the concrete React
/// control it mirrors: pressed for the button Toggle and checked for TreeCheckbox.
#[test]
fn mounted_toggle_appearances_stamp_only_their_native_live_state_channel() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧬️contract/🧫️fixtures/♿️retained-toggle-semantics/🔣️.json")).expect("🔀️ the retained-toggle fixture parses");
    for (index, case) in fixture["cases"].as_array().expect("toggle cases").iter().enumerate() {
        let source_state = case["component"]["on"].as_bool().expect("authored toggle state");
        let record: UiNodeRecord = serde_json::from_value(serde_json::json!({
            "id": index,
            "key": format!("#toggle-{index}"),
            "component": case["component"],
            "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
            "style": {},
            "activity": "idle",
            "accessibility": { "label": case["accessibleLabel"] }
        }))
        .unwrap_or_else(|error| panic!("{}: fixture record deserializes: {error}", case["id"].as_str().expect("case id")));
        let header = UiDocumentLeaseHeader {
            generation: FIXTURE_GENERATION,
            surface: SurfaceId::try_from(format!("toggle.semantic.{index}").as_str()).expect("fixture surface id"),
            revision: UiRevision(0),
            root: record.id,
            layout_epoch: 0,
            node_count: 1,
        };
        let node_id = record.id;
        let mut document = UiDocumentTree::new(header).expect("fixture header admits");
        document.try_upsert_record(record).expect("fixture record admits");
        let mut tree = UiTree::new();
        tree.publish_document(document);
        let mut cursor = UiDocumentReconcileCursor::default();
        cursor.rearm(FIXTURE_GENERATION);
        for _ in 0..64 {
            if matches!(tree.step_document_reconcile(&mut cursor, "procedural-main", "generation3d"), UiDocumentReconcileStep::Complete) {
                break;
            }
        }
        let mounted = tree.document_node(node_id).expect("toggle mounted");
        let crate::wgpu::component::ui::UiNode::Toggle(toggle) = &mut tree.node_mut(mounted).expect("mounted toggle is live").spec.0 else { panic!("fixture toggle reconciles as UiNode::Toggle") };
        toggle.presence.selected = !source_state;
        let projected = accessibility_projection(&tree).pop().expect("mounted toggle is projected");
        let live_state = !source_state;
        match case["expected"]["stateAttribute"].as_str().expect("state attribute") {
            "aria-pressed" => assert_eq!((projected.role.as_str(), projected.pressed, projected.checked), ("button", Some(live_state), None), "{}", projected.key),
            "aria-checked" => assert_eq!((projected.role.as_str(), projected.checked, projected.pressed), ("checkbox", Some(live_state), None), "{}", projected.key),
            attribute => panic!("unexpected state attribute {attribute}"),
        }
    }
}

/// ♿️ The headline: a mounted document answers the FULL projection the shared fixture declares, in
/// pre-order and with the right depth — the tree an assistive technology reads.
#[test]
fn a_mounted_document_publishes_the_accessibility_tree_the_shared_fixture_declares() {
    let law = law();
    let tree = mounted_tree(&law);
    let projection = accessibility_projection(&tree);
    let expected = law["expected"].as_array().expect("fixture expectation");
    assert_eq!(projection.len(), expected.len(), "one projected node per published record, none dropped");
    for (node, row) in projection.iter().zip(expected) {
        assert_eq!(node.node_id, row["nodeId"].as_u64().expect("fixture node id"), "pre-order: the reading order an assistive technology walks");
        assert_eq!(node.key, row["key"].as_str().expect("fixture key"));
        assert_eq!(node.depth, row["depth"].as_u64().expect("fixture depth") as usize, "{}: depth", node.key);
        assert_eq!(node.role, row["role"].as_str().expect("fixture role"), "{}: role", node.key);
        assert_eq!(node.label.as_deref(), row["label"].as_str(), "{}: label", node.key);
        assert_eq!(node.description.as_deref(), row["description"].as_str(), "{}: description", node.key);
        assert_eq!(node.live, row["live"].as_str().expect("fixture live"), "{}: live region", node.key);
        assert_eq!(node.shortcut.as_deref(), row["shortcut"].as_str(), "{}: shortcut", node.key);
        assert_eq!(node.hidden, row["hidden"].as_bool().expect("fixture hidden"), "{}: hidden", node.key);
        assert_eq!(node.focusable, row["focusable"].as_bool().expect("fixture focusable"), "{}: focusable", node.key);
        assert_eq!(node.actionable, row["actionable"].as_bool().expect("fixture actionable"), "{}: actionable", node.key);
        assert_eq!(node.value_min, row["valueMin"].as_f64(), "{}: valueMin", node.key);
        assert_eq!(node.value_max, row["valueMax"].as_f64(), "{}: valueMax", node.key);
        assert_eq!(node.value_now, row["valueNow"].as_f64(), "{}: valueNow", node.key);
        assert_eq!(node.value_text.as_deref(), row["valueText"].as_str(), "{}: valueText", node.key);
        assert_eq!(node.busy, row["busy"].as_bool().unwrap_or(false), "{}: busy", node.key);
    }
    eprintln!("[DEBUG] wgpu accessibility projection: {} nodes published in pre-order from a mounted document", projection.len());
}

/// 🏷️ THE law the audit asked for: every focusable-or-actionable node that carries an
/// `AccessibilitySpec` label appears in the projection, by name. A renderer that drops one has made
/// a control unreachable to a screen reader while still painting it.
#[test]
fn every_labelled_reachable_node_appears_in_the_projection() {
    let law = law();
    let tree = mounted_tree(&law);
    let projection = accessibility_projection(&tree);
    let announced: Vec<u64> = accessibility_announced(&projection).iter().map(|node| node.node_id).collect();
    let declared: Vec<u64> = law["announced"].as_array().expect("fixture announced").iter().map(|id| id.as_u64().expect("announced id")).collect();
    assert_eq!(announced, declared, "exactly the reachable, named nodes — no fewer, and none invented");
    for record in law["document"]["nodes"].as_array().expect("fixture nodes") {
        let Some(label) = record["accessibility"]["label"].as_str() else { continue };
        let node_id = record["id"].as_u64().expect("fixture node id");
        let projected = projection.iter().find(|node| node.node_id == node_id).expect("every published record is projected");
        assert_eq!(projected.label.as_deref(), Some(label), "node {node_id}: the authored label reaches the projection verbatim");
    }
    assert!(!announced.is_empty(), "the corpus must actually exercise a reachable control");
    eprintln!("[DEBUG] wgpu accessibility projection: {} of {} nodes reachable by name", announced.len(), projection.len());
}

/// 🕳️ A window that has published no document announces nothing, rather than faulting — "not laid
/// out" and "nothing to announce" read the same to a consumer, and a probe tells them apart from the
/// window list instead.
#[test]
fn an_unpublished_window_announces_nothing_instead_of_faulting() {
    let tree = UiTree::new();
    assert!(accessibility_projection(&tree).is_empty());
    eprintln!("[DEBUG] wgpu accessibility projection: an unpublished window answers an empty projection");
}

/// 🎯️ Live state the published document does not carry is stamped from the ARENA: the retained
/// node's own focus flag, and the laid-out rect the paint pass consumes.
#[test]
fn the_projection_carries_the_live_focus_and_laid_out_rect_from_the_arena() {
    let law = law();
    let mut tree = mounted_tree(&law);
    let width_id = UiNodeId(2);
    let mounted = tree.document_node(width_id).expect("the numeric field mounted");
    assert!(accessibility_projection(&tree).iter().all(|node| !node.focused), "nothing is focused before anything takes focus");
    tree.node_mut(mounted).expect("the mounted node is live").flags.set(crate::wgpu::tree::NodeFlags::FOCUSED, true);
    let projection = accessibility_projection(&tree);
    let focused: Vec<u64> = projection.iter().filter(|node| node.focused).map(|node| node.node_id).collect();
    assert_eq!(focused, vec![width_id.0], "exactly the arena node holding FOCUSED is announced as focused");
    eprintln!("[DEBUG] wgpu accessibility projection: focus stamped from the arena onto node {:?}", focused);
}

/// 📶️ A mounted progress bar publishes `progressbar` with its value attributes while determinate and
/// only `busy` while indeterminate — and mounts as the retained `UiNode::Progress` the paint pass fills.
#[test]
fn a_mounted_progress_bar_announces_its_value_or_busy_and_mounts_as_a_retained_progress_node() {
    let law = law();
    let tree = mounted_tree(&law);
    let projection = accessibility_projection(&tree);
    let determinate = projection.iter().find(|node| node.key == "#evaluation").expect("the determinate bar is projected");
    assert_eq!((determinate.role.as_str(), determinate.value_min, determinate.value_max, determinate.value_now, determinate.busy), ("progressbar", Some(0.0), Some(100.0), Some(12.0), false));
    let indeterminate = projection.iter().find(|node| node.key == "#preparation").expect("the indeterminate bar is projected");
    assert_eq!((indeterminate.value_min, indeterminate.value_max, indeterminate.value_now, indeterminate.value_text.as_deref(), indeterminate.busy), (None, None, None, None, true));
    for (id, total) in [(UiNodeId(7), Some(100.0)), (UiNodeId(8), None)] {
        let mounted = tree.document_node(id).expect("the progress record mounted");
        let crate::wgpu::component::ui::UiNode::Progress(node) = &tree.node(mounted).expect("the mounted node is live").spec.0 else { panic!("node {id:?} mounts as UiNode::Progress") };
        assert_eq!(node.total, total, "node {id:?}: the total survives reconcile verbatim");
    }
}
