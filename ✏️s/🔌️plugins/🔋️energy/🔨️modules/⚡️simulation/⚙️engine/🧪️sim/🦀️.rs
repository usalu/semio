//! 🚀️ Engine orchestration: Model + SimulationConfig → Results.

extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_error as error;
extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_meters as meters;
extern crate semio_s_plugin_energy_engine_metrics as metrics;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_output as output;
extern crate semio_s_plugin_energy_engine_precompute as precompute;
extern crate semio_s_plugin_energy_engine_results as results;
extern crate semio_s_plugin_energy_engine_site as site;
extern crate semio_s_plugin_energy_engine_sizing as sizing;
extern crate semio_s_plugin_energy_engine_units as units;
#[cfg(test)]
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_plugin_energy_engine_sim_retirement as retirement;
extern crate semio_s_plugin_energy_engine_sim_wire as wire;

use crate::retirement::{ENERGY_RUNG_DEMAND, close_config_step, close_meter_table_step, close_model_step, close_results_step, close_sizing_step, close_string_step, close_string_vector_step, close_summaries_step, close_time_series_step, config_is_terminal_empty, energy_done, energy_grant_covers, energy_guarded, energy_idle, energy_one, energy_rung, energy_self_funded, energy_turn, model_is_terminal_empty, summaries_are_terminal_empty};
use crate::wire::{ENERGY_WIRE_HEADER_BYTES, ENERGY_WIRE_QUEUE_SLOTS, EnergyCommitReservation, EnergyJobCursor, EnergyJobPreview, EnergyJobStage, EnergyNumericalBounds, EnergyNumericalCensus, EnergyNumericalDimension, EnergyPublicationChannels, EnergyQualityTier, EnergyWireBuild, EnergyWireIdentity, EnergyWireKind, EnergyWireLease, EnergyWirePacket, EnergyWireRejection, close_retained_packet, decode_preview_packet, end_use_code, energy_spend, energy_wire_header, fuel_code, unit_code, weather_copy_target};
use crate::error::Error;
use crate::kernel::{HourWeather, SimulationConfig, SimulationEnvironment, SimulationKernel, SimulationModel, SurfaceState, TimestepBuilder, TimestepWork, WindowState, ZoneState};
use crate::meters::{EndUse, FuelType, Meter, MeterTable};
use crate::metrics::{EmissionFactors, EnvironmentalMetrics, ResilienceMetrics, SourceEnergyFactors};
use crate::model::{FixedTable, Model};
use crate::output::{TimeSeries, TimeSeriesAppendError, TimeSeriesTable};
use crate::precompute::{PrecomputeBuilder, PrecomputedModel};
use crate::results::{Results, RunMetadata, SizingTables, SummaryRow, SummaryTables};
use crate::site::WeatherRecord;
use crate::sizing::{SizingBuilder, SizingConfig};
use crate::units::Unit;
use semio_framework_job::{
    allocate_operation_id, default_now_us, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RevisionId,
    OriginalTurnFailure, StepBudget, StepContext,
};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use std::time::Instant;

/// 🌡️ Temperature every zone and construction node starts from before warmup.
const INITIAL_TEMPERATURE_C: f64 = 23.0;

/// 🧭️ What one internal step decided before any outcome is lent to the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergyRun {
    Yield,
    Cancelled,
    Preview,
    Checkpoint(u64),
    Fault,
    Complete,
}

/// 🏷️ Empty marker payloads lent as outcome payloads; the real packets travel through the wire queues.
struct EnergyOutcomeMarkers {
    preview: RetainedJobPayload,
    checkpoint: RetainedJobPayload,
    fault: RetainedJobPayload,
    commit_state: RetainedJobPayload,
    commit_output: RetainedJobPayload,
}

impl Default for EnergyOutcomeMarkers {
    fn default() -> Self {
        Self {
            preview: RetainedJobPayload::empty(JobPayloadStream::Preview),
            checkpoint: RetainedJobPayload::empty(JobPayloadStream::CheckpointState),
            fault: RetainedJobPayload::empty(JobPayloadStream::Fault),
            commit_state: RetainedJobPayload::empty(JobPayloadStream::CommitState),
            commit_output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput),
        }
    }
}

/// ♻️ Exact rejected authority; model/config identity remains owned and directly retryable.
#[derive(Debug)]
pub struct EnergyAdmissionRejected {
    pub operation: Operation,
    pub model: Model,
    pub config: SimulationConfig,
    pub census: EnergyNumericalCensus,
    pub dimension: EnergyNumericalDimension,
}

impl EnergyAdmissionRejected {
    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry(self, bounds: EnergyNumericalBounds) -> Result<EnergyJob, Self> {
        EnergyJob::admit(self.operation, self.model, self.config, bounds)
    }

    /// 📏️ Quotes the next popped model or config owner.
    pub fn close_demands(&self) -> RetirementDemand {
        if self.terminal_is_empty() { RetirementDemand::default() } else { ENERGY_RUNG_DEMAND }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        if self.terminal_is_empty() {
            return energy_done();
        }
        if !energy_grant_covers(grant, ENERGY_RUNG_DEMAND) {
            return energy_idle();
        }
        if let Some(released) = close_model_step(&mut self.model, grant.maximum_release_bytes) {
            return energy_rung(released);
        }
        if let Some(released) = close_config_step(&mut self.config, grant.maximum_release_bytes) {
            return energy_rung(released);
        }
        energy_done()
    }

