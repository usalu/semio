//! 🏛️ `s.energy.model` ANSI/ASHRAE 140 §5.2 simulation case — Rust adapter.
//!
//! The oracle role answers with the committed `🔮️energyplus.json` read literally; the subject role
//! runs this repository's own engine through the crate-side bridge
//! `semio_s_plugin_energy::bestest::results_report_json` (the `Results` tables are `pub(crate)`,
//! so the projection has to be produced inside the crate and handed over as text, exactly as
//! `energy_model_mutation_report_json` does for the mutation case).
//!
//! ⚠️ There is no skip channel in this host — "a missing registration, a panic and an error are all
//! results, never a silent skip" (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs`).
//! A case whose EnergyPlus reference has not been produced yet therefore fails with an explicit
//! message naming the absent file, rather than agreeing with nothing.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Cases
/// 🏛️ The conditioned cases that HAVE a committed EnergyPlus reference, mirroring the feature's
/// `Examples` tables. 630/930 (fins) and 650/950 (night ventilation) are deliberately absent: the
/// oracle side has not translated them, and listing them here would register scenarios that can
/// only ever fail for want of a reference (`📓️bestest-contract.md` amendment, 2026-09-06 06:30).
const CONDITIONED: &[&str] = &["600", "610", "620", "640", "900", "910", "920", "940"];

/// 🏛️ The free-float cases, judged on zone temperature rather than delivered energy.
const FREE_FLOAT: &[&str] = &["600FF", "900FF"];

/// 🏛️ The cases whose peak demands are compared.
const PEAK: &[&str] = &["600", "900"];

/// 🏛️ The cases whose full 8760-hour zone-temperature trace is compared.
const HOURLY: &[&str] = &["600", "600FF", "900FF"];

/// 🏛️ Every registered case, for the parameter cross-check that needs no simulator at all.
const ALL: &[&str] = &["600", "600FF", "610", "620", "630", "640", "650", "900", "900FF", "910", "920", "930", "940", "950"];

/// 🌦️ The one weather file both producers read.
#[cfg(feature = "sut")]
const WEATHER_ASSET: &str = "shared://🌦️denver-tmy/🌦️.epw";

/// 📏️ Tolerances from `📓️bestest-contract.md`. Annual and peak are relative; the temperature ones
/// are absolute kelvin.
/// 📏️ Heating stays at 20 %; cooling is 10 % because the amended contract pins the simple-glazing
/// route's own +5.7…+8.1 % offset against a layered EnergyPlus window as a known, documented
/// difference rather than slack to be spent twice.
#[cfg(feature = "sut")]
const ANNUAL_HEATING_TOLERANCE: f64 = 0.20;
#[cfg(feature = "sut")]
const ANNUAL_COOLING_TOLERANCE: f64 = 0.10;
#[cfg(feature = "sut")]
const PEAK_RELATIVE_TOLERANCE: f64 = 0.25;
#[cfg(feature = "sut")]
const FREE_FLOAT_TOLERANCE_K: f64 = 2.5;
#[cfg(feature = "sut")]
const HOURLY_RMSE_TOLERANCE_K: f64 = 2.0;

#[cfg(feature = "sut")]
fn model_asset(case: &str) -> String {
    format!("shared://🏛️bestest-{case}/🔋️model.json")
}

fn reference_asset(case: &str) -> String {
    format!("shared://🏛️bestest-{case}/🔮️energyplus.json")
}
//#endregion 🔖️Cases

//#region 🔖️Reading
/// 📥️ The committed EnergyPlus reference for one case, or an error naming exactly what is absent.
fn reference(ctx: &Context, case: &str) -> Result<Json, String> {
    let uri = reference_asset(case);
    ctx.fixture_json(&uri).map_err(|error| format!("case {case}: no committed EnergyPlus reference at {uri} ({error}) — the oracle for this case has not been produced yet, so there is nothing to compare against"))
}

#[cfg(feature = "sut")]
fn number(value: &Json, path: &[&str]) -> Result<f64, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor.get(key).ok_or_else(|| format!("the result document carries no {:?} member", path.join(".")))?;
    }
    match cursor {
        Json::Number(value) => Ok(*value),
        other => Err(format!("{:?} is {}, not a number", path.join("."), other.to_string())),
    }
}

#[cfg(feature = "sut")]
fn series(value: &Json, path: &[&str]) -> Result<Vec<f64>, String> {
    let mut cursor = value;
    for key in path {
        cursor = cursor.get(key).ok_or_else(|| format!("the result document carries no {:?} member", path.join(".")))?;
    }
    match cursor {
        Json::Array(items) => items
            .iter()
            .map(|item| match item {
                Json::Number(value) => Ok(*value),
                other => Err(format!("{:?} holds {}, not a number", path.join("."), other.to_string())),
            })
            .collect(),
        other => Err(format!("{:?} is {}, not an array", path.join("."), other.to_string())),
    }
}
//#endregion 🔖️Reading

