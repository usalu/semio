//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 3 of the cut by subject).

use super::*;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct DemoEphemeralPreparationFactory {
    footprint: ArtifactStoreOneItemFootprint,
    #[factory_child]
    published_root: Arc<DemoPublishedRootWitness>,
    #[factory_child]
    snapshots: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>>,
    #[factory_child]
    mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<DemoMutation>>,
}

impl DemoEphemeralPreparationFactory {
    fn admissible() -> Self {
        Self { footprint: ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 64 }, published_root: Arc::new(DemoPublishedRootWitness::new()), snapshots: Arc::new(DemoSnapshotRetirementFactory), mutations: Arc::new(DemoMutationRetirementFactory) }
    }
}

struct DemoEphemeralPreparation {
    request: Option<ArtifactEphemeralOneItemPreparationRequest<DemoSnapshot, DemoMutation>>,
    prepared: Option<ArtifactEphemeralOneItemPrepared<DemoSnapshot>>,
    semantic_refusal: std::mem::ManuallyDrop<Option<semio_framework_os_spr::MutationApplyError>>,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    base: Option<ArtifactEphemeralBaseRead<DemoSnapshot>>,
    mutation: Option<DemoMutation>,
    active: Option<Box<dyn ErasedSnapshotRetirement>>,
    snapshots: Option<Arc<dyn SnapshotRetirementFactory<DemoSnapshot>>>,
    mutations: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<DemoMutation>>>,
    factories: [Option<semio_framework_value::FactoryAuthority>;3],
    published_root: Option<Arc<DemoPublishedRootWitness>>,
    cancelled: bool,
    closing: bool,
}

impl ArtifactEphemeralOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoEphemeralPreparationFactory {
    fn preflight(&self, _mutation: &DemoMutation) -> Result<ArtifactStoreOneItemFootprint, String> {
        Ok(self.footprint)
    }

    fn begin(
        &self,
        request: ArtifactEphemeralOneItemPreparationRequest<DemoSnapshot, DemoMutation>,
    ) -> Result<Box<dyn ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation>>, ArtifactEphemeralOneItemPreparationRequest<DemoSnapshot, DemoMutation>> {
        Ok(Box::new(DemoEphemeralPreparation { request: Some(request), prepared: None, semantic_refusal: std::mem::ManuallyDrop::new(None), checkpoint: ArtifactStoreOneItemCheckpoint::default(), base: None, mutation: None, active: None, snapshots: Some(self.snapshots.clone()), mutations: Some(self.mutations.clone()), factories: [None,None,None], published_root: Some(self.published_root.clone()), cancelled: false, closing: false }))
    }
}

impl DemoEphemeralPreparation {
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        use semio_framework_value::RetirementDemand;
        let nested=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"ephemeral child depth overflow"))?;Ok(demand)};
        if let Some(owner)=self.active.as_ref(){return nested(artifact_retirement_box_demands(owner,body)?);}
        if self.semantic_refusal.is_some() { let mut demand = artifact_retirement_owned_birth_demands(&self.semantic_refusal)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "semantic refusal close depth overflow"))?; return Ok(demand); }
        if self.request.is_some(){return Ok(RetirementDemand {copy_bytes:std::mem::size_of::<ArtifactEphemeralOneItemPreparationRequest<DemoSnapshot,DemoMutation>>(),depth:1,..Default::default()});}
        if let Some(prepared)=self.prepared.as_ref(){return Ok(RetirementDemand {capacity_bytes:self.snapshots.as_ref().expect("original root issuer").retirement_birth_bytes(&prepared.next_root),depth:2,..Default::default()});}
        if let Some(base)=self.base.as_ref(){return match &base.0 {ArtifactEphemeralBaseOwner::Transient(root)=>Ok(RetirementDemand {capacity_bytes:self.snapshots.as_ref().expect("original base issuer").retirement_birth_bytes(root),depth:2,..Default::default()}),ArtifactEphemeralBaseOwner::Presence(_)|ArtifactEphemeralBaseOwner::TransientRead(_)=>Ok(RetirementDemand {depth:1,..Default::default()})};}
        if let Some(mutation)=self.mutation.as_ref(){return Ok(RetirementDemand {capacity_bytes:self.mutations.as_ref().expect("original mutation issuer").retirement_birth_bytes(mutation),depth:2,..Default::default()});}
        if self.published_root.is_some()||self.snapshots.is_some()||self.mutations.is_some(){return Ok(RetirementDemand {copy_bytes:std::mem::size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
        self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()),|factory|nested(factory.demands(body)?))
    }
}

impl ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> for DemoEphemeralPreparation {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        if !grant.permits_one() || self.cancelled {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.semantic_refusal.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original preparation retains its semantic refusal owner")); }
        if self.prepared.is_some() {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        let copied_bytes = std::mem::size_of::<DemoSnapshot>() + std::mem::size_of::<ArtifactEphemeralOneItemPreparationRequest<DemoSnapshot, DemoMutation>>() + std::mem::size_of::<ArtifactEphemeralOneItemPrepared<DemoSnapshot>>();
        let demand = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::shared_retirement_allocation_bytes::<DemoSnapshot>(), depth: 1 };
        if copied_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.depth > grant.maximum_depth { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let mut progress = demand.admit(grant.retained_grant())?;
        progress.copied_bytes = copied_bytes;
        let request = self.request.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "ephemeral preparation lost its request"))?;
        let outcome = request.mutation.diff(&request.base);
        if outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error) {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "demo ephemeral mutation rejected against its base"));
        }
        let original = match semio_framework_os_spr::apply_diff(outcome.diff(), &request.base) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original ephemeral mutation application refused")); } };
        let witness = self.published_root.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "ephemeral preparation lost its original witness"))?;
        let next_root = Arc::new(original);
        witness.publish(&next_root);
        let request = self.request.take().expect("borrowed admitted original request remains present");
        self.base = Some(request.base); self.mutation = Some(request.mutation);
        self.prepared = Some(ArtifactEphemeralOneItemPrepared { next_root });
        self.checkpoint = ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [7; 32] };
        Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, progress))
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactEphemeralOneItemPrepared<DemoSnapshot>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<ArtifactEphemeralOneItemPrepared<DemoSnapshot>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty=RetainedCloneProgress::default();let grant=grant.retained_grant();
        if !self.closing||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"ephemeral original close depth is not funded"));}
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
        let child=RetainedCloneGrant {maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if self.active.is_some(){return artifact_retirement_box_close_step(&mut self.active,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.semantic_refusal.is_some() { return artifact_retirement_admit_owned(&mut self.semantic_refusal, &mut self.active, child); }
        if let Some(request)=self.request.take(){self.base=Some(request.base);self.mutation=Some(request.mutation);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        if let Some(prepared)=self.prepared.take(){return match self.snapshots.as_ref().expect("original prepared root issuer").retire(prepared.next_root,child){Ok((owner,progress))=>{self.active=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{self.prepared=Some(ArtifactEphemeralOneItemPrepared {next_root:original});Err(error)}};}
        if let Some(base)=self.base.take(){
            return match base.0 {
                ArtifactEphemeralBaseOwner::Transient(root)=>match self.snapshots.as_ref().expect("original transient base issuer").retire(root,child){Ok((owner,progress))=>{self.active=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{self.base = Some(ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::Transient(original)));Err(error)}},
                ArtifactEphemeralBaseOwner::Presence(read)=>match read.try_return_to_registry_witness(){Ok(witness)=>{drop(witness);Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}))},Err(original)=>{self.base = Some(ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::Presence(original)));Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original presence base registry is busy"))}},
                ArtifactEphemeralBaseOwner::TransientRead(read)=>match read.try_return_to_registry_witness(){Ok(witness)=>{drop(witness);Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}))},Err(original)=>{self.base = Some(ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::TransientRead(original)));Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original transient base registry is busy"))}},
            };
        }
        if let Some(mutation)=self.mutation.take(){return match self.mutations.as_ref().expect("original ephemeral mutation issuer").retire_owned(mutation,child){Ok((owner,progress))=>{self.active=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{self.mutation=Some(original);Err(error)}};}
        if let Some(witness)=self.published_root.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=witness;self.factories[0]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        if let Some(snapshot)=self.snapshots.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=snapshot;self.factories[1]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        if let Some(mutation)=self.mutations.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=mutation;self.factories[2]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        let slot=self.factories.iter_mut().find(|slot|slot.is_some()).expect("original ephemeral close capability");let factory=slot.as_mut().unwrap();let step=factory.step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,factory.terminal_is_empty(),"original ephemeral preparation issuer")?;if factory.terminal_is_empty(){*slot=None;}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.semantic_refusal.is_none()&&self.request.is_none()&&self.prepared.is_none()&&self.base.is_none()&&self.mutation.is_none()&&self.active.is_none()&&self.snapshots.is_none()&&self.mutations.is_none()&&self.published_root.is_none()&&self.factories.iter().all(Option::is_none)}
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DemoRetainedCloneEditMode {
    Apply,
    Reject,
    Fault,
    OverBudget,
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct DemoRetainedCloneEdit {
    mode: DemoRetainedCloneEditMode,
}

struct DemoRetainedCloneEditCursor {
    mode: DemoRetainedCloneEditMode,
    phase: u8,
    inverse: Option<Vec<DemoMutation>>,
    active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    semantic_refusal: std::mem::ManuallyDrop<Option<semio_framework_os_spr::MutationApplyError>>,
    cancelled: bool,
    closing: bool,
}

impl DemoRetainedCloneEditCursor {
    fn close_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(retirement) = self.active_retirement.as_ref() {
            let mut demand = artifact_retirement_box_demands(retirement, body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "retained clone fixture close depth overflow"))?;
            return Ok(demand);
        }
        if self.semantic_refusal.is_some() { let mut demand = artifact_retirement_owned_birth_demands(&self.semantic_refusal)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "semantic refusal close depth overflow"))?; return Ok(demand); }
        if let Some(mutation) = self.inverse.as_ref().and_then(|inverse| inverse.last()) {
            return Ok(RetirementDemand { capacity_bytes: DemoMutationRetirementFactory.retirement_birth_bytes(mutation), depth: 2, ..Default::default() });
        }
        if let Some(inverse) = self.inverse.as_ref() {
            return Ok(RetirementDemand { release_bytes: inverse.capacity().checked_mul(std::mem::size_of::<DemoMutation>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"inverse backing extent overflow"))?, depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
}

impl RetainedCloneEdit<DemoSnapshot, DemoMutation> for DemoRetainedCloneEdit {
    type Cursor = DemoRetainedCloneEditCursor;

    fn preflight(&self, _mutation: &DemoMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        if self.mode == DemoRetainedCloneEditMode::Reject {
            return Err("retained clone fixture rejects before owner transfer".into());
        }
        Ok(ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 65_536 })
    }

    fn begin_demand(&self) -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 } }
    fn snapshot_cursor_birth_demand(&self) -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 } }
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError> {
        let progress=self.begin_demand().admit(grant)?;
        Ok((DemoRetainedCloneEditCursor { mode: self.mode, phase: 0, inverse: None, semantic_refusal: std::mem::ManuallyDrop::new(None), active_retirement: None, cancelled: false, closing: false },progress))
    }
}

impl RetainedCloneEditCursor<DemoSnapshot, DemoMutation> for DemoRetainedCloneEditCursor {
    fn foreign_step_presence(&self)->bool{false}
    fn advance(&mut self, base: RetainedCloneRef<'_, DemoSnapshot>, post: &mut DemoSnapshot, mutation: RetainedCloneRef<'_, DemoMutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
        if self.cancelled || self.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress::default()));
        }
        if self.semantic_refusal.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original preparation retains its semantic refusal owner")); }
        if self.mode == DemoRetainedCloneEditMode::OverBudget {
            return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress {
                copied_items: grant.maximum_items.saturating_add(1),
                copied_bytes: grant.maximum_copy_bytes.saturating_add(1),
                retained_capacity_bytes: grant.maximum_capacity_bytes.saturating_add(1), released_bytes: 0 }));
        }
        if self.phase == 0 {
            self.phase = 1;
            return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: 0 }));
        }
        if self.mode == DemoRetainedCloneEditMode::Fault {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "retained clone fixture edit fault"));
        }
        if self.phase == 1 {
            let bytes = std::mem::size_of::<DemoMutation>();
            if grant.maximum_capacity_bytes < bytes { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
            let inverse = mutation.get().inverse(base.get())?;
            let valid = inverse.len() == 1;
            self.inverse = Some(inverse);
            if !valid { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "retained clone fixture expected one exact inverse row").with_retained_progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() })); }
            self.phase = 2;
            return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: bytes, released_bytes: 0 }));
        }
        let copied_bytes = std::mem::size_of::<DemoSnapshot>();
        if grant.maximum_copy_bytes < copied_bytes { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
        let outcome = mutation.get().diff(base.get());
        if outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error) {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "retained clone fixture mutation rejected against its immutable base"));
        }
        let original = match semio_framework_os_spr::apply_diff(outcome.diff(), base.get()) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original retained mutation application refused")); } };
        *post = original;
        self.phase = 3;
        Ok(RetainedCloneEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
    }

    fn take_inverse(&mut self) -> Option<Vec<DemoMutation>> {
        (self.phase == 3).then(|| self.inverse.take()).flatten()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "retained clone fixture close exceeds granted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active_retirement.is_some() {
            let step = artifact_retirement_box_close_step(&mut self.active_retirement, child)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.semantic_refusal.is_some() { return artifact_retirement_admit_owned(&mut self.semantic_refusal, &mut self.active_retirement, child); }
        if let Some(inverse) = self.inverse.as_mut() {
            if let Some(mutation) = inverse.pop() {
                return match DemoMutationRetirementFactory.retire_owned(mutation, child) {
                    Ok((owner, progress)) => {
                        self.active_retirement = Some(owner);
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                    Err((error, mutation)) => {
                        inverse.push(mutation);
                        Err(error)
                    }
                };
            }
            drop(self.inverse.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demand(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.semantic_refusal.is_none() && self.inverse.is_none() && self.active_retirement.is_none()
    }
}

fn retained_clone_preparation_factory(mode: DemoRetainedCloneEditMode) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> {
    Arc::new(RetainedClonePreparationFactory::new(Arc::new(DemoRetainedCloneEdit { mode }), Arc::new(DemoMutationRetirementFactory), Arc::new(DemoSnapshotRetirementFactory), 64).expect("retained clone preparation fixture factory"))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ProbedRetainedClonePreparationFactory {
    #[factory_child]
    inner: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>>,
    #[factory_child]
    dropped: Arc<std::sync::atomic::AtomicUsize>,
}

impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for ProbedRetainedClonePreparationFactory {
    fn begin_batch_digest(&self,edit:&mut Option<Box<Edit<DemoMutation>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ArtifactStoreBatchDigest<DemoMutation>>,RetainedCloneProgress)>,ValueError>{admit_artifact_batch_digest(edit,grant)}
    fn preflight(&self, mutation: &DemoMutation, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> { self.inner.preflight(mutation,lane) }
    fn begin_demand(&self, mutation: &DemoMutation, lane: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
        let child = self.inner.begin_demand(mutation,lane)?;
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand {
            capacity_bytes: child.capacity_bytes.checked_add(std::mem::size_of::<ProbedRetainedClonePreparation>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"probe source extent overflow"))?,
            depth: child.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"probe source depth overflow"))?,
        })
    }
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>)> {
        let demand = match self.begin_demand(&request.mutation,request.lane) { Ok(demand)=>demand,Err(error)=>return Err((error,request)) };
        if let Err(error) = demand.admit(grant.retained_grant()) { return Err((error,request)); }
        let wrapper = std::mem::size_of::<ProbedRetainedClonePreparation>();
        let child = ArtifactStoreOneItemGrant { maximum_capacity_bytes: grant.maximum_capacity_bytes-wrapper, maximum_depth: grant.maximum_depth-1, ..grant };
        match self.inner.begin(request,child) {
            Ok((inner,mut progress)) => {
                assert!(progress.fits(child.retained_grant()),"original typed ingress must honor its supplied child grant");
                progress.retained_capacity_bytes += wrapper;
                Ok((Box::new(ProbedRetainedClonePreparation { inner:Some(inner),dropped:Some(Arc::clone(&self.dropped)),counter:None,closing:false }),progress))
            }
            Err((error,original))=>Err((error,original)),
        }
    }
}

