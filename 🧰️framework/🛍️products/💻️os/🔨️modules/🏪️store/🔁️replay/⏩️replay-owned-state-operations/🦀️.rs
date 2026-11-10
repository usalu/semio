//! ⏩️ Replay owned state operations: the owned replay steps, the retirement and the drive of the replay owned state. See [`super`].

use super::*;
use semio_os_spr::ColdMutation;
use semio_framework_value::{AuthoringWallet, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkPoll, ValueError, retained_clone::RetainedCloneStep};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

/// ⛽️ Computational work one replay frontier step may spend inside its one original effect.
const ARTIFACT_STORE_REPLAY_STEP_WORK: u64 = 64;

/// 🪢️ Opens the one original effect of a replay step: a native step rides its decoder's own Work recipient, a cold step a bounded authoring stock; `None` means the work quote was not admitted and nothing ran.
fn replay_effect<R>(grant: RetainedCloneGrant, control: Option<&mut NativeDecodeControl<'_>>, run: impl FnOnce(Option<&mut NativeDecodeControl<'_>>, &mut RetainedAllocationEffect<'_>) -> R) -> Result<Option<R>, VcsError> {
    let opened = match control {
        Some(control) => control.with_effect(grant, ARTIFACT_STORE_REPLAY_STEP_WORK, |control, effect| run(Some(control), effect)),
        None => {
            let stock = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: semio_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, maximum_capacity_bytes: semio_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, maximum_release_bytes: semio_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, maximum_depth: 64 };
            AuthoringWallet::new(stock, u128::from(ARTIFACT_STORE_REPLAY_STEP_WORK)).run(grant.restricted_by(stock), ARTIFACT_STORE_REPLAY_STEP_WORK, |effect| run(None, effect)).map(|done| done.map(|done| done.output))
        }
    };
    opened.map_err(VcsError::InverseRefused)
}

/// 🧾️ A progressing step that recorded no work pays exactly one unit.
fn paid(effect: &mut RetainedAllocationEffect<'_>, result: Result<RetainedCloneProgress, ValueError>) -> Result<RetainedCloneProgress, ValueError> {
    let progress = result?;
    if effect.performed_work() == 0 && progress != RetainedCloneProgress::default() && !effect.performed(1)? { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "replay step exceeds its work ceiling")); }
    Ok(progress)
}

/// 🧾️ Charges the work a grant-shaped child reported, then yields its step.
fn charged(effect: &mut RetainedAllocationEffect<'_>, work: u64, result: Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
    if !effect.performed(work)? { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "replay step exceeds its work ceiling")); }
    result
}

/// ⏳️ A pending poll keeps its owner resident and makes no progress this turn; a settled failure is the typed refusal.
fn settle_poll(poll: OriginalRetirementPoll) -> Result<RetainedCloneStep, VcsError> {
    match poll {
        RetainedWorkPoll::Pending { .. } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())),
        RetainedWorkPoll::Complete(turn) => turn.result.map_err(|failure| VcsError::InverseRefused(failure.into_value_error())),
    }
}

impl<P,Mutation:super::Mutation<P>+OpBinary> ReplayOwnedState<P,Mutation>{

