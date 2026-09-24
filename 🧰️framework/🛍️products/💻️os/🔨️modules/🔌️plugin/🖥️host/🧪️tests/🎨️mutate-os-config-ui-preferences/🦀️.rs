//! 🎨️ UI-preferences exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `os-config-ui-preferences-mutation-semantics` (`../../../../../🎚️config/🔮️oracles/🔣️.json`):
//! `os.config.ui-preferences` is this operating system's own preference record with no third-party
//! implementation, so `oracle` here reads the committed, independently handcrafted per-kind
//! specification vectors (`../../../../../🎚️config/🧬️schema/🧬️mutations/<slug>/🧫️fixtures/<vector>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_ui_preferences_config_mutation_reporting` over the full nine-kind
//! `UiPreferencesConfigMutation` vocabulary, one `sets-*` (applied) and one `keeps-*` (no-op) vector
//! per kind.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Vectors
/// 🏷️ Every committed vector's scenario id, in the catalog's own order — duplicated, not imported,
/// because the oracle-only build must not link the subject crate. The contract's mutation-coverage
/// gate keeps this list honest against the catalog. `sets-*` vectors are applied, `keeps-*` no-ops.
const VECTORS: &[&str] = &[
    "sets-appearance",
    "keeps-appearance",
    "sets-layout",
    "keeps-layout",
    "sets-driver",
    "keeps-driver",
    "sets-custom-driver",
    "keeps-custom-driver",
    "sets-locale",
    "keeps-locale",
    "sets-terminology",
    "keeps-terminology",
    "sets-theme",
    "keeps-theme",
    "sets-custom-theme",
    "keeps-custom-theme",
    "sets-keybinding",
    "keeps-keybinding",
];
//#endregion 🔖️Vectors

//#region 🔖️Fixtures
/// 🧫️ Embeds one vector's `(before, mutation, after, outcome)` quintet text, read literally.
macro_rules! vector {
    ($leaf:literal, $vector:literal) => {
        (
            include_str!(concat!("../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/📸️snapshot/⬅️before/🔣️.json")),
            include_str!(concat!("../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/🦠️mutation/🔣️.json")),
            include_str!(concat!("../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/📸️snapshot/➡️after/🔣️.json")),
            include_str!(concat!("../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/🎯️outcome/🔣️.json")),
        )
    };
}

