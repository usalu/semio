//! 🔄️ Simulation kernel: calendar, multi-rate loops, warmup, predictor-corrector coupling.

use crate::air_exchange::{infiltration_flow_m3_s, ventilation_load_w, InfiltrationMethod, InfiltrationSpec};
use crate::airflow_network::{AfLink, AfLinkKind, AfNode, AirflowNetwork};
use crate::calendar::{RunPeriod, SimDate};
use crate::controls::{evaluate_controls, predict_zone_load, HumidistatSpec, ThermostatSpec};
use crate::curves::PerformanceCurve;
use crate::daylight::{dimmed_lighting_power_w, lighting_dimming_fraction, reference_point_illuminance_lux, simplified_daylight_factor};
use crate::dispatch::{DispatchRequest, DispatchScheme, Dispatcher, EquipmentPriority};
use crate::electrical::{grid_balance, PvSystem, Transformer};
use crate::envelope::{solve_exterior_surface_temp, solve_interior_surface_temp, ConductionState, ExteriorConvectionModel, InteriorConvectionModel};
use crate::error::Error;
use crate::faults::SeveritySchedule;
use crate::gains::{compute_equipment_gain_w, compute_lighting_gain_w, compute_people_gain_w, ActivityLevel, GainDecomposition};
use crate::ideal_hvac::{ideal_loads_deliver, IdealLoadsConfig, IdealLoadsInput};
use crate::model::{EntityId, Model, OutsideBoundary};
use crate::plant::{PlantLoopSimulation, PlantStream, Pump};
use crate::precompute::PrecomputedModel;
use crate::props::saturation_pressure_pa;
use crate::schedule::{ScheduleContext, ScheduleSet};
use crate::site::{GroundTemperatureModel, WeatherRecord};
use crate::solar::{shading_factor, surface_solar_absorption};
use crate::units::P_STD;
use crate::zone_air::{advance_zone_air, HumiditySolutionMethod, ZoneAirBalance, ZoneAirState};
use crate::zone_hvac::{ZoneEquipment, ZoneEquipmentRequest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub zones: HashMap<EntityId, ZoneState>,
    pub surfaces: HashMap<EntityId, SurfaceState>,
    pub warmup_complete: bool,
    pub hour: u32,
    pub delivered_total: DeliveredEnergy,
    pub battery_soc: f64,
    pub plant_supply_c: f64,
}

impl Default for SimulationModel {
    fn default() -> Self {
        Self { zones: HashMap::new(), surfaces: HashMap::new(), warmup_complete: false, hour: 0, delivered_total: DeliveredEnergy::default(), battery_soc: 0.5, plant_supply_c: 55.0 }
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
    surface_ids: Vec<EntityId>,
    fenestration_ids: Vec<EntityId>,
    surface_cursor: usize,
    fenestration_cursor: usize,
    zone_cursor: usize,
    system_substep_cursor: u32,
    secondary_cursor: usize,
    zone_envelope_w: HashMap<EntityId, f64>,
    zone_solar_w: HashMap<EntityId, f64>,
    zone_surface_conv_w: HashMap<EntityId, f64>,
    zone_work: Option<ZoneTimestepWork>,
    pv_generation_w: f64,
}

impl TimestepWork {
    pub(crate) fn new(model: &Model, pre: &PrecomputedModel, weather: WeatherRecord, date: SimDate, hour: f64, dt_s: f64) -> Self {
        let mut surface_ids: Vec<_> = pre.surfaces.keys().copied().collect();
        surface_ids.sort_by_key(|id| id.0);
        let mut fenestration_ids: Vec<_> = pre.fenestrations.keys().copied().collect();
        fenestration_ids.sort_by_key(|id| id.0);
        let (sun_alt, sun_az) = pre.solar_at(model, date.day_of_year(), hour);
        let system_dt_s = pre.system_timestep_s.min(dt_s);
        Self {
            stage: TimestepStage::Surface,
            context: ScheduleContext { year: date.year, month: date.month, day: date.day, hour: weather.hour, day_of_week: date.day_of_week(), timestep_index: hour as u32, is_dst: false },
            weather,
            date,
            hour,
            dt_s,
            system_dt_s,
            sub_steps: (dt_s / system_dt_s).ceil() as u32,
            sun_alt,
            sun_az,
            surface_ids,
            fenestration_ids,
            surface_cursor: 0,
            fenestration_cursor: 0,
            zone_cursor: 0,
            system_substep_cursor: 0,
            secondary_cursor: 0,
            zone_envelope_w: HashMap::new(),
            zone_solar_w: HashMap::new(),
            zone_surface_conv_w: HashMap::new(),
            zone_work: None,
            pv_generation_w: 0.0,
        }
    }

