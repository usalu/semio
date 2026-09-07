//! ⚡️ `s.energy.model` epJSON io case — Rust adapter, SUBJECT role for every scenario.
//!
//! The subject's real output is the RAW epJSON: `Outcome::with_raw` hands those exact bytes to the
//! Python oracle under the feature's `@oracle-input-subject-raw` tag, which is how a third-party
//! JSON Schema validator and EnergyPlus itself get to read the document semio wrote without any
//! part of this crate being in the path. The PROJECTION beside them is what the subject claims
//! about those bytes; the oracle recomputes every claimed field from the bytes themselves, so a
//! wrong document cannot agree.
//!
//! The bridge into the crate is text in, text out (`epjson_from_model_json`,
//! `epjson_diagnostics_json`), for the same reason `bestest::model_json` is: this adapter cannot
//! name `Model`, `EnergyModelSnapshot` or `pack::json::Value`.
//!
//! ⚠️ There is no skip channel in this host — a missing toolchain, a missing fixture and a wrong
//! answer are all results. A case with no committed reference fails with a message naming the file.
//!
//! @see ../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📓️w6-epjson-io.md

use semio_repo_test_host::Adapter;

//#region 🔖️Cases
/// ⚡️ Every case whose `🔋️model.json` and `🔮️energyplus.json` are both committed — the ten the
/// honeybee route produced. 630/930 (fins) and 650/950 (night ventilation) have no reference, so
/// registering them could only manufacture reds.
const VALIDATED: &[&str] = &["600", "600FF", "610", "620", "640", "900", "900FF", "910", "920", "940"];

/// ⚡️ The cases whose export is also SIMULATED. Each is ~25 s of EnergyPlus and every one of them
/// exercises the same codec on the same object subset, so the run family stays at four while the
/// schema family covers all ten.
const SIMULATED: &[&str] = &["600", "600FF", "900", "900FF"];

