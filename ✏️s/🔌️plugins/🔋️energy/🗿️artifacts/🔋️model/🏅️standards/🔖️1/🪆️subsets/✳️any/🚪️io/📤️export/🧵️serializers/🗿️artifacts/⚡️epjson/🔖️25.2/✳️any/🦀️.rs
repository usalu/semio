//! model -> epJSON
//!
//! ⚡️ REAL codec, not a bridge: a hand-written EnergyPlus 25.2 `epJSON` writer over
//! [`pack::json`], with no third-party translator in the path. Every document it emits validates
//! against EnergyPlus's own bundled `Energy+.schema.epJSON` (draft-04-shaped JSON Schema) and runs
//! unmodified under `energyplus -a -w <epw> -d <dir> <file.epJSON>` — both are asserted by
//! `🧪️tests/🏛️export-epjson-runs-in-energyplus`, the second (honeybee-free) physics oracle route.
//!
//! Object types written: `Version`, `SimulationControl`, `Building`, `Site:Location`,
//! `Site:GroundTemperature:BuildingSurface`, `GlobalGeometryRules`, `Timestep`, `RunPeriod`,
//! `ScheduleTypeLimits`, `Schedule:Constant`, `Schedule:Compact`, `Material`, `Material:NoMass`,
//! `WindowMaterial:SimpleGlazingSystem`, `Construction`, `Zone`, `BuildingSurface:Detailed`,
//! `FenestrationSurface:Detailed`, `Shading:Overhang:Projection`, `Shading:Fin:Projection`,
//! `ZoneInfiltration:DesignFlowRate`, `People`, `Lights`, `ElectricEquipment`,
//! `ThermostatSetpoint:DualSetpoint`, `ZoneControl:Thermostat`, `ZoneHVAC:IdealLoadsAirSystem`,
//! `ZoneHVAC:EquipmentList`, `ZoneHVAC:EquipmentConnections`, `NodeList`, `Sizing:Zone`,
//! `Output:Variable`, `Output:Meter`, `Output:SQLite`.
//!
//! # Three conventions this file DECLARES, because the semio schema does not state them
//! 1. **Terrain and solar distribution.** [`Model`] carries neither. `Building.terrain` is written
//!    as `Country` and `solar_distribution` as `FullInteriorAndExterior` — the ANSI/ASHRAE 140
//!    §5.2 settings, and byte-identical to what the honeybee→OpenStudio route emits, so the two
//!    producers stay comparable (`📓️w2-oracle-toolchain.md` §5).
//! 2. **Aperture geometry.** [`Fenestration`] carries an area (plus an optional height/sill) and
//!    no vertices. Windows on one host surface are laid out one per equal horizontal bay, centred,
//!    with `height_m`/`sill_height_m` when non-zero and otherwise a 1.5 width/height aspect at a
//!    0.2 m sill — the same rule the honeybee translator declares, which reproduces ASHRAE 140's
//!    own two 3.0 m × 2.0 m south windows at x 0.5–3.5 / 4.5–7.5 exactly.
//! 3. **Vertex winding.** `GlobalGeometryRules` is written `UpperLeftCorner` /
//!    `Counterclockwise` / `World`, and `Surface.vertices_m` is emitted verbatim: the model's own
//!    documented winding is already "counter-clockwise seen from OUTSIDE" (see
//!    `⚙️engine/🏛️bestest/🦀️.rs`'s `surfaces()`), which is exactly what `Counterclockwise` means
//!    to EnergyPlus. `starting_vertex_position` is not consulted for `*:Detailed` surfaces.
//!
//! Anything the model can express but epJSON's covered subset cannot is reported as an
//! [`EpJsonDiagnostic`], never silently dropped.
//!
//! @see https://energyplus.readthedocs.io/en/latest/schema.html
//! @see ../../../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📓️w6-epjson-io.md
use crate::air_exchange::InfiltrationMethod;
use crate::artifacts::model::EnergyModelSnapshot;
use crate::model::{Construction, EntityId, Fenestration, Material, Model, OutsideBoundary, ScheduleId, Surface, SurfaceClass};
use pack::json::{Object, Value};

//#region 🔖️Constants
/// ⚡️ The EnergyPlus release whose `Energy+.schema.epJSON` this codec targets.
pub const EPJSON_VERSION: &str = "25.2";
/// 🌾️ Declared convention #1 — see the module docstring.
pub const EPJSON_TERRAIN: &str = "Country";
/// ☀️ Declared convention #1 — see the module docstring.
pub const EPJSON_SOLAR_DISTRIBUTION: &str = "FullInteriorAndExterior";
/// 🪟️ Declared convention #2 — width/height of an aperture with no stated `height_m`.
pub const APERTURE_ASPECT: f64 = 1.5;
/// 🪟️ Declared convention #2 — sill of an aperture with no stated `sill_height_m`.
pub const APERTURE_SILL_M: f64 = 0.2;
/// 📅️ Name of the dimensionless `ScheduleTypeLimits` every emitted schedule references.
pub const SCHEDULE_LIMITS_ANY: &str = "semio Any Number";
/// 🎛️ Name of the `ScheduleTypeLimits` the thermostat control-type schedule references.
pub const SCHEDULE_LIMITS_CONTROL: &str = "semio Control Type";
/// 🎛️ Name of the constant `4` schedule that selects `ThermostatSetpoint:DualSetpoint`.
pub const DUAL_SETPOINT_CONTROL_SCHEDULE: &str = "semio Dual Setpoint Control";
/// 🧮️ Timesteps per hour written into `Timestep`; matches the ASHRAE 140 reference runs.
pub const TIMESTEPS_PER_HOUR: i64 = 6;
/// 📊️ The four zone variables `📓️bestest-contract.md` compares both producers on.
pub const CONTRACT_OUTPUT_VARIABLES: &[&str] = &["Zone Mean Air Temperature", "Zone Ideal Loads Supply Air Total Heating Energy", "Zone Ideal Loads Supply Air Total Cooling Energy", "Surface Window Transmitted Solar Radiation Energy"];
/// 🧮️ The two ideal-loads meters the contract's annual totals are read from.
pub const CONTRACT_OUTPUT_METERS: &[&str] = &["Heating:DistrictHeatingWater", "Cooling:DistrictCooling"];
//#endregion 🔖️Constants