struct ProbedRetainedClonePreparation {
    inner: Option<Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>>,
    dropped: Option<Arc<std::sync::atomic::AtomicUsize>>,
    counter: Option<semio_framework_value::FactoryAuthority>,
    closing: bool,
}

impl ProbedRetainedClonePreparation {
    fn close_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(inner)=self.inner.as_ref() {
            if inner.terminal_is_empty() { return Ok(RetirementDemand { release_bytes:std::mem::size_of_val(inner.as_ref()),depth:1,..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes:inner.next_close_copy_byte_demand()?,capacity_bytes:inner.next_close_capacity_byte_demand(body)?,release_bytes:inner.next_close_release_byte_demand()?,depth:inner.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"probe child depth overflow"))? });
        }
        if self.dropped.is_some() { return Ok(RetirementDemand { copy_bytes:std::mem::size_of::<Arc<std::sync::atomic::AtomicUsize>>(),depth:1,..Default::default() }); }
        if let Some(counter)=self.counter.as_ref() { let mut demand=counter.demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"probe counter depth overflow"))?;return Ok(demand); }
        Ok(Default::default())
    }
}

impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for ProbedRetainedClonePreparation {
    fn advance(&mut self,grant:ArtifactStoreOneItemGrant)->Result<ArtifactStoreOneItemPreparationStep,ValueError> {
        if self.closing || grant.maximum_depth<=1 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        self.inner.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"probe no longer owns its typed preparation"))?.advance(ArtifactStoreOneItemGrant { maximum_depth:grant.maximum_depth-1,..grant })
    }
    fn checkpoint(&self)->ArtifactStoreOneItemCheckpoint { self.inner.as_ref().map_or_else(ArtifactStoreOneItemCheckpoint::default,|inner|inner.checkpoint()) }
    fn prepared(&self)->Option<&ArtifactStoreOneItemPrepared<DemoSnapshot,DemoMutation>> { self.inner.as_ref().and_then(|inner|inner.prepared()) }
    fn take_prepared(&mut self)->Option<ArtifactStoreOneItemPrepared<DemoSnapshot,DemoMutation>> { self.inner.as_mut().and_then(|inner|inner.take_prepared()) }
    fn cancel(&mut self) { self.closing=true;if let Some(inner)=self.inner.as_mut(){inner.cancel();} }
    fn begin_close(&mut self) { self.closing=true;if let Some(inner)=self.inner.as_mut(){inner.begin_close();} }
    fn close_step(&mut self,grant:ArtifactStoreOneItemGrant)->Result<RetainedCloneStep,ValueError> {
        let grant=grant.retained_grant();let empty=RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        let demand=self.close_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"probe close exceeds supplied depth")); }
        if grant.maximum_copy_bytes<demand.copy_bytes || grant.maximum_capacity_bytes<demand.capacity_bytes || grant.maximum_release_bytes<demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child=RetainedCloneGrant { maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant };
        if let Some(inner)=self.inner.as_mut() {
            if inner.terminal_is_empty(){let released_bytes=std::mem::size_of_val(inner.as_ref());drop(self.inner.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes,..empty}));}
            let step=inner.close_step(ArtifactStoreOneItemGrant {maximum_items:child.maximum_items,maximum_copy_bytes:child.maximum_copy_bytes,maximum_capacity_bytes:child.maximum_capacity_bytes,maximum_release_bytes:child.maximum_release_bytes,maximum_depth:child.maximum_depth})?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,inner.terminal_is_empty(),"original probe preparation")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(original)=self.dropped.take() {
            original.fetch_add(1,std::sync::atomic::Ordering::SeqCst);
            let original:Arc<dyn semio_framework_value::FactoryRetirement>=original;
            self.counter=Some(semio_framework_value::FactoryAuthority::new(original));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..empty}));
        }
        if let Some(counter)=self.counter.as_mut() {
            let step=counter.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,counter.terminal_is_empty(),"original probe completion counter")?;
            if counter.terminal_is_empty(){self.counter=None;}
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.depth)}
    fn terminal_is_empty(&self)->bool { self.closing&&self.inner.is_none()&&self.dropped.is_none()&&self.counter.is_none() }
}


#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RetainedTextEdit;

struct RetainedTextEditCursor {
    closing: bool,
}

impl RetainedCloneEdit<RetainedTextSnapshot, RetainedTextMutation> for RetainedTextEdit {
    type Cursor = RetainedTextEditCursor;

    fn preflight(&self, _mutation: &RetainedTextMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        Ok(ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 16_384 })
    }

    fn begin_demand(&self) -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 } }
    fn snapshot_cursor_birth_demand(&self) -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 } }
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError> {
        let progress=self.begin_demand().admit(grant)?;
        Ok((RetainedTextEditCursor { closing: false },progress))
    }
}

impl RetainedCloneEditCursor<RetainedTextSnapshot, RetainedTextMutation> for RetainedTextEditCursor {
    fn foreign_step_presence(&self)->bool{false}
    fn advance(&mut self, _base: RetainedCloneRef<'_, RetainedTextSnapshot>, _post: &mut RetainedTextSnapshot, _mutation: RetainedCloneRef<'_, RetainedTextMutation>, _grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"retained text edit must not run after contiguous-capacity refusal"))
    }

    fn take_inverse(&mut self) -> Option<Vec<RetainedTextMutation>> {
        None
    }

    fn cancel(&mut self) {}

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        Ok(if self.closing && grant.maximum_items != 0 { RetainedCloneStep::Complete(empty) } else { RetainedCloneStep::Progress(empty) })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(1)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}


fn fixture_one_item_grant(row: &serde_json::Value) -> ArtifactStoreOneItemGrant {
    ArtifactStoreOneItemGrant { maximum_items:row["maximumItems"].as_u64().expect("fixture supplied items") as usize,maximum_copy_bytes:row["maximumCopyBytes"].as_u64().expect("fixture supplied copy") as usize,maximum_capacity_bytes:row["maximumCapacityBytes"].as_u64().expect("fixture supplied capacity") as usize,maximum_release_bytes:row["maximumReleaseBytes"].as_u64().expect("fixture supplied release") as usize,maximum_depth:row["maximumDepth"].as_u64().expect("fixture supplied depth") as usize }
}

fn close_retained_clone_preparation_publication<P: Send + Sync + 'static, M: Send + 'static>(publication: &mut ArtifactStoreBatchPublication<P, M>, grant: RetainedCloneGrant) -> usize {
    publication.begin_close();
    for turn in 1..10_000 {
        publication.next_copy_byte_demand().expect("current publication copy frontier");
        publication.next_capacity_byte_demand(grant.maximum_copy_bytes).expect("current publication capacity frontier");
        publication.next_release_byte_demand().expect("current publication release frontier");
        publication.next_depth_demand().expect("current publication depth frontier");
        let step = publication.close_step(grant).expect("retained clone publication consumes the supplied caller policy");
        assert!(step.progress().fits(grant) && step.progress().copied_items <= grant.maximum_items, "one close turn stays inside every independently granted axis");
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(publication.terminal_is_empty());return turn; }
    }
    panic!("retained clone publication did not reach terminal empty under the supplied policy");
}

#[semio_framework_async_macros::async_test]
async fn retained_clone_preparation_store_lifecycle_matches_neutral_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️snapshot-clone/🧪️tests/🧫️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation lifecycle fixture");
    let grant = fixture_one_item_grant(&fixture["grant"]);
    for (index, row) in fixture["cases"].as_array().expect("lifecycle cases").iter().enumerate() {
        let initial = row["initial"].as_i64().expect("initial") as i32;
        let value = row["value"].as_i64().expect("value") as i32;
        let kind = row["kind"].as_str().expect("kind");
        let mode = match kind {
            "rejection" => DemoRetainedCloneEditMode::Reject,
            "fault" => DemoRetainedCloneEditMode::Fault,
            "overBudget" => DemoRetainedCloneEditMode::OverBudget,
            _ => DemoRetainedCloneEditMode::Apply,
        };
        let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", row["id"].as_str().expect("case id"), DemoSnapshot { n: Some(initial) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        let generation = store.generation_now();
        let revision = store.content_revision_now();
        let factory = retained_clone_preparation_factory(mode);
        let admitted = store.begin_apply_batch(
            semio_framework_job::OperationId(index as u64 + 1),
            generation,
            revision,
            store.local_actor_id().0.clone(),
            vec![DemoMutation::SetN(SetN { n: value })],
            HistoryLane::Document,
            Some(&factory),
            None,
        );
        let mut published = false;
        let mut terminal_empty = true;
        if kind == "rejection" {
            if let Ok(mut publication) = admitted {
                let mut refused = false;
                for _ in 0..262_144 {
                    match store.advance_apply_batch(&mut publication, grant) {
                        Err(_) | Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => { refused = true; break; }
                        Ok(ArtifactStoreOneItemAdvance::Published(_)) => panic!("{} published a rejected gesture", row["id"]),
                        Ok(_) => {}
                    }
                }
                assert!(refused, "{} rejects before a SnapshotRead owner is transferred", row["id"]);
                publication.begin_close();
                close_retained_clone_preparation_publication(&mut publication, grant.retained_grant());
                terminal_empty = publication.terminal_is_empty();
            }
        } else {
            let mut publication = admitted.unwrap_or_else(|rejected| panic!("{} admission: {}", row["id"], rejected.reason));
            let interrupt = row["interruptAfterTurns"].as_u64().expect("interrupt turns") as usize;
            for _ in 0..interrupt {
                match store.advance_apply_batch(&mut publication, grant) {
                    Ok(ArtifactStoreOneItemAdvance::Published(_)) => {
                        published = true;
                        break;
                    }
                    Ok(ArtifactStoreOneItemAdvance::PreparationRefused) if kind == "fault" => {
                        assert!(publication.fault().expect("a refused preparation retains its fault").to_string().contains("fixture edit fault"));
                        break;
                    }
                    Ok(_) => {}
                    Err(error) if kind == "fault" => {
                        assert!(error.to_string().contains("fixture edit fault"));
                        break;
                    }
                    Err(error) => panic!("{} interrupted prefix: {error}", row["id"]),
                }
            }
            match kind {
                "success" => {
                    let mut last = None;
                    for _ in 0..262_144 {
                        let advance = store.advance_apply_batch(&mut publication, grant).expect("retained clone success turn");
                        last = Some(advance);
                        match advance {
                            ArtifactStoreOneItemAdvance::Published(_) => {
                                published = true;
                                break;
                            }
                            ArtifactStoreOneItemAdvance::PreparationProgress(_, progress) => assert!(progress.fits(grant.retained_grant())),
                            ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::Blocked => {}
                            ArtifactStoreOneItemAdvance::AwaitingAck(_) | ArtifactStoreOneItemAdvance::Complete | ArtifactStoreOneItemAdvance::PreparationRefused => panic!("success reached a terminal state before publishing"),
                        }
                    }
                    assert!(published, "success publishes within its declared retained turn envelope; last advance {last:?}, grant {grant:?}, phase {:?}", publication.phase());
                    assert!(publication.acknowledge());
                }
                "cancel" => assert!(store.cancel_apply_batch(&mut publication)),
                "stale" => {
                    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: initial + 1 })], transaction: None }).await.expect("superseding edit publishes");
                    let error = match store.advance_apply_batch(&mut publication, grant) {
                        Err(error) => error,
                        Ok(_) => panic!("retained clone publication must reject its stale base"),
                    };
                    assert!(error.to_string().contains("stale"));
                }
                "fault" => {
                    let mut faulted = publication.fault().is_some();
                    for _ in 0..262_144 {
                        match store.advance_apply_batch(&mut publication, grant) {
                            Err(error) => {
                                assert!(error.to_string().contains("fixture edit fault"));
                                faulted = true;
                                break;
                            }
                            Ok(ArtifactStoreOneItemAdvance::Published(_)) => panic!("faulting edit published"),
                            Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => {
                                assert!(publication.fault().expect("a refused preparation retains its fault").to_string().contains("fixture edit fault"));
                                faulted = true;
                                break;
                            }
                            Ok(_) => {}
                        }
                    }
                    assert!(faulted, "fault case reaches its injected retained edit fault");
                    publication.begin_close();
                }
                "overBudget" => {
                    let mut refusal = None;
                    for _ in 0..262_144 {
                        match store.advance_apply_batch(&mut publication, grant) {
                            Err(error) => {
                                refusal = Some(error);
                                break;
                            }
                            Ok(ArtifactStoreOneItemAdvance::Published(_)) => panic!("over-budget retained edit published"),
                            Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => {
                                refusal = Some(VcsError::ValidationFailed(publication.fault().expect("a refused preparation retains its fault").to_string()));
                                break;
                            }
                            Ok(_) => {}
                        }
                    }
                    let error = refusal.expect("over-budget retained edit is refused before publication");
                    assert!(error.to_string().contains("exceeded its item, copy, capacity, or release grant"), "over-budget refusal text: {error}");
                    publication.begin_close();
                }
                _ => {}
            }
            let close_turns = close_retained_clone_preparation_publication(&mut publication, grant.retained_grant());
            assert!(close_turns > 1, "the retained owner lifecycle remains interruptible through terminal close");
            terminal_empty = publication.terminal_is_empty();
        }
        let actual = serde_json::json!({
            "published": published,
            "value": store.snapshot_ref().n,
            "history": store.applied_edit_ids().len(),
            "terminalEmpty": terminal_empty,
        });
        assert_eq!(actual, row["expected"], "{} matches the language-neutral lifecycle oracle", row["id"]);
        if kind == "success" {
            test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("retained clone publication undo");
            assert_eq!(store.snapshot_ref().n, Some(initial));
            test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("retained clone publication redo");
            assert_eq!(store.snapshot_ref().n, Some(value));
        }
        close_demo_artifact_store(&mut store);
    }
}

#[semio_framework_async_macros::async_test]
async fn interrupted_retained_clone_publication_handoffs_to_store_maintenance() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️snapshot-clone/🧪️tests/🧫️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation lifecycle fixture");
    let row = &fixture["handoff"];
    let grant = fixture_one_item_grant(&row["closeGrant"]);
    let dropped = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> =
        Arc::new(ProbedRetainedClonePreparationFactory { inner: retained_clone_preparation_factory(DemoRetainedCloneEditMode::Apply), dropped: Arc::clone(&dropped) });
    let initial = DemoSnapshot { n: Some(31) };
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-driver-loss", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let mut publication = Some(
        store
            .begin_apply_batch(
                semio_framework_job::OperationId(71),
                store.generation_now(),
                store.content_revision_now(),
                "retained-driver-loss".into(),
                vec![DemoMutation::SetN(SetN { n: 32 })],
                HistoryLane::Document,
                Some(&factory),
                None,
            )
            .expect("retained clone publication is admitted"),
    );
    for _ in 0..65_536 {
        let driver = publication.as_mut().expect("live publication driver");
        assert!(matches!(store.advance_apply_batch(driver, grant), Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..))));
        if driver.preparation.is_some() { break; }
    }
    assert!(publication.as_ref().is_some_and(|driver| driver.preparation.is_some()), "the driver is interrupted while it owns the retained preparation cursor");
    assert!(store.handoff_batch_publication_close(&mut publication).expect("Store accepts publication close handoff"));
    assert!(publication.is_none(), "the interrupted driver retains no cursor owner after handoff");
    assert_eq!(dropped.load(std::sync::atomic::Ordering::SeqCst), 0, "the retained preparation remains unfinished after driver loss inside Store maintenance");
    let mut close_turns = 0usize;
    loop {
        close_turns += 1;
        let demand = store.maintenance_retirements_demands(grant.maximum_copy_bytes).expect("Store maintenance quotes the handed-off publication frontier");
        let turn = grant.retained_grant();
        assert!(demand.copy_bytes <= turn.maximum_copy_bytes && demand.capacity_bytes <= turn.maximum_capacity_bytes && demand.release_bytes <= turn.maximum_release_bytes && demand.depth <= turn.maximum_depth);
        let step = store.maintenance_retirements_step(turn).expect("Store maintenance retires the handed-off publication");
        assert!(step.progress().fits(turn) && step.progress().copied_items <= grant.maximum_items);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        assert!(close_turns < 4_096, "Store maintenance reaches terminal emptiness");
    }
    let actual = serde_json::json!({
        "driverOwnsCursor": publication.is_none() && dropped.load(std::sync::atomic::Ordering::SeqCst) == 1,
        "sourcePreserved": store.snapshot_ref() == &initial && store.applied_edit_ids().is_empty(),
        "closeRequiresMultipleTurns": close_turns > 1,
        "terminalEmpty": store.maintenance_retirements_terminal_is_empty(),
    });
    assert_eq!(actual, row["expected"], "Store driver-loss handoff matches the language-neutral oracle");
    close_demo_artifact_store(&mut store);
}

