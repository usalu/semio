/// 🧳️ Retained clone preparation fixtures: the demo ephemeral preparation factory and preparation, the retained clone edit and its cursor, the probed preparation factory and the retained text edit.

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
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
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
        let original = match protocol::apply_diff_cold(outcome.diff(), &request.base) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original ephemeral mutation application refused")); } };
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

    fn preflight(&self, _mutation: &DemoMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, &'static str> {
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
    fn advance_demands(&self, _: RetainedCloneRef<'_, DemoSnapshot>, _: &DemoSnapshot, _: RetainedCloneRef<'_, DemoMutation>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(semio_framework_value::RetirementDemand { copy_bytes: if self.phase >= 2 { std::mem::size_of::<DemoSnapshot>() } else { 0 }, capacity_bytes: if self.phase == 1 { std::mem::size_of::<DemoMutation>() } else { 0 }, depth: 1, ..Default::default() }) }
    fn advance(&mut self, base: RetainedCloneRef<'_, DemoSnapshot>, post: &mut DemoSnapshot, mutation: RetainedCloneRef<'_, DemoMutation>, body:usize,grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
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
        let original = match protocol::apply_diff_cold(outcome.diff(), base.get()) { Ok(original) => original, Err(error) => { *self.semantic_refusal = Some(error); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original retained mutation application refused")); } };
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
        fn begin_copy_byte_demand(&self,input:&DemoMutation,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; self.inner.begin_copy_byte_demand(input,lane)?.checked_add(std::mem::size_of::<ProbedRetainedClonePreparation>()*2).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original wrapper header quote overflow"))}

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
    fn advance(&mut self,body:usize,grant:ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<ArtifactStoreOneItemPreparationStep,ValueError> {
        if self.closing || grant.maximum_depth<=1 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        self.inner.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"probe no longer owns its typed preparation"))?.advance(body,ArtifactStoreOneItemGrant { maximum_depth:grant.maximum_depth-1,..grant },receiving)
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

    fn preflight(&self, _mutation: &RetainedTextMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, &'static str> {
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
    fn advance_demands(&self, _: RetainedCloneRef<'_, RetainedTextSnapshot>, _: &RetainedTextSnapshot, _: RetainedCloneRef<'_, RetainedTextMutation>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained text edit must not quote after contiguous-capacity refusal")) }
    fn advance(&mut self, _base: RetainedCloneRef<'_, RetainedTextSnapshot>, _post: &mut RetainedTextSnapshot, _mutation: RetainedCloneRef<'_, RetainedTextMutation>, body:usize,_grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
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
