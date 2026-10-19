//! 🫧️ Exact-window transient state with independent generation authority.

use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_window_config::registry::{WindowRegistry, fits};
use semio_framework_value::{FactoryAuthority, FactoryRetirement, RetirementDemand, ValueError, ValueRefusalKind, RetainedCloneGrant, RetainedCloneProgress};
#[cfg(test)] use semio_framework_value::{RetainedCloneStep};
use semio_framework_plugin_ownership::transient_publication::transient_store_disposer;
use crate::{protocol, store};
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::any::Any;
use semio_framework_plugin_window_config::window_mutation::ErasedWindowMutationValue;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::collections::BTreeMap;
use std::sync::Arc;

fn next_key<K: Ord + Clone, V>(map: &BTreeMap<K, V>, cursor: Option<&K>) -> Option<K> {
    cursor.and_then(|cursor| map.range((std::ops::Bound::Excluded(cursor), std::ops::Bound::Unbounded)).next().map(|(key, _)| key.clone())).or_else(|| map.keys().next().cloned())
}

fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "window transient retirement depth overflow"))?;
    Ok(demand)
}

fn demand_refusal(demand: RetirementDemand, grant: RetainedCloneGrant) -> Result<bool, Fault> {
    if grant.maximum_depth < demand.depth {
        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.depth"), "window transient retirement exceeds its admitted depth"));
    }
    Ok(grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes)
}

fn demand_fault(error: ValueError) -> Fault {
    error.into_fault()
}

/// 🪟️ Declares the transient schema and bounded owners of one concrete window kind.
pub trait WindowTransientOwner: Send + Sync + 'static {
    const WINDOW_KIND_ID: &'static str;
    type State: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + pack::snapshot::ArtifactDsl + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static;
    type Mutation: protocol::Mutation<Self::State> + PartialEq + Send + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + 'static;

    fn build_mutation_retirement_factory()->Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>>{Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<Self::Mutation>::default())}
    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation>;
}

pub struct WindowTransientOwnerBundle<P, M> {
    pub preparation: Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<P, M>>,
    pub state_retirement: Arc<dyn store::ArtifactOwnedValueRetirementFactory<P>>,
    pub mutation_retirement: Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>,
}

impl<P, M> WindowTransientOwnerBundle<P, M> {
    pub fn new(
        preparation: Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<P, M>>,
        state_retirement: Arc<dyn store::ArtifactOwnedValueRetirementFactory<P>>,
        mutation_retirement: Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>,
    ) -> Self {
        Self { preparation, state_retirement, mutation_retirement }
    }
}

/// 📬️ One typed mutation addressed to one exact concrete window instance.
#[derive(semio_framework_value::RetireOwned)]
pub struct WindowTransientMutation {
    #[doc(hidden)]
    pub window_id: String,
    #[doc(hidden)]
    pub window_kind_id: &'static str,
    #[doc(hidden)]
    pub mutation: Box<dyn ErasedWindowMutationValue>,
}

impl std::fmt::Debug for WindowTransientMutation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("WindowTransientMutation").field("window_id", &self.window_id).field("window_kind_id", &self.window_kind_id).finish_non_exhaustive()
    }
}

/// 🙅️ A window transient emission the registry refused, with its mutation handed back: a refusal is the caller's
/// typed fault, never a mutation silently dropped.
#[derive(Debug)]
#[doc(hidden)]
pub struct RejectedWindowTransientEmission {
    pub mutation: WindowTransientMutation,
    pub fault: Fault,
}

impl WindowTransientMutation {
    pub fn of<O: WindowTransientOwner>(window_id: impl Into<String>, mutation: O::Mutation) -> Self {
        Self::from_issued(window_id,O::WINDOW_KIND_ID,mutation,O::build_mutation_retirement_factory())
    }

