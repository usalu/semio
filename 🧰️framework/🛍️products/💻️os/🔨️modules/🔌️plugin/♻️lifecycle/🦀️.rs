//! ♻️ Store owners and disposers, store initialization and snapshot close retention, and the close, envelope and maintenance ladders.
use protocol::{OpBinary, OpText};
use semio_framework::{kernel::ActorId, Fault};
use pack::snapshot::ArtifactPack;
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore, ConfigStore, os_store::store_schema::HistoryLane, Mutation};
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use store::ArtifactStoreInitializationAuthority;
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::bounded_ownership::PluginLifecycleStep;
use crate::sdk_fault::plugin_sdk_fault;



/// 🧹️ Optional exact-owner disposer retained until the associated artifact value is empty.
pub type ArtifactDisposal<T> = Option<Box<dyn ArtifactOwnedDisposer<T>>>;

/// 🏗️ Admits initialization or returns the original document envelope.
pub type ArtifactInitializationAdmission<S, M> = Result<ArtifactStoreInitializationJob<S, M>, ArtifactEnvelope<S, M>>;

/// 📖️ Non-capturing provider of one document schema, its semantic mutation roster and, row for row, each
/// kind's leaf payload JSON Schema (`Mutation::INPUT_SCHEMAS`; shorter or empty when the aggregate publishes none).
pub type OwnerMutationRoster = fn() -> (&'static str, &'static [protocol::SemanticDescriptor], &'static [&'static str]);

pub trait ArtifactOwnedDisposer<T>: Send {
    fn retirement_demands(&self, owner: &T, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, owner: &mut T, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
    fn terminal_is_empty(&self, owner: &T) -> bool;
    /// 📐️ Declares the original Box extent after every retained child is physically empty.
    fn terminal_frame_release_bytes(&self) -> Option<usize> { None }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct BoundedConfigRetirementFactory<T>(std::marker::PhantomData<fn() -> T>);

impl<T> BoundedConfigRetirementFactory<T> {
    fn new() -> Self { Self(std::marker::PhantomData) }
}

impl<T: semio_framework_value::retirement::RetireOwned> store::ArtifactOwnedValueRetirementFactory<T> for BoundedConfigRetirementFactory<T> {
    fn retirement_birth_bytes(&self, _: &T) -> usize { semio_framework_value::retirement::owned_retirement_birth_bytes::<T>() }

    fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

impl<T: semio_framework_value::retirement::RetireOwned + Sync> store::SnapshotRetirementFactory<T> for BoundedConfigRetirementFactory<T> {
    fn retirement_birth_bytes(&self, _: &std::sync::Arc<T>) -> usize { semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<T>() }

    fn retire(&self, snapshot: std::sync::Arc<T>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, std::sync::Arc<T>)> {
        semio_framework_value::retirement::arc_custody::admit_shared_retirement(snapshot, grant, false)
    }
}

#[cfg(test)]
#[test]
fn original_bounded_config_factory_retains_source_until_exact_physical_retirement() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🏪️store/♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["boundedConfigFactory"];
    let mut source = law["body"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize);
    let pointer = source.as_ptr();
    let source_bytes = source.capacity();
    let factory = BoundedConfigRetirementFactory::<String>::new();
    let frame = store::ArtifactOwnedValueRetirementFactory::retirement_birth_bytes(&factory, &source);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: law["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: frame, maximum_release_bytes: 0, maximum_depth: 1 };
    for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: frame - 1, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| store::ArtifactOwnedValueRetirementFactory::retire_owned(&factory, source, denied));
        source = match result { Err((_, original)) => original, Ok(_) => panic!("original config factory admitted an unfunded constructor") };
        assert_eq!(source.as_ptr(), pointer);
        assert_eq!(source.capacity(), source_bytes);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| store::ArtifactOwnedValueRetirementFactory::retire_owned(&factory, source, grant));
    let (owner, progress) = result.unwrap_or_else(|_| panic!("original config typed constructor refused its exact grant"));
    assert!(progress.fits(grant));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
    assert_eq!(heap.requested_bytes, frame);
    let mut owner = Some(owner);
    let mut born = progress.retained_capacity_bytes;
    let mut released = progress.released_bytes;
    let mut turns = 0;
    while let Some(current) = owner.as_ref() {
        let copy = current.next_copy_byte_demand().unwrap().max(grant.maximum_copy_bytes);
        let demand = store::os_store::snapshot_read::artifact_retirement_box_demands(current, copy).unwrap();
        let turn = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        for denied in [Some(RetainedCloneGrant { maximum_items: 0, ..turn }), (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..turn })].into_iter().flatten() {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut owner, denied));
            assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut owner, turn));
        let progress = step.unwrap().progress();
        assert!(progress.fits(turn));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        born += heap.requested_bytes;
        released += heap.released_bytes;
        turns += 1;
        assert!(turns <= source_bytes * 4 + 512);
    }
    assert_eq!(released, source_bytes + born);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
}

