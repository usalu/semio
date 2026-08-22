//! 🚀️ Engine orchestration: Model + SimulationConfig → Results.

use crate::economics::{compute_lcca, LccaParameters, UtilityTariff};
use crate::error::Error;
use crate::kernel::{SimulationConfig, SimulationEnvironment, SimulationKernel, SimulationModel, SurfaceState, TimestepWork, ZoneState};
use crate::meters::{EndUse, FuelType, MeterTable};
use crate::metrics::{compute_environmental, compute_resilience, EmissionFactors, EnvironmentalMetrics, ResilienceMetrics, SourceEnergyFactors};
use crate::model::Model;
use crate::output::TimeSeriesTable;
use crate::precompute::{PrecomputeBuilder, PrecomputedModel};
use crate::results::{Results, RunMetadata, SizingTables, SummaryTables};
use crate::site::WeatherRecord;
use crate::sizing::{SizingBuilder, SizingConfig};
use crate::units::Unit;
use crate::zone_air::ZoneAirState;
use semio_framework_job::{
    allocate_operation_id, default_now_ms, run_to_completion, BatchDriveConfig, BatchJobParams, CancelToken, Checkpoint, CommitCandidate, Generation, InteractiveJob, InteractiveStage, JobFault, Operation, RevisionId, StepContext, StepOutcome,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

// #region 🔖️EnergyJob
/// ⚡️ Fidelity label carried by every preview so provisional fields cannot be mistaken for a
/// validated final simulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyQualityTier {
    SteadyStateEstimate,
    DesignDay,
    CoarseTimestep,
    Final,
}

/// 🧭️ Persistent stage of [`EnergyJob`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyJobStage {
    Validate,
    ResolveWeather,
    Precompute,
    InitializeZones,
    InitializeSurfaces,
    WarmupTimestep,
    StartRun,
    RunZoneTimestep,
    AggregateZone,
    AggregateFacility,
    PublishTimestep,
    Finalize,
    Size,
    FinalizeSummaries,
    FinalizeMetrics,
    FinalizeEconomics,
    BuildResults,
    PublishFinal,
    EncodeOutput,
    Complete,
}

/// 📸️ Typed view of the latest replaceable energy preview.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnergyJobPreview {
    pub sequence: u64,
    pub tier: EnergyQualityTier,
    pub stage: EnergyJobStage,
    pub warmup_hour: u32,
    pub timestep: u32,
    pub total_timesteps: u32,
    pub zone_temperatures_c: Vec<(u32, f64)>,
    pub zone_heating_w: Vec<(u32, f64)>,
    pub zone_cooling_w: Vec<(u32, f64)>,
    pub facility_electricity_kwh: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EnergyCheckpointState {
    base_revision: u64,
    generation: u64,
    seed: u64,
    stage: EnergyJobStage,
    tier: EnergyQualityTier,
    weather: Vec<WeatherRecord>,
    weather_cursor: usize,
    weather_target: usize,
    precompute: Option<PrecomputeBuilder>,
    pre: Option<PrecomputedModel>,
    state: Option<SimulationModel>,
    initialize_cursor: usize,
    warmup_hour: u32,
    previous_temperatures: HashMap<crate::model::EntityId, f64>,
    previous_loads: HashMap<crate::model::EntityId, f64>,
    run_hours: Option<crate::calendar::RunPeriodHours>,
    total_timesteps: u32,
    hour_index: u32,
    aggregate_zone_cursor: usize,
    timestep_work: Option<TimestepWork>,
    rng_state: u64,
    checkpoint_due: bool,
    time_series: TimeSeriesTable,
    meters: MeterTable,
    time_series_order: Vec<String>,
    meter_order: Vec<String>,
    zone_temperature_history: Vec<f64>,
    last_preview: Option<EnergyJobPreview>,
    result: Option<Results>,
    sizing_builder: Option<SizingBuilder>,
    final_sizing: Option<SizingTables>,
    final_summaries: SummaryTables,
    final_environmental: Option<EnvironmentalMetrics>,
    final_resilience: Option<ResilienceMetrics>,
    commit_output: Vec<u8>,
    encode_section: u8,
    encode_record_cursor: usize,
    encode_sample_cursor: usize,
}

/// ⚡️ Resumable simulation operation. Weather decoding, model precomputation, initialization,
/// warmup, metering and publication are cursor-owned; the batch API drives this same job.
pub struct EnergyJob {
    operation: Operation,
    model: Model,
    config: SimulationConfig,
    stage: EnergyJobStage,
    tier: EnergyQualityTier,
    weather: Vec<WeatherRecord>,
    weather_cursor: usize,
    weather_target: usize,
    precompute: Option<PrecomputeBuilder>,
    pre: Option<PrecomputedModel>,
    state: Option<SimulationModel>,
    initialize_cursor: usize,
    warmup_hour: u32,
    previous_temperatures: HashMap<crate::model::EntityId, f64>,
    previous_loads: HashMap<crate::model::EntityId, f64>,
    run_hours: Option<crate::calendar::RunPeriodHours>,
    total_timesteps: u32,
    hour_index: u32,
    aggregate_zone_cursor: usize,
    timestep_work: Option<TimestepWork>,
    rng_state: u64,
    checkpoint_due: bool,
    time_series: TimeSeriesTable,
    meters: MeterTable,
    time_series_order: Vec<String>,
    meter_order: Vec<String>,
    zone_temperature_history: Vec<f64>,
    last_preview: Option<EnergyJobPreview>,
    result: Option<Results>,
    sizing_builder: Option<SizingBuilder>,
    final_sizing: Option<SizingTables>,
    final_summaries: SummaryTables,
    final_environmental: Option<EnvironmentalMetrics>,
    final_resilience: Option<ResilienceMetrics>,
    commit_output: Vec<u8>,
    encode_section: u8,
    encode_record_cursor: usize,
    encode_sample_cursor: usize,
    started: Instant,
}