#[semio_framework_async_macros::async_test]
async fn retained_clone_preparation_refuses_contiguous_capacity_larger_than_one_store_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️snapshot-clone/🧪️tests/🧫️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation lifecycle fixture");
    let grant = fixture_one_item_grant(&fixture["grant"]);
    let text_bytes = fixture["largeCapacity"]["stringByteLength"].as_u64().expect("large string byte length") as usize;
    let expected_code = fixture["largeCapacity"]["expectedCode"].as_str().expect("large capacity refusal code");
    let initial = RetainedTextSnapshot { text: "x".repeat(text_bytes) };
    let mut store = ArtifactStore::bare(create_document_envelope::<RetainedTextSnapshot, RetainedTextMutation>("retained.text/v1", "retained-text-capacity", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(complete_test_catalog(DocumentStoreOwners::admit_source_constructor(physical_test_close_grant(), || (semio_framework_value::factory::SharedValueRetirementFactory::<RetainedTextSnapshot>::default(), semio_framework_value::factory::OwnedValueRetirementFactory::<RetainedTextSnapshot>::default(), semio_framework_value::factory::OwnedValueRetirementFactory::<RetainedTextMutation>::default(), ArtifactStoreCursorDisposer::<RetainedTextSnapshot, RetainedTextMutation>::new())))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<RetainedTextSnapshot, RetainedTextMutation>> = Arc::new(
        RetainedClonePreparationFactory::new(
            Arc::new(RetainedTextEdit),
            Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<RetainedTextMutation>::default()),
            Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<RetainedTextSnapshot>::default()),
            64,
        )
        .expect("retained text preparation factory"),
    );
    let mut publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(41),
            store.generation_now(),
            store.content_revision_now(),
            "retained-text-capacity".into(),
            vec![RetainedTextMutation::SetRetainedText(SetRetainedText { text: "y".into() })],
            HistoryLane::Document,
            Some(&factory),
            None,
        )
        .expect("the operation-wide footprint is admitted before per-turn contiguous allocation");
    let mut refusal = None;
    let mut last = None;
    for _ in 0..65_536 {
        let advance = store.advance_apply_batch(&mut publication, grant);
        last = Some(format!("{advance:?}"));
        match advance {
            Err(error) => {
                refusal = Some(error);
                break;
            }
            Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => {
                refusal = Some(VcsError::ValidationFailed(publication.fault().expect("a refused preparation retains its fault").to_string()));
                break;
            }
            Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..) | ArtifactStoreOneItemAdvance::Blocked) => {}
            Ok(_) => panic!("oversized contiguous owner reached an invalid state before refusal"),
        }
    }
    let error = refusal.unwrap_or_else(|| panic!("a contiguous string larger than the turn capacity grant cannot be allocated; last advance {last:?}, grant {grant:?}"));
    assert!(error.to_string().contains(expected_code), "capacity refusal exposes the stable language-neutral code: {error}");
    assert_eq!(store.snapshot_ref(), &initial);
    assert!(store.applied_edit_ids().is_empty());
    assert!(close_retained_clone_preparation_publication(&mut publication, grant.retained_grant()) > 1);
    close_test_store(&mut store);
    assert!(SpaceMember::close_owned_terminal_is_empty(&store));
}

/// 🎟️ Supplies the immutable plain test cleanup policy independently of the owner's frontier.
fn ephemeral_test_close_grant() -> ArtifactStoreOneItemGrant {
    let grant = physical_test_close_grant();
    ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items, maximum_copy_bytes: grant.maximum_copy_bytes, maximum_capacity_bytes: grant.maximum_capacity_bytes, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth }
}

fn close_ephemeral_publication(publication: &mut ArtifactEphemeralOneItemPublication<DemoSnapshot, DemoMutation>) {
    for _ in 0..64 {
        let grant = ephemeral_test_close_grant();
        publication.retirement_demands(grant.maximum_copy_bytes).expect("actual cleanup frontier is fallible, not grant authority");
        let step = publication.close_step(grant).expect("ephemeral publication closes");
        assert!(step.progress().fits(grant.retained_grant()));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(publication.terminal_is_empty());
            return;
        }
    }
    panic!("ephemeral publication did not reach terminal empty");
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_single_retry_ack_and_move_only_root_preserve_generation_revision_and_history() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-single", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let factory = Arc::new(DemoOneItemPreparationFactory::admissible());
    let admitted: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factory.clone();
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(1), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 7 })], HistoryLane::Document, Some(&admitted), None)
        .expect("explicit domain factory admits");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: 4096,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };
    let receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = store.advance_apply_batch(&mut publication, grant).expect("one bounded durable step") {
            break receipt;
        }
    };
    assert_eq!(receipt, LaneItemReceipt { generation_before: generation, generation_after: generation + 1 });
    assert_eq!(store.snapshot().expect("published snapshot"), DemoSnapshot { n: Some(7) });
    assert_eq!(store.applied_edit_ids().len(), 1);
    assert_ne!(store.content_revision_now(), revision);
    let prepared_root = factory.published_root.lock().expect("root witness lock").as_ref().and_then(std::sync::Weak::upgrade).expect("published root remains owned");
    assert!(Arc::ptr_eq(&prepared_root, &store.snapshot_root()), "commit moves the exact prepared Arc root");
    assert!(publication.retry());
    assert!(publication.retry());
    assert!(!publication.retry());
    assert_eq!(store.generation_now(), generation + 1, "retry never republishes");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}


/// 🧺️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B: drives one batched publication to its receipt
/// under a bounded per-turn grant, exactly as a host actor tick drives it.
async fn publish_demo_batch(
    store: &mut ArtifactStore<DemoSnapshot, DemoMutation>,
    operation: u64,
    mutations: Vec<DemoMutation>,
) -> (ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation>, Result<LaneItemReceipt, VcsError>) {
    publish_demo_batch_with(store, operation, mutations, DemoOneItemPreparationFactory::admissible()).await
}

/// 🧺️ The same drive against an explicitly chosen app preflight, so a footprint declaration can be
/// varied without touching any other admission gate.
async fn publish_demo_batch_with(
    store: &mut ArtifactStore<DemoSnapshot, DemoMutation>,
    operation: u64,
    mutations: Vec<DemoMutation>,
    app_factory: DemoOneItemPreparationFactory,
) -> (ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation>, Result<LaneItemReceipt, VcsError>) {
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(app_factory);
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(operation), store.generation_now(), store.content_revision_now(), store.local_actor_id().0.clone(), mutations, HistoryLane::Document, Some(&factory), None)
        .unwrap_or_else(|rejected| panic!("batched admission: {}", rejected.reason));
    let grant = batch_turn_grant();
    for _ in 0..1_048_576 {
        match store.advance_apply_batch(&mut publication, grant) {
            Ok(ArtifactStoreOneItemAdvance::Published(receipt)) => return (publication, Ok(receipt)),
            Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => {
                let reason = publication.fault().unwrap_or("batched preparation refused").to_string();
                return (publication, Err(VcsError::ValidationFailed(reason)));
            }
            Ok(_) => {}
            Err(error) => return (publication, Err(error)),
        }
    }
    panic!("batched publication never reached its receipt inside its bounded turn budget: phase={:?}, checkpoint={:?}, fault={:?}", publication.phase(), publication.progress(), publication.fault());
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_reset_preserves_capacity_for_retained_batch_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️reset-publication/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let initial = row["initial"].as_i64().unwrap() as i32;
        let after = row["after"].as_i64().unwrap() as i32;
        let fresh = || create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "reset-publication", DemoSnapshot { n: Some(initial) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = ArtifactStore::bare(fresh()).await;
        store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        store.reset(fresh()).await.expect("empty-history reset");
        let (mut publication, receipt) = publish_demo_batch(&mut store, 41, vec![DemoMutation::SetN(SetN { n: after })]).await;
        assert!(publication.acknowledge() || receipt.is_err());
        publication.begin_close();
        close_durable_publication(&mut publication);
        let outcome = receipt.map(|_| serde_json::to_value(store.snapshot_ref().n).unwrap());
        close_demo_artifact_store(&mut store);
        assert_eq!(outcome.unwrap(), row["after"]);
    }
}

/// 🧺️ ONE gesture of 200 mutations is ONE `Edit` in ONE ledger slot and ONE undo step — the
/// `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` ceiling that used to fault a `setActiveExample` load at
/// its 65th mutation is structurally out of reach, and the staged edit is byte-for-byte the edit
/// `ArtifactCommand::Apply` records for the same mutation list.
///
/// 🧮️ `SetN`, not `AddN`: `AddN` emits one info message per operation and 200 of them exceed the
/// edit-message byte authority `ArtifactCommand::Apply` records — a limit of the message lane, not
/// of the staged edit, and the oracle below must apply the very same list.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step() {
    const ITEMS: usize = 200;
    const { assert!(ITEMS > semio_framework_os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY, "the fixture must exceed the fixed edit ledger capacity to prove the ceiling is gone") };
    let mutations = (0..ITEMS).map(|index| DemoMutation::SetN(SetN { n: index as i32 + 1 })).collect::<Vec<_>>();
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-gesture", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let (mut publication, receipt) = publish_demo_batch(&mut store, 1, mutations.clone()).await;
    let receipt = receipt.expect("a two-hundred item gesture publishes once");
    assert_eq!(receipt, LaneItemReceipt { generation_before: generation, generation_after: generation + 1 });
    assert_eq!(publication.admitted_items(), ITEMS);
    assert_eq!(store.snapshot_ref().n, Some(ITEMS as i32), "every admitted mutation folded against the running post root");
    assert_eq!(store.applied_edit_ids().len(), 1, "one gesture consumes exactly one applied-edit slot");
    assert_eq!(store.log.envelope.vcs.edits.len(), 1, "one gesture consumes exactly one history ledger slot");
    let staged = store.log.envelope.vcs.edits.last().expect("staged gesture edit").clone();
    assert_eq!(staged.forwards.len(), ITEMS);
    assert_eq!(staged.mutation_meta.len(), ITEMS);
    assert_eq!(staged.inverse.len(), ITEMS);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let mut oracle = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-oracle", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    oracle.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    test_support::dispatch_test_command(&mut oracle, ArtifactCommand::Apply { mutations, transaction: None }).await.expect("the batched command oracle applies the same list");
    let expected = oracle.envelope.vcs.edits.last().expect("oracle edit");
    assert_eq!(staged.forwards, expected.forwards, "a staged gesture records the same forwards ArtifactCommand::Apply does");
    assert_eq!(staged.inverse, expected.inverse, "a staged gesture records the same inverse ordering ArtifactCommand::Apply does");
    assert_eq!(oracle.snapshot_ref().n, store.snapshot_ref().n);
    close_demo_artifact_store(&mut oracle);

    let post = store.snapshot_ref().clone();
    let mut backwards = post.clone();
    for operation in staged.inverse.iter().rev() {
        backwards = semio_framework_os_spr::apply_diff(operation.diff(&backwards).diff(), &backwards).expect("staged inverse applies tail-first");
    }
    assert_eq!(backwards.n, Some(0), "the staged inverse is applied in reverse order of the forwards");
    let mut forwards = post;
    for operation in &staged.inverse {
        forwards = semio_framework_os_spr::apply_diff(operation.diff(&forwards).diff(), &forwards).expect("staged inverse applies");
    }
    assert_ne!(forwards.n, Some(0), "consuming the staged inverse head-first is NOT the undo of the gesture");

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("one gesture is one undo step");
    assert_eq!(store.snapshot_ref().n, Some(0), "one undo reverts the whole gesture");
    assert!(store.applied_edit_ids().is_empty());
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("one gesture is one redo step");
    assert_eq!(store.snapshot_ref().n, Some(ITEMS as i32));
    close_demo_artifact_store(&mut store);
}

/// 🧺️ The single-mutation publication is literally the `N = 1` case of the same machine: same
/// phases, same receipt, one ledger slot — and the same identity rule as `ArtifactCommand::Apply` (audit W1G-8): its one
/// mutation is named after its edit, so a remote replica's one-forward edit resolves every transition naming it.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_publication_of_one_mutation_is_the_single_item_case() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-single", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let (mut publication, receipt) = publish_demo_batch(&mut store, 2, vec![DemoMutation::SetN(SetN { n: 7 })]).await;
    receipt.expect("a single-item gesture publishes");
    assert_eq!(publication.admitted_items(), 1);
    assert_eq!(store.snapshot_ref().n, Some(7));
    let staged = store.log.envelope.vcs.edits.last().expect("staged edit");
    assert_eq!(staged.forwards, vec![DemoMutation::SetN(SetN { n: 7 })]);
    assert_eq!(staged.inverse.iter().cloned().collect::<Vec<_>>(), vec![DemoMutation::AssignN(AssignN { n: Some(0) })]);
    assert_eq!(staged.mutation_meta[0].mutation_id, Some(MutationId(staged.id.clone())), "a one-mutation gesture names its mutation after its edit on the batched route as on every other");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    let (mut pair, receipt) = publish_demo_batch(&mut store, 3, vec![DemoMutation::SetN(SetN { n: 8 }), DemoMutation::SetN(SetN { n: 9 })]).await;
    receipt.expect("a two-item gesture publishes");
    let staged = store.log.envelope.vcs.edits.last().expect("staged two-item edit");
    assert_eq!(staged.mutation_meta.iter().map(|meta| meta.mutation_id.clone()).collect::<Vec<_>>(), vec![Some(MutationId(format!("{}#0", staged.id))), Some(MutationId(format!("{}#1", staged.id)))], "a genuine multi-mutation gesture keeps its positional ids");
    assert!(pair.acknowledge());
    close_durable_publication(&mut pair);
    close_demo_artifact_store(&mut store);
}

/// 🧺️ The fold contract, stated from both sides — the law the whole procedural 3d app boot broke.
/// `ArtifactStoreOneItemFootprint::work_items` counts staged edit ROWS (one forward plus every
/// inverse row), so a durable preflight that declares ONE work item for a point-invertible mutation
/// fail-closes its own single-item gesture inside `fold_batch_item`, with the exact message the
/// runtime reported; the very same gesture through
/// [`ArtifactStoreOneItemFootprint::for_leaf`] publishes and stages both rows. The
/// store's contract is therefore exactly what its own fixtures declare — the `work_items: 1` that ~40
/// app preflights carried was the defect (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_fold_refuses_a_one_work_item_declaration_and_accepts_the_invertible_one() {
    assert_eq!(ArtifactStoreOneItemFootprint::for_leaf(&DemoMutation::SetN(SetN { n: 1 }), 512), ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 512 });
    assert_eq!(ArtifactStoreOneItemFootprint::for_ephemeral_item(512), ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 512 });

    let mut under = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "fold-under-declared", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    under.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = under.generation_now();
    let (mut publication, receipt) = publish_demo_batch_with(&mut under, 7, vec![DemoMutation::SetN(SetN { n: 9 })], DemoOneItemPreparationFactory::under_declared()).await;
    assert_eq!(receipt.expect_err("a one-work-item declaration cannot carry a point-invertible item"), VcsError::TooLarge { rows: 2, capacity: 1 });
    assert_eq!(under.generation_now(), generation, "a refused fold publishes nothing");
    assert_eq!(under.snapshot_ref().n, Some(0));
    assert!(under.applied_edit_ids().is_empty());
    publication.begin_close();
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut under);

    let mut exact = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "fold-invertible", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    exact.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let (mut publication, receipt) = publish_demo_batch_with(&mut exact, 8, vec![DemoMutation::SetN(SetN { n: 9 })], DemoOneItemPreparationFactory::admissible()).await;
    receipt.expect("the invertible declaration carries the same item");
    assert_eq!(exact.snapshot_ref().n, Some(9));
    let staged = exact.envelope.vcs.edits.last().expect("staged edit");
    assert_eq!(staged.forwards.len() + staged.inverse.len(), ArtifactStoreOneItemFootprint::for_leaf(&DemoMutation::SetN(SetN { n: 0 }), 0).work_items, "the exact declaration is the rows the item actually folds");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut exact);
}