/// 🎛️ Exact one-page retirement catalog for explicitly bounded app configuration stores.
pub fn bounded_config_store_owners<C, M>(grant: RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<C, M>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<C, M>>
where
    C: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Mutation<C> + semio_framework_os_kernel::ColdMutation<C> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree,
{
    store::os_store::document_store_owners::bounded_artifact_store_owners::<C, M>(grant)
}

/// 🎛️ Exact one-item PUBLICATION authority for an explicitly bounded app configuration store —
/// the publication twin of [`bounded_config_store_owners`], which only supplies retirement.
/// Without one, `ArtifactApp::build_config_store_one_item_preparation_factory` stays `None` and
/// EVERY tool declaring the `config` publication lane fails closed with
/// `interactive-job.publication-authority-missing` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END:
/// that is exactly what made every generation3d viewer action undispatchable).
///
/// `prefix` labels the minted edit ids; `maximum_bytes` is the admission envelope a single
/// encoded config mutation may occupy.
pub fn bounded_config_store_one_item_preparation_factory<C, M>(prefix: &'static str, maximum_bytes: usize) -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<C, M>>
where
    C: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: Clone + OpBinary + Mutation<C> + semio_framework_os_kernel::ColdMutation<C> + store::ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    std::sync::Arc::new(BoundedConfigPreparationFactory::<C, M> { prefix, maximum_bytes, marker: std::marker::PhantomData, snapshot_factory: std::sync::Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<C>::default()), mutation_factory: std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<M>::default()) })
}

/// 🌱️ Quotes the original preparation source Arc and both original issuer Arcs while their retirement tickets remain pending.
pub fn bounded_config_store_one_item_preparation_factory_source_birth_demand<C: 'static, M: 'static>() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand {
    semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<BoundedConfigPreparationFactory<C, M>>() + semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::factory::SharedValueRetirementFactory<C>>() + semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::factory::OwnedValueRetirementFactory<M>>(), depth: 1 }
}

/// 🌱️ Prices the exact bounded preparation factory Arc, its two original issuer Arcs and every original retirement ticket.
pub fn bounded_config_store_one_item_preparation_factory_birth_bytes<C: 'static, M: 'static>() -> usize {
    semio_framework_value::factory_constructor_birth_bytes::<BoundedConfigPreparationFactory<C, M>>(semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::factory::SharedValueRetirementFactory<C>>() + semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::factory::OwnedValueRetirementFactory<M>>())
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct BoundedConfigPreparationFactory<C, M> {
    #[factory_child]
    snapshot_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<C>>,
    #[factory_child]
    mutation_factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>,
    prefix: &'static str,
    maximum_bytes: usize,
    marker: std::marker::PhantomData<fn() -> (C, M)>,
}

struct BoundedConfigPreparation<C: 'static, M: 'static> {
    prefix: &'static str,
    maximum_bytes: usize,
    base: Option<store::os_store::snapshot_read::SnapshotRead<C>>,
    mutation: Option<M>,
    authority: Option<std::sync::Arc<store::os_store::one_item_publication::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<C, M>>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    pub closing: bool,
    snapshot_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<C>>>,
    mutation_factory: Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>>,
    active_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    returned_read: Option<ControlledRetirement<store::os_store::snapshot_read::SnapshotReadReturn>>,
    returned_factories: [Option<semio_framework_value::FactoryAuthority>; 2],
    closing_factories: [Option<semio_framework_value::FactoryAuthority>; 2],
}

fn bounded_config_mutation_retained_bytes<M: OpBinary>(prefix: &str, mutation: &M) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| format!("{prefix}-mutation-encode-failed"))
}

