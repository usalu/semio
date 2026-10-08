//! 🎮️ Domain-owned cooperative semantic preparation for one history operation.

use crate::os_spr::{InputReplacement, MutationApplyError, MutationMessage};
use super::{ArtifactStoreOneItemGrant, ErasedSnapshotRetirement};
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::sync::Arc;

pub(super) use crate::os_spr::command::{ArtifactReplayRetirementFactory, ReplayRetirement};

/// 📨️ Moves every original operation row into native pages before one final edit selection.
pub(super) struct OperationMessageSettlement {
    drain: Option<crate::os_spr::command::OperationMessageDrain>,
    pending: Option<crate::os_spr::command::MutationMessageRetirement>,
    retiring: Option<crate::os_spr::command::MutationMessageLedgerRetirement>,
    closing: bool,
}
impl OperationMessageSettlement {
    fn new(outcome: crate::os_spr::MutationReplayOutcome) -> Self { Self { drain: Some(crate::os_spr::command::OperationMessageDrain::new(outcome)), pending: None, retiring: None, closing: false } }
    fn step(&mut self, ledger: &mut crate::os_spr::command::ReplayMessageAccumulator) -> Result<bool, ValueError> {
        let drain = self.drain.as_mut().expect("operation retains its original row drain");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: drain.next_copy_byte_demand(ledger), maximum_capacity_bytes: drain.next_capacity_byte_demand(ledger)?, maximum_release_bytes: drain.next_release_byte_demand(), maximum_depth: 1 };
        if grant.maximum_copy_bytes.saturating_add(grant.maximum_capacity_bytes) > 4096 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "operation message page exceeds one body/capacity grant")); }
        drain.step(ledger, grant)?;
        Ok(drain.is_finished())
    }
    fn take(&mut self) -> crate::os_spr::MutationReplayOutcome { let result = self.drain.as_mut().and_then(crate::os_spr::command::OperationMessageDrain::take).expect("completed original operation drain"); self.drain = None; result }
    pub(super) fn begin_close(&mut self) { self.closing = true; }
    pub(super) fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(if self.drain.is_some() { std::mem::size_of::<crate::os_spr::MutationReplayOutcome>() + std::mem::size_of::<Option<MutationMessage>>() } else if let Some(pending) = self.pending.as_ref() { pending.next_copy_byte_demand() } else { self.retiring.as_ref().map_or(0, crate::os_spr::command::MutationMessageLedgerRetirement::next_copy_byte_demand) }) }
    pub(super) fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    pub(super) fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if self.drain.is_some() { return Ok(0); } if let Some(pending) = self.pending.as_ref() { return Ok(pending.next_release_byte_demand()); } self.retiring.as_ref().map_or(Ok(0), crate::os_spr::command::MutationMessageLedgerRetirement::next_release_byte_demand) }
    pub(super) fn next_depth_demand(&self) -> Result<usize, ValueError> { if self.drain.is_some() { return Ok(1); } if let Some(pending) = self.pending.as_ref() { return pending.next_depth_demand().map(|depth| depth + 1); } self.retiring.as_ref().map_or(Ok(0), |owner| owner.next_depth_demand().map(|depth| depth + 1)) }
    pub(super) fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let idle = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 || grant.maximum_copy_bytes < self.next_copy_byte_demand()? || grant.maximum_release_bytes < self.next_release_byte_demand()? || grant.maximum_depth < self.next_depth_demand()? { return Ok(RetainedCloneStep::Progress(idle)); }
        if let Some(drain) = self.drain.take() { let (outcome, pending) = drain.into_parts(); self.pending = pending.map(crate::os_spr::command::MutationMessageRetirement::new); self.retiring = Some(crate::os_spr::command::MutationMessageLedgerRetirement::from_replay_outcome(outcome)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<crate::os_spr::MutationReplayOutcome>() + std::mem::size_of::<Option<MutationMessage>>(), ..idle })); }
        if let Some(pending) = self.pending.as_mut() { let progress = pending.close_step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }); if pending.terminal_is_empty() { self.pending = None; } return Ok(RetainedCloneStep::Progress(progress)); }
        if let Some(retiring) = self.retiring.as_mut() { let step = retiring.close_step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?; if retiring.terminal_is_empty() { self.retiring = None; } return Ok(step); }
        Ok(RetainedCloneStep::Complete(idle))
    }
    pub(super) fn terminal_is_empty(&self) -> bool { self.drain.is_none() && self.pending.is_none() && self.retiring.is_none() }
    pub(super) fn close_cold(&mut self) { self.begin_close(); while !self.terminal_is_empty() { self.close_step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }).expect("cold original operation message retirement"); } }
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

