//! 📤️ Ephemeral one item publication: the prepared publication, the base read and owner, the preparation request, trait and factory, the admission rejection and the ephemeral one item publication. See [`super`].

use super::*;

//#region 📬️EphemeralOneItemPublication
pub struct ArtifactEphemeralOneItemPrepared<P> {
    pub next_root: Arc<P>,
}

pub struct ArtifactEphemeralBaseRead<P>(pub ArtifactEphemeralBaseOwner<P>);

/// 🫧️ Which owner a preparation's base came from. Public so an app can build the exact request its
/// own registered ephemeral preparation factory receives, and drive it directly in a conformance
/// test, without standing up the whole presence/transient store around it.
pub enum ArtifactEphemeralBaseOwner<P> {
    Presence(SnapshotRead<P>),
    Transient(Arc<P>),
    TransientRead(SnapshotRead<P>),
}

impl<P> AsRef<P> for ArtifactEphemeralBaseRead<P> {
    fn as_ref(&self) -> &P {
        match &self.0 {
            ArtifactEphemeralBaseOwner::Presence(read) => read.get(),
            ArtifactEphemeralBaseOwner::Transient(root) => root.as_ref(),
            ArtifactEphemeralBaseOwner::TransientRead(read) => read.get(),
        }
    }
}

impl<P> std::ops::Deref for ArtifactEphemeralBaseRead<P> {
    type Target = P;
    fn deref(&self) -> &P {
        self.as_ref()
    }
}

pub struct ArtifactEphemeralOneItemPreparationRequest<P, Mutation> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub base: ArtifactEphemeralBaseRead<P>,
    pub mutation: Mutation,
}

pub trait ArtifactEphemeralOneItemPreparation<P, Mutation>: Send {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError>;
    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint;
    fn prepared(&self) -> Option<&ArtifactEphemeralOneItemPrepared<P>>;
    fn take_prepared(&mut self) -> Option<ArtifactEphemeralOneItemPrepared<P>>;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub trait ArtifactEphemeralOneItemPreparationFactory<P, Mutation>: Send + Sync + semio_framework_value::FactoryRetirement {
    fn preflight(&self, mutation: &Mutation) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin(&self, request: ArtifactEphemeralOneItemPreparationRequest<P, Mutation>) -> Result<Box<dyn ArtifactEphemeralOneItemPreparation<P, Mutation>>, ArtifactEphemeralOneItemPreparationRequest<P, Mutation>>;
}

#[derive(Debug)]
pub struct ArtifactEphemeralOneItemAdmissionRejected<Mutation> {
    pub reason: String,
    pub mutation: Mutation,
}

impl<Mutation> ArtifactEphemeralOneItemAdmissionRejected<Mutation> {
    pub fn into_owners(self) -> (String, Mutation) {
        (self.reason, self.mutation)
    }
}

pub struct ArtifactEphemeralOneItemPublication<P, Mutation> {
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) expected_generation: u64,
    pub(super) footprint: ArtifactStoreOneItemFootprint,
    pub(super) preparation: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEphemeralOneItemPreparation<P, Mutation>>>>,
    pub(super) displaced_root_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(super) root_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    pub(super) returned_read_registry: std::mem::ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,
    pub(super) returned_read_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(super) owned_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub(super) receipt: std::mem::ManuallyDrop<Option<LaneItemReceipt>>,
    /// 📍️ Last checkpoint the preparation owner reported, kept so progress survives the turn the
    /// owner is released on: a host polling `progress()` must never see a gesture walk backwards.
    pub(super) retained_checkpoint: ArtifactStoreOneItemCheckpoint,
    pub(super) factory_retirement: [Option<semio_framework_value::FactoryAuthority>; 2],
    pub(super) attempts: u8,
    pub(super) published: bool,
    pub(super) cancel_requested: bool,
    pub(super) close_started: bool,
    pub(super) fault: std::mem::ManuallyDrop<Option<ValueError>>,
    pub(super) phase: ArtifactStoreOneItemPublicationPhase,
}