//#region 🔖️Diagnostic
/// 🚨️ One thing the model states that this codec's epJSON subset cannot carry. Reported, never
/// dropped: `code` is a stable machine token, `subject` names the offending entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpJsonDiagnostic {
    pub code: String,
    pub subject: String,
    pub message: String,
}

impl EpJsonDiagnostic {
    pub fn new(code: impl Into<String>, subject: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), subject: subject.into(), message: message.into() }
    }

    /// 🌳️ Wire form, so a caller can surface diagnostics next to the document itself.
    pub fn to_json(&self) -> Value {
        Value::Object(Object::from_iter([("code".to_string(), Value::from(self.code.as_str())), ("subject".to_string(), Value::from(self.subject.as_str())), ("message".to_string(), Value::from(self.message.as_str()))]))
    }
}
//#endregion 🔖️Diagnostic

//#region 🔖️Names
/// 🏷️ Deterministic epJSON name for a schedule, which the semio model identifies by number only.
/// The number is part of the name so the import leaf recovers the exact [`ScheduleId`].
pub fn schedule_name(id: ScheduleId) -> String {
    format!("Schedule {}", id.0)
}

/// 🏷️ Falls back to a stable id-derived name when an entity carries an empty one.
fn entity_name(kind: &str, id: EntityId, name: &str) -> String {
    if name.trim().is_empty() {
        format!("{kind} {}", id.0)
    } else {
        name.to_string()
    }
}

/// 🪟️ Name of the single-layer glazing construction generated for one fenestration.
pub fn glazing_construction_name(window: &str) -> String {
    format!("{window} Glazing Construction")
}

/// 🪟️ Name of the `WindowMaterial:SimpleGlazingSystem` generated for one fenestration.
pub fn glazing_material_name(window: &str) -> String {
    format!("{window} Glazing")
}

/// 🌬️ Node/list/equipment names of one zone's ideal-loads plumbing.
pub fn ideal_loads_name(zone: &str) -> String {
    format!("{zone} Ideal Loads Air System")
}
pub fn equipment_list_name(zone: &str) -> String {
    format!("{zone} Equipment List")
}
pub fn inlet_node_list_name(zone: &str) -> String {
    format!("{zone} Inlet Node List")
}
pub fn supply_node_name(zone: &str) -> String {
    format!("{zone} Supply Inlet")
}
pub fn zone_air_node_name(zone: &str) -> String {
    format!("{zone} Zone Air Node")
}
pub fn return_node_name(zone: &str) -> String {
    format!("{zone} Return Outlet")
}
pub fn dual_setpoint_name(zone: &str) -> String {
    format!("{zone} Setpoints")
}
pub fn thermostat_name(zone: &str) -> String {
    format!("{zone} Thermostat")
}
//#endregion 🔖️Names

//#region 🔖️Geometry
type Vec3 = [f64; 3];

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn norm(a: Vec3) -> f64 {
    dot(a, a).sqrt()
}

fn normalized(a: Vec3) -> Option<Vec3> {
    let length = norm(a);
    if length <= 1e-12 {
        None
    } else {
        Some([a[0] / length, a[1] / length, a[2] / length])
    }
}

/// 📐️ Newell outward normal of a planar polygon wound counter-clockwise seen from outside.
pub fn surface_normal(vertices: &[Vec3]) -> Option<Vec3> {
    if vertices.len() < 3 {
        return None;
    }
    let mut accumulated = [0.0f64; 3];
    for index in 0..vertices.len() {
        let current = vertices[index];
        let next = vertices[(index + 1) % vertices.len()];
        accumulated[0] += (current[1] - next[1]) * (current[2] + next[2]);
        accumulated[1] += (current[2] - next[2]) * (current[0] + next[0]);
        accumulated[2] += (current[0] - next[0]) * (current[1] + next[1]);
    }
    normalized(accumulated)
}

/// 📐️ Right-handed in-plane basis `(u, v)` with `u × v = n`; `u` is horizontal wherever the
/// surface is not itself horizontal, so `v` is the surface's own "up" and a window sill measures
/// along it.
fn surface_basis(normal: Vec3) -> (Vec3, Vec3) {
    let up = [0.0, 0.0, 1.0];
    let horizontal = normalized(cross(up, normal)).or_else(|| normalized(cross([0.0, 1.0, 0.0], normal))).unwrap_or([1.0, 0.0, 0.0]);
    (horizontal, cross(normal, horizontal))
}

/// 🪟️ Declared convention #2 — the rectangle of window `index` of `count` on `host`.
pub fn aperture_rectangle(host: &Surface, window: &Fenestration, index: usize, count: usize) -> Option<[Vec3; 4]> {
    let normal = surface_normal(&host.vertices_m)?;
    let (u, v) = surface_basis(normal);
    let origin = *host.vertices_m.first()?;
    let projected: Vec<(f64, f64)> = host.vertices_m.iter().map(|vertex| (dot(sub(*vertex, origin), u), dot(sub(*vertex, origin), v))).collect();
    let (mut u_min, mut u_max, mut v_min) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY);
    for (uc, vc) in &projected {
        u_min = u_min.min(*uc);
        u_max = u_max.max(*uc);
        v_min = v_min.min(*vc);
    }
    if !u_min.is_finite() || !u_max.is_finite() || !v_min.is_finite() || window.area_m2 <= 0.0 {
        return None;
    }
    let height = if window.height_m > 0.0 { window.height_m } else { (window.area_m2 / APERTURE_ASPECT).sqrt() };
    let width = window.area_m2 / height;
    let sill = if window.sill_height_m > 0.0 { window.sill_height_m } else { APERTURE_SILL_M };
    let bay = (u_max - u_min) / count.max(1) as f64;
    let centre = u_min + bay * (index as f64 + 0.5);
    let (u0, u1) = (centre - width / 2.0, centre + width / 2.0);
    let (v0, v1) = (v_min + sill, v_min + sill + height);
    let at = |a: f64, b: f64| [origin[0] + u[0] * a + v[0] * b, origin[1] + u[1] * a + v[1] * b, origin[2] + u[2] * a + v[2] * b];
    Some([at(u0, v0), at(u1, v0), at(u1, v1), at(u0, v1)])
}
//#endregion 🔖️Geometry

//#region 🔖️Helpers
fn entry(name: impl Into<String>, fields: impl IntoIterator<Item = (String, Value)>) -> (String, Value) {
    (name.into(), Value::Object(Object::from_iter(fields)))
}

