//! 📡️ Energy engine retained wire: checkpoint, preview and commit packets with their lease and lossless queue, the job
//! stage, quality and preview vocabulary, and the numerical admission census they carry.

#[cfg(test)]
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_meters as meters;
extern crate semio_s_plugin_energy_engine_precompute as precompute;
extern crate semio_s_plugin_energy_engine_site as site;
extern crate semio_s_plugin_energy_engine_units as units;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_sim_retirement as retirement;

use crate::retirement::{energy_grant_covers, energy_self_funded, energy_turn};
use crate::kernel::{SimulationConfig, SimulationEnvironment, SimulationKernel, TimestepWork};
use crate::meters::{EndUse, FuelType};
use crate::model::Model;
use crate::precompute::PrecomputedModel;
use crate::site::WeatherRecord;
use crate::units::Unit;
use semio_framework_job::{
    InteractiveJob, InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPayload, StepContext,
};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub const ENERGY_WIRE_MAGIC: [u8; 8] = *b"SMENERGY";

pub const ENERGY_WIRE_VERSION: u16 = 1;

pub const ENERGY_WIRE_QUEUE_SLOTS: usize = 4;

pub const ENERGY_WIRE_LEASE_SLOTS: usize = 64;

pub const ENERGY_WIRE_HEADER_BYTES: usize = 80;

pub const ENERGY_CHECKPOINT_BYTES: usize = 164;

/// 💸️ Charges one owner close turn to the running step's own wallet and reports whether that owner finished.
pub fn energy_spend(context: &mut StepContext<'_>, step: Result<RetainedCloneStep, ValueError>) -> Result<bool, EnergyWireRejection> {
    let (progress, done) = match step {
        Ok(RetainedCloneStep::Progress(progress)) => (progress, false),
        Ok(RetainedCloneStep::Complete(progress)) => (progress, true),
        Err(_) => return Err(EnergyWireRejection::Backing),
    };
    context.consume_retained(progress).map_err(|_| EnergyWireRejection::Backing)?;
    Ok(done)
}

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
    pub kind: EnergyWireKind,
    pub identity: EnergyWireIdentity,
    pub payload: RetainedJobPayload,
    pub preview: Option<EnergyJobPreview>,
    pub reservation: Option<EnergyCommitReservation>,
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

#[expect(clippy::large_enum_variant, reason = "The fixed recovery registry accepts abandoned packet ownership from Drop without allocating.")]
pub enum EnergyWireLeaseRecoverySlot {
    Vacant,
    Reserved(u64),
    Abandoned(u64, EnergyWirePacket),
}

pub static ENERGY_WIRE_LEASE_RECOVERY: Mutex<[EnergyWireLeaseRecoverySlot; ENERGY_WIRE_LEASE_SLOTS]> = Mutex::new([const { EnergyWireLeaseRecoverySlot::Vacant }; ENERGY_WIRE_LEASE_SLOTS]);

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

    /// 📏️ Quotes the next original payload page or ledger this packet would release.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        self.payload.retirement_demands()
    }

    /// 🎟️ Releases one payload frontier only when the caller's grant covers its quote.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.payload.retirement_demands()?;
        if !energy_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.payload.close_step(grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.terminal_is_empty()
    }
}

pub struct EnergyWireQueue {
    kind: EnergyWireKind,
    slots: [Option<EnergyWirePacket>; ENERGY_WIRE_QUEUE_SLOTS],
    head: usize,
    pub len: usize,
    next_token: u64,
    pub in_flight: Option<EnergyWireInFlight>,
    pub reserved_push: bool,
}

#[derive(Clone, Copy)]
pub struct EnergyWireInFlight {
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

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn push(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
        if packet.kind != self.kind || self.len == ENERGY_WIRE_QUEUE_SLOTS || self.reserved_push {
            return Err(packet);
        }
        let index = (self.head + self.len) % ENERGY_WIRE_QUEUE_SLOTS;
        self.slots[index] = Some(packet);
        self.len += 1;
        Ok(())
    }

    pub fn reserve_push(&mut self) -> Result<(), EnergyWireRejection> {
        if self.len == ENERGY_WIRE_QUEUE_SLOTS || self.reserved_push {
            return Err(EnergyWireRejection::Saturated);
        }
        self.reserved_push = true;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn push_reserved(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
        if packet.kind != self.kind || !self.reserved_push || self.len == ENERGY_WIRE_QUEUE_SLOTS {
            return Err(packet);
        }
        self.reserved_push = false;
        let index = (self.head + self.len) % ENERGY_WIRE_QUEUE_SLOTS;
        self.slots[index] = Some(packet);
        self.len += 1;
        Ok(())
    }

    pub fn take(&mut self) -> Option<EnergyWireLease> {
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

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
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

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
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

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_transfer(&mut self, mut lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
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

    pub fn take_terminal(&mut self) -> Option<EnergyWirePacket> {
        self.recover_lost();
        if self.len == 0 || self.in_flight.is_some() {
            return None;
        }
        let packet = self.slots[self.head].take()?;
        self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
        self.len -= 1;
        Some(packet)
    }

    pub fn retained_totals_at(&self, index: usize) -> Option<(usize, usize, usize)> {
        if index < ENERGY_WIRE_QUEUE_SLOTS {
            return self.slots[index].as_ref().map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1)));
        }
        if index == ENERGY_WIRE_QUEUE_SLOTS {
            return self.in_flight.map(|flight| (flight.pages, flight.bytes, flight.pages.saturating_add(1)));
        }
        None
    }