    #[doc(hidden)]
    pub fn from_issued<T:Clone+Send+'static>(window_id:impl Into<String>,window_kind_id:&'static str,mutation:T,factory:Arc<dyn store::ArtifactOwnedValueRetirementFactory<T>>)->Self{Self{window_id:window_id.into(),window_kind_id,mutation:Box::new(FactoryBoxedValue{original:Box::new(mutation),factory})}}

    pub fn window_id(&self) -> &str {
        &self.window_id
    }

    pub fn window_kind_id(&self) -> &str {
        self.window_kind_id
    }
}

impl semio_framework_plugin_window_config::window_mutation::refresh::WindowRefreshSnapshot for WindowTransientSnapshot{fn swap_address(&mut self,other:&mut Self){std::mem::swap(&mut self.window_id,&mut other.window_id);}}
impl WindowTransientSnapshot{pub fn try_duplicate(&self)->Result<Self,ValueError>{Ok(Self{window_id:self.window_id.clone(),window_kind_id:self.window_kind_id,generation:self.generation,document_generation:self.document_generation,snapshot:self.snapshot.try_duplicate()?})}}
/// 📖️ Immutable snapshot of one exact window-owned transient partition.
#[derive(semio_framework_value::RetireOwned)]
pub struct WindowTransientSnapshot {
    #[doc(hidden)]
    pub window_id: String,
    #[doc(hidden)]
    pub window_kind_id: &'static str,
    #[doc(hidden)]
    pub generation: u64,
    #[doc(hidden)]
    pub document_generation: u64,
    #[doc(hidden)]
    pub snapshot: store::os_store::snapshot_read::ErasedSnapshotRead,
}

impl WindowTransientSnapshot {
    pub fn document_generation(&self) -> u64 {
        self.document_generation
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn window_id(&self) -> &str {
        &self.window_id
    }

    pub fn window_kind_id(&self) -> &str {
        self.window_kind_id
    }

    pub fn get<O: WindowTransientOwner>(&self) -> Option<&O::State> {
        (self.window_kind_id == O::WINDOW_KIND_ID).then(|| self.snapshot.get::<O::State>()).flatten()
    }
}

#[derive(semio_framework_value::RetireOwned)]
#[doc(hidden)]
pub struct WindowTransientAuthority {
    pub window_id: String,
    pub window_kind_id: String,
    pub generation: u64,
    pub snapshot: WindowTransientSnapshot,
    pending_snapshot:Option<WindowTransientSnapshot>,
    retired_snapshots:semio_framework_value::retirement_queue::RetirementQueue,
}

#[doc(hidden)]
pub trait ErasedWindowTransientPublication: Send {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn window_kind_id(&self) -> &str;
    fn document_generation(&self) -> u64;
    fn phase(&self) -> store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase;
    fn fault(&self) -> Option<&str>;
    fn preparation_refusal(&self) -> Option<&ValueError>;
    fn acknowledge(&mut self) -> bool;
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError>;
    fn ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn close_ingress(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

struct TypedWindowTransientPublication<O: WindowTransientOwner> {
    address_retirement:Option<ControlledRetirement<String>>,
    authority: WindowConfigAuthorityIdentity,
    window_id: semio_framework_value::SharedUtf8,
    pub(crate) document_generation: u64,
    publication: store::ArtifactEphemeralOneItemPublication<O::State, O::Mutation>,
    ingress:ControlledRetirement<FactoryBoxedPublication<O::Mutation>>,
}

impl<O: WindowTransientOwner> ErasedWindowTransientPublication for TypedWindowTransientPublication<O> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn window_kind_id(&self) -> &str {
        O::WINDOW_KIND_ID
    }

    fn document_generation(&self) -> u64 {
        self.document_generation
    }

    fn phase(&self) -> store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase {
        self.publication.phase()
    }

    fn fault(&self) -> Option<&str> {
        self.publication.fault()
    }
    fn preparation_refusal(&self) -> Option<&ValueError> { self.publication.preparation_refusal() }

    fn acknowledge(&mut self) -> bool {
        self.publication.acknowledge()
    }

    fn begin_close(&mut self) {
        self.publication.begin_close();
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, ValueError> {
        if !self.ingress.terminal_is_empty(){return self.ingress.step(grant).map(|step|semio_framework_value::RetainedCloneStep::Progress(step.progress()));}
        self.publication.close_step(store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items, maximum_copy_bytes: grant.maximum_copy_bytes, maximum_capacity_bytes: grant.maximum_capacity_bytes, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth })
    }

    fn ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(self.ingress.next_demand(body)?)}
    fn close_ingress(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>{self.ingress.step(grant)}
    fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.ingress.terminal_is_empty(){self.ingress_demands(body)}else if !self.publication.terminal_is_empty(){self.publication.retirement_demands(body)}else{crate::window_mutation::address_demands(&self.window_id,&self.address_retirement,body)}}
    fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty()&&self.ingress.terminal_is_empty()&&crate::window_mutation::address_terminal(&self.window_id,&self.address_retirement)
    }
}

