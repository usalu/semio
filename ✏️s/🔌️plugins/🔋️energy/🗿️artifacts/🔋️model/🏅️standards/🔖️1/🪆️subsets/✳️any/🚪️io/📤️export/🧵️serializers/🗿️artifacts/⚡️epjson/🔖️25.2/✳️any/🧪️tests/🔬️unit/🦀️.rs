use super::*;

fn case(name: &str) -> Model {
    crate::bestest::model(name).expect("bestest case")
}

#[semio_framework_async_macros::async_test]
async fn writes_every_contract_object_type_for_case_600() {
    let (document, diagnostics) = encode_model_with_diagnostics(&case("600"));
    let Value::Object(root) = &document else { panic!("epJSON root must be an object") };
    for required in [
        "Version",
        "SimulationControl",
        "Building",
        "ShadowCalculation",
        "Site:Location",
        "GlobalGeometryRules",
        "Timestep",
        "RunPeriod",
        "ScheduleTypeLimits",
        "Schedule:Constant",
        "Material",
        "Material:NoMass",
        "WindowMaterial:Glazing",
        "WindowMaterial:Gas",
        "WindowMaterial:SimpleGlazingSystem",
        "Construction",
        "Zone",
        "BuildingSurface:Detailed",
        "FenestrationSurface:Detailed",
        "ZoneInfiltration:DesignFlowRate",
        "ElectricEquipment",
        "ThermostatSetpoint:DualSetpoint",
        "ZoneControl:Thermostat",
        "ZoneHVAC:IdealLoadsAirSystem",
        "ZoneHVAC:EquipmentList",
        "ZoneHVAC:EquipmentConnections",
        "NodeList",
        "Output:Variable",
        "Output:Meter",
        "Output:SQLite",
    ] {
        assert!(root.contains_key(required), "case 600 epJSON is missing {required}");
    }
    assert!(diagnostics.is_empty(), "case 600 must export without refusals, got {diagnostics:?}");
}

#[semio_framework_async_macros::async_test]
async fn places_the_two_ashrae_140_south_windows_where_the_standard_puts_them() {
    let model = case("600");
    let south = model.surfaces.iter().find(|surface| surface.name == "South Wall").expect("south wall");
    let hosted: Vec<&Fenestration> = model.fenestrations.iter().filter(|window| window.surface_id == south.id).collect();
    assert_eq!(hosted.len(), 2, "case 600 has two south windows");
    let mut spans = Vec::new();
    for (index, window) in hosted.iter().enumerate() {
        let rectangle = aperture_rectangle(south, window, index, hosted.len()).expect("rectangle");
        let xs: Vec<f64> = rectangle.iter().map(|vertex| vertex[0]).collect();
        let zs: Vec<f64> = rectangle.iter().map(|vertex| vertex[2]).collect();
        spans.push((xs.iter().cloned().fold(f64::INFINITY, f64::min), xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max), zs.iter().cloned().fold(f64::INFINITY, f64::min), zs.iter().cloned().fold(f64::NEG_INFINITY, f64::max)));
    }
    spans.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("finite"));
    for (index, expected) in [(0.5, 3.5), (4.5, 7.5)].iter().enumerate() {
        assert!((spans[index].0 - expected.0).abs() < 1e-6, "window {index} starts at {} not {}", spans[index].0, expected.0);
        assert!((spans[index].1 - expected.1).abs() < 1e-6, "window {index} ends at {} not {}", spans[index].1, expected.1);
        assert!((spans[index].2 - 0.2).abs() < 1e-6, "window {index} sill is {}", spans[index].2);
        assert!((spans[index].3 - 2.2).abs() < 1e-6, "window {index} head is {}", spans[index].3);
    }
}

#[semio_framework_async_macros::async_test]
async fn outward_normals_survive_the_documented_counterclockwise_winding() {
    let model = case("600");
    let expected: [(&str, [f64; 3]); 6] = [("South Wall", [0.0, -1.0, 0.0]), ("East Wall", [1.0, 0.0, 0.0]), ("North Wall", [0.0, 1.0, 0.0]), ("West Wall", [-1.0, 0.0, 0.0]), ("Roof", [0.0, 0.0, 1.0]), ("Floor", [0.0, 0.0, -1.0])];
    for (name, normal) in expected {
        let surface = model.surfaces.iter().find(|surface| surface.name == name).expect("surface");
        let computed = surface_normal(&surface.vertices_m).expect("normal");
        for axis in 0..3 {
            assert!((computed[axis] - normal[axis]).abs() < 1e-9, "{name} normal {computed:?} != {normal:?}");
        }
    }
}

fn object<'a>(root: &'a Object, kind: &str, name: &str) -> &'a Object {
    let Some(Value::Object(group)) = root.get(kind) else { panic!("{kind} is missing") };
    let Some(Value::Object(entry)) = group.get(name) else { panic!("{kind} {name:?} is missing") };
    entry
}