fn field(key: &str, value: impl Into<Value>) -> (String, Value) {
    (key.to_string(), value.into())
}

fn yes_no(flag: bool) -> Value {
    Value::from(if flag { "Yes" } else { "No" })
}

fn vertex_value(vertex: Vec3) -> Value {
    Value::Object(Object::from_iter([field("vertex_x_coordinate", vertex[0]), field("vertex_y_coordinate", vertex[1]), field("vertex_z_coordinate", vertex[2])]))
}

/// 🧱️ EnergyPlus rejects a massive layer with zero density or a specific heat under 100 J/kg·K —
/// exactly how ANSI/ASHRAE 140 tabulates its floor insulation — so such a layer becomes a
/// `Material:NoMass` with `R = thickness / conductivity`, the same substitution NREL's own
/// BESTEST encoding makes.
pub fn is_massless(material: &Material) -> bool {
    material.density_kg_m3 <= 0.0 || material.specific_heat_j_kg_k < 100.0
}

fn surface_type(class: SurfaceClass) -> &'static str {
    match class {
        SurfaceClass::Roof => "Roof",
        SurfaceClass::Ceiling => "Ceiling",
        SurfaceClass::Floor | SurfaceClass::Ground => "Floor",
        _ => "Wall",
    }
}

fn boundary(condition: &OutsideBoundary) -> &'static str {
    match condition {
        OutsideBoundary::OutdoorAir => "Outdoors",
        OutsideBoundary::Ground => "Ground",
        OutsideBoundary::OtherSideTemperature => "OtherSideCoefficients",
        OutsideBoundary::Adiabatic | OutsideBoundary::Interzone(_) => "Adiabatic",
    }
}
//#endregion 🔖️Helpers

//#region 🔖️Encoder
/// 📤️ Encodes one [`Model`] as an EnergyPlus 25.2 epJSON document, together with everything the
/// covered object subset could not carry.
pub fn encode_model_with_diagnostics(model: &Model) -> (Value, Vec<EpJsonDiagnostic>) {
    let mut diagnostics = Vec::new();
    let mut document = Object::new();
    let zone_name_of = |id: EntityId| model.zones.iter().find(|zone| zone.id == id).map(|zone| entity_name("Zone", zone.id, &zone.name));
    let material_name_of = |id: EntityId| model.materials.iter().find(|material| material.id == id).map(|material| entity_name("Material", material.id, &material.name));
    let construction_name_of = |id: EntityId| model.constructions.iter().find(|construction| construction.id == id).map(|construction| entity_name("Construction", construction.id, &construction.name));

    document.insert("Version", Value::Object(Object::from_iter([entry("Version 1", [field("version_identifier", EPJSON_VERSION)])])));

    let zone_sizing = !model.sizing_objects.is_empty();
    document.insert(
        "SimulationControl",
        Value::Object(Object::from_iter([entry(
            "SimulationControl 1",
            [field("do_zone_sizing_calculation", yes_no(zone_sizing)), field("do_system_sizing_calculation", yes_no(false)), field("do_plant_sizing_calculation", yes_no(false)), field("run_simulation_for_sizing_periods", yes_no(false)), field("run_simulation_for_weather_file_run_periods", yes_no(true))],
        )])),
    );

    let building = entity_name("Building", EntityId(0), &model.name);
    document.insert("Building", Value::Object(Object::from_iter([entry(building.clone(), [field("north_axis", model.site.north_axis_deg), field("terrain", EPJSON_TERRAIN), field("solar_distribution", EPJSON_SOLAR_DISTRIBUTION)])])));
    document.insert(
        "Site:Location",
        Value::Object(Object::from_iter([entry(format!("{building} Site"), [field("latitude", model.site.latitude_deg), field("longitude", model.site.longitude_deg), field("time_zone", model.site.time_zone_hours), field("elevation", model.site.elevation_m)])])),
    );

    if model.surfaces.iter().any(|surface| matches!(surface.outside_boundary_condition, OutsideBoundary::Ground)) {
        const MONTHS: [&str; 12] = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];
        let temperatures = MONTHS.iter().enumerate().map(|(index, month)| field(&format!("{month}_ground_temperature"), model.ground_temperature.building_surface_c[index])).collect::<Vec<_>>();
        document.insert("Site:GroundTemperature:BuildingSurface", Value::Object(Object::from_iter([entry("Site:GroundTemperature:BuildingSurface 1", temperatures)])));
    }

    document.insert("GlobalGeometryRules", Value::Object(Object::from_iter([entry("GlobalGeometryRules 1", [field("starting_vertex_position", "UpperLeftCorner"), field("vertex_entry_direction", "Counterclockwise"), field("coordinate_system", "World")])])));
    document.insert("Timestep", Value::Object(Object::from_iter([entry("Timestep 1", [field("number_of_timesteps_per_hour", TIMESTEPS_PER_HOUR)])])));

    let period = &model.run_period;
    document.insert(
        "RunPeriod",
        Value::Object(Object::from_iter([entry(
            "Run Period 1",
            [
                field("begin_month", period.start_month as i64),
                field("begin_day_of_month", period.start_day as i64),
                field("begin_year", period.year as i64),
                field("end_month", period.end_month as i64),
                field("end_day_of_month", period.end_day as i64),
                field("end_year", period.year as i64),
                field("use_weather_file_holidays_and_special_days", yes_no(false)),
                field("use_weather_file_daylight_saving_period", yes_no(false)),
                field("apply_weekend_holiday_rule", yes_no(false)),
                field("use_weather_file_rain_indicators", yes_no(true)),
                field("use_weather_file_snow_indicators", yes_no(true)),
            ],
        )])),
    );

    encode_schedules(model, &mut document, &mut diagnostics);
    encode_materials(model, &mut document);
    encode_constructions(model, &mut document, &material_name_of, &mut diagnostics);
    encode_zones(model, &mut document);
    encode_surfaces(model, &mut document, &zone_name_of, &construction_name_of, &mut diagnostics);
    encode_gains(model, &mut document, &zone_name_of, &mut diagnostics);
    encode_hvac(model, &mut document, &zone_name_of, &mut diagnostics);
    encode_outputs(model, &mut document);
    (Value::Object(document), diagnostics)
}

/// 📤️ [`encode_model_with_diagnostics`] without the diagnostics channel.
pub fn encode_model(model: &Model) -> Value {
    encode_model_with_diagnostics(model).0
}

