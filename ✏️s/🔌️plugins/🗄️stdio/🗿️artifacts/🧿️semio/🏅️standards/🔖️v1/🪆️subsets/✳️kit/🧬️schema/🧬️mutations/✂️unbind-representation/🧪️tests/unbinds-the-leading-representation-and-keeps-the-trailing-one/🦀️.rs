//! 🧪️ `unbind-representation` fixture — `unbinds-the-leading-representation-and-keeps-the-trailing-one`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: `representations` is addressed BY INDEX (links
//! have no id of their own), so an out-of-range index is Error `mutation.target-missing`;
//! otherwise the list is rebuilt with that position removed. Unbinding index 0 out of two is the
//! case that renumbers what is left, which is what makes the inverse's index handling observable.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioKitSnapshot {
    serde_json::from_str(BEFORE).expect("unbind-representation before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("unbind-representation after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("unbind-representation mutation decodes")
}

/// ▶️ The leading link goes; the trailing one slides down to index 0.
#[semio_framework_async_macros::async_test]
async fn unbinds_the_link_at_index_zero() {
    let base = before();
    assert_eq!(base.representations.len(), 2, "the fixture needs a sibling link for the renumbering to be observable");
    let produced = mutation().diff(&base).diff().apply(&base).expect("unbind-representation applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "unbind-representation/unbinds-the-leading-representation-and-keeps-the-trailing-one: applied state differs from the committed after-snapshot");
    assert_eq!(produced.representations.len(), base.representations.len() - 1, "unbind-representation removes exactly one link");
    assert_eq!(produced.representations[0], base.representations[1], "the trailing link slides down into index 0");
    assert_eq!(produced.types, base.types, "unbinding must not touch the type the link named as its role");
}

/// ↩️ The undo restores the captured link AT ITS OWN INDEX, which for index 0 of a two-link pool
/// means re-declaring the tail rather than re-binding one link. `bind-representation` can only
/// append, so the single `bind` this test used to demand left the pool as `[trailing, leading]` —
/// the right two links in the wrong order — and the `assert_eq!(current, base)` below was the
/// assertion that caught it once `mutate-semio-kit`'s subject phase started running (ticket
/// 26/08/23/END-TO-END-TESTING-REFACTOR). The shape assertions now describe the real remedy: lift
/// the tail off with index-addressed unbinds, then re-declare it in order.
#[test]
fn the_undo_restores_the_captured_link_at_its_own_index() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    let tail = base.representations.len();
    assert_eq!(undo.len(), 2 * tail - 1, "undoing an unbind at index 0 lifts off the whole tail and re-declares it: {} unbind(s) then {} bind(s)", tail - 1, tail);
    assert!(matches!(undo[0], SemioKitMutation::UnbindRepresentation(_)), "the tail is lifted off first");
    let SemioKitMutation::BindRepresentation(rebind) = &undo[1] else { panic!("the escrowed link is re-declared first, as a bind-representation") };
    assert_eq!(rebind.target, base.representations[0].target, "the undo must recapture the unbound link's own target");
    assert_eq!(rebind.role, base.representations[0].role, "and its own role");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward unbind-representation applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("every undo step applies");
    }
    assert_eq!(current, base, "unbind-representation/unbinds-the-leading-representation-and-keeps-the-trailing-one: the undo did not restore the before-snapshot");
    assert_eq!(current.representations[0], base.representations[0], "and it put the escrowed link back at index 0, not at the end");
}

/// 🔣️ Snapshots and the `{"UnbindRepresentation":{"index":0}}` payload are canonical — links are index-addressed, so the payload carries a number rather than an id.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "unbind-representation/unbinds-the-leading-representation-and-keeps-the-trailing-one: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("unbind-representation mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("unbind-representation mutation reparses");
    assert_eq!(reencoded, original, "unbind-representation/unbinds-the-leading-representation-and-keeps-the-trailing-one: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: index 0 is within the two-link list, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "unbind-representation/unbinds-the-leading-representation-and-keeps-the-trailing-one: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "unbinding an in-range link must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `representations` slot, carrying the SHORTENED list.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "unbind-representation/unbinds-the-leading-representation-and-keeps-the-trailing-one: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed unbind-representation diff decodes");
    assert_eq!(decoded.representations.as_ref().map(|list| list.values.len()), Some(1), "the diff carries the shortened link list, not a removal marker");
    assert!(decoded.types.is_none() && decoded.designs.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.properties.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "unbind-representation/unbinds-the-leading-representation-and-keeps-the-trailing-one: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed unbind-representation diff decodes");
    let produced = decoded.apply(&before()).expect("committed unbind-representation diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "unbind-representation/unbinds-the-leading-representation-and-keeps-the-trailing-one: committed diff did not carry before to after");
}