    pub fn terminal_is_empty(&self) -> bool {
        model_is_terminal_empty(&self.model) && config_is_terminal_empty(&self.config)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
enum WarmupConvergenceStage {
    TemperatureCheck,
    LoadCheck,
    TemperatureHistory,
    LoadHistory,
    Complete,
}

#[cfg(test)]
const P7C1_WARMUP_CONVERGENCE_STAGES: [WarmupConvergenceStage; 5] =
    [WarmupConvergenceStage::TemperatureCheck, WarmupConvergenceStage::LoadCheck, WarmupConvergenceStage::TemperatureHistory, WarmupConvergenceStage::LoadHistory, WarmupConvergenceStage::Complete];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
struct WarmupConvergenceWork {
    stage: WarmupConvergenceStage,
    cursor: usize,
    temperature_converged: bool,
    load_converged: bool,
    evaluate: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
enum ValidationStage {
    ReserveZones,
    ReserveMaterials,
    ReserveConstructions,
    ReserveSurfaces,
    IndexZones,
    IndexMaterials,
    IndexConstructions,
    IndexSurfaces,
    CheckZones,
    CheckMaterials,
    CheckConstructions,
    CheckConstructionLayers,
    CheckSurfaces,
    CheckFenestrations,
    CheckZoneReferences,
    Complete,
}

#[cfg(test)]
const P7C1_VALIDATION_STAGES: [ValidationStage; 16] = [
    ValidationStage::ReserveZones,
    ValidationStage::ReserveMaterials,
    ValidationStage::ReserveConstructions,
    ValidationStage::ReserveSurfaces,
    ValidationStage::IndexZones,
    ValidationStage::IndexMaterials,
    ValidationStage::IndexConstructions,
    ValidationStage::IndexSurfaces,
    ValidationStage::CheckZones,
    ValidationStage::CheckMaterials,
    ValidationStage::CheckConstructions,
    ValidationStage::CheckConstructionLayers,
    ValidationStage::CheckSurfaces,
    ValidationStage::CheckFenestrations,
    ValidationStage::CheckZoneReferences,
    ValidationStage::Complete,
];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
struct ValidationWork {
    stage: ValidationStage,
    cursor: usize,
    child_cursor: usize,
    zone_reference_family: u8,
    zone_ids: FixedTable<model::EntityId, ()>,
    material_ids: FixedTable<model::EntityId, ()>,
    construction_ids: FixedTable<model::EntityId, ()>,
    surface_ids: FixedTable<model::EntityId, ()>,
    fatal_code: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
enum FinalizationStage {
    MeterTotals,
    FloorArea,
    HistoryResilience,
    TariffPeriod,
    LifeCycleYear,
    SummaryElectricity,
    SummaryGas,
    SummaryEui,
    SummaryAnnualCost,
    SummaryLcca,
    Environmental,
    Resilience,
    Complete,
}

#[cfg(test)]
const P7C1_FINALIZATION_STAGES: [FinalizationStage; 13] = [
    FinalizationStage::MeterTotals,
    FinalizationStage::FloorArea,
    FinalizationStage::HistoryResilience,
    FinalizationStage::TariffPeriod,
    FinalizationStage::LifeCycleYear,
    FinalizationStage::SummaryElectricity,
    FinalizationStage::SummaryGas,
    FinalizationStage::SummaryEui,
    FinalizationStage::SummaryAnnualCost,
    FinalizationStage::SummaryLcca,
    FinalizationStage::Environmental,
    FinalizationStage::Resilience,
    FinalizationStage::Complete,
];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
struct FinalizationWork {
    stage: FinalizationStage,
    cursor: usize,
    electricity_kwh: f64,
    gas_kwh: f64,
    floor_area_m2: f64,
    annual_cost: f64,
    present_value_energy: f64,
    resilience: ResilienceMetrics,
    row_key: String,
    row_unit: String,
    row_cursor: usize,
    row_part: u8,
    row_reserved: bool,
    row_backing_rejected: bool,
}

impl Default for FinalizationWork {
    fn default() -> Self {
        Self {
            stage: FinalizationStage::MeterTotals,
            cursor: 0,
            electricity_kwh: 0.0,
            gas_kwh: 0.0,
            floor_area_m2: 0.0,
            annual_cost: 0.0,
            present_value_energy: 0.0,
            resilience: ResilienceMetrics::default(),
            row_key: String::new(),
            row_unit: String::new(),
            row_cursor: 0,
            row_part: 0,
            row_reserved: false,
            row_backing_rejected: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
enum ResultBuildStage {
    ModelName,
    ModelVersion,
    WeatherLocation,
    Assemble,
    Complete,
}

#[cfg(test)]
const P7C1_RESULT_BUILD_STAGES: [ResultBuildStage; 5] = [ResultBuildStage::ModelName, ResultBuildStage::ModelVersion, ResultBuildStage::WeatherLocation, ResultBuildStage::Assemble, ResultBuildStage::Complete];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
struct ResultBuildWork {
    stage: ResultBuildStage,
    cursor: usize,
    reserved: bool,
    model_name: String,
    model_version: String,
    weather_location: String,
}

impl Default for ResultBuildWork {
    fn default() -> Self {
        Self { stage: ResultBuildStage::ModelName, cursor: 0, reserved: false, model_name: String::new(), model_version: String::new(), weather_location: String::new() }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
enum AggregateZoneStage {
    Temperature,
    Heating,
    Cooling,
    Fan,
    Complete,
}

#[cfg(test)]
const P7C1_AGGREGATE_STAGES: [AggregateZoneStage; 5] = [AggregateZoneStage::Temperature, AggregateZoneStage::Heating, AggregateZoneStage::Cooling, AggregateZoneStage::Fan, AggregateZoneStage::Complete];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
struct AggregateZoneWork {
    stage: AggregateZoneStage,
    phase: u8,
    copy_index: usize,
    cursor: usize,
    reserved: [bool; 3],
    names: [String; 3],
    timestamps: Vec<f64>,
    values: Vec<f64>,
    backing_rejected: bool,
    series_fault: Option<TimeSeriesAppendError>,
}

impl AggregateZoneWork {
    fn new() -> Self {
        Self { stage: AggregateZoneStage::Temperature, phase: 0, copy_index: 0, cursor: 0, reserved: [false; 3], names: std::array::from_fn(|_| String::new()), timestamps: Vec::new(), values: Vec::new(), backing_rejected: false, series_fault: None }
    }

    fn advance(&mut self, stage: AggregateZoneStage) {
        self.stage = stage;
        self.phase = 0;
        self.copy_index = 0;
        self.cursor = 0;
        self.reserved = [false; 3];
        self.names = std::array::from_fn(|_| String::new());
        self.timestamps = Vec::new();
        self.values = Vec::new();
        self.backing_rejected = false;
        self.series_fault = None;
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct EnergyAdmittedResultBacking {
    pub series_slots: usize,
    pub meter_slots: usize,
    samples_per_series: usize,
    pub history_slots: usize,
    summary_slots: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
enum OutputFault {
    MissingResult,
    BackingRejected,
    MissingMeter,
    MissingSeries,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CommitCensusStage {
    Base,
    MeterName,
    MeterRecord,
    SeriesName,
    SeriesHeader,
    SeriesSample,
    Summary,
    PreviewChannel,
    CheckpointChannel,
    CommitChannel,
    FaultChannel,
    ReadyPacket,
    RestoreInput,
    Reserve,
    ReservePage,
    Complete,
}

#[derive(Debug)]
struct CommitCensusWork {
    stage: CommitCensusStage,
    record: usize,
    sample: usize,
    byte: usize,
    channel_slot: usize,
    output_bytes: usize,
    output_items: usize,
    resident_pages: usize,
    resident_bytes: usize,
    resident_items: usize,
    reservation: Option<EnergyCommitReservation>,
}

impl Default for CommitCensusWork {
    fn default() -> Self {
        Self { stage: CommitCensusStage::Base, record: 0, sample: 0, byte: 0, channel_slot: 0, output_bytes: 0, output_items: 0, resident_pages: 0, resident_bytes: 0, resident_items: 0, reservation: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
enum WeatherFault {
    SlotRejected,
}

impl Default for ValidationWork {
    fn default() -> Self {
        Self {
            stage: ValidationStage::ReserveZones,
            cursor: 0,
            child_cursor: 0,
            zone_reference_family: 0,
            zone_ids: FixedTable::default(),
            material_ids: FixedTable::default(),
            construction_ids: FixedTable::default(),
            surface_ids: FixedTable::default(),
            fatal_code: 0,
        }
    }
}

/// ⚡️ Resumable simulation operation. Weather decoding, model precomputation, initialization,
/// warmup, metering and publication are cursor-owned; the batch API drives this same job.
#[doc(hidden)]
pub struct EnergyJobAuthority {
    pub operation: Operation,
    model: Model,
    config: SimulationConfig,
    numerical_census: EnergyNumericalCensus,
    pub stage: EnergyJobStage,
    pub tier: EnergyQualityTier,
    weather: FixedTable<usize, WeatherRecord>,
    pub weather_cursor: usize,
    pub weather_target: usize,
    precompute: Option<PrecomputeBuilder>,
    validation: ValidationWork,
    pre: Option<PrecomputedModel>,
    state: Option<SimulationModel>,
    initialize_cursor: usize,
    pub initialize_backing_stage: u8,
    pub warmup_hour: u32,
    previous_temperatures: Vec<f64>,
    previous_loads: Vec<f64>,
    warmup_convergence: Option<WarmupConvergenceWork>,
    run_hours: Option<calendar::RunPeriodHours>,
    pub total_timesteps: u32,
    pub run_backing_stage: u8,
    pub hour_index: u32,
    pub aggregate_zone_cursor: usize,
    aggregate_zone_work: Option<AggregateZoneWork>,
    pub aggregate_facility_cursor: u8,
    aggregate_facility_work: Option<AggregateZoneWork>,
    timestep_work: Option<TimestepWork>,
    timestep_builder: Option<TimestepBuilder>,
    pub rng_state: u64,
    pub checkpoint_due: bool,
    pub result_backing: EnergyAdmittedResultBacking,
    pub time_series: TimeSeriesTable,
    pub meters: MeterTable,
    time_series_order: Vec<String>,
    meter_order: Vec<String>,
    pub zone_temperature_history: Vec<f64>,
    facility_electricity_j: f64,
    publication: EnergyPublicationChannels,
    pub restore_input: Option<EnergyWirePacket>,
    wire_build: Option<EnergyWireBuild>,
    pub wire_ready: Option<EnergyWirePacket>,
    result: Option<Results>,
    sizing_builder: Option<SizingBuilder>,
    final_sizing: Option<SizingTables>,
    final_summaries: SummaryTables,
    final_environmental: Option<EnvironmentalMetrics>,
    final_resilience: Option<ResilienceMetrics>,
    finalization: FinalizationWork,
    result_build: ResultBuildWork,
    output_writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
    output_payload: Option<RetainedJobPayload>,
    commit_census: Option<CommitCensusWork>,
    commit_reservation: Option<EnergyCommitReservation>,
    commit_acknowledged: bool,
    pub commit_output: Option<EnergyWirePacket>,
    markers: EnergyOutcomeMarkers,
    pending_outcome: Option<EnergyRun>,
    commit_pages_mounted: usize,
    commit_items_encoded: usize,
    encode_section: u8,
    encode_record_cursor: usize,
    encode_sample_cursor: usize,
    encode_fragment_phase: u8,
    encode_name_cursor: usize,
    output_fault: Option<OutputFault>,
    weather_fault: Option<WeatherFault>,
    started: Instant,
    closing: bool,
    close_string_owner: String,
}

pub const ENERGY_ABANDONMENT_SLOTS: usize = 64;

enum EnergyAbandonmentSlot {
    Vacant,
    Reserved(Operation),
    Abandoned(Operation, Box<EnergyJobAuthority>),
}

static ENERGY_ABANDONMENT_REGISTRY: Mutex<[EnergyAbandonmentSlot; ENERGY_ABANDONMENT_SLOTS]> = Mutex::new([const { EnergyAbandonmentSlot::Vacant }; ENERGY_ABANDONMENT_SLOTS]);

pub struct EnergyJob {
    authority: Option<Box<EnergyJobAuthority>>,
    abandonment_slot: usize,
}

impl std::fmt::Debug for EnergyJob {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("EnergyJob").field("abandonment_slot", &self.abandonment_slot).field("mounted", &self.authority.is_some()).finish()
    }
}

impl Deref for EnergyJob {
    type Target = EnergyJobAuthority;

    fn deref(&self) -> &Self::Target {
        self.authority.as_deref().expect("Energy authority is mounted")
    }
}

impl DerefMut for EnergyJob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.authority.as_deref_mut().expect("Energy authority is mounted")
    }
}

impl Drop for EnergyJob {
    fn drop(&mut self) {
        let Some(authority) = self.authority.take() else { return };
        if authority.closing && authority_is_terminal_empty(&authority) {
            release_energy_abandonment_slot(self.abandonment_slot);
            return;
        }
        let operation = authority.operation;
        let mut registry = ENERGY_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        debug_assert!(matches!(registry.get(self.abandonment_slot), Some(EnergyAbandonmentSlot::Reserved(candidate)) if candidate.operation == operation.operation && candidate.generation == operation.generation));
        registry[self.abandonment_slot] = EnergyAbandonmentSlot::Abandoned(operation, authority);
    }
}

fn claim_energy_abandonment_slot(operation: Operation) -> Option<usize> {
    let mut registry = ENERGY_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let index = registry.iter().position(|slot| matches!(slot, EnergyAbandonmentSlot::Vacant))?;
    registry[index] = EnergyAbandonmentSlot::Reserved(operation);
    Some(index)
}

fn release_energy_abandonment_slot(index: usize) {
    let mut registry = ENERGY_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry[index] = EnergyAbandonmentSlot::Vacant;
}

impl EnergyJob {
    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn new(operation: Operation, model: Model, config: SimulationConfig) -> Result<Self, EnergyAdmissionRejected> {
        Self::admit(operation, model, config, EnergyNumericalBounds::default())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn admit(operation: Operation, model: Model, config: SimulationConfig, bounds: EnergyNumericalBounds) -> Result<Self, EnergyAdmissionRejected> {
        let Some(census) = EnergyNumericalCensus::observe(&model, &config) else {
            return Err(EnergyAdmissionRejected { operation, model, config, census: EnergyNumericalCensus::default(), dimension: EnergyNumericalDimension::ArithmeticOverflow });
        };
        if let Some(dimension) = census.first_exceeded(bounds.0) {
            return Err(EnergyAdmissionRejected { operation, model, config, census, dimension });
        }
        let Some(abandonment_slot) = claim_energy_abandonment_slot(operation) else {
            return Err(EnergyAdmissionRejected { operation, model, config, census, dimension: EnergyNumericalDimension::ProcessJobs });
        };
        let mut weather = FixedTable::default();
        if weather.admit(census.weather_records).is_err() {
            release_energy_abandonment_slot(abandonment_slot);
            return Err(EnergyAdmissionRejected { operation, model, config, census, dimension: EnergyNumericalDimension::WeatherRecords });
        }
        let weather_target = weather_copy_target(&config);
        Ok(Self {
            authority: Some(Box::new(EnergyJobAuthority {
                operation,
                model,
                config,
                numerical_census: census,
                stage: EnergyJobStage::Validate,
                tier: EnergyQualityTier::SteadyStateEstimate,
                weather,
                weather_cursor: 0,
                weather_target,
                precompute: None,
                validation: ValidationWork::default(),
                pre: None,
                state: None,
                initialize_cursor: 0,
                initialize_backing_stage: 0,
                warmup_hour: 0,
                previous_temperatures: Vec::new(),
                previous_loads: Vec::new(),
                warmup_convergence: None,
                run_hours: None,
                total_timesteps: 0,
                run_backing_stage: 0,
                hour_index: 0,
                aggregate_zone_cursor: 0,
                aggregate_zone_work: None,
                aggregate_facility_cursor: 0,
                aggregate_facility_work: None,
                timestep_work: None,
                timestep_builder: None,
                rng_state: 0x9e37_79b9_7f4a_7c15 ^ operation.seed,
                checkpoint_due: false,
                result_backing: EnergyAdmittedResultBacking::default(),
                time_series: TimeSeriesTable::default(),
                meters: MeterTable::default(),
                time_series_order: Vec::new(),
                meter_order: Vec::new(),
                zone_temperature_history: Vec::new(),
                facility_electricity_j: 0.0,
                publication: EnergyPublicationChannels::default(),
                restore_input: None,
                wire_build: None,
                wire_ready: None,
                result: None,
                sizing_builder: None,
                final_sizing: None,
                final_summaries: SummaryTables::default(),
                final_environmental: None,
                final_resilience: None,
                finalization: FinalizationWork::default(),
                result_build: ResultBuildWork::default(),
                output_writer: None,
                output_payload: None,
                commit_census: None,
                commit_reservation: None,
                commit_acknowledged: false,
                commit_output: None,
                markers: EnergyOutcomeMarkers::default(),
                pending_outcome: None,
                commit_pages_mounted: 0,
                commit_items_encoded: 0,
                encode_section: 0,
                encode_record_cursor: 0,
                encode_sample_cursor: 0,
                encode_fragment_phase: 0,
                encode_name_cursor: 0,
                output_fault: None,
                weather_fault: None,
                started: Instant::now(),
                closing: false,
                close_string_owner: String::new(),
            })),
            abandonment_slot,
        })
    }

    pub fn recover_abandoned(operation: Operation) -> Option<Self> {
        let mut registry = ENERGY_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let index = registry.iter().position(|slot| matches!(slot, EnergyAbandonmentSlot::Abandoned(candidate, _) if candidate.operation == operation.operation && candidate.generation == operation.generation))?;
        let EnergyAbandonmentSlot::Abandoned(_, authority) = std::mem::replace(&mut registry[index], EnergyAbandonmentSlot::Reserved(operation)) else { unreachable!("matched abandoned Energy authority") };
        Some(Self { authority: Some(authority), abandonment_slot: index })
    }
}

impl EnergyJobAuthority {
    pub fn stage(&self) -> EnergyJobStage {
        self.stage
    }

    pub fn cursor(&self) -> EnergyJobCursor {
        EnergyJobCursor {
            stage: self.stage,
            tier: self.tier,
            warmup_hour: self.warmup_hour,
            warmup_hours: self.config.warmup_days.saturating_mul(24),
            timestep: self.hour_index,
            total_timesteps: self.total_timesteps,
            facility_electricity_kwh: self.facility_electricity_j / 3_600_000.0,
        }
    }

    pub fn preview(&self) -> Option<&EnergyJobPreview> {
        self.publication.preview.as_ref().and_then(EnergyWirePacket::preview)
    }

    pub fn take_preview_packet(&mut self, generation: Generation) -> Result<Option<EnergyWirePacket>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.preview.take())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry_preview_packet(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
        if !self.packet_matches(&packet, EnergyWireKind::Preview) || self.publication.preview.is_some() || self.publication.retiring_preview.is_some() {
            return Err(packet);
        }
        self.publication.preview = Some(packet);
        Ok(())
    }

    pub fn take_checkpoint_packet(&mut self, generation: Generation) -> Result<Option<EnergyWireLease>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.checkpoints.take())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.checkpoints.retry(lease)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.checkpoints.ack(lease)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_checkpoint_for_restore(&mut self, lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
        self.publication.checkpoints.ack_transfer(lease)
    }

    pub fn take_commit_packet(&mut self, generation: Generation) -> Result<Option<EnergyWireLease>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.commits.take())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.commits.retry(lease)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        match self.publication.commits.ack(lease) {
            Ok(()) => {
                self.commit_acknowledged = true;
                Ok(())
            }
            Err(lease) => Err(lease),
        }
    }

    pub fn take_fault_packet(&mut self, generation: Generation) -> Result<Option<EnergyWireLease>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.faults.take())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.faults.retry(lease)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.faults.ack(lease)
    }

    fn packet_matches(&self, packet: &EnergyWirePacket, kind: EnergyWireKind) -> bool {
        packet.kind == kind
            && packet.identity.operation == self.operation.operation.0
            && packet.identity.base_revision == self.operation.base_revision.0
            && packet.identity.generation == self.operation.generation.0
            && packet.identity.seed == self.operation.seed
    }

    pub fn take_results(&mut self) -> Option<Results> {
        self.result.take()
    }

    /// 🧱️ The live per-surface energy table WHILE the run is running — the only path to per-surface
    /// physics before the job settles, since the framework tool-run driver discards the settled
    /// `Complete` candidate's payload unread. A run job reads this on a tier boundary and encodes it
    /// into its tick payload (`ToolRunView::payload`). `None` before the state is initialized.
    pub fn per_surface_energy(&self) -> Option<&results::SurfaceEnergyTable> {
        self.state.as_ref().map(|state| &state.per_surface)
    }

    fn set_stage(&mut self, context: &mut StepContext<'_>, stage: EnergyJobStage) {
        self.stage = stage;
        context.set_stage(stage.label());
    }

    /// 🌦️ Weather-table index of the first hour of the run period.
    fn run_start_record(&self) -> usize {
        if self.weather.len() < 8_760 {
            return 0;
        }
        let period = SimulationKernel::run_period(&self.config);
        (calendar::SimDate::new(period.year, period.start_month, period.start_day).day_of_year() as usize - 1) * 24
    }

    /// 🌦️ The hour at `index` with its neighbours the way an hourly weather file is interpolated:
    /// the previous hour (the day's last hour when the run or the warmup day starts), the current
    /// hour, and the next hour of the same day (the day's first hour after its last).
    fn hour_weather(&self, index: usize, first_of_period: bool) -> HourWeather {
        let len = self.weather.len().max(1);
        let record = |i: usize| *self.weather.get_index(i % len).expect("admitted weather record");
        let hour = index % 24;
        let day_start = index - hour;
        let previous = if hour == 0 && (first_of_period || index == 0) { record(day_start + 23) } else { record(index + len - 1) };
        let next = if hour == 23 { record(day_start) } else { record(index + 1) };
        HourWeather { previous, current: record(index), next }
    }

    fn weather_record(&self, index: usize) -> WeatherRecord {
        if let Some(epw) = &self.config.weather {
            if let Some(record) = epw.records.get(index) {
                return *record;
            }
        }
        match self.config.environment {
            SimulationEnvironment::HeatingDesignDay => site::design_day_hour(index as u32, -10.0),
            SimulationEnvironment::CoolingDesignDay => site::design_day_hour(index as u32, 35.0),
            SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => site::synthetic_hour(index as u32),
        }
    }

    fn wire_identity(&self, sequence: u64) -> EnergyWireIdentity {
        EnergyWireIdentity { operation: self.operation.operation.0, base_revision: self.operation.base_revision.0, generation: self.operation.generation.0, seed: self.operation.seed, sequence }
    }

    pub fn start_wire(&mut self, kind: EnergyWireKind, sequence: u64, stream: JobPayloadStream) -> Result<(), EnergyWireRejection> {
        if self.numerical_census.operations != 1 || self.numerical_census.process_jobs != 1 {
            return Err(EnergyWireRejection::Operations);
        }
        if self.numerical_census.pages == 0 || self.numerical_census.observed_items == 0 {
            return Err(EnergyWireRejection::Pages);
        }
        if self.wire_build.is_some() || self.wire_ready.is_some() || self.commit_reservation.is_some() {
            return Err(EnergyWireRejection::Saturated);
        }
        self.wire_build = Some(EnergyWireBuild { kind, identity: self.wire_identity(sequence), writer: semio_framework_job::RetainedJobPayloadWriter::new(stream), field: 0 });
        Ok(())
    }

    fn wire_fragment(&self, kind: EnergyWireKind, field: u8) -> Option<([u8; ENERGY_WIRE_HEADER_BYTES], usize)> {
        let mut bytes = [0; ENERGY_WIRE_HEADER_BYTES];
        let length = match field {
            0 => {
                bytes = energy_wire_header(kind, self.wire_identity(self.wire_build.as_ref()?.identity.sequence), self.numerical_census, self.stage, self.tier);
                ENERGY_WIRE_HEADER_BYTES
            }
            1 => {
                bytes[..4].copy_from_slice(&self.warmup_hour.to_le_bytes());
                bytes[4..8].copy_from_slice(&self.hour_index.to_le_bytes());
                bytes[8..12].copy_from_slice(&self.total_timesteps.to_le_bytes());
                if kind == EnergyWireKind::Preview {
                    bytes[12..20].copy_from_slice(&(self.facility_electricity_j / 3_600_000.0).to_le_bytes());
                    20
                } else {
                    12
                }
            }
            2 if kind == EnergyWireKind::Checkpoint => {
                bytes[..8].copy_from_slice(&self.rng_state.to_le_bytes());
                bytes[8..16].copy_from_slice(&(self.weather_cursor as u64).to_le_bytes());
                bytes[16..24].copy_from_slice(&(self.weather_target as u64).to_le_bytes());
                24
            }
            3 if kind == EnergyWireKind::Checkpoint => {
                bytes[..8].copy_from_slice(&(self.aggregate_zone_cursor as u64).to_le_bytes());
                bytes[8] = self.aggregate_facility_cursor;
                bytes[9] = self.initialize_backing_stage;
                bytes[10] = self.run_backing_stage;
                bytes[11] = u8::from(self.checkpoint_due);
                bytes[16..24].copy_from_slice(&(self.time_series.series.len() as u64).to_le_bytes());
                bytes[24..32].copy_from_slice(&(self.meters.meters.len() as u64).to_le_bytes());
                bytes[32..40].copy_from_slice(&(self.zone_temperature_history.len() as u64).to_le_bytes());
                bytes[40..48].copy_from_slice(&self.numerical_digest().to_le_bytes());
                48
            }
            _ => return None,
        };
        Some((bytes, length))
    }

    pub fn step_wire_build(&mut self, context: &mut StepContext<'_>) -> Result<bool, EnergyWireRejection> {
        let (kind, field) = self.wire_build.as_ref().map(|build| (build.kind, build.field)).ok_or(EnergyWireRejection::Backing)?;
        let Some((fragment, length)) = self.wire_fragment(kind, field) else {
            let build = self.wire_build.as_mut().expect("wire build remains mounted");
            if build.writer.staged_page_len().is_some() {
                build.writer.commit_staged_page().map_err(|_| EnergyWireRejection::Backing)?;
                return Ok(false);
            }
            let build = self.wire_build.take().expect("wire build remains mounted");
            let payload = build.writer.finish().map_err(|_| EnergyWireRejection::Backing)?;
            self.wire_ready = Some(EnergyWirePacket { kind: build.kind, identity: build.identity, payload, preview: None, reservation: None });
            return Ok(true);
        };
        let build = self.wire_build.as_mut().expect("wire build remains mounted");
        if build.writer.staged_page_len().is_none() {
            build.writer.begin_staged_page(context).map_err(|_| EnergyWireRejection::Backing)?;
            return Ok(false);
        }
        if build.writer.staged_page_remaining() < length {
            build.writer.commit_staged_page().map_err(|_| EnergyWireRejection::Backing)?;
            return Ok(false);
        }
        build.writer.write_staged(&fragment[..length]).map_err(|_| EnergyWireRejection::Backing)?;
        build.field = build.field.checked_add(1).ok_or(EnergyWireRejection::Items)?;
        Ok(false)
    }

    fn begin_preview(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return EnergyRun::Fault,
        };
        if self.start_wire(EnergyWireKind::Preview, sequence, JobPayloadStream::Preview).is_err() {
            return EnergyRun::Yield;
        }
        EnergyRun::Yield
    }

    fn drive_wire_publication(&mut self, context: &mut StepContext<'_>) -> Option<EnergyRun> {
        if let Some(retiring) = self.publication.retiring_preview.as_mut() {
            let step = retiring.close_step(context.retained_grant());
            match energy_spend(context, step) {
                Ok(true) => self.publication.retiring_preview = None,
                Ok(false) => return Some(EnergyRun::Yield),
                Err(_) => return Some(Self::fault()),
            }
        }
        if self.wire_build.is_some() {
            if self.step_wire_build(context).is_err() {
                return Some(Self::fault());
            }
            return Some(EnergyRun::Yield);
        }
        let mut packet = self.wire_ready.take()?;
        match packet.kind {
            EnergyWireKind::Preview => {
                if self.publication.preview.is_some() {
                    self.publication.retiring_preview = self.publication.preview.take();
                    self.wire_ready = Some(packet);
                    return Some(EnergyRun::Yield);
                }
                let Some(preview) = decode_preview_packet(&packet) else {
                    self.wire_ready = Some(packet);
                    return Some(Self::fault());
                };
                packet.preview = Some(preview);
                self.publication.preview = Some(packet);
                Some(EnergyRun::Preview)
            }
            EnergyWireKind::Checkpoint => match self.publication.checkpoints.push(packet) {
                Ok(()) => Some(EnergyRun::Checkpoint(self.hour_index as u64)),
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(EnergyRun::Yield)
                }
            },
            EnergyWireKind::Commit => match self.publication.commits.push(packet) {
                Ok(()) => {
                    self.stage = EnergyJobStage::Complete;
                    Some(EnergyRun::Yield)
                }
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(EnergyRun::Yield)
                }
            },
            EnergyWireKind::Fault => match self.publication.faults.push(packet) {
                Ok(()) => Some(Self::fault()),
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(EnergyRun::Yield)
                }
            },
        }
    }

    #[cfg(test)]
    fn numerical_cursor_signature(&self) -> [u64; 13] {
        [
            self.stage.code() as u64,
            self.tier.code() as u64,
            self.weather_cursor as u64,
            self.weather_target as u64,
            self.initialize_cursor as u64,
            self.warmup_hour as u64,
            self.hour_index as u64,
            self.aggregate_zone_cursor as u64,
            self.aggregate_facility_cursor as u64,
            self.encode_section as u64,
            self.encode_record_cursor as u64,
            self.encode_sample_cursor as u64,
            self.encode_name_cursor as u64,
        ]
    }

    pub fn numerical_digest(&self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        for value in [
            self.stage.code() as u64,
            self.tier.code() as u64,
            self.rng_state,
            self.weather_cursor as u64,
            self.weather_target as u64,
            self.initialize_cursor as u64,
            self.initialize_backing_stage as u64,
            self.warmup_hour as u64,
            self.total_timesteps as u64,
            self.run_backing_stage as u64,
            self.hour_index as u64,
            self.aggregate_zone_cursor as u64,
            self.aggregate_facility_cursor as u64,
            self.facility_electricity_j.to_bits(),
            self.time_series.series.len() as u64,
            self.meters.meters.len() as u64,
            self.zone_temperature_history.len() as u64,
            self.previous_temperatures.len() as u64,
            self.previous_loads.len() as u64,
            self.state.as_ref().map_or(0, |state| state.zones.len()) as u64,
            self.state.as_ref().map_or(0, |state| state.surfaces.len()) as u64,
            self.validation.stage as u64,
            self.precompute.is_some() as u64,
            self.pre.is_some() as u64,
            self.timestep_builder.is_some() as u64,
            self.timestep_work.is_some() as u64,
            self.aggregate_zone_work.is_some() as u64,
            self.aggregate_facility_work.is_some() as u64,
            self.checkpoint_due as u64,
        ] {
            digest ^= value;
            digest = digest.wrapping_mul(0x100_0000_01b3);
        }
        if let Some(builder) = self.precompute.as_ref() {
            for value in builder.retained_wire_signature() {
                digest ^= value;
                digest = digest.wrapping_mul(0x100_0000_01b3);
            }
        }
        if let Some(builder) = self.timestep_builder.as_ref() {
            for value in builder.retained_wire_signature() {
                digest ^= value;
                digest = digest.wrapping_mul(0x100_0000_01b3);
            }
        }
        if let Some(work) = self.timestep_work.as_ref() {
            for value in work.retained_wire_signature() {
                digest ^= value;
                digest = digest.wrapping_mul(0x100_0000_01b3);
            }
        }
        for work in [self.aggregate_zone_work.as_ref(), self.aggregate_facility_work.as_ref()].into_iter().flatten() {
            for value in [work.stage as u64, work.phase as u64, work.copy_index as u64, work.cursor as u64, work.timestamps.len() as u64, work.values.len() as u64] {
                digest ^= value;
                digest = digest.wrapping_mul(0x100_0000_01b3);
            }
        }
        digest
    }

    fn fault() -> EnergyRun {
        EnergyRun::Fault
    }

    fn begin_fault(&mut self) -> EnergyRun {
        if self.start_wire(EnergyWireKind::Fault, 0, JobPayloadStream::Fault).is_ok() {
            EnergyRun::Yield
        } else {
            Self::fault()
        }
    }

    fn step_validation(&mut self) -> bool {
        let work = &mut self.validation;
        match work.stage {
            ValidationStage::ReserveZones => {
                if work.zone_ids.admit(self.model.zones.len()).is_err() {
                    work.fatal_code = 1;
                }
                advance_validation(work, ValidationStage::ReserveMaterials);
            }
            ValidationStage::ReserveMaterials => {
                if work.material_ids.admit(self.model.materials.len() + self.model.glazing_materials.len() + self.model.gas_materials.len()).is_err() {
                    work.fatal_code = 1;
                }
                advance_validation(work, ValidationStage::ReserveConstructions);
            }
            ValidationStage::ReserveConstructions => {
                if work.construction_ids.admit(self.model.constructions.len()).is_err() {
                    work.fatal_code = 1;
                }
                advance_validation(work, ValidationStage::ReserveSurfaces);
            }
            ValidationStage::ReserveSurfaces => {
                if work.surface_ids.admit(self.model.surfaces.len()).is_err() {
                    work.fatal_code = 1;
                }
                advance_validation(work, ValidationStage::IndexZones);
            }
            ValidationStage::IndexZones => {
                if let Some(zone) = self.model.zones.get(work.cursor) {
                    match work.zone_ids.insert(zone.id, ()) {
                        Ok(None) => {}
                        Ok(Some(_)) => work.fatal_code = 2,
                        Err(_) => work.fatal_code = 1,
                    }
                    work.cursor += 1;
                } else {
                    if work.zone_ids.is_empty() {
                        work.fatal_code = 3;
                    }
                    advance_validation(work, ValidationStage::IndexMaterials);
                }
            }
            ValidationStage::IndexMaterials => {
                let materials = self.model.materials.len();
                let glazing = self.model.glazing_materials.len();
                let layer = self.model.materials.get(work.cursor).map(|material| material.id).or_else(|| work.cursor.checked_sub(materials).and_then(|index| self.model.glazing_materials.get(index)).map(|material| material.id)).or_else(|| work.cursor.checked_sub(materials + glazing).and_then(|index| self.model.gas_materials.get(index)).map(|material| material.id));
                if let Some(layer_id) = layer {
                    match work.material_ids.insert(layer_id, ()) {
                        Ok(None) => {}
                        Ok(Some(_)) => work.fatal_code = 4,
                        Err(_) => work.fatal_code = 1,
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::IndexConstructions);
                }
            }
            ValidationStage::IndexConstructions => {
                if let Some(construction) = self.model.constructions.get(work.cursor) {
                    match work.construction_ids.insert(construction.id, ()) {
                        Ok(None) => {}
                        Ok(Some(_)) => work.fatal_code = 5,
                        Err(_) => work.fatal_code = 1,
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::IndexSurfaces);
                }
            }
            ValidationStage::IndexSurfaces => {
                if let Some(surface) = self.model.surfaces.get(work.cursor) {
                    match work.surface_ids.insert(surface.id, ()) {
                        Ok(None) => {}
                        Ok(Some(_)) => work.fatal_code = 6,
                        Err(_) => work.fatal_code = 1,
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckZones);
                }
            }
            ValidationStage::CheckZones => {
                if let Some(zone) = self.model.zones.get(work.cursor) {
                    if zone.volume_m3 <= 0.0 {
                        work.fatal_code = 7;
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckMaterials);
                }
            }
            ValidationStage::CheckMaterials => {
                if let Some(material) = self.model.materials.get(work.cursor) {
                    if material.thickness_m <= 0.0 || material.conductivity_w_m_k <= 0.0 {
                        work.fatal_code = 8;
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckConstructions);
                }
            }
            ValidationStage::CheckConstructions => {
                if let Some(construction) = self.model.constructions.get(work.cursor) {
                    if construction.layer_material_ids.is_empty() {
                        work.fatal_code = 9;
                        work.cursor += 1;
                    } else {
                        work.stage = ValidationStage::CheckConstructionLayers;
                        work.child_cursor = 0;
                    }
                } else {
                    advance_validation(work, ValidationStage::CheckSurfaces);
                }
            }
            ValidationStage::CheckConstructionLayers => {
                let construction = &self.model.constructions[work.cursor];
                if let Some(material_id) = construction.layer_material_ids.get(work.child_cursor) {
                    if !work.material_ids.contains_key(material_id) {
                        work.fatal_code = 10;
                    }
                    work.child_cursor += 1;
                } else {
                    work.cursor += 1;
                    work.child_cursor = 0;
                    work.stage = ValidationStage::CheckConstructions;
                }
            }
            ValidationStage::CheckSurfaces => {
                if let Some(surface) = self.model.surfaces.get(work.cursor) {
                    if !work.zone_ids.contains_key(&surface.zone_id) || !work.construction_ids.contains_key(&surface.construction_id) || surface.vertices_m.len() < 3 {
                        work.fatal_code = 11;
                    }
                    if let model::OutsideBoundary::Interzone(other) = surface.outside_boundary_condition {
                        if !work.surface_ids.contains_key(&other) {
                            work.fatal_code = 12;
                        }
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckFenestrations);
                }
            }
            ValidationStage::CheckFenestrations => {
                if let Some(fenestration) = self.model.fenestrations.get(work.cursor) {
                    if !work.surface_ids.contains_key(&fenestration.surface_id) {
                        work.fatal_code = 13;
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckZoneReferences);
                }
            }
            ValidationStage::CheckZoneReferences => {
                let zone_id = match work.zone_reference_family {
                    0 => self.model.thermostats.get(work.cursor).map(|value| value.zone_id),
                    1 => self.model.humidistats.get(work.cursor).map(|value| value.zone_id),
                    2 => self.model.ideal_loads.get(work.cursor).map(|value| value.zone_id),
                    3 => self.model.zone_equipment.get(work.cursor).map(|value| value.zone_id),
                    4 => self.model.mechanical_ventilations.get(work.cursor).map(|value| value.zone_id),
                    5 => self.model.daylight_zones.get(work.cursor).map(|value| value.zone_id),
                    _ => {
                        work.stage = ValidationStage::Complete;
                        None
                    }
                };
                if let Some(zone_id) = zone_id {
                    if !work.zone_ids.contains_key(&zone_id) {
                        work.fatal_code = 14;
                    }
                    work.cursor += 1;
                } else if work.zone_reference_family <= 5 {
                    work.zone_reference_family += 1;
                    work.cursor = 0;
                }
            }
            ValidationStage::Complete => return true,
        }
        false
    }

    fn step_finalization(&mut self) -> bool {
        let work = &mut self.finalization;
        match work.stage {
            FinalizationStage::MeterTotals => {
                if let Some(name) = self.meter_order.get(work.cursor) {
                    let _ = name;
                    if let Some(meter) = self.meters.meters.get_index(work.cursor) {
                        match meter.fuel {
                            FuelType::Electricity => work.electricity_kwh += meter.energy_kwh(),
                            FuelType::NaturalGas => work.gas_kwh += meter.energy_kwh(),
                            _ => {}
                        }
                    }
                    work.cursor += 1;
                } else {
                    advance_finalization(work, FinalizationStage::FloorArea);
                }
            }
            FinalizationStage::FloorArea => {
                if let Some(zone) = self.model.zones.get(work.cursor) {
                    work.floor_area_m2 += zone.volume_m3 / 3.0;
                    work.cursor += 1;
                } else {
                    work.floor_area_m2 = work.floor_area_m2.max(1.0);
                    advance_finalization(work, FinalizationStage::HistoryResilience);
                }
            }
            FinalizationStage::HistoryResilience => {
                if let Some(temperature) = self.zone_temperature_history.get(work.cursor).copied() {
                    if temperature > 32.0 {
                        work.resilience.hours_above_heat_index_32c += 1;
                    }
                    if temperature < 10.0 {
                        work.resilience.hours_below_10c += 1;
                    }
                    if temperature < 19.0 {
                        work.resilience.unmet_heating_hours += 1;
                    }
                    if temperature > 27.0 {
                        work.resilience.unmet_cooling_hours += 1;
                    }
                    work.cursor += 1;
                } else {
                    advance_finalization(work, FinalizationStage::TariffPeriod);
                }
            }
            FinalizationStage::TariffPeriod => {
                work.annual_cost = work.electricity_kwh * 0.1 * 12.0;
                advance_finalization(work, FinalizationStage::LifeCycleYear);
            }
            FinalizationStage::LifeCycleYear => {
                if work.cursor < 25 {
                    let year = work.cursor + 1;
                    work.present_value_energy += work.annual_cost / 1.03_f64.powi(year as i32);
                    work.cursor += 1;
                } else {
                    advance_finalization(work, FinalizationStage::SummaryElectricity);
                }
            }
            FinalizationStage::SummaryElectricity => {
                let value = work.electricity_kwh;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "Electricity", value, "kWh") {
                    advance_finalization(work, FinalizationStage::SummaryGas);
                }
            }
            FinalizationStage::SummaryGas => {
                let value = work.gas_kwh;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "Natural Gas", value, "kWh") {
                    advance_finalization(work, FinalizationStage::SummaryEui);
                }
            }
            FinalizationStage::SummaryEui => {
                let value = work.electricity_kwh / work.floor_area_m2;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "Energy Use Intensity", value, "kWh/m²") {
                    advance_finalization(work, FinalizationStage::SummaryAnnualCost);
                }
            }
            FinalizationStage::SummaryAnnualCost => {
                let value = work.annual_cost;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "Annual Energy Cost", value, "USD") {
                    advance_finalization(work, FinalizationStage::SummaryLcca);
                }
            }
            FinalizationStage::SummaryLcca => {
                let value = work.present_value_energy;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "LCCA Present Value", value, "USD") {
                    advance_finalization(work, FinalizationStage::Environmental);
                }
            }
            FinalizationStage::Environmental => {
                let factors = SourceEnergyFactors::default();
                let emissions = EmissionFactors::default();
                self.final_environmental = Some(EnvironmentalMetrics {
                    site_energy_kwh: work.electricity_kwh + work.gas_kwh,
                    source_energy_kwh: work.electricity_kwh * factors.electricity + work.gas_kwh * factors.natural_gas,
                    co2_kg: work.electricity_kwh * emissions.electricity_kg_per_kwh + work.gas_kwh * emissions.natural_gas_kg_per_kwh,
                });
                advance_finalization(work, FinalizationStage::Resilience);
            }
            FinalizationStage::Resilience => {
                self.final_resilience = Some(work.resilience.clone());
                advance_finalization(work, FinalizationStage::Complete);
            }
            FinalizationStage::Complete => return true,
        }
        false
    }

    fn step_aggregate_zone(&mut self) -> bool {
        let Some(zone) = self.model.zones.get(self.aggregate_zone_cursor) else { return true };
        let Some(zone_state) = self.state.as_ref().and_then(|state| state.zones.get(&zone.id)) else { return true };
        let work = self.aggregate_zone_work.get_or_insert_with(AggregateZoneWork::new);
        let dt_s = 3600.0;
        match work.stage {
            AggregateZoneStage::Temperature => {
                if self.hour_index == 0 && work.phase == 0 {
                    if !copy_aggregate_names_step(work, ["Zone Air Temperature [", zone.name.as_str(), "]"]) {
                        return false;
                    }
                    work.phase = 1;
                } else if self.hour_index != 0 && work.phase == 0 {
                    work.phase = 4;
                }
                if work.phase == 1 {
                    if work.timestamps.try_reserve_exact(self.total_timesteps as usize).is_err() {
                        work.backing_rejected = true;
                        return false;
                    }
                    work.phase = 2;
                    return false;
                }
                if work.phase == 2 {
                    if work.values.try_reserve_exact(self.total_timesteps as usize).is_err() {
                        work.backing_rejected = true;
                        return false;
                    }
                    self.result_backing.samples_per_series = self.total_timesteps as usize;
                    work.phase = 3;
                    return false;
                }
                if work.phase == 3 {
                    if self.time_series.series.len() >= self.result_backing.series_slots {
                        work.backing_rejected = true;
                        return false;
                    }
                    let [map_key, series_key, order_key] = std::mem::replace(&mut work.names, std::array::from_fn(|_| String::new()));
                    if self
                        .time_series
                        .series
                        .insert_stable(
                            map_key,
                            TimeSeries { key: series_key, timestamps_hours: std::mem::take(&mut work.timestamps), values: std::mem::take(&mut work.values), unit: Unit::Celsius, admitted_samples: self.result_backing.samples_per_series },
                        )
                        .is_err()
                    {
                        work.backing_rejected = true;
                        return false;
                    }
                    self.time_series_order.push(order_key);
                    work.phase = 4;
                    return false;
                }
                let series = self.time_series.series.get_index_mut(self.aggregate_zone_cursor).expect("admitted zone series");
                if self.zone_temperature_history.len() >= self.result_backing.history_slots {
                    work.backing_rejected = true;
                    return false;
                }
                if let Err(fault) = series.append_admitted(self.hour_index as f64, zone_state.mean_air_temp_c) {
                    work.series_fault = Some(fault);
                    work.backing_rejected = true;
                    return false;
                }
                self.zone_temperature_history.push(zone_state.mean_air_temp_c);
                work.advance(AggregateZoneStage::Heating);
            }
            AggregateZoneStage::Heating => {
                if self.hour_index == 0 && work.phase == 0 {
                    if !copy_aggregate_names_step(work, ["", zone.name.as_str(), " Heating"]) {
                        return false;
                    }
                    work.phase = 1;
                } else if self.hour_index != 0 && work.phase == 0 {
                    work.phase = 2;
                }
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::Electricity, EndUse::Heating);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.aggregate_zone_cursor * 3).expect("admitted heating meter").accumulate(zone_state.delivered.heating_w, dt_s, self.hour_index as f64);
                self.facility_electricity_j += zone_state.delivered.heating_w * dt_s;
                work.advance(AggregateZoneStage::Cooling);
            }
            AggregateZoneStage::Cooling => {
                if self.hour_index == 0 && work.phase == 0 {
                    if !copy_aggregate_names_step(work, ["", zone.name.as_str(), " Cooling"]) {
                        return false;
                    }
                    work.phase = 1;
                } else if self.hour_index != 0 && work.phase == 0 {
                    work.phase = 2;
                }
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::Electricity, EndUse::Cooling);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.aggregate_zone_cursor * 3 + 1).expect("admitted cooling meter").accumulate(zone_state.delivered.cooling_w, dt_s, self.hour_index as f64);
                self.facility_electricity_j += zone_state.delivered.cooling_w * dt_s;
                work.advance(AggregateZoneStage::Fan);
            }
            AggregateZoneStage::Fan => {
                if self.hour_index == 0 && work.phase == 0 {
                    if !copy_aggregate_names_step(work, ["", zone.name.as_str(), " Fans"]) {
                        return false;
                    }
                    work.phase = 1;
                } else if self.hour_index != 0 && work.phase == 0 {
                    work.phase = 2;
                }
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::Electricity, EndUse::Fans);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.aggregate_zone_cursor * 3 + 2).expect("admitted fan meter").accumulate(zone_state.delivered.fan_w, dt_s, self.hour_index as f64);
                self.facility_electricity_j += zone_state.delivered.fan_w * dt_s;
                work.advance(AggregateZoneStage::Complete);
            }
            AggregateZoneStage::Complete => {
                self.aggregate_zone_work = None;
                return true;
            }
        }
        false
    }

    fn step_aggregate_facility(&mut self) -> bool {
        let Some(state) = &self.state else { return true };
        let dt_s = 3600.0;
        if self.aggregate_facility_cursor > 1 {
            self.aggregate_facility_cursor = 0;
            self.aggregate_facility_work = None;
            return true;
        }
        let work = self.aggregate_facility_work.get_or_insert_with(AggregateZoneWork::new);
        if self.hour_index == 0 && work.phase == 0 {
            let source = if self.aggregate_facility_cursor == 0 { "Facility Heating" } else { "Facility PV" };
            if !copy_aggregate_names_step(work, [source, "", ""]) {
                return false;
            }
            work.phase = 1;
        } else if self.hour_index != 0 && work.phase == 0 {
            work.phase = 2;
        }
        match self.aggregate_facility_cursor {
            0 => {
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::Electricity, EndUse::Heating);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.model.zones.len() * 3).expect("admitted facility heating meter").accumulate(state.delivered_total.heating_w, dt_s, self.hour_index as f64);
                self.facility_electricity_j += state.delivered_total.heating_w * dt_s;
                self.aggregate_facility_cursor = 1;
                self.aggregate_facility_work = None;
            }
            1 => {
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::OnSiteGeneration, EndUse::Generators);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.model.zones.len() * 3 + 1).expect("admitted facility PV meter").accumulate(-state.delivered_total.pv_generation_w, dt_s, self.hour_index as f64);
                self.aggregate_facility_cursor = 2;
                self.aggregate_facility_work = None;
            }
            _ => {
                self.aggregate_facility_cursor = 0;
                return true;
            }
        }
        false
    }

    fn step_result_build(&mut self) -> Result<bool, Error> {
        match self.result_build.stage {
            ResultBuildStage::ModelName => {
                if copy_string_step(&mut self.result_build.model_name, &mut self.result_build.cursor, &mut self.result_build.reserved, &self.model.name)? {
                    advance_result_build(&mut self.result_build, ResultBuildStage::ModelVersion);
                }
            }
            ResultBuildStage::ModelVersion => {
                if copy_string_step(&mut self.result_build.model_version, &mut self.result_build.cursor, &mut self.result_build.reserved, &self.model.version)? {
                    advance_result_build(&mut self.result_build, ResultBuildStage::WeatherLocation);
                }
            }
            ResultBuildStage::WeatherLocation => {
                let source = self.config.weather.as_ref().map_or("synthetic", |weather| weather.location.as_str());
                if copy_string_step(&mut self.result_build.weather_location, &mut self.result_build.cursor, &mut self.result_build.reserved, source)? {
                    advance_result_build(&mut self.result_build, ResultBuildStage::Assemble);
                }
            }
            ResultBuildStage::Assemble => {
                self.result = Some(Results {
                    time_series: std::mem::take(&mut self.time_series),
                    meters: std::mem::take(&mut self.meters),
                    summaries: std::mem::take(&mut self.final_summaries),
                    sizing: self.final_sizing.take().unwrap_or_default(),
                    environmental: self.final_environmental.take().unwrap_or_default(),
                    resilience: self.final_resilience.take().unwrap_or_default(),
                    diagnostics: Default::default(),
                    // 🧱️ The per-surface accumulator, projected to kWh. `self.state` is still resident
                    // here — it is only released by `close_step`'s bounded retirement.
                    per_surface: self.state.as_ref().map(|state| state.per_surface.summaries().collect()).unwrap_or_default(),
                    run_metadata: RunMetadata {
                        model_name: std::mem::take(&mut self.result_build.model_name),
                        model_version: std::mem::take(&mut self.result_build.model_version),
                        weather_location: std::mem::take(&mut self.result_build.weather_location),
                        timesteps: self.hour_index,
                        warmup_days: self.config.warmup_days,
                        elapsed_ms: self.started.elapsed().as_millis() as u64,
                    },
                });
                advance_result_build(&mut self.result_build, ResultBuildStage::Complete);
            }
            ResultBuildStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    fn write_output_fragment(&mut self, context: &mut StepContext<'_>, fragment: &[u8]) -> Result<bool, OutputFault> {
        let writer = self.output_writer.as_mut().ok_or(OutputFault::BackingRejected)?;
        if writer.staged_page_len().is_none() {
            let reservation = self.commit_reservation.as_mut().ok_or(OutputFault::BackingRejected)?;
            if self.commit_pages_mounted >= reservation.pages {
                return Err(OutputFault::BackingRejected);
            }
            let source = reservation.take_source(self.commit_pages_mounted).ok_or(OutputFault::BackingRejected)?;
            match context.admit_payload_page(writer, source) {
                Ok(page) => page.stage(),
                Err(rejected) => {
                    reservation.restore_source(self.commit_pages_mounted, rejected.into_source());
                    return Err(OutputFault::BackingRejected);
                }
            }
            self.commit_pages_mounted += 1;
            return Ok(false);
        }
        if writer.staged_page_remaining() < fragment.len() {
            writer.commit_staged_page().map_err(|_| OutputFault::BackingRejected)?;
            return Ok(false);
        }
        writer.write_staged(fragment).map_err(|_| OutputFault::BackingRejected)?;
        self.commit_items_encoded = self.commit_items_encoded.checked_add(1).ok_or(OutputFault::BackingRejected)?;
        Ok(true)
    }

    fn step_commit_census(&mut self) -> Result<bool, OutputFault> {
        let result = self.result.as_ref().ok_or(OutputFault::MissingResult)?;
        let mut work = self.commit_census.take().unwrap_or_default();
        let add_output = |work: &mut CommitCensusWork, bytes: usize, items: usize| -> Result<(), OutputFault> {
            work.output_bytes = work.output_bytes.checked_add(bytes).ok_or(OutputFault::BackingRejected)?;
            work.output_items = work.output_items.checked_add(items).ok_or(OutputFault::BackingRejected)?;
            Ok(())
        };
        let add_resident = |work: &mut CommitCensusWork, totals: Option<(usize, usize, usize)>| -> Result<(), OutputFault> {
            if let Some((pages, bytes, items)) = totals {
                work.resident_pages = work.resident_pages.checked_add(pages).ok_or(OutputFault::BackingRejected)?;
                work.resident_bytes = work.resident_bytes.checked_add(bytes).ok_or(OutputFault::BackingRejected)?;
                work.resident_items = work.resident_items.checked_add(items).ok_or(OutputFault::BackingRejected)?;
            }
            Ok(())
        };
        match work.stage {
            CommitCensusStage::Base => {
                add_output(&mut work, ENERGY_WIRE_HEADER_BYTES + 20 + 4 + 4, 4)?;
                work.stage = CommitCensusStage::MeterName;
            }
            CommitCensusStage::MeterName => {
                let Some(meter) = result.meters.meters.get_index(work.record) else {
                    work.record = 0;
                    work.byte = 0;
                    work.sample = 0;
                    work.stage = CommitCensusStage::SeriesName;
                    self.commit_census = Some(work);
                    return Ok(false);
                };
                if work.sample == 0 {
                    add_output(&mut work, 4, 1)?;
                    work.sample = 1;
                } else if work.byte < meter.name.len() {
                    let character = meter.name[work.byte..].chars().next().ok_or(OutputFault::MissingMeter)?;
                    add_output(&mut work, character.len_utf8(), 1)?;
                    work.byte += character.len_utf8();
                } else {
                    work.stage = CommitCensusStage::MeterRecord;
                }
            }
            CommitCensusStage::MeterRecord => {
                add_output(&mut work, 29, 1)?;
                work.record += 1;
                work.byte = 0;
                work.sample = 0;
                work.stage = CommitCensusStage::MeterName;
            }
            CommitCensusStage::SeriesName => {
                let Some(series) = result.time_series.series.get_index(work.record) else {
                    work.record = 0;
                    work.byte = 0;
                    work.sample = 0;
                    work.stage = CommitCensusStage::Summary;
                    self.commit_census = Some(work);
                    return Ok(false);
                };
                if work.sample == 0 {
                    add_output(&mut work, 4, 1)?;
                    work.sample = 1;
                } else if work.byte < series.key.len() {
                    let character = series.key[work.byte..].chars().next().ok_or(OutputFault::MissingSeries)?;
                    add_output(&mut work, character.len_utf8(), 1)?;
                    work.byte += character.len_utf8();
                } else {
                    work.stage = CommitCensusStage::SeriesHeader;
                }
            }
            CommitCensusStage::SeriesHeader => {
                add_output(&mut work, 5, 1)?;
                work.sample = 0;
                work.stage = CommitCensusStage::SeriesSample;
            }
            CommitCensusStage::SeriesSample => {
                let series = result.time_series.series.get_index(work.record).ok_or(OutputFault::MissingSeries)?;
                if work.sample < series.values.len() {
                    add_output(&mut work, 16, 1)?;
                    work.sample += 1;
                } else {
                    work.record += 1;
                    work.byte = 0;
                    work.sample = 0;
                    work.stage = CommitCensusStage::SeriesName;
                }
            }
            CommitCensusStage::Summary => {
                if work.record < result.summaries.annual_energy.len() {
                    add_output(&mut work, 8, 1)?;
                    work.record += 1;
                } else {
                    work.record = 0;
                    work.stage = CommitCensusStage::PreviewChannel;
                }
            }
            CommitCensusStage::PreviewChannel => {
                let packet = if work.channel_slot == 0 { self.publication.preview.as_ref() } else { self.publication.retiring_preview.as_ref() };
                add_resident(&mut work, packet.map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1))))?;
                work.channel_slot += 1;
                if work.channel_slot > 1 {
                    work.stage = CommitCensusStage::CheckpointChannel;
                    work.channel_slot = 0;
                }
            }
            CommitCensusStage::CheckpointChannel | CommitCensusStage::CommitChannel | CommitCensusStage::FaultChannel => {
                let queue = match work.stage {
                    CommitCensusStage::CheckpointChannel => &self.publication.checkpoints,
                    CommitCensusStage::CommitChannel => &self.publication.commits,
                    CommitCensusStage::FaultChannel => &self.publication.faults,
                    _ => unreachable!("channel stage matched"),
                };
                let totals = queue.retained_totals_at(work.channel_slot);
                add_resident(&mut work, totals)?;
                work.channel_slot += 1;
                if work.channel_slot > ENERGY_WIRE_QUEUE_SLOTS {
                    work.channel_slot = 0;
                    work.stage = match work.stage {
                        CommitCensusStage::CheckpointChannel => CommitCensusStage::CommitChannel,
                        CommitCensusStage::CommitChannel => CommitCensusStage::FaultChannel,
                        CommitCensusStage::FaultChannel => CommitCensusStage::ReadyPacket,
                        _ => unreachable!("channel stage matched"),
                    };
                }
            }
            CommitCensusStage::ReadyPacket => {
                add_resident(&mut work, self.wire_ready.as_ref().map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1))))?;
                work.stage = CommitCensusStage::RestoreInput;
            }
            CommitCensusStage::RestoreInput => {
                add_resident(&mut work, self.restore_input.as_ref().map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1))))?;
                work.stage = CommitCensusStage::Reserve;
            }
            CommitCensusStage::Reserve => {
                let pages = work.output_bytes.checked_add(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - 1).ok_or(OutputFault::BackingRejected)? / semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
                let aggregate_pages = work.resident_pages.checked_add(pages).ok_or(OutputFault::BackingRejected)?;
                let aggregate_bytes = work.resident_bytes.checked_add(work.output_bytes).ok_or(OutputFault::BackingRejected)?;
                let aggregate_items = work.resident_items.checked_add(work.output_items).and_then(|items| items.checked_add(pages)).ok_or(OutputFault::BackingRejected)?;
                if self.publication.commits.len >= ENERGY_WIRE_QUEUE_SLOTS
                    || pages > self.numerical_census.pages
                    || aggregate_pages > semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES
                    || aggregate_bytes > self.numerical_census.observed_bytes
                    || aggregate_bytes > semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES
                    || aggregate_items > self.numerical_census.observed_items
                {
                    return Err(OutputFault::BackingRejected);
                }
                self.publication.commits.reserve_push().map_err(|_| OutputFault::BackingRejected)?;
                work.reservation = Some(EnergyCommitReservation::new(pages, work.output_bytes, work.output_items));
                work.stage = CommitCensusStage::ReservePage;
            }
            CommitCensusStage::ReservePage => {
                let reservation = work.reservation.as_mut().ok_or(OutputFault::BackingRejected)?;
                if reservation.prepared < reservation.pages {
                    reservation.sources[reservation.prepared] = Some(semio_framework_job::JobPayloadPageSource::new());
                    reservation.prepared += 1;
                } else {
                    self.commit_reservation = work.reservation.take();
                    work.stage = CommitCensusStage::Complete;
                }
            }
            CommitCensusStage::Complete => {
                return Ok(true);
            }
        }
        self.commit_census = Some(work);
        Ok(false)
    }

    fn encode_output_step(&mut self, context: &mut StepContext<'_>) -> Result<bool, OutputFault> {
        let Some(result) = self.result.as_ref() else { return Err(OutputFault::MissingResult) };
        match self.encode_section {
            0 => {
                let identity = self.wire_identity(0);
                let header = energy_wire_header(EnergyWireKind::Commit, identity, self.numerical_census, self.stage, self.tier);
                if self.write_output_fragment(context, &header)? {
                    self.encode_section = 1;
                }
            }
            1 => {
                let mut fragment = [0; 20];
                fragment[..8].copy_from_slice(b"RESULTS1");
                fragment[8..12].copy_from_slice(&result.run_metadata.timesteps.to_le_bytes());
                fragment[12..16].copy_from_slice(&result.run_metadata.warmup_days.to_le_bytes());
                fragment[16..20].copy_from_slice(&(self.meter_order.len() as u32).to_le_bytes());
                if self.write_output_fragment(context, &fragment)? {
                    self.encode_section = 2;
                }
            }
            2 => {
                if self.meter_order.get(self.encode_record_cursor).is_some() {
                    let Some(meter) = result.meters.meters.get_index(self.encode_record_cursor) else { return Err(OutputFault::MissingMeter) };
                    if self.encode_fragment_phase == 0 {
                        if self.write_output_fragment(context, &(meter.name.len() as u32).to_le_bytes())? {
                            self.encode_fragment_phase = 1;
                        }
                    } else if self.encode_fragment_phase == 1 && self.encode_name_cursor < meter.name.len() {
                        let character = meter.name[self.encode_name_cursor..].chars().next().ok_or(OutputFault::MissingMeter)?;
                        let mut storage = [0; 4];
                        let fragment = character.encode_utf8(&mut storage).as_bytes();
                        if self.write_output_fragment(context, fragment)? {
                            self.encode_name_cursor += fragment.len();
                        }
                    } else {
                        let mut fragment = [0; 29];
                        fragment[0] = fuel_code(meter.fuel);
                        fragment[1..5].copy_from_slice(&end_use_code(meter.end_use).to_le_bytes());
                        fragment[5..13].copy_from_slice(&meter.energy_j.to_le_bytes());
                        fragment[13..21].copy_from_slice(&meter.peak_demand_w.to_le_bytes());
                        fragment[21..29].copy_from_slice(&meter.peak_demand_hour.to_le_bytes());
                        if self.write_output_fragment(context, &fragment)? {
                            self.encode_record_cursor += 1;
                            self.encode_name_cursor = 0;
                            self.encode_fragment_phase = 0;
                        }
                    }
                } else {
                    let fragment = (self.time_series_order.len() as u32).to_le_bytes();
                    if self.write_output_fragment(context, &fragment)? {
                        self.encode_section = 3;
                        self.encode_record_cursor = 0;
                        self.encode_name_cursor = 0;
                        self.encode_fragment_phase = 0;
                    }
                }
            }
            3 => {
                let Some(_) = self.time_series_order.get(self.encode_record_cursor) else {
                    let fragment = (result.summaries.annual_energy.len() as u32).to_le_bytes();
                    if self.write_output_fragment(context, &fragment)? {
                        self.encode_section = 4;
                        self.encode_record_cursor = 0;
                        self.encode_sample_cursor = 0;
                        self.encode_fragment_phase = 0;
                    }
                    return Ok(false);
                };
                let Some(series) = result.time_series.series.get_index(self.encode_record_cursor) else { return Err(OutputFault::MissingSeries) };
                let series_len = series.values.len();
                if self.encode_fragment_phase == 0 {
                    if self.write_output_fragment(context, &(series.key.len() as u32).to_le_bytes())? {
                        self.encode_fragment_phase = 1;
                    }
                } else if self.encode_fragment_phase == 1 && self.encode_name_cursor < series.key.len() {
                    let character = series.key[self.encode_name_cursor..].chars().next().ok_or(OutputFault::MissingSeries)?;
                    let mut storage = [0; 4];
                    let fragment = character.encode_utf8(&mut storage).as_bytes();
                    if self.write_output_fragment(context, fragment)? {
                        self.encode_name_cursor += fragment.len();
                    }
                } else if self.encode_fragment_phase <= 2 {
                    let mut fragment = [0; 5];
                    fragment[0] = unit_code(series.unit);
                    fragment[1..5].copy_from_slice(&(series_len as u32).to_le_bytes());
                    if self.write_output_fragment(context, &fragment)? {
                        self.encode_fragment_phase = 3;
                    }
                } else if let Some((&timestamp, &value)) = series.timestamps_hours.get(self.encode_sample_cursor).zip(series.values.get(self.encode_sample_cursor)) {
                    let mut fragment = [0; 16];
                    fragment[..8].copy_from_slice(&timestamp.to_le_bytes());
                    fragment[8..].copy_from_slice(&value.to_le_bytes());
                    if self.write_output_fragment(context, &fragment)? {
                        self.encode_sample_cursor += 1;
                    }
                }
                if self.encode_sample_cursor >= series_len {
                    self.encode_record_cursor += 1;
                    self.encode_sample_cursor = 0;
                    self.encode_name_cursor = 0;
                    self.encode_fragment_phase = 0;
                }
            }
            4 => {
                if let Some(row) = result.summaries.annual_energy.get(self.encode_record_cursor) {
                    if self.write_output_fragment(context, &row.value.to_le_bytes())? {
                        self.encode_record_cursor += 1;
                    }
                } else {
                    self.encode_section = 5;
                }
            }
            _ => return Ok(true),
        }
        Ok(false)
    }

    pub fn run(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Self::fault();
        }
        if context.is_cancelled() {
            return EnergyRun::Cancelled;
        }
        if context.should_yield() {
            return EnergyRun::Yield;
        }
        context.consume_fuel(1);
        if let Some(outcome) = self.drive_wire_publication(context) {
            return outcome;
        }
        match self.stage {
            EnergyJobStage::Validate => {
                let complete = self.step_validation();
                if self.validation.fatal_code != 0 {
                    { return self.begin_fault(); }
                }
                if !complete {
                    return EnergyRun::Yield;
                }
                self.weather_target = weather_copy_target(&self.config);
                self.set_stage(context, EnergyJobStage::ResolveWeather);
                EnergyRun::Yield
            }
            EnergyJobStage::ResolveWeather => {
                if self.weather_cursor < self.weather_target {
                    let record = self.weather_record(self.weather_cursor);
                    if self.weather.insert_stable(self.weather_cursor, record).is_err() {
                        self.weather_fault = Some(WeatherFault::SlotRejected);
                        { return self.begin_fault(); }
                    }
                    self.weather_cursor += 1;
                    if self.weather_cursor.is_multiple_of(256) {
                        return self.begin_preview(context);
                    }
                    return EnergyRun::Yield;
                }
                self.precompute = Some(PrecomputeBuilder::new(self.config.zone_timestep_minutes, self.config.system_timestep_minutes));
                self.set_stage(context, EnergyJobStage::Precompute);
                EnergyRun::Yield
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
                    if builder.backing_rejected() {
                        { return self.begin_fault(); }
                    }
                }
                EnergyRun::Yield
            }
            EnergyJobStage::InitializeZones => {
                if self.initialize_backing_stage == 0 {
                    if self.state.as_mut().expect("state exists while reserving zones").zones.admit(self.model.zones.len()).is_err() {
                        { return self.begin_fault(); }
                    }
                    self.initialize_backing_stage = 1;
                    return EnergyRun::Yield;
                }
                if let Some(zone) = self.model.zones.get(self.initialize_cursor) {
                    let weather = *self.weather.get_index(0).expect("admitted weather record");
                    if self.state.as_mut().expect("state exists while initializing").zones.insert(zone.id, ZoneState::new(INITIAL_TEMPERATURE_C, weather.humidity_ratio())).is_err() {
                        { return self.begin_fault(); }
                    }
                    self.initialize_cursor += 1;
                } else {
                    self.initialize_cursor = 0;
                    self.initialize_backing_stage = 2;
                    self.set_stage(context, EnergyJobStage::InitializeSurfaces);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::InitializeSurfaces => {
                if self.initialize_backing_stage == 2 {
                    if self.state.as_mut().expect("state exists while reserving surfaces").surfaces.admit(self.model.surfaces.len()).is_err() {
                        { return self.begin_fault(); }
                    }
                    // 🧱️ The per-surface energy table is admitted once, here, for exactly one row per
                    // opaque surface and one per fenestration — never grown afterwards.
                    if self.state.as_mut().expect("state exists while reserving surfaces").per_surface.admit(self.model.surfaces.len(), self.model.fenestrations.len()).is_err() {
                        return self.begin_fault();
                    }
                    self.initialize_backing_stage = 3;
                    return EnergyRun::Yield;
                }
                if self.initialize_backing_stage == 3 {
                    if let Some(surface) = self.model.surfaces.get(self.initialize_cursor) {
                        if let Some(nodes) = self.pre.as_ref().and_then(|pre| pre.surfaces.get(&surface.id)).map(|precomputed| precomputed.chain.nodes()) {
                            let mut temperatures_c = Vec::new();
                            if temperatures_c.try_reserve_exact(nodes).is_err() {
                                { return self.begin_fault(); }
                            }
                            temperatures_c.resize(nodes, INITIAL_TEMPERATURE_C);
                            if self.state.as_mut().expect("state exists while initializing").surfaces.insert(surface.id, SurfaceState { temperatures_c, inside_convection_w_m2k: 0.0 }).is_err() {
                                { return self.begin_fault(); }
                            }
                            if self.state.as_mut().expect("state exists while initializing").per_surface.insert_opaque(surface.id).is_err() {
                                return self.begin_fault();
                            }
                        }
                        self.initialize_cursor += 1;
                        return EnergyRun::Yield;
                    }
                    self.initialize_cursor = 0;
                    self.initialize_backing_stage = 4;
                    return EnergyRun::Yield;
                }
                if self.initialize_backing_stage == 4 {
                    if self.state.as_mut().expect("state exists while reserving windows").windows.admit(self.model.fenestrations.len()).is_err() {
                        { return self.begin_fault(); }
                    }
                    self.initialize_backing_stage = 5;
                    return EnergyRun::Yield;
                }
                if self.initialize_backing_stage == 5 {
                    if let Some(fenestration) = self.model.fenestrations.get(self.initialize_cursor) {
                        if self.pre.as_ref().is_some_and(|pre| pre.windows.contains_key(&fenestration.id)) {
                            if self.state.as_mut().expect("state exists while initializing").windows.insert(fenestration.id, WindowState { face_temperatures_c: [INITIAL_TEMPERATURE_C; 2 * model::MAX_PANES], inside_convection_w_m2k: 0.0 }).is_err() {
                                { return self.begin_fault(); }
                            }
                            if self.state.as_mut().expect("state exists while initializing").per_surface.insert_window(fenestration.id).is_err() {
                                return self.begin_fault();
                            }
                        }
                        self.initialize_cursor += 1;
                        return EnergyRun::Yield;
                    }
                    self.initialize_cursor = 0;
                    self.initialize_backing_stage = 6;
                    return EnergyRun::Yield;
                }
                let (faces, nodes) = self.pre.as_ref().map_or((0, 2), |pre| (pre.maximum_enclosure_faces, pre.maximum_nodes));
                if self.state.as_mut().expect("state exists while reserving the solver").solver.reserve(faces, nodes).is_err() {
                    { return self.begin_fault(); }
                }
                self.set_stage(context, EnergyJobStage::InitializeWarmupHistory);
                self.begin_preview(context)
            }
            EnergyJobStage::InitializeWarmupHistory => {
                let zones = self.pre.as_ref().map_or(0, |pre| pre.zone_order.len());
                if self.previous_temperatures.capacity() < zones {
                    if self.previous_temperatures.try_reserve_exact(zones).is_err() {
                        { return self.begin_fault(); }
                    }
                    return EnergyRun::Yield;
                }
                if self.previous_temperatures.len() < zones {
                    self.previous_temperatures.push(f64::NAN);
                    return EnergyRun::Yield;
                }
                if self.previous_loads.capacity() < zones {
                    if self.previous_loads.try_reserve_exact(zones).is_err() {
                        { return self.begin_fault(); }
                    }
                    return EnergyRun::Yield;
                }
                if self.previous_loads.len() < zones {
                    self.previous_loads.push(f64::NAN);
                    return EnergyRun::Yield;
                }
                self.set_stage(context, EnergyJobStage::WarmupTimestep);
                EnergyRun::Yield
            }
            EnergyJobStage::WarmupTimestep => {
                let warmup_hours = self.config.warmup_days.saturating_mul(24);
                if self.warmup_hour >= warmup_hours {
                    if let Some(state) = &mut self.state {
                        state.warmup_complete = true;
                    }
                    self.set_stage(context, EnergyJobStage::StartRun);
                    return EnergyRun::Yield;
                }
                let pre = self.pre.as_ref().expect("precompute complete before warmup");
                if self.timestep_work.is_none() && self.timestep_builder.is_none() {
                    let start = self.run_start_record();
                    let weather = self.hour_weather(start + self.warmup_hour as usize % 24, true);
                    let date = calendar::SimDate::new(weather.current.year, weather.current.month, weather.current.day);
                    self.timestep_builder = Some(TimestepBuilder::new(weather, date, self.warmup_hour as f64));
                    return EnergyRun::Yield;
                }
                if self.timestep_work.is_none() {
                    match self.timestep_builder.as_mut().expect("warmup timestep builder").step(&self.model, pre) {
                        Ok(Some(work)) => {
                            self.timestep_work = Some(work);
                            self.timestep_builder = None;
                        }
                        Ok(None) => return EnergyRun::Yield,
                        Err(_) => return self.begin_fault(),
                    }
                }
                self.timestep_work.as_mut().expect("warmup timestep work exists").step(&self.model, &self.config, pre, self.state.as_mut().expect("state initialized before warmup"));
                // 🧵️ Completion is acknowledged BEFORE the fuel check and the rng rotates ONCE per
                // completed timestep: `step` already charged this call's unit at entry, so a one-unit
                // grant is always exhausted here — checking `should_yield` first starved the job in
                // this stage forever, and rotating per call made the chronology depend on the grant.
                if !self.timestep_work.as_ref().is_some_and(TimestepWork::is_complete) {
                    return EnergyRun::Yield;
                }
                self.rng_state = self.rng_state.rotate_left(17).wrapping_mul(0x94d0_49bb_1331_11eb);
                self.timestep_work = None;
                self.warmup_convergence =
                    Some(WarmupConvergenceWork { stage: WarmupConvergenceStage::TemperatureCheck, cursor: 0, temperature_converged: true, load_converged: true, evaluate: self.warmup_hour > 24 && self.warmup_hour.is_multiple_of(24) });
                self.set_stage(context, EnergyJobStage::WarmupConvergence);
                EnergyRun::Yield
            }
            EnergyJobStage::WarmupConvergence => {
                let pre = self.pre.as_ref().expect("precompute complete during warmup convergence");
                let state = self.state.as_ref().expect("state exists during warmup convergence");
                let work = self.warmup_convergence.as_mut().expect("warmup convergence cursor exists");
                match work.stage {
                    WarmupConvergenceStage::TemperatureCheck => {
                        if !work.evaluate {
                            work.stage = WarmupConvergenceStage::LoadCheck;
                            work.cursor = 0;
                        } else if let Some(zone_id) = pre.zone_order.get(work.cursor) {
                            let current = state.zones.get(zone_id).map_or(f64::NAN, |zone| zone.air.temp_c);
                            work.temperature_converged &= self.previous_temperatures.get(work.cursor).is_some_and(|previous| (current - previous).abs() <= self.config.tolerances.temperature_k);
                            work.cursor += 1;
                        } else {
                            work.stage = WarmupConvergenceStage::LoadCheck;
                            work.cursor = 0;
                        }
                    }
                    WarmupConvergenceStage::LoadCheck => {
                        if !work.evaluate {
                            work.stage = WarmupConvergenceStage::TemperatureHistory;
                            work.cursor = 0;
                        } else if let Some(zone_id) = pre.zone_order.get(work.cursor) {
                            let current = state.zones.get(zone_id).map_or(f64::NAN, |zone| zone.heating_demand_w + zone.cooling_demand_w);
                            work.load_converged &= self.previous_loads.get(work.cursor).is_some_and(|previous| (current - previous).abs() <= self.config.tolerances.energy_w);
                            work.cursor += 1;
                        } else {
                            work.stage = WarmupConvergenceStage::TemperatureHistory;
                            work.cursor = 0;
                        }
                    }
                    WarmupConvergenceStage::TemperatureHistory => {
                        if let Some(zone_id) = pre.zone_order.get(work.cursor) {
                            self.previous_temperatures[work.cursor] = state.zones.get(zone_id).map_or(f64::NAN, |zone| zone.air.temp_c);
                            work.cursor += 1;
                        } else {
                            work.stage = WarmupConvergenceStage::LoadHistory;
                            work.cursor = 0;
                        }
                    }
                    WarmupConvergenceStage::LoadHistory => {
                        if let Some(zone_id) = pre.zone_order.get(work.cursor) {
                            self.previous_loads[work.cursor] = state.zones.get(zone_id).map_or(f64::NAN, |zone| zone.heating_demand_w + zone.cooling_demand_w);
                            work.cursor += 1;
                        } else {
                            work.stage = WarmupConvergenceStage::Complete;
                        }
                    }
                    WarmupConvergenceStage::Complete => {
                        let converged = work.evaluate && work.temperature_converged && work.load_converged;
                        self.warmup_convergence = None;
                        if converged {
                            self.warmup_hour = self.config.warmup_days.saturating_mul(24);
                        } else {
                            self.warmup_hour = self.warmup_hour.saturating_add(1);
                        }
                        self.set_stage(context, EnergyJobStage::WarmupTimestep);
                        if self.warmup_hour.is_multiple_of(24) {
                            return self.begin_preview(context);
                        }
                    }
                }
                EnergyRun::Yield
            }
            EnergyJobStage::StartRun => {
                let zones = self.numerical_census.series;
                let meters = self.numerical_census.meters;
                match self.run_backing_stage {
                    0 => {
                        let period = SimulationKernel::run_period(&self.config);
                        self.total_timesteps = period.total_hours();
                        self.run_hours = Some(period.hours());
                        self.tier = match self.config.environment {
                            SimulationEnvironment::HeatingDesignDay | SimulationEnvironment::CoolingDesignDay | SimulationEnvironment::CustomDesignPeriod => EnergyQualityTier::DesignDay,
                            SimulationEnvironment::WeatherRunPeriod => EnergyQualityTier::CoarseTimestep,
                        };
                    }
                    1 => {
                        if self.time_series.series.admit(zones).is_err() {
                            { return self.begin_fault(); }
                        }
                        self.result_backing.series_slots = self.time_series.series.capacity();
                    }
                    2 => {
                        if self.meters.meters.admit(meters).is_err() {
                            { return self.begin_fault(); }
                        }
                        self.result_backing.meter_slots = self.meters.meters.capacity();
                    }
                    3 => {
                        if self.time_series_order.try_reserve_exact(zones).is_err() {
                            { return self.begin_fault(); }
                        }
                    }
                    4 => {
                        if self.meter_order.try_reserve_exact(meters).is_err() {
                            { return self.begin_fault(); }
                        }
                    }
                    5 => {
                        if self.zone_temperature_history.try_reserve_exact(self.numerical_census.history_values).is_err() {
                            { return self.begin_fault(); }
                        }
                        self.result_backing.history_slots = self.zone_temperature_history.capacity();
                    }
                    6 => {
                        if self.final_summaries.annual_energy.try_reserve_exact(self.numerical_census.summary_rows).is_err() {
                            { return self.begin_fault(); }
                        }
                        self.result_backing.summary_slots = self.final_summaries.annual_energy.capacity();
                    }
                    _ => {
                        self.set_stage(context, EnergyJobStage::RunZoneTimestep);
                        return EnergyRun::Yield;
                    }
                }
                self.run_backing_stage += 1;
                EnergyRun::Yield
            }
            EnergyJobStage::RunZoneTimestep => {
                let pre = self.pre.as_ref().expect("precompute complete before run");
                if self.timestep_work.is_none() && self.timestep_builder.is_none() {
                    let Some((date, hour, _)) = self.run_hours.as_mut().and_then(|hours| hours.next()) else {
                        self.set_stage(context, EnergyJobStage::Finalize);
                        return EnergyRun::Yield;
                    };
                    let record = if self.weather.len() >= 8_760 { (date.day_of_year() as usize - 1) * 24 + hour as usize } else { self.hour_index as usize };
                    let mut weather = self.hour_weather(record, self.hour_index == 0);
                    for record in [&mut weather.previous, &mut weather.current, &mut weather.next] {
                        record.year = date.year;
                        record.month = date.month;
                        record.day = date.day;
                    }
                    weather.current.hour = hour;
                    self.timestep_builder = Some(TimestepBuilder::new(weather, date, self.hour_index as f64));
                    return EnergyRun::Yield;
                }
                if self.timestep_work.is_none() {
                    match self.timestep_builder.as_mut().expect("run timestep builder").step(&self.model, pre) {
                        Ok(Some(work)) => {
                            self.timestep_work = Some(work);
                            self.timestep_builder = None;
                        }
                        Ok(None) => return EnergyRun::Yield,
                        Err(_) => return self.begin_fault(),
                    }
                }
                self.timestep_work.as_mut().expect("run timestep work exists").step(&self.model, &self.config, pre, self.state.as_mut().expect("state initialized before run"));
                // 🧵️ Same order as the warmup stage: acknowledge completion first, rotate once per timestep.
                if !self.timestep_work.as_ref().is_some_and(TimestepWork::is_complete) {
                    return EnergyRun::Yield;
                }
                self.rng_state = self.rng_state.rotate_left(17).wrapping_mul(0x94d0_49bb_1331_11eb);
                self.timestep_work = None;
                self.aggregate_zone_cursor = 0;
                self.set_stage(context, EnergyJobStage::AggregateZone);
                EnergyRun::Yield
            }
            EnergyJobStage::AggregateZone => {
                if self.aggregate_zone_cursor < self.model.zones.len() {
                    let complete = self.step_aggregate_zone();
                    if self.aggregate_zone_work.as_ref().is_some_and(|work| work.backing_rejected) {
                        { return self.begin_fault(); }
                    }
                    if complete {
                        self.aggregate_zone_cursor += 1;
                    }
                } else {
                    self.set_stage(context, EnergyJobStage::AggregateFacility);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::AggregateFacility => {
                let complete = self.step_aggregate_facility();
                if self.aggregate_facility_work.as_ref().is_some_and(|work| work.backing_rejected) {
                    { return self.begin_fault(); }
                }
                if complete {
                    self.hour_index = self.hour_index.saturating_add(1);
                    self.checkpoint_due = self.hour_index.is_multiple_of(24);
                    self.set_stage(context, EnergyJobStage::PublishTimestep);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::PublishTimestep => {
                if self.checkpoint_due {
                    self.checkpoint_due = false;
                    if self.start_wire(EnergyWireKind::Checkpoint, self.hour_index as u64, JobPayloadStream::CheckpointState).is_err() {
                        return EnergyRun::Yield;
                    }
                    return EnergyRun::Yield;
                }
                self.set_stage(context, EnergyJobStage::RunZoneTimestep);
                self.begin_preview(context)
            }
            EnergyJobStage::Finalize => {
                self.tier = EnergyQualityTier::Final;
                self.sizing_builder = Some(SizingBuilder::new(SizingConfig::default()));
                self.finalization = FinalizationWork::default();
                self.set_stage(context, EnergyJobStage::Size);
                EnergyRun::Yield
            }
            EnergyJobStage::Size => {
                let builder = self.sizing_builder.as_mut().expect("sizing builder exists");
                if builder.fault().is_some() {
                    { return self.begin_fault(); }
                }
                if builder.is_complete(&self.model) {
                    self.final_sizing = self.sizing_builder.take().map(SizingBuilder::finish);
                    self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                } else {
                    builder.step(&self.model);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::FinalizeSummaries => {
                let complete = self.step_finalization();
                if self.finalization.row_backing_rejected {
                    { return self.begin_fault(); }
                }
                if complete {
                    self.set_stage(context, EnergyJobStage::BuildResults);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::FinalizeMetrics => {
                self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                EnergyRun::Yield
            }
            EnergyJobStage::FinalizeEconomics => {
                self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                EnergyRun::Yield
            }
            EnergyJobStage::BuildResults => match self.step_result_build() {
                Ok(true) => {
                    self.set_stage(context, EnergyJobStage::PublishFinal);
                    self.begin_preview(context)
                }
                Ok(false) => EnergyRun::Yield,
                Err(_) => self.begin_fault(),
            },
            EnergyJobStage::PublishFinal => {
                self.set_stage(context, EnergyJobStage::EncodeOutput);
                EnergyRun::Yield
            }
            EnergyJobStage::EncodeOutput => {
                if self.commit_reservation.is_none() {
                    if self.commit_census.is_none() {
                        self.commit_census = Some(CommitCensusWork::default());
                        return EnergyRun::Yield;
                    }
                    match self.step_commit_census() {
                        Ok(false) => return EnergyRun::Yield,
                        Ok(true) => {}
                        Err(fault) => {
                            self.output_fault = Some(fault);
                            return self.begin_fault();
                        }
                    }
                }
                if self.output_writer.is_none() && self.output_payload.is_none() {
                    self.output_writer = Some(semio_framework_job::RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput));
                    return EnergyRun::Yield;
                }
                match self.encode_output_step(context) {
                    Ok(false) => return EnergyRun::Yield,
                    Ok(true) => {}
                    Err(fault) => {
                        self.output_fault = Some(fault);
                        return self.begin_fault();
                    }
                }
                if let Some(writer) = self.output_writer.as_mut() {
                    if writer.staged_page_len().is_some() {
                        if writer.commit_staged_page().is_err() {
                            self.output_fault = Some(OutputFault::BackingRejected);
                            return self.begin_fault();
                        }
                        return EnergyRun::Yield;
                    }
                    let writer = self.output_writer.take().expect("output writer exists");
                    match writer.finish() {
                        Ok(payload) => {
                            let reservation = self.commit_reservation.as_ref().expect("commit reservation remains mounted");
                            if payload.page_count() != reservation.pages || payload.len() != reservation.bytes || self.commit_pages_mounted != reservation.pages || self.commit_items_encoded != reservation.items {
                                self.output_payload = Some(payload);
                                self.output_fault = Some(OutputFault::BackingRejected);
                                return self.begin_fault();
                            }
                            self.output_payload = Some(payload);
                        }
                        Err(writer) => {
                            self.output_writer = Some(writer);
                            return EnergyRun::Yield;
                        }
                    }
                    let payload = self.output_payload.take().expect("prepared commit payload exists");
                    let packet = EnergyWirePacket { kind: EnergyWireKind::Commit, identity: self.wire_identity(0), payload, preview: None, reservation: self.commit_reservation.take() };
                    match self.publication.commits.push_reserved(packet) {
                        Ok(()) => {
                            self.commit_pages_mounted = 0;
                            self.commit_items_encoded = 0;
                            self.set_stage(context, EnergyJobStage::Complete);
                        }
                        Err(packet) => {
                            self.output_payload = Some(packet.payload);
                            self.commit_reservation = packet.reservation;
                            return EnergyRun::Yield;
                        }
                    }
                }
                EnergyRun::Yield
            }
            EnergyJobStage::Complete => {
                if std::mem::take(&mut self.commit_acknowledged) {
                    return EnergyRun::Complete;
                }
                let Some(packet) = self.publication.commits.take_terminal() else { return EnergyRun::Yield };
                self.commit_output = Some(packet);
                EnergyRun::Complete
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    /// 🏷️ Resolves the outcome markers or the leased commit packet into the immutable descriptor's payload pair.
    fn commit_payloads(&self) -> (&RetainedJobPayload, &RetainedJobPayload) {
        (&self.markers.commit_state, self.commit_output.as_ref().map_or(&self.markers.commit_output, |packet| &packet.payload))
    }

    /// 📏️ Quotes the next close frontier: a payload owner, a staged page source, or one popped ladder rung.
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if authority_is_terminal_empty(self) {
            return Ok(RetirementDemand::default());
        }
        if let Some(build) = self.wire_build.as_ref() {
            return build.writer.retirement_demands();
        }
        if let Some(writer) = self.output_writer.as_ref() {
            return writer.retirement_demands();
        }
        if let Some(payload) = self.output_payload.as_ref() {
            return payload.retirement_demands();
        }
        for packet in [self.wire_ready.as_ref(), self.restore_input.as_ref(), self.publication.preview.as_ref(), self.publication.retiring_preview.as_ref(), self.commit_output.as_ref()].into_iter().flatten() {
            return packet.retirement_demands();
        }
        for queue in [&self.publication.checkpoints, &self.publication.commits, &self.publication.faults] {
            if let Some(demand) = queue.retirement_demands() {
                return demand;
            }
        }
        for reservation in [self.commit_census.as_ref().and_then(|work| work.reservation.as_ref()), self.commit_reservation.as_ref()].into_iter().flatten() {
            if let Some(demand) = reservation.close_source_demand() {
                return Ok(demand);
            }
        }
        Ok(ENERGY_RUNG_DEMAND)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        self.closing = true;
        if authority_is_terminal_empty(self) {
            return Ok(energy_done());
        }
        if let Some(build) = self.wire_build.as_mut() {
            build.writer.begin_close();
            let (step, done) = energy_turn(energy_guarded(grant, build.writer.retirement_demands(), || build.writer.close_step(grant)))?;
            if done {
                self.wire_build = None;
            }
            return Ok(step);
        }
        if let Some(writer) = self.output_writer.as_mut() {
            writer.begin_close();
            let (step, done) = energy_turn(energy_guarded(grant, writer.retirement_demands(), || writer.close_step(grant)))?;
            if done {
                self.output_writer = None;
            }
            return Ok(step);
        }
        if let Some(payload) = self.output_payload.as_mut() {
            let (step, done) = energy_turn(energy_guarded(grant, payload.retirement_demands(), || payload.close_step(grant)))?;
            if done {
                self.output_payload = None;
            }
            return Ok(step);
        }
        macro_rules! close_packet {
            ($packet:expr) => {
                if let Some(packet) = $packet.as_mut() {
                    let (step, done) = energy_turn(packet.close_step(grant))?;
                    if done {
                        $packet = None;
                    }
                    return Ok(step);
                }
            };
        }
        close_packet!(self.wire_ready);
        close_packet!(self.restore_input);
        close_packet!(self.publication.preview);
        close_packet!(self.publication.retiring_preview);
        close_packet!(self.commit_output);
        for queue in [&mut self.publication.checkpoints, &mut self.publication.commits, &mut self.publication.faults] {
            if let Some(step) = queue.close_step(grant)? {
                return Ok(step);
            }
        }
        macro_rules! close_sources {
            ($reservation:expr) => {
                if let Some(demand) = $reservation.as_ref().and_then(EnergyCommitReservation::close_source_demand) {
                    if !energy_grant_covers(grant, demand) {
                        return Ok(energy_idle());
                    }
                    let released_bytes = $reservation.as_mut().and_then(EnergyCommitReservation::close_source_step).unwrap_or_default();
                    return Ok(energy_rung((1, released_bytes)));
                }
            };
        }
        if let Some(work) = self.commit_census.as_mut() {
            close_sources!(work.reservation);
            self.commit_census = None;
            return Ok(energy_one());
        }
        if self.commit_reservation.is_some() {
            close_sources!(self.commit_reservation);
            self.commit_reservation = None;
            return Ok(energy_one());
        }
        if !energy_grant_covers(grant, ENERGY_RUNG_DEMAND) {
            return Ok(energy_idle());
        }
        let maximum_bytes = grant.maximum_release_bytes;
        let maximum_items = grant.maximum_items;
        if std::mem::take(&mut self.commit_pages_mounted) != 0 || std::mem::take(&mut self.commit_items_encoded) != 0 {
            return Ok(energy_one());
        }
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.close_string_owner, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        macro_rules! pop_owner {
            ($owners:expr) => {
                if $owners.pop().is_some() {
                    return Ok(energy_one());
                }
            };
        }
        pop_owner!(self.weather);
        if let Some((released_items, released_bytes)) = close_string_vector_step(&mut self.time_series_order, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_string_vector_step(&mut self.meter_order, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_time_series_step(&mut self.time_series, &mut self.close_string_owner, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_meter_table_step(&mut self.meters, &mut self.close_string_owner, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        pop_owner!(self.zone_temperature_history);
        pop_owner!(self.previous_temperatures);
        pop_owner!(self.previous_loads);
        if let Some(work) = self.aggregate_zone_work.as_mut() {
            if let Some((released_items, released_bytes)) = close_aggregate_work_step(work, maximum_bytes) {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.aggregate_zone_work = None;
            return Ok(energy_one());
        }
        if let Some(work) = self.aggregate_facility_work.as_mut() {
            if let Some((released_items, released_bytes)) = close_aggregate_work_step(work, maximum_bytes) {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.aggregate_facility_work = None;
            return Ok(energy_one());
        }
        macro_rules! remove_validation_entry {
            ($owners:expr) => {{
                if $owners.pop().is_some() {
                    return Ok(energy_one());
                }
            }};
        }
        remove_validation_entry!(self.validation.zone_ids);
        remove_validation_entry!(self.validation.material_ids);
        remove_validation_entry!(self.validation.construction_ids);
        remove_validation_entry!(self.validation.surface_ids);
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.finalization.row_unit, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.finalization.row_key, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if !self.result_build.model_name.is_empty() || !self.result_build.model_version.is_empty() || !self.result_build.weather_location.is_empty() {
            if maximum_bytes == 0 {
                return Ok(energy_idle());
            }
            let released_bytes = self.result_build.weather_location.pop().or_else(|| self.result_build.model_version.pop()).or_else(|| self.result_build.model_name.pop()).map_or(0, |character| character.len_utf8());
            return Ok(energy_rung((0, released_bytes)));
        }
        if let Some(builder) = self.timestep_builder.as_mut() {
            if !builder.close_step(maximum_items) {
                return Ok(energy_one());
            }
            self.timestep_builder = None;
            return Ok(energy_one());
        }
        if let Some(work) = self.timestep_work.as_mut() {
            if !work.close_step(maximum_items) {
                return Ok(energy_one());
            }
            self.timestep_work = None;
            return Ok(energy_one());
        }
        if let Some(builder) = self.precompute.as_mut() {
            if !builder.close_step(maximum_items) {
                return Ok(energy_one());
            }
            self.precompute = None;
            return Ok(energy_one());
        }
        if let Some(pre) = self.pre.as_mut() {
            if !pre.close_step(maximum_items) {
                return Ok(energy_one());
            }
            self.pre = None;
            return Ok(energy_one());
        }
        if let Some(state) = self.state.as_mut() {
            if state.zones.pop().is_some() {
                return Ok(energy_one());
            }
            if state.surfaces.pop().is_some() {
                return Ok(energy_one());
            }
            if state.windows.pop().is_some() {
                return Ok(energy_one());
            }
            if state.per_surface.pop() {
                return Ok(energy_one());
            }
            self.state = None;
            return Ok(energy_one());
        }
        if let Some(results) = self.result.as_mut() {
            if let Some((released_items, released_bytes)) = close_results_step(results, &mut self.close_string_owner, maximum_bytes) {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.result = None;
            return Ok(energy_one());
        }
        if let Some(sizing) = self.final_sizing.as_mut() {
            if let Some((released_items, released_bytes)) = close_sizing_step(sizing, maximum_bytes) {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.final_sizing = None;
            return Ok(energy_one());
        }
        if let Some((released_items, released_bytes)) = close_summaries_step(&mut self.final_summaries, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some(builder) = self.sizing_builder.as_mut() {
            let (complete, released_items, released_bytes) = builder.close_step(maximum_bytes);
            if !complete {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.sizing_builder = None;
            return Ok(energy_one());
        }
        if let Some((released_items, released_bytes)) = close_model_step(&mut self.model, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_config_step(&mut self.config, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if self.run_hours.take().is_some() || self.final_environmental.take().is_some() || self.final_resilience.take().is_some() || self.output_fault.take().is_some() || self.weather_fault.take().is_some() {
            return Ok(energy_one());
        }
        Ok(energy_done())
    }

    fn terminal_is_empty(&self) -> bool {
        authority_is_terminal_empty(self)
    }
}

impl InteractiveJob for EnergyJob {
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        let authority = &mut **self;
        let run = match authority.pending_outcome.take() {
            Some(run) => run,
            None => authority.run(context),
        };
        let admitted = match run {
            EnergyRun::Yield => return Ok(None),
            EnergyRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context)?,
            EnergyRun::Preview => JobOutcomeBorrow::admit_preview(context, &authority.markers.preview)?,
            EnergyRun::Checkpoint(applied_progress) => JobOutcomeBorrow::admit_checkpoint(context, &authority.markers.checkpoint, applied_progress)?,
            EnergyRun::Fault => JobOutcomeBorrow::admit_fault(context, &authority.markers.fault)?,
            EnergyRun::Complete => {
                let output = authority.commit_output.as_ref().map_or(&authority.markers.commit_output, |packet| &packet.payload);
                JobOutcomeBorrow::admit_complete(context, Some(&authority.markers.commit_state), Some(output))?
            }
        };
        if admitted.is_none() {
            authority.pending_outcome = Some(run);
        }
        Ok(admitted)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::PreviewReady => descriptor.preview(&self.markers.preview),
            JobOutcomeKind::CheckpointReady { .. } => descriptor.checkpoint(&self.markers.checkpoint),
            JobOutcomeKind::Complete => {
                let (state, output) = self.commit_payloads();
                descriptor.complete(Some(state), Some(output))
            }
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Fault => descriptor.fault(&self.markers.fault),
        }
    }

    fn begin_close(&mut self) {
        EnergyJobAuthority::begin_close(self);
    }

    fn close_step(&mut self, context: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        Ok(EnergyJobAuthority::close_step(self, context.retained_grant())?)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        EnergyJobAuthority::terminal_is_empty(self)
    }
}

fn advance_validation(work: &mut ValidationWork, stage: ValidationStage) {
    work.stage = stage;
    work.cursor = 0;
    work.child_cursor = 0;
}

fn advance_finalization(work: &mut FinalizationWork, stage: FinalizationStage) {
    work.stage = stage;
    work.cursor = 0;
    work.row_cursor = 0;
    work.row_part = 0;
    work.row_reserved = false;
}

fn build_summary_row_step(work: &mut FinalizationWork, summaries: &mut SummaryTables, maximum_slots: usize, key: &str, value: f64, unit: &str) -> bool {
    let (owner, source) = if work.row_part == 0 { (&mut work.row_key, key) } else { (&mut work.row_unit, unit) };
    if !work.row_reserved {
        if owner.try_reserve_exact(source.len()).is_err() {
            work.row_backing_rejected = true;
            return false;
        }
        work.row_reserved = true;
        return false;
    }
    if let Some(character) = source.get(work.row_cursor..).and_then(|remaining| remaining.chars().next()) {
        owner.push(character);
        work.row_cursor += character.len_utf8();
        return false;
    }
    if work.row_part == 0 {
        work.row_part = 1;
        work.row_cursor = 0;
        work.row_reserved = false;
        return false;
    }
    if summaries.annual_energy.len() >= maximum_slots {
        work.row_backing_rejected = true;
        return false;
    }
    summaries.annual_energy.push(SummaryRow { key: std::mem::take(&mut work.row_key), value, unit: std::mem::take(&mut work.row_unit) });
    true
}

fn advance_result_build(work: &mut ResultBuildWork, stage: ResultBuildStage) {
    work.stage = stage;
    work.cursor = 0;
    work.reserved = false;
}

fn copy_string_step(output: &mut String, cursor: &mut usize, reserved: &mut bool, source: &str) -> Result<bool, Error> {
    if !*reserved {
        output.try_reserve_exact(source.len()).map_err(|_| Error::severe("energy metadata backing rejected"))?;
        *reserved = true;
        return Ok(false);
    }
    let Some(character) = source.get(*cursor..).and_then(|remaining| remaining.chars().next()) else {
        return Ok(true);
    };
    output.push(character);
    *cursor += character.len_utf8();
    Ok(false)
}

fn copy_aggregate_names_step(work: &mut AggregateZoneWork, parts: [&str; 3]) -> bool {
    if work.copy_index >= work.names.len() {
        return true;
    }
    let length = parts.iter().map(|part| part.len()).sum();
    if !work.reserved[work.copy_index] {
        if work.names[work.copy_index].try_reserve_exact(length).is_err() {
            work.backing_rejected = true;
            return false;
        }
        work.reserved[work.copy_index] = true;
        return false;
    }
    let mut offset = work.cursor;
    for part in parts {
        if offset < part.len() {
            if let Some(character) = part.get(offset..).and_then(|remaining| remaining.chars().next()) {
                work.names[work.copy_index].push(character);
                work.cursor += character.len_utf8();
                return false;
            }
        }
        offset = offset.saturating_sub(part.len());
    }
    work.copy_index += 1;
    work.cursor = 0;
    work.copy_index >= work.names.len()
}

fn insert_admitted_meter(table: &mut MeterTable, order: &mut Vec<String>, work: &mut AggregateZoneWork, maximum_slots: usize, fuel: FuelType, end_use: EndUse) {
    if table.meters.len() >= maximum_slots {
        work.backing_rejected = true;
        return;
    }
    let [map_key, meter_name, order_key] = std::mem::replace(&mut work.names, std::array::from_fn(|_| String::new()));
    if table.meters.insert_stable(map_key, Meter { name: meter_name, fuel, end_use, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 }).is_err() {
        work.backing_rejected = true;
        return;
    }
    order.push(order_key);
}

fn close_aggregate_work_step(work: &mut AggregateZoneWork, maximum_bytes: usize) -> Option<(usize, usize)> {
    if work.timestamps.pop().is_some() || work.values.pop().is_some() {
        return Some((1, 0));
    }
    for name in &mut work.names {
        if let Some(released) = close_string_step(name, maximum_bytes) {
            return Some(released);
        }
    }
    None
}

fn authority_is_terminal_empty(authority: &EnergyJobAuthority) -> bool {
    authority.closing
        && authority.output_writer.is_none()
        && authority.output_payload.is_none()
        && authority.commit_census.is_none()
        && authority.commit_reservation.is_none()
        && authority.commit_pages_mounted == 0
        && authority.commit_items_encoded == 0
        && authority.wire_build.is_none()
        && authority.wire_ready.is_none()
        && authority.restore_input.is_none()
        && authority.commit_output.is_none()
        && authority.publication.preview.is_none()
        && authority.publication.retiring_preview.is_none()
        && authority.publication.checkpoints.len == 0
        && !authority.publication.checkpoints.reserved_push
        && authority.publication.commits.len == 0
        && !authority.publication.commits.reserved_push
        && authority.publication.faults.len == 0
        && !authority.publication.faults.reserved_push
        && authority.close_string_owner.is_empty()
        && authority.weather.is_empty()
        && authority.time_series_order.is_empty()
        && authority.meter_order.is_empty()
        && authority.time_series.series.is_empty()
        && authority.meters.meters.is_empty()
        && authority.zone_temperature_history.is_empty()
        && authority.previous_temperatures.is_empty()
        && authority.previous_loads.is_empty()
        && authority.aggregate_zone_work.is_none()
        && authority.aggregate_facility_work.is_none()
        && authority.validation.zone_ids.is_empty()
        && authority.validation.material_ids.is_empty()
        && authority.validation.construction_ids.is_empty()
        && authority.validation.surface_ids.is_empty()
        && authority.finalization.row_key.is_empty()
        && authority.finalization.row_unit.is_empty()
        && authority.result_build.model_name.is_empty()
        && authority.result_build.model_version.is_empty()
        && authority.result_build.weather_location.is_empty()
        && authority.precompute.is_none()
        && authority.pre.is_none()
        && authority.state.is_none()
        && authority.timestep_builder.is_none()
        && authority.run_hours.is_none()
        && authority.timestep_work.is_none()
        && authority.result.is_none()
        && authority.sizing_builder.is_none()
        && authority.final_sizing.is_none()
        && summaries_are_terminal_empty(&authority.final_summaries)
        && authority.final_environmental.is_none()
        && authority.final_resilience.is_none()
        && authority.output_fault.is_none()
        && authority.weather_fault.is_none()
        && model_is_terminal_empty(&authority.model)
        && config_is_terminal_empty(&authority.config)
}

/// ⚡️ Headless BEM simulation engine.
pub struct Engine;

impl Engine {
    /// ⚡️ Create the persistent operation used by interactive hosts and the batch adapter.
    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn job(model: Model, config: SimulationConfig) -> Result<EnergyJob, EnergyAdmissionRejected> {
        EnergyJob::new(Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 0), model, config)
    }

    /// ⚡️ Run full building energy simulation.
    pub fn run(model: Model, config: SimulationConfig) -> Result<Results, Error> {
        enum Seen {
            Yield,
            Preview,
            Checkpoint,
            Complete,
            Fault,
            Cancelled,
        }
        let mut job = Self::job(model, config).map_err(|_| Error::severe("energy numerical admission rejected"))?;
        let operation = job.operation.operation;
        let generation = job.operation.generation;
        let mut sequence = 0;
        let seen = loop {
            let mut receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation, generation, StepBudget::new(32, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut receipt);
            let seen = match InteractiveJob::step(&mut job, &mut context) {
                Ok(Some(JobOutcomeBorrow::PreviewReady { .. })) => Seen::Preview,
                Ok(Some(JobOutcomeBorrow::CheckpointReady { .. })) => Seen::Checkpoint,
                Ok(Some(JobOutcomeBorrow::Complete { .. })) => Seen::Complete,
                Ok(Some(JobOutcomeBorrow::Fault { .. })) | Err(_) => Seen::Fault,
                Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => Seen::Cancelled,
                Ok(Some(JobOutcomeBorrow::Yield { .. }) | None) => Seen::Yield,
            };
            match seen {
                Seen::Preview => {
                    if let Some(mut packet) = job.take_preview_packet(generation).map_err(|_| Error::severe("energy preview generation rejected"))? {
                        close_retained_packet(&mut packet);
                    }
                }
                Seen::Checkpoint => {
                    if let Some(mut lease) = job.take_checkpoint_packet(generation).map_err(|_| Error::severe("energy checkpoint generation rejected"))? {
                        close_retained_packet(lease.packet_mut());
                        job.ack_checkpoint_packet(lease).map_err(|_| Error::severe("energy checkpoint ACK rejected"))?;
                    }
                }
                Seen::Yield => {}
                terminal => break terminal,
            }
        };
        let result = match seen {
            Seen::Complete => job.take_results().ok_or_else(|| Error::severe("energy job completed without results")),
            Seen::Fault => Err(Error::severe("energy simulation fault")),
            Seen::Cancelled => Err(Error::severe("energy simulation cancelled")),
            Seen::Yield | Seen::Preview | Seen::Checkpoint => Err(Error::severe("energy batch adapter stopped before a terminal outcome")),
        };
        close_owned_job(&mut job);
        result
    }
}

/// 🎟️ The bounded five-currency authority the batch adapter supplies to every admitted step.
pub const ENERGY_BATCH_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🚪️ Runs the job's own close ladder to completion with grants it quotes itself.
pub fn close_owned_job(job: &mut EnergyJob) {
    InteractiveJob::begin_close(job);
    let mut idle = 0usize;
    loop {
        if InteractiveJob::terminal_is_empty(job) {
            return;
        }
        let grant = energy_self_funded(job.close_demands().expect("a locally owned energy job quotes its close demand"));
        let step = EnergyJobAuthority::close_step(job, grant).expect("a locally owned energy job close is never refused");
        assert!(step.progress().fits(grant), "an energy close receipt must fit its own quoted grant");
        assert!(!matches!(step, InteractiveJobCloseStep::Blocked), "a locally owned energy job close has no external owner");
        idle = if step.progress() == RetainedCloneProgress::default() && !matches!(step, InteractiveJobCloseStep::Complete { .. }) { idle + 1 } else { 0 };
        assert!(idle < 1_000, "the energy close ladder stalled");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