fn encode_schedules(model: &Model, document: &mut Object, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let schedules = &model.schedules;
    let mut limits = vec![entry(SCHEDULE_LIMITS_ANY, [field("numeric_type", "Continuous"), field("unit_type", "Dimensionless")])];
    let thermostat = !model.thermostats.is_empty();
    if thermostat {
        limits.push(entry(SCHEDULE_LIMITS_CONTROL, [field("lower_limit_value", 0.0), field("upper_limit_value", 4.0), field("numeric_type", "Discrete"), field("unit_type", "Control")]));
    }
    document.insert("ScheduleTypeLimits", Value::Object(Object::from_iter(limits)));

    let mut constants: Vec<(String, Value)> = schedules.constants.iter().map(|constant| entry(schedule_name(constant.id), [field("schedule_type_limits_name", SCHEDULE_LIMITS_ANY), field("hourly_value", constant.value)])).collect();
    if thermostat {
        constants.push(entry(DUAL_SETPOINT_CONTROL_SCHEDULE, [field("schedule_type_limits_name", SCHEDULE_LIMITS_CONTROL), field("hourly_value", 4.0)]));
    }
    if !constants.is_empty() {
        document.insert("Schedule:Constant", Value::Object(Object::from_iter(constants)));
    }

    let compact: Vec<(String, Value)> = schedules
        .daily
        .iter()
        .map(|daily| {
            let mut data = vec![Value::Object(Object::from_iter([field("field", "Through: 12/31")])), Value::Object(Object::from_iter([field("field", "For: AllDays")]))];
            for (hour, value) in daily.hourly_values.iter().enumerate() {
                let clamped = daily.limits.map(|limit| value.clamp(limit.min, limit.max)).unwrap_or(*value);
                data.push(Value::Object(Object::from_iter([field("field", format!("Until: {:02}:00", hour + 1))])));
                data.push(Value::Object(Object::from_iter([field("field", pack::json::format_f64(clamped))])));
            }
            entry(schedule_name(daily.id), [field("schedule_type_limits_name", SCHEDULE_LIMITS_ANY), ("data".to_string(), Value::Array(data))])
        })
        .collect();
    if !compact.is_empty() {
        document.insert("Schedule:Compact", Value::Object(Object::from_iter(compact)));
    }

    for weekly in &schedules.weekly {
        diagnostics.push(EpJsonDiagnostic::new("epjson.schedule.weekly-unsupported", schedule_name(weekly.id), "a weekly schedule is not written to epJSON: ScheduleSet::weekly_value indexes its 7 daily ids with a 1-based day of week clamped to 6, so Sunday collapses onto Saturday and the mapping onto Schedule:Compact day types is not yet decidable"));
    }
    for annual in &schedules.annual {
        diagnostics.push(EpJsonDiagnostic::new("epjson.schedule.annual-unsupported", schedule_name(annual.id), "an annual rule schedule is not written to epJSON: its holiday-date overrides have no Schedule:Compact equivalent without a RunPeriodControl:SpecialDays projection"));
    }
    for series in &schedules.time_series {
        diagnostics.push(EpJsonDiagnostic::new("epjson.schedule.time-series-unsupported", schedule_name(series.id), "a time-series schedule is not written to epJSON: Schedule:File needs an out-of-band CSV this synchronous codec cannot publish"));
    }
}

fn encode_materials(model: &Model, document: &mut Object) {
    let mut massive: Vec<(String, Value)> = Vec::new();
    let mut massless: Vec<(String, Value)> = Vec::new();
    for material in &model.materials {
        let name = entity_name("Material", material.id, &material.name);
        let optical = [field("thermal_absorptance", material.thermal_absorptance), field("solar_absorptance", material.solar_absorptance), field("visible_absorptance", material.visible_absorptance)];
        if is_massless(material) {
            let resistance = if material.conductivity_w_m_k > 0.0 { material.thickness_m / material.conductivity_w_m_k } else { 0.001 };
            massless.push(entry(name, [field("roughness", "Rough"), field("thermal_resistance", resistance.max(0.001))].into_iter().chain(optical)));
        } else {
            massive.push(entry(
                name,
                [field("roughness", "Rough"), field("thickness", material.thickness_m), field("conductivity", material.conductivity_w_m_k), field("density", material.density_kg_m3), field("specific_heat", material.specific_heat_j_kg_k)].into_iter().chain(optical),
            ));
        }
    }
    if !massive.is_empty() {
        document.insert("Material", Value::Object(Object::from_iter(massive)));
    }
    if !massless.is_empty() {
        document.insert("Material:NoMass", Value::Object(Object::from_iter(massless)));
    }
}

fn encode_constructions(model: &Model, document: &mut Object, material_name_of: &dyn Fn(EntityId) -> Option<String>, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    const LAYER_KEYS: [&str; 10] = ["outside_layer", "layer_2", "layer_3", "layer_4", "layer_5", "layer_6", "layer_7", "layer_8", "layer_9", "layer_10"];
    let mut glazing: Vec<(String, Value)> = Vec::new();
    let mut constructions: Vec<(String, Value)> = Vec::new();
    for construction in &model.constructions {
        let name = entity_name("Construction", construction.id, &construction.name);
        if construction.layer_material_ids.len() > LAYER_KEYS.len() {
            diagnostics.push(EpJsonDiagnostic::new("epjson.construction.too-many-layers", name.clone(), format!("EnergyPlus Construction carries at most {} layers, this one has {}; the surplus is dropped", LAYER_KEYS.len(), construction.layer_material_ids.len())));
        }
        let layers: Vec<(String, Value)> = construction
            .layer_material_ids
            .iter()
            .take(LAYER_KEYS.len())
            .enumerate()
            .filter_map(|(index, id)| match material_name_of(*id) {
                Some(material) => Some(field(LAYER_KEYS[index], material)),
                None => {
                    diagnostics.push(EpJsonDiagnostic::new("epjson.construction.unknown-layer", name.clone(), format!("layer {index} references material {} which the model does not define", id.0)));
                    None
                }
            })
            .collect();
        constructions.push(entry(name, layers));
    }
    for window in &model.fenestrations {
        let name = entity_name("Fenestration", window.id, &window.name);
        let material = glazing_material_name(&name);
        glazing.push(entry(material.clone(), [field("u_factor", window.u_value_w_m2k), field("solar_heat_gain_coefficient", window.shgc), field("visible_transmittance", window.vlt)]));
        constructions.push(entry(glazing_construction_name(&name), [field("outside_layer", material)]));
        if window.frame_conductance_w_k > 0.0 || window.divider_conductance_w_k > 0.0 {
            diagnostics.push(EpJsonDiagnostic::new("epjson.fenestration.frame-dropped", name, "frame/divider conductance has no WindowMaterial:SimpleGlazingSystem equivalent and is not written; use WindowProperty:FrameAndDivider once the schema carries frame geometry"));
        }
    }
    if !glazing.is_empty() {
        document.insert("WindowMaterial:SimpleGlazingSystem", Value::Object(Object::from_iter(glazing)));
    }
    if !constructions.is_empty() {
        document.insert("Construction", Value::Object(Object::from_iter(constructions)));
    }
}

