//! 📖️ Byte-bounded canonical reading over exact frozen typed root ownership.

use super::*;
use std::mem::ManuallyDrop;
use semio_framework_value::{ValueError, RetirementDemand, FactoryAuthority, FactoryProducerReply, OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementPollExit, retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress}};

/// 🏭️ Retires the resident frozen root through its typed factory inside the caller's effect; the root stays resident until the factory takes it.
fn retire_snapshot<T>(factory: &dyn SnapshotRetirementFactory<T>, root: &mut Option<Arc<T>>, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), OriginalRetirementFailure> {
    let (mut output, mut reply, mut attempted) = (None, None, None);
    let producer = match FactoryProducerReply::borrow(&mut output, &mut reply, &mut attempted) { Ok(producer) => producer, Err(error) => return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Err(error.into()), work: 0 }) };
    let _ = factory.retire(root, effect, producer);
    let Some(turn) = reply.take() else { return RetainedWorkPoll::Pending { work: effect.performed_work() }; };
    let result = match (turn.result, output.take()) { (Ok(progress), Some(owner)) => Ok((owner, progress)), (Ok(_), None) => Err(ValueError::invariant("typed factory completed without publishing its retirement owner").into()), (Err(failure), _) => Err(failure) };
    RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: turn.work })
}

//#region 📦️ReaderOwnership
/// 📖️ Retains a frozen typed Arc and borrowed traversal; no Store publication authority is exposed.
pub struct ArtifactCanonicalJsonReader<T> {
    pub owned: ManuallyDrop<ReaderState<T>>,
}

#[doc(hidden)]
pub struct ReaderState<T> {
    pub encoder: ArtifactCanonicalEditEncoder,
    pub root: Option<Arc<T>>,
    pub retirement: Option<Arc<dyn SnapshotRetirementFactory<T>>>,
    pub active: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub factory_close: Option<FactoryAuthority>,
    pub completed_bytes: u64,
    pub cancelled: bool,
    pub failed: bool,
    pub closing: bool,
}

impl<T> ReaderState<T> {
    fn new(root: Arc<T>, retirement: Arc<dyn SnapshotRetirementFactory<T>>) -> Self {
        Self { encoder: ArtifactCanonicalEditEncoder::default(), root: Some(root), retirement: Some(retirement), active: None, factory_close: None, completed_bytes: 0, cancelled: false, failed: false, closing: false }
    }

    pub fn completed_bytes(&self) -> u64 {
        self.completed_bytes
    }
    pub fn is_complete(&self) -> bool {
        !self.cancelled && !self.failed && !self.closing && self.encoder.is_complete()
    }
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn take_root(&mut self) -> Option<Arc<T>> {
        let transferable = if self.closing { self.encoder.terminal_is_empty() } else { self.is_complete() };
        if !transferable || self.active.is_some() {
            return None;
        }
        self.encoder.reset().ok()?;
        self.closing = true;
        self.root.take()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.encoder.terminal_is_empty() && self.root.is_none() && self.retirement.is_none() && self.active.is_none() && self.factory_close.is_none()
    }

}

