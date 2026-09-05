//! 🧪️ `sign-in` fixture — `🪪️replaces-the-active-session-with-a-second-account`.
//!
//! `sign-in` is one of the identity config facet's two mutation kinds: it establishes the OS-wide
//! session, and — the branch this case pins — REPLACES one that is already established. The facet's
//! `Diff` type is `IdentitySetting` itself, so the committed diff is the whole post-op record; its
//! `apply` ignores `base` outright, which is why a replacement can be expressed as a single step.
//!
//! ↩️ The inverse reads BASE, never the payload: replacing Ada's session with Grace's must undo to
//! `sign-in(Ada)`, not to `sign-out`. That distinction is the reason this fixture starts from an
//! ALREADY signed-in record — a fixture starting from `null` would invert to `sign-out` and leave
//! the replacement branch unmeasured.
//!
//! 🔣️ Shape note: `IdentitySetting` is `#[serde(transparent)]` over `Option<Identity>`, so a signed-in
//! record IS the bare `Identity` object and a signed-out one is bare `null`. `IdentityConfigMutation`
//! is internally tagged on `"mutation"` with camel-case variant names, and `SignIn` is a newtype
//! variant, so its fields sit beside the tag rather than nested under it.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1).

use super::{Identity, IdentityConfigMutation, IdentitySetting};

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
    serde_json::from_str(MUTATION).expect("sign-in mutation decodes")
}
fn session(setting: &IdentitySetting) -> Identity {
    setting.0.clone().expect("the setting carries a session")
}

/// ▶️ Signing in over an active session replaces the whole record — every field moves to the new
/// account, including the issue time, and nothing of the prior identity survives.
#[test]
fn replaces_the_whole_active_session() {
    let base = before();
    let outcome = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(&mutation(), &base);
    let applied = protocol::MutationDiff::apply(outcome.diff(), &base).expect("sign-in applies to its committed before-setting");
    assert_eq!(applied, expected_after(), "sign-in/replaces-the-active-session-with-a-second-account: the replaced session differs from the committed after-snapshot");
    assert_eq!(session(&applied).user_id, "grace", "sign-in/replaces-the-active-session-with-a-second-account: the payload's account must land verbatim on the setting");
    assert_ne!(session(&applied).email, session(&base).email, "sign-in/replaces-the-active-session-with-a-second-account: the prior account must not survive a replacement");
    assert_ne!(session(&applied).issued_at_ms, session(&base).issued_at_ms, "sign-in/replaces-the-active-session-with-a-second-account: the prior issue time must not survive a replacement");
}

/// ↩️ The inverse of replacing a session is `sign-in(prior)` — read off BASE — and it restores the
/// committed before-record exactly. A `sign-out` here would sign the operator out of an account
/// they were signed into before the mutation ran.
#[test]
fn restoring_the_prior_session_restores_before() {
    let base = before();
    let inverse = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "sign-in/replaces-the-active-session-with-a-second-account: exactly one undo step");
    let IdentityConfigMutation::SignIn(undo) = &inverse[0] else {
        panic!("sign-in/replaces-the-active-session-with-a-second-account: undoing a replacement must be a sign-in, never a sign-out");
    };
    assert_eq!(undo.user_id, "ada", "sign-in/replaces-the-active-session-with-a-second-account: the undo must carry BASE's own prior account");
    let forward = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward sign-in applies");
    for step in &inverse {
        let redo = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(redo.diff(), &snapshot).expect("the sign-in inverse step applies");
    }
    assert_eq!(snapshot, base, "sign-in/replaces-the-active-session-with-a-second-account: restoring the prior account did not restore the before-setting");
}

/// 🔣️ Both committed settings and the `signIn` payload are canonical: the setting is the bare
/// `Identity` object because `IdentitySetting` is transparent, and the payload's six fields sit
/// BESIDE the `"mutation"` tag rather than nested under it.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: IdentitySetting = serde_json::from_str(text).expect("identity setting decodes");
        let reencoded = serde_json::to_value(decoded).expect("identity setting encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("identity setting reparses");
        assert_eq!(reencoded, original, "sign-in/replaces-the-active-session-with-a-second-account: committed {label} setting JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("signIn payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("signIn payload reparses");
    assert_eq!(reencoded, original, "sign-in/replaces-the-active-session-with-a-second-account: committed signIn JSON is not canonical");
    assert_eq!(original.get("mutation").and_then(serde_json::Value::as_str), Some("signIn"), "sign-in/replaces-the-active-session-with-a-second-account: the payload must be internally tagged on \"mutation\"");
}

/// 🎯️ `sign-in` carries no guard, so an accepted replacement is message-free and the declared
/// `applied` outcome must hold.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "sign-in/replaces-the-active-session-with-a-second-account: this fixture declares an applied outcome");
    let produced = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "sign-in/replaces-the-active-session-with-a-second-account: establishing a different session must not raise a diagnostic");
    assert!(produced.messages().is_empty(), "sign-in/replaces-the-active-session-with-a-second-account: an accepted sign-in emits no diagnostics");
}

/// 🔺️ The committed diff is the whole post-op `IdentitySetting` — this facet's declared `Diff` type
/// is the record itself, so the diff carries the new session outright rather than a delta.
#[test]
fn produces_committed_diff() {
    let outcome = <IdentityConfigMutation as protocol::Mutation<IdentitySetting>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced sign-in diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "sign-in/replaces-the-active-session-with-a-second-account: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `IdentitySetting` and re-encodes unchanged.
#[test]
fn committed_diff_is_canonical() {
    let decoded: IdentitySetting = serde_json::from_str(DIFF).expect("committed sign-in diff decodes");
    assert_eq!(session(&decoded).email, "grace@studio.example", "sign-in/replaces-the-active-session-with-a-second-account: the committed diff must carry the new session");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "sign-in/replaces-the-active-session-with-a-second-account: committed diff JSON is not canonical");
}

/// 🩹 The committed diff carries the before-setting to the after-setting — and because this facet's
/// `apply` ignores `base` outright, the diff IS the after-setting.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: IdentitySetting = serde_json::from_str(DIFF).expect("committed sign-in diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-setting");
    assert_eq!(produced, expected_after(), "sign-in/replaces-the-active-session-with-a-second-account: committed diff did not carry before to after");
}