/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one scenario —
/// this IS the independently handcrafted vector the no-oracle decision rests on, never recomputed.
fn fixture_text(scenario: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match scenario {
        "sets-appearance" => vector!("🌗️set-appearance", "✏️sets-appearance"),
        "keeps-appearance" => vector!("🌗️set-appearance", "🟰️keeps-appearance"),
        "sets-layout" => vector!("📐️set-layout", "✏️sets-layout"),
        "keeps-layout" => vector!("📐️set-layout", "🟰️keeps-layout"),
        "sets-driver" => vector!("🕹️set-driver", "✏️sets-driver"),
        "keeps-driver" => vector!("🕹️set-driver", "🟰️keeps-driver"),
        "sets-custom-driver" => vector!("🚗️set-custom-driver", "✏️sets-custom-driver"),
        "keeps-custom-driver" => vector!("🚗️set-custom-driver", "🟰️keeps-custom-driver"),
        "sets-locale" => vector!("🗣️set-locale", "✏️sets-locale"),
        "keeps-locale" => vector!("🗣️set-locale", "🟰️keeps-locale"),
        "sets-terminology" => vector!("📖️set-terminology", "✏️sets-terminology"),
        "keeps-terminology" => vector!("📖️set-terminology", "🟰️keeps-terminology"),
        "sets-theme" => vector!("🖼️set-theme", "✏️sets-theme"),
        "keeps-theme" => vector!("🖼️set-theme", "🟰️keeps-theme"),
        "sets-custom-theme" => vector!("🎨️set-custom-theme", "✏️sets-custom-theme"),
        "keeps-custom-theme" => vector!("🎨️set-custom-theme", "🟰️keeps-custom-theme"),
        "sets-keybinding" => vector!("⌨️set-keybinding-override", "✏️sets-keybinding"),
        "keeps-keybinding" => vector!("⌨️set-keybinding-override", "🟰️keeps-keybinding"),
        other => panic!("mutate-os-config-ui-preferences: no specification vector registered for scenario {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER record, read literally.
fn mutate_oracle_for(scenario: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(scenario);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE record — undoing a kind must return to
/// exactly where the specification vector started.
fn inverse_oracle_for(scenario: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(scenario);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}

/// 🔁️ The identity carrier's reference answer: the committed fully populated record itself.
fn round_trip_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (before, _mutation, _after, _outcome) = fixture_text("keeps-keybinding");
    Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_plugin_host::opening_config::mutations::UiPreferencesConfigMutation;
    use semio_framework_plugin_host::opening_config::{apply_ui_preferences_config_mutation_reporting, decode_ui_preferences_config_mutation_json, decode_ui_preferences_json, encode_ui_preferences_json, inverse_ui_preferences_config_mutation_steps, UiLocale, UiPreferences};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️FixtureDecode
    fn record_of(text: &str, label: &str, scenario: &str) -> Result<UiPreferences, String> {
        decode_ui_preferences_json(text).map_err(|error| format!("mutate-os-config-ui-preferences: the committed {label}-record for {scenario:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, scenario: &str) -> Result<UiPreferencesConfigMutation, String> {
        decode_ui_preferences_config_mutation_json(text).map_err(|error| format!("mutate-os-config-ui-preferences: the committed mutation payload for {scenario:?} must decode: {error}"))
    }

    fn projection(record: &UiPreferences) -> Result<Json, String> {
        parse_json(&encode_ui_preferences_json(record))
    }

    /// 🔤️ The projection with every object's members in key order, so member comparisons state the
    /// record's content and never the encoder's map iteration order.
    fn ordered(value: &Json) -> Json {
        match value {
            Json::Object(entries) => {
                let mut sorted: Vec<(String, Json)> = entries.iter().map(|(key, member)| (key.clone(), ordered(member))).collect();
                sorted.sort_by(|left, right| left.0.cmp(&right.0));
                Json::Object(sorted)
            }
            Json::Array(items) => Json::Array(items.iter().map(ordered).collect()),
            other => other.clone(),
        }
    }

    fn disagreement(what: &str, got: &UiPreferences, expected: &UiPreferences) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_ui_preferences_json(got), encode_ui_preferences_json(expected))
    }

    /// 🔎️ The camel-case member names whose projected values differ between two records, read off
    /// the TYPED values' canonical projections — the observable a `field` claim is checked against.
    fn moved_fields(before: &UiPreferences, after: &UiPreferences) -> Result<Vec<String>, String> {
        let (before, after) = (ordered(&projection(before)?), ordered(&projection(after)?));
        let members = ["appearance", "layout", "driverId", "customDrivers", "locale", "terminology", "themeId", "customThemes", "keybindingOverrides"];
        Ok(members.iter().filter(|member| before.get(member) != after.get(member)).map(|member| member.to_string()).collect())
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law: an `applied` vector moves exactly its declared preference and a
    /// `no-op` vector moves nothing. Comparing the whole record against a fixture written from the
    /// same bug would pass an implementation that returned the fixture; naming the member would not.
    fn field_claim_holds(scenario: &str, field: &str, status: &str, base: &UiPreferences, after: &UiPreferences) -> Result<(), String> {
        let moved = moved_fields(base, after)?;
        let expected: Vec<String> = if status == "applied" { vec![field.to_string()] } else { Vec::new() };
        if moved != expected {
            return Err(format!("mutate-{scenario}: the feature declares the {status} vector moves {expected:?}, but the implementation moved {moved:?}"));
        }
        Ok(())
    }

    /// 🎯️ The committed outcome claim: an `applied` vector raises nothing and a `no-op` vector raises
    /// exactly one `mutation.no-op` Warning — the committed outcome's own status and code.
    fn outcome_matches(scenario: &str, declared_status: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        let status = declared.str("status");
        if status != declared_status {
            return Err(format!("mutate-{scenario}: the feature declares a {declared_status:?} vector, but the committed outcome declares {status:?}"));
        }
        let expected: Vec<(String, String)> = if status == "no-op" { vec![("mutation.no-op".to_string(), "Warning".to_string())] } else { Vec::new() };
        if raised != expected.as_slice() {
            return Err(format!("mutate-{scenario}: the committed outcome declares {status:?} with {expected:?}, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the vector's mutation to the committed before-record and asserts, in role, that the
    /// result IS the committed after-record, that exactly the declared preference moved, and that the
    /// reported diagnostics are the committed ones.
    pub fn mutate(scenario: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            if row.str("vector") != scenario {
                return Err(format!("mutate-{scenario}: the feature row names vector {:?}", row.str("vector")));
            }
            let (before, mutation, after, outcome) = super::fixture_text(scenario);
            let base = record_of(before, "before", scenario)?;
            let expected = record_of(after, "after", scenario)?;
            let mutation = mutation_of(mutation, scenario)?;
            let mut current = base.clone();
            let raised = apply_ui_preferences_config_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{scenario}: the applied record does not match the committed after-record"), &current, &expected));
            }
            outcome_matches(scenario, &row.str("status"), &parse_json(outcome)?, &raised)?;
            field_claim_holds(scenario, &row.str("field"), &row.str("status"), &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-record exactly. Every kind reads its undo off BASE, so a keyed
    /// upsert into an empty map undoes to a removal rather than to the value it just wrote.
    pub fn inverse(scenario: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, _after, _outcome) = super::fixture_text(scenario);
            let base = record_of(before, "before", scenario)?;
            let mutation = mutation_of(mutation, scenario)?;
            let mut current = base.clone();
            let raised = apply_ui_preferences_config_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{scenario}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{scenario}: the forward mutation left the record untouched, so restoring it proves nothing"));
            }
            for step in inverse_ui_preferences_config_mutation_steps(&mutation, &base) {
                let undone = apply_ui_preferences_config_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{scenario}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {scenario:?} and then its own inverse did not restore the original"), &current, &base));
            }
            if !moved_fields(&base, &current)?.is_empty() {
                return Err(format!("inverse-{scenario}: {} does not hold its original value again", row.str("field")));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The identity law for a record whose only carrier is its own JSON projection. `os.config`
    /// has no `.dsl.semio` or `.pack.semio` form, so the honest statement is a decode/re-encode that
    /// must reproduce the committed projection exactly. The decode is proven real by reading the
    /// locale, the custom driver's scale and the keybinding back off the TYPED value.
    pub fn round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let (before, _mutation, _after, _outcome) = super::fixture_text("keeps-keybinding");
        let record = record_of(before, "before", "keeps-keybinding")?;
        let scale = record.custom_drivers.get("studio").and_then(|driver| driver.config.get("scale")).and_then(|scale| scale.as_f64());
        if record.locale != Some(UiLocale::De) || scale != Some(1.25) || record.keybinding_overrides.get("edit.undo").map(String::as_str) != Some("Meta+Z") {
            return Err(format!("identity-round-trip: the committed record holds locale de, driver scale 1.25 and Meta+Z for edit.undo, but the decoded value holds {}", encode_ui_preferences_json(&record)));
        }
        let reencoded = encode_ui_preferences_json(&record);
        let reparsed = record_of(&reencoded, "re-encoded", "keeps-keybinding")?;
        if reparsed != record {
            return Err(disagreement("identity-round-trip: decoding the re-encoded record did not reproduce the typed value", &reparsed, &record));
        }
        let projection = parse_json(&reencoded)?;
        if ordered(&projection) != ordered(&parse_json(before)?) {
            return Err(format!("identity-round-trip: the re-encoded projection differs from the committed record\n     got: {reencoded}\nexpected: {before}"));
        }
        Ok(Outcome::with_raw(reencoded.into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loops mirror the feature's `Examples` tables exactly: every vector is applied, and
/// every applied (`sets-*`) vector is also undone.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for scenario in VECTORS {
        built = built.oracle(&format!("mutate-{scenario}"), mutate_oracle_for(scenario));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{scenario}"), subject::mutate(scenario));
        }
        if scenario.starts_with("sets-") {
            built = built.oracle(&format!("inverse-{scenario}"), inverse_oracle_for(scenario));
            #[cfg(feature = "sut")]
            {
                built = built.subject(&format!("inverse-{scenario}"), subject::inverse(scenario));
            }
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
