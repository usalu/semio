//! 🔄️ Simulation kernel: the hourly timestep cursor machine and its coupled surface/zone heat
//! balance.
//!
//! Each hour first resolves every zone's schedules, gains, infiltration and setpoints, then advances
//! the building through `3600 s / Δt` implicit heat-balance steps. One step of one zone solves the
//! EnergyPlus heat-balance method simultaneously rather than by lagged iteration: every construction
//! is an implicit finite-difference chain reduced to its inside face, every window a massless
//! glazing chain, the inside faces exchange long wave through the enclosure's gray-body factors, and
//! the zone air stores heat by the third-order backward difference. The thermostat pins the air at
//! its active setpoint and the ideal loads deliver exactly the heat that keeps it there.

use crate::air_exchange::{infiltration_flow_m3_s, InfiltrationSpec};
use crate::calendar::{RunPeriod, SimDate};
use crate::controls::{HumidistatSpec, ThermostatSpec};
use crate::curves::PerformanceCurve;
use crate::daylight::{dimmed_lighting_power_w, lighting_dimming_fraction, reference_point_illuminance_lux, simplified_daylight_factor};
use crate::electrical::{grid_balance, PvSystem, Transformer};
use crate::envelope::{eliminate_chain, exterior_convection_w_m2k, exterior_radiation_w_m2k, interior_convection_w_m2k, is_windward, substitute_chain, wind_speed_at_height};
use crate::error::Error;
use crate::fenestration::{gap_conductance_w_m2k, glazing_interior_convection_w_m2k, MAX_PANES};
use crate::gains::{compute_equipment_gain_w, compute_lighting_gain_w, compute_people_gain_w, ActivityLevel, GainDecomposition};
use crate::ideal_hvac::{ideal_loads_deliver, IdealLoadsConfig, IdealLoadsInput};
use crate::model::{EntityId, FixedTable, Model, OutsideBoundary};
use crate::num::solve_dense_in_place;
use crate::plant::{PlantLoopSimulation, PlantStream, Pump};
use crate::precompute::{EnclosureFace, EnclosureRadiation, PrecomputedModel};
use crate::props::{moist_air_cp_j_per_kg_k, moist_air_density};
use crate::schedule::{ScheduleContext, ScheduleSet};
use crate::site::{sun_direction, GroundTemperatureModel, TimestepWeather, WeatherRecord};
use crate::solar::{beam_overlap_m2, incidence_cosine, sunlit_fraction, SkyState};
use crate::units::STEFAN_BOLTZMANN;
use crate::zone_air::ZoneAirState;
use crate::zone_hvac::{ZoneEquipment, ZoneEquipmentRequest};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

const KELVIN: f64 = 273.15;
const FIXED_TEMPERATURE_CONDUCTANCE_W_M2K: f64 = 1.0e9;
const GROUND_REFLECTANCE: f64 = 0.2;

// #region 🔖️Config
/// ⚙️ Simulation environment type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum SimulationEnvironment {
    WeatherRunPeriod,
    HeatingDesignDay,
    CoolingDesignDay,
    CustomDesignPeriod,
}

/// ⚙️ Convergence tolerances.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct ConvergenceTolerances {
    pub temperature_k: f64,
    pub humidity_ratio: f64,
    pub mass_flow: f64,
    pub energy_w: f64,
    pub max_iterations: u32,
}

impl Default for ConvergenceTolerances {
    fn default() -> Self {
        Self { temperature_k: 0.01, humidity_ratio: 1e-5, mass_flow: 1e-4, energy_w: 1.0, max_iterations: 20 }
    }
}

/// ⚙️ Simulation configuration. `zone_timestep_minutes` is the interval at which weather, sun
/// position, shading and exterior film coefficients are refreshed; `system_timestep_minutes` the
/// implicit step of the coupled surface/air balance and its HVAC control (never longer than the
/// zone timestep). Both divide the hour.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SimulationConfig {
    pub environment: SimulationEnvironment,
    pub zone_timestep_minutes: u32,
    pub system_timestep_minutes: u32,
    pub warmup_days: u32,
    pub run_period_start_month: u8,
    pub run_period_start_day: u8,
    pub run_period_end_month: u8,
    pub run_period_end_day: u8,
    pub tolerances: ConvergenceTolerances,
    pub schedules: ScheduleSet,
    pub weather: Option<crate::site::EpwWeather>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            environment: SimulationEnvironment::WeatherRunPeriod,
            zone_timestep_minutes: 60,
            system_timestep_minutes: 60,
            warmup_days: 7,
            run_period_start_month: 1,
            run_period_start_day: 1,
            run_period_end_month: 12,
            run_period_end_day: 31,
            tolerances: ConvergenceTolerances::default(),
            schedules: ScheduleSet::default(),
            weather: None,
        }
    }
}
// #endregion 🔖️Config

// #region 🔖️DeliveredEnergy
/// ⚡️ Delivered energy per timestep for metering.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct DeliveredEnergy {
    pub heating_w: f64,
    pub cooling_w: f64,
    pub fan_w: f64,
    pub pump_w: f64,
    pub compressor_w: f64,
    pub gas_w: f64,
    pub pv_generation_w: f64,
    pub battery_charge_w: f64,
    pub shw_electric_w: f64,
    pub shw_gas_w: f64,
    pub refrigeration_w: f64,
    pub water_pump_w: f64,
}

impl DeliveredEnergy {
    pub fn total_electric_w(&self) -> f64 {
        self.heating_w + self.cooling_w + self.fan_w + self.pump_w + self.compressor_w + self.shw_electric_w + self.refrigeration_w + self.water_pump_w - self.pv_generation_w + self.battery_charge_w
    }

    fn scaled(&self, factor: f64) -> Self {
        Self {
            heating_w: self.heating_w * factor,
            cooling_w: self.cooling_w * factor,
            fan_w: self.fan_w * factor,
            pump_w: self.pump_w * factor,
            compressor_w: self.compressor_w * factor,
            gas_w: self.gas_w * factor,
            pv_generation_w: self.pv_generation_w * factor,
            battery_charge_w: self.battery_charge_w * factor,
            shw_electric_w: self.shw_electric_w * factor,
            shw_gas_w: self.shw_gas_w * factor,
            refrigeration_w: self.refrigeration_w * factor,
            water_pump_w: self.water_pump_w * factor,
        }
    }
}
// #endregion 🔖️DeliveredEnergy

// #region 🔖️State
/// 🔄️ Per-zone simulation state. `mean_air_temp_c` and `delivered` are the averages over the last
/// completed hour; `air` is the state after the last heat-balance step.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct ZoneState {
    pub air: ZoneAirState,
    pub mean_air_temp_c: f64,
    pub mean_radiant_temp_c: f64,
    pub heating_demand_w: f64,
    pub cooling_demand_w: f64,
    pub unmet_heating_w: f64,
    pub unmet_cooling_w: f64,
    pub delivered: DeliveredEnergy,
    pub hour_temperature_sum_c: f64,
    pub hour_delivered: DeliveredEnergy,
    pub hour_steps: u32,
}

impl ZoneState {
    pub(crate) fn new(temp_c: f64, humidity_ratio: f64) -> Self {
        Self {
            air: ZoneAirState::new(temp_c, humidity_ratio),
            mean_air_temp_c: temp_c,
            mean_radiant_temp_c: temp_c,
            heating_demand_w: 0.0,
            cooling_demand_w: 0.0,
            unmet_heating_w: 0.0,
            unmet_cooling_w: 0.0,
            delivered: DeliveredEnergy::default(),
            hour_temperature_sum_c: 0.0,
            hour_delivered: DeliveredEnergy::default(),
            hour_steps: 0,
        }
    }
}

/// 🧱️ Node temperatures of an opaque surface, outside face first, and its last room-side film.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SurfaceState {
    pub temperatures_c: Vec<f64>,
    pub inside_convection_w_m2k: f64,
}

impl SurfaceState {
    /// 🧱️ Inside-face temperature.
    pub fn inside_temp_c(&self) -> f64 {
        self.temperatures_c.last().copied().unwrap_or(0.0)
    }
}

/// 🪟️ Glazing face temperatures of a window, outside face first.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct WindowState {
    pub face_temperatures_c: [f64; 2 * MAX_PANES],
    pub inside_convection_w_m2k: f64,
}

/// 🧮️ Reserved scratch of the zone heat-balance solver, sized once for the largest enclosure.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SolverWorkspace {
    pub faces: usize,
    pub nodes: usize,
    pub matrix: Vec<f64>,
    pub scratch: Vec<f64>,
    pub rhs: Vec<f64>,
    pub solution: Vec<f64>,
    pub inside_absorbed_w_m2: Vec<f64>,
    pub pane_absorbed_w_m2: Vec<f64>,
    pub inside_convection_w_m2k: Vec<f64>,
    pub reduction: Vec<f64>,
    pub chain_a: Vec<f64>,
    pub chain_b: Vec<f64>,
}

impl SolverWorkspace {
    /// 🧮️ Reserves every buffer for enclosures of up to `faces` faces and chains of `nodes` nodes.
    pub(crate) fn reserve(&mut self, faces: usize, nodes: usize) -> Result<(), Error> {
        let order = faces + 1;
        let reject = || Error::severe("energy heat-balance workspace backing rejected");
        let exact = faces.min(crate::precompute::EXACT_ENCLOSURE_FACES) + 1;
        for (buffer, length) in [
            (&mut self.matrix, exact * exact),
            (&mut self.scratch, exact * exact),
            (&mut self.rhs, order),
            (&mut self.solution, order),
            (&mut self.inside_absorbed_w_m2, faces),
            (&mut self.pane_absorbed_w_m2, faces * MAX_PANES),
            (&mut self.inside_convection_w_m2k, faces),
            (&mut self.reduction, faces * 3),
            (&mut self.chain_a, faces * nodes.max(2 * MAX_PANES)),
            (&mut self.chain_b, faces * nodes.max(2 * MAX_PANES)),
        ] {
            buffer.try_reserve_exact(length).map_err(|_| reject())?;
            buffer.resize(length, 0.0);
        }
        self.faces = faces;
        self.nodes = nodes.max(2 * MAX_PANES);
        Ok(())
    }
}

/// 🔄️ Full simulation state.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SimulationModel {
    pub(crate) zones: FixedTable<EntityId, ZoneState>,
    pub(crate) surfaces: FixedTable<EntityId, SurfaceState>,
    pub(crate) windows: FixedTable<EntityId, WindowState>,
    pub(crate) solver: SolverWorkspace,
    /// 🧱️ Per-surface energy integrated over the RUN period only — every accumulate site is gated on
    /// `warmup_complete`, so the warmup days never contribute.
    pub(crate) per_surface: crate::results::SurfaceEnergyTable,
    pub(crate) warmup_complete: bool,
    pub(crate) hour: u32,
    pub(crate) delivered_total: DeliveredEnergy,
    pub(crate) battery_soc: f64,
    pub(crate) plant_supply_c: f64,
}

impl Default for SimulationModel {
    fn default() -> Self {
        Self {
            zones: FixedTable::default(),
            surfaces: FixedTable::default(),
            windows: FixedTable::default(),
            solver: SolverWorkspace::default(),
            per_surface: crate::results::SurfaceEnergyTable::default(),
            warmup_complete: false,
            hour: 0,
            delivered_total: DeliveredEnergy::default(),
            battery_soc: 0.5,
            plant_supply_c: 55.0,
        }
    }
}
// #endregion 🔖️State

// #region 🔖️TimestepJob
/// 🧭️ Bounded phase of one persistent hourly timestep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) enum TimestepStage {
    Zone,
    Balance,
    ZoneCommit,
    SecondaryPlant,
    SecondaryPv,
    SecondaryBattery,
    SecondaryServiceHotWater,
    SecondaryRefrigeration,
    SecondaryWater,
    Complete,
}

