//! 🚶️ Hydration step: the single step of the retained persisted document hydration through its phases. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let grant = semio_framework_os_store_member_open::member_step_grant(cx, grant);
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        if let Some(diagnostic) = self.diagnostic {
            return PersistedDocumentHydrationStep::Rejected(diagnostic);
        }
        if let Err(diagnostic) = self.check_authority(cx) {
            return self.reject(diagnostic);
        }
        match self.drive_active(cx, grant) {
            Ok(true) => return PersistedDocumentHydrationStep::Pending(self.progress()),
            Ok(false) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        if cx.should_yield() {
            return PersistedDocumentHydrationStep::Pending(self.progress());
        }
        match self.phase {
            Phase::ScanPack => {
                let pack = self.pack.as_ref().expect("persisted Pack remains retained through scan");
                let remaining = pack.len().saturating_sub(self.pack_scanned);
                if remaining == 0 {
                    self.phase = Phase::DecodePack;
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                let scanned = remaining.min(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
                if scanned == 0 {
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                self.pack_hasher.update(&self.pack.as_ref().expect("retained Pack remains present")[self.pack_scanned..self.pack_scanned + scanned]);
                self.pack_scanned += scanned;
                cx.consume_fuel(scanned as u64);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::DecodePack => {
                let initial = match P::decode_pack(self.pack.as_ref().expect("scanned Pack remains retained")) {
                    Ok(initial) => initial,
                    Err(_) => return self.reject(MemberOpenDiagnostic::Decode),
                };
                *self.initial = Some(initial);
                self.phase = Phase::Begin;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::Begin => {
                if self.actor.0.trim().is_empty() { return self.reject(MemberOpenDiagnostic::Initialization); }
                let history = self.history.as_ref().expect("decoded history remains retained");
                let expected = self.expected.as_ref().expect("document identity remains retained");
                let schema = self.schema.as_ref().expect("document schema remains retained");
                let Some(composition) = history.composition.as_ref() else { return self.reject(MemberOpenDiagnostic::Identity) };
                let dialect_matches = composition.dialect.as_ref().is_some_and(|(artifact_kind, standard, subset)| artifact_kind == &expected.dialect.artifact_kind && standard == &expected.dialect.standard && subset == &expected.dialect.subset);
                let history_owner = match composition.owner.as_ref() {
                    Some((parent, slot, child_id)) => match ArtifactRef::parse_uri(parent) {
                        Ok(parent) => Some(OwnerRef { parent, slot: slot.clone(), child_id: child_id.clone() }),
                        Err(_) => return self.reject(MemberOpenDiagnostic::Owner),
                    },
                    None => None,
                };
                if history.doc_id != expected.artifact_id || history.schema != *schema || !dialect_matches || history_owner.as_ref() != self.owner.as_ref() {
                    return self.reject(MemberOpenDiagnostic::Identity);
                }
                if semio_framework_os_io::publish_native_snapshot::<P>().is_err() { return self.reject(MemberOpenDiagnostic::Initialization); }
                *self.fold_job = Some(semio_framework_os_spr::HistoryLog::fold_job(history.clone(), semio_framework_os_spr::HistoryShape::Document));
                self.phase = Phase::Fold;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::Fold => {
                let mut fold_grant = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
                                let body = fold_grant.maximum_copy_bytes;
let job = self.fold_job.as_mut().expect("history fold job remains retained");
                let required_copy = match job.next_step_grant(1, fold_grant.maximum_copy_bytes) { Ok(required) => required.maximum_copy_bytes, Err(_) => return self.reject(MemberOpenDiagnostic::Replay) };
                fold_grant.maximum_copy_bytes = fold_grant.maximum_copy_bytes.max(required_copy.min(grant.maximum_copy_bytes));
                let result = job.step(fold_grant, body, &mut || cx.should_yield());
                self.fold_completed = job.completed();
                match result {
                    Ok(semio_framework_os_spr::HistoryFoldJobStep::Pending { progress, .. }) if progress.fits(fold_grant) => {
                        if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                    }
                    Ok(semio_framework_os_spr::HistoryFoldJobStep::Ready { value: (fold, transitions, replay_order, conflicts), progress }) => {
                        *self.replay_ids = Some(replay_order);
                        *self.fold = Some(fold);
                        *self.normalized_transitions = Some(transitions);
                        *self.normalized_conflicts = Some(conflicts);
                        if !progress.fits(fold_grant) || !self.fold_job.as_ref().is_some_and(ErasedSnapshotRetirement::terminal_is_empty) { return self.reject(MemberOpenDiagnostic::Replay); }
                        if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                        self.fold_job.take();
                        *self.initialization_catalog = Some(semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog::empty());
                        self.phase = Phase::AdmitInitializationCatalog;
                    }
                    _ => return self.reject(MemberOpenDiagnostic::Replay),
                }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::AdmitInitializationCatalog => {
                let child = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
                let catalog = self.initialization_catalog.as_mut().expect("initialization retains its original page catalog");
                let progress = match catalog.admit_next(child) { Ok(progress) => progress, Err(_) => return self.reject(MemberOpenDiagnostic::Initialization) };
                if !progress.fits(child) { return self.reject(MemberOpenDiagnostic::Initialization); }
                let ready = catalog.admission_is_complete();
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                if ready { self.phase = Phase::BindGenesis; }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::BindGenesis => {
                if !self.initialization_catalog.as_ref().is_some_and(semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog::admission_is_complete) { return self.reject(MemberOpenDiagnostic::Initialization); }
                let child = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
                let birth = match semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis::<P>::verified_pack_birth_demand() { Ok(birth) => birth, Err(_) => return self.reject(MemberOpenDiagnostic::Capacity) };
                if birth.admit(child).is_err() { return PersistedDocumentHydrationStep::Pending(self.progress()); }
                let history = std::sync::Arc::get_mut(self.history.as_mut().expect("decoded history remains retained")).expect("fold aliases close before genesis binding");
                let fold = self.fold.as_mut().expect("derived history remains retained");
                let initial = self.initial.take().expect("typed initial snapshot remains retained");
                let initial_digest = self.pack_digest.unwrap_or_else(|| *self.pack_hasher.finalize().as_bytes());
                let pack = self.pack.take().expect("stored genesis Pack remains retained");
                let (genesis, progress) = match semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis::admit_verified_pack(initial, pack, initial_digest, child) {
                    Ok(admitted) => admitted,
                    Err((_, initial, pack)) => { *self.initial = Some(initial); *self.pack = Some(pack); return self.reject(MemberOpenDiagnostic::Initialization); }
                };
                let expected = self.expected.take().expect("document identity remains retained");
                let schema = self.schema.take().expect("document schema remains retained");
                let mut envelope = crate::empty_document_envelope_from_genesis_owners::<P, M>(schema, expected.artifact_id, genesis, None);
                envelope.dialect = Some(expected.dialect);
                envelope.owner = self.owner.take();
                envelope.active_alternative_id = fold.alternative.take();
                envelope.viewer_checkpoint_id = history.viewer_checkpoint.take();
                envelope.cursor.as_mut().expect("new document retains its original cursor pages").checkpoint_id = fold.checkpoint.take();
                envelope.transitions = self.normalized_transitions.take().expect("normalized transition owners remain retained");
                let supersessions = std::mem::take(&mut fold.supersessions);
                envelope.conflicts = self.normalized_conflicts.take().expect("normalized conflict owners remain retained");
                let current = envelope.vcs.genesis.facts().share_snapshot();
                let mut runtime = ArtifactStoreInitializationRuntime::new_with_owner_catalog(&envelope.id, &envelope.schema, current, initial_digest, semio_framework_os_spr::ActorId(std::mem::take(&mut self.actor.0)), self.initialization_catalog.take().expect("fully admitted initialization catalog remains original"));
                let installed = runtime.set_supersessions(supersessions);
                *self.runtime = Some(runtime);
                *self.envelope = Some(envelope);
                if installed.is_err() {
                    return self.reject(MemberOpenDiagnostic::Initialization);
                }
                self.phase = Phase::AdmitEnvelope;
                self.operation_index = 0;
                if !progress.fits(child) { return self.reject(MemberOpenDiagnostic::Initialization); }
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::AdmitEnvelope => {
                let child = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
                let envelope = self.envelope.as_mut().expect("hydration retains its original partially admitted envelope");
                let progress = match envelope.admit_document_next(child) { Ok(progress) => progress, Err(_) => return self.reject(MemberOpenDiagnostic::Initialization) };
                if !progress.fits(child) { return self.reject(MemberOpenDiagnostic::Initialization); }
                let ready = envelope.document_admission_is_complete();
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                if ready { self.phase = Phase::BuildAppliedCursor; }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::BuildAppliedCursor | Phase::BuildRedoCursor => {
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
            Phase::BeginEdit => {
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
            Phase::DecodeForward | Phase::DecodeInverse => {
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
            Phase::DecodeMetadata => {
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
            Phase::FinishEdit => {
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
            Phase::HydrateChanges => {
                let fold = self.fold.as_mut().expect("history fold remains retained");
                if let Some(change) = fold.changes.get_mut(self.operation_index) {
                    let change = semio_framework_os_vcs::Change { id: std::mem::take(&mut change.id), edit_ids: std::mem::take(&mut change.edit_ids), description: change.description.take(), saved_at: std::mem::take(&mut change.saved_at) };
                    if let Err(change) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.changes.try_push(change) {
                        *self.active = Some(Box::new(semio_framework_os_store_read::ArtifactStoreHistoryMetadataRetirement::change(change)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateCheckpoints;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydrateCheckpoints => {
                let fold = self.fold.as_mut().expect("history fold remains retained");
                if let Some(source) = fold.checkpoints.get_mut(self.operation_index) {
                    let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
                    if self.pin_index == 0 {
                        let checkpoint = semio_framework_os_vcs::Checkpoint { id: std::mem::take(&mut source.id), change_ids: std::mem::take(&mut source.change_ids), parent_id: source.parent_id.take(), authors: Vec::with_capacity(source.authors.len()), message: source.message.take(), timestamp: std::mem::take(&mut source.timestamp), composition_pins: Vec::new() };
                        if let Err(checkpoint) = envelope.vcs.checkpoints.try_push(checkpoint) {
                            *self.active = Some(Box::new(semio_framework_os_store_read::ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint)));
                            return self.reject(MemberOpenDiagnostic::Capacity);
                        }
                        self.pin_index = 1;
                    } else if let Some(author) = source.authors.get_mut(self.pin_index - 1) {
                        envelope.vcs.checkpoints.get_mut(self.operation_index).expect("hydrated checkpoint remains indexed").authors.push(semio_framework_os_vcs::Author { id: std::mem::take(&mut author.id), name: std::mem::take(&mut author.name), avatar: author.avatar.take() });
                        self.pin_index += 1;
                    } else {
                        self.operation_index += 1;
                        self.pin_index = 0;
                    }
                } else {
                    self.operation_index = 0;
                    self.pin_index = 0;
                    self.phase = Phase::HydrateAlternatives;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydrateAlternatives => {
                let fold = self.fold.as_mut().expect("history fold remains retained");
                if let Some(alternative) = fold.alternatives.get_mut(self.operation_index) {
                    let alternative = semio_framework_os_vcs::Alternative { id: std::mem::take(&mut alternative.id), name: std::mem::take(&mut alternative.name), checkpoint_ids: std::mem::take(&mut alternative.checkpoint_ids) };
                    if let Err(alternative) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.alternatives.try_push(alternative) {
                        *self.active = Some(Box::new(semio_framework_os_store_read::ArtifactStoreHistoryMetadataRetirement::alternative(alternative)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydratePins;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydratePins => {
                let Some(source) = self.fold.as_ref().expect("history fold remains retained").checkpoints.get(self.pin_group_index) else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::PrepareReplay;
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                };
                let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
                let Some(checkpoint) = envelope.vcs.checkpoints.get_mut(self.pin_group_index) else {
                    return self.reject(MemberOpenDiagnostic::Replay);
                };
                if self.pin_index == 0 && checkpoint.composition_pins.try_reserve_exact(source.pins.len()).is_err() {
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                if let Some(pin) = source.pins.get(self.pin_index) {
                    let child_ref = match ArtifactRef::parse_uri(&pin.child_uri) {
                        Ok(reference) => reference,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    };
                    if !self.pin_refs.as_mut().expect("hydrated checkpoint pin index remains retained").insert(child_ref.clone()) {
                        return self.reject(MemberOpenDiagnostic::Replay);
                    }
                    checkpoint.composition_pins.push(CompositionPin { child_ref, checkpoint_id: pin.checkpoint_id.clone() });
                    self.pin_index += 1;
                } else {
                    if !owned_frame_funded::<std::collections::HashSet<ArtifactRef>>(grant) { return PersistedDocumentHydrationStep::Pending(self.progress()); }
                    let previous = std::mem::take(self.pin_refs.as_mut().expect("hydrated checkpoint pin index remains retained"));
                    if let Err(diagnostic) = self.admit_active(previous, cx, grant) { return self.reject(diagnostic); }
                    self.pin_group_index += 1;
                    self.pin_index = 0;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::PrepareReplay => {
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
            Phase::ReplayCursor => {
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
            Phase::AdoptInverse => {
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
            Phase::AdoptMessages => {
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
            Phase::SettleReplay => {
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
            Phase::BeginTargets => {
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
            Phase::DecodeTarget => {
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
            Phase::TargetInputs => {
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
            Phase::SeedApplied | Phase::SeedRedo => {
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
            Phase::RetireHistory => {
                macro_rules! retire {
                    ($field:ident) => { if self.$field.is_some() { let step = semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.$field, &mut self.active, RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant }); return self.admitted_step(step, cx); } };
                }
                retire!(history);
                retire!(mutation_lookup);
                retire!(pin_refs);
                retire!(edit_lookup);
                retire!(fold);
                self.phase = Phase::RetirePack;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::RetirePack => {
                if self.pack.is_some() {
                    let step = semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.pack, &mut self.active, RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant });
                    return self.admitted_step(step, cx);
                }
                self.phase = if self.target == PersistedDocumentHydrationTarget::Envelope { Phase::CloseEnvelopeRuntime } else { Phase::Finish };
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::CloseEnvelopeRuntime => {
                let runtime = self.runtime.as_mut().expect("envelope hydration runtime remains retained");
                let owners = self.owners.as_ref().expect("envelope hydration owner catalog remains retained");
                match drive_hydration_runtime(runtime, &owners.initial_snapshot_retirement, cx, grant) {
                    Ok(true) => {
                        self.runtime.take();
                        self.phase = Phase::CloseEnvelopeOwners;
                    }
                    Ok(false) => {}
                    Err(error) => return self.reject(error),
                }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::CloseEnvelopeOwners => {
                let owners = self.owners.as_mut().expect("envelope hydration owner catalog remains retained");
                let child = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
                let Ok(step) = owners.close_uninstalled_owners_step(child) else { return self.reject(MemberOpenDiagnostic::Initialization) };
                if semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "hydration envelope owner catalog").is_err() { return self.reject(MemberOpenDiagnostic::Initialization); }
                match step {
                    RetainedCloneStep::Complete(_) => {
                        self.owners.take();
                        let envelope = self.envelope.take().expect("hydrated envelope remains retained until owner-catalog close");
                        assert!(self.runtime.is_none(), "envelope hydration runtime closes before handoff");
                        self.terminal = true;
                        return PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Envelope(envelope));
                    }
                    RetainedCloneStep::Progress(progress) => if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); },
                }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::Finish => {
                let envelope = self.envelope.take().expect("hydrated envelope remains retained");
                let output = match self.target {
                    PersistedDocumentHydrationTarget::Store { generation } => {
                        let mut runtime = self.runtime.take().expect("hydrated runtime remains retained");
                        let cursor = envelope.cursor.as_ref().expect("persisted cursor remains retained");
                        runtime.set_current_checkpoint_id(cursor.checkpoint_id.clone());
                        let owners = self.owners.take().expect("store hydration owners remain retained");
                        PersistedDocumentHydrationOutput::Store(Box::new(ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, generation, owners)))
                    }
                    PersistedDocumentHydrationTarget::Envelope => unreachable!("envelope hydration hands off while closing its uninstalled owner catalog"),
                };
                self.terminal = true;
                PersistedDocumentHydrationStep::Ready(output)
            }
            Phase::Rejected => PersistedDocumentHydrationStep::Rejected(self.diagnostic.unwrap_or(MemberOpenDiagnostic::Stale)),
        }
    }
}
