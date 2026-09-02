//! 🧪️ `reorder-nodes` fixture — `moves-the-leading-path-node-to-the-end-of-the-layer-root`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a parent that is not a group, or a `from` past
//! the end, is Error `mutation.target-missing`; `from == to` is Warning `mutation.no-op`. The move
//! is expressed as a REMOVED-plus-ADDED pair inside one `children` triple, and the shared
//! `apply_indexed` replays it as modified → removed (descending) → added (ascending) — which is
//! precisely why `0 -> 2` over three siblings lands the node LAST.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioDrawingSnapshot {
    serde_json::from_str(BEFORE).expect("reorder-nodes before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("reorder-nodes after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("reorder-nodes mutation decodes")
}

/// ▶️ The path node leaves the head and lands at the tail; the other two shift down.
#[semio_framework_async_macros::async_test]
async fn moves_the_leading_child_past_the_other_two() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("reorder-nodes applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-nodes/moves-the-leading-path-node-to-the-end-of-the-layer-root: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { children: base_children, .. } = &base.layers[0].root else { panic!("the layer root is a group") };
    assert_eq!(children.len(), base_children.len(), "a reorder is a permutation — it may never add or drop a sibling");
    assert_eq!(children[2], base_children[0], "the moved node must sit last after the remove-then-insert");
    assert_eq!((children[0].clone(), children[1].clone()), (base_children[1].clone(), base_children[2].clone()), "the siblings it jumped over keep their relative order");
}

/// ↩️ The undo addresses the index the node LANDED at (`min(to, len - 1)`), not the requested `to`.
#[semio_framework_async_macros::async_test]
async fn the_undo_reorder_moves_the_node_back_to_the_head() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "reorder-nodes undoes as exactly one reorder-nodes");
    let SemioDrawingMutation::ReorderNodes(back) = &undo[0] else { panic!("reorder-nodes must undo as reorder-nodes") };
    assert_eq!((back.from, back.to), (2, 0), "the undo addresses the landed index and sends it back to the original one");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward reorder-nodes applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo reorder-nodes applies");
    }
    assert_eq!(current, base, "reorder-nodes/moves-the-leading-path-node-to-the-end-of-the-layer-root: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ReorderNodes":{"parent":{…},"from":0,"to":2}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-nodes/moves-the-leading-path-node-to-the-end-of-the-layer-root: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("reorder-nodes mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("reorder-nodes mutation reparses");
    assert_eq!(reencoded, original, "reorder-nodes/moves-the-leading-path-node-to-the-end-of-the-layer-root: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the parent resolves to a group, from is in range and differs from to, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-nodes/moves-the-leading-path-node-to-the-end-of-the-layer-root: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "a genuine sibling move must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children` triple carrying BOTH a removed index and an added entry — that pair IS the move.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-nodes/moves-the-leading-path-node-to-the-end-of-the-layer-root: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed reorder-nodes diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(children.removed, vec![0usize], "the node leaves its old position");
    assert_eq!(children.added.len(), 1, "and is re-inserted with its content carried in the diff");
    assert_eq!(children.added[0].index, 2, "at the requested target position");
    assert!(children.modified.is_empty(), "a reorder modifies no sibling in place");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-nodes/moves-the-leading-path-node-to-the-end-of-the-layer-root: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed reorder-nodes diff decodes");
    let produced = decoded.apply(&before()).expect("committed reorder-nodes diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-nodes/moves-the-leading-path-node-to-the-end-of-the-layer-root: committed diff did not carry before to after");
}