pub use crate::os_spr::command::ArtifactReplayPrepared;

/// 🧩️ A domain cursor splits input admission, cloning, inverse, apply and settlement into granted turns.
pub trait ArtifactReplayPreparation<P, M>: Send {
    fn next_advance_copy_demand(&self, original: &M, replacement: Option<&InputReplacement>) -> Result<usize, ValueError>;
    fn next_advance_capacity_demand(&self, original: &M, replacement: Option<&InputReplacement>, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    fn next_advance_release_demand(&self, original: &M, replacement: Option<&InputReplacement>) -> Result<usize, ValueError>;
    fn next_advance_depth_demand(&self, original: &M, replacement: Option<&InputReplacement>) -> Result<usize, ValueError>;
    fn advance(&mut self, original: &M, replacement: Option<&InputReplacement>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactReplayPreparationStep, String>;
    fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<P, M>>;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🏭️ An admitted artifact capability begins with an immutable projection alias in constant work.
pub trait ArtifactReplayPreparationFactory<P, M>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn preparation_birth_bytes(&self, base: &Arc<P>, context: ArtifactReplayPreparationContext) -> usize;
    fn begin(&self, base: Arc<P>, context: ArtifactReplayPreparationContext, grant: RetainedCloneGrant) -> Result<(Box<dyn ArtifactReplayPreparation<P, M>>, RetainedCloneProgress), (ValueError, Arc<P>)>;
}

/// 🛡️ Freshness and deadline fences around the domain's individual preparation units.
pub struct ArtifactReplayPreparationTask<P, M> {
    context: ArtifactReplayPreparationContext,
    cursor: Box<dyn ArtifactReplayPreparation<P, M>>,
    completed: ArtifactReplayPreparationProgress,
    cancelled: bool,
    ready: bool,
}

impl<P, M> ArtifactReplayPreparationTask<P, M> {
    pub fn new(context: ArtifactReplayPreparationContext, cursor: Box<dyn ArtifactReplayPreparation<P, M>>) -> Self {
        Self { context, cursor, completed: ArtifactReplayPreparationProgress::default(), cancelled: false, ready: false }
    }

    pub fn step(&mut self, original: &M, replacement: Option<&InputReplacement>, generation: u64, revision: [u8; 32], grant: ArtifactStoreOneItemGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ArtifactReplayPreparationStep, String> {
        if generation != self.context.generation || revision != self.context.base_revision {
            return Err("history.operation.stale-preparation".into());
        }
        if self.cancelled || !grant.permits_one() || deadline() {
            return Ok(ArtifactReplayPreparationStep::Blocked);
        }
        if self.ready { return Ok(ArtifactReplayPreparationStep::Prepared(RetainedCloneProgress::default())); }
        let step = admit_replay_preparation_step(self.cursor.advance(original, replacement, grant)?, grant)?;
        let progress = step.progress();
        self.completed.items = self.completed.items.saturating_add(progress.copied_items);
        self.completed.bytes = self.completed.bytes.saturating_add(progress.copied_bytes).saturating_add(progress.retained_capacity_bytes).saturating_add(progress.released_bytes);
        self.ready = matches!(step, ArtifactReplayPreparationStep::Prepared(_));
        Ok(step)
    }

    pub fn progress(&self) -> ArtifactReplayPreparationProgress { self.completed }

    pub fn next_grant(&self, original: &M, replacement: Option<&InputReplacement>, generation: u64, revision: [u8; 32]) -> Result<ArtifactStoreOneItemGrant, ValueError> {
        if generation != self.context.generation || revision != self.context.base_revision { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history.operation.stale-preparation")); }
        if self.ready || self.cancelled { return Ok(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 0 }); }
        let copy = self.cursor.next_advance_copy_demand(original, replacement)?;
        let capacity = self.cursor.next_advance_capacity_demand(original, replacement, 4096usize.saturating_sub(copy))?;
        let depth = self.cursor.next_advance_depth_demand(original, replacement)?;
        if copy.saturating_add(capacity) > 4096 || depth > 64 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "operation semantic work exceeds one body/capacity/depth grant")); }
        Ok(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: self.cursor.next_advance_release_demand(original, replacement)?, maximum_depth: depth.max(1) })
    }

    pub fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<P, M>> { (self.ready && !self.cancelled).then(|| self.cursor.take_prepared()).flatten() }

    pub fn cancel(&mut self) {
        self.cancelled = true;
        self.cursor.cancel();
    }

    pub fn into_retirement(self) -> ArtifactReplayPreparationRetirement<P, M> { ArtifactReplayPreparationRetirement::new(self.cursor) }
}

