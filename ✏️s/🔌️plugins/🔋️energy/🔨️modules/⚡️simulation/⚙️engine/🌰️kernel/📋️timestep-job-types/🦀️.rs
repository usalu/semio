//! 📋️ The timestep stage vocabulary, the resumable work records, the builder and the cursor accessors of `TimestepWork`.

use super::*;

// #region 🔖️TimestepJob
/// 🧭️ Bounded phase of one persistent hourly timestep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum TimestepStage {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum ZonePreparationStage {
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

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct ZonePreparationWork {
    pub(super) stage: ZonePreparationStage,
    pub(super) cursor: usize,
    pub(super) zone_index: usize,
    pub(super) zone_id: EntityId,
    pub(super) floor_area_m2: f64,
    pub(super) zone_temp_c: f64,
    pub(super) zone_humidity_ratio: f64,
    pub(super) daylight_target_lux: f64,
    pub(super) daylight_transmittance: f64,
    pub(super) daylight_area_m2: f64,
    pub(super) lighting_dim: f64,
    pub(super) internal_gain: GainDecomposition,
    pub(super) infiltration_flow_m3_s: f64,
    pub(super) airflow_zone_node: Option<u32>,
    pub(super) airflow_flow_m3_s: f64,
    pub(super) mechanical_flow_m3_s: f64,
    pub(super) heating_setpoint_c: f64,
    pub(super) cooling_setpoint_c: f64,
    pub(super) thermostat_schedule: u8,
    pub(super) thermostat: ThermostatSpec,
    pub(super) humidistat: Option<HumidistatSpec>,
    pub(super) controlled: bool,
}

