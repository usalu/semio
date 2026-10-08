//! 🧪️ `change-merge-policy` fixture — `🪪️tightens-the-authority-to-vigilant`.
//!
//! `change-merge-policy` is the merge-policy config facet's one mutation kind: it sets
//! `os.config.merge-policy`'s single `policy` field. Its diff oracle has exactly one guard — the
//! same policy is already active ⇒ Warning `mutation.no-op` carrying an EMPTY `MergePolicyDiff`, which the central
//! applier leaves the active policy under: a `Vigilant` authority is never loosened back to `Normal` by a no-op.
//! This case takes the other branch — `Normal` → `Vigilant`, tightening quarantine from "reject
//! Error and worse" to "reject Warning and worse".
//!
//! 🛡️ Shape note: the sparse `MergePolicyDiff` carries the absolute new policy, and `MergePolicy` carries NO
//! `rename_all`, so the wire spellings are the PascalCase variant names — `"Vigilant"`, not
//! `"vigilant"`. This fixture is the pin on that.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use super::{MergePolicyConfigMutation, MergePolicyDiff, MergePolicySetting};

const BEFORE: &str = include_str!("../../🧫️fixtures/🪪️tightens-the-authority-to-vigilant/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/🪪️tightens-the-authority-to-vigilant/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/🪪️tightens-the-authority-to-vigilant/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/🪪️tightens-the-authority-to-vigilant/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/🪪️tightens-the-authority-to-vigilant/🎯️outcome/🔣️.json");

fn before() -> MergePolicySetting {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before merge-policy setting decodes")
}
fn expected_after() -> MergePolicySetting {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after merge-policy setting decodes")
}
fn mutation() -> MergePolicyConfigMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("change-merge-policy mutation decodes")
}
fn json_value<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}

/// ▶️ Tightening to `Vigilant` replaces the whole setting record; the authority now quarantines
/// outcomes whose worst level is merely a `Warning`.
#[test]
fn tightens_the_active_policy() {
    let base = before();
    let outcome = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("change-merge-policy applies to its committed before-setting");
    assert_eq!(applied, expected_after(), "change-merge-policy/tightens-the-authority-to-vigilant: the tightened setting differs from the committed after-snapshot");
    assert_eq!(applied.policy, protocol::MergePolicy::Vigilant, "change-merge-policy/tightens-the-authority-to-vigilant: the payload's policy must land verbatim on the setting");
    assert!(applied.policy.rejects(semio_framework_diagnostic::Severity::Warning), "change-merge-policy/tightens-the-authority-to-vigilant: Vigilant is the only policy that quarantines a Warning");
}

/// ↩️ The inverse reads BASE's policy — never the diff — so undoing hands the authority back to
/// `Normal`, the default a never-configured authority starts on.
#[test]
fn restoring_the_prior_policy_restores_before() {
    let base = before();
    let inverse = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "change-merge-policy/tightens-the-authority-to-vigilant: exactly one undo step");
    let MergePolicyConfigMutation::ChangeMergePolicy(undo) = &inverse[0];
    assert_eq!(undo.policy, protocol::MergePolicy::Normal, "change-merge-policy/tightens-the-authority-to-vigilant: the undo must carry BASE's own prior policy");
    let forward = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff(forward.diff(), &base).expect("forward change-merge-policy applies");
    for step in &inverse {
        let redo = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(redo.diff(), &snapshot).expect("the change-merge-policy inverse step applies");
    }
    assert_eq!(snapshot, base, "change-merge-policy/tightens-the-authority-to-vigilant: restoring Normal did not restore the before-setting");
}

/// 🔣️ Both committed settings and the `changeMergePolicy` payload are canonical — the payload is
/// internally tagged on `"mutation"`, while its `policy` value keeps the PascalCase variant name.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MergePolicySetting = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("merge-policy setting decodes");
        let reencoded = json_value(&decoded);
        let original: serde_json::Value = serde_json::from_str(text).expect("merge-policy setting reparses");
        assert_eq!(reencoded, original, "change-merge-policy/tightens-the-authority-to-vigilant: committed {label} setting JSON is not canonical");
    }
    let reencoded = json_value(&mutation());
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("changeMergePolicy payload reparses");
    assert_eq!(reencoded, original, "change-merge-policy/tightens-the-authority-to-vigilant: committed changeMergePolicy JSON is not canonical");
}

/// 🎯️ `Vigilant` differs from the active `Normal`, so the single equality guard does not fire and
/// the declared `applied` outcome must be message-free.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-merge-policy/tightens-the-authority-to-vigilant: this fixture declares an applied outcome");
    let produced = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "change-merge-policy/tightens-the-authority-to-vigilant: changing to a different policy must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "change-merge-policy/tightens-the-authority-to-vigilant: an accepted policy change emits no diagnostics");
}

/// 🔺️ The produced sparse diff is the committed `🔺️diff`.
#[test]
fn produces_committed_diff() {
    let outcome = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(&mutation(), &before());
    let produced = json_value(outcome.diff());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-merge-policy/tightens-the-authority-to-vigilant: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to the facet's sparse diff type and re-encodes unchanged.
#[test]
fn committed_diff_is_canonical() {
    let decoded: MergePolicyDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed change-merge-policy diff decodes");
    assert_eq!(decoded.policy, Some(protocol::MergePolicy::Vigilant), "change-merge-policy/tightens-the-authority-to-vigilant: the committed diff must carry the tightened policy");
    let reencoded = json_value(&decoded);
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-merge-policy/tightens-the-authority-to-vigilant: committed diff JSON is not canonical");
}

/// 🩹 The committed diff carries the before-setting to the after-setting through the central applier.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: MergePolicyDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed change-merge-policy diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-setting");
    assert_eq!(produced, expected_after(), "change-merge-policy/tightens-the-authority-to-vigilant: committed diff did not carry before to after");
}

/// ➕️ The concrete inverse's diffs sum to the negative of the forward diff (L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
