//! 🧪️ `create-layer` fixture — `🟪️inserts-a-second-layer-above-the-base-layer`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a duplicate layer id is FATAL
//! `mutation.duplicate-id`; otherwise the layer lands at `min(index, layers.len())` as an
//! `IndexAdded` entry in the `layers` triple. `layers` is INDEX-keyed (z-order matters) even though
//! a layer also carries an id, so the diff transports a position, not a name.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioDrawingSnapshot {
    serde_json::from_str(BEFORE).expect("create-layer before snapshot decodes")
}
fn expected_after() -> SemioDrawingSnapshot {
    serde_json::from_str(AFTER).expect("create-layer after snapshot decodes")
}
fn mutation() -> SemioDrawingMutation {
    serde_json::from_str(MUTATION).expect("create-layer mutation decodes")
}

/// ▶️ The overlay layer lands on top; the base layer and every style stay put.
#[semio_framework_async_macros::async_test]
async fn inserts_the_overlay_layer_on_top() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-layer applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-layer/inserts-a-second-layer-above-the-base-layer: applied state differs from the committed after-snapshot");
    assert_eq!(produced.layers.len(), base.layers.len() + 1, "create-layer adds exactly one layer");
    assert_eq!(produced.layers[1].id, "l2", "the new layer occupies the requested z-order slot");
    assert!(!produced.layers[1].visible, "the payload's own visibility flag lands, not a default");
    assert_eq!(produced.layers[0], base.layers[0], "the pre-existing layer must be byte-identical");
    assert_eq!(produced.styles, base.styles, "creating a layer must not touch the style table");
}

/// ↩️ `create-layer`'s undo is a single `delete-layer` addressed by ID, not by index.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_layer_removes_the_overlay_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-layer undoes as exactly one delete-layer");
    let SemioDrawingMutation::DeleteLayer(remove) = &undo[0] else { panic!("create-layer must undo as delete-layer") };
    assert_eq!(remove.id, "l2", "the undo addresses the layer by ID even though the forward diff used a position");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-layer applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-layer applies");
    }
    assert_eq!(current, base, "create-layer/inserts-a-second-layer-above-the-base-layer: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `DrawNode` is internally tagged on `kind`, so a group root encodes as `{"kind":"group","transform":…,"children":[…]}`, and `DrawCanvas.background` is omitted entirely when unset.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioDrawingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-layer/inserts-a-second-layer-above-the-base-layer: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-layer mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-layer mutation reparses");
    assert_eq!(reencoded, original, "create-layer/inserts-a-second-layer-above-the-base-layer: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no layer with id l2 exists, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-layer/inserts-a-second-layer-above-the-base-layer: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a layer with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `layers.added`, carrying an index.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-layer/inserts-a-second-layer-above-the-base-layer: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and the `canvas` slot — which NO leaf in
/// this subset ever writes — stays absent from it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed create-layer diff decodes");
    assert!(decoded.canvas.is_none(), "no drawing mutation writes the canvas slot");
    let layers = decoded.layers.as_ref().expect("create-layer must write the layers triple");
    assert_eq!(layers.added.len(), 1, "exactly one layer is added");
    assert_eq!(layers.added[0].index, 1, "the add carries its z-order POSITION");
    assert!(layers.removed.is_empty() && layers.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.styles.is_none(), "the style table must stay untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-layer/inserts-a-second-layer-above-the-base-layer: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDrawingDiff = serde_json::from_str(DIFF).expect("committed create-layer diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-layer diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-layer/inserts-a-second-layer-above-the-base-layer: committed diff did not carry before to after");
}