fn corner(entry: &Object, index: usize) -> Vec3 {
    let read = |axis: &str| entry.get(&format!("vertex_{index}_{axis}_coordinate")).and_then(Value::as_f64).expect("aperture corner");
    [read("x"), read("y"), read("z")]
}

fn first_vertex(entry: &Object) -> Vec3 {
    let Some(Value::Array(vertices)) = entry.get("vertices") else { panic!("vertices") };
    let Some(Value::Object(vertex)) = vertices.first() else { panic!("first vertex") };
    let read = |key: &str| vertex.get(key).and_then(Value::as_f64).expect("coordinate");
    [read("vertex_x_coordinate"), read("vertex_y_coordinate"), read("vertex_z_coordinate")]
}

/// 💨️ The engine runs `ScheduledAch` as schedule × rate, so the document states a constant design
/// flow whatever the model's (unused) coefficient fields hold — case 600 carries zeros there.
#[semio_framework_async_macros::async_test]
async fn a_scheduled_air_change_rate_is_a_constant_design_flow() {
    let model = case("600");
    assert!(model.infiltrations.iter().all(|entry| entry.constant_term_coefficient == 0.0), "case 600's infiltration carries the unused zero coefficients");
    let Value::Object(root) = encode_model(&model) else { panic!("object") };
    let Some(Value::Object(group)) = root.get("ZoneInfiltration:DesignFlowRate") else { panic!("infiltration") };
    for (_, value) in group.iter() {
        let Value::Object(entry) = value else { panic!("object") };
        let read = |key: &str| entry.get(key).and_then(Value::as_f64).expect("coefficient");
        assert_eq!([read("constant_term_coefficient"), read("temperature_term_coefficient"), read("velocity_term_coefficient"), read("velocity_squared_term_coefficient")], CONSTANT_DESIGN_FLOW_COEFFICIENTS);
    }
}

/// 🪟️ Case 600's windows name the two-pane `Double Clear Glazing` stack, so their apertures are that
/// stack; the simple-glazing fallback travels as a material only, with no construction of its own.
#[semio_framework_async_macros::async_test]
async fn a_window_with_a_glazing_construction_is_its_layered_stack() {
    let model = case("600");
    let Value::Object(root) = encode_model(&model) else { panic!("object") };
    for window in &model.fenestrations {
        let stack = glazing_stack(&model, window).expect("case 600 binds its windows to a glazing construction");
        assert_eq!(object(&root, "FenestrationSurface:Detailed", &window.name).get("construction_name").and_then(Value::as_str), Some(stack.name.as_str()));
        let Some(Value::Object(constructions)) = root.get("Construction") else { panic!("Construction") };
        assert!(!constructions.contains_key(&glazing_construction_name(&window.name)), "{}: a bound window writes no simple-glazing construction", window.name);
        assert_eq!(object(&root, "WindowMaterial:SimpleGlazingSystem", &glazing_material_name(&window.name)).get("u_factor").and_then(Value::as_f64), Some(window.u_value_w_m2k));
    }
    let unbound = Model { fenestrations: model.fenestrations.iter().cloned().map(|window| Fenestration { glazing_construction_id: None, ..window }).collect(), ..model.clone() };
    let Value::Object(root) = encode_model(&unbound) else { panic!("object") };
    for window in &unbound.fenestrations {
        assert_eq!(object(&root, "FenestrationSurface:Detailed", &window.name).get("construction_name").and_then(Value::as_str), Some(glazing_construction_name(&window.name).as_str()));
    }
}

/// 📐️ Every ring starts at its upper-left corner seen from outside — exactly where NREL's own ASHRAE
/// 140 encoding (honeybee → OpenStudio, the committed reference's producer) starts it.
#[semio_framework_async_macros::async_test]
async fn every_ring_starts_at_its_upper_left_corner_seen_from_outside() {
    let model = case("600");
    let Value::Object(root) = encode_model(&model) else { panic!("object") };
    for (surface, expected) in [("South Wall", [0.0, 0.0, 2.7]), ("East Wall", [8.0, 0.0, 2.7]), ("North Wall", [8.0, 6.0, 2.7]), ("West Wall", [0.0, 6.0, 2.7]), ("Roof", [0.0, 6.0, 2.7]), ("Floor", [8.0, 6.0, 0.0])] {
        let first = first_vertex(object(&root, "BuildingSurface:Detailed", surface));
        assert!((0..3).all(|axis| (first[axis] - expected[axis]).abs() < 1e-9), "{surface} starts at {first:?}, not its upper-left corner {expected:?}");
    }
    for (window, upper_left, lower_left) in [("South Window West", [0.5, 0.0, 2.2], [0.5, 0.0, 0.2]), ("South Window East", [4.5, 0.0, 2.2], [4.5, 0.0, 0.2])] {
        let entry = object(&root, "FenestrationSurface:Detailed", window);
        for (index, expected) in [(1, upper_left), (2, lower_left)] {
            let found = corner(entry, index);
            assert!((0..3).all(|axis| (found[axis] - expected[axis]).abs() < 1e-9), "{window} corner {index} is {found:?}, not {expected:?}");
        }
    }
    for surface in &model.surfaces {
        let rotated = upper_left_first(&surface.vertices_m);
        let offset = rotated.iter().position(|vertex| *vertex == surface.vertices_m[0]).expect("a rotation keeps every vertex");
        assert!((0..rotated.len()).all(|index| rotated[(offset + index) % rotated.len()] == surface.vertices_m[index]), "{} keeps its winding", surface.name);
    }
}

