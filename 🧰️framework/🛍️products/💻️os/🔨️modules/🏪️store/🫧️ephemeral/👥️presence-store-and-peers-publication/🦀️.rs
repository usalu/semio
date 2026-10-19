//! 👥️ Presence store and peers publication: the presence store, the peers publication and commit, the presence retirement default and the presence store operations. See [`super`].

use super::*;

/// 👥️ The PRESENCE lane's store: ephemeral SHARED state — a last-writer-wins roster, NOT an
/// event log.
///
/// 🎯️ Why not `ArtifactStore` (which `ConfigStore`/`DraftStore` both alias): presence has no
/// history, no undo, no checkpoints and no merge. Each peer is the sole author of its own presence
/// value, and the wire already carries whole encoded snapshots per peer
/// (`ClientFrame::Presence { peer }` / `ServerFrame::Presence { peers }`), so a later frame from a
/// peer simply SUPERSEDES its earlier one. Modelling that as an op log would mint an unbounded
/// history of cursor positions nobody can ever undo.
///
/// `generation` bumps on every local change so the host can tell "something to broadcast" from
/// "nothing changed" without diffing snapshots — the signal a heartbeat coalescer throttles on.
pub struct PresenceStore<P, Mutation> {
    pub(super) local: std::mem::ManuallyDrop<Option<Arc<P>>>,
    pub(super) peers: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    pub(super) peer_retirement_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    pub(super) local_retirement_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    pub(super) local_reads: std::mem::ManuallyDrop<Option<SnapshotReadRegistryHandle>>,
    pub(super) active_returned_local: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(super) generation: u64,
    pub(super) close_started: bool,
    pub(super) _mutation: PhantomData<fn() -> Mutation>,
}

#[cfg(any(test, feature = "testkit"))]
impl<P, Mutation> PresenceStore<P, Mutation> {
    /// 🪝️ Test hook: the live local presence root, absent once its owner is closed.
    #[doc(hidden)]
    pub fn local_root(&self) -> Option<&Arc<P>> {
        self.local.as_ref()
    }
}

pub struct PresencePeersPublication<P> {
    pub(super) base_root: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    pub(super) candidate: std::mem::ManuallyDrop<Option<PresencePeersRoot<P>>>,
    pub(super) created: std::mem::ManuallyDrop<PresencePeersRetiredEntries<P>>,
    pub(super) retired: std::mem::ManuallyDrop<PresencePeersRetiredEntries<P>>,
    pub(super) active: std::mem::ManuallyDrop<Option<PresencePeersRetirement<P>>>,
    pub(super) factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    pub(super) factory_close: Option<semio_framework_value::FactoryAuthority>,
    pub(super) prune_cursor: usize,
    pub(super) pruning_complete: bool,
}

pub struct PresencePeersCommit<P> {
    pub(super) base_root: std::mem::ManuallyDrop<Arc<PresencePeersRoot<P>>>,
    pub(super) root: std::mem::ManuallyDrop<Arc<PresencePeersRoot<P>>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<PresencePeersRetirement<P>>>,
    pub(super) factory: std::mem::ManuallyDrop<Arc<dyn SnapshotRetirementFactory<P>>>,
    pub(super) transferred: bool,
}

impl<P> PresencePeersCommit<P> {
    pub(super) fn into_parts(mut self) -> (Arc<PresencePeersRoot<P>>, Arc<PresencePeersRoot<P>>, Option<PresencePeersRetirement<P>>, Arc<dyn SnapshotRetirementFactory<P>>) { self.transferred = true; unsafe { (std::mem::ManuallyDrop::take(&mut self.base_root), std::mem::ManuallyDrop::take(&mut self.root), std::mem::ManuallyDrop::take(&mut self.retirement), std::mem::ManuallyDrop::take(&mut self.factory)) } }
}

impl<P> Drop for PresencePeersCommit<P> { fn drop(&mut self) { assert!(std::thread::panicking() || self.transferred, "original peer commit requires consuming admitted publication or retirement"); } }

