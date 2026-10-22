//! 🌫️ Transient and interaction stores: the transient store with its retirement and the interaction store with its default and operations. See [`super`].

use super::*;

/// 🫧️ The TRANSIENT lane's store: ephemeral LOCAL-ONLY UI state.
///
/// 🎯️ Presence minus the roster. Nothing here is ever shared, persisted, packed, checkpointed or
/// undone — it is exactly the state that used to hide in plugin `thread_local!`s and untyped shell
/// fields, given a typed home so it can be reached only through `Emit`/`Lanes` like every other
/// lane. If a value must survive a reload it belongs in `config`; if a peer must see it, in
/// `presence`; if it is document content, in the artifact (or its draft).
pub struct TransientStore<P, Mutation> {
    pub(super) current: Option<Arc<P>>,
    pub(super) reads: Option<SnapshotReadRegistryHandle>,
    pub(super) close_started: bool,
    active_returned_read: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub(super) generation: u64,
    pub(super) _mutation: PhantomData<fn() -> Mutation>,
}

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<P: Clone + Default, Mutation: super::Mutation<P> + protocol::ColdMutation<P>> Default for TransientStore<P, Mutation> {
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P: Clone, Mutation: super::Mutation<P> + protocol::ColdMutation<P>> TransientStore<P, Mutation> {
    /// 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(current: P) -> Self {
        Self { current: Some(Arc::new(current)), reads: Some(SnapshotReadRegistryHandle::new()), close_started: false, active_returned_read: None, generation: 0, _mutation: PhantomData }
    }

    pub async fn current(&self) -> &P {
        self.current.as_ref().expect("live transient root remains").as_ref()
    }

    /// 🧵️ Captures the event-maintained transient root without cloning its payload.
    pub fn current_root(&self) -> Arc<P> {
        self.current.as_ref().expect("live transient root remains").clone()
    }

    /// 🧵️ Captures one tracked transient root capability for retained work.
    pub fn current_read(&self) -> Result<SnapshotRead<P>, String>
    where
        P: Send + Sync + 'static,
    {
        let owner = Arc::clone(self.current.as_ref().expect("live transient root remains"));
        let lease = self.reads.as_ref().expect("live transient read registry remains").try_issue(owner.clone()).map_err(|_| "transient read registry is busy or exhausted".to_string())?;
        Ok(SnapshotRead::new(owner, lease))
    }

    /// 🧰️ Captures the same tracked capability across a heterogeneous owner registry.
    pub fn current_read_erased(&self) -> Result<ErasedSnapshotRead, String>
    where
        P: Send + Sync + 'static,
    {
        let owner = Arc::clone(self.current.as_ref().expect("live transient root remains"));
        let lease = self.reads.as_ref().expect("live transient read registry remains").try_issue(owner.clone()).map_err(|_| "transient read registry is busy or exhausted".to_string())?;
        Ok(ErasedSnapshotRead::new(owner, lease))
    }

    /// 🧹️ Advances one returned read through its exact owned-value retirement authority.
    pub fn maintenance_returned_reads_step(&mut self, factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where
        P: Send + Sync + 'static,
    {
        match self.reads.as_ref() { Some(reads) => advance_returned_snapshot_read(reads, &mut self.active_returned_read, factory, grant), None => Ok(RetainedCloneStep::Complete(Default::default())) }
    }

    /// 🧹️ Advances one returned read through a root-only retirement authority, for stores that own no owned-value factory.
    pub fn maintenance_returned_reads_step_root(&mut self, factory: &dyn SnapshotRetirementFactory<P>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where
        P: Send + Sync + 'static,
    {
        match self.reads.as_ref() { Some(reads) => advance_returned_snapshot_read_root(reads, &mut self.active_returned_read, factory, grant), None => Ok(RetainedCloneStep::Complete(Default::default())) }
    }

    /// 📏️ Quotes the next returned read's birth or physical retirement under a root-only retirement authority.
    pub fn maintenance_returned_reads_demands_root(&self, factory: &dyn SnapshotRetirementFactory<P>, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where
        P: Send + Sync + 'static,
    {
        if let Some(owner) = self.active_returned_read.as_ref() { return artifact_retirement_box_demands(owner, body); }
        match self.reads.as_ref() {
            Some(reads) if reads.has_returned() => reads.returned_admission_demands::<P>(|root| semio_framework_value::RetirementDemand { capacity_bytes: factory.retirement_birth_bytes(root), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error),
            _ => Ok(Default::default()),
        }
    }

    /// 🧾️ A returned read, or its active retirement, still awaits the returned-read pump.
    pub fn returned_read_pending(&self) -> bool {
        self.active_returned_read.is_some() || self.reads.as_ref().is_some_and(|reads| reads.has_returned())
    }

    pub fn returned_reads_terminal_is_empty(&self) -> bool {
        self.active_returned_read.is_none() && self.reads.as_ref().is_none_or(|reads| reads.terminal_is_empty())
    }

    /// 🔒️ A live tracked read, not yet returned, is the only owner that keeps the read registry from terminal-empty custody.
    pub fn live_read_holds_registry(&self) -> bool {
        self.active_returned_read.is_none() && self.reads.as_ref().is_some_and(|reads| !reads.has_returned() && !reads.terminal_is_empty())
    }

    pub fn current_weak(&self) -> std::sync::Weak<P> {
        self.current.as_ref().map_or_else(std::sync::Weak::new, Arc::downgrade)
    }

    pub fn current_matches(&self, root: &std::sync::Weak<P>) -> bool {
        root.upgrade().is_some_and(|root| self.current.as_ref().is_some_and(|current| Arc::ptr_eq(&root, current)))
    }

    /// ✍️ Applies transient operations atomically, without history.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> protocol::MutationApplyResult<()> {
        let mut candidate = self.current.as_ref().expect("live transient root remains").as_ref().clone();
        for mutation in mutations {
            candidate = apply_operation::<P, Mutation>(&candidate, mutation, 0).0?;
        }
        if !mutations.is_empty() {
            self.current = Some(Arc::new(candidate));
            self.generation = self.generation.wrapping_add(1);
        }
        Ok(())
    }

    pub fn begin_publish_one(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        mutation: Mutation,
        factory: Option<&dyn ArtifactEphemeralOneItemPreparationFactory<P, Mutation>>,
        root_retirement_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    ) -> Result<ArtifactEphemeralOneItemPublication<P, Mutation>, ArtifactEphemeralOneItemAdmissionRejected<Mutation>> {
        let reject = |reason: String, mutation: Mutation| ArtifactEphemeralOneItemAdmissionRejected { reason, mutation };
        if self.close_started || self.generation != expected_generation {
            return Err(reject("transient publication generation is stale".into(), mutation));
        }
        let Some(factory) = factory else {
            return Err(reject("transient publication requires an explicit app-owned retained preparation factory".into(), mutation));
        };
        let Some(root_retirement_factory) = root_retirement_factory else {
            return Err(reject("transient publication requires an explicit local-root retirement factory".into(), mutation));
        };
        let footprint = match factory.preflight(&mutation) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            Ok(_) => return Err(reject("transient publication footprint exceeds its fixed item or byte capacity".into(), mutation)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        let request =
            ArtifactEphemeralOneItemPreparationRequest { operation, generation: semio_framework_job::Generation(expected_generation), base: ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::Transient(Arc::clone(self.current.as_ref().expect("live transient root remains")))), mutation };
        let preparation = match factory.begin(request) {
            Ok(preparation) => preparation,
            Err(request) => return Err(reject("transient preparation factory rejected its exact owner bundle".into(), request.mutation)),
        };
        Ok(ArtifactEphemeralOneItemPublication {
            operation,
            expected_generation,
            footprint,
            preparation: std::mem::ManuallyDrop::new(Some(preparation)),
            displaced_root_retirement: std::mem::ManuallyDrop::new(None),
            root_retirement_factory: std::mem::ManuallyDrop::new(Some(root_retirement_factory)),
            returned_read_registry: std::mem::ManuallyDrop::new(None),
            returned_read_retirement: std::mem::ManuallyDrop::new(None),
            owned_retirement_factory: std::mem::ManuallyDrop::new(None),
            receipt: std::mem::ManuallyDrop::new(None),
            retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            factory_retirement: [None, None],
            attempts: 0,
            published: false,
            cancel_requested: false,
            close_started: false,
            fault: std::mem::ManuallyDrop::new(None),
            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
        })
    }

    /// 📬️ Begins one transient publication from a tracked base read. Displaced roots use
    /// unique-or-release retirement so a still-live tool read cannot create a close cycle.
    pub fn begin_publish_one_leased(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        mutation: Mutation,
        factory: &dyn ArtifactEphemeralOneItemPreparationFactory<P, Mutation>,
        owned_retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    ) -> Result<ArtifactEphemeralOneItemPublication<P, Mutation>, ArtifactEphemeralOneItemAdmissionRejected<Mutation>>
    where
        P: Send + Sync + 'static,
    {
        let reject = |reason: String, mutation: Mutation| ArtifactEphemeralOneItemAdmissionRejected { reason, mutation };
        if self.close_started || self.generation != expected_generation {
            return Err(reject("transient publication generation is stale".into(), mutation));
        }
        let footprint = match factory.preflight(&mutation) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            Ok(_) => return Err(reject("transient publication footprint exceeds its fixed item or byte capacity".into(), mutation)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        let base = match self.current_read() {
            Ok(read) => ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::TransientRead(read)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        let request = ArtifactEphemeralOneItemPreparationRequest { operation, generation: semio_framework_job::Generation(expected_generation), base, mutation };
        let preparation = match factory.begin(request) {
            Ok(preparation) => preparation,
            Err(request) => return Err(reject("transient preparation factory rejected its exact owner bundle".into(), request.mutation)),
        };
        Ok(ArtifactEphemeralOneItemPublication {
            operation,
            expected_generation,
            footprint,
            preparation: std::mem::ManuallyDrop::new(Some(preparation)),
            displaced_root_retirement: std::mem::ManuallyDrop::new(None),
            root_retirement_factory: std::mem::ManuallyDrop::new(None),
            returned_read_registry: std::mem::ManuallyDrop::new(Some(SnapshotReadRegistryAliasRetirement::new(self.reads.as_ref().expect("live transient read registry remains").clone()))),
            returned_read_retirement: std::mem::ManuallyDrop::new(None),
            owned_retirement_factory: std::mem::ManuallyDrop::new(Some(owned_retirement_factory)),
            receipt: std::mem::ManuallyDrop::new(None),
            retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            factory_retirement: [None, None],
            attempts: 0,
            published: false,
            cancel_requested: false,
            close_started: false,
            fault: std::mem::ManuallyDrop::new(None),
            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
        })
    }

    pub fn advance_publish_one(&mut self, publication: &mut ArtifactEphemeralOneItemPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemAdvance, String>
    where
        P: Send + Sync + 'static,
    {
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Complete {
            return Ok(ArtifactStoreOneItemAdvance::Complete);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Closing {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::AwaitingAck {
            return Ok(ArtifactStoreOneItemAdvance::AwaitingAck(publication.receipt.expect("transient publication awaiting ACK retains its receipt")));
        }
        if publication.cancel_requested {
            publication.begin_close();
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if self.close_started || self.generation != publication.expected_generation {
            *publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"transient publication became stale before commit"));
            publication.phase = ArtifactStoreOneItemPublicationPhase::Fault;
            publication.begin_close();
            return Err("transient publication became stale before commit".into());
        }
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        match publication.phase {
            ArtifactStoreOneItemPublicationPhase::Preparing => match publication.advance_preparation(grant, receiving) {
                Err(refused)=>Ok(refused),
                Ok(ArtifactStoreOneItemPreparationStep::Progress(checkpoint, ownership)) => Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership)),
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, ownership)) => {
                    publication.phase = ArtifactStoreOneItemPublicationPhase::PreparingCursor;
                    Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership))
                }
                Ok(ArtifactStoreOneItemPreparationStep::Blocked) => Ok(ArtifactStoreOneItemAdvance::Blocked),
            },
            ArtifactStoreOneItemPublicationPhase::PreparingCursor => {
                if publication.preparation.as_ref().and_then(|owner| owner.prepared()).is_none() {
                    return Err("transient prepared root is absent".into());
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
            }
            ArtifactStoreOneItemPublicationPhase::PreflightingCommit => {
                if self.generation == u64::MAX || publication.footprint.work_items == 0 {
                    return Err("transient publication fixed commit authority is exhausted".into());
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::Publishing;
                Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
            }
            ArtifactStoreOneItemPublicationPhase::Publishing => {
                let prepared = publication.preparation.as_mut().and_then(|owner| owner.take_prepared()).ok_or_else(|| "transient publication lost its prepared root".to_string())?;
                let generation_before = self.generation;
                let previous = self.current.replace(prepared.next_root).expect("live transient publication root remains");
                *publication.displaced_root_retirement = Some(if let Some(factory) = publication.owned_retirement_factory.as_ref() {
                    Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
                } else {
                    ArtifactStoreDisplacedSnapshotRetirement::boxed(previous, Arc::clone(publication.root_retirement_factory.as_ref().ok_or_else(|| "transient publication lost its local-root retirement factory".to_string())?))
                });
                self.generation += 1;
                let receipt = LaneItemReceipt { generation_before, generation_after: self.generation };
                publication.expected_generation = self.generation;
                *publication.receipt = Some(receipt);
                publication.published = true;
                publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                Ok(ArtifactStoreOneItemAdvance::Published(receipt, semio_framework_value::RetainedCloneProgress::default()))
            }
            ArtifactStoreOneItemPublicationPhase::Fault => Ok(ArtifactStoreOneItemAdvance::PreparationRefused),
            ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete => unreachable!("handled transient publication control phase"),
        }
    }

    pub fn cancel_publish_one(&mut self, publication: &mut ArtifactEphemeralOneItemPublication<P, Mutation>) -> bool {
        if matches!(publication.phase, ArtifactStoreOneItemPublicationPhase::Complete | ArtifactStoreOneItemPublicationPhase::Closing) {
            return false;
        }
        publication.cancel_requested = true;
        publication.begin_close();
        true
    }

    /// 🎯️ Applies one retained local-only item against an exact live generation.
    pub fn apply_one(&mut self, expected_generation: u64, mutation: Mutation) -> Result<LaneItemReceipt, (String, Mutation)> {
        if self.close_started || self.generation != expected_generation {
            return Err(("transient publication generation is stale".into(), mutation));
        }
        let before = self.generation;
        let candidate = match apply_operation::<P, Mutation>(self.current.as_ref().expect("live transient root remains").as_ref(), &mutation, 0).0 {
            Ok(candidate) => candidate,
            Err(error) => return Err((error.to_string(), mutation)),
        };
        self.current = Some(Arc::new(candidate));
        self.generation = self.generation.wrapping_add(1);
        Ok(LaneItemReceipt { generation_before: before, generation_after: self.generation })
    }

    /// 🔄️ Discards everything — what a host does when a view closes.
    pub async fn reset(&mut self, current: P) {
        assert!(!self.close_started, "closed transient store cannot be reset");
        self.current = Some(Arc::new(current));
        self.generation = self.generation.wrapping_add(1);
    }

    /// 🔢️ Bumps on every change, so a renderer can skip untouched frames.
    pub async fn generation(&self) -> u64 {
        self.generation
    }

    /// 🪪️ Reads the event-maintained transient generation without suspension.
    pub fn generation_now(&self) -> u64 {
        self.generation
    }

    /// 📏️ Borrows the next returned read's actual construction or physical retirement demand.
    pub fn maintenance_returned_reads_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static {
        if let Some(owner) = self.active_returned_read.as_ref() { return artifact_retirement_box_demands(owner, body); }
        if self.reads.is_none() { return Ok(Default::default()); }
        if self.reads.as_ref().expect("live transient read registry remains").has_returned() { return self.reads.as_ref().expect("live transient read registry remains").returned_admission_demands::<P>(|_| semio_framework_value::RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error); }
        Ok(Default::default())
    }

    /// 🏁️ Moves every original root and tracked read into close custody without a replacement allocation.
    pub fn begin_close_retirement(&mut self, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> TransientStoreRetirement<P>
    where P: Send + Sync + 'static {
        self.close_started = true;
        TransientStoreRetirement { root: std::mem::ManuallyDrop::new(self.current.take()), reads: std::mem::ManuallyDrop::new(self.reads.take().map(SnapshotReadRegistryAliasRetirement::new)), active_returned: std::mem::ManuallyDrop::new(self.active_returned_read.take()), active_root: std::mem::ManuallyDrop::new(None), factory: Some(factory), factory_close: None }
    }

    /// 🫙️ The live shell owns no root, tracked registry or returned read after consuming close.
    pub fn close_terminal_is_empty(&self) -> bool { self.current.is_none() && self.reads.is_none() && self.active_returned_read.is_none() }

    /// ♻️ Detaches this store's exact root/read registry and installs a fresh terminal root.
    pub fn begin_retirement(&mut self, terminal: P, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> TransientStoreRetirement<P>
    where
        P: Send + Sync + 'static,
    {
        let next_generation = self.generation.wrapping_add(1);
        let displaced = std::mem::replace(self, Self::new(terminal));
        self.generation = next_generation;
        TransientStoreRetirement::new(displaced, factory)
    }

    /// 🧾️ Closed stores retain no substitute root, read registry or returned cursor.
    pub fn detached_terminal_is_empty(&self) -> bool { self.close_started && self.current.is_none() && self.reads.is_none() && self.active_returned_read.is_none() }

}

/// ♻️ Detached transient root plus every returned/live tracked read.
pub struct TransientStoreRetirement<P> {
    pub(super) root: std::mem::ManuallyDrop<Option<Arc<P>>>,
    pub(super) reads: std::mem::ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,
    pub(super) active_returned: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    active_root: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(super) factory: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>,
    pub(super) factory_close: Option<semio_framework_value::FactoryAuthority>,
}

impl<P> TransientStoreRetirement<P> {
    pub(super) fn new<Mutation>(store: TransientStore<P, Mutation>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Self {
        let store = std::mem::ManuallyDrop::new(store);
        Self { root: std::mem::ManuallyDrop::new(unsafe { std::ptr::read(&store.current) }), reads: std::mem::ManuallyDrop::new(unsafe { std::ptr::read(&store.reads) }.map(SnapshotReadRegistryAliasRetirement::new)), active_returned: std::mem::ManuallyDrop::new(unsafe { std::ptr::read(&store.active_returned_read) }), active_root: std::mem::ManuallyDrop::new(None), factory: Some(factory), factory_close: None }
    }
    pub fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.reads.is_none() && self.active_returned.is_none() && self.active_root.is_none() && self.factory.is_none() && self.factory_close.is_none() }
    /// 🔒️ The retirement can only continue once the live tracked read that holds the registry is returned.
    pub fn awaits_live_read(&self) -> bool { self.root.is_none() && self.active_returned.is_none() && self.active_root.is_none() && self.reads.as_ref().is_some_and(|reads| !reads.has_returned() && !reads.terminal_is_empty()) }
}

impl<P: Send + Sync + 'static> TransientStoreRetirement<P> {
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "transient retirement depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active_returned.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if let Some(reads) = self.reads.as_ref().filter(|reads| reads.has_returned()) { return nested(reads.returned_admission_demands::<P>(|_| RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error)?); }
        if let Some(active) = self.active_root.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if self.root.is_some() { return Ok(RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 2, ..Default::default() }); }
        if self.reads.is_some() { return snapshot_registry_alias_demands(&self.reads); }
        if self.factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.factory_close.as_ref().map_or(Ok(Default::default()), |factory| nested(factory.demands(body)?))
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "transient retirement exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(reads) = self.reads.as_ref() { if self.active_returned.is_some() || reads.has_returned() { return advance_returned_snapshot_read(reads, &mut self.active_returned, self.factory.as_ref().expect("retained transient factory"), child).map(|step| RetainedCloneStep::Progress(step.progress())); } }
        if self.active_root.is_some() { return artifact_retirement_box_close_step(&mut self.active_root, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(root) = self.root.take() {
            return match ReturnedSnapshotReadRetirement::admit(root, self.factory.as_ref().expect("retained transient factory").clone(), child) { Ok((owner, receipt)) => { *self.active_root = Some(owner); if !receipt.fits(child) || receipt.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "transient constructor changed admitted receipt")); } Ok(RetainedCloneStep::Progress(receipt)) }, Err((error, root, alias)) => { *self.root = Some(root); drop(alias); Err(error) } };
        }
        if let Some(reads) = self.reads.as_ref() { if !reads.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(Default::default())); } return snapshot_registry_alias_close_step(&mut self.reads, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(factory) = self.factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        let factory = self.factory_close.as_mut().expect("retained transient factory ticket"); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "transient factory")?; if factory.terminal_is_empty() { self.factory_close = None; } Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
}

