//! 🧪️ `set-default-app` fixture — `repins-the-cad-editor-to-the-drafting-app`.
//!
//! `set-default-app` pins one `(dialect, role) -> app` default. Its diff oracle guards the exact
//! triple (`dialect == && role == && app ==` ⇒ Warning `mutation.no-op`) and otherwise FILTERS OUT
//! any prior entry for the same `(dialect, role)` before appending the new pin — so a re-pin is a
//! remove-then-append, never an in-place edit, and the re-pinned entry moves to the tail of
//! `defaults`. The sibling `viewer` pin for the very same dialect must survive untouched: the
//! coordinate is the PAIR, not the dialect alone.
//!
//! 🎚️ Shape note: this config facet is small enough that its `Mutation::Diff` IS `OpeningPreferences`
//! itself — a whole-record diff whose `apply` ignores `base` entirely — so the committed
//! `../✏️repins-the-cad-editor-to-the-drafting-app/🔺️diff/🔣️.json` is the full post-op preferences record, not a sparse per-field delta.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use super::super::super::OpeningPreferences;
use super::super::OpeningConfigMutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> OpeningPreferences {
    dsl::os_pack::json::from_json_str(BEFORE).expect("before opening preferences decode")
}
fn expected_after() -> OpeningPreferences {
    dsl::os_pack::json::from_json_str(AFTER).expect("after opening preferences decode")
}
fn mutation() -> OpeningConfigMutation {
    dsl::os_pack::json::from_json_str(MUTATION).expect("set-default-app mutation decodes")
}
fn json_value<T: dsl::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&dsl::os_pack::json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}

/// ▶️ Re-pinning the cad editor to the drafting plugin replaces that one entry and leaves the
/// viewer pin for the same dialect in place.
#[test]
fn repins_the_editor_and_keeps_the_viewer_pin() {
    let base = before();
    let outcome = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &base);
    let applied = protocol::MutationDiff::apply(outcome.diff(), &base).expect("set-default-app applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "set-default-app/repins-the-cad-editor-to-the-drafting-app: the re-pinned preferences differ from the committed after-snapshot");
    assert_eq!(applied.defaults.len(), base.defaults.len(), "set-default-app/repins-the-cad-editor-to-the-drafting-app: re-pinning an occupied coordinate must replace, never accumulate a second entry");
    assert_eq!(applied.defaults.last().map(|entry| entry.app.plugin_id.as_str()), Some("drafting"), "set-default-app/repins-the-cad-editor-to-the-drafting-app: the new pin lands at the tail, since the oracle filters then appends");
}

/// ↩️ `set-default-app`'s inverse reads BASE's entry for the same `(dialect, role)`: a prior pin
/// exists here, so the undo is another `set-default-app` carrying the cad plugin back — never the
/// `clear-default-app` the oracle would emit for a previously unpinned coordinate.
#[test]
fn repinning_the_prior_app_restores_before() {
    let base = before();
    let inverse = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "set-default-app/repins-the-cad-editor-to-the-drafting-app: exactly one undo step");
    assert!(matches!(inverse[0], OpeningConfigMutation::SetDefaultApp(_)), "set-default-app/repins-the-cad-editor-to-the-drafting-app: an occupied coordinate must undo via set-default-app, not clear-default-app");
    let forward = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward set-default-app applies");
    for step in &inverse {
        let undo = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the set-default-app inverse step applies");
    }
    assert_eq!(snapshot, base, "set-default-app/repins-the-cad-editor-to-the-drafting-app: re-pinning the cad editor did not restore the before-preferences");
}

/// 🔣️ Both committed preference records and the `setDefaultApp` payload are canonical — `role`
/// rides as the camelCase wire spelling `"editor"`, and the payload is internally tagged.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: OpeningPreferences = dsl::os_pack::json::from_json_str(text).expect("opening preferences decode");
        let reencoded = json_value(&decoded);
        let original: serde_json::Value = serde_json::from_str(text).expect("opening preferences reparse");
        assert_eq!(reencoded, original, "set-default-app/repins-the-cad-editor-to-the-drafting-app: committed {label} preferences JSON is not canonical");
    }
    let reencoded = json_value(&mutation());
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("setDefaultApp payload reparses");
    assert_eq!(reencoded, original, "set-default-app/repins-the-cad-editor-to-the-drafting-app: committed setDefaultApp JSON is not canonical");
}

/// 🎯️ The drafting app is not already pinned for `(cad, editor)`, so the exact-triple no-op guard
/// does not fire and the declared `applied` outcome must be message-free.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-default-app/repins-the-cad-editor-to-the-drafting-app: this fixture declares an applied outcome");
    let produced = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "set-default-app/repins-the-cad-editor-to-the-drafting-app: pinning a different app must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "set-default-app/repins-the-cad-editor-to-the-drafting-app: an accepted pin emits no diagnostics");
}

/// 🔺️ The committed diff is the whole post-op `OpeningPreferences` record — this facet's declared
/// `Diff` type — and it must carry BOTH entries, the untouched viewer pin included.
#[test]
fn produces_committed_diff() {
    let outcome = <OpeningConfigMutation as protocol::Mutation<OpeningPreferences>>::diff(&mutation(), &before());
    let produced = json_value(outcome.diff());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-default-app/repins-the-cad-editor-to-the-drafting-app: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `OpeningPreferences` and re-encodes unchanged.
#[test]
fn committed_diff_is_canonical() {
    let decoded: OpeningPreferences = dsl::os_pack::json::from_json_str(DIFF).expect("committed set-default-app diff decodes");
    assert_eq!(decoded.defaults.len(), 2, "set-default-app/repins-the-cad-editor-to-the-drafting-app: the whole-record diff must restate every surviving pin, not just the changed one");
    let reencoded = json_value(&decoded);
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-default-app/repins-the-cad-editor-to-the-drafting-app: committed diff JSON is not canonical");
}

/// 🩹 The committed diff carries the before-record to the after-record — and because this facet's
/// `apply` ignores `base` outright, the diff IS the after-record.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: OpeningPreferences = dsl::os_pack::json::from_json_str(DIFF).expect("committed set-default-app diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-preferences");
    assert_eq!(produced, expected_after(), "set-default-app/repins-the-cad-editor-to-the-drafting-app: committed diff did not carry before to after");
}