/// 🧺️ The ONE fold-contract declaration every bounded config lane makes: the mutation's own
/// forward row plus its schema-declared inverse rows ([`store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf`],
/// design §20.5), so `ArtifactStore::fold_batch_item` measures exactly what the lane declares.
fn admit_bounded_config_mutation<C, M: OpBinary + Mutation<C>>(prefix: &str, maximum_bytes: usize, mutation: &M) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = bounded_config_mutation_retained_bytes(prefix, mutation)?;
    if retained_bytes > maximum_bytes {
        return Err(format!("{prefix}-mutation-envelope"));
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<C, M>(mutation, retained_bytes))
}

impl<C, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<C, M> for BoundedConfigPreparationFactory<C, M>
where
    C: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: store::ArtifactCanonicalJsonTree + Clone + OpBinary + Mutation<C> + semio_framework_os_kernel::ColdMutation<C> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    fn begin_batch_digest(&self,edit:&mut Option<Box<protocol::Edit<M>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<M>>,RetainedCloneProgress)>,ValueError>{store::admit_artifact_batch_digest(edit,grant)}
    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != HistoryLane::Document {
            return Err(format!("{}-lane", self.prefix));
        }
        admit_bounded_config_mutation::<C, M>(self.prefix, self.maximum_bytes, mutation)
    }

    fn begin_demand(&self, _mutation: &M, lane: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
        if lane != HistoryLane::Document || self.maximum_bytes == 0 || self.maximum_bytes > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "bounded config preparation has no admissible lane or byte bound"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<BoundedConfigPreparation<C, M>>(), depth: 1 })
    }
    fn begin_copy_byte_demand(&self,input:&M,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<C, M, M>>().checked_add(std::mem::size_of::<BoundedConfigPreparation<C, M>>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<C, M, M>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<C, M>>, RetainedCloneProgress), (ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<C, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let retained_bytes = bounded_config_mutation_retained_bytes(self.prefix, &request.mutation).unwrap_or(self.maximum_bytes.saturating_add(1));
        if request.lane != HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > self.maximum_bytes
        {
            return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "bounded config preparation original request refused"), request));
        }
        Ok((Box::new(BoundedConfigPreparation::<C, M> {
            prefix: self.prefix,
            maximum_bytes: self.maximum_bytes,
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
            snapshot_factory: Some(request.snapshot_retirement), mutation_factory: Some(request.mutation_retirement), active_retirement: None, returned_read: None, returned_factories: Default::default(), closing_factories: Default::default(),
        }), progress))
    }
}

impl<C, M> BoundedConfigPreparation<C, M>
    where C: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
          M: Clone + OpBinary + Mutation<C> + semio_framework_os_kernel::ColdMutation<C> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static {
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let nested=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"bounded config preparation depth overflow"))?;Ok(demand)};
  if let Some(owner)=self.active_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{nested(owner.next_demand(body)?)};}
  if let Some(owner)=self.returned_read.as_ref(){return nested(owner.next_demand(body)?);}
  if let Some(factory)=self.returned_factories.iter().find_map(Option::as_ref){return nested(factory.demands(body)?);}
  if self.prepared.is_some(){let birth=store::os_store::one_item_publication::ArtifactStoreOneItemPrepared::<C,M>::retirement_birth_demand();return Ok(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth+1,..Default::default()});}
  if self.mutation.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactoryOwnedRetirement::<M>::constructor_capacity_bytes(),depth:2,..Default::default()});}
  if self.base.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<store::os_store::snapshot_read::SnapshotRead<C>>(),depth:1,..Default::default()});}
  if let Some(authority)=self.authority.as_ref(){let birth=authority.retirement_birth_demand();return Ok(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth+1,..Default::default()});}
  if self.snapshot_factory.is_some()||self.mutation_factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
  self.closing_factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()),|factory|nested(factory.demands(body)?))
 }
}

