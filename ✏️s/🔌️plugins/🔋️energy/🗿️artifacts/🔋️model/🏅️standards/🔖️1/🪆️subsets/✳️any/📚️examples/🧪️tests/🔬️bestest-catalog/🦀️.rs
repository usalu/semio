//! 🏛️ The bundled ANSI/ASHRAE 140 §5.2 catalogue against this subset's committed data: the EnergyPlus comparison over
//! the committed Denver TMY year, and the laws that keep the committed case fixtures and example DSL assets equal to
//! what `crate::bestest` builds today.

use crate::bestest::{is_free_float, model, model_json, project, run, CASES};
use crate::model::Model;

//#region 🔮️EnergyPlusComparison
/// 🔮️ INDICATIVE ANSI/ASHRAE 140 §5.2 reference envelopes (the min/max across the reference
/// programs, as published in NREL/TP-550-43827), printed beside every metric so a deviation from
/// EnergyPlus can be read against the spread the standard itself allows.
///
/// ⚠️ These are transcribed, not machine-read from the report, so they are CONTEXT ONLY: the
/// pass/fail gate below is the measured deviation from the committed EnergyPlus run and nothing
/// else. A mistyped envelope must never be able to turn a red into a green.
/// `(case, heating_kwh_min, heating_kwh_max, cooling_kwh_min, cooling_kwh_max)`.
const REFERENCE_ENVELOPE: &[(&str, f64, f64, f64, f64)] = &[
    ("600", 4296.0, 5709.0, 6137.0, 8448.0),
    ("610", 4355.0, 5786.0, 3915.0, 6139.0),
    ("620", 4613.0, 5944.0, 3417.0, 5049.0),
    ("630", 5050.0, 6469.0, 2129.0, 3701.0),
    ("640", 2751.0, 3803.0, 5952.0, 8097.0),
    ("900", 1170.0, 2041.0, 2132.0, 3669.0),
    ("910", 1512.0, 2282.0, 1109.0, 2239.0),
    ("920", 3313.0, 4300.0, 2129.0, 3417.0),
    ("930", 4143.0, 5335.0, 2255.0, 3901.0),
    ("940", 793.0, 1411.0, 2079.0, 3241.0),
];

fn weather_fixture() -> Option<String> {
    std::fs::read_to_string(subset_root().join("🧪️tests").join("🧫️fixtures").join("🌦️denver-tmy").join("🌦️.epw")).ok()
}

fn energyplus_reference(case: &str) -> Option<semio_framework_pack_json::Value> {
    let text = std::fs::read_to_string(subset_root().join("🧪️tests").join("🧫️fixtures").join(format!("🏛️bestest-{case}")).join("🔮️energyplus.json")).ok()?;
    semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()
}

fn member(value: &semio_framework_pack_json::Value, path: &[&str]) -> Option<f64> {
    let mut cursor = value;
    for key in path {
        cursor = cursor.get(key)?;
    }
    cursor.as_f64()
}

fn envelope(case: &str) -> Option<(f64, f64, f64, f64)> {
    REFERENCE_ENVELOPE.iter().find(|(id, ..)| *id == case).map(|(_, hmin, hmax, cmin, cmax)| (*hmin, *hmax, *cmin, *cmax))
}

/// 🏛️ THE comparison. Runs every case that has a committed EnergyPlus reference against the
/// committed Denver TMY year and reports, per metric, semio vs EnergyPlus vs the ANSI/ASHRAE 140
/// envelope. A case with no committed reference is reported as such and never silently passes.
///
/// Deliberately `#[ignore]`: a full year × ten cases is minutes of work, which does not belong in
/// the default `cargo test`. Run it explicitly with `--ignored` (extra arguments of the crate's `test` dashboard command).
#[test]
#[ignore = "full-year comparison against the committed EnergyPlus references; run explicitly"]
fn bestest_cases_compared_with_energyplus() {
    let Some(weather) = weather_fixture() else {
        panic!("🧫️fixtures/🌦️denver-tmy/🌦️.epw is not committed — there is nothing to compare against");
    };
    let mut lines: Vec<String> = Vec::new();
    let mut failures: Vec<String> = Vec::new();
    for case in CASES {
        let Some(reference) = energyplus_reference(case) else {
            lines.push(format!("{case:<6} | no committed 🔮️energyplus.json"));
            continue;
        };
        let results = run(case, crate::epw::parse(&weather).expect("reference weather"), 7).unwrap_or_else(|diagnostics| panic!("case {case} must run: {:?}", diagnostics.messages));
        let projected = project(case, &results);
        if is_free_float(case) {
            for (metric, ours, path) in [("minC", projected.free_float_min_c, ["freeFloat", "minC"]), ("maxC", projected.free_float_max_c, ["freeFloat", "maxC"]), ("meanC", projected.free_float_mean_c, ["freeFloat", "meanC"])] {
                let theirs = member(&reference, &path).unwrap_or(f64::NAN);
                lines.push(format!("{case:<6} | {metric:<12} | semio {ours:>10.3} | E+ {theirs:>10.3} | Δ {:>8.3} K", ours - theirs));
                if (ours - theirs).abs() > FREE_FLOAT_TOLERANCE_K {
                    failures.push(format!("{case} {metric}: semio {ours:.3} vs E+ {theirs:.3}"));
                }
            }
            continue;
        }
        let bounds = envelope(case);
        for (metric, ours, path, low, high, tolerance) in [
            ("heating kWh", projected.annual_heating_kwh, ["annual", "heatingKwh"], bounds.map(|b| b.0), bounds.map(|b| b.1), ANNUAL_HEATING_TOLERANCE),
            ("cooling kWh", projected.annual_cooling_kwh, ["annual", "coolingKwh"], bounds.map(|b| b.2), bounds.map(|b| b.3), ANNUAL_COOLING_TOLERANCE),
        ] {
            let theirs = member(&reference, &path).unwrap_or(f64::NAN);
            let relative = if theirs.abs() > 1e-9 { (ours - theirs).abs() / theirs.abs() } else { f64::NAN };
            let envelope_text = match (low, high) {
                (Some(low), Some(high)) => format!("140 envelope {low:.0}…{high:.0} [{}]", if ours >= low && ours <= high { "in" } else { "out" }),
                _ => "140 envelope n/a".to_string(),
            };
            lines.push(format!("{case:<6} | {metric:<12} | semio {ours:>10.1} | E+ {theirs:>10.1} | rel {relative:>7.3} | {envelope_text}"));
            if relative > tolerance {
                failures.push(format!("{case} {metric}: semio {ours:.1} vs E+ {theirs:.1} (rel {relative:.3}), {envelope_text}"));
            }
        }
    }
    println!("\n🏛️ ANSI/ASHRAE 140 §5.2 — semio vs EnergyPlus 25.2.0 vs the reference envelope");
    for line in &lines {
        println!("{line}");
    }
    assert!(failures.is_empty(), "{} metric(s) outside tolerance:\n  {}", failures.len(), failures.join("\n  "));
}