//#region 🔖️Comparison
/// 📏️ One metric's verdict, so a case that agrees on annual energy but not on peaks says so.
#[cfg(feature = "sut")]
struct Verdict {
    metric: String,
    subject: f64,
    oracle: f64,
    difference: f64,
    tolerance: f64,
}

#[cfg(feature = "sut")]
impl Verdict {
    fn relative(metric: &str, subject: f64, oracle: f64, tolerance: f64) -> Verdict {
        let difference = if oracle.abs() > 1e-9 { (subject - oracle).abs() / oracle.abs() } else { (subject - oracle).abs() };
        Verdict { metric: metric.to_string(), subject, oracle, difference, tolerance }
    }

    fn absolute(metric: &str, subject: f64, oracle: f64, tolerance: f64) -> Verdict {
        Verdict { metric: metric.to_string(), subject, oracle, difference: (subject - oracle).abs(), tolerance }
    }

    fn holds(&self) -> bool {
        self.difference <= self.tolerance
    }

    fn describe(&self) -> String {
        format!("{}: semio {:.3}, EnergyPlus {:.3}, deviation {:.4} against a tolerance of {:.4}", self.metric, self.subject, self.oracle, self.difference, self.tolerance)
    }
}

#[cfg(feature = "sut")]
fn judge(case: &str, verdicts: Vec<Verdict>, subject: Json) -> Result<Outcome, String> {
    let failures: Vec<String> = verdicts.iter().filter(|verdict| !verdict.holds()).map(Verdict::describe).collect();
    if failures.is_empty() {
        return Ok(Outcome::projection(subject));
    }
    Err(format!("case {case} disagrees with EnergyPlus on {} of {} metrics — {}", failures.len(), verdicts.len(), failures.join(" | ")))
}

#[cfg(feature = "sut")]
fn rmse(subject: &[f64], oracle: &[f64]) -> Result<f64, String> {
    if subject.len() != oracle.len() {
        return Err(format!("the two hourly series have different lengths: semio {} vs EnergyPlus {}", subject.len(), oracle.len()));
    }
    if subject.is_empty() {
        return Err("the hourly series are empty".to_string());
    }
    let sum: f64 = subject.iter().zip(oracle).map(|(a, b)| (a - b) * (a - b)).sum();
    Ok((sum / subject.len() as f64).sqrt())
}
//#endregion 🔖️Comparison

