//! 🦀️ Opening-preferences exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR. Recorded no-oracle decision
//! `os-config-opening-preferences-mutation-semantics`
//! (`../../../../../🎚️config/🧪️oracle/🔣️component.json`): `os.config.opening` is this operating
//! system's own preference record with no third-party implementation, so `oracle` here reads the
//! committed, independently handcrafted per-kind specification fixtures
//! (`../../../../../🎚️config/🧬️schema/🧬️mutations/<slug>/🧪️tests/<fixture>/`) literally — no
//! recomputation, no reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_opening_config_mutation_reporting` over the full two-kind `OpeningConfigMutation`
//! vocabulary.
//!
//! **Why the case sits here rather than beside the vocabulary.** `🎚️config` has no crate of its
//! own. `📦️glue.rs` in `🖥️host/📦️packages/🦀️rust` is the ONE place `OpeningConfigMutation` is
//! mounted, and the generated test host resolves its subject crate by walking UP from the case
//! owner — from `🎚️config` that walk reaches the OS kernel, which does not expose the facet at all.
//! The feature says the same thing in prose so a reader is not left guessing.
//!
//! **Scope.** The same host mount now compiles the merge-policy and identity vocabularies and their
//! direct-leaf unit tests. This exhaustive case remains intentionally scoped to the two opening
//! kinds and their committed fixtures.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.
//!
//! **Why the shared `⚖️law` module is not used here.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` is
//! reachable only where the stdio oracle crate is linked into the generated host, which happens for
//! a case whose owner sits under `✏️s/🔌️plugins/🗄️stdio`. This case's owner does not, and making one
//! plugin's test tree a build dependency of the OS plugin host would be worse than restating three
//! laws. They are stated inline, in the same words and with the same strictness.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors the derive-generated `OpeningConfigMutation` descriptor order — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, while `dsl::Mutations`
/// mechanically derives the production vocabulary from the two wrapped direct leaves.
const KINDS: &[&str] = &["set-default-app", "clear-default-app"];
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind,
/// read literally via `include_str!` — this IS the independently handcrafted vector the no-oracle
/// decision rests on, never recomputed.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "set-default-app" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🧪️tests/repins-the-cad-editor-to-the-drafting-app/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🧪️tests/repins-the-cad-editor-to-the-drafting-app/🦠️mutation/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🧪️tests/repins-the-cad-editor-to-the-drafting-app/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🧪️tests/repins-the-cad-editor-to-the-drafting-app/🎯️outcome/🔣️component.json"),
        ),
        "clear-default-app" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🧪️tests/unpins-the-cad-editor-and-keeps-the-viewer-pin/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🧪️tests/unpins-the-cad-editor-and-keeps-the-viewer-pin/🦠️mutation/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🧪️tests/unpins-the-cad-editor-and-keeps-the-viewer-pin/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🧪️tests/unpins-the-cad-editor-and-keeps-the-viewer-pin/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-os-config-opening: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER document, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE document — undoing either kind must return
/// to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}