impl<C, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<C, M> for BoundedConfigPreparation<C, M>
where
    C: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: Clone + OpBinary + Mutation<C> + semio_framework_os_kernel::ColdMutation<C> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    fn advance(&mut self, body:usize,grant: store::os_store::one_item::ArtifactStoreOneItemGrant, _receiving: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, ValueError> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint,Default::default()));
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"bounded config semantic preparation requires its original incremental mutation owner"))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<C, M>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<C, M>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,grant:store::os_store::one_item::ArtifactStoreOneItemGrant)->Result<RetainedCloneStep,ValueError>{
        let grant=grant.retained_grant();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !self.closing||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let demand=self.close_demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"bounded config preparation exceeds admitted depth"));}if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if let Some(owner)=self.active_retirement.as_mut(){if owner.terminal_is_empty(){self.active_retirement=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}));}return owner.close_step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(owner)=self.returned_read.as_mut(){if owner.terminal_is_empty(){self.returned_read=None;return Ok(RetainedCloneStep::Progress(Default::default()));}return owner.step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(slot)=self.returned_factories.iter_mut().find(|slot|slot.is_some()){let factory=slot.as_mut().unwrap();let step=factory.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,factory.terminal_is_empty(),"returned preparation issuer alias")?;if factory.terminal_is_empty(){*slot=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(prepared)=self.prepared.take(){let mutations=self.mutation_factory.as_ref().expect("captured window mutation issuer").clone();let snapshots=self.snapshot_factory.as_ref().expect("captured window root issuer").clone();return match prepared.admit_retirement(mutations,snapshots,child){Ok((owner,receipt))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original,mutations,snapshots))=>{self.prepared=Some(original);self.returned_factories=[Some(semio_framework_value::FactoryAuthority::new(mutations)),Some(semio_framework_value::FactoryAuthority::new(snapshots))];Err(error)}};}
        if self.mutation.is_some(){let(owner,receipt)=semio_framework_value::FactoryOwnedRetirement::admit_original(&mut self.mutation,self.mutation_factory.as_ref().expect("captured window mutation issuer"),child)?.expect("funded original window mutation remains");self.active_retirement=Some(owner);return Ok(RetainedCloneStep::Progress(receipt));}
        if let Some(base)=self.base.take(){match base.try_return_to_registry_witness(){Ok(witness)=>{self.returned_read=Some(ControlledRetirement::new(witness).map_err(|(error,_)|error).expect("returned registry witness has genuine ownership"));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));},Err(original)=>{self.base=Some(original);return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"bounded config original base read refuses registry return"));}}}
        if let Some(authority)=self.authority.take(){return match authority.retire(child){Ok((owner,receipt))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{self.authority=Some(original);Err(error)}};}
        if let Some(factory)=self.mutation_factory.take(){let factory:std::sync::Arc<dyn semio_framework_value::FactoryRetirement>=factory;self.closing_factories[0]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(factory)=self.snapshot_factory.take(){let factory:std::sync::Arc<dyn semio_framework_value::FactoryRetirement>=factory;self.closing_factories[1]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        let slot=self.closing_factories.iter_mut().find(|slot|slot.is_some()).expect("bounded config original factory close");let factory=slot.as_mut().unwrap();let step=factory.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,factory.terminal_is_empty(),"window captured factory")?;if factory.terminal_is_empty(){*slot=None;}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.base.is_none()&&self.mutation.is_none()&&self.authority.is_none()&&self.prepared.is_none()&&self.active_retirement.is_none()&&self.returned_read.is_none()&&self.returned_factories.iter().all(Option::is_none)&&self.snapshot_factory.is_none()&&self.mutation_factory.is_none()&&self.closing_factories.iter().all(Option::is_none)}
}

/// 🧹️ Explicit close adapter paired with [`bounded_config_store_owners`].
pub fn bounded_config_store_disposer<C, M>() -> Box<dyn ArtifactOwnedDisposer<ConfigStore<C, M>>>
where
    C: semio_framework_value::ToValue + Send + Sync + 'static,
    M: semio_framework_value::ToValue + semio_framework_value::FromValue + Mutation<C> + OpBinary + OpText + Send + 'static,
{
    Box::new(ArtifactDocumentStoreDisposer::<C, M>::new())
}

/// 🧮️ Prices the exact bounded document constructor without creating a catalog.
pub fn bounded_document_store_owners_birth_bytes<P, M>() -> usize
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Mutation<P> + semio_framework_os_kernel::ColdMutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned,
{
    store::os_store::document_store_owners::bounded_artifact_store_owners_birth_bytes::<P, M>()
}