    pub(super) fn advance_with_operation_cap(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, original:Option<RetainedCloneGrant>, mut control:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>, cold_operation:Option<fn(&mut Self,&Edit<Mutation>)->Result<(),VcsError>>, cold_prefix:Option<fn(&mut Self,&Edit<Mutation>)->Result<(),VcsError>>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if maximum_operations == 0 && !self.finished { return Ok(ReplayStep::Pending(self.progress)); }
        let mut completed_operations = 0usize;
        let cold = original.is_none() && self.operation_preparation_factory.is_none() && cold_operation.is_some() && cold_prefix.is_some();
        let original = if cold { Some(Self::cold_authority_grant()) } else { original };
        while !self.finished {
            if let Some(retirement) = self.replay_retirement.as_mut() {
                let body = retirement.next_copy_byte_demand().map_err(VcsError::InverseRefused)?;
                let capacity = retirement.next_capacity_byte_demand(body).map_err(VcsError::InverseRefused)?;
                let grant=original.ok_or_else(||VcsError::ValidationFailed("native replay cleanup requires the original caller grant".into()))?;
                let Some(poll) = replay_effect(grant, control.as_deref_mut(), |_, effect| {let body=effect.grant().maximum_copy_bytes;retirement.close_step(body,effect)})? else { return Ok(ReplayStep::Pending(self.progress)); };
                let step = settle_poll(poll)?;self.native_receipt=step.progress();
                if !step.progress().fits(grant) { return Err(VcsError::ValidationFailed("replay cleanup exceeded its admitted grant".into())); }
                if retirement.terminal_is_empty() {
                    self.replay_retirement = None;
                }
                if deadline() || step.progress().copied_items == 0 { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            if let Some(owner) = self.prefix_message_retirement.as_mut() {
                let body = owner.next_copy_byte_demand();
                let grant=original.ok_or_else(||VcsError::ValidationFailed("native prefix cleanup requires the original caller grant".into()))?;
                let Some(step) = replay_effect(grant, control.as_deref_mut(), |_, effect| { let result = owner.close_step(grant); charged(effect, owner.last_turn_work(), result) })? else { return Ok(ReplayStep::Pending(self.progress)); };
                let step = step.map_err(VcsError::InverseRefused)?;self.native_receipt=step.progress();
                if !step.progress().fits(grant) { return Err(VcsError::ValidationFailed("prefix diagnostic cleanup exceeded its actual grant".into())); }
                if owner.terminal_is_empty() { self.prefix_message_retirement = None; }
                if deadline() || step.progress().copied_items == 0 { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            if self.convergence_pending_finish {
                self.convergence_pending_finish = false;
                self.finished = true;
                self.progress.done = self.progress.total;
                break;
            }
            if self.convergence_search.is_some() {
                self.step_convergence();
                if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            let Some(edit_id) = self.order.get(self.cursor) else {
                self.finished = true;
                break;
            };
            let edit = edits.replay_edit_at(self.cursor,edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            if self.cursor < self.prefix_until {
                if self.operation < edit.forwards.len() {
                    if self.operation_preparation_factory.is_some() {
                        if !self.step_prepared_operation(edit, true, original, control.as_deref_mut(), deadline)? { deadline(); return Ok(ReplayStep::Pending(self.progress)); }
                        self.operation += 1;
                    completed_operations = completed_operations.saturating_add(1);
                        if self.prefix_work_progress { self.progress.done = self.progress.done.saturating_add(1); }
                        if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                        continue;
                    }
                    let Some(cold_prefix)=cold_prefix.filter(|_|cold)else{return Err(VcsError::InverseRefused(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native prefix replay requires its retained original preparation issuer")));};
                    cold_prefix(self,edit)?;
                    self.operation += 1;
                    completed_operations = completed_operations.saturating_add(1);
                    if self.prefix_work_progress { self.progress.done = self.progress.done.saturating_add(1); }
                    if deadline() || completed_operations >= maximum_operations { return Ok(ReplayStep::Pending(self.progress)); }
                    continue;
                }
                self.cursor+=1;self.operation=0;self.record();continue;
            }
            if self.operation < edit.forwards.len() {
                if self.operation_preparation_factory.is_none(){
                    let Some(cold_operation)=cold_operation.filter(|_|cold)else{return Err(VcsError::InverseRefused(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native replay requires its retained original preparation issuer")));};
                    cold_operation(self,edit)?;
                    self.operation+=1;
                    completed_operations=completed_operations.saturating_add(1);
                    self.progress.done=self.progress.done.saturating_add(1);
                    if deadline()||completed_operations>=maximum_operations{return Ok(ReplayStep::Pending(self.progress));}
                    continue;
                }
                if !self.step_prepared_operation(edit,false,original,control.as_deref_mut(),deadline)?{deadline();return Ok(ReplayStep::Pending(self.progress));}
                self.operation+=1;
                completed_operations=completed_operations.saturating_add(1);
                if deadline()||completed_operations>=maximum_operations{return Ok(ReplayStep::Pending(self.progress));}
                continue;
            }
            if !self.step_close_edit(&edit.id,original,control.as_deref_mut())? {
                if deadline() { return Ok(ReplayStep::Pending(self.progress)); }
                continue;
            }
            if !self.operation_progress { self.progress.done = self.progress.done.saturating_add(1); }
            self.cursor += 1;
            self.operation = 0;
            self.record();
            self.converge();
            if self.cursor == self.order.len() { self.finished = true; }
            if !self.finished && deadline() { return Ok(ReplayStep::Pending(self.progress)); }
        }
        Ok(ReplayStep::Finished(self.progress))
    }


    /// 🧊️ The grant of the unscheduled cold authority: a replay stepped without an original caller grant and without an installed preparation issuer folds on the cold diff arithmetic under exactly this unbounded envelope.
    fn cold_authority_grant() -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: 1 << 16 }
    }

    /// 📨️ Settles one bounded diagnostic unit before sealing an edit's staged owners.
    fn step_close_edit(&mut self, edit_id: &str, original:Option<RetainedCloneGrant>,mut control:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>) -> Result<bool, VcsError> {
        let Some(settlement) = self.message_settlement.as_mut() else {
            self.message_settlement = Some(semio_os_store_ledger::edit_message_clamp::settlement::EditMessageSettlement::new(edit_id, &self.edit_messages, self.closed_outcomes, self.outcomes.len()));
            return Ok(false);
        };
        if !settlement.is_finished() {
            let grant=original.ok_or_else(||VcsError::ValidationFailed("native edit settlement requires the original caller grant".into()))?;

            let capacity=settlement.next_capacity_byte_demand().map_err(VcsError::InverseRefused)?;
            if grant.maximum_capacity_bytes<capacity{return Ok(false)}
            match control.as_deref_mut(){
                Some(control)=>{control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(capacity).map_err(VcsError::InverseRefused)?;}
                None if grant==Self::cold_authority_grant()=>{}
                None=>return Err(VcsError::InverseRefused(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native edit diagnostics require their decoding continuation"))),
            }
            let Some(progress)=replay_effect(grant,control.as_deref_mut(),|_,effect|{let result=settlement.step(&mut self.edit_messages,&mut self.outcomes,effect);paid(effect,result)})? else{return Ok(false)};
            self.native_receipt=progress.map_err(VcsError::InverseRefused)?;
            return Ok(false);
        }
        let rejects = match self.mode {
            ReplayMode::Merge(policy) => semio_os_spr::worst_level(settlement.messages()).is_some_and(|level| policy.rejects(level)),
            ReplayMode::Report => false,
        };
        if rejects {
            if let Some(candidate) = self.candidate.take() { self.begin_replay_snapshot_retirement(candidate); return Ok(false); }
            if !self.edit_inverse.terminal_is_empty() {
                let inverse = std::mem::take(&mut self.edit_inverse);
                self.begin_replay_mutations_retirement(inverse);
                return Ok(false);
            }
        } else if let Some(candidate) = self.candidate.take() {
            if let Some(previous) = self.state.replace(candidate) { self.begin_replay_snapshot_retirement(previous); return Ok(false); }
        }
        let grant=original.ok_or_else(||VcsError::HistoryReplaying)?;
        macro_rules! reserve_output{($field:ident,$kind:ty)=>{if self.$field.len()==self.$field.capacity(){let old=self.$field.capacity()*std::mem::size_of::<$kind>();let bytes=(self.$field.len()+1).checked_mul(std::mem::size_of::<$kind>()).ok_or_else(||VcsError::HistoryReplaying)?;let d=semio_framework_value::RetirementDemand{copy_bytes:old+std::mem::size_of::<Vec<$kind>>(),capacity_bytes:bytes,release_bytes:old,depth:1};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(false)}match control.as_deref_mut(){Some(control)=>{control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(bytes).map_err(VcsError::InverseRefused)?;}None if grant==Self::cold_authority_grant()=>{}None=>return Err(VcsError::HistoryReplaying)}self.$field.try_reserve_exact(1).map_err(|_|VcsError::HistoryReplaying)?;self.native_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:bytes,released_bytes:old};return Ok(false)}};}
        reserve_output!(replayed,semio_os_spr::EditMessages);
        if rejects{reserve_output!(quarantined,String);}else{reserve_output!(rebased_inverse,(String,semio_framework_value_persistent::list::PagedList<Mutation,{usize::MAX}>));reserve_output!(committed,String);}
        let count=if rejects{2}else{3};let bytes=edit_id.len().checked_mul(count).ok_or(VcsError::HistoryReplaying)?;let copy=bytes+std::mem::size_of::<semio_os_spr::EditMessages>()+std::mem::size_of::<semio_os_store_ledger::edit_message_clamp::settlement::EditMessageSettlement>()+std::mem::size_of::<semio_framework_value_persistent::list::PagedList<Mutation,{usize::MAX}>>();let d=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:bytes,depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(false)}match control.as_deref_mut(){Some(control)=>{control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(bytes).map_err(VcsError::InverseRefused)?;}None if grant==Self::cold_authority_grant()=>{}None=>return Err(VcsError::HistoryReplaying)}self.native_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:bytes,..Default::default()};
        let messages = self.message_settlement.as_mut().expect("completed edit message selection").take_messages().expect("final bounded diagnostic owner");
        drop(self.message_settlement.take());
        self.closed_outcomes = self.outcomes.len();
        let inverse = std::mem::take(&mut self.edit_inverse);
        self.replayed.push(semio_os_spr::EditMessages { edit_id: edit_id.to_string(), messages });
        if rejects {
            self.quarantined.push(edit_id.to_string());
        } else {
            self.rebased_inverse.push((edit_id.to_string(), inverse));
            self.committed.push(edit_id.to_string());
        }
        Ok(true)
    }

    pub fn record(&mut self) {
        if self.mode != ReplayMode::Report || self.cursor % self.record_stride != 0 || self.cursor >= self.order.len() {
            return;
        }
        if let Some(state) = self.state.as_ref() {
            self.recorded.push((self.cursor, Arc::clone(state)));
        }
    }

    fn converge(&mut self) {
        if self.mode != ReplayMode::Report { return; }
        let Some(convergence) = self.convergence.as_ref() else { return };
        if self.cursor < convergence.horizon || self.cursor >= self.order.len() { return; }
        self.convergence_search = Some(0);
    }

    fn step_convergence(&mut self) {
        let Some(index) = self.convergence_search else { return };
        let Some(convergence) = self.convergence.as_ref() else { self.convergence_search = None; return };
        let Some((length, previous)) = convergence.checkpoints.get(index) else { self.convergence_search = None; return };
        if *length != self.cursor {
            self.convergence_search = Some(index + 1);
            return;
        }
        self.convergence_search = None;
        let Some(state) = self.state.as_ref() else { return };
        let pointer_equal = Arc::ptr_eq(state, previous);
        let cold_authority = self.operation_preparation_factory.is_none() && self.replay_retirement_factory.is_none();
        if !pointer_equal && !(cold_authority && (convergence.equal)(state, previous)) { return; }
        let Some(head) = convergence.checkpoints.last().filter(|(length, _)| *length == self.order.len()).map(|(_, head)| Arc::clone(head)) else { return };
        self.converged_at = Some(self.cursor);
        self.convergence_pending_finish = true;
        if let Some(previous) = self.state.replace(head) { self.begin_replay_snapshot_retirement(previous); }
    }
}

impl<P,Mutation:super::Mutation<P>> ReplayOwnedState<P,Mutation>{

    pub(super) fn new_counted(mode: ReplayMode, base: Arc<P>, order: semio_os_vcs::HistoryPageStack<String>, from: usize, schema: String, supersessions: EffectiveSupersessions, total: u32) -> Result<Self, VcsError> {
        let total = total.saturating_add(u32::try_from(order.len().saturating_sub(from)).unwrap_or(u32::MAX));
        let replay = Self {
            mode,
            order,
            from,
            prefix_until: from,
            prefix_work_progress: false,
            operation_progress: false,
            cursor: from,
            operation: 0,
            schema,
            supersessions,
            state: Some(base),
            candidate: None,
            edit_messages: semio_os_spr::command::ReplayMessageAccumulator::default(),
            edit_inverse: Default::default(),
            message_settlement: None,
            closed_outcomes: 0,
            committed: Vec::new(),
            quarantined: Vec::new(),
            rebased_inverse: Vec::new(),
            replayed: Vec::new(),
            outcomes: Vec::new(),
            report_worst: None,
            conflict_target:Vec::new(),unit_flags: protocol::HistoryFoldIndex::new(),
            operation_preparation_factory: None,
            operation_preparation: None,
            prepared_operation: None,operation_identity:None,
            operation_retirement: None,
            operation_message_settlement: None,
            preparation_completed: ArtifactReplayPreparationProgress::default(),native_receipt:Default::default(),
            replay_retirement_factory: None,
            replay_retirement: None,
            prefix_message_retirement: None,
            convergence: None,
            convergence_search: None,
            convergence_pending_finish: false,
            converged_at: None,
            record_stride: usize::MAX,
            recorded: Vec::new(),
            drafts: protocol::HistoryInputDrafts::new(),
            generation: 0,
            revision: [0; 32],
            progress: ReplayProgress { done: 0, total },
            finished: false,
        };
        if replay.from > replay.order.len() {
            return Err(VcsError::ValidationFailed(format!("edit replay starts at {} past its {}-edit order", replay.from, replay.order.len())));
        }
        Ok(replay)
    }

    /// 🆔️ The store generation this replay was started against.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// 🗒️ The draft inputs this replay previews.
    pub fn drafts(&self) -> &protocol::HistoryInputDrafts {
        &self.drafts
    }

    /// 📈️ Replay work completed, including an edit's final diagnostic settlement.
    pub fn progress(&self) -> ReplayProgress {
        self.progress
    }

    /// ⌛️ Whether every operation of the suffix was replayed (or the replay converged).
    pub fn is_finished(&self) -> bool {
        self.finished
    }
}

impl<P, Mutation> ReplayOwnedState<P, Mutation>
where
    P: Clone,
    Mutation: super::Mutation<P> + ColdMutation<P> + OpBinary,
{

    pub(super) fn step_with_operation_cap(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, original:Option<RetainedCloneGrant>, mut control:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {self.advance_with_operation_cap(edits,maximum_operations,original,control,Some(Self::replay_operation),Some(Self::replay_prefix_operation),deadline)}

    /// 🐣️ A replay of `order[from..]` starting from `base`, the projection after `order[..from]`, reading every
    /// operation of a document of `schema` through `supersessions`. `edits` must hold every edit the suffix names.
    pub(super) fn new(mode: ReplayMode, base: Arc<P>, order: Vec<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, edits: &impl ReplayEdits<Mutation>) -> Result<Self, VcsError> {
        if from > order.len() { return Err(VcsError::ValidationFailed(format!("edit replay starts at {from} past its {}-edit order", order.len()))); }
        let mut total = 0u32;
        for (offset,edit_id) in order[from..].iter().enumerate() {
            let edit = edits.replay_edit_at(from+offset,edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            total = total.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));
        }
        let mut catalog = semio_os_vcs::HistoryPageStack::new();
        catalog.extend(order);
        Self::new_counted(mode, base, catalog, from, schema.to_owned(), supersessions, total)
    }


    /// 🐣️ [`Self::new`] starting from `base`, the projection after `order[..base_length]` for a `base_length <= from`: the
    /// replay first folds `order[base_length..from]` plainly — stepped under the same deadline as its suffix and recording the
    /// prefix ring on the way — and then replays `order[from..]` as [`Self::new`] does, so no synchronous fold precedes it.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new_after_prefix(mode: ReplayMode, base_length: usize, base: Arc<P>, order: Vec<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, edits: &impl ReplayEdits<Mutation>) -> Result<Self, VcsError> {
        if base_length > from {
            return Err(VcsError::ValidationFailed(format!("edit replay base after {base_length} edits lies past its start {from}")));
        }
        let mut replay = Self::new(mode, base, order, from, schema, supersessions, edits)?;
        replay.cursor = base_length;
        Ok(replay)
    }

    pub fn with_convergence(mut self, convergence: Option<ReplayConvergence<P>>) -> Self {
        self.replace_convergence(convergence);
        self
    }

    pub(super) fn replace_convergence(&mut self, convergence: Option<ReplayConvergence<P>>) {
        if let Some(previous) = std::mem::replace(&mut self.convergence, convergence) {
            for (_, checkpoint) in previous.checkpoints {
                retire_shared_projection::<P, Mutation>(checkpoint);
            }
        }
    }

    /// 🧵️ Takes `appended` operations a tick appended to `edit_id` into the total when that edit is the unreplayed tail of
    /// this replay's order (the grown edit is read by id when the replay gets there), and forgets the convergence probe,
    /// whose previous head predates the tick.
    pub fn absorb_tail_growth(&mut self, edit_id: &str, appended: usize) {
        if self.cursor < self.order.len() && self.order.last().is_some_and(|tail| tail == edit_id) {
            self.progress.total = self.progress.total.saturating_add(u32::try_from(appended).unwrap_or(u32::MAX));
        }
        self.replace_convergence(None);
    }

    /// 🎙️ Keeps the projection after every replayed prefix whose length is a multiple of `stride`, for the prefix ring —
    /// seeded with `prefixes`, the retained projections the base was folded through, and with the base itself when its
    /// length is such a multiple.
    pub fn recording(mut self, stride: usize, prefixes: Vec<(usize, Arc<P>)>) -> Self {
        self.record_stride = stride.max(1);
        self.recorded.extend(prefixes);
        self.record();
        self
    }

    /// 🔢️ Stamps the store generation and content revision this replay was started against — the freshness a finished
    /// replay is committed under ([`ArtifactStore::commit_finished_replay`]).
    pub fn with_base(mut self, generation: u64, revision: [u8; 32]) -> Self {
        self.generation = generation;
        self.revision = revision;
        self
    }

    /// 🖊️ Records the draft inputs this replay previews, which a finalization authors as the supersession.
    pub fn with_drafts(mut self, drafts: protocol::HistoryInputDrafts) -> Self {
        self.drafts = drafts;
        self
    }





    /// ⏭️ Replays operations until the replay finishes or `deadline`, asked after every operation, answers `true`.
    pub fn step(&mut self, edits: &impl ReplayEdits<Mutation>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        self.step_with_operation_cap(edits, usize::MAX, None, None, deadline)
    }

    /// 🔢️ Caps folded mutations while retaining every structural deadline and owner check.
    pub fn step_operations(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if !self.operation_progress {
            if self.progress.done != 0 { return Err(VcsError::ValidationFailed("operation-counted replay authority must be selected before work begins".into())); }
            self.progress.total = self.progress.total.saturating_sub(u32::try_from(self.order.len().saturating_sub(self.from)).unwrap_or(u32::MAX));
            self.operation_progress = true;
        }
        self.step_with_operation_cap(edits, maximum_operations, None, None, deadline)
    }


    /// 🫀️ Replays operation `self.operation` of `edit`: its effective input is re-diffed and re-inverted against the running
    /// state and its outcome recorded. A `Report` replay never aborts on a leaf: an inverse the leaf refuses on the replayed
    /// state is one `Fatal` `mutation.inverse-refused` message on that mutation, the operation folds forward exactly as every
    /// messageless fold site folds it, and its edit stages no inverse rows for it — the report blocks finalizing until that

    /// ⏭️ Folds one prefix operation on the cold diff arithmetic: a prefix replays messageless and stages no rows.
    fn replay_prefix_operation(&mut self, edit: &Edit<Mutation>) -> Result<(), VcsError> {
        let index = self.operation;
        let id = semio_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).ok_or_else(|| VcsError::ValidationFailed("recorded operation identity is absent".into()))?;
        if edit.forwards[index].may_emit_foreign_steps() {
            let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            self.unit_flags.insert((digest, index), semio_os_spr::ColdMutation::foreign_step_source(&edit.forwards[index], self.state.as_ref().ok_or_else(|| VcsError::ValidationFailed("edit replay lost its running projection".into()))?, 0).map_err(VcsError::from)?.is_some());
        }
        let effective = effective_operation::<P, Mutation>(&edit.forwards[index], index as u32, id, &self.schema, &self.supersessions);
        if let Some(operation) = effective.operation() {
            let state = self.state.as_mut().ok_or_else(|| VcsError::ValidationFailed("edit replay lost its running projection".into()))?;
            if let (Some(next), _) = fold_operation::<P, Mutation>(state, operation, index as u32) { <Arc<P> as ColdFoldProjection<P, Mutation>>::advance(state, next); }
        }
        Ok(())
    }
    /// mutation is edited or withdrawn. A `Merge` replay refuses with [`VcsError::InverseRefused`].
    fn replay_operation(&mut self, edit: &Edit<Mutation>) -> Result<(), VcsError> where P:Clone, Mutation:semio_os_spr::ColdMutation<P> {
        let index = self.operation;
        let mutation_id = semio_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).ok_or_else(|| VcsError::ValidationFailed("recorded operation identity is absent".into()))?;
        if edit.forwards[index].may_emit_foreign_steps() {
            let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            let base = self.candidate.as_ref().or(self.state.as_ref()).expect("replay operation state");
            self.unit_flags.insert((digest, index), semio_os_spr::ColdMutation::foreign_step_source(&edit.forwards[index],base,0).map_err(VcsError::from)?.is_some());
        }
        let effective = effective_operation::<P, Mutation>(&edit.forwards[index], index as u32, mutation_id, &self.schema, &self.supersessions);
        let mut messages: Vec<semio_os_spr::MutationMessage> = effective.fault().cloned().into_iter().collect();
        if let Some(operation) = effective.operation() {
            let base = match self.mode {
                ReplayMode::Merge(_) => self.candidate.as_ref().or(self.state.as_ref()),
                ReplayMode::Report => self.state.as_ref(),
            }
            .ok_or_else(|| VcsError::ValidationFailed("edit replay lost its running projection".into()))?;
            let (back, refused) = match operation.inverse(base) {
                Ok(mut back) => {
                    back.reverse();
                    (back, None)
                }
                Err(cause) if self.mode == ReplayMode::Report => (Vec::new(), Some(cause)),
                Err(cause) => return Err(VcsError::InverseRefused(cause)),
            };
            let next = match self.mode {
                ReplayMode::Report => {
                    let (next, operation_messages) = fold_operation::<P, Mutation>(base, operation, index as u32);
                    messages = operation_messages;
                    next
                }
                ReplayMode::Merge(_) => {
                    let (applied, operation_messages) = apply_operation::<P, Mutation>(base, operation, index as u32);
                    messages = operation_messages;
                    match applied {
                        Ok(next) => Some(next),
                        Err(error) => {
                            retire_scratch_operations::<P, Mutation>(back);
                            return Err(error.into());
                        }
                    }
                }
            };
            if let Some(cause) = refused {
                let mut fatal = semio_os_spr::MutationMessage::fatal("mutation.inverse-refused", format!("the operation has no inverse on the replayed state: {cause}")).at([effective.mutation_id.0.clone()]);
                fatal.op_index = Some(index as u32);
                messages.push(fatal);
            }
            match next {
                Some(next) => {
                    self.edit_inverse.extend(back);
                    let next = Arc::new(next);
                    let displaced = match self.mode {
                        ReplayMode::Merge(_) => self.candidate.replace(next),
                        ReplayMode::Report => self.state.replace(next),
                    };
                    if let Some(displaced) = displaced {
                        retire_shared_projection::<P, Mutation>(displaced);
                    }
                }
                None => retire_scratch_operations::<P, Mutation>(back),
            }
        }
        let worst = semio_os_spr::worst_level(&messages);
        self.report_worst = self.report_worst.max(worst);
        let mut drain = semio_os_spr::command::OperationMessageDrain::new(protocol::MutationReplayOutcome { mutation_id: effective.mutation_id.clone(), edit_id: edit.id.clone(), op_index: index as u32, worst, messages, superseded: effective.superseded(), withdrawn: effective.withdrawn() });
        while !drain.is_finished() {
            let demand=drain.next_demand(&self.edit_messages).map_err(VcsError::InverseRefused)?;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let Some(progress) = replay_effect(grant, None, |_, effect| { let result = drain.step(&mut self.edit_messages, effect); paid(effect, result) })? else { return Err(VcsError::ValidationFailed("cold replay message drain was not admitted".into())); };
            if progress.map_err(VcsError::InverseRefused)? == RetainedCloneProgress::default() { return Err(VcsError::ValidationFailed("cold replay message drain made no progress".into())); }
        }
        self.outcomes.push(drain.take().expect("cold original operation messages transferred"));
        Ok(())
    }





}

//#region 🎮️OperationPreparation
mod operation_preparation {
    use super::*;
    use crate::replay_preparation::{ArtifactReplayPreparationRequest, OperationMessageSettlement, ReplayRetirement};
    use semio_os_spr::{InputReplacement, MutationMessage};
    use semio_os_store_one_item::ArtifactStoreOneItemGrant;
    use semio_framework_value::ValueError;

    impl<P, M> crate::ReplayOwnedState<P, M>
    where M: semio_os_spr::Mutation<P> + semio_os_spr::OpBinary {
        /// ♻️ Binds both installed cleanup authorities without imposing native bounds on cold replay construction.
        pub fn with_retirement_factories(mut self, snapshots: Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>) -> Self
        where P: Send + Sync + 'static, M: Send + 'static {
            self.replay_retirement_factory = Some(semio_os_spr::command::registered_replay_retirement_factory(snapshots, mutations));
            self
        }

        /// 🎟️ Exposes the next exact allocation release separately from the ordinary body-byte grant.
        pub fn replay_retirement_release_byte_demand(&self) -> Result<usize, ValueError> {
            self.replay_retirement.as_ref().map_or(Ok(0), ReplayRetirement::next_release_byte_demand)
        }

        pub(crate) fn begin_replay_snapshot_retirement(&mut self, owner: Arc<P>) {
            if let Some(factory) = self.replay_retirement_factory.as_ref() {
                self.replay_retirement.get_or_insert_with(|| ReplayRetirement::new(Arc::clone(factory))).stage_snapshot(owner);
            } else { semio_os_store_projection::retire_shared_projection::<P, M>(owner); }
        }

        pub(crate) fn begin_replay_mutations_retirement(&mut self, owners: semio_framework_value_persistent::list::PagedList<M, {usize::MAX}>) {
            if let Some(factory) = self.replay_retirement_factory.as_ref() {
                self.replay_retirement.get_or_insert_with(|| ReplayRetirement::new(Arc::clone(factory))).stage_mutations(owners);
            } else { semio_os_store_command::retire_scratch_operations::<P, M>(owners); }
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

        /// 🧭️ Reports the replay mode required by the current original history operation.
        pub fn original_preparation_mode(&self) -> crate::replay_preparation::ArtifactReplayPreparationMode { if self.cursor < self.prefix_until { crate::replay_preparation::ArtifactReplayPreparationMode::Prefix } else { crate::replay_preparation::ArtifactReplayPreparationMode::Report } }

        /// 🧾️ Transfers the actual last native receipt exactly once.
        pub fn take_native_receipt(&mut self) -> semio_framework_value::retained_clone::RetainedCloneProgress { std::mem::take(&mut self.native_receipt) }

        /// 🚦️ A new semantic producer begins only after every preceding settlement and close frontier empties.
        pub fn original_preparation_ready(&self) -> bool { self.operation_preparation.is_none() && self.prepared_operation.is_none() && self.operation_retirement.is_none() && self.operation_message_settlement.is_none() && self.replay_retirement.is_none() && self.prefix_message_retirement.is_none() }

        /// 🎟️ Quotes the installed semantic factory's birth before an original request is moved into it.
        pub fn original_preparation_birth_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
            let factory = self.operation_preparation_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original replay requires its installed semantic factory"))?;
            Ok(factory.preparation_birth_demand())
        }

        /// 🌱️ Moves a genuine retained request into the installed factory and keeps its cursor as the active task; a refusal leaves the request with the caller.
        pub fn admit_original_preparation(&mut self, request: &mut Option<ArtifactReplayPreparationRequest<P, M>>, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneProgress>, ValueError> {
            if self.operation_preparation.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original replay already retains an active preparation")); }
            let factory = Arc::clone(self.operation_preparation_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original replay requires its installed semantic factory"))?);
            let demand = factory.preparation_birth_demand();
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(None); }
            let context = request.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original replay requires a retained request"))?.context;
            let taken = request.take().expect("checked retained request");
            match factory.begin(taken, grant) {
                Ok((cursor, progress)) => { self.operation_preparation = Some(ArtifactReplayPreparationTask::new(context, cursor)); Ok(Some(progress)) }
                Err((error, returned)) => { *request = Some(returned); Err(error) }
            }
        }

        pub(crate) fn step_prepared_operation(&mut self, edit: &semio_os_spr::Edit<M>, prefix: bool, original:Option<RetainedCloneGrant>, mut control:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>, _deadline: &mut dyn FnMut() -> bool) -> Result<bool, semio_os_vcs::VcsError> {
            use crate::ReplayMode;
    use semio_os_store_ledger::CursorRevisionAccumulator;
            use semio_os_vcs::VcsError;
            let original=original.ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native replay requires the caller's original operation grant")))?;
            let grant=ArtifactStoreOneItemGrant{maximum_items:original.maximum_items,maximum_copy_bytes:original.maximum_copy_bytes,maximum_capacity_bytes:original.maximum_capacity_bytes,maximum_release_bytes:original.maximum_release_bytes,maximum_depth:original.maximum_depth};
            if let Some(settlement) = self.operation_message_settlement.as_mut() {
                let Some(stepped)=replay_effect(original,control.as_deref_mut(),|_,effect|{let result=settlement.step(&mut self.edit_messages,effect);match result{Ok((done,progress))=>paid(effect,Ok(progress)).map(|progress|(done,progress)),Err(error)=>Err(error)}})? else{return Ok(false)};
                let (done,progress)=stepped.map_err(VcsError::InverseRefused)?;
                self.native_receipt=progress;
                if done {
                    let outcome = settlement.take();
                    self.report_worst = self.report_worst.max(outcome.worst);
                    self.outcomes.push(outcome);
                    self.operation_message_settlement = None;
                    return Ok(true);
                }
                return Ok(false);
            }
            if let Some(retirement) = self.operation_retirement.as_mut() {
                let control=control.ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native replay retirement requires its original decoder")))?;
                let Some(polled)=replay_effect(original,Some(control),|control,effect|->Result<OriginalRetirementPoll,ValueError>{let control=control.ok_or_else(||ValueError::invariant("native replay retirement lost its decoder"))?;control.with_retirement(original,|receiving|Ok(retirement.close_step(effect,receiving)))})? else{return Ok(false)};
                let step=settle_poll(polled.map_err(VcsError::InverseRefused)?)?;
                self.native_receipt=step.progress();
                if retirement.terminal_is_empty() {
                    self.operation_retirement = None;
                }
                return Ok(false);
            }
            let index = self.operation;
            let mutation_id = semio_os_spr::mutation_id_for_edit_operation::<P, M>(edit, index).ok_or_else(|| VcsError::ValidationFailed("recorded operation identity is absent".into()))?;
            if self.prepared_operation.is_none() {
                if self.operation_preparation.is_none() {
                    return Ok(false);
                }
                let task = self.operation_preparation.as_mut().expect("active operation preparation");
                if task.is_ready(){let copy=size_of::<ArtifactReplayPrepared<P,M>>()+size_of::<ArtifactReplayPreparationTask<P,M>>()+size_of::<ArtifactReplayPreparationRetirement<P,M>>();if original.maximum_copy_bytes<copy||original.maximum_depth==0||original.maximum_items==0{return Ok(false);}let work=task.progress();self.preparation_completed.items=self.preparation_completed.items.saturating_add(work.items);self.preparation_completed.bytes=self.preparation_completed.bytes.saturating_add(work.bytes);self.prepared_operation=Some(task.take_prepared().ok_or_else(||VcsError::ValidationFailed("prepared operation has no owners".into()))?);self.operation_retirement=Some(self.operation_preparation.take().unwrap().into_retirement());self.native_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(false);}
                let control=control.ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native replay requires its original construction decoder")))?;
                let step=task.step(self.generation,self.revision,original.maximum_copy_bytes,grant,control,&mut || false).map_err(|failure|VcsError::InverseRefused(failure.into_value_error()))?;
                self.native_receipt=step.progress();
                return Ok(false);
            }
            let prepared = self.prepared_operation.as_mut().expect("settled operation owners");
            if !prefix && prepared.next.is_some() && prepared.apply_refusal.is_none() && prepared.input_refusal.is_none() {
                if let Ok(inverse) = prepared.inverse.as_mut() {
                    if !inverse.is_empty() {
                        if !self.edit_inverse.has_reserved_slot() {
                            let demand = self.edit_inverse.next_allocation_bytes().map_err(|error| VcsError::InverseRefused(error.into()))?;
                            if demand > grant.maximum_copy_bytes { return Err(VcsError::ValidationFailed("replay inverse page exceeds its body grant".into())); }
                            let Some(reserved) = replay_effect(original, control.as_deref_mut(), |_, effect| self.edit_inverse.reserve_one(effect).result)? else { return Ok(false); };
                            match reserved {
                                Ok(semio_framework_value_persistent::list::PagedListReservationOutcome::Ready(step)) => self.native_receipt = RetainedCloneProgress { copied_items: usize::from(step.progressed), retained_capacity_bytes: step.allocated_bytes, ..Default::default() },
                                Ok(semio_framework_value_persistent::list::PagedListReservationOutcome::Denied) => {}
                                Err(semio_framework_value_persistent::list::PagedListReservationFailure::Work(error)) => return Err(VcsError::InverseRefused(error)),
                                Err(semio_framework_value_persistent::list::PagedListReservationFailure::Allocation { error, .. }) => return Err(VcsError::InverseRefused(error.refusal().into())),
                            }
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
                    self.prefix_message_retirement = Some(semio_os_spr::command::MutationMessageLedgerRetirement::new(String::new(), std::mem::take(&mut prepared.messages)));
                }
                return Ok(true);
            }
            let supersession = self.supersessions.get(&mutation_id);
            self.operation_message_settlement = Some(OperationMessageSettlement::new(semio_os_spr::MutationReplayOutcome {
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
}
//#endregion 🎮️OperationPreparation
