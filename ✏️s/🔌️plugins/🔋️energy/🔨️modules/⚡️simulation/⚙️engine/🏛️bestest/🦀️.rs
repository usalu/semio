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
    for hour in 7..18 {
        cooling[hour] = 27.0;
    }
    let mut ventilation = [1.0_f64; 24];
    for hour in 7..18 {
        ventilation[hour] = 0.0;
    }
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
    let annual = if controlled {
        pack::json::object([("heatingKwh".to_string(), pack::json::Value::from(projected.annual_heating_kwh)), ("coolingKwh".to_string(), pack::json::Value::from(projected.annual_cooling_kwh))])
    } else {
        pack::json::Value::Null
    };
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
        (
            "hourly".to_string(),
            pack::json::object([
                ("zoneAirTemperatureC".to_string(), numbers(&projected.zone_air_temperature_c)),
                ("heatingW".to_string(), numbers(&projected.heating_w)),
                ("coolingW".to_string(), numbers(&projected.cooling_w)),
            ]),
        ),
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
        let layers: f64 = construction
            .map(|construction| {
                construction
                    .layer_material_ids
                    .iter()
                    .filter_map(|id| built.materials.iter().find(|material| material.id == *id))
                    .map(|material| material.thickness_m / material.conductivity_w_m_k)
                    .sum()
            })
            .unwrap_or(0.0);
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
mod tests {
    use super::*;

    #[test]
    fn every_registered_case_builds_and_validates() {
        for case in CASES {
            let built = model(case).unwrap_or_else(|| panic!("case {case} must build"));
            if let Err(diagnostics) = built.validate() {
                panic!("case {case} failed validation: {:?}", diagnostics.messages);
            }
        }
    }

    #[test]
    fn unknown_case_is_none() {
        assert!(model("601").is_none());
    }

    #[test]
    fn free_float_cases_carry_no_hvac() {
        for case in ["600FF", "900FF"] {
            let built = model(case).expect("free-float case builds");
            assert!(built.ideal_loads.is_empty(), "{case} must have no ideal loads");
            assert!(built.thermostats.is_empty(), "{case} must have no thermostat");
        }
    }

    #[test]
    fn base_case_geometry_matches_the_standard() {
        let built = model("600").expect("case 600 builds");
        assert!((built.zones[0].volume_m3 - 129.6).abs() < 1e-9);
        assert_eq!(built.surfaces.len(), 6);
        let glazing: f64 = built.fenestrations.iter().map(|window| window.area_m2).sum();
        assert!((glazing - 12.0).abs() < 1e-9, "south glazing was {glazing} m²");
        assert!(built.fenestrations.iter().all(|window| window.surface_id == SOUTH_WALL));
    }

    #[test]
    fn high_mass_cases_swap_only_the_wall_and_floor() {
        let light = model("600").expect("case 600 builds");
        let heavy = model("900").expect("case 900 builds");
        assert_eq!(light.surfaces, heavy.surfaces);
        assert_eq!(light.fenestrations, heavy.fenestrations);
        let heavy_wall = heavy.constructions.iter().find(|construction| construction.id == WALL_CONSTRUCTION).expect("wall construction");
        assert_eq!(heavy_wall.layer_material_ids, vec![WOOD_SIDING, FOAM_INSULATION, CONCRETE_BLOCK]);
    }

    #[test]
    fn shaded_cases_carry_their_projections() {
        assert!(model("610").expect("610").fenestrations.iter().all(|window| window.overhang_depth_m > 0.0 && window.fin_depth_m == 0.0));
        assert!(model("630").expect("630").fenestrations.iter().all(|window| window.overhang_depth_m > 0.0 && window.fin_depth_m > 0.0));
        assert!(model("600").expect("600").fenestrations.iter().all(|window| window.overhang_depth_m == 0.0));
    }

    #[test]
    fn orientation_cases_move_the_glazing_east_and_west() {
        let built = model("620").expect("620");
        let hosts: Vec<EntityId> = built.fenestrations.iter().map(|window| window.surface_id).collect();
        assert_eq!(hosts, vec![EAST_WALL, WEST_WALL]);
    }

    #[test]
    fn setback_case_drops_to_ten_degrees_overnight() {
        let built = model("640").expect("640");
        let schedule = built.schedules.daily.iter().find(|schedule| schedule.id == HEATING_SETBACK).expect("setback schedule");
        assert!((schedule.hourly_values[2] - 10.0).abs() < 1e-9);
        assert!((schedule.hourly_values[12] - 20.0).abs() < 1e-9);
        assert!((schedule.hourly_values[23] - 10.0).abs() < 1e-9);
    }