fn encode_zones(model: &Model, document: &mut Object) {
    let zones: Vec<(String, Value)> = model
        .zones
        .iter()
        .map(|zone| {
            entry(
                entity_name("Zone", zone.id, &zone.name),
                [field("direction_of_relative_north", 0.0), field("x_origin", 0.0), field("y_origin", 0.0), field("z_origin", 0.0), field("multiplier", zone.multiplier.max(1) as i64), field("volume", zone.volume_m3), field("part_of_total_floor_area", yes_no(zone.part_of_total_floor_area))],
            )
        })
        .collect();
    if !zones.is_empty() {
        document.insert("Zone", Value::Object(Object::from_iter(zones)));
    }
}

fn encode_surfaces(model: &Model, document: &mut Object, zone_name_of: &dyn Fn(EntityId) -> Option<String>, construction_name_of: &dyn Fn(EntityId) -> Option<String>, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let mut surfaces: Vec<(String, Value)> = Vec::new();
    for surface in &model.surfaces {
        let name = entity_name("Surface", surface.id, &surface.name);
        let (Some(zone), Some(construction)) = (zone_name_of(surface.zone_id), construction_name_of(surface.construction_id)) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.surface.dangling-reference", name, "the surface references a zone or construction the model does not define and is not written"));
            continue;
        };
        if surface.multiplier > 1 {
            diagnostics.push(EpJsonDiagnostic::new("epjson.surface.multiplier-dropped", name.clone(), "BuildingSurface:Detailed has no multiplier field; repeat the surface geometry instead"));
        }
        let mut fields = vec![field("surface_type", surface_type(surface.class)), field("construction_name", construction), field("zone_name", zone), field("outside_boundary_condition", boundary(&surface.outside_boundary_condition)), field("sun_exposure", if surface.sun_exposed { "SunExposed" } else { "NoSun" }), field("wind_exposure", if surface.wind_exposed { "WindExposed" } else { "NoWind" })];
        if let OutsideBoundary::Interzone(other) = surface.outside_boundary_condition {
            match model.surfaces.iter().find(|candidate| candidate.id == other) {
                Some(peer) => {
                    fields[3] = field("outside_boundary_condition", "Surface");
                    fields.push(field("outside_boundary_condition_object", entity_name("Surface", peer.id, &peer.name)));
                }
                None => diagnostics.push(EpJsonDiagnostic::new("epjson.surface.dangling-interzone", name.clone(), format!("the interzone partner surface {} is not defined; the surface is written as Adiabatic", other.0))),
            }
        }
        fields.push(("vertices".to_string(), Value::Array(surface.vertices_m.iter().map(|vertex| vertex_value(*vertex)).collect())));
        surfaces.push(entry(name, fields));
    }
    if !surfaces.is_empty() {
        document.insert("BuildingSurface:Detailed", Value::Object(Object::from_iter(surfaces)));
    }

    let mut apertures: Vec<(String, Value)> = Vec::new();
    let mut overhangs: Vec<(String, Value)> = Vec::new();
    let mut fins: Vec<(String, Value)> = Vec::new();
    for surface in &model.surfaces {
        let hosted: Vec<&Fenestration> = model.fenestrations.iter().filter(|window| window.surface_id == surface.id).collect();
        for (index, window) in hosted.iter().enumerate() {
            let name = entity_name("Fenestration", window.id, &window.name);
            let Some(rectangle) = aperture_rectangle(surface, window, index, hosted.len()) else {
                diagnostics.push(EpJsonDiagnostic::new("epjson.fenestration.undetermined-geometry", name, "the host surface is degenerate or the aperture area is non-positive, so no rectangle could be placed"));
                continue;
            };
            let mut fields = vec![field("surface_type", "Window"), field("construction_name", glazing_construction_name(&name)), field("building_surface_name", entity_name("Surface", surface.id, &surface.name))];
            for (corner, vertex) in rectangle.iter().enumerate() {
                fields.push(field(&format!("vertex_{}_x_coordinate", corner + 1), vertex[0]));
                fields.push(field(&format!("vertex_{}_y_coordinate", corner + 1), vertex[1]));
                fields.push(field(&format!("vertex_{}_z_coordinate", corner + 1), vertex[2]));
            }
            apertures.push(entry(name.clone(), fields));
            if window.overhang_depth_m > 0.0 {
                overhangs.push(entry(format!("{name} Overhang"), [field("window_or_door_name", name.clone()), field("height_above_window_or_door", window.overhang_offset_m), field("tilt_angle_from_window_door", 90.0), field("left_extension_from_window_door_width", 0.0), field("right_extension_from_window_door_width", 0.0), field("depth_as_fraction_of_window_door_height", window.overhang_depth_m / if window.height_m > 0.0 { window.height_m } else { (window.area_m2 / APERTURE_ASPECT).sqrt() })]));
            }
            if window.fin_depth_m > 0.0 {
                let height = if window.height_m > 0.0 { window.height_m } else { (window.area_m2 / APERTURE_ASPECT).sqrt() };
                fins.push(entry(
                    format!("{name} Fins"),
                    [
                        field("window_or_door_name", name.clone()),
                        field("left_extension_from_window_door", window.fin_offset_m),
                        field("left_distance_above_top_of_window", 0.0),
                        field("left_distance_below_bottom_of_window", 0.0),
                        field("left_tilt_angle_from_window_door", 90.0),
                        field("left_depth_as_fraction_of_window_door_width", window.fin_depth_m / height),
                        field("right_extension_from_window_door", window.fin_offset_m),
                        field("right_distance_above_top_of_window", 0.0),
                        field("right_distance_below_bottom_of_window", 0.0),
                        field("right_tilt_angle_from_window_door", 90.0),
                        field("right_depth_as_fraction_of_window_door_width", window.fin_depth_m / height),
                    ],
                ));
            }
        }
    }
    for window in &model.fenestrations {
        if !model.surfaces.iter().any(|surface| surface.id == window.surface_id) {
            diagnostics.push(EpJsonDiagnostic::new("epjson.fenestration.dangling-host", entity_name("Fenestration", window.id, &window.name), format!("host surface {} is not defined by the model", window.surface_id.0)));
        }
    }
    if !apertures.is_empty() {
        document.insert("FenestrationSurface:Detailed", Value::Object(Object::from_iter(apertures)));
    }
    if !overhangs.is_empty() {
        document.insert("Shading:Overhang:Projection", Value::Object(Object::from_iter(overhangs)));
    }
    if !fins.is_empty() {
        document.insert("Shading:Fin:Projection", Value::Object(Object::from_iter(fins)));
    }
}

