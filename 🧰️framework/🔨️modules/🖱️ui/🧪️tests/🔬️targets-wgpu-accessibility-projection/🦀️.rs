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
