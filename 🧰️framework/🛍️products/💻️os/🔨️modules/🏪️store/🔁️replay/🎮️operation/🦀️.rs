//! 🎮️ Domain-owned cooperative semantic preparation for one history operation.

use semio_framework_os_spr::{InputReplacement, MutationApplyError, MutationMessage};
use semio_framework_os_store_one_item::ArtifactStoreOneItemGrant;
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

pub use semio_framework_os_spr::command::{ArtifactReplayRetirementFactory, ReplayRetirement};

/// 📨️ Moves every original operation row into native pages before one final edit selection.
pub struct OperationMessageSettlement {
    pub drain: Option<semio_framework_os_spr::command::OperationMessageDrain>,
    pub pending: Option<semio_framework_os_spr::command::MutationMessageRetirement>,
    pub retiring: Option<semio_framework_os_spr::command::MutationMessageLedgerRetirement>,
    pub closing: bool,
}
impl OperationMessageSettlement {
    pub fn new(outcome: semio_framework_os_spr::MutationReplayOutcome) -> Self { Self { drain: Some(semio_framework_os_spr::command::OperationMessageDrain::new(outcome)), pending: None, retiring: None, closing: false } }
    pub fn step(&mut self, ledger: &mut semio_framework_os_spr::command::ReplayMessageAccumulator, grant: RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress), ValueError> {
        let drain = self.drain.as_mut().expect("operation retains its original row drain");
        let progress=drain.step(ledger, grant)?;
        Ok((drain.is_finished(),progress))
    }
    pub fn take(&mut self) -> semio_framework_os_spr::MutationReplayOutcome { let result = self.drain.as_mut().and_then(semio_framework_os_spr::command::OperationMessageDrain::take).expect("completed original operation drain"); self.drain = None; result }
    pub fn begin_close(&mut self) { self.closing = true; }
    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(if self.drain.is_some() { size_of::<semio_framework_os_spr::MutationReplayOutcome>() + size_of::<Option<MutationMessage>>() } else if let Some(pending) = self.pending.as_ref() { pending.next_copy_byte_demand() } else { self.retiring.as_ref().map_or(0, semio_framework_os_spr::command::MutationMessageLedgerRetirement::next_copy_byte_demand) }) }
    pub fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if self.drain.is_some() { return Ok(0); } if let Some(pending) = self.pending.as_ref() { return Ok(pending.next_release_byte_demand()); } self.retiring.as_ref().map_or(Ok(0), semio_framework_os_spr::command::MutationMessageLedgerRetirement::next_release_byte_demand) }
    pub fn next_depth_demand(&self) -> Result<usize, ValueError> { if self.drain.is_some() { return Ok(1); } if let Some(pending) = self.pending.as_ref() { return pending.next_depth_demand().map(|depth| depth + 1); } self.retiring.as_ref().map_or(Ok(0), |owner| owner.next_depth_demand().map(|depth| depth + 1)) }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let idle = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 || grant.maximum_copy_bytes < self.next_copy_byte_demand()? || grant.maximum_release_bytes < self.next_release_byte_demand()? || grant.maximum_depth < self.next_depth_demand()? { return Ok(RetainedCloneStep::Progress(idle)); }
        if let Some(drain) = self.drain.take() { let (outcome, pending) = drain.into_parts(); self.pending = pending.map(semio_framework_os_spr::command::MutationMessageRetirement::new); self.retiring = Some(semio_framework_os_spr::command::MutationMessageLedgerRetirement::from_replay_outcome(outcome)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<semio_framework_os_spr::MutationReplayOutcome>() + size_of::<Option<MutationMessage>>(), ..idle })); }
        if let Some(pending) = self.pending.as_mut() { let progress = pending.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }); if pending.terminal_is_empty() { self.pending = None; } return Ok(RetainedCloneStep::Progress(progress)); }
        if let Some(retiring) = self.retiring.as_mut() { let step = retiring.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?; if retiring.terminal_is_empty() { self.retiring = None; } return Ok(step); }
        Ok(RetainedCloneStep::Complete(idle))
    }
    pub fn terminal_is_empty(&self) -> bool { self.drain.is_none() && self.pending.is_none() && self.retiring.is_none() }
    pub fn close_cold(&mut self) { self.begin_close(); while !self.terminal_is_empty() { self.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }).expect("cold original operation message retirement"); } }
}
impl Drop for OperationMessageSettlement { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "original operation messages must transfer or physically close"); } }

/// 🧭️ Semantic work requested without minting publication metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactReplayPreparationMode {
    Prefix,
    Report,
    Merge,
}