/// 🧭️ One-semantic-unit preparation cursor for a zone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) enum ZonePreparationStage {
    Begin,
    DaylightConfig,
    DaylightArea,
    DaylightEvaluate,
    People,
    Lighting,
    Equipment,
    Infiltration,
    AirflowNode,
    AirflowLink,
    AirflowSolve,
    MechanicalVentilation,
    Thermostat,
    Humidistat,
    Publish,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
struct ZonePreparationWork {
    stage: ZonePreparationStage,
    cursor: usize,
    zone_index: usize,
    zone_id: EntityId,
    floor_area_m2: f64,
    zone_temp_c: f64,
    zone_humidity_ratio: f64,
    daylight_target_lux: f64,
    daylight_transmittance: f64,
    daylight_area_m2: f64,
    lighting_dim: f64,
    internal_gain: GainDecomposition,
    infiltration_flow_m3_s: f64,
    airflow_zone_node: Option<u32>,
    airflow_flow_m3_s: f64,
    mechanical_flow_m3_s: f64,
    heating_setpoint_c: f64,
    cooling_setpoint_c: f64,
    thermostat_schedule: u8,
    thermostat: ThermostatSpec,
    humidistat: Option<HumidistatSpec>,
    controlled: bool,
}

/// 🌡️ One zone's hourly boundary conditions resolved from its schedules.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) struct ZoneLoads {
    convective_w: f64,
    radiant_w: f64,
    latent_w: f64,
    outdoor_air_m3_s: f64,
    heating_setpoint_c: f64,
    cooling_setpoint_c: f64,
    floor_area_m2: f64,
    controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) enum ScheduleLookupStage {
    Constant,
    Annual,
    AnnualHoliday,
    AnnualRule,
    ResolveDaily,
    Weekly,
    DirectDaily,
    TimeSeries,
}

/// 🧭️ Bounded phase of one zone's heat-balance step.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) enum BalanceStage {
    Weather,
    Clear,
    WindowSolar,
    BeamPatches,
    DiffuseSolar,
    Faces,
    Solve,
    IdealLoad,
    Fault,
    ApplyIdealLoad,
    ZoneEquipment,
    Settle,
    Commit,
}

#[cfg(test)]
pub(crate) const P7C1_TIMESTEP_STAGES: [TimestepStage; 10] = [
    TimestepStage::Zone,
    TimestepStage::Balance,
    TimestepStage::ZoneCommit,
    TimestepStage::SecondaryPlant,
    TimestepStage::SecondaryPv,
    TimestepStage::SecondaryBattery,
    TimestepStage::SecondaryServiceHotWater,
    TimestepStage::SecondaryRefrigeration,
    TimestepStage::SecondaryWater,
    TimestepStage::Complete,
];

#[cfg(test)]
pub(crate) const P7C1_ZONE_PREPARATION_STAGES: [ZonePreparationStage; 15] = [
    ZonePreparationStage::Begin,
    ZonePreparationStage::DaylightConfig,
    ZonePreparationStage::DaylightArea,
    ZonePreparationStage::DaylightEvaluate,
    ZonePreparationStage::People,
    ZonePreparationStage::Lighting,
    ZonePreparationStage::Equipment,
    ZonePreparationStage::Infiltration,
    ZonePreparationStage::AirflowNode,
    ZonePreparationStage::AirflowLink,
    ZonePreparationStage::AirflowSolve,
    ZonePreparationStage::MechanicalVentilation,
    ZonePreparationStage::Thermostat,
    ZonePreparationStage::Humidistat,
    ZonePreparationStage::Publish,
];

#[cfg(test)]
pub(crate) const P7C1_BALANCE_STAGES: [BalanceStage; 13] = [
    BalanceStage::Weather,
    BalanceStage::Clear,
    BalanceStage::WindowSolar,
    BalanceStage::BeamPatches,
    BalanceStage::DiffuseSolar,
    BalanceStage::Faces,
    BalanceStage::Solve,
    BalanceStage::IdealLoad,
    BalanceStage::Fault,
    BalanceStage::ApplyIdealLoad,
    BalanceStage::ZoneEquipment,
    BalanceStage::Settle,
    BalanceStage::Commit,
];

#[cfg(test)]
pub(crate) const P7C1_PLANT_STAGES: [PlantStage; 4] = [PlantStage::ReduceZoneLoad, PlantStage::BuildPriority, PlantStage::Dispatch, PlantStage::Simulate];

#[cfg(test)]
pub(crate) const P7C1_SCHEDULE_LOOKUP_STAGES: [ScheduleLookupStage; 8] = [
    ScheduleLookupStage::Constant,
    ScheduleLookupStage::Annual,
    ScheduleLookupStage::AnnualHoliday,
    ScheduleLookupStage::AnnualRule,
    ScheduleLookupStage::ResolveDaily,
    ScheduleLookupStage::Weekly,
    ScheduleLookupStage::DirectDaily,
    ScheduleLookupStage::TimeSeries,
];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
struct ScheduleLookupWork {
    requested_id: crate::model::ScheduleId,
    stage: ScheduleLookupStage,
    cursor: usize,
    annual_index: Option<usize>,
    daily_id: Option<crate::model::ScheduleId>,
    daily_fallback: ScheduleLookupStage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) enum PlantStage {
    ReduceZoneLoad,
    BuildPriority,
    Dispatch,
    Simulate,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
struct PlantWork {
    plant_index: usize,
    stage: PlantStage,
    zone_cursor: usize,
    equipment_cursor: usize,
    total_load_w: f64,
    remaining_load_w: f64,
    first_load_w: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
struct BatteryWork {
    battery_index: usize,
    zone_cursor: usize,
    net_load_w: f64,
}

/// 🧮️ Cursor of one zone's heat-balance step.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
struct BalanceWork {
    stage: BalanceStage,
    zone_cursor: usize,
    face_cursor: usize,
    back_cursor: usize,
    zone_diffuse_w: f64,
    air_diagonal_w_k: f64,
    air_rhs_w: f64,
    mean_radiant_sums: [f64; 6],
    free_temp_c: f64,
    target_temp_c: Option<f64>,
    required_w: f64,
    delivered_w: f64,
    remaining_heating_w: f64,
    remaining_cooling_w: f64,
    fault_factor: f64,
    ideal_cursor: usize,
    fault_cursor: usize,
    equipment_cursor: usize,
    selected_ideal: Option<usize>,
    delivered: DeliveredEnergy,
}

impl BalanceWork {
    fn new() -> Self {
        Self {
            stage: BalanceStage::Weather,
            zone_cursor: 0,
            face_cursor: 0,
            back_cursor: 0,
            zone_diffuse_w: 0.0,
            air_diagonal_w_k: 0.0,
            air_rhs_w: 0.0,
            mean_radiant_sums: [0.0; 6],
            free_temp_c: 0.0,
            target_temp_c: None,
            required_w: 0.0,
            delivered_w: 0.0,
            remaining_heating_w: 0.0,
            remaining_cooling_w: 0.0,
            fault_factor: 1.0,
            ideal_cursor: 0,
            fault_cursor: 0,
            equipment_cursor: 0,
            selected_ideal: None,
            delivered: DeliveredEnergy::default(),
        }
    }

    fn next_zone(&mut self) {
        let zone_cursor = self.zone_cursor + 1;
        *self = Self { stage: BalanceStage::Clear, zone_cursor, ..Self::new() };
    }
}

/// 🌦️ The three hourly weather records one hour interpolates between.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) struct HourWeather {
    pub(crate) previous: WeatherRecord,
    pub(crate) current: WeatherRecord,
    pub(crate) next: WeatherRecord,
}

/// ⏱️ Cursor-owned execution state for one hourly timestep.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) struct TimestepWork {
    stage: TimestepStage,
    context: ScheduleContext,
    weather: HourWeather,
    date: SimDate,
    hour: f64,
    dt_s: f64,
    substeps: u32,
    substep_cursor: u32,
    zone_steps: u32,
    step_weather: TimestepWeather,
    sky: SkyState,
    sun_alt: f64,
    sun_az: f64,
    zone_cursor: usize,
    secondary_cursor: usize,
    zone_loads: Vec<ZoneLoads>,
    zone_preparation: Option<ZonePreparationWork>,
    balance: Option<BalanceWork>,
    plant_work: Option<PlantWork>,
    battery_work: Option<BatteryWork>,
    schedule_lookup: Option<ScheduleLookupWork>,
    pv_generation_w: f64,
}

/// 🧱️ Persistent backing constructor for one timestep.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) struct TimestepBuilder {
    stage: u8,
    cursor: usize,
    context: ScheduleContext,
    weather: HourWeather,
    date: SimDate,
    hour: f64,
    zone_loads: Vec<ZoneLoads>,
}

#[cfg(test)]
pub(crate) const P7C1_TIMESTEP_BUILDER_STAGES: [u8; 3] = [0, 1, 2];

impl TimestepBuilder {
    pub(crate) fn retained_wire_signature(&self) -> [u64; 6] {
        [self.stage as u64, self.cursor as u64, self.zone_loads.len() as u64, self.zone_loads.capacity() as u64, self.date.day_of_year() as u64, self.hour.to_bits()]
    }

    pub(crate) fn new(weather: HourWeather, date: SimDate, hour: f64) -> Self {
        let current = weather.current;
        Self {
            stage: 0,
            cursor: 0,
            context: ScheduleContext { year: date.year, month: date.month, day: date.day, hour: current.hour, day_of_week: date.day_of_week(), timestep_index: hour as u32, is_dst: false },
            weather,
            date,
            hour,
            zone_loads: Vec::new(),
        }
    }

    pub(crate) fn step(&mut self, model: &Model, pre: &PrecomputedModel) -> Result<Option<TimestepWork>, Error> {
        let zones = pre.zone_order.len();
        match self.stage {
            0 => {
                self.zone_loads.try_reserve_exact(zones).map_err(|_| Error::severe("energy timestep zone-load backing rejected"))?;
                self.stage = 1;
            }
            1 => {
                if self.cursor < zones {
                    self.zone_loads.push(ZoneLoads::default());
                    self.cursor += 1;
                } else {
                    self.stage = 2;
                }
            }
            _ => {
                let midpoint = sun_direction(model.site.latitude_deg, model.site.longitude_deg, model.site.time_zone_hours, self.date.day_of_year(), self.weather.current.hour as f64 + 0.5);
                let zone_steps = (3600.0 / pre.zone_timestep_s).round().max(1.0) as u32;
                let substeps = (3600.0 / pre.balance_step_s()).round().max(1.0) as u32;
                return Ok(Some(TimestepWork {
                    stage: TimestepStage::Zone,
                    context: self.context,
                    weather: self.weather,
                    date: self.date,
                    hour: self.hour,
                    dt_s: 3600.0 / substeps as f64,
                    substeps,
                    substep_cursor: 0,
                    zone_steps,
                    step_weather: TimestepWeather::interpolate(&self.weather.previous, &self.weather.current, &self.weather.next, 1, zone_steps),
                    sky: SkyState::default(),
                    sun_alt: midpoint[2].clamp(-1.0, 1.0).asin().to_degrees(),
                    sun_az: midpoint[0].atan2(midpoint[1]).to_degrees().rem_euclid(360.0),
                    zone_cursor: 0,
                    secondary_cursor: 0,
                    zone_loads: std::mem::take(&mut self.zone_loads),
                    zone_preparation: None,
                    balance: None,
                    plant_work: None,
                    battery_work: None,
                    schedule_lookup: None,
                    pv_generation_w: 0.0,
                }));
            }
        }
        Ok(None)
    }

    #[cfg(test)]
    pub(crate) fn stage_for_gate(&self) -> u8 {
        self.stage
    }

    #[cfg(test)]
    pub(crate) fn set_stage_for_gate(&mut self, stage: u8) {
        self.stage = stage;
    }

    pub(crate) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 {
            return false;
        }
        self.zone_loads.pop().is_none()
    }
}