impl EnergyJob {
    pub fn new(operation: Operation, model: Model, config: SimulationConfig) -> Self {
        Self {
            operation,
            model,
            config,
            stage: EnergyJobStage::Validate,
            tier: EnergyQualityTier::SteadyStateEstimate,
            weather: Vec::new(),
            weather_cursor: 0,
            weather_target: 0,
            precompute: None,
            pre: None,
            state: None,
            initialize_cursor: 0,
            warmup_hour: 0,
            previous_temperatures: HashMap::new(),
            previous_loads: HashMap::new(),
            run_hours: None,
            total_timesteps: 0,
            hour_index: 0,
            aggregate_zone_cursor: 0,
            timestep_work: None,
            rng_state: 0x9e37_79b9_7f4a_7c15 ^ operation.seed,
            checkpoint_due: false,
            time_series: TimeSeriesTable::default(),
            meters: MeterTable::default(),
            time_series_order: Vec::new(),
            meter_order: Vec::new(),
            zone_temperature_history: Vec::new(),
            last_preview: None,
            result: None,
            sizing_builder: None,
            final_sizing: None,
            final_summaries: SummaryTables::default(),
            final_environmental: None,
            final_resilience: None,
            commit_output: Vec::new(),
            encode_section: 0,
            encode_record_cursor: 0,
            encode_sample_cursor: 0,
            started: Instant::now(),
        }
    }

    pub fn stage(&self) -> EnergyJobStage {
        self.stage
    }

    pub fn preview(&self) -> Option<&EnergyJobPreview> {
        self.last_preview.as_ref()
    }

    pub fn take_results(&mut self) -> Option<Results> {
        self.result.take()
    }

    /// ♻️ Restores every mutable simulation accumulator and cursor from an owned checkpoint.
    pub fn from_checkpoint(operation: Operation, model: Model, config: SimulationConfig, bytes: &[u8]) -> Result<Self, Error> {
        let payload = bytes.strip_prefix(b"ENERGY2").ok_or_else(|| Error::severe("invalid energy checkpoint header"))?;
        let checkpoint: EnergyCheckpointState = serde_json::from_slice(payload).map_err(|error| Error::severe(format!("invalid energy checkpoint: {error}")))?;
        if checkpoint.base_revision != operation.base_revision.0 || checkpoint.generation != operation.generation.0 || checkpoint.seed != operation.seed {
            return Err(Error::severe("energy checkpoint operation mismatch"));
        }
        Ok(Self {
            operation,
            model,
            config,
            stage: checkpoint.stage,
            tier: checkpoint.tier,
            weather: checkpoint.weather,
            weather_cursor: checkpoint.weather_cursor,
            weather_target: checkpoint.weather_target,
            precompute: checkpoint.precompute,
            pre: checkpoint.pre,
            state: checkpoint.state,
            initialize_cursor: checkpoint.initialize_cursor,
            warmup_hour: checkpoint.warmup_hour,
            previous_temperatures: checkpoint.previous_temperatures,
            previous_loads: checkpoint.previous_loads,
            run_hours: checkpoint.run_hours,
            total_timesteps: checkpoint.total_timesteps,
            hour_index: checkpoint.hour_index,
            aggregate_zone_cursor: checkpoint.aggregate_zone_cursor,
            timestep_work: checkpoint.timestep_work,
            rng_state: checkpoint.rng_state,
            checkpoint_due: checkpoint.checkpoint_due,
            time_series: checkpoint.time_series,
            meters: checkpoint.meters,
            time_series_order: checkpoint.time_series_order,
            meter_order: checkpoint.meter_order,
            zone_temperature_history: checkpoint.zone_temperature_history,
            last_preview: checkpoint.last_preview,
            result: checkpoint.result,
            sizing_builder: checkpoint.sizing_builder,
            final_sizing: checkpoint.final_sizing,
            final_summaries: checkpoint.final_summaries,
            final_environmental: checkpoint.final_environmental,
            final_resilience: checkpoint.final_resilience,
            commit_output: checkpoint.commit_output,
            encode_section: checkpoint.encode_section,
            encode_record_cursor: checkpoint.encode_record_cursor,
            encode_sample_cursor: checkpoint.encode_sample_cursor,
            started: Instant::now(),
        })
    }

    fn set_stage(&mut self, context: &mut StepContext<'_>, stage: EnergyJobStage) {
        self.stage = stage;
        context.set_stage(stage.label());
    }

    fn weather_record(&self, index: usize) -> WeatherRecord {
        if let Some(epw) = &self.config.weather {
            if let Some(record) = epw.records.get(index) {
                return *record;
            }
        }
        match self.config.environment {
            SimulationEnvironment::HeatingDesignDay => design_day_hour(index as u32, -10.0),
            SimulationEnvironment::CoolingDesignDay => design_day_hour(index as u32, 35.0),
            SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => synthetic_hour(index as u32),
        }
    }

    fn publish_preview(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        let sequence = context.next_preview_sequence();
        let mut zone_temperatures_c = Vec::new();
        let mut zone_heating_w = Vec::new();
        let mut zone_cooling_w = Vec::new();
        if let Some(state) = &self.state {
            let mut zone_ids: Vec<_> = state.zones.keys().copied().collect();
            zone_ids.sort_by_key(|id| id.0);
            for id in zone_ids {
                if let Some(zone) = state.zones.get(&id) {
                    zone_temperatures_c.push((id.0, zone.air.temp_c));
                    zone_heating_w.push((id.0, zone.delivered.heating_w));
                    zone_cooling_w.push((id.0, zone.delivered.cooling_w));
                }
            }
        }
        let preview = EnergyJobPreview {
            sequence,
            tier: self.tier,
            stage: self.stage,
            warmup_hour: self.warmup_hour,
            timestep: self.hour_index,
            total_timesteps: self.total_timesteps,
            zone_temperatures_c,
            zone_heating_w,
            zone_cooling_w,
            facility_electricity_kwh: self.meters.facility_total_kwh(FuelType::Electricity),
        };
        let payload = encode_preview(&preview);
        self.last_preview = Some(preview);
        StepOutcome::PreviewReady(payload)
    }

    fn checkpoint(&self) -> StepOutcome {
        StepOutcome::CheckpointReady(Checkpoint { state: self.encode_state(false), applied_progress: self.warmup_hour as u64 + self.hour_index as u64 })
    }

