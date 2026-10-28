//! ⏩️ Replay owned state operations: the owned replay steps, the retirement and the drive of the replay owned state. See [`super`].

use super::*;
use semio_framework_os_spr::ColdMutation;

impl<P, Mutation> ReplayOwnedState<P, Mutation>
where
    P: Clone,
    Mutation: super::Mutation<P> + ColdMutation<P> + OpBinary,
{
    /// 🐣️ A replay of `order[from..]` starting from `base`, the projection after `order[..from]`, reading every
    /// operation of a document of `schema` through `supersessions`. `edits` must hold every edit the suffix names.
    pub(super) fn new(mode: ReplayMode, base: Arc<P>, order: Vec<String>, from: usize, schema: &str, supersessions: EffectiveSupersessions, edits: &impl ReplayEdits<Mutation>) -> Result<Self, VcsError> {
        if from > order.len() { return Err(VcsError::ValidationFailed(format!("edit replay starts at {from} past its {}-edit order", order.len()))); }
        let mut total = 0u32;
        for (offset,edit_id) in order[from..].iter().enumerate() {
            let edit = edits.replay_edit_at(from+offset,edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            total = total.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));
        }
        let mut catalog = semio_framework_os_vcs::HistoryPageStack::new();
        catalog.extend(order);
        Self::new_counted(mode, base, catalog, from, schema.to_owned(), supersessions, total)
    }

    pub(super) fn new_counted(mode: ReplayMode, base: Arc<P>, order: semio_framework_os_vcs::HistoryPageStack<String>, from: usize, schema: String, supersessions: EffectiveSupersessions, total: u32) -> Result<Self, VcsError> {
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
            edit_messages: semio_framework_os_spr::command::ReplayMessageAccumulator::default(),
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

    pub(super) fn step_with_operation_cap(&mut self, edits: &impl ReplayEdits<Mutation>, maximum_operations: usize, original:Option<RetainedCloneGrant>, mut control:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if maximum_operations == 0 && !self.finished { return Ok(ReplayStep::Pending(self.progress)); }
        let mut completed_operations = 0usize;
        let cold = original.is_none() && self.operation_preparation_factory.is_none();
        let original = if cold { Some(Self::cold_authority_grant()) } else { original };
        while !self.finished {
            if let Some(retirement) = self.replay_retirement.as_mut() {
                let body = retirement.next_copy_byte_demand().map_err(VcsError::InverseRefused)?;
                let capacity = retirement.next_capacity_byte_demand(body).map_err(VcsError::InverseRefused)?;
                let grant=original.ok_or_else(||VcsError::ValidationFailed("native replay cleanup requires the original caller grant".into()))?;
                let step = retirement.close_step(grant).map_err(VcsError::InverseRefused)?;self.native_receipt=step.progress();
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
                let step = owner.close_step(grant).map_err(VcsError::InverseRefused)?;self.native_receipt=step.progress();
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
                    return Err(VcsError::InverseRefused(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native prefix replay requires its retained original preparation issuer")));
                }
                self.cursor+=1;self.operation=0;self.record();continue;
            }
            if self.operation < edit.forwards.len() {
                if self.operation_preparation_factory.is_none(){
                    if !cold{return Err(VcsError::InverseRefused(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native replay requires its retained original preparation issuer")));}
                    self.replay_operation(edit)?;
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

    /// 🫀️ Replays operation `self.operation` of `edit`: its effective input is re-diffed and re-inverted against the running
    /// state and its outcome recorded. A `Report` replay never aborts on a leaf: an inverse the leaf refuses on the replayed
    /// state is one `Fatal` `mutation.inverse-refused` message on that mutation, the operation folds forward exactly as every
    /// messageless fold site folds it, and its edit stages no inverse rows for it — the report blocks finalizing until that
    /// mutation is edited or withdrawn. A `Merge` replay refuses with [`VcsError::InverseRefused`].
    fn replay_operation(&mut self, edit: &Edit<Mutation>) -> Result<(), VcsError> where P:Clone, Mutation:semio_framework_os_spr::ColdMutation<P> {
        let index = self.operation;
        let mutation_id = semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded operation identity");
        if edit.forwards[index].may_emit_foreign_steps() {
            let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            let base = self.candidate.as_ref().or(self.state.as_ref()).expect("replay operation state");
            self.unit_flags.insert((digest, index), semio_framework_os_spr::ColdMutation::foreign_step_source(&edit.forwards[index],base,0).map_err(VcsError::from)?.is_some());
        }
        let effective = effective_operation::<P, Mutation>(&edit.forwards[index], index as u32, mutation_id, &self.schema, &self.supersessions);
        let mut messages: Vec<semio_framework_os_spr::MutationMessage> = effective.fault().cloned().into_iter().collect();
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
                let mut fatal = semio_framework_os_spr::MutationMessage::fatal("mutation.inverse-refused", format!("the operation has no inverse on the replayed state: {cause}")).at([effective.mutation_id.0.clone()]);
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
        let worst = semio_framework_os_spr::worst_level(&messages);
        self.report_worst = self.report_worst.max(worst);
        let mut drain = semio_framework_os_spr::command::OperationMessageDrain::new(protocol::MutationReplayOutcome { mutation_id: effective.mutation_id.clone(), edit_id: edit.id.clone(), op_index: index as u32, worst, messages, superseded: effective.superseded(), withdrawn: effective.withdrawn() });
        while !drain.is_finished() {
            let demand=drain.next_demand(&self.edit_messages).map_err(VcsError::InverseRefused)?;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            drain.step(&mut self.edit_messages, grant).map_err(VcsError::InverseRefused)?;
        }
        self.outcomes.push(drain.take().expect("cold original operation messages transferred"));
        Ok(())
    }

    /// 📨️ Settles one bounded diagnostic unit before sealing an edit's staged owners.
    fn step_close_edit(&mut self, edit_id: &str, original:Option<RetainedCloneGrant>,mut control:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>) -> Result<bool, VcsError> {
        let Some(settlement) = self.message_settlement.as_mut() else {
            self.message_settlement = Some(semio_framework_os_store_ledger::edit_message_clamp::settlement::EditMessageSettlement::new(edit_id, &self.edit_messages, self.closed_outcomes, self.outcomes.len()));
            return Ok(false);
        };
        if !settlement.is_finished() {
            let grant=original.ok_or_else(||VcsError::ValidationFailed("native edit settlement requires the original caller grant".into()))?;

            let capacity=settlement.next_capacity_byte_demand().map_err(VcsError::InverseRefused)?;
            if grant.maximum_capacity_bytes<capacity{return Ok(false)}
            let control=control.ok_or_else(||VcsError::InverseRefused(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native edit diagnostics require their decoding continuation")))?;
            control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(capacity).map_err(VcsError::InverseRefused)?;
            self.native_receipt=settlement.step(&mut self.edit_messages,&mut self.outcomes,grant).map_err(VcsError::InverseRefused)?;
            return Ok(false);
        }
        let rejects = match self.mode {
            ReplayMode::Merge(policy) => semio_framework_os_spr::worst_level(settlement.messages()).is_some_and(|level| policy.rejects(level)),
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
        macro_rules! reserve_output{($field:ident,$kind:ty)=>{if self.$field.len()==self.$field.capacity(){let old=self.$field.capacity()*std::mem::size_of::<$kind>();let bytes=(self.$field.len()+1).checked_mul(std::mem::size_of::<$kind>()).ok_or_else(||VcsError::HistoryReplaying)?;let d=semio_framework_value::RetirementDemand{copy_bytes:old+std::mem::size_of::<Vec<$kind>>(),capacity_bytes:bytes,release_bytes:old,depth:1};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(false)}let control=control.as_deref_mut().ok_or_else(||VcsError::HistoryReplaying)?;control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(bytes).map_err(VcsError::InverseRefused)?;self.$field.try_reserve_exact(1).map_err(|_|VcsError::HistoryReplaying)?;self.native_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:bytes,released_bytes:old};return Ok(false)}};}
        reserve_output!(replayed,semio_framework_os_spr::EditMessages);
        if rejects{reserve_output!(quarantined,String);}else{reserve_output!(rebased_inverse,(String,semio_framework_value_persistent::list::PagedList<Mutation,{usize::MAX}>));reserve_output!(committed,String);}
        let count=if rejects{2}else{3};let bytes=edit_id.len().checked_mul(count).ok_or(VcsError::HistoryReplaying)?;let copy=bytes+std::mem::size_of::<semio_framework_os_spr::EditMessages>()+std::mem::size_of::<semio_framework_os_store_ledger::edit_message_clamp::settlement::EditMessageSettlement>()+std::mem::size_of::<semio_framework_value_persistent::list::PagedList<Mutation,{usize::MAX}>>();let d=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:bytes,depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(false)}let control=control.as_deref_mut().ok_or(VcsError::HistoryReplaying)?;control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(bytes).map_err(VcsError::InverseRefused)?;self.native_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:bytes,..Default::default()};
        let messages = self.message_settlement.as_mut().expect("completed edit message selection").take_messages().expect("final bounded diagnostic owner");
        drop(self.message_settlement.take());
        self.closed_outcomes = self.outcomes.len();
        let inverse = std::mem::take(&mut self.edit_inverse);
        self.replayed.push(semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages });
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

//#region 🎮️OperationPreparation
mod operation_preparation {
    use super::*;
    use crate::replay_preparation::{ArtifactReplayPreparationRequest, OperationMessageSettlement, ReplayRetirement};
    use semio_framework_os_spr::{InputReplacement, MutationMessage};
    use semio_framework_os_store_one_item::ArtifactStoreOneItemGrant;
    use semio_framework_value::ValueError;

    impl<P, M> crate::ReplayOwnedState<P, M>
    where P: Clone, M: semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + semio_framework_os_spr::OpBinary {
        /// ♻️ Binds both installed cleanup authorities without imposing native bounds on cold replay construction.
        pub fn with_retirement_factories(mut self, snapshots: Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>) -> Self
        where P: Send + Sync + 'static, M: Send + 'static {
            self.replay_retirement_factory = Some(semio_framework_os_spr::command::registered_replay_retirement_factory(snapshots, mutations));
            self
        }

        /// 🎟️ Exposes the next exact allocation release separately from the ordinary body-byte grant.
        pub fn replay_retirement_release_byte_demand(&self) -> Result<usize, ValueError> {
            self.replay_retirement.as_ref().map_or(Ok(0), ReplayRetirement::next_release_byte_demand)
        }

        pub(crate) fn begin_replay_snapshot_retirement(&mut self, owner: Arc<P>) {
            if let Some(factory) = self.replay_retirement_factory.as_ref() {
                self.replay_retirement.get_or_insert_with(|| ReplayRetirement::new(Arc::clone(factory))).stage_snapshot(owner);
            } else { semio_framework_os_store_projection::retire_shared_projection::<P, M>(owner); }
        }

        pub(crate) fn begin_replay_mutations_retirement(&mut self, owners: semio_framework_value_persistent::list::PagedList<M, {usize::MAX}>) {
            if let Some(factory) = self.replay_retirement_factory.as_ref() {
                self.replay_retirement.get_or_insert_with(|| ReplayRetirement::new(Arc::clone(factory))).stage_mutations(owners);
            } else { semio_framework_os_store_command::retire_scratch_operations::<P, M>(owners); }
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

        pub(crate) fn step_prepared_operation(&mut self, edit: &semio_framework_os_spr::Edit<M>, prefix: bool, original:Option<RetainedCloneGrant>, control:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>, _deadline: &mut dyn FnMut() -> bool) -> Result<bool, semio_framework_os_vcs::VcsError> {
            use crate::ReplayMode;
    use semio_framework_os_store_ledger::CursorRevisionAccumulator;
            use semio_framework_os_vcs::VcsError;
            let original=original.ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native replay requires the caller's original operation grant")))?;
            let grant=ArtifactStoreOneItemGrant{maximum_items:original.maximum_items,maximum_copy_bytes:original.maximum_copy_bytes,maximum_capacity_bytes:original.maximum_capacity_bytes,maximum_release_bytes:original.maximum_release_bytes,maximum_depth:original.maximum_depth};
            if let Some(settlement) = self.operation_message_settlement.as_mut() {
                if settlement.step(&mut self.edit_messages,original).map_err(VcsError::InverseRefused)? {
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
                let step=control.with_retirement(original,|receiving|retirement.close_step(original,receiving)).map_err(VcsError::InverseRefused)?;
                self.native_receipt=step.progress();
                if retirement.terminal_is_empty() {
                    self.operation_retirement = None;
                }
                return Ok(false);
            }
            let index = self.operation;
            let mutation_id = semio_framework_os_spr::mutation_id_for_edit_operation::<P, M>(edit, index).expect("recorded operation identity");
            if self.prepared_operation.is_none() {
                if self.operation_preparation.is_none() {
                    return Ok(false);
                }
                let task = self.operation_preparation.as_mut().expect("active operation preparation");
                if task.is_ready(){let copy=size_of::<ArtifactReplayPrepared<P,M>>()+size_of::<ArtifactReplayPreparationTask<P,M>>()+size_of::<ArtifactReplayPreparationRetirement<P,M>>();if original.maximum_copy_bytes<copy||original.maximum_depth==0||original.maximum_items==0{return Ok(false);}let work=task.progress();self.preparation_completed.items=self.preparation_completed.items.saturating_add(work.items);self.preparation_completed.bytes=self.preparation_completed.bytes.saturating_add(work.bytes);self.prepared_operation=Some(task.take_prepared().ok_or_else(||VcsError::ValidationFailed("prepared operation has no owners".into()))?);self.operation_retirement=Some(self.operation_preparation.take().unwrap().into_retirement());self.native_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(false);}
                let step = task.step(self.generation,self.revision,original.maximum_copy_bytes,grant,control.ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native replay requires its original construction decoder")))?, &mut || false).map_err(VcsError::InverseRefused)?;
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
                    self.prefix_message_retirement = Some(semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(String::new(), std::mem::take(&mut prepared.messages)));
                }
                return Ok(true);
            }
            let supersession = self.supersessions.get(&mutation_id);
            self.operation_message_settlement = Some(OperationMessageSettlement::new(semio_framework_os_spr::MutationReplayOutcome {
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