impl<P, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation> {
    /// 🧫️ Borrows the exact retained preparation for native custody laws.
    #[cfg(any(test,feature="testkit"))]
    pub fn original_preparation(&self) -> Option<&dyn ArtifactEphemeralOneItemPreparation<P, Mutation>> { self.preparation.as_deref() }

    /// 🧪️ Builds a publication already closing around one preparation so native custody laws can walk its retirement demands.
    #[cfg(any(test,feature="testkit"))]
    pub fn closing_with_preparation(operation: semio_framework_job::OperationId, preparation: Box<dyn ArtifactEphemeralOneItemPreparation<P, Mutation>>) -> Self {
        use std::mem::ManuallyDrop as Held;
        Self {
            operation, expected_generation: 0,
            footprint: ArtifactStoreOneItemFootprint::for_ephemeral_item(0),
            preparation: Held::new(Some(preparation)),
            displaced_root_retirement: Held::new(None), root_retirement_factory: Held::new(None),
            returned_read_registry: Held::new(None), returned_read_retirement: Held::new(None),
            owned_retirement_factory: Held::new(None), receipt: Held::new(None),
            retained_checkpoint: Default::default(), factory_retirement: [None, None],
            attempts: 0, published: false, cancel_requested: false, close_started: true,
            fault: Held::new(None), phase: ArtifactStoreOneItemPublicationPhase::Closing,
        }
    }

    pub fn operation(&self) -> semio_framework_job::OperationId {
        self.operation
    }

    pub fn phase(&self) -> ArtifactStoreOneItemPublicationPhase {
        self.phase
    }

    /// 📍️ Monotone gesture progress: the live preparation's own checkpoint while it owns one, and the
    /// last checkpoint it reported once it has been released. Answering `default()` (all zeros) the
    /// moment the owner goes away made a batch's progress snap back to 0 between items — measured on
    /// `🌊️flow` as `preparation progress changed from 22013 to 0` (ticket 26/09/19 `📓️flow.md` §5.4).
    pub fn progress(&self) -> ArtifactStoreOneItemCheckpoint {
        self.preparation.as_ref().map_or(self.retained_checkpoint, |owner| owner.checkpoint())
    }

    pub fn fault(&self) -> Option<&str> { self.fault.as_ref().map(|fault|fault.message.as_ref()) }

    /// 🚫️ Borrows the one original refusal and its actual consumed physical currencies.
    pub fn preparation_refusal(&self) -> Option<&ValueError> { self.fault.as_ref() }