/// 🧺️ The SAME law for the multi-item gesture, and the reason the defect stayed invisible until a
/// one-mutation command ran it: `fold_batch_item` compares each candidate against the GESTURE-WIDE
/// merged declaration, so `N` under-declared items still fold (2 rows ≤ `N` for every `N ≥ 2`) and
/// only the `PreflightingCommit` gate catches the overrun — with the sibling message
/// `batched prepared candidate failed its exact fixed commit contract`, `2N` staged rows against a
/// declared `N`. The single-item case is the one with zero slack, which is why the boot-time
/// `setActiveExample` was the first thing to die. Both gates pass exactly when every item declares
/// through [`ArtifactStoreOneItemFootprint::for_leaf`]
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_commit_refuses_an_under_declared_multi_item_gesture_and_accepts_the_invertible_one() {
    const ITEMS: usize = 3;
    let mutations = || (0..ITEMS).map(|index| DemoMutation::SetN(SetN { n: index as i32 + 1 })).collect::<Vec<_>>();
    assert_eq!(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 512 }.merged(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 512 }).work_items, 2, "a merged declaration sums the items' rows");

    let mut under = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "fold-batch-under-declared", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    under.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = under.generation_now();
    let (mut publication, receipt) = publish_demo_batch_with(&mut under, 9, mutations(), DemoOneItemPreparationFactory::under_declared()).await;
    assert_eq!(
        receipt.expect_err("an under-declared multi-item gesture cannot commit its inverse rows"),
        VcsError::TooLarge { rows: 6, capacity: 3 },
        "the per-item fold admits every candidate at N >= 2; the gesture-wide commit gate is what refuses"
    );
    assert_eq!(under.generation_now(), generation, "a refused commit publishes nothing");
    assert_eq!(under.snapshot_ref().n, Some(0));
    assert!(under.applied_edit_ids().is_empty());
    assert!(under.envelope.vcs.edits.is_empty(), "a refused gesture consumes no ledger slot");
    publication.begin_close();
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut under);

    let mut exact = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "fold-batch-invertible", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    exact.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let (mut publication, receipt) = publish_demo_batch_with(&mut exact, 10, mutations(), DemoOneItemPreparationFactory::admissible()).await;
    receipt.expect("the invertible declaration carries the same gesture");
    assert_eq!(exact.snapshot_ref().n, Some(ITEMS as i32));
    let staged = exact.envelope.vcs.edits.last().expect("staged gesture edit");
    assert_eq!(staged.forwards.len(), ITEMS);
    assert_eq!(staged.inverse.len(), ITEMS);
    assert_eq!(staged.forwards.len() + staged.inverse.len(), ArtifactStoreOneItemFootprint::for_leaf(&DemoMutation::SetN(SetN { n: 0 }), 0).work_items * ITEMS, "the merged invertible declaration is exactly the rows the gesture folds");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut exact);
}

/// 🧺️ A gesture whose fourth mutation cannot prepare against the running post root commits
/// NOTHING: no ledger slot, no applied edit, no root replacement, no generation bump — and the
/// half-staged edit still closes to its exact terminal-empty witness.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_rejection_mid_batch_leaves_no_partial_edit_and_still_retires_exactly() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-rejection", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let mutations = vec![DemoMutation::SetN(SetN { n: 1 }), DemoMutation::AddN(AddN { delta: 2 }), DemoMutation::DeleteN(DeleteN {}), DemoMutation::SetN(SetN { n: 5 })];
    let (mut publication, outcome) = publish_demo_batch(&mut store, 3, mutations).await;
    let error = outcome.expect_err("the fourth mutation cannot prepare against a deleted target");
    assert!(error.to_string().contains("demo retained mutation rejected against its base"), "{error}");
    assert!(publication.staged_items() < publication.admitted_items(), "the batch faulted before every item was staged");
    assert_eq!(store.generation_now(), generation, "a rejected gesture never bumps the generation");
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()), "a rejected gesture never replaces the root");
    assert!(store.applied_edit_ids().is_empty(), "a rejected gesture leaves no partial applied edit");
    assert!(store.log.envelope.vcs.edits.is_empty(), "a rejected gesture consumes no history ledger slot");
    publication.begin_close();
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}

/// 🧺️ Cancelling a gesture mid-flight retires the already-staged forwards, inverses, metadata and
/// running post root one owner per bounded turn, and never publishes.
#[semio_framework_async_macros::async_test]
async fn artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_without_publishing() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "batched-cancel", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let root = store.snapshot_root();
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(4), generation, store.content_revision_now(), "retained-test".into(), (0..32).map(|_| DemoMutation::AddN(AddN { delta: 1 })).collect(), HistoryLane::Document, Some(&factory), None)
        .expect("a thirty-two item gesture admits");
    let grant = batch_turn_grant();
    while publication.staged_items() < 3 {
        store.advance_apply_batch(&mut publication, grant).expect("bounded staging turn");
    }
    assert!(store.cancel_apply_batch(&mut publication));
    assert!(matches!(store.advance_apply_batch(&mut publication, grant), Ok(ArtifactStoreOneItemAdvance::Blocked)));
    close_durable_publication(&mut publication);
    assert_eq!(store.generation_now(), generation, "a cancelled gesture never publishes");
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()));
    assert!(store.log.envelope.vcs.edits.is_empty());
    close_demo_artifact_store(&mut store);
}


#[semio_framework_async_macros::async_test]
async fn retained_latest_wins_cold_rebase_preserves_admitted_cursor_capacity_for_next_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🔌️plugin/🧪️tests/🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "cold-retained-rebase", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let capacity = store.log.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 13 })], transaction: None }).await.unwrap();
    assert_eq!(store.log.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity(), capacity);
    assert_eq!(serde_json::json!(store.generation_now()), fixture["rebase"]["afterGeneration"]);
    let revision = store.content_revision_now();
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut publication = store.begin_apply_batch(semio_framework_job::OperationId(93), store.generation_now(), revision, "fixture".into(), vec![DemoMutation::SetN(SetN { n: 97 })], HistoryLane::Document, Some(&factory), None).unwrap();
    let grant = fixture_one_item_grant(&fixture);
    for _ in 0..16_384 {
        if matches!(store.advance_apply_batch(&mut publication, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)) {
            break;
        }
    }
    assert_eq!(serde_json::to_value(store.snapshot_root().as_ref()).unwrap(), serde_json::json!({ "n": 97 }));
    assert_eq!(store.generation_now(), 2);
    assert_eq!(store.log.envelope.cursor.as_ref().unwrap().applied_edit_ids.capacity(), capacity);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
    close_demo_artifact_store(&mut store);
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_digest_helper_matches_validation_and_rejects_forged_cursor_history() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-digest-authority", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let forged: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::forged_digest());
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(11), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 12 })], HistoryLane::Document, Some(&forged), None)
        .expect("Store-minted immutable authority admits the domain owner");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: 4096,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };
    while publication.preparation.as_ref().and_then(|owner| owner.prepared()).is_none() {
        assert!(matches!(store.advance_apply_batch(&mut publication, grant), Ok(ArtifactStoreOneItemAdvance::Progress(_) | ArtifactStoreOneItemAdvance::PreparationProgress(..))));
    }
    let authority = publication.authority.as_ref().expect("publication retains Store authority");
    let prepared = publication.preparation.as_ref().and_then(|owner| owner.prepared()).expect("domain prepared candidate");
    assert_eq!(authority.operation(), semio_framework_job::OperationId(11));
    assert_eq!(authority.generation(), semio_framework_job::Generation(generation));
    assert_eq!(authority.base_revision(), revision);
    assert_eq!(authority.base_applied_edit_count(), 0);
    let canonical = authority.prepared_edit_digest(&prepared.edit()).expect("Store helper accepts its exact semantic edit");
    assert_eq!(canonical, CursorRevisionAccumulator::edit_digest(&prepared.edit()), "public helper and private Store validation share one canonical digest law");
    assert_ne!(prepared.edit_digest(), canonical, "hostile domain altered only its exposed candidate digest");
    let mut rejected = false;
    for _ in 0..65_536 {
        match store.advance_apply_batch(&mut publication, grant) {
            Err(_) | Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => { rejected = true; break; }
            Ok(ArtifactStoreOneItemAdvance::Published(_)) => panic!("a domain-forged digest published"),
            Ok(_) => {}
        }
    }
    assert!(rejected, "Store recomputation rejects a domain-forged digest before publication");
    publication.begin_close();
    close_durable_publication(&mut publication);
    assert_eq!(store.generation_now(), generation);
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()), "forged cursor/history authority cannot replace the root");
    close_demo_artifact_store(&mut store);
}

#[semio_framework_async_macros::async_test]
async fn artifact_store_one_item_stale_saturation_and_cancel_leave_root_generation_and_revision_unchanged() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-rejections", DemoSnapshot { n: Some(3) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let root = store.snapshot_root();
    let admissible: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let stale = store.begin_apply_batch(semio_framework_job::OperationId(2), generation + 1, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 9 })], HistoryLane::Document, Some(&admissible), None);
    assert!(stale.is_err());

    let mut cancelled =
        store.begin_apply_batch(semio_framework_job::OperationId(3), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 8 })], HistoryLane::Document, Some(&admissible), None).expect("fresh publication admits");
    assert!(store.cancel_apply_batch(&mut cancelled));
    close_durable_publication(&mut cancelled);
    assert_eq!(store.generation_now(), generation);
    assert_eq!(store.content_revision_now(), revision);
    assert!(Arc::ptr_eq(&root, &store.snapshot_root()));

    for n in 1..=65 {
        test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n })], transaction: None }).await.expect("an edit past one history page");
    }
    assert_eq!(store.applied_edit_ids().len(), 65);
    assert_eq!(serde_json::to_value(store.snapshot_root().as_ref()).unwrap(), serde_json::json!({ "n": 65 }));
    assert_ne!(store.generation_now(), generation);
    close_demo_artifact_store(&mut store);
}

/// 🕰️ ticket 26/09/18 HT3a: a durable committer stamps a server-derived HLC on the edit it will
/// hash into its canonical command, so the Store must mint the publication authority at that exact
/// tick — the semantic-edit contract stays an exact equality and a stamp that is not strictly ahead
/// of the Store's own clock is refused before any owner is reserved.
#[semio_framework_async_macros::async_test]
async fn artifact_store_stamped_publication_clock_is_admitted_ahead_and_refused_behind() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "stamped-clock", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let revision = store.content_revision_now();
    let stamp = HybridLogicalTimestamp { actor: 7, physical_ms: store.log.clock.physical_ms + 1_000, logical: 0 };
    let ahead: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(stamp));
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(11), generation, revision, "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 5 })], HistoryLane::Document, Some(&ahead), None)
        .expect("a stamped clock strictly ahead of the Store clock admits");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: 4096,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };
    let receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = store.advance_apply_batch(&mut publication, grant).expect("one bounded stamped step") {
            break receipt;
        }
    };
    assert_eq!(receipt, LaneItemReceipt { generation_before: generation, generation_after: generation + 1 });
    assert_eq!(store.log.clock, stamp, "the committed Store clock is the stamp the committer hashed into its command");
    let published = store.log.envelope.vcs.edits.last().expect("one published stamped edit");
    assert_eq!(published.mutation_meta.first().expect("stamped authority metadata").timestamp, stamp);
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let behind: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(stamp));
    let refused = store
        .begin_apply_batch(semio_framework_job::OperationId(12), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 6 })], HistoryLane::Document, Some(&behind), None)
        .err()
        .expect("a stamp at or behind the Store clock is refused");
    assert_eq!(refused.reason, "stamped publication clock is not strictly after this Store's own last published edit");
    assert_eq!(store.generation_now(), generation + 1, "a refused stamp never publishes");
    assert_eq!(store.log.clock, stamp);

    let same_millisecond = HybridLogicalTimestamp { actor: 7, physical_ms: stamp.physical_ms, logical: stamp.logical + 1 };
    let tied: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(same_millisecond));
    let mut tied_publication = store
        .begin_apply_batch(semio_framework_job::OperationId(13), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 7 })], HistoryLane::Document, Some(&tied), None)
        .expect("a second approval in the same millisecond carries the next logical tick and admits");
    let tied_receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = store.advance_apply_batch(&mut tied_publication, grant).expect("one bounded same-millisecond step") {
            break receipt;
        }
    };
    assert_eq!(tied_receipt, LaneItemReceipt { generation_before: generation + 1, generation_after: generation + 2 });
    assert_eq!(store.log.clock, same_millisecond, "the same-millisecond stamp is the committed clock, so the command hash still rebuilds");
    assert!(tied_publication.acknowledge());
    close_durable_publication(&mut tied_publication);
    close_demo_artifact_store(&mut store);
}

/// 🌱️ ticket 26/09/18 HT4: a Store seeds `clock` from `now_ms()` at construction, which is a fact
/// about the machine and not about the document. A document with no published edit of its own is
/// therefore after nothing, and a server-minted stamp far behind that seed — a hub preparing the
/// first approval on a genesis document — must publish rather than be refused against a wall clock
/// the document never saw.
#[semio_framework_async_macros::async_test]
async fn artifact_store_stamped_publication_clock_admits_a_genesis_document_behind_its_construction_seed() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "stamped-genesis-clock", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let stamp = HybridLogicalTimestamp { actor: 1, physical_ms: 1_004, logical: 0 };
    assert!(store.log.clock.physical_ms > stamp.physical_ms, "the construction seed is a wall-clock tick far ahead of the minted stamp");
    let genesis: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(stamp));
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(21), generation, store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 3 })], HistoryLane::Document, Some(&genesis), None)
        .expect("a genesis document is after nothing, so the minted stamp admits");
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: 4096,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };
    loop {
        if let ArtifactStoreOneItemAdvance::Published(_) = store.advance_apply_batch(&mut publication, grant).expect("one bounded genesis step") {
            break;
        }
    }
    assert_eq!(store.log.clock, stamp, "the committed clock is the stamp, never the construction seed");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let behind: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped(stamp));
    let refused = store
        .begin_apply_batch(semio_framework_job::OperationId(22), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 4 })], HistoryLane::Document, Some(&behind), None)
        .err()
        .expect("once this Store has published an edit, its clock is a document fact and the replay is refused");
    assert_eq!(refused.reason, "stamped publication clock is not strictly after this Store's own last published edit");
    close_demo_artifact_store(&mut store);
}

/// 🔏 ticket 26/09/18 HT5: a durable committer hashes its canonical command over BOTH the stamped
/// clock and the stamped mutation id, then persists that hash. If the Store mints its own
/// content-addressed `edit_id()` for the publication anyway, the committed edit carries an identity
/// no verifier can bind back to the approval it executed — which is exactly what the hub's WAL
/// reported as "committed decision does not bind its approval target — mutation-id" and what the
/// actor frontier reported as `edit-…` where the approval's mutation id was required. So a stamped
/// publication publishes under the stamped identity, as `Edit.id` AND as the folded item's
/// `mutation_id`, while every unstamped gesture keeps the store's content-addressed mint (its one mutation named
/// after it, audit W1G-8).
#[semio_framework_async_macros::async_test]
async fn artifact_store_stamped_publication_carries_its_committed_identity_into_the_published_edit() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "stamped-identity", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let generation = store.generation_now();
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: 4096,maximum_capacity_bytes: 4096,maximum_release_bytes: 262144,maximum_depth: 4096 };

    let approval = "7d6bd4f3246ae47f074ed7a02011c0cf";
    let stamp = HybridLogicalTimestamp { actor: 1, physical_ms: 1_004, logical: 0 };
    let stamped: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::stamped_identity(stamp, approval));
    let mut publication = store
        .begin_apply_batch(semio_framework_job::OperationId(31), generation, store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 5 })], HistoryLane::Document, Some(&stamped), None)
        .expect("a stamped identity admits with its stamped clock");
    loop {
        if let ArtifactStoreOneItemAdvance::Published(_) = store.advance_apply_batch(&mut publication, grant).expect("one bounded stamped-identity step") {
            break;
        }
    }
    let published = store.log.envelope.vcs.edits.last().expect("one published stamped edit");
    assert_eq!(published.id, approval, "the published edit publishes under the identity the durable decision hashed");
    assert_eq!(published.mutation_meta.first().expect("stamped authority metadata").mutation_id.as_ref().map(|mutation| mutation.0.as_str()), Some(approval), "the folded item keeps the approval's mutation id, never the store's `#position` rewrite");
    assert_eq!(store.log.applied_edit_ids.last().map(String::as_str), Some(approval), "the applied cursor names the stamped identity, so the actor frontier's head edit is the approval");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);

    let local: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut gesture = store
        .begin_apply_batch(semio_framework_job::OperationId(32), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![DemoMutation::SetN(SetN { n: 6 })], HistoryLane::Document, Some(&local), None)
        .expect("an unstamped local gesture admits unchanged");
    loop {
        if let ArtifactStoreOneItemAdvance::Published(_) = store.advance_apply_batch(&mut gesture, grant).expect("one bounded unstamped step") {
            break;
        }
    }
    let gestured = store.log.envelope.vcs.edits.last().expect("one published local edit");
    assert_ne!(gestured.id, approval, "an unstamped gesture keeps the identity its own preparation minted, never a stamped one, got {}", gestured.id);
    assert!(gestured.id.starts_with("retained-one-item-"), "the preparation factory mints the unstamped identity, got {}", gestured.id);
    assert_eq!(gestured.mutation_meta.first().expect("local authority metadata").mutation_id.as_ref().map(|mutation| mutation.0.clone()), Some(gestured.id.clone()), "an unstamped one-mutation gesture names its mutation after its edit");
    assert!(gesture.acknowledge());
    close_durable_publication(&mut gesture);
    close_demo_artifact_store(&mut store);
}