/// 🔁️ The identity carrier's reference answer: the committed record itself. The projection is the
/// committed text and the subject must reproduce it from a typed value alone.
fn round_trip_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (before, _mutation, _after, _outcome) = fixture_text("set-default-app");
    Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_plugin_host::opening_config::mutations::OpeningConfigMutation;
    use semio_framework_plugin_host::opening_config::{apply_opening_config_mutation_reporting, decode_opening_config_mutation_json, decode_opening_preferences_json, encode_opening_preferences_json, inverse_opening_config_mutation_steps, OpeningPreferences};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️FixtureDecode
    fn record_of(text: &str, label: &str, kind: &str) -> Result<OpeningPreferences, String> {
        decode_opening_preferences_json(text).map_err(|error| format!("mutate-os-config-opening: the committed {label}-record for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<OpeningConfigMutation, String> {
        decode_opening_config_mutation_json(text).map_err(|error| format!("mutate-os-config-opening: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(record: &OpeningPreferences) -> Result<Json, String> {
        parse_json(&encode_opening_preferences_json(record))
    }

    fn disagreement(what: &str, got: &OpeningPreferences, expected: &OpeningPreferences) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_opening_preferences_json(got), encode_opening_preferences_json(expected))
    }

    /// 🔎️ The app pinned for `role`, read off the TYPED value rather than off the text — which is
    /// what makes the sibling claim and the identity scenario statements about a real decode.
    fn pinned(record: &OpeningPreferences, role: &str) -> Option<String> {
        record.defaults.iter().find(|entry| format!("{:?}", entry.role).to_lowercase() == role).map(|entry| entry.app.plugin_id.clone())
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law, in the exact form this facet can state it: the addressed pin moved
    /// and the SIBLING pin did not. Both kinds edit one key of one list, so a whole-document
    /// comparison against a fixture written from the same bug would pass an implementation that
    /// rebuilt the list; requiring the sibling's plugin id to survive unchanged would not.
    fn pin_claim_holds(kind: &str, sibling: &str, pins: usize, base: &OpeningPreferences, after: &OpeningPreferences) -> Result<(), String> {
        if after.defaults.len() != pins {
            return Err(format!("mutate-{kind}: the feature declares {pins} pin(s) after this mutation, but the document holds {}", after.defaults.len()));
        }
        if base == after {
            return Err(format!("mutate-{kind}: the mutation left the preference record unchanged — the scenario would report a pass for a mutation it never observed"));
        }
        let (was, now) = (pinned(base, sibling), pinned(after, sibling));
        if was.is_none() {
            return Err(format!("mutate-{kind}: the feature names {sibling:?} as the surviving sibling pin, but the committed before-record carries no pin for that role"));
        }
        if was != now {
            return Err(format!("mutate-{kind}: the {sibling:?} pin must survive untouched, but it went from {was:?} to {now:?}"));
        }
        Ok(())
    }

    /// 🎯️ The committed outcome claim: both vectors are clean `applied` vectors, so any diagnostic
    /// at all is a divergence — including the `mutation.no-op` this facet raises when a pin is set
    /// to the app it already carries or cleared when it was never set.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        if declared.str("status") != "applied" {
            return Err(format!("mutate-{kind}: both committed opening-preferences vectors are clean applied vectors, but this one declares {:?}", declared.str("status")));
        }
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-record and asserts, in role, that the result IS
    /// the committed after-record, that the sibling pin survived, and that the reported diagnostics
    /// are the committed ones.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, after, outcome) = super::fixture_text(kind);
            let base = record_of(before, "before", kind)?;
            let expected = record_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_opening_config_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied record does not match the committed after-record"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(outcome)?, &raised)?;
            let pins = match row.get("pins") {
                Some(Json::Number(value)) => *value as usize,
                _ => return Err(format!("mutate-{kind}: the feature's Examples row must declare a numeric `pins` count")),
            };
            pin_claim_holds(kind, &row.str("sibling"), pins, &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-record exactly, pin ORDER included. `clear-default-app`'s undo
    /// carries the weight — its payload holds only the `(dialect, role)` key, so the app it puts
    /// back is one the mutation never travelled with.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = record_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_opening_config_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the record untouched, so restoring it proves nothing"));
            }
            for step in inverse_opening_config_mutation_steps(&mutation, &base) {
                let undone = apply_opening_config_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The identity law for a record whose only carrier is its own JSON projection. `os.config`
    /// has no `.dsl.semio` or `.pack.semio` form — it is persisted as this document and nothing else
    /// — so the honest statement is a decode/re-encode that must reproduce the committed projection
    /// exactly. Byte-identity is therefore the CORRECT answer here rather than the tripwire it is
    /// for a format with writer freedom, and the decode is proven real by reading both pins back off
    /// the TYPED value: a shortcut that handed the input text back would satisfy the projection
    /// comparison and fail the moment `pinned()` asks the typed record what it holds.
    pub fn round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let (before, _mutation, _after, _outcome) = super::fixture_text("set-default-app");
        let record = record_of(before, "before", "set-default-app")?;
        if pinned(&record, "viewer").as_deref() != Some("cad") || pinned(&record, "editor").as_deref() != Some("cad") {
            return Err(format!("identity-round-trip: the committed record pins the cad viewer and the cad editor, but the decoded value holds {}", encode_opening_preferences_json(&record)));
        }
        let reencoded = encode_opening_preferences_json(&record);
        let reparsed = record_of(&reencoded, "re-encoded", "set-default-app")?;
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
/// id, so the loop mirrors the feature's `Examples` tables exactly. `identity-round-trip` is
/// registered in BOTH roles here, unlike the artifact cases in this wave: this record's only carrier
/// IS its committed JSON projection, so the oracle can answer with the committed text literally and
/// the subject must reproduce it from a typed value alone.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
