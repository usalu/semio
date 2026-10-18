//! 🚶️ Hydration advance: the single advance step of the retained config store hydration through its phases. See [`super`].

use super::*;

impl<P, M> RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn advance(&mut self, grant: RetainedCloneGrant) -> ConfigStoreHydrationStep<P, M> {
        self.last_retirement_progress = Default::default();
        let maximum_items = grant.maximum_items;
        let maximum_bytes = grant.maximum_copy_bytes;
        if let Some(diagnostic) = self.diagnostic {
            return ConfigStoreHydrationStep::Rejected(diagnostic);
        }
        if maximum_items == 0 {
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        match self.drive_active(grant) {
            Ok(Some(progress)) => { self.last_retirement_progress = progress; return ConfigStoreHydrationStep::Pending(self.progress()); }
            Ok(None) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        if self.fold_auxiliary.is_some() {
            match Self::admit_input(&mut self.fold_auxiliary, grant) {
                Ok(Some((owner, progress))) => {
                    *self.active = Some(owner);
                    if !progress.fits(grant) { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
                    self.last_retirement_progress = progress;
                }
                Ok(None) => {}
                Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
            }
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        if self.retired_messages.is_some() {
            let capacity = size_of::<semio_framework_os_spr::command::MutationMessageLedgerRetirement>();
            if grant.maximum_depth < 2 { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            if grant.maximum_capacity_bytes < capacity { return ConfigStoreHydrationStep::Pending(self.progress()); }
            let messages = self.retired_messages.take().expect("original exhausted messages retain their edit identity");
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            match semio_framework_value::retirement::frame::admit_retirement_frame(messages, child, |messages| semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                Ok((owner, progress)) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                Err((_, messages)) => { *self.retired_messages = Some(messages); return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            }
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        if self.pending_source_lane.is_some() {
            match Self::admit_input(&mut self.pending_source_lane, grant) {
                Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                Ok(None) => {}
                Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
            }
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        if let Some(snapshot) = self.pending_snapshot.as_ref() {
            let owners = self.owners.as_ref().expect("config pending snapshot retains its exact factory");
            let capacity = owners.initial_snapshot_retirement.retirement_birth_bytes(snapshot);
            if grant.maximum_depth < 2 { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            if grant.maximum_capacity_bytes < capacity { return ConfigStoreHydrationStep::Pending(self.progress()); }
            let snapshot = self.pending_snapshot.take().expect("observed exact pending config snapshot");
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            match owners.retire_initial_snapshot_owned(snapshot, child) {
                Ok((owner, progress)) if progress.fits(child) && progress.retained_capacity_bytes == capacity => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                Ok((owner, _)) => { *self.active = Some(owner); return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
                Err((_, snapshot)) => { *self.pending_snapshot = Some(snapshot); return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            }
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        if self.pending_retired_payload.is_some() {
            if grant.maximum_depth < 2 { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let capacity = semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<semio_framework_os_spr::OpPayload>();
            if child.maximum_capacity_bytes < capacity { return ConfigStoreHydrationStep::Pending(self.progress()); }
            let payload = self.pending_retired_payload.take().expect("decoded original payload retains its custody slot");
            match semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(payload, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); self.last_retirement_progress = progress; return ConfigStoreHydrationStep::Pending(self.progress()); }
                Err((_, payload)) => { *self.pending_retired_payload = Some(payload); return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            }
        }
        match self.phase {
            Phase::Begin => {
                if self.actor.0.trim().is_empty() { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
                let history = self.history.as_ref().expect("config history remains retained");
                let expected_id = self.expected_id.as_ref().expect("config identity remains retained");
                let schema = self.schema.as_ref().expect("config schema remains retained");
                if history.doc_id != *expected_id || history.schema != *schema || history.composition.is_some() || !history.conflicts.is_empty() {
                    return self.reject(ConfigStoreHydrationDiagnostic::Identity);
                }
                *self.fold_job = Some(semio_framework_os_spr::HistoryLog::fold_job(history.clone(), semio_framework_os_spr::HistoryShape::Config));
                self.phase = Phase::Fold;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::Fold => {
                let job = self.fold_job.as_mut().expect("config fold job remains retained");
                let result = job.step(grant, &mut || false);
                self.fold_completed = job.completed();
                match result {
                    Ok(semio_framework_os_spr::HistoryFoldJobStep::Pending { progress, .. }) if progress.fits(grant) => { self.last_retirement_progress = progress; }
                    Ok(semio_framework_os_spr::HistoryFoldJobStep::Ready { value: (fold, transitions, replay_order, conflicts), progress }) => {
                        let valid_auxiliary = replay_order.is_empty() && conflicts.is_empty();
                        *self.fold = Some(fold);
                        *self.normalized_transitions = Some(transitions);
                        if replay_order.capacity() != 0 || conflicts.capacity() != 0 { *self.fold_auxiliary = Some((replay_order, conflicts)); }
                        if !progress.fits(grant) || !valid_auxiliary || !self.fold_job.as_ref().is_some_and(ErasedSnapshotRetirement::terminal_is_empty) { return self.reject(ConfigStoreHydrationDiagnostic::Replay); }
                        self.last_retirement_progress = progress;
                        self.fold_job.take();
                        self.phase = Phase::BindGenesis;
                    }
                    _ => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::BindGenesis => {
                let history = std::sync::Arc::get_mut(self.history.as_mut().expect("config history remains retained")).expect("config fold aliases close before genesis binding");
                let fold = self.fold.as_mut().expect("derived config history remains retained");
                let initial = self.initial.take().expect("typed config initial snapshot remains retained");
                let current = self.current.take().expect("typed config current snapshot remains retained");
                let expected_id = self.expected_id.take().expect("config identity remains retained");
                let schema = self.schema.take().expect("config schema remains retained");
                let mut envelope = semio_framework_os_store_document::create_document_envelope_from_genesis::<P, M>(&schema, &expected_id, initial, None);
                envelope.history_shape = semio_framework_os_spr::HistoryShape::Config;
                envelope.cursor = Some(semio_framework_os_store_space_schema::ArtifactCursor::new(Vec::new(), Vec::new(), fold.checkpoint.take()));
                envelope.transitions = self.normalized_transitions.take().expect("normalized config transitions remain retained");
                *self.source_edits = Some(semio_framework_os_store_retirement::resident_backing::PagedSourceIterator::new(std::mem::take(&mut history.edits),Self::vacant_history_edit));
                *self.runtime = Some(ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, std::sync::Arc::new(current), self.initial_digest, semio_framework_os_spr::ActorId(std::mem::take(&mut self.actor.0))));
                *self.expected_id = Some(expected_id);
                *self.schema = Some(schema);
                *self.envelope = Some(envelope);
                self.operation_index = 0;
                self.phase = Phase::BuildAppliedCursor;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::BuildAppliedCursor | Phase::BuildRedoCursor => {
                let fold = self.fold.as_mut().expect("derived config history remains retained");
                let applied = self.phase == Phase::BuildAppliedCursor;
                let ids = if applied { &mut fold.applied } else { &mut fold.redo };
                if let Some(id) = ids.get_mut(self.operation_index) {
                    let cursor = self.envelope.as_mut().expect("config envelope remains retained").cursor.as_mut().expect("config cursor remains retained");
                    if applied { cursor.applied_edit_ids.push(std::mem::take(id)); } else { cursor.redo_edit_ids.push(std::mem::take(id)); }
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = if applied { Phase::BuildRedoCursor } else { Phase::BeginEdit };
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::BeginEdit => {
                let transfer=if self.source_edits.as_ref().expect("config source edits remain retained").publication_complete(){Ok(None)}else{self.source_edits.as_mut().expect("config source edits remain retained").take_next_into(&mut self.pending_source_edit,grant)};
                match transfer{Ok(Some(progress))=>{self.last_retirement_progress=progress;if self.pending_source_edit.is_none(){return ConfigStoreHydrationStep::Pending(self.progress());}},Ok(None)=>{},Err(_)=>return self.reject(ConfigStoreHydrationDiagnostic::Initialization)}
                let Some(source)=self.pending_source_edit.take()else{
                    match Self::close_source(&mut self.source_edits, grant) {
                        Ok((terminal, progress)) => { self.last_retirement_progress = progress; if !terminal { return ConfigStoreHydrationStep::Pending(self.progress()); } }
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::ValidateReplay;
                    return ConfigStoreHydrationStep::Pending(self.progress());
                };
                if source.actor.as_deref().is_none_or(|actor| actor.trim().is_empty()) {
                    *self.pending_source_edit = Some(source);
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                if source.id.is_empty() || source.id.len() > self.maximum_value_bytes || maximum_bytes < source.id.len().saturating_mul(2) {
                    *self.pending_source_edit = Some(source);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                let Some(metadata) = source.meta else {
                    *self.pending_source_edit = Some(semio_framework_os_spr::HistoryEdit { meta: None, ..source });
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                };
                if metadata.len() != source.ops.len() {
                    *self.pending_source_edit = Some(semio_framework_os_spr::HistoryEdit { meta: Some(metadata), ..source });
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                *self.pending_source_lane = source.lane;
                let edit = Edit {
                    line: source.line,
                    id: source.id,
                    actor: source.actor,
                    forwards: Vec::new(),
                    inverse: Vec::new().into(),
                    mutation_meta: Vec::new(),
                    verb: source.verb,
                    sequence_number: self.edit_index as i32 + 1,
                    started_at: source.started_at,
                    finished_at: source.finished_at,
                };
                *self.source_forwards = Some(semio_framework_os_store_retirement::resident_backing::SourceIterator::new(source.ops));
                *self.source_inverse = Some(semio_framework_os_store_retirement::resident_backing::SourceIterator::new(source.inverse));
                *self.source_metadata = Some(semio_framework_os_store_retirement::resident_backing::SourceIterator::new(metadata));
                let runtime = self.runtime.as_mut().expect("config hydration runtime remains retained");
                if runtime.seed_mutation(semio_framework_os_spr::MutationId(edit.id.clone())).is_err() {
                    self.retire_edit(edit);
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                runtime.observe_sequence(self.edit_index as i32 + 1);
                *self.pending_messages = Some(semio_framework_os_spr::EditMessages { edit_id: edit.id.clone(), messages: Vec::new() });
                *self.pending_edit = Some(edit);
                self.operation_index = 0;
                self.phase = Phase::DecodeForward;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::DecodeForward | Phase::DecodeInverse => {
                let payloads = if self.phase == Phase::DecodeForward { self.source_forwards.as_mut() } else { self.source_inverse.as_mut() };
                if self.pending_payload.is_none() {
                    *self.pending_payload = payloads.expect("config operation source remains retained").next();
                }
                if let Some(payload) = self.pending_payload.as_ref() {
                    let payload_bytes = Self::payload_bytes(payload);
                    if payload_bytes == 0 || payload_bytes > self.maximum_value_bytes {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if maximum_bytes < payload_bytes.max(size_of::<M>()) {
                        return ConfigStoreHydrationStep::Pending(self.progress());
                    }
                    if self.phase == Phase::DecodeInverse {
                        let edit = self.pending_edit.as_mut().expect("pending config edit remains retained");
                        if !edit.inverse.has_reserved_slot() {
                            match edit.inverse.reserve_one(maximum_bytes) {
                                Ok(_) => return ConfigStoreHydrationStep::Pending(self.progress()),
                                Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Capacity),
                            }
                        }
                    }
                    let operation = match Self::decode_operation(payload) {
                        Ok(operation) => operation,
                        Err(diagnostic) => return self.reject(diagnostic),
                    };
                    let edit = self.pending_edit.as_mut().expect("pending config edit remains retained");
                    if self.phase == Phase::DecodeForward && edit.forwards.try_reserve_exact(1).is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if self.phase == Phase::DecodeForward {
                        edit.forwards.push(operation);
                    } else {
                        edit.inverse.push_reserved(operation).unwrap_or_else(|_| panic!("funded config inverse slot"));
                    }
                    let payload = self.pending_payload.take().expect("decoded config payload remains retained");
                    *self.pending_retired_payload = Some(payload);
                    self.operation_index += 1;
                } else {
                    if self.phase == Phase::DecodeForward {
                        match Self::close_source(&mut self.source_forwards, grant) {
                        Ok((terminal, progress)) => { self.last_retirement_progress = progress; if !terminal { return ConfigStoreHydrationStep::Pending(self.progress()); } }
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    } else {
                        match Self::close_source(&mut self.source_inverse, grant) {
                        Ok((terminal, progress)) => { self.last_retirement_progress = progress; if !terminal { return ConfigStoreHydrationStep::Pending(self.progress()); } }
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    }
                    self.operation_index = 0;
                    self.phase = if self.phase == Phase::DecodeForward { Phase::DecodeInverse } else { Phase::DecodeMetadata };
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::DecodeMetadata => {
                if self.pending_metadata.is_none() {
                    *self.pending_metadata = self.source_metadata.as_mut().expect("config metadata source remains retained").next();
                }
                if let Some(source) = self.pending_metadata.as_ref() {
                    if source.author_id.as_deref().is_none_or(|actor| actor.trim().is_empty()) { return self.reject(ConfigStoreHydrationDiagnostic::Replay); }
                    let retained_bytes = Self::metadata_retained_bytes(source);
                    if retained_bytes > self.maximum_value_bytes {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if maximum_bytes < retained_bytes.max(size_of::<semio_framework_os_spr::MutationMeta>()) {
                        return ConfigStoreHydrationStep::Pending(self.progress());
                    }
                    let source = self.pending_metadata.take().expect("bounded config metadata remains retained");
                    let (meta, messages) = match mutation_meta_from_history_op_meta(source) {
                        Ok(decoded) => decoded,
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                    };
                    let edit_id = self.pending_edit.as_ref().expect("pending config edit remains retained").id.as_str();
                    if let Some(id) = &meta.mutation_id {
                        if self.runtime.as_mut().expect("config hydration runtime remains retained").seed_edit_operation(edit_id, id.clone()).is_err() {
                            return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                        }
                    }
                    self.runtime.as_mut().expect("config hydration runtime remains retained").observe_timestamp(meta.timestamp);
                    let edit = self.pending_edit.as_mut().expect("pending config edit remains retained");
                    if edit.mutation_meta.try_reserve_exact(1).is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    let pending_messages = self.pending_messages.as_mut().expect("pending message owner remains retained");
                    if pending_messages.messages.try_reserve_exact(messages.len()).is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    edit.mutation_meta.push(meta);
                    pending_messages.messages.extend(messages);
                    self.operation_index += 1;
                } else {
                    match Self::close_source(&mut self.source_metadata, grant) {
                        Ok((terminal, progress)) => { self.last_retirement_progress = progress; if !terminal { return ConfigStoreHydrationStep::Pending(self.progress()); } }
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    self.operation_index = 0;
                    self.phase = Phase::FinishEdit;
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::FinishEdit => {
                let mut edit = self.pending_edit.take().expect("completed config edit remains retained");
                if edit.id.len() > self.maximum_value_bytes || maximum_bytes < edit.id.len() {
                    self.retire_edit(edit);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                semio_framework_os_store_ledger::stamp_edit_semantics::<P, M>(&mut edit, &self.envelope.as_ref().expect("config envelope remains retained").schema);
                let edit_id = edit.id.clone();
                let edit_position = self.envelope.as_ref().expect("config envelope remains retained").vcs.edits.len();
                if let Err(edit) = self.envelope.as_mut().expect("config envelope remains retained").vcs.edits.try_push(edit) {
                    self.retire_edit(edit);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                if self.edit_lookup.as_mut().expect("config edit lookup remains retained").insert(edit_id, edit_position).is_some() {
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                let messages = self.pending_messages.take().expect("completed message owner remains retained");
                if messages.messages.is_empty() {
                    *self.retired_messages = Some(messages);
                } else if let Err(messages) = self.envelope.as_mut().expect("config envelope remains retained").edit_messages.admit(messages) {
                    *self.pending_messages = Some(messages);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                self.edit_index += 1;
                self.record_index += 1;
                self.phase = Phase::BeginEdit;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::ValidateReplay => {
                let envelope = self.envelope.as_ref().expect("config envelope remains retained");
                if let Some(edit) = envelope.vcs.edits.get(self.record_index) {
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let validation = self.validation.as_mut().expect("config validation projection remains retained");
                        let next = match semio_framework_os_vcs::apply_mutation(validation, operation) {
                            Ok((next, _)) => next,
                            Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(validation, next);
                        *self.pending_snapshot = Some(displaced);
                        self.operation_index += 1;
                    } else {
                        self.record_index += 1;
                        self.operation_index = 0;
                    }
                } else {
                    let validation = self.validation.take().expect("completed config validation projection remains retained");
                    *self.pending_snapshot = Some(validation);
                    self.phase = Phase::RetireValidation;
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::RetireValidation => {
                self.record_index = 0;
                self.operation_index = 0;
                self.phase = Phase::ReplayCursor;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::ReplayCursor => {
                let cursor = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref()).expect("persisted config cursor remains retained");
                if let Some(edit_id) = cursor.applied_edit_ids.get(self.record_index) {
                    let envelope = self.envelope.as_ref().expect("config envelope remains retained");
                    let Some(position) = self.edit_lookup.as_ref().expect("config edit lookup remains retained").get(edit_id).copied() else { return self.reject(ConfigStoreHydrationDiagnostic::Replay) };
                    let edit = envelope.vcs.edits.get(position).expect("indexed config edit remains retained");
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let current = self.runtime.as_mut().and_then(ArtifactStoreInitializationRuntime::current_mut).expect("config current remains retained");
                        let next = match semio_framework_os_vcs::apply_mutation(current, operation) {
                            Ok((next, _)) => next,
                            Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(current, next);
                        *self.pending_snapshot = Some(displaced);
                        self.operation_index += 1;
                    } else {
                        self.record_index += 1;
                        self.operation_index = 0;
                    }
                } else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::SeedApplied;
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::SeedApplied | Phase::SeedRedo => {
                let envelope = self.envelope.as_ref().expect("config envelope remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted config cursor remains retained");
                let ids = if self.phase == Phase::SeedApplied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
                if let Some(id) = ids.get(self.record_index) {
                    let Some(position) = self.edit_lookup.as_ref().expect("config edit lookup remains retained").get(id).copied() else { return self.reject(ConfigStoreHydrationDiagnostic::Replay) };
                    let edit = envelope.vcs.edits.get(position).expect("indexed config edit remains retained");
                    let result = if self.phase == Phase::SeedApplied {
                        self.runtime.as_mut().expect("config hydration runtime remains retained").push_applied_edit(edit, envelope.vcs.edits.key_at(position).expect("indexed config edit generation remains retained"))
                    } else {
                        self.runtime.as_mut().expect("config hydration runtime remains retained").push_redo_edit(edit, envelope.vcs.edits.key_at(position).expect("indexed config edit generation remains retained"))
                    };
                    if result.is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    self.record_index += 1;
                } else {
                    self.record_index = 0;
                    self.phase = if self.phase == Phase::SeedApplied { Phase::SeedRedo } else { Phase::RetireHistory };
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::RetireHistory => {
                if self.history.is_some() {
                    match Self::admit_input(&mut self.history, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if self.fold.is_some() {
                    match Self::admit_input(&mut self.fold, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if self.edit_lookup.is_some() {
                    match Self::admit_input(&mut self.edit_lookup, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if self.expected_id.is_some() {
                    match Self::admit_input(&mut self.expected_id, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if self.schema.is_some() {
                    match Self::admit_input(&mut self.schema, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                self.phase = Phase::Finish;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::Finish => {
                let envelope = self.envelope.take().expect("hydrated config envelope remains retained");
                let mut runtime = self.runtime.take().expect("hydrated config runtime remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted config cursor remains retained");
                runtime.set_current_checkpoint_id(cursor.checkpoint_id.clone());
                let owners = self.owners.take().expect("config hydration owners remain retained");
                self.terminal = true;
                ConfigStoreHydrationStep::Ready(Box::new(semio_framework_os_store_engine::lifted_store::config_store_from_initialized_runtime_with_owners(envelope, runtime, self.generation, owners)))
            }
            Phase::Rejected => ConfigStoreHydrationStep::Rejected(self.diagnostic.unwrap_or(ConfigStoreHydrationDiagnostic::Cancelled)),
        }
    }
}
