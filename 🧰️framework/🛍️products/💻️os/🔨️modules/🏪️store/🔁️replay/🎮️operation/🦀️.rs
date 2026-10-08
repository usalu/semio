//! 🎮️ Domain-owned cooperative semantic preparation for one history operation.

use crate::os_spr::{InputReplacement, MutationApplyError, MutationMessage};
use super::{ArtifactStoreOneItemGrant, ErasedSnapshotRetirement, SnapshotRetirementStep};
use semio_framework_value::ValueError;
use std::sync::Arc;

#[path = "📨️messages/🦀️.rs"]
pub mod messages;

#[path = "♻️retirement/🦀️.rs"]
mod retirement;
pub(super) use retirement::ArtifactReplayRetirementFactory;

/// 📨️ Preserves the complete severity witness while bounding and copying the operation's visible ledger.
pub(super) struct OperationMessageSettlement {
    outcome: Option<crate::os_spr::MutationReplayOutcome>,
    clamp: Option<super::edit_message_clamp::EditMessageClamp>,
    copy: Option<messages::MessageCopyCursor>,
    index: usize,
    measuring: bool,
    closing_copy: bool,
}

impl OperationMessageSettlement {
    fn new(outcome: crate::os_spr::MutationReplayOutcome) -> Self {
        Self { outcome: Some(outcome), clamp: None, copy: None, index: 0, measuring: true, closing_copy: false }
    }

    fn step(&mut self, ledger: &mut Vec<MutationMessage>) -> Result<bool, ValueError> {
        let outcome = self.outcome.as_mut().expect("active operation outcome");
        if self.measuring {
            if let Some(message) = outcome.messages.get_mut(self.index) {
                message.op_index = Some(outcome.op_index);
                outcome.worst = outcome.worst.max(Some(message.level));
                self.index += 1;
                return Ok(false);
            }
            self.measuring = false;
            self.index = 0;
            self.clamp = Some(super::edit_message_clamp::EditMessageClamp::new(&outcome.edit_id, &outcome.messages));
            return Ok(false);
        }
        if let Some(clamp) = self.clamp.as_mut() {
            if clamp.step(&mut outcome.messages, 4096)? { self.clamp = None; }
            return Ok(false);
        }
        if self.closing_copy {
            let copy = self.copy.as_mut().expect("closing operation message copy");
            if copy.close_step(1, 4096)? == SnapshotRetirementStep::Complete {
                self.copy = None;
                self.closing_copy = false;
                self.index += 1;
            }
            return Ok(false);
        }
        let Some(source) = outcome.messages.get(self.index) else { return Ok(true) };
        if self.copy.is_none() { self.copy = Some(messages::MessageCopyCursor::new()); return Ok(false); }
        let copy = self.copy.as_mut().expect("active operation message copy");
        if matches!(copy.advance(source, messages::MessageCopyGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096 })?, messages::MessageCopyStep::Complete(_)) {
            ledger.push(copy.take().expect("complete operation message copy"));
            copy.begin_close();
            self.closing_copy = true;
        }
        Ok(false)
    }

    fn take(&mut self) -> crate::os_spr::MutationReplayOutcome { self.outcome.take().expect("completed operation outcome") }

    pub(super) fn retire_item(&mut self) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        if let Some(copy) = self.copy.take() { return Some(copy.into_retirement()); }
        if let Some(clamp) = self.clamp.as_mut() {
            if let Some(child) = clamp.retire_item() { return Some(child); }
            clamp.finish_retirement();
            self.clamp = None;
        }
        if let Some(outcome) = self.outcome.take() {
            return Some(Box::new(super::ArtifactStoreMessageLedgerRetirement {
                state: std::mem::ManuallyDrop::new(Some(super::ArtifactStoreMessageLedgerRetirementState {
                    strings: [Some(outcome.mutation_id.0), Some(outcome.edit_id), None], messages: outcome.messages, targets: Vec::new(), active_bytes: None,
                })),
            }));
        }
        None
    }

    pub(super) fn close_cold(&mut self) {
        while let Some(mut child) = self.retire_item() {
            while child.close_step(1, usize::MAX).expect("cold operation message settlement retirement") != SnapshotRetirementStep::Complete {}
        }
    }
}

impl Drop for OperationMessageSettlement {
    fn drop(&mut self) { assert!(self.outcome.is_none() && self.clamp.is_none() && self.copy.is_none() || std::thread::panicking(), "operation messages dropped before exact settlement retirement"); }
}

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

/// 📈️ Actual work consumed by one turn; payload and owner scaffolds both use byte credit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArtifactReplayPreparationProgress {
    pub items: usize,
    pub bytes: usize,
}

/// ⏯️ A producer has completed one bounded unit or exposed its prepared owners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactReplayPreparationStep {
    Blocked,
    Pending(ArtifactReplayPreparationProgress),
    Prepared(ArtifactReplayPreparationProgress),
}

impl ArtifactReplayPreparationStep {
    pub fn progress(self) -> ArtifactReplayPreparationProgress {
        match self {
            Self::Blocked => ArtifactReplayPreparationProgress::default(),
            Self::Pending(progress) | Self::Prepared(progress) => progress,
        }
    }
}