#[doc(hidden)]
pub type WindowTransientStore<O> = store::TransientStore<<O as WindowTransientOwner>::State, <O as WindowTransientOwner>::Mutation>;

struct WindowTransientPartition<O: WindowTransientOwner> {
    store: WindowTransientStore<O>,
    disposer: Option<Box<dyn ArtifactOwnedDisposer<WindowTransientStore<O>>>>,
}

impl<O: WindowTransientOwner> WindowTransientPartition<O> {
    fn new(factory: Arc<dyn store::ArtifactOwnedValueRetirementFactory<O::State>>) -> Self {
        Self { store: store::TransientStore::new(O::State::default()), disposer: Some(transient_store_disposer::<O::State, O::Mutation>(factory)) }
    }
}

trait ErasedWindowTransientStoreOwner: Send {
    fn capture(&mut self, window_id: &str, document_generation: u64) -> Result<WindowTransientAuthority, Fault>;
    fn refresh_demands(&self,authority:&WindowTransientAuthority,body:usize)->Result<RetirementDemand,ValueError>;
    fn refresh(&mut self,authority:&mut WindowTransientAuthority,grant:RetainedCloneGrant)->Result<semio_framework_plugin_window_config::window_mutation::WindowAuthorityRefreshStep,Fault>;
    fn begin(&mut self, operation: semio_framework_job::OperationId, expected_generation: u64, document_generation: u64, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, RejectedWindowTransientEmission>;
    fn advance(&mut self, publication: &mut dyn ErasedWindowTransientPublication, grant: store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault>;
    fn maintenance_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn maintenance_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
    fn terminal_is_empty(&self) -> bool;
}

#[doc(hidden)]
pub struct TypedWindowTransientStoreOwner<O: WindowTransientOwner> {
    #[doc(hidden)]
    pub partitions: WindowRegistry<String, WindowTransientPartition<O>>,
    #[doc(hidden)]
    pub owners: Option<WindowTransientOwnerBundle<O::State, O::Mutation>>,
    #[doc(hidden)]
    pub factory_close: [Option<FactoryAuthority>; 3],
    #[doc(hidden)]
    pub partition_close_cursor: usize,
    #[doc(hidden)]
    pub partition_open: usize,
    #[doc(hidden)]
    pub partition_address: Option<ControlledRetirement<String>>,
}

impl<O: WindowTransientOwner> TypedWindowTransientStoreOwner<O> {
    #[doc(hidden)]
    pub fn partition(&mut self, window_id: &str) -> &mut WindowTransientPartition<O> {
        if !self.partitions.contains_key(window_id) {
            let factory = self.owners.as_ref().expect("live transient issuers remain").state_retirement.clone();
            self.partitions.insert(window_id.to_owned(), WindowTransientPartition::<O>::new(factory));
            self.partition_open += 1;
        }
        self.partitions.get_mut(window_id).expect("original transient partition remains")
    }
}

impl<O: WindowTransientOwner> ErasedWindowTransientStoreOwner for TypedWindowTransientStoreOwner<O> {
    fn capture(&mut self, window_id: &str, document_generation: u64) -> Result<WindowTransientAuthority, Fault> {
        let partition = self.partition(window_id);
        let snapshot = partition.store.current_read_erased().map_err(Fault::from)?;
        Ok(WindowTransientAuthority {
            window_id: window_id.to_string(),
            window_kind_id: O::WINDOW_KIND_ID.to_string(),
            generation: partition.store.generation_now(),
            pending_snapshot:None,retired_snapshots:Default::default(),
            snapshot: WindowTransientSnapshot { window_id: window_id.to_string(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation_now(), document_generation, snapshot },
        })
    }

    fn refresh_demands(&self,authority:&WindowTransientAuthority,_body:usize)->Result<RetirementDemand,ValueError>{semio_framework_plugin_window_config::window_mutation::refresh::refresh_demands(&authority.pending_snapshot,&authority.retired_snapshots)}
    fn refresh(&mut self,authority:&mut WindowTransientAuthority,grant:RetainedCloneGrant)->Result<semio_framework_plugin_window_config::window_mutation::WindowAuthorityRefreshStep,Fault>{
        let demand=self.refresh_demands(authority,grant.maximum_copy_bytes).map_err(demand_fault)?;
        if demand_refusal(demand,grant)?{return Ok(semio_framework_plugin_window_config::window_mutation::WindowAuthorityRefreshStep::Pending(Default::default()));}
        if authority.pending_snapshot.is_none(){let partition=self.partition(&authority.window_id);let snapshot=partition.store.current_read_erased().map_err(Fault::from)?;authority.pending_snapshot=Some(WindowTransientSnapshot{window_id:String::new(),window_kind_id:O::WINDOW_KIND_ID,generation:partition.store.generation_now(),document_generation:authority.snapshot.document_generation,snapshot});return Ok(semio_framework_plugin_window_config::window_mutation::WindowAuthorityRefreshStep::Pending(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        let step=semio_framework_plugin_window_config::window_mutation::refresh::refresh_transfer(&mut authority.snapshot,&mut authority.pending_snapshot,&mut authority.retired_snapshots,grant).map_err(demand_fault)?;
        if matches!(step,semio_framework_plugin_window_config::window_mutation::WindowAuthorityRefreshStep::Ready(_)){authority.generation=authority.snapshot.generation;}Ok(step)
    }
    fn begin(&mut self, operation: semio_framework_job::OperationId, expected_generation: u64, document_generation: u64, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, RejectedWindowTransientEmission> {
        let WindowTransientMutation { window_id, window_kind_id, mutation } = mutation;
        if !mutation.as_any().is::<O::Mutation>(){return Err(RejectedWindowTransientEmission{mutation:WindowTransientMutation{window_id,window_kind_id,mutation},fault:Fault::new(FaultOrigin::Framework,FaultCode::new("window-transient.mutation-type"),"window transient mutation did not match its registered window owner")});}
        let typed=mutation.into_any().downcast::<FactoryBoxedValue<O::Mutation>>().expect("registered payload carrier preserves its typed owner");
        let (typed,ingress)=typed.take_for_publication();
        let preparation = self.owners.as_ref().expect("live transient issuers remain").preparation.clone();
        let retirement = self.owners.as_ref().expect("live transient issuers remain").state_retirement.clone();
        let partition = self.partition(&window_id);
        match partition.store.begin_publish_one_leased(operation, expected_generation, typed, preparation.as_ref(), retirement) {
            Ok(publication) => Ok(Box::new(TypedWindowTransientPublication::<O> { window_id, document_generation, publication,address_retirement:None,ingress:ControlledRetirement::new(ingress).map_err(|(error,_)|error).expect("typed ingress owns genuine facets") })),
            Err(rejected) => {
                let (reason, mutation) = rejected.into_owners();
                let fault = Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.admission"), reason);
                Err(RejectedWindowTransientEmission { mutation: WindowTransientMutation { window_id, window_kind_id, mutation: ingress.restore(mutation) }, fault })
            }
        }
    }

    fn advance(&mut self, publication: &mut dyn ErasedWindowTransientPublication, grant: store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault> {
        let publication = publication
            .as_any_mut()
            .downcast_mut::<TypedWindowTransientPublication<O>>()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.publication-type"), "window transient publication did not match its registered window owner"))?;
        self.partition(&publication.window_id).store.advance_publish_one(&mut publication.publication, grant,receiving).map_err(|reason| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.publication"), reason))
    }

    fn maintenance_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        for partition in self.partitions.values() {
            let demand = partition.store.maintenance_returned_reads_demands(body)?;
            if demand != RetirementDemand::default() { return Ok(demand); }
        }
        Ok(Default::default())
    }

    fn maintenance_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let demand = self.maintenance_demands(grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        if demand == RetirementDemand::default() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        if !fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        let factory = &self.owners.as_ref().expect("live transient issuer remains").state_retirement;
        for partition in self.partitions.values_mut() {
            if partition.store.maintenance_returned_reads_demands(grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))? != RetirementDemand::default() {
                let step = partition.store.maintenance_returned_reads_step(factory, grant).map_err(|error| Fault::from(error.into_message()))?;
                return Ok(PluginLifecycleStep::Progress(step.progress()));
            }
        }
        Ok(PluginLifecycleStep::Complete(Default::default()))
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if let Some(owner) = self.partition_address.as_ref() {
            return Ok(owner.next_demand(body)?);
        }
        if let Some(partition) = (self.partition_open != 0).then(|| self.partitions.get_index(self.partition_close_cursor)).flatten() {
            if let Some(owner) = partition.disposer.as_ref() {
                return if owner.terminal_is_empty(&partition.store) { Ok(RetirementDemand { copy_bytes: size_of::<Option<Box<dyn ArtifactOwnedDisposer<WindowTransientStore<O>>>>>() + size_of::<usize>() * 2, release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }) } else { owner.retirement_demands(&partition.store, body).map(|mut demand| { demand.copy_bytes = demand.copy_bytes.max(size_of::<usize>()); demand }) };
            }
            return Ok(RetirementDemand { copy_bytes: size_of::<usize>(), depth: 1, ..Default::default() });
        }
        if !self.partitions.is_empty() { return self.partitions.pop_demand(); }
        if !self.partitions.terminal_is_empty() { return self.partitions.backing_demand(); }
        if self.owners.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<WindowTransientOwnerBundle<O::State, O::Mutation>>(), depth: 1, ..Default::default() }); }
        self.factory_close.iter().flatten().next().map_or(Ok(Default::default()), |factory| factory.demands(body))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        if !fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if let Some(owner) = self.partition_address.as_mut() {
            let step = owner.step(grant).map_err(|error| Fault::from(error.into_message()))?;
            if owner.terminal_is_empty() { self.partition_address = None; }
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        let partition_count = self.partitions.len();
        if let Some(partition) = (self.partition_open != 0).then(|| self.partitions.get_index_mut(self.partition_close_cursor)).flatten() {
            if let Some(owner) = partition.disposer.as_mut() {
                if !owner.terminal_is_empty(&partition.store) {
                    let step = owner.close_step(&mut partition.store, grant)?;
                    if matches!(step, PluginLifecycleStep::AwaitingInput { .. } | PluginLifecycleStep::Blocked { .. }) || step.progress().is_some_and(|progress| progress == RetainedCloneProgress::default()) {
                        self.partition_close_cursor = (self.partition_close_cursor + 1) % partition_count;
                        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), ..Default::default() }));
                    }
                    return Ok(match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other });
                }
                drop(partition.disposer.take());
                self.partition_open -= 1;
                self.partition_close_cursor = (self.partition_close_cursor + 1) % partition_count;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }));
            }
            self.partition_close_cursor = (self.partition_close_cursor + 1) % partition_count;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if !self.partitions.is_empty() {
            let ((address, partition), progress) = self.partitions.pop_original(grant).map_err(|error| Fault::from(error.into_message()))?.expect("funded original transient partition remains");
            assert!(partition.store.close_terminal_is_empty() && partition.disposer.is_none());
            drop(partition);
            self.partition_address = Some(ControlledRetirement::new(address).map_err(|(error, _)| Fault::from(error.into_message()))?);
            return Ok(PluginLifecycleStep::Progress(progress));
        }
        if !self.partitions.terminal_is_empty() {
            return self.partitions.close_backing_step(grant).map(|step| PluginLifecycleStep::Progress(step.progress())).map_err(|error| Fault::from(error.into_message()));
        }
        if let Some(owners) = self.owners.take() {
            let preparation: Arc<dyn FactoryRetirement> = owners.preparation;
            let state: Arc<dyn FactoryRetirement> = owners.state_retirement;
            let mutation: Arc<dyn FactoryRetirement> = owners.mutation_retirement;
            self.factory_close = [Some(FactoryAuthority::new(preparation)), Some(FactoryAuthority::new(state)), Some(FactoryAuthority::new(mutation))];
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        let slot = self.factory_close.iter_mut().find(|slot| slot.is_some()).expect("original transient issuer retirement remains");
        let factory = slot.as_mut().unwrap();
        let step = factory.step(grant).map_err(|error| Fault::from(error.into_message()))?;
        if factory.terminal_is_empty() { *slot = None; }
        Ok(PluginLifecycleStep::retained(step, self.terminal_is_empty()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.partitions.terminal_is_empty() && self.partition_address.is_none() && self.owners.is_none() && self.factory_close.iter().all(Option::is_none)
    }
}

/// 🗂️ Runtime registry of heterogeneous window-owned transient schemas.
pub struct WindowTransientOwnerRegistry {
    #[doc(hidden)]
    pub document_generation: u64,
    #[doc(hidden)]
    pub owners: WindowRegistry<&'static str, Option<Box<dyn ErasedWindowTransientStoreOwner>>>,
    owner_close_cursor: usize,
    #[doc(hidden)]
    pub owner_open: usize,
}

impl WindowTransientOwnerRegistry {
    #[doc(hidden)]
    pub fn for_document_generation(document_generation: u64) -> Self {
        Self { document_generation, owners: WindowRegistry::new(), owner_close_cursor: 0, owner_open: 0 }
    }

    #[doc(hidden)]
    pub fn document_generation(&self) -> u64 {
        self.document_generation
    }

    pub fn register<O: WindowTransientOwner>(&mut self) -> Result<(), Fault> {
        if O::WINDOW_KIND_ID.is_empty() || self.owners.contains_key(O::WINDOW_KIND_ID) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner"), "window transient owner id is empty or already registered"));
        }
        self.owners.insert(O::WINDOW_KIND_ID, Some(Box::new(TypedWindowTransientStoreOwner::<O> { partitions: WindowRegistry::new(), owners: Some(O::build_owners()), factory_close: [None, None, None], partition_close_cursor: 0, partition_open: 0, partition_address: None })));
        self.owner_open += 1;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    #[doc(hidden)]
    pub fn refresh_demands(&self, authority: &WindowTransientAuthority, body: usize) -> Result<RetirementDemand, ValueError> {
        self.owners.get(authority.window_kind_id.as_str()).and_then(Option::as_ref).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "window transient refresh has no registered concrete window owner"))?.refresh_demands(authority, body)
    }

    #[doc(hidden)]
    pub fn refresh(&mut self, authority: &mut WindowTransientAuthority, grant: RetainedCloneGrant) -> Result<semio_framework_plugin_window_config::window_mutation::WindowAuthorityRefreshStep, Fault> {
        if authority.snapshot.document_generation != self.document_generation {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.document-generation"), "window transient refresh belongs to a replaced document"));
        }
        self.owners.get_mut(authority.window_kind_id.as_str()).and_then(Option::as_mut).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner"), "window transient refresh has no registered concrete window owner"))?.refresh(authority, grant)
    }

    #[doc(hidden)]
    pub fn capture(&mut self, view_state: Option<&ViewModel>) -> Result<Option<WindowTransientAuthority>, Fault> {
        let Some(view_state) = view_state else { return Ok(None) };
        let Some(window_id) = view_state.window_id.as_deref() else { return Ok(None) };
        let window = view_state
            .window_instances
            .iter()
            .find(|window| window.id == window_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.window-context"), "target window is absent from the exact ViewModel window instance roster"))?;
        let Some(owner) = self.owners.get_mut(window.window_kind_id.as_str()).and_then(Option::as_mut) else { return Ok(None) };
        owner.capture(window_id, self.document_generation).map(Some)
    }

    /// 📬️ Begins publishing `mutation` under the operation's captured `authority`; a refusal hands the mutation back.
    #[doc(hidden)]
    pub fn begin(&mut self, operation: semio_framework_job::OperationId, authority: &WindowTransientAuthority, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, RejectedWindowTransientEmission> {
        let refuse = |mutation, code: &str, message: &str| Err(RejectedWindowTransientEmission { mutation, fault: Fault::new(FaultOrigin::Framework, FaultCode::new(code), message) });
        if authority.snapshot.document_generation != self.document_generation {
            return refuse(mutation, "window-transient.document-generation", "window transient emission belongs to a replaced document");
        }
        if mutation.window_id != authority.window_id || mutation.window_kind_id != authority.window_kind_id {
            return refuse(mutation, "window-transient.address", "window transient emission does not match the operation's exact captured window authority");
        }
        let document_generation = self.document_generation;
        match self.owners.get_mut(authority.window_kind_id.as_str()).and_then(Option::as_mut) {
            Some(owner) => owner.begin(operation, authority.generation, document_generation, mutation),
            None => refuse(mutation, "window-transient.owner", "window transient emission has no registered concrete window owner"),
        }
    }

    #[doc(hidden)]
    pub fn advance(&mut self, publication: &mut dyn ErasedWindowTransientPublication, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault> {
        if publication.document_generation() != self.document_generation {
            publication.begin_close();
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.document-generation"), "window transient publication belongs to a replaced document"));
        }
        self.owners
            .get_mut(publication.window_kind_id())
            .and_then(Option::as_mut)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner"), "window transient publication lost its registered concrete window owner"))?
            .advance(publication, grant,receiving)
    }

    #[doc(hidden)]
    pub fn maintenance_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        for owner in self.owners.values().flatten() {
            let demand = owner.maintenance_demands(body)?;
            if demand != RetirementDemand::default() { return Ok(demand); }
        }
        Ok(Default::default())
    }

    #[doc(hidden)]
    pub fn maintenance_terminal_is_empty(&self) -> bool {
        self.maintenance_demands(0).is_ok_and(|demand| demand == RetirementDemand::default())
    }

    #[doc(hidden)]
    pub fn maintenance_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let demand = self.maintenance_demands(grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        if demand == RetirementDemand::default() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        if !fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        for owner in self.owners.values_mut().flatten() {
            if owner.maintenance_demands(grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))? != RetirementDemand::default() { return owner.maintenance_step(grant); }
        }
        Ok(PluginLifecycleStep::Complete(Default::default()))
    }

    #[doc(hidden)]
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if self.owner_open != 0 {
            if let Some(owner) = self.owners.get_index(self.owner_close_cursor).unwrap().as_ref() {
                return if owner.terminal_is_empty() { Ok(RetirementDemand { copy_bytes: size_of::<Option<Box<dyn ErasedWindowTransientStoreOwner>>>() + size_of::<usize>() * 2, release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }) } else { owner.retirement_demands(body).map(|mut demand| { demand.copy_bytes = demand.copy_bytes.max(size_of::<usize>()); demand }) };
            }
            return Ok(RetirementDemand { copy_bytes: size_of::<usize>(), depth: 1, ..Default::default() });
        }
        if !self.owners.is_empty() { return self.owners.pop_demand(); }
        self.owners.backing_demand()
    }

    #[doc(hidden)]
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        if !fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if self.owner_open != 0 {
            let count = self.owners.len();
            let slot = self.owners.get_index_mut(self.owner_close_cursor).unwrap();
            if let Some(owner) = slot.as_mut() {
                if !owner.terminal_is_empty() {
                    let step = owner.close_step(grant)?;
                    if matches!(step, PluginLifecycleStep::AwaitingInput { .. } | PluginLifecycleStep::Blocked { .. }) || step.progress().is_some_and(|progress| progress == RetainedCloneProgress::default()) {
                        self.owner_close_cursor = (self.owner_close_cursor + 1) % count;
                        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<usize>(), ..Default::default() }));
                    }
                    return Ok(match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other });
                }
                drop(slot.take());
                self.owner_open -= 1;
                self.owner_close_cursor = (self.owner_close_cursor + 1) % count;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }));
            }
            self.owner_close_cursor = (self.owner_close_cursor + 1) % count;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if !self.owners.is_empty() {
            let ((_, owner), progress) = self.owners.pop_original(grant).map_err(|error| Fault::from(error.into_message()))?.expect("funded original empty transient slot remains");
            assert!(owner.is_none());
            return Ok(PluginLifecycleStep::Progress(progress));
        }
        self.owners.close_backing_step(grant).map(|step| PluginLifecycleStep::retained(step, self.terminal_is_empty())).map_err(|error| Fault::from(error.into_message()))
    }

    #[doc(hidden)]
    pub fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}

