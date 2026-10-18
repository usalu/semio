//! 🗃️ Progress overlay store: the overlay store operations and the checkout with its drop. See [`super`].

use super::*;

//#region 🎨️JobProgressOverlay
impl JobProgressOverlayStore {
    pub fn new() -> Self {
        Self {
            active: std::array::from_fn(|_| JobProgressSlot::vacant()),
            retirements: std::array::from_fn(|_| JobProgressRetirementSlot::vacant()),
            closing_actors: [None; JOB_PROGRESS_ACTIVE_CAPACITY],
            close_all: false,
            next_token: 1,
            owned_items: 0,
            owned_bytes: 0,
            reserved_items: 0,
            reserved_bytes: 0,
        }
    }

    pub fn live_authority(&self, actor: ActorId, job: u64) -> Option<JobProgressLiveAuthority> {
        self.active.iter().find(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job).map(|slot| JobProgressLiveAuthority::new(slot.identity.operation, slot.identity.base_revision, slot.identity.generation))
    }

    pub fn has_visible_preview(&self, actor: ActorId, job: u64) -> bool {
        self.active.iter().any(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job && slot.preview.is_some())
    }

    pub fn owned_items(&self) -> usize {
        self.owned_items
    }

    pub fn owned_bytes(&self) -> usize {
        self.owned_bytes
    }

    pub fn begin_operation(&mut self, actor: ActorId, job: u64, live: JobProgressLiveAuthority) -> Result<(), JobProgressFault> {
        if self.close_all || self.closing_actors.iter().flatten().any(|closing| *closing == actor) {
            return Err(JobProgressFault::Cancelled);
        }
        if self.active.iter().any(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job) {
            return Err(JobProgressFault::Busy);
        }
        let slot = self.active.iter_mut().find(|slot| !slot.occupied && slot.reservation.is_none()).ok_or(JobProgressFault::Capacity)?;
        slot.epoch = slot.epoch.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
        slot.occupied = true;
        slot.published = false;
        slot.identity = JobProgressIdentity { actor, job, operation: live.operation, base_revision: live.base_revision, generation: live.generation, step_sequence: 0, preview_sequence: 0 };
        Ok(())
    }