impl<T: Send + Sync + 'static> ReaderState<T> {
    pub fn demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if !self.encoder.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: self.encoder.next_close_release_byte_demand(), depth: 1, ..Default::default() }); }
        if let Some(active) = self.active.as_ref() {
            let mut demand = artifact_retirement_box_demands(active, maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical reader depth overflow"))?;
            return Ok(demand);
        }
        if let Some(root) = self.root.as_ref() { let factory = self.retirement.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical reader lost its root retirement factory"))?; return Ok(RetirementDemand { capacity_bytes: factory.retirement_birth_bytes(root), depth: 2, ..Default::default() }); }
        if self.retirement.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.as_ref() { let mut demand = factory.demands(maximum_body_bytes)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "reader factory depth overflow"))?; return Ok(demand); }
        Ok(Default::default())
    }
    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = self.close_action(effect);
        RetirementPollExit::settle(effect, result)
    }
    fn close_action(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical reader exceeds admitted depth").into()); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.encoder.terminal_is_empty() { let progress = self.encoder.close_step(grant)?.progress(); RetirementPollExit::pay(effect, 1)?; return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if self.active.is_some() { let step = RetirementPollExit::resume(artifact_retirement_box_close_step(&mut self.active, &mut effect.nested(child)))?; return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.root.is_some() {
            let Some(factory) = self.retirement.as_ref() else { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical reader lost its root retirement factory").into()); };
            let (active, progress) = RetirementPollExit::resume(retire_snapshot(factory.as_ref(), &mut self.root, &mut effect.nested(child)))?;
            self.active = Some(active);
            if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical reader constructor changed its receipt").into()); }
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if let Some(factory) = self.retirement.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(factory) = self.factory_close.as_mut() {
            let step = RetirementPollExit::resume(factory.step(&mut effect.nested(child)))?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "canonical reader factory")?;
            if factory.terminal_is_empty() { self.factory_close = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl<T: ArtifactCanonicalJson + Send + 'static> ReaderState<T> {
    pub fn encode_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<ArtifactCanonicalJsonTreeStep, ArtifactCanonicalJsonEncodeError> {
        if !grant.permits_one() || self.cancelled || self.failed || self.closing || output.is_empty() {
            return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 });
        }
        let maximum = grant.maximum_copy_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
        let root = self.root.as_ref().ok_or_else(|| ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason: ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-reader.root-missing") })?;
        let result = self.encoder.encode_chunk(root.as_ref(), &mut output[..maximum],grant.retained_grant());
        let count = match &result {
            Ok(step) => step.written_bytes,
            Err(error) => {
                self.failed = true;
                error.written_bytes
            }
        };
        let Some(completed) = self.completed_bytes.checked_add(count as u64) else {
            self.failed = true;
            let progress = match &result { Ok(step) => step.ownership.progress(), Err(error) => error.reason.retained_progress() };
            return Err(ArtifactCanonicalJsonEncodeError { written_bytes: count, reason: ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"canonical-reader.work-overflow").with_retained_progress(progress) });
        };
        self.completed_bytes = completed;
        result
    }
}

impl<T> ArtifactCanonicalJsonReader<T> {
    pub fn new(root: Arc<T>, retirement: Arc<dyn SnapshotRetirementFactory<T>>) -> Self {
        Self { owned: ManuallyDrop::new(ReaderState::new(root, retirement)) }
    }
    pub fn completed_bytes(&self) -> u64 {
        self.owned.completed_bytes()
    }
    pub fn is_complete(&self) -> bool {
        self.owned.is_complete()
    }
    pub fn cancel(&mut self) {
        self.owned.cancel();
    }
    pub fn begin_close(&mut self) {
        self.owned.begin_close();
    }
    pub fn take_root(&mut self) -> Option<Arc<T>> {
        self.owned.take_root()
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.owned.terminal_is_empty()
    }

}

impl<T: Send + Sync + 'static> ArtifactCanonicalJsonReader<T> {
    pub fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> { self.owned.demands(maximum_body_bytes) }
    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { self.owned.close_step(effect) }
}

impl<T: ArtifactCanonicalJson + Send + 'static> ArtifactCanonicalJsonReader<T> {
    pub fn encode_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<ArtifactCanonicalJsonTreeStep, ArtifactCanonicalJsonEncodeError> {
        self.owned.encode_chunk(grant, output)
    }
}

impl<T> Drop for ArtifactCanonicalJsonReader<T> {
    fn drop(&mut self) {
        if !self.owned.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("canonical reader dropped before exact root transfer or retirement");
            }
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.owned);
        }
    }
}
//#endregion 📦️ReaderOwnership
//#region 🧪️ReaderLaws

//#endregion 🧪️ReaderLaws