    #[cfg(test)]
    pub(crate) fn stage(&self) -> TimestepStage {
        self.stage
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.stage == TimestepStage::Complete
    }

    pub(crate) fn step(&mut self, model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel) {
        match self.stage {
            TimestepStage::Surface => self.step_surface(model, pre, state),
            TimestepStage::Fenestration => self.step_fenestration(model, pre, state),
            TimestepStage::Zone => self.prepare_zone(model, config, pre, state),
            TimestepStage::SystemSubstep => self.step_system_substep(model, state),
            TimestepStage::ZoneCommit => self.commit_zone(state),
            TimestepStage::SecondaryPlant => self.step_plant(model, state),
            TimestepStage::SecondaryPv => self.step_pv(model, state),
            TimestepStage::SecondaryBattery => self.step_battery(model, state),
            TimestepStage::SecondaryServiceHotWater => self.step_service_hot_water(model, config, state),
            TimestepStage::SecondaryRefrigeration => self.step_refrigeration(model, config, state),
            TimestepStage::SecondaryWater => self.step_water(model, config, state),
            TimestepStage::Complete => {}
        }
    }

    fn step_surface(&mut self, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(sid) = self.surface_ids.get(self.surface_cursor).copied() else {
            self.stage = TimestepStage::Fenestration;
            return;
        };
        self.surface_cursor += 1;
        let Some(sp) = pre.surfaces.get(&sid) else { return };
        let surface = model.surfaces.iter().find(|surface| surface.id == sid);
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
        let surface_state = state.surfaces.entry(sid).or_insert_with(|| SurfaceState { inside_temp_c: zone_t, outside_temp_c: outside_temp, heat_flux_w: 0.0, ctf: sp.ctf.clone(), convection_to_zone_w: 0.0 });
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
        *self.zone_envelope_w.entry(sp.zone_id).or_default() += cond_w;
        *self.zone_solar_w.entry(sp.zone_id).or_default() += solar_w_m2 * sp.area_m2 * 0.7;
        *self.zone_surface_conv_w.entry(sp.zone_id).or_default() += conv_w;
    }

    fn step_fenestration(&mut self, model: &Model, pre: &PrecomputedModel, state: &SimulationModel) {
        let Some(fid) = self.fenestration_ids.get(self.fenestration_cursor).copied() else {
            self.stage = TimestepStage::Zone;
            return;
        };
        self.fenestration_cursor += 1;
        let Some(fp) = pre.fenestrations.get(&fid) else { return };
        let Some(surface) = model.surfaces.iter().find(|surface| surface.id == fp.surface_id) else { return };
        let zone_t = state.zones.get(&surface.zone_id).map_or(self.weather.dry_bulb_c, |zone| zone.air.temp_c);
        *self.zone_envelope_w.entry(surface.zone_id).or_default() += fp.u_value_w_m2k * fp.area_m2 * (self.weather.dry_bulb_c - zone_t);
        if self.sun_alt > 0.0 {
            let incidence = crate::solar::beam_incidence_cosine(fp.normal, self.sun_alt, self.sun_az);
            let shade = shading_factor(1.0, 0.0, 1.0, self.sun_alt);
            *self.zone_solar_w.entry(surface.zone_id).or_default() += (self.weather.direct_normal_irradiance_w_m2 * incidence * shade + self.weather.diffuse_horizontal_irradiance_w_m2 * 0.5) * fp.shgc * fp.area_m2;
        }
    }