struct FaultingEphemeralTask {
    invalid_receipt: bool,
    closing: bool,
}

impl ArtifactEphemeralPreparationTask<DemoSnapshot, DemoMutation> for FaultingEphemeralTask {
    fn advance(&mut self, _: &DemoSnapshot, _: &mut Option<DemoMutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactEphemeralPreparationTaskStep<DemoSnapshot>, ValueError> {
        if !self.invalid_receipt {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"injected construction failure"));
        }
        let root=Arc::new(DemoSnapshot { n: Some(99) });
        let ownership=RetainedCloneProgress{copied_items:1,copied_bytes:grant.maximum_copy_bytes+1,retained_capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<DemoSnapshot>(),released_bytes:0};
        Ok(ArtifactEphemeralPreparationTaskStep::Prepared { root,ownership, checkpoint: ArtifactStoreOneItemCheckpoint { completed_items: 1, completed_bytes: grant.maximum_copy_bytes as u64 + 1, ..Default::default() } })
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
}

impl ErasedSnapshotRetirement for FaultingEphemeralTask {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        assert!(self.closing);
        let empty = RetainedCloneProgress::default();
        Ok(if grant.maximum_items == 0 { RetainedCloneStep::Progress(empty) } else { RetainedCloneStep::Complete(empty) })
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }
    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {
        Ok(0)
    }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(1)
    }
}

/// 🚫️ A refused ephemeral preparation either raises its fault or answers `PreparationRefused` with the fault retained on the publication.
fn ephemeral_refusal(advance: Result<ArtifactStoreOneItemAdvance, String>, retained_fault: Option<&str>) -> String {
    match advance {
        Err(error) => error,
        Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => retained_fault.expect("a refused ephemeral preparation retains its fault").to_string(),
        Ok(other) => panic!("a faulting ephemeral preparation advanced instead of refusing: {other:?}"),
    }
}

#[test]
fn ephemeral_transfer_preparation_faults_close_presence_and_transient_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🫧️preparation-fault/🔣️.json")).unwrap();
    let factory = ArtifactEphemeralTaskPreparationFactory::new(
        |_| Ok(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 64 }),
        |_, mutation| Ok(Box::new(FaultingEphemeralTask { invalid_receipt: matches!(mutation, DemoMutation::SetN(SetN { n: 1 })), closing: false })),
        Arc::new(DemoInitialSnapshotRetirementFactory),
        Arc::new(DemoMutationRetirementFactory),
    );
    for row in fixture["cases"].as_array().unwrap() {
        let mutation = || DemoMutation::SetN(SetN { n: row["mutation"].as_i64().unwrap() as i32 });
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: 7,maximum_capacity_bytes: 7,maximum_release_bytes: 7,maximum_depth: 64 };
        let mut presence = PresenceStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
        let presence_root_factory: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>> = Arc::new(DemoSnapshotRetirementFactory);
        presence.install_local_retirement_factory(presence_root_factory.clone()).unwrap();
        let mut presence_publication = presence.begin_publish_one(semio_framework_job::OperationId(1), 0, mutation(), Some(&factory), Some(presence_root_factory)).unwrap();
        let mut transient = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
        let mut transient_publication = transient.begin_publish_one_leased(semio_framework_job::OperationId(2), 0, mutation(), &factory, Arc::new(DemoInitialSnapshotRetirementFactory)).unwrap();
        let presence_advance = presence.advance_publish_one(&mut presence_publication, grant);
        let presence_error = ephemeral_refusal(presence_advance, presence_publication.fault());
        let transient_advance = transient.advance_publish_one(&mut transient_publication, grant);
        let transient_error = ephemeral_refusal(transient_advance, transient_publication.fault());
        assert_eq!(presence_error, row["fault"].as_str().unwrap());
        assert_eq!(transient_error, presence_error);
        for publication in [&mut presence_publication, &mut transient_publication] {
            assert_eq!(publication.fault(), Some(presence_error.as_str()));
            assert_eq!(format!("{:?}", publication.phase()), fixture["phase"].as_str().unwrap());
            assert_eq!(publication.close_step(ArtifactStoreOneItemGrant { maximum_items: 0,maximum_copy_bytes: 4096,maximum_capacity_bytes: 4096,maximum_release_bytes: 4096,maximum_depth: 64 }).unwrap().progress(), RetainedCloneProgress::default());
            close_ephemeral_publication(publication);
            assert!(publication.terminal_is_empty());
        }
        assert_eq!(serde_json::to_value(presence.local().n).unwrap(), fixture["initial"]);
        assert_eq!(serde_json::to_value(transient.current_root().n).unwrap(), fixture["initial"]);
        let mut retirement = presence.begin_retirement(Arc::new(DemoSnapshot { n: Some(0) }), |value| value.n == Some(0)).ok().unwrap();
        test_support::drive_retirement(&mut retirement).expect("presence retirement reaches its terminal-empty witness under its quoted demands");
        assert!(retirement.terminal_is_empty());
    }
}

/// 📍️ An ephemeral one-item publication's `progress()` is MONOTONE across its whole life. It used to
/// answer `ArtifactStoreOneItemCheckpoint::default()` — all zeros — the moment `close_step` released
/// its preparation owner, so a host polling a gesture saw its progress snap back to 0 between items
/// (measured on `🌊️flow` as `preparation progress changed from 22013 to 0`, ticket 26/09/19
/// `📓️flow.md` §5.4). Every window-transient, presence and transient publication polls through this
/// exact reader, and its batched twin `ArtifactStoreBatchPublication::progress()` already retains its
/// checkpoint the same way.
#[semio_framework_async_macros::async_test]
async fn an_ephemeral_one_item_publication_reports_monotone_progress_across_owner_release() {
    let grant = ephemeral_test_close_grant();
    let factory = DemoEphemeralPreparationFactory::admissible();
    let mut transient = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    let mut publication = transient.begin_publish_one(semio_framework_job::OperationId(41), 0, DemoMutation::SetN(SetN { n: 4 }), Some(&factory), Some(Arc::new(DemoSnapshotRetirementFactory))).expect("ephemeral publication admits");
    let mut seen = publication.progress();
    let mut check = |next: ArtifactStoreOneItemCheckpoint, seen: &mut ArtifactStoreOneItemCheckpoint, phase: &str| {
        assert!(
            next.completed_items >= seen.completed_items && next.completed_bytes >= seen.completed_bytes,
            "ephemeral progress went backwards at {phase}: {:?} items / {:?} bytes after {:?} items / {:?} bytes",
            next.completed_items,
            next.completed_bytes,
            seen.completed_items,
            seen.completed_bytes
        );
        *seen = next;
    };
    loop {
        let advance = transient.advance_publish_one(&mut publication, grant).expect("ephemeral step");
        check(publication.progress(), &mut seen, "advance");
        if matches!(advance, ArtifactStoreOneItemAdvance::Published(_)) {
            break;
        }
    }
    let published = publication.progress();
    assert!(published.completed_items > 0, "a published ephemeral gesture reports the work it actually did, not zero");
    assert!(publication.acknowledge());
    for _ in 0..2048 {
        let step = publication.close_step(ephemeral_test_close_grant()).expect("ephemeral close advances");
        check(publication.progress(), &mut seen, "close");
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert_eq!(publication.progress(), published, "a fully retired ephemeral publication still reports the gesture it completed");
    close_ephemeral_publication(&mut publication);
}

#[semio_framework_async_macros::async_test]
async fn presence_and_transient_one_item_publications_are_retained_stale_safe_cancelable_and_exactly_closed() {
    let grant = ephemeral_test_close_grant();
    let presence_factory = DemoEphemeralPreparationFactory::admissible();
    let mut presence = PresenceStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    let root_factory: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>> = Arc::new(DemoSnapshotRetirementFactory);
    presence.install_local_retirement_factory(root_factory.clone()).unwrap();
    let old_presence_root = Arc::downgrade(presence.local.as_ref().expect("live local presence owner"));
    let mut presence_publication = presence.begin_publish_one(semio_framework_job::OperationId(5), 0, DemoMutation::SetN(SetN { n: 5 }), Some(&presence_factory), Some(root_factory.clone())).expect("presence factory admits");
    let presence_receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = presence.advance_publish_one(&mut presence_publication, grant).expect("presence step") {
            break receipt;
        }
    };
    assert_eq!(presence_receipt, LaneItemReceipt { generation_before: 0, generation_after: 1 });
    assert_eq!(presence.local().n, Some(5));
    let presence_root = presence_factory.published_root.lock().expect("presence witness").as_ref().and_then(std::sync::Weak::upgrade).expect("presence root");
    assert!(std::ptr::eq(presence_root.as_ref(), presence.local()));
    assert!(presence_publication.acknowledge());
    assert!(old_presence_root.upgrade().is_some(), "displaced presence root remains retained before bounded close");
    let _ = presence_publication.close_step(ephemeral_test_close_grant()).expect("presence close advances one owner");
    assert!(old_presence_root.upgrade().is_some(), "first close unit cannot destroy the displaced root");
    close_ephemeral_publication(&mut presence_publication);
    for _ in 0..2048 {
        if matches!(presence.maintenance_local_reads_step(physical_test_close_grant()).unwrap(), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(old_presence_root.upgrade().is_none(), "bounded retirement releases the displaced presence root");
    assert!(presence.begin_publish_one(semio_framework_job::OperationId(6), 0, DemoMutation::SetN(SetN { n: 9 }), Some(&presence_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());

    let transient_factory = DemoEphemeralPreparationFactory::admissible();
    let mut transient = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(2) });
    let root = transient.current_root();
    let mut cancelled = transient.begin_publish_one(semio_framework_job::OperationId(7), 0, DemoMutation::SetN(SetN { n: 6 }), Some(&transient_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).expect("transient factory admits");
    assert!(transient.cancel_publish_one(&mut cancelled));
    close_ephemeral_publication(&mut cancelled);
    assert_eq!(transient.generation_now(), 0);
    assert!(Arc::ptr_eq(&root, &transient.current_root()));
    drop(root);
    let old_transient_root = Arc::downgrade(&transient.current_root());
    let mut transient_publication =
        transient.begin_publish_one(semio_framework_job::OperationId(8), 0, DemoMutation::SetN(SetN { n: 11 }), Some(&transient_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).expect("transient publication admits");
    let transient_receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = transient.advance_publish_one(&mut transient_publication, grant).expect("transient step") {
            break receipt;
        }
    };
    assert_eq!(transient_receipt, LaneItemReceipt { generation_before: 0, generation_after: 1 });
    assert_eq!(transient.current().await.n, Some(11));
    let transient_root = transient_factory.published_root.lock().expect("transient witness").as_ref().and_then(std::sync::Weak::upgrade).expect("transient root");
    assert!(Arc::ptr_eq(&transient_root, &transient.current_root()));
    assert!(transient_publication.acknowledge());
    assert!(old_transient_root.upgrade().is_some(), "displaced transient root remains retained before bounded close");
    let _ = transient_publication.close_step(ephemeral_test_close_grant()).expect("transient close advances one owner");
    assert!(old_transient_root.upgrade().is_some(), "first close unit cannot destroy the displaced transient root");
    close_ephemeral_publication(&mut transient_publication);
    assert!(old_transient_root.upgrade().is_none(), "bounded retirement releases the displaced transient root");
    let oversized =
        DemoEphemeralPreparationFactory { footprint: ArtifactStoreOneItemFootprint { work_items: ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS + 1, retained_bytes: ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES }, published_root: Arc::new(DemoPublishedRootWitness::new()), snapshots: Arc::new(DemoSnapshotRetirementFactory), mutations: Arc::new(DemoMutationRetirementFactory) };
    assert!(transient.begin_publish_one(semio_framework_job::OperationId(9), 1, DemoMutation::SetN(SetN { n: 7 }), Some(&oversized), Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());
    assert!(transient.begin_publish_one(semio_framework_job::OperationId(10), 1, DemoMutation::SetN(SetN { n: 7 }), None, Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());
    drop(presence_root);
    let mut close = presence.begin_retirement(Arc::new(DemoSnapshot { n: Some(0) }), |value| value.n == Some(0)).ok().unwrap();
    test_support::drive_retirement(&mut close).expect("presence retirement reaches its terminal-empty witness under its quoted demands");
    assert!(close.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "terminal-empty witness")]
async fn artifact_store_one_item_drop_rejects_an_unclosed_publication_owner() {
    let store = Box::leak(Box::new(ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-drop", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await));
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(10),
            store.generation_now(),
            store.content_revision_now(),
            "retained-test".into(),
            vec![DemoMutation::SetN(SetN { n: 1 })],
            HistoryLane::Document,
            Some(&(Arc::new(DemoOneItemPreparationFactory::admissible()) as Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>>)),
            None,
        )
        .expect("publication admits");
    drop(publication);
}

/// 🪲️ A failing body that still holds a live, non-terminal `ArtifactStore` must surface as ONE
/// ordinary panic. Unguarded, the `Drop` witness fired during the unwind, and that double panic
/// became `thread caused non-unwinding panic. aborting.` — a `SIGABRT` that erased libtest's
/// summary for every other test in the binary. Sibling law to
/// `artifact_store_one_item_drop_rejects_an_unclosed_publication_owner` directly above, which
/// proves the witness stays strict when nothing is unwinding.
#[semio_framework_async_macros::async_test]
async fn a_panicking_body_holding_a_live_store_unwinds_instead_of_aborting_the_process() {
    let store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "unwind-witness", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _live = store;
        panic!("simulated assertion failure with a live store in scope");
    }));
    std::panic::set_hook(previous);
    let payload = outcome.expect_err("the simulated failure must unwind, not abort the process");
    let message = payload.downcast_ref::<String>().map(String::as_str).or_else(|| payload.downcast_ref::<&'static str>().copied());
    assert_eq!(message, Some("simulated assertion failure with a live store in scope"));
}


//#region 📸️CompoundEnvelopeReadLaws
struct GroupReadTriggerSnapshot {
    value: i32,
    commit: Option<Arc<Mutex<semio_framework_os_vcs::ArtifactGroupVisibilityOwner>>>,
    reads: Arc<std::sync::atomic::AtomicUsize>,
}

impl ToValue for GroupReadTriggerSnapshot {
    fn to_value(&self) -> DslValue {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if let Some(owner) = self.commit.as_ref() {
            owner.lock().expect("injected serializer decision").commit();
        }
        self.value.to_value()
    }
}

fn group_read_fixture_edit(id: &str) -> Edit<()> {
    Edit { line: None, id: id.into(), actor: None, forwards: Vec::new(), inverse: Vec::new().into(), mutation_meta: Vec::new(), verb: None, sequence_number: 0, started_at: String::new(), finished_at: None }
}

fn group_read_fixture_envelope(snapshot: GroupReadTriggerSnapshot) -> ArtifactEnvelope<GroupReadTriggerSnapshot, ()> {
    let mut edits = ArtifactHistoryLedger::new();
    edits.try_push(group_read_fixture_edit("initial")).unwrap();
    ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema: "group-read/v1".into(),
        id: "group-reader".into(),
        vcs: ArtifactVcs { genesis: AdmittedArtifactGenesis::from_decoded_pack(snapshot, vec![0]), edits, changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() },
        backbone: None,
        active_alternative_id: None,
        cursor: Some(ArtifactCursor::new(vec!["initial".into()], Vec::new(), None)),
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::HistoryFoldIndex::new(),
        viewer_checkpoint_id: None,
        edit_messages: ArtifactEditMessageLedger::new(),
        conflicts: Vec::new(),
        transitions: Vec::new(),
        history_shape: semio_framework_os_spr::HistoryShape::Document,
        open_transaction: None,
    })
}

/// 📏️ Drives an original owner within the caller's separately declared five-axis policy.
fn drive_retirement_within(owner: &mut dyn ErasedSnapshotRetirement, grant: RetainedCloneGrant) {
    for _ in 0..4096 {
        if owner.terminal_is_empty() { return; }
        owner.next_copy_byte_demand().expect("fallible current copy frontier");owner.next_capacity_byte_demand(grant.maximum_copy_bytes).expect("fallible current capacity frontier");owner.next_release_byte_demand().expect("fallible current release frontier");owner.next_depth_demand().expect("fallible current depth frontier");
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(grant).expect("supplied retirement turn"));
        assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        if matches!(step,RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty());return; }
    }
    panic!("original owner did not reach terminal empty within bounded supplied turns");
}

