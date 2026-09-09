//! 🏛️ ANSI/ASHRAE Standard 140 §5.2 (BESTEST) building-thermal-envelope and fabric-load cases,
//! authored directly against this engine's own typed [`crate::model::Model`].
//!
//! Case 600 is the standard's base case and every other §5.2 case is defined as a delta from it,
//! so this module is shaped the same way: one shared catalogue of ASHRAE 140 materials,
//! constructions, geometry and schedules, then one `case_*` function per case that states only its
//! own delta. The numbers are the standard's own §5.2 tables, handcrafted here — never generated,
//! never scraped.
//!
//! Reference cross-checks: NREL/TP-550-43827 *EnergyPlus Testing with ANSI/ASHRAE Standard 140*
//! carries the reference envelopes; NREL's `BESTEST-GSR` repository carries the same case set as
//! OpenStudio measures. Neither is consumed here — this is an independent authoring of the spec.

use crate::air_exchange::InfiltrationMethod;
use crate::error::{Diagnostics, Error};
use crate::kernel::{SimulationConfig, SimulationEnvironment};
use crate::model::{Construction, EntityId, EquipmentGain, Fenestration, IdealLoadsSystem, Infiltration, Material, MechanicalVentilation, Model, OutsideBoundary, ScheduleId, Site, Surface, SurfaceClass, Thermostat, Zone};
use crate::results::Results;
use crate::schedule::{ConstantSchedule, DailySchedule, ScheduleInterpolation, ScheduleSet};
use crate::site::EpwWeather;

// #region 🔖️Registry
/// 🗂️ Every §5.2 case this engine can build, in the standard's own order.
pub const CASES: &[&str] = &["600", "600FF", "610", "620", "630", "640", "650", "900", "900FF", "910", "920", "930", "940", "950"];

/// 🏛️ Build one case's model, or `None` for an id outside [`CASES`].
pub fn model(case: &str) -> Option<Model> {
    let built = match case {
        "600" => case_600(),
        "600FF" => free_float(case_600(), "BESTEST 600FF"),
        "610" => case_610(),
        "620" => case_620(),
        "630" => case_630(),
        "640" => case_640(),
        "650" => case_650(),
        "900" => high_mass(case_600(), "BESTEST 900"),
        "900FF" => free_float(high_mass(case_600(), "BESTEST 900FF"), "BESTEST 900FF"),
        "910" => high_mass(case_610(), "BESTEST 910"),
        "920" => high_mass(case_620(), "BESTEST 920"),
        "930" => high_mass(case_630(), "BESTEST 930"),
        "940" => high_mass(case_640(), "BESTEST 940"),
        "950" => high_mass(case_650(), "BESTEST 950"),
        _ => return None,
    };
    Some(built)
}

/// 🌡️ True for the free-float cases, which carry no HVAC at all and are judged on zone temperature
/// rather than on delivered energy.
pub fn is_free_float(case: &str) -> bool {
    case.ends_with("FF")
}
// #endregion 🔖️Registry

// #region 🔖️Ids
const ZONE: EntityId = EntityId(1);

const WOOD_SIDING: EntityId = EntityId(10);
const FIBERGLASS_QUILT: EntityId = EntityId(11);
const PLASTERBOARD: EntityId = EntityId(12);
const ROOF_DECK: EntityId = EntityId(13);
const ROOF_QUILT: EntityId = EntityId(14);
const ROOF_PLASTERBOARD: EntityId = EntityId(15);
const TIMBER_FLOORING: EntityId = EntityId(16);
const FLOOR_INSULATION: EntityId = EntityId(17);
const CONCRETE_BLOCK: EntityId = EntityId(18);
const FOAM_INSULATION: EntityId = EntityId(19);
const CONCRETE_SLAB: EntityId = EntityId(20);
const HIGH_MASS_FLOOR_INSULATION: EntityId = EntityId(21);

const WALL_CONSTRUCTION: EntityId = EntityId(30);
const FLOOR_CONSTRUCTION: EntityId = EntityId(31);
const ROOF_CONSTRUCTION: EntityId = EntityId(32);

const SOUTH_WALL: EntityId = EntityId(40);
const EAST_WALL: EntityId = EntityId(41);
const NORTH_WALL: EntityId = EntityId(42);
const WEST_WALL: EntityId = EntityId(43);
const ROOF: EntityId = EntityId(44);
const FLOOR: EntityId = EntityId(45);

const WINDOW_A: EntityId = EntityId(50);
const WINDOW_B: EntityId = EntityId(51);

