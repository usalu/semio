//! 🚀️ Engine orchestration: Model + SimulationConfig → Results.

use crate::error::Error;
use crate::kernel::{SimulationConfig, SimulationEnvironment, SimulationKernel, SimulationModel, SurfaceState, TimestepBuilder, TimestepWork, ZoneState};
use crate::meters::{EndUse, FuelType, Meter, MeterTable};
use crate::metrics::{EmissionFactors, EnvironmentalMetrics, ResilienceMetrics, SourceEnergyFactors};
use crate::model::{FixedTable, Model};
use crate::output::{TimeSeries, TimeSeriesAppendError, TimeSeriesTable};
use crate::precompute::{PrecomputeBuilder, PrecomputedModel};
use crate::results::{Results, RunMetadata, SizingResult, SizingTables, SummaryRow, SummaryTables};
use crate::site::WeatherRecord;
use crate::sizing::{SizingBuilder, SizingConfig};
use crate::units::Unit;
use crate::zone_air::ZoneAirState;
use semio_framework_job::{allocate_operation_id, default_now_us, CancelToken, Checkpoint, CommitCandidate, Generation, InteractiveJob, JobFault, Operation, RevisionId, StepContext, StepOutcome};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use std::time::Instant;

// #region 🔖️RetainedWire
const ENERGY_WIRE_MAGIC: [u8; 8] = *b"SMENERGY";
const ENERGY_WIRE_VERSION: u16 = 1;
const ENERGY_WIRE_QUEUE_SLOTS: usize = 4;
const ENERGY_WIRE_LEASE_SLOTS: usize = 64;
const ENERGY_WIRE_HEADER_BYTES: usize = 80;
const ENERGY_CHECKPOINT_BYTES: usize = 164;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnergyWireKind {
    Preview = 1,
    Checkpoint = 2,
    Commit = 3,
    Fault = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyWireIdentity {
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
    pub seed: u64,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergyWireRejection {
    Header,
    Version,
    Kind,
    Identity,
    Items,
    Pages,
    Operations,
    ProcessJobs,
    Saturated,
    Backing,
}

pub struct EnergyWirePacket {
    kind: EnergyWireKind,
    identity: EnergyWireIdentity,
    payload: semio_framework_job::RetainedJobPayload,
    preview: Option<EnergyJobPreview>,
    reservation: Option<EnergyCommitReservation>,
}

pub struct EnergyWireLease {
    kind: EnergyWireKind,
    token: u64,
    recovery_slot: usize,
    packet: Option<EnergyWirePacket>,
}

impl std::fmt::Debug for EnergyWireLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("EnergyWireLease").field("kind", &self.kind).field("token", &self.token).field("identity", &self.packet.as_ref().map(|packet| packet.identity)).finish()
    }
}

impl EnergyWireLease {
    pub fn identity(&self) -> EnergyWireIdentity {
        self.packet.as_ref().expect("live lease owns packet").identity
    }

    pub fn packet(&self) -> &EnergyWirePacket {
        self.packet.as_ref().expect("live lease owns packet")
    }

    pub fn packet_mut(&mut self) -> &mut EnergyWirePacket {
        self.packet.as_mut().expect("live lease owns packet")
    }
}

enum EnergyWireLeaseRecoverySlot {
    Vacant,
    Reserved(u64),
    Abandoned(u64, EnergyWirePacket),
}

static ENERGY_WIRE_LEASE_RECOVERY: Mutex<[EnergyWireLeaseRecoverySlot; ENERGY_WIRE_LEASE_SLOTS]> = Mutex::new([const { EnergyWireLeaseRecoverySlot::Vacant }; ENERGY_WIRE_LEASE_SLOTS]);

impl Drop for EnergyWireLease {
    fn drop(&mut self) {
        let Some(packet) = self.packet.take() else { return };
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        debug_assert!(matches!(registry[self.recovery_slot], EnergyWireLeaseRecoverySlot::Reserved(token) if token == self.token));
        registry[self.recovery_slot] = EnergyWireLeaseRecoverySlot::Abandoned(self.token, packet);
    }
}

impl std::fmt::Debug for EnergyWirePacket {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("EnergyWirePacket").field("kind", &self.kind).field("identity", &self.identity).field("pages", &self.payload.page_count()).field("bytes", &self.payload.len()).finish()
    }
}

impl EnergyWirePacket {
    pub fn identity(&self) -> EnergyWireIdentity {
        self.identity
    }

    pub fn preview(&self) -> Option<&EnergyJobPreview> {
        self.preview.as_ref()
    }

    pub fn ack_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::JobPayloadCloseStep {
        self.payload.close_step(maximum_items, maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.terminal_is_empty()
    }
}

struct EnergyWireQueue {
    kind: EnergyWireKind,
    slots: [Option<EnergyWirePacket>; ENERGY_WIRE_QUEUE_SLOTS],
    head: usize,
    len: usize,
    next_token: u64,
    in_flight: Option<EnergyWireInFlight>,
    reserved_push: bool,
}

#[derive(Clone, Copy)]
struct EnergyWireInFlight {
    token: u64,
    recovery_slot: usize,
    generation: u64,
    pages: usize,
    bytes: usize,
}

impl EnergyWireQueue {
    fn new(kind: EnergyWireKind) -> Self {
        Self { kind, slots: std::array::from_fn(|_| None), head: 0, len: 0, next_token: 0, in_flight: None, reserved_push: false }
    }

    fn push(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
        if packet.kind != self.kind || self.len == ENERGY_WIRE_QUEUE_SLOTS || self.reserved_push {
            return Err(packet);
        }
        let index = (self.head + self.len) % ENERGY_WIRE_QUEUE_SLOTS;
        self.slots[index] = Some(packet);
        self.len += 1;
        Ok(())
    }

    fn reserve_push(&mut self) -> Result<(), EnergyWireRejection> {
        if self.len == ENERGY_WIRE_QUEUE_SLOTS || self.reserved_push {
            return Err(EnergyWireRejection::Saturated);
        }
        self.reserved_push = true;
        Ok(())
    }

    fn push_reserved(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
        if packet.kind != self.kind || !self.reserved_push || self.len == ENERGY_WIRE_QUEUE_SLOTS {
            return Err(packet);
        }
        self.reserved_push = false;
        let index = (self.head + self.len) % ENERGY_WIRE_QUEUE_SLOTS;
        self.slots[index] = Some(packet);
        self.len += 1;
        Ok(())
    }

    fn take(&mut self) -> Option<EnergyWireLease> {
        self.recover_lost();
        if self.len == 0 || self.in_flight.is_some() {
            return None;
        }
        let packet = self.slots[self.head].take()?;
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(recovery_slot) = registry.iter().position(|slot| matches!(slot, EnergyWireLeaseRecoverySlot::Vacant)) else {
            self.slots[self.head] = Some(packet);
            return None;
        };
        let token = self.next_token;
        self.next_token = self.next_token.checked_add(1).expect("Energy queue lease sequence exhausted");
        registry[recovery_slot] = EnergyWireLeaseRecoverySlot::Reserved(token);
        self.in_flight = Some(EnergyWireInFlight { token, recovery_slot, generation: packet.identity.generation, pages: packet.payload.page_count(), bytes: packet.payload.len() });
        Some(EnergyWireLease { kind: self.kind, token, recovery_slot, packet: Some(packet) })
    }

    fn retry(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        if lease.kind != self.kind || !self.in_flight.is_some_and(|flight| flight.token == lease.token && flight.recovery_slot == lease.recovery_slot && lease.packet.as_ref().is_some_and(|packet| packet.identity.generation == flight.generation)) {
            return Err(lease);
        }
        let packet = lease.packet.take().expect("retry lease owns exact packet");
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(registry[lease.recovery_slot], EnergyWireLeaseRecoverySlot::Reserved(token) if token == lease.token) {
            lease.packet = Some(packet);
            return Err(lease);
        }
        registry[lease.recovery_slot] = EnergyWireLeaseRecoverySlot::Vacant;
        self.slots[self.head] = Some(packet);
        self.in_flight = None;
        Ok(())
    }

    fn ack(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        if lease.kind != self.kind
            || !self.in_flight.is_some_and(|flight| flight.token == lease.token && flight.recovery_slot == lease.recovery_slot && lease.packet.as_ref().is_some_and(|packet| packet.identity.generation == flight.generation))
            || !lease.packet().terminal_is_empty()
        {
            return Err(lease);
        }
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(registry[lease.recovery_slot], EnergyWireLeaseRecoverySlot::Reserved(token) if token == lease.token) {
            return Err(lease);
        }
        lease.packet.take();
        registry[lease.recovery_slot] = EnergyWireLeaseRecoverySlot::Vacant;
        self.in_flight = None;
        self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
        self.len -= 1;
        Ok(())
    }

    fn ack_transfer(&mut self, mut lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
        if lease.kind != self.kind || !self.in_flight.is_some_and(|flight| flight.token == lease.token && flight.recovery_slot == lease.recovery_slot && lease.packet.as_ref().is_some_and(|packet| packet.identity.generation == flight.generation)) {
            return Err(lease);
        }
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(registry[lease.recovery_slot], EnergyWireLeaseRecoverySlot::Reserved(token) if token == lease.token) {
            return Err(lease);
        }
        let packet = lease.packet.take().expect("transfer lease owns exact packet");
        registry[lease.recovery_slot] = EnergyWireLeaseRecoverySlot::Vacant;
        self.in_flight = None;
        self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
        self.len -= 1;
        Ok(packet)
    }

    fn recover_lost(&mut self) -> bool {
        let Some(EnergyWireInFlight { token, recovery_slot, generation, .. }) = self.in_flight else { return false };
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(&registry[recovery_slot], EnergyWireLeaseRecoverySlot::Abandoned(candidate, packet) if *candidate == token && packet.identity.generation == generation) {
            return false;
        }
        let EnergyWireLeaseRecoverySlot::Abandoned(_, packet) = std::mem::replace(&mut registry[recovery_slot], EnergyWireLeaseRecoverySlot::Vacant) else { unreachable!("matched abandoned wire lease") };
        self.slots[self.head] = Some(packet);
        self.in_flight = None;
        true
    }

    fn take_terminal(&mut self) -> Option<EnergyWirePacket> {
        self.recover_lost();
        if self.len == 0 || self.in_flight.is_some() {
            return None;
        }
        let packet = self.slots[self.head].take()?;
        self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
        self.len -= 1;
        Some(packet)
    }

    fn retained_totals_at(&self, index: usize) -> Option<(usize, usize, usize)> {
        if index < ENERGY_WIRE_QUEUE_SLOTS {
            return self.slots[index].as_ref().map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1)));
        }
        if index == ENERGY_WIRE_QUEUE_SLOTS {
            return self.in_flight.map(|flight| (flight.pages, flight.bytes, flight.pages.saturating_add(1)));
        }
        None
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
        self.recover_lost();
        if self.in_flight.is_some() {
            return Some((0, 0));
        }
        if self.len == 0 && self.reserved_push {
            self.reserved_push = false;
            return Some((1, 0));
        }
        let packet = self.slots[self.head].as_mut()?;
        match packet.payload.close_step(maximum_items, maximum_bytes) {
            semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => Some((released_items, released_bytes)),
            semio_framework_job::JobPayloadCloseStep::Complete => {
                self.slots[self.head] = None;
                self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
                self.len -= 1;
                Some((1, 0))
            }
        }
    }
}

struct EnergyPublicationChannels {
    preview: Option<EnergyWirePacket>,
    retiring_preview: Option<EnergyWirePacket>,
    checkpoints: EnergyWireQueue,
    commits: EnergyWireQueue,
    faults: EnergyWireQueue,
}

impl Default for EnergyPublicationChannels {
    fn default() -> Self {
        Self { preview: None, retiring_preview: None, checkpoints: EnergyWireQueue::new(EnergyWireKind::Checkpoint), commits: EnergyWireQueue::new(EnergyWireKind::Commit), faults: EnergyWireQueue::new(EnergyWireKind::Fault) }
    }
}

struct EnergyWireBuild {
    kind: EnergyWireKind,
    identity: EnergyWireIdentity,
    writer: semio_framework_job::RetainedJobPayloadWriter,
    field: u8,
}

fn energy_wire_header(kind: EnergyWireKind, identity: EnergyWireIdentity, census: EnergyNumericalCensus, stage: EnergyJobStage, tier: EnergyQualityTier) -> [u8; ENERGY_WIRE_HEADER_BYTES] {
    let mut bytes = [0; ENERGY_WIRE_HEADER_BYTES];
    bytes[..8].copy_from_slice(&ENERGY_WIRE_MAGIC);
    bytes[8..10].copy_from_slice(&ENERGY_WIRE_VERSION.to_le_bytes());
    bytes[10] = kind as u8;
    bytes[11] = stage.code();
    bytes[12] = tier.code();
    bytes[16..24].copy_from_slice(&identity.operation.to_le_bytes());
    bytes[24..32].copy_from_slice(&identity.base_revision.to_le_bytes());
    bytes[32..40].copy_from_slice(&identity.generation.to_le_bytes());
    bytes[40..48].copy_from_slice(&identity.seed.to_le_bytes());
    bytes[48..56].copy_from_slice(&identity.sequence.to_le_bytes());
    bytes[56..64].copy_from_slice(&(census.observed_items as u64).to_le_bytes());
    bytes[64..72].copy_from_slice(&(census.pages as u64).to_le_bytes());
    bytes[72..76].copy_from_slice(&(census.operations as u32).to_le_bytes());
    bytes[76..80].copy_from_slice(&(census.process_jobs as u32).to_le_bytes());
    bytes
}
// #endregion

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
    InitializeWarmupHistory,
    WarmupTimestep,
    WarmupConvergence,
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
    pub facility_electricity_kwh: f64,
}

// #region 🔖️NumericalAdmission
/// 📏️ Schema-first simultaneous working-set census for an Energy numerical job.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnergyNumericalCensus {
    pub zones: usize,
    pub surfaces: usize,
    pub fenestrations: usize,
    pub people: usize,
    pub lighting: usize,
    pub equipment: usize,
    pub infiltrations: usize,
    pub airflow_nodes: usize,
    pub airflow_links: usize,
    pub mechanical_ventilations: usize,
    pub thermostats: usize,
    pub humidistats: usize,
    pub ideal_loads: usize,
    pub faults: usize,
    pub zone_equipment: usize,
    pub plant_loops: usize,
    pub plant_equipment: usize,
    pub pv_systems: usize,
    pub batteries: usize,
    pub service_hot_water: usize,
    pub refrigeration: usize,
    pub water: usize,
    pub weather_records: usize,
    pub timesteps: usize,
    pub meters: usize,
    pub series: usize,
    pub samples: usize,
    pub history_values: usize,
    pub summary_rows: usize,
    pub identifier_bytes: usize,
    pub observed_items: usize,
    pub observed_bytes: usize,
    pub pages: usize,
    pub operations: usize,
    pub process_jobs: usize,
}

/// 🚧️ The independent admitted dimension that rejected an exact owner graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergyNumericalDimension {
    Zones,
    Surfaces,
    Fenestrations,
    People,
    Lighting,
    Equipment,
    Infiltrations,
    AirflowNodes,
    AirflowLinks,
    MechanicalVentilations,
    Thermostats,
    Humidistats,
    IdealLoads,
    Faults,
    ZoneEquipment,
    PlantLoops,
    PlantEquipment,
    PvSystems,
    Batteries,
    ServiceHotWater,
    Refrigeration,
    Water,
    WeatherRecords,
    Timesteps,
    Meters,
    Series,
    Samples,
    HistoryValues,
    SummaryRows,
    IdentifierBytes,
    ObservedItems,
    ObservedBytes,
    Pages,
    Operations,
    ProcessJobs,
    ArithmeticOverflow,
}

/// 🛡️ Checked maxima for every independent numerical dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyNumericalBounds(pub EnergyNumericalCensus);