    fn encode_state(&self, complete: bool) -> Vec<u8> {
        let checkpoint = EnergyCheckpointState {
            base_revision: self.operation.base_revision.0,
            generation: self.operation.generation.0,
            seed: self.operation.seed,
            stage: if complete { EnergyJobStage::Complete } else { self.stage },
            tier: self.tier,
            weather: self.weather.clone(),
            weather_cursor: self.weather_cursor,
            weather_target: self.weather_target,
            precompute: self.precompute.clone(),
            pre: self.pre.clone(),
            state: self.state.clone(),
            initialize_cursor: self.initialize_cursor,
            warmup_hour: self.warmup_hour,
            previous_temperatures: self.previous_temperatures.clone(),
            previous_loads: self.previous_loads.clone(),
            run_hours: self.run_hours.clone(),
            total_timesteps: self.total_timesteps,
            hour_index: self.hour_index,
            aggregate_zone_cursor: self.aggregate_zone_cursor,
            timestep_work: self.timestep_work.clone(),
            rng_state: self.rng_state,
            checkpoint_due: self.checkpoint_due,
            time_series: self.time_series.clone(),
            meters: self.meters.clone(),
            time_series_order: self.time_series_order.clone(),
            meter_order: self.meter_order.clone(),
            zone_temperature_history: self.zone_temperature_history.clone(),
            last_preview: self.last_preview.clone(),
            result: self.result.clone(),
            sizing_builder: self.sizing_builder.clone(),
            final_sizing: self.final_sizing.clone(),
            final_summaries: self.final_summaries.clone(),
            final_environmental: self.final_environmental.clone(),
            final_resilience: self.final_resilience.clone(),
            commit_output: self.commit_output.clone(),
            encode_section: self.encode_section,
            encode_record_cursor: self.encode_record_cursor,
            encode_sample_cursor: self.encode_sample_cursor,
        };
        let mut bytes = b"ENERGY2".to_vec();
        bytes.extend(serde_json::to_vec(&checkpoint).expect("energy checkpoint state is serializable"));
        bytes
    }

    fn fault(error: &Error) -> StepOutcome {
        StepOutcome::Fault(JobFault { detail: error.to_string().into_bytes() })
    }

    fn aggregate_zone(&mut self) {
        let Some(zone) = self.model.zones.get(self.aggregate_zone_cursor) else {
            return;
        };
        let Some(state) = &self.state else { return };
        let Some(zone_state) = state.zones.get(&zone.id) else { return };
        let key = format!("Zone Air Temperature [{}]", zone.name);
        if self.hour_index == 0 {
            self.time_series_order.push(key.clone());
        }
        self.time_series.record(&key, self.hour_index as f64, zone_state.air.temp_c, Unit::Celsius);
        self.zone_temperature_history.push(zone_state.air.temp_c);
        let heating_name = format!("{} Heating", zone.name);
        let cooling_name = format!("{} Cooling", zone.name);
        let fan_name = format!("{} Fans", zone.name);
        if self.hour_index == 0 {
            self.meter_order.extend([heating_name.clone(), cooling_name.clone(), fan_name.clone()]);
        }
        self.meters.get_or_create(&heating_name, FuelType::Electricity, EndUse::Heating).accumulate(zone_state.delivered.heating_w, self.pre.as_ref().map_or(0.0, |pre| pre.zone_timestep_s), self.hour_index as f64);
        self.meters.get_or_create(&cooling_name, FuelType::Electricity, EndUse::Cooling).accumulate(zone_state.delivered.cooling_w, self.pre.as_ref().map_or(0.0, |pre| pre.zone_timestep_s), self.hour_index as f64);
        self.meters.get_or_create(&fan_name, FuelType::Electricity, EndUse::Fans).accumulate(zone_state.delivered.fan_w, self.pre.as_ref().map_or(0.0, |pre| pre.zone_timestep_s), self.hour_index as f64);
    }

    fn aggregate_facility(&mut self) {
        let Some(state) = &self.state else { return };
        let dt_s = self.pre.as_ref().map_or(0.0, |pre| pre.zone_timestep_s);
        if self.hour_index == 0 {
            self.meter_order.extend(["Facility Heating".into(), "Facility PV".into()]);
        }
        self.meters.get_or_create("Facility Heating", FuelType::Electricity, EndUse::Heating).accumulate(state.delivered_total.heating_w, dt_s, self.hour_index as f64);
        self.meters.get_or_create("Facility PV", FuelType::OnSiteGeneration, EndUse::Generators).accumulate(-state.delivered_total.pv_generation_w, dt_s, self.hour_index as f64);
    }

    fn finalize_summaries(&mut self) {
        let electricity_kwh = self.meters.facility_total_kwh(FuelType::Electricity);
        let gas_kwh = self.meters.facility_total_kwh(FuelType::NaturalGas);
        self.final_summaries.add_annual("Electricity", electricity_kwh, "kWh");
        self.final_summaries.add_annual("Natural Gas", gas_kwh, "kWh");
        let floor_area = self.model.zones.iter().map(|zone| zone.volume_m3 / 3.0).sum::<f64>().max(1.0);
        self.final_summaries.add_annual("Energy Use Intensity", electricity_kwh / floor_area, "kWh/m²");
    }

    fn finalize_metrics(&mut self) {
        let electricity_kwh = self.meters.facility_total_kwh(FuelType::Electricity);
        let gas_kwh = self.meters.facility_total_kwh(FuelType::NaturalGas);
        self.final_environmental = Some(compute_environmental(electricity_kwh, gas_kwh, &SourceEnergyFactors::default(), &EmissionFactors::default()));
        self.final_resilience = Some(compute_resilience(&self.zone_temperature_history, 20.0, 26.0, true));
    }

    fn finalize_economics(&mut self) {
        let electricity_kwh = self.meters.facility_total_kwh(FuelType::Electricity);
        let tariff = UtilityTariff { name: "Default".into(), fuel: FuelType::Electricity, periods: vec![], fixed_monthly_charge: 10.0, ratchet_percent: 0.0 };
        let annual_cost = tariff.energy_cost(electricity_kwh, 12, 6) * 12.0;
        let lcca = compute_lcca(annual_cost, &LccaParameters { study_period_years: 25, discount_rate: 0.03, inflation_rate: 0.02, initial_cost: 0.0, annual_maintenance: 0.0, replacement_cost: 0.0, replacement_interval_years: 15 });
        self.final_summaries.add_annual("Annual Energy Cost", annual_cost, "USD");
        self.final_summaries.add_annual("LCCA Present Value", lcca.present_value_total, "USD");
    }