const INTERNAL_GAIN: EntityId = EntityId(60);
const INFILTRATION: EntityId = EntityId(61);
const THERMOSTAT: EntityId = EntityId(62);
const IDEAL_LOADS: EntityId = EntityId(63);
const NIGHT_VENTILATION: EntityId = EntityId(64);

const ALWAYS_ON: ScheduleId = ScheduleId(1);
const HEATING_SETPOINT: ScheduleId = ScheduleId(2);
const COOLING_SETPOINT: ScheduleId = ScheduleId(3);
const HEATING_SETBACK: ScheduleId = ScheduleId(4);
const HEATING_DISABLED: ScheduleId = ScheduleId(5);
const COOLING_DAYTIME_ONLY: ScheduleId = ScheduleId(6);
const NIGHT_VENTILATION_SCHEDULE: ScheduleId = ScheduleId(7);
// #endregion 🔖️Ids

// #region 🔖️Dimensions
const WIDTH_M: f64 = 8.0;
const DEPTH_M: f64 = 6.0;
const HEIGHT_M: f64 = 2.7;
const VOLUME_M3: f64 = WIDTH_M * DEPTH_M * HEIGHT_M;
const FLOOR_AREA_M2: f64 = WIDTH_M * DEPTH_M;

/// 💡️ 200 W continuous, 60 % radiative / 40 % convective, wholly sensible (§5.2.1.8).
const INTERNAL_GAIN_W: f64 = 200.0;
/// 💨️ 0.5 air changes per hour, constant (§5.2.1.6).
const INFILTRATION_ACH: f64 = 0.5;
/// 🌡️ Ground below the floor is held at a constant 10 °C (§5.2.1.5).
const GROUND_TEMPERATURE_C: f64 = 10.0;
/// 🌬️ Case 650/950 night ventilation: 1703.16 m³/h.
const NIGHT_VENTILATION_M3_S: f64 = 1703.16 / 3600.0;
/// 🪟️ Window sill 0.2 m above the floor, 2.0 m of glazing height (§5.2.1.4).
const WINDOW_SILL_M: f64 = 0.2;
const WINDOW_HEIGHT_M: f64 = 2.0;
const WINDOW_WIDTH_M: f64 = 3.0;
/// 🌳️ Cases 610/630/910/930 attach a 1.0 m projection.
const PROJECTION_DEPTH_M: f64 = 1.0;
/// 🌳️ The case 610 overhang sits at roof level, 0.5 m above the window head.
const OVERHANG_OFFSET_M: f64 = HEIGHT_M - (WINDOW_SILL_M + WINDOW_HEIGHT_M);
// #endregion 🔖️Dimensions

// #region 🔖️Catalogue
fn material(id: EntityId, name: &str, thickness_m: f64, conductivity_w_m_k: f64, density_kg_m3: f64, specific_heat_j_kg_k: f64) -> Material {
    Material { id, name: name.into(), thickness_m, conductivity_w_m_k, density_kg_m3, specific_heat_j_kg_k, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 }
}

/// 🧱️ Every §5.2 material, both the lightweight (600-series) and the high-mass (900-series) set.
fn materials() -> Vec<Material> {
    vec![
        material(WOOD_SIDING, "Wood Siding", 0.009, 0.140, 530.0, 900.0),
        material(FIBERGLASS_QUILT, "Fiberglass Quilt", 0.066, 0.040, 12.0, 840.0),
        material(PLASTERBOARD, "Plasterboard", 0.012, 0.160, 950.0, 840.0),
        material(ROOF_DECK, "Roof Deck", 0.019, 0.140, 530.0, 900.0),
        material(ROOF_QUILT, "Roof Fiberglass Quilt", 0.1118, 0.040, 12.0, 840.0),
        material(ROOF_PLASTERBOARD, "Roof Plasterboard", 0.010, 0.160, 950.0, 840.0),
        material(TIMBER_FLOORING, "Timber Flooring", 0.025, 0.140, 650.0, 1200.0),
        material(FLOOR_INSULATION, "Floor Insulation", 1.003, 0.040, 0.0, 0.0),
        material(CONCRETE_BLOCK, "Concrete Block", 0.100, 0.510, 1400.0, 1000.0),
        material(FOAM_INSULATION, "Foam Insulation", 0.0615, 0.040, 10.0, 1400.0),
        material(CONCRETE_SLAB, "Concrete Slab", 0.080, 1.130, 1400.0, 1000.0),
        material(HIGH_MASS_FLOOR_INSULATION, "High Mass Floor Insulation", 1.007, 0.040, 0.0, 0.0),
    ]
}

