// 🚀️ Ready parent initialization: the ready composed parent initialization phase, authority and its store initialization implementation.

//#region 🧬️ComposedParentFixture
#[derive(Clone, Copy, PartialEq, Eq)]
enum ReadyComposedParentInitializationPhase {
    Begin,
    Seed,
    Replay,
    Applied,
    Redo,
    Build,
    Complete,
}

struct ReadyComposedParentInitialization {
    pub envelope: Option<store::os_store::store_schema::ArtifactEnvelope<ComposedParentSnapshot, RecursiveFixtureMutation>>,
    pub runtime: Option<store::ArtifactStoreInitializationRuntime<ComposedParentSnapshot>>,
    pub candidate: Option<store::ArtifactStore<ComposedParentSnapshot, RecursiveFixtureMutation>>,
    pub retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub candidate_generation: u64,
    pub actor: protocol::ActorId,
    edit_index: usize,
    lane_index: usize,
    pub operation_index: usize,
    pub phase: ReadyComposedParentInitializationPhase,
    pub closing: bool,
}

impl ReadyComposedParentInitialization {
    pub fn new(envelope: store::os_store::store_schema::ArtifactEnvelope<ComposedParentSnapshot, RecursiveFixtureMutation>, candidate_generation: u64, actor: protocol::ActorId) -> Self {
        Self {
            envelope: Some(envelope),
            runtime: None,
            candidate: None,
            retirement: None,
            candidate_generation,
            actor,
            edit_index: 0,
            lane_index: 0,
            operation_index: 0,
            phase: ReadyComposedParentInitializationPhase::Begin,
            closing: false,
        }
    }

    fn from_candidate(candidate: store::ArtifactStore<ComposedParentSnapshot, RecursiveFixtureMutation>) -> Self {
        let actor = candidate.local_actor_id().clone();
        Self {
            envelope: None,
            runtime: None,
            candidate: Some(candidate),
            retirement: None,
            actor,
            candidate_generation: 0,
            edit_index: 0,
            lane_index: 0,
            operation_index: 0,
            phase: ReadyComposedParentInitializationPhase::Complete,
            closing: false,
        }
    }

    pub fn fault(code: &[u8]) -> semio_framework_value::ValueError {
        semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, String::from_utf8_lossy(code).into_owned())
    }
}