fn encode_gains(model: &Model, document: &mut Object, zone_name_of: &dyn Fn(EntityId) -> Option<String>, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let mut infiltrations: Vec<(String, Value)> = Vec::new();
    for infiltration in &model.infiltrations {
        let name = entity_name("Infiltration", infiltration.id, "");
        let Some(zone) = zone_name_of(infiltration.zone_id) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.infiltration.dangling-zone", name, "the infiltration object references an undefined zone"));
            continue;
        };
        let (method, magnitude) = match infiltration.method {
            InfiltrationMethod::ScheduledAch => ("AirChanges/Hour", field("air_changes_per_hour", infiltration.design_flow_ach)),
            InfiltrationMethod::PerExteriorArea => ("Flow/ExteriorArea", field("flow_rate_per_exterior_surface_area", infiltration.flow_per_exterior_area_m3_s_m2)),
            InfiltrationMethod::EffectiveLeakageArea | InfiltrationMethod::WindAndStack => {
                diagnostics.push(EpJsonDiagnostic::new("epjson.infiltration.method-unsupported", name, "EffectiveLeakageArea and WindAndStack map onto ZoneInfiltration:EffectiveLeakageArea / :FlowCoefficient, which this codec does not write; the object is refused rather than approximated as a design flow rate"));
                continue;
            }
        };
        infiltrations.push(entry(
            format!("{zone} Infiltration {}", infiltration.id.0),
            [
                field("zone_or_zonelist_or_space_or_spacelist_name", zone),
                field("schedule_name", schedule_name(infiltration.schedule_id)),
                field("design_flow_rate_calculation_method", method),
                magnitude,
                field("constant_term_coefficient", infiltration.constant_term_coefficient),
                field("temperature_term_coefficient", infiltration.temperature_term_coefficient),
                field("velocity_term_coefficient", infiltration.velocity_term_coefficient),
                field("velocity_squared_term_coefficient", infiltration.velocity_squared_term_coefficient),
            ],
        ));
    }
    if !infiltrations.is_empty() {
        document.insert("ZoneInfiltration:DesignFlowRate", Value::Object(Object::from_iter(infiltrations)));
    }

    let mut people: Vec<(String, Value)> = Vec::new();
    for occupancy in &model.people {
        let Some(zone) = zone_name_of(occupancy.zone_id) else { continue };
        people.push(entry(
            format!("{zone} People {}", occupancy.id.0),
            [
                field("zone_or_zonelist_or_space_or_spacelist_name", zone),
                field("number_of_people_schedule_name", schedule_name(occupancy.schedule_id)),
                field("number_of_people_calculation_method", "People/Area"),
                field("people_per_floor_area", occupancy.people_per_area),
                field("fraction_radiant", occupancy.radiant_fraction),
                field("sensible_heat_fraction", occupancy.sensible_fraction),
                field("activity_level_schedule_name", schedule_name(occupancy.activity_schedule_id)),
            ],
        ));
    }
    if !people.is_empty() {
        document.insert("People", Value::Object(Object::from_iter(people)));
    }

    let mut lights: Vec<(String, Value)> = Vec::new();
    for lighting in &model.lighting {
        let Some(zone) = zone_name_of(lighting.zone_id) else { continue };
        lights.push(entry(
            format!("{zone} Lights {}", lighting.id.0),
            [
                field("zone_or_zonelist_or_space_or_spacelist_name", zone),
                field("schedule_name", schedule_name(lighting.schedule_id)),
                field("design_level_calculation_method", "Watts/Area"),
                field("watts_per_floor_area", lighting.watts_per_area),
                field("return_air_fraction", lighting.return_air_fraction),
                field("fraction_radiant", lighting.radiant_fraction),
                field("fraction_visible", lighting.visible_fraction),
            ],
        ));
    }
    if !lights.is_empty() {
        document.insert("Lights", Value::Object(Object::from_iter(lights)));
    }

    let mut equipment: Vec<(String, Value)> = Vec::new();
    for gain in &model.equipment {
        let Some(zone) = zone_name_of(gain.zone_id) else { continue };
        equipment.push(entry(
            format!("{zone} Equipment {}", gain.id.0),
            [
                field("zone_or_zonelist_or_space_or_spacelist_name", zone),
                field("schedule_name", schedule_name(gain.schedule_id)),
                field("design_level_calculation_method", "Watts/Area"),
                field("watts_per_floor_area", gain.watts_per_area),
                field("fraction_latent", gain.latent_fraction),
                field("fraction_radiant", gain.radiant_fraction),
                field("fraction_lost", 0.0),
            ],
        ));
    }
    if !equipment.is_empty() {
        document.insert("ElectricEquipment", Value::Object(Object::from_iter(equipment)));
    }
}