/// 🧱️ Lightweight (case 600) constructions. Layers are listed OUTSIDE first, matching both the
/// standard's own tables and this engine's precompute convention for exterior optical properties.
fn light_constructions() -> Vec<Construction> {
    vec![
        Construction { id: WALL_CONSTRUCTION, name: "Lightweight Exterior Wall".into(), layer_material_ids: vec![WOOD_SIDING, FIBERGLASS_QUILT, PLASTERBOARD] },
        Construction { id: FLOOR_CONSTRUCTION, name: "Lightweight Floor".into(), layer_material_ids: vec![FLOOR_INSULATION, TIMBER_FLOORING] },
        Construction { id: ROOF_CONSTRUCTION, name: "Roof".into(), layer_material_ids: vec![ROOF_DECK, ROOF_QUILT, ROOF_PLASTERBOARD] },
    ]
}

/// 🧱️ High-mass (case 900) wall and floor; the roof is unchanged from the 600 series.
fn heavy_constructions() -> Vec<Construction> {
    vec![
        Construction { id: WALL_CONSTRUCTION, name: "High Mass Exterior Wall".into(), layer_material_ids: vec![WOOD_SIDING, FOAM_INSULATION, CONCRETE_BLOCK] },
        Construction { id: FLOOR_CONSTRUCTION, name: "High Mass Floor".into(), layer_material_ids: vec![HIGH_MASS_FLOOR_INSULATION, CONCRETE_SLAB] },
        Construction { id: ROOF_CONSTRUCTION, name: "Roof".into(), layer_material_ids: vec![ROOF_DECK, ROOF_QUILT, ROOF_PLASTERBOARD] },
    ]
}

fn wall(id: EntityId, name: &str, vertices_m: Vec<[f64; 3]>) -> Surface {
    Surface { id, name: name.into(), zone_id: ZONE, class: SurfaceClass::ExteriorWall, vertices_m, construction_id: WALL_CONSTRUCTION, outside_boundary_condition: OutsideBoundary::OutdoorAir, sun_exposed: true, wind_exposed: true, multiplier: 1 }
}

/// 📐️ The 8 m × 6 m × 2.7 m box. `x` runs west→east, `y` south→north, `z` up; every polygon is
/// wound counter-clockwise seen from OUTSIDE, so the Newell normal points out of the zone.
fn surfaces() -> Vec<Surface> {
    vec![
        wall(SOUTH_WALL, "South Wall", vec![[0.0, 0.0, 0.0], [WIDTH_M, 0.0, 0.0], [WIDTH_M, 0.0, HEIGHT_M], [0.0, 0.0, HEIGHT_M]]),
        wall(EAST_WALL, "East Wall", vec![[WIDTH_M, 0.0, 0.0], [WIDTH_M, DEPTH_M, 0.0], [WIDTH_M, DEPTH_M, HEIGHT_M], [WIDTH_M, 0.0, HEIGHT_M]]),
        wall(NORTH_WALL, "North Wall", vec![[WIDTH_M, DEPTH_M, 0.0], [0.0, DEPTH_M, 0.0], [0.0, DEPTH_M, HEIGHT_M], [WIDTH_M, DEPTH_M, HEIGHT_M]]),
        wall(WEST_WALL, "West Wall", vec![[0.0, DEPTH_M, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, HEIGHT_M], [0.0, DEPTH_M, HEIGHT_M]]),
        Surface {
            id: ROOF,
            name: "Roof".into(),
            zone_id: ZONE,
            class: SurfaceClass::Roof,
            vertices_m: vec![[0.0, 0.0, HEIGHT_M], [WIDTH_M, 0.0, HEIGHT_M], [WIDTH_M, DEPTH_M, HEIGHT_M], [0.0, DEPTH_M, HEIGHT_M]],
            construction_id: ROOF_CONSTRUCTION,
            outside_boundary_condition: OutsideBoundary::OutdoorAir,
            sun_exposed: true,
            wind_exposed: true,
            multiplier: 1,
        },
        Surface {
            id: FLOOR,
            name: "Floor".into(),
            zone_id: ZONE,
            class: SurfaceClass::Floor,
            vertices_m: vec![[0.0, 0.0, 0.0], [0.0, DEPTH_M, 0.0], [WIDTH_M, DEPTH_M, 0.0], [WIDTH_M, 0.0, 0.0]],
            construction_id: FLOOR_CONSTRUCTION,
            outside_boundary_condition: OutsideBoundary::Ground,
            sun_exposed: false,
            wind_exposed: false,
            multiplier: 1,
        },
    ]
}

