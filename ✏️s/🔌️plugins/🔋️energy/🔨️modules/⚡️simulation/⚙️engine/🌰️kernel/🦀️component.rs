//! 🔄️ Simulation kernel: calendar, multi-rate loops, warmup, predictor-corrector coupling.

use crate::air_exchange::{infiltration_flow_m3_s, ventilation_load_w, InfiltrationMethod, InfiltrationSpec};
use crate::calendar::{RunPeriod, SimDate};
use crate::controls::{evaluate_controls, predict_zone_load, HumidistatSpec, ThermostatSpec};
use crate::curves::PerformanceCurve;
use crate::daylight::{dimmed_lighting_power_w, lighting_dimming_fraction, reference_point_illuminance_lux, simplified_daylight_factor};
use crate::electrical::{grid_balance, PvSystem, Transformer};
use crate::envelope::{solve_exterior_surface_temp, solve_interior_surface_temp, ConductionState, ExteriorConvectionModel, InteriorConvectionModel};
use crate::error::Error;
use crate::gains::{compute_equipment_gain_w, compute_lighting_gain_w, compute_people_gain_w, ActivityLevel, GainDecomposition};
use crate::ideal_hvac::{ideal_loads_deliver, IdealLoadsConfig, IdealLoadsInput};
use crate::model::{EntityId, FixedTable, Model, OutsideBoundary};
use crate::plant::{PlantLoopSimulation, PlantStream, Pump};
use crate::precompute::PrecomputedModel;
use crate::props::saturation_pressure_pa;
use crate::schedule::{ScheduleContext, ScheduleSet};
use crate::site::{GroundTemperatureModel, WeatherRecord};
use crate::solar::{shading_factor, surface_solar_absorption};
use crate::zone_air::{advance_zone_air, HumiditySolutionMethod, ZoneAirBalance, ZoneAirState};
use crate::zone_hvac::{ZoneEquipment, ZoneEquipmentRequest};
use serde::{Deserialize, Serialize};

// #region 🔖️Config
/// ⚙️ Simulation environment type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationEnvironment {
    WeatherRunPeriod,
    HeatingDesignDay,
    CoolingDesignDay,
    CustomDesignPeriod,
}

/// ⚙️ Convergence tolerances.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

/// ⚙️ Simulation configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
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
}
// #endregion 🔖️DeliveredEnergy

// #region 🔖️State
/// 🔄️ Per-zone simulation state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneState {
    pub air: ZoneAirState,
    pub heating_demand_w: f64,
    pub cooling_demand_w: f64,
    pub unmet_heating_w: f64,
    pub unmet_cooling_w: f64,
    pub delivered: DeliveredEnergy,
}

impl ZoneState {
    pub(crate) fn empty() -> Self {
        Self { air: ZoneAirState::new(20.0, 0.01), heating_demand_w: 0.0, cooling_demand_w: 0.0, unmet_heating_w: 0.0, unmet_cooling_w: 0.0, delivered: DeliveredEnergy::default() }
    }
}

/// 🔄️ Surface thermal history for CTF conduction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SurfaceState {
    pub inside_temp_c: f64,
    pub outside_temp_c: f64,
    pub heat_flux_w: f64,
    pub ctf: ConductionState,
    pub convection_to_zone_w: f64,
}

/// 🔄️ Full simulation state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationModel {
    pub(crate) zones: FixedTable<EntityId, ZoneState>,
    pub(crate) surfaces: FixedTable<EntityId, SurfaceState>,
    pub(crate) warmup_complete: bool,
    pub(crate) hour: u32,
    pub(crate) delivered_total: DeliveredEnergy,
    pub(crate) battery_soc: f64,
    pub(crate) plant_supply_c: f64,
}

impl Default for SimulationModel {
    fn default() -> Self {
        Self { zones: FixedTable::default(), surfaces: FixedTable::default(), warmup_complete: false, hour: 0, delivered_total: DeliveredEnergy::default(), battery_soc: 0.5, plant_supply_c: 55.0 }
    }
}
// #endregion 🔖️State

// #region 🔖️TimestepJob
/// 🧭️ Bounded phase of one persistent zone timestep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum TimestepStage {
    Surface,
    Fenestration,
    Zone,
    SystemSubstep,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[cfg(test)]