/// 🪪️ Fixed authority retained throughout one semantic preparation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactReplayPreparationContext {
    pub mode: ArtifactReplayPreparationMode,
    pub operation_index: u32,
    pub generation: u64,
    pub base_revision: [u8; 32],
}

/// 📈️ Cumulative operation work exposed without publishing native currency details.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArtifactReplayPreparationProgress {
    pub items: usize,
    pub bytes: usize,
}

/// ⏯️ A producer has completed one bounded unit or exposed its prepared owners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactReplayPreparationStep {
    Blocked,
    Pending(RetainedCloneProgress),
    Prepared(RetainedCloneProgress),
}

impl ArtifactReplayPreparationStep {
    pub fn progress(self) -> RetainedCloneProgress {
        match self {
            Self::Blocked => RetainedCloneProgress::default(),
            Self::Pending(progress) | Self::Prepared(progress) => progress,
        }
    }
}

pub use semio_framework_os_spr::command::ArtifactReplayPrepared;
#[path = "🧬️native/🦀️.rs"]
pub mod native;
pub use native::{ArtifactReplayPreparationRequest,ArtifactMutationSource};


/// 🧩️ A domain cursor splits input admission, cloning, inverse, apply and settlement into granted turns.
pub trait ArtifactReplayPreparation<P, M>: Send {
    fn advance_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn advance(&mut self, body:usize,grant: ArtifactStoreOneItemGrant, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactReplayPreparationStep, ValueError>;
    fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<P, M>>;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant, receiving: &mut dyn semio_framework_value::OriginalRetirementControl) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🏭️ An admitted artifact capability begins with an immutable projection alias in constant work.
pub trait ArtifactReplayPreparationFactory<P, M: Sync>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn preparation_birth_demand(&self) -> semio_framework_value::RetirementDemand;
    fn begin(&self, request: ArtifactReplayPreparationRequest<P, M>, grant: RetainedCloneGrant) -> Result<(Box<dyn ArtifactReplayPreparation<P, M>>, RetainedCloneProgress), (ValueError, ArtifactReplayPreparationRequest<P, M>)>;
}

/// 🛡️ Freshness and deadline fences around the domain's individual preparation units.
pub struct ArtifactReplayPreparationTask<P, M> {
    pub context: ArtifactReplayPreparationContext,
    pub cursor: Box<dyn ArtifactReplayPreparation<P, M>>,
    pub completed: ArtifactReplayPreparationProgress,
    pub cancelled: bool,
    pub ready: bool,
    original_control: Option<(u64, Option<u64>)>,
}

impl<P, M> ArtifactReplayPreparationTask<P, M> {
    pub fn new(context: ArtifactReplayPreparationContext, cursor: Box<dyn ArtifactReplayPreparation<P, M>>) -> Self {
        Self { context, cursor, completed: ArtifactReplayPreparationProgress::default(), cancelled: false, ready: false, original_control: None }
    }

    pub fn step(&mut self, generation: u64, revision: [u8; 32], body:usize, grant: ArtifactStoreOneItemGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, deadline: &mut dyn FnMut() -> bool) -> Result<ArtifactReplayPreparationStep, ValueError> {
        if generation != self.context.generation || revision != self.context.base_revision {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history.operation.stale-preparation"));
        }
        if self.cancelled || !grant.permits_one() || deadline() {
            return Ok(ArtifactReplayPreparationStep::Blocked);
        }
        if self.ready { return Ok(ArtifactReplayPreparationStep::Prepared(RetainedCloneProgress::default())); }
        let identity=(control.ledger_identity(),control.retirement_recipient_identity());
        if self.original_control.is_some_and(|original|original!=identity){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"semantic preparation requires its original decoder and recipient"));}
        self.original_control=Some(identity);
        let step = admit_replay_preparation_step(self.cursor.advance(body,grant,control)?, grant)?;
        let progress = step.progress();
        self.completed.items = self.completed.items.saturating_add(progress.copied_items);
        self.completed.bytes = self.completed.bytes.saturating_add(progress.copied_bytes).saturating_add(progress.retained_capacity_bytes).saturating_add(progress.released_bytes);
        self.ready = matches!(step, ArtifactReplayPreparationStep::Prepared(_));
        Ok(step)
    }

    pub fn validate_retirement_control(&self,control:&dyn semio_framework_value::OriginalRetirementControl)->Result<(),ValueError>{if self.original_control.is_some_and(|original|original!=(control.ledger_identity(),control.retirement_recipient_identity())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"semantic retirement requires its original decoder and recipient"));}Ok(())}
    pub fn progress(&self) -> ArtifactReplayPreparationProgress { self.completed }
    /// 🏁️ Keeps prepared owners in the exact producer until a separately granted task handoff.
    pub fn is_ready(&self)->bool{self.ready&&!self.cancelled}

    pub fn demands(&self, generation: u64, revision: [u8; 32],body:usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if generation != self.context.generation || revision != self.context.base_revision { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history.operation.stale-preparation")); }
        if self.ready || self.cancelled { return Ok(Default::default()); }
        self.cursor.advance_demands(body)
    }

    pub fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<P, M>> { (self.ready && !self.cancelled).then(|| self.cursor.take_prepared()).flatten() }

    pub fn cancel(&mut self) {
        self.cancelled = true;
        self.cursor.cancel();
    }

    pub fn into_retirement(self) -> ArtifactReplayPreparationRetirement<P, M> { ArtifactReplayPreparationRetirement { cursor: { let mut cursor=self.cursor;cursor.cancel();cursor.begin_close();Some(cursor) }, original_control:self.original_control } }
}

