//! 🚀️ Drawing store initialization: the initialization phase, the initialization authority with its sealing and close steps, the plugin authority implementation and the initialization job.

use super::*;

//#region 🔖️RetainedStoreInitialization
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DrawingStoreInitializationPhase {
    InitializeArena,
    AdmitOwners,
    BindGenesis,
    ValidateEnvelope,
    ValidateEditId { edit: usize },
    ValidateEditMeta { edit: usize, meta: usize },
    ValidateEdit { index: usize },
    StartInitialClone,
    CloneInitialSnapshot,
    TakeInitialSnapshot,
    CloseInitialSnapshotClone,
    CloseInitialSnapshotSource,
    MoveInitialOwner,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    ReturnForward { position: usize, edit: usize, mutation: usize },
    PrepareApplied { position: usize, edit: usize, field: u8 },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    PrepareRedo { position: usize, edit: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

pub(super) struct DrawingStoreInitializationAuthority {
    pub actor: protocol::ActorId,
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub arena_bootstrap_job: DrawingMutationArenaBootstrapJob,
    pub envelope: std::mem::ManuallyDrop<Option<store::os_store::store_schema::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>>>,
    pub runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<DrawingSnapshot>>>,
    pub candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<DrawingSnapshot, DrawingMutation>>>,
    pub retire_pending: std::mem::ManuallyDrop<Option<DrawingRetirementOwner>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub owner_catalog: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationOwnerCatalog>>,
    pub owners: std::mem::ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<DrawingSnapshot, DrawingMutation>>>,
    pub forward_source: std::mem::ManuallyDrop<Option<RetainedCloneSource<DrawingMutation>>>,
    pub forward_from_ledger: bool,
    pub snapshot_source: std::mem::ManuallyDrop<Option<RetainedCloneSource<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>>>,
    pub mutation_candidate: std::mem::ManuallyDrop<Option<DrawingMutationCandidateAuthority>>,
    pub prepared_history_id: std::mem::ManuallyDrop<Option<String>>,
    pub publication: semio_framework_job_publication::RetainedJobPublication,
    pub edit_index: store::ArtifactStoreInitializationEditIndex,
    /// 🧬️ The paged clone of `envelope.vcs.genesis.facts().snapshot()` that becomes the runtime's live fold —
    /// the envelope keeps its own initial snapshot, which `print_document_pack` / the cold from-scratch
    /// fold read back (moving it out left every loaded document's `.pack` empty — ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17).
    pub initial_clone: std::mem::ManuallyDrop<Option<DrawingSnapshot>>,
    pub initial_snapshot_clone: std::mem::ManuallyDrop<Option<<DrawingSnapshot as RetainedClone>::Cursor>>,
    pub initial_clone_source: std::mem::ManuallyDrop<Option<RetainedCloneSource<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>>>,
    pub phase: DrawingStoreInitializationPhase,
    pub resume_phase: Option<DrawingStoreInitializationPhase>,
    pub cancel_requested: bool,
    pub fault: Option<Vec<u8>>,
    pub terminal_handoff: bool,
}

impl DrawingStoreInitializationAuthority {
    pub fn new(
        envelope: store::os_store::store_schema::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>,
        owner_catalog: Result<store::ArtifactStoreInitializationOwnerCatalog, &'static str>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Self {
        let bootstrap_job = DrawingMutationArenaBootstrapJob::new(operation, generation);
        let (owner_catalog, arena_bootstrap_job, phase, fault) = match (owner_catalog, bootstrap_job) {
            (Ok(owner_catalog), Ok(arena_bootstrap_job)) => (Some(owner_catalog), arena_bootstrap_job, DrawingStoreInitializationPhase::InitializeArena, None),
            (Err(error), Ok(mut arena_bootstrap_job)) => {
                arena_bootstrap_job.terminal = true;
                (None, arena_bootstrap_job, DrawingStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec()))
            }
            (_, Err(error)) => (None, DrawingMutationArenaBootstrapJob::inactive(operation, generation), DrawingStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec())),
        };
        Self {
            actor,
            operation,
            generation,
            arena_bootstrap_job,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            retire_pending: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            owner_catalog: std::mem::ManuallyDrop::new(owner_catalog),
            owners: std::mem::ManuallyDrop::new(None),
            forward_source: std::mem::ManuallyDrop::new(None),
            forward_from_ledger: false,
            snapshot_source: std::mem::ManuallyDrop::new(None),
            mutation_candidate: std::mem::ManuallyDrop::new(None),
            prepared_history_id: std::mem::ManuallyDrop::new(None),
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            initial_clone: std::mem::ManuallyDrop::new(None),
            initial_snapshot_clone: std::mem::ManuallyDrop::new(None),
            initial_clone_source: std::mem::ManuallyDrop::new(None),
            phase,
            resume_phase: None,
            cancel_requested: false,
            fault,
            terminal_handoff: false,
        }
    }

    pub fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    pub fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    pub fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = DrawingStoreInitializationPhase::RetireFault;
    }

    pub fn seal_forward(&mut self, edit: usize, index: usize, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        cx.consume_fuel(1);
        let funded = grant.maximum_items > 0
            && grant.maximum_depth >= 1
            && grant.maximum_copy_bytes >= RetainedCloneSource::<DrawingMutation>::constructor_copy_bytes()
            && grant.maximum_capacity_bytes >= RetainedCloneSource::<DrawingMutation>::constructor_capacity_bytes::<()>();
        if !funded || self.retire_pending.is_some() {
            return DrawingInitializationTurn::Yield;
        }
        let replacement = {
            let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its forward is sealed");
            let runtime = self.runtime.as_ref().expect("Drawing runtime remains retained while its forward is sealed");
            let entry = envelope.vcs.edits.get(edit).expect("Drawing applied edit remains retained");
            let superseded = runtime.effective_forward(entry, index, &envelope.schema).is_some_and(|effective| effective.superseded());
            if superseded {
                let original = entry.forwards.get(index).expect("Drawing superseded forward remains retained");
                let mutation_id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| Mutation::<DrawingSnapshot>::mutation_id(original)).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                runtime.supersessions().get(&mutation_id).map(|supersession| store::os_store::replay::admit_replacement::<DrawingSnapshot, DrawingMutation>(original, &supersession.replacement, &envelope.schema))
            } else {
                None
            }
        };
        let (owned, from_ledger) = match replacement {
            Some(Ok(Some(replacement))) => (replacement, false),
            Some(_) => {
                self.fail(b"drawing-store.initializer-superseded-forward");
                return DrawingInitializationTurn::Yield;
            }
            None => {
                let slot = self.envelope.as_mut().and_then(|envelope| envelope.vcs.edits.get_mut(edit)).and_then(|entry| entry.forwards.get_mut(index)).expect("Drawing applied forward remains retained");
                (std::mem::replace(slot, drawing_forward_placeholder()), true)
            }
        };
        match RetainedCloneSource::admit_owned(owned, (), grant) {
            Ok((source, progress)) => {
                *self.forward_source = Some(source);
                self.forward_from_ledger = from_ledger;
                if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
            }
            Err((error, owned, ())) => {
                if from_ledger {
                    let slot = self.envelope.as_mut().and_then(|envelope| envelope.vcs.edits.get_mut(edit)).and_then(|entry| entry.forwards.get_mut(index)).expect("Drawing applied forward slot remains retained");
                    drop(std::mem::replace(slot, owned));
                } else {
                    *self.retire_pending = Some(DrawingRetirementOwner::Mutation(owned));
                }
                self.fault = Some(error.into_message().into_bytes());
                self.phase = DrawingStoreInitializationPhase::RetireFault;
            }
        }
        DrawingInitializationTurn::Yield
    }

    pub fn seal_snapshot(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        cx.consume_fuel(1);
        let funded = grant.maximum_items > 0
            && grant.maximum_depth >= 1
            && grant.maximum_copy_bytes >= RetainedCloneSource::<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>::constructor_copy_bytes()
            && grant.maximum_capacity_bytes >= RetainedCloneSource::<DrawingSnapshot, std::sync::Arc<DrawingSnapshot>>::constructor_capacity_bytes::<()>();
        if !funded {
            return DrawingInitializationTurn::Yield;
        }
        let alias = self.runtime.as_ref().expect("Drawing runtime remains retained while its snapshot is sealed").share_current();
        match RetainedCloneSource::admit_borrowed(alias, |original: &std::sync::Arc<DrawingSnapshot>| &**original, grant) {
            Ok((source, progress)) => {
                *self.snapshot_source = Some(source);
                if let Err(error) = cx.consume_retained(progress) { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
            }
            Err((error, _alias)) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
        }
        DrawingInitializationTurn::Yield
    }

    pub fn release_snapshot(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingInitializationTurn {
        let grant = cx.retained_grant();
        cx.consume_fuel(1);
        let source = self.snapshot_source.as_mut().expect("sealed Drawing snapshot remains retained until its exact release");
        match source.close_step(grant).and_then(|step| { cx.consume_retained(step.progress())?; Ok(()) }) {
            Ok(()) => { if source.terminal_is_empty() { drop(self.snapshot_source.take()); } }
            Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
        }
        DrawingInitializationTurn::Yield
    }

    pub fn pump_active(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
        let grant = cx.retained_grant();
        let step = if self.active.is_some() {
            store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant)?
        } else if self.retire_pending.is_some() {
            store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.active, grant)?
        } else {
            return Ok(false);
        };
        cx.consume_retained(step.progress())?;
        cx.consume_fuel(1);
        Ok(true)
    }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "Drawing initializer close depth overflow"))?; Ok(demand) };
        let unit = semio_framework_value::RetirementDemand { depth: 1, ..Default::default() };
        if !self.publication.terminal_is_empty() { return nested(self.publication.retirement_demands()?); }
        if let Some(active) = self.active.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?); }
        if self.retire_pending.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.retire_pending)?); }
        if self.prepared_history_id.is_some() { return Ok(unit); }
        if let Some(clone) = self.initial_snapshot_clone.as_ref() {
            return nested(semio_framework_value::RetirementDemand { copy_bytes: clone.next_close_copy_byte_demand()?, capacity_bytes: clone.next_close_capacity_byte_demand(body)?, release_bytes: clone.next_close_release_byte_demand()?, depth: clone.next_close_depth_demand()? });
        }
        if let Some(source) = self.initial_clone_source.as_ref() {
            if self.initial_snapshot_clone.is_none() {
                return nested(semio_framework_value::RetirementDemand { copy_bytes: source.next_close_copy_byte_demand()?, capacity_bytes: source.next_close_capacity_byte_demand(body)?, release_bytes: source.next_close_release_byte_demand()?, depth: source.next_close_depth_demand()? });
            }
        }
        if self.initial_clone.is_some() { return Ok(unit); }
        if let Some(candidate) = self.mutation_candidate.as_ref() { return nested(candidate.close_demands(body)?); }
        if let Some(source) = self.snapshot_source.as_ref() {
            return nested(semio_framework_value::RetirementDemand { copy_bytes: source.next_close_copy_byte_demand()?, capacity_bytes: source.next_close_capacity_byte_demand(body)?, release_bytes: source.next_close_release_byte_demand()?, depth: source.next_close_depth_demand()? });
        }
        if let Some(source) = self.forward_source.as_ref() {
            return nested(semio_framework_value::RetirementDemand { copy_bytes: source.next_close_copy_byte_demand()?, capacity_bytes: source.next_close_capacity_byte_demand(body)?, release_bytes: source.next_close_release_byte_demand()?, depth: source.next_close_depth_demand()? });
        }
        if let Some(runtime) = self.runtime.as_ref() { return nested(runtime.initialization_retirement_demands(body)?); }
        if self.owner_catalog.is_some() { return nested(initialization_catalog_close_demands(&self.owner_catalog)?); }
        if let Some(retirement) = self.envelope_retirement.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body)?); }
        if let Some(envelope) = self.envelope.as_ref() {
            return match self.owners.as_ref() {
                Some(owners) => nested(owners.uninstalled_envelope_retirement_demands(envelope)),
                None => nested(store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<DrawingSnapshot, DrawingMutation>()?),
            };
        }
        if let Some(owners) = self.owners.as_ref() { return nested(owners.uninstalled_owners_demands(body)?); }
        Ok(unit)
    }

    pub fn close_original(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty_inner() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Drawing initializer close exceeds original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if !self.publication.terminal_is_empty() { return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.active.is_some() { return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.retire_pending.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.retire_pending, &mut self.active, child); }
        if let Some(value) = self.prepared_history_id.take() {
            *self.retire_pending = Some(DrawingRetirementOwner::HistoryId(value));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(clone) = self.initial_snapshot_clone.as_mut() {
            clone.begin_close();
            let step = clone.close_step(child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) && clone.terminal_is_empty() { drop(self.initial_snapshot_clone.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(source) = self.initial_clone_source.as_mut() {
            let step = source.close_step(child)?;
            if source.terminal_is_empty() { drop(self.initial_clone_source.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(value) = self.initial_clone.take() {
            *self.retire_pending = Some(DrawingRetirementOwner::Snapshot(value));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.mutation_candidate.is_some() {
            let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut);
            let candidate = self.mutation_candidate.as_mut().expect("Drawing mutation candidate remains retained during close");
            let step = candidate.close_step(current, child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                if !candidate.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing mutation candidate reported a false terminal")); }
                drop(self.mutation_candidate.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(source) = self.snapshot_source.as_mut() {
            let step = source.close_step(child)?;
            if source.terminal_is_empty() { drop(self.snapshot_source.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(source) = self.forward_source.as_mut() {
            let step = source.close_step(child)?;
            if source.terminal_is_empty() { drop(self.forward_source.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<DrawingSnapshot>> = std::sync::Arc::new(DrawingSnapshotRetirementFactory::default());
            let step = runtime.close_step(&factory, child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                if !runtime.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing initialization runtime reported a false terminal")); }
                drop(self.runtime.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.owner_catalog.is_some() {
            let step = close_initialization_catalog(&mut self.owner_catalog, child)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope_retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.envelope_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            let Some(owners) = self.owners.take() else {
                return match store::os_store::document_store_owners::bounded_artifact_store_owners::<DrawingSnapshot, DrawingMutation>(child) {
                    Ok((owners, progress)) => { *self.owners = Some(owners); Ok(RetainedCloneStep::Progress(progress)) }
                    Err(refused) => { *self.owners = refused.owners; Err(refused.error.with_retained_progress(refused.progress)) }
                };
            };
            let envelope = self.envelope.take().expect("observed original Drawing envelope");
            return match owners.retire_envelope_uninstalled(envelope, child) {
                Ok((owner, progress)) => { *self.envelope_retirement = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, owners, envelope)) => { *self.owners = Some(owners); *self.envelope = Some(envelope); Err(error) }
            };
        }
        if let Some(owners) = self.owners.as_mut() {
            let step = owners.close_uninstalled_owners_step(child)?;
            if owners.uninstalled_owners_terminal_is_empty() { drop(self.owners.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.arena_bootstrap_job.terminal
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.retire_pending.is_none()
            && self.envelope_retirement.is_none()
            && self.owner_catalog.is_none()
            && self.owners.is_none()
            && self.forward_source.is_none()
            && self.snapshot_source.is_none()
            && self.mutation_candidate.is_none()
            && self.prepared_history_id.is_none()
            && self.initial_clone.is_none()
            && self.initial_clone_source.is_none()
            && self.initial_snapshot_clone.is_none()
            && self.publication.terminal_is_empty()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority {
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.close_demands(maximum_copy_bytes)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let outcome = match self.turn(cx) {
            DrawingInitializationTurn::Yield => semio_framework_job::JobOutcomeBorrow::admit_yield(cx),
            DrawingInitializationTurn::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            DrawingInitializationTurn::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            DrawingInitializationTurn::Fault => {
                let detail = self.fault.as_deref().unwrap_or(b"drawing-store.initializer-fault");
                self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, detail, cx)
            }
        };
        outcome.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Drawing initializer lends only yield and terminal outcomes")),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, DrawingStoreInitializationPhase::Cancelled | DrawingStoreInitializationPhase::Fault) {
            self.phase = DrawingStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<RetainedCloneStep, semio_framework_job::OriginalTurnFailure> {
        let grant = cx.retained_grant();
        self.begin_close();
        let step = self.close_original(grant)?;
        cx.consume_retained(step.progress())?;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            drop(self.mutation_candidate.take());
            self.terminal_handoff = true;
        }
        Ok(step)
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<DrawingSnapshot, DrawingMutation>> {
        if self.phase != DrawingStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        drop(self.mutation_candidate.take());
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for DrawingStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner() || std::thread::panicking(), "Drawing store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn drawing_document_store_initialization_job(
    envelope: store::os_store::store_schema::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<DrawingSnapshot, DrawingMutation> {
    let owner_catalog = store::ArtifactStoreInitializationOwnerCatalog::try_new();
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(DrawingStoreInitializationAuthority::new(envelope, owner_catalog, operation, generation, actor)))
}
//#endregion 🔖️RetainedStoreInitialization