/// 🪟️ One §5.2 window: 3 m × 2 m of double clear glazing, U = 3.0 W/(m²·K), SHGC 0.787.
fn window(id: EntityId, name: &str, surface_id: EntityId) -> Fenestration {
    Fenestration {
        id,
        name: name.into(),
        surface_id,
        u_value_w_m2k: 3.0,
        shgc: 0.787,
        vlt: 0.86,
        area_m2: WINDOW_WIDTH_M * WINDOW_HEIGHT_M,
        height_m: WINDOW_HEIGHT_M,
        sill_height_m: WINDOW_SILL_M,
        frame_conductance_w_k: 0.0,
        divider_conductance_w_k: 0.0,
        overhang_depth_m: 0.0,
        overhang_offset_m: 0.0,
        fin_depth_m: 0.0,
        fin_offset_m: 0.0,
        glazing_construction_id: None,
    }
}

fn constant(id: ScheduleId, value: f64) -> ConstantSchedule {
    ConstantSchedule { id, value }
}

fn daily(id: ScheduleId, hourly_values: [f64; 24]) -> DailySchedule {
    DailySchedule { id, hourly_values, interpolation: ScheduleInterpolation::Discrete, limits: None }
}

/// 📅️ The schedules cases 600/900 need: always-on gains and infiltration, 20 °C heating, 27 °C
/// cooling. Setpoint schedules carry the setpoint ITSELF in °C, not a normalized fraction.
fn base_schedules() -> ScheduleSet {
    ScheduleSet { constants: vec![constant(ALWAYS_ON, 1.0), constant(HEATING_SETPOINT, 20.0), constant(COOLING_SETPOINT, 27.0)], ..ScheduleSet::default() }
}