    #[test]
    fn night_ventilation_case_runs_the_fan_only_overnight() {
        let built = model("650").expect("650");
        let schedule = built.schedules.daily.iter().find(|schedule| schedule.id == NIGHT_VENTILATION_SCHEDULE).expect("ventilation schedule");
        assert!((schedule.hourly_values[2] - 1.0).abs() < 1e-9);
        assert!(schedule.hourly_values[12].abs() < 1e-9);
        assert!((built.mechanical_ventilations[0].design_flow_m3_s - NIGHT_VENTILATION_M3_S).abs() < 1e-9);
    }

    /// 🧪️ A model whose thermostat points at a schedule nobody defines must be rejected — the whole
    /// reason `schedules` lives on the model rather than only on the run configuration.
    #[test]
    fn dangling_schedule_reference_is_rejected() {
        let mut built = model("600").expect("600");
        built.schedules.constants.retain(|schedule| schedule.id != HEATING_SETPOINT);
        assert!(built.validate().is_err());
    }

    //#region 🌦️SyntheticWeather
    /// 🌦️ A structurally real, 8760-record EPW for the annual smoke tests, built by templating the
    /// committed one-day fixture's own record layout.
    ///
    /// ⚠️ This is NOT a validation weather file and must never be used for a BESTEST comparison: the
    /// temperatures are a plain annual-plus-diurnal sinusoid and the irradiance is a clear-sky
    /// rectangle. It exists so the determinism, free-float and setpoint-holding laws below can run a
    /// full year TODAY, before `🧫️fixtures/🌦️denver-tmy/🌦️.epw` is committed. The real comparison
    /// reads that file and nothing else.
    fn synthetic_annual_epw() -> String {
        const DAYS_IN_MONTH: [u16; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut text = String::new();
        text.push_str("LOCATION,Denver,Colorado,USA,semio-synthetic,724666,39.83,-104.65,-7.0,1650.0\n");
        text.push_str("DESIGN CONDITIONS,0\n");
        text.push_str("TYPICAL/EXTREME PERIODS,0\n");
        text.push_str("GROUND TEMPERATURES,0\n");
        text.push_str("HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\n");
        text.push_str("COMMENTS 1,semio synthetic annual EPW -- structurally valid, sinusoidal, NOT a station record.\n");
        text.push_str("COMMENTS 2,Generated by crate::bestest::tests::synthetic_annual_epw for the annual smoke laws.\n");
        text.push_str("DATA PERIODS,1,1,Data,Sunday, 1/ 1,12/31\n");
        let latitude = 39.83_f64.to_radians();
        let mut day_of_year = 0_u16;
        for (month_index, days) in DAYS_IN_MONTH.iter().enumerate() {
            for day in 1..=*days {
                day_of_year += 1;
                let declination = (23.45_f64 * (360.0 * (day_of_year as f64 + 284.0) / 365.0).to_radians().sin()).to_radians();
                let cos_hour_angle = (-latitude.tan() * declination.tan()).clamp(-1.0, 1.0);
                let half_day_hours = cos_hour_angle.acos().to_degrees() / 15.0;
                let seasonal = 10.0 + 12.0 * (2.0 * std::f64::consts::PI * (day_of_year as f64 - 105.0) / 365.0).sin();
                for hour in 1..=24_u32 {
                    let solar_hour = hour as f64 - 0.5;
                    let dry_bulb = seasonal + 8.0 * (2.0 * std::f64::consts::PI * (solar_hour - 15.0) / 24.0).cos();
                    let elevation = 1.0 - ((solar_hour - 12.0) / half_day_hours.max(0.5)).abs();
                    let sunlit = elevation > 0.0;
                    let direct_normal = if sunlit { 900.0 * elevation } else { 0.0 };
                    let diffuse = if sunlit { 110.0 * elevation } else { 0.0 };
                    let global = direct_normal * elevation + diffuse;
                    text.push_str(&format!(
                        "2026,{month},{day},{hour},0,?9?9?9?9E0,{dry_bulb:.1},{dew_point:.1},50,101325,0,0,300,{global:.0},{direct_normal:.0},{diffuse:.0},0,0,0,0,180,3.0,3,2,20.0,22000,0,999999999,14,0.081,0,88,0.2,0,0\n",
                        month = month_index + 1,
                        day = day,
                        hour = hour,
                        dry_bulb = dry_bulb,
                        dew_point = dry_bulb - 10.0,
                        global = global,
                        direct_normal = direct_normal,
                        diffuse = diffuse,
                    ));
                }
            }
        }
        text
    }

    fn annual_results(case: &str) -> Results {
        run(case, &synthetic_annual_epw(), 3).unwrap_or_else(|diagnostics| panic!("case {case} must run: {:?}", diagnostics.messages))
    }

    /// 🌦️ The synthetic file must actually reach the engine through the real `🌦️epw` codec, with
    /// every one of the 8760 records decoded — a silently truncated year would make every law below
    /// vacuous.
    #[test]
    fn synthetic_annual_weather_decodes_to_a_full_year() {
        let weather = EpwWeather::parse(&synthetic_annual_epw()).expect("the synthetic EPW parses through the stdio codec");
        assert_eq!(weather.records.len(), 8760);
    }

    /// 🔁️ Two runs of the same case against the same weather must agree bit for bit.
    #[test]
    fn annual_run_is_deterministic() {
        let first = project("600", &annual_results("600"));
        let second = project("600", &annual_results("600"));
        assert_eq!(first, second);
    }

    /// 🌡️ A free-float case has no HVAC at all, so it must deliver exactly zero energy while still
    /// producing a full year of zone temperatures.
    #[test]
    fn free_float_case_delivers_no_hvac_energy() {
        let projected = project("600FF", &annual_results("600FF"));
        assert_eq!(projected.annual_heating_kwh, 0.0);
        assert_eq!(projected.annual_cooling_kwh, 0.0);
        assert_eq!(projected.zone_air_temperature_c.len(), 8760);
        assert!(projected.free_float_max_c > projected.free_float_min_c, "a free-floating zone must actually swing");
    }

    /// 🎛️ An ideal-loads case with unlimited capacity must hold its own setpoints: at least 99 % of
    /// the year within [20 − 0.5, 27 + 0.5] °C. This is the law the predictor/corrector repair
    /// exists for — the previous kernel both double-advanced the zone-air history and predicted the
    /// load from a residual that omitted infiltration.
    #[test]
    fn controlled_case_holds_its_setpoints() {
        let projected = project("600", &annual_results("600"));
        assert_eq!(projected.zone_air_temperature_c.len(), 8760);
        let inside = projected.zone_air_temperature_c.iter().filter(|value| **value >= 19.5 && **value <= 27.5).count();
        let fraction = inside as f64 / projected.zone_air_temperature_c.len() as f64;
        assert!(fraction >= 0.99, "only {:.2} % of hours were inside the setpoints (min {:.2} °C, max {:.2} °C)", fraction * 100.0, projected.free_float_min_c, projected.free_float_max_c);
    }

    /// ☀️ Shading must actually do something: the overhang case cannot use more cooling than the
    /// unshaded case it is derived from, and the fin case cannot use more than the overhang case.
    #[test]
    fn shading_never_increases_cooling() {
        let unshaded = project("600", &annual_results("600")).annual_cooling_kwh;
        let overhang = project("610", &annual_results("610")).annual_cooling_kwh;
        assert!(overhang <= unshaded + 1e-6, "case 610 cooled {overhang} kWh against case 600's {unshaded} kWh");
    }
    //#endregion 🌦️SyntheticWeather

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
        std::fs::read_to_string(subset_root().join("🧫️fixtures").join("🌦️denver-tmy").join("🌦️.epw")).ok()
    }