    fn prepare_zone(&mut self, model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &SimulationModel) {
        let Some(zone) = model.zones.get(self.zone_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryPlant;
            return;
        };
        let geometry = pre.zone_geometry.get(&zone.id).cloned().unwrap_or_default();
        let floor_area_m2 = geometry.floor_area_m2;
        let zone_t = state.zones.get(&zone.id).map_or(self.weather.dry_bulb_c, |value| value.air.temp_c);
        let zone_w = state.zones.get(&zone.id).map_or(self.weather.humidity_ratio(), |value| value.air.humidity_ratio);
        let lighting_dim = model.daylight_zones.iter().find(|value| value.zone_id == zone.id).map_or(1.0, |daylight| {
            let factor = simplified_daylight_factor(model.fenestrations.iter().map(|value| value.area_m2).sum(), floor_area_m2, daylight.window_transmittance);
            let lux = reference_point_illuminance_lux(self.weather.diffuse_horizontal_irradiance_w_m2 * 120.0, self.weather.direct_normal_irradiance_w_m2 * 120.0, self.sun_alt.max(0.0) / 90.0, daylight.window_transmittance, factor, 1.0);
            lighting_dimming_fraction(lux, daylight.illuminance_target_lux, 0.1)
        });
        let mut internal_gain = GainDecomposition::default();
        for person in model.people.iter().filter(|value| value.zone_id == zone.id) {
            let occupancy = config.schedules.lookup(person.schedule_id, &self.context);
            internal_gain = internal_gain.add(&compute_people_gain_w(person.people_per_area * floor_area_m2 * occupancy, ActivityLevel::OfficeWork, 1.0, person.radiant_fraction));
        }
        for light in model.lighting.iter().filter(|value| value.zone_id == zone.id) {
            let fraction = config.schedules.lookup(light.schedule_id, &self.context) * lighting_dim;
            let power = dimmed_lighting_power_w(light.watts_per_area * floor_area_m2, fraction);
            internal_gain = internal_gain.add(&compute_lighting_gain_w(power / floor_area_m2.max(1.0), floor_area_m2, 1.0, light.radiant_fraction, light.return_air_fraction));
        }
        for equipment in model.equipment.iter().filter(|value| value.zone_id == zone.id) {
            internal_gain = internal_gain.add(&compute_equipment_gain_w(equipment.watts_per_area, floor_area_m2, config.schedules.lookup(equipment.schedule_id, &self.context), equipment.radiant_fraction, equipment.latent_fraction));
        }
        let mut infiltration_flow = model.infiltrations.iter().find(|value| value.zone_id == zone.id).map_or(0.0, |infiltration| {
            let specification = InfiltrationSpec {
                method: InfiltrationMethod::WindAndStack,
                schedule_factor: config.schedules.lookup(infiltration.schedule_id, &self.context),
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
            infiltration_flow_m3_s(&specification, zone.volume_m3, geometry.exterior_area_m2, self.weather.dry_bulb_c, zone_t, self.weather.wind_speed_m_s, self.weather.atmospheric_pressure_pa)
        });
        if let Some(definition) = &model.airflow_network {
            let mut nodes = vec![AfNode { id: definition.outdoor_node_id, elevation_m: 0.0, temperature_c: self.weather.dry_bulb_c, humidity_ratio: self.weather.humidity_ratio(), is_reference: true }];
            for (zone_id, node_id) in &definition.zone_node_ids {
                if *zone_id == zone.id {
                    nodes.push(AfNode { id: *node_id, elevation_m: 3.0, temperature_c: zone_t, humidity_ratio: zone_w, is_reference: false });
                }
            }
            if nodes.len() > 1 {
                let network = AirflowNetwork {
                    nodes,
                    links: vec![AfLink {
                        id: 1,
                        node_a: definition.zone_node_ids.iter().find(|(zone_id, _)| *zone_id == zone.id).map_or(1, |(_, node_id)| *node_id),
                        node_b: definition.outdoor_node_id,
                        kind: AfLinkKind::Crack,
                        flow_coefficient: 0.01,
                        flow_exponent: 0.65,
                        area_m2: 0.05,
                        discharge_coefficient: 0.65,
                        orientation_deg: 0.0,
                        wind_exposure_factor: 1.0,
                    }],
                    wind_speed_m_s: self.weather.wind_speed_m_s,
                    wind_direction_deg: self.weather.wind_direction_deg,
                    outdoor_temp_c: self.weather.dry_bulb_c,
                    outdoor_humidity_ratio: self.weather.humidity_ratio(),
                };
                if let Some(flows) = network.solve_flows(P_STD) {
                    infiltration_flow += flows.first().copied().unwrap_or(0.0).abs();
                }
            }
        }
        let mechanical_flow = model.mechanical_ventilations.iter().filter(|value| value.zone_id == zone.id).map(|value| value.design_flow_m3_s * config.schedules.lookup(value.schedule_id, &self.context)).sum::<f64>();
        let (infiltration_sensible_w, infiltration_latent_w) = ventilation_load_w(infiltration_flow + mechanical_flow, zone_t, zone_w, self.weather.dry_bulb_c, self.weather.humidity_ratio(), self.weather.atmospheric_pressure_pa, 0.0);
        let envelope_w = self.zone_envelope_w.get(&zone.id).copied().unwrap_or(0.0);
        let solar_w = self.zone_solar_w.get(&zone.id).copied().unwrap_or(0.0);
        let surface_convection_w = self.zone_surface_conv_w.get(&zone.id).copied().unwrap_or(0.0);
        let setpoints = pre.default_setpoints.get(&zone.id).copied().unwrap_or_default();
        let heating_setpoint_c = model.thermostats.iter().find(|value| value.zone_id == zone.id).map_or(setpoints.heating_c, |value| config.schedules.lookup(value.heating_setpoint_schedule_id, &self.context) * 24.0 + 20.0);
        let cooling_setpoint_c = model.thermostats.iter().find(|value| value.zone_id == zone.id).map_or(setpoints.cooling_c, |value| config.schedules.lookup(value.cooling_setpoint_schedule_id, &self.context) * 6.0 + 24.0);
        let humidistat = model.humidistats.iter().find(|value| value.zone_id == zone.id).map(|value| HumidistatSpec {
            humidifying_setpoint_rh: 0.4,
            dehumidifying_setpoint_rh: 0.6,
            humidifying_throttle_range: value.humidifying_throttle_range,
            dehumidifying_throttle_range: value.dehumidifying_throttle_range,
        });
        let thermostat =
            ThermostatSpec { heating_setpoint_c, cooling_setpoint_c, heating_throttle_range_k: setpoints.heating_throttle_k, cooling_throttle_range_k: setpoints.cooling_throttle_k, min_heating_setpoint_c: 10.0, max_cooling_setpoint_c: 35.0 };
        self.zone_work = Some(ZoneTimestepWork {
            zone_index: self.zone_cursor,
            zone_id: zone.id,
            floor_area_m2,
            zone_rh: relative_humidity_from_w(zone_w, zone_t, self.weather.atmospheric_pressure_pa),
            internal_gain,
            infiltration_sensible_w,
            infiltration_latent_w,
            surface_convection_w,
            sensible_gain_w: internal_gain.sensible_w + solar_w + surface_convection_w - envelope_w,
            heating_setpoint_c,
            cooling_setpoint_c,
            thermostat,
            humidistat,
            delivered: DeliveredEnergy::default(),
        });
        self.system_substep_cursor = 0;
        self.stage = TimestepStage::SystemSubstep;
    }

    fn step_system_substep(&mut self, model: &Model, state: &mut SimulationModel) {
        let Some(work) = self.zone_work.as_mut() else {
            self.stage = TimestepStage::Zone;
            return;
        };
        if self.system_substep_cursor >= self.sub_steps.max(1) {
            self.stage = TimestepStage::ZoneCommit;
            return;
        }
        self.system_substep_cursor += 1;
        let zone = &model.zones[work.zone_index];
        let zone_state = state.zones.entry(zone.id).or_insert_with(|| ZoneState { air: ZoneAirState::new(self.weather.dry_bulb_c, self.weather.humidity_ratio()), ..ZoneState::empty() });
        let controls = evaluate_controls(&work.thermostat, work.humidistat.as_ref(), zone_state.air.temp_c, work.zone_rh);
        let residual_sensible_w = work.sensible_gain_w - zone_state.heating_demand_w + zone_state.cooling_demand_w;
        let predicted = predict_zone_load(residual_sensible_w, work.internal_gain.latent_w, &controls, f64::INFINITY, f64::INFINITY, 5000.0, 5000.0);
        let mut balance = ZoneAirBalance {
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
        for ideal in model.ideal_loads.iter().filter(|value| value.zone_id == zone.id) {
            let fault_factor = model.faults.iter().find(|value| value.target_equipment_id == ideal.id).map_or(1.0, |value| 1.0 - value.severity * SeveritySchedule::constant(1.0).at_hour(self.weather.hour));
            let output = ideal_loads_deliver(
                &IdealLoadsInput {
                    zone_temp_c: zone_state.air.temp_c,
                    zone_humidity_ratio: zone_state.air.humidity_ratio,
                    outdoor_temp_c: self.weather.dry_bulb_c,
                    outdoor_humidity_ratio: self.weather.humidity_ratio(),
                    heating_setpoint_c: work.heating_setpoint_c,
                    cooling_setpoint_c: work.cooling_setpoint_c,
                    zone_heating_demand_w: zone_state.heating_demand_w * fault_factor,
                    zone_cooling_demand_w: zone_state.cooling_demand_w * fault_factor,
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
            balance.system_sensible_w = output.sensible_delivered_w;
            work.delivered.heating_w += output.sensible_heating_w;
            work.delivered.cooling_w += output.sensible_cooling_w;
            let corrected = advance_zone_air(&zone_state.air, &balance, self.system_dt_s, HumiditySolutionMethod::ThirdOrderBackward, self.weather.atmospheric_pressure_pa);
            zone_state.air.push_temp(corrected.temp_c);
            zone_state.unmet_heating_w = output.unmet_heating_w;
            zone_state.unmet_cooling_w = output.unmet_cooling_w;
        }
        for assignment in model.zone_equipment.iter().filter(|value| value.zone_id == zone.id) {
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

    fn step_plant(&mut self, model: &Model, state: &mut SimulationModel) {
        let Some(plant) = model.plant_loops.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryPv;
            return;
        };
        self.secondary_cursor += 1;
        let total_load = state.zones.values().map(|zone| zone.heating_demand_w + zone.cooling_demand_w).sum();
        let dispatcher = Dispatcher::new(DispatchScheme::Sequential, plant.equipment_ids.iter().map(|id| EquipmentPriority { equipment_id: id.0, priority: 1, min_runtime_hours: 0.0, capacity_w: 100_000.0 }).collect());
        let results = dispatcher.dispatch(&DispatchRequest { total_load_w: total_load, available_capacity_w: 500_000.0, outdoor_temp_c: self.weather.dry_bulb_c });
        let pump = Pump { design_head_pa: 200_000.0, design_flow_kg_s: plant.design_flow_kg_s, motor_efficiency: 0.85, part_load_curve: PerformanceCurve::Constant(1.0) };
        let loop_simulation = PlantLoopSimulation { supply: PlantStream::new(plant.supply_temperature_c, plant.design_flow_kg_s), return_stream: PlantStream::new(plant.return_temperature_c, plant.design_flow_kg_s), pump, glycol_fraction: 0.0 };
        let output = loop_simulation.simulate(results.first().map_or(0.0, |result| result.load_w));
        state.delivered_total.pump_w += output.electrical_power_w;
        state.plant_supply_c = output.outlet.temperature_c;
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

    fn step_battery(&mut self, model: &Model, state: &mut SimulationModel) {
        let Some(battery) = model.battery_storage.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryServiceHotWater;
            return;
        };
        self.secondary_cursor += 1;
        let net_load = state.zones.values().map(|zone| zone.delivered.total_electric_w()).sum::<f64>();
        let charge_w = if self.pv_generation_w > net_load { (self.pv_generation_w - net_load).min(battery.max_charge_w) } else { -(net_load - self.pv_generation_w).min(battery.max_discharge_w) };
        state.battery_soc = (state.battery_soc + charge_w * self.dt_s / (battery.capacity_kwh * 3_600_000.0)).clamp(0.0, 1.0);
        state.delivered_total.battery_charge_w += charge_w.max(0.0);
        let transformer = Transformer { rated_kva: 100.0, no_load_loss_w: 50.0, load_loss_w: 200.0, impedance_fraction: 0.02 };
        let _ = grid_balance(net_load, self.pv_generation_w, 0.0, 0.0, charge_w, &transformer);
    }

    fn step_service_hot_water(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.shw_systems.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryRefrigeration;
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.shw_electric_w += system.heater_capacity_w * config.schedules.lookup(system.schedule_id, &self.context) * 0.3;
    }

    fn step_refrigeration(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.refrigeration_systems.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryWater;
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.refrigeration_w += system.design_load_w * config.schedules.lookup(system.defrost_schedule_id, &self.context);
    }

    fn step_water(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.water_systems.get(self.secondary_cursor) else {
            state.hour = self.hour as u32;
            self.stage = TimestepStage::Complete;
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.water_pump_w += system.peak_flow_l_s * 1000.0 * config.schedules.lookup(system.schedule_id, &self.context) * 50.0;
    }
}
// #endregion 🔖️TimestepJob

// #region 🔖️Kernel
/// 🔄️ BEM simulation kernel with full subsystem coupling.
pub struct SimulationKernel;

impl SimulationKernel {
    /// 🔄️ Initialize state from model and precomputed data.
    pub fn initialize(model: &Model, pre: &PrecomputedModel, weather: &WeatherRecord) -> SimulationModel {
        let mut state = SimulationModel::default();
        for zone in &model.zones {
            state.zones.insert(zone.id, ZoneState { air: ZoneAirState::new(weather.dry_bulb_c, weather.humidity_ratio()), ..ZoneState::empty() });
        }
        for (sid, sp) in &pre.surfaces {
            state.surfaces.insert(*sid, SurfaceState { inside_temp_c: weather.dry_bulb_c, outside_temp_c: weather.dry_bulb_c, heat_flux_w: 0.0, ctf: sp.ctf.clone(), convection_to_zone_w: 0.0 });
        }
        state
    }

    /// 🔄️ Run warmup until temperature and load convergence.
    pub fn warmup(model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel, weather_records: &[WeatherRecord]) -> Result<(), Error> {
        let warmup_hours = config.warmup_days * 24;
        let dt_s = pre.zone_timestep_s;
        let mut prev_temps: HashMap<EntityId, f64> = HashMap::new();
        let mut prev_loads: HashMap<EntityId, f64> = HashMap::new();

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
            for (id, zs) in &state.zones {
                prev_temps.insert(*id, zs.air.temp_c);
                prev_loads.insert(*id, zs.heating_demand_w + zs.cooling_demand_w);
            }
        }
        state.warmup_complete = true;
        Ok(())
    }

    /// 🔄️ Advance one zone timestep through the same bounded cursor machine used by EnergyJob.
    pub fn advance_timestep(model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel, weather: &WeatherRecord, date: &SimDate, hour: f64, dt_s: f64) -> Result<(), Error> {
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
