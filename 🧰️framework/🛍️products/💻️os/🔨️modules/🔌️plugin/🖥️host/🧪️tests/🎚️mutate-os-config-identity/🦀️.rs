//! 🦀️ Identity exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `os-config-identity-mutation-semantics` (`../../../../../🎚️config/🔣️oracle.json`):
//! `os.config.identity` is this operating system's own session record with no third-party
//! implementation, so `oracle` here reads the committed, independently handcrafted per-kind
//! specification fixtures
//! (`../../../../../🎚️config/🧬️schema/🧬️mutations/<slug>/🧪️tests/<fixture>/`) literally — no
//! recomputation, no reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_identity_config_mutation_reporting` over the full two-kind `IdentityConfigMutation`
//! vocabulary.
//!
//! **Why the case sits here rather than beside the vocabulary.** `🎚️config` has no crate of its
//! own; `../../🧪️tests/🎚️mutate-os-config-identity/🦀️.rs` in `🖥️host/📦️packages/🦀️rust` is the ONE place `IdentityConfigMutation` is
//! mounted, and the generated test host resolves its subject crate by walking UP from the case
//! owner. The feature says the same thing in prose so a reader is not left guessing.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors the derive-generated `IdentityConfigMutation` descriptor order — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog.
const KINDS: &[&str] = &["sign-in", "sign-out"];
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind,
/// read literally via `include_str!` — this IS the independently handcrafted vector the no-oracle
/// decision rests on, never recomputed.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "sign-in" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🧪️tests/🪪️replaces-the-active-session-with-a-second-account/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🧪️tests/🪪️replaces-the-active-session-with-a-second-account/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🧪️tests/🪪️replaces-the-active-session-with-a-second-account/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🧪️tests/🪪️replaces-the-active-session-with-a-second-account/🎯️outcome/🔣️.json"),
        ),
        "sign-out" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🧪️tests/🪪️clears-the-active-session/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🧪️tests/🪪️clears-the-active-session/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🧪️tests/🪪️clears-the-active-session/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🧪️tests/🪪️clears-the-active-session/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-os-config-identity: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER record, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE record — undoing either kind must return
/// to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}

/// 🔮️ The empty-inverse guard's reference answer: the committed signed-out record. Signing out of
/// a record that already holds no session leaves it exactly where it was.
fn signed_out_guard_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (_before, _mutation, after, _outcome) = fixture_text("sign-out");
    Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
}