/// 📏️ `📓️bestest-contract.md` amendment 2026-09-06: heating 20 %, cooling 10 %. The cooling
/// band is tighter because the whole-assembly simple-glazing window both producers consume
/// (U 3.0 / SHGC 0.787) already sits a documented +5.7…+8.1 % above a layer-by-layer EnergyPlus
/// window, and that offset must not be spent twice.
const ANNUAL_HEATING_TOLERANCE: f64 = 0.20;
const ANNUAL_COOLING_TOLERANCE: f64 = 0.10;
const FREE_FLOAT_TOLERANCE_K: f64 = 2.5;
//#endregion 🔮️EnergyPlusComparison

//#region 🧫️Fixtures
/// 📁️ The committed fixture tree, resolved from the crate manifest rather than the process
/// working directory so it holds under `cargo test` from anywhere.
fn subset_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn fixture_path(case: &str) -> std::path::PathBuf {
    subset_root().join("🧪️tests").join("🧫️fixtures").join(format!("🏛️bestest-{case}")).join("🔋️model.json")
}

fn example_asset_path(directory: &str) -> std::path::PathBuf {
    subset_root().join("🖼️assets").join(directory).join("🗣️.dsl.semio")
}

fn example_dsl(case: &str) -> String {
    let built = model(case).unwrap_or_else(|| panic!("case {case} must build"));
    let snapshot = crate::snapshot::energy_snapshot_with_state(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA, &built, None);
    <crate::snapshot::EnergyModelSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snapshot)
}

/// 🧫️ THE generator. Deliberately inert unless `SEMIO_ENERGY_BESTEST_REGENERATE` is set, so a
/// normal `cargo test` can never make the assertion below pass by rewriting what it checks.
#[test]
fn regenerate_bestest_fixtures() {
    if std::env::var_os("SEMIO_ENERGY_BESTEST_REGENERATE").is_none() {
        return;
    }
    for case in CASES {
        let path = fixture_path(case);
        std::fs::create_dir_all(path.parent().expect("fixture directory")).expect("fixture directory is writable");
        std::fs::write(&path, model_json(case).expect("case serializes")).expect("fixture is writable");
        let asset = example_asset_path(&format!("🏛️bestest-{case}"));
        std::fs::create_dir_all(asset.parent().expect("asset directory")).expect("asset directory is writable");
        std::fs::write(&asset, example_dsl(case)).expect("asset is writable");
    }
    std::fs::write(example_asset_path("🎬️demo"), example_dsl("600")).expect("demo asset is writable");
}

/// 🧫️ Every case's committed fixture must be byte-identical to what the builder produces today.
#[test]
fn committed_bestest_fixtures_match_the_builders() {
    for case in CASES {
        let path = fixture_path(case);
        let committed = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("committed fixture {} is missing: {error}", path.display()));
        let built = model_json(case).expect("case serializes");
        assert_eq!(committed, built, "committed fixture for case {case} is stale — rerun with SEMIO_ENERGY_BESTEST_REGENERATE=1");
    }
}

/// 🧫️ And every committed fixture must decode back into exactly the model it was made from.
#[test]
fn committed_bestest_fixtures_decode_back_into_their_models() {
    for case in CASES {
        let committed = std::fs::read_to_string(fixture_path(case)).expect("committed fixture is present");
        let decoded: Model = semio_framework_pack_json::from_json_str(&committed, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("committed fixture for case {case} does not decode: {error}"));
        assert_eq!(decoded, model(case).expect("case builds"), "committed fixture for case {case} decodes into a different model");
    }
}

/// 🧫️ Each example's committed DSL asset must carry the same case model, through this subset's
/// own text codec rather than through the fixture JSON.
#[test]
fn committed_example_assets_match_the_builders() {
    for case in CASES {
        let path = example_asset_path(&format!("🏛️bestest-{case}"));
        let committed = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("committed asset {} is missing: {error}", path.display()));
        assert_eq!(committed, example_dsl(case), "committed asset for case {case} is stale — rerun with SEMIO_ENERGY_BESTEST_REGENERATE=1");
    }
    assert_eq!(std::fs::read_to_string(example_asset_path("🎬️demo")).expect("demo asset is present"), example_dsl("600"));
}
//#endregion 🧫️Fixtures