fn close_group_read_fixture(envelope: ArtifactEnvelope<GroupReadTriggerSnapshot, ()>) {
    let mut owners = envelope.into_owners();
    let mut cursor = ArtifactStoreCursorRetirement::new(owners.cursor.take().unwrap());
    test_support::drive_retirement(&mut cursor).expect("group read fixture cursor reaches its terminal-empty witness under its quoted demands");
    assert!(cursor.terminal_is_empty());
    while owners.vcs.edits.pop().is_some() {}
}

#[test]
fn retained_group_envelope_read_captures_history_and_cursor_before_decision() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️group-read.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let owner = Arc::new(Mutex::new(semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new()));
        let view = owner.lock().unwrap().view();
        let inject_commit = case["capture"] == "pending" && case["decision"] == "committed";
        let reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut envelope = group_read_fixture_envelope(GroupReadTriggerSnapshot { value: 0, commit: inject_commit.then(|| owner.clone()), reads: reads.clone() });
        let prepared_ids: Vec<String> = serde_json::from_value(fixture["prepared"]["appliedEditIds"].clone()).unwrap();
        envelope.cursor.as_mut().unwrap().stage_group_owned(ArtifactCursorOwners { applied_edit_ids: prepared_ids.clone().into(), redo_edit_ids: Vec::new().into(), checkpoint_id: None }, &view).unwrap();
        for id in &prepared_ids[1..] {
            let reservation = envelope.vcs.edits.reserve_group_one(&view).unwrap();
            envelope.vcs.edits.stage_group_reserved(reservation, group_read_fixture_edit(id), &view).unwrap();
        }
        if case["capture"] == "committed" {
            assert!(owner.lock().unwrap().commit());
        }
        let read = envelope.capture_read().unwrap();
        if case["decision"] == "aborted" {
            assert!(owner.lock().unwrap().abort());
        }
        if inject_commit { assert!(owner.lock().unwrap().commit()); }
        let captured: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&read)).unwrap();
        let fresh: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&envelope.capture_read().unwrap())).unwrap();
        assert_eq!(reads.load(std::sync::atomic::Ordering::SeqCst), 0);
        for (value, selector) in [(&captured, "captured"), (&fresh, "fresh")] {
            let expected = &fixture[case[selector].as_str().unwrap()];
            let actual_ids: Vec<_> = value["vcs"]["edits"].as_array().unwrap().iter().map(|edit| edit["id"].clone()).collect();
            assert_eq!(serde_json::json!(actual_ids), expected["history"], "{} {selector}", case["id"]);
            assert_eq!(value["cursor"]["appliedEditIds"], expected["appliedEditIds"], "{} {selector}", case["id"]);
        }
        drop(read);
        let retired_cursor = if view.committed() {
            envelope.vcs.edits.adopt_group(&view).unwrap();
            envelope.cursor.as_mut().unwrap().adopt_group_owned(&view).unwrap()
        } else {
            owner.lock().unwrap().abort();
            while envelope.vcs.edits.abort_group_one(&view).unwrap().is_some() {}
            envelope.cursor.as_mut().unwrap().abort_group_owned(&view).unwrap()
        };
        let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(retired_cursor));
        test_support::drive_retirement(&mut retirement).expect("retired group cursor reaches its terminal-empty witness under its quoted demands");
        assert!(retirement.terminal_is_empty());
        close_group_read_fixture(envelope);
    }
}

#[test]
fn retained_group_envelope_read_rejects_foreign_cursor_visibility_before_serialization() {
    let mut history_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let history = history_owner.view();
    let mut cursor_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let cursor = cursor_owner.view();
    let reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut envelope = group_read_fixture_envelope(GroupReadTriggerSnapshot { value: 0, commit: None, reads: reads.clone() });
    let reservation = envelope.vcs.edits.reserve_group_one(&history).unwrap();
    envelope.vcs.edits.stage_group_reserved(reservation, group_read_fixture_edit("foreign"), &history).unwrap();
    envelope.cursor.as_mut().unwrap().stage_group_owned(ArtifactCursorOwners::default(), &cursor).unwrap();
    assert!(envelope.capture_read().is_err());
    assert!(envelope.capture_read().map(|read| semio_framework_pack_json::to_json_string(&read)).is_err());
    assert_eq!(reads.load(std::sync::atomic::Ordering::SeqCst), 0);
    history_owner.abort();
    cursor_owner.abort();
    while envelope.vcs.edits.abort_group_one(&history).unwrap().is_some() {}
    drop(envelope.cursor.as_mut().unwrap().abort_group_owned(&cursor).unwrap());
    close_group_read_fixture(envelope);
}

//#endregion 📸️CompoundEnvelopeReadLaws

#[semio_framework_async_macros::async_test]
async fn retained_group_cursor_shares_history_visibility_and_retires_displaced_roots() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️group-cursor.json")).expect("group cursor fixture");
    let mut cursor: ArtifactCursor = serde_json::from_value(fixture["before"].clone()).expect("independent old cursor");
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["after"].clone()).expect("independent next cursor");
    let mut owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let view = owner.view();
    let mut history = ArtifactHistoryLedger::new();
    history.try_push("initial".to_string()).expect("old history");
    cursor.stage_group_owned(next, &view).expect("retained prepared cursor");
    for id in ["member-a", "member-b"] {
        let reservation = history.reserve_group_one(&view).expect("one history reservation");
        history.stage_group_reserved(reservation, id.to_string(), &view).expect("one prepared history record");
        assert_eq!(serde_json::to_value(&cursor).expect("old cursor serialization"), fixture["before"]);
        assert_eq!(cursor.applied_edit_ids.len(), 1);
        assert_eq!(history.len(), 1);
    }
    let held_before_commit: &ArtifactCursorOwners = &cursor;
    let held_history = history.iter();
    assert!(owner.commit());
    assert_eq!(held_before_commit.applied_edit_ids, ["initial".to_string()]);
    assert_eq!(held_history.cloned().collect::<Vec<_>>(), vec!["initial"]);
    assert_eq!(serde_json::to_value(&cursor).expect("committed cursor serialization"), fixture["after"]);
    assert_eq!(cursor.applied_edit_ids, history.iter().cloned().collect::<Vec<_>>());
    assert!(cursor.abort_group_owned(&view).is_err());
    let displaced = cursor.adopt_group_owned(&view).expect("exact displaced cursor owner");
    history.adopt_group(&view).expect("nonpublishing history adoption");
    assert_eq!(serde_json::to_value(&cursor).expect("adopted cursor serialization"), fixture["after"]);
    let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(displaced));
    drive_retirement_within(&mut retirement, RetainedCloneGrant { maximum_items:fixture["maximumItems"].as_u64().expect("item grant") as usize,maximum_copy_bytes:fixture["maximumBytes"].as_u64().expect("logical byte work") as usize,..physical_test_close_grant() });
    assert!(retirement.terminal_is_empty());
    while history.pop().is_some() {}
    assert!(history.terminal_is_empty());
    let mut abort_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let abort_view = abort_owner.view();
    let mut wrong_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let wrong_view = wrong_owner.view();
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["before"].clone()).expect("cancelled cursor owner");
    cursor.stage_group_owned(next, &abort_view).expect("unpublished next cursor");
    assert!(wrong_owner.abort());
    assert!(cursor.abort_group_owned(&wrong_view).is_err());
    assert!(abort_owner.abort());
    let cancelled = cursor.abort_group_owned(&abort_view).expect("returned exact uncommitted owner");
    assert_eq!(serde_json::to_value(cancelled).expect("cancelled owner oracle"), fixture["before"]);
    assert_eq!(serde_json::to_value(cursor).expect("unchanged committed cursor"), fixture["after"]);
}

#[semio_framework_async_macros::async_test]
async fn retained_group_cursor_empty_base_and_dropped_publisher_return_every_staged_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️group-cursor.json")).expect("group cursor fixture");
    let mut cursor = ArtifactCursor::default();
    let mut history = ArtifactHistoryLedger::<String>::new();
    let owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let view = owner.view();
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["after"].clone()).expect("prepared cursor");
    cursor.stage_group_owned(next, &view).expect("staged empty-base cursor");
    #[cfg(not(target_arch = "wasm32"))]
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cursor.applied_edit_ids.push("forbidden".into()))).is_err(), "DerefMut cannot expose either root while staged");
    let reservation = history.reserve_group_one(&view).expect("reserved but not staged slot");
    assert!(cursor.applied_edit_ids.is_empty() && history.is_empty());
    assert_eq!(serde_json::to_value(&cursor).expect("empty cursor"), serde_json::json!({}));
    drop(owner);
    assert!(!view.pending() && !view.committed(), "dropping the unique publisher aborts the decision without dropping staged payloads");
    history.cancel_reservation(reservation).expect("exact cancelled reservation");
    assert_eq!(history.abort_group_one(&view), Ok(None));
    assert!(history.terminal_is_empty());
    let cancelled = cursor.abort_group_owned(&view).expect("exact staged cursor survives publisher drop");
    assert_eq!(serde_json::to_value(&cancelled).expect("cancelled owner"), fixture["after"]);
    let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(cancelled));
    drive_retirement_within(&mut retirement, physical_test_close_grant());
    assert!(retirement.terminal_is_empty());
    assert!(cursor.applied_edit_ids.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn canonical_runtime_seed_retains_duplicate_owners_and_preflights_before_building() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌱️runtime-seed.json")).expect("runtime seed fixture");
    let ids: Vec<MutationId> = serde_json::from_value::<Vec<String>>(fixture["identities"].clone()).expect("fixture identities").into_iter().map(MutationId).collect();
    let expected: Vec<String> = serde_json::from_value(fixture["applied"].clone()).expect("independent fixture applied identities");
    let retired: Vec<String> = serde_json::from_value(fixture["retired"].clone()).expect("independent fixture duplicate identities");
    semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&ids).expect("duplicate identities are valid existing authority");
    let (mut dag, duplicates) = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::adopt_runtime_seed(ids);
    assert_eq!(duplicates, retired);
    let mut applied = Vec::new();
    while let Some(semio_framework_os_spr::MutationDagCloseOwner::Identity(id)) = dag.take_one_close_owner() {
        applied.push(id);
    }
    applied.reverse();
    assert_eq!(applied, expected);
    assert!(dag.terminal_is_empty());
    let mut retirement = ArtifactStoreStringVectorRetirement::new(duplicates);
    drive_retirement_within(&mut retirement, RetainedCloneGrant { maximum_items:fixture["maximumItems"].as_u64().expect("item grant") as usize,maximum_copy_bytes:fixture["maximumBytes"].as_u64().expect("logical byte work") as usize,..physical_test_close_grant() });
    assert!(retirement.terminal_is_empty());
    let oversized = vec![MutationId("x".repeat(semio_framework_os_spr::causal::MUTATION_DAG_IDENTIFIER_BYTES + 1))];
    assert!(semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&oversized).is_err());
    let saturated: Vec<_> = (0..=semio_framework_os_spr::causal::MUTATION_DAG_CAPACITY).map(|index| MutationId(index.to_string())).collect();
    assert!(semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&saturated).is_err());
}

#[semio_framework_async_macros::async_test]
async fn canonical_revision_distinguishes_interior_aba_across_load_and_reset() {
    let mut original = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "revision-aba", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    original.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    for n in [1, 2, 3] {
        test_support::dispatch_test_command(&mut original, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n })], transaction: None }).await.expect("seed revision edit");
    }
    let original_revision = original.content_revision().await;
    let mut changed = owned_test_envelope(&original).await;
    changed.vcs.edits[1].forwards = vec![DemoMutation::SetN(SetN { n: 99 })];
    changed.vcs.edits[1].inverse = vec![DemoMutation::AssignN(AssignN { n: Some(1) })].into();
    changed.vcs.edits[2].inverse = vec![DemoMutation::AssignN(AssignN { n: Some(99) })].into();

    let mut loaded = ArtifactStore::bare(changed).await;
    loaded.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    assert_eq!(original.snapshot().expect("original snapshot").n, loaded.snapshot().expect("loaded snapshot").n, "interior ABA keeps the same materialized endpoint");
    assert_ne!(original_revision, loaded.content_revision().await, "canonical identity must cover the changed interior edit, not only cursor endpoints");

    original.reset(owned_test_envelope(&loaded).await).await.expect("reset changed history");
    assert_eq!(original.content_revision().await, loaded.content_revision().await, "reset reconstruction must recover the canonical loaded identity");
    close_demo_artifact_store(&mut loaded);
    close_demo_artifact_store(&mut original);
}


#[semio_framework_async_macros::async_test]
async fn rejected_remote_ingest_keeps_state_and_dag_unpoisoned() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let valid = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 7 })).await;
    let mut malformed = valid.clone();
    malformed.diff.payload = vec![0xff];
    let before = owned_test_envelope(&store).await;

    assert!(store.ingest_remote(malformed).await.is_err(), "malformed remote data must reject before committing the DAG or history");
    assert_eq!(store.envelope(), &before);
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(0) });

    store.ingest_remote(valid).await.expect("the rejected envelope must not poison its mutation id in the DAG");
    assert_eq!(store.snapshot().expect("accepted snapshot"), DemoSnapshot { n: Some(7) });
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn remote_ingest_requires_duplicate_mutation_payload_equivalence() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let accepted = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 2 })).await;
    store.ingest_remote(accepted.clone()).await.expect("first remote envelope");
    let before = owned_test_envelope(&store).await;

    store.ingest_remote(accepted.clone()).await.expect("an exact duplicate is idempotent");
    let mut conflict = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 9 })).await;
    conflict.mutation_id = accepted.mutation_id.clone();
    let error = store.ingest_remote(conflict).await.expect_err("the same mutation id may not carry a different payload");
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("conflicts with its established payload")));
    assert_eq!(store.envelope(), &before);
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(2) });
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn a_remote_transition_repeating_a_known_id_with_another_payload_is_refused_without_mutation() {
    let mut author = fresh_demo_store().await;
    test_support::dispatch_test_command(&mut author, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("author edit");
    test_support::dispatch_test_command(&mut author, ArtifactCommand::Undo).await.expect("author undo");
    let events = author.event_log().expect("author event log");
    let mut replica = fresh_demo_store().await;
    for event in events.clone() {
        replica.ingest_remote(event).await.expect("replica ingests the author's events");
    }
    assert_eq!(replica.snapshot().expect("replica snapshot"), DemoSnapshot { n: Some(0) }, "the replicated revert withdraws the edit");
    let before = owned_test_envelope(&replica).await;
    let mut forged = events.last().expect("the revert transition").clone();
    let semio_framework_os_spr::HistoryTransition::Revert { mutation_ids } = semio_framework_os_spr::history_transition_from_envelope(&forged).expect("decode").expect("a transition") else { panic!("the last event is the revert") };
    forged.diff.payload = semio_framework_os_spr::encode_history_transition(&semio_framework_os_spr::HistoryTransition::Reinstate { mutation_ids });
    let error = replica.ingest_remote(forged).await.expect_err("a known transition id may not carry another payload");
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("conflicts with its established payload")));
    assert_eq!(replica.envelope(), &before);
    retire_demo_envelope(before);
}