fn base_model(name: &str) -> Model {
    Model {
        name: name.into(),
        version: "ashrae-140-5.2".into(),
        site: Site { latitude_deg: 39.83, longitude_deg: -104.65, elevation_m: 1650.0, time_zone_hours: -7.0, north_axis_deg: 0.0 },
        zones: vec![Zone { id: ZONE, name: "Zone".into(), volume_m3: VOLUME_M3, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
        surfaces: surfaces(),
        materials: materials(),
        constructions: light_constructions(),
        equipment: vec![EquipmentGain { id: INTERNAL_GAIN, zone_id: ZONE, schedule_id: ALWAYS_ON, watts_per_area: INTERNAL_GAIN_W / FLOOR_AREA_M2, radiant_fraction: 0.6, latent_fraction: 0.0 }],
        infiltrations: vec![Infiltration {
            id: INFILTRATION,
            zone_id: ZONE,
            schedule_id: ALWAYS_ON,
            method: InfiltrationMethod::ScheduledAch,
            design_flow_ach: INFILTRATION_ACH,
            flow_per_exterior_area_m3_s_m2: 0.0,
            effective_leakage_area_m2: 0.0,
            discharge_coefficient: 0.65,
            stack_height_m: HEIGHT_M,
            constant_term_coefficient: 0.0,
            temperature_term_coefficient: 0.0,
            velocity_term_coefficient: 0.0,
            velocity_squared_term_coefficient: 0.0,
        }],
        thermostats: vec![Thermostat { id: THERMOSTAT, zone_id: ZONE, heating_setpoint_schedule_id: HEATING_SETPOINT, cooling_setpoint_schedule_id: COOLING_SETPOINT, heating_throttle_range_k: 0.0, cooling_throttle_range_k: 0.0 }],
        ideal_loads: vec![IdealLoadsSystem {
            id: IDEAL_LOADS,
            zone_id: ZONE,
            max_heating_supply_air_temp_c: 50.0,
            min_cooling_supply_air_temp_c: 13.0,
            max_heating_capacity_w: None,
            max_cooling_capacity_w: None,
            outdoor_air_per_person_m3_s: 0.0,
            outdoor_air_per_area_m3_s_m2: 0.0,
        }],
        ground_temperature: crate::model::GroundTemperatureConfig { building_surface_c: [GROUND_TEMPERATURE_C; 12], shallow_c: [GROUND_TEMPERATURE_C; 12], deep_c: GROUND_TEMPERATURE_C },
        schedules: base_schedules(),
        ..Model::default()
    }
}
// #endregion 🔖️Catalogue

// #region 🔖️Cases
/// 🏛️ Case 600 — lightweight, 12 m² of south glazing, 20/27 °C setpoints.
fn case_600() -> Model {
    let mut built = base_model("BESTEST 600");
    built.fenestrations = vec![window(WINDOW_A, "South Window West", SOUTH_WALL), window(WINDOW_B, "South Window East", SOUTH_WALL)];
    built
}

/// 🏛️ Case 610 — case 600 plus a 1 m horizontal overhang across the full south wall at roof level.
fn case_610() -> Model {
    let mut built = case_600();
    built.name = "BESTEST 610".into();
    for fenestration in &mut built.fenestrations {
        fenestration.overhang_depth_m = PROJECTION_DEPTH_M;
        fenestration.overhang_offset_m = OVERHANG_OFFSET_M;
    }
    built
}

/// 🏛️ Case 620 — case 600's glazing moved to 6 m² east and 6 m² west.
fn case_620() -> Model {
    let mut built = base_model("BESTEST 620");
    built.fenestrations = vec![window(WINDOW_A, "East Window", EAST_WALL), window(WINDOW_B, "West Window", WEST_WALL)];
    built
}

/// 🏛️ Case 630 — case 620 with a 1 m overhang above and a 1 m fin beside each window.
fn case_630() -> Model {
    let mut built = case_620();
    built.name = "BESTEST 630".into();
    for fenestration in &mut built.fenestrations {
        fenestration.overhang_depth_m = PROJECTION_DEPTH_M;
        fenestration.overhang_offset_m = 0.0;
        fenestration.fin_depth_m = PROJECTION_DEPTH_M;
        fenestration.fin_offset_m = 0.0;
    }
    built
}

/// 🏛️ Case 640 — case 600 with the heating setpoint set back to 10 °C from 23:00 to 07:00.
fn case_640() -> Model {
    let mut built = case_600();
    built.name = "BESTEST 640".into();
    let mut hourly = [20.0_f64; 24];
    for hour in (0..7).chain(23..24) {
        hourly[hour] = 10.0;
    }
    built.schedules.constants.retain(|schedule| schedule.id != HEATING_SETPOINT);
    built.schedules.daily.push(daily(HEATING_SETBACK, hourly));
    built.thermostats[0].heating_setpoint_schedule_id = HEATING_SETBACK;
    built
}

/// 🏛️ Case 650 — case 600 with no heating, cooling only from 07:00 to 18:00, and 1703.16 m³/h of
/// night ventilation from 18:00 to 07:00. The "disabled" setpoints are far outside any weather the
/// zone can reach, which is how a schedule-driven thermostat expresses "off".
fn case_650() -> Model {
    let mut built = case_600();
    built.name = "BESTEST 650".into();
    let mut cooling = [100.0_f64; 24];
    cooling[7..18].fill(27.0);
    let mut ventilation = [1.0_f64; 24];
    ventilation[7..18].fill(0.0);
    built.schedules.constants.retain(|schedule| schedule.id != HEATING_SETPOINT && schedule.id != COOLING_SETPOINT);
    built.schedules.constants.push(constant(HEATING_DISABLED, -100.0));
    built.schedules.daily.push(daily(COOLING_DAYTIME_ONLY, cooling));
    built.schedules.daily.push(daily(NIGHT_VENTILATION_SCHEDULE, ventilation));
    built.thermostats[0].heating_setpoint_schedule_id = HEATING_DISABLED;
    built.thermostats[0].cooling_setpoint_schedule_id = COOLING_DAYTIME_ONLY;
    built.mechanical_ventilations = vec![MechanicalVentilation { id: NIGHT_VENTILATION, zone_id: ZONE, schedule_id: NIGHT_VENTILATION_SCHEDULE, design_flow_m3_s: NIGHT_VENTILATION_M3_S, fan_total_efficiency: 1.0, fan_delta_pressure_pa: 0.0 }];
    built
}

/// 🏛️ The 900-series delta: the same case with high-mass wall and floor constructions.
fn high_mass(mut built: Model, name: &str) -> Model {
    built.name = name.into();
    built.constructions = heavy_constructions();
    built
}

/// 🏛️ The free-float delta: no thermostat, no ideal-loads system, nothing conditioning the zone.
fn free_float(mut built: Model, name: &str) -> Model {
    built.name = name.into();
    built.thermostats.clear();
    built.ideal_loads.clear();
    built.schedules.constants.retain(|schedule| schedule.id != HEATING_SETPOINT && schedule.id != COOLING_SETPOINT);
    built
}
// #endregion 🔖️Cases

// #region 🔖️Run
/// ⚙️ The §5.2 run: a full Denver year at an hourly zone timestep after a warmup.
///
/// `schedules` is copied out of the model, which is the authority: [`SimulationConfig`] still owns
/// its own `ScheduleSet` because the kernel's admission census and close-step pump read it there,
/// so the projection is derived here rather than duplicated by the caller.
pub fn simulation_config(model: &Model, weather: Option<EpwWeather>, warmup_days: u32) -> SimulationConfig {
    SimulationConfig {
        environment: SimulationEnvironment::WeatherRunPeriod,
        zone_timestep_minutes: 60,
        system_timestep_minutes: 60,
        warmup_days,
        run_period_start_month: 1,
        run_period_start_day: 1,
        run_period_end_month: 12,
        run_period_end_day: 31,
        schedules: model.schedules.clone(),
        weather,
        ..SimulationConfig::default()
    }
}

/// 🏛️ Validate, then run, one case against an EPW text.
pub fn run(case: &str, epw_text: &str, warmup_days: u32) -> Result<Results, Diagnostics> {
    let built = model(case).ok_or_else(|| single(Error::fatal(format!("no ANSI/ASHRAE 140 §5.2 case is registered as {case:?}"))))?;
    built.validate()?;
    let weather = EpwWeather::parse(epw_text).map_err(single)?;
    let config = simulation_config(&built, Some(weather), warmup_days);
    crate::sim::Engine::run(built, config).map_err(single)
}

fn single(error: Error) -> Diagnostics {
    let mut diagnostics = Diagnostics::default();
    diagnostics.push(error);
    diagnostics
}
// #endregion 🔖️Run

// #region 🔖️Projection
/// 📊️ One case's results in the shape both producers of the BESTEST comparison emit.
#[derive(Clone, Debug, PartialEq)]
pub struct CaseResults {
    pub case: String,
    pub timestep_minutes: u32,
    pub annual_heating_kwh: f64,
    pub annual_cooling_kwh: f64,
    pub peak_heating_kw: f64,
    pub peak_heating_hour: u32,
    pub peak_cooling_kw: f64,
    pub peak_cooling_hour: u32,
    pub free_float_min_c: f64,
    pub free_float_min_hour: u32,
    pub free_float_max_c: f64,
    pub free_float_max_hour: u32,
    pub free_float_mean_c: f64,
    pub zone_air_temperature_c: Vec<f64>,
    pub heating_w: Vec<f64>,
    pub cooling_w: Vec<f64>,
}

/// 📊️ Project a [`Results`] onto [`CaseResults`].
///
/// ⚠️ Honest gap, left visible rather than faked: the aggregation pass in
/// `⚙️engine/🧪️sim/🦀️.rs`'s `step_aggregate_zone` registers exactly ONE time series per zone
/// (`Zone Air Temperature [<zone>]`) and puts heating/cooling into METERS, which retain only the
/// running total and the peak — not an hourly trace. So `heating_w`/`cooling_w` come back EMPTY
/// here and the comparison harness reports the hourly-power metrics as unavailable instead of
/// comparing zeros against EnergyPlus. Filling them needs `series` to become `zones × 3` in
/// `EnergyNumericalCensus::observe` plus a series branch in the `Heating`/`Cooling` aggregation
/// stages; that is a real change to the admission census, not a projection detail.
pub fn project(case: &str, results: &Results) -> CaseResults {
    let temperatures: Vec<f64> = results.time_series.series.values().next().map(|series| series.values.clone()).unwrap_or_default();
    let heating = results.meters.meters.iter().find(|(key, _)| key.ends_with(" Heating") && !key.starts_with("Facility")).map(|(_, meter)| meter);
    let cooling = results.meters.meters.iter().find(|(key, _)| key.ends_with(" Cooling")).map(|(_, meter)| meter);
    let (min_c, min_hour) = extremum(&temperatures, true);
    let (max_c, max_hour) = extremum(&temperatures, false);
    CaseResults {
        case: case.to_string(),
        timestep_minutes: 60,
        annual_heating_kwh: heating.map_or(0.0, |meter| meter.energy_kwh()),
        annual_cooling_kwh: cooling.map_or(0.0, |meter| meter.energy_kwh()),
        peak_heating_kw: heating.map_or(0.0, |meter| meter.peak_demand_w / 1000.0),
        peak_heating_hour: heating.map_or(0, |meter| meter.peak_demand_hour as u32),
        peak_cooling_kw: cooling.map_or(0.0, |meter| meter.peak_demand_w / 1000.0),
        peak_cooling_hour: cooling.map_or(0, |meter| meter.peak_demand_hour as u32),
        free_float_min_c: min_c,
        free_float_min_hour: min_hour,
        free_float_max_c: max_c,
        free_float_max_hour: max_hour,
        free_float_mean_c: if temperatures.is_empty() { 0.0 } else { temperatures.iter().sum::<f64>() / temperatures.len() as f64 },
        zone_air_temperature_c: temperatures,
        heating_w: Vec::new(),
        cooling_w: Vec::new(),
    }
}

/// 📄️ One case's results as the BESTEST comparison contract's own JSON — the crate-side bridge the
/// language-neutral test adapter calls, since `Results`' tables are `pub(crate)` and unnameable
/// outside this crate.
pub fn results_report_json(case: &str, epw_text: &str, epw_file: &str, epw_sha256: &str, warmup_days: u32) -> Result<String, String> {
    let results = run(case, epw_text, warmup_days).map_err(|diagnostics| diagnostics.messages.iter().map(|message| message.message.clone()).collect::<Vec<_>>().join("; "))?;
    Ok(report_json(&project(case, &results), epw_file, epw_sha256))
}

/// 📄️ Serialize a [`CaseResults`] into `semio.energy.bestest-results/1`.
pub fn report_json(projected: &CaseResults, epw_file: &str, epw_sha256: &str) -> String {
    let numbers = |values: &[f64]| pack::json::array(values.iter().map(|value| pack::json::Value::from(*value)));
    let controlled = !is_free_float(&projected.case);
    let annual =
        if controlled { pack::json::object([("heatingKwh".to_string(), pack::json::Value::from(projected.annual_heating_kwh)), ("coolingKwh".to_string(), pack::json::Value::from(projected.annual_cooling_kwh))]) } else { pack::json::Value::Null };
    let peak = if controlled {
        pack::json::object([
            ("heatingKw".to_string(), pack::json::Value::from(projected.peak_heating_kw)),
            ("heatingHour".to_string(), pack::json::Value::from(projected.peak_heating_hour)),
            ("coolingKw".to_string(), pack::json::Value::from(projected.peak_cooling_kw)),
            ("coolingHour".to_string(), pack::json::Value::from(projected.peak_cooling_hour)),
        ])
    } else {
        pack::json::Value::Null
    };
    let free_float = if controlled {
        pack::json::Value::Null
    } else {
        pack::json::object([
            ("minC".to_string(), pack::json::Value::from(projected.free_float_min_c)),
            ("minHour".to_string(), pack::json::Value::from(projected.free_float_min_hour)),
            ("maxC".to_string(), pack::json::Value::from(projected.free_float_max_c)),
            ("maxHour".to_string(), pack::json::Value::from(projected.free_float_max_hour)),
            ("meanC".to_string(), pack::json::Value::from(projected.free_float_mean_c)),
        ])
    };
    let report = pack::json::object([
        ("schema".to_string(), pack::json::Value::from("semio.energy.bestest-results/1")),
        ("case".to_string(), pack::json::Value::from(projected.case.as_str())),
        (
            "producer".to_string(),
            pack::json::object([
                ("name".to_string(), pack::json::Value::from("semio-energy-engine")),
                ("version".to_string(), pack::json::Value::from(env!("CARGO_PKG_VERSION"))),
                ("via".to_string(), pack::json::Value::from(concat!("semio-s-plugin-energy ", env!("CARGO_PKG_VERSION")))),
            ]),
        ),
        ("weather".to_string(), pack::json::object([("file".to_string(), pack::json::Value::from(epw_file)), ("sha256".to_string(), if epw_sha256.is_empty() { pack::json::Value::Null } else { pack::json::Value::from(epw_sha256) })])),
        ("timestepMinutes".to_string(), pack::json::Value::from(projected.timestep_minutes)),
        ("annual".to_string(), annual),
        ("peak".to_string(), peak),
        ("freeFloat".to_string(), free_float),
        ("hourly".to_string(), pack::json::object([("zoneAirTemperatureC".to_string(), numbers(&projected.zone_air_temperature_c)), ("heatingW".to_string(), numbers(&projected.heating_w)), ("coolingW".to_string(), numbers(&projected.cooling_w))])),
    ]);
    pack::json::to_string(&report)
}

/// 📄️ One case's model as the engine's own canonical JSON — the exact bytes the committed
/// `🧫️fixtures/🏛️bestest-<case>/🔋️model.json` carries and the `model` field of an
/// `EnergyModelSnapshot` holds.
pub fn model_json(case: &str) -> Option<String> {
    Some(pack::json::to_json_string(&model(case)?))
}

/// 📄️ The inverse of [`model_json`] — the committed fixture back into a typed model.
pub fn model_from_json(text: &str) -> Result<Model, String> {
    pack::json::from_json_str::<Model>(text).map_err(|error| error.to_string())
}

/// 📐️ The derived quantities ANSI/ASHRAE 140 §5.2 states directly (areas, air-to-air U-values,
/// glazing, infiltration flow, internal gain), so a second implementation can check this
/// repository's reading of the standard against its own without simulating anything.
///
/// The U-values are the air-to-air ones the standard quotes: layer resistances plus this engine's
/// own interior and exterior film resistances, the same sum `crate::precompute` performs when it
/// builds a surface's CTF.
pub fn case_parameters_json(case: &str) -> Option<String> {
    let built = model(case)?;
    let resistance = |construction_id: EntityId| -> f64 {
        let construction = built.constructions.iter().find(|construction| construction.id == construction_id);
        let layers: f64 =
            construction.map_or(0.0, |construction| construction.layer_material_ids.iter().filter_map(|id| built.materials.iter().find(|material| material.id == *id)).map(|material| material.thickness_m / material.conductivity_w_m_k).sum());
        layers + crate::material::R_FILM_INTERIOR_M2K_W + crate::material::R_FILM_EXTERIOR_M2K_W
    };
    let surfaces = pack::json::array(built.surfaces.iter().map(|surface| {
        let gross = crate::geometry::surface_area_m2(&surface.vertices_m);
        let glazed: f64 = built.fenestrations.iter().filter(|window| window.surface_id == surface.id).map(|window| window.area_m2).sum();
        let orientation = crate::geometry::surface_tilt_azimuth(crate::geometry::polygon_normal(&surface.vertices_m), built.site.north_axis_deg);
        pack::json::object([
            ("name".to_string(), pack::json::Value::from(surface.name.as_str())),
            ("grossAreaM2".to_string(), pack::json::Value::from(gross)),
            ("netOpaqueAreaM2".to_string(), pack::json::Value::from(gross - glazed)),
            ("uValueWM2K".to_string(), pack::json::Value::from(1.0 / resistance(surface.construction_id))),
            ("tiltDeg".to_string(), pack::json::Value::from(orientation.tilt_deg)),
            ("azimuthDeg".to_string(), pack::json::Value::from(orientation.azimuth_deg)),
        ])
    }));
    let windows = pack::json::array(built.fenestrations.iter().map(|window| {
        pack::json::object([
            ("name".to_string(), pack::json::Value::from(window.name.as_str())),
            ("areaM2".to_string(), pack::json::Value::from(window.area_m2)),
            ("uValueWM2K".to_string(), pack::json::Value::from(window.u_value_w_m2k)),
            ("shgc".to_string(), pack::json::Value::from(window.shgc)),
            ("overhangDepthM".to_string(), pack::json::Value::from(window.overhang_depth_m)),
            ("finDepthM".to_string(), pack::json::Value::from(window.fin_depth_m)),
        ])
    }));
    let zone = built.zones.first()?;
    let report = pack::json::object([
        ("schema".to_string(), pack::json::Value::from("semio.energy.bestest-parameters/1")),
        ("case".to_string(), pack::json::Value::from(case)),
        ("zoneVolumeM3".to_string(), pack::json::Value::from(zone.volume_m3)),
        ("floorAreaM2".to_string(), pack::json::Value::from(FLOOR_AREA_M2)),
        ("infiltrationAch".to_string(), pack::json::Value::from(built.infiltrations.first().map_or(0.0, |infiltration| infiltration.design_flow_ach))),
        ("internalGainW".to_string(), pack::json::Value::from(built.equipment.iter().map(|gain| gain.watts_per_area * FLOOR_AREA_M2).sum::<f64>())),
        ("conditioned".to_string(), pack::json::Value::Bool(!built.ideal_loads.is_empty())),
        ("groundTemperatureC".to_string(), pack::json::Value::from(built.ground_temperature.building_surface_c[0])),
        ("surfaces".to_string(), surfaces),
        ("windows".to_string(), windows),
    ]);
    Some(pack::json::to_string(&report))
}

fn extremum(values: &[f64], minimum: bool) -> (f64, u32) {
    let mut best = if minimum { f64::INFINITY } else { f64::NEG_INFINITY };
    let mut hour = 0;
    for (index, value) in values.iter().enumerate() {
        if (minimum && *value < best) || (!minimum && *value > best) {
            best = *value;
            hour = index as u32;
        }
    }
    if values.is_empty() {
        (0.0, 0)
    } else {
        (best, hour)
    }
}
// #endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
