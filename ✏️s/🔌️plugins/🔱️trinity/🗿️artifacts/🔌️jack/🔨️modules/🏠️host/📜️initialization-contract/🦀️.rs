//! 📜️ Jack store initialization contract: the `ArtifactStoreInitializationAuthority` implementation and the document store initialization job.

use super::*;

//#region 🔖️RetainedStoreInitialization
impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority {
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(maximum_copy_bytes, true)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, ValueError> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"jack-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::Cancelled) {
            self.phase = JackStoreInitializationPhase::RetireCancelled;
        }
        if !matches!(self.phase, JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::RetireFault | JackStoreInitializationPhase::Cancelled | JackStoreInitializationPhase::Fault | JackStoreInitializationPhase::Complete) {
            match self.settle_displaced(cx) {
                Ok(true) => {
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                Ok(false) => {}
                Err(error) => self.fail_with(error),
            }
        }
        match self.phase {
            JackStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                let Some(actor) = self.actor.take() else {
                    self.fail(b"jack-store.initializer-actor-missing");
                    return Ok(None);
                };
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), actor));
                self.phase = JackStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"jack-store.initializer-envelope-missing");
                    return Ok(None);
                };
                if envelope.schema != crate::standards::v1::subsets::any::schema::graph::TRINITY_GRAPH_SCHEMA || envelope.id.is_empty() || envelope.id.len() > JACK_OWNED_FIELD_BYTES {
                    self.fail(b"jack-store.initializer-envelope-invalid");
                } else {
                    self.phase = JackStoreInitializationPhase::ValidateEdit { index: 0 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated Jack envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, JACK_OWNED_FIELD_BYTES) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = JackStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = JackStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"jack-store.initializer-duplicate-or-hostile-edit"),
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Jack envelope remains retained during initial clone").vcs.genesis.facts().snapshot();
                let clone = self.clone.as_mut().expect("Jack initial clone authority remains retained");
                let complete = match clone.step(source, cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.message.as_bytes());
                        return Ok(None);
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("Jack initial snapshot was built one semantic field at a time");
                    drop(self.clone.take());
                    match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, Self::snapshot_factory()) {
                        Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                        Err(initial) => {
                            *self.displaced = Some(initial);
                            self.fail(b"initializer-owned-workspace-adoption");
                        }
                    }
                }
                Ok(None)
            }
            JackStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = JackStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return Ok(None);
                };
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fail(error.as_bytes());
                            self.phase = JackStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fail(error.as_bytes());
                            self.phase = JackStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = JackStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Jack runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = JackStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = JackStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fail(error.as_bytes());
                        self.phase = JackStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Writer runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = JackStoreInitializationPhase::FindRedo { position: 0 };
                    return Ok(None);
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"jack-store.initializer-applied-edit-missing");
                    return Ok(None);
                };
                if edit.id == id {
                    self.phase = JackStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"jack-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    *self.clone = Some(JackSnapshotCloneAuthority::new());
                    self.phase = JackStoreInitializationPhase::CloneInitial;
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("Jack applied edit remains retained");
                match self.runtime.as_mut().expect("Jack runtime remains retained while its forwards fold").fold_forward(entry, mutation, &envelope.schema, JACK_OWNED_FIELD_BYTES) {
                    Ok(store::ArtifactStoreInitializationForward::Folded { displaced, fuel }) => {
                        if let Some(previous) = displaced {
                            *self.displaced = Some(previous);
                        }
                        self.phase = JackStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(fuel as u64);
                    }
                    Ok(store::ArtifactStoreInitializationForward::Exhausted) => self.phase = JackStoreInitializationPhase::CommitApplied { position, edit },
                    Err(_) => self.fail(b"jack-store.initializer-forward-encoding"),
                }
                Ok(None)
            }
            JackStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack applied edit remains retained");

                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained");
                if let Err(error) = runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fail(error.as_bytes());
                    self.phase = JackStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = JackStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = JackStoreInitializationPhase::BuildOwners;
                    return Ok(None);
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"jack-store.initializer-redo-edit-missing");
                    return Ok(None);
                };
                if edit.id == id {
                    self.phase = JackStoreInitializationPhase::CommitRedo { position, edit: scan };
                } else {
                    self.fail(b"jack-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Writer runtime remains retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fail(error.as_bytes());
                    self.phase = JackStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = JackStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::BuildOwners => {
                match store::os_store::document_store_owners::bounded_artifact_store_owners::<JackSnapshot, TrinityGraphMutation>(cx.retained_grant()) {
                    Ok((owners, progress)) => {
                        *self.owners = Some(owners);
                        match cx.consume_retained(progress) {
                            Ok(()) => self.phase = JackStoreInitializationPhase::BuildCandidate,
                            Err(error) => self.fail_with(error),
                        }
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        let _ = cx.consume_retained(refused.progress);
                        self.fail_with(refused.error);
                    }
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"jack-store.initializer-generation-exhausted");
                    return Ok(None);
                };
                let envelope = self.envelope.take().expect("Jack envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Writer runtime remains retained until atomic store construction");
                let owners = self.owners.take().expect("Jack store catalog remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, owners);
                *self.candidate = Some(candidate);
                self.phase = JackStoreInitializationPhase::Complete;
                Ok(None)
            }
            JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::RetireFault => match self.close_for_step(cx) {
                Ok(false) => Ok(None),
                Ok(true) => {
                    self.phase = if self.phase == JackStoreInitializationPhase::RetireCancelled { JackStoreInitializationPhase::Cancelled } else { JackStoreInitializationPhase::Fault };
                    Ok(None)
                }
                Err(error) => {
                    if self.fault.is_none() {
                        self.fail_with(error);
                    }
                    Ok(None)
                }
            },
            JackStoreInitializationPhase::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            JackStoreInitializationPhase::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            JackStoreInitializationPhase::Fault => match self.fault.as_ref() {
                Some(fault) => self.fault_publication.advance_from_fault(fault, cx),
                None => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "jack initializer fault phase lost its fault")),
            },
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete if self.phase == JackStoreInitializationPhase::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Fault if self.fault.is_some() => self.fault_publication.borrow_outcome(descriptor),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "jack initializer cannot resolve this outcome kind")),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, JackStoreInitializationPhase::Cancelled | JackStoreInitializationPhase::Fault) {
            self.phase = JackStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.begin_close();
        self.close_original(grant, true)
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<JackSnapshot, TrinityGraphMutation>> {
        if self.phase != JackStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for JackStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty_inner(), "Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn jack_document_store_initialization_job(
    envelope: store::os_store::store_schema::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<JackSnapshot, TrinityGraphMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(JackStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}
//#endregion 🔖️RetainedStoreInitialization
