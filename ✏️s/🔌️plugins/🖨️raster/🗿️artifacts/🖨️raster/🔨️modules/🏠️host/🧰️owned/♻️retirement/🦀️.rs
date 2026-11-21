//! ♻️ raster retirement: the snapshot and mutation retirement factories, the displaced value, the retirement slot and the grant helpers.

use super::*;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, FactoryProducerReply, FactoryProducerReplyComplete, OriginalRetirementFailure, OwnedValueRetirementFactory, RetainedAllocationEffect, RetainedWorkOutcome, SharedValueRetirementFactory, SnapshotRetirementFactory, retirement::arc_custody::original::SharedOriginalStorage};

type RasterRetirementReply<'a> = FactoryProducerReply<'a, dyn store::ErasedSnapshotRetirement, OriginalRetirementFailure>;
type RasterRetirementReplyComplete<'a> = FactoryProducerReplyComplete<'a, OriginalRetirementFailure>;

/// 🎟️ Retained workers borrow an existing caller or return bounded raw receipts without creating source authority.
pub trait RasterWork{
    fn should_yield(&mut self)->bool;
    fn is_cancelled(&self)->bool;
    fn retained_grant(&self)->RetainedCloneGrant;
    fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>;
    fn consume_fuel(&mut self,work:u64);
    fn consume_work(&mut self, work: u128) { self.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX)); }
    fn close_owned(&mut self, close: &mut dyn FnMut(&mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome) -> Result<RetainedCloneStep, ValueError>;
    fn fuel_remaining(&self)->u64;
}

