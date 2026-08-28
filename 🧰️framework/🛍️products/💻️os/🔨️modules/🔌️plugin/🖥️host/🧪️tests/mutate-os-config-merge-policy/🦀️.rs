//! 🦀️ Merge-policy exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `os-config-merge-policy-mutation-semantics`
//! (`../../../../../🎚️config/🧪️oracle/🔣️.json`): `os.config.merge-policy` is this operating
//! system's own authority configuration with no third-party implementation, so `oracle` here reads
//! the committed, independently handcrafted specification fixture
//! (`../../../../../🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/<fixture>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_merge_policy_config_mutation_reporting` over the whole one-kind
//! `MergePolicyConfigMutation` vocabulary.
//!
//! **Why the case sits here rather than beside the vocabulary.** `🎚️config` has no crate of its
//! own; `📦️glue.rs` in `🖥️host/📦️packages/🦀️rust` is the ONE place `MergePolicyConfigMutation` is
//! mounted, and the generated test host resolves its subject crate by walking UP from the case
//! owner. The feature says the same thing in prose so a reader is not left guessing.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors the derive-generated `MergePolicyConfigMutation` descriptor order — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog.
const KINDS: &[&str] = &["change-merge-policy"];
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind,
/// read literally via `include_str!` — this IS the independently handcrafted vector the no-oracle
/// decision rests on, never recomputed.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-merge-policy" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/tightens-the-authority-to-vigilant/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/tightens-the-authority-to-vigilant/🦠️mutation/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/tightens-the-authority-to-vigilant/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🧪️tests/tightens-the-authority-to-vigilant/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-os-config-merge-policy: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER setting, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE setting — undoing the kind must return to
/// exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}

/// 🔮️ The guard's reference answer: the committed BEFORE setting again. Re-applying the policy the
/// record already carries must hand that record back UNCHANGED — not the facet default.
fn no_op_guard_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (before, _mutation, _after, _outcome) = fixture_text("change-merge-policy");
    Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
}