impl TimestepWork {
    pub(crate) fn retained_wire_signature(&self) -> [u64; 16] {
        [
            self.stage as u64,
            self.substep_cursor as u64,
            self.substeps as u64,
            self.zone_cursor as u64,
            self.secondary_cursor as u64,
            self.zone_loads.len() as u64,
            self.balance.as_ref().map_or(0, |work| work.stage as u64 + 1),
            self.balance.as_ref().map_or(0, |work| work.zone_cursor as u64),
            self.balance.as_ref().map_or(0, |work| work.face_cursor as u64),
            self.zone_preparation.is_some() as u64,
            self.plant_work.is_some() as u64,
            self.battery_work.is_some() as u64,
            self.schedule_lookup.is_some() as u64,
            self.zone_steps as u64,
            self.hour.to_bits(),
            self.pv_generation_w.to_bits(),
        ]
    }

    #[cfg(test)]
    pub(crate) fn new(model: &Model, pre: &PrecomputedModel, weather: HourWeather, date: SimDate, hour: f64) -> Self {
        let mut builder = TimestepBuilder::new(weather, date, hour);
        loop {
            if let Some(work) = builder.step(model, pre).expect("headless timestep admission") {
                return work;
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn stage(&self) -> TimestepStage {
        self.stage
    }

    #[cfg(test)]
    pub(crate) fn zone_preparation_stage(&self) -> Option<ZonePreparationStage> {
        self.zone_preparation.as_ref().map(|work| work.stage)
    }

    #[cfg(test)]
    pub(crate) fn balance_stage(&self) -> Option<BalanceStage> {
        self.balance.as_ref().map(|work| work.stage)
    }

    #[cfg(test)]
    pub(crate) fn plant_stage(&self) -> Option<PlantStage> {
        self.plant_work.as_ref().map(|work| work.stage)
    }

    #[cfg(test)]
    pub(crate) fn schedule_lookup_stage(&self) -> Option<ScheduleLookupStage> {
        self.schedule_lookup.as_ref().map(|work| work.stage)
    }

    #[cfg(test)]
    pub(crate) fn set_stage_for_gate(&mut self, stage: TimestepStage) {
        self.stage = stage;
    }

    #[cfg(test)]
    pub(crate) fn set_zone_preparation_stage_for_gate(&mut self, stage: ZonePreparationStage) -> bool {
        let Some(work) = self.zone_preparation.as_mut() else { return false };
        work.stage = stage;
        true
    }

    #[cfg(test)]
    pub(crate) fn set_balance_stage_for_gate(&mut self, stage: BalanceStage) -> bool {
        let Some(work) = self.balance.as_mut() else { return false };
        work.stage = stage;
        true
    }

    #[cfg(test)]
    pub(crate) fn set_plant_stage_for_gate(&mut self, stage: PlantStage) -> bool {
        let Some(work) = self.plant_work.as_mut() else { return false };
        work.stage = stage;
        true
    }

    #[cfg(test)]
    pub(crate) fn set_schedule_lookup_stage_for_gate(&mut self, stage: ScheduleLookupStage) -> bool {
        let Some(work) = self.schedule_lookup.as_mut() else { return false };
        work.stage = stage;
        true
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.stage == TimestepStage::Complete
    }

    pub(crate) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 {
            return false;
        }
        if self.zone_loads.pop().is_some() {
            return false;
        }
        self.zone_preparation = None;
        self.balance = None;
        self.plant_work = None;
        self.battery_work = None;
        true
    }

    pub(crate) fn step(&mut self, model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel) {
        match self.stage {
            TimestepStage::Zone => self.step_zone_preparation(model, config, pre, state),
            TimestepStage::Balance => self.step_balance(model, pre, state),
            TimestepStage::ZoneCommit => self.step_zone_commit(pre, state),
            TimestepStage::SecondaryPlant => self.step_plant_bounded(model, pre, state),
            TimestepStage::SecondaryPv => self.step_pv(model, state),
            TimestepStage::SecondaryBattery => self.step_battery_bounded(model, pre, state),
            TimestepStage::SecondaryServiceHotWater => self.step_service_hot_water(model, config, state),
            TimestepStage::SecondaryRefrigeration => self.step_refrigeration(model, config, state),
            TimestepStage::SecondaryWater => self.step_water(model, config, state),
            TimestepStage::Complete => {}
        }
    }

    fn step_zone_preparation(&mut self, model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(zone) = model.zones.get(self.zone_cursor) else {
            self.zone_cursor = 0;
            self.substep_cursor = 0;
            self.balance = Some(BalanceWork::new());
            self.stage = TimestepStage::Balance;
            return;
        };
        if self.zone_preparation.is_none() {
            if self.zone_cursor == 0 {
                state.delivered_total = DeliveredEnergy::default();
            }
            let geometry = pre.zone_geometry.get(&zone.id).cloned().unwrap_or_default();
            let zone_state = state.zones.get(&zone.id);
            let zone_temp_c = zone_state.map_or(self.weather.current.dry_bulb_c, |value| value.air.temp_c);
            let zone_humidity_ratio = zone_state.map_or(self.step_weather.humidity_ratio, |value| value.air.humidity_ratio);
            let setpoints = pre.default_setpoints.get(&zone.id).copied().unwrap_or_default();
            self.zone_preparation = Some(ZonePreparationWork {
                stage: ZonePreparationStage::Begin,
                cursor: 0,
                zone_index: self.zone_cursor,
                zone_id: zone.id,
                floor_area_m2: geometry.floor_area_m2,
                zone_temp_c,
                zone_humidity_ratio,
                daylight_target_lux: 0.0,
                daylight_transmittance: 0.0,
                daylight_area_m2: 0.0,
                lighting_dim: 1.0,
                internal_gain: GainDecomposition::default(),
                infiltration_flow_m3_s: 0.0,
                airflow_zone_node: None,
                airflow_flow_m3_s: 0.0,
                mechanical_flow_m3_s: 0.0,
                heating_setpoint_c: setpoints.heating_c,
                cooling_setpoint_c: setpoints.cooling_c,
                thermostat_schedule: 0,
                thermostat: ThermostatSpec {
                    heating_setpoint_c: setpoints.heating_c,
                    cooling_setpoint_c: setpoints.cooling_c,
                    heating_throttle_range_k: setpoints.heating_throttle_k,
                    cooling_throttle_range_k: setpoints.cooling_throttle_k,
                    min_heating_setpoint_c: 10.0,
                    max_cooling_setpoint_c: 35.0,
                },
                humidistat: None,
                controlled: false,
            });
            return;
        }
        let work = self.zone_preparation.as_mut().expect("zone preparation authority exists");
        match work.stage {
            ZonePreparationStage::Begin => advance_zone_preparation(work, ZonePreparationStage::DaylightConfig),
            ZonePreparationStage::DaylightConfig => {
                if let Some(daylight) = model.daylight_zones.get(work.cursor) {
                    if daylight.zone_id == work.zone_id && work.daylight_target_lux == 0.0 {
                        work.daylight_target_lux = daylight.illuminance_target_lux;
                        work.daylight_transmittance = daylight.window_transmittance;
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::DaylightArea);
                }
            }
            ZonePreparationStage::DaylightArea => {
                if let Some(fenestration) = model.fenestrations.get(work.cursor) {
                    let belongs_to_zone = pre.surface_indices.get(&fenestration.surface_id).and_then(|index| model.surfaces.get(*index)).is_some_and(|surface| surface.zone_id == work.zone_id);
                    if belongs_to_zone {
                        work.daylight_area_m2 += fenestration.area_m2;
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::DaylightEvaluate);
                }
            }
            ZonePreparationStage::DaylightEvaluate => {
                if work.daylight_target_lux > 0.0 {
                    let factor = simplified_daylight_factor(work.daylight_area_m2, work.floor_area_m2, work.daylight_transmittance);
                    let lux = reference_point_illuminance_lux(self.weather.current.diffuse_horizontal_irradiance_w_m2 * 120.0, self.weather.current.direct_normal_irradiance_w_m2 * 120.0, self.sun_alt.max(0.0) / 90.0, work.daylight_transmittance, factor, 1.0);
                    work.lighting_dim = lighting_dimming_fraction(lux, work.daylight_target_lux, 0.1);
                }
                advance_zone_preparation(work, ZonePreparationStage::People);
            }
            ZonePreparationStage::People => {
                if let Some(person) = model.people.get(work.cursor) {
                    if person.zone_id == work.zone_id {
                        let Some(occupancy) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, person.schedule_id, &self.context) else {
                            return;
                        };
                        work.internal_gain = work.internal_gain.add(&compute_people_gain_w(person.people_per_area * work.floor_area_m2 * occupancy, ActivityLevel::OfficeWork, 1.0, person.radiant_fraction));
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Lighting);
                }
            }
            ZonePreparationStage::Lighting => {
                if let Some(light) = model.lighting.get(work.cursor) {
                    if light.zone_id == work.zone_id {
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, light.schedule_id, &self.context) else {
                            return;
                        };
                        let fraction = schedule * work.lighting_dim;
                        let power = dimmed_lighting_power_w(light.watts_per_area * work.floor_area_m2, fraction);
                        work.internal_gain = work.internal_gain.add(&compute_lighting_gain_w(power / work.floor_area_m2.max(1.0), work.floor_area_m2, 1.0, light.radiant_fraction, light.return_air_fraction));
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Equipment);
                }
            }
            ZonePreparationStage::Equipment => {
                if let Some(equipment) = model.equipment.get(work.cursor) {
                    if equipment.zone_id == work.zone_id {
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, equipment.schedule_id, &self.context) else {
                            return;
                        };
                        work.internal_gain = work.internal_gain.add(&compute_equipment_gain_w(equipment.watts_per_area, work.floor_area_m2, schedule, equipment.radiant_fraction, equipment.latent_fraction));
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Infiltration);
                }
            }
            ZonePreparationStage::Infiltration => {
                if let Some(infiltration) = model.infiltrations.get(work.cursor) {
                    if infiltration.zone_id == work.zone_id {
                        let Some(schedule_factor) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, infiltration.schedule_id, &self.context) else {
                            return;
                        };
                        let geometry = pre.zone_geometry.get(&work.zone_id).cloned().unwrap_or_default();
                        let specification = InfiltrationSpec {
                            method: infiltration.method,
                            schedule_factor,
                            ach: infiltration.design_flow_ach,
                            flow_per_exterior_area_m3_s_m2: infiltration.flow_per_exterior_area_m3_s_m2,
                            effective_leakage_area_m2: infiltration.effective_leakage_area_m2,
                            discharge_coefficient: infiltration.discharge_coefficient,
                            constant_coefficient: infiltration.constant_term_coefficient,
                            temperature_coefficient: infiltration.temperature_term_coefficient,
                            velocity_coefficient: infiltration.velocity_term_coefficient,
                            velocity_squared_coefficient: infiltration.velocity_squared_term_coefficient,
                            stack_height_m: infiltration.stack_height_m,
                        };
                        work.infiltration_flow_m3_s += infiltration_flow_m3_s(&specification, zone.volume_m3, geometry.exterior_area_m2, self.weather.current.dry_bulb_c, work.zone_temp_c, self.weather.current.wind_speed_m_s, self.weather.current.atmospheric_pressure_pa);
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::AirflowNode);
                }
            }
            ZonePreparationStage::AirflowNode => {
                let node = model.airflow_network.as_ref().and_then(|network| network.zone_node_ids.get(work.cursor));
                if let Some((zone_id, node_id)) = node {
                    if *zone_id == work.zone_id && work.airflow_zone_node.is_none() {
                        work.airflow_zone_node = Some(*node_id);
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::AirflowLink);
                }
            }
            ZonePreparationStage::AirflowLink => {
                let link = model.airflow_network.as_ref().and_then(|network| network.link_ids.get(work.cursor));
                if link.is_some() {
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::AirflowSolve);
                }
            }
            ZonePreparationStage::AirflowSolve => {
                if work.cursor < config.tolerances.max_iterations.max(1) as usize {
                    if work.cursor == 0 && work.airflow_zone_node.is_some() {
                        let stack = (work.zone_temp_c - self.weather.current.dry_bulb_c).abs().sqrt();
                        work.airflow_flow_m3_s = 0.01 * (self.weather.current.wind_speed_m_s + stack).powf(0.65);
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::MechanicalVentilation);
                }
            }
            ZonePreparationStage::MechanicalVentilation => {
                if let Some(ventilation) = model.mechanical_ventilations.get(work.cursor) {
                    if ventilation.zone_id == work.zone_id {
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, ventilation.schedule_id, &self.context) else {
                            return;
                        };
                        work.mechanical_flow_m3_s += ventilation.design_flow_m3_s * schedule;
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Thermostat);
                }
            }
            ZonePreparationStage::Thermostat => {
                if let Some(thermostat) = model.thermostats.get(work.cursor) {
                    if thermostat.zone_id == work.zone_id {
                        work.controlled = true;
                        if work.thermostat_schedule == 0 {
                            let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, thermostat.heating_setpoint_schedule_id, &self.context) else {
                                return;
                            };
                            work.heating_setpoint_c = schedule;
                            work.thermostat_schedule = 1;
                            return;
                        }
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, thermostat.cooling_setpoint_schedule_id, &self.context) else {
                            return;
                        };
                        work.cooling_setpoint_c = schedule;
                        work.thermostat_schedule = 0;
                    }
                    work.cursor += 1;
                } else {
                    work.thermostat.heating_setpoint_c = work.heating_setpoint_c;
                    work.thermostat.cooling_setpoint_c = work.cooling_setpoint_c;
                    advance_zone_preparation(work, ZonePreparationStage::Humidistat);
                }
            }
            ZonePreparationStage::Humidistat => {
                if let Some(humidistat) = model.humidistats.get(work.cursor) {
                    if humidistat.zone_id == work.zone_id && work.humidistat.is_none() {
                        work.humidistat = Some(HumidistatSpec {
                            humidifying_setpoint_rh: 0.4,
                            dehumidifying_setpoint_rh: 0.6,
                            humidifying_throttle_range: humidistat.humidifying_throttle_range,
                            dehumidifying_throttle_range: humidistat.dehumidifying_throttle_range,
                        });
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Publish);
                }
            }
            ZonePreparationStage::Publish => {
                let zone_index = pre.zone_indices.get(&work.zone_id).copied().unwrap_or(work.zone_index);
                self.zone_loads[zone_index] = ZoneLoads {
                    convective_w: work.internal_gain.convective_w,
                    radiant_w: work.internal_gain.radiant_w,
                    latent_w: work.internal_gain.latent_w,
                    outdoor_air_m3_s: work.infiltration_flow_m3_s + work.airflow_flow_m3_s + work.mechanical_flow_m3_s,
                    heating_setpoint_c: work.heating_setpoint_c,
                    cooling_setpoint_c: work.cooling_setpoint_c,
                    floor_area_m2: work.floor_area_m2,
                    controlled: work.controlled,
                };
                self.zone_preparation = None;
                self.zone_cursor += 1;
            }
        }
    }

    fn step_balance(&mut self, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let mut work = self.balance.take().unwrap_or_else(BalanceWork::new);
        let finished = self.balance_unit(&mut work, model, pre, state);
        if finished {
            self.zone_cursor = 0;
            self.stage = TimestepStage::ZoneCommit;
        } else {
            self.balance = Some(work);
        }
    }

    fn balance_unit(&mut self, work: &mut BalanceWork, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) -> bool {
        match work.stage {
            BalanceStage::Weather => {
                let zone_step = self.substep_cursor * self.zone_steps / self.substeps.max(1) + 1;
                let weather = TimestepWeather::interpolate(&self.weather.previous, &self.weather.current, &self.weather.next, zone_step, self.zone_steps);
                let hour_of_day = self.weather.current.hour as f64 + zone_step as f64 / self.zone_steps as f64;
                let sun = sun_direction(model.site.latitude_deg, model.site.longitude_deg, model.site.time_zone_hours, self.date.day_of_year(), hour_of_day);
                self.sky = SkyState::new(sun, weather.beam_normal_w_m2, weather.diffuse_horizontal_w_m2, model.site.elevation_m, GROUND_REFLECTANCE);
                self.step_weather = weather;
                *work = BalanceWork { stage: BalanceStage::Clear, ..BalanceWork::new() };
            }
            BalanceStage::Clear => {
                let Some(zone_id) = pre.zone_order.get(work.zone_cursor).copied() else {
                    self.substep_cursor += 1;
                    if self.substep_cursor >= self.substeps {
                        return true;
                    }
                    *work = BalanceWork::new();
                    return false;
                };
                let faces = pre.enclosures.get(&zone_id).map_or(0, |enclosure| enclosure.faces.len());
                let solver = &mut state.solver;
                solver.inside_absorbed_w_m2[..faces].fill(0.0);
                solver.pane_absorbed_w_m2[..faces * MAX_PANES].fill(0.0);
                solver.inside_convection_w_m2k[..faces].fill(0.0);
                solver.reduction[..faces * 3].fill(0.0);
                if faces <= crate::precompute::EXACT_ENCLOSURE_FACES {
                    let order = faces + 1;
                    solver.matrix[..order * order].fill(0.0);
                    solver.rhs[..order].fill(0.0);
                }
                work.stage = BalanceStage::WindowSolar;
            }
            BalanceStage::WindowSolar => self.unit_window_solar(work, pre, state),
            BalanceStage::BeamPatches => self.unit_beam_patch(work, pre, state),
            BalanceStage::DiffuseSolar => {
                let zone_id = pre.zone_order[work.zone_cursor];
                let Some(enclosure) = pre.enclosures.get(&zone_id) else {
                    work.stage = BalanceStage::Faces;
                    return false;
                };
                let flux = work.zone_diffuse_w * enclosure.diffuse_solar_multiplier;
                for (index, face) in enclosure.faces.iter().enumerate() {
                    match face {
                        EnclosureFace::Opaque(id) => state.solver.inside_absorbed_w_m2[index] += flux * pre.surfaces.get(id).map_or(0.0, |s| s.inside_solar_absorptance),
                        EnclosureFace::Window(id) => {
                            if let Some(window) = pre.windows.get(id) {
                                for pane in 0..window.glazing.panes {
                                    state.solver.pane_absorbed_w_m2[index * MAX_PANES + pane] += flux * window.glazing.diffuse_back_absorptance[pane];
                                }
                            }
                        }
                    }
                }
                work.stage = BalanceStage::Faces;
                work.face_cursor = 0;
            }
            BalanceStage::Faces => self.unit_face(work, model, pre, state),
            BalanceStage::Solve => {
                let loads = self.zone_loads[work.zone_cursor];
                self.unit_solve(work, model, loads, pre, state);
            }
            BalanceStage::IdealLoad => {
                let zone_id = pre.zone_order[work.zone_cursor];
                if let Some(ideal) = model.ideal_loads.get(work.ideal_cursor) {
                    if ideal.zone_id == zone_id {
                        work.selected_ideal = Some(work.ideal_cursor);
                        work.fault_cursor = 0;
                        work.fault_factor = 1.0;
                        work.stage = BalanceStage::Fault;
                    } else {
                        work.ideal_cursor += 1;
                    }
                } else {
                    work.stage = BalanceStage::ZoneEquipment;
                }
            }
            BalanceStage::Fault => {
                let ideal = &model.ideal_loads[work.selected_ideal.expect("selected ideal load")];
                if let Some(fault) = model.faults.get(work.fault_cursor) {
                    if fault.target_equipment_id == ideal.id {
                        work.fault_factor = 1.0 - pre.fault_severity.get(&ideal.id).copied().unwrap_or(fault.severity);
                        work.stage = BalanceStage::ApplyIdealLoad;
                    } else {
                        work.fault_cursor += 1;
                    }
                } else {
                    work.stage = BalanceStage::ApplyIdealLoad;
                }
            }
            BalanceStage::ApplyIdealLoad => {
                let ideal = &model.ideal_loads[work.selected_ideal.expect("selected ideal load")];
                let loads = self.zone_loads[work.zone_cursor];
                let zone_state = state.zones.get(&pre.zone_order[work.zone_cursor]);
                let output = ideal_loads_deliver(
                    &IdealLoadsInput {
                        zone_temp_c: zone_state.map_or(work.free_temp_c, |zone| zone.air.temp_c),
                        zone_humidity_ratio: zone_state.map_or(self.step_weather.humidity_ratio, |zone| zone.air.humidity_ratio),
                        outdoor_temp_c: self.step_weather.dry_bulb_c,
                        outdoor_humidity_ratio: self.step_weather.humidity_ratio,
                        heating_setpoint_c: loads.heating_setpoint_c,
                        cooling_setpoint_c: loads.cooling_setpoint_c,
                        zone_heating_demand_w: work.remaining_heating_w * work.fault_factor,
                        zone_cooling_demand_w: work.remaining_cooling_w * work.fault_factor,
                        occupancy: 1.0,
                        floor_area_m2: loads.floor_area_m2,
                    },
                    &IdealLoadsConfig {
                        max_heating_supply_air_temp_c: ideal.max_heating_supply_air_temp_c,
                        min_cooling_supply_air_temp_c: ideal.min_cooling_supply_air_temp_c,
                        max_heating_capacity_w: ideal.max_heating_capacity_w,
                        max_cooling_capacity_w: ideal.max_cooling_capacity_w,
                        outdoor_air_per_person_m3_s: ideal.outdoor_air_per_person_m3_s,
                        outdoor_air_per_area_m3_s_m2: ideal.outdoor_air_per_area_m3_s_m2,
                    },
                );
                work.delivered_w += output.sensible_heating_w - output.sensible_cooling_w;
                work.remaining_heating_w = (work.remaining_heating_w - output.sensible_heating_w).max(0.0);
                work.remaining_cooling_w = (work.remaining_cooling_w - output.sensible_cooling_w).max(0.0);
                work.delivered.heating_w += output.sensible_heating_w;
                work.delivered.cooling_w += output.sensible_cooling_w;
                work.ideal_cursor += 1;
                work.selected_ideal = None;
                work.stage = BalanceStage::IdealLoad;
            }
            BalanceStage::ZoneEquipment => {
                let zone_id = pre.zone_order[work.zone_cursor];
                if let Some(assignment) = model.zone_equipment.get(work.equipment_cursor) {
                    if assignment.zone_id == zone_id && (work.remaining_heating_w > 0.0 || work.remaining_cooling_w > 0.0) {
                        let equipment = match assignment.equipment_type {
                            crate::model::ZoneEquipmentType::FanCoil => ZoneEquipment::FanCoil {
                                heating: None,
                                cooling: None,
                                fan: crate::fans::Fan {
                                    fan_type: crate::fans::FanType::VariableVolume,
                                    max_flow_m3_s: 0.5,
                                    max_pressure_rise_pa: 500.0,
                                    motor_efficiency: 0.85,
                                    pressure_curve: PerformanceCurve::Constant(1.0),
                                    efficiency_curve: PerformanceCurve::Constant(0.7),
                                    part_load_curve: PerformanceCurve::Constant(1.0),
                                },
                                max_flow_m3_s: 0.5,
                            },
                            _ => ZoneEquipment::Baseboard { heating: crate::coils::HeatingCoil::Electric { capacity_w: assignment.heating_capacity_w, efficiency: 1.0 } },
                        };
                        let zone_state = state.zones.get(&zone_id);
                        let output = equipment.simulate(&ZoneEquipmentRequest {
                            zone_temperature_c: zone_state.map_or(work.free_temp_c, |zone| zone.air.temp_c),
                            zone_humidity_ratio: zone_state.map_or(self.step_weather.humidity_ratio, |zone| zone.air.humidity_ratio),
                            heating_load_w: work.remaining_heating_w,
                            cooling_load_w: work.remaining_cooling_w,
                            outdoor_temperature_c: self.step_weather.dry_bulb_c,
                            outdoor_humidity_ratio: self.step_weather.humidity_ratio,
                            outdoor_pressure_pa: self.step_weather.pressure_pa,
                            supply_air_temp_c: 16.0,
                            supply_air_humidity_ratio: self.step_weather.humidity_ratio,
                            supply_mass_flow_kg_s: 0.1,
                        });
                        let heating = output.delivered_heating_w.clamp(0.0, work.remaining_heating_w);
                        let cooling = output.delivered_cooling_w.clamp(0.0, work.remaining_cooling_w);
                        work.delivered_w += heating - cooling;
                        work.remaining_heating_w -= heating;
                        work.remaining_cooling_w -= cooling;
                        work.delivered.heating_w += heating;
                        work.delivered.cooling_w += cooling;
                        work.delivered.fan_w += output.fan_power_w;
                        work.delivered.compressor_w += output.compressor_power_w;
                        work.delivered.gas_w += output.gas_consumption_w;
                    }
                    work.equipment_cursor += 1;
                } else {
                    work.stage = BalanceStage::Settle;
                }
            }
            BalanceStage::Settle => self.unit_settle(work, model, pre, state),
            BalanceStage::Commit => self.unit_commit(work, model, pre, state),
        }
        false
    }

    fn unit_window_solar(&mut self, work: &mut BalanceWork, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let Some(enclosure) = pre.enclosures.get(&zone_id) else {
            work.stage = BalanceStage::BeamPatches;
            return;
        };
        let Some(face) = enclosure.faces.get(work.face_cursor).copied() else {
            work.stage = BalanceStage::BeamPatches;
            work.face_cursor = 0;
            work.back_cursor = 0;
            return;
        };
        let index = work.face_cursor;
        work.face_cursor += 1;
        let EnclosureFace::Window(id) = face else { return };
        let Some(window) = pre.windows.get(&id) else { return };
        if !window.sun_exposed || self.sky.sun[2] <= 0.0 {
            return;
        }
        let cosine = incidence_cosine(window.normal, self.sky.sun);
        let lit = if cosine <= 0.0 { 0.0 } else if window.casters.is_empty() { 1.0 } else { sunlit_fraction(&window.polygon, window.normal, std::iter::empty(), window.casters.iter().map(|caster| pre.casters[*caster].as_slice()), self.sky.sun) };
        state.solver.reduction[index * 3] = lit;
        let incident = self.sky.incident(window.normal, lit, window.sky_isotropic_ratio, window.sky_horizon_ratio);
        let glazing = &window.glazing;
        let mut absorbed_w_m2 = 0.0;
        for pane in 0..glazing.panes {
            let pane_w_m2 = incident.beam_w_m2 * glazing.beam_front_absorptance(pane, cosine) + incident.diffuse_w_m2() * glazing.diffuse_front_absorptance[pane];
            state.solver.pane_absorbed_w_m2[index * MAX_PANES + pane] += pane_w_m2;
            absorbed_w_m2 += pane_w_m2;
        }
        let transmitted_w = incident.diffuse_w_m2() * glazing.diffuse_transmittance * window.area_m2;
        // 🧱️ This window's own shortwave ledger for the timestep: everything that passes through the
        // glazing into the zone (diffuse here, beam through `beam_transmittance` — the same product
        // `unit_beam_patch` redistributes onto the back faces) and everything the panes keep.
        if state.warmup_complete {
            let beam_transmitted_w = incident.beam_w_m2 * glazing.beam_transmittance(cosine) * window.area_m2;
            let dt = self.dt_s;
            if let Some(row) = state.per_surface.window_mut(id) {
                row.accumulate_solar_transmitted(transmitted_w + beam_transmitted_w, dt);
                row.accumulate_solar_absorbed(absorbed_w_m2 * window.area_m2, dt);
            }
        }
        let n = enclosure.faces.len();
        match &enclosure.radiation {
            EnclosureRadiation::Exchange { view_factors, .. } => {
                for (back_index, back) in enclosure.faces.iter().enumerate() {
                    let part = transmitted_w * view_factors[index * n + back_index];
                    if part <= 0.0 {
                        continue;
                    }
                    match back {
                        EnclosureFace::Opaque(back_id) => {
                            let Some(surface) = pre.surfaces.get(back_id) else { continue };
                            if surface.area_m2 > 0.0 {
                                state.solver.inside_absorbed_w_m2[back_index] += part * surface.inside_solar_absorptance / surface.area_m2;
                            }
                            if state.warmup_complete {
                                let dt = self.dt_s;
                                if let Some(row) = state.per_surface.opaque_mut(*back_id) {
                                    row.accumulate_solar_absorbed(part * surface.inside_solar_absorptance, dt);
                                }
                            }
                            work.zone_diffuse_w += part * (1.0 - surface.inside_solar_absorptance);
                        }
                        EnclosureFace::Window(back_id) => {
                            let Some(other) = pre.windows.get(back_id) else { continue };
                            let mut absorbed = 0.0;
                            for pane in 0..other.glazing.panes {
                                state.solver.pane_absorbed_w_m2[back_index * MAX_PANES + pane] += part * other.glazing.diffuse_back_absorptance[pane] / other.area_m2;
                                absorbed += other.glazing.diffuse_back_absorptance[pane];
                            }
                            if state.warmup_complete {
                                let dt = self.dt_s;
                                if let Some(row) = state.per_surface.window_mut(*back_id) {
                                    row.accumulate_solar_absorbed(part * absorbed, dt);
                                }
                            }
                            work.zone_diffuse_w += part * other.glazing.diffuse_back_reflectance;
                        }
                    }
                }
            }
            EnclosureRadiation::MeanRadiant { .. } => work.zone_diffuse_w += transmitted_w,
        }
    }

    fn unit_beam_patch(&mut self, work: &mut BalanceWork, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let Some(enclosure) = pre.enclosures.get(&zone_id) else {
            work.stage = BalanceStage::DiffuseSolar;
            return;
        };
        let Some(face) = enclosure.faces.get(work.face_cursor).copied() else {
            work.stage = BalanceStage::DiffuseSolar;
            work.face_cursor = 0;
            return;
        };
        let lit = state.solver.reduction[work.face_cursor * 3];
        let window = match face {
            EnclosureFace::Window(id) => pre.windows.get(&id).filter(|window| window.sun_exposed && lit > 0.0 && self.sky.beam_normal_w_m2 > 0.0 && incidence_cosine(window.normal, self.sky.sun) > 0.0),
            EnclosureFace::Opaque(_) => None,
        };
        let Some(window) = window else {
            work.face_cursor += 1;
            work.back_cursor = 0;
            return;
        };
        let Some(back) = enclosure.faces.get(work.back_cursor).copied() else {
            work.face_cursor += 1;
            work.back_cursor = 0;
            return;
        };
        let back_index = work.back_cursor;
        work.back_cursor += 1;
        let sun = self.sky.sun;
        let cosine = incidence_cosine(window.normal, sun);
        let beam_w_m2 = self.sky.beam_normal_w_m2 * window.glazing.beam_transmittance(cosine) * cosine * lit;
        match back {
            EnclosureFace::Opaque(id) => {
                let Some(surface) = pre.surfaces.get(&id) else { return };
                let mut overlap = beam_overlap_m2(&window.polygon, window.normal, &surface.polygon, surface.normal, sun);
                for hosted in &surface.windows {
                    if let Some(opening) = pre.windows.get(hosted) {
                        overlap -= beam_overlap_m2(&window.polygon, window.normal, &opening.polygon, opening.normal, sun);
                    }
                }
                let absorbed_w = beam_w_m2 * overlap.max(0.0);
                if surface.area_m2 > 0.0 {
                    state.solver.inside_absorbed_w_m2[back_index] += absorbed_w * surface.inside_solar_absorptance / surface.area_m2;
                }
                if state.warmup_complete {
                    let dt = self.dt_s;
                    if let Some(row) = state.per_surface.opaque_mut(id) {
                        row.accumulate_solar_absorbed(absorbed_w * surface.inside_solar_absorptance, dt);
                    }
                }
                work.zone_diffuse_w += absorbed_w * (1.0 - surface.inside_solar_absorptance);
            }
            EnclosureFace::Window(id) => {
                let Some(other) = pre.windows.get(&id) else { return };
                let arriving_w = beam_w_m2 * beam_overlap_m2(&window.polygon, window.normal, &other.polygon, other.normal, sun);
                if arriving_w <= 0.0 {
                    return;
                }
                let back_cosine = incidence_cosine(other.normal, sun).abs();
                let mut absorbed = 0.0;
                for pane in 0..other.glazing.panes {
                    let fraction = other.glazing.beam_back_absorptance(pane, back_cosine);
                    absorbed += fraction;
                    state.solver.pane_absorbed_w_m2[back_index * MAX_PANES + pane] += arriving_w * fraction / other.area_m2;
                }
                if state.warmup_complete {
                    let dt = self.dt_s;
                    if let Some(row) = state.per_surface.window_mut(id) {
                        row.accumulate_solar_absorbed(arriving_w * absorbed, dt);
                    }
                }
                work.zone_diffuse_w += arriving_w * (1.0 - absorbed - other.glazing.beam_back_transmittance(back_cosine)).max(0.0);
            }
        }
    }

    fn unit_face(&mut self, work: &mut BalanceWork, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let Some(enclosure) = pre.enclosures.get(&zone_id) else {
            work.stage = BalanceStage::Solve;
            return;
        };
        let n = enclosure.faces.len();
        let Some(face) = enclosure.faces.get(work.face_cursor).copied() else {
            work.stage = BalanceStage::Solve;
            return;
        };
        let index = work.face_cursor;
        work.face_cursor += 1;
        let loads = self.zone_loads[work.zone_cursor];
        let air_c = state.zones.get(&zone_id).map_or(self.step_weather.dry_bulb_c, |zone| zone.air.temp_c);
        let humidity_ratio = state.zones.get(&zone_id).map_or(self.step_weather.humidity_ratio, |zone| zone.air.humidity_ratio);
        let weather = self.step_weather;
        let dt = self.dt_s;
        let radiant_w_m2 = if enclosure.radiant_weight_total_m2 > 0.0 { loads.radiant_w * enclosure.emissivities[index] / enclosure.radiant_weight_total_m2 } else { 0.0 };
        let nodes = state.solver.nodes;
        let (inside_c, diagonal, rhs, inside_h, area) = match face {
            EnclosureFace::Opaque(id) => {
                let (Some(surface), Some(surface_state)) = (pre.surfaces.get(&id), state.surfaces.get(&id)) else { return };
                let chain = &surface.chain;
                let count = chain.nodes();
                let outside_c = surface_state.temperatures_c[0];
                let (h_outside, q_outside) = match surface.boundary {
                    OutsideBoundary::OutdoorAir => {
                        let cosine = incidence_cosine(surface.normal, self.sky.sun);
                        let solar_w_m2 = if surface.sun_exposed && self.sky.sun[2] > 0.0 {
                            let openings = surface.windows.iter().filter_map(|window| pre.windows.get(window)).map(|window| window.polygon.as_slice());
                            let lit = if cosine <= 0.0 { 0.0 } else if surface.casters.is_empty() { 1.0 } else { sunlit_fraction(&surface.polygon, surface.normal, openings, surface.casters.iter().map(|caster| pre.casters[*caster].as_slice()), self.sky.sun) };
                            self.sky.incident(surface.normal, lit, surface.sky_isotropic_ratio, surface.sky_horizon_ratio).total_w_m2()
                        } else {
                            0.0
                        };
                        let wet = weather.raining && surface.wind_exposed;
                        let exterior_c = if wet { weather.wet_bulb_c } else { weather.dry_bulb_c };
                        let wind = if surface.wind_exposed { wind_speed_at_height(weather.wind_speed_m_s, surface.centroid_height_m) } else { 0.0 };
                        let h_convection = if wet { 1000.0 } else { exterior_convection_w_m2k(outside_c, weather.dry_bulb_c, surface.normal[2], wind, is_windward(surface.normal[2], surface.azimuth_deg, weather.wind_direction_deg), surface.roughness_multiplier) };
                        let (h_sky, h_air, h_ground) = exterior_radiation_w_m2k(outside_c, weather.dry_bulb_c, weather.sky_temperature_c, surface.outside_emissivity, surface.normal[2]);
                        // 🧱️ The outside face's own shortwave gain for this timestep, the same product
                        // that drives `q_outside` below.
                        if state.warmup_complete {
                            let absorbed_w = surface.outside_solar_absorptance * solar_w_m2 * surface.area_m2;
                            if let Some(row) = state.per_surface.opaque_mut(id) {
                                row.accumulate_solar_absorbed(absorbed_w, dt);
                            }
                        }
                        (h_convection + h_air + h_sky + h_ground, (h_convection + h_air) * exterior_c + h_sky * weather.sky_temperature_c + h_ground * weather.dry_bulb_c + surface.outside_solar_absorptance * solar_w_m2)
                    }
                    OutsideBoundary::Ground => (FIXED_TEMPERATURE_CONDUCTANCE_W_M2K, FIXED_TEMPERATURE_CONDUCTANCE_W_M2K * GroundTemperatureModel::Monthly { temperatures_c: model.ground_temperature.building_surface_c }.temperature_c(self.date.day_of_year())),
                    OutsideBoundary::OtherSideTemperature => (FIXED_TEMPERATURE_CONDUCTANCE_W_M2K, FIXED_TEMPERATURE_CONDUCTANCE_W_M2K * air_c),
                    OutsideBoundary::Adiabatic => (0.0, 0.0),
                    OutsideBoundary::Interzone(partner) => {
                        let partner_c = state.surfaces.get(&partner).map_or(air_c, SurfaceState::inside_temp_c);
                        (FIXED_TEMPERATURE_CONDUCTANCE_W_M2K, FIXED_TEMPERATURE_CONDUCTANCE_W_M2K * partner_c)
                    }
                };
                let offset = index * nodes;
                let (a, b) = (&mut state.solver.chain_a[offset..offset + count - 1], &mut state.solver.chain_b[offset..offset + count - 1]);
                eliminate_chain(&chain.conductance_w_m2k, &chain.capacitance_j_m2k, &surface_state.temperatures_c, |_| 0.0, dt, h_outside, q_outside, a, b);
                let inside_c = surface_state.inside_temp_c();
                let inside_h = interior_convection_w_m2k(inside_c, air_c, surface.normal[2]);
                let storage = chain.capacitance_j_m2k[count - 1] / dt;
                let last = chain.conductance_w_m2k[count - 2];
                let diagonal = storage + last * (1.0 - b[count - 2]) + inside_h;
                let rhs = storage * inside_c + last * a[count - 2] + state.solver.inside_absorbed_w_m2[index] + radiant_w_m2;
                (inside_c, diagonal, rhs, inside_h, surface.area_m2)
            }
            EnclosureFace::Window(id) => {
                let (Some(window), Some(window_state)) = (pre.windows.get(&id), state.windows.get(&id)) else { return };
                let glazing = &window.glazing;
                let count = glazing.faces();
                let faces_c = window_state.face_temperatures_c;
                let ratio = window.coefficient_adjustment;
                let wet = weather.raining && window.wind_exposed;
                let exterior_c = if wet { weather.wet_bulb_c } else { weather.dry_bulb_c };
                let wind = if window.wind_exposed { wind_speed_at_height(weather.wind_speed_m_s, window.centroid_height_m) } else { 0.0 };
                let h_convection = ratio * if wet { 1000.0 } else { exterior_convection_w_m2k(faces_c[0], weather.dry_bulb_c, window.normal[2], wind, is_windward(window.normal[2], window.azimuth_deg, weather.wind_direction_deg), 1.0) };
                let view_sky = 0.5 * (1.0 + window.normal[2]);
                let split = view_sky.sqrt();
                let ambient_emission = STEFAN_BOLTZMANN * (exterior_c + KELVIN).powi(4);
                let incoming = view_sky * (split * STEFAN_BOLTZMANN * (weather.sky_temperature_c + KELVIN).powi(4) + (1.0 - split) * ambient_emission) + (1.0 - view_sky) * ambient_emission;
                let outside_k = faces_c[0] + KELVIN;
                let emissivity = glazing.emissivity_front[0];
                let h_linear = 4.0 * emissivity * STEFAN_BOLTZMANN * outside_k.powi(3);
                let h_outside = h_convection + h_linear;
                let q_outside = h_convection * exterior_c + emissivity * incoming + 3.0 * emissivity * STEFAN_BOLTZMANN * outside_k.powi(4) - h_linear * KELVIN;
                let mut conductance = [0.0; 2 * MAX_PANES];
                for pane in 0..glazing.panes {
                    conductance[2 * pane] = glazing.pane_conductance_w_m2k[pane];
                    if pane + 1 < glazing.panes {
                        let (outer_k, inner_k) = (faces_c[2 * pane + 1] + KELVIN, faces_c[2 * pane + 2] + KELVIN);
                        let (e1, e2) = (glazing.emissivity_back[pane], glazing.emissivity_front[pane + 1]);
                        let effective = e1 * e2 / (1.0 - (1.0 - e1) * (1.0 - e2));
                        conductance[2 * pane + 1] = gap_conductance_w_m2k(outer_k, inner_k, glazing.gap_width_m[pane], window.height_m, window.tilt_deg, glazing.gap_gas[pane]) + STEFAN_BOLTZMANN * effective * (outer_k * outer_k + inner_k * inner_k) * (outer_k + inner_k);
                    }
                }
                let sources = |node: usize| state_pane_source(&state.solver.pane_absorbed_w_m2, index, node);
                let capacitance = [0.0; 2 * MAX_PANES];
                let offset = index * nodes;
                let mut a = [0.0; 2 * MAX_PANES];
                let mut b = [0.0; 2 * MAX_PANES];
                eliminate_chain(&conductance[..count - 1], &capacitance[..count], &faces_c[..count], sources, dt, h_outside, q_outside, &mut a[..count - 1], &mut b[..count - 1]);
                state.solver.chain_a[offset..offset + count - 1].copy_from_slice(&a[..count - 1]);
                state.solver.chain_b[offset..offset + count - 1].copy_from_slice(&b[..count - 1]);
                let inside_c = faces_c[count - 1];
                let inside_h = ratio * glazing_interior_convection_w_m2k(inside_c, air_c, humidity_ratio, weather.pressure_pa, window.height_m, window.tilt_deg);
                let last = conductance[count - 2];
                let diagonal = last * (1.0 - b[count - 2]) + inside_h;
                let rhs = last * a[count - 2] + state_pane_source(&state.solver.pane_absorbed_w_m2, index, count - 1) + radiant_w_m2;
                (inside_c, diagonal, rhs, inside_h, window.area_m2)
            }
        };
        state.solver.inside_convection_w_m2k[index] = inside_h;
        work.air_diagonal_w_k += area * inside_h;
        match &enclosure.radiation {
            EnclosureRadiation::Exchange { exchange_factors, .. } => {
                let order = n + 1;
                let inside_k = inside_c + KELVIN;
                let mut diagonal = diagonal;
                for (other_index, other) in enclosure.faces.iter().enumerate() {
                    if other_index == index {
                        continue;
                    }
                    let other_k = face_inside_c(state, pre, *other, air_c) + KELVIN;
                    let h_radiation = STEFAN_BOLTZMANN * exchange_factors[index * n + other_index] * (inside_k * inside_k + other_k * other_k) * (inside_k + other_k);
                    diagonal += h_radiation;
                    state.solver.matrix[index * order + other_index] = -h_radiation;
                }
                state.solver.matrix[index * order + index] = diagonal;
                state.solver.matrix[index * order + n] = -inside_h;
                state.solver.matrix[n * order + index] = -area * inside_h;
                state.solver.rhs[index] = rhs;
            }
            EnclosureRadiation::MeanRadiant { participation } => {
                let mean_k = state.zones.get(&zone_id).map_or(air_c, |zone| zone.mean_radiant_temp_c) + KELVIN;
                let inside_k = inside_c + KELVIN;
                let h_radiation = participation[index] * (mean_k * mean_k + inside_k * inside_k) * (mean_k + inside_k);
                let total = diagonal + h_radiation;
                let (alpha, beta, gamma) = (rhs / total, inside_h / total, h_radiation / total);
                state.solver.reduction[index * 3] = alpha;
                state.solver.reduction[index * 3 + 1] = beta;
                state.solver.reduction[index * 3 + 2] = gamma;
                let sums = &mut work.mean_radiant_sums;
                sums[0] += area * inside_h * (1.0 - beta);
                sums[1] += area * inside_h * gamma;
                sums[2] += area * inside_h * alpha;
                sums[3] += area * h_radiation * (1.0 - gamma);
                sums[4] += area * h_radiation * beta;
                sums[5] += area * h_radiation * alpha;
            }
        }
    }

    fn unit_solve(&mut self, work: &mut BalanceWork, model: &Model, loads: ZoneLoads, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let zone = &model.zones[work.zone_cursor];
        let n = pre.enclosures.get(&zone_id).map_or(0, |enclosure| enclosure.faces.len());
        let exact = pre.enclosures.get(&zone_id).is_none_or(|enclosure| matches!(enclosure.radiation, EnclosureRadiation::Exchange { .. }));
        let weather = self.step_weather;
        let Some(zone_state) = state.zones.get(&zone_id) else { return };
        let capacity = zone_state.air.capacity_rate_w_k(zone.volume_m3 * zone.multiplier.max(1) as f64, weather.pressure_pa, self.dt_s);
        let (storage, history) = zone_state.air.storage_terms(capacity);
        let outdoor_capacity = loads.outdoor_air_m3_s * moist_air_density(weather.dry_bulb_c, weather.humidity_ratio, weather.pressure_pa) * moist_air_cp_j_per_kg_k(weather.humidity_ratio);
        let base_diagonal = storage + outdoor_capacity;
        work.air_rhs_w = history + outdoor_capacity * weather.dry_bulb_c + loads.convective_w;
        let solver = &mut state.solver;
        let order = n + 1;
        let sums = work.mean_radiant_sums;
        let free = if exact {
            solver.matrix[n * order + n] = base_diagonal + work.air_diagonal_w_k;
            solver.rhs[n] = work.air_rhs_w;
            work.air_diagonal_w_k = base_diagonal + work.air_diagonal_w_k;
            solver.scratch[..order * order].copy_from_slice(&solver.matrix[..order * order]);
            solver.solution[..order].copy_from_slice(&solver.rhs[..order]);
            if solve_dense_in_place(&mut solver.scratch[..order * order], &mut solver.solution[..order], order) { solver.solution[n] } else { zone_state.air.temp_c }
        } else {
            work.air_diagonal_w_k = base_diagonal + sums[0];
            let determinant = work.air_diagonal_w_k * sums[3] - sums[1] * sums[4];
            if determinant.abs() > 1e-12 { ((work.air_rhs_w + sums[2]) * sums[3] + sums[1] * sums[5]) / determinant } else { zone_state.air.temp_c }
        };
        work.free_temp_c = free;
        work.target_temp_c = if !loads.controlled {
            None
        } else if free < loads.heating_setpoint_c {
            Some(loads.heating_setpoint_c)
        } else if free > loads.cooling_setpoint_c {
            Some(loads.cooling_setpoint_c)
        } else {
            None
        };
        work.required_w = match work.target_temp_c {
            None => 0.0,
            Some(target) if exact => {
                for row in 0..n {
                    for column in 0..n {
                        solver.scratch[row * n + column] = solver.matrix[row * order + column];
                    }
                    solver.solution[row] = solver.rhs[row] - solver.matrix[row * order + n] * target;
                }
                if solve_dense_in_place(&mut solver.scratch[..n * n], &mut solver.solution[..n], n) {
                    work.air_diagonal_w_k * target + (0..n).map(|column| solver.matrix[n * order + column] * solver.solution[column]).sum::<f64>() - work.air_rhs_w
                } else {
                    0.0
                }
            }
            Some(target) => {
                let mean = if sums[3].abs() > 1e-12 { (sums[5] + sums[4] * target) / sums[3] } else { target };
                work.air_diagonal_w_k * target - sums[1] * mean - work.air_rhs_w - sums[2]
            }
        };
        work.remaining_heating_w = work.required_w.max(0.0);
        work.remaining_cooling_w = (-work.required_w).max(0.0);
        work.delivered_w = 0.0;
        work.ideal_cursor = 0;
        work.equipment_cursor = 0;
        work.stage = BalanceStage::IdealLoad;
    }

    fn unit_settle(&mut self, work: &mut BalanceWork, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let _ = model;
        let zone_id = pre.zone_order[work.zone_cursor];
        let n = pre.enclosures.get(&zone_id).map_or(0, |enclosure| enclosure.faces.len());
        let exact = pre.enclosures.get(&zone_id).is_none_or(|enclosure| matches!(enclosure.radiation, EnclosureRadiation::Exchange { .. }));
        let met = work.target_temp_c.is_some() && (work.delivered_w - work.required_w).abs() <= 1e-9 * work.required_w.abs().max(1.0);
        let solver = &mut state.solver;
        let order = n + 1;
        let sums = work.mean_radiant_sums;
        let air_c = if exact {
            if met {
                let target = work.target_temp_c.expect("met target");
                solver.solution[n] = target;
                target
            } else {
                solver.scratch[..order * order].copy_from_slice(&solver.matrix[..order * order]);
                solver.solution[..order].copy_from_slice(&solver.rhs[..order]);
                solver.solution[n] += work.delivered_w;
                if solve_dense_in_place(&mut solver.scratch[..order * order], &mut solver.solution[..order], order) {
                    solver.solution[n]
                } else {
                    work.free_temp_c
                }
            }
        } else {
            let air = if met {
                work.target_temp_c.expect("met target")
            } else {
                let determinant = work.air_diagonal_w_k * sums[3] - sums[1] * sums[4];
                if determinant.abs() > 1e-12 { ((work.air_rhs_w + sums[2] + work.delivered_w) * sums[3] + sums[1] * sums[5]) / determinant } else { work.free_temp_c }
            };
            solver.solution[0] = air;
            solver.solution[1] = if sums[3].abs() > 1e-12 { (sums[5] + sums[4] * air) / sums[3] } else { air };
            air
        };
        work.free_temp_c = air_c;
        if let Some(zone_state) = state.zones.get_mut(&zone_id) {
            zone_state.heating_demand_w = work.required_w.max(0.0);
            zone_state.cooling_demand_w = (-work.required_w).max(0.0);
            zone_state.unmet_heating_w = work.remaining_heating_w;
            zone_state.unmet_cooling_w = work.remaining_cooling_w;
        }
        work.face_cursor = 0;
        work.stage = BalanceStage::Commit;
    }

    fn unit_commit(&mut self, work: &mut BalanceWork, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let enclosure = pre.enclosures.get(&zone_id);
        let n = enclosure.map_or(0, |enclosure| enclosure.faces.len());
        let nodes = state.solver.nodes;
        if let Some(face) = enclosure.and_then(|enclosure| enclosure.faces.get(work.face_cursor)).copied() {
            let index = work.face_cursor;
            work.face_cursor += 1;
            let inside_c = match &enclosure.expect("enclosure").radiation {
                EnclosureRadiation::Exchange { .. } => state.solver.solution[index],
                EnclosureRadiation::MeanRadiant { .. } => {
                    let reduction = &state.solver.reduction[index * 3..index * 3 + 3];
                    reduction[0] + reduction[1] * state.solver.solution[0] + reduction[2] * state.solver.solution[1]
                }
            };
            let offset = index * nodes;
            let inside_h = state.solver.inside_convection_w_m2k[index];
            // 🧱️ The settled zone-air ⇄ inside-face convective flux for this timestep, which IS the
            // envelope term of the zone air balance `unit_solve`/`unit_settle` just closed. Positive
            // when the air is warmer than the face, i.e. heat leaving the zone through it.
            if state.warmup_complete {
                let (id, area_m2) = match face {
                    EnclosureFace::Opaque(id) => (id, pre.surfaces.get(&id).map_or(0.0, |surface| surface.area_m2)),
                    EnclosureFace::Window(id) => (id, pre.windows.get(&id).map_or(0.0, |window| window.area_m2)),
                };
                let flux_w = area_m2 * inside_h * (work.free_temp_c - inside_c);
                let dt = self.dt_s;
                let row = match face {
                    EnclosureFace::Opaque(_) => state.per_surface.opaque_mut(id),
                    EnclosureFace::Window(_) => state.per_surface.window_mut(id),
                };
                if let Some(row) = row {
                    row.accumulate_conduction(flux_w, dt);
                }
            }
            match face {
                EnclosureFace::Opaque(id) => {
                    let count = pre.surfaces.get(&id).map_or(0, |surface| surface.chain.nodes());
                    let solver = &state.solver;
                    if let Some(surface_state) = state.surfaces.get_mut(&id) {
                        substitute_chain(&solver.chain_a[offset..offset + count - 1], &solver.chain_b[offset..offset + count - 1], inside_c, &mut surface_state.temperatures_c[..count]);
                        surface_state.inside_convection_w_m2k = inside_h;
                    }
                }
                EnclosureFace::Window(id) => {
                    let count = pre.windows.get(&id).map_or(0, |window| window.glazing.faces());
                    let solver = &state.solver;
                    if let Some(window_state) = state.windows.get_mut(&id) {
                        substitute_chain(&solver.chain_a[offset..offset + count - 1], &solver.chain_b[offset..offset + count - 1], inside_c, &mut window_state.face_temperatures_c[..count]);
                        window_state.inside_convection_w_m2k = inside_h;
                    }
                }
            }
            return;
        }
        let exact = enclosure.is_none_or(|enclosure| matches!(enclosure.radiation, EnclosureRadiation::Exchange { .. }));
        let air_c = work.free_temp_c;
        let mean_radiant_c = if exact {
            let enclosure = enclosure.map(|enclosure| (enclosure.areas_m2.as_slice(), enclosure.emissivities.as_slice()));
            enclosure.map_or(air_c, |(areas, emissivities)| {
                let weight: f64 = (0..n).map(|i| areas[i] * emissivities[i]).sum();
                if weight > 0.0 { (0..n).map(|i| areas[i] * emissivities[i] * state.solver.solution[i]).sum::<f64>() / weight } else { air_c }
            })
        } else {
            state.solver.solution[1]
        };
        let zone = &model.zones[work.zone_cursor];
        let loads = self.zone_loads[work.zone_cursor];
        let weather = self.step_weather;
        let outdoor_mass_flow = loads.outdoor_air_m3_s * moist_air_density(weather.dry_bulb_c, weather.humidity_ratio, weather.pressure_pa);
        let delivered = work.delivered;
        if let Some(zone_state) = state.zones.get_mut(&zone_id) {
            let humidity = zone_state.air.next_humidity_ratio(zone.volume_m3 * zone.multiplier.max(1) as f64, weather.pressure_pa, self.dt_s, outdoor_mass_flow, weather.humidity_ratio, loads.latent_w);
            zone_state.air.commit(air_c, humidity);
            zone_state.mean_radiant_temp_c = mean_radiant_c;
            zone_state.hour_temperature_sum_c += air_c;
            zone_state.hour_steps += 1;
            zone_state.hour_delivered = accumulate_delivered(&zone_state.hour_delivered, &delivered);
        }
        work.next_zone();
    }

    fn step_zone_commit(&mut self, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(zone_id) = pre.zone_order.get(self.zone_cursor).copied() else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryPlant;
            return;
        };
        if let Some(zone_state) = state.zones.get_mut(&zone_id) {
            let steps = zone_state.hour_steps.max(1) as f64;
            zone_state.mean_air_temp_c = if zone_state.hour_steps == 0 { zone_state.air.temp_c } else { zone_state.hour_temperature_sum_c / steps };
            zone_state.delivered = zone_state.hour_delivered.scaled(1.0 / steps);
            zone_state.hour_temperature_sum_c = 0.0;
            zone_state.hour_delivered = DeliveredEnergy::default();
            zone_state.hour_steps = 0;
            let delivered = zone_state.delivered;
            state.delivered_total = accumulate_delivered(&state.delivered_total, &delivered);
        }
        self.zone_cursor += 1;
    }

    fn step_plant_bounded(&mut self, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(plant) = model.plant_loops.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.plant_work = None;
            self.stage = TimestepStage::SecondaryPv;
            return;
        };
        let work = self.plant_work.get_or_insert(PlantWork { plant_index: self.secondary_cursor, stage: PlantStage::ReduceZoneLoad, zone_cursor: 0, equipment_cursor: 0, total_load_w: 0.0, remaining_load_w: 0.0, first_load_w: 0.0 });
        match work.stage {
            PlantStage::ReduceZoneLoad => {
                if let Some(zone_id) = pre.zone_order.get(work.zone_cursor) {
                    if let Some(zone) = state.zones.get(zone_id) {
                        work.total_load_w += zone.heating_demand_w + zone.cooling_demand_w;
                    }
                    work.zone_cursor += 1;
                } else {
                    work.remaining_load_w = work.total_load_w;
                    work.stage = PlantStage::BuildPriority;
                }
            }
            PlantStage::BuildPriority => {
                if work.equipment_cursor < plant.equipment_ids.len() {
                    work.equipment_cursor += 1;
                } else {
                    work.equipment_cursor = 0;
                    work.stage = PlantStage::Dispatch;
                }
            }
            PlantStage::Dispatch => {
                if plant.equipment_ids.get(work.equipment_cursor).is_some() {
                    let load = if work.remaining_load_w.is_nan() { 100_000.0 } else { work.remaining_load_w.clamp(0.0, 100_000.0) };
                    if work.equipment_cursor == 0 {
                        work.first_load_w = load;
                    }
                    work.remaining_load_w -= load;
                    work.equipment_cursor += 1;
                } else {
                    work.stage = PlantStage::Simulate;
                }
            }
            PlantStage::Simulate => {
                let pump = Pump { design_head_pa: 200_000.0, design_flow_kg_s: plant.design_flow_kg_s, motor_efficiency: 0.85, part_load_curve: PerformanceCurve::Constant(1.0) };
                let loop_simulation =
                    PlantLoopSimulation { supply: PlantStream::new(plant.supply_temperature_c, plant.design_flow_kg_s), return_stream: PlantStream::new(plant.return_temperature_c, plant.design_flow_kg_s), pump, glycol_fraction: 0.0 };
                let output = loop_simulation.simulate(work.first_load_w);
                state.delivered_total.pump_w += output.electrical_power_w;
                state.plant_supply_c = output.outlet.temperature_c;
                self.secondary_cursor += 1;
                self.plant_work = None;
            }
        }
    }

    fn step_pv(&mut self, model: &Model, state: &mut SimulationModel) {
        let Some(pv) = model.pv_systems.get(self.secondary_cursor) else {
            state.delivered_total.pv_generation_w += self.pv_generation_w;
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryBattery;
            return;
        };
        self.secondary_cursor += 1;
        let system =
            PvSystem { dc_capacity_w: pv.dc_capacity_w, module_efficiency: pv.module_efficiency, area_m2: pv.area_m2, inverter_efficiency: pv.inverter_efficiency, temperature_coefficient: -0.004, tilt_deg: pv.tilt_deg, azimuth_deg: pv.azimuth_deg };
        let plane_irradiance = (self.weather.current.direct_normal_irradiance_w_m2 + self.weather.current.diffuse_horizontal_irradiance_w_m2) * system.orientation_factor(self.sun_alt, self.sun_az);
        self.pv_generation_w += system.simulate(plane_irradiance, self.weather.current.dry_bulb_c + 10.0);
    }

    fn step_battery_bounded(&mut self, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(battery) = model.battery_storage.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.battery_work = None;
            self.stage = TimestepStage::SecondaryServiceHotWater;
            return;
        };
        let work = self.battery_work.get_or_insert(BatteryWork { battery_index: self.secondary_cursor, zone_cursor: 0, net_load_w: 0.0 });
        if let Some(zone_id) = pre.zone_order.get(work.zone_cursor) {
            if let Some(zone) = state.zones.get(zone_id) {
                work.net_load_w += zone.delivered.total_electric_w();
            }
            work.zone_cursor += 1;
            return;
        }
        let charge_w = if self.pv_generation_w > work.net_load_w { (self.pv_generation_w - work.net_load_w).min(battery.max_charge_w) } else { -(work.net_load_w - self.pv_generation_w).min(battery.max_discharge_w) };
        state.battery_soc = (state.battery_soc + charge_w * 3600.0 / (battery.capacity_kwh * 3_600_000.0)).clamp(0.0, 1.0);
        state.delivered_total.battery_charge_w += charge_w.max(0.0);
        let transformer = Transformer { rated_kva: 100.0, no_load_loss_w: 50.0, load_loss_w: 200.0, impedance_fraction: 0.02 };
        let _ = grid_balance(work.net_load_w, self.pv_generation_w, 0.0, 0.0, charge_w, &transformer);
        self.secondary_cursor += 1;
        self.battery_work = None;
    }

    fn step_service_hot_water(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.shw_systems.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryRefrigeration;
            return;
        };
        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, system.schedule_id, &self.context) else {
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.shw_electric_w += system.heater_capacity_w * schedule * 0.3;
    }

    fn step_refrigeration(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.refrigeration_systems.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryWater;
            return;
        };
        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, system.defrost_schedule_id, &self.context) else {
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.refrigeration_w += system.design_load_w * schedule;
    }

    fn step_water(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.water_systems.get(self.secondary_cursor) else {
            state.hour = self.hour as u32;
            self.stage = TimestepStage::Complete;
            return;
        };
        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, system.schedule_id, &self.context) else {
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.water_pump_w += system.peak_flow_l_s * 1000.0 * schedule * 50.0;
    }
}
// #endregion 🔖️TimestepJob