impl Default for WindowTransientOwnerRegistry {
    fn default() -> Self { Self::for_document_generation(0) }
}

/// 📦️ The ephemeral lanes of one completion payload: presence and transient mutations plus the window-transient mutations, retired as one owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct EphemeralParts<P, T> {
    pub presence: Vec<P>,
    pub transient: Vec<T>,
    pub window_transient: Vec<WindowTransientMutation>,
}

//#region 🔖️TransientRoot
/// 📦️ Declares the ephemeral transfer of a whole-root window transient ([`transient_root!`]): the mutation's owned retirement
/// (the state is `RetireOwned`), `$mutation::footprint` (one admitted item of the encoded root's bytes, refused above the store's
/// one-item bound) and `$mutation::into_state`, under a compile-time assertion that state and mutation stay within the
/// transfer's inline bound (audit F8). [`window_transient_owners!`] declares it; a module that writes its own owners (one
/// owner pair per window kind with a window config) declares it alone.
#[macro_export]
macro_rules! window_transient_transfer {
    (state: $state:ident, mutation: $mutation:ident $(,)?) => {
        const _: () = assert!(
            std::mem::size_of::<$state>() <= $crate::__kernel::ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES && std::mem::size_of::<$mutation>() <= $crate::__kernel::ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES,
            concat!(stringify!($state), " exceeds the ephemeral transfer's inline bound: box its large fields")
        );

        impl $mutation {
            #[doc = "📏️ The one admitted item this mutation publishes: the encoded root's bytes."]
            pub fn footprint(&self) -> Result<$crate::__kernel::one_item::ArtifactStoreOneItemFootprint, String> {
                let Self::Snapshot { transient } = self;
                let footprint = $crate::__kernel::one_item::ArtifactStoreOneItemFootprint::for_ephemeral_item($crate::__pack_json::to_json_string(transient).len());
                footprint.is_admissible().then_some(footprint).ok_or_else(|| concat!(stringify!($state), " exceeds its retained publication envelope").to_string())
            }

            #[doc = "📦️ The root this mutation installs, moved out without a copy."]
            pub fn into_state(self) -> $state {
                let Self::Snapshot { transient } = self;
                transient
            }
        }

    };
}
//#endregion 🔖️TransientRoot

/// 🫧️ Read-only view of the TRANSIENT lane: ephemeral state local to this client that is
/// never document content — the typed replacement for plugin `thread_local!` scratch state.
pub struct TransientView<'a, T> {
    pub snapshot: &'a T,
    pub window: Option<&'a WindowTransientSnapshot>,
}

impl<T> TransientView<'_, T> {
    pub fn window<O: WindowTransientOwner>(&self) -> Option<&O::State> {
        self.window.and_then(WindowTransientSnapshot::get::<O>)
    }
}