fn encode_hvac(model: &Model, document: &mut Object, zone_name_of: &dyn Fn(EntityId) -> Option<String>, diagnostics: &mut Vec<EpJsonDiagnostic>) {
    let mut setpoints: Vec<(String, Value)> = Vec::new();
    let mut controls: Vec<(String, Value)> = Vec::new();
    for thermostat in &model.thermostats {
        let Some(zone) = zone_name_of(thermostat.zone_id) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.thermostat.dangling-zone", entity_name("Thermostat", thermostat.id, ""), "the thermostat references an undefined zone"));
            continue;
        };
        if thermostat.heating_throttle_range_k != 0.0 || thermostat.cooling_throttle_range_k != 0.0 {
            diagnostics.push(EpJsonDiagnostic::new("epjson.thermostat.throttle-range-dropped", zone.clone(), "ThermostatSetpoint:DualSetpoint has no throttling range; EnergyPlus applies its own deadband, so the model's throttle ranges are not written"));
        }
        setpoints.push(entry(dual_setpoint_name(&zone), [field("heating_setpoint_temperature_schedule_name", schedule_name(thermostat.heating_setpoint_schedule_id)), field("cooling_setpoint_temperature_schedule_name", schedule_name(thermostat.cooling_setpoint_schedule_id))]));
        controls.push(entry(thermostat_name(&zone), [field("zone_or_zonelist_name", zone.clone()), field("control_type_schedule_name", DUAL_SETPOINT_CONTROL_SCHEDULE), field("control_1_object_type", "ThermostatSetpoint:DualSetpoint"), field("control_1_name", dual_setpoint_name(&zone))]));
    }
    if !setpoints.is_empty() {
        document.insert("ThermostatSetpoint:DualSetpoint", Value::Object(Object::from_iter(setpoints)));
        document.insert("ZoneControl:Thermostat", Value::Object(Object::from_iter(controls)));
    }

    let mut systems: Vec<(String, Value)> = Vec::new();
    let mut lists: Vec<(String, Value)> = Vec::new();
    let mut connections: Vec<(String, Value)> = Vec::new();
    let mut node_lists: Vec<(String, Value)> = Vec::new();
    for ideal in &model.ideal_loads {
        let Some(zone) = zone_name_of(ideal.zone_id) else {
            diagnostics.push(EpJsonDiagnostic::new("epjson.ideal-loads.dangling-zone", entity_name("IdealLoads", ideal.id, ""), "the ideal loads system references an undefined zone"));
            continue;
        };
        if ideal.outdoor_air_per_person_m3_s > 0.0 || ideal.outdoor_air_per_area_m3_s_m2 > 0.0 {
            diagnostics.push(EpJsonDiagnostic::new("epjson.ideal-loads.outdoor-air-dropped", zone.clone(), "outdoor-air rates need a DesignSpecification:OutdoorAir object, which this codec does not write; the system is emitted without mechanical outdoor air"));
        }
        let mut fields = vec![field("zone_supply_air_node_name", supply_node_name(&zone)), field("maximum_heating_supply_air_temperature", ideal.max_heating_supply_air_temp_c), field("minimum_cooling_supply_air_temperature", ideal.min_cooling_supply_air_temp_c)];
        match ideal.max_heating_capacity_w {
            Some(capacity) => {
                fields.push(field("heating_limit", "LimitCapacity"));
                fields.push(field("maximum_sensible_heating_capacity", capacity));
            }
            None => fields.push(field("heating_limit", "NoLimit")),
        }
        match ideal.max_cooling_capacity_w {
            Some(capacity) => {
                fields.push(field("cooling_limit", "LimitCapacity"));
                fields.push(field("maximum_total_cooling_capacity", capacity));
            }
            None => fields.push(field("cooling_limit", "NoLimit")),
        }
        fields.push(field("dehumidification_control_type", "None"));
        systems.push(entry(ideal_loads_name(&zone), fields));
        lists.push(entry(
            equipment_list_name(&zone),
            [
                field("load_distribution_scheme", "SequentialLoad"),
                (
                    "equipment".to_string(),
                    Value::Array(vec![Value::Object(Object::from_iter([field("zone_equipment_object_type", "ZoneHVAC:IdealLoadsAirSystem"), field("zone_equipment_name", ideal_loads_name(&zone)), field("zone_equipment_cooling_sequence", 1i64), field("zone_equipment_heating_or_no_load_sequence", 1i64)]))]),
                ),
            ],
        ));
        node_lists.push(entry(inlet_node_list_name(&zone), [("nodes".to_string(), Value::Array(vec![Value::Object(Object::from_iter([field("node_name", supply_node_name(&zone))]))]))]));
        connections.push(entry(
            zone.clone(),
            [field("zone_name", zone.clone()), field("zone_conditioning_equipment_list_name", equipment_list_name(&zone)), field("zone_air_inlet_node_or_nodelist_name", inlet_node_list_name(&zone)), field("zone_air_node_name", zone_air_node_name(&zone)), field("zone_return_air_node_or_nodelist_name", return_node_name(&zone))],
        ));
    }
    if !systems.is_empty() {
        document.insert("ZoneHVAC:IdealLoadsAirSystem", Value::Object(Object::from_iter(systems)));
        document.insert("ZoneHVAC:EquipmentList", Value::Object(Object::from_iter(lists)));
        document.insert("ZoneHVAC:EquipmentConnections", Value::Object(Object::from_iter(connections)));
        document.insert("NodeList", Value::Object(Object::from_iter(node_lists)));
    }

    let sizing: Vec<(String, Value)> = model
        .sizing_objects
        .iter()
        .filter_map(|object| {
            let zone = zone_name_of(object.zone_id)?;
            Some(entry(
                format!("{zone} Sizing {}", object.id.0),
                [field("zone_or_zonelist_name", zone), field("zone_cooling_design_supply_air_temperature", 13.0), field("zone_heating_design_supply_air_temperature", 50.0), field("zone_cooling_design_supply_air_humidity_ratio", 0.0085), field("zone_heating_design_supply_air_humidity_ratio", 0.008)],
            ))
        })
        .collect();
    if !sizing.is_empty() {
        document.insert("Sizing:Zone", Value::Object(Object::from_iter(sizing)));
    }
}