/// 🧳️ Semantic owners produced against the immutable base; inverse and apply refusals are independent.
pub struct ArtifactReplayPrepared<P, M> {
    pub next: Option<Arc<P>>,
    pub inverse: Result<semio_framework_value::list::PagedList<M, {usize::MAX}>, ValueError>,
    pub messages: Vec<MutationMessage>,
    pub apply_refusal: Option<MutationApplyError>,
    pub input_refusal: Option<String>,
    pub foreign_steps: bool,
}

/// 🧩️ A domain cursor splits input admission, cloning, inverse, apply and settlement into granted turns.
pub trait ArtifactReplayPreparation<P, M>: Send {
    fn advance(&mut self, original: &M, replacement: Option<&InputReplacement>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactReplayPreparationStep, String>;
    fn take_prepared(&mut self) -> Option<ArtifactReplayPrepared<P, M>>;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🏭️ An admitted artifact capability begins with an immutable projection alias in constant work.
pub trait ArtifactReplayPreparationFactory<P, M>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn begin(&self, base: Arc<P>, context: ArtifactReplayPreparationContext) -> Result<Box<dyn ArtifactReplayPreparation<P, M>>, String>;
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
        if self.ready { return Ok(ArtifactReplayPreparationStep::Prepared(ArtifactReplayPreparationProgress::default())); }
        let step = admit_replay_preparation_step(self.cursor.advance(original, replacement, grant)?, grant)?;
        let progress = step.progress();
        self.completed.items = self.completed.items.saturating_add(progress.items);
        self.completed.bytes = self.completed.bytes.saturating_add(progress.bytes);
        self.ready = matches!(step, ArtifactReplayPreparationStep::Prepared(_));
        Ok(step)
    }

    pub fn progress(&self) -> ArtifactReplayPreparationProgress { self.completed }

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
    if progress.items > grant.maximum_items.min(1) || progress.bytes > grant.maximum_bytes {
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
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        let Some(cursor) = self.cursor.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        let step = cursor.close_step(ArtifactStoreOneItemGrant { maximum_items: maximum_items.min(1), maximum_bytes })?;
        if matches!(step, SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > maximum_items.min(1) || released_bytes > maximum_bytes) {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "history operation preparation retirement exceeds its grant"));
        }
        if step == SnapshotRetirementStep::Complete {
            if !cursor.terminal_is_empty() {
                return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "history operation preparation closed with live owners"));
            }
            self.cursor = None;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool { self.cursor.is_none() }
}

impl<P, M> Drop for ArtifactReplayPreparationRetirement<P, M> {
    fn drop(&mut self) {
        assert!(self.cursor.is_none() || std::thread::panicking(), "history operation preparation retirement dropped before terminal-empty ownership");
    }
}

impl<P, M> super::EditReplay<P, M>
where M: crate::os_spr::Mutation<P> + crate::os_spr::OpBinary {
    /// ♻️ Binds both installed cleanup authorities without imposing native bounds on cold replay construction.
    pub fn with_retirement_factories(mut self, snapshots: Arc<dyn super::ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn super::ArtifactOwnedValueRetirementFactory<M>>) -> Self
    where P: Send + Sync + 'static, M: Send + 'static {
        self.replay_retirement_factory = Some(retirement::registered(snapshots, mutations));
        self
    }

    /// 🎟️ Exposes the next exact allocation release separately from the ordinary body-byte grant.
    pub fn replay_retirement_byte_demand(&self) -> usize {
        self.replay_retirement.as_ref().map_or(0, |owner| owner.next_close_byte_demand())
    }

    pub(super) fn begin_replay_snapshot_retirement(&mut self, owner: Arc<P>) {
        if let Some(factory) = self.replay_retirement_factory.as_ref() {
            let child = factory.snapshot(owner);
            self.replay_retirement = Some(match self.replay_retirement.take() { Some(previous) => retirement::chain(previous, child), None => child });
        } else { super::retire_shared_projection::<P, M>(owner); }
    }

    pub(super) fn begin_replay_mutations_retirement(&mut self, owners: semio_framework_value::list::PagedList<M, {usize::MAX}>) {
        if let Some(factory) = self.replay_retirement_factory.as_ref() {
            let child = factory.mutations(owners);
            self.replay_retirement = Some(match self.replay_retirement.take() { Some(previous) => retirement::chain(previous, child), None => child });
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
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 };
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
            if retirement.close_step(1, grant.maximum_bytes).map_err(VcsError::InverseRefused)? == SnapshotRetirementStep::Complete {
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
                let cursor = self.operation_preparation_factory.as_ref().expect("admitted operation factory").begin(base.clone(), context).map_err(VcsError::ValidationFailed)?;
                self.operation_preparation = Some(ArtifactReplayPreparationTask::new(context, cursor));
                return Ok(false);
            }
            let task = self.operation_preparation.as_mut().expect("active operation preparation");
            let replacement = self.supersessions.get(&mutation_id).map(|entry| &entry.replacement);
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
                        if demand > grant.maximum_bytes { return Err(VcsError::ValidationFailed("replay inverse page exceeds its body grant".into())); }
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
                let child = Box::new(super::ArtifactStoreMessageLedgerRetirement::new(String::new(), std::mem::take(&mut prepared.messages)));
                self.replay_retirement = Some(match self.replay_retirement.take() { Some(previous) => retirement::chain(previous, child), None => child });
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
