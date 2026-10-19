//! ♻️ Energy engine checkpoint restore: replays one retained checkpoint packet into a fresh numerical job.

extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_sim_retirement as retirement;
extern crate semio_s_plugin_energy_engine_sim_wire as wire;
extern crate semio_s_plugin_energy_engine_sim as sim;

use crate::retirement::{ENERGY_RUNG_DEMAND, close_config_step, close_model_step, config_is_terminal_empty, energy_done, energy_grant_covers, energy_idle, energy_one, energy_rung, energy_turn, model_is_terminal_empty};
use crate::wire::{ENERGY_CHECKPOINT_BYTES, ENERGY_WIRE_HEADER_BYTES, ENERGY_WIRE_MAGIC, ENERGY_WIRE_VERSION, EnergyJobStage, EnergyNumericalBounds, EnergyNumericalCensus, EnergyNumericalDimension, EnergyQualityTier, EnergyWireIdentity, EnergyWireKind, EnergyWireLease, EnergyWirePacket, EnergyWireRejection, energy_spend, energy_stage_from_code, energy_tier_from_code};
use crate::sim::{ENERGY_ABANDONMENT_SLOTS, EnergyJob, EnergyJobAuthority, EnergyRun};
use crate::kernel::{SimulationConfig, SimulationEnvironment};
use crate::model::Model;
use semio_framework_job::{
    InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RevisionId, StepContext,
};
use semio_framework_value::{RetirementDemand, ValueError,RetainedCloneStep};
use std::mem::size_of;
use std::sync::Mutex;

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
    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry(self, bounds: EnergyNumericalBounds) -> Result<EnergyRestoreJob, Self> {
        EnergyRestoreJob::admit(self.operation, self.model, self.config, self.packet, bounds)
    }

    /// 📏️ Quotes the retained packet page first, then one popped model or config owner.
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.packet.terminal_is_empty() {
            return self.packet.retirement_demands();
        }
        Ok(if self.terminal_is_empty() { RetirementDemand::default() } else { ENERGY_RUNG_DEMAND })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        if !self.packet.terminal_is_empty() {
            return Ok(energy_turn(self.packet.close_step(grant))?.0);
        }
        if self.terminal_is_empty() {
            return Ok(energy_done());
        }
        if !energy_grant_covers(grant, ENERGY_RUNG_DEMAND) {
            return Ok(energy_idle());
        }
        if let Some(released) = close_model_step(&mut self.model, grant.maximum_release_bytes) {
            return Ok(energy_rung(released));
        }
        if let Some(released) = close_config_step(&mut self.config, grant.maximum_release_bytes) {
            return Ok(energy_rung(released));
        }
        Ok(energy_done())
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
    input_digest: [u8;32],
    bounds: EnergyNumericalBounds,
    rebuild: Option<EnergyJob>,
    replay_retiring: Option<EnergyWirePacket>,
    replay_checkpoint_lease: Option<EnergyWireLease>,
    replay_failed: Option<EnergyWireRejection>,
    ready: bool,
    abandonment_slot: usize,
}

impl std::fmt::Debug for EnergyRestoreJob {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("EnergyRestoreJob").field("operation", &self.operation).field("field", &self.field).field("mounted", &self.packet.is_some()).finish()
    }
}

pub struct EnergyRestoreAbandoned {
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
    input_digest: [u8;32],
    bounds: EnergyNumericalBounds,
    rebuild: Option<EnergyJob>,
    replay_retiring: Option<EnergyWirePacket>,
    replay_checkpoint_lease: Option<EnergyWireLease>,
    replay_failed: Option<EnergyWireRejection>,
    ready: bool,
}

pub enum EnergyRestoreAbandonmentSlot {
    Vacant,
    Reserved(Operation),
    Abandoned(Operation, Box<EnergyRestoreAbandoned>),
}

pub static ENERGY_RESTORE_ABANDONMENT_REGISTRY: Mutex<[EnergyRestoreAbandonmentSlot; ENERGY_ABANDONMENT_SLOTS]> = Mutex::new([const { EnergyRestoreAbandonmentSlot::Vacant }; ENERGY_ABANDONMENT_SLOTS]);