/// 🐛️ Two replicas each commit a checkpoint over the same edit: every change on either replica must
/// name edits that replica actually holds, because a `Commit` transition names wire operations and
/// each replica resolves them against its own edit ledger.
#[semio_framework_async_macros::async_test]
async fn checkpoint_transitions_resolve_against_each_replicas_own_edits() {
    let mut a = fresh_demo_store().await;
    test_support::dispatch_test_command(&mut a, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 }), DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("a's two-operation edit");
    let mut b = fresh_demo_store().await;
    for event in a.event_log().expect("a's log") {
        b.ingest_remote(event).await.expect("b ingests a's operations");
    }
    test_support::dispatch_test_command(&mut b, ArtifactCommand::CommitCheckpoint { message: Some("b checkpoint".into()), authors: Vec::new() }).await.expect("b's own checkpoint");
    test_support::dispatch_test_command(&mut a, ArtifactCommand::CommitCheckpoint { message: Some("a checkpoint".into()), authors: Vec::new() }).await.expect("a's own checkpoint");
    for event in a.event_log().expect("a's log") {
        b.ingest_remote(event).await.expect("b ingests a's checkpoint transition");
    }
    for event in b.event_log().expect("b's log") {
        a.ingest_remote(event).await.expect("a ingests b's checkpoint transition");
    }
    for store in [&a, &b] {
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2, "both checkpoints are facts on every replica");
        for change in &store.envelope().vcs.changes {
            for edit_id in &change.edit_ids {
                assert!(store.envelope().vcs.edits.iter().any(|edit| edit.id == *edit_id), "change {} references edit {edit_id} the replica does not hold", change.id);
            }
        }
    }
    assert_eq!(a.snapshot().expect("a"), b.snapshot().expect("b"));
    assert_eq!(a.current_checkpoint_id().await, b.current_checkpoint_id().await, "the later commit in HLC order is current on both replicas");
}


/// 🛰️ A `DeleteN` at an earlier HLC and a `SetN` at a later HLC — replayed in HLC order, the
/// `SetN` lands on an already-deleted target and raises `mutation.target-missing` (Error).
async fn modify_vs_delete_envelopes() -> (semio_framework_os_spr::MutationEnvelope, semio_framework_os_spr::MutationEnvelope) {
    let delete = mutation_envelope_at("deleter", "op-delete", DemoMutation::DeleteN(DeleteN {}), HybridLogicalTimestamp::new(1, 100), Vec::new());
    let modify = mutation_envelope_at("modifier", "op-modify", DemoMutation::SetN(SetN { n: 42 }), HybridLogicalTimestamp::new(2, 200), Vec::new());
    (delete, modify)
}

#[semio_framework_async_macros::async_test]
async fn modify_vs_delete_quarantines_under_normal_and_vigilant() {
    for policy in [semio_framework_os_spr::MergePolicy::Normal, semio_framework_os_spr::MergePolicy::Vigilant] {
        let mut store = fresh_demo_store().await;
        store.set_merge_policy(policy);
        let (delete, modify) = modify_vs_delete_envelopes().await;
        store.ingest_remote(delete).await.expect("the delete alone raises no message and always applies");
        let pre_merge = store.snapshot().expect("pre-merge snapshot");
        let pre_merge_ids = store.applied_edit_ids().to_vec();
        let report = store.ingest_remote(modify).await.expect("a policy rejection is a MergeReport, not an Err");
        assert!(!report.accepted, "{policy:?} must reject an Error-level modify-vs-delete conflict");
        assert_eq!(report.worst, Some(semio_framework_diagnostic::Severity::Error));
        assert_eq!(store.snapshot().expect("unchanged snapshot"), pre_merge, "{policy:?}: state stays pre-merge on reject");
        assert_eq!(store.applied_edit_ids(), pre_merge_ids.as_slice(), "{policy:?}: applied_edit_ids stays pre-merge on reject");
        let conflict_id = report.conflict.clone().expect("a reject must raise a conflict");
        let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded on the store");
        assert_eq!(conflict.status, semio_framework_os_spr::ConflictStatus::Open);
        assert!(matches!(conflict.kind, semio_framework_os_spr::ConflictKind::Quarantined { .. }), "{policy:?}: a rejected batch quarantines, it never degrades");
        assert!(store.open_conflicts().any(|conflict| conflict.id == conflict_id));
    }
}

#[semio_framework_async_macros::async_test]
async fn modify_vs_delete_applies_under_laissez_faire_with_a_degraded_conflict() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let report = store.ingest_remote(modify).await.expect("LaissezFaire only rejects Fatal");
    assert!(report.accepted);
    assert_eq!(report.worst, Some(semio_framework_diagnostic::Severity::Error));
    assert_eq!(store.snapshot().expect("snapshot").n, None, "the modify's part is absent — LAW 2 (an Error message ⇒ no change to its target)");
    assert!(report.replayed.iter().any(|edit_messages| edit_messages.messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "an Error message must be reported");
    let conflict_id = report.conflict.expect("worst >= Warning must raise a Degraded conflict");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded");
    assert!(matches!(conflict.kind, semio_framework_os_spr::ConflictKind::Degraded { .. }));
    assert_eq!(conflict.status, semio_framework_os_spr::ConflictStatus::Open);
}

#[semio_framework_async_macros::async_test]
async fn chronological_determinism_any_arrival_order_converges() {
    let a = mutation_envelope_at("actor-a", "op-a", DemoMutation::SetN(SetN { n: 10 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    let b = mutation_envelope_at("actor-b", "op-b", DemoMutation::SetN(SetN { n: 20 }), HybridLogicalTimestamp::new(2, 300), Vec::new());

    let mut forward = fresh_demo_store().await;
    forward.ingest_remote(a.clone()).await.expect("a");
    forward.ingest_remote(b.clone()).await.expect("b");

    let mut reversed = fresh_demo_store().await;
    reversed.ingest_remote(b).await.expect("b");
    reversed.ingest_remote(a).await.expect("a");

    assert_eq!(forward.snapshot().expect("snapshot"), reversed.snapshot().expect("snapshot"));
    assert_eq!(forward.applied_edit_ids(), reversed.applied_edit_ids(), "both must land in the same HLC order regardless of arrival order");
    assert_eq!(forward.applied_edit_ids(), &["op-a".to_string(), "op-b".to_string()]);
    assert_eq!(forward.conflicts().len(), reversed.conflicts().len());
    assert!(forward.conflicts().is_empty(), "two non-conflicting SetN edits raise no conflict");
}

#[semio_framework_async_macros::async_test]
async fn empty_store_replays_a_remote_event_log_in_hlc_order_and_keeps_its_local_policy() {
    let mut remote = fresh_demo_store().await;
    let later = mutation_envelope_at("later-peer", "later-op", DemoMutation::SetN(SetN { n: 20 }), HybridLogicalTimestamp::new(2, 300), Vec::new());
    let earlier = mutation_envelope_at("earlier-peer", "earlier-op", DemoMutation::SetN(SetN { n: 10 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    remote.ingest_remote(later).await.expect("remote receives later edit first");
    remote.ingest_remote(earlier).await.expect("remote receives earlier edit second");

    let mut local = fresh_demo_store().await;
    local.set_merge_policy(semio_framework_os_spr::MergePolicy::Vigilant);
    for event in remote.event_log().expect("remote event log") {
        local.ingest_remote(event).await.expect("local replays the remote events");
    }

    assert_eq!(local.0.merge_policy(), semio_framework_os_spr::MergePolicy::Vigilant, "the receiving store's local-only policy is never carried by events");
    assert_eq!(local.snapshot().expect("replayed snapshot"), DemoSnapshot { n: Some(20) });
    assert_eq!(local.applied_edit_ids(), &["earlier-op".to_string(), "later-op".to_string()], "replay orders events by HLC rather than by log or arrival order");
    assert_eq!(local.conflicts(), remote.conflicts());
}

#[semio_framework_async_macros::async_test]
async fn a_backbone_genesis_of_another_document_is_refused_without_mutation() {
    let foreign = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(5) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let (channel, remote) = ChannelBackbone::pair("genesis").await;
    remote.push(BackboneMessage::Genesis { pack: foreign.envelope().vcs.genesis.pack().to_vec() }).await.expect("push foreign genesis");
    let mut target = fresh_demo_store().await;
    let error = target.attach_backbone(Backbones::Channel(channel)).await.expect_err("a different genesis names a different document");
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("different initial snapshot")));
    assert_eq!(target.snapshot().expect("unchanged"), DemoSnapshot { n: Some(0) });

    let (channel, remote) = ChannelBackbone::pair("genesis-shared").await;
    remote.push(BackboneMessage::Genesis { pack: DemoSnapshot { n: Some(0) }.encode_pack() }).await.expect("push shared genesis");
    let mut peer = fresh_demo_store().await;
    peer.attach_backbone(Backbones::Channel(channel)).await.expect("a shared genesis is the same document");
}

#[semio_framework_async_macros::async_test]
async fn quarantine_accept_equals_laissez_faire_result() {
    let mut quarantined = fresh_demo_store().await;
    quarantined.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    quarantined.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    let reject_report = quarantined.ingest_remote(modify.clone()).await.expect("reject is a report");
    assert!(!reject_report.accepted);
    let conflict_id = reject_report.conflict.expect("conflict raised");
    quarantined.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Accept).await.expect("accept");

    let mut laissez_faire = fresh_demo_store().await;
    laissez_faire.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    laissez_faire.ingest_remote(delete).await.expect("delete applies cleanly");
    laissez_faire.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    assert_eq!(quarantined.snapshot().expect("snapshot"), laissez_faire.snapshot().expect("snapshot"));
    assert_eq!(quarantined.applied_edit_ids(), laissez_faire.applied_edit_ids());
    assert_eq!(quarantined.conflicts().iter().filter(|conflict| conflict.status == semio_framework_os_spr::ConflictStatus::Open).count(), 0, "no Open conflict remains — accept must not raise a second conflict");
    assert_eq!(quarantined.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("original conflict kept").status, semio_framework_os_spr::ConflictStatus::Accepted);
}

#[semio_framework_async_macros::async_test]
async fn quarantine_discard_preserves_state() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_discard = store.snapshot().expect("pre-discard snapshot");
    let pre_discard_ids = store.applied_edit_ids().to_vec();
    let reject_report = store.ingest_remote(modify).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    store.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Discard).await.expect("discard");
    assert_eq!(store.snapshot().expect("snapshot"), pre_discard, "a discarded batch must never be applied");
    assert_eq!(store.applied_edit_ids(), pre_discard_ids.as_slice());
    assert_eq!(store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict kept").status, semio_framework_os_spr::ConflictStatus::Discarded);
}

#[semio_framework_async_macros::async_test]
async fn ledger_matches_a_fresh_replay_of_the_same_envelopes() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let modify_edit_id = modify.mutation_id.0.clone();

    let mut first = fresh_demo_store().await;
    first.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    first.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    first.ingest_remote(modify.clone()).await.expect("modify applies under LaissezFaire");

    let mut replay = fresh_demo_store().await;
    replay.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    replay.ingest_remote(delete).await.expect("delete applies cleanly");
    replay.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    assert_eq!(first.messages_for_edit(&modify_edit_id), replay.messages_for_edit(&modify_edit_id));
    assert!(!first.messages_for_edit(&modify_edit_id).is_empty(), "the modify edit must have raised a message");
    assert_eq!(first.snapshot().expect("snapshot"), replay.snapshot().expect("snapshot"));
}

/// ⏱️ Local edits get large physical-ms HLCs (the local clock ticks off the real wall clock).
///
/// A remote edit stamped with a tiny HLC — guaranteed to sort before both local edits.
#[semio_framework_async_macros::async_test]
async fn applied_edit_ids_stay_sorted_by_hlc_after_a_backdated_remote_insert() {
    let mut store = fresh_demo_store().await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("local apply 1");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("local apply 2");
    let local_ids = store.applied_edit_ids().to_vec();

    let backdated = mutation_envelope_at("backdated-actor", "op-backdated", DemoMutation::SetN(SetN { n: 99 }), HybridLogicalTimestamp::new(9, 1), Vec::new());
    store.ingest_remote(backdated).await.expect("backdated insert");

    assert_eq!(store.applied_edit_ids()[0], "op-backdated", "the backdated edit must sort before both local edits");
    assert!(store.applied_edit_ids().iter().skip(1).eq(local_ids.iter()));

    let envelope = store.envelope();
    let hlcs: Vec<HybridLogicalTimestamp> = store.applied_edit_ids().iter().map(|id| envelope.vcs.edits.iter().find(|edit| edit.id == *id).and_then(|edit| edit.mutation_meta.first()).map(|meta| meta.timestamp).expect("meta")).collect();
    let mut cmp_keys = Vec::with_capacity(hlcs.len());
    for hlc in &hlcs {
        cmp_keys.push(hlc.cmp_key());
    }
    assert!(cmp_keys.windows(2).all(|pair| pair[0] <= pair[1]), "applied_edit_ids must stay HLC-sorted: {hlcs:?}");
}

//#region 🔖️TestkitLawWiring
// 🎯️ G2 verification barrier (26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
// CONFLICTS) — the tests above assert the same scenarios by hand; these route the SAME
// scenarios through the frozen `📡️spr/🧫️fixtures` `🔖️Merge`/`🔖️Conflict` law helpers so the
// laws are proven to hold against this store's real `ingest_remote`/`resolve_conflict`/
// `messages_for_edit`/`.spr` codec, not only against testkit's own synthetic self-tests.

#[semio_framework_async_macros::async_test]
async fn testkit_law_modify_vs_delete_holds_under_normal_and_vigilant() {
    for policy in [semio_framework_os_spr::MergePolicy::Normal, semio_framework_os_spr::MergePolicy::Vigilant] {
        let mut store = fresh_demo_store().await;
        store.set_merge_policy(policy);
        let (delete, modify) = modify_vs_delete_envelopes().await;
        store.ingest_remote(delete).await.expect("delete applies cleanly");
        let pre_merge = store.snapshot().expect("pre-merge snapshot");
        let report = store.ingest_remote(modify).await.expect("a policy rejection is a MergeReport, not an Err");
        let post_merge = store.snapshot().expect("post-merge snapshot");
        semio_framework_os_spr::protocol_laws::assert_modify_vs_delete(policy, &pre_merge, &post_merge, &report, store.conflicts(), |snapshot: &DemoSnapshot| snapshot.n.is_some()).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_modify_vs_delete_holds_under_laissez_faire() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_merge = store.snapshot().expect("pre-merge snapshot");
    let report = store.ingest_remote(modify).await.expect("LaissezFaire only rejects Fatal");
    let post_merge = store.snapshot().expect("post-merge snapshot");
    semio_framework_os_spr::protocol_laws::assert_modify_vs_delete(semio_framework_os_spr::MergePolicy::LaissezFaire, &pre_merge, &post_merge, &report, store.conflicts(), |snapshot: &DemoSnapshot| snapshot.n.is_some()).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_chronological_determinism_holds_for_a_real_modify_vs_delete_batch() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let envelopes = [delete, modify];
    semio_framework_os_spr::protocol_laws::assert_chronological_determinism(envelopes.len(), 7, 6, async |order| {
        let mut store = fresh_demo_store().await;
        for &index in order {
            store.ingest_remote(envelopes[index].clone()).await.expect("real store ingest must not hard-error even when the batch ends up quarantined");
        }
        let snapshot = store.snapshot().expect("snapshot");
        let applied = store.applied_edit_ids().to_vec();
        let conflict_ids: Vec<semio_framework_os_spr::ConflictId> = store.conflicts().iter().map(|conflict| conflict.id.clone()).collect();
        (snapshot, applied, conflict_ids)
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_quarantine_accept_equals_laissez_faire_via_real_store() {
    let mut quarantined = fresh_demo_store().await;
    quarantined.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    quarantined.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    let reject_report = quarantined.ingest_remote(modify.clone()).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    quarantined.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Accept).await.expect("accept");

    let mut laissez_faire = fresh_demo_store().await;
    laissez_faire.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    laissez_faire.ingest_remote(delete).await.expect("delete applies cleanly");
    laissez_faire.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    let accepted_state = quarantined.snapshot().expect("accepted snapshot");
    let laissez_faire_state = laissez_faire.snapshot().expect("laissez-faire snapshot");
    semio_framework_os_spr::protocol_laws::assert_quarantine_accept_equals_laissez_faire(&accepted_state, &laissez_faire_state).await;
}

/// 🗃️ `relayed`: every edit id this store's persisted history (`applied_edit_ids`) could ever
/// ship onward via `flush_outbound`/`snapshot_pack` — a discarded batch is only `seed_
/// applied` on the dag, never added to `applied_edit_ids`/`vcs.edits`, so it can never appear
/// here; this is the real set flush_outbound draws from, not a fabricated stand-in.
#[semio_framework_async_macros::async_test]
async fn testkit_law_quarantine_discard_preserves_state_via_real_store() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_discard = store.snapshot().expect("pre-discard snapshot");
    let reject_report = store.ingest_remote(modify.clone()).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    store.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Discard).await.expect("discard");
    let post_discard = store.snapshot().expect("post-discard snapshot");
    let relayed = store.applied_edit_ids().to_vec();
    semio_framework_os_spr::protocol_laws::assert_quarantine_discard_preserves_state(&pre_discard, &post_discard, std::slice::from_ref(&modify.mutation_id.0), &relayed).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_ledger_matches_replay_via_real_store() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let modify_edit_id = modify.mutation_id.0.clone();

    let mut first = fresh_demo_store().await;
    first.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    first.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    first.ingest_remote(modify.clone()).await.expect("modify applies under LaissezFaire");

    let mut replay = fresh_demo_store().await;
    replay.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    replay.ingest_remote(delete).await.expect("delete applies cleanly");
    replay.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    let mut ledger = HashMap::new();
    ledger.insert(modify_edit_id.clone(), first.messages_for_edit(&modify_edit_id).to_vec());
    let mut replayed = HashMap::new();
    replayed.insert(modify_edit_id.clone(), replay.messages_for_edit(&modify_edit_id).to_vec());
    assert!(!ledger[&modify_edit_id].is_empty(), "the modify edit must have raised a message for this law to be meaningful");

    semio_framework_os_spr::protocol_laws::assert_ledger_matches_replay(&ledger, &replayed).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_conflict_spr_round_trip_via_real_store() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let report = store.ingest_remote(modify).await.expect("reject is a report");
    let conflict_id = report.conflict.expect("conflict raised");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded").clone();

    let pack_bytes = DemoSnapshot { n: Some(0) }.encode_pack();
    let encode = async |conflict: &semio_framework_os_spr::Conflict| -> Vec<u8> {
        let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        envelope.conflicts = vec![conflict.clone()];
        let bytes = print_document_spr(&envelope).await.expect("encode conflict via the real .spr codec");
        retire_demo_envelope(envelope);
        bytes
    };
    let decode = async |bytes: &[u8]| -> semio_framework_os_spr::Conflict {
        let parsed = parse_document_spr::<DemoSnapshot, DemoMutation>(&pack_bytes, bytes, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("decode conflict via the real .spr codec");
        let conflict = parsed.envelope.conflicts.first().cloned().expect("one conflict round-tripped");
        retire_demo_envelope(parsed.envelope);
        conflict
    };
    semio_framework_os_spr::protocol_laws::assert_conflict_spr_round_trip(&conflict, encode, decode).await;
}

//#endregion 🔖️TestkitLawWiring

//#region 🔖️RobustnessTests
// 🎯️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1 — security/
// robustness audit findings HIGH-1, HIGH-2, MEDIUM-3, MEDIUM-4 (`📓️j1-robustness-fixes.md`).
// Every test here FAILS without its corresponding fix.

/// 👻️ HIGH-1: an id named in `order[k..]` but absent from `edits` means `applied_edit_ids`/
/// `vcs.edits` fell out of sync (crash/recovery, a partially-applied ingest) — a bare
/// `continue` here used to silently compute a WRONG snapshot instead of failing loudly.
#[semio_framework_async_macros::async_test]
async fn replay_suffix_partitioned_errors_loudly_on_a_ghost_edit_id_instead_of_silently_dropping_it() {
    let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
    let order = vec!["ghost-edit".to_string()];
    let error = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::replay_suffix_partitioned(Arc::new(DemoSnapshot { n: Some(0) }), &order, 0, &edits, "demo/v1", &EffectiveSupersessions::new(), semio_framework_os_spr::MergePolicy::Normal)
        .err()
        .expect("a ghost edit id must be a loud, typed VcsError");
    assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
}

/// 🔦️ HIGH-2: minting a `ConflictId` from a `filter_map` that silently drops missing ids can
/// (if every id is missing) hash an EMPTY mutation-id set into a content-addressed conflict
/// id — a content address that addresses no content, so unrelated conflicts can collide on
/// it. `edits_for_ids` is the strict replacement both `ingest_remote` mint sites now use.
#[semio_framework_async_macros::async_test]
async fn edits_for_ids_errors_loudly_on_a_ghost_edit_id_instead_of_silently_filtering_it() {
    let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
    let error = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::edits_for_ids(&["ghost-edit".to_string()], &edits).expect_err("an id that resolves to nothing must fail loudly, never vanish from the mutation-id set");
    assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
}

/// 🛰️ A synthetic `Open` conflict for capacity/pruning fixtures — content doesn't matter, only
/// that its `ConflictId`/timestamp are distinct per `seed` and it never gets touched by real
/// replay logic (these tests push it directly onto `envelope.conflicts`, bypassing `ingest_
/// remote`/`resolve_conflict` entirely, exactly like the existing hand-built conflict fixtures
/// in `🔖️PreviewWireTests` above).
async fn synthetic_open_conflict(seed: u64) -> semio_framework_os_spr::Conflict {
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![format!("synthetic-edit-{seed}")] };
    let artifact = ArtifactId("demo".into());
    let mutation_ids = vec![MutationId(format!("synthetic-op-{seed}"))];
    let timestamp = HybridLogicalTimestamp::new(seed, seed);
    semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId::new(&kind, &artifact, &mutation_ids, &timestamp).await,
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages: Vec::new(),
        actors: vec![ActorId(format!("synthetic-actor-{seed}").into())],
        timestamp,
    }
}

#[semio_framework_async_macros::async_test]
async fn conflict_retirement_cursors_quarantined_payloads_messages_actors_and_ids() {
    let mut envelope = foreign_mutation_envelope("conflict-actor", DemoMutation::SetN(SetN { n: 9 })).await;
    envelope.dependencies = (0..32).map(|index| MutationId(format!("dependency-{index}-{}", "d".repeat(128)))).collect();
    envelope.diff.payload = vec![7; 16 * 1024];
    envelope.inverse.payload = vec![3; 16 * 1024];
    let kind = semio_framework_os_spr::ConflictKind::Quarantined { envelopes: vec![envelope] };
    let timestamp = HybridLogicalTimestamp::new(41, 43);
    let conflict = semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId("conflict-document".repeat(64)), &[MutationId("conflict-mutation".repeat(64))], &timestamp).await,
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages: vec![semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", "message".repeat(256)).at(["target".repeat(256)])],
        actors: vec![ActorId("actor".repeat(256).into())],
        timestamp,
    };
    let mut retirement = ArtifactStoreConflictRetirement::new(conflict);
    test_support::drive_retirement(&mut retirement).expect("deep conflict retirement terminates across repeated interruption under its quoted demands");
    assert!(retirement.terminal_is_empty());
}