    pub fn retirement_demands(&self) -> Option<Result<RetirementDemand, ValueError>> {
        if self.in_flight.is_some() {
            return Some(Ok(RetirementDemand::default()));
        }
        if self.len == 0 && self.reserved_push {
            return Some(Ok(RetirementDemand { depth: 1, ..Default::default() }));
        }
        self.slots[self.head].as_ref().map(EnergyWirePacket::retirement_demands)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<InteractiveJobCloseStep>, ValueError> {
        self.recover_lost();
        if self.in_flight.is_some() {
            return Ok(Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }));
        }
        if self.len == 0 && self.reserved_push {
            if !energy_grant_covers(grant, RetirementDemand { depth: 1, ..Default::default() }) {
                return Ok(Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }));
            }
            self.reserved_push = false;
            return Ok(Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }));
        }
        let Some(packet) = self.slots[self.head].as_mut() else { return Ok(None) };
        let (step, done) = energy_turn(packet.close_step(grant))?;
        if done {
            self.slots[self.head] = None;
            self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
            self.len -= 1;
        }
        Ok(Some(step))
    }
}

pub struct EnergyPublicationChannels {
    pub preview: Option<EnergyWirePacket>,
    pub retiring_preview: Option<EnergyWirePacket>,
    pub checkpoints: EnergyWireQueue,
    pub commits: EnergyWireQueue,
    pub faults: EnergyWireQueue,
}

impl Default for EnergyPublicationChannels {
    fn default() -> Self {
        Self { preview: None, retiring_preview: None, checkpoints: EnergyWireQueue::new(EnergyWireKind::Checkpoint), commits: EnergyWireQueue::new(EnergyWireKind::Commit), faults: EnergyWireQueue::new(EnergyWireKind::Fault) }
    }
}

pub struct EnergyWireBuild {
    pub kind: EnergyWireKind,
    pub identity: EnergyWireIdentity,
    pub writer: semio_framework_job::RetainedJobPayloadWriter,
    pub field: u8,
}

pub fn energy_wire_header(kind: EnergyWireKind, identity: EnergyWireIdentity, census: EnergyNumericalCensus, stage: EnergyJobStage, tier: EnergyQualityTier) -> [u8; ENERGY_WIRE_HEADER_BYTES] {
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

/// ⚡️ Fidelity label carried by every preview so provisional fields cannot be mistaken for a
/// validated final simulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum EnergyQualityTier {
    SteadyStateEstimate,
    DesignDay,
    CoarseTimestep,
    Final,
}

/// 🧭️ Persistent stage of [`EnergyJob`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct EnergyJobPreview {
    pub sequence: u64,
    pub tier: EnergyQualityTier,
    pub stage: EnergyJobStage,
    pub warmup_hour: u32,
    pub timestep: u32,
    pub total_timesteps: u32,
    pub facility_electricity_kwh: f64,
}

/// 🧭️ Live numerical cursor of [`EnergyJob`]: what a host projects into run progress without decoding a
/// retained preview packet. `timestep` counts completed run timesteps, `warmup_hour` the warmup cursor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergyJobCursor {
    pub stage: EnergyJobStage,
    pub tier: EnergyQualityTier,
    pub warmup_hour: u32,
    pub warmup_hours: u32,
    pub timestep: u32,
    pub total_timesteps: u32,
    pub facility_electricity_kwh: f64,
}

/// 📏️ Schema-first simultaneous working-set census for an Energy numerical job.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
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

impl EnergyNumericalCensus {
    pub fn observe(model: &Model, config: &SimulationConfig) -> Option<Self> {
        let timesteps = SimulationKernel::run_period(config).total_hours() as usize;
        let weather_records = observed_weather_records(config);
        let airflow_nodes = model.airflow_network.as_ref().map_or(0, |network| network.zone_node_ids.capacity().saturating_add(1));
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
            identifier_bytes,
        ];
        let observed_items = checked_sum(dimensions)?;
        let observed_bytes = observed_model_bytes(model, config)?.checked_add(weather_records.checked_mul(size_of::<Option<(usize, WeatherRecord)>>())?)?.checked_add(samples.checked_mul(size_of::<f64>() * 3)?)?.checked_add(identifier_bytes)?;
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