/// 🎟️ Validates the producer's actual consumption before any prepared owner is adopted.
pub fn admit_replay_preparation_step(step: ArtifactReplayPreparationStep, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactReplayPreparationStep, ValueError> {
    let progress = step.progress();
    if progress.copied_items > 1 || !progress.fits(grant.retained_grant()) {
        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history.operation.preparation-exceeds-grant").with_retained_progress(progress));
    }
    if !grant.permits_one() && step != ArtifactReplayPreparationStep::Blocked {
        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history.operation.preparation-without-grant").with_retained_progress(progress));
    }
    Ok(step)
}

/// ♻️ Exact cancellation handoff for an active domain semantic cursor.
pub struct ArtifactReplayPreparationRetirement<P, M> {
    pub cursor: Option<Box<dyn ArtifactReplayPreparation<P, M>>>,
    original_control: Option<(u64, Option<u64>)>,
}

impl<P, M> ArtifactReplayPreparationRetirement<P, M> {
    pub fn new(mut cursor: Box<dyn ArtifactReplayPreparation<P, M>>) -> Self {
        cursor.cancel();
        cursor.begin_close();
        Self { cursor: Some(cursor), original_control:None }
    }
}

impl<P, M> ArtifactReplayPreparationRetirement<P, M> {
    pub fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{let Some(cursor)=self.cursor.as_ref()else{return Ok(Default::default())};if cursor.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of_val(cursor.as_ref()),depth:1,..Default::default()});}let mut demand=cursor.close_demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"operation preparation cleanup depth overflow"))?;Ok(demand)}
    pub fn close_step(&mut self, grant: RetainedCloneGrant, receiving: &mut dyn semio_framework_value::OriginalRetirementControl) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        let Some(cursor) = self.cursor.as_ref() else { return Ok(RetainedCloneStep::Complete(empty)) };
        let demand=self.next_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        let identity=(receiving.ledger_identity(),receiving.retirement_recipient_identity());
        if self.original_control.is_some_and(|original|original!=identity){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"semantic retirement requires its original decoder and recipient"));}
        self.original_control=Some(identity);
        if cursor.terminal_is_empty() {
            let bytes = size_of_val(cursor.as_ref());
            drop(self.cursor.take());
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let cursor = self.cursor.as_mut().unwrap();
        let step = cursor.close_step(child, receiving)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, cursor.terminal_is_empty(), "operation preparation payload").map(|step| RetainedCloneStep::Progress(step.progress()))
    }
    pub fn terminal_is_empty(&self) -> bool { self.cursor.is_none() }
    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), |cursor| if cursor.terminal_is_empty() { Ok(0) } else { cursor.next_close_copy_byte_demand() }) }
    pub fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), |cursor| if cursor.terminal_is_empty() { Ok(0) } else { cursor.next_close_capacity_byte_demand(body) }) }
    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), |cursor| if cursor.terminal_is_empty() { Ok(size_of_val(cursor.as_ref())) } else { cursor.next_close_release_byte_demand() }) }
    pub fn next_depth_demand(&self) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), |cursor| if cursor.terminal_is_empty() { Ok(1) } else { cursor.next_close_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "operation preparation cleanup depth overflow")) }) }
}

impl<P, M> Drop for ArtifactReplayPreparationRetirement<P, M> {
    fn drop(&mut self) {
        assert!(self.cursor.is_none() || std::thread::panicking(), "history operation preparation retirement dropped before terminal-empty ownership");
    }
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path="🌱️derived/🦀️.rs"]
pub mod derived_operation;
pub use derived_operation::{ArtifactDerivedOperation,ArtifactDerivedOperationMode};

#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
