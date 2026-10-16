//! 🎯️ Subject role: this repository's own mutation report checked against the committed vectors, gated behind the sut feature.

//#region 🔖️Subject
use super::{canonical, vector, DSL_ASSET, UNOBSERVABLE};
use semio_repo_test_host::{parse_json, Context, Json, Outcome};
use semio_s_artifact_energy_model::standards::v1::subsets::any::io::text::mutations::energy_model_mutation_report_json;
use semio_s_artifact_energy_model::standards::v1::subsets::any::io::text::snapshot::energy_model_identity_report_json;
use semio_repo_test_host::law;

//#region 🔖️Report
fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
    report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
}

fn members(report: &Json, key: &str) -> Result<Vec<Json>, String> {
    match member(report, key)? {
        Json::Array(items) => Ok(items.clone()),
        other => Err(format!("the report's {key:?} member is {}, not an array", other.to_string())),
    }
}

pub(super) fn text(report: &Json, key: &str) -> Result<String, String> {
    match member(report, key)? {
        Json::String(value) => Ok(value.clone()),
        other => Err(format!("the report's {key:?} member is {}, not a string", other.to_string())),
    }
}

fn strings(value: &Json, key: &str) -> Vec<String> {
    value
        .array(key)
        .iter()
        .map(|segment| match segment {
            Json::String(text) => text.clone(),
            other => other.to_string(),
        })
        .collect()
}

/// 🎯️ Checks the produced diagnostics against the ones the committed `🎯️outcome` declares.
fn declared_outcome_holds(id: &str, produced: &[Json], outcome: &Json) -> Result<(), String> {
    let codes: Vec<String> = produced.iter().map(|message| message.str("code")).collect();
    let levels: Vec<String> = produced.iter().map(|message| message.str("level")).collect();
    if outcome.str("status") == "rejected" {
        let expected = outcome.str("code");
        if codes != vec![expected.clone()] {
            return Err(format!("mutate-{id}: the vector declares a rejection with code {expected:?}, the implementation raised {codes:?}"));
        }
        if !levels.iter().any(|level| level == "error" || level == "fatal") {
            return Err(format!("mutate-{id}: the vector declares a rejection, but the implementation raised it at {levels:?}"));
        }
        let path = strings(outcome, "path");
        let target = strings(&produced[0], "target");
        if !path.is_empty() && target != path {
            return Err(format!("mutate-{id}: the vector declares the offending address {path:?}, the implementation reported {target:?}"));
        }
        return Ok(());
    }
    let expected: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).collect();
    if codes != expected {
        return Err(format!("mutate-{id}: the vector declares the diagnostics {expected:?}, the implementation raised {codes:?}"));
    }
    match levels.iter().find(|level| level.as_str() == "error" || level.as_str() == "fatal") {
        Some(level) => Err(format!("mutate-{id}: the vector declares an applied outcome, but the implementation raised a {level}")),
        None => Ok(()),
    }
}
//#endregion 🔖️Report

//#region 🔖️Handlers
/// 🎯️ Applies the vector and asserts the resulting document, the produced delta and the declared
/// diagnostics all match what the vector commits to.
pub fn mutate(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let committed = vector(id);
        let report = parse_json(&energy_model_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("mutate-{id}: the committed vector did not reach this subset's own codec: {error}"))?)?;
        let applied = member(&report, "snapshot")?;
        let expected = member(&report, "expectedSnapshot")?;
        if let Some(first) = law::divergence(applied, expected) {
            return Err(format!("mutate-{id}: the applied document is not the committed after-snapshot — {first}"));
        }
        if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {
            return Err(format!("mutate-{id}: the produced delta is not the committed 🔺️diff — {first}"));
        }
        declared_outcome_holds(id, &members(&report, "messages")?, &canonical(committed.outcome))?;
        if canonical(committed.outcome).str("status") != "rejected" {
            law::mutation_is_observable(committed.kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
        }
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
    }
}

/// ↩️ The metamorphic inverse law: applying the vector and then its OWN computed inverse must
/// restore the committed before-snapshot exactly.
pub fn inverse(id: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let committed = vector(id);
        let report = parse_json(&energy_model_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("inverse-{id}: the committed vector did not reach this subset's own codec: {error}"))?)?;
        let faults: Vec<String> = members(&report, "inverseMessages")?
            .iter()
            .filter(|message| {
                let level = message.str("level");
                level == "error" || level == "fatal"
            })
            .map(|message| message.str("code"))
            .collect();
        if !faults.is_empty() {
            return Err(format!("inverse-{id}: an inverse step was rejected with {faults:?}, so the document never got the chance to return"));
        }
        let restored = member(&report, "inverseSnapshot")?;
        law::inverse_restores(committed.kind, restored, member(&report, "base")?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored.clone()))
    }
}

/// 🔁️ The real committed document through this subset's own two codecs.
pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    let text_bytes = String::from_utf8(ctx.input_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
    let report = parse_json(&energy_model_identity_report_json(&text_bytes).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
    let parsed = member(&report, "parsed")?;
    law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
    law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
    if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
        return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
    }
    Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed.clone()))
}
//#endregion 🔖️Handlers
//#endregion 🔖️Subject