    pub fn preflight(&mut self, cx: &mut semio_framework_job::StepContext<'_>, actor: ActorId, publication: &JobPublication, live: JobProgressLiveAuthority) -> Result<JobProgressAdmission, JobProgressFault> {
        let identity = JobProgressIdentity::from_publication(actor, publication);
        if cx.is_cancelled() || self.close_all || self.closing_actors.iter().flatten().any(|closing| *closing == actor) {
            return Err(JobProgressFault::Cancelled);
        }
        if cx.should_yield() {
            return Err(JobProgressFault::Budget);
        }
        if cx.operation().0 != identity.operation || cx.generation().0 != identity.generation || !live.accepts(identity) {
            return Err(JobProgressFault::Stale);
        }
        let (owner_items, owner_bytes, _) = publication_owner_shape(publication, true)?;
        let kind = JobProgressKind::of(&publication.outcome);
        let active_index = self.active.iter().position(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == publication.turn.job).ok_or(JobProgressFault::Missing)?;
        let active_epoch = self.active[active_index].epoch;
        if self.active[active_index].checked_out {
            return Err(JobProgressFault::CheckedOut);
        }
        if self.active[active_index].awaiting_ack || self.active[active_index].reservation.is_some() {
            return Err(JobProgressFault::Busy);
        }
        if active_epoch == u64::MAX {
            return Err(JobProgressFault::Exhausted);
        }
        if !self.active[active_index].identity.same_operation(identity) {
            return Err(JobProgressFault::Stale);
        }
        let prior_preview_sequence = if self.active[active_index].published {
            let expected_step = self.active[active_index].identity.step_sequence.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
            if identity.step_sequence != expected_step {
                return Err(JobProgressFault::StepSequence);
            }
            self.active[active_index].identity.preview_sequence
        } else {
            if identity.step_sequence != 0 {
                return Err(JobProgressFault::StepSequence);
            }
            0
        };
        let expected_preview = if kind == JobProgressKind::Preview { prior_preview_sequence.checked_add(1).ok_or(JobProgressFault::Exhausted)? } else { prior_preview_sequence };
        if identity.preview_sequence != expected_preview {
            return Err(JobProgressFault::PreviewSequence);
        }
        let has_preview = self.active[active_index].preview.is_some();
        let adoption_retirements = match kind {
            JobProgressKind::Preview => usize::from(has_preview),
            JobProgressKind::Checkpoint => 1,
            JobProgressKind::CommitValidated | JobProgressKind::Fault => 1 + usize::from(has_preview),
            JobProgressKind::Cancelled => usize::from(has_preview),
            JobProgressKind::Yield => 0,
        };
        let staged_preview = kind == JobProgressKind::Preview;
        let retirement_count = adoption_retirements + usize::from(staged_preview);
        if retirement_count > 2 {
            return Err(JobProgressFault::Capacity);
        }
        let mut retirement_indices = [usize::MAX; 2];
        let mut found = 0usize;
        for (index, retirement) in self.retirements.iter().enumerate() {
            if retirement.owner.is_none() && retirement.reservation.is_none() {
                retirement_indices[found] = index;
                found += 1;
                if found == retirement_count {
                    break;
                }
            }
        }
        if found != retirement_count {
            return Err(JobProgressFault::Capacity);
        }
        let next_items = self.owned_items.checked_add(self.reserved_items).and_then(|total| total.checked_add(owner_items)).ok_or(JobProgressFault::Capacity)?;
        let next_bytes = self.owned_bytes.checked_add(self.reserved_bytes).and_then(|total| total.checked_add(owner_bytes)).ok_or(JobProgressFault::Capacity)?;
        if next_items > JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS || next_bytes > JOB_PROGRESS_TOTAL_MAXIMUM_BYTES {
            return Err(JobProgressFault::Capacity);
        }
        let token = self.next_token;
        self.next_token = self.next_token.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
        self.active[active_index].reservation = Some(token);
        for index in retirement_indices.iter().copied().take(retirement_count) {
            self.retirements[index].reservation = Some(token);
        }
        self.reserved_items += owner_items;
        self.reserved_bytes += owner_bytes;
        cx.consume_fuel(1);
        Ok(JobProgressAdmission { token, active_index, active_epoch, identity, live, retirement_indices, abort_retirement: staged_preview.then_some(retirement_indices[adoption_retirements]), owner_items, owner_bytes })
    }

    #[expect(clippy::needless_pass_by_value, reason = "Cancellation consumes the single-use reservation authority even though its accounting fields are copied.")]
    pub fn cancel_admission(&mut self, admission: JobProgressAdmission) -> Result<(), JobProgressFault> {
        let slot = self.active.get(admission.active_index).ok_or(JobProgressFault::Token)?;
        if slot.epoch != admission.active_epoch || slot.reservation != Some(admission.token) {
            return Err(JobProgressFault::Token);
        }
        for index in admission.retirement_indices.iter().copied().filter(|index| *index != usize::MAX) {
            let retirement = self.retirements.get(index).ok_or(JobProgressFault::Token)?;
            if retirement.reservation != Some(admission.token) || retirement.owner.is_some() {
                return Err(JobProgressFault::Token);
            }
        }
        self.active[admission.active_index].reservation = None;
        for index in admission.retirement_indices.iter().copied().filter(|index| *index != usize::MAX) {
            let retirement = &mut self.retirements[index];
            retirement.reservation = None;
        }
        self.reserved_items = self.reserved_items.saturating_sub(admission.owner_items);
        self.reserved_bytes = self.reserved_bytes.saturating_sub(admission.owner_bytes);
        Ok(())
    }