/// 🗃️ Exact one-page retirement catalog for an explicitly bounded document store.
pub fn bounded_document_store_owners<P, M>(grant: RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<P, M>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<P, M>>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Mutation<P> + semio_framework_os_kernel::ColdMutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree,
{
    bounded_config_store_owners::<P, M>(grant)
}

/// 🧽️ Explicit close adapter paired with [`bounded_document_store_owners`].
pub fn bounded_document_store_disposer<P, M>() -> Box<dyn ArtifactOwnedDisposer<ArtifactStore<P, M>>>
where
    P: semio_framework_value::ToValue + Send + Sync + 'static,
    M: semio_framework_value::ToValue + semio_framework_value::FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    Box::new(ArtifactDocumentStoreDisposer::<P, M>::new())
}

#[path="../🌱️initialization/♻️retirement/🦀️.rs"]
mod bounded_initializer;

pub use bounded_initializer::bounded_document_store_initialization_job;

/// 🧹️ Explicit adapter from an app-owned document-store close catalog to the
/// plugin close protocol. Apps must return this adapter themselves; the framework never installs
/// one from type bounds or a permissive default.
pub struct ArtifactDocumentStoreDisposer<P, Mutation>(std::marker::PhantomData<fn() -> (P, Mutation)>);

impl<P, Mutation> Default for ArtifactDocumentStoreDisposer<P, Mutation> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P, Mutation> ArtifactDocumentStoreDisposer<P, Mutation> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<P, Mutation> ArtifactOwnedDisposer<ArtifactStore<P, Mutation>> for ArtifactDocumentStoreDisposer<P, Mutation>
where
    P: semio_framework_value::ToValue + Send + Sync + 'static,
    Mutation: semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn retirement_demands(&self, owner: &ArtifactStore<P, Mutation>, body: usize) -> Result<RetirementDemand, ValueError> {
        owner.close_owned_demands(body)
    }

    fn close_step(&mut self, owner: &mut ArtifactStore<P, Mutation>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let step = owner.close_owned_store_step(grant).map_err(|error| plugin_sdk_fault(error.into_message()))?;
        let terminal = owner.close_owned_store_terminal_is_empty();
        let step = semio_framework_value::retained_clone::admit_retained_clone_close(grant, step, terminal, "original document Store disposer").map_err(|error| plugin_sdk_fault(error.into_message()))?;
        Ok(PluginLifecycleStep::retained(step, terminal))
    }

    fn terminal_is_empty(&self, owner: &ArtifactStore<P, Mutation>) -> bool {
        owner.close_owned_store_terminal_is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}

pub struct ArtifactStoreInitializationJob<P, Mutation>
where
    P: semio_framework_value::ToValue,
    Mutation: semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
{
    authority: ManuallyDrop<Option<Box<dyn ArtifactStoreInitializationAuthority<P, Mutation>>>>,
    cancel_requested: ManuallyDrop<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    cancel_arc_live: bool,
    terminal_handoff: bool,
}