impl RasterWork for semio_framework_job::StepContext<'_>{
    fn should_yield(&mut self)->bool{semio_framework_job::StepContext::should_yield(self)}
    fn is_cancelled(&self)->bool{semio_framework_job::StepContext::is_cancelled(self)}
    fn retained_grant(&self)->RetainedCloneGrant{semio_framework_job::StepContext::retained_grant(self)}
    fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{semio_framework_job::StepContext::consume_retained(self,progress)}
    fn consume_fuel(&mut self,work:u64){semio_framework_job::StepContext::consume_fuel(self,work);}
    fn fuel_remaining(&self)->u64{semio_framework_job::StepContext::fuel_remaining(self)}
    fn close_owned(&mut self, close: &mut dyn FnMut(&mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome) -> Result<RetainedCloneStep, ValueError> {
        let mut grant = self.retained_grant();
        grant.maximum_items = grant.maximum_items.min(1);
        let Some((turn, performed)) = self.with_original_effect(grant, |_, effect| close(effect))? else {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        };
        self.consume_work(performed);
        turn.result
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterSnapshotRetirementFactory;

macro_rules! raster_owned_retirement_factory {
    ($factory:ty, $value:ty) => {
        impl ArtifactOwnedValueRetirementFactory<$value> for $factory {
            fn retirement_birth_bytes(&self, value: &$value) -> usize {
                OwnedValueRetirementFactory::default().retirement_birth_bytes(value)
            }

            fn retire_owned_demands(&self, value: &$value) -> Result<semio_framework_value::RetirementDemand, ValueError> {
                OwnedValueRetirementFactory::default().retire_owned_demands(value)
            }

            fn retire_owned_work_demand(&self, value: &$value) -> Result<u64, ValueError> {
                OwnedValueRetirementFactory::default().retire_owned_work_demand(value)
            }

            fn retire_owned<'a>(&self, value: &mut Option<$value>, effect: &mut RetainedAllocationEffect<'_>, reply: RasterRetirementReply<'a>) -> RasterRetirementReplyComplete<'a> {
                OwnedValueRetirementFactory::default().retire_owned(value, effect, reply)
            }

            fn recover_retire_owned<'a>(&self, value: &mut Option<$value>, effect: &mut RetainedAllocationEffect<'_>, reply: RasterRetirementReply<'a>) -> RasterRetirementReplyComplete<'a> {
                OwnedValueRetirementFactory::default().recover_retire_owned(value, effect, reply)
            }
        }
    };
}

raster_owned_retirement_factory!(RasterSnapshotRetirementFactory, RasterSnapshot);

impl SnapshotRetirementFactory<RasterSnapshot> for RasterSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, snapshot: &SharedOriginalStorage<RasterSnapshot>) -> usize {
        SharedValueRetirementFactory::default().retirement_birth_bytes(snapshot)
    }

    fn retire_demands(&self, snapshot: &SharedOriginalStorage<RasterSnapshot>) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        SharedValueRetirementFactory::default().retire_demands(snapshot)
    }

    fn retire_work_demand(&self, snapshot: &SharedOriginalStorage<RasterSnapshot>) -> Result<u64, ValueError> {
        SharedValueRetirementFactory::default().retire_work_demand(snapshot)
    }

    fn retire<'a>(&self, snapshot: &mut Option<SharedOriginalStorage<RasterSnapshot>>, effect: &mut RetainedAllocationEffect<'_>, reply: RasterRetirementReply<'a>) -> RasterRetirementReplyComplete<'a> {
        SharedValueRetirementFactory::default().retire(snapshot, effect, reply)
    }

    fn recover_retire<'a>(&self, snapshot: &mut Option<SharedOriginalStorage<RasterSnapshot>>, effect: &mut RetainedAllocationEffect<'_>, reply: RasterRetirementReply<'a>) -> RasterRetirementReplyComplete<'a> {
        SharedValueRetirementFactory::default().recover_retire(snapshot, effect, reply)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterMutationRetirementFactory;

raster_owned_retirement_factory!(RasterMutationRetirementFactory, RasterMutation);

/// ♻️ Advances the selected resident owner under its receiving effect.
pub fn raster_retire_owned<T: semio_framework_value::retirement::RetireOwned>(
    pending: &mut Option<T>,
    active: &mut Option<Box<dyn store::ErasedSnapshotRetirement>>,
    effect: &mut RetainedAllocationEffect<'_>,
) -> RetainedWorkOutcome {
    use store::os_store::snapshot_read::{artifact_retirement_admit_owned, artifact_retirement_box_close_step, settle_close_poll};
    if active.is_none() {
        return artifact_retirement_admit_owned(pending, active, effect);
    }
    let poll = artifact_retirement_box_close_step(active, effect.grant().maximum_copy_bytes, effect);
    let work = poll.work();
    RetainedWorkOutcome { result: settle_close_poll(poll), work }
}

/// ⛽️ Records a selected local close action before changing its resident owner.
pub fn raster_close_unit(effect: &mut RetainedAllocationEffect<'_>) -> Result<(), ValueError> {
    if effect.performed(1)? { return Ok(()); }
    Err(ValueError::literal(ValueRefusalKind::WorkLimit, "Raster close exceeded its work grant"))
}

/// ♻️ Every value a stepwise Raster authority displaces or abandons; the framework's owned retirement closes it under an explicit grant.
#[derive(semio_framework_value::RetireOwned)]
pub enum RasterDisplaced {
    String(String),
    Layer(RasterLayerNode),
    Snapshot(RasterSnapshot),
    AssetEntry((String, RasterAssetChild)),
    ValueEntry((String, semio_framework_value::DslValue)),
    Value(semio_framework_value::DslValue),
    Mutations(Vec<RasterMutation>),
}

/// ⛽️ Quotes the selected boxed retirement action or a single pending custody move.
pub fn raster_owned_close_work(active: Option<&dyn store::ErasedSnapshotRetirement>, pending: bool, body: usize) -> Result<u64, ValueError> {
    match active {
        Some(owner) if !owner.terminal_is_empty() => owner.next_computational_work(body),
        _ => Ok(u64::from(pending || active.is_some())),
    }
}

/// 🗑️ One displaced owner and its admitted retirement frame; the owner stays in `pending` until a grant funds its frame.
pub struct RasterRetirementSlot {
    pub pending: std::mem::ManuallyDrop<Option<RasterDisplaced>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl RasterRetirementSlot {
    pub fn new() -> Self {
        Self { pending: std::mem::ManuallyDrop::new(None), active: std::mem::ManuallyDrop::new(None) }
    }

    pub fn put(&mut self, owner: RasterDisplaced) {
        assert!(self.is_empty(), "Raster retirement slot admits one displaced owner at a time");
        *self.pending = Some(owner);
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_none() && self.active.is_none()
    }

    pub fn birth_demand() -> semio_framework_value::RetirementDemand {
        let capacity_bytes = semio_framework_value::retirement::owned_retirement_birth_bytes::<RasterDisplaced>();
        semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<RasterDisplaced>>() + capacity_bytes, capacity_bytes, depth: 2, ..Default::default() }
    }

    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body);
        }
        store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending)
    }

    pub fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        raster_owned_close_work(self.active.as_deref(), self.pending.is_some(), body)
    }

    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome {
        let grant = effect.grant();
        let mut work=0;let result=(||{
        let empty = RetainedCloneProgress::default();
        if self.is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 || effect.remaining_work_units()? == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Raster displaced owner exceeds admitted depth"));
        }
        if raster_short(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let turn = raster_retire_owned(&mut self.pending, &mut self.active, effect);
        work = turn.work;
        let step = turn.result?;
        Ok(RetainedCloneStep::Progress(step.progress()))
        })();semio_framework_value::RetainedWorkOutcome{result,work}
    }
}

impl Drop for RasterRetirementSlot {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_empty(), "Raster retirement slot reached Drop before its displaced owner retired");
        if self.is_empty() {
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.pending);
                std::mem::ManuallyDrop::drop(&mut self.active);
            }
        }
    }
}

pub fn raster_short(grant: RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes
}

pub fn raster_finish(step: RetainedCloneStep, terminal: bool) -> RetainedCloneStep {
    match step {
        RetainedCloneStep::Complete(progress) | RetainedCloneStep::Progress(progress) if terminal => RetainedCloneStep::Complete(progress),
        RetainedCloneStep::Complete(progress) | RetainedCloneStep::Progress(progress) => RetainedCloneStep::Progress(progress),
    }
}

pub fn raster_nested(mut demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "Raster nested retirement depth overflow"))?;
    Ok(demand)
}

pub fn raster_child_grant(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }
}

pub fn raster_reserve_unit(cx: &mut dyn RasterWork) -> bool {
    if cx.should_yield() || cx.fuel_remaining() == 0 {
        return false;
    }
    cx.consume_fuel(1);
    true
}