/// 🚧️ MEDIUM-3: a peer that keeps sending a batch this replica keeps quarantining can grow
/// `envelope.conflicts` without bound (the dag never advances on quarantine, so the SAME
/// envelope is eligible for redelivery forever). Hitting the open-conflict cap must be a
/// loud, typed refusal — atomic, nothing applied — never a silent drop or overwrite.
#[semio_framework_async_macros::async_test]
async fn ingest_remote_refuses_a_new_open_conflict_once_the_backlog_is_at_capacity() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("the delete alone raises no message and always applies");

    let cap = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::OPEN_CONFLICT_CAP;
    for seed in 0..cap as u64 {
        store.0.envelope.conflicts.push(synthetic_open_conflict(seed).await);
    }
    assert_eq!(store.open_conflicts().count(), cap, "fixture must actually be at capacity for this test to be meaningful");

    let before_edits = store.envelope().vcs.edits.len();
    let before_snapshot = store.snapshot().expect("pre-attempt snapshot");
    let before_ids = store.applied_edit_ids().to_vec();

    let error = store.ingest_remote(modify).await.expect_err("minting one more Open conflict past the cap must be a loud, typed refusal");
    assert!(matches!(&error, VcsError::ValidationFailed(message) if message.contains("capacity")), "got {error:?}");
    assert_eq!(store.envelope().vcs.edits.len(), before_edits, "a refused mint must be fully atomic — nothing about the attempted batch applied");
    assert_eq!(store.snapshot().expect("snapshot"), before_snapshot);
    assert_eq!(store.applied_edit_ids(), before_ids.as_slice());
    assert_eq!(store.open_conflicts().count(), cap, "the backlog must stay exactly at capacity, never silently grow past it");
}

/// ✂️ MEDIUM-3: resolved (`Accepted`/`Discarded`) conflicts are closed historical facts, so
/// unlike `Open` ones they are prunable — oldest push order evicted first, capped, while an
/// `Open` conflict is never touched no matter how far over cap the resolved backlog grows.
#[semio_framework_async_macros::async_test]
async fn resolved_conflicts_are_pruned_oldest_first_once_the_ledger_exceeds_its_cap_while_open_ones_survive() {
    let mut store = fresh_demo_store().await;
    let cap = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::RESOLVED_CONFLICT_CAP;
    let open = synthetic_open_conflict(u64::MAX).await;
    store.0.envelope.conflicts.push(open.clone());
    let overflow: u64 = 20;
    let resolved_count = cap as u64 + overflow;
    for seed in 0..resolved_count {
        let mut resolved = synthetic_open_conflict(seed).await;
        resolved.status = semio_framework_os_spr::ConflictStatus::Accepted;
        store.0.envelope.conflicts.push(resolved);
    }
    assert_eq!(store.conflicts().len(), resolved_count as usize + 1);

    store.0.prune_resolved_conflicts().expect("bounded resolved-conflict retirement");

    assert_eq!(store.conflicts().len(), cap, "pruning must bring the ledger back down to the cap");
    assert!(store.conflicts().iter().any(|conflict| conflict.id == open.id && conflict.status == semio_framework_os_spr::ConflictStatus::Open), "the Open conflict must never be evicted, no matter how far over cap the resolved backlog grows");
    let surviving_seeds: HashSet<u64> = store.conflicts().iter().filter(|conflict| conflict.status == semio_framework_os_spr::ConflictStatus::Accepted).map(|conflict| conflict.timestamp.physical_ms).collect();
    assert_eq!(surviving_seeds.len(), cap - 1);
    for evicted_seed in 0..=overflow {
        assert!(!surviving_seeds.contains(&evicted_seed), "seed {evicted_seed} was among the oldest resolved conflicts and must be pruned first");
    }
    assert!(surviving_seeds.contains(&(resolved_count - 1)), "the newest resolved conflict must survive pruning");
}

/// 🧼️ MEDIUM-4: one `ingest_remote` batch that quarantines BOTH a brand-new edit (`op-c`, never
/// committed before) AND a previously-committed edit a rewind now retroactively invalidates
/// (`op-a`, which carried a REAL non-empty `mutation.cascade` message from its first commit)
/// — proving `replace_edit_messages(.., empty)` clears the stale ledger entry correctly even
/// when it runs in the same pass as a never-populated one, regardless of which kind an id is.
///
/// `op-b` (earlier HLC than `op-a`) forces a rewind that replays `op-a` against a DELETED
/// target the second time around. `op-c` (later HLC, brand new) is submitted FIRST but
/// depends on `op-b`, so it buffers in the dag and is released alongside `op-b` in the SAME
/// `ingest_remote` call/batch — the mixed-kind scenario MEDIUM-4 asks for.
#[semio_framework_async_macros::async_test]
async fn quarantine_message_clearing_is_correct_for_a_mixed_new_and_retroactive_batch() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);

    let a = mutation_envelope_at("actor-a", "op-a", DemoMutation::AddN(AddN { delta: 5 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    store.ingest_remote(a).await.expect("op-a commits cleanly the first time, on a fresh n=0 target");
    assert!(!store.messages_for_edit("op-a").is_empty(), "fixture must carry real prior ledger content for this test to be meaningful");
    assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: Some(5) });

    let b = mutation_envelope_at("actor-b", "op-b", DemoMutation::DeleteN(DeleteN {}), HybridLogicalTimestamp::new(1, 50), Vec::new());
    let c = mutation_envelope_at("actor-c", "op-c", DemoMutation::AddN(AddN { delta: 1 }), HybridLogicalTimestamp::new(2, 150), vec![MutationId("op-b".into())]);
    store.ingest_remote(c).await.expect("op-c buffers behind its unmet dependency on op-b");
    let report = store.ingest_remote(b).await.expect("op-b's arrival releases op-b and op-c into the same batch");

    assert!(!report.accepted, "op-c never committed");
    assert_eq!(store.applied_edit_ids(), &["op-b".to_string()], "only op-b committed — op-a dropped out on retroactive invalidation, op-c never entered");
    assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: None });

    assert!(store.messages_for_edit("op-a").is_empty(), "op-a's stale non-empty ledger entry must be cleared, not left stale, once it is retroactively quarantined");
    assert!(store.messages_for_edit("op-c").is_empty(), "op-c never committed, so it must never gain a ledger entry");
    assert!(store.envelope().edit_messages.iter().all(|entry| entry.edit_id != "op-a"), "a cleared entry must be fully removed from the durable ledger, not left behind as an empty Vec");
    assert!(store.envelope().edit_messages.iter().all(|entry| entry.edit_id != "op-c"), "op-c must never appear in the durable ledger at all");

    let conflict_id = report.conflict.expect("the mixed quarantine batch must raise one conflict");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded on the store");
    assert_eq!(conflict.status, semio_framework_os_spr::ConflictStatus::Open);
    match &conflict.kind {
        semio_framework_os_spr::ConflictKind::Quarantined { envelopes } => assert_eq!(envelopes.len(), 2, "both op-a (retroactive) and op-c (new) must be quarantined TOGETHER in one conflict"),
        other => panic!("expected a Quarantined conflict, got {other:?}"),
    }
    assert_eq!(store.conflicts().len(), 1, "op-a's clean first commit never raised a conflict of its own — only this one mixed-batch conflict must exist");
}

//#endregion 🔖️RobustnessTests

#[semio_framework_async_macros::async_test]
async fn composition_pins_rederive_checkpoint_identity_without_partial_mutation() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() }).await.expect("checkpoint");
    let original_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
    let before = owned_test_envelope(&store).await;
    let invalid = semio_framework_os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: demo_child_dialect() }, checkpoint_id: "child-checkpoint".into() };

    assert!(store.set_checkpoint_composition_pins(&original_checkpoint_id, vec![invalid]).await.is_err());
    assert_eq!(store.envelope(), &before);

    let pin = semio_framework_os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef { artifact_id: "child".into(), dialect: demo_child_dialect() }, checkpoint_id: "child-checkpoint".into() };
    store.set_checkpoint_composition_pins(&original_checkpoint_id, vec![pin]).await.expect("valid pin update");
    let rederived = &store.envelope().vcs.checkpoints[0];
    assert_ne!(rederived.id, original_checkpoint_id);
    assert_eq!(rederived.composition_pins.len(), 1);
    let mut revalidated = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(owned_test_envelope(&store).await, store.local_actor_id().clone()).await.expect("a persisted pinned checkpoint must validate its rederived identity");
    revalidated.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    close_test_store(&mut revalidated);
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn materialize_replays_forward_mutations() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
    assert_eq!(store.envelope().vcs.edits.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(0));
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
}