fn encode_outputs(model: &Model, document: &mut Object) {
    let mut variables: Vec<(String, Value)> = Vec::new();
    for (index, name) in CONTRACT_OUTPUT_VARIABLES.iter().enumerate() {
        variables.push(entry(format!("Output:Variable {}", index + 1), [field("key_value", "*"), field("variable_name", *name), field("reporting_frequency", "Hourly")]));
    }
    for (index, requested) in model.output_variables.iter().enumerate() {
        if CONTRACT_OUTPUT_VARIABLES.contains(&requested.name.as_str()) {
            continue;
        }
        let frequency = match requested.reporting_frequency {
            crate::model::OutputReportFrequency::Timestep => "Detailed",
            crate::model::OutputReportFrequency::Hourly => "Hourly",
            crate::model::OutputReportFrequency::Daily => "Daily",
            crate::model::OutputReportFrequency::Monthly => "Monthly",
            crate::model::OutputReportFrequency::RunPeriod => "RunPeriod",
        };
        let key = if requested.key.trim().is_empty() { "*".to_string() } else { requested.key.clone() };
        variables.push(entry(format!("Output:Variable {}", CONTRACT_OUTPUT_VARIABLES.len() + index + 1), [field("key_value", key), field("variable_name", requested.name.as_str()), field("reporting_frequency", frequency)]));
    }
    document.insert("Output:Variable", Value::Object(Object::from_iter(variables)));
    document.insert("Output:Meter", Value::Object(Object::from_iter(CONTRACT_OUTPUT_METERS.iter().enumerate().map(|(index, meter)| entry(format!("Output:Meter {}", index + 1), [field("key_name", *meter), field("reporting_frequency", "Hourly")])))));
    document.insert("Output:SQLite", Value::Object(Object::from_iter([entry("Output:SQLite 1", [field("option_type", "SimpleAndTabular")])])));
}
//#endregion 🔖️Encoder

//#region 🔖️Leaf
// 🚫️async: E1 pure codec registration hook, no io — see R9.
pub fn register() {}

/// 🚨️ The diagnostics that make a document unusable rather than merely lossy: a refusal is
/// something EnergyPlus would have had to be told and this subset cannot say, so the export stops.
/// A lossy-but-runnable note (a dropped surface multiplier, a dropped frame conductance) is
/// reported and the document is still written.
pub fn refusals(diagnostics: &[EpJsonDiagnostic]) -> Vec<&EpJsonDiagnostic> {
    diagnostics.iter().filter(|diagnostic| diagnostic.code.ends_with("-unsupported") || diagnostic.code.ends_with("dangling-zone")).collect()
}

/// 📤️ The io-leaf entry point: this artifact's snapshot as an epJSON document tree.
pub fn serialize(snapshot: &EnergyModelSnapshot) -> Result<Value, store::TextError> {
    let (document, diagnostics) = encode_model_with_diagnostics(&snapshot.model);
    let refused = refusals(&diagnostics);
    if !refused.is_empty() {
        let detail = refused.iter().map(|diagnostic| format!("{} ({})", diagnostic.message, diagnostic.subject)).collect::<Vec<_>>().join("; ");
        return Err(store::TextError::new(format!("model -> epJSON: {detail}"), dsl::TextSpan::at(1, 1)));
    }
    Ok(document)
}

/// 📤️ The io-leaf byte entry point: pretty-printed epJSON, the form EnergyPlus reads.
pub fn serialize_bytes(snapshot: &EnergyModelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(format!("{}\n", pack::json::to_string_pretty(&serialize(snapshot)?)).into_bytes())
}
//#endregion 🔖️Leaf

//#region 🌉️Bridge
/// 🌉️ Text in, text out. The out-of-crate test adapter for
/// `🧪️tests/🏛️export-epjson-runs-in-energyplus` cannot name [`Model`], [`EnergyModelSnapshot`] or
/// [`Value`], so the case's one committed input — `🧫️fixtures/🏛️bestest-<case>/🔋️model.json` — goes
/// in as its own canonical JSON and the epJSON comes back as the exact bytes EnergyPlus is handed.
/// Same shape and same reason as `bestest::model_json`.
pub fn epjson_from_model_json(model_json: &str) -> Result<String, String> {
    let model: Model = pack::json::from_json_str(model_json).map_err(|error| format!("the committed model does not decode: {error}"))?;
    let (document, diagnostics) = encode_model_with_diagnostics(&model);
    let refused = refusals(&diagnostics);
    if !refused.is_empty() {
        return Err(format!("model -> epJSON refused: {}", refused.iter().map(|diagnostic| format!("{} ({})", diagnostic.message, diagnostic.subject)).collect::<Vec<_>>().join("; ")));
    }
    Ok(format!("{}\n", pack::json::to_string_pretty(&document)))
}

/// 🌉️ The same export's full diagnostic list as a JSON array, so a scenario can assert that a
/// document carrying no refusals also dropped nothing quietly.
pub fn epjson_diagnostics_json(model_json: &str) -> Result<String, String> {
    let model: Model = pack::json::from_json_str(model_json).map_err(|error| format!("the committed model does not decode: {error}"))?;
    let (_, diagnostics) = encode_model_with_diagnostics(&model);
    Ok(pack::json::to_string(&Value::Array(diagnostics.iter().map(EpJsonDiagnostic::to_json).collect())))
}
//#endregion 🌉️Bridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn case(name: &str) -> Model {
        crate::bestest::model(name).expect("bestest case")
    }

    #[semio_framework_async_macros::async_test]
    async fn writes_every_contract_object_type_for_case_600() {
        let (document, diagnostics) = encode_model_with_diagnostics(&case("600"));
        let Value::Object(root) = &document else { panic!("epJSON root must be an object") };
        for required in ["Version", "SimulationControl", "Building", "Site:Location", "GlobalGeometryRules", "Timestep", "RunPeriod", "ScheduleTypeLimits", "Schedule:Constant", "Material", "Material:NoMass", "WindowMaterial:SimpleGlazingSystem", "Construction", "Zone", "BuildingSurface:Detailed", "FenestrationSurface:Detailed", "ZoneInfiltration:DesignFlowRate", "ElectricEquipment", "ThermostatSetpoint:DualSetpoint", "ZoneControl:Thermostat", "ZoneHVAC:IdealLoadsAirSystem", "ZoneHVAC:EquipmentList", "ZoneHVAC:EquipmentConnections", "NodeList", "Output:Variable", "Output:Meter", "Output:SQLite"] {
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
    /// `🔮️oracle🔋️energy⚡️epjson` launch entry have a real file to hand EnergyPlus without first
    /// running a Rust test, and so a reviewer can read the document the codec actually writes.
    /// Exactly the four the `🏛️export-epjson-runs-in-energyplus` case simulates.
    const COMMITTED_EPJSON_CASES: [&str; 4] = ["600", "600FF", "900", "900FF"];

    fn epjson_fixture_path(case: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures").join(format!("🏛️bestest-{case}")).join("⚡️model.epJSON")
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
}
//#endregion 🧪️Tests
