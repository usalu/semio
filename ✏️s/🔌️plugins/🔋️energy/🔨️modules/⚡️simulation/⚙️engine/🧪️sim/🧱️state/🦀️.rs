//! 🧱️ Job state: stage enums, work cursors, the admission rejection, the job authority, the abandonment registry and the job owner.

use super::*;

/// 🌡️ Temperature every zone and construction node starts from before warmup.
pub(super) const INITIAL_TEMPERATURE_C: f64 = 23.0;

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
pub(super) struct EnergyOutcomeMarkers {
    pub(super) preview: RetainedJobPayload,
    pub(super) checkpoint: RetainedJobPayload,
    pub(super) fault: RetainedJobPayload,
    pub(super) commit_state: RetainedJobPayload,
    pub(super) commit_output: RetainedJobPayload,
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
pub(super) enum WarmupConvergenceStage {
    TemperatureCheck,
    LoadCheck,
    TemperatureHistory,
    LoadHistory,
    Complete,
}

#[cfg(any(test, feature = "testkit"))]
pub const P7C1_WARMUP_CONVERGENCE_STAGES: [WarmupConvergenceStage; 5] =
    [WarmupConvergenceStage::TemperatureCheck, WarmupConvergenceStage::LoadCheck, WarmupConvergenceStage::TemperatureHistory, WarmupConvergenceStage::LoadHistory, WarmupConvergenceStage::Complete];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct WarmupConvergenceWork {
    pub stage: WarmupConvergenceStage,
    pub(super) cursor: usize,
    pub(super) temperature_converged: bool,
    pub(super) load_converged: bool,
    pub(super) evaluate: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) enum ValidationStage {
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
    CheckPvThermal,
    Complete,
}

#[cfg(any(test, feature = "testkit"))]
pub const P7C1_VALIDATION_STAGES: [ValidationStage; 17] = [
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
    ValidationStage::CheckPvThermal,
    ValidationStage::Complete,
];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct ValidationWork {
    pub stage: ValidationStage,
    pub(super) cursor: usize,
    pub(super) child_cursor: usize,
    pub(super) zone_reference_family: u8,
    pub(super) zone_ids: FixedTable<model::EntityId, ()>,
    pub(super) material_ids: FixedTable<model::EntityId, ()>,
    pub(super) construction_ids: FixedTable<model::EntityId, ()>,
    pub(super) surface_ids: FixedTable<model::EntityId, ()>,
    pub(super) fatal_code: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) enum FinalizationStage {
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

#[cfg(any(test, feature = "testkit"))]
pub const P7C1_FINALIZATION_STAGES: [FinalizationStage; 13] = [
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
pub(super) struct FinalizationWork {
    pub stage: FinalizationStage,
    pub(super) cursor: usize,
    pub(super) electricity_kwh: f64,
    pub(super) gas_kwh: f64,
    pub(super) floor_area_m2: f64,
    pub(super) annual_cost: f64,
    pub(super) present_value_energy: f64,
    pub(super) resilience: ResilienceMetrics,
    pub(super) row_key: String,
    pub(super) row_unit: String,
    pub(super) row_cursor: usize,
    pub(super) row_part: u8,
    pub(super) row_reserved: bool,
    pub(super) row_backing_rejected: bool,
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
pub(super) enum ResultBuildStage {
    ModelName,
    ModelVersion,
    WeatherLocation,
    Assemble,
    Complete,
}

#[cfg(any(test, feature = "testkit"))]
pub const P7C1_RESULT_BUILD_STAGES: [ResultBuildStage; 5] = [ResultBuildStage::ModelName, ResultBuildStage::ModelVersion, ResultBuildStage::WeatherLocation, ResultBuildStage::Assemble, ResultBuildStage::Complete];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct ResultBuildWork {
    pub stage: ResultBuildStage,
    pub(super) cursor: usize,
    pub(super) reserved: bool,
    pub(super) model_name: String,
    pub(super) model_version: String,
    pub(super) weather_location: String,
}

impl Default for ResultBuildWork {
    fn default() -> Self {
        Self { stage: ResultBuildStage::ModelName, cursor: 0, reserved: false, model_name: String::new(), model_version: String::new(), weather_location: String::new() }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) enum AggregateZoneStage {
    Temperature,
    Heating,
    Cooling,
    Fan,
    Complete,
}

#[cfg(any(test, feature = "testkit"))]
pub const P7C1_AGGREGATE_STAGES: [AggregateZoneStage; 5] = [AggregateZoneStage::Temperature, AggregateZoneStage::Heating, AggregateZoneStage::Cooling, AggregateZoneStage::Fan, AggregateZoneStage::Complete];

#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) struct AggregateZoneWork {
    pub stage: AggregateZoneStage,
    pub(super) phase: u8,
    pub(super) copy_index: usize,
    pub(super) cursor: usize,
    pub(super) reserved: [bool; 3],
    pub(super) names: [String; 3],
    pub(super) timestamps: Vec<f64>,
    pub(super) values: Vec<f64>,
    pub(super) backing_rejected: bool,
    pub(super) series_fault: Option<TimeSeriesAppendError>,
}

impl AggregateZoneWork {
    pub(super) fn new() -> Self {
        Self { stage: AggregateZoneStage::Temperature, phase: 0, copy_index: 0, cursor: 0, reserved: [false; 3], names: std::array::from_fn(|_| String::new()), timestamps: Vec::new(), values: Vec::new(), backing_rejected: false, series_fault: None }
    }

    pub(super) fn advance(&mut self, stage: AggregateZoneStage) {
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
    pub(super) samples_per_series: usize,
    pub history_slots: usize,
    pub(super) summary_slots: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub(super) enum OutputFault {
    MissingResult,
    BackingRejected,
    MissingMeter,
    MissingSeries,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CommitCensusStage {
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
pub(super) struct CommitCensusWork {
    pub(super) stage: CommitCensusStage,
    pub(super) record: usize,
    pub(super) sample: usize,
    pub(super) byte: usize,
    pub(super) channel_slot: usize,
    pub(super) output_bytes: usize,
    pub(super) output_items: usize,
    pub(super) resident_pages: usize,
    pub(super) resident_bytes: usize,
    pub(super) resident_items: usize,
    pub(super) reservation: Option<EnergyCommitReservation>,
}

impl Default for CommitCensusWork {
    fn default() -> Self {
        Self { stage: CommitCensusStage::Base, record: 0, sample: 0, byte: 0, channel_slot: 0, output_bytes: 0, output_items: 0, resident_pages: 0, resident_bytes: 0, resident_items: 0, reservation: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum WeatherFault {
    SlotRejected,
    ChronologyRejected,
    RunHourMissing,
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
    pub input_identity: EnergyInputIdentity,
    pub operation: Operation,
    pub model: Model,
    pub(super) config: SimulationConfig,
    pub(super) numerical_census: EnergyNumericalCensus,
    pub input_identity_bytes: usize,
    pub input_identity_ready: bool,
    pub stage: EnergyJobStage,
    pub tier: EnergyQualityTier,
    pub weather: FixedTable<usize, WeatherRecord>,
    pub weather_cursor: usize,
    pub weather_target: usize,
    pub precompute: Option<PrecomputeBuilder>,
    pub precompute_close: semio_framework_value::retained_clone::RetainedCloneClose,
    pub(super) validation: ValidationWork,
    pub(super) pre: Option<PrecomputedModel>,
    pub(super) state: Option<SimulationModel>,
    pub(super) initialize_cursor: usize,
    pub initialize_backing_stage: u8,
    pub warmup_hour: u32,
    pub(super) previous_temperatures: Vec<f64>,
    pub(super) previous_loads: Vec<f64>,
    pub warmup_convergence: Option<WarmupConvergenceWork>,
    pub(super) run_hours: Option<calendar::RunPeriodHours>,
    pub total_timesteps: u32,
    pub run_backing_stage: u8,
    pub hour_index: u32,
    pub aggregate_zone_cursor: usize,
    pub aggregate_zone_work: Option<AggregateZoneWork>,
    pub aggregate_facility_cursor: u8,
    pub aggregate_facility_work: Option<AggregateZoneWork>,
    pub timestep_work: Option<TimestepWork>,
    pub timestep_builder: Option<TimestepBuilder>,
    pub rng_state: u64,
    pub checkpoint_due: bool,
    pub result_backing: EnergyAdmittedResultBacking,
    pub time_series: TimeSeriesTable,
    pub meters: MeterTable,
    pub(super) time_series_order: Vec<String>,
    pub(super) meter_order: Vec<String>,
    pub zone_temperature_history: Vec<f64>,
    pub(super) facility_electricity_j: f64,
    pub publication: EnergyPublicationChannels,
    pub restore_input: Option<EnergyWirePacket>,
    pub wire_build: Option<EnergyWireBuild>,
    pub wire_ready: Option<EnergyWirePacket>,
    pub(super) result: Option<Results>,
    pub sizing_builder: Option<SizingBuilder>,
    pub(super) final_sizing: Option<SizingTables>,
    pub(super) final_summaries: SummaryTables,
    pub(super) final_environmental: Option<EnvironmentalMetrics>,
    pub(super) final_resilience: Option<ResilienceMetrics>,
    pub finalization: FinalizationWork,
    pub result_build: ResultBuildWork,
    pub(super) output_writer: Option<semio_framework_job_payload::RetainedJobPayloadWriter>,
    pub(super) output_payload: Option<RetainedJobPayload>,
    pub(super) commit_census: Option<CommitCensusWork>,
    pub(super) commit_reservation: Option<EnergyCommitReservation>,
    pub(super) commit_acknowledged: bool,
    pub commit_output: Option<EnergyWirePacket>,
    pub(super) markers: EnergyOutcomeMarkers,
    pub(super) pending_outcome: Option<EnergyRun>,
    pub(super) commit_pages_mounted: usize,
    pub(super) commit_items_encoded: usize,
    pub encode_section: u8,
    pub(super) encode_record_cursor: usize,
    pub(super) encode_sample_cursor: usize,
    pub(super) encode_fragment_phase: u8,
    pub(super) encode_name_cursor: usize,
    pub(super) output_fault: Option<OutputFault>,
    pub weather_fault: Option<WeatherFault>,
    pub(super) started: Instant,
    pub(super) closing: bool,
    pub(super) close_string_owner: String,
    closing_tail:Option<semio_framework_value::retirement::controlled::ControlledRetirement<EnergyAuthorityRetirement>>,
    tail_moved:bool,
}

pub const ENERGY_ABANDONMENT_SLOTS: usize = 64;

enum EnergyAbandonmentSlot {
    Vacant,
    Reserved(Operation),
    Abandoned(Operation, Box<EnergyJobAuthority>),
}

static ENERGY_ABANDONMENT_REGISTRY: Mutex<[EnergyAbandonmentSlot; ENERGY_ABANDONMENT_SLOTS]> = Mutex::new([const { EnergyAbandonmentSlot::Vacant }; ENERGY_ABANDONMENT_SLOTS]);

pub struct EnergyJob {
    pub(super) authority: Option<Box<EnergyJobAuthority>>,
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
                input_identity: EnergyInputIdentity::default(),
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
                precompute_close: Default::default(),
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
                closing_tail:None,
                tail_moved:false,
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