#[semio_framework_async_macros::async_test]
async fn massless_ashrae_140_insulation_becomes_material_no_mass() {
    let model = case("600");
    let insulation = model.materials.iter().find(|material| material.name == "Floor Insulation").expect("floor insulation");
    assert!(is_massless(insulation), "the standard tabulates this layer with zero density and specific heat");
    let Value::Object(root) = encode_model(&model) else { panic!("object") };
    let Some(Value::Object(no_mass)) = root.get("Material:NoMass") else { panic!("Material:NoMass") };
    let Some(Value::Object(entry)) = no_mass.get("Floor Insulation") else { panic!("Floor Insulation") };
    let resistance = entry.get("thermal_resistance").and_then(Value::as_f64).expect("resistance");
    assert!((resistance - insulation.thickness_m / insulation.conductivity_w_m_k).abs() < 1e-9, "R must be thickness/conductivity, got {resistance}");
}

#[semio_framework_async_macros::async_test]
async fn a_free_float_case_writes_no_hvac_at_all() {
    let Value::Object(root) = encode_model(&case("600FF")) else { panic!("object") };
    for absent in ["ZoneHVAC:IdealLoadsAirSystem", "ZoneControl:Thermostat", "ThermostatSetpoint:DualSetpoint", "ZoneHVAC:EquipmentList"] {
        assert!(!root.contains_key(absent), "600FF must not carry {absent}");
    }
}

//#region 🧫️Fixtures
/// 🧫️ The cases whose exported document is COMMITTED, so `oracle-epjson` and the
/// `🔮️oracles🔋️energy⚡️epjson` launch entry have a real file to hand EnergyPlus without first
/// running a Rust test, and so a reviewer can read the document the codec actually writes.
/// Exactly the four the `🏛️export-epjson-runs-in-energyplus` case simulates.
const COMMITTED_EPJSON_CASES: [&str; 4] = ["600", "600FF", "900", "900FF"];

fn epjson_fixture_path(case: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures").join(format!("🏛️bestest-{case}")).join("⚡️model.epJSON")
}

fn exported(name: &str) -> String {
    format!("{}\n", pack::json::to_string_pretty(&encode_model(&case(name))))
}

/// 🧫️ THE generator, inert unless `SEMIO_ENERGY_EPJSON_REGENERATE` is set, so a normal
/// `cargo test` can never make the guard below pass by rewriting what it checks.
#[test]
fn regenerate_committed_epjson_fixtures() {
    if std::env::var_os("SEMIO_ENERGY_EPJSON_REGENERATE").is_none() {
        return;
    }
    for name in COMMITTED_EPJSON_CASES {
        let path = epjson_fixture_path(name);
        std::fs::create_dir_all(path.parent().expect("fixture directory")).expect("fixture directory is writable");
        std::fs::write(&path, exported(name)).expect("fixture is writable");
    }
}

/// 🧫️ Every committed document must be byte-identical to what the codec writes today.
#[test]
fn committed_epjson_fixtures_match_the_codec() {
    for name in COMMITTED_EPJSON_CASES {
        let path = epjson_fixture_path(name);
        let committed = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("committed epJSON {} is missing: {error} — rerun with SEMIO_ENERGY_EPJSON_REGENERATE=1", path.display()));
        assert_eq!(committed, exported(name), "committed epJSON for case {name} is stale — rerun with SEMIO_ENERGY_EPJSON_REGENERATE=1");
    }
}
//#endregion 🧫️Fixtures

#[semio_framework_async_macros::async_test]
async fn a_time_series_schedule_is_reported_rather_than_dropped() {
    let mut model = case("600");
    model.schedules.time_series.push(crate::schedule::TimeSeriesSchedule { id: ScheduleId(99), values: vec![1.0, 2.0], timestep_seconds: 3600 });
    let (_, diagnostics) = encode_model_with_diagnostics(&model);
    assert!(diagnostics.iter().any(|diagnostic| diagnostic.code == "epjson.schedule.time-series-unsupported"), "expected a structured refusal, got {diagnostics:?}");
}
