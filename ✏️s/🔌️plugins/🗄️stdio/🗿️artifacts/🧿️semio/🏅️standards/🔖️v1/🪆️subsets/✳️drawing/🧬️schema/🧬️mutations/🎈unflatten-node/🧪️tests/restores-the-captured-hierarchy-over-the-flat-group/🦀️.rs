//! 🧪️ `unflatten` fixture — `restores-the-captured-hierarchy-over-the-flat-group`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unresolvable path is Error
//! `mutation.target-missing`, a node that ALREADY equals the captured `original` is Warning
//! `mutation.no-op`. Otherwise the diff is a `Replace` carrying the payload's whole captured node.
//! `unflatten` is the only leaf in this subset whose payload embeds an entire `DrawNode` as
//! restore data — it is the undo half of `flatten`, and its OWN inverse is `flatten` again.

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
    serde_json::from_str(BEFORE).expect("unflatten before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("unflatten after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("unflatten mutation decodes")
}

/// ▶️ The flat group regains its nested structure, exactly as captured.
#[semio_framework_async_macros::async_test]
async fn restores_the_nested_structure_from_the_captured_node() {
    let base = before();
    let DrawNode::Group { children: base_children, .. } = &base.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { children: before_inner, .. } = &base_children[2] else { panic!("child #2 is the flat group") };
    assert!(!before_inner.iter().any(|node| matches!(node, DrawNode::Group { .. })), "the fixture starts from a genuinely FLAT group");
    let produced = mutation().diff(&base).diff().apply(&base).expect("unflatten applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "unflatten/restores-the-captured-hierarchy-over-the-flat-group: applied state differs from the committed after-snapshot");
    let DrawNode::Group { children, .. } = &produced.layers[0].root else { panic!("the layer root is a group") };
    let DrawNode::Group { children: inner, .. } = &children[2] else { panic!("child #2 is still a group") };
    assert!(matches!(inner[0], DrawNode::Group { .. }), "the captured nesting is back");
    assert_eq!(children[0], base_children[0], "the sibling nodes must be byte-identical");
}

/// ↩️ `unflatten`'s undo is a bare `flatten` at the same path — no capture needed, because
/// flattening the restored hierarchy reproduces the flat node deterministically.
#[semio_framework_async_macros::async_test]
async fn the_undo_flatten_needs_no_capture_of_its_own() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "unflatten undoes as exactly one flatten");
    let SemioDrawingMutation::FlattenNode(reflatten) = &undo[0] else { panic!("unflatten must undo as flatten") };
    assert_eq!(reflatten.at.path, vec![2usize], "the undo addresses the very same node path");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward unflatten applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo flatten applies");
    }
    assert_eq!(current, base, "unflatten/restores-the-captured-hierarchy-over-the-flat-group: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the payload embeds a whole `DrawNode`, so the recursive `kind`-tagged encoding appears inside the mutation itself.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "unflatten/restores-the-captured-hierarchy-over-the-flat-group: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("unflatten mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("unflatten mutation reparses");
    assert_eq!(reencoded, original, "unflatten/restores-the-captured-hierarchy-over-the-flat-group: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the path resolves and the node genuinely differs from the captured original, so neither target-missing nor no-op may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "unflatten/restores-the-captured-hierarchy-over-the-flat-group: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "restoring a hierarchy that genuinely differs from the current node must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. One `children.modified` entry whose node diff is the REPLACE arm carrying the captured node verbatim.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "unflatten/restores-the-captured-hierarchy-over-the-flat-group: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed unflatten diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("the layers triple must be present");
    assert!(layers.removed.is_empty() && layers.added.is_empty(), "a node-level edit modifies its layer, never removes or re-adds it");
    let layer_diff = &layers.modified[0].diff;
    assert!(layer_diff.id.is_none() && layer_diff.name.is_none() && layer_diff.visible.is_none(), "a node-level edit must not touch the layer's own scalar fields");
    let root = layer_diff.root.as_ref().expect("the layer diff must carry a root node diff");
    let crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Group(root_group) = root else { panic!("the layer root is a group, so its diff must be the Group arm") };
    let children = root_group.children.as_ref().expect("the root group diff must carry a children triple");
    assert!(root_group.transform.is_none(), "editing a child must not rewrite the root group's own transform");
    assert_eq!(children.modified.len(), 1, "exactly one child is rewritten");
    assert!(matches!(children.modified[0].diff, crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawNodeDiff::Replace { .. }), "restoring a hierarchy goes through Replace, never through the per-field Group arm");
    assert!(children.removed.is_empty() && children.added.is_empty(), "the node keeps its position — only its contents are rewritten");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "unflatten/restores-the-captured-hierarchy-over-the-flat-group: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed unflatten diff decodes");
    let produced = decoded.apply(&before()).expect("committed unflatten diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "unflatten/restores-the-captured-hierarchy-over-the-flat-group: committed diff did not carry before to after");
}