/// 🎟️ Validates the producer's actual consumption before any prepared owner is adopted.
pub fn admit_replay_preparation_step(step: ArtifactReplayPreparationStep, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactReplayPreparationStep, String> {
    let progress = step.progress();
    if progress.copied_items > 1 || !progress.fits(grant.retained_grant()) || progress.copied_bytes.saturating_add(progress.retained_capacity_bytes) > 4096 {
        return Err("history.operation.preparation-exceeds-grant".into());
    }
    if !grant.permits_one() && step != ArtifactReplayPreparationStep::Blocked {
        return Err("history.operation.preparation-without-grant".into());
    }
    Ok(step)
}

/// ♻️ Exact cancellation handoff for an active domain semantic cursor.
pub struct ArtifactReplayPreparationRetirement<P, M> {
    cursor: Option<Box<dyn ArtifactReplayPreparation<P, M>>>,
}

impl<P, M> ArtifactReplayPreparationRetirement<P, M> {
    pub fn new(mut cursor: Box<dyn ArtifactReplayPreparation<P, M>>) -> Self {
        cursor.cancel();
        cursor.begin_close();
        Self { cursor: Some(cursor) }
    }
}

impl<P, M> ErasedSnapshotRetirement for ArtifactReplayPreparationRetirement<P, M> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        let Some(cursor) = self.cursor.as_ref() else { return Ok(RetainedCloneStep::Complete(empty)) };
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < self.next_copy_byte_demand()? || grant.maximum_capacity_bytes < self.next_capacity_byte_demand(grant.maximum_copy_bytes)? || grant.maximum_release_bytes < self.next_release_byte_demand()? || grant.maximum_depth < self.next_depth_demand()? { return Ok(RetainedCloneStep::Progress(empty)); }
        if cursor.terminal_is_empty() {
            let bytes = std::mem::size_of_val(cursor.as_ref());
            drop(self.cursor.take());
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let cursor = self.cursor.as_mut().unwrap();
        let step = cursor.close_step(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, cursor.terminal_is_empty(), "operation preparation payload").map(|step| RetainedCloneStep::Progress(step.progress()))
    }
    fn terminal_is_empty(&self) -> bool { self.cursor.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), |cursor| if cursor.terminal_is_empty() { Ok(0) } else { cursor.next_close_copy_byte_demand() }) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), |cursor| if cursor.terminal_is_empty() { Ok(0) } else { cursor.next_close_capacity_byte_demand(body) }) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), |cursor| if cursor.terminal_is_empty() { Ok(std::mem::size_of_val(cursor.as_ref())) } else { cursor.next_close_release_byte_demand() }) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), |cursor| if cursor.terminal_is_empty() { Ok(1) } else { cursor.next_close_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "operation preparation cleanup depth overflow")) }) }
}