    pub(super) fn advance_preparation(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ArtifactStoreOneItemAdvance> {
        let result = self.preparation.as_mut().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"ephemeral publication lost its preparation owner")).and_then(|owner| {
            let grant=ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant };
            let step=owner.advance(grant)?;
            validate_original_preparation_step(step,grant,owner.prepared().is_some())
        });
        match result {
            Ok(step)=>Ok(step),
            Err(reason)=>{*self.fault=Some(reason);self.begin_close();Err(ArtifactStoreOneItemAdvance::PreparationRefused)}
        }
    }

    pub fn retry(&mut self) -> bool {
        if self.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck || self.attempts >= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES {
            return false;
        }
        self.attempts += 1;
        true
    }

    pub fn acknowledge(&mut self) -> bool {
        if self.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck {
            return false;
        }
        self.phase = ArtifactStoreOneItemPublicationPhase::Closing;
        self.begin_close();
        true
    }

    pub fn begin_close(&mut self) {
        if self.close_started {
            return;
        }
        self.close_started = true;
        self.phase = if self.phase == ArtifactStoreOneItemPublicationPhase::Complete { ArtifactStoreOneItemPublicationPhase::Complete } else { ArtifactStoreOneItemPublicationPhase::Closing };
        if let Some(owner) = self.preparation.as_mut() {
            if !self.published {
                self.cancel_requested = true;
                owner.cancel();
            }
            owner.begin_close();
        }
    }

    /// 📐️ Borrows the original cleanup frontier without spending its physical authority.
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> where P: Send + Sync + 'static {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "ephemeral cleanup depth overflow"))?; Ok(demand) };
        if let Some(owner) = self.preparation.as_ref() {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
            return nested(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(body)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? });
        }
        if let Some(owner) = self.returned_read_retirement.as_ref() { return nested(artifact_retirement_box_demands(owner, body)?); }
        if let Some(registry) = self.returned_read_registry.as_ref().filter(|registry| registry.has_returned()) { return nested(registry.returned_admission_demands::<P>(|_| RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error)?); }
        if let Some(owner) = self.displaced_root_retirement.as_ref() { return nested(artifact_retirement_box_demands(owner, body)?); }
        if self.returned_read_registry.is_some() { return snapshot_registry_alias_demands(&self.returned_read_registry); }
        if let Some(factory) = self.factory_retirement.iter().flatten().next() { return nested(factory.demands(body)?); }
        if self.root_retirement_factory.is_some() || self.owned_retirement_factory.is_some() || self.receipt.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(fault) = self.fault.as_ref() { let release_bytes=match &fault.message{std::borrow::Cow::Borrowed(_)=>0,std::borrow::Cow::Owned(text)=>text.capacity()};return Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() }); }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    /// ♻️ Closes one original child or its separately admitted physical frame.
    pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> where P: Send + Sync + 'static {
        let grant = grant.retained_grant();let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "ephemeral cleanup exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.begin_close();
        if let Some(owner) = self.preparation.as_mut() {
            if owner.terminal_is_empty() { self.retained_checkpoint = owner.checkpoint();let bytes = size_of_val(owner.as_ref());drop(self.preparation.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty })); }
            let child = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_copy_bytes, maximum_capacity_bytes: grant.maximum_capacity_bytes, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth - 1 };
            let step = owner.close_step(child)?;let step = semio_framework_value::retained_clone::admit_retained_clone_close(child.retained_grant(), step, owner.terminal_is_empty(), "ephemeral original preparation")?;return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.returned_read_retirement.is_some() || self.returned_read_registry.as_ref().is_some_and(|registry| registry.has_returned()) {
            let registry = self.returned_read_registry.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "returned ephemeral child lost its registry"))?;let factory = self.owned_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "returned ephemeral child lost its issuer"))?;
            return advance_returned_snapshot_read(registry, &mut self.returned_read_retirement, factory, RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant }).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.displaced_root_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.displaced_root_retirement, RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant }).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.returned_read_registry.is_some() { return snapshot_registry_alias_close_step(&mut self.returned_read_registry, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(index) = self.factory_retirement.iter().position(Option::is_some) {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };let factory = self.factory_retirement[index].as_mut().expect("observed original factory");let step = factory.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "ephemeral original factory")?;if factory.terminal_is_empty() { self.factory_retirement[index] = None; }return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(factory) = self.root_retirement_factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;self.factory_retirement[0] = Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if let Some(factory) = self.owned_retirement_factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;self.factory_retirement[1] = Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if self.receipt.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if let Some(fault) = self.fault.take() { drop(fault);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
        self.phase = ArtifactStoreOneItemPublicationPhase::Complete;Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == ArtifactStoreOneItemPublicationPhase::Complete
            && self.preparation.is_none()
            && self.displaced_root_retirement.is_none()
            && self.root_retirement_factory.is_none()
            && self.returned_read_registry.is_none()
            && self.returned_read_retirement.is_none()
            && self.owned_retirement_factory.is_none()
            && self.receipt.is_none()
            && self.fault.is_none()
            && self.factory_retirement.iter().all(Option::is_none)
    }
}

impl<P, Mutation> Drop for ArtifactEphemeralOneItemPublication<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "ephemeral one-item publication reached Drop without its terminal-empty witness");
    }
}
//#endregion 📬️EphemeralOneItemPublication