impl Default for EnergyNumericalBounds {
    fn default() -> Self {
        Self(EnergyNumericalCensus {
            zones: 65_536,
            surfaces: 1_048_576,
            fenestrations: 1_048_576,
            people: 1_048_576,
            lighting: 1_048_576,
            equipment: 1_048_576,
            infiltrations: 1_048_576,
            airflow_nodes: 1_048_576,
            airflow_links: 1_048_576,
            mechanical_ventilations: 1_048_576,
            thermostats: 65_536,
            humidistats: 65_536,
            ideal_loads: 1_048_576,
            faults: 1_048_576,
            zone_equipment: 1_048_576,
            plant_loops: 65_536,
            plant_equipment: 1_048_576,
            pv_systems: 1_048_576,
            batteries: 1_048_576,
            service_hot_water: 1_048_576,
            refrigeration: 1_048_576,
            water: 1_048_576,
            weather_records: 1_048_576,
            timesteps: 1_048_576,
            meters: 1_048_576,
            series: 65_536,
            samples: 67_108_864,
            history_values: 67_108_864,
            summary_rows: 1_048_576,
            identifier_bytes: 268_435_456,
            observed_items: 134_217_728,
            observed_bytes: 2_147_483_648,
            pages: 131_072,
            operations: 1,
            process_jobs: 1,
        })
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
    pub fn retry(self, bounds: EnergyNumericalBounds) -> Result<EnergyJob, Self> {
        EnergyJob::admit(self.operation, self.model, self.config, bounds)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some((released_items, released_bytes)) = close_model_step(&mut self.model, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = close_config_step(&mut self.config, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        model_is_terminal_empty(&self.model) && config_is_terminal_empty(&self.config)
    }
}

/// ♻️ Exact retained wire and Model+Config owner rejected before restore admission.
#[derive(Debug)]
pub struct EnergyCheckpointRejected {
    pub operation: Operation,
    pub model: Model,
    pub config: SimulationConfig,
    pub packet: EnergyWirePacket,
    pub census: EnergyNumericalCensus,
    pub reason: EnergyCheckpointRejectionReason,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergyCheckpointRejectionReason {
    InvalidHeader,
    InvalidPayload,
    OperationMismatch,
    Numerical(EnergyNumericalDimension),
    ProcessJobs,
}

impl EnergyCheckpointRejected {
    pub fn retry(self, bounds: EnergyNumericalBounds) -> Result<EnergyRestoreJob, Self> {
        EnergyRestoreJob::admit(self.operation, self.model, self.config, self.packet, bounds)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if !self.packet.terminal_is_empty() {
            return match self.packet.ack_step(1, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
            };
        }
        if let Some((released_items, released_bytes)) = close_model_step(&mut self.model, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = close_config_step(&mut self.config, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.packet.terminal_is_empty() && model_is_terminal_empty(&self.model) && config_is_terminal_empty(&self.config)
    }
}

/// 📥️ Retained field-at-a-time restore authority; no model owner moves until decoding completes.
pub struct EnergyRestoreJob {
    operation: Operation,
    model: Option<Model>,
    config: Option<SimulationConfig>,
    packet: Option<EnergyWirePacket>,
    census: EnergyNumericalCensus,
    field: u8,
    stage: EnergyJobStage,
    tier: EnergyQualityTier,
    warmup_hour: u32,
    hour_index: u32,
    total_timesteps: u32,
    rng_state: u64,
    weather_cursor: usize,
    weather_target: usize,
    aggregate_zone_cursor: usize,
    aggregate_facility_cursor: u8,
    initialize_backing_stage: u8,
    run_backing_stage: u8,
    checkpoint_due: bool,
    time_series_count: usize,
    meter_count: usize,
    history_count: usize,
    digest: u64,
    bounds: EnergyNumericalBounds,
    rebuild: Option<EnergyJob>,
    replay_retiring: Option<EnergyWirePacket>,
    replay_checkpoint_lease: Option<EnergyWireLease>,
    replay_checkpoint_pending: bool,
    replay_failed: Option<EnergyWireRejection>,
    ready: bool,
    abandonment_slot: usize,
}

impl std::fmt::Debug for EnergyRestoreJob {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("EnergyRestoreJob").field("operation", &self.operation).field("field", &self.field).field("mounted", &self.packet.is_some()).finish()
    }
}

struct EnergyRestoreAbandoned {
    model: Option<Model>,
    config: Option<SimulationConfig>,
    packet: EnergyWirePacket,
    census: EnergyNumericalCensus,
    field: u8,
    stage: EnergyJobStage,
    tier: EnergyQualityTier,
    warmup_hour: u32,
    hour_index: u32,
    total_timesteps: u32,
    rng_state: u64,
    weather_cursor: usize,
    weather_target: usize,
    aggregate_zone_cursor: usize,
    aggregate_facility_cursor: u8,
    initialize_backing_stage: u8,
    run_backing_stage: u8,
    checkpoint_due: bool,
    time_series_count: usize,
    meter_count: usize,
    history_count: usize,
    digest: u64,
    bounds: EnergyNumericalBounds,
    rebuild: Option<EnergyJob>,
    replay_retiring: Option<EnergyWirePacket>,
    replay_checkpoint_lease: Option<EnergyWireLease>,
    replay_checkpoint_pending: bool,
    replay_failed: Option<EnergyWireRejection>,
    ready: bool,
}

enum EnergyRestoreAbandonmentSlot {
    Vacant,
    Reserved(Operation),
    Abandoned(Operation, Box<EnergyRestoreAbandoned>),
}

static ENERGY_RESTORE_ABANDONMENT_REGISTRY: Mutex<[EnergyRestoreAbandonmentSlot; ENERGY_ABANDONMENT_SLOTS]> = Mutex::new([const { EnergyRestoreAbandonmentSlot::Vacant }; ENERGY_ABANDONMENT_SLOTS]);

impl EnergyRestoreJob {
    pub fn admit(operation: Operation, model: Model, config: SimulationConfig, packet: EnergyWirePacket, bounds: EnergyNumericalBounds) -> Result<Self, EnergyCheckpointRejected> {
        let census = EnergyNumericalCensus::observe(&model, &config).unwrap_or_default();
        let reject = |model, config, packet, reason| EnergyCheckpointRejected { operation, model, config, packet, census, reason };
        let Some(observed) = EnergyNumericalCensus::observe(&model, &config) else {
            return Err(reject(model, config, packet, EnergyCheckpointRejectionReason::Numerical(EnergyNumericalDimension::ArithmeticOverflow)));
        };
        if let Some(dimension) = observed.first_exceeded(bounds.0) {
            return Err(reject(model, config, packet, EnergyCheckpointRejectionReason::Numerical(dimension)));
        }
        if packet.kind != EnergyWireKind::Checkpoint || packet.payload.page_count() != 1 || packet.payload.page_count() > observed.pages || packet.payload.len() != ENERGY_CHECKPOINT_BYTES {
            return Err(reject(model, config, packet, EnergyCheckpointRejectionReason::InvalidHeader));
        }
        let Some(header) = packet.payload.page(0).filter(|page| page.len() >= ENERGY_WIRE_HEADER_BYTES) else {
            return Err(reject(model, config, packet, EnergyCheckpointRejectionReason::InvalidHeader));
        };
        if header[..8] != ENERGY_WIRE_MAGIC || u16::from_le_bytes([header[8], header[9]]) != ENERGY_WIRE_VERSION || header[10] != EnergyWireKind::Checkpoint as u8 {
            return Err(reject(model, config, packet, EnergyCheckpointRejectionReason::InvalidHeader));
        }
        let declared_items = u64::from_le_bytes(header[56..64].try_into().expect("fixed item cap"));
        let declared_pages = u64::from_le_bytes(header[64..72].try_into().expect("fixed page cap"));
        let declared_operations = u32::from_le_bytes(header[72..76].try_into().expect("fixed operation cap"));
        let declared_jobs = u32::from_le_bytes(header[76..80].try_into().expect("fixed process cap"));
        if declared_items != observed.observed_items as u64 || declared_pages != observed.pages as u64 || declared_operations != 1 || declared_jobs != 1 {
            return Err(reject(model, config, packet, EnergyCheckpointRejectionReason::InvalidPayload));
        }
        let identity = EnergyWireIdentity {
            operation: u64::from_le_bytes(header[16..24].try_into().expect("fixed operation field")),
            base_revision: u64::from_le_bytes(header[24..32].try_into().expect("fixed revision field")),
            generation: u64::from_le_bytes(header[32..40].try_into().expect("fixed generation field")),
            seed: u64::from_le_bytes(header[40..48].try_into().expect("fixed seed field")),
            sequence: u64::from_le_bytes(header[48..56].try_into().expect("fixed sequence field")),
        };
        if identity != packet.identity || identity.operation != operation.operation.0 || identity.base_revision != operation.base_revision.0 || identity.generation != operation.generation.0 || identity.seed != operation.seed {
            return Err(reject(model, config, packet, EnergyCheckpointRejectionReason::OperationMismatch));
        }
        let mut registry = ENERGY_RESTORE_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(abandonment_slot) = registry.iter().position(|slot| matches!(slot, EnergyRestoreAbandonmentSlot::Vacant)) else {
            return Err(reject(model, config, packet, EnergyCheckpointRejectionReason::ProcessJobs));
        };
        registry[abandonment_slot] = EnergyRestoreAbandonmentSlot::Reserved(operation);
        drop(registry);
        Ok(Self {
            operation,
            model: Some(model),
            config: Some(config),
            packet: Some(packet),
            census: observed,
            field: 0,
            stage: EnergyJobStage::Validate,
            tier: EnergyQualityTier::SteadyStateEstimate,
            warmup_hour: 0,
            hour_index: 0,
            total_timesteps: 0,
            rng_state: 0,
            weather_cursor: 0,
            weather_target: 0,
            aggregate_zone_cursor: 0,
            aggregate_facility_cursor: 0,
            initialize_backing_stage: 0,
            run_backing_stage: 0,
            checkpoint_due: false,
            time_series_count: 0,
            meter_count: 0,
            history_count: 0,
            digest: 0,
            bounds,
            rebuild: None,
            replay_retiring: None,
            replay_checkpoint_lease: None,
            replay_checkpoint_pending: false,
            replay_failed: None,
            ready: false,
            abandonment_slot,
        })
    }

    pub fn recover_abandoned(operation: Operation) -> Option<Self> {
        let mut registry = ENERGY_RESTORE_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let index = registry.iter().position(|slot| matches!(slot, EnergyRestoreAbandonmentSlot::Abandoned(candidate, _) if candidate.operation == operation.operation && candidate.generation == operation.generation))?;
        let EnergyRestoreAbandonmentSlot::Abandoned(_, authority) = std::mem::replace(&mut registry[index], EnergyRestoreAbandonmentSlot::Reserved(operation)) else { unreachable!("matched restore authority") };
        Some(Self {
            operation,
            model: authority.model,
            config: authority.config,
            packet: Some(authority.packet),
            census: authority.census,
            field: authority.field,
            stage: authority.stage,
            tier: authority.tier,
            warmup_hour: authority.warmup_hour,
            hour_index: authority.hour_index,
            total_timesteps: authority.total_timesteps,
            rng_state: authority.rng_state,
            weather_cursor: authority.weather_cursor,
            weather_target: authority.weather_target,
            aggregate_zone_cursor: authority.aggregate_zone_cursor,
            aggregate_facility_cursor: authority.aggregate_facility_cursor,
            initialize_backing_stage: authority.initialize_backing_stage,
            run_backing_stage: authority.run_backing_stage,
            checkpoint_due: authority.checkpoint_due,
            time_series_count: authority.time_series_count,
            meter_count: authority.meter_count,
            history_count: authority.history_count,
            digest: authority.digest,
            bounds: authority.bounds,
            rebuild: authority.rebuild,
            replay_retiring: authority.replay_retiring,
            replay_checkpoint_lease: authority.replay_checkpoint_lease,
            replay_checkpoint_pending: authority.replay_checkpoint_pending,
            replay_failed: authority.replay_failed,
            ready: authority.ready,
            abandonment_slot: index,
        })
    }

    pub fn step(&mut self, context: &mut StepContext<'_>) -> Result<bool, EnergyWireRejection> {
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        if context.is_cancelled() || context.should_yield() {
            return Ok(false);
        }
        if self.field < 4 {
            context.consume_fuel(1);
            let page = self.packet.as_ref().and_then(|packet| packet.payload.single_page()).ok_or(EnergyWireRejection::Header)?;
            match self.field {
                0 => {
                    self.stage = energy_stage_from_code(page[11]).ok_or(EnergyWireRejection::Header)?;
                    self.tier = energy_tier_from_code(page[12]).ok_or(EnergyWireRejection::Header)?;
                }
                1 => {
                    self.warmup_hour = u32::from_le_bytes(page[80..84].try_into().map_err(|_| EnergyWireRejection::Header)?);
                    self.hour_index = u32::from_le_bytes(page[84..88].try_into().map_err(|_| EnergyWireRejection::Header)?);
                    self.total_timesteps = u32::from_le_bytes(page[88..92].try_into().map_err(|_| EnergyWireRejection::Header)?);
                }
                2 => {
                    self.rng_state = u64::from_le_bytes(page[92..100].try_into().map_err(|_| EnergyWireRejection::Header)?);
                    self.weather_cursor = u64::from_le_bytes(page[100..108].try_into().map_err(|_| EnergyWireRejection::Header)?) as usize;
                    self.weather_target = u64::from_le_bytes(page[108..116].try_into().map_err(|_| EnergyWireRejection::Header)?) as usize;
                    if self.weather_cursor > self.weather_target || self.weather_target > self.census.weather_records {
                        return Err(EnergyWireRejection::Items);
                    }
                }
                3 => {
                    let aggregate_zone_cursor = u64::from_le_bytes(page[116..124].try_into().map_err(|_| EnergyWireRejection::Header)?) as usize;
                    self.time_series_count = u64::from_le_bytes(page[132..140].try_into().map_err(|_| EnergyWireRejection::Header)?) as usize;
                    self.meter_count = u64::from_le_bytes(page[140..148].try_into().map_err(|_| EnergyWireRejection::Header)?) as usize;
                    self.history_count = u64::from_le_bytes(page[148..156].try_into().map_err(|_| EnergyWireRejection::Header)?) as usize;
                    if aggregate_zone_cursor > self.census.zones || self.time_series_count > self.census.series || self.meter_count > self.census.meters || self.history_count > self.census.history_values {
                        return Err(EnergyWireRejection::Items);
                    }
                    self.aggregate_zone_cursor = aggregate_zone_cursor;
                    self.aggregate_facility_cursor = page[124];
                    self.initialize_backing_stage = page[125];
                    self.run_backing_stage = page[126];
                    self.checkpoint_due = page[127] != 0;
                    self.digest = u64::from_le_bytes(page[156..164].try_into().map_err(|_| EnergyWireRejection::Header)?);
                }
                _ => unreachable!("decode field is bounded"),
            }
            self.field += 1;
            return Ok(false);
        }
        if let Some(packet) = self.replay_retiring.as_mut() {
            if packet.terminal_is_empty() {
                self.replay_retiring = None;
            } else {
                let _ = packet.ack_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return Ok(false);
        }
        if let Some(lease) = self.replay_checkpoint_lease.as_mut() {
            if !lease.packet().terminal_is_empty() {
                let _ = lease.packet_mut().ack_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                return Ok(false);
            }
            let lease = self.replay_checkpoint_lease.take().expect("empty replay checkpoint lease remains owned");
            if let Err(lease) = self.rebuild.as_mut().ok_or(EnergyWireRejection::Backing)?.ack_checkpoint_packet(lease) {
                self.replay_checkpoint_lease = Some(lease);
                return Err(EnergyWireRejection::Backing);
            }
            return Ok(false);
        }
        if self.replay_checkpoint_pending {
            self.replay_checkpoint_lease = self.rebuild.as_mut().ok_or(EnergyWireRejection::Backing)?.take_checkpoint_packet(self.operation.generation).map_err(|_| EnergyWireRejection::Identity)?;
            self.replay_checkpoint_pending = false;
            return Ok(false);
        }
        if let Some(failure) = self.replay_failed {
            return Err(failure);
        }
        if self.rebuild.is_none() {
            context.consume_fuel(1);
            let model = self.model.take().ok_or(EnergyWireRejection::Backing)?;
            let config = self.config.take().ok_or(EnergyWireRejection::Backing)?;
            match EnergyJob::admit(self.operation, model, config, self.bounds) {
                Ok(job) => self.rebuild = Some(job),
                Err(rejected) => {
                    self.model = Some(rejected.model);
                    self.config = Some(rejected.config);
                    return Err(EnergyWireRejection::Backing);
                }
            }
            return Ok(false);
        }
        let expected = (
            self.stage,
            self.tier,
            self.warmup_hour,
            self.hour_index,
            self.total_timesteps,
            self.rng_state,
            self.weather_cursor,
            self.weather_target,
            self.aggregate_zone_cursor,
            self.aggregate_facility_cursor,
            self.initialize_backing_stage,
            self.run_backing_stage,
            self.checkpoint_due,
            self.time_series_count,
            self.meter_count,
            self.history_count,
            self.digest,
        );
        let rebuild = self.rebuild.as_mut().expect("rebuild mounted");
        let scalar_match = rebuild.stage == expected.0
            && rebuild.tier == expected.1
            && rebuild.warmup_hour == expected.2
            && rebuild.hour_index == expected.3
            && rebuild.total_timesteps == expected.4
            && rebuild.rng_state == expected.5
            && rebuild.weather_cursor == expected.6
            && rebuild.weather_target == expected.7
            && rebuild.aggregate_zone_cursor == expected.8
            && rebuild.aggregate_facility_cursor == expected.9
            && rebuild.initialize_backing_stage == expected.10
            && rebuild.run_backing_stage == expected.11
            && rebuild.time_series.series.len() == expected.13
            && rebuild.meters.meters.len() == expected.14
            && rebuild.zone_temperature_history.len() == expected.15
            && rebuild.result_backing.series_slots >= expected.13
            && rebuild.result_backing.meter_slots >= expected.14
            && rebuild.result_backing.history_slots >= expected.15;
        if scalar_match && rebuild.checkpoint_due != expected.12 {
            context.consume_fuel(1);
            rebuild.checkpoint_due = expected.12;
            return Ok(false);
        }
        if scalar_match && rebuild.numerical_digest() == expected.16 {
            self.ready = true;
            return Ok(true);
        }
        match rebuild.step(context) {
            StepOutcome::Yield => {}
            StepOutcome::PreviewReady(mut notice) => {
                if !notice.terminal_is_empty() {
                    self.replay_retiring = Some(EnergyWirePacket { kind: EnergyWireKind::Preview, identity: self.packet.as_ref().expect("restore packet").identity, payload: notice, preview: None, reservation: None });
                    return Ok(false);
                }
                self.replay_retiring = rebuild.take_preview_packet(self.operation.generation).map_err(|_| EnergyWireRejection::Identity)?;
            }
            StepOutcome::CheckpointReady(mut checkpoint) => {
                if !checkpoint.state.terminal_is_empty() {
                    self.replay_retiring = Some(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity: self.packet.as_ref().expect("restore packet").identity, payload: checkpoint.state, preview: None, reservation: None });
                    self.replay_checkpoint_pending = true;
                    return Ok(false);
                }
                if let Some(lease) = rebuild.take_checkpoint_packet(self.operation.generation).map_err(|_| EnergyWireRejection::Identity)? {
                    self.replay_checkpoint_lease = Some(lease);
                }
            }
            StepOutcome::Fault(mut fault) => {
                self.replay_failed = Some(EnergyWireRejection::Backing);
                if !fault.detail.terminal_is_empty() {
                    self.replay_retiring = Some(EnergyWirePacket { kind: EnergyWireKind::Fault, identity: self.packet.as_ref().expect("restore packet").identity, payload: fault.detail, preview: None, reservation: None });
                    return Ok(false);
                }
                return Err(self.replay_failed.expect("replay failure retained"));
            }
            StepOutcome::Complete(mut candidate) => {
                self.replay_failed = Some(EnergyWireRejection::Items);
                if !candidate.state.terminal_is_empty() {
                    self.replay_retiring = Some(EnergyWirePacket { kind: EnergyWireKind::Commit, identity: self.packet.as_ref().expect("restore packet").identity, payload: candidate.state, preview: None, reservation: None });
                    return Ok(false);
                }
                if !candidate.output.terminal_is_empty() {
                    self.replay_retiring = Some(EnergyWirePacket { kind: EnergyWireKind::Commit, identity: self.packet.as_ref().expect("restore packet").identity, payload: candidate.output, preview: None, reservation: None });
                    return Ok(false);
                }
                return Err(self.replay_failed.expect("replay completion before target retained"));
            }
            StepOutcome::Cancelled => return Ok(false),
        }
        Ok(false)
    }

    pub fn finish(mut self, context: &StepContext<'_>) -> Result<EnergyJob, Self> {
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation || !self.ready {
            return Err(self);
        }
        let Some(mut job) = self.rebuild.take() else {
            return Err(self);
        };
        if job.time_series.series.len() != self.time_series_count
            || job.meters.meters.len() != self.meter_count
            || job.zone_temperature_history.len() != self.history_count
            || job.result_backing.series_slots < self.time_series_count
            || job.result_backing.meter_slots < self.meter_count
            || job.result_backing.history_slots < self.history_count
            || job.numerical_digest() != self.digest
        {
            self.rebuild = Some(job);
            return Err(self);
        }
        job.restore_input = self.packet.take();
        let mut registry = ENERGY_RESTORE_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        registry[self.abandonment_slot] = EnergyRestoreAbandonmentSlot::Vacant;
        drop(registry);
        Ok(job)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(packet) = self.replay_retiring.as_mut() {
            let step = packet.ack_step(1, maximum_bytes);
            if packet.terminal_is_empty() {
                self.replay_retiring = None;
            }
            return match step {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
            };
        }
        if let Some(mut lease) = self.replay_checkpoint_lease.take() {
            if !lease.packet().terminal_is_empty() {
                let step = lease.packet_mut().ack_step(1, maximum_bytes);
                self.replay_checkpoint_lease = Some(lease);
                return match step {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            let Some(rebuild) = self.rebuild.as_mut() else {
                self.replay_checkpoint_lease = Some(lease);
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            };
            if let Err(lease) = rebuild.ack_checkpoint_packet(lease) {
                self.replay_checkpoint_lease = Some(lease);
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            }
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(packet) = self.packet.as_mut() {
            let step = packet.ack_step(1, maximum_bytes);
            if packet.terminal_is_empty() {
                self.packet = None;
            }
            return match step {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
            };
        }
        if let Some(rebuild) = self.rebuild.as_mut() {
            rebuild.begin_close();
            return match rebuild.close_step(1, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Complete => {
                    self.rebuild = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                step => step,
            };
        }
        if let Some(model) = self.model.as_mut() {
            if let Some((released_items, released_bytes)) = close_model_step(model, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
            self.model = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(config) = self.config.as_mut() {
            if let Some((released_items, released_bytes)) = close_config_step(config, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
            self.config = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.packet.is_none() && self.model.is_none() && self.config.is_none() && self.rebuild.is_none() && self.replay_retiring.is_none() && self.replay_checkpoint_lease.is_none() && !self.replay_checkpoint_pending
    }
}

impl Drop for EnergyRestoreJob {
    fn drop(&mut self) {
        let Some(packet) = self.packet.take() else { return };
        let authority = EnergyRestoreAbandoned {
            model: self.model.take(),
            config: self.config.take(),
            packet,
            census: self.census,
            field: self.field,
            stage: self.stage,
            tier: self.tier,
            warmup_hour: self.warmup_hour,
            hour_index: self.hour_index,
            total_timesteps: self.total_timesteps,
            rng_state: self.rng_state,
            weather_cursor: self.weather_cursor,
            weather_target: self.weather_target,
            aggregate_zone_cursor: self.aggregate_zone_cursor,
            aggregate_facility_cursor: self.aggregate_facility_cursor,
            initialize_backing_stage: self.initialize_backing_stage,
            run_backing_stage: self.run_backing_stage,
            checkpoint_due: self.checkpoint_due,
            time_series_count: self.time_series_count,
            meter_count: self.meter_count,
            history_count: self.history_count,
            digest: self.digest,
            bounds: self.bounds,
            rebuild: self.rebuild.take(),
            replay_retiring: self.replay_retiring.take(),
            replay_checkpoint_lease: self.replay_checkpoint_lease.take(),
            replay_checkpoint_pending: self.replay_checkpoint_pending,
            replay_failed: self.replay_failed.take(),
            ready: self.ready,
        };
        let mut registry = ENERGY_RESTORE_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        registry[self.abandonment_slot] = EnergyRestoreAbandonmentSlot::Abandoned(self.operation, Box::new(authority));
    }
}

impl EnergyNumericalCensus {
    fn observe(model: &Model, config: &SimulationConfig) -> Option<Self> {
        let timesteps = SimulationKernel::run_period(config).total_hours() as usize;
        let weather_records = observed_weather_records(config);
        let airflow_nodes = model.airflow_network.as_ref().map_or(0, |network| network.zone_node_ids.capacity().checked_add(1).unwrap_or(usize::MAX));
        let airflow_links = model.airflow_network.as_ref().map_or(0, |network| network.link_ids.capacity());
        let plant_equipment = checked_sum(model.plant_loops.iter().map(|plant| plant.equipment_ids.capacity()))?;
        let meters = model.zones.capacity().checked_mul(3)?.checked_add(2)?;
        let series = model.zones.capacity();
        let samples = timesteps.checked_mul(series)?;
        let history_values = samples;
        let summary_rows = 7usize;
        let identifier_bytes = observed_identifier_bytes(model, config)?;
        let dimensions = [
            model.zones.capacity(),
            model.surfaces.capacity(),
            model.fenestrations.capacity(),
            model.people.capacity(),
            model.lighting.capacity(),
            model.equipment.capacity(),
            model.infiltrations.capacity(),
            airflow_nodes,
            airflow_links,
            model.mechanical_ventilations.capacity(),
            model.thermostats.capacity(),
            model.humidistats.capacity(),
            model.ideal_loads.capacity(),
            model.faults.capacity(),
            model.zone_equipment.capacity(),
            model.plant_loops.capacity(),
            plant_equipment,
            model.pv_systems.capacity(),
            model.battery_storage.capacity(),
            model.shw_systems.capacity(),
            model.refrigeration_systems.capacity(),
            model.water_systems.capacity(),
            weather_records,
            timesteps,
            meters,
            series,
            samples,
            history_values,
            summary_rows,
        ];
        let observed_items = checked_sum(dimensions.into_iter())?;
        let observed_bytes =
            observed_model_bytes(model, config)?.checked_add(weather_records.checked_mul(std::mem::size_of::<Option<(usize, WeatherRecord)>>())?)?.checked_add(samples.checked_mul(std::mem::size_of::<f64>() * 3)?)?.checked_add(identifier_bytes)?;
        let pages = observed_bytes.checked_add(16_383)?.checked_div(16_384)?;
        Some(Self {
            zones: model.zones.capacity(),
            surfaces: model.surfaces.capacity(),
            fenestrations: model.fenestrations.capacity(),
            people: model.people.capacity(),
            lighting: model.lighting.capacity(),
            equipment: model.equipment.capacity(),
            infiltrations: model.infiltrations.capacity(),
            airflow_nodes,
            airflow_links,
            mechanical_ventilations: model.mechanical_ventilations.capacity(),
            thermostats: model.thermostats.capacity(),
            humidistats: model.humidistats.capacity(),
            ideal_loads: model.ideal_loads.capacity(),
            faults: model.faults.capacity(),
            zone_equipment: model.zone_equipment.capacity(),
            plant_loops: model.plant_loops.capacity(),
            plant_equipment,
            pv_systems: model.pv_systems.capacity(),
            batteries: model.battery_storage.capacity(),
            service_hot_water: model.shw_systems.capacity(),
            refrigeration: model.refrigeration_systems.capacity(),
            water: model.water_systems.capacity(),
            weather_records,
            timesteps,
            meters,
            series,
            samples,
            history_values,
            summary_rows,
            identifier_bytes,
            observed_items,
            observed_bytes,
            pages,
            operations: 1,
            process_jobs: 1,
        })
    }

    fn first_exceeded(self, maximum: Self) -> Option<EnergyNumericalDimension> {
        macro_rules! dimension {
            ($field:ident, $variant:ident) => {
                if self.$field > maximum.$field {
                    return Some(EnergyNumericalDimension::$variant);
                }
            };
        }
        dimension!(zones, Zones);
        dimension!(surfaces, Surfaces);
        dimension!(fenestrations, Fenestrations);
        dimension!(people, People);
        dimension!(lighting, Lighting);
        dimension!(equipment, Equipment);
        dimension!(infiltrations, Infiltrations);
        dimension!(airflow_nodes, AirflowNodes);
        dimension!(airflow_links, AirflowLinks);
        dimension!(mechanical_ventilations, MechanicalVentilations);
        dimension!(thermostats, Thermostats);
        dimension!(humidistats, Humidistats);
        dimension!(ideal_loads, IdealLoads);
        dimension!(faults, Faults);
        dimension!(zone_equipment, ZoneEquipment);
        dimension!(plant_loops, PlantLoops);
        dimension!(plant_equipment, PlantEquipment);
        dimension!(pv_systems, PvSystems);
        dimension!(batteries, Batteries);
        dimension!(service_hot_water, ServiceHotWater);
        dimension!(refrigeration, Refrigeration);
        dimension!(water, Water);
        dimension!(weather_records, WeatherRecords);
        dimension!(timesteps, Timesteps);
        dimension!(meters, Meters);
        dimension!(series, Series);
        dimension!(samples, Samples);
        dimension!(history_values, HistoryValues);
        dimension!(summary_rows, SummaryRows);
        dimension!(identifier_bytes, IdentifierBytes);
        dimension!(observed_items, ObservedItems);
        dimension!(observed_bytes, ObservedBytes);
        dimension!(pages, Pages);
        dimension!(operations, Operations);
        dimension!(process_jobs, ProcessJobs);
        None
    }
}

fn observed_weather_records(config: &SimulationConfig) -> usize {
    config.weather.as_ref().map_or_else(
        || match config.environment {
            SimulationEnvironment::HeatingDesignDay | SimulationEnvironment::CoolingDesignDay => 24,
            SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => 8_760,
        },
        |weather| weather.records.capacity().max(weather.records.len().max(1)),
    )
}

fn weather_copy_target(config: &SimulationConfig) -> usize {
    config.weather.as_ref().map_or_else(
        || match config.environment {
            SimulationEnvironment::HeatingDesignDay | SimulationEnvironment::CoolingDesignDay => 24,
            SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => 8_760,
        },
        |weather| weather.records.len().max(1),
    )
}

fn checked_sum(values: impl IntoIterator<Item = usize>) -> Option<usize> {
    values.into_iter().try_fold(0usize, usize::checked_add)
}

fn observed_identifier_bytes(model: &Model, config: &SimulationConfig) -> Option<usize> {
    let mut bytes = model.name.capacity().checked_add(model.version.capacity())?;
    macro_rules! names {
        ($owners:expr) => {
            for owner in $owners {
                bytes = bytes.checked_add(owner.name.capacity())?;
            }
        };
    }
    names!(&model.zones);
    names!(&model.spaces);
    names!(&model.surfaces);
    names!(&model.fenestrations);
    names!(&model.materials);
    names!(&model.constructions);
    names!(&model.setpoint_managers);
    names!(&model.air_loops);
    names!(&model.plant_loops);
    names!(&model.shading_surfaces);
    names!(&model.space_lists);
    names!(&model.thermal_enclosures);
    names!(&model.electrical_load_centers);
    for output in &model.output_variables {
        bytes = bytes.checked_add(output.name.capacity())?.checked_add(output.key.capacity())?;
    }
    if let Some(weather) = &config.weather {
        bytes = bytes.checked_add(weather.location.capacity())?;
    }
    for zone in &model.zones {
        let temperature = "Zone Air Temperature [".len().checked_add(zone.name.len())?.checked_add(1)?;
        let heating = zone.name.len().checked_add(" Heating".len())?;
        let cooling = zone.name.len().checked_add(" Cooling".len())?;
        let fans = zone.name.len().checked_add(" Fans".len())?;
        bytes = bytes.checked_add(temperature.checked_mul(3)?)?;
        bytes = bytes.checked_add(heating.checked_add(cooling)?.checked_add(fans)?.checked_mul(3)?)?;
    }
    bytes = bytes.checked_add(("Facility Heating".len() + "Facility PV".len()).checked_mul(3)?)?;
    bytes = bytes.checked_add(model.name.len())?.checked_add(model.version.len())?;
    if let Some(weather) = &config.weather {
        bytes = bytes.checked_add(weather.location.len())?;
    }
    for (key, unit) in [("Electricity", "kWh"), ("Natural Gas", "kWh"), ("Energy Use Intensity", "kWh/m²"), ("Annual Energy Cost", "USD"), ("LCCA Present Value", "USD")] {
        bytes = bytes.checked_add(key.len())?.checked_add(unit.len())?;
    }
    Some(bytes)
}

fn observed_model_bytes(model: &Model, config: &SimulationConfig) -> Option<usize> {
    let mut bytes = 0usize;
    macro_rules! backing {
        ($owners:expr) => {
            bytes = bytes.checked_add(observed_vector_bytes(&$owners)?)?;
        };
    }
    backing!(model.zones);
    backing!(model.spaces);
    backing!(model.surfaces);
    backing!(model.fenestrations);
    backing!(model.materials);
    backing!(model.constructions);
    backing!(model.people);
    backing!(model.lighting);
    backing!(model.equipment);
    backing!(model.thermostats);
    backing!(model.humidistats);
    backing!(model.setpoint_managers);
    backing!(model.ideal_loads);
    backing!(model.zone_equipment);
    backing!(model.air_loops);
    backing!(model.plant_loops);
    backing!(model.outdoor_air_systems);
    backing!(model.infiltrations);
    backing!(model.mechanical_ventilations);
    backing!(model.shading_surfaces);
    backing!(model.space_lists);
    backing!(model.thermal_enclosures);
    backing!(model.adjacency_pairs);
    backing!(model.electrical_load_centers);
    backing!(model.pv_systems);
    backing!(model.battery_storage);
    backing!(model.shw_systems);
    backing!(model.solar_thermal_systems);
    backing!(model.refrigeration_systems);
    backing!(model.water_systems);
    backing!(model.faults);
    backing!(model.output_variables);
    backing!(model.sizing_objects);
    backing!(model.daylight_zones);
    backing!(model.room_air_models);
    for surface in &model.surfaces {
        bytes = bytes.checked_add(surface.vertices_m.capacity().checked_mul(std::mem::size_of::<[f64; 3]>())?)?;
    }
    for construction in &model.constructions {
        bytes = bytes.checked_add(construction.layer_material_ids.capacity().checked_mul(std::mem::size_of::<crate::model::EntityId>())?)?;
    }
    for plant in &model.plant_loops {
        bytes = bytes.checked_add(plant.equipment_ids.capacity().checked_mul(std::mem::size_of::<crate::model::EntityId>())?)?;
    }
    for air_loop in &model.air_loops {
        bytes = bytes.checked_add(observed_vector_bytes(&air_loop.terminal_zone_ids)?)?;
    }
    for shading in &model.shading_surfaces {
        bytes = bytes.checked_add(observed_vector_bytes(&shading.vertices_m)?)?;
    }
    for list in &model.space_lists {
        bytes = bytes.checked_add(observed_vector_bytes(&list.space_ids)?)?;
    }
    for enclosure in &model.thermal_enclosures {
        bytes = bytes.checked_add(observed_vector_bytes(&enclosure.zone_ids)?)?;
    }
    for center in &model.electrical_load_centers {
        bytes = bytes.checked_add(observed_vector_bytes(&center.generator_ids)?)?;
        bytes = bytes.checked_add(observed_vector_bytes(&center.pv_ids)?)?;
        bytes = bytes.checked_add(observed_vector_bytes(&center.battery_ids)?)?;
    }
    if let Some(network) = &model.airflow_network {
        bytes = bytes.checked_add(network.zone_node_ids.capacity().checked_mul(std::mem::size_of::<(crate::model::EntityId, u32)>())?)?;
        bytes = bytes.checked_add(network.link_ids.capacity().checked_mul(std::mem::size_of::<u32>())?)?;
    }
    if let Some(weather) = &config.weather {
        bytes = bytes.checked_add(weather.records.capacity().checked_mul(std::mem::size_of::<WeatherRecord>())?)?;
    }
    backing!(config.schedules.constants);
    backing!(config.schedules.daily);
    backing!(config.schedules.weekly);
    backing!(config.schedules.annual);
    backing!(config.schedules.time_series);
    for schedule in &config.schedules.annual {
        bytes = bytes.checked_add(observed_vector_bytes(&schedule.rules)?)?;
        bytes = bytes.checked_add(observed_vector_bytes(&schedule.holiday_dates)?)?;
    }
    for schedule in &config.schedules.time_series {
        bytes = bytes.checked_add(observed_vector_bytes(&schedule.values)?)?;
    }
    Some(bytes)
}

fn observed_vector_bytes<T>(owners: &Vec<T>) -> Option<usize> {
    owners.capacity().checked_mul(std::mem::size_of::<T>())
}
// #endregion 🔖️NumericalAdmission

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WarmupConvergenceWork {
    stage: WarmupConvergenceStage,
    cursor: usize,
    temperature_converged: bool,
    load_converged: bool,
    evaluate: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ValidationWork {
    stage: ValidationStage,
    cursor: usize,
    child_cursor: usize,
    zone_reference_family: u8,
    zone_ids: FixedTable<crate::model::EntityId, ()>,
    material_ids: FixedTable<crate::model::EntityId, ()>,
    construction_ids: FixedTable<crate::model::EntityId, ()>,
    surface_ids: FixedTable<crate::model::EntityId, ()>,
    fatal_code: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum ResultBuildStage {
    ModelName,
    ModelVersion,
    WeatherLocation,
    Assemble,
    Complete,
}

#[cfg(test)]
const P7C1_RESULT_BUILD_STAGES: [ResultBuildStage; 5] = [ResultBuildStage::ModelName, ResultBuildStage::ModelVersion, ResultBuildStage::WeatherLocation, ResultBuildStage::Assemble, ResultBuildStage::Complete];

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum AggregateZoneStage {
    Temperature,
    Heating,
    Cooling,
    Fan,
    Complete,
}

#[cfg(test)]
const P7C1_AGGREGATE_STAGES: [AggregateZoneStage; 5] = [AggregateZoneStage::Temperature, AggregateZoneStage::Heating, AggregateZoneStage::Cooling, AggregateZoneStage::Fan, AggregateZoneStage::Complete];

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
struct EnergyAdmittedResultBacking {
    series_slots: usize,
    meter_slots: usize,
    samples_per_series: usize,
    history_slots: usize,
    summary_slots: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug)]
struct EnergyCommitReservation {
    pages: usize,
    bytes: usize,
    items: usize,
    prepared: usize,
    sources: [Option<semio_framework_job::JobPayloadPageSource>; semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES],
}

impl EnergyCommitReservation {
    fn new(pages: usize, bytes: usize, items: usize) -> Self {
        Self { pages, bytes, items, prepared: 0, sources: std::array::from_fn(|_| None) }
    }

    fn take_source(&mut self, index: usize) -> Option<semio_framework_job::JobPayloadPageSource> {
        self.sources.get_mut(index)?.take()
    }

    fn restore_source(&mut self, index: usize, source: semio_framework_job::JobPayloadPageSource) {
        self.sources[index] = Some(source);
    }

    fn close_source_step(&mut self) -> bool {
        let Some(source) = self.sources.iter_mut().find(|source| source.is_some()) else { return false };
        source.take();
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    operation: Operation,
    model: Model,
    config: SimulationConfig,
    numerical_census: EnergyNumericalCensus,
    stage: EnergyJobStage,
    tier: EnergyQualityTier,
    weather: FixedTable<usize, WeatherRecord>,
    weather_cursor: usize,
    weather_target: usize,
    precompute: Option<PrecomputeBuilder>,
    validation: ValidationWork,
    pre: Option<PrecomputedModel>,
    state: Option<SimulationModel>,
    initialize_cursor: usize,
    initialize_backing_stage: u8,
    warmup_hour: u32,
    previous_temperatures: Vec<f64>,
    previous_loads: Vec<f64>,
    warmup_convergence: Option<WarmupConvergenceWork>,
    run_hours: Option<crate::calendar::RunPeriodHours>,
    total_timesteps: u32,
    run_backing_stage: u8,
    hour_index: u32,
    aggregate_zone_cursor: usize,
    aggregate_zone_work: Option<AggregateZoneWork>,
    aggregate_facility_cursor: u8,
    aggregate_facility_work: Option<AggregateZoneWork>,
    timestep_work: Option<TimestepWork>,
    timestep_builder: Option<TimestepBuilder>,
    rng_state: u64,
    checkpoint_due: bool,
    result_backing: EnergyAdmittedResultBacking,
    time_series: TimeSeriesTable,
    meters: MeterTable,
    time_series_order: Vec<String>,
    meter_order: Vec<String>,
    zone_temperature_history: Vec<f64>,
    facility_electricity_j: f64,
    publication: EnergyPublicationChannels,
    restore_input: Option<EnergyWirePacket>,
    wire_build: Option<EnergyWireBuild>,
    wire_ready: Option<EnergyWirePacket>,
    result: Option<Results>,
    sizing_builder: Option<SizingBuilder>,
    final_sizing: Option<SizingTables>,
    final_summaries: SummaryTables,
    final_environmental: Option<EnvironmentalMetrics>,
    final_resilience: Option<ResilienceMetrics>,
    finalization: FinalizationWork,
    result_build: ResultBuildWork,
    output_writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
    output_payload: Option<semio_framework_job::RetainedJobPayload>,
    commit_census: Option<CommitCensusWork>,
    commit_reservation: Option<EnergyCommitReservation>,
    commit_acknowledged: bool,
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

const ENERGY_ABANDONMENT_SLOTS: usize = 64;

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
    pub fn new(operation: Operation, model: Model, config: SimulationConfig) -> Result<Self, EnergyAdmissionRejected> {
        Self::admit(operation, model, config, EnergyNumericalBounds::default())
    }

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

    pub fn stage(&self) -> EnergyJobStage {
        self.stage
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

    pub fn retry_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.checkpoints.retry(lease)
    }

    pub fn ack_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.checkpoints.ack(lease)
    }

    pub fn ack_checkpoint_for_restore(&mut self, lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
        self.publication.checkpoints.ack_transfer(lease)
    }

    pub fn take_commit_packet(&mut self, generation: Generation) -> Result<Option<EnergyWireLease>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.commits.take())
    }

    pub fn retry_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.commits.retry(lease)
    }

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

    pub fn retry_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.faults.retry(lease)
    }

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

    fn wire_identity(&self, sequence: u64) -> EnergyWireIdentity {
        EnergyWireIdentity { operation: self.operation.operation.0, base_revision: self.operation.base_revision.0, generation: self.operation.generation.0, seed: self.operation.seed, sequence }
    }

    fn start_wire(&mut self, kind: EnergyWireKind, sequence: u64, stream: semio_framework_job::JobPayloadStream) -> Result<(), EnergyWireRejection> {
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

    fn step_wire_build(&mut self, context: &mut StepContext<'_>) -> Result<bool, EnergyWireRejection> {
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

    fn begin_preview(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return StepOutcome::Fault(JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
        };
        if self.start_wire(EnergyWireKind::Preview, sequence, semio_framework_job::JobPayloadStream::Preview).is_err() {
            return StepOutcome::Yield;
        }
        StepOutcome::Yield
    }

    fn drive_wire_publication(&mut self, context: &mut StepContext<'_>) -> Option<StepOutcome> {
        if let Some(retiring) = self.publication.retiring_preview.as_mut() {
            match retiring.payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                semio_framework_job::JobPayloadCloseStep::Complete => self.publication.retiring_preview = None,
                semio_framework_job::JobPayloadCloseStep::Pending { .. } => return Some(StepOutcome::Yield),
            }
        }
        if self.wire_build.is_some() {
            if self.step_wire_build(context).is_err() {
                return Some(Self::fault(&Error::severe("energy wire backing rejected")));
            }
            return Some(StepOutcome::Yield);
        }
        let mut packet = self.wire_ready.take()?;
        match packet.kind {
            EnergyWireKind::Preview => {
                if self.publication.preview.is_some() {
                    self.publication.retiring_preview = self.publication.preview.take();
                    self.wire_ready = Some(packet);
                    return Some(StepOutcome::Yield);
                }
                let Some(preview) = decode_preview_packet(&packet) else {
                    self.wire_ready = Some(packet);
                    return Some(Self::fault(&Error::severe("energy preview schema rejected")));
                };
                packet.preview = Some(preview);
                self.publication.preview = Some(packet);
                Some(StepOutcome::PreviewReady(semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Preview)))
            }
            EnergyWireKind::Checkpoint => match self.publication.checkpoints.push(packet) {
                Ok(()) => Some(StepOutcome::CheckpointReady(Checkpoint { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), applied_progress: self.hour_index as u64 })),
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(StepOutcome::Yield)
                }
            },
            EnergyWireKind::Commit => match self.publication.commits.push(packet) {
                Ok(()) => {
                    self.stage = EnergyJobStage::Complete;
                    Some(StepOutcome::Yield)
                }
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(StepOutcome::Yield)
                }
            },
            EnergyWireKind::Fault => match self.publication.faults.push(packet) {
                Ok(()) => Some(Self::fault(&Error::severe("energy retained fault"))),
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(StepOutcome::Yield)
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

    fn numerical_digest(&self) -> u64 {
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

    fn fault(error: &Error) -> StepOutcome {
        let _ = error;
        StepOutcome::Fault(JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) })
    }

    fn begin_fault(&mut self) -> StepOutcome {
        if self.start_wire(EnergyWireKind::Fault, 0, semio_framework_job::JobPayloadStream::Fault).is_ok() {
            StepOutcome::Yield
        } else {
            Self::fault(&Error::severe("energy fault channel unavailable"))
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
                if work.material_ids.admit(self.model.materials.len()).is_err() {
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
                if let Some(material) = self.model.materials.get(work.cursor) {
                    match work.material_ids.insert(material.id, ()) {
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
                    if let crate::model::OutsideBoundary::Interzone(other) = surface.outside_boundary_condition {
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
        let dt_s = self.pre.as_ref().map_or(0.0, |pre| pre.zone_timestep_s);
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
                if let Err(fault) = series.append_admitted(self.hour_index as f64, zone_state.air.temp_c) {
                    work.series_fault = Some(fault);
                    work.backing_rejected = true;
                    return false;
                }
                self.zone_temperature_history.push(zone_state.air.temp_c);
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
        let dt_s = self.pre.as_ref().map_or(0.0, |pre| pre.zone_timestep_s);
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
                add_resident(&mut work, queue.retained_totals_at(work.channel_slot))?;
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
                    || aggregate_pages > self.numerical_census.pages
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
        if let Some(outcome) = self.drive_wire_publication(context) {
            return outcome;
        }
        match self.stage {
            EnergyJobStage::Validate => {
                let complete = self.step_validation();
                if self.validation.fatal_code != 0 {
                    return self.begin_fault();
                }
                if !complete {
                    return StepOutcome::Yield;
                }
                self.weather_target = weather_copy_target(&self.config);
                self.set_stage(context, EnergyJobStage::ResolveWeather);
                StepOutcome::Yield
            }
            EnergyJobStage::ResolveWeather => {
                if self.weather_cursor < self.weather_target {
                    let record = self.weather_record(self.weather_cursor);
                    if self.weather.insert_stable(self.weather_cursor, record).is_err() {
                        self.weather_fault = Some(WeatherFault::SlotRejected);
                        return self.begin_fault();
                    }
                    self.weather_cursor += 1;
                    if self.weather_cursor.is_multiple_of(256) {
                        return self.begin_preview(context);
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
                    if builder.backing_rejected() {
                        return self.begin_fault();
                    }
                }
                StepOutcome::Yield
            }
            EnergyJobStage::InitializeZones => {
                if self.initialize_backing_stage == 0 {
                    if self.state.as_mut().expect("state exists while reserving zones").zones.admit(self.model.zones.len()).is_err() {
                        return self.begin_fault();
                    }
                    self.initialize_backing_stage = 1;
                    return StepOutcome::Yield;
                }
                if let Some(zone) = self.model.zones.get(self.initialize_cursor) {
                    let weather = *self.weather.get_index(0).expect("admitted weather record");
                    if self.state.as_mut().expect("state exists while initializing").zones.insert(zone.id, ZoneState { air: ZoneAirState::new(weather.dry_bulb_c, weather.humidity_ratio()), ..ZoneState::empty() }).is_err() {
                        return self.begin_fault();
                    }
                    self.initialize_cursor += 1;
                } else {
                    self.initialize_cursor = 0;
                    self.initialize_backing_stage = 2;
                    self.set_stage(context, EnergyJobStage::InitializeSurfaces);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::InitializeSurfaces => {
                if self.initialize_backing_stage == 2 {
                    if self.state.as_mut().expect("state exists while reserving surfaces").surfaces.admit(self.model.surfaces.len()).is_err() {
                        return self.begin_fault();
                    }
                    self.initialize_backing_stage = 3;
                    return StepOutcome::Yield;
                }
                if let Some(surface) = self.model.surfaces.get(self.initialize_cursor) {
                    if let Some(precomputed) = self.pre.as_ref().and_then(|pre| pre.surfaces.get(&surface.id)) {
                        let temperature = self.weather.get_index(0).expect("admitted weather record").dry_bulb_c;
                        if self
                            .state
                            .as_mut()
                            .expect("state exists while initializing")
                            .surfaces
                            .insert(surface.id, SurfaceState { inside_temp_c: temperature, outside_temp_c: temperature, heat_flux_w: 0.0, ctf: precomputed.ctf.clone(), convection_to_zone_w: 0.0 })
                            .is_err()
                        {
                            return self.begin_fault();
                        }
                    }
                    self.initialize_cursor += 1;
                } else {
                    self.initialize_cursor = 0;
                    self.set_stage(context, EnergyJobStage::InitializeWarmupHistory);
                    return self.begin_preview(context);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::InitializeWarmupHistory => {
                let zones = self.pre.as_ref().map_or(0, |pre| pre.zone_order.len());
                if self.previous_temperatures.capacity() < zones {
                    if self.previous_temperatures.try_reserve_exact(zones).is_err() {
                        return self.begin_fault();
                    }
                    return StepOutcome::Yield;
                }
                if self.previous_temperatures.len() < zones {
                    self.previous_temperatures.push(f64::NAN);
                    return StepOutcome::Yield;
                }
                if self.previous_loads.capacity() < zones {
                    if self.previous_loads.try_reserve_exact(zones).is_err() {
                        return self.begin_fault();
                    }
                    return StepOutcome::Yield;
                }
                if self.previous_loads.len() < zones {
                    self.previous_loads.push(f64::NAN);
                    return StepOutcome::Yield;
                }
                self.set_stage(context, EnergyJobStage::WarmupTimestep);
                StepOutcome::Yield
            }
            EnergyJobStage::WarmupTimestep => {
                let warmup_hours = self.config.warmup_days.saturating_mul(24);
                if self.warmup_hour >= warmup_hours {
                    if let Some(state) = &mut self.state {
                        state.warmup_complete = true;
                    }
                    self.set_stage(context, EnergyJobStage::StartRun);
                    return StepOutcome::Yield;
                }
                let pre = self.pre.as_ref().expect("precompute complete before warmup");
                if self.timestep_work.is_none() && self.timestep_builder.is_none() {
                    let weather = *self.weather.get_index(self.warmup_hour as usize % self.weather.len()).expect("admitted warmup weather record");
                    let date = crate::calendar::SimDate::new(weather.year, weather.month, weather.day);
                    self.timestep_builder = Some(TimestepBuilder::new(&self.model, pre, weather, date, self.warmup_hour as f64, pre.zone_timestep_s));
                    return StepOutcome::Yield;
                }
                if self.timestep_work.is_none() {
                    match self.timestep_builder.as_mut().expect("warmup timestep builder").step(&self.model, pre) {
                        Ok(Some(work)) => {
                            self.timestep_work = Some(work);
                            self.timestep_builder = None;
                        }
                        Ok(None) => return StepOutcome::Yield,
                        Err(_) => return self.begin_fault(),
                    }
                }
                self.timestep_work.as_mut().expect("warmup timestep work exists").step(&self.model, &self.config, pre, self.state.as_mut().expect("state initialized before warmup"));
                if context.should_yield() {
                    return StepOutcome::Yield;
                }
                self.rng_state = self.rng_state.rotate_left(17).wrapping_mul(0x94d0_49bb_1331_11eb);
                if !self.timestep_work.as_ref().is_some_and(TimestepWork::is_complete) {
                    return StepOutcome::Yield;
                }
                self.timestep_work = None;
                self.warmup_convergence =
                    Some(WarmupConvergenceWork { stage: WarmupConvergenceStage::TemperatureCheck, cursor: 0, temperature_converged: true, load_converged: true, evaluate: self.warmup_hour > 24 && self.warmup_hour.is_multiple_of(24) });
                self.set_stage(context, EnergyJobStage::WarmupConvergence);
                StepOutcome::Yield
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
                StepOutcome::Yield
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
                            return self.begin_fault();
                        }
                        self.result_backing.series_slots = self.time_series.series.capacity();
                    }
                    2 => {
                        if self.meters.meters.admit(meters).is_err() {
                            return self.begin_fault();
                        }
                        self.result_backing.meter_slots = self.meters.meters.capacity();
                    }
                    3 => {
                        if self.time_series_order.try_reserve_exact(zones).is_err() {
                            return self.begin_fault();
                        }
                    }
                    4 => {
                        if self.meter_order.try_reserve_exact(meters).is_err() {
                            return self.begin_fault();
                        }
                    }
                    5 => {
                        if self.zone_temperature_history.try_reserve_exact(self.numerical_census.history_values).is_err() {
                            return self.begin_fault();
                        }
                        self.result_backing.history_slots = self.zone_temperature_history.capacity();
                    }
                    6 => {
                        if self.final_summaries.annual_energy.try_reserve_exact(self.numerical_census.summary_rows).is_err() {
                            return self.begin_fault();
                        }
                        self.result_backing.summary_slots = self.final_summaries.annual_energy.capacity();
                    }
                    _ => {
                        self.set_stage(context, EnergyJobStage::RunZoneTimestep);
                        return StepOutcome::Yield;
                    }
                }
                self.run_backing_stage += 1;
                StepOutcome::Yield
            }
            EnergyJobStage::RunZoneTimestep => {
                let pre = self.pre.as_ref().expect("precompute complete before run");
                if self.timestep_work.is_none() && self.timestep_builder.is_none() {
                    let Some((date, hour, _)) = self.run_hours.as_mut().and_then(|hours| hours.next()) else {
                        self.set_stage(context, EnergyJobStage::Finalize);
                        return StepOutcome::Yield;
                    };
                    let mut weather = *self.weather.get_index(self.hour_index as usize % self.weather.len()).expect("admitted run weather record");
                    weather.year = date.year;
                    weather.month = date.month;
                    weather.day = date.day;
                    weather.hour = hour;
                    self.timestep_builder = Some(TimestepBuilder::new(&self.model, pre, weather, date, self.hour_index as f64, pre.zone_timestep_s));
                    return StepOutcome::Yield;
                }
                if self.timestep_work.is_none() {
                    match self.timestep_builder.as_mut().expect("run timestep builder").step(&self.model, pre) {
                        Ok(Some(work)) => {
                            self.timestep_work = Some(work);
                            self.timestep_builder = None;
                        }
                        Ok(None) => return StepOutcome::Yield,
                        Err(_) => return self.begin_fault(),
                    }
                }
                self.timestep_work.as_mut().expect("run timestep work exists").step(&self.model, &self.config, pre, self.state.as_mut().expect("state initialized before run"));
                if context.should_yield() {
                    return StepOutcome::Yield;
                }
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
                    let complete = self.step_aggregate_zone();
                    if self.aggregate_zone_work.as_ref().is_some_and(|work| work.backing_rejected) {
                        return self.begin_fault();
                    }
                    if complete {
                        self.aggregate_zone_cursor += 1;
                    }
                } else {
                    self.set_stage(context, EnergyJobStage::AggregateFacility);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::AggregateFacility => {
                let complete = self.step_aggregate_facility();
                if self.aggregate_facility_work.as_ref().is_some_and(|work| work.backing_rejected) {
                    return self.begin_fault();
                }
                if complete {
                    self.hour_index = self.hour_index.saturating_add(1);
                    self.checkpoint_due = self.hour_index.is_multiple_of(24);
                    self.set_stage(context, EnergyJobStage::PublishTimestep);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::PublishTimestep => {
                if self.checkpoint_due {
                    self.checkpoint_due = false;
                    if self.start_wire(EnergyWireKind::Checkpoint, self.hour_index as u64, semio_framework_job::JobPayloadStream::CheckpointState).is_err() {
                        return StepOutcome::Yield;
                    }
                    return StepOutcome::Yield;
                }
                self.set_stage(context, EnergyJobStage::RunZoneTimestep);
                self.begin_preview(context)
            }
            EnergyJobStage::Finalize => {
                self.tier = EnergyQualityTier::Final;
                self.sizing_builder = Some(SizingBuilder::new(SizingConfig::default()));
                self.finalization = FinalizationWork::default();
                self.set_stage(context, EnergyJobStage::Size);
                StepOutcome::Yield
            }
            EnergyJobStage::Size => {
                let builder = self.sizing_builder.as_mut().expect("sizing builder exists");
                if builder.fault().is_some() {
                    return self.begin_fault();
                }
                if builder.is_complete(&self.model) {
                    self.final_sizing = self.sizing_builder.take().map(SizingBuilder::finish);
                    self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                } else {
                    builder.step(&self.model);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::FinalizeSummaries => {
                let complete = self.step_finalization();
                if self.finalization.row_backing_rejected {
                    return self.begin_fault();
                }
                if complete {
                    self.set_stage(context, EnergyJobStage::BuildResults);
                }
                StepOutcome::Yield
            }
            EnergyJobStage::FinalizeMetrics => {
                self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                StepOutcome::Yield
            }
            EnergyJobStage::FinalizeEconomics => {
                self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                StepOutcome::Yield
            }
            EnergyJobStage::BuildResults => match self.step_result_build() {
                Ok(true) => {
                    self.set_stage(context, EnergyJobStage::PublishFinal);
                    self.begin_preview(context)
                }
                Ok(false) => StepOutcome::Yield,
                Err(_) => self.begin_fault(),
            },
            EnergyJobStage::PublishFinal => {
                self.set_stage(context, EnergyJobStage::EncodeOutput);
                StepOutcome::Yield
            }
            EnergyJobStage::EncodeOutput => {
                if self.commit_reservation.is_none() {
                    if self.commit_census.is_none() {
                        self.commit_census = Some(CommitCensusWork::default());
                        return StepOutcome::Yield;
                    }
                    match self.step_commit_census() {
                        Ok(false) => return StepOutcome::Yield,
                        Ok(true) => {}
                        Err(fault) => {
                            self.output_fault = Some(fault);
                            return self.begin_fault();
                        }
                    }
                }
                if self.output_writer.is_none() && self.output_payload.is_none() {
                    self.output_writer = Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput));
                    return StepOutcome::Yield;
                }
                match self.encode_output_step(context) {
                    Ok(false) => return StepOutcome::Yield,
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
                        return StepOutcome::Yield;
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
                            return StepOutcome::Yield;
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
                            return StepOutcome::Yield;
                        }
                    }
                }
                StepOutcome::Yield
            }
            EnergyJobStage::Complete => {
                if std::mem::take(&mut self.commit_acknowledged) {
                    return StepOutcome::Complete(CommitCandidate {
                        state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                        output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                    });
                }
                let Some(packet) = self.publication.commits.take_terminal() else { return StepOutcome::Yield };
                StepOutcome::Complete(CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState), output: packet.payload })
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.closing = true;
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(build) = self.wire_build.as_mut() {
            build.writer.begin_close();
            return match build.writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.wire_build = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        if let Some(writer) = self.output_writer.as_mut() {
            writer.begin_close();
            return match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.output_writer = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        if let Some(payload) = self.output_payload.as_mut() {
            return match payload.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.output_payload = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        macro_rules! close_packet {
            ($packet:expr) => {
                if let Some(packet) = $packet.as_mut() {
                    return match packet.payload.close_step(maximum_items, maximum_bytes) {
                        semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                        semio_framework_job::JobPayloadCloseStep::Complete => {
                            $packet = None;
                            semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                        }
                    };
                }
            };
        }
        close_packet!(self.wire_ready);
        close_packet!(self.restore_input);
        close_packet!(self.publication.preview);
        close_packet!(self.publication.retiring_preview);
        if let Some((released_items, released_bytes)) = self.publication.checkpoints.close_step(maximum_items, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = self.publication.commits.close_step(maximum_items, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = self.publication.faults.close_step(maximum_items, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some(work) = self.commit_census.as_mut() {
            if work.reservation.as_mut().is_some_and(EnergyCommitReservation::close_source_step) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES };
            }
            self.commit_census = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(reservation) = self.commit_reservation.as_mut() {
            if reservation.close_source_step() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES };
            }
            self.commit_reservation = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if std::mem::take(&mut self.commit_pages_mounted) != 0 || std::mem::take(&mut self.commit_items_encoded) != 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.close_string_owner, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        macro_rules! pop_owner {
            ($owners:expr) => {
                if $owners.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            };
        }
        pop_owner!(self.weather);
        if let Some((released_items, released_bytes)) = close_string_vector_step(&mut self.time_series_order, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = close_string_vector_step(&mut self.meter_order, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = close_time_series_step(&mut self.time_series, &mut self.close_string_owner, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = close_meter_table_step(&mut self.meters, &mut self.close_string_owner, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        pop_owner!(self.zone_temperature_history);
        pop_owner!(self.previous_temperatures);
        pop_owner!(self.previous_loads);
        if let Some(work) = self.aggregate_zone_work.as_mut() {
            if let Some((released_items, released_bytes)) = close_aggregate_work_step(work, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
            self.aggregate_zone_work = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(work) = self.aggregate_facility_work.as_mut() {
            if let Some((released_items, released_bytes)) = close_aggregate_work_step(work, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
            self.aggregate_facility_work = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        macro_rules! remove_validation_entry {
            ($owners:expr) => {{
                if $owners.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            }};
        }
        remove_validation_entry!(self.validation.zone_ids);
        remove_validation_entry!(self.validation.material_ids);
        remove_validation_entry!(self.validation.construction_ids);
        remove_validation_entry!(self.validation.surface_ids);
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.finalization.row_unit, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.finalization.row_key, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if !self.result_build.model_name.is_empty() || !self.result_build.model_version.is_empty() || !self.result_build.weather_location.is_empty() {
            if maximum_bytes == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let released_bytes = self.result_build.weather_location.pop().or_else(|| self.result_build.model_version.pop()).or_else(|| self.result_build.model_name.pop()).map_or(0, |character| character.len_utf8());
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes };
        }
        if let Some(builder) = self.timestep_builder.as_mut() {
            if !builder.close_step(maximum_items) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.timestep_builder = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(work) = self.timestep_work.as_mut() {
            if !work.close_step(maximum_items) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.timestep_work = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(builder) = self.precompute.as_mut() {
            if !builder.close_step(maximum_items) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.precompute = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(pre) = self.pre.as_mut() {
            if !pre.close_step(maximum_items) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.pre = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(state) = self.state.as_mut() {
            if state.zones.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if state.surfaces.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            self.state = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(results) = self.result.as_mut() {
            if let Some((released_items, released_bytes)) = close_results_step(results, &mut self.close_string_owner, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
            self.result = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(sizing) = self.final_sizing.as_mut() {
            if let Some((released_items, released_bytes)) = close_sizing_step(sizing, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
            self.final_sizing = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some((released_items, released_bytes)) = close_summaries_step(&mut self.final_summaries, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some(builder) = self.sizing_builder.as_mut() {
            let (complete, released_items, released_bytes) = builder.close_step(maximum_bytes);
            if !complete {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
            self.sizing_builder = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some((released_items, released_bytes)) = close_model_step(&mut self.model, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Some((released_items, released_bytes)) = close_config_step(&mut self.config, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if self.run_hours.take().is_some() || self.final_environmental.take().is_some() || self.final_resilience.take().is_some() || self.output_fault.take().is_some() || self.weather_fault.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        authority_is_terminal_empty(self)
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
            EnergyJobStage::InitializeWarmupHistory => "energy.initialize-warmup-history",
            EnergyJobStage::WarmupTimestep => "energy.warmup-timestep",
            EnergyJobStage::WarmupConvergence => "energy.warmup-convergence",
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

fn energy_stage_from_code(code: u8) -> Option<EnergyJobStage> {
    const STAGES: [EnergyJobStage; 22] = [
        EnergyJobStage::Validate,
        EnergyJobStage::ResolveWeather,
        EnergyJobStage::Precompute,
        EnergyJobStage::InitializeZones,
        EnergyJobStage::InitializeSurfaces,
        EnergyJobStage::InitializeWarmupHistory,
        EnergyJobStage::WarmupTimestep,
        EnergyJobStage::WarmupConvergence,
        EnergyJobStage::StartRun,
        EnergyJobStage::RunZoneTimestep,
        EnergyJobStage::AggregateZone,
        EnergyJobStage::AggregateFacility,
        EnergyJobStage::PublishTimestep,
        EnergyJobStage::Finalize,
        EnergyJobStage::Size,
        EnergyJobStage::FinalizeSummaries,
        EnergyJobStage::FinalizeMetrics,
        EnergyJobStage::FinalizeEconomics,
        EnergyJobStage::BuildResults,
        EnergyJobStage::PublishFinal,
        EnergyJobStage::EncodeOutput,
        EnergyJobStage::Complete,
    ];
    STAGES.get(code as usize).copied()
}

fn energy_tier_from_code(code: u8) -> Option<EnergyQualityTier> {
    const TIERS: [EnergyQualityTier; 4] = [EnergyQualityTier::SteadyStateEstimate, EnergyQualityTier::DesignDay, EnergyQualityTier::CoarseTimestep, EnergyQualityTier::Final];
    TIERS.get(code as usize).copied()
}

fn decode_preview_packet(packet: &EnergyWirePacket) -> Option<EnergyJobPreview> {
    let page = packet.payload.single_page()?;
    if packet.kind != EnergyWireKind::Preview
        || page.len() != ENERGY_WIRE_HEADER_BYTES + 20
        || page[..8] != ENERGY_WIRE_MAGIC
        || u16::from_le_bytes(page[8..10].try_into().ok()?) != ENERGY_WIRE_VERSION
        || page[10] != EnergyWireKind::Preview as u8
        || u64::from_le_bytes(page[16..24].try_into().ok()?) != packet.identity.operation
        || u64::from_le_bytes(page[24..32].try_into().ok()?) != packet.identity.base_revision
        || u64::from_le_bytes(page[32..40].try_into().ok()?) != packet.identity.generation
        || u64::from_le_bytes(page[40..48].try_into().ok()?) != packet.identity.seed
        || u64::from_le_bytes(page[48..56].try_into().ok()?) != packet.identity.sequence
    {
        return None;
    }
    Some(EnergyJobPreview {
        sequence: packet.identity.sequence,
        tier: energy_tier_from_code(page[12])?,
        stage: energy_stage_from_code(page[11])?,
        warmup_hour: u32::from_le_bytes(page[80..84].try_into().ok()?),
        timestep: u32::from_le_bytes(page[84..88].try_into().ok()?),
        total_timesteps: u32::from_le_bytes(page[88..92].try_into().ok()?),
        facility_electricity_kwh: f64::from_le_bytes(page[92..100].try_into().ok()?),
    })
}

fn fuel_code(fuel: FuelType) -> u8 {
    match fuel {
        FuelType::Electricity => 0,
        FuelType::NaturalGas => 1,
        FuelType::Propane => 2,
        FuelType::FuelOil => 3,
        FuelType::DistrictHeating => 4,
        FuelType::DistrictCooling => 5,
        FuelType::Steam => 6,
        FuelType::Water => 7,
        FuelType::OnSiteGeneration => 8,
    }
}

fn end_use_code(end_use: EndUse) -> u32 {
    match end_use {
        EndUse::Heating => 0,
        EndUse::Cooling => 1,
        EndUse::InteriorLighting => 2,
        EndUse::ExteriorLighting => 3,
        EndUse::InteriorEquipment => 4,
        EndUse::ExteriorEquipment => 5,
        EndUse::Fans => 6,
        EndUse::Pumps => 7,
        EndUse::HeatRejection => 8,
        EndUse::Humidification => 9,
        EndUse::Dehumidification => 10,
        EndUse::WaterSystems => 11,
        EndUse::Refrigeration => 12,
        EndUse::Generators => 13,
        EndUse::Custom(value) => 0x8000_0000 | value,
    }
}

fn unit_code(unit: Unit) -> u8 {
    match unit {
        Unit::Dimensionless => 0,
        Unit::Meters => 1,
        Unit::SquareMeters => 2,
        Unit::CubicMeters => 3,
        Unit::Kelvin => 4,
        Unit::Celsius => 5,
        Unit::Pascals => 6,
        Unit::Watts => 7,
        Unit::Joules => 8,
        Unit::KilogramsPerSecond => 9,
        Unit::CubicMetersPerSecond => 10,
        Unit::KilowattHours => 11,
        Unit::HumidityRatio => 12,
        Unit::Percent => 13,
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

fn close_string_vector_step(owners: &mut Vec<String>, maximum_bytes: usize) -> Option<(usize, usize)> {
    let owner = owners.last_mut()?;
    if !owner.is_empty() {
        if maximum_bytes == 0 {
            return Some((0, 0));
        }
        return owner.pop().map(|character| (0, character.len_utf8()));
    }
    owners.pop();
    Some((1, 0))
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

fn close_results_step(results: &mut Results, close_string_owner: &mut String, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some(released) = close_time_series_step(&mut results.time_series, close_string_owner, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_meter_table_step(&mut results.meters, close_string_owner, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_summaries_step(&mut results.summaries, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_sizing_step(&mut results.sizing, maximum_bytes) {
        return Some(released);
    }
    if let Some(error) = results.diagnostics.messages.last_mut() {
        if let Some(context) = error.context.as_mut() {
            if let Some(released) = close_string_step(context, maximum_bytes) {
                return Some(released);
            }
            error.context = None;
            return Some((1, 0));
        }
        if let Some(released) = close_string_step(&mut error.message, maximum_bytes) {
            return Some(released);
        }
        results.diagnostics.messages.pop();
        return Some((1, 0));
    }
    if let Some(released) = close_string_step(&mut results.run_metadata.weather_location, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_string_step(&mut results.run_metadata.model_version, maximum_bytes) {
        return Some(released);
    }
    close_string_step(&mut results.run_metadata.model_name, maximum_bytes)
}

fn close_time_series_step(table: &mut TimeSeriesTable, close_string_owner: &mut String, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some((_, series)) = table.series.last_mut() {
        if series.values.pop().is_some() || series.timestamps_hours.pop().is_some() {
            return Some((1, 0));
        }
        if let Some(released) = close_string_step(&mut series.key, maximum_bytes) {
            return Some(released);
        }
    }
    if table.series.is_empty() {
        return None;
    }
    if let Some((key, _)) = table.series.pop() {
        *close_string_owner = key;
    }
    Some((1, 0))
}

fn close_meter_table_step(table: &mut MeterTable, close_string_owner: &mut String, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some((_, meter)) = table.meters.last_mut() {
        if let Some(released) = close_string_step(&mut meter.name, maximum_bytes) {
            return Some(released);
        }
    }
    if table.meters.is_empty() {
        return None;
    }
    if let Some((key, _)) = table.meters.pop() {
        *close_string_owner = key;
    }
    Some((1, 0))
}

fn close_summaries_step(summaries: &mut SummaryTables, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some(released) = close_summary_rows_step(&mut summaries.annual_energy, maximum_bytes) {
        return Some(released);
    }
    if let Some((_, rows)) = summaries.monthly_energy.last_mut() {
        if let Some(released) = close_summary_rows_step(rows, maximum_bytes) {
            return Some(released);
        }
    }
    if !summaries.monthly_energy.is_empty() {
        summaries.monthly_energy.pop();
        return Some((1, 0));
    }
    if let Some(released) = close_summary_rows_step(&mut summaries.peak_loads, maximum_bytes) {
        return Some(released);
    }
    close_summary_rows_step(&mut summaries.comfort, maximum_bytes)
}

fn close_summary_rows_step(rows: &mut Vec<SummaryRow>, maximum_bytes: usize) -> Option<(usize, usize)> {
    let row = rows.last_mut()?;
    if let Some(released) = close_string_step(&mut row.unit, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_string_step(&mut row.key, maximum_bytes) {
        return Some(released);
    }
    rows.pop();
    Some((1, 0))
}

fn close_sizing_step(sizing: &mut SizingTables, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some(released) = close_sizing_rows_step(&mut sizing.zone_loads, maximum_bytes) {
        return Some(released);
    }
    close_sizing_rows_step(&mut sizing.equipment, maximum_bytes)
}

fn close_sizing_rows_step(rows: &mut Vec<SizingResult>, maximum_bytes: usize) -> Option<(usize, usize)> {
    let row = rows.last_mut()?;
    if let Some(released) = close_string_step(&mut row.component, maximum_bytes) {
        return Some(released);
    }
    rows.pop();
    Some((1, 0))
}

fn close_string_step(owner: &mut String, maximum_bytes: usize) -> Option<(usize, usize)> {
    if owner.is_empty() {
        return None;
    }
    if maximum_bytes == 0 {
        return Some((0, 0));
    }
    owner.pop().map(|character| (0, character.len_utf8()))
}

fn summaries_are_terminal_empty(summaries: &SummaryTables) -> bool {
    summaries.annual_energy.is_empty() && summaries.monthly_energy.is_empty() && summaries.peak_loads.is_empty() && summaries.comfort.is_empty()
}

fn close_model_step(model: &mut Model, maximum_bytes: usize) -> Option<(usize, usize)> {
    macro_rules! close_string {
        ($owner:expr) => {
            if !$owner.is_empty() {
                if maximum_bytes == 0 {
                    return Some((0, 0));
                }
                let bytes = $owner.pop().map_or(0, |character| character.len_utf8());
                return Some((0, bytes));
            }
        };
    }
    macro_rules! close_named {
        ($owners:expr) => {
            if let Some(owner) = $owners.last_mut() {
                close_string!(owner.name);
                $owners.pop();
                return Some((1, 0));
            }
        };
    }
    macro_rules! close_plain {
        ($owners:expr) => {
            if $owners.pop().is_some() {
                return Some((1, 0));
            }
        };
    }
    close_string!(model.name);
    close_string!(model.version);
    close_named!(model.zones);
    close_named!(model.spaces);
    if let Some(surface) = model.surfaces.last_mut() {
        close_plain!(surface.vertices_m);
        close_string!(surface.name);
        model.surfaces.pop();
        return Some((1, 0));
    }
    close_named!(model.fenestrations);
    close_named!(model.materials);
    if let Some(construction) = model.constructions.last_mut() {
        close_plain!(construction.layer_material_ids);
        close_string!(construction.name);
        model.constructions.pop();
        return Some((1, 0));
    }
    close_plain!(model.people);
    close_plain!(model.lighting);
    close_plain!(model.equipment);
    close_plain!(model.thermostats);
    close_plain!(model.humidistats);
    close_named!(model.setpoint_managers);
    close_plain!(model.ideal_loads);
    close_plain!(model.zone_equipment);
    if let Some(loop_owner) = model.air_loops.last_mut() {
        close_plain!(loop_owner.terminal_zone_ids);
        close_string!(loop_owner.name);
        model.air_loops.pop();
        return Some((1, 0));
    }
    if let Some(loop_owner) = model.plant_loops.last_mut() {
        close_plain!(loop_owner.equipment_ids);
        close_string!(loop_owner.name);
        model.plant_loops.pop();
        return Some((1, 0));
    }
    close_plain!(model.outdoor_air_systems);
    close_plain!(model.infiltrations);
    close_plain!(model.mechanical_ventilations);
    if let Some(shading) = model.shading_surfaces.last_mut() {
        close_plain!(shading.vertices_m);
        close_string!(shading.name);
        model.shading_surfaces.pop();
        return Some((1, 0));
    }
    if let Some(list) = model.space_lists.last_mut() {
        close_plain!(list.space_ids);
        close_string!(list.name);
        model.space_lists.pop();
        return Some((1, 0));
    }
    if let Some(enclosure) = model.thermal_enclosures.last_mut() {
        close_plain!(enclosure.zone_ids);
        close_string!(enclosure.name);
        model.thermal_enclosures.pop();
        return Some((1, 0));
    }
    close_plain!(model.adjacency_pairs);
    if let Some(network) = model.airflow_network.as_mut() {
        close_plain!(network.zone_node_ids);
        close_plain!(network.link_ids);
        model.airflow_network = None;
        return Some((1, 0));
    }
    if let Some(center) = model.electrical_load_centers.last_mut() {
        close_plain!(center.generator_ids);
        close_plain!(center.pv_ids);
        close_plain!(center.battery_ids);
        close_string!(center.name);
        model.electrical_load_centers.pop();
        return Some((1, 0));
    }
    close_plain!(model.pv_systems);
    close_plain!(model.battery_storage);
    close_plain!(model.shw_systems);
    close_plain!(model.solar_thermal_systems);
    close_plain!(model.refrigeration_systems);
    close_plain!(model.water_systems);
    close_plain!(model.faults);
    if let Some(output) = model.output_variables.last_mut() {
        close_string!(output.key);
        close_string!(output.name);
        model.output_variables.pop();
        return Some((1, 0));
    }
    close_plain!(model.sizing_objects);
    close_plain!(model.daylight_zones);
    close_plain!(model.room_air_models);
    None
}

/// 🧹 Exact-owner bounded disposer for a partially captured Energy model. Mounted product capture
/// uses this narrow interface so cancellation never drops a nested record, string, or page whole.
pub struct EnergyModelCloseCursor {
    model: Option<Model>,
}

impl EnergyModelCloseCursor {
    pub fn new(model: Model) -> Self {
        Self { model: Some(model) }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        let Some(model) = self.model.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete };
        if let Some((released_items, released_bytes)) = close_model_step(model, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        self.model = None;
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.model.is_none()
    }
}

fn close_config_step(config: &mut SimulationConfig, maximum_bytes: usize) -> Option<(usize, usize)> {
    macro_rules! close_plain {
        ($owners:expr) => {
            if $owners.pop().is_some() {
                return Some((1, 0));
            }
        };
    }
    if let Some(weather) = config.weather.as_mut() {
        close_plain!(weather.records);
        if !weather.location.is_empty() {
            if maximum_bytes == 0 {
                return Some((0, 0));
            }
            return weather.location.pop().map(|character| (0, character.len_utf8()));
        }
        config.weather = None;
        return Some((1, 0));
    }
    if let Some(schedule) = config.schedules.time_series.last_mut() {
        close_plain!(schedule.values);
        config.schedules.time_series.pop();
        return Some((1, 0));
    }
    if let Some(schedule) = config.schedules.annual.last_mut() {
        close_plain!(schedule.rules);
        close_plain!(schedule.holiday_dates);
        config.schedules.annual.pop();
        return Some((1, 0));
    }
    close_plain!(config.schedules.weekly);
    close_plain!(config.schedules.daily);
    close_plain!(config.schedules.constants);
    None
}

fn model_is_terminal_empty(model: &Model) -> bool {
    model.name.is_empty()
        && model.version.is_empty()
        && model.zones.is_empty()
        && model.spaces.is_empty()
        && model.surfaces.is_empty()
        && model.fenestrations.is_empty()
        && model.materials.is_empty()
        && model.constructions.is_empty()
        && model.people.is_empty()
        && model.lighting.is_empty()
        && model.equipment.is_empty()
        && model.thermostats.is_empty()
        && model.humidistats.is_empty()
        && model.setpoint_managers.is_empty()
        && model.ideal_loads.is_empty()
        && model.zone_equipment.is_empty()
        && model.air_loops.is_empty()
        && model.plant_loops.is_empty()
        && model.outdoor_air_systems.is_empty()
        && model.infiltrations.is_empty()
        && model.mechanical_ventilations.is_empty()
        && model.shading_surfaces.is_empty()
        && model.space_lists.is_empty()
        && model.thermal_enclosures.is_empty()
        && model.adjacency_pairs.is_empty()
        && model.airflow_network.is_none()
        && model.electrical_load_centers.is_empty()
        && model.pv_systems.is_empty()
        && model.battery_storage.is_empty()
        && model.shw_systems.is_empty()
        && model.solar_thermal_systems.is_empty()
        && model.refrigeration_systems.is_empty()
        && model.water_systems.is_empty()
        && model.faults.is_empty()
        && model.output_variables.is_empty()
        && model.sizing_objects.is_empty()
        && model.daylight_zones.is_empty()
        && model.room_air_models.is_empty()
}

fn config_is_terminal_empty(config: &SimulationConfig) -> bool {
    config.weather.is_none() && config.schedules.constants.is_empty() && config.schedules.daily.is_empty() && config.schedules.weekly.is_empty() && config.schedules.annual.is_empty() && config.schedules.time_series.is_empty()
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
// #endregion 🔖️EnergyJob

// #region 🔖️Engine
/// ⚡️ Headless BEM simulation engine.
pub struct Engine;

impl Engine {
    /// ⚡️ Create the persistent operation used by interactive hosts and the batch adapter.
    pub fn job(model: Model, config: SimulationConfig) -> Result<EnergyJob, EnergyAdmissionRejected> {
        EnergyJob::new(Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 0), model, config)
    }

    /// ⚡️ Run full building energy simulation.
    pub fn run(model: Model, config: SimulationConfig) -> Result<Results, Error> {
        let mut job = Self::job(model, config).map_err(|_| Error::severe("energy numerical admission rejected"))?;
        let operation = job.operation.operation;
        let generation = job.operation.generation;
        let mut sequence = 0;
        let outcome = loop {
            let mut context = StepContext::new(operation, generation, semio_framework_job::StepBudget::new(32, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            match job.step(&mut context) {
                StepOutcome::PreviewReady(mut notice) => {
                    close_retained_payload(&mut notice);
                    if let Some(mut packet) = job.take_preview_packet(generation).map_err(|_| Error::severe("energy preview generation rejected"))? {
                        close_retained_payload(&mut packet.payload);
                    }
                }
                StepOutcome::CheckpointReady(mut checkpoint) => {
                    close_retained_payload(&mut checkpoint.state);
                    if let Some(mut lease) = job.take_checkpoint_packet(generation).map_err(|_| Error::severe("energy checkpoint generation rejected"))? {
                        close_retained_payload(&mut lease.packet_mut().payload);
                        job.ack_checkpoint_packet(lease).map_err(|_| Error::severe("energy checkpoint ACK rejected"))?;
                    }
                }
                terminal @ (StepOutcome::Complete(_) | StepOutcome::Fault(_) | StepOutcome::Cancelled) => break terminal,
                StepOutcome::Yield => {}
            }
        };
        let result = match outcome {
            StepOutcome::Complete(mut candidate) => {
                close_retained_payload(&mut candidate.state);
                close_retained_payload(&mut candidate.output);
                job.take_results().ok_or_else(|| Error::severe("energy job completed without results"))
            }
            StepOutcome::Fault(mut fault) => {
                close_retained_payload(&mut fault.detail);
                Err(Error::severe("energy simulation fault"))
            }
            StepOutcome::Cancelled => Err(Error::severe("energy simulation cancelled")),
            StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => Err(Error::severe("energy batch adapter stopped before a terminal outcome")),
        };
        InteractiveJob::begin_close(&mut job);
        loop {
            if matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                break;
            }
        }
        result
    }
}
// #endregion 🔖️Engine

fn close_retained_payload(payload: &mut semio_framework_job::RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

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

    fn build_checkpoint_packet(job: &mut EnergyJob) -> EnergyWirePacket {
        let operation = job.operation;
        job.start_wire(EnergyWireKind::Checkpoint, job.hour_index as u64, semio_framework_job::JobPayloadStream::CheckpointState).expect("checkpoint preflight");
        let mut sequence = 0;
        for _ in 0..32 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            if job.step_wire_build(&mut context).expect("checkpoint fragment") {
                return job.wire_ready.take().expect("sealed checkpoint packet");
            }
        }
        panic!("checkpoint packet did not seal within fixed field bound")
    }

    fn retained_test_packet(operation: Operation, identity: EnergyWireIdentity, bytes: &[u8]) -> EnergyWirePacket {
        let mut writer = semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CheckpointState);
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        let mut page = writer.admit_page(&mut context).expect("test page admission");
        page.write(bytes).expect("single fixed checkpoint page");
        page.commit();
        EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: writer.finish().expect("sealed test packet"), preview: None, reservation: None }
    }

    #[test]
    fn p7c2_wire_schema_maximum_plus_one_and_exact_restore_owner() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(91), Generation(7), 0x55aa);
        let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
        let packet = build_checkpoint_packet(&mut source);
        assert_eq!(packet.payload.len(), ENERGY_CHECKPOINT_BYTES);
        assert_eq!(packet.payload.page_count(), 1);
        let model = test_model_single_zone();
        let config = SimulationConfig::default();
        let pointer = model.zones.as_ptr();
        let census = EnergyNumericalCensus::observe(&model, &config).expect("restore census");
        let mut maximum = EnergyNumericalBounds::default().0;
        maximum.zones = census.zones - 1;
        let rejected = EnergyRestoreJob::admit(operation, model, config, packet, EnergyNumericalBounds(maximum)).expect_err("zone MAX+1 rejects before restore mount");
        assert_eq!(rejected.model.zones.as_ptr(), pointer);
        assert_eq!(rejected.reason, EnergyCheckpointRejectionReason::Numerical(EnergyNumericalDimension::Zones));
        let restore = rejected.retry(EnergyNumericalBounds::default()).expect("exact packet and Model+Config retry");
        drop(restore);
        let mut recovered = EnergyRestoreJob::recover_abandoned(operation).expect("drop requeues exact restore authority");
        let mut sequence = 0;
        for _ in 0..4 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            let _ = recovered.step(&mut context).expect("field decode");
        }
        drop(recovered);
        assert!(EnergyRestoreJob::recover_abandoned(Operation { generation: Generation(8), ..operation }).is_none());
        let mut recovered = EnergyRestoreJob::recover_abandoned(operation).expect("same generation recovers once");
        for _ in 0..32 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            if recovered.step(&mut context).expect("restore rebuild") {
                break;
            }
        }
        let install_context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        let mut restored = recovered.finish(&install_context).expect("restored authority");
        InteractiveJob::begin_close(&mut restored);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut restored, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                break;
            }
        }
        InteractiveJob::begin_close(&mut source);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut source, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                return;
            }
        }
        panic!("source wire authority did not close")
    }

    #[test]
    fn p7c2_live_schema_mutations_reject_before_restore_mount() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(94), Generation(12), 0xdead);
        let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
        let packet = build_checkpoint_packet(&mut source);
        let identity = packet.identity;
        let baseline = payload_bytes(packet.payload);
        let mutations: [fn(&mut Vec<u8>); 8] = [
            |bytes| bytes[0] ^= 0xff,
            |bytes| bytes[8] = bytes[8].wrapping_add(1),
            |bytes| bytes[10] = 0xff,
            |bytes| bytes[32] = bytes[32].wrapping_add(1),
            |bytes| bytes[56] = bytes[56].wrapping_add(1),
            |bytes| bytes[64] = bytes[64].wrapping_add(1),
            |bytes| bytes[72] = 2,
            |bytes| bytes[76] = 2,
        ];
        for mutate in mutations {
            let mut bytes = baseline.clone();
            mutate(&mut bytes);
            let packet = retained_test_packet(operation, identity, &bytes);
            let mut rejected = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect_err("hostile header/cap mutation");
            while !rejected.packet.terminal_is_empty() {
                let _ = rejected.packet.ack_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
        }
        let mut trailing = baseline.clone();
        trailing.push(0);
        let packet = retained_test_packet(operation, identity, &trailing);
        let mut rejected = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect_err("trailing byte mutation");
        while !rejected.packet.terminal_is_empty() {
            let _ = rejected.packet.ack_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        let mut ignored_digest = baseline;
        ignored_digest[156] ^= 0x80;
        let packet = retained_test_packet(operation, identity, &ignored_digest);
        let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("digest mutation passes bounded header admission");
        let mut sequence = 0;
        for _ in 0..64 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            assert!(!restore.step(&mut context).expect("digest mutation remains retained while replay proves mismatch"));
        }
        assert!(!restore.ready, "decoded numerical digest cannot be ignored by fresh-job restore");
        for offset in [132usize, 140, 148] {
            let mut ignored_count = ignored_digest.clone();
            ignored_count[156] ^= 0x80;
            ignored_count[offset] = 1;
            let packet = retained_test_packet(operation, identity, &ignored_count);
            let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("under-cap decoded count mutation passes bounded header admission");
            for _ in 0..64 {
                let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
                assert!(!restore.step(&mut context).expect("decoded count remains retained while replay proves mismatch"));
            }
            assert!(!restore.ready, "decoded table/history count cannot be cap-checked then discarded");
        }
        InteractiveJob::begin_close(&mut source);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut source, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                return;
            }
        }
        panic!("schema mutation source did not close")
    }

    #[test]
    fn p7c2_lossless_queue_saturation_retains_identity_and_fifo_order() {
        let mut queue = EnergyWireQueue::new(EnergyWireKind::Checkpoint);
        for sequence in 0..ENERGY_WIRE_QUEUE_SLOTS as u64 {
            let identity = EnergyWireIdentity { operation: 1, base_revision: 2, generation: 3, seed: 4, sequence };
            queue
                .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), preview: None, reservation: None })
                .expect("exact queue maximum");
        }
        let overflow_identity = EnergyWireIdentity { operation: 1, base_revision: 2, generation: 3, seed: 4, sequence: ENERGY_WIRE_QUEUE_SLOTS as u64 };
        let rejected = queue
            .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity: overflow_identity, payload: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), preview: None, reservation: None })
            .expect_err("queue MAX+1 retains packet");
        assert_eq!(rejected.identity, overflow_identity);
        let first = queue.take().expect("first lease");
        assert_eq!(first.identity().sequence, 0);
        queue.retry(first).expect("retry exact head");
        for sequence in 0..ENERGY_WIRE_QUEUE_SLOTS as u64 {
            let lease = queue.take().expect("FIFO lease");
            assert_eq!(lease.identity().sequence, sequence);
            queue.ack(lease).expect("empty packet ACK");
        }
    }

    #[test]
    fn p7c2_lossless_queue_drop_and_panic_recover_exact_in_flight_head() {
        let identity = EnergyWireIdentity { operation: 11, base_revision: 12, generation: 13, seed: 14, sequence: 15 };
        let mut queue = EnergyWireQueue::new(EnergyWireKind::Checkpoint);
        queue
            .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), preview: None, reservation: None })
            .expect("fixed queue accepts head");
        drop(queue.take().expect("drop lease"));
        let recovered = queue.take().expect("Drop republishes exact in-flight head");
        assert_eq!(recovered.identity(), identity);
        queue.retry(recovered).expect("retry returns exact head to original slot");
        let panic_lease = queue.take().expect("panic lease");
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _owned = panic_lease;
            panic!("hostile consumer panic");
        }));
        let recovered = queue.take().expect("panic Drop republishes exact head");
        assert_eq!(recovered.identity(), identity);
        queue.ack(recovered).expect("empty recovered packet ACKs once");
        assert_eq!(queue.len, 0);
    }

    #[test]
    fn p7c3_commit_lease_ack_is_the_exact_terminal_detach_witness() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(121), Generation(17), 0x7c3);
        let mut job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("terminal witness admission");
        job.stage = EnergyJobStage::Complete;
        let identity = job.wire_identity(1);
        job.publication
            .commits
            .push(EnergyWirePacket { kind: EnergyWireKind::Commit, identity, payload: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput), preview: None, reservation: None })
            .expect("prepared exact commit");

        let lease = job.take_commit_packet(operation.generation).expect("fresh generation").expect("exact final lease");
        let mut sequence = 0;
        let mut leased = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        assert_eq!(job.step(&mut leased), StepOutcome::Yield, "an unacknowledged exact lease must retain terminal ownership");
        assert_eq!(job.publication.commits.len, 1);
        assert!(job.publication.commits.in_flight.is_some());

        job.ack_commit_packet(lease).expect("exact empty commit ACK");
        assert_eq!(job.publication.commits.len, 0);
        assert!(job.publication.commits.in_flight.is_none());
        let mut acknowledged = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        let mut terminal = job.step(&mut acknowledged);
        assert!(matches!(terminal, StepOutcome::Complete(_)), "the exact ACK must make the numerical authority terminal");
        assert!(terminal.terminal_is_empty(), "the consumer retained and closed the only commit owner before terminal detach");
        assert!(matches!(terminal.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete));

        InteractiveJob::begin_close(&mut job);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut job, 1, 4), semio_framework_job::InteractiveJobCloseStep::Complete) {
                return;
            }
        }
        panic!("ACK-terminal Energy authority did not close")
    }

    #[test]
    fn p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(96), Generation(15), 0xcafe);
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let mut job = EnergyJob::new(operation, test_model_single_zone(), config).expect("preview source admission");
        let mut sequence = 0;
        for _ in 0..50_000 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            match job.step(&mut context) {
                StepOutcome::PreviewReady(mut notice) => {
                    close_retained_payload(&mut notice);
                    let projected = job.preview().expect("installed packet owns typed preview projection").clone();
                    let mut packet = job.take_preview_packet(operation.generation).expect("fresh preview generation").expect("canonical preview packet");
                    assert_eq!(packet.payload.len(), ENERGY_WIRE_HEADER_BYTES + 20);
                    let decoded = decode_preview_packet(&packet).expect("SMENERGY preview schema");
                    assert_eq!(packet.preview(), Some(&decoded));
                    assert_eq!(projected, decoded);
                    close_retained_payload(&mut packet.payload);
                    if decoded.facility_electricity_kwh > 0.0 {
                        return;
                    }
                }
                StepOutcome::CheckpointReady(mut checkpoint) => {
                    close_retained_payload(&mut checkpoint.state);
                    let mut lease = job.take_checkpoint_packet(operation.generation).expect("checkpoint generation").expect("checkpoint lease");
                    close_retained_payload(&mut lease.packet_mut().payload);
                    job.ack_checkpoint_packet(lease).expect("checkpoint ACK");
                }
                StepOutcome::Yield => {}
                StepOutcome::Fault(fault) => panic!("preview source faulted: {fault:?}"),
                StepOutcome::Cancelled => panic!("preview source cancelled"),
                StepOutcome::Complete(_) => break,
            }
        }
        panic!("canonical preview never exposed a substantive retained facility total")
    }

    #[test]
    fn p7c2_restore_stale_step_and_install_preserve_exact_replay_authority() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(95), Generation(14), 0xbeef);
        let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
        let packet = build_checkpoint_packet(&mut source);
        let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("restore admission");
        let mut sequence = 0;
        let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        assert_eq!(restore.step(&mut stale), Err(EnergyWireRejection::Identity));
        assert_eq!(restore.field, 0);
        for _ in 0..64 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            if restore.step(&mut context).expect("bounded replay step") {
                break;
            }
        }
        assert!(restore.ready);
        let stale_install = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        let restore = restore.finish(&stale_install).expect_err("stale install retains exact replay authority");
        let install = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        let mut restored = restore.finish(&install).expect("fresh generation installs exact replay authority");
        InteractiveJob::begin_close(&mut restored);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut restored, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                break;
            }
        }
        InteractiveJob::begin_close(&mut source);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut source, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                return;
            }
        }
        panic!("stale restore fixture owners did not close")
    }

    #[test]
    fn p7c2_cancel_deadline_and_stale_generation_gate_wire_before_mutation() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(92), Generation(5), 99);
        let mut job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("admission");
        job.start_wire(EnergyWireKind::Checkpoint, 0, semio_framework_job::JobPayloadStream::CheckpointState).expect("wire start");
        let before = job.wire_build.as_ref().expect("wire").field;
        let cancel = CancelToken::root_now();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_us, &mut sequence);
        assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!(job.wire_build.as_ref().expect("wire").field, before);
        let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(0, 0), CancelToken::root_now(), default_now_us, &mut sequence);
        assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
        assert_eq!(job.wire_build.as_ref().expect("wire").field, before);
        assert!(matches!(job.take_checkpoint_packet(Generation(6)), Err(EnergyWireRejection::Identity)));
    }

    #[test]
    fn p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let operation = Operation::new(allocate_operation_id(), RevisionId(93), Generation(9), 777);
        let mut original = EnergyJob::new(operation, model.clone(), config.clone()).expect("original admission");
        let mut sequence = 0;
        let packet = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            match original.step(&mut context) {
                StepOutcome::PreviewReady(mut notice) => {
                    close_retained_payload(&mut notice);
                    let mut packet = original.take_preview_packet(operation.generation).expect("fresh preview").expect("preview packet");
                    close_retained_payload(&mut packet.payload);
                }
                StepOutcome::CheckpointReady(mut checkpoint) => {
                    close_retained_payload(&mut checkpoint.state);
                    let packet = build_checkpoint_packet(&mut original);
                    let mut lease = original.take_checkpoint_packet(operation.generation).expect("fresh checkpoint").expect("checkpoint lease");
                    close_retained_payload(&mut lease.packet_mut().payload);
                    original.ack_checkpoint_packet(lease).expect("checkpoint ACK advances only after close");
                    break packet;
                }
                StepOutcome::Yield => {}
                other => panic!("checkpoint expected before terminal: {other:?}"),
            }
        };
        let mut restore = EnergyRestoreJob::admit(operation, model, config, packet, EnergyNumericalBounds::default()).expect("restore admission");
        for _ in 0..100_000 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            if restore.step(&mut context).expect("restore field/rebuild") {
                break;
            }
        }
        let install_context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        let restored = restore.finish(&install_context).expect("restore finish");
        let (_, _, _, original_bytes, _) = drive_energy_job_with_fuel(original, 1);
        let (_, _, _, restored_bytes, _) = drive_energy_job_with_fuel(restored, 4);
        assert_eq!(restored_bytes, original_bytes);
    }

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
            let mut context = StepContext::new(operation, generation, semio_framework_job::StepBudget::new(fuel, u64::MAX), cancel.clone(), default_now_us, &mut preview_sequence);
            let outcome = job.step(&mut context);
            let elapsed = start.elapsed();
            worst = worst.max(elapsed);
            match outcome {
                StepOutcome::PreviewReady(mut notice) => {
                    close_retained_payload(&mut notice);
                    let mut packet = job.take_preview_packet(generation).expect("fresh preview generation").expect("preview packet");
                    previews.push(packet.preview().expect("typed projection belongs to canonical preview packet").clone());
                    close_retained_payload(&mut packet.payload);
                }
                StepOutcome::CheckpointReady(mut checkpoint) => {
                    close_retained_payload(&mut checkpoint.state);
                    let mut lease = job.take_checkpoint_packet(generation).expect("fresh checkpoint generation").expect("checkpoint lease");
                    close_retained_payload(&mut lease.packet_mut().payload);
                    job.ack_checkpoint_packet(lease).expect("checkpoint ACK");
                    checkpoints += 1;
                }
                StepOutcome::Complete(candidate) => return (job, previews, checkpoints, payload_bytes(candidate.output), worst),
                StepOutcome::Fault(fault) => panic!("energy job faulted: {fault:?}"),
                StepOutcome::Cancelled => panic!("energy job unexpectedly cancelled"),
                StepOutcome::Yield => {}
            }
        }
        panic!("energy job did not complete within the deterministic step bound")
    }

    fn payload_bytes(mut payload: semio_framework_job::RetainedJobPayload) -> Vec<u8> {
        let mut bytes = Vec::new();
        for index in 0..payload.page_count() {
            bytes.extend_from_slice(payload.page(index).expect("retained output page"));
        }
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        bytes
    }

    #[test]
    fn engine_runs_single_zone() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 3, environment: SimulationEnvironment::WeatherRunPeriod, ..Default::default() };
        let results = Engine::run(model, config).unwrap();
        assert!(results.run_metadata.timesteps > 0);
        assert!(results.meters.facility_total_kwh(FuelType::Electricity) >= 0.0);
    }