impl<P> Drop for TransientStoreRetirement<P> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "transient store retirement reached Drop before terminal-empty ownership"); if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.root); std::mem::ManuallyDrop::drop(&mut self.reads); std::mem::ManuallyDrop::drop(&mut self.active_returned); std::mem::ManuallyDrop::drop(&mut self.active_root); } } }
}

//#region 🔖️InteractionStore
/// 🕹️ Ephemeral LOCAL-ONLY lane for the framework interaction mechanism's hover half — the
/// `PresenceStore`/`TransientStore` sibling for `InteractionState.hover`.
///
/// 🎯️ Only hover lives here. Selection + `active_mode`/`active_granularity` are
/// **persisted-local** (`InteractionState`'s other three fields) and already flow through the
/// ordinary `ConfigStore` (`ArtifactStore<Config, ConfigMutation>`) via `ArtifactCommand::
/// ApplyInLane { lane: HistoryLane::Interaction, .. }` — no new store type needed for that half,
/// it is the ENTIRE point of the `HistoryLane` mechanism above. Hover must never take that path
/// (never persisted, never in `Edit`/`Change` history), so it gets this dedicated ephemeral
/// sibling instead, shaped exactly like `TransientStore` (single local value, no peer roster —
/// peers' hover arrives over the wire as `PresencePeer.interaction`, not through this store).
///
/// `S`/`Mutation` are supplied by the caller (mirrors `PresenceStore<A::Presence,
/// A::PresenceMutation>` in `VcsArtifactApp`): this crate has no dependency edge to the
/// `semio-framework` crate that owns `InteractionState`/`DomainHover` (that crate depends on
/// THIS one, for its `spr`/`dsl`/`pack` types — the reverse edge would cycle), so `S` stays
/// generic here and gets instantiated with the app's concrete hover-shaped type one layer up,
/// where that dependency edge actually exists.
#[derive(Clone, Debug)]
pub struct InteractionStore<S, Mutation> {
    hover: S,
    pub(super) generation: u64,
    pub(super) _mutation: PhantomData<fn() -> Mutation>,
}

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<S: Clone + Default, Mutation: super::Mutation<S> + protocol::ColdMutation<S>> Default for InteractionStore<S, Mutation> {
    fn default() -> Self {
        Self::new(S::default())
    }
}

