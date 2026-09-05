//! 🧪️ `sign-out` fixture — `🪪️clears-the-active-session`.
//!
//! `sign-out` is the identity config facet's second mutation kind and the only one that can produce
//! a signed-out record. Because `IdentitySetting` is `#[serde(transparent)]` over `Option<Identity>`,
//! the signed-out state is the bare JSON literal `null` — not `{}`, not an absent file — and this
//! fixture is the pin on that spelling.
//!
//! ↩️ `sign-out`'s inverse reads BASE and is therefore `sign-in(prior)` when a session was active
//! and EMPTY when it was not. This case takes the first branch: signing out of Ada's session must
//! undo to Ada's session, token and issue time included. The empty branch has no committed vector
//! because it produces no step to commit — it is pinned by the leaf's own unit test instead.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1).

use super::super::sign_in::{Identity, IdentitySetting};
use super::super::IdentityConfigMutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> IdentitySetting {
    serde_json::from_str(BEFORE).expect("before identity setting decodes")
}
fn expected_after() -> IdentitySetting {
    serde_json::from_str(AFTER).expect("after identity setting decodes")
}
fn mutation() -> IdentityConfigMutation {
    serde_json::from_str(MUTATION).expect("sign-out mutation decodes")
}
fn session(setting: &IdentitySetting) -> Identity {
    setting.0.clone().expect("the setting carries a session")
}

/// ▶️ Signing out clears the whole record: the committed after-setting holds no session at all, so
/// no field of the prior account can be read back off it.
#[test]
fn clears_the_whole_active_session() {
    let base = before();
    assert!(base.0.is_some(), "sign-out/clears-the-active-session: the before-setting must carry a session or the clear measures nothing");
    let outcome = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(&mutation(), &base);
    let applied = protocol::MutationDiff::apply(outcome.diff(), &base).expect("sign-out applies to its committed before-setting");
    assert_eq!(applied, expected_after(), "sign-out/clears-the-active-session: the cleared setting differs from the committed after-snapshot");
    assert!(applied.0.is_none(), "sign-out/clears-the-active-session: a signed-out record must hold no session");
    assert_eq!(applied, IdentitySetting::default(), "sign-out/clears-the-active-session: the signed-out record is the facet's own default");
}

/// ↩️ The inverse of clearing an ACTIVE session is `sign-in(prior)`, read off BASE, and it restores
/// the committed before-record exactly — token and issue time included.
#[test]
fn restoring_the_prior_session_restores_before() {
    let base = before();
    let inverse = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "sign-out/clears-the-active-session: clearing an active session undoes in exactly one step");
    let IdentityConfigMutation::SignIn(undo) = &inverse[0] else {
        panic!("sign-out/clears-the-active-session: undoing a sign-out must be a sign-in");
    };
    assert_eq!(Identity::from(undo), session(&base), "sign-out/clears-the-active-session: the undo must carry BASE's non-secret identity");
    let forward = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward sign-out applies");
    for step in &inverse {
        let redo = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(redo.diff(), &snapshot).expect("the sign-out inverse step applies");
    }
    assert_eq!(snapshot, base, "sign-out/clears-the-active-session: restoring the prior session did not restore the before-setting");
}

/// 🔣️ The committed JSON is canonical, and the signed-out record is the bare literal `null` —
/// the spelling `#[serde(transparent)]` over `Option<Identity>` forces.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: IdentitySetting = serde_json::from_str(text).expect("identity setting decodes");
        let reencoded = serde_json::to_value(decoded).expect("identity setting encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("identity setting reparses");
        assert_eq!(reencoded, original, "sign-out/clears-the-active-session: committed {label} setting JSON is not canonical");
    }
    assert_eq!(AFTER.trim(), "null", "sign-out/clears-the-active-session: a signed-out record is the bare literal null");
    let reencoded = serde_json::to_value(mutation()).expect("signOut payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("signOut payload reparses");
    assert_eq!(reencoded, original, "sign-out/clears-the-active-session: committed signOut JSON is not canonical");
    assert_eq!(original, serde_json::json!({ "mutation": "signOut" }), "sign-out/clears-the-active-session: the payload carries the tag and nothing else");
}

/// 🎯️ `sign-out` carries no guard, so clearing an active session is message-free and the declared
/// `applied` outcome must hold.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "sign-out/clears-the-active-session: this fixture declares an applied outcome");
    let produced = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "sign-out/clears-the-active-session: clearing an active session must not raise a diagnostic");
    assert!(produced.messages().is_empty(), "sign-out/clears-the-active-session: an accepted sign-out emits no diagnostics");
}

/// 🔺️ The committed diff is the whole post-op `IdentitySetting`, which for a sign-out is `null`.
#[test]
fn produces_committed_diff() {
    let outcome = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced sign-out diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "sign-out/clears-the-active-session: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `IdentitySetting` and re-encodes unchanged.
#[test]
fn committed_diff_is_canonical() {
    let decoded: IdentitySetting = serde_json::from_str(DIFF).expect("committed sign-out diff decodes");
    assert!(decoded.0.is_none(), "sign-out/clears-the-active-session: the committed diff must carry the cleared record");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "sign-out/clears-the-active-session: committed diff JSON is not canonical");
}

/// 🩹 The committed diff carries the before-setting to the after-setting — and because this facet's
/// `apply` ignores `base` outright, the diff IS the after-setting.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: IdentitySetting = serde_json::from_str(DIFF).expect("committed sign-out diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-setting");
    assert_eq!(produced, expected_after(), "sign-out/clears-the-active-session: committed diff did not carry before to after");
}