impl EnergyRestoreJob {
    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
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
            input_digest: [0;32],
            bounds,
            rebuild: None,
            replay_retiring: None,
            replay_checkpoint_lease: None,
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
            input_digest: authority.input_digest,
            bounds: authority.bounds,
            rebuild: authority.rebuild,
            replay_retiring: authority.replay_retiring,
            replay_checkpoint_lease: authority.replay_checkpoint_lease,
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
        if self.field < 5 {
            let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: [2,12,24,48,32][self.field as usize] + size_of::<u8>(), ..Default::default() };
            if !receipt.fits(context.retained_grant()) { return Ok(false); }
            context.consume_retained(receipt).map_err(|_| EnergyWireRejection::Backing)?;
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
                4 => self.input_digest.copy_from_slice(&page[164..196]),
                _ => unreachable!("decode field is bounded"),
            }
            self.field += 1;
            return Ok(false);
        }
        if let Some(packet) = self.replay_retiring.as_mut() {
            if packet.terminal_is_empty() {
                self.replay_retiring = None;
            } else {
                let step = packet.close_step(context.retained_grant());
                energy_spend(context, step)?;
            }
            return Ok(false);
        }
        if let Some(lease) = self.replay_checkpoint_lease.as_mut() {
            if !lease.packet().terminal_is_empty() {
                let step = lease.packet_mut().close_step(context.retained_grant());
                energy_spend(context, step)?;
                return Ok(false);
            }
            let lease = self.replay_checkpoint_lease.take().expect("empty replay checkpoint lease remains owned");
            if let Err(lease) = self.rebuild.as_mut().ok_or(EnergyWireRejection::Backing)?.ack_checkpoint_packet(lease) {
                self.replay_checkpoint_lease = Some(lease);
                return Err(EnergyWireRejection::Backing);
            }
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
        if rebuild.input_identity.digest().is_none() { context.consume_fuel(1); let _ = rebuild.step_input_identity(context)?; return Ok(false); }
        if rebuild.input_identity.digest() != Some(&self.input_digest) { self.replay_failed = Some(EnergyWireRejection::Identity); return Err(EnergyWireRejection::Identity); }
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
        match rebuild.run(context) {
            EnergyRun::Yield | EnergyRun::Cancelled => {}
            EnergyRun::Preview => {
                self.replay_retiring = rebuild.take_preview_packet(self.operation.generation).map_err(|_| EnergyWireRejection::Identity)?;
            }
            EnergyRun::Checkpoint(_) => {
                if let Some(lease) = rebuild.take_checkpoint_packet(self.operation.generation).map_err(|_| EnergyWireRejection::Identity)? {
                    self.replay_checkpoint_lease = Some(lease);
                }
            }
            EnergyRun::Fault => {
                self.replay_failed = Some(EnergyWireRejection::Backing);
                return Err(EnergyWireRejection::Backing);
            }
            EnergyRun::Complete => {
                self.replay_failed = Some(EnergyWireRejection::Items);
                self.replay_retiring = rebuild.commit_output.take();
                return Err(EnergyWireRejection::Items);
            }
        }
        Ok(false)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
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
            || job.input_identity.digest() != Some(&self.input_digest)
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

    /// 📏️ Quotes the next close frontier: replay packets, the restore input, the rebuild job, then one popped owner.
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(packet) = self.replay_retiring.as_ref() {
            return packet.retirement_demands();
        }
        if let Some(lease) = self.replay_checkpoint_lease.as_ref() {
            return if lease.packet().terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else { lease.packet().retirement_demands() };
        }
        if let Some(packet) = self.packet.as_ref() {
            return packet.retirement_demands();
        }
        if let Some(rebuild)=self.rebuild.as_ref(){if rebuild.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<EnergyJob>>(),depth:1,..Default::default()})}return Ok(RetirementDemand{copy_bytes:rebuild.next_close_copy_byte_demand()?,capacity_bytes:rebuild.next_close_capacity_byte_demand(0)?,release_bytes:rebuild.next_close_release_byte_demand()?,depth:rebuild.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"Energy restore numerical child depth overflow"))?});}
        Ok(if self.model.is_some() || self.config.is_some() { ENERGY_RUNG_DEMAND } else { RetirementDemand::default() })
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if self.replay_retiring.is_none()&&self.replay_checkpoint_lease.is_none()&&self.packet.is_none(){
            if let Some(rebuild)=self.rebuild.as_mut(){
                if !rebuild.terminal_is_empty(){let grant=cx.retained_grant();if grant.maximum_depth<2{return Ok(energy_idle())}return cx.with_retained_loan(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant},|cx|rebuild.close_step(cx));}
                return semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<Option<EnergyJob>>(),depth:1,..Default::default()},|_|{drop(self.rebuild.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<EnergyJob>>(),..Default::default()}))});
            }
        }
        let demand=self.close_demands()?;
        semio_framework_job::close_original_retained_step(cx,demand,|loan|self.close_owned_frontier(loan).map(|step|RetainedCloneStep::Progress(step.progress())))
    }

    fn close_owned_frontier(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        if let Some(packet) = self.replay_retiring.as_mut() {
            let (step, _) = energy_turn(packet.close_step(grant))?;
            if packet.terminal_is_empty() {
                self.replay_retiring = None;
            }
            return Ok(step);
        }
        if let Some(lease) = self.replay_checkpoint_lease.as_mut() {
            if !lease.packet().terminal_is_empty() {
                let (step, _) = energy_turn(lease.packet_mut().close_step(grant))?;
                return Ok(step);
            }
            if !energy_grant_covers(grant, RetirementDemand { depth: 1, ..Default::default() }) {
                return Ok(energy_idle());
            }
            let Some(rebuild) = self.rebuild.as_mut() else {
                return Ok(InteractiveJobCloseStep::Blocked);
            };
            let lease=self.replay_checkpoint_lease.take().unwrap();
            if let Err(lease) = rebuild.ack_checkpoint_packet(lease) {
                self.replay_checkpoint_lease = Some(lease);
                return Ok(InteractiveJobCloseStep::Blocked);
            }
            return Ok(energy_one());
        }
        if let Some(packet) = self.packet.as_mut() {
            let (step, _) = energy_turn(packet.close_step(grant))?;
            if packet.terminal_is_empty() {
                self.packet = None;
            }
            return Ok(step);
        }
        if self.model.is_none() && self.config.is_none() {
            return Ok(energy_done());
        }
        if !energy_grant_covers(grant, ENERGY_RUNG_DEMAND) {
            return Ok(energy_idle());
        }
        if let Some(model) = self.model.as_mut() {
            if let Some(released) = close_model_step(model, grant.maximum_release_bytes) {
                return Ok(energy_rung(released));
            }
            self.model = None;
            return Ok(energy_one());
        }
        if let Some(config) = self.config.as_mut() {
            if let Some(released) = close_config_step(config, grant.maximum_release_bytes) {
                return Ok(energy_rung(released));
            }
            self.config = None;
            return Ok(energy_one());
        }
        Ok(energy_done())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.packet.is_none() && self.model.is_none() && self.config.is_none() && self.rebuild.is_none() && self.replay_retiring.is_none() && self.replay_checkpoint_lease.is_none()
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
            input_digest: self.input_digest,
            bounds: self.bounds,
            rebuild: self.rebuild.take(),
            replay_retiring: self.replay_retiring.take(),
            replay_checkpoint_lease: self.replay_checkpoint_lease.take(),
            replay_failed: self.replay_failed.take(),
            ready: self.ready,
        };
        let mut registry = ENERGY_RESTORE_ABANDONMENT_REGISTRY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        debug_assert!(matches!(registry.get(self.abandonment_slot), Some(EnergyRestoreAbandonmentSlot::Reserved(candidate)) if candidate.operation == self.operation.operation && candidate.generation == self.operation.generation));
        registry[self.abandonment_slot] = EnergyRestoreAbandonmentSlot::Abandoned(self.operation, Box::new(authority));
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