fn advance_zone_preparation(work: &mut ZonePreparationWork, stage: ZonePreparationStage) {
    work.stage = stage;
    work.cursor = 0;
}

fn schedule_lookup_step(work: &mut Option<ScheduleLookupWork>, schedules: &ScheduleSet, requested_id: crate::model::ScheduleId, context: &ScheduleContext) -> Option<f64> {
    let cursor = work.get_or_insert(ScheduleLookupWork { requested_id, stage: ScheduleLookupStage::Constant, cursor: 0, annual_index: None, daily_id: None, daily_fallback: ScheduleLookupStage::Weekly });
    debug_assert_eq!(cursor.requested_id, requested_id);
    match cursor.stage {
        ScheduleLookupStage::Constant => {
            if let Some(schedule) = schedules.constants.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule.id == requested_id {
                    return Some(finish_schedule_lookup(work, schedule.value));
                }
            } else {
                cursor.stage = ScheduleLookupStage::Annual;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::Annual => {
            if let Some(schedule) = schedules.annual.get(cursor.cursor) {
                if schedule.id == requested_id {
                    cursor.annual_index = Some(cursor.cursor);
                    cursor.stage = ScheduleLookupStage::AnnualHoliday;
                    cursor.cursor = 0;
                } else {
                    cursor.cursor += 1;
                }
            } else {
                cursor.stage = ScheduleLookupStage::Weekly;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::AnnualHoliday => {
            let annual = &schedules.annual[cursor.annual_index.expect("annual schedule cursor")];
            if let Some(date) = annual.holiday_dates.get(cursor.cursor) {
                cursor.cursor += 1;
                if *date == (context.year, context.month, context.day) {
                    if let Some(daily_id) = annual.holiday_daily_schedule_id {
                        cursor.daily_id = Some(daily_id);
                        cursor.daily_fallback = ScheduleLookupStage::Weekly;
                        cursor.stage = ScheduleLookupStage::ResolveDaily;
                        cursor.cursor = 0;
                    }
                }
            } else {
                cursor.stage = ScheduleLookupStage::AnnualRule;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::AnnualRule => {
            let annual = &schedules.annual[cursor.annual_index.expect("annual schedule cursor")];
            if let Some(rule) = annual.rules.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule_date_in_range(context.month, context.day, rule.start_month, rule.start_day, rule.end_month, rule.end_day) {
                    cursor.daily_id = Some(rule.daily_schedule_id);
                    cursor.daily_fallback = ScheduleLookupStage::Weekly;
                    cursor.stage = ScheduleLookupStage::ResolveDaily;
                    cursor.cursor = 0;
                }
            } else {
                cursor.daily_id = Some(annual.default_daily_schedule_id);
                cursor.daily_fallback = ScheduleLookupStage::Weekly;
                cursor.stage = ScheduleLookupStage::ResolveDaily;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::ResolveDaily => {
            if let Some(schedule) = schedules.daily.get(cursor.cursor) {
                cursor.cursor += 1;
                if Some(schedule.id) == cursor.daily_id {
                    let mut value = schedule.hourly_values[(context.hour as usize).min(23)];
                    if let Some(limits) = schedule.limits {
                        value = value.clamp(limits.min, limits.max);
                    }
                    return Some(finish_schedule_lookup(work, value));
                }
            } else {
                cursor.stage = cursor.daily_fallback;
                cursor.cursor = 0;
                cursor.daily_id = None;
            }
        }
        ScheduleLookupStage::Weekly => {
            cursor.annual_index = None;
            if let Some(schedule) = schedules.weekly.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule.id == requested_id {
                    cursor.daily_id = Some(schedule.daily_schedule_ids[(context.day_of_week as usize).min(6)]);
                    cursor.daily_fallback = ScheduleLookupStage::DirectDaily;
                    cursor.stage = ScheduleLookupStage::ResolveDaily;
                    cursor.cursor = 0;
                }
            } else {
                cursor.stage = ScheduleLookupStage::DirectDaily;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::DirectDaily => {
            if let Some(schedule) = schedules.daily.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule.id == requested_id {
                    let mut value = schedule.hourly_values[(context.hour as usize).min(23)];
                    if let Some(limits) = schedule.limits {
                        value = value.clamp(limits.min, limits.max);
                    }
                    return Some(finish_schedule_lookup(work, value));
                }
            } else {
                cursor.stage = ScheduleLookupStage::TimeSeries;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::TimeSeries => {
            if let Some(schedule) = schedules.time_series.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule.id == requested_id {
                    let index = (context.timestep_index as usize).min(schedule.values.len().saturating_sub(1));
                    return Some(finish_schedule_lookup(work, schedule.values.get(index).copied().unwrap_or(1.0)));
                }
            } else {
                return Some(finish_schedule_lookup(work, 1.0));
            }
        }
    }
    None
}

fn finish_schedule_lookup(work: &mut Option<ScheduleLookupWork>, value: f64) -> f64 {
    *work = None;
    value
}

fn schedule_date_in_range(month: u8, day: u8, start_month: u8, start_day: u8, end_month: u8, end_day: u8) -> bool {
    let current = month as u16 * 32 + day as u16;
    let start = start_month as u16 * 32 + start_day as u16;
    let end = end_month as u16 * 32 + end_day as u16;
    if start <= end {
        current >= start && current <= end
    } else {
        current >= start || current <= end
    }
}

// #region 🔖️Kernel
/// 🔄️ BEM simulation kernel with full subsystem coupling.
pub struct SimulationKernel;

impl SimulationKernel {
    /// 🔄️ Initialize state from model and precomputed data.
    #[cfg(test)]
    pub(crate) fn initialize(model: &Model, pre: &PrecomputedModel, weather: &WeatherRecord) -> SimulationModel {
        let mut state = SimulationModel::default();
        state.zones.admit(model.zones.len()).expect("test zone state backing");
        state.surfaces.admit(pre.surfaces.len()).expect("test surface state backing");
        state.windows.admit(pre.windows.len()).expect("test window state backing");
        state.solver.reserve(pre.maximum_enclosure_faces, pre.maximum_nodes).expect("test solver backing");
        for zone in &model.zones {
            let _ = state.zones.insert(zone.id, ZoneState::new(weather.dry_bulb_c, weather.humidity_ratio()));
        }
        for (id, surface) in pre.surfaces.iter() {
            let _ = state.surfaces.insert(*id, SurfaceState { temperatures_c: vec![weather.dry_bulb_c; surface.chain.nodes()], inside_convection_w_m2k: 0.0 });
        }
        for (id, _) in pre.windows.iter() {
            let _ = state.windows.insert(*id, WindowState { face_temperatures_c: [weather.dry_bulb_c; 2 * MAX_PANES], inside_convection_w_m2k: 0.0 });
        }
        state
    }

    /// 🔄️ Advance one hour through the same bounded cursor machine used by EnergyJob.
    #[cfg(test)]
    pub(crate) fn advance_timestep(model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel, weather: &WeatherRecord, date: &SimDate, hour: f64) -> Result<(), Error> {
        let mut work = TimestepWork::new(model, pre, HourWeather { previous: *weather, current: *weather, next: *weather }, *date, hour);
        while !work.is_complete() {
            work.step(model, config, pre, state);
        }
        Ok(())
    }

    /// 🔄️ Check energy balance for diagnostics.
    pub fn energy_balance_check(input_w: f64, stored_w: f64, output_w: f64) -> f64 {
        (input_w - stored_w - output_w).abs()
    }

    /// 📅️ Build run period from config.
    pub fn run_period(config: &SimulationConfig) -> RunPeriod {
        RunPeriod { start_month: config.run_period_start_month, start_day: config.run_period_start_day, end_month: config.run_period_end_month, end_day: config.run_period_end_day, year: 2026 }
    }
}
// #endregion 🔖️Kernel

fn accumulate_delivered(total: &DeliveredEnergy, step: &DeliveredEnergy) -> DeliveredEnergy {
    DeliveredEnergy {
        heating_w: total.heating_w + step.heating_w,
        cooling_w: total.cooling_w + step.cooling_w,
        fan_w: total.fan_w + step.fan_w,
        pump_w: total.pump_w + step.pump_w,
        compressor_w: total.compressor_w + step.compressor_w,
        gas_w: total.gas_w + step.gas_w,
        pv_generation_w: total.pv_generation_w + step.pv_generation_w,
        battery_charge_w: total.battery_charge_w + step.battery_charge_w,
        shw_electric_w: total.shw_electric_w + step.shw_electric_w,
        shw_gas_w: total.shw_gas_w + step.shw_gas_w,
        refrigeration_w: total.refrigeration_w + step.refrigeration_w,
        water_pump_w: total.water_pump_w + step.water_pump_w,
    }
}

fn state_pane_source(pane_absorbed_w_m2: &[f64], face: usize, node: usize) -> f64 {
    0.5 * pane_absorbed_w_m2[face * MAX_PANES + node / 2]
}

fn face_inside_c(state: &SimulationModel, pre: &PrecomputedModel, face: EnclosureFace, fallback_c: f64) -> f64 {
    match face {
        EnclosureFace::Opaque(id) => state.surfaces.get(&id).map_or(fallback_c, SurfaceState::inside_temp_c),
        EnclosureFace::Window(id) => match (state.windows.get(&id), pre.windows.get(&id)) {
            (Some(window_state), Some(window)) => window_state.face_temperatures_c[window.glazing.faces() - 1],
            _ => fallback_c,
        },
    }
}

#[cfg(test)]
fn default_weather(hour: u32) -> WeatherRecord {
    WeatherRecord {
        year: 2026,
        month: 1,
        day: 1,
        hour: (hour % 24) as u8,
        minute: 0,
        dry_bulb_c: 20.0,
        dew_point_c: 10.0,
        relative_humidity: 0.5,
        atmospheric_pressure_pa: 101_325.0,
        wind_speed_m_s: 2.0,
        wind_direction_deg: 0.0,
        direct_normal_irradiance_w_m2: 0.0,
        diffuse_horizontal_irradiance_w_m2: 0.0,
        horizontal_infrared_w_m2: 250.0,
        precipitation_mm: 0.0,
        snow_depth_mm: 0.0,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