    fn build_results(&mut self) {
        self.result = Some(Results {
            time_series: std::mem::take(&mut self.time_series),
            meters: std::mem::take(&mut self.meters),
            summaries: std::mem::take(&mut self.final_summaries),
            sizing: self.final_sizing.take().unwrap_or_default(),
            environmental: self.final_environmental.take().unwrap_or_default(),
            resilience: self.final_resilience.take().unwrap_or_default(),
            diagnostics: Default::default(),
            run_metadata: RunMetadata {
                model_name: self.model.name.clone(),
                model_version: self.model.version.clone(),
                weather_location: self.config.weather.as_ref().map_or_else(|| "synthetic".into(), |weather| weather.location.clone()),
                timesteps: self.hour_index,
                warmup_days: self.config.warmup_days,
                elapsed_ms: self.started.elapsed().as_millis() as u64,
            },
        });
    }

    fn encode_output_step(&mut self) -> bool {
        let Some(result) = self.result.as_ref() else { return true };
        match self.encode_section {
            0 => {
                self.commit_output.extend_from_slice(b"ENERGYOUT1");
                append_string(&mut self.commit_output, &result.run_metadata.model_name);
                append_string(&mut self.commit_output, &result.run_metadata.model_version);
                append_string(&mut self.commit_output, &result.run_metadata.weather_location);
                self.commit_output.extend_from_slice(&result.run_metadata.timesteps.to_le_bytes());
                self.commit_output.extend_from_slice(&result.run_metadata.warmup_days.to_le_bytes());
                self.commit_output.extend_from_slice(&(self.meter_order.len() as u32).to_le_bytes());
                self.encode_section = 1;
            }
            1 => {
                if let Some(name) = self.meter_order.get(self.encode_record_cursor) {
                    if let Some(meter) = result.meters.meters.get(name) {
                        append_string(&mut self.commit_output, name);
                        append_string(&mut self.commit_output, &format!("{:?}", meter.fuel));
                        append_string(&mut self.commit_output, &format!("{:?}", meter.end_use));
                        self.commit_output.extend_from_slice(&meter.energy_j.to_le_bytes());
                        self.commit_output.extend_from_slice(&meter.peak_demand_w.to_le_bytes());
                        self.commit_output.extend_from_slice(&meter.peak_demand_hour.to_le_bytes());
                    }
                    self.encode_record_cursor += 1;
                } else {
                    self.commit_output.extend_from_slice(&(self.time_series_order.len() as u32).to_le_bytes());
                    self.encode_section = 2;
                    self.encode_record_cursor = 0;
                }
            }
            2 => {
                let Some(key) = self.time_series_order.get(self.encode_record_cursor) else {
                    self.commit_output.extend_from_slice(&(result.summaries.annual_energy.len() as u32).to_le_bytes());
                    self.encode_section = 3;
                    self.encode_record_cursor = 0;
                    self.encode_sample_cursor = 0;
                    return false;
                };
                let Some(series) = result.time_series.series.get(key) else {
                    self.encode_record_cursor += 1;
                    self.encode_sample_cursor = 0;
                    return false;
                };
                if self.encode_sample_cursor == 0 {
                    append_string(&mut self.commit_output, key);
                    append_string(&mut self.commit_output, &format!("{:?}", series.unit));
                    self.commit_output.extend_from_slice(&(series.values.len() as u32).to_le_bytes());
                }
                if let Some((&timestamp, &value)) = series.timestamps_hours.get(self.encode_sample_cursor).zip(series.values.get(self.encode_sample_cursor)) {
                    self.commit_output.extend_from_slice(&timestamp.to_le_bytes());
                    self.commit_output.extend_from_slice(&value.to_le_bytes());
                    self.encode_sample_cursor += 1;
                }
                if self.encode_sample_cursor >= series.values.len() {
                    self.encode_record_cursor += 1;
                    self.encode_sample_cursor = 0;
                }
            }
            3 => {
                if let Some(row) = result.summaries.annual_energy.get(self.encode_record_cursor) {
                    append_string(&mut self.commit_output, &row.key);
                    self.commit_output.extend_from_slice(&row.value.to_le_bytes());
                    append_string(&mut self.commit_output, &row.unit);
                    self.encode_record_cursor += 1;
                } else {
                    self.encode_section = 4;
                }
            }
            _ => return true,
        }
        false
    }
}

