//! 🔁️ Replay and target phases: the replay preparation and cursor, inverse and message adoption, replay settling and the target phases of the hydration step. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static + semio_framework_os_spr::ColdMutation<P>,
{
    /// 🎞 Prepares the replay.
    pub(super) fn step_prepare_replay(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let ids = self.replay_ids.as_mut().expect("prepared replay identity owners remain retained");
        if let Some(id) = ids.get_mut(self.record_index) {
            let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
            let Some(position) = self.edit_lookup.as_ref().expect("hydrated edit index remains retained").get(id).copied() else { return self.reject(MemberOpenDiagnostic::Replay) };
            let edit = envelope.vcs.edits.get(position).expect("indexed hydrated edit remains retained");
            self.replay_total = self.replay_total.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));
            self.replay_order.as_mut().expect("prepared replay order remains retained").push(std::mem::take(id));
            self.record_index += 1;
        } else {
            self.replay_ids.take();
            let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
            let supersessions = std::mem::take(&mut *self.runtime.as_mut().expect("hydration runtime remains retained").supersessions);
            match EditReplay::new_counted(semio_framework_os_store_replay::ReplayMode::Report, envelope.vcs.genesis.facts().share_snapshot(), self.replay_order.take().expect("prepared replay order remains retained"), 0, &envelope.schema, supersessions, self.replay_total) {
                Ok(replay) => { *self.replay = Some(replay); self.phase = Phase::ReplayCursor; },
                Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
            }
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// ⏯ Advances the replay cursor.
    pub(super) fn step_replay_cursor(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
        let replay = self.replay.as_mut().expect("hydration replay remains retained");
        let edits = HydratedReplayEdits { edits: &envelope.vcs.edits, positions: self.edit_lookup.as_ref().expect("hydrated replay index remains retained") };
        let stepped = replay.step(&edits, &mut || {
            cx.consume_fuel(1);
            cx.should_yield()
        });
        match stepped {
            Ok(ReplayStep::Pending(_)) => return PersistedDocumentHydrationStep::Pending(self.progress()),
            Ok(ReplayStep::Finished(_)) => {}
            Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
        }
        let finished = self.replay.as_mut().expect("finished hydration replay remains retained").finish();
        match finished { Ok(result) => *self.loaded_result = Some(result), Err(_) => return self.reject(MemberOpenDiagnostic::Replay) }
        self.phase = Phase::AdoptInverse;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// ↩ Adopts the replayed inverse.
    pub(super) fn step_adopt_inverse(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let result = self.loaded_result.as_mut().expect("completed hydration replay remains retained");
        if let Some((id, inverse)) = result.rebased_inverse.last_mut() {
            let Some(position) = self.edit_lookup.as_ref().expect("hydrated edit index remains retained").get(id).copied() else { return self.reject(MemberOpenDiagnostic::Replay) };
            let edit = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.edits.get_mut(position).expect("indexed hydrated edit remains retained");
            let displaced = std::mem::replace(&mut edit.inverse, std::mem::take(inverse));
            let (id, _) = result.rebased_inverse.pop().expect("adopted inverse identity remains retained");
            *self.active = Some(Box::new(semio_framework_os_store_ledger::edit_retirement::ArtifactStoreOperationRowsRetirement::new(displaced, self.owners.as_ref().expect("hydration owner catalog remains retained").mutation_retirement.clone()).with_identity(id)));
        } else { self.phase = Phase::AdoptMessages; }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 💬 Adopts the replayed messages.
    pub(super) fn step_adopt_messages(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let result = self.loaded_result.as_mut().expect("completed hydration replay remains retained");
        if result.replayed.last().is_some_and(|entry| entry.messages.is_empty()) && !owned_frame_funded::<(semio_framework_os_spr::EditMessages, Option<semio_framework_os_spr::EditMessages>)>(grant) { return PersistedDocumentHydrationStep::Pending(self.progress()); }
        if let Some(entry) = result.replayed.pop() {
            let ledger = &mut self.envelope.as_mut().expect("hydrated envelope remains retained").edit_messages;
            if entry.messages.is_empty() {
                let previous = ledger.remove_id(&entry.edit_id);
                if let Err(diagnostic) = self.admit_active((entry, previous), cx, grant) { return self.reject(diagnostic); }
            } else if let Some(previous) = ledger.get_mut_by_id(&entry.edit_id) {
                let previous = std::mem::replace(previous, entry);
                *self.active = Some(Box::new(semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(previous.edit_id, previous.messages)));
            } else if let Err(entry) = ledger.admit(entry) {
                *self.active = Some(Box::new(semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(entry.edit_id, entry.messages)));
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
        } else {
            let runtime = self.runtime.as_mut().expect("hydration runtime remains retained");
            runtime.set_supersessions(std::mem::take(&mut result.supersessions)).expect("replay returns the original effective-input owner");
            let state = result.take_state().expect("completed replay retains its reached projection");
            let factory = self.owners.as_ref().expect("hydration owner catalog remains retained").initial_snapshot_retirement.clone();
            *self.active = Some(Box::new(semio_framework_value::factory::FactorySharedRetirement::new(runtime.replace_current_alias(state), factory)));
            self.phase = Phase::SettleReplay;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// ⚖ Settles the replay.
    pub(super) fn step_settle_replay(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        if let Some(result) = self.loaded_result.take() {
            let owners = self.owners.as_ref().expect("hydration owner catalog remains retained");
            let mut retirement = self.loaded_replay_retirement(owners);
            *retirement.loaded_replay = self.replay.take();
            *retirement.finished = Some(result);
            *self.active = Some(Box::new(retirement));
        } else {
            self.record_index = 0;
            self.operation_index = 0;
            self.phase = Phase::BeginTargets;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🎯 Begins the target phases.
    pub(super) fn step_begin_targets(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        if self.target_address.is_some() {
            let step = semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.target_address, &mut self.active, RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant });
            return self.admitted_step(step, cx);
        }
        if self.target_source.is_none() {
            *self.target_source = Some(std::sync::Arc::new(std::mem::take(&mut self.envelope.as_mut().expect("hydrated envelope remains retained").transitions)));
            self.record_index = 0;
        }
        let source = self.target_source.as_ref().expect("immutable transition source remains retained");
        if let Some(transition) = source.get(self.record_index) {
            let mut at = 0;
            let kind = semio_framework_os_spr::wire::read_varint_u64(&transition.diff.payload, &mut at);
            if !transition.target.is_empty() || !matches!(kind, Ok(6)) { self.record_index += 1; }
            else {
                let source = source.clone();
                let index = self.record_index;
                *self.target_decoder = Some(semio_framework_os_spr::HistoryFoldJob::new(move |control| async move { semio_framework_os_spr::decode_history_transition_controlled(&source[index].diff.payload, &control).await }));
                self.phase = Phase::DecodeTarget;
            }
        } else {
            let transitions = std::sync::Arc::into_inner(self.target_source.take().expect("immutable transition source remains retained")).expect("target decoder aliases close before transition handoff");
            self.envelope.as_mut().expect("hydrated envelope remains retained").transitions = transitions;
            self.record_index = 0;
            self.phase = Phase::SeedApplied;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🎙 Decodes one target.
    pub(super) fn step_decode_target(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let decode_grant = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
        let job = self.target_decoder.as_mut().expect("transition target decoder remains retained");
        match job.step(decode_grant, decode_grant.maximum_copy_bytes, &mut || cx.should_yield()) {
            Ok(semio_framework_os_spr::HistoryFoldJobStep::Pending { progress, .. }) if progress.fits(decode_grant) => {
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
            }
            Ok(semio_framework_os_spr::HistoryFoldJobStep::Ready { value: target, progress }) => {
                *self.pending_target = Some(target);
                if !progress.fits(decode_grant) || !self.target_decoder.as_ref().is_some_and(ErasedSnapshotRetirement::terminal_is_empty) { return self.reject(MemberOpenDiagnostic::Replay); }
                if !matches!(self.pending_target.as_ref(), Some(semio_framework_os_spr::HistoryTransition::Supersede(_))) { return self.reject(MemberOpenDiagnostic::Replay); }
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                self.target_decoder.take();
                self.operation_index = 0;
                self.phase = Phase::TargetInputs;
            }
            _ => return self.reject(MemberOpenDiagnostic::Replay),
        }
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 📥 Collects the target inputs.
    pub(super) fn step_target_inputs(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let Some(semio_framework_os_spr::HistoryTransition::Supersede(target)) = self.pending_target.as_ref() else { return self.reject(MemberOpenDiagnostic::Replay); };
        if let Some(input) = target.inputs.get(self.operation_index) {
            let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
            let operation = self.mutation_lookup.as_ref().expect("hydrated mutation index remains retained").get(&input.target).and_then(|(edit, index)| envelope.vcs.edits.get(*edit).and_then(|edit| edit.forwards.get(*index)));
            let original = operation.map(<M as Mutation<P>>::conflict_target).unwrap_or_default();
            let replacement = operation.and_then(|operation| semio_framework_os_store_replay::admit_replacement::<P, M>(operation, &input.replacement, &envelope.schema).ok().flatten());
            if replacement.is_none() && !owned_frame_funded::<(Vec<String>, Option<Vec<String>>, Option<Vec<String>>, Vec<String>)>(grant) { return PersistedDocumentHydrationStep::Pending(self.progress()); }
            let replacement_target = replacement.as_ref().map(<M as Mutation<P>>::conflict_target);
            let mut written = match &replacement_target { Some(target) => semio_framework_os_store_text_io::text_format::common_address(&original, target), None => semio_framework_os_store_text_io::text_format::common_address(&original, &original) };
            let previous = self.target_address.take();
            let next = match &previous { Some(known) => semio_framework_os_store_text_io::text_format::common_address(known, &written), None => std::mem::take(&mut written) };
            *self.target_address = Some(next);
            let retired_strings = (original, replacement_target, previous, written);
            if let Some(replacement) = replacement {
                let owners = self.owners.as_ref().expect("hydration owner catalog remains retained");
                let (original, replacement_target, previous, written) = retired_strings;
                let strings = original.into_iter().chain(replacement_target.into_iter().flatten()).chain(previous.into_iter().flatten()).chain(written).collect::<Vec<String>>();
                *self.active = Some(Box::new(semio_framework_os_store_ledger::edit_retirement::ArtifactStoreOperationRowsRetirement::new(vec![replacement], owners.mutation_retirement.clone()).with_strings(strings)));
            } else if let Err(diagnostic) = self.admit_active(retired_strings, cx, grant) { return self.reject(diagnostic); }
            self.operation_index += 1;
        } else {
            let step = match semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.pending_target, &mut self.active, RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant }) {
                Ok(step) => step,
                Err(_) => return self.reject(MemberOpenDiagnostic::Initialization),
            };
            if self.pending_target.is_some() { return self.admitted_step(Ok(step), cx); }
            let source = std::sync::Arc::get_mut(self.target_source.as_mut().expect("immutable transition source remains retained")).expect("decoder aliases close before target mutation");
            let original = std::mem::replace(&mut source[self.record_index].target, self.target_address.take().unwrap_or_default());
            if original.capacity() != 0 { *self.target_address = Some(original); }
            self.record_index += 1;
            self.phase = Phase::BeginTargets;
            return self.admitted_step(Ok(step), cx);
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🌾 Seeds the applied or redo target.
    pub(super) fn step_seed_applied(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
        let cursor = envelope.cursor.as_ref().expect("persisted cursor remains retained");
        let ids = if self.phase == Phase::SeedApplied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
        if let Some(id) = ids.get(self.record_index) {
            let Some(position) = self.edit_lookup.as_ref().expect("hydrated edit index remains retained").get(id).copied() else { return self.reject(MemberOpenDiagnostic::Replay) };
            let edit = envelope.vcs.edits.get(position).expect("indexed hydrated edit remains retained");
            let ledger_key = envelope.vcs.edits.key_at(position).expect("indexed hydrated ledger key remains retained");
            let result = if self.phase == Phase::SeedApplied {
                self.runtime.as_mut().expect("hydration runtime remains retained").push_applied_edit(edit, ledger_key)
            } else {
                self.runtime.as_mut().expect("hydration runtime remains retained").push_redo_edit(edit, ledger_key)
            };
            if result.is_err() {
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
            self.record_index += 1;
        } else {
            self.record_index = 0;
            self.phase = if self.phase == Phase::SeedApplied { Phase::SeedRedo } else { Phase::RetireHistory };
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }
}
