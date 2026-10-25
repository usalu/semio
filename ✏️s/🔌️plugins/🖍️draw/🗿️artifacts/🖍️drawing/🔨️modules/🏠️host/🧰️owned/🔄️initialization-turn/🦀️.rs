//! 🔄️ Drawing initialization turn: the turn outcome and the `turn` of the store initialization authority that drives its phase machine one bounded turn at a time.

use super::*;

//#region 🔖️RetainedStoreInitialization
/// 🔁️ What one Drawing initializer turn produced before the job lends its outcome.
pub(super) enum DrawingInitializationTurn {
    Yield,
    Complete,
    Cancelled,
    Fault,
}

impl DrawingStoreInitializationAuthority {
    pub fn turn(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.arena_bootstrap_job.terminal = true;
            self.fail(b"drawing-store.initializer-stale-authority");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, DrawingStoreInitializationPhase::InitializeArena | DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::Cancelled) {
            self.phase = DrawingStoreInitializationPhase::RetireCancelled;
        }
        match self.pump_active(cx) {
            Err(error) => {
                self.fault = Some(error.into_message().into_bytes());
                self.phase = DrawingStoreInitializationPhase::RetireFault;
            }
            Ok(true) => return DrawingInitializationTurn::Yield,
            Ok(false) => {}
        }
        if !matches!(self.phase, DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::RetireFault | DrawingStoreInitializationPhase::Cancelled | DrawingStoreInitializationPhase::Fault | DrawingStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(cx.retained_grant()) {
                    Ok(RetainedCloneStep::Complete(_)) => {}
                    Ok(RetainedCloneStep::Progress(progress)) => {
                        if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                        cx.consume_fuel(1);
                        return DrawingInitializationTurn::Yield;
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
            }
        }
        match self.phase {
            DrawingStoreInitializationPhase::InitializeArena => {
                match self.arena_bootstrap_job.step(cx) {
                    DrawingMutationArenaBootstrapStep::Ready => self.phase = DrawingStoreInitializationPhase::AdmitOwners,
                    DrawingMutationArenaBootstrapStep::Pending { .. } | DrawingMutationArenaBootstrapStep::Blocked => {}
                    DrawingMutationArenaBootstrapStep::Cancelled => self.phase = DrawingStoreInitializationPhase::RetireCancelled,
                    DrawingMutationArenaBootstrapStep::Fault(error) => self.fail(error.as_bytes()),
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::AdmitOwners => self.turn_admit_owners(cx),
            DrawingStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                let owner_catalog = self.owner_catalog.take().expect("pre-admitted Drawing owner catalog");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new_with_owner_catalog(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), self.actor.clone(), owner_catalog));
                self.phase = DrawingStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"drawing-store.initializer-envelope-missing");
                    return DrawingInitializationTurn::Yield;
                };
                if envelope.schema != semio_s_artifact_draw_drawing_model::DRAWING_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > DRAWING_OWNED_FIELD_BYTES {
                    self.fail(b"drawing-store.initializer-envelope-invalid");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditId { edit: 0 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditId { edit } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEdit { index: 0 };
                    return DrawingInitializationTurn::Yield;
                };
                if entry.id.is_empty() || entry.id.len() > DRAWING_OWNED_FIELD_BYTES || entry.actor.as_ref().is_some_and(|actor| actor.len() > DRAWING_OWNED_FIELD_BYTES) || entry.started_at.len() > DRAWING_OWNED_FIELD_BYTES {
                    self.fail(b"drawing-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta: 0 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                let entry = envelope.vcs.edits.get(edit).expect("Drawing edit remains retained during metadata validation");
                let Some(value) = entry.mutation_meta.get(meta) else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditId { edit: edit + 1 };
                    return DrawingInitializationTurn::Yield;
                };
                if value.mutation_id.as_ref().is_some_and(|id| id.0.len() > DRAWING_OWNED_FIELD_BYTES) {
                    self.fail(b"drawing-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta: meta + 1 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, usize::MAX) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = DrawingStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = DrawingStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"drawing-store.initializer-duplicate-edit"),
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::StartInitialClone => self.turn_start_initial_clone(cx),
            DrawingStoreInitializationPhase::CloneInitialSnapshot => {
                let grant = cx.retained_grant();
                let source = self.initial_clone_source.as_ref().expect("retained Drawing snapshot clone source");
                let clone = self.initial_snapshot_clone.as_mut().expect("retained native Drawing snapshot clone");
                match clone.advance(source.borrow(), grant.maximum_copy_bytes, grant).and_then(|step| { cx.consume_retained(step.progress())?; Ok(step) }) {
                    Ok(step) => {
                        cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                        if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = DrawingStoreInitializationPhase::TakeInitialSnapshot; }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes());self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::TakeInitialSnapshot => {
                let clone = self.initial_snapshot_clone.as_mut().expect("completed Drawing snapshot clone");
                *self.initial_clone = clone.take();
                clone.begin_close();
                if self.initial_clone.is_some() { self.phase = DrawingStoreInitializationPhase::CloseInitialSnapshotClone; }
                else { self.fail(b"drawing-store.initializer-clone-false-terminal"); }
                cx.consume_fuel(size_of::<DrawingSnapshot>().max(1) as u64);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CloseInitialSnapshotClone => {
                let grant = cx.retained_grant();
                let clone = self.initial_snapshot_clone.as_mut().expect("closing native Drawing snapshot clone");
                match clone.close_step(grant).and_then(|step| { cx.consume_retained(step.progress())?; Ok(step) }) {
                    Ok(step) => {
                        cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                        if matches!(step, RetainedCloneStep::Complete(_)) && clone.terminal_is_empty() { drop(self.initial_snapshot_clone.take());self.phase = DrawingStoreInitializationPhase::CloseInitialSnapshotSource; }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes());self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CloseInitialSnapshotSource => {
                let grant = cx.retained_grant();
                let source = self.initial_clone_source.as_mut().expect("closing Drawing snapshot clone source");
                match source.close_step(grant).and_then(|step| { cx.consume_retained(step.progress())?; Ok(step) }) {
                    Ok(step) => {
                        cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                        if source.terminal_is_empty() { drop(self.initial_clone_source.take()); self.phase = DrawingStoreInitializationPhase::MoveInitialOwner; }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes());self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::MoveInitialOwner => {
                let _envelope = self.envelope.as_ref().expect("Drawing envelope remains retained during initial owner move");
                let initial = self.initial_clone.take().expect("Drawing initial clone remains retained until the runtime adopts it");
                match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(DrawingSnapshotRetirementFactory::default())) {
                    Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                    Err(initial) => {
                        *self.retire_pending = Some(DrawingRetirementOwner::Snapshot(initial));
                        self.fail(b"initializer-owned-workspace-adoption");
                    }
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::SeedHistory { edit, lane, index } => self.turn_seed_history(edit, lane, index, cx),
            DrawingStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Drawing runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = DrawingStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = DrawingStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = DrawingStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::FindApplied { position } => self.turn_find_applied(position, cx),
            DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation } => self.turn_apply_forward(position, edit, mutation, cx),
            DrawingStoreInitializationPhase::ReturnForward { position, edit, mutation } => self.turn_return_forward(position, edit, mutation, cx),
            DrawingStoreInitializationPhase::PrepareApplied { position, edit, field } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                match field {
                    0 => {
                        *self.prepared_history_id = Some(clone_drawing_string(&entry.id).expect("validated Drawing applied id remains admitted"));
                        self.phase = DrawingStoreInitializationPhase::CommitApplied { position, edit };
                        cx.consume_fuel(entry.id.len().max(1) as u64);
                    }
                    _ => self.fail(b"drawing-store.initializer-applied-preparation"),
                }
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CommitApplied { position, edit } => {
                let id = self.prepared_history_id.take().expect("Drawing applied id was retained in its own preparation grant");
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained");
                if let Err(error) = runtime.push_applied_admitted(id, entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = DrawingStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::FindRedo { position } => self.turn_find_redo(position, cx),
            DrawingStoreInitializationPhase::PrepareRedo { position, edit } => {
                let id = &self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained").id;
                *self.prepared_history_id = Some(clone_drawing_string(id).expect("validated Drawing redo id remains admitted"));
                self.phase = DrawingStoreInitializationPhase::CommitRedo { position, edit };
                cx.consume_fuel(id.len().max(1) as u64);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.prepared_history_id.take().expect("Drawing redo id was retained in its own preparation grant");
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Drawing runtime remains retained").push_redo_admitted(id, entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = DrawingStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                DrawingInitializationTurn::Yield
            }
            DrawingStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"drawing-store.initializer-generation-exhausted");
                    return DrawingInitializationTurn::Yield;
                };
                let envelope = self.envelope.take().expect("Drawing envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Drawing runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, self.owners.take().expect("Drawing document store owners remain admitted until atomic store construction"));
                *self.candidate = Some(candidate);
                self.phase = DrawingStoreInitializationPhase::Complete;
                DrawingInitializationTurn::Complete
            }
            DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::RetireFault => match self.close_original(cx.retained_grant()).and_then(|step| { cx.consume_retained(step.progress())?; cx.consume_fuel(1); Ok(matches!(step, RetainedCloneStep::Complete(_))) }) {
                Ok(false) => DrawingInitializationTurn::Yield,
                Ok(true) => {
                    self.terminal_handoff = true;
                    if self.phase == DrawingStoreInitializationPhase::RetireCancelled {
                        self.phase = DrawingStoreInitializationPhase::Cancelled;
                        DrawingInitializationTurn::Cancelled
                    } else {
                        self.phase = DrawingStoreInitializationPhase::Fault;
                        DrawingInitializationTurn::Fault
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    DrawingInitializationTurn::Yield
                }
            },
            DrawingStoreInitializationPhase::Complete => DrawingInitializationTurn::Complete,
            DrawingStoreInitializationPhase::Cancelled => DrawingInitializationTurn::Cancelled,
            DrawingStoreInitializationPhase::Fault => DrawingInitializationTurn::Fault,
        }
    }

    /// 🧱️ Admits the arena owners of the initialization.
    fn turn_admit_owners(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        let funded = store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<DrawingSnapshot, DrawingMutation>().is_ok_and(|demand| grant.maximum_items > 0 && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_depth >= demand.depth);
        if funded {
            match store::os_store::document_store_owners::bounded_artifact_store_owners::<DrawingSnapshot, DrawingMutation>(grant) {
                Ok((owners, progress)) => {
                    *self.owners = Some(owners);
                    match cx.consume_retained(progress) {
                        Ok(()) => self.phase = DrawingStoreInitializationPhase::ValidateEnvelope,
                        Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                    }
                }
                Err(refused) => {
                    *self.owners = refused.owners;
                    let _ = cx.consume_retained(refused.progress);
                    self.fault = Some(refused.error.into_message().into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                }
            }
        }
        cx.consume_fuel(1);
        DrawingInitializationTurn::Yield
    }

    /// 🪵️ Starts the clone of the initial snapshot.
    fn turn_start_initial_clone(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        let funded = grant.maximum_items > 0
            && grant.maximum_depth >= 1
            && grant.maximum_copy_bytes >= RetainedCloneSource::<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>::constructor_copy_bytes()
            && grant.maximum_capacity_bytes >= RetainedCloneSource::<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>::constructor_capacity_bytes::<()>();
        if funded {
            let genesis = self.envelope.as_ref().expect("retained Drawing clone genesis").vcs.genesis.facts().share_snapshot();
            match RetainedCloneSource::admit_borrowed(genesis, |original: &std::sync::Arc<DrawingSnapshot>| &**original, grant) {
                Ok((source, progress)) => {
                    *self.initial_clone_source = Some(source);
                    *self.initial_snapshot_clone = Some(DrawingSnapshot::retained_clone_cursor());
                    match cx.consume_retained(progress) {
                        Ok(()) => self.phase = DrawingStoreInitializationPhase::CloneInitialSnapshot,
                        Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                    }
                }
                Err((error, _genesis)) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
            }
        }
        cx.consume_fuel(1);
        DrawingInitializationTurn::Yield
    }

    /// 🔩️ Seeds one history lane edit.
    fn turn_seed_history(&mut self, edit: usize, lane: u8, index: usize, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while causal history is seeded");
        let Some(entry) = envelope.vcs.edits.get(edit) else {
            self.phase = DrawingStoreInitializationPhase::FoldSupersessions { transition: 0 };
            return DrawingInitializationTurn::Yield;
        };
        let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained while history is seeded");
        match lane {
            0 => {
                if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {
                    runtime.observe_sequence(entry.sequence_number);
                    self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                }
            }
            1 if index < entry.forwards.len() => {
                let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                }
            }
            1 => self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
            2 if index < entry.mutation_meta.len() => {
                runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
            }
            _ => self.phase = DrawingStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
        }
        cx.consume_fuel(1);
        DrawingInitializationTurn::Yield
    }

    /// ⚙️ Finds the next applied edit of the history.
    fn turn_find_applied(&mut self, position: usize, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let Some(id) = self.applied_id(position) else {
            let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
            self.runtime.as_mut().expect("Drawing runtime remains retained").set_current_checkpoint_id(checkpoint);
            self.phase = DrawingStoreInitializationPhase::FindRedo { position: 0 };
            return DrawingInitializationTurn::Yield;
        };
        let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
        let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained");
        let Some(edit) = envelope.vcs.edits.get(scan) else {
            self.fail(b"drawing-store.initializer-applied-edit-missing");
            return DrawingInitializationTurn::Yield;
        };
        if edit.id == id {
            self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
        } else {
            self.fail(b"drawing-store.initializer-applied-edit-missing");
        }
        cx.consume_fuel(1);
        DrawingInitializationTurn::Yield
    }

    /// 🧲️ Applies one forward mutation of an applied edit.
    fn turn_apply_forward(&mut self, position: usize, edit: usize, mutation: usize, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        if self.forward_source.is_none() {
        let needs_workspace = {
            let envelope = self.envelope.as_ref().expect("retained initializer envelope");
            let runtime = self.runtime.as_ref().expect("retained initializer runtime");
            envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
        };
        if needs_workspace && self.snapshot_source.is_none() && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
            self.resume_phase = Some(self.phase);
        
            self.phase = DrawingStoreInitializationPhase::StartInitialClone;
            cx.consume_fuel(1);
            return DrawingInitializationTurn::Yield;
        }
        if self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).is_none_or(|entry| mutation >= entry.forwards.len()) {
            self.phase = DrawingStoreInitializationPhase::PrepareApplied { position, edit, field: 0 };
            return DrawingInitializationTurn::Yield;
        }
        let withdrawn = {
            let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its forwards fold");
            envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).is_none_or(|effective| effective.operation().is_none())
        };
        if withdrawn {
            self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
            cx.consume_fuel(1);
            return DrawingInitializationTurn::Yield;
        }
        if self.mutation_candidate.is_none() {
            match DrawingMutationCandidateAuthority::try_new(self.operation, self.generation) {
                Ok(candidate) => *self.mutation_candidate = Some(candidate),
                Err(DrawingMutationArenaBorrowError::NotReady | DrawingMutationArenaBorrowError::Contended) => return DrawingInitializationTurn::Yield,
                Err(error) => {
                    self.fail(error.as_str().as_bytes());
                    return DrawingInitializationTurn::Yield;
                }
            }
            cx.consume_fuel(1);
            return DrawingInitializationTurn::Yield;
        }
        return self.seal_forward(edit, mutation, cx);
        }
        let need_shared = {
            let forward = self.forward_source.as_ref().expect("sealed Drawing forward remains retained").borrow();
            self.mutation_candidate.as_ref().expect("Drawing mutation candidate remains retained").needs_shared_snapshot(forward.get())
        };
        if need_shared && self.snapshot_source.is_none() {
            return self.seal_snapshot(cx);
        }
        if !need_shared && self.snapshot_source.is_some() {
            return self.release_snapshot(cx);
        }
        let forward = self.forward_source.as_ref().expect("sealed Drawing forward remains retained").borrow();
        let runtime = self.runtime.as_mut().expect("retained initializer runtime");
        let candidate = self.mutation_candidate.as_mut().expect("Drawing mutation candidate remains retained");
        let stepped = match self.snapshot_source.as_ref() {
            Some(sealed) => candidate.step(DrawingSourceAccess::Shared(runtime.current_ref(), sealed.borrow()), forward, cx),
            None => candidate.step(DrawingSourceAccess::Exclusive(runtime.current_mut().expect("Drawing runtime current snapshot remains retained")), forward, cx),
        };
        let candidate_complete = match stepped {
            Ok(complete) => complete,
            Err(error) => {
                self.fail(error.as_bytes());
                return DrawingInitializationTurn::Yield;
            }
        };
        if candidate_complete {
            self.mutation_candidate.as_mut().expect("Drawing completed mutation candidate remains retained").take().expect("Drawing mutation overlay terminal commit witness remains exact");
            drop(self.mutation_candidate.take());
            self.phase = DrawingStoreInitializationPhase::ReturnForward { position, edit, mutation };
        }
        DrawingInitializationTurn::Yield
    }

    /// 🪛️ Returns the forward mutation of an applied edit.
    fn turn_return_forward(&mut self, position: usize, edit: usize, mutation: usize, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        let source = self.forward_source.as_mut().expect("sealed Drawing forward remains retained until its exact return");
        match source.take_authority(grant) {
            Ok(RetainedCloneSourceTake::Pending(progress)) => {
                if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
            }
            Ok(RetainedCloneSourceTake::Ready(original, progress)) => {
                if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                drop(self.forward_source.take());
                if self.forward_from_ledger {
                    let slot = self.envelope.as_mut().and_then(|envelope| envelope.vcs.edits.get_mut(edit)).and_then(|entry| entry.forwards.get_mut(mutation)).expect("Drawing ledger forward slot remains retained");
                    drop(std::mem::replace(slot, original));
                } else {
                    *self.retire_pending = Some(DrawingRetirementOwner::Mutation(original));
                }
                if matches!(self.phase, DrawingStoreInitializationPhase::ReturnForward { .. }) {
                    self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                }
            }
            Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
        }
        cx.consume_fuel(1);
        DrawingInitializationTurn::Yield
    }

    /// 🔧️ Finds the next redo edit of the history.
    fn turn_find_redo(&mut self, position: usize, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let Some(id) = self.redo_id(position) else {
            self.edit_index.clear();
            self.phase = DrawingStoreInitializationPhase::BuildCandidate;
            return DrawingInitializationTurn::Yield;
        };
        let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
        let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained");
        let Some(edit) = envelope.vcs.edits.get(scan) else {
            self.fail(b"drawing-store.initializer-redo-edit-missing");
            return DrawingInitializationTurn::Yield;
        };
        if edit.id == id {
            self.phase = DrawingStoreInitializationPhase::PrepareRedo { position, edit: scan };
        } else {
            self.fail(b"drawing-store.initializer-redo-edit-missing");
        }
        cx.consume_fuel(1);
        DrawingInitializationTurn::Yield
    }
}
//#endregion 🔖️RetainedStoreInitialization