/// 🔁️ The identity carrier's reference answer: the committed signed-in record itself.
fn round_trip_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (before, _mutation, _after, _outcome) = fixture_text("sign-in");
    Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_plugin_host::opening_config::mutations::{
        apply_identity_config_mutation_reporting, decode_identity_config_mutation_json, decode_identity_setting_json, encode_identity_setting_json, inverse_identity_config_mutation_steps, IdentityConfigMutation, IdentitySetting,
    };
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️FixtureDecode
    fn record_of(text: &str, label: &str, kind: &str) -> Result<IdentitySetting, String> {
        decode_identity_setting_json(text).map_err(|error| format!("mutate-os-config-identity: the committed {label}-record for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<IdentityConfigMutation, String> {
        decode_identity_config_mutation_json(text).map_err(|error| format!("mutate-os-config-identity: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(record: &IdentitySetting) -> Result<Json, String> {
        parse_json(&encode_identity_setting_json(record))
    }

    fn disagreement(what: &str, got: &IdentitySetting, expected: &IdentitySetting) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_identity_setting_json(got), encode_identity_setting_json(expected))
    }

    /// 🔎️ The signed-in account id, read off the TYPED value rather than off the text — which is
    /// what makes the account claim and the identity scenario statements about a real decode. A
    /// signed-out record answers the feature's own spelling, `none`.
    fn account(record: &IdentitySetting) -> String {
        record.0.as_ref().map_or_else(|| "none".to_string(), |identity| identity.user_id.clone())
    }

    /// 🔎️ The session token, read off the typed value — the field an undo is most likely to
    /// fabricate rather than restore.
    fn token(record: &IdentitySetting) -> Option<String> {
        record.0.as_ref().map(|identity| identity.email.clone())
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law in the exact form a whole-record vocabulary can state it: the
    /// declared account is the one now held AND the account the record used to hold is gone.
    /// Comparing the whole document against a fixture written from the same bug would pass an
    /// implementation that simply returned the fixture; naming both endpoints in the row would not.
    fn account_claim_holds(kind: &str, declared: &str, was: &str, base: &IdentitySetting, after: &IdentitySetting) -> Result<(), String> {
        if account(base) != was {
            return Err(format!("mutate-{kind}: the feature declares the record starts signed in as {was:?}, but the committed before-record holds {:?}", account(base)));
        }
        if base == after {
            return Err(format!("mutate-{kind}: the mutation left the identity record unchanged — the scenario would report a pass for a mutation it never observed"));
        }
        if account(after) != declared {
            return Err(format!("mutate-{kind}: the feature declares the record holds {declared:?} afterwards, but it holds {:?}", account(after)));
        }
        if declared != "none" && token(after) == token(base) {
            return Err(format!("mutate-{kind}: a replaced session must carry its own token, but the prior session's token survived"));
        }
        Ok(())
    }

    /// 🎯️ The committed outcome claim: both vectors are clean `applied` vectors, so any diagnostic
    /// at all is a divergence.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        if declared.str("status") != "applied" {
            return Err(format!("mutate-{kind}: both committed identity vectors are clean applied vectors, but this one declares {:?}", declared.str("status")));
        }
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-record and asserts, in role, that the result IS
    /// the committed after-record, that the declared account really moved, and that the reported
    /// diagnostics are the committed ones.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, after, outcome) = super::fixture_text(kind);
            let base = record_of(before, "before", kind)?;
            let expected = record_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_identity_config_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied record does not match the committed after-record"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(outcome)?, &raised)?;
            account_claim_holds(kind, &row.str("account"), &row.str("wasAccount"), &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-record exactly — token and issue time included. Both kinds read
    /// their undo off BASE, so the account they put back is one the mutation never travelled with.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = record_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_identity_config_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the record untouched, so restoring it proves nothing"));
            }
            for step in inverse_identity_config_mutation_steps(&mutation, &base) {
                let undone = apply_identity_config_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            if account(&current) != row.str("wasAccount") {
                return Err(format!("inverse-{kind}: the restored record must hold {:?} once more, but it holds {:?}", row.str("wasAccount"), account(&current)));
            }
            if token(&current) != token(&base) {
                return Err(format!("inverse-{kind}: the undo restored the account but fabricated a session token"));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🚧️ The branch no committed vector can express, because it produces nothing to commit: an
    /// inverse read off a record that holds no session must be EMPTY. A vocabulary that answered
    /// with a `sign-in` here would have to invent an account, and undoing a no-op would sign the
    /// operator in as somebody.
    pub fn signed_out_guard(_ctx: &Context) -> Result<Outcome, String> {
        let (_before, mutation, after, _outcome) = super::fixture_text("sign-out");
        let base = record_of(after, "after", "sign-out")?;
        if base.0.is_some() {
            return Err("signed-out-inverse-is-empty: the committed after-record of the sign-out vector must hold no session".to_string());
        }
        let mutation = mutation_of(mutation, "sign-out")?;
        let mut current = base.clone();
        let raised = apply_identity_config_mutation_reporting(&mut current, &mutation);
        if !raised.is_empty() {
            return Err(format!("signed-out-inverse-is-empty: signing out of a signed-out record must be accepted silently, but the implementation raised {raised:?}"));
        }
        if current != base {
            return Err(disagreement("signed-out-inverse-is-empty: signing out of a signed-out record must leave it exactly where it was", &current, &base));
        }
        let steps = inverse_identity_config_mutation_steps(&mutation, &base);
        if !steps.is_empty() {
            return Err(format!("signed-out-inverse-is-empty: the inverse must be empty, but the implementation offered {} step(s)", steps.len()));
        }
        let projection = projection(&current)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    /// 🔁️ The identity law for a record whose only carrier is its own JSON projection. `os.config`
    /// has no `.dsl.semio` or `.pack.semio` form, so the honest statement is a decode/re-encode that
    /// must reproduce the committed projection exactly. The decode is proven real by reading the
    /// account AND its token back off the TYPED value: a shortcut that handed the input text back
    /// would satisfy the projection comparison and fail the moment `account()` asks the typed value.
    pub fn round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let (before, _mutation, _after, _outcome) = super::fixture_text("sign-in");
        let record = record_of(before, "before", "sign-in")?;
        if account(&record) != "ada" || token(&record).as_deref() != Some("session-ada-0001") {
            return Err(format!("identity-round-trip: the committed record holds Ada's session, but the decoded value holds {}", encode_identity_setting_json(&record)));
        }
        let reencoded = encode_identity_setting_json(&record);
        let reparsed = record_of(&reencoded, "re-encoded", "sign-in")?;
        if reparsed != record {
            return Err(disagreement("identity-round-trip: decoding the re-encoded record did not reproduce the typed value", &reparsed, &record));
        }
        let projection = parse_json(&reencoded)?;
        Ok(Outcome::with_raw(reencoded.into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("signed-out-inverse-is-empty", signed_out_guard_oracle).oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("signed-out-inverse-is-empty", subject::signed_out_guard).subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