    pub fn publish_reserved(&mut self, cx: &mut semio_framework_job::StepContext<'_>, admission: JobProgressAdmission, publication: JobPublication, live: JobProgressLiveAuthority) -> Result<JobProgressReceipt, JobProgressRejected> {
        let identity = JobProgressIdentity::from_publication(admission.identity.actor, &publication);
        let slot_valid = self.active.get(admission.active_index).is_some_and(|slot| slot.epoch == admission.active_epoch && slot.reservation == Some(admission.token));
        if cx.is_cancelled() || cx.operation().0 != identity.operation || cx.generation().0 != identity.generation {
            let _ = self.cancel_admission(admission);
            return Err(JobProgressRejected::new(JobProgressFault::Cancelled, publication));
        }
        if !slot_valid || identity != admission.identity || live != admission.live || !live.accepts(identity) {
            let _ = self.cancel_admission(admission);
            return Err(JobProgressRejected::new(JobProgressFault::Stale, publication));
        }
        let kind = JobProgressKind::of(&publication.outcome);
        let applied_progress = match &publication.outcome {
            JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.applied_progress,
            _ => 0,
        };
        let retains_incoming = matches!(kind, JobProgressKind::Preview | JobProgressKind::Checkpoint | JobProgressKind::CommitValidated | JobProgressKind::Fault);
        let incoming = if retains_incoming {
            match JobPublicationOwner::try_new(publication) {
                Ok(owner) => Some(owner),
                Err((fault, publication)) => {
                    let _ = self.cancel_admission(admission);
                    return Err(JobProgressRejected::new(fault, publication));
                }
            }
        } else {
            drop(publication);
            None
        };
        let prior = JobProgressPriorState {
            published: self.active[admission.active_index].published,
            terminal: self.active[admission.active_index].terminal,
            identity: self.active[admission.active_index].identity,
            kind: self.active[admission.active_index].kind,
            applied_progress: self.active[admission.active_index].applied_progress,
        };
        let displaced = match kind {
            JobProgressKind::Preview | JobProgressKind::CommitValidated | JobProgressKind::Cancelled | JobProgressKind::Fault => self.active[admission.active_index].preview.take(),
            JobProgressKind::Yield | JobProgressKind::Checkpoint => None,
        };
        let mut retirement_cursor = 0usize;
        let mut displaced_retirement = None;
        if let Some(displaced) = displaced {
            let index = admission.retirement_indices[retirement_cursor];
            self.retirements[index].reservation = None;
            self.retirements[index].owner = Some(displaced);
            displaced_retirement = Some(index);
            retirement_cursor += 1;
        }
        if kind == JobProgressKind::Preview {
            self.active[admission.active_index].preview = incoming;
        } else if let Some(incoming) = incoming {
            let index = admission.retirement_indices[retirement_cursor];
            self.retirements[index].reservation = None;
            self.retirements[index].owner = Some(incoming);
        }
        let next_epoch = admission.active_epoch + 1;
        let slot = &mut self.active[admission.active_index];
        slot.epoch = next_epoch;
        slot.occupied = true;
        slot.published = true;
        slot.awaiting_ack = true;
        slot.checked_out = false;
        slot.terminal = kind.terminal();
        slot.reservation = None;
        slot.identity = identity;
        slot.kind = kind;
        slot.applied_progress = applied_progress;
        self.reserved_items = self.reserved_items.saturating_sub(admission.owner_items);
        self.reserved_bytes = self.reserved_bytes.saturating_sub(admission.owner_bytes);
        if retains_incoming {
            self.owned_items += admission.owner_items;
            self.owned_bytes += admission.owner_bytes;
        }
        let abort_retirement = admission.abort_retirement.map(|index| (index, admission.token));
        Ok(JobProgressReceipt { identity, kind, applied_progress, owner_bytes: admission.owner_bytes, active_index: admission.active_index, active_epoch: next_epoch, prior, displaced_retirement, abort_retirement })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact retained progress receipt without allocating or losing its prior owner.")]
    pub fn acknowledge(&mut self, receipt: JobProgressReceipt) -> Result<(), (JobProgressFault, JobProgressReceipt)> {
        let Some(slot) = self.active.get_mut(receipt.active_index) else {
            return Err((JobProgressFault::Token, receipt));
        };
        if slot.epoch != receipt.active_epoch || !slot.occupied || !slot.awaiting_ack || slot.identity != receipt.identity {
            return Err((JobProgressFault::Token, receipt));
        }
        if let Some((index, token)) = receipt.abort_retirement {
            let Some(retirement) = self.retirements.get_mut(index) else {
                return Err((JobProgressFault::Token, receipt));
            };
            if retirement.reservation != Some(token) || retirement.owner.is_some() {
                return Err((JobProgressFault::Token, receipt));
            }
            retirement.reservation = None;
        }
        slot.awaiting_ack = false;
        if slot.terminal {
            if slot.preview.is_some() {
                slot.awaiting_ack = true;
                return Err((JobProgressFault::Token, receipt));
            }
            slot.clear();
        }
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact retained progress receipt without allocating or losing its prior owner.")]
    pub fn abort(&mut self, receipt: JobProgressReceipt) -> Result<(), (JobProgressFault, JobProgressReceipt)> {
        let Some(slot) = self.active.get_mut(receipt.active_index) else {
            return Err((JobProgressFault::Token, receipt));
        };
        if slot.epoch != receipt.active_epoch || !slot.occupied || !slot.awaiting_ack || slot.identity != receipt.identity {
            return Err((JobProgressFault::Token, receipt));
        }
        if receipt.kind == JobProgressKind::Preview {
            let Some(owner) = slot.preview.take() else {
                return Err((JobProgressFault::Token, receipt));
            };
            let Some((index, token)) = receipt.abort_retirement else {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            };
            let Some(retirement) = self.retirements.get_mut(index) else {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            };
            if retirement.reservation != Some(token) || retirement.owner.is_some() {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            }
            retirement.reservation = None;
            retirement.owner = Some(owner);
        }
        let restored_preview = if let Some(index) = receipt.displaced_retirement {
            let Some(retirement) = self.retirements.get_mut(index) else {
                return Err((JobProgressFault::Token, receipt));
            };
            let Some(owner) = retirement.owner.take() else {
                return Err((JobProgressFault::Token, receipt));
            };
            Some(owner)
        } else {
            None
        };
        slot.published = receipt.prior.published;
        slot.awaiting_ack = false;
        slot.checked_out = false;
        slot.terminal = receipt.prior.terminal;
        slot.reservation = None;
        slot.identity = receipt.prior.identity;
        slot.kind = receipt.prior.kind;
        slot.applied_progress = receipt.prior.applied_progress;
        slot.preview = restored_preview.or_else(|| slot.preview.take());
        Ok(())
    }

    pub fn retain_rejected(&mut self, rejected: JobProgressRejected) -> Result<(), JobProgressRejected> {
        let Some(index) = self.retirements.iter().position(|slot| slot.owner.is_none() && slot.reservation.is_none()) else {
            return Err(rejected);
        };
        let Ok((items, bytes, _)) = publication_owner_shape(rejected.publication(), false) else {
            return Err(rejected);
        };
        if self.owned_items.checked_add(self.reserved_items).and_then(|total| total.checked_add(items)).is_none_or(|total| total > JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS)
            || self.owned_bytes.checked_add(self.reserved_bytes).and_then(|total| total.checked_add(bytes)).is_none_or(|total| total > JOB_PROGRESS_TOTAL_MAXIMUM_BYTES)
        {
            return Err(rejected);
        }
        let publication = rejected.into_publication();
        let owner = match JobPublicationOwner::try_new(publication) {
            Ok(owner) => owner,
            Err((fault, publication)) => return Err(JobProgressRejected::new(fault, publication)),
        };
        self.owned_items = self.owned_items.saturating_add(items);
        self.owned_bytes = self.owned_bytes.saturating_add(bytes);
        self.retirements[index].owner = Some(owner);
        Ok(())
    }

    pub fn take(&mut self, actor: ActorId, job: u64) -> Result<JobProgressCheckout<'_>, JobProgressFault> {
        let slot = self.active.iter_mut().find(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job).ok_or(JobProgressFault::Missing)?;
        if slot.awaiting_ack || slot.checked_out {
            return Err(JobProgressFault::Busy);
        }
        let owner = slot.preview.take().ok_or(JobProgressFault::Missing)?;
        slot.checked_out = true;
        let epoch = slot.epoch;
        Ok(JobProgressCheckout { slot, owner: Some(owner), epoch })
    }

    pub fn begin_close_actor(&mut self, actor: ActorId) -> Result<(), JobProgressFault> {
        if self.closing_actors.iter().flatten().any(|current| *current == actor) {
            return Ok(());
        }
        let slot = self.closing_actors.iter_mut().find(|slot| slot.is_none()).ok_or(JobProgressFault::Capacity)?;
        *slot = Some(actor);
        Ok(())
    }

    pub fn begin_close_all(&mut self) {
        self.close_all = true;
    }

    pub fn has_close_work(&self) -> bool {
        self.retirements.iter().any(|slot| slot.owner.is_some()) || self.closing_actors.iter().any(Option::is_some) || self.close_all
    }

    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> JobProgressCloseStep {
        if cx.should_yield() {
            return JobProgressCloseStep::Blocked;
        }
        if let Some(index) = self.retirements.iter().position(|slot| slot.owner.is_some()) {
            cx.consume_fuel(1);
            let (terminal, released_items, released_bytes) = self.retirements[index].owner.as_mut().expect("job progress retirement owner").close_one();
            self.owned_items = self.owned_items.saturating_sub(released_items);
            self.owned_bytes = self.owned_bytes.saturating_sub(released_bytes);
            if terminal {
                let owner = self.retirements[index].owner.take().expect("terminal job progress retirement owner");
                assert!(owner.terminal_is_empty(), "terminal job progress retirement witness changed before removal");
            }
            return JobProgressCloseStep::Pending { released_items, released_bytes };
        }
        let closing_actor = self.closing_actors.iter().flatten().next().copied();
        if let Some(index) = self.active.iter().position(|slot| slot.occupied && (self.close_all || closing_actor == Some(slot.identity.actor))) {
            if self.active[index].checked_out {
                return JobProgressCloseStep::Blocked;
            }
            cx.consume_fuel(1);
            if let Some(owner) = self.active[index].preview.take() {
                let Some(retirement) = self.retirements.iter_mut().find(|slot| slot.owner.is_none() && slot.reservation.is_none()) else {
                    self.active[index].preview = Some(owner);
                    return JobProgressCloseStep::Blocked;
                };
                retirement.owner = Some(owner);
                return JobProgressCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.active[index].clear();
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(index) = self.closing_actors.iter().position(Option::is_some) {
            cx.consume_fuel(1);
            self.closing_actors[index] = None;
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.close_all {
            cx.consume_fuel(1);
            self.close_all = false;
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        JobProgressCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.active.iter().all(|slot| !slot.occupied && slot.reservation.is_none())
            && self.retirements.iter().all(|slot| slot.owner.is_none() && slot.reservation.is_none())
            && self.closing_actors.iter().all(Option::is_none)
            && !self.close_all
            && self.owned_items == 0
            && self.owned_bytes == 0
            && self.reserved_items == 0
            && self.reserved_bytes == 0
    }

    pub fn actor_terminal_is_empty(&self, actor: ActorId) -> bool {
        self.active.iter().all(|slot| !slot.occupied || slot.identity.actor != actor) && self.closing_actors.iter().flatten().all(|closing| *closing != actor) && self.retirements.iter().all(|slot| slot.owner.is_none())
    }
}

/// 🔄️ Borrowed preview owner whose Drop returns the exact owner to its original fixed slot.
pub struct JobProgressCheckout<'a> {
    pub(super) slot: &'a mut JobProgressSlot,
    pub(super) owner: Option<JobPublicationOwner>,
    pub(super) epoch: u64,
}

impl JobProgressCheckout<'_> {
    pub fn identity(&self) -> JobProgressIdentity {
        self.slot.identity
    }

    pub fn preview(&self) -> &[u8] {
        match &self.owner.as_ref().expect("checked-out preview owner").publication.outcome {
            JobStepOutcome::PreviewReady { preview } => preview,
            _ => &[],
        }
    }

    pub fn resume(self) {}
}

impl Drop for JobProgressCheckout<'_> {
    fn drop(&mut self) {
        assert_eq!(self.slot.epoch, self.epoch, "checked-out job preview ABA");
        assert!(self.slot.checked_out && self.slot.preview.is_none(), "checked-out job preview slot changed before handback");
        self.slot.preview = self.owner.take();
        self.slot.checked_out = false;
    }
}
//#endregion 🎨️JobProgressOverlay
