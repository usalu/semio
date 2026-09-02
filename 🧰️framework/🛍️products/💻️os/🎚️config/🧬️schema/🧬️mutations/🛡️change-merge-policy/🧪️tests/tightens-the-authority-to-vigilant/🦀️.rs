//! 🧪️ `change-merge-policy` fixture — `tightens-the-authority-to-vigilant`.
//!
//! `change-merge-policy` is the merge-policy config facet's one mutation kind: it sets
//! `os.config.merge-policy`'s single `policy` field. Its diff oracle has exactly one guard — the
//! same policy is already active ⇒ Warning `mutation.no-op` carrying the UNCHANGED base record back.
//! That guard uses `MutationOutcome::new(*base)` rather than `MutationOutcome::empty()` precisely
//! because this facet's diff IS the whole record: an "empty" diff would apply as
//! `MergePolicySetting::default()` and silently loosen a `Vigilant` authority back to `Normal`.
//! This case takes the other branch — `Normal` → `Vigilant`, tightening quarantine from "reject
//! Error and worse" to "reject Warning and worse".
//!
//! 🛡️ Shape note: `MergePolicySetting` is its own `Mutation::Diff`, and `MergePolicy` carries NO
//! `rename_all`, so the wire spellings are the PascalCase variant names — `"Vigilant"`, not
//! `"vigilant"`. This fixture is the pin on that.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use super::{MergePolicyConfigMutation, MergePolicySetting};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> MergePolicySetting {
    serde_json::from_str(BEFORE).expect("before merge-policy setting decodes")
}
fn expected_after() -> MergePolicySetting {
    serde_json::from_str(AFTER).expect("after merge-policy setting decodes")
}
fn mutation() -> MergePolicyConfigMutation {
    serde_json::from_str(MUTATION).expect("change-merge-policy mutation decodes")
}

/// ▶️ Tightening to `Vigilant` replaces the whole setting record; the authority now quarantines
/// outcomes whose worst level is merely a `Warning`.
#[test]
fn tightens_the_active_policy() {
    let base = before();
    let outcome = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(&mutation(), &base);
    let applied = protocol::MutationDiff::apply(outcome.diff(), &base).expect("change-merge-policy applies to its committed before-setting");
    assert_eq!(applied, expected_after(), "change-merge-policy/tightens-the-authority-to-vigilant: the tightened setting differs from the committed after-snapshot");
    assert_eq!(applied.policy, protocol::MergePolicy::Vigilant, "change-merge-policy/tightens-the-authority-to-vigilant: the payload's policy must land verbatim on the setting");
    assert!(applied.policy.rejects(protocol::Severity::Warning), "change-merge-policy/tightens-the-authority-to-vigilant: Vigilant is the only policy that quarantines a Warning");
}

/// ↩️ The inverse reads BASE's policy — never the diff — so undoing hands the authority back to
/// `Normal`, the default a never-configured authority starts on.
#[test]
fn restoring_the_prior_policy_restores_before() {
    let base = before();
    let inverse = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-merge-policy/tightens-the-authority-to-vigilant: exactly one undo step");
    let MergePolicyConfigMutation::ChangeMergePolicy(undo) = &inverse[0];
    assert_eq!(undo.policy, protocol::MergePolicy::Normal, "change-merge-policy/tightens-the-authority-to-vigilant: the undo must carry BASE's own prior policy");
    let forward = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward change-merge-policy applies");
    for step in &inverse {
        let redo = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(redo.diff(), &snapshot).expect("the change-merge-policy inverse step applies");
    }
    assert_eq!(snapshot, base, "change-merge-policy/tightens-the-authority-to-vigilant: restoring Normal did not restore the before-setting");
}

/// 🔣️ Both committed settings and the `changeMergePolicy` payload are canonical — the payload is
/// internally tagged on `"mutation"`, while its `policy` value keeps the PascalCase variant name.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MergePolicySetting = serde_json::from_str(text).expect("merge-policy setting decodes");
        let reencoded = serde_json::to_value(decoded).expect("merge-policy setting encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("merge-policy setting reparses");
        assert_eq!(reencoded, original, "change-merge-policy/tightens-the-authority-to-vigilant: committed {label} setting JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("changeMergePolicy payload encodes");
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

/// 🔺️ The committed diff is the whole post-op `MergePolicySetting` — this facet's declared `Diff`
/// type is the record itself, so the diff carries the new policy outright rather than a delta.
#[test]
fn produces_committed_diff() {
    let outcome = <MergePolicyConfigMutation as protocol::Mutation<MergePolicySetting>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced change-merge-policy diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-merge-policy/tightens-the-authority-to-vigilant: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `MergePolicySetting` and re-encodes unchanged.
#[test]
fn committed_diff_is_canonical() {
    let decoded: MergePolicySetting = serde_json::from_str(DIFF).expect("committed change-merge-policy diff decodes");
    assert_eq!(decoded.policy, protocol::MergePolicy::Vigilant, "change-merge-policy/tightens-the-authority-to-vigilant: the committed diff must carry the tightened policy");
    let reencoded = serde_json::to_value(decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-merge-policy/tightens-the-authority-to-vigilant: committed diff JSON is not canonical");
}

/// 🩹 The committed diff carries the before-setting to the after-setting — and because this facet's
/// `apply` ignores `base` outright, the diff IS the after-setting.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: MergePolicySetting = serde_json::from_str(DIFF).expect("committed change-merge-policy diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-setting");
    assert_eq!(produced, expected_after(), "change-merge-policy/tightens-the-authority-to-vigilant: committed diff did not carry before to after");
}
