//! ✉️ Edit decoding phases: the envelope admission, cursor building and the forward, inverse and metadata decoding of the hydration step. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_os_spr::StoreState,
    M: semio_framework_os_spr::StoreMutationFor<P>,
{
    /// 🛂 Admits one edit envelope.
    pub(super) fn step_admit_envelope(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let child = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
        let envelope = self.envelope.as_mut().expect("hydration retains its original partially admitted envelope");
        let progress = match envelope.admit_document_next(child) { Ok(progress) => progress, Err(_) => return self.reject(MemberOpenDiagnostic::Initialization) };
        if !progress.fits(child) { return self.reject(MemberOpenDiagnostic::Initialization); }
        let ready = envelope.document_admission_is_complete();
        if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
        if ready { self.phase = Phase::BuildAppliedCursor; }
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🧭 Builds the applied or redo cursor.
    pub(super) fn step_build_applied_cursor(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let fold = self.fold.as_mut().expect("derived history remains retained");
        let applied = self.phase == Phase::BuildAppliedCursor;
        let ids = if applied { &mut fold.applied } else { &mut fold.redo };
        if let Some(id) = ids.get_mut(self.operation_index) {
            let cursor = self.envelope.as_mut().expect("hydrated envelope remains retained").cursor.as_mut().expect("hydrated cursor remains retained");
            if applied { cursor.applied_edit_ids.push(std::mem::take(id)); } else { cursor.redo_edit_ids.push(std::mem::take(id)); }
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = if applied { Phase::BuildRedoCursor } else { Phase::BeginEdit };
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// ✏ Begins one edit.
    pub(super) fn step_begin_edit(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let history = self.history.as_ref().expect("decoded history remains retained");
        let Some(source) = history.edits.get(self.edit_index) else {
            self.operation_index = 0;
            self.phase = Phase::HydrateChanges;
            cx.consume_fuel(1);
            return PersistedDocumentHydrationStep::Pending(self.progress());
        };
        if source.actor.as_deref().is_none_or(|actor| actor.trim().is_empty()) { return self.reject(MemberOpenDiagnostic::Replay); }
        let mut forwards = Vec::new();
        let mut inverse = Vec::new();
        let mut mutation_meta = Vec::new();
        if forwards.try_reserve_exact(source.ops.len()).is_err() || inverse.try_reserve_exact(source.inverse.len()).is_err() || mutation_meta.try_reserve_exact(source.meta.as_ref().map_or(0, Vec::len)).is_err() {
            return self.reject(MemberOpenDiagnostic::Capacity);
        }
        let edit = Edit {
            line: source.line.clone(),
            id: source.id.clone(),
            actor: source.actor.clone(),
            forwards,
            inverse: inverse.into(),
            mutation_meta,
            verb: source.verb.clone(),
            sequence_number: self.edit_index as i32 + 1,
            started_at: source.started_at.clone(),
            finished_at: source.finished_at.clone(),
        };
        let runtime = self.runtime.as_mut().expect("hydration runtime remains retained");
        if runtime.seed_mutation(semio_framework_os_spr::MutationId(edit.id.clone())).is_err() {
            *self.pending_edit = Some(edit);
            return self.reject(MemberOpenDiagnostic::Replay);
        }
        runtime.observe_sequence(self.edit_index as i32 + 1);
        *self.pending_messages = Some(semio_framework_os_spr::EditMessages { edit_id: edit.id.clone(), messages: Vec::new() });
        *self.pending_edit = Some(edit);
        self.operation_index = 0;
        self.phase = Phase::DecodeForward;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// ➡ Decodes the forward or inverse mutation.
    pub(super) fn step_decode_forward(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let source = &self.history.as_ref().expect("decoded history remains retained").edits[self.edit_index];
        let payloads = if self.phase == Phase::DecodeForward { &source.ops } else { &source.inverse };
        if let Some(payload) = payloads.get(self.operation_index) {
            let operation = match Self::decode_operation(payload) {
                Ok(operation) => operation,
                Err(diagnostic) => return self.reject(diagnostic),
            };
            let edit = self.pending_edit.as_mut().expect("pending typed edit remains retained");
            if self.phase == Phase::DecodeForward {
                edit.forwards.push(operation);
            } else {
                edit.inverse.push(operation);
            }
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = if self.phase == Phase::DecodeForward { Phase::DecodeInverse } else { Phase::DecodeMetadata };
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🏷 Decodes the edit metadata.
    pub(super) fn step_decode_metadata(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let source = &self.history.as_ref().expect("decoded history remains retained").edits[self.edit_index];
        let Some(metadata) = source.meta.as_ref() else { return self.reject(MemberOpenDiagnostic::Replay) };
        if metadata.len() != source.ops.len() {
            return self.reject(MemberOpenDiagnostic::Replay);
        }
        if let Some(meta) = metadata.get(self.operation_index) {
            if meta.author_id.as_deref().is_none_or(|actor| actor.trim().is_empty()) { return self.reject(MemberOpenDiagnostic::Replay); }
            let (meta, messages) = match mutation_meta_from_history_op_meta(meta.clone()) {
                Ok(decoded) => decoded,
                Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
            };
            let edit_id = self.pending_edit.as_ref().expect("pending typed edit remains retained").id.as_str();
            if let Some(id) = &meta.mutation_id {
                self.mutation_lookup.as_mut().expect("hydrated mutation index remains retained").insert(id.clone(), (self.edit_index, self.operation_index));
                if self.runtime.as_mut().expect("hydration runtime remains retained").seed_edit_operation(edit_id, id.clone()).is_err() {
                    return self.reject(MemberOpenDiagnostic::Replay);
                }
            }
            self.runtime.as_mut().expect("hydration runtime remains retained").observe_timestamp(meta.timestamp);
            self.pending_edit.as_mut().expect("pending typed edit remains retained").mutation_meta.push(meta);
            self.pending_messages.as_mut().expect("pending message owner remains retained").messages.extend(messages);
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = Phase::FinishEdit;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// ✔ Finishes one edit.
    pub(super) fn step_finish_edit(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let mut edit = self.pending_edit.take().expect("completed typed edit remains retained");
        let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
        semio_framework_os_store_ledger::stamp_edit_semantics::<P, M>(&mut edit, &envelope.schema);
        self.edit_lookup.as_mut().expect("hydrated edit index remains retained").insert(edit.id.clone(), envelope.vcs.edits.len());
        if let Err(edit) = envelope.vcs.edits.try_push(edit) {
            *self.pending_edit = Some(edit);
            return self.reject(MemberOpenDiagnostic::Capacity);
        }
        let messages = self.pending_messages.take().expect("completed message owner remains retained");
        if !messages.messages.is_empty() {
            if let Err(messages) = self.envelope.as_mut().expect("hydrated envelope remains retained").edit_messages.admit(messages) {
                *self.active = Some(Box::new(semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(messages.edit_id, messages.messages)));
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
        }
        self.edit_index += 1;
        self.record_index += 1;
        self.phase = Phase::BeginEdit;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }
}