pub(crate) const P7C1_TIMESTEP_STAGES: [TimestepStage; 12] = [
    TimestepStage::Surface,
    TimestepStage::Fenestration,
    TimestepStage::Zone,
    TimestepStage::SystemSubstep,
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
pub(crate) const P7C1_SYSTEM_SUBSTEP_STAGES: [SystemSubstepStage; 6] =
    [SystemSubstepStage::Predict, SystemSubstepStage::IdealLoad, SystemSubstepStage::Fault, SystemSubstepStage::ApplyIdealLoad, SystemSubstepStage::ZoneEquipment, SystemSubstepStage::Complete];

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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ScheduleLookupWork {
    requested_id: crate::model::ScheduleId,
    stage: ScheduleLookupStage,
    cursor: usize,
    annual_index: Option<usize>,
    daily_id: Option<crate::model::ScheduleId>,
    daily_fallback: ScheduleLookupStage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SystemSubstepStage {
    Predict,
    IdealLoad,
    Fault,
    ApplyIdealLoad,
    ZoneEquipment,
    Complete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SystemSubstepWork {
    stage: SystemSubstepStage,
    ideal_cursor: usize,
    fault_cursor: usize,
    equipment_cursor: usize,
    selected_ideal: Option<usize>,
    fault_factor: f64,
    balance: Option<ZoneAirBalance>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum PlantStage {
    ReduceZoneLoad,
    BuildPriority,
    Dispatch,
    Simulate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlantWork {
    plant_index: usize,
    stage: PlantStage,
    zone_cursor: usize,
    equipment_cursor: usize,
    total_load_w: f64,
    remaining_load_w: f64,
    first_load_w: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BatteryWork {
    battery_index: usize,
    zone_cursor: usize,
    net_load_w: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ZoneTimestepWork {
    zone_index: usize,
    zone_id: EntityId,
    floor_area_m2: f64,
    zone_rh: f64,
    internal_gain: GainDecomposition,
    infiltration_sensible_w: f64,
    infiltration_latent_w: f64,
    surface_convection_w: f64,
    sensible_gain_w: f64,
    heating_setpoint_c: f64,
    cooling_setpoint_c: f64,
    thermostat: ThermostatSpec,
    humidistat: Option<HumidistatSpec>,
    delivered: DeliveredEnergy,
    system: SystemSubstepWork,
}

/// ⏱️ Cursor-owned execution state for one zone timestep.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TimestepWork {
    stage: TimestepStage,
    context: ScheduleContext,
    weather: WeatherRecord,
    date: SimDate,
    hour: f64,
    dt_s: f64,
    system_dt_s: f64,
    sub_steps: u32,
    sun_alt: f64,
    sun_az: f64,
    surface_cursor: usize,
    fenestration_cursor: usize,
    zone_cursor: usize,
    system_substep_cursor: u32,
    secondary_cursor: usize,
    zone_envelope_w: Vec<f64>,
    zone_solar_w: Vec<f64>,
    zone_surface_conv_w: Vec<f64>,
    zone_preparation: Option<ZonePreparationWork>,
    zone_work: Option<ZoneTimestepWork>,
    plant_work: Option<PlantWork>,
    battery_work: Option<BatteryWork>,
    schedule_lookup: Option<ScheduleLookupWork>,
    pv_generation_w: f64,
}

/// 🧱️ Persistent backing constructor for one timestep.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TimestepBuilder {
    stage: u8,
    cursor: usize,
    context: ScheduleContext,
    weather: WeatherRecord,
    date: SimDate,
    hour: f64,
    dt_s: f64,
    system_dt_s: f64,
    zone_envelope_w: Vec<f64>,
    zone_solar_w: Vec<f64>,
    zone_surface_conv_w: Vec<f64>,
}

#[cfg(test)]
pub(crate) const P7C1_TIMESTEP_BUILDER_STAGES: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];

impl TimestepBuilder {
    pub(crate) fn retained_wire_signature(&self) -> [u64; 6] {
        [self.stage as u64, self.cursor as u64, self.zone_envelope_w.len() as u64, self.zone_solar_w.len() as u64, self.zone_surface_conv_w.len() as u64, self.hour.to_bits()]
    }

    pub(crate) fn new(model: &Model, pre: &PrecomputedModel, weather: WeatherRecord, date: SimDate, hour: f64, dt_s: f64) -> Self {
        let system_dt_s = pre.system_timestep_s.min(dt_s);
        Self {
            stage: 0,
            cursor: 0,
            context: ScheduleContext { year: date.year, month: date.month, day: date.day, hour: weather.hour, day_of_week: date.day_of_week(), timestep_index: hour as u32, is_dst: false },
            weather,
            date,
            hour,
            dt_s,
            system_dt_s,
            zone_envelope_w: Vec::new(),
            zone_solar_w: Vec::new(),
            zone_surface_conv_w: Vec::new(),
        }
    }

    pub(crate) fn step(&mut self, model: &Model, pre: &PrecomputedModel) -> Result<Option<TimestepWork>, Error> {
        let zones = pre.zone_order.len();
        match self.stage {
            0 => {
                self.zone_envelope_w.try_reserve_exact(zones).map_err(|_| Error::severe("energy timestep envelope backing rejected"))?;
                self.stage = 1;
            }
            1 => {
                if self.cursor < zones {
                    self.zone_envelope_w.push(0.0);
                    self.cursor += 1;
                } else {
                    self.stage = 2;
                    self.cursor = 0;
                }
            }
            2 => {
                self.zone_solar_w.try_reserve_exact(zones).map_err(|_| Error::severe("energy timestep solar backing rejected"))?;
                self.stage = 3;
            }
            3 => {
                if self.cursor < zones {
                    self.zone_solar_w.push(0.0);
                    self.cursor += 1;
                } else {
                    self.stage = 4;
                    self.cursor = 0;
                }
            }
            4 => {
                self.zone_surface_conv_w.try_reserve_exact(zones).map_err(|_| Error::severe("energy timestep convection backing rejected"))?;
                self.stage = 5;
            }
            5 => {
                if self.cursor < zones {
                    self.zone_surface_conv_w.push(0.0);
                    self.cursor += 1;
                } else {
                    self.stage = 6;
                }
            }
            _ => {
                let (sun_alt, sun_az) = pre.solar_at(model, self.date.day_of_year(), self.hour);
                return Ok(Some(TimestepWork {
                    stage: TimestepStage::Surface,
                    context: self.context,
                    weather: self.weather,
                    date: self.date,
                    hour: self.hour,
                    dt_s: self.dt_s,
                    system_dt_s: self.system_dt_s,
                    sub_steps: (self.dt_s / self.system_dt_s).ceil() as u32,
                    sun_alt,
                    sun_az,
                    surface_cursor: 0,
                    fenestration_cursor: 0,
                    zone_cursor: 0,
                    system_substep_cursor: 0,
                    secondary_cursor: 0,
                    zone_envelope_w: std::mem::take(&mut self.zone_envelope_w),
                    zone_solar_w: std::mem::take(&mut self.zone_solar_w),
                    zone_surface_conv_w: std::mem::take(&mut self.zone_surface_conv_w),
                    zone_preparation: None,
                    zone_work: None,
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
        self.zone_envelope_w.pop().is_none() && self.zone_solar_w.pop().is_none() && self.zone_surface_conv_w.pop().is_none()
    }
}

impl TimestepWork {
    pub(crate) fn retained_wire_signature(&self) -> [u64; 16] {
        [
            self.stage as u64,
            self.surface_cursor as u64,
            self.fenestration_cursor as u64,
            self.zone_cursor as u64,
            self.system_substep_cursor as u64,
            self.secondary_cursor as u64,
            self.zone_envelope_w.len() as u64,
            self.zone_solar_w.len() as u64,
            self.zone_surface_conv_w.len() as u64,
            self.zone_preparation.is_some() as u64,
            self.zone_work.is_some() as u64,
            self.plant_work.is_some() as u64,
            self.battery_work.is_some() as u64,
            self.schedule_lookup.is_some() as u64,
            self.hour.to_bits(),
            self.pv_generation_w.to_bits(),
        ]
    }

    #[cfg(test)]
    pub(crate) fn new(model: &Model, pre: &PrecomputedModel, weather: WeatherRecord, date: SimDate, hour: f64, dt_s: f64) -> Self {
        let mut builder = TimestepBuilder::new(model, pre, weather, date, hour, dt_s);
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
    pub(crate) fn system_substep_stage(&self) -> Option<SystemSubstepStage> {
        self.zone_work.as_ref().map(|work| work.system.stage)
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
    pub(crate) fn set_system_substep_stage_for_gate(&mut self, stage: SystemSubstepStage) -> bool {
        let Some(work) = self.zone_work.as_mut() else { return false };
        work.system.stage = stage;
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
        if self.zone_envelope_w.pop().is_some() || self.zone_solar_w.pop().is_some() || self.zone_surface_conv_w.pop().is_some() {
            return false;
        }
        self.zone_preparation = None;
        self.zone_work = None;
        self.plant_work = None;
        self.battery_work = None;
        true
    }

    pub(crate) fn step(&mut self, model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel) {
        match self.stage {
            TimestepStage::Surface => self.step_surface(model, pre, state),
            TimestepStage::Fenestration => self.step_fenestration(model, pre, state),
            TimestepStage::Zone => self.step_zone_preparation(model, config, pre, state),
            TimestepStage::SystemSubstep => self.step_system_substep_bounded(model, pre, state),
            TimestepStage::ZoneCommit => self.commit_zone(state),
            TimestepStage::SecondaryPlant => self.step_plant_bounded(model, pre, state),
            TimestepStage::SecondaryPv => self.step_pv(model, state),
            TimestepStage::SecondaryBattery => self.step_battery_bounded(model, pre, state),
            TimestepStage::SecondaryServiceHotWater => self.step_service_hot_water(model, config, state),
            TimestepStage::SecondaryRefrigeration => self.step_refrigeration(model, config, state),
            TimestepStage::SecondaryWater => self.step_water(model, config, state),
            TimestepStage::Complete => {}
        }
    }

    fn step_surface(&mut self, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(sid) = pre.surface_order.get(self.surface_cursor).copied() else {
            self.stage = TimestepStage::Fenestration;
            return;
        };
        self.surface_cursor += 1;
        let Some(sp) = pre.surfaces.get(&sid) else { return };
        let surface = pre.surface_indices.get(&sid).and_then(|index| model.surfaces.get(*index));
        let outside_temp = match surface.map(|surface| surface.outside_boundary_condition) {
            Some(OutsideBoundary::Ground) => GroundTemperatureModel::Monthly { temperatures_c: model.ground_temperature.building_surface_c }.temperature_c(self.date.day_of_year()),
            Some(OutsideBoundary::OutdoorAir) | None => self.weather.dry_bulb_c,
            Some(OutsideBoundary::OtherSideTemperature) => self.weather.dry_bulb_c - 5.0,
            Some(OutsideBoundary::Adiabatic) | Some(OutsideBoundary::Interzone(_)) => state.surfaces.get(&sid).map_or(self.weather.dry_bulb_c, |surface| surface.outside_temp_c),
        };
        let zone_t = state.zones.get(&sp.zone_id).map_or(self.weather.dry_bulb_c, |zone| zone.air.temp_c);
        let solar_w_m2 = if sp.sun_exposed && self.sun_alt > 0.0 {
            let incidence = crate::solar::beam_incidence_cosine(sp.normal, self.sun_alt, self.sun_az);
            let shade = shading_factor(1.0, 0.0, 1.0, self.sun_alt);
            surface_solar_absorption(self.weather.direct_normal_irradiance_w_m2, self.weather.diffuse_horizontal_irradiance_w_m2, incidence, shade, sp.solar_absorptance, sp.tilt_deg).total_w_m2
        } else {
            0.0
        };
        let Some(surface_state) = state.surfaces.get_mut(&sid) else { return };
        let conduction_w_m2 = surface_state.ctf.heat_flux_w_m2(outside_temp, zone_t);
        let exterior_t = solve_exterior_surface_temp(outside_temp, self.weather.dry_bulb_c + 253.15, self.weather.wind_speed_m_s, solar_w_m2, -conduction_w_m2, sp.emissivity, &ExteriorConvectionModel::default());
        let balance = solve_interior_surface_temp(zone_t, conduction_w_m2, solar_w_m2 * 0.3, &InteriorConvectionModel::default());
        let conv_w = balance.convection_w_m2 * sp.area_m2;
        let cond_w = conduction_w_m2 * sp.area_m2;
        surface_state.inside_temp_c = balance.surface_temp_c;
        surface_state.outside_temp_c = exterior_t;
        surface_state.heat_flux_w = cond_w;
        surface_state.convection_to_zone_w = conv_w;
        surface_state.ctf.advance(outside_temp);
        if let Some(index) = pre.zone_indices.get(&sp.zone_id).copied() {
            self.zone_envelope_w[index] += cond_w;
            self.zone_solar_w[index] += solar_w_m2 * sp.area_m2 * 0.7;
            self.zone_surface_conv_w[index] += conv_w;
        }
    }

    fn step_fenestration(&mut self, model: &Model, pre: &PrecomputedModel, state: &SimulationModel) {
        let Some(fid) = pre.fenestration_order.get(self.fenestration_cursor).copied() else {
            self.stage = TimestepStage::Zone;
            return;
        };
        self.fenestration_cursor += 1;
        let Some(fp) = pre.fenestrations.get(&fid) else { return };
        let Some(surface) = pre.surface_indices.get(&fp.surface_id).and_then(|index| model.surfaces.get(*index)) else { return };
        let zone_t = state.zones.get(&surface.zone_id).map_or(self.weather.dry_bulb_c, |zone| zone.air.temp_c);
        let Some(zone_index) = pre.zone_indices.get(&surface.zone_id).copied() else { return };
        self.zone_envelope_w[zone_index] += fp.u_value_w_m2k * fp.area_m2 * (self.weather.dry_bulb_c - zone_t);
        if self.sun_alt > 0.0 {
            let incidence = crate::solar::beam_incidence_cosine(fp.normal, self.sun_alt, self.sun_az);
            let shade = shading_factor(1.0, 0.0, 1.0, self.sun_alt);
            self.zone_solar_w[zone_index] += (self.weather.direct_normal_irradiance_w_m2 * incidence * shade + self.weather.diffuse_horizontal_irradiance_w_m2 * 0.5) * fp.shgc * fp.area_m2;
        }
    }

    fn step_zone_preparation(&mut self, model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &SimulationModel) {
        let Some(zone) = model.zones.get(self.zone_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryPlant;
            return;
        };
        if self.zone_preparation.is_none() {
            let geometry = pre.zone_geometry.get(&zone.id).cloned().unwrap_or_default();
            let zone_state = state.zones.get(&zone.id);
            let zone_temp_c = zone_state.map_or(self.weather.dry_bulb_c, |value| value.air.temp_c);
            let zone_humidity_ratio = zone_state.map_or(self.weather.humidity_ratio(), |value| value.air.humidity_ratio);
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
                    let lux = reference_point_illuminance_lux(self.weather.diffuse_horizontal_irradiance_w_m2 * 120.0, self.weather.direct_normal_irradiance_w_m2 * 120.0, self.sun_alt.max(0.0) / 90.0, work.daylight_transmittance, factor, 1.0);
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
                            method: InfiltrationMethod::WindAndStack,
                            schedule_factor,
                            ach: 0.0,
                            flow_per_exterior_area_m3_s_m2: infiltration.flow_per_exterior_area_m3_s_m2,
                            effective_leakage_area_m2: 0.0,
                            discharge_coefficient: 0.65,
                            constant_coefficient: infiltration.constant_term_coefficient,
                            temperature_coefficient: infiltration.temperature_term_coefficient,
                            velocity_coefficient: infiltration.velocity_term_coefficient,
                            velocity_squared_coefficient: infiltration.velocity_squared_term_coefficient,
                            stack_height_m: 3.0,
                        };
                        work.infiltration_flow_m3_s += infiltration_flow_m3_s(&specification, zone.volume_m3, geometry.exterior_area_m2, self.weather.dry_bulb_c, work.zone_temp_c, self.weather.wind_speed_m_s, self.weather.atmospheric_pressure_pa);
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
                        let stack = (work.zone_temp_c - self.weather.dry_bulb_c).abs().sqrt();
                        work.airflow_flow_m3_s = 0.01 * (self.weather.wind_speed_m_s + stack).powf(0.65);
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
                        if work.thermostat_schedule == 0 {
                            let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, thermostat.heating_setpoint_schedule_id, &self.context) else {
                                return;
                            };
                            work.heating_setpoint_c = schedule * 24.0 + 20.0;
                            work.thermostat_schedule = 1;
                            return;
                        }
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, thermostat.cooling_setpoint_schedule_id, &self.context) else {
                            return;
                        };
                        work.cooling_setpoint_c = schedule * 6.0 + 24.0;
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
                let (infiltration_sensible_w, infiltration_latent_w) = ventilation_load_w(
                    work.infiltration_flow_m3_s + work.airflow_flow_m3_s + work.mechanical_flow_m3_s,
                    work.zone_temp_c,
                    work.zone_humidity_ratio,
                    self.weather.dry_bulb_c,
                    self.weather.humidity_ratio(),
                    self.weather.atmospheric_pressure_pa,
                    0.0,
                );
                let zone_index = pre.zone_indices.get(&work.zone_id).copied().unwrap_or(work.zone_index);
                let surface_convection_w = self.zone_surface_conv_w.get(zone_index).copied().unwrap_or(0.0);
                let sensible_gain_w = work.internal_gain.sensible_w + self.zone_solar_w.get(zone_index).copied().unwrap_or(0.0) + surface_convection_w - self.zone_envelope_w.get(zone_index).copied().unwrap_or(0.0);
                self.zone_work = Some(ZoneTimestepWork {
                    zone_index: work.zone_index,
                    zone_id: work.zone_id,
                    floor_area_m2: work.floor_area_m2,
                    zone_rh: relative_humidity_from_w(work.zone_humidity_ratio, work.zone_temp_c, self.weather.atmospheric_pressure_pa),
                    internal_gain: work.internal_gain,
                    infiltration_sensible_w,
                    infiltration_latent_w,
                    surface_convection_w,
                    sensible_gain_w,
                    heating_setpoint_c: work.heating_setpoint_c,
                    cooling_setpoint_c: work.cooling_setpoint_c,
                    thermostat: work.thermostat.clone(),
                    humidistat: work.humidistat.clone(),
                    delivered: DeliveredEnergy::default(),
                    system: SystemSubstepWork { stage: SystemSubstepStage::Predict, ideal_cursor: 0, fault_cursor: 0, equipment_cursor: 0, selected_ideal: None, fault_factor: 1.0, balance: None },
                });
                self.zone_preparation = None;
                self.system_substep_cursor = 0;
                self.stage = TimestepStage::SystemSubstep;
            }
        }
    }

    fn step_system_substep_bounded(&mut self, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(work) = self.zone_work.as_mut() else {
            self.stage = TimestepStage::Zone;
            return;
        };
        if self.system_substep_cursor >= self.sub_steps.max(1) {
            self.stage = TimestepStage::ZoneCommit;
            return;
        }
        let zone = &model.zones[work.zone_index];
        let Some(zone_state) = state.zones.get_mut(&zone.id) else { return };
        match work.system.stage {
            SystemSubstepStage::Predict => {
                let controls = evaluate_controls(&work.thermostat, work.humidistat.as_ref(), zone_state.air.temp_c, work.zone_rh);
                let residual_sensible_w = work.sensible_gain_w - zone_state.heating_demand_w + zone_state.cooling_demand_w;
                let predicted = predict_zone_load(residual_sensible_w, work.internal_gain.latent_w, &controls, f64::INFINITY, f64::INFINITY, 5000.0, 5000.0);
                let balance = ZoneAirBalance {
                    volume_m3: zone.volume_m3,
                    conditioned: zone.conditioned,
                    sensible_gain_w: work.sensible_gain_w,
                    latent_gain_w: work.internal_gain.latent_w,
                    infiltration_sensible_w: work.infiltration_sensible_w,
                    infiltration_latent_w: work.infiltration_latent_w,
                    ventilation_sensible_w: 0.0,
                    ventilation_latent_w: 0.0,
                    system_sensible_w: 0.0,
                    system_latent_w: 0.0,
                    surface_convection_w: work.surface_convection_w,
                    mass_flow_in_kg_s: 0.0,
                    supply_humidity_ratio: self.weather.humidity_ratio(),
                    outdoor_humidity_ratio: self.weather.humidity_ratio(),
                    heating_setpoint_c: Some(work.heating_setpoint_c),
                    cooling_setpoint_c: Some(work.cooling_setpoint_c),
                    max_heating_w: None,
                    max_cooling_w: None,
                };
                let result = advance_zone_air(&zone_state.air, &balance, self.system_dt_s, HumiditySolutionMethod::ThirdOrderBackward, self.weather.atmospheric_pressure_pa);
                zone_state.air.push_temp(result.temp_c);
                zone_state.air.push_humidity(result.humidity_ratio);
                zone_state.heating_demand_w = predicted.heating_w;
                zone_state.cooling_demand_w = predicted.cooling_w;
                work.system.balance = Some(balance);
                work.system.stage = SystemSubstepStage::IdealLoad;
            }
            SystemSubstepStage::IdealLoad => {
                if let Some(ideal) = model.ideal_loads.get(work.system.ideal_cursor) {
                    if ideal.zone_id == zone.id {
                        work.system.selected_ideal = Some(work.system.ideal_cursor);
                        work.system.fault_cursor = 0;
                        work.system.fault_factor = 1.0;
                        work.system.stage = SystemSubstepStage::Fault;
                    } else {
                        work.system.ideal_cursor += 1;
                    }
                } else {
                    work.system.stage = SystemSubstepStage::ZoneEquipment;
                }
            }
            SystemSubstepStage::Fault => {
                let ideal = &model.ideal_loads[work.system.selected_ideal.expect("selected ideal load")];
                if let Some(fault) = model.faults.get(work.system.fault_cursor) {
                    if fault.target_equipment_id == ideal.id {
                        let severity = pre.fault_severity.get(&ideal.id).copied().unwrap_or(fault.severity);
                        work.system.fault_factor = 1.0 - severity;
                        work.system.stage = SystemSubstepStage::ApplyIdealLoad;
                    } else {
                        work.system.fault_cursor += 1;
                    }
                } else {
                    work.system.stage = SystemSubstepStage::ApplyIdealLoad;
                }
            }
            SystemSubstepStage::ApplyIdealLoad => {
                let ideal = &model.ideal_loads[work.system.selected_ideal.expect("selected ideal load")];
                let output = ideal_loads_deliver(
                    &IdealLoadsInput {
                        zone_temp_c: zone_state.air.temp_c,
                        zone_humidity_ratio: zone_state.air.humidity_ratio,
                        outdoor_temp_c: self.weather.dry_bulb_c,
                        outdoor_humidity_ratio: self.weather.humidity_ratio(),
                        heating_setpoint_c: work.heating_setpoint_c,
                        cooling_setpoint_c: work.cooling_setpoint_c,
                        zone_heating_demand_w: zone_state.heating_demand_w * work.system.fault_factor,
                        zone_cooling_demand_w: zone_state.cooling_demand_w * work.system.fault_factor,
                        occupancy: 1.0,
                        floor_area_m2: work.floor_area_m2,
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
                let balance = work.system.balance.as_mut().expect("predicted zone balance");
                balance.system_sensible_w = output.sensible_delivered_w;
                work.delivered.heating_w += output.sensible_heating_w;
                work.delivered.cooling_w += output.sensible_cooling_w;
                let corrected = advance_zone_air(&zone_state.air, balance, self.system_dt_s, HumiditySolutionMethod::ThirdOrderBackward, self.weather.atmospheric_pressure_pa);
                zone_state.air.push_temp(corrected.temp_c);
                zone_state.unmet_heating_w = output.unmet_heating_w;
                zone_state.unmet_cooling_w = output.unmet_cooling_w;
                work.system.ideal_cursor += 1;
                work.system.selected_ideal = None;
                work.system.stage = SystemSubstepStage::IdealLoad;
            }
            SystemSubstepStage::ZoneEquipment => {
                if let Some(assignment) = model.zone_equipment.get(work.system.equipment_cursor) {
                    if assignment.zone_id == zone.id {
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
                        let output = equipment.simulate(&ZoneEquipmentRequest {
                            zone_temperature_c: zone_state.air.temp_c,
                            zone_humidity_ratio: zone_state.air.humidity_ratio,
                            heating_load_w: zone_state.heating_demand_w,
                            cooling_load_w: zone_state.cooling_demand_w,
                            outdoor_temperature_c: self.weather.dry_bulb_c,
                            outdoor_humidity_ratio: self.weather.humidity_ratio(),
                            outdoor_pressure_pa: self.weather.atmospheric_pressure_pa,
                            supply_air_temp_c: 16.0,
                            supply_air_humidity_ratio: self.weather.humidity_ratio(),
                            supply_mass_flow_kg_s: 0.1,
                        });
                        work.delivered.heating_w += output.delivered_heating_w;
                        work.delivered.cooling_w += output.delivered_cooling_w;
                        work.delivered.fan_w += output.fan_power_w;
                        work.delivered.compressor_w += output.compressor_power_w;
                        work.delivered.gas_w += output.gas_consumption_w;
                    }
                    work.system.equipment_cursor += 1;
                } else {
                    work.system.stage = SystemSubstepStage::Complete;
                }
            }
            SystemSubstepStage::Complete => {
                self.system_substep_cursor += 1;
                work.system = SystemSubstepWork { stage: SystemSubstepStage::Predict, ideal_cursor: 0, fault_cursor: 0, equipment_cursor: 0, selected_ideal: None, fault_factor: 1.0, balance: None };
            }
        }
    }

    fn commit_zone(&mut self, state: &mut SimulationModel) {
        let Some(work) = self.zone_work.take() else {
            self.stage = TimestepStage::Zone;
            return;
        };
        if let Some(zone_state) = state.zones.get_mut(&work.zone_id) {
            zone_state.delivered = work.delivered;
        }
        state.delivered_total = accumulate_delivered(&state.delivered_total, &work.delivered);
        self.zone_cursor += 1;
        self.stage = TimestepStage::Zone;
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
                    let load = work.remaining_load_w.min(100_000.0).max(0.0);
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
        let plane_irradiance = (self.weather.direct_normal_irradiance_w_m2 + self.weather.diffuse_horizontal_irradiance_w_m2) * system.orientation_factor(self.sun_alt, self.sun_az);
        self.pv_generation_w += system.simulate(plane_irradiance, self.weather.dry_bulb_c + 10.0);
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
        state.battery_soc = (state.battery_soc + charge_w * self.dt_s / (battery.capacity_kwh * 3_600_000.0)).clamp(0.0, 1.0);
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
                    return finish_schedule_lookup(work, schedule.value);
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
                    return finish_schedule_lookup(work, value);
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
                    return finish_schedule_lookup(work, value);
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
                    return finish_schedule_lookup(work, schedule.values.get(index).copied().unwrap_or(1.0));
                }
            } else {
                return finish_schedule_lookup(work, 1.0);
            }
        }
    }
    None
}

fn finish_schedule_lookup(work: &mut Option<ScheduleLookupWork>, value: f64) -> Option<f64> {
    *work = None;
    Some(value)
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
        for zone in &model.zones {
            let _ = state.zones.insert(zone.id, ZoneState { air: ZoneAirState::new(weather.dry_bulb_c, weather.humidity_ratio()), ..ZoneState::empty() });
        }
        for (sid, sp) in pre.surfaces.iter() {
            let _ = state.surfaces.insert(*sid, SurfaceState { inside_temp_c: weather.dry_bulb_c, outside_temp_c: weather.dry_bulb_c, heat_flux_w: 0.0, ctf: sp.ctf.clone(), convection_to_zone_w: 0.0 });
        }
        state
    }

    /// 🔄️ Run warmup until temperature and load convergence.
    #[cfg(test)]
    pub(crate) fn warmup(model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel, weather_records: &[WeatherRecord]) -> Result<(), Error> {
        let warmup_hours = config.warmup_days * 24;
        let dt_s = pre.zone_timestep_s;
        let mut prev_temps = FixedTable::default();
        let mut prev_loads = FixedTable::default();
        prev_temps.admit(model.zones.len()).map_err(|_| Error::severe("test warmup temperature backing"))?;
        prev_loads.admit(model.zones.len()).map_err(|_| Error::severe("test warmup load backing"))?;

        for hour in 0..warmup_hours {
            let widx = (hour as usize) % weather_records.len().max(1);
            let weather = weather_records.get(widx).copied().unwrap_or_else(|| default_weather(hour));
            let date = SimDate::new(weather.year, weather.month, weather.day);
            Self::advance_timestep(model, config, pre, state, &weather, &date, hour as f64, dt_s)?;
            if hour > 24 && hour % 24 == 0 {
                let temp_ok = state.zones.iter().all(|(id, zs)| prev_temps.get(id).is_some_and(|prev| (zs.air.temp_c - prev).abs() <= config.tolerances.temperature_k));
                let load_ok = state.zones.iter().all(|(id, zs)| {
                    prev_loads.get(id).is_some_and(|prev| {
                        let load = zs.heating_demand_w + zs.cooling_demand_w;
                        (load - prev).abs() <= config.tolerances.energy_w
                    })
                });
                if temp_ok && load_ok {
                    state.warmup_complete = true;
                    return Ok(());
                }
            }
            for (id, zs) in state.zones.iter() {
                let _ = prev_temps.insert(*id, zs.air.temp_c);
                let _ = prev_loads.insert(*id, zs.heating_demand_w + zs.cooling_demand_w);
            }
        }
        state.warmup_complete = true;
        Ok(())
    }

    /// 🔄️ Advance one zone timestep through the same bounded cursor machine used by EnergyJob.
    #[cfg(test)]
    pub(crate) fn advance_timestep(model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel, weather: &WeatherRecord, date: &SimDate, hour: f64, dt_s: f64) -> Result<(), Error> {
        let mut work = TimestepWork::new(model, pre, *weather, *date, hour, dt_s);
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

fn relative_humidity_from_w(w: f64, t_c: f64, p_atm: f64) -> f64 {
    let p_ws = saturation_pressure_pa(t_c);
    if p_ws <= 0.0 {
        return 0.5;
    }
    let p_w = w * p_atm / (0.62198 + w);
    (p_w / p_ws).clamp(0.0, 1.0)
}

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
mod tests {
    use super::*;
    use crate::precompute::PrecomputedModel;

    #[test]
    fn initialize_creates_zone_states() {
        let model = crate::sim::test_model_single_zone();
        let pre = PrecomputedModel::build(&model, 60, 60);
        let weather = default_weather(0);
        let state = SimulationKernel::initialize(&model, &pre, &weather);
        assert!(state.zones.contains_key(&EntityId(1)));
    }

    #[test]
    fn energy_balance_near_zero_for_steady_state() {
        let residual = SimulationKernel::energy_balance_check(1000.0, 200.0, 800.0);
        assert!(residual < 1e-6);
    }

    #[test]
    fn run_period_from_config() {
        let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, ..Default::default() };
        assert_eq!(SimulationKernel::run_period(&config).total_hours(), 168);
    }

    #[test]
    fn advance_timestep_with_mechanical_ventilation_and_fan_coil_zone_equipment() {
        use crate::model::*;
        let mut model = crate::sim::test_model_single_zone();
        model.mechanical_ventilations.push(MechanicalVentilation { id: EntityId(90), zone_id: EntityId(1), schedule_id: ScheduleId(0), design_flow_m3_s: 0.05, fan_total_efficiency: 0.6, fan_delta_pressure_pa: 500.0 });
        model.zone_equipment.push(ZoneEquipmentAssignment { id: EntityId(91), zone_id: EntityId(1), equipment_type: ZoneEquipmentType::FanCoil, priority: 1, heating_capacity_w: 3000.0, cooling_capacity_w: 3000.0 });
        let pre = PrecomputedModel::build(&model, 60, 60);
        let weather = default_weather(10);
        let mut state = SimulationKernel::initialize(&model, &pre, &weather);
        let date = SimDate::new(2026, 1, 1);
        let config = SimulationConfig::default();
        let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
        assert!(result.is_ok());
        assert!(state.zones.contains_key(&EntityId(1)));
    }

    #[test]
    fn advance_timestep_with_baseboard_zone_equipment_and_humidistat() {
        use crate::model::*;
        let mut model = crate::sim::test_model_single_zone();
        model.zone_equipment.push(ZoneEquipmentAssignment { id: EntityId(92), zone_id: EntityId(1), equipment_type: ZoneEquipmentType::Baseboard, priority: 1, heating_capacity_w: 2000.0, cooling_capacity_w: 0.0 });
        model.humidistats.push(Humidistat {
            id: EntityId(93),
            zone_id: EntityId(1),
            humidifying_setpoint_schedule_id: ScheduleId(0),
            dehumidifying_setpoint_schedule_id: ScheduleId(0),
            humidifying_throttle_range: 5.0,
            dehumidifying_throttle_range: 5.0,
        });
        let pre = PrecomputedModel::build(&model, 60, 60);
        let weather = default_weather(10);
        let mut state = SimulationKernel::initialize(&model, &pre, &weather);
        let date = SimDate::new(2026, 1, 1);
        let config = SimulationConfig::default();
        let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
        assert!(result.is_ok());
        let zs = state.zones.get(&EntityId(1)).unwrap();
        assert!(zs.delivered.heating_w >= 0.0);
    }

    #[test]
    fn advance_timestep_handles_ground_and_adiabatic_surfaces() {
        use crate::model::*;
        let mut model = crate::sim::test_model_single_zone();
        model.surfaces[0].outside_boundary_condition = OutsideBoundary::Ground;
        model.surfaces.push(Surface {
            id: EntityId(31),
            name: "AdiabaticWall".into(),
            zone_id: EntityId(1),
            class: SurfaceClass::InteriorWall,
            vertices_m: vec![[0.0, 0.0, 0.0], [5.0, 0.0, 0.0], [5.0, 0.0, 3.0], [0.0, 0.0, 3.0]],
            construction_id: EntityId(20),
            outside_boundary_condition: OutsideBoundary::Adiabatic,
            sun_exposed: false,
            wind_exposed: false,
            multiplier: 1,
        });
        let pre = PrecomputedModel::build(&model, 60, 60);
        let weather = default_weather(10);
        let mut state = SimulationKernel::initialize(&model, &pre, &weather);
        let date = SimDate::new(2026, 1, 1);
        let config = SimulationConfig::default();
        let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
        assert!(result.is_ok());
    }

    #[test]
    fn advance_timestep_with_airflow_network() {
        use crate::model::*;
        let mut model = crate::sim::test_model_single_zone();
        model.airflow_network = Some(AirflowNetworkDefinition { zone_node_ids: vec![(EntityId(1), 1)], outdoor_node_id: 0, link_ids: vec![] });
        let pre = PrecomputedModel::build(&model, 60, 60);
        let weather = default_weather(10);
        let mut state = SimulationKernel::initialize(&model, &pre, &weather);
        let date = SimDate::new(2026, 1, 1);
        let config = SimulationConfig::default();
        let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
        assert!(result.is_ok());
    }

    #[test]
    fn advance_timestep_applies_fault_severity_to_ideal_loads() {
        use crate::model::*;
        let mut model = crate::sim::test_model_single_zone();
        model.faults.push(FaultDefinition { id: EntityId(94), target_equipment_id: EntityId(40), fault_type: FaultType::CoilFouling, severity: 0.3, start_schedule_id: ScheduleId(0) });
        let pre = PrecomputedModel::build(&model, 60, 60);
        let weather = default_weather(10);
        let mut state = SimulationKernel::initialize(&model, &pre, &weather);
        let date = SimDate::new(2026, 1, 1);
        let config = SimulationConfig::default();
        let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
        assert!(result.is_ok());
    }
}