    #[test]
    fn engine_deterministic_repeatability() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let r1 = Engine::run(model.clone(), config.clone()).unwrap();
        let r2 = Engine::run(model, config).unwrap();
        assert_eq!(r1.run_metadata.timesteps, r2.run_metadata.timesteps);
        assert!((r1.meters.facility_total_kwh(FuelType::Electricity) - r2.meters.facility_total_kwh(FuelType::Electricity)).abs() < 1e-3);
    }

    #[test]
    fn ashrae_140_case600_base() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let results = Engine::run(model, config).unwrap();
        let temps = results.time_series.get("Zone Air Temperature [Zone1]");
        assert!(temps.is_some());
    }

    #[test]
    fn invalid_model_rejected() {
        let model = Model::default();
        assert!(Engine::run(model, SimulationConfig::default()).is_err());
    }

    #[test]
    fn energy_conservation_order_of_magnitude() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let results = Engine::run(model, config).unwrap();
        let total_kwh = results.meters.facility_total_kwh(FuelType::Electricity);
        assert!(total_kwh < 1_000_000.0);
    }

    #[test]
    fn full_topology_e2e() {
        let model = test_model_full_topology();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let results = Engine::run(model, config).unwrap();
        assert!(results.run_metadata.timesteps >= 48);
        assert!(results.summaries.annual_energy.len() >= 3);
    }

    #[test]
    fn hvac_bestest_heating_day() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let results = Engine::run(model, config).unwrap();
        assert_eq!(results.run_metadata.timesteps, 24);
        assert!(results.time_series.get("Zone Air Temperature [Zone1]").is_some());
    }

    #[test]
    fn run_period_honors_calendar() {
        let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 7, year: 2026 };
        assert_eq!(period.total_hours(), 168);
        let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, warmup_days: 0, ..Default::default() };
        let model = test_model_single_zone();
        let results = Engine::run(model, config).unwrap();
        assert_eq!(results.run_metadata.timesteps, 168);
    }

    #[test]
    fn energy_job_previews_checkpoints_and_commits_bounded_steps() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let (mut job, previews, checkpoints, output, worst) = drive_energy_job(Engine::job(model, config).expect("energy admission"));
        assert!(!previews.is_empty());
        assert!(previews.windows(2).all(|pair| pair[0].sequence < pair[1].sequence));
        assert!(previews.iter().any(|preview| preview.tier == EnergyQualityTier::SteadyStateEstimate));
        assert_eq!(previews.last().map(|preview| preview.tier), Some(EnergyQualityTier::Final));
        assert!(checkpoints > 0, "P7c2 publishes retained lossless checkpoints");
        assert!(output.starts_with(&ENERGY_WIRE_MAGIC));
        assert!(worst < std::time::Duration::from_millis(8), "worst energy step was {worst:?}");
        let results = job.take_results().expect("completed job retains typed results for the batch adapter");
        assert_eq!(results.run_metadata.timesteps, 24);
    }

    #[test]
    fn energy_job_cancellation_and_freshness_precede_mutation() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, ..Default::default() };
        let operation = Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 11);
        let mut cancelled_job = EnergyJob::new(operation, model.clone(), config.clone()).expect("energy admission");
        let cancel = CancelToken::root_now();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_us, &mut sequence);
        assert_eq!(cancelled_job.step(&mut context), StepOutcome::Cancelled);
        assert_eq!(cancelled_job.stage(), EnergyJobStage::Validate);

        let mut stale_job = EnergyJob::new(operation, model, config).expect("energy admission");
        let mut stale_sequence = 0;
        let mut stale_context = StepContext::new(operation.operation, Generation(4), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut stale_sequence);
        assert!(matches!(stale_job.step(&mut stale_context), StepOutcome::Fault(_)));
        assert_eq!(stale_job.stage(), EnergyJobStage::Validate);
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

    #[test]
    fn p7c1_exact_max_rejection_preserves_and_retries_same_owner() {
        let model = test_model_full_topology();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let census = EnergyNumericalCensus::observe(&model, &config).expect("checked census");
        let operation = Operation::new(allocate_operation_id(), RevisionId(17), Generation(9), 31);
        assert!(EnergyJob::admit(operation, model.clone(), config.clone(), EnergyNumericalBounds(census)).is_ok(), "exact observed MAX must succeed");

        let pointer = model.zones.as_ptr();
        let mut maximum = census;
        maximum.zones -= 1;
        let rejected = EnergyJob::admit(operation, model, config, EnergyNumericalBounds(maximum)).expect_err("MAX+1 zone owner must reject before transfer");
        assert_eq!(rejected.dimension, EnergyNumericalDimension::Zones);
        assert_eq!(rejected.model.zones.as_ptr(), pointer, "rejected authority retains the exact allocation identity");
        assert!(rejected.retry(EnergyNumericalBounds(census)).is_ok(), "exact rejected owner retries after sufficient credit");
    }

    #[test]
    fn p7c1_every_independent_maximum_plus_one_dimension_rejects() {
        let census = EnergyNumericalCensus {
            zones: 1,
            surfaces: 1,
            fenestrations: 1,
            people: 1,
            lighting: 1,
            equipment: 1,
            infiltrations: 1,
            airflow_nodes: 1,
            airflow_links: 1,
            mechanical_ventilations: 1,
            thermostats: 1,
            humidistats: 1,
            ideal_loads: 1,
            faults: 1,
            zone_equipment: 1,
            plant_loops: 1,
            plant_equipment: 1,
            pv_systems: 1,
            batteries: 1,
            service_hot_water: 1,
            refrigeration: 1,
            water: 1,
            weather_records: 1,
            timesteps: 1,
            meters: 1,
            series: 1,
            samples: 1,
            history_values: 1,
            summary_rows: 1,
            identifier_bytes: 1,
            observed_items: 1,
            observed_bytes: 1,
            pages: 1,
            operations: 1,
            process_jobs: 1,
        };
        macro_rules! rejects {
            ($field:ident, $variant:ident) => {{
                let mut maximum = census;
                maximum.$field = 0;
                assert_eq!(census.first_exceeded(maximum), Some(EnergyNumericalDimension::$variant));
            }};
        }
        rejects!(zones, Zones);
        rejects!(surfaces, Surfaces);
        rejects!(fenestrations, Fenestrations);
        rejects!(people, People);
        rejects!(lighting, Lighting);
        rejects!(equipment, Equipment);
        rejects!(infiltrations, Infiltrations);
        rejects!(airflow_nodes, AirflowNodes);
        rejects!(airflow_links, AirflowLinks);
        rejects!(mechanical_ventilations, MechanicalVentilations);
        rejects!(thermostats, Thermostats);
        rejects!(humidistats, Humidistats);
        rejects!(ideal_loads, IdealLoads);
        rejects!(faults, Faults);
        rejects!(zone_equipment, ZoneEquipment);
        rejects!(plant_loops, PlantLoops);
        rejects!(plant_equipment, PlantEquipment);
        rejects!(pv_systems, PvSystems);
        rejects!(batteries, Batteries);
        rejects!(service_hot_water, ServiceHotWater);
        rejects!(refrigeration, Refrigeration);
        rejects!(water, Water);
        rejects!(weather_records, WeatherRecords);
        rejects!(timesteps, Timesteps);
        rejects!(meters, Meters);
        rejects!(series, Series);
        rejects!(samples, Samples);
        rejects!(history_values, HistoryValues);
        rejects!(summary_rows, SummaryRows);
        rejects!(identifier_bytes, IdentifierBytes);
        rejects!(observed_items, ObservedItems);
        rejects!(observed_bytes, ObservedBytes);
        rejects!(pages, Pages);
        rejects!(operations, Operations);
        rejects!(process_jobs, ProcessJobs);
    }

    #[test]
    fn p7c1_live_owner_capacity_mutations_reject_the_declared_dimension() {
        macro_rules! capacity_law {
            ($field:ident, $census_field:ident, $variant:ident) => {{
                let mut model = test_model_full_topology();
                let config = SimulationConfig::default();
                model.$field.shrink_to_fit();
                let before = EnergyNumericalCensus::observe(&model, &config).expect("baseline census");
                model.$field.reserve_exact(1);
                let after = EnergyNumericalCensus::observe(&model, &config).expect("mutated census");
                assert!(after.$census_field > before.$census_field, "live mutation did not increase {}", stringify!($census_field));
                let mut maximum = EnergyNumericalBounds::default().0;
                maximum.$census_field = before.$census_field;
                let rejected = EnergyJob::admit(Operation::new(allocate_operation_id(), RevisionId(52), Generation(6), 211), model, config, EnergyNumericalBounds(maximum)).expect_err("live MAX+1 owner must reject");
                assert_eq!(rejected.dimension, EnergyNumericalDimension::$variant);
            }};
        }
        capacity_law!(zones, zones, Zones);
        capacity_law!(surfaces, surfaces, Surfaces);
        capacity_law!(fenestrations, fenestrations, Fenestrations);
        capacity_law!(people, people, People);
        capacity_law!(lighting, lighting, Lighting);
        capacity_law!(equipment, equipment, Equipment);
        capacity_law!(infiltrations, infiltrations, Infiltrations);
        capacity_law!(mechanical_ventilations, mechanical_ventilations, MechanicalVentilations);
        capacity_law!(thermostats, thermostats, Thermostats);
        capacity_law!(humidistats, humidistats, Humidistats);
        capacity_law!(ideal_loads, ideal_loads, IdealLoads);
        capacity_law!(faults, faults, Faults);
        capacity_law!(zone_equipment, zone_equipment, ZoneEquipment);
        capacity_law!(plant_loops, plant_loops, PlantLoops);
        capacity_law!(pv_systems, pv_systems, PvSystems);
        capacity_law!(battery_storage, batteries, Batteries);
        capacity_law!(shw_systems, service_hot_water, ServiceHotWater);
        capacity_law!(refrigeration_systems, refrigeration, Refrigeration);
        capacity_law!(water_systems, water, Water);
    }

    #[test]
    fn p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one() {
        let model = test_model_single_zone();
        let mut records = vec![design_day_hour(0, -10.0), design_day_hour(1, -9.0)];
        records.shrink_to_fit();
        let logical_records = records.len().max(1);
        records.reserve_exact(3);
        assert!(records.capacity() > logical_records, "reserve-only mutation must increase owned weather backing");
        let pointer = records.as_ptr();
        let config = SimulationConfig { weather: Some(crate::site::EpwWeather { location: "fixed".into(), latitude_deg: 0.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, records }), ..Default::default() };
        let census = EnergyNumericalCensus::observe(&model, &config).expect("weather census");
        assert_eq!(census.weather_records, config.weather.as_ref().expect("weather").records.capacity());
        assert!(census.weather_records > logical_records, "census must not collapse backing capacity to len/max1");
        let operation = Operation::new(allocate_operation_id(), RevisionId(63), Generation(12), 251);
        let mut maximum = EnergyNumericalBounds::default().0;
        maximum.weather_records = logical_records;
        let rejected = EnergyJob::admit(operation, model, config, EnergyNumericalBounds(maximum)).expect_err("weather MAX+1 rejects exact owner");
        assert_eq!(rejected.dimension, EnergyNumericalDimension::WeatherRecords);
        assert_eq!(rejected.config.weather.as_ref().expect("weather owner").records.as_ptr(), pointer);
        let mut job = rejected.retry(EnergyNumericalBounds::default()).expect("weather owner retry");
        assert_eq!(job.weather.capacity(), census.weather_records);
        assert_eq!(job.weather.len(), 0);
        job.stage = EnergyJobStage::ResolveWeather;
        let mut preview_sequence = 0;
        for _ in 0..2 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut preview_sequence);
            assert_eq!(job.step(&mut context), StepOutcome::Yield);
        }
        assert_eq!(job.weather.len(), 2);
        assert_eq!(job.weather.capacity(), census.weather_records);
        let before_records = [*job.weather.get_index(0).expect("weather zero"), *job.weather.get_index(1).expect("weather one")];
        for index in logical_records..census.weather_records {
            assert!(job.weather.insert_stable(index, design_day_hour(index as u32, -8.0)).is_ok());
        }
        assert!(job.weather.insert_stable(census.weather_records, design_day_hour(census.weather_records as u32, -8.0)).is_err());
        assert_eq!(job.weather.len(), census.weather_records);
        assert_eq!(job.weather.capacity(), census.weather_records);
        assert_eq!(*job.weather.get_index(0).expect("weather zero"), before_records[0]);
        assert_eq!(*job.weather.get_index(1).expect("weather one"), before_records[1]);
        job.weather_cursor = 1;
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut preview_sequence);
        let mut outcome = job.step(&mut context);
        assert!(matches!(outcome, StepOutcome::Fault(_)));
        assert_eq!(job.weather_fault, Some(WeatherFault::SlotRejected));
        while !matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
        InteractiveJob::begin_close(&mut job);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                return;
            }
        }
        panic!("fixed weather authority did not close")
    }

    #[test]
    fn p7c1_weather_reserve_only_capacity_is_independently_charged_to_items() {
        let model = test_model_single_zone();
        let mut config =
            SimulationConfig { weather: Some(crate::site::EpwWeather { location: "items".into(), latitude_deg: 0.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, records: vec![design_day_hour(0, -10.0)] }), ..Default::default() };
        config.weather.as_mut().expect("weather").records.shrink_to_fit();
        let before = EnergyNumericalCensus::observe(&model, &config).expect("baseline weather census");
        config.weather.as_mut().expect("weather").records.reserve_exact(7);
        let after = EnergyNumericalCensus::observe(&model, &config).expect("reserve-only weather census");
        assert!(after.weather_records > before.weather_records);
        let weather_delta = after.weather_records - before.weather_records;
        assert_eq!(after.observed_items - before.observed_items, weather_delta);
        assert_eq!(after.observed_bytes - before.observed_bytes, weather_delta * (std::mem::size_of::<WeatherRecord>() + std::mem::size_of::<Option<(usize, WeatherRecord)>>()));
        assert_eq!(after.pages, (after.observed_bytes + 16_383) / 16_384);
        let pointer = config.weather.as_ref().expect("weather").records.as_ptr();
        let mut maximum = EnergyNumericalBounds::default().0;
        maximum.observed_items = after.observed_items - 1;
        let operation = Operation::new(allocate_operation_id(), RevisionId(64), Generation(13), 257);
        let rejected = EnergyJob::admit(operation, model, config, EnergyNumericalBounds(maximum)).expect_err("weather backing item MAX+1 rejects before mount");
        assert_eq!(rejected.dimension, EnergyNumericalDimension::ObservedItems);
        assert_eq!(rejected.config.weather.as_ref().expect("weather").records.as_ptr(), pointer);
        let mut job = rejected.retry(EnergyNumericalBounds::default()).expect("weather item owner retry");
        InteractiveJob::begin_close(&mut job);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                return;
            }
        }
        panic!("weather item retry authority did not close")
    }

    #[test]
    fn p7c1_one_fuel_chronology_is_identical_at_one_two_four_and_default_grants() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let operation = Operation::new(allocate_operation_id(), RevisionId(21), Generation(4), 91);
        let output = |fuel| drive_energy_job_with_fuel(EnergyJob::new(operation, model.clone(), config.clone()).expect("energy admission"), fuel).3;
        assert_eq!(output(1), output(2));
        assert_eq!(output(1), output(4));
        assert_eq!(output(1), output(32));
    }

    #[test]
    fn p7c1_close_releases_no_more_than_one_owner_or_character_per_grant() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(1), 1);
        let mut job = EnergyJob::new(operation, test_model_full_topology(), SimulationConfig::default()).expect("energy admission");
        InteractiveJob::begin_close(&mut job);
        for _ in 0..100_000 {
            match InteractiveJob::close_step(&mut job, 1, 4) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= 4);
                }
                semio_framework_job::InteractiveJobCloseStep::Complete => {
                    assert!(InteractiveJob::terminal_is_empty(&job));
                    return;
                }
                semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("Energy close cannot block"),
            }
        }
        panic!("Energy close did not retire within its admitted bound")
    }

    #[test]
    fn p7c1_direct_drop_requeues_exact_generation_and_resumes_partial_close() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(33), Generation(7), 101);
        let mut job = EnergyJob::new(operation, test_model_full_topology(), SimulationConfig::default()).expect("energy admission");
        let retained_pointer = job.model.zones.as_ptr();
        InteractiveJob::begin_close(&mut job);
        assert!(matches!(InteractiveJob::close_step(&mut job, 1, 4), semio_framework_job::InteractiveJobCloseStep::Pending { .. }));
        drop(job);

        let mut recovered = EnergyJob::recover_abandoned(operation).expect("same generation abandonment authority");
        assert_eq!(recovered.model.zones.as_ptr(), retained_pointer);
        assert!(EnergyJob::recover_abandoned(Operation { generation: Generation(8), ..operation }).is_none());
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut recovered, 1, 4), semio_framework_job::InteractiveJobCloseStep::Complete) {
                assert!(InteractiveJob::terminal_is_empty(&recovered));
                return;
            }
        }
        panic!("recovered Energy close did not retire within its admitted bound")
    }

    #[test]
    fn p7c1_panic_unwind_requeues_the_same_incomplete_authority_once() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(34), Generation(11), 131);
        let job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("energy admission");
        let retained_pointer = job.model.zones.as_ptr();
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _authority = job;
            panic!("hostile Energy session unwind");
        }));
        assert!(unwind.is_err());
        let mut recovered = EnergyJob::recover_abandoned(operation).expect("panic requeued exact authority");
        assert_eq!(recovered.model.zones.as_ptr(), retained_pointer);
        assert!(EnergyJob::recover_abandoned(operation).is_none(), "recovery is single-owner");
        InteractiveJob::begin_close(&mut recovered);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut recovered, 1, 4), semio_framework_job::InteractiveJobCloseStep::Complete) {
                return;
            }
        }
        panic!("panic-recovered Energy authority did not close")
    }

    #[test]
    fn p7c1_cancel_and_deadline_gate_every_declared_numerical_substage_before_mutation() {
        let stages = [
            EnergyJobStage::Validate,
            EnergyJobStage::ResolveWeather,
            EnergyJobStage::Precompute,
            EnergyJobStage::InitializeZones,
            EnergyJobStage::InitializeSurfaces,
            EnergyJobStage::InitializeWarmupHistory,
            EnergyJobStage::WarmupTimestep,
            EnergyJobStage::WarmupConvergence,
            EnergyJobStage::StartRun,
            EnergyJobStage::RunZoneTimestep,
            EnergyJobStage::AggregateZone,
            EnergyJobStage::AggregateFacility,
            EnergyJobStage::PublishTimestep,
            EnergyJobStage::Finalize,
            EnergyJobStage::Size,
            EnergyJobStage::FinalizeSummaries,
            EnergyJobStage::FinalizeMetrics,
            EnergyJobStage::FinalizeEconomics,
            EnergyJobStage::BuildResults,
            EnergyJobStage::PublishFinal,
            EnergyJobStage::EncodeOutput,
            EnergyJobStage::Complete,
        ];
        let operation = Operation::new(allocate_operation_id(), RevisionId(41), Generation(5), 151);
        let mut job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("energy admission");
        for stage in stages {
            job.stage = stage;
            let before = job.numerical_cursor_signature();
            let cancel = CancelToken::root_now();
            cancel.cancel_now();
            let mut sequence = 0;
            let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_us, &mut sequence);
            assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
            assert_eq!(job.numerical_cursor_signature(), before, "cancel mutated {stage:?}");

            let mut deadline_sequence = 0;
            let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(0, 0), CancelToken::root_now(), default_now_us, &mut deadline_sequence);
            assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
            assert_eq!(job.numerical_cursor_signature(), before, "deadline mutated {stage:?}");
        }
        InteractiveJob::begin_close(&mut job);
        for _ in 0..100_000 {
            if matches!(InteractiveJob::close_step(&mut job, 1, 4), semio_framework_job::InteractiveJobCloseStep::Complete) {
                return;
            }
        }
        panic!("gated Energy job did not close")
    }

    #[test]
    fn p7c1_live_nested_authorities_gate_cancel_deadline_and_stale_before_mutation() {
        use crate::kernel::{P7C1_PLANT_STAGES, P7C1_SCHEDULE_LOOKUP_STAGES, P7C1_SYSTEM_SUBSTEP_STAGES, P7C1_TIMESTEP_BUILDER_STAGES, P7C1_TIMESTEP_STAGES, P7C1_ZONE_PREPARATION_STAGES};
        use crate::precompute::{P7C1_PRECOMPUTE_STAGES, P7C1_SURFACE_PRECOMPUTE_STAGES};
        use crate::sizing::P7C1_SIZING_STAGES;

        fn assert_gates(job: &mut EnergyJob, operation: Operation) {
            let before = job.numerical_cursor_signature();
            let cancel = CancelToken::root_now();
            cancel.cancel_now();
            let mut sequence = 0;
            let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_us, &mut sequence);
            assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
            assert_eq!(job.numerical_cursor_signature(), before);

            let mut deadline_sequence = 0;
            let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(0, 0), CancelToken::root_now(), default_now_us, &mut deadline_sequence);
            assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
            assert_eq!(job.numerical_cursor_signature(), before);

            let mut stale_sequence = 0;
            let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut stale_sequence);
            let mut outcome = job.step(&mut stale);
            assert!(matches!(outcome, StepOutcome::Fault(_)));
            assert_eq!(job.numerical_cursor_signature(), before);
            while !matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
        }

        let operation = Operation::new(allocate_operation_id(), RevisionId(61), Generation(9), 241);
        let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let mut job = EnergyJob::new(operation, test_model_full_topology(), config).expect("live nested gate admission");
        let validation_stage = job.validation.stage;
        for stage in P7C1_VALIDATION_STAGES {
            job.validation.stage = stage;
            assert_gates(&mut job, operation);
        }
        job.validation.stage = validation_stage;
        let finalization_stage = job.finalization.stage;
        for stage in P7C1_FINALIZATION_STAGES {
            job.finalization.stage = stage;
            assert_gates(&mut job, operation);
        }
        job.finalization.stage = finalization_stage;
        let result_stage = job.result_build.stage;
        for stage in P7C1_RESULT_BUILD_STAGES {
            job.result_build.stage = stage;
            assert_gates(&mut job, operation);
        }
        job.result_build.stage = result_stage;
        let initialize_backing_stage = job.initialize_backing_stage;
        for stage in 0..=2 {
            job.initialize_backing_stage = stage;
            assert_gates(&mut job, operation);
        }
        job.initialize_backing_stage = initialize_backing_stage;
        let run_backing_stage = job.run_backing_stage;
        for stage in 0..=6 {
            job.run_backing_stage = stage;
            assert_gates(&mut job, operation);
        }
        job.run_backing_stage = run_backing_stage;
        let encode_section = job.encode_section;
        for section in 0..=5 {
            job.encode_section = section;
            assert_gates(&mut job, operation);
        }
        job.encode_section = encode_section;
        let mut preview_sequence = 0;
        let mut precompute_gated = false;
        let mut surface_precompute_gated = false;
        let mut timestep_builder_gated = false;
        let mut timestep_gated = false;
        let mut zone_gated = false;
        let mut system_gated = false;
        let mut plant_gated = false;
        let mut schedule_gated = false;
        let mut warmup_gated = false;
        let mut aggregate_zone_gated = false;
        let mut aggregate_facility_gated = false;
        let mut sizing_gated = false;

        for _ in 0..200_000 {
            if !precompute_gated && job.precompute.is_some() {
                let original = job.precompute.as_ref().expect("precompute").stage();
                for stage in P7C1_PRECOMPUTE_STAGES {
                    job.precompute.as_mut().expect("precompute").set_stage_for_gate(stage);
                    assert_gates(&mut job, operation);
                }
                job.precompute.as_mut().expect("precompute").set_stage_for_gate(original);
                precompute_gated = true;
            }
            if !surface_precompute_gated && job.precompute.as_ref().and_then(PrecomputeBuilder::surface_stage_for_gate).is_some() {
                let original = job.precompute.as_ref().and_then(PrecomputeBuilder::surface_stage_for_gate).expect("surface precompute stage");
                for stage in P7C1_SURFACE_PRECOMPUTE_STAGES {
                    assert!(job.precompute.as_mut().expect("precompute").set_surface_stage_for_gate(stage));
                    assert_gates(&mut job, operation);
                }
                job.precompute.as_mut().expect("precompute").set_surface_stage_for_gate(original);
                surface_precompute_gated = true;
            }
            if !timestep_builder_gated && job.timestep_builder.is_some() {
                let original = job.timestep_builder.as_ref().expect("timestep builder").stage_for_gate();
                for stage in P7C1_TIMESTEP_BUILDER_STAGES {
                    job.timestep_builder.as_mut().expect("timestep builder").set_stage_for_gate(stage);
                    assert_gates(&mut job, operation);
                }
                job.timestep_builder.as_mut().expect("timestep builder").set_stage_for_gate(original);
                timestep_builder_gated = true;
            }
            if job.timestep_work.is_some() {
                if !timestep_gated {
                    let original = job.timestep_work.as_ref().expect("timestep").stage();
                    for stage in P7C1_TIMESTEP_STAGES {
                        job.timestep_work.as_mut().expect("timestep").set_stage_for_gate(stage);
                        assert_gates(&mut job, operation);
                    }
                    job.timestep_work.as_mut().expect("timestep").set_stage_for_gate(original);
                    timestep_gated = true;
                }
                if !zone_gated && job.timestep_work.as_ref().and_then(TimestepWork::zone_preparation_stage).is_some() {
                    let original = job.timestep_work.as_ref().and_then(TimestepWork::zone_preparation_stage).expect("zone stage");
                    for stage in P7C1_ZONE_PREPARATION_STAGES {
                        assert!(job.timestep_work.as_mut().expect("timestep").set_zone_preparation_stage_for_gate(stage));
                        assert_gates(&mut job, operation);
                    }
                    job.timestep_work.as_mut().expect("timestep").set_zone_preparation_stage_for_gate(original);
                    zone_gated = true;
                }
                if !system_gated && job.timestep_work.as_ref().and_then(TimestepWork::system_substep_stage).is_some() {
                    let original = job.timestep_work.as_ref().and_then(TimestepWork::system_substep_stage).expect("system stage");
                    for stage in P7C1_SYSTEM_SUBSTEP_STAGES {
                        assert!(job.timestep_work.as_mut().expect("timestep").set_system_substep_stage_for_gate(stage));
                        assert_gates(&mut job, operation);
                    }
                    job.timestep_work.as_mut().expect("timestep").set_system_substep_stage_for_gate(original);
                    system_gated = true;
                }
                if !plant_gated && job.timestep_work.as_ref().and_then(TimestepWork::plant_stage).is_some() {
                    let original = job.timestep_work.as_ref().and_then(TimestepWork::plant_stage).expect("plant stage");
                    for stage in P7C1_PLANT_STAGES {
                        assert!(job.timestep_work.as_mut().expect("timestep").set_plant_stage_for_gate(stage));
                        assert_gates(&mut job, operation);
                    }
                    job.timestep_work.as_mut().expect("timestep").set_plant_stage_for_gate(original);
                    plant_gated = true;
                }
                if !schedule_gated && job.timestep_work.as_ref().and_then(TimestepWork::schedule_lookup_stage).is_some() {
                    let original = job.timestep_work.as_ref().and_then(TimestepWork::schedule_lookup_stage).expect("schedule stage");
                    for stage in P7C1_SCHEDULE_LOOKUP_STAGES {
                        assert!(job.timestep_work.as_mut().expect("timestep").set_schedule_lookup_stage_for_gate(stage));
                        assert_gates(&mut job, operation);
                    }
                    job.timestep_work.as_mut().expect("timestep").set_schedule_lookup_stage_for_gate(original);
                    schedule_gated = true;
                }
            }
            if !warmup_gated && job.warmup_convergence.is_some() {
                let original = job.warmup_convergence.as_ref().expect("warmup convergence").stage;
                for stage in P7C1_WARMUP_CONVERGENCE_STAGES {
                    job.warmup_convergence.as_mut().expect("warmup convergence").stage = stage;
                    assert_gates(&mut job, operation);
                }
                job.warmup_convergence.as_mut().expect("warmup convergence").stage = original;
                warmup_gated = true;
            }
            if !aggregate_zone_gated && job.aggregate_zone_work.is_some() {
                let original = job.aggregate_zone_work.as_ref().expect("zone aggregate").stage;
                for stage in P7C1_AGGREGATE_STAGES {
                    job.aggregate_zone_work.as_mut().expect("zone aggregate").stage = stage;
                    assert_gates(&mut job, operation);
                }
                job.aggregate_zone_work.as_mut().expect("zone aggregate").stage = original;
                aggregate_zone_gated = true;
            }
            if !aggregate_facility_gated && job.aggregate_facility_work.is_some() {
                let original = job.aggregate_facility_work.as_ref().expect("facility aggregate").stage;
                for stage in P7C1_AGGREGATE_STAGES {
                    job.aggregate_facility_work.as_mut().expect("facility aggregate").stage = stage;
                    assert_gates(&mut job, operation);
                }
                job.aggregate_facility_work.as_mut().expect("facility aggregate").stage = original;
                aggregate_facility_gated = true;
            }
            if !sizing_gated && job.sizing_builder.is_some() {
                let original = job.sizing_builder.as_ref().expect("sizing").stage_for_gate();
                for stage in P7C1_SIZING_STAGES {
                    job.sizing_builder.as_mut().expect("sizing").set_stage_for_gate(stage);
                    assert_gates(&mut job, operation);
                }
                job.sizing_builder.as_mut().expect("sizing").set_stage_for_gate(original);
                sizing_gated = true;
            }
            if precompute_gated && surface_precompute_gated && timestep_builder_gated && timestep_gated && zone_gated && system_gated && plant_gated && schedule_gated && warmup_gated && aggregate_zone_gated && aggregate_facility_gated && sizing_gated
            {
                InteractiveJob::begin_close(&mut job);
                for _ in 0..100_000 {
                    if matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                        return;
                    }
                }
                panic!("live nested gate job did not close")
            }
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut preview_sequence);
            let mut outcome = job.step(&mut context);
            while !matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
        }
        panic!("live nested cursor family was never mounted")
    }

    #[test]
    fn p7c1_language_agnostic_law_fixture_matches_reference_parser() {
        let source = include_str!("../../../../../🪨️tests/p7c1-energy-numerical-laws.json");
        let reference: serde_json::Value = serde_json::from_str(source).expect("reference JSON parser");
        let marker = "\"schema\": \"";
        let start = source.find(marker).expect("schema field") + marker.len();
        let end = start + source[start..].find('"').expect("schema terminator");
        let schema = &source[start..end];
        assert_eq!(reference["schema"].as_str(), Some(schema));
        assert_eq!(reference["step"]["fuel"].as_u64(), Some(1));
    }
}