/// 🧾️ The cases whose export is checked to have dropped nothing without saying so.
const ACCOUNTED: &[&str] = &["600", "900"];
//#endregion 🔖️Cases

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};

    /// 📏️ Both sides of this comparison are EnergyPlus 25.2.0 reading the same simple glazing, so
    /// the band is 3 % rather than the sibling case's 10 %: the documented glazing offset is common
    /// to both and cannot show up here as a difference.
    const ANNUAL_TOLERANCE: f64 = 0.03;
    /// 📏️ Free-float cases are judged on temperature, in kelvin.
    const FREE_FLOAT_TOLERANCE_K: f64 = 0.5;

    const FACTS_SCHEMA: &str = "semio.energy.epjson-facts/1";
    const ACCOUNTING_SCHEMA: &str = "semio.energy.epjson-accounting/1";
    const ROUTE_SCHEMA: &str = "semio.energy.epjson-route/1";

    fn model_asset(case: &str) -> String {
        format!("asset://🧫️fixtures/🏛️bestest-{case}/🔋️model.json")
    }

    /// 📥️ The committed case model, or an error naming exactly what is absent.
    fn model_text(ctx: &Context, case: &str) -> Result<String, String> {
        let uri = model_asset(case);
        let bytes = ctx.fixture_bytes(&uri).map_err(|error| format!("case {case}: no committed model at {uri} ({error})"))?;
        String::from_utf8(bytes).map_err(|error| format!("case {case}: the committed model is not UTF-8: {error}"))
    }

    fn object_names(document: &Json, kind: &str) -> Vec<String> {
        match document.get(kind) {
            Some(Json::Object(entries)) => entries.iter().map(|(name, _)| name.clone()).collect(),
            _ => Vec::new(),
        }
    }

    fn object_count(document: &Json, kind: &str) -> f64 {
        object_names(document, kind).len() as f64
    }

    fn first_number(document: &Json, kind: &str, field: &str) -> Option<f64> {
        match document.get(kind) {
            Some(Json::Object(entries)) => entries.first().and_then(|(_, value)| match value.get(field) {
                Some(Json::Number(number)) => Some(*number),
                _ => None,
            }),
            _ => None,
        }
    }

    /// 🔢️ Six decimals, so two producers that reach the same number by different arithmetic still
    /// project the same bytes under an exact comparison.
    fn round6(value: f64) -> f64 {
        (value * 1e6).round() / 1e6
    }

    fn round2(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    fn distance(a: &[f64; 3], b: &[f64; 3]) -> f64 {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
    }

    /// 📐️ The four corners of one `FenestrationSurface:Detailed`, in field order.
    fn aperture_corners(fields: &Json) -> Option<[[f64; 3]; 4]> {
        let mut corners = [[0.0f64; 3]; 4];
        for (index, corner) in corners.iter_mut().enumerate() {
            for (axis, name) in ["x", "y", "z"].iter().enumerate() {
                match fields.get(&format!("vertex_{}_{name}_coordinate", index + 1)) {
                    Some(Json::Number(value)) => corner[axis] = *value,
                    _ => return None,
                }
            }
        }
        Some(corners)
    }

    /// 📐️ Total glazed area of the document, from the aperture rectangles themselves rather than
    /// from the model the document was written out of.
    fn aperture_area(document: &Json) -> f64 {
        match document.get("FenestrationSurface:Detailed") {
            Some(Json::Object(entries)) => entries.iter().filter_map(|(_, fields)| aperture_corners(fields)).map(|corners| distance(&corners[0], &corners[1]) * distance(&corners[1], &corners[2])).sum(),
            _ => 0.0,
        }
    }

    /// 📐️ Planar polygon area of one `BuildingSurface:Detailed`, as half the Newell vector's length.
    fn newell_area(fields: &Json) -> f64 {
        let vertices: Vec<[f64; 3]> = match fields.get("vertices") {
            Some(Json::Array(items)) => items
                .iter()
                .map(|vertex| {
                    let read = |key: &str| match vertex.get(key) {
                        Some(Json::Number(value)) => *value,
                        _ => 0.0,
                    };
                    [read("vertex_x_coordinate"), read("vertex_y_coordinate"), read("vertex_z_coordinate")]
                })
                .collect(),
            _ => Vec::new(),
        };
        if vertices.len() < 3 {
            return 0.0;
        }
        let mut accumulated = [0.0f64; 3];
        for index in 0..vertices.len() {
            let current = vertices[index];
            let next = vertices[(index + 1) % vertices.len()];
            accumulated[0] += (current[1] - next[1]) * (current[2] + next[2]);
            accumulated[1] += (current[2] - next[2]) * (current[0] + next[0]);
            accumulated[2] += (current[0] - next[0]) * (current[1] + next[1]);
        }
        0.5 * (accumulated[0] * accumulated[0] + accumulated[1] * accumulated[1] + accumulated[2] * accumulated[2]).sqrt()
    }

    fn number_member(name: &str, value: f64) -> (String, Json) {
        (name.to_string(), Json::Number(value))
    }

    fn string_array(values: Vec<String>) -> Json {
        Json::Array(values.into_iter().map(Json::String).collect())
    }

    fn sorted(mut values: Vec<String>) -> Vec<String> {
        values.sort();
        values
    }

    /// 📋️ Everything both implementations read out of the exported document. Sorted, rounded, and
    /// derived from the DOCUMENT rather than from the model, so the two sides read one artifact.
    fn document_facts(case: &str, document: &Json) -> Json {
        let object_types = sorted(match document {
            Json::Object(entries) => entries.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>(),
            _ => Vec::new(),
        });
        let counts = Json::Object(vec![
            number_member("materials", object_count(document, "Material") + object_count(document, "Material:NoMass")),
            number_member("constructions", object_count(document, "Construction")),
            number_member("glazing", object_count(document, "WindowMaterial:SimpleGlazingSystem")),
            number_member("schedules", object_count(document, "Schedule:Constant") + object_count(document, "Schedule:Compact")),
            number_member("infiltration", object_count(document, "ZoneInfiltration:DesignFlowRate")),
            number_member("idealLoads", object_count(document, "ZoneHVAC:IdealLoadsAirSystem")),
            number_member("thermostats", object_count(document, "ZoneControl:Thermostat")),
            number_member("outputVariables", object_count(document, "Output:Variable")),
        ]);
        let version = document
            .get("Version")
            .and_then(|group| match group {
                Json::Object(entries) => entries.first().map(|(_, fields)| fields.str("version_identifier")),
                _ => None,
            })
            .unwrap_or_default();
        let run_period = document
            .get("RunPeriod")
            .and_then(|group| match group {
                Json::Object(entries) => entries.first().map(|(_, fields)| {
                    let read = |key: &str| match fields.get(key) {
                        Some(Json::Number(value)) => *value as i64,
                        _ => 0,
                    };
                    format!("{}/{}-{}/{}", read("begin_month"), read("begin_day_of_month"), read("end_month"), read("end_day_of_month"))
                }),
                _ => None,
            })
            .unwrap_or_default();
        Json::Object(vec![
            ("schema".to_string(), Json::String(FACTS_SCHEMA.to_string())),
            ("case".to_string(), Json::String(case.to_string())),
            ("epJsonVersion".to_string(), Json::String(version)),
            ("objectTypes".to_string(), string_array(object_types)),
            ("zoneNames".to_string(), string_array(sorted(object_names(document, "Zone")))),
            ("surfaceNames".to_string(), string_array(sorted(object_names(document, "BuildingSurface:Detailed")))),
            ("apertureNames".to_string(), string_array(sorted(object_names(document, "FenestrationSurface:Detailed")))),
            ("counts".to_string(), counts),
            number_member("zoneVolumeM3", round6(first_number(document, "Zone", "volume").unwrap_or(0.0))),
            number_member("totalApertureAreaM2", round6(aperture_area(document))),
            number_member("windowUFactor", round6(first_number(document, "WindowMaterial:SimpleGlazingSystem", "u_factor").unwrap_or(0.0))),
            number_member("windowShgc", round6(first_number(document, "WindowMaterial:SimpleGlazingSystem", "solar_heat_gain_coefficient").unwrap_or(0.0))),
            number_member("infiltrationAch", round6(first_number(document, "ZoneInfiltration:DesignFlowRate", "air_changes_per_hour").unwrap_or(0.0))),
            ("runPeriod".to_string(), Json::String(run_period)),
        ])
    }

    /// 🧾️ Every name the committed model states, so the oracle can check that none of them vanished
    /// from the document without a diagnostic naming it.
    fn model_entity_names(model: &Json) -> Vec<String> {
        let mut named = Vec::new();
        for (collection, kind) in [("zones", "zone"), ("surfaces", "surface"), ("fenestrations", "fenestration"), ("materials", "material"), ("constructions", "construction")] {
            for entity in model.array(collection) {
                let name = entity.str("name");
                if !name.trim().is_empty() {
                    named.push(format!("{kind} {name}"));
                }
            }
        }
        sorted(named)
    }

    /// 📤️ The one place the crate is called: the committed model in, the exported epJSON out.
    fn export(ctx: &Context, case: &str) -> Result<(String, Json), String> {
        let model_json = model_text(ctx, case)?;
        let epjson = semio_s_plugin_energy::artifacts::model::io::export::serializers::artifacts::epjson::v25_2::any::epjson_from_model_json(&model_json).map_err(|error| format!("case {case}: {error}"))?;
        let document = semio_repo_test_host::parse_json(&epjson)?;
        Ok((epjson, document))
    }

    fn publish(ctx: &Context, epjson: &str) -> Result<(), String> {
        let path = ctx.artifact("semio", "⚡️model.epJSON")?;
        std::fs::write(&path, epjson).map_err(|error| format!("cannot write {}: {error}", path.display()))
    }

    pub fn schema_validity(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (epjson, document) = export(ctx, case)?;
            publish(ctx, &epjson)?;
            Ok(Outcome::with_raw(epjson.into_bytes(), document_facts(case, &document)))
        }
    }

    pub fn nothing_dropped(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let model_json = model_text(ctx, case)?;
            let (epjson, _) = export(ctx, case)?;
            publish(ctx, &epjson)?;
            let diagnostics = semio_s_plugin_energy::artifacts::model::io::export::serializers::artifacts::epjson::v25_2::any::epjson_diagnostics_json(&model_json).map_err(|error| format!("case {case}: {error}"))?;
            let codes = match semio_repo_test_host::parse_json(&diagnostics)? {
                Json::Array(items) => items.iter().map(|item| item.str("code")).collect::<Vec<_>>(),
                _ => Vec::new(),
            };
            let model = semio_repo_test_host::parse_json(&model_json)?;
            let missing: Vec<String> = model_entity_names(&model).into_iter().filter(|entry| !epjson.contains(&format!("\"{}", entry.split_once(' ').map(|(_, name)| name).unwrap_or(entry)))).collect();
            if !missing.is_empty() && codes.is_empty() {
                return Err(format!("case {case}: {} model entities are absent from the exported epJSON and NO diagnostic names them: {}", missing.len(), missing.join(", ")));
            }
            Ok(Outcome::with_raw(
                epjson.into_bytes(),
                Json::Object(vec![
                    ("schema".to_string(), Json::String(ACCOUNTING_SCHEMA.to_string())),
                    ("case".to_string(), Json::String(case.to_string())),
                    ("unaccountedEntities".to_string(), string_array(missing)),
                    ("diagnosticCodes".to_string(), string_array(sorted(codes))),
                ]),
            ))
        }
    }

    /// ⚡️ The claim the oracle's own EnergyPlus run has to reproduce: no schema violation, the
    /// building EnergyPlus reports back in its own `eplusout.eio`, and agreement with the committed
    /// reference on every judged metric.
    pub fn energyplus_run(case: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (epjson, document) = export(ctx, case)?;
            publish(ctx, &epjson)?;
            let free_float = object_count(&document, "ZoneHVAC:IdealLoadsAirSystem") == 0.0;
            let floor_area: f64 = match document.get("BuildingSurface:Detailed") {
                Some(Json::Object(entries)) => entries.iter().filter(|(_, fields)| fields.str("surface_type") == "Floor").map(|(_, fields)| newell_area(fields)).sum(),
                _ => 0.0,
            };
            let echoed = Json::Object(vec![
                number_member("zoneVolumeM3", round2(first_number(&document, "Zone", "volume").unwrap_or(0.0))),
                number_member("floorAreaM2", round2(floor_area)),
                number_member("exteriorWindowAreaM2", round2(aperture_area(&document))),
                number_member("surfaces", object_count(&document, "BuildingSurface:Detailed")),
                number_member("subSurfaces", object_count(&document, "FenestrationSurface:Detailed")),
            ]);
            Ok(Outcome::with_raw(
                epjson.into_bytes(),
                Json::Object(vec![
                    ("schema".to_string(), Json::String(ROUTE_SCHEMA.to_string())),
                    ("case".to_string(), Json::String(case.to_string())),
                    number_member("epJsonSchemaViolations", 0.0),
                    ("energyPlus".to_string(), echoed),
                    ("annualWithinTolerance".to_string(), if free_float { Json::Null } else { Json::Bool(true) }),
                    ("freeFloatWithinTolerance".to_string(), if free_float { Json::Bool(true) } else { Json::Null }),
                    number_member("annualToleranceRelative", ANNUAL_TOLERANCE),
                    number_member("freeFloatToleranceK", FREE_FLOAT_TOLERANCE_K),
                ]),
            ))
        }
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration by FULL expanded scenario id, mirroring the feature's `Examples` tables. Subject
/// role only: the reference side is EnergyPlus itself, driven by the Python adapter, and registering
/// this crate as its own oracle would compare the codec with itself.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for case in VALIDATED {
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("schema-validity-{case}"), subject::schema_validity(case));
        }
        let _ = case;
    }
    for case in ACCOUNTED {
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("nothing-dropped-{case}"), subject::nothing_dropped(case));
        }
        let _ = case;
    }
    for case in SIMULATED {
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("energyplus-run-{case}"), subject::energyplus_run(case));
        }
        let _ = case;
    }
    built
}
//#endregion 🔖️Registration