impl store::ArtifactStoreInitializationAuthority<ComposedParentSnapshot, RecursiveFixtureMutation> for ReadyComposedParentInitialization {
    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| {
            demand.depth += 1;
            demand
        };
        if let Some(candidate) = self.candidate.as_ref() {
            return candidate.close_owned_demands(body).map(nested);
        }
        if let Some(retirement) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return runtime.initialization_retirement_demands(body).map(nested);
        }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.envelope.is_some()), ..Default::default() })
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> {
        if self.closing || cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.retirement.is_some() {
            if cx.should_yield() {
                return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            let grant=cx.retained_grant();
            let stepped = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| (grant, step));
            return match stepped {
                Ok((grant, step)) if step.progress().fits(grant) => {
                    let progress = step.progress();
                    cx.consume_retained(progress)?;
                    cx.consume_fuel((progress.copied_items + progress.released_bytes).max(1) as u64);
                    semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
                }
                _ => Err((Self::fault(b"recursive-parent-replay-retirement-rejected")).into()),
            };
        }
        if cx.should_yield() {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let grant=cx.retained_grant();
            let settled=runtime.settle_current_retirement_step(grant).and_then(|step|{cx.consume_retained(step.progress())?;Ok((grant,step))});
            match settled {
                Ok((_, store::RetainedCloneStep::Complete(_))) => {}
                Ok((grant, store::RetainedCloneStep::Progress(progress))) if progress.fits(grant) => {
                    cx.consume_fuel((progress.copied_items + progress.released_bytes).max(1) as u64);
                    return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
                }
                _ => return Err((Self::fault(b"recursive-parent-current-retirement-rejected")).into()),
            }
        }
        cx.consume_fuel(1);
        let envelope = match self.envelope.as_ref() {
            Some(envelope) => envelope,
            None if self.phase == ReadyComposedParentInitializationPhase::Complete => {
                return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            None => return Err((Self::fault(b"recursive-parent-envelope-owner-missing")).into()),
        };
        match self.phase {
            ReadyComposedParentInitializationPhase::Begin => {
                let initial = envelope.vcs.genesis.facts().share_snapshot();
                let initial_digest = envelope.vcs.genesis.facts().digest();
                self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, initial_digest, self.actor.clone()));
                self.phase = ReadyComposedParentInitializationPhase::Seed;
            }
            ReadyComposedParentInitializationPhase::Seed => {
                if let Some(edit) = envelope.vcs.edits.get(self.edit_index) {
                    let Some(runtime) = self.runtime.as_mut() else { return Err((Self::fault(b"recursive-parent-runtime-owner-missing")).into()) };
                    if self.operation_index == 0 {
                        if runtime.seed_mutation(protocol::MutationId(edit.id.clone())).is_err() {
                            return Err((Self::fault(b"recursive-parent-edit-seed-rejected")).into());
                        }
                        self.operation_index = 1;
                    } else if let Some(meta) = edit.mutation_meta.get(self.operation_index - 1) {
                        if let Some(id) = &meta.mutation_id {
                            if runtime.seed_edit_operation(&edit.id, id.clone()).is_err() {
                                return Err((Self::fault(b"recursive-parent-mutation-seed-rejected")).into());
                            }
                        }
                        runtime.observe_timestamp(meta.timestamp);
                        self.operation_index += 1;
                    } else {
                        runtime.observe_sequence(edit.sequence_number);
                        self.operation_index = 0;
                        self.edit_index += 1;
                    }
                } else {
                    self.edit_index = 0;
                    self.operation_index = 0;
                    self.phase = ReadyComposedParentInitializationPhase::Replay;
                }
            }
            ReadyComposedParentInitializationPhase::Replay => {
                let Some(cursor) = envelope.cursor.as_ref() else { return Err((Self::fault(b"recursive-parent-cursor-missing")).into()) };
                if let Some(edit_id) = cursor.applied_edit_ids.get(self.lane_index) {
                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id) else { return Err((Self::fault(b"recursive-parent-applied-edit-missing")).into()) };
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let Some(runtime) = self.runtime.as_mut() else {
                            return Err((Self::fault(b"recursive-parent-current-owner-missing")).into());
                        };
                        let current = runtime.current_ref();
                        let next = match protocol::apply_diff_cold(operation.diff(current).diff(), current) {
                            Ok(next) => next,
                            Err(_) => return Err((Self::fault(b"recursive-parent-replay-rejected")).into()),
                        };
                        if runtime.adopt_current_owned(next, std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<ComposedParentSnapshot>::default())).is_err() {
                            return Err((Self::fault(b"recursive-parent-current-adoption-rejected")).into());
                        }
                        self.operation_index += 1;
                    } else {
                        self.operation_index = 0;
                        self.lane_index += 1;
                    }
                } else {
                    self.lane_index = 0;
                    self.phase = ReadyComposedParentInitializationPhase::Applied;
                }
            }
            ReadyComposedParentInitializationPhase::Applied | ReadyComposedParentInitializationPhase::Redo => {
                let Some(cursor) = envelope.cursor.as_ref() else { return Err((Self::fault(b"recursive-parent-cursor-missing")).into()) };
                let ids = if self.phase == ReadyComposedParentInitializationPhase::Applied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
                if let Some(id) = ids.get(self.lane_index) {
                    let Some((ledger_key, edit)) = envelope.vcs.edits.find_near_key(|edit| edit.id == *id) else { return Err((Self::fault(b"recursive-parent-lane-edit-missing")).into()) };
                    let Some(runtime) = self.runtime.as_mut() else { return Err((Self::fault(b"recursive-parent-runtime-owner-missing")).into()) };
                    let result = if self.phase == ReadyComposedParentInitializationPhase::Applied { runtime.push_applied_edit(edit, ledger_key) } else { runtime.push_redo_edit(edit, ledger_key) };
                    if result.is_err() {
                        return Err((Self::fault(b"recursive-parent-lane-seed-rejected")).into());
                    }
                    self.lane_index += 1;
                } else {
                    self.lane_index = 0;
                    self.phase = if self.phase == ReadyComposedParentInitializationPhase::Applied {
                        ReadyComposedParentInitializationPhase::Redo
                    } else {
                        ReadyComposedParentInitializationPhase::Build
                    };
                }
            }
            ReadyComposedParentInitializationPhase::Build => {
                let cursor = envelope.cursor.as_ref().expect("checked recursive parent cursor");
                let runtime = self.runtime.as_mut().expect("recursive parent runtime remains retained");
                runtime.set_current_checkpoint_id(cursor.checkpoint_id.clone());
                let envelope = self.envelope.take().expect("recursive parent envelope remains retained");
                let runtime = self.runtime.take().expect("recursive parent runtime remains retained");
                let owners = funded_recursive_fixture_owners::<ComposedParentSnapshot>("recursive-parent").expect("funded recursive parent owners");
                self.candidate = Some(store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, self.candidate_generation, owners));
                self.phase = ReadyComposedParentInitializationPhase::Complete;
            }
            ReadyComposedParentInitializationPhase::Complete => {
                return semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
        }
        semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        semio_framework_plugin_testing_laws::artifact_app_laws::fixture_job_outcome(descriptor)
    }

    fn request_cancel(&mut self) {
        self.closing = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<ComposedParentSnapshot, RecursiveFixtureMutation>> {
        self.candidate.take()
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<store::RetainedCloneStep,semio_framework_job::OriginalTurnFailure> {
        self.begin_close();
        if self.terminal_is_empty(){return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))}
        let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let empty = store::RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(store::RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        let demand = <Self as store::ArtifactStoreInitializationAuthority<ComposedParentSnapshot, RecursiveFixtureMutation>>::retirement_demands(self, grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "composed parent initializer retirement exceeds admitted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        let child = store::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(candidate) = self.candidate.as_mut() {
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, candidate.close_owned_step(child)?, candidate.close_owned_terminal_is_empty(), "composed parent initializer candidate")?;
            if matches!(step, store::RetainedCloneStep::Complete(_)) {
                drop(self.candidate.take());
            }
            return Ok(store::RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<ComposedParentSnapshot>> = std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<ComposedParentSnapshot>::default());
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, runtime.close_step(&factory, child)?, runtime.terminal_is_empty(), "composed parent initializer runtime")?;
            if matches!(step, store::RetainedCloneStep::Complete(_)) {
                drop(self.runtime.take());
            }
            return Ok(store::RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(envelope) = self.envelope.take() {
            let bundle = ComposedParentApp::<true>::build_envelope_decode_owner_bundle().expect("recursive parent retirement catalog");
            self.retirement = Some(bundle.retire_envelope(envelope));
            return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        Ok(store::RetainedCloneStep::Complete(empty))

        })?;
        Ok(if self.terminal_is_empty(){semio_framework_value::retained_clone::RetainedCloneStep::Complete(step.progress())}else{semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())})
    }

    fn terminal_is_empty(&self) -> bool {
        self.envelope.is_none() && self.runtime.is_none() && self.candidate.is_none() && self.retirement.is_none()
    }
}

impl Drop for ReadyComposedParentInitialization {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "recursive parent initializer dropped before exact handoff or retained close");
    }
}
//#endregion 🧬️ComposedParentFixture