    pub fn first_exceeded(self, maximum: Self) -> Option<EnergyNumericalDimension> {
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

pub fn observed_weather_records(config: &SimulationConfig) -> usize {
    config.weather.as_ref().map_or_else(
        || match config.environment {
            SimulationEnvironment::HeatingDesignDay | SimulationEnvironment::CoolingDesignDay => 24,
            SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => 8_760,
        },
        |weather| weather.records.capacity().max(weather.records.len().max(1)),
    )
}

pub fn weather_copy_target(config: &SimulationConfig) -> usize {
    config.weather.as_ref().map_or_else(
        || match config.environment {
            SimulationEnvironment::HeatingDesignDay | SimulationEnvironment::CoolingDesignDay => 24,
            SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => 8_760,
        },
        |weather| weather.records.len().max(1),
    )
}

pub fn checked_sum(values: impl IntoIterator<Item = usize>) -> Option<usize> {
    values.into_iter().try_fold(0usize, usize::checked_add)
}

pub fn observed_identifier_bytes(model: &Model, config: &SimulationConfig) -> Option<usize> {
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
    names!(&model.glazing_materials);
    names!(&model.gas_materials);
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

pub fn observed_model_bytes(model: &Model, config: &SimulationConfig) -> Option<usize> {
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
    backing!(model.glazing_materials);
    backing!(model.gas_materials);
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
        bytes = bytes.checked_add(surface.vertices_m.capacity().checked_mul(size_of::<[f64; 3]>())?)?;
    }
    for construction in &model.constructions {
        bytes = bytes.checked_add(construction.layer_material_ids.capacity().checked_mul(size_of::<model::EntityId>())?)?;
    }
    for plant in &model.plant_loops {
        bytes = bytes.checked_add(plant.equipment_ids.capacity().checked_mul(size_of::<model::EntityId>())?)?;
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
        bytes = bytes.checked_add(network.zone_node_ids.capacity().checked_mul(size_of::<(model::EntityId, u32)>())?)?;
        bytes = bytes.checked_add(network.link_ids.capacity().checked_mul(size_of::<u32>())?)?;
    }
    if let Some(weather) = &config.weather {
        bytes = bytes.checked_add(weather.records.capacity().checked_mul(size_of::<WeatherRecord>())?)?;
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

pub fn observed_vector_bytes<T>(owners: &Vec<T>) -> Option<usize> {
    owners.capacity().checked_mul(size_of::<T>())
}

impl EnergyJobStage {
    pub fn label(self) -> &'static str {
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

    pub fn code(self) -> u8 {
        self as u8
    }
}

impl EnergyQualityTier {
    pub fn code(self) -> u8 {
        self as u8
    }
}

pub fn energy_stage_from_code(code: u8) -> Option<EnergyJobStage> {
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

pub fn energy_tier_from_code(code: u8) -> Option<EnergyQualityTier> {
    const TIERS: [EnergyQualityTier; 4] = [EnergyQualityTier::SteadyStateEstimate, EnergyQualityTier::DesignDay, EnergyQualityTier::CoarseTimestep, EnergyQualityTier::Final];
    TIERS.get(code as usize).copied()
}

pub fn decode_preview_packet(packet: &EnergyWirePacket) -> Option<EnergyJobPreview> {
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

pub fn fuel_code(fuel: FuelType) -> u8 {
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

pub fn end_use_code(end_use: EndUse) -> u32 {
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

pub fn unit_code(unit: Unit) -> u8 {
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

/// ♻️ Releases one locally owned packet turn by turn with grants it quotes itself.
pub fn close_retained_packet(packet: &mut EnergyWirePacket) {
    while !packet.terminal_is_empty() {
        let grant = energy_self_funded(packet.retirement_demands().expect("a locally owned packet quotes its close demand"));
        energy_turn(packet.close_step(grant)).expect("a locally owned packet close is never refused");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[derive(Debug)]
pub struct EnergyCommitReservation {
    pub pages: usize,
    pub bytes: usize,
    pub items: usize,
    pub prepared: usize,
    pub sources: [Option<semio_framework_job::JobPayloadPageSource>; semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES],
}

impl EnergyCommitReservation {
    pub fn new(pages: usize, bytes: usize, items: usize) -> Self {
        Self { pages, bytes, items, prepared: 0, sources: std::array::from_fn(|_| None) }
    }

    pub fn take_source(&mut self, index: usize) -> Option<semio_framework_job::JobPayloadPageSource> {
        self.sources.get_mut(index)?.take()
    }

    pub fn restore_source(&mut self, index: usize, source: semio_framework_job::JobPayloadPageSource) {
        self.sources[index] = Some(source);
    }

    pub fn close_source_demand(&self) -> Option<RetirementDemand> {
        self.sources.iter().flatten().next().map(|source| RetirementDemand { release_bytes: source.allocated_capacity_bytes(), depth: 1, ..Default::default() })
    }

    pub fn close_source_step(&mut self) -> Option<usize> {
        let source = self.sources.iter_mut().find(|source| source.is_some())?;
        source.take().map(|source| source.allocated_capacity_bytes())
    }
}