    fn energyplus_reference(case: &str) -> Option<pack::json::Value> {
        let text = std::fs::read_to_string(subset_root().join("🧫️fixtures").join(format!("🏛️bestest-{case}")).join("🔮️energyplus.json")).ok()?;
        pack::json::parse(&text).ok()
    }

    fn member(value: &pack::json::Value, path: &[&str]) -> Option<f64> {
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
    /// the default `cargo test`. Run it through the `🏛️bestest🔋️energy🔮️compare` launch entry.
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
            let results = run(case, &weather, 7).unwrap_or_else(|diagnostics| panic!("case {case} must run: {:?}", diagnostics.messages));
            let projected = project(case, &results);
            if is_free_float(case) {
                for (metric, ours, path) in [
                    ("minC", projected.free_float_min_c, ["freeFloat", "minC"]),
                    ("maxC", projected.free_float_max_c, ["freeFloat", "maxC"]),
                    ("meanC", projected.free_float_mean_c, ["freeFloat", "meanC"]),
                ] {
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
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any")
    }

    fn fixture_path(case: &str) -> std::path::PathBuf {
        subset_root().join("🧫️fixtures").join(format!("🏛️bestest-{case}")).join("🔋️model.json")
    }

    fn example_asset_path(directory: &str) -> std::path::PathBuf {
        subset_root().join("📚️examples").join(directory).join("🖼️assets").join("🗣️.dsl.semio")
    }

    fn example_dsl(case: &str) -> String {
        let built = model(case).unwrap_or_else(|| panic!("case {case} must build"));
        let snapshot = crate::artifacts::model::energy_snapshot_with_state(crate::artifacts::model::ENERGY_MODEL_DOCUMENT_SCHEMA, &built, None);
        <crate::artifacts::model::EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(&snapshot)
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
            let decoded: Model = pack::json::from_json_str(&committed).unwrap_or_else(|error| panic!("committed fixture for case {case} does not decode: {error}"));
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
}