impl<P: Send + Sync + 'static> PresencePeersPublication<P> {
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() {
            let mut demand = active.retirement_demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence peer publication depth overflow"))?;
            return Ok(demand);
        }
        if self.candidate.is_none() && self.created.len == 0 && self.retired.len == 0 && self.base_root.is_none() {
            if self.factory.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
            if let Some(factory) = self.factory_close.as_ref() { let mut demand = factory.demands(body)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "peer publication factory depth overflow"))?; return Ok(demand); }
        }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence peer publication exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let moved = RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty });
        if let Some(active) = self.active.as_mut() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = active.close_step(child)?;
            let step = admit_retained_clone_close(child, step, active.terminal_is_empty(), "presence peer publication cleanup")?;
            if active.terminal_is_empty() { drop(self.active.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(candidate) = self.candidate.as_mut() {
            if candidate.len != 0 {
                candidate.len -= 1;
                *self.active = Some(PresencePeersRetirement::new(PresencePeersRetiredEntries::one(candidate.entries[candidate.len].take()), self.factory.as_ref().expect("retained original peer factory").clone()));
            } else {
                drop(self.candidate.take());
            }
            return Ok(moved);
        }
        if self.retired.len != 0 {
            self.retired.len -= 1;
            let index = self.retired.len;
            *self.active = Some(PresencePeersRetirement::new(PresencePeersRetiredEntries::one(self.retired.entries[index].take()), self.factory.as_ref().expect("retained original peer factory").clone()));
            return Ok(moved);
        }
        if self.created.len != 0 {
            let created = std::mem::replace(&mut *self.created, PresencePeersRetiredEntries::empty());
            *self.active = Some(PresencePeersRetirement::new(created, self.factory.as_ref().expect("retained original peer factory").clone()));
            return Ok(moved);
        }
        if let Some(root) = self.base_root.take() {
            if Arc::strong_count(&root) == 1 { *self.active = Some(PresencePeersRetirement::from_root(root, Some(self.factory.as_ref().expect("retained original peer factory").clone()), PresencePeerRootCustody::BaseAlias)); } else { drop(root); }
            return Ok(moved);
        }
        if let Some(factory) = self.factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(factory) = self.factory_close.as_mut() { let step = factory.step(child)?; let step = admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original peer publication factory")?; if factory.terminal_is_empty() { self.factory_close = None; } return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }); }
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.base_root.is_none() && self.candidate.is_none() && self.created.len == 0 && self.retired.len == 0 && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none()
    }
}

impl<P> Drop for PresencePeersPublication<P> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(
                std::thread::panicking() || (self.base_root.is_none() && self.candidate.is_none() && self.created.len == 0 && self.retired.len == 0 && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none()),
                "incomplete presence peer publication reached Drop without bounded disposal"
            );
        }
    }
}

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<P: Clone + Default, Mutation: super::Mutation<P>> Default for PresenceStore<P, Mutation> {
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P: Clone, Mutation: super::Mutation<P>> PresenceStore<P, Mutation> {
    /// 🏗️ A roster holding only this actor's own initial presence.
    /// 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(local: P) -> Self {
        Self {
            local: std::mem::ManuallyDrop::new(Some(Arc::new(local))),
            peers: std::mem::ManuallyDrop::new(Some(Arc::new(PresencePeersRoot::empty()))),
            peer_retirement_factory: None,
            local_retirement_factory: None,
            local_reads: std::mem::ManuallyDrop::new(Some(SnapshotReadRegistryHandle::new())),
            active_returned_local: std::mem::ManuallyDrop::new(None),
            generation: 0,
            close_started: false,
            _mutation: PhantomData,
        }
    }

    /// 👤️ This actor's own presence — what gets broadcast.
    pub fn local(&self) -> &P {
        self.local.as_ref().expect("live local presence owner").as_ref()
    }

    /// 🧵️ Captures the event-maintained local presence root without cloning its payload.
    pub fn local_read(&self) -> Result<SnapshotRead<P>, String>
    where
        P: Send + Sync + 'static,
    {
        if self.close_started || self.local_retirement_factory.is_none() {
            return Err("presence local read requires a live exact local retirement owner".into());
        }
        let owner = Arc::clone(self.local.as_ref().expect("live local presence owner"));
        let lease = self.local_reads.as_ref().expect("live presence read registry").try_issue(owner.clone()).map_err(|_| "presence local read registry is busy or exhausted".to_string())?;
        Ok(SnapshotRead::new(owner, lease))
    }

    pub fn install_local_retirement_factory(&mut self, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Result<(), String> {
        if self.close_started || self.local_retirement_factory.is_some() {
            return Err("presence local retirement factory is already installed".into());
        }
        self.local_retirement_factory = Some(factory);
        Ok(())
    }

    pub fn maintenance_local_reads_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Send + Sync + 'static,
    {
        match self.local_reads.as_ref() { Some(reads) => presence_retirement::advance_returned_local(reads, &mut self.active_returned_local, self.local_retirement_factory.as_ref(), grant), None => Ok(RetainedCloneStep::Complete(Default::default())) }
    }
    /// 📏️ Quotes the next returned local-read maintenance turn on every axis without advancing it.
    pub fn maintenance_local_reads_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static,
    {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence local maintenance depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active_returned_local.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if let Some(reads) = self.local_reads.as_ref().filter(|reads| reads.has_returned()) {
            let factory = self.local_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence returned read lacks its original factory"))?;
            return nested(reads.returned_admission_demands::<P>(|root| RetirementDemand { capacity_bytes: factory.retirement_birth_bytes(root), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error)?);
        }
        Ok(Default::default())
    }
    pub fn local_read_maintenance_is_idle(&self) -> bool { self.active_returned_local.is_none() && self.local_reads.as_ref().is_none_or(|reads| !reads.has_returned()) }

    /// ✍️ Applies local presence operations atomically, bumping `generation` after the full batch.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> protocol::MutationApplyResult<()>
    where
        P: Send + Sync + 'static,
    {
        if self.close_started {
            return Err(protocol::MutationApplyError::new("mutation.apply.store-closed", "presence store no longer admits local mutations"));
        }
        if mutations.is_empty() {
            return Ok(());
        }
        let previous = self.local_read().map_err(|reason| protocol::MutationApplyError::new("mutation.apply.local-owner", reason))?;
        let mut candidate = self.local.as_ref().expect("live local presence owner").as_ref().clone();
        for mutation in mutations {
            candidate = apply_operation::<P, Mutation>(&candidate, mutation, 0).0?;
        }
        if !mutations.is_empty() {
            *self.local = Some(Arc::new(candidate));
            self.generation = self.generation.wrapping_add(1);
        }
        previous.return_to_registry();
        Ok(())
    }

    pub fn begin_publish_one(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        mutation: Mutation,
        factory: Option<&dyn ArtifactEphemeralOneItemPreparationFactory<P, Mutation>>,
        root_retirement_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    ) -> Result<ArtifactEphemeralOneItemPublication<P, Mutation>, ArtifactEphemeralOneItemAdmissionRejected<Mutation>>
    where
        P: Send + Sync + 'static,
    {
        let reject = |reason: String, mutation: Mutation| ArtifactEphemeralOneItemAdmissionRejected { reason, mutation };
        if self.close_started {
            return Err(reject("presence store no longer admits local publication".into(), mutation));
        }
        if self.generation != expected_generation {
            return Err(reject("presence publication generation is stale".into(), mutation));
        }
        let Some(factory) = factory else {
            return Err(reject("presence publication requires an explicit app-owned retained preparation factory".into(), mutation));
        };
        let Some(root_retirement_factory) = root_retirement_factory else {
            return Err(reject("presence publication requires an explicit local-root retirement factory".into(), mutation));
        };
        let footprint = match factory.preflight(&mutation) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            Ok(_) => return Err(reject("presence publication footprint exceeds its fixed item or byte capacity".into(), mutation)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        if !self.local_retirement_factory.as_ref().is_some_and(|installed| Arc::ptr_eq(installed, &root_retirement_factory)) {
            return Err(reject("presence publication requires its exact installed local-root retirement factory".into(), mutation));
        }
        let base = match self.local_read() {
            Ok(read) => ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::Presence(read)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        let request = ArtifactEphemeralOneItemPreparationRequest { operation, generation: semio_framework_job::Generation(expected_generation), base, mutation };
        let preparation = match factory.begin(request) {
            Ok(preparation) => preparation,
            Err(request) => return Err(reject("presence preparation factory rejected its exact owner bundle".into(), request.mutation)),
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

    pub fn advance_publish_one(&mut self, publication: &mut ArtifactEphemeralOneItemPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String>
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
            return Ok(ArtifactStoreOneItemAdvance::AwaitingAck(publication.receipt.expect("presence publication awaiting ACK retains its receipt")));
        }
        if self.close_started {
            publication.begin_close();
            return Err("presence store closed before publication".into());
        }
        if publication.cancel_requested {
            publication.begin_close();
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if self.generation != publication.expected_generation {
            *publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"presence publication became stale before commit"));
            publication.phase = ArtifactStoreOneItemPublicationPhase::Fault;
            publication.begin_close();
            return Err("presence publication became stale before commit".into());
        }
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        match publication.phase {
            ArtifactStoreOneItemPublicationPhase::Preparing => match publication.advance_preparation(grant) {
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
                    return Err("presence prepared root is absent".into());
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
            }
            ArtifactStoreOneItemPublicationPhase::PreflightingCommit => {
                if self.generation == u64::MAX || publication.footprint.work_items == 0 {
                    return Err("presence publication fixed commit authority is exhausted".into());
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::Publishing;
                Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
            }
            ArtifactStoreOneItemPublicationPhase::Publishing => {
                let prepared = publication.preparation.as_mut().and_then(|owner| owner.take_prepared()).ok_or_else(|| "presence publication lost its prepared root".to_string())?;
                let generation_before = self.generation;
                let previous = self.local.replace(prepared.next_root).expect("live original local presence");
                let retirement_factory = publication.root_retirement_factory.as_ref().ok_or_else(|| "presence publication lost its local-root retirement factory".to_string())?;
                *publication.displaced_root_retirement = Some(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous, Arc::clone(retirement_factory)));
                self.generation += 1;
                let receipt = LaneItemReceipt { generation_before, generation_after: self.generation };
                publication.expected_generation = self.generation;
                *publication.receipt = Some(receipt);
                publication.published = true;
                publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                Ok(ArtifactStoreOneItemAdvance::Published(receipt))
            }
            ArtifactStoreOneItemPublicationPhase::Fault => Ok(ArtifactStoreOneItemAdvance::PreparationRefused),
            ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete => unreachable!("handled presence publication control phase"),
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

    /// 🎯️ Applies one retained publication item against an exact generation and returns its
    /// per-lane receipt. A stale item is rejected before its mutation is inspected.
    pub fn apply_one(&mut self, expected_generation: u64, mutation: Mutation) -> Result<LaneItemReceipt, (String, Mutation)>
    where
        P: Send + Sync + 'static,
    {
        if self.close_started {
            return Err(("presence store no longer admits local mutations".into(), mutation));
        }
        if self.generation != expected_generation {
            return Err(("presence publication generation is stale".into(), mutation));
        }
        let before = self.generation;
        let previous = match self.local_read() {
            Ok(previous) => previous,
            Err(reason) => return Err((reason, mutation)),
        };
        let candidate = match apply_operation::<P, Mutation>(self.local.as_ref().expect("live local presence owner").as_ref(), &mutation, 0).0 {
            Ok(candidate) => candidate,
            Err(error) => return Err((error.to_string(), mutation)),
        };
        *self.local = Some(Arc::new(candidate));
        self.generation = self.generation.wrapping_add(1);
        previous.return_to_registry();
        Ok(LaneItemReceipt { generation_before: before, generation_after: self.generation })
    }

    pub fn peers_root(&self) -> Arc<PresencePeersRoot<P>> {
        Arc::clone(self.peers.as_ref().expect("live peer presence owner"))
    }

    pub fn install_peer_retirement_factory(&mut self, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Result<(), String> {
        if self.close_started || self.peer_retirement_factory.is_some() {
            return Err("presence peer retirement factory is already installed".into());
        }
        self.peer_retirement_factory = Some(factory);
        Ok(())
    }

    pub fn peer_retirement_available(&self) -> bool {
        self.peer_retirement_factory.is_some()
    }

    /// 🔗️ The installed peer-root retirement issuer, lent to a captured alias so its last holder can retire the roster it ends up owning.
    pub fn peer_retirement_factory(&self) -> Option<Arc<dyn SnapshotRetirementFactory<P>>> {
        self.peer_retirement_factory.clone()
    }

    pub fn begin_peer_retirement(&self, retired: PresencePeersRetiredEntries<P>) -> Result<PresencePeersRetirement<P>, PresencePeersRetiredEntries<P>>
    where
        P: Send + Sync + 'static,
    {
        let Some(factory) = self.peer_retirement_factory.as_ref() else { return Err(retired) };
        Ok(PresencePeersRetirement::new(retired, factory.clone()))
    }

    pub fn begin_peer_publication(&self, grant: RetainedCloneGrant) -> Result<(PresencePeersPublication<P>, RetainedCloneProgress), ValueError>
    where P: Send + Sync + 'static,
    {
        if self.close_started { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence store no longer admits peer publication")); }
        let factory = self.peer_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "presence publication has no original snapshot retirement factory"))?;
        PresencePeersPublication::new(self.peers.as_ref().expect("live original peer owner"), factory, grant)
    }

    pub fn publish_peer_commit(&mut self, commit: PresencePeersCommit<P>) -> Result<Option<PresencePeersRetirement<P>>, PresencePeersCommit<P>>
    where
        P: Send + Sync + 'static,
    {
        if self.close_started || !Arc::ptr_eq(self.peers.as_ref().expect("live peer presence owner"), &commit.base_root) || !self.peer_retirement_factory.as_ref().is_some_and(|factory| Arc::ptr_eq(factory, &commit.factory)) {
            return Err(commit);
        }
        let (base_root, root, retired, factory) = commit.into_parts();
        let previous = self.peers.replace(root).expect("live original peer presence");
        let mut retirement = retired.unwrap_or_else(|| PresencePeersRetirement::new(PresencePeersRetiredEntries::empty(), factory));
        *retirement.root = Some(previous);
        drop(base_root);
        Ok(Some(retirement))
    }

    /// 🔢️ Bumps on every local change; never on adopting a remote peer (that needs no rebroadcast).
    pub async fn generation(&self) -> u64 {
        self.generation
    }

    /// 🪪️ Reads the event-maintained generation at an already-owned operation boundary.
    pub fn generation_now(&self) -> u64 {
        self.generation
    }
}