/// 🔁️ The identity carrier's reference answer: the committed setting itself.
fn round_trip_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (before, _mutation, _after, _outcome) = fixture_text("change-merge-policy");
    Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_plugin_host::opening_config::mutations::{
        apply_merge_policy_config_mutation_reporting, decode_merge_policy_config_mutation_json, decode_merge_policy_setting_json, encode_merge_policy_setting_json, inverse_merge_policy_config_mutation_steps, ChangeMergePolicy,
        MergePolicyConfigMutation, MergePolicySetting,
    };
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️FixtureDecode
    fn setting_of(text: &str, label: &str, kind: &str) -> Result<MergePolicySetting, String> {
        decode_merge_policy_setting_json(text).map_err(|error| format!("mutate-os-config-merge-policy: the committed {label}-setting for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<MergePolicyConfigMutation, String> {
        decode_merge_policy_config_mutation_json(text).map_err(|error| format!("mutate-os-config-merge-policy: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(setting: &MergePolicySetting) -> Result<Json, String> {
        parse_json(&encode_merge_policy_setting_json(setting))
    }

    fn disagreement(what: &str, got: &MergePolicySetting, expected: &MergePolicySetting) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_merge_policy_setting_json(got), encode_merge_policy_setting_json(expected))
    }

    /// 🔎️ The active policy NAME, read off the TYPED value rather than off the text — which is what
    /// makes the observability claim and the identity scenario statements about a real decode.
    fn active(setting: &MergePolicySetting) -> String {
        format!("{:?}", setting.policy)
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law in the exact form a single-field record can state it: the declared
    /// policy is now active AND the policy the record used to carry is gone. Comparing the whole
    /// document against a fixture written from the same bug would pass an implementation that
    /// simply returned the fixture; naming both endpoints in the feature row would not.
    fn policy_claim_holds(kind: &str, policy: &str, was: &str, base: &MergePolicySetting, after: &MergePolicySetting) -> Result<(), String> {
        if active(base) != was {
            return Err(format!("mutate-{kind}: the feature declares the record starts on {was:?}, but the committed before-setting carries {:?}", active(base)));
        }
        if base == after {
            return Err(format!("mutate-{kind}: the mutation left the setting unchanged — the scenario would report a pass for a mutation it never observed"));
        }
        if active(after) != policy {
            return Err(format!("mutate-{kind}: the feature declares {policy:?} is active afterwards, but the setting carries {:?}", active(after)));
        }
        Ok(())
    }

    /// 🎯️ The committed outcome claim: this vector is a clean `applied` vector, so any diagnostic at
    /// all is a divergence — including the `mutation.no-op` the facet raises when the policy asked
    /// for is the policy already active.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        if declared.str("status") != "applied" {
            return Err(format!("mutate-{kind}: the committed merge-policy vector is a clean applied vector, but this one declares {:?}", declared.str("status")));
        }
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-setting and asserts, in role, that the result IS
    /// the committed after-setting, that the declared policy really moved, and that the reported
    /// diagnostics are the committed ones.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, after, outcome) = super::fixture_text(kind);
            let base = setting_of(before, "before", kind)?;
            let expected = setting_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base;
            let raised = apply_merge_policy_config_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied setting does not match the committed after-setting"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(outcome)?, &raised)?;
            policy_claim_holds(kind, &row.str("policy"), &row.str("wasPolicy"), &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-setting. The undo carries the weight — its payload holds the
    /// policy read off BASE, which the forward mutation never travelled with.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = setting_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base;
            let raised = apply_merge_policy_config_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the setting untouched, so restoring it proves nothing"));
            }
            for step in inverse_merge_policy_config_mutation_steps(&mutation, &base) {
                let undone = apply_merge_policy_config_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            if active(&current) != row.str("wasPolicy") {
                return Err(format!("inverse-{kind}: the restored setting must carry {:?} once more, but it carries {:?}", row.str("wasPolicy"), active(&current)));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🚧️ The one guard the vocabulary carries, and the reason it is not `MutationOutcome::empty()`:
    /// asking for the policy already active must WARN and hand the record back unchanged. An empty
    /// diff would apply as `MergePolicySetting::default()` and loosen a tightened authority in
    /// silence, so this scenario asserts both halves — the warning AND the surviving record.
    pub fn no_op_guard(_ctx: &Context) -> Result<Outcome, String> {
        let (before, _mutation, _after, _outcome) = super::fixture_text("change-merge-policy");
        let base = setting_of(before, "before", "change-merge-policy")?;
        let redundant = MergePolicyConfigMutation::ChangeMergePolicy(ChangeMergePolicy { policy: base.policy });
        let mut current = base;
        let raised = apply_merge_policy_config_mutation_reporting(&mut current, &redundant);
        if !raised.iter().any(|(code, level)| code == "mutation.no-op" && level == "Warning") {
            return Err(format!("no-op-guard: re-applying the active policy must raise a mutation.no-op Warning, but the implementation raised {raised:?}"));
        }
        if current != base {
            return Err(disagreement("no-op-guard: a refused change must hand the record back UNCHANGED, never reset it to the facet default", &current, &base));
        }
        if current == MergePolicySetting::default() && base != MergePolicySetting::default() {
            return Err("no-op-guard: the refused change reset the authority to the facet default".to_string());
        }
        let projection = projection(&current)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    /// 🔁️ The identity law for a record whose only carrier is its own JSON projection. `os.config`
    /// has no `.dsl.semio` or `.pack.semio` form, so the honest statement is a decode/re-encode that
    /// must reproduce the committed projection exactly. The decode is proven real by reading the
    /// active policy back off the TYPED value: a shortcut that handed the input text back would
    /// satisfy the projection comparison and fail the moment `active()` asks the typed record.
    pub fn round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let (before, _mutation, _after, _outcome) = super::fixture_text("change-merge-policy");
        let setting = setting_of(before, "before", "change-merge-policy")?;
        if active(&setting) != "Normal" {
            return Err(format!("identity-round-trip: the committed setting carries the Normal authority, but the decoded value holds {:?}", active(&setting)));
        }
        let reencoded = encode_merge_policy_setting_json(&setting);
        let reparsed = setting_of(&reencoded, "re-encoded", "change-merge-policy")?;
        if reparsed != setting {
            return Err(disagreement("identity-round-trip: decoding the re-encoded setting did not reproduce the typed value", &reparsed, &setting));
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
    built = built.oracle("no-op-guard", no_op_guard_oracle).oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("no-op-guard", subject::no_op_guard).subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