/// 🌡️ One zone's hourly boundary conditions resolved from its schedules.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(crate) struct ZoneLoads {
    pub(super) convective_w: f64,
    pub(super) radiant_w: f64,
    pub(super) latent_w: f64,
    pub(super) outdoor_air_m3_s: f64,
    pub(super) heating_setpoint_c: f64,
    pub(super) cooling_setpoint_c: f64,
    pub(super) floor_area_m2: f64,
    pub(super) controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum ScheduleLookupStage {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum BalanceStage {
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

pub const P7C1_TIMESTEP_STAGES: [TimestepStage; 10] = [
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

pub const P7C1_ZONE_PREPARATION_STAGES: [ZonePreparationStage; 15] = [
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

pub const P7C1_BALANCE_STAGES: [BalanceStage; 13] = [
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

pub const P7C1_PLANT_STAGES: [PlantStage; 4] = [PlantStage::ReduceZoneLoad, PlantStage::BuildPriority, PlantStage::Dispatch, PlantStage::Simulate];

pub const P7C1_SCHEDULE_LOOKUP_STAGES: [ScheduleLookupStage; 8] = [
    ScheduleLookupStage::Constant,
    ScheduleLookupStage::Annual,
    ScheduleLookupStage::AnnualHoliday,
    ScheduleLookupStage::AnnualRule,
    ScheduleLookupStage::ResolveDaily,
    ScheduleLookupStage::Weekly,
    ScheduleLookupStage::DirectDaily,
    ScheduleLookupStage::TimeSeries,
];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct ScheduleLookupWork {
    pub(super) requested_id: schedule::ScheduleId,
    pub(super) stage: ScheduleLookupStage,
    pub(super) cursor: usize,
    pub(super) annual_index: Option<usize>,
    pub(super) daily_id: Option<schedule::ScheduleId>,
    pub(super) daily_fallback: ScheduleLookupStage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum PlantStage {
    ReduceZoneLoad,
    BuildPriority,
    Dispatch,
    Simulate,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct PlantWork {
    pub(super) plant_index: usize,
    pub(super) stage: PlantStage,
    pub(super) zone_cursor: usize,
    pub(super) equipment_cursor: usize,
    pub(super) total_load_w: f64,
    pub(super) remaining_load_w: f64,
    pub(super) first_load_w: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct BatteryWork {
    pub(super) battery_index: usize,
    pub(super) zone_cursor: usize,
    pub(super) net_load_w: f64,
}

/// 🧮️ Cursor of one zone's heat-balance step.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct BalanceWork {
    pub(super) stage: BalanceStage,
    pub(super) zone_cursor: usize,
    pub(super) face_cursor: usize,
    pub(super) back_cursor: usize,
    pub(super) zone_diffuse_w: f64,
    pub(super) air_diagonal_w_k: f64,
    pub(super) air_rhs_w: f64,
    pub(super) mean_radiant_sums: [f64; 6],
    pub(super) free_temp_c: f64,
    pub(super) target_temp_c: Option<f64>,
    pub(super) required_w: f64,
    pub(super) delivered_w: f64,
    pub(super) remaining_heating_w: f64,
    pub(super) remaining_cooling_w: f64,
    pub(super) fault_factor: f64,
    pub(super) ideal_cursor: usize,
    pub(super) fault_cursor: usize,
    pub(super) equipment_cursor: usize,
    pub(super) selected_ideal: Option<usize>,
    pub(super) delivered: DeliveredEnergy,
}

impl BalanceWork {
    pub(super) fn new() -> Self {
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

    pub(super) fn next_zone(&mut self) {
        let zone_cursor = self.zone_cursor + 1;
        *self = Self { stage: BalanceStage::Clear, zone_cursor, ..Self::new() };
    }
}

/// 🌦️ The three hourly weather records one hour interpolates between.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct HourWeather {
    pub previous: WeatherRecord,
    pub current: WeatherRecord,
    pub next: WeatherRecord,
}

/// ⏱️ Cursor-owned execution state for one hourly timestep.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct TimestepWork {
    pub(super) stage: TimestepStage,
    pub(super) context: ScheduleContext,
    pub(super) weather: HourWeather,
    pub(super) date: SimDate,
    pub(super) hour: f64,
    pub(super) dt_s: f64,
    pub(super) substeps: u32,
    pub(super) substep_cursor: u32,
    pub(super) zone_steps: u32,
    pub(super) step_weather: TimestepWeather,
    pub(super) sky: SkyState,
    pub(super) sun_alt: f64,
    pub(super) sun_az: f64,
    pub(super) zone_cursor: usize,
    pub(super) secondary_cursor: usize,
    pub(super) zone_loads: Vec<ZoneLoads>,
    pub(super) zone_preparation: Option<ZonePreparationWork>,
    pub(super) balance: Option<BalanceWork>,
    pub(super) plant_work: Option<PlantWork>,
    pub(super) battery_work: Option<BatteryWork>,
    pub(super) schedule_lookup: Option<ScheduleLookupWork>,
    pub(super) pv_generation_w: f64,
}

/// 🧱️ Persistent backing constructor for one timestep.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct TimestepBuilder {
    pub(super) stage: u8,
    pub(super) cursor: usize,
    pub(super) context: ScheduleContext,
    pub(super) weather: HourWeather,
    pub(super) date: SimDate,
    pub(super) hour: f64,
    pub(super) zone_loads: Vec<ZoneLoads>,
}

pub const P7C1_TIMESTEP_BUILDER_STAGES: [u8; 3] = [0, 1, 2];

impl TimestepBuilder {
    pub fn retained_wire_signature(&self) -> [u64; 6] {
        [self.stage as u64, self.cursor as u64, self.zone_loads.len() as u64, self.zone_loads.capacity() as u64, self.date.day_of_year() as u64, self.hour.to_bits()]
    }

    pub fn new(weather: HourWeather, date: SimDate, hour: f64) -> Self {
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

    pub fn step(&mut self, model: &Model, pre: &PrecomputedModel) -> Result<Option<TimestepWork>, Error> {
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

    pub fn stage_for_gate(&self) -> u8 {
        self.stage
    }

    pub fn set_stage_for_gate(&mut self, stage: u8) {
        self.stage = stage;
    }

    pub fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 {
            return false;
        }
        self.zone_loads.pop().is_none()
    }
}

impl TimestepWork {
    pub fn retained_wire_signature(&self) -> [u64; 16] {
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

    pub fn new(model: &Model, pre: &PrecomputedModel, weather: HourWeather, date: SimDate, hour: f64) -> Self {
        let mut builder = TimestepBuilder::new(weather, date, hour);
        loop {
            if let Some(work) = builder.step(model, pre).expect("headless timestep admission") {
                return work;
            }
        }
    }

    pub fn stage(&self) -> TimestepStage {
        self.stage
    }

    pub fn zone_preparation_stage(&self) -> Option<ZonePreparationStage> {
        self.zone_preparation.as_ref().map(|work| work.stage)
    }

    pub fn balance_stage(&self) -> Option<BalanceStage> {
        self.balance.as_ref().map(|work| work.stage)
    }

    pub fn plant_stage(&self) -> Option<PlantStage> {
        self.plant_work.as_ref().map(|work| work.stage)
    }

    pub fn schedule_lookup_stage(&self) -> Option<ScheduleLookupStage> {
        self.schedule_lookup.as_ref().map(|work| work.stage)
    }

    pub fn set_stage_for_gate(&mut self, stage: TimestepStage) {
        self.stage = stage;
    }

    pub fn set_zone_preparation_stage_for_gate(&mut self, stage: ZonePreparationStage) -> bool {
        let Some(work) = self.zone_preparation.as_mut() else { return false };
        work.stage = stage;
        true
    }

    pub fn set_balance_stage_for_gate(&mut self, stage: BalanceStage) -> bool {
        let Some(work) = self.balance.as_mut() else { return false };
        work.stage = stage;
        true
    }

    pub fn set_plant_stage_for_gate(&mut self, stage: PlantStage) -> bool {
        let Some(work) = self.plant_work.as_mut() else { return false };
        work.stage = stage;
        true
    }

    pub fn set_schedule_lookup_stage_for_gate(&mut self, stage: ScheduleLookupStage) -> bool {
        let Some(work) = self.schedule_lookup.as_mut() else { return false };
        work.stage = stage;
        true
    }

    pub fn is_complete(&self) -> bool {
        self.stage == TimestepStage::Complete
    }

    pub fn close_step(&mut self, maximum_items: usize) -> bool {
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

    pub fn step(&mut self, model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel) {
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
}
// #endregion 🔖️TimestepJob