impl<S: Clone, Mutation: super::Mutation<S> + protocol::ColdMutation<S>> InteractionStore<S, Mutation> {
    /// 🏗️ Starts with this actor's own initial hover state (typically empty).
    /// 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(hover: S) -> Self {
        Self { hover, generation: 0, _mutation: PhantomData }
    }

    /// 👁️ This actor's own hover — what the presence heartbeat mirrors onto `PresenceDomain.hovered`.
    pub async fn hover(&self) -> &S {
        &self.hover
    }

    /// ✍️ Applies hover updates atomically. The caller's `Mutation` impl is expected to route
    /// through the framework's `next_hover` pure fn before landing here.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> protocol::MutationApplyResult<()> {
        let mut candidate = self.hover.clone();
        for mutation in mutations {
            candidate = apply_operation::<S, Mutation>(&candidate, mutation, 0).0?;
        }
        if !mutations.is_empty() {
            self.hover = candidate;
            self.generation = self.generation.wrapping_add(1);
        }
        Ok(())
    }

    /// 🔄️ Discards hover — a host clears this when a view/window closes (nothing left to hover).
    pub async fn reset(&mut self, hover: S) {
        self.hover = hover;
        self.generation = self.generation.wrapping_add(1);
    }

    /// 🔢️ Bumps on every local hover change; drives the same broadcast-coalescing signal as
    /// `PresenceStore::generation`.
    pub async fn generation(&self) -> u64 {
        self.generation
    }
}
//#endregion 🔖️InteractionStore