impl InteractiveJob for EnergyJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Self::fault(&Error::severe("energy job operation or generation mismatch"));
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        context.consume_fuel(1);
        match self.stage {
            EnergyJobStage::Validate => {
                if let Err(diagnostics) = self.model.validate() {
                    let error = diagnostics.messages.into_iter().find(|message| message.severity == crate::error::Severity::Fatal).unwrap_or_else(|| Error::severe("model validation failed"));
                    return Self::fault(&error);
                }
                self.weather_target = self.config.weather.as_ref().map_or_else(
                    || match self.config.environment {
                        SimulationEnvironment::HeatingDesignDay | SimulationEnvironment::CoolingDesignDay => 24,
                        SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => 8_760,
                    },
                    |weather| weather.records.len().max(1),
                );
                self.set_stage(context, EnergyJobStage::ResolveWeather);
                StepOutcome::Yield
            }
            EnergyJobStage::ResolveWeather => {
                if self.weather_cursor < self.weather_target {
                    self.weather.push(self.weather_record(self.weather_cursor));
                    self.weather_cursor += 1;
                    if self.weather_cursor.is_multiple_of(256) {
                        return self.publish_preview(context);
                    }
                    return StepOutcome::Yield;
                }
                self.precompute = Some(PrecomputeBuilder::new(self.config.zone_timestep_minutes, self.config.system_timestep_minutes));
                self.set_stage(context, EnergyJobStage::Precompute);
                StepOutcome::Yield
            }
            EnergyJobStage::Precompute => {
                let builder = self.precompute.as_mut().expect("precompute builder exists in Precompute stage");
                if builder.is_complete() {
                    self.pre = self.precompute.take().map(PrecomputeBuilder::finish);
                    self.state = Some(SimulationModel::default());
                    self.initialize_cursor = 0;
                    self.set_stage(context, EnergyJobStage::InitializeZones);
                } else {
                    builder.step(&self.model);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::InitializeZones => {
                if let Some(zone) = self.model.zones.get(self.initialize_cursor) {
                    let weather = self.weather[0];
                    self.state.as_mut().expect("state exists while initializing").zones.insert(zone.id, ZoneState { air: ZoneAirState::new(weather.dry_bulb_c, weather.humidity_ratio()), ..ZoneState::empty() });
                    self.initialize_cursor += 1;
                } else {
                    self.initialize_cursor = 0;
                    self.set_stage(context, EnergyJobStage::InitializeSurfaces);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::InitializeSurfaces => {
                if let Some(surface) = self.model.surfaces.get(self.initialize_cursor) {
                    if let Some(precomputed) = self.pre.as_ref().and_then(|pre| pre.surfaces.get(&surface.id)) {
                        let temperature = self.weather[0].dry_bulb_c;
                        self.state
                            .as_mut()
                            .expect("state exists while initializing")
                            .surfaces
                            .insert(surface.id, SurfaceState { inside_temp_c: temperature, outside_temp_c: temperature, heat_flux_w: 0.0, ctf: precomputed.ctf.clone(), convection_to_zone_w: 0.0 });
                    }
                    self.initialize_cursor += 1;
                } else {
                    self.set_stage(context, EnergyJobStage::WarmupTimestep);
                    return self.publish_preview(context);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::WarmupTimestep => {
                let warmup_hours = self.config.warmup_days.saturating_mul(24);
                if self.warmup_hour >= warmup_hours {
                    if let Some(state) = &mut self.state {
                        state.warmup_complete = true;
                    }
                    self.set_stage(context, EnergyJobStage::StartRun);
                    return self.checkpoint();
                }
                let pre = self.pre.as_ref().expect("precompute complete before warmup");
                if self.timestep_work.is_none() {
                    let weather = self.weather[self.warmup_hour as usize % self.weather.len()];
                    let date = crate::calendar::SimDate::new(weather.year, weather.month, weather.day);
                    self.timestep_work = Some(TimestepWork::new(&self.model, pre, weather, date, self.warmup_hour as f64, pre.zone_timestep_s));
                }
                self.timestep_work.as_mut().expect("warmup timestep work exists").step(&self.model, &self.config, pre, self.state.as_mut().expect("state initialized before warmup"));
                self.rng_state = self.rng_state.rotate_left(17).wrapping_mul(0x94d0_49bb_1331_11eb);
                if !self.timestep_work.as_ref().is_some_and(TimestepWork::is_complete) {
                    return StepOutcome::Yield;
                }
                self.timestep_work = None;
                if self.warmup_hour > 24 && self.warmup_hour.is_multiple_of(24) {
                    let state = self.state.as_ref().expect("state exists after warmup step");
                    let temperature_converged = state.zones.iter().all(|(id, zone)| self.previous_temperatures.get(id).is_some_and(|previous| (zone.air.temp_c - previous).abs() <= self.config.tolerances.temperature_k));
                    let load_converged = state.zones.iter().all(|(id, zone)| self.previous_loads.get(id).is_some_and(|previous| (zone.heating_demand_w + zone.cooling_demand_w - previous).abs() <= self.config.tolerances.energy_w));
                    if temperature_converged && load_converged {
                        self.warmup_hour = warmup_hours;
                    }
                }
                if let Some(state) = &self.state {
                    for (id, zone) in &state.zones {
                        self.previous_temperatures.insert(*id, zone.air.temp_c);
                        self.previous_loads.insert(*id, zone.heating_demand_w + zone.cooling_demand_w);
                    }
                }
                self.warmup_hour = self.warmup_hour.saturating_add(1);
                if self.warmup_hour.is_multiple_of(24) {
                    return self.publish_preview(context);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::StartRun => {
                let period = SimulationKernel::run_period(&self.config);
                self.total_timesteps = period.total_hours();
                self.run_hours = Some(period.hours());
                self.tier = match self.config.environment {
                    SimulationEnvironment::HeatingDesignDay | SimulationEnvironment::CoolingDesignDay | SimulationEnvironment::CustomDesignPeriod => EnergyQualityTier::DesignDay,
                    SimulationEnvironment::WeatherRunPeriod => EnergyQualityTier::CoarseTimestep,
                };
                self.set_stage(context, EnergyJobStage::RunZoneTimestep);
                StepOutcome::Yield
            }
            EnergyJobStage::RunZoneTimestep => {
                let pre = self.pre.as_ref().expect("precompute complete before run");
                if self.timestep_work.is_none() {
                    let Some((date, hour, _)) = self.run_hours.as_mut().and_then(|hours| hours.next()) else {
                        self.set_stage(context, EnergyJobStage::Finalize);
                        return StepOutcome::Yield;
                    };
                    let mut weather = self.weather[self.hour_index as usize % self.weather.len()];
                    weather.year = date.year;
                    weather.month = date.month;
                    weather.day = date.day;
                    weather.hour = hour;
                    self.timestep_work = Some(TimestepWork::new(&self.model, pre, weather, date, self.hour_index as f64, pre.zone_timestep_s));
                }
                self.timestep_work.as_mut().expect("run timestep work exists").step(&self.model, &self.config, pre, self.state.as_mut().expect("state initialized before run"));
                self.rng_state = self.rng_state.rotate_left(17).wrapping_mul(0x94d0_49bb_1331_11eb);
                if !self.timestep_work.as_ref().is_some_and(TimestepWork::is_complete) {
                    return StepOutcome::Yield;
                }
                self.timestep_work = None;
                self.aggregate_zone_cursor = 0;
                self.set_stage(context, EnergyJobStage::AggregateZone);
                StepOutcome::Yield
            }
            EnergyJobStage::AggregateZone => {
                if self.aggregate_zone_cursor < self.model.zones.len() {
                    self.aggregate_zone();
                    self.aggregate_zone_cursor += 1;
                } else {
                    self.set_stage(context, EnergyJobStage::AggregateFacility);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::AggregateFacility => {
                self.aggregate_facility();
                self.hour_index = self.hour_index.saturating_add(1);
                self.checkpoint_due = self.hour_index.is_multiple_of(24);
                self.set_stage(context, EnergyJobStage::PublishTimestep);
                StepOutcome::Yield
            }
            EnergyJobStage::PublishTimestep => {
                if self.checkpoint_due {
                    self.checkpoint_due = false;
                    return self.checkpoint();
                }
                self.set_stage(context, EnergyJobStage::RunZoneTimestep);
                self.publish_preview(context)
            }
            EnergyJobStage::Finalize => {
                self.tier = EnergyQualityTier::Final;
                self.sizing_builder = Some(SizingBuilder::new(SizingConfig::default()));
                self.set_stage(context, EnergyJobStage::Size);
                StepOutcome::Yield
            }
            EnergyJobStage::Size => {
                let builder = self.sizing_builder.as_mut().expect("sizing builder exists");
                if builder.is_complete(&self.model) {
                    self.final_sizing = self.sizing_builder.take().map(SizingBuilder::finish);
                    self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                } else {
                    builder.step(&self.model);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::FinalizeSummaries => {
                self.finalize_summaries();
                self.set_stage(context, EnergyJobStage::FinalizeMetrics);
                StepOutcome::Yield
            }
            EnergyJobStage::FinalizeMetrics => {
                self.finalize_metrics();
                self.set_stage(context, EnergyJobStage::FinalizeEconomics);
                StepOutcome::Yield
            }
            EnergyJobStage::FinalizeEconomics => {
                self.finalize_economics();
                self.set_stage(context, EnergyJobStage::BuildResults);
                StepOutcome::Yield
            }
            EnergyJobStage::BuildResults => {
                self.build_results();
                self.set_stage(context, EnergyJobStage::PublishFinal);
                self.publish_preview(context)
            }
            EnergyJobStage::PublishFinal => {
                self.set_stage(context, EnergyJobStage::EncodeOutput);
                StepOutcome::Yield
            }
            EnergyJobStage::EncodeOutput => {
                if self.encode_output_step() {
                    self.set_stage(context, EnergyJobStage::Complete);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::Complete => StepOutcome::Complete(CommitCandidate { state: self.encode_state(true), output: std::mem::take(&mut self.commit_output) }),
        }
    }
}

impl EnergyJobStage {
    fn label(self) -> &'static str {
        match self {
            EnergyJobStage::Validate => "energy.validate",
            EnergyJobStage::ResolveWeather => "energy.resolve-weather",
            EnergyJobStage::Precompute => "energy.precompute",
            EnergyJobStage::InitializeZones => "energy.initialize-zones",
            EnergyJobStage::InitializeSurfaces => "energy.initialize-surfaces",
            EnergyJobStage::WarmupTimestep => "energy.warmup-timestep",
            EnergyJobStage::StartRun => "energy.start-run",
            EnergyJobStage::RunZoneTimestep => "energy.run-zone-timestep",
            EnergyJobStage::AggregateZone => "energy.aggregate-zone",
            EnergyJobStage::AggregateFacility => "energy.aggregate-facility",
            EnergyJobStage::PublishTimestep => "energy.publish-timestep",
            EnergyJobStage::Finalize => "energy.finalize",
            EnergyJobStage::Size => "energy.size",
            EnergyJobStage::FinalizeSummaries => "energy.finalize-summaries",
            EnergyJobStage::FinalizeMetrics => "energy.finalize-metrics",
            EnergyJobStage::FinalizeEconomics => "energy.finalize-economics",
            EnergyJobStage::BuildResults => "energy.build-results",
            EnergyJobStage::PublishFinal => "energy.publish-final",
            EnergyJobStage::EncodeOutput => "energy.encode-output",
            EnergyJobStage::Complete => "energy.complete",
        }
    }

    fn code(self) -> u8 {
        self as u8
    }
}

impl EnergyQualityTier {
    fn code(self) -> u8 {
        self as u8
    }
}

fn encode_preview(preview: &EnergyJobPreview) -> Vec<u8> {
    let mut bytes = b"ENERGYP1".to_vec();
    bytes.extend_from_slice(&preview.sequence.to_le_bytes());
    bytes.push(preview.tier.code());
    bytes.push(preview.stage.code());
    bytes.extend_from_slice(&preview.warmup_hour.to_le_bytes());
    bytes.extend_from_slice(&preview.timestep.to_le_bytes());
    bytes.extend_from_slice(&preview.total_timesteps.to_le_bytes());
    bytes.extend_from_slice(&preview.facility_electricity_kwh.to_le_bytes());
    bytes.extend_from_slice(&(preview.zone_temperatures_c.len() as u32).to_le_bytes());
    for (id, temperature) in &preview.zone_temperatures_c {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.extend_from_slice(&temperature.to_le_bytes());
    }
    bytes
}

fn append_string(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
}
// #endregion 🔖️EnergyJob

// #region 🔖️Engine
/// ⚡️ Headless BEM simulation engine.
pub struct Engine;

impl Engine {
    /// ⚡️ Create the persistent operation used by interactive hosts and the batch adapter.
    pub fn job(model: &Model, config: &SimulationConfig) -> EnergyJob {
        EnergyJob::new(Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 0), model.clone(), config.clone())
    }

    /// ⚡️ Run full building energy simulation.
    pub fn run(model: &Model, config: &SimulationConfig) -> Result<Results, Error> {
        let mut job = Self::job(model, config);
        let operation = job.operation.operation;
        let generation = job.operation.generation;
        let outcome = run_to_completion(
            &mut job,
            &BatchJobParams { operation, generation, cancel: CancelToken::root_now(), config: BatchDriveConfig { site: "energy.engine", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 32, step_budget_ms: 2 }, now_ms: default_now_ms },
        );
        match outcome {
            StepOutcome::Complete(_) => job.take_results().ok_or_else(|| Error::severe("energy job completed without results")),
            StepOutcome::Fault(fault) => Err(Error::severe(String::from_utf8_lossy(&fault.detail))),
            StepOutcome::Cancelled => Err(Error::severe("energy simulation cancelled")),
            StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => Err(Error::severe("energy batch adapter stopped before a terminal outcome")),
        }
    }
}
// #endregion 🔖️Engine

fn synthetic_hour(h: u32) -> WeatherRecord {
    let day = h / 24;
    let hour = h % 24;
    let month = (day / 30 + 1).min(12) as u8;
    let t_base = 15.0 + 10.0 * ((day as f64 / 365.0) * 2.0 * std::f64::consts::PI).sin();
    let t_daily = 5.0 * ((hour as f64 - 14.0) / 12.0 * std::f64::consts::PI).cos();
    WeatherRecord {
        year: 2026,
        month,
        day: (day % 30 + 1) as u8,
        hour: hour as u8,
        minute: 0,
        dry_bulb_c: t_base + t_daily,
        dew_point_c: t_base - 5.0,
        relative_humidity: 0.5,
        atmospheric_pressure_pa: 101_325.0,
        wind_speed_m_s: 3.0,
        wind_direction_deg: 180.0,
        direct_normal_irradiance_w_m2: if (6..18).contains(&hour) { 500.0 } else { 0.0 },
        diffuse_horizontal_irradiance_w_m2: if (6..18).contains(&hour) { 100.0 } else { 0.0 },
        horizontal_infrared_w_m2: 250.0,
        precipitation_mm: 0.0,
        snow_depth_mm: 0.0,
    }
}

fn design_day_hour(hour: u32, dry_bulb_c: f64) -> WeatherRecord {
    WeatherRecord {
        year: 2026,
        month: 1,
        day: 1,
        hour: hour as u8,
        minute: 0,
        dry_bulb_c,
        dew_point_c: dry_bulb_c - 10.0,
        relative_humidity: 0.5,
        atmospheric_pressure_pa: 101_325.0,
        wind_speed_m_s: 3.0,
        wind_direction_deg: 180.0,
        direct_normal_irradiance_w_m2: if dry_bulb_c > 20.0 && (8..17).contains(&hour) { 800.0 } else { 0.0 },
        diffuse_horizontal_irradiance_w_m2: if dry_bulb_c > 20.0 && (8..17).contains(&hour) { 150.0 } else { 0.0 },
        horizontal_infrared_w_m2: 250.0,
        precipitation_mm: 0.0,
        snow_depth_mm: 0.0,
    }
}

// #region 🔖️Fixtures
/// 🧪️ Build a minimal test model for integration tests.
pub fn test_model_single_zone() -> Model {
    use crate::model::*;
    Model {
        name: "BESTEST Single Zone".into(),
        version: "1.0".into(),
        site: Site { latitude_deg: 45.0, longitude_deg: 0.0, elevation_m: 100.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
        zones: vec![Zone { id: EntityId(1), name: "Zone1".into(), volume_m3: 106.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
        materials: vec![Material {
            id: EntityId(10),
            name: "Insulation".into(),
            thickness_m: 0.1,
            conductivity_w_m_k: 0.04,
            density_kg_m3: 50.0,
            specific_heat_j_kg_k: 1000.0,
            thermal_absorptance: 0.9,
            solar_absorptance: 0.7,
            visible_absorptance: 0.7,
        }],
        constructions: vec![Construction { id: EntityId(20), name: "Wall".into(), layer_material_ids: vec![EntityId(10)] }],
        surfaces: vec![Surface {
            id: EntityId(30),
            name: "ExtWall".into(),
            zone_id: EntityId(1),
            class: SurfaceClass::ExteriorWall,
            vertices_m: vec![[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 0.0, 3.0], [0.0, 0.0, 3.0]],
            construction_id: EntityId(20),
            outside_boundary_condition: OutsideBoundary::OutdoorAir,
            sun_exposed: true,
            wind_exposed: true,
            multiplier: 1,
        }],
        ideal_loads: vec![IdealLoadsSystem {
            id: EntityId(40),
            zone_id: EntityId(1),
            max_heating_supply_air_temp_c: 50.0,
            min_cooling_supply_air_temp_c: 13.0,
            max_heating_capacity_w: None,
            max_cooling_capacity_w: None,
            outdoor_air_per_person_m3_s: 0.00944,
            outdoor_air_per_area_m3_s_m2: 0.0,
        }],
        ..Default::default()
    }
}

/// 🧪️ Full topology test model with plant, PV, AFN, daylight.
pub fn test_model_full_topology() -> Model {
    use crate::model::*;
    let mut model = test_model_single_zone();
    model.name = "Full Topology".into();
    model.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0 });
    model.plant_loops.push(PlantLoopConfig { id: EntityId(60), name: "Hot Water".into(), loop_type: PlantLoopType::Heating, supply_temperature_c: 55.0, return_temperature_c: 45.0, design_flow_kg_s: 2.0, equipment_ids: vec![EntityId(61)] });
    model.pv_systems.push(PvSystemAssignment { id: EntityId(70), dc_capacity_w: 5000.0, area_m2: 25.0, tilt_deg: 30.0, azimuth_deg: 180.0, module_efficiency: 0.2, inverter_efficiency: 0.96 });
    model.daylight_zones.push(DaylightZoneConfig { id: EntityId(80), zone_id: EntityId(1), illuminance_target_lux: 500.0, glare_limit: 0.4, window_transmittance: 0.6 });
    model
}
// #endregion 🔖️Fixtures

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::RunPeriod;

    fn drive_energy_job(job: EnergyJob) -> (EnergyJob, Vec<EnergyJobPreview>, usize, Vec<u8>, std::time::Duration) {
        drive_energy_job_with_fuel(job, 1)
    }

    fn drive_energy_job_with_fuel(mut job: EnergyJob, fuel: u64) -> (EnergyJob, Vec<EnergyJobPreview>, usize, Vec<u8>, std::time::Duration) {
        let operation = job.operation.operation;
        let generation = job.operation.generation;
        let cancel = CancelToken::root_now();
        let mut preview_sequence = 0;
        let mut previews = Vec::new();
        let mut checkpoints = 0;
        let mut worst = std::time::Duration::ZERO;
        for _ in 0..50_000 {
            let start = Instant::now();
            let mut context = StepContext::new(operation, generation, semio_framework_job::StepBudget::new(fuel, u64::MAX), cancel.clone(), default_now_ms, &mut preview_sequence);
            let outcome = job.step(&mut context);
            let elapsed = start.elapsed();
            worst = worst.max(elapsed);
            match outcome {
                StepOutcome::PreviewReady(_) => previews.push(job.preview().expect("typed preview accompanies payload").clone()),
                StepOutcome::CheckpointReady(_) => checkpoints += 1,
                StepOutcome::Complete(candidate) => return (job, previews, checkpoints, candidate.output, worst),
                StepOutcome::Fault(fault) => panic!("energy job faulted: {}", String::from_utf8_lossy(&fault.detail)),
                StepOutcome::Cancelled => panic!("energy job unexpectedly cancelled"),
                StepOutcome::Yield => {}
            }
        }
        panic!("energy job did not complete within the deterministic step bound")
    }

    #[test]
    fn engine_runs_single_zone() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 3, environment: SimulationEnvironment::WeatherRunPeriod, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        assert!(results.run_metadata.timesteps > 0);
        assert!(results.meters.facility_total_kwh(FuelType::Electricity) >= 0.0);
    }

    #[test]
    fn engine_deterministic_repeatability() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let r1 = Engine::run(&model, &config).unwrap();
        let r2 = Engine::run(&model, &config).unwrap();
        assert_eq!(r1.run_metadata.timesteps, r2.run_metadata.timesteps);
        assert!((r1.meters.facility_total_kwh(FuelType::Electricity) - r2.meters.facility_total_kwh(FuelType::Electricity)).abs() < 1e-3);
    }

    #[test]
    fn ashrae_140_case600_base() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        let temps = results.time_series.get("Zone Air Temperature [Zone1]");
        assert!(temps.is_some());
    }

    #[test]
    fn invalid_model_rejected() {
        let model = Model::default();
        assert!(Engine::run(&model, &SimulationConfig::default()).is_err());
    }

    #[test]
    fn energy_conservation_order_of_magnitude() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        let total_kwh = results.meters.facility_total_kwh(FuelType::Electricity);
        assert!(total_kwh < 1_000_000.0);
    }

    #[test]
    fn full_topology_e2e() {
        let model = test_model_full_topology();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        assert!(results.run_metadata.timesteps >= 48);
        assert!(results.summaries.annual_energy.len() >= 3);
    }

    #[test]
    fn hvac_bestest_heating_day() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        assert_eq!(results.run_metadata.timesteps, 24);
        assert!(results.time_series.get("Zone Air Temperature [Zone1]").is_some());
    }

    #[test]
    fn run_period_honors_calendar() {
        let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 7, year: 2026 };
        assert_eq!(period.total_hours(), 168);
        let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, warmup_days: 0, ..Default::default() };
        let model = test_model_single_zone();
        let results = Engine::run(&model, &config).unwrap();
        assert_eq!(results.run_metadata.timesteps, 168);
    }

    #[test]
    fn energy_job_previews_checkpoints_and_commits_bounded_steps() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let (mut job, previews, checkpoints, output, worst) = drive_energy_job(Engine::job(&model, &config));
        assert!(!previews.is_empty());
        assert!(previews.windows(2).all(|pair| pair[0].sequence < pair[1].sequence));
        assert!(previews.iter().any(|preview| preview.tier == EnergyQualityTier::SteadyStateEstimate));
        assert_eq!(previews.last().map(|preview| preview.tier), Some(EnergyQualityTier::Final));
        assert!(checkpoints >= 2);
        assert!(output.starts_with(b"ENERGYOUT1"));
        assert!(worst < std::time::Duration::from_millis(8), "worst energy step was {worst:?}");
        let results = job.take_results().expect("completed job retains typed results for the batch adapter");
        assert_eq!(results.run_metadata.timesteps, 24);
    }

    #[test]
    fn energy_job_cancellation_and_freshness_precede_mutation() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, ..Default::default() };
        let operation = Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 11);
        let mut cancelled_job = EnergyJob::new(operation, model.clone(), config.clone());
        let cancel = CancelToken::root_now();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_ms, &mut sequence);
        assert_eq!(cancelled_job.step(&mut context), StepOutcome::Cancelled);
        assert_eq!(cancelled_job.stage(), EnergyJobStage::Validate);

        let mut stale_job = EnergyJob::new(operation, model, config);
        let mut stale_sequence = 0;
        let mut stale_context = StepContext::new(operation.operation, Generation(4), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_ms, &mut stale_sequence);
        assert!(matches!(stale_job.step(&mut stale_context), StepOutcome::Fault(_)));
        assert_eq!(stale_job.stage(), EnergyJobStage::Validate);
    }

    #[test]
    fn energy_job_checkpoint_restore_and_fuel_batches_are_deterministic() {
        let model = test_model_full_topology();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let operation = Operation::new(allocate_operation_id(), RevisionId(9), Generation(4), 73);
        let mut original = EnergyJob::new(operation, model.clone(), config.clone());
        let mut preview_sequence = 0;
        for _ in 0..128 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_ms, &mut preview_sequence);
            let _ = original.step(&mut context);
            if original.timestep_work.is_some() {
                break;
            }
        }
        assert!(original.timestep_work.is_some(), "checkpoint must capture an active timestep cursor");
        let checkpoint_timestep_stage = original.timestep_work.as_ref().map(TimestepWork::stage);
        let checkpoint = original.encode_state(false);
        let restored = EnergyJob::from_checkpoint(operation, model.clone(), config.clone(), &checkpoint).expect("checkpoint restore");
        assert_eq!(restored.timestep_work.as_ref().map(TimestepWork::stage), checkpoint_timestep_stage);
        assert_eq!(restored.encode_state(false), checkpoint);
        let (mut original, _, _, original_output, _) = drive_energy_job_with_fuel(original, 1);
        let (mut restored, _, _, restored_output, _) = drive_energy_job_with_fuel(restored, 64);
        assert_eq!(original_output, restored_output);
        let original_results = original.take_results().expect("original results");
        let restored_results = restored.take_results().expect("restored results");
        assert_eq!(original_results.time_series, restored_results.time_series);
        assert_eq!(original_results.meters, restored_results.meters);

        let (_, _, _, one_output, _) = drive_energy_job_with_fuel(EnergyJob::new(operation, model.clone(), config.clone()), 1);
        let (_, _, _, many_output, _) = drive_energy_job_with_fuel(EnergyJob::new(operation, model, config), 128);
        assert_eq!(one_output, many_output);
    }

    #[test]
    fn adversarial_timestep_work_unit_stays_below_watchdog() {
        let mut model = test_model_single_zone();
        let template = model.surfaces[0].clone();
        model.surfaces = (0..16_384)
            .map(|index| {
                let mut surface = template.clone();
                surface.id = crate::model::EntityId(1_000 + index);
                surface
            })
            .collect();
        let pre = PrecomputedModel::build(&model, 60, 15);
        let weather = design_day_hour(12, 35.0);
        let date = crate::calendar::SimDate::new(weather.year, weather.month, weather.day);
        let mut state = SimulationKernel::initialize(&model, &pre, &weather);
        let mut work = TimestepWork::new(&model, &pre, weather, date, 12.0, pre.zone_timestep_s);
        let start = Instant::now();
        work.step(&model, &SimulationConfig::default(), &pre, &mut state);
        assert!(start.elapsed() < std::time::Duration::from_millis(8), "one adversarial energy work unit exceeded watchdog: {:?}", start.elapsed());
    }
}