impl<P, Mutation> ArtifactStoreInitializationJob<P, Mutation>
where
    P: semio_framework_value::ToValue,
    Mutation: semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
{
    pub fn new(authority: Box<dyn ArtifactStoreInitializationAuthority<P, Mutation>>) -> Self {
        Self { authority: ManuallyDrop::new(Some(authority)), cancel_requested: ManuallyDrop::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false))), cancel_arc_live: true, terminal_handoff: false }
    }

    /// 📏️ Publish domain allocation demand without spending work or granting its release.
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(authority) = self.authority.as_ref() {
            if authority.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<Option<Box<dyn ArtifactStoreInitializationAuthority<P, Mutation>>>>(), release_bytes: size_of_val(authority.as_ref()), depth: 1, ..Default::default() }); }
            let mut demand = authority.retirement_demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"initializer original authority depth overflow"))?;
            return Ok(demand);
        }
        if self.cancel_arc_live {
            if std::sync::Arc::weak_count(&self.cancel_requested) != 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"initializer cancellation Arc retains an external weak owner")); }
            let cancel_arc_release = if std::sync::Arc::strong_count(&self.cancel_requested) == 1 { semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicBool>() } else { 0 };
            return Ok(RetirementDemand { copy_bytes:size_of::<std::sync::Arc<std::sync::atomic::AtomicBool>>(), release_bytes:cancel_arc_release, depth:1, ..Default::default() });
        }
        Ok(Default::default())
    }

    pub fn cancel_signal(&self) -> std::sync::Arc<std::sync::atomic::AtomicBool> {
        assert!(self.cancel_arc_live,"initializer cancellation source is terminal");
        std::sync::Arc::clone(&self.cancel_requested)
    }

    fn request_cancel_now(&self) {
        if self.cancel_arc_live { self.cancel_requested.store(true, std::sync::atomic::Ordering::Release); }
    }

    pub fn take_candidate(&mut self) -> Option<Box<ArtifactStore<P, Mutation>>> {
        let candidate = self.authority.as_mut()?.take_boxed_candidate()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    /// 📈️ The initializer's `(operations folded, operations to fold)` while its authority is retained.
    pub fn progress(&self) -> (u64, u64) {
        self.authority.as_ref().map_or((0, 0), |authority| authority.progress())
    }

    pub fn accept_terminal_failure(&mut self) -> bool {
        if !self.authority.as_ref().is_some_and(|authority| authority.terminal_is_empty()) {
            return false;
        }
        self.terminal_handoff = true;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.authority.is_none() && !self.cancel_arc_live && self.terminal_handoff
    }

    /// ♻️ One grant-shaped close turn of the retained authority, quoted by [`Self::retirement_demands`], for owners that already hold the surrounding original caller.
    pub fn close_granted(&mut self, _grant: RetainedCloneGrant, demand: RetirementDemand) -> Result<semio_framework_value::RetainedCloneStep, ValueError> {
        if self.authority.is_some(){drop(self.authority.take());return Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))}
        unsafe{ManuallyDrop::drop(&mut self.cancel_requested)}self.cancel_arc_live=false;self.terminal_handoff=true;Ok(semio_framework_value::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
    }
}

impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactStoreInitializationJob<P, Mutation>
where
    P: semio_framework_value::ToValue + Send,
    Mutation: semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + Send,
{
    fn next_close_copy_byte_demand(&self) -> Result<usize,ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self,body:usize) -> Result<usize,ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize,ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize,ValueError> { Ok(self.retirement_demands(0)?.depth) }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        match self.authority.as_mut() {
            Some(authority) => {
                if self.cancel_requested.load(std::sync::atomic::Ordering::Acquire) {
                    authority.request_cancel();
                }
                authority.step(cx)
            }
            None if self.terminal_handoff => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from),
            None => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact-store.initializer-owner-missing").into()),
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        match self.authority.as_ref() {
            Some(authority) => authority.borrow_outcome(descriptor),
            None if self.terminal_handoff && descriptor.kind() == semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            None => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact-store.initializer-owner-missing")),
        }
    }

    fn begin_close(&mut self) {
        self.request_cancel_now();
        if let Some(authority) = self.authority.as_mut() {
            authority.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        self.begin_close();if self.terminal_is_empty(){return Ok(Step::Complete{progress:Default::default()})}let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
        if let Some(authority)=self.authority.as_mut(){
            if !authority.terminal_is_empty(){
                let remaining=cx.retained_grant();
                if cx.retirement_should_yield()||remaining.maximum_items==0||remaining.maximum_depth<demand.depth||remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<demand.capacity_bytes||remaining.maximum_release_bytes<demand.release_bytes{return Ok(Step::Pending{progress:Default::default()})}
                let child=RetainedCloneGrant{maximum_items:1,maximum_depth:remaining.maximum_depth-1,..remaining};
                let step=cx.with_retained_loan(child,|same|authority.close_step(same))?;
                return Ok(Step::Pending{progress:step.progress()})
            }
        }
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_granted(grant,demand))?;
        Ok(match step{Step::Pending{progress}|Step::Complete{progress}=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Step::Blocked=>Step::Blocked})
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactStoreInitializationJob::terminal_is_empty(self)
    }
}

impl<P, Mutation> Drop for ArtifactStoreInitializationJob<P, Mutation>
where
    P: semio_framework_value::ToValue,
    Mutation: semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.authority.is_none() && !self.cancel_arc_live && self.terminal_handoff, "artifact store initialization job reached Drop before exact candidate handoff or terminal retained close");
    }
}

#[path="../🎞️media/📥️snapshot/🦀️.rs"]
mod snapshot_source;
pub use snapshot_source::ArtifactSnapshotCloseLease;
pub use snapshot_source::{ArtifactMediaSnapshotSource,ArtifactSnapshotCloseRetention};