impl<P, M> Drop for ArtifactReplayPreparationRetirement<P, M> {
    fn drop(&mut self) {
        assert!(self.cursor.is_none() || std::thread::panicking(), "history operation preparation retirement dropped before terminal-empty ownership");
    }
}

impl<P, M> super::ReplayOwnedState<P, M>
where M: crate::os_spr::Mutation<P> + crate::os_spr::OpBinary {
    /// ♻️ Binds both installed cleanup authorities without imposing native bounds on cold replay construction.
    pub fn with_retirement_factories(mut self, snapshots: Arc<dyn super::ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn super::ArtifactOwnedValueRetirementFactory<M>>) -> Self
    where P: Send + Sync + 'static, M: Send + 'static {
        self.replay_retirement_factory = Some(crate::os_spr::command::registered_replay_retirement_factory(snapshots, mutations));
        self
    }

    /// 🎟️ Exposes the next exact allocation release separately from the ordinary body-byte grant.
    pub fn replay_retirement_release_byte_demand(&self) -> Result<usize, ValueError> {
        self.replay_retirement.as_ref().map_or(Ok(0), ReplayRetirement::next_release_byte_demand)
    }

    pub(super) fn begin_replay_snapshot_retirement(&mut self, owner: Arc<P>) {
        if let Some(factory) = self.replay_retirement_factory.as_ref() {
            self.replay_retirement.get_or_insert_with(|| ReplayRetirement::new(Arc::clone(factory))).stage_snapshot(owner);
        } else { super::retire_shared_projection::<P, M>(owner); }
    }

    pub(super) fn begin_replay_mutations_retirement(&mut self, owners: semio_framework_value::list::PagedList<M, {usize::MAX}>) {
        if let Some(factory) = self.replay_retirement_factory.as_ref() {
            self.replay_retirement.get_or_insert_with(|| ReplayRetirement::new(Arc::clone(factory))).stage_mutations(owners);
        } else { super::retire_scratch_operations::<P, M>(owners); }
    }

    /// 🎮️ Selects the admitted domain producer for every prefix and suffix operation.
    pub fn with_operation_preparation(mut self, factory: Arc<dyn ArtifactReplayPreparationFactory<P, M>>) -> Self {
        self.operation_preparation_factory = Some(factory);
        self
    }

    /// 📈️ Monotone semantic work survives each completed domain cursor's retirement.
    pub fn operation_preparation_progress(&self) -> ArtifactReplayPreparationProgress {
        let active = self.operation_preparation.as_ref().map_or(ArtifactReplayPreparationProgress::default(), ArtifactReplayPreparationTask::progress);
        ArtifactReplayPreparationProgress { items: self.preparation_completed.items.saturating_add(active.items), bytes: self.preparation_completed.bytes.saturating_add(active.bytes) }
    }

    pub(super) fn step_prepared_operation(&mut self, edit: &crate::os_spr::Edit<M>, prefix: bool, _deadline: &mut dyn FnMut() -> bool) -> Result<bool, crate::os_vcs::VcsError> {
        use super::{ReplayMode, CursorRevisionAccumulator};
        use crate::os_vcs::VcsError;
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 };
        if let Some(settlement) = self.operation_message_settlement.as_mut() {
            if settlement.step(&mut self.edit_messages).map_err(VcsError::InverseRefused)? {
                let outcome = settlement.take();
                self.report_worst = self.report_worst.max(outcome.worst);
                self.outcomes.push(outcome);
                self.operation_message_settlement = None;
                return Ok(true);
            }
            return Ok(false);
        }
        if let Some(retirement) = self.operation_retirement.as_mut() {
            let copy = retirement.next_copy_byte_demand().map_err(VcsError::InverseRefused)?;
            let capacity = retirement.next_capacity_byte_demand(copy).map_err(VcsError::InverseRefused)?;
            if copy.saturating_add(capacity) > 4096 { return Err(VcsError::ValidationFailed("operation cleanup exceeds one native body/capacity grant".into())); }
            retirement.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: retirement.next_release_byte_demand().map_err(VcsError::InverseRefused)?, maximum_depth: retirement.next_depth_demand().map_err(VcsError::InverseRefused)? }).map_err(VcsError::InverseRefused)?;
            if retirement.terminal_is_empty() {
                self.operation_retirement = None;
            }
            return Ok(false);
        }
        let index = self.operation;
        let mutation_id = crate::os_spr::mutation_id_for_edit_operation::<P, M>(edit, index).expect("recorded operation identity");
        if self.prepared_operation.is_none() {
            if self.operation_preparation.is_none() {
                let context = ArtifactReplayPreparationContext {
                    mode: if prefix { ArtifactReplayPreparationMode::Prefix } else { match self.mode { ReplayMode::Report => ArtifactReplayPreparationMode::Report, ReplayMode::Merge(_) => ArtifactReplayPreparationMode::Merge } },
                    operation_index: index as u32,
                    generation: self.generation,
                    base_revision: self.revision,
                };
                let base = if prefix || self.mode == ReplayMode::Report { self.state.as_ref() } else { self.candidate.as_ref().or(self.state.as_ref()) }.ok_or_else(|| VcsError::ValidationFailed("operation preparation lost its base".into()))?;
                let factory = self.operation_preparation_factory.as_ref().expect("admitted operation factory");
                let bytes = factory.preparation_birth_bytes(base, context);
                if bytes > 4096 { return Err(VcsError::ValidationFailed("operation preparation native constructor exceeds one capacity grant".into())); }
                let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: bytes, maximum_release_bytes: 0, maximum_depth: 1 };
                let (cursor, progress) = factory.begin(base.clone(), context, birth).map_err(|(error, base)| { drop(base); VcsError::InverseRefused(error) })?;
                self.operation_preparation = Some(ArtifactReplayPreparationTask::new(context, cursor));
                if !progress.fits(birth) || progress.retained_capacity_bytes != bytes { return Err(VcsError::ValidationFailed("operation preparation changed its admitted constructor receipt".into())); }
                return Ok(false);
            }
            let task = self.operation_preparation.as_mut().expect("active operation preparation");
            let replacement = self.supersessions.get(&mutation_id).map(|entry| &entry.replacement);
            let grant = task.next_grant(&edit.forwards[index], replacement, self.generation, self.revision).map_err(VcsError::InverseRefused)?;
            let step = task.step(&edit.forwards[index], replacement, self.generation, self.revision, grant, &mut || false).map_err(VcsError::ValidationFailed)?;
            if matches!(step, ArtifactReplayPreparationStep::Prepared(_)) {
                let work = task.progress();
                self.preparation_completed.items = self.preparation_completed.items.saturating_add(work.items);
                self.preparation_completed.bytes = self.preparation_completed.bytes.saturating_add(work.bytes);
                self.prepared_operation = Some(task.take_prepared().ok_or_else(|| VcsError::ValidationFailed("prepared operation has no owners".into()))?);
                self.operation_retirement = Some(self.operation_preparation.take().expect("prepared operation cursor").into_retirement());
            }
            return Ok(false);
        }
        let prepared = self.prepared_operation.as_mut().expect("settled operation owners");
        if !prefix && prepared.next.is_some() && prepared.apply_refusal.is_none() && prepared.input_refusal.is_none() {
            if let Ok(inverse) = prepared.inverse.as_mut() {
                if !inverse.is_empty() {
                    if !self.edit_inverse.has_reserved_slot() {
                        let demand = self.edit_inverse.next_allocation_bytes().map_err(|error| VcsError::InverseRefused(error.into()))?;
                        if demand > grant.maximum_copy_bytes { return Err(VcsError::ValidationFailed("replay inverse page exceeds its body grant".into())); }
                        self.edit_inverse.reserve_one(demand).map_err(|error| VcsError::InverseRefused(error.refusal().into()))?;
                        return Ok(false);
                    }
                    let operation = inverse.pop().expect("prepared inverse owner");
                    if let Err(operation) = self.edit_inverse.push_reserved(operation) {
                        inverse.push_reserved(operation).unwrap_or_else(|_| panic!("unchanged prepared inverse slot"));
                        return Err(VcsError::ValidationFailed("replay inverse lost its admitted slot".into()));
                    }
                    return Ok(false);
                }
            }
        }
        let mut prepared = self.prepared_operation.take().expect("settled operation owners");
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        if edit.forwards[index].may_emit_foreign_steps() { self.unit_flags.insert((digest, index), prepared.foreign_steps); }
        let inverse = match std::mem::replace(&mut prepared.inverse, Ok(Default::default())) {
            Ok(inverse) => inverse,
            Err(cause) if prefix => { drop(cause); Default::default() },
            Err(cause) if self.mode == ReplayMode::Report => {
                prepared.messages.push(MutationMessage::fatal("mutation.inverse-refused", format!("the operation has no inverse on the replayed state: {cause}")).at([mutation_id.0.clone()]));
                Default::default()
            }
            Err(cause) => {
                if let Some(next) = prepared.next.take() { self.begin_replay_snapshot_retirement(next); }
                self.prepared_operation = Some(prepared);
                return Err(VcsError::InverseRefused(cause));
            }
        };
        if let Some(cause) = prepared.apply_refusal.take() {
            if !prefix && self.mode != ReplayMode::Report {
                if let Some(next) = prepared.next.take() { self.begin_replay_snapshot_retirement(next); }
                self.begin_replay_mutations_retirement(inverse);
                self.prepared_operation = Some(prepared);
                return Err(cause.into());
            }
            if let Some(next) = prepared.next.take() { self.begin_replay_snapshot_retirement(next); }
            prepared.messages.push(MutationMessage::fatal(cause.code, cause.message).at(cause.target));
        }
        let invalid = prepared.input_refusal.is_some();
        if let Some(reason) = prepared.input_refusal.take() {
            if let Some(next) = prepared.next.take() { self.begin_replay_snapshot_retirement(next); }
            let transition_id = self.supersessions.get(&mutation_id).map_or("", |entry| entry.transition_id.as_str());
            prepared.messages.push(MutationMessage::fatal("mutation.invariant", format!("supersession {transition_id} of {} {reason}; the operation folds as a no-op", mutation_id.0)).at([mutation_id.0.clone()]));
        }
        match prepared.next.take() {
            Some(next) => {
                self.begin_replay_mutations_retirement(inverse);
                let displaced = if !prefix && self.mode != ReplayMode::Report { self.candidate.replace(next) } else { self.state.replace(next) };
                if let Some(displaced) = displaced { self.begin_replay_snapshot_retirement(displaced); }
            }
            None => self.begin_replay_mutations_retirement(inverse),
        }
        if prefix {
            if !prepared.messages.is_empty() || prepared.messages.capacity() != 0 {
                assert!(self.prefix_message_retirement.is_none(), "prefix diagnostic cleanup must close before another operation");
                self.prefix_message_retirement = Some(super::ArtifactStoreMessageLedgerRetirement::new(String::new(), std::mem::take(&mut prepared.messages)));
            }
            return Ok(true);
        }
        let supersession = self.supersessions.get(&mutation_id);
        self.operation_message_settlement = Some(OperationMessageSettlement::new(crate::os_spr::MutationReplayOutcome {
            mutation_id,
            edit_id: edit.id.clone(),
            op_index: index as u32,
            worst: None,
            messages: prepared.messages,
            superseded: supersession.is_some(),
            withdrawn: !invalid && supersession.is_some_and(|entry| matches!(entry.replacement, InputReplacement::Withdrawn)),
        }));
        Ok(false)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