//#region 🔖️Oracle
/// 🔮️ The reference answer: the committed EnergyPlus document, read literally.
fn oracle_for(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let document = reference(ctx, case)?;
        Ok(Outcome::projection(document))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{judge, model_asset, number, reference, rmse, series, Verdict, ANNUAL_COOLING_TOLERANCE, ANNUAL_HEATING_TOLERANCE, FREE_FLOAT_TOLERANCE_K, HOURLY_RMSE_TOLERANCE_K, PEAK_RELATIVE_TOLERANCE, WEATHER_ASSET};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_energy::bestest;

    /// 🏃️ Run one case through this repository's own engine and write its result document into the
    /// scenario work directory as `⚙️semio.json` before comparing anything.
    fn simulate(ctx: &Context, case: &str) -> Result<Json, String> {
        let committed = String::from_utf8(ctx.fixture_bytes(&model_asset(case))?).map_err(|error| format!("case {case}: the committed model is not UTF-8: {error}"))?;
        let built = bestest::model_json(case).ok_or_else(|| format!("case {case} is not registered in the engine's own §5.2 catalogue"))?;
        if committed != built {
            return Err(format!("case {case}: the committed 🔋️model.json is not what the case builder produces today — regenerate the fixtures before trusting any comparison"));
        }
        let weather_path = ctx.fixture(WEATHER_ASSET)?;
        let weather = std::fs::read_to_string(&weather_path).map_err(|error| format!("cannot read {}: {error}", weather_path.display()))?;
        let digest = semio_repo_test_host::sha256_hex(weather.as_bytes());
        let text = bestest::results_report_json(case, &weather, WEATHER_ASSET, &digest, 3).map_err(|error| format!("case {case}: the engine refused the committed model: {error}"))?;
        let path = ctx.artifact("semio", "⚙️semio.json")?;
        std::fs::write(&path, &text).map_err(|error| format!("cannot write {}: {error}", path.display()))?;
        parse_json(&text)
    }

    pub fn annual_energy(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let subject = simulate(ctx, case)?;
            let oracle = reference(ctx, case)?;
            let verdicts = vec![
                Verdict::relative("annual heating kWh", number(&subject, &["annual", "heatingKwh"])?, number(&oracle, &["annual", "heatingKwh"])?, ANNUAL_HEATING_TOLERANCE),
                Verdict::relative("annual cooling kWh", number(&subject, &["annual", "coolingKwh"])?, number(&oracle, &["annual", "coolingKwh"])?, ANNUAL_COOLING_TOLERANCE),
            ];
            judge(case, verdicts, subject)
        }
    }

    pub fn peak_load(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let subject = simulate(ctx, case)?;
            let oracle = reference(ctx, case)?;
            let verdicts = vec![
                Verdict::relative("peak heating kW", number(&subject, &["peak", "heatingKw"])?, number(&oracle, &["peak", "heatingKw"])?, PEAK_RELATIVE_TOLERANCE),
                Verdict::relative("peak cooling kW", number(&subject, &["peak", "coolingKw"])?, number(&oracle, &["peak", "coolingKw"])?, PEAK_RELATIVE_TOLERANCE),
            ];
            judge(case, verdicts, subject)
        }
    }

    pub fn free_float(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let subject = simulate(ctx, case)?;
            let oracle = reference(ctx, case)?;
            let verdicts = vec![
                Verdict::absolute("free-float minimum °C", number(&subject, &["freeFloat", "minC"])?, number(&oracle, &["freeFloat", "minC"])?, FREE_FLOAT_TOLERANCE_K),
                Verdict::absolute("free-float maximum °C", number(&subject, &["freeFloat", "maxC"])?, number(&oracle, &["freeFloat", "maxC"])?, FREE_FLOAT_TOLERANCE_K),
                Verdict::absolute("free-float mean °C", number(&subject, &["freeFloat", "meanC"])?, number(&oracle, &["freeFloat", "meanC"])?, FREE_FLOAT_TOLERANCE_K),
            ];
            judge(case, verdicts, subject)
        }
    }

    pub fn hourly_temperature(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let subject = simulate(ctx, case)?;
            let oracle = reference(ctx, case)?;
            let error = rmse(&series(&subject, &["hourly", "zoneAirTemperatureC"])?, &series(&oracle, &["hourly", "zoneAirTemperatureC"])?)?;
            judge(case, vec![Verdict::absolute("hourly zone air temperature RMSE K", error, 0.0, HOURLY_RMSE_TOLERANCE_K)], subject)
        }
    }

    /// 📐️ The engine's own reading of the case parameters ANSI/ASHRAE 140 §5.2 states directly —
    /// compared against the Python oracle's independent derivation from the SAME committed model,
    /// so a drift in either reading of the standard shows up before any simulator is involved.
    pub fn case_parameters(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let text = bestest::case_parameters_json(case).ok_or_else(|| format!("case {case} is not registered in the engine's own §5.2 catalogue"))?;
            parse_json(&text).map(Outcome::projection)
        }
    }

    /// 🔁️ The committed model is exactly the builder's output, and it survives this subset's own
    /// canonical JSON in both directions.
    pub fn model_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = String::from_utf8(ctx.fixture_bytes(&model_asset("600"))?).map_err(|error| format!("the committed model is not UTF-8: {error}"))?;
        let built = bestest::model_json("600").ok_or("case 600 is not registered")?;
        if committed != built {
            return Err("the committed 🔋️model.json for case 600 is not what the case builder produces today".to_string());
        }
        let decoded = bestest::model_from_json(&committed)?;
        if decoded != bestest::model("600").ok_or("case 600 is not registered")? {
            return Err("the committed 🔋️model.json for case 600 decodes into a different model than the builder produces".to_string());
        }
        parse_json(&committed).map(Outcome::projection)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id, so the loop
/// mirrors the feature's `Examples` tables exactly.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for case in ALL {
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("case-parameters-{case}"), subject::case_parameters(case));
        }
        let _ = case;
    }
    for case in CONDITIONED {
        built = built.oracle(&format!("annual-energy-{case}"), oracle_for(case));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("annual-energy-{case}"), subject::annual_energy(case));
        }
    }
    for case in PEAK {
        built = built.oracle(&format!("peak-load-{case}"), oracle_for(case));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("peak-load-{case}"), subject::peak_load(case));
        }
    }
    for case in FREE_FLOAT {
        built = built.oracle(&format!("free-float-{case}"), oracle_for(case));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("free-float-{case}"), subject::free_float(case));
        }
    }
    for case in HOURLY {
        built = built.oracle(&format!("hourly-temperature-{case}"), oracle_for(case));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("hourly-temperature-{case}"), subject::hourly_temperature(case));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("model-round-trip", subject::model_round_trip);
    }
    built
}
//#endregion 🔖️Registration
