//! 🎚️ Exact-window persisted-local configuration with independent event-stream authority.

use super::app::{ArtifactOwnedDisposer, PluginLifecycleStep};
use crate::{protocol, store};
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::any::Any;
use super::window_mutation::ErasedWindowMutationValue;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use semio_framework_value::{OriginalAliasBatch,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[path="🧬️preparation/♻️custody/🦀️.rs"]
pub(crate) mod preparation_custody;

#[path = "📥️retained/🦀️.rs"]
mod retained;
#[path = "🗂️registry/🦀️.rs"]
pub(crate) mod registry;
pub(crate) use registry::WindowRegistry;
pub use retained::{WindowConfigPackLoad, WindowConfigPackLoadDiagnostic, WindowConfigPackLoadPhase, WindowConfigPackLoadProgress, WindowConfigPackLoadStep};

/// 🪟️ Declares one concrete window kind's persisted-local configuration owner.
pub trait WindowConfigOwner: Send + Sync + 'static {
    const WINDOW_KIND_ID: &'static str;
    const SCHEMA: &'static str;
    const MAXIMUM_PUBLICATION_BYTES: usize;
    type State: semio_framework_value::retained_clone::RetainedClone + Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + store::ConfigRecord + store::ArtifactPack + semio_framework_dsl_record::DslField + semio_framework_value::retirement::RetireOwned + 'static;
    type Mutation: protocol::Mutation<Self::State> + store::ArtifactCanonicalJsonTree + PartialEq + Send + Sync + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + 'static;

    type Edit: store::snapshot_clone_preparation::RetainedCloneEdit<Self::State,Self::Mutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize;
    fn build_retained_edit() -> Arc<Self::Edit>;

    fn build_mutation_retirement_factory()->Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>>{Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default())}
    fn build_snapshot_retirement_factory()->Arc<dyn store::SnapshotRetirementFactory<Self::State>>{Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<Self::State>::default())}
    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError>;
    fn build_one_item_preparation_factory() -> Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>>;
    fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>>;
}

pub fn bounded_window_config_store_owners<O: WindowConfigOwner>() -> Result<store::DocumentStoreOwners<O::State, O::Mutation>, ValueError> {
    store::funded_bounded_artifact_store_owners::<O::State, O::Mutation>()
}

/// 🧬️ Uses the original domain edit issuer and Store's admitted clone, metadata and canonical-seal owners.
pub fn bounded_window_config_preparation_factory<O: WindowConfigOwner>() -> Arc<dyn store::ArtifactStoreOneItemPreparationFactory<O::State, O::Mutation>> {
    assert!(O::MAXIMUM_PUBLICATION_BYTES>0&&O::MAXIMUM_PUBLICATION_BYTES<=store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,"original window owner declares its admissible publication bound");
    Arc::new(store::snapshot_clone_preparation::RetainedClonePreparationFactory::new(O::build_retained_edit(),O::build_mutation_retirement_factory(),O::build_snapshot_retirement_factory(),O::MAXIMUM_PREPARATION_DEPTH).expect("original window owner declares nonzero preparation depth"))
}

pub fn bounded_window_config_store_disposer<O: WindowConfigOwner>() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<O::State, O::Mutation>>> {
    super::app::bounded_config_store_disposer::<O::State, O::Mutation>()
}

#[cfg(test)]
#[path="🧬️preparation/🧪️tests/🦀️.rs"]
mod original_preparation_tests;

#[cfg(test)]
#[path = "🧪️tests/🪟️retained-window-config/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🪪️pack-identity/🦀️.rs"]
mod pack_identity_tests;

#[cfg(test)]
#[path = "🧪️tests/📥️retained-pack-load/🦀️.rs"]
mod retained_pack_load_tests;

/// 📬️ One typed config mutation addressed to one exact concrete window instance; clonable, so a press holds it as one of its
/// provisional leaves (design §20.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
#[derive(semio_framework_value::RetireOwned)]
pub struct WindowConfigMutation {
    window_id: String,
    window_kind_id: &'static str,
    mutation: Box<dyn ErasedWindowMutationValue>,
}

impl Clone for WindowConfigMutation {
    fn clone(&self) -> Self {
        Self { window_id: self.window_id.clone(), window_kind_id: self.window_kind_id, mutation: self.mutation.clone_value() }
    }
}

impl std::fmt::Debug for WindowConfigMutation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("WindowConfigMutation").field("window_id", &self.window_id).field("window_kind_id", &self.window_kind_id).finish_non_exhaustive()
    }
}

/// 🚫️ A refused window-config emission hands its mutation BACK. The retained publication ladder
/// pops one mutation out of the operation's `Emit` before it begins a batch, so a refusal that kept the
/// mutation left the retry with nothing to publish and the operation completed clean — no page, no
/// fault, the amend lost (ticket 26/09/19 `📓️flow.md` §5.2). Every other publication lane of that ladder
/// already returns its owners on rejection; this is the window-config lane's carrier for the same rule.
pub struct RejectedWindowConfigEmission {
    pub mutation: WindowConfigMutation,
    pub fault: Fault,
}

impl std::fmt::Debug for RejectedWindowConfigEmission {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("RejectedWindowConfigEmission").field("fault", &self.fault).finish_non_exhaustive()
    }
}

impl WindowConfigMutation {
    pub fn of<O: WindowConfigOwner>(window_id: impl Into<String>, mutation: O::Mutation) -> Self {
        Self::from_issued(window_id,O::WINDOW_KIND_ID,mutation,O::build_mutation_retirement_factory())
    }

    pub(crate) fn from_issued<T:Clone+Send+'static>(window_id:impl Into<String>,window_kind_id:&'static str,mutation:T,factory:Arc<dyn store::ArtifactOwnedValueRetirementFactory<T>>)->Self{Self{window_id:window_id.into(),window_kind_id,mutation:Box::new(FactoryBoxedValue{original:Box::new(mutation),factory})}}

    pub fn window_id(&self) -> &str {
        &self.window_id
    }

    pub fn window_kind_id(&self) -> &str {
        self.window_kind_id
    }
}

/// 📖️ Immutable snapshot of one exact window-owned config partition.
#[derive(Clone)]
pub struct WindowConfigSnapshot {
    window_id: String,
    window_kind_id: &'static str,
    generation: u64,
    revision: [u8; 32],
    snapshot: Arc<dyn Any + Send + Sync>,
}

impl WindowConfigSnapshot {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn revision(&self) -> [u8; 32] {
        self.revision
    }

    pub fn window_id(&self) -> &str {
        &self.window_id
    }

    pub fn window_kind_id(&self) -> &str {
        self.window_kind_id
    }

    pub fn get<O: WindowConfigOwner>(&self) -> Option<&O::State> {
        (self.window_kind_id == O::WINDOW_KIND_ID).then(|| self.snapshot.as_ref().downcast_ref::<O::State>()).flatten()
    }
}

/// 💾 Persisted envelope for one exact window config partition.
pub struct WindowConfigPack {
    pub window_id: String,
    pub window_kind_id: String,
    pub files: store::ArtifactPackFiles,
}

#[derive(Clone)]
pub(crate) struct WindowConfigAuthority {
    pub window_id: String,
    pub window_kind_id: String,
    pub generation: u64,
    pub revision: [u8; 32],
    pub snapshot: WindowConfigSnapshot,
}

pub(crate) trait ErasedWindowConfigPublication: Send {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn window_kind_id(&self) -> &str;
    fn phase(&self) -> store::ArtifactStoreOneItemPublicationPhase;
    fn fault(&self) -> Option<&str>;
    fn acknowledge(&mut self) -> bool;
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, semio_framework_value::ValueError>;
    fn ingress_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>;
    fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>;
    fn close_ingress(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

struct TypedWindowConfigPublication<O: WindowConfigOwner> {
    window_id: ControlledRetirement<String>,
    operation: semio_framework_job::OperationId,
    generation: u64,
    revision: [u8;32],
    actor: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
    pending: std::mem::ManuallyDrop<Vec<O::Mutation>>,
    transaction: Option<protocol::TransactionRef>,
    publication: std::mem::ManuallyDrop<Option<store::ArtifactStoreBatchPublication<O::State, O::Mutation>>>,
    ingress: ControlledRetirement<FactoryBoxedPublication<O::Mutation>>,
    cancelled: Option<ControlledRetirement<Box<FactoryBoxedValue<O::Mutation>>>>,
    closing: bool,
}

impl<O: WindowConfigOwner> TypedWindowConfigPublication<O> {
    fn pending_bytes(&self)->usize{self.pending.capacity().checked_mul(std::mem::size_of::<O::Mutation>()).expect("admitted original window backing remains bounded")}
    fn controlled_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{
        Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})
    }
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.pending.is_empty(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<O::Mutation>()+std::mem::size_of::<FactoryBoxedValue<O::Mutation>>()+std::mem::size_of::<Option<ControlledRetirement<Box<FactoryBoxedValue<O::Mutation>>>>>(),depth:1,..Default::default()});}
        if self.pending_bytes()!=0{return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Vec<O::Mutation>>(),release_bytes:self.pending_bytes(),depth:1,..Default::default()});}
        if let Some(actor)=self.actor.as_ref(){return if actor.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ControlledRetirement<semio_framework_value::SharedUtf8>>>(),depth:1,..Default::default()})}else{Self::controlled_demand(actor,body)};}
        if !self.ingress.terminal_is_empty(){return Self::controlled_demand(&self.ingress,body);}
        if let Some(cancelled)=self.cancelled.as_ref(){return if cancelled.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ControlledRetirement<Box<FactoryBoxedValue<O::Mutation>>>>>(),depth:1,..Default::default()})}else{Self::controlled_demand(cancelled,body)};}
        if let Some(publication)=self.publication.as_ref(){return if publication.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<store::ArtifactStoreBatchPublication<O::State,O::Mutation>>>(),depth:1,..Default::default()})}else{publication.retirement_demands(body)};}
        Self::controlled_demand(&self.window_id,body)
    }
}

impl<O: WindowConfigOwner> Drop for TypedWindowConfigPublication<O>{
    fn drop(&mut self){
        assert!(std::thread::panicking()||self.terminal_is_empty(),"original window publication abandoned before paid closure");
        if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.pending);std::mem::ManuallyDrop::drop(&mut self.publication);}}
    }
}

impl<O: WindowConfigOwner> ErasedWindowConfigPublication for TypedWindowConfigPublication<O> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn window_kind_id(&self) -> &str {
        O::WINDOW_KIND_ID
    }

    fn phase(&self) -> store::ArtifactStoreOneItemPublicationPhase {
        self.publication.as_ref().map_or(if self.closing{store::ArtifactStoreOneItemPublicationPhase::Closing}else{store::ArtifactStoreOneItemPublicationPhase::Preparing},|publication|publication.phase())
    }

    fn fault(&self) -> Option<&str> {
        self.publication.as_ref().and_then(|publication|publication.fault())
    }

    fn acknowledge(&mut self) -> bool {
        let accepted=self.publication.as_mut().is_some_and(|publication|publication.acknowledge());
        if accepted{self.closing=true;}
        accepted
    }

    fn begin_close(&mut self) {
        self.closing=true;
        if let Some(publication)=self.publication.as_mut(){publication.begin_close();}
    }

    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, semio_framework_value::ValueError> {
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication requires its close decision"));}
        let demand=self.close_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.pending.is_empty(){
            if self.publication.is_some()||self.pending.len()!=1||!self.ingress.original_is_untouched(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original window cancellation lost its unconsumed typed carrier"));}
            let original=self.pending.pop().expect("quoted original window mutation remains");
            let carrier=self.ingress.take_original().expect("quoted original carrier remains untouched").restore(original);
            self.cancelled=Some(ControlledRetirement::new(carrier).map_err(|(error,_)|error)?);
        }else if self.pending_bytes()!=0{drop(std::mem::take(&mut *self.pending));}
        else if let Some(actor)=self.actor.as_mut(){if actor.terminal_is_empty(){drop(self.actor.take());}else{return actor.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}}
        else if !self.ingress.terminal_is_empty(){return self.ingress.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        else if let Some(cancelled)=self.cancelled.as_mut(){if cancelled.terminal_is_empty(){drop(self.cancelled.take());}else{return cancelled.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}}
        else if let Some(publication)=self.publication.as_mut(){if publication.terminal_is_empty(){drop(self.publication.take());}else{return publication.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}}
        else{return self.window_id.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
    }

    fn ingress_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:self.ingress.next_copy_byte_demand()?,capacity_bytes:self.ingress.next_capacity_byte_demand(body)?,release_bytes:self.ingress.next_release_byte_demand()?,depth:self.ingress.next_depth_demand()?})}
    fn close_ingress(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.ingress.step(grant)}
    fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.close_demand(body)}
    fn terminal_is_empty(&self) -> bool {
        self.closing&&self.pending.is_empty()&&self.pending_bytes()==0&&self.transaction.is_none()&&self.actor.is_none()&&self.publication.is_none()&&self.ingress.terminal_is_empty()&&self.cancelled.is_none()&&self.window_id.terminal_is_empty()
    }
}

type WindowConfigStore<O> = store::ConfigStore<<O as WindowConfigOwner>::State, <O as WindowConfigOwner>::Mutation>;

struct WindowConfigPartition<O: WindowConfigOwner> {
    store: WindowConfigStore<O>,
    disposer: Option<Box<dyn ArtifactOwnedDisposer<WindowConfigStore<O>>>>,
    pending_preview: Option<WindowConfigSnapshot>,
    pending_preview_address: Option<String>,
    pending_preview_alias: Option<Arc<O::State>>,
    pending_preview_displaced: Option<Vec<Arc<O::State>>>,
    preview_retirement: Option<OriginalAliasBatch<O::State>>,
}

impl<O:WindowConfigOwner> WindowConfigPartition<O>{
    fn preview_retirement_pending(&self)->bool{self.pending_preview.is_some()||self.pending_preview_address.is_some()||self.pending_preview_alias.is_some()||self.pending_preview_displaced.is_some()||self.preview_retirement.is_some()}
    fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.preview_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<OriginalAliasBatch<O::State>>>(),depth:1,..Default::default()})}else{owner.next_demand(WindowConfigStore::<O>::snapshot_alias_retirement_birth_bytes())};}
        if self.pending_preview_alias.is_some()||self.pending_preview_displaced.is_some(){return Ok(OriginalAliasBatch::<O::State>::constructor_demand());}
        if self.pending_preview.is_some(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<WindowConfigSnapshot>>()+std::mem::size_of::<Option<Arc<O::State>>>()+std::mem::size_of::<Option<String>>(),depth:1,..Default::default()});}
        if let Some(address)=self.pending_preview_address.as_ref(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<String>>(),release_bytes:address.capacity(),depth:1,..Default::default()});}
        Ok(Default::default())
    }
    fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.preview_retirement_pending(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.preview_retirement_demand()?;
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(owner)=self.preview_retirement.as_mut(){
            if owner.terminal_is_empty(){drop(self.preview_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            let birth=WindowConfigStore::<O>::snapshot_alias_retirement_birth_bytes();return owner.advance(birth,grant,|alias,child|self.store.retire_snapshot_alias(alias,child));
        }
        if self.pending_preview_alias.is_some(){let(owner,progress)=OriginalAliasBatch::admit_alias_original(&mut self.pending_preview_alias,grant)?.expect("funded original window alias remains");self.preview_retirement=Some(owner);return Ok(RetainedCloneStep::Progress(progress));}
        if self.pending_preview_displaced.is_some(){let(owner,progress)=OriginalAliasBatch::admit_original(&mut self.pending_preview_displaced,grant)?.expect("funded displaced original window aliases remain");self.preview_retirement=Some(owner);return Ok(RetainedCloneStep::Progress(progress));}
        if let Some(preview)=self.pending_preview.take(){
            let WindowConfigSnapshot{window_id,window_kind_id,generation,revision,snapshot}=preview;
            match Arc::downcast::<O::State>(snapshot){Ok(alias)=>{self.pending_preview_alias=Some(alias);self.pending_preview_address=Some(window_id);},Err(snapshot)=>{self.pending_preview=Some(WindowConfigSnapshot{window_id,window_kind_id,generation,revision,snapshot});return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original window preview no longer matches its registered native owner"));}}
        }else{drop(self.pending_preview_address.take());}
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
    }
}

trait ErasedWindowConfigStoreOwner: Send {
    fn capture<'a>(&'a mut self, window_id: &'a str) -> Pin<Box<dyn Future<Output = Result<WindowConfigAuthority, Fault>> + 'a>>;
    fn dispatch<'a, 'b>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>>;
    fn admit_begin(&mut self, operation: semio_framework_job::OperationId, actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, mutations: &mut Vec<WindowConfigMutation>, authority: &WindowConfigAuthority, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedWindowConfigPublication>,RetainedCloneProgress)>,ValueError>;
    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault>;
    fn refresh(&mut self, authority: &mut WindowConfigAuthority) -> Result<(), Fault>;
    fn packs<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WindowConfigPack>, Fault>> + 'a>>;
    fn begin_retained_load(&mut self, registry_lifetime: u64, pack: WindowConfigPack) -> WindowConfigPackLoad;
    fn commit_retained_load(&mut self, registry_lifetime: u64, load: &mut dyn retained::ErasedWindowConfigPackLoad) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic>;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
    fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn close_direct_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn direct_ingress_terminal_is_empty(&self)->bool;
    fn terminal_is_empty(&self) -> bool;
    fn maintenance_retirements_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault>;
    fn maintenance_retirements_terminal_is_empty(&self) -> bool;
    fn maintenance_retirements_under_pressure(&self) -> bool;
    fn pointer_values(&self, pointers: &[String]) -> Vec<(String, Vec<Option<semio_framework_value::DslValue>>)>;
    fn snapshot(&self, window_id: &str) -> Option<WindowConfigSnapshot>;
    fn preview(&mut self, window_id: &str, mutations: &[&WindowConfigMutation]) -> Option<WindowConfigSnapshot>;
    fn preview_available(&self,window_id:&str)->bool;
    fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool;
    fn retire_preview(&mut self,preview:&mut Option<WindowConfigSnapshot>)->bool;
    fn preview_retirement_pending(&self)->bool;
    fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>;
    fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
}

#[derive(semio_framework_value::RetireOwned)]
struct DirectIngress<M:Send+'static>{mutations:Vec<WindowConfigMutation>,allocations:Vec<FactoryBoxedPublication<M>>}
struct TypedWindowConfigStoreOwner<O: WindowConfigOwner> {
    direct_ingress:ControlledRetirement<DirectIngress<O::Mutation>>,
    partitions: WindowRegistry<String, WindowConfigPartition<O>>,
    partition_close_cursor: usize,
    partition_address_retirement: Option<ControlledRetirement<String>>,
    actor: Option<protocol::ActorId>,
    actor_retirement: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
}

impl<O: WindowConfigOwner> TypedWindowConfigStoreOwner<O> {
    async fn partition(&mut self, window_id: &str) -> Result<&mut WindowConfigPartition<O>, Fault> {
        if !self.partitions.contains_key(window_id) {
            let id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
            let envelope = store::create_config_envelope::<O::State, O::Mutation>(O::SCHEMA, &id, O::State::default(), None).await;
            let mut config = store::ConfigStore::new(envelope, self.actor.as_ref().expect("live window owner retains its opened actor").clone()).await.map_err(|error| error.into_fault())?;
            self.partitions.insert(window_id.to_string(), WindowConfigPartition { store: config, disposer: Some(O::build_store_disposer()), pending_preview:None,pending_preview_address:None,pending_preview_alias:None,pending_preview_displaced:None,preview_retirement:None });
        }
        let partition = self.partitions.get_mut(window_id).expect("initialized window config partition remains owned");
        if !partition.store.owned_disposer_installed() {
            store::install_unscheduled_catalog(&mut partition.store, O::build_store_owners()).map_err(|error| error.into_fault())?;
        }
        Ok(partition)
    }
}

impl<O: WindowConfigOwner> ErasedWindowConfigStoreOwner for TypedWindowConfigStoreOwner<O> {
    fn capture<'a>(&'a mut self, window_id: &'a str) -> Pin<Box<dyn Future<Output = Result<WindowConfigAuthority, Fault>> + 'a>> {
        Box::pin(async move {
            let partition = self.partition(window_id).await?;
            Ok(WindowConfigAuthority {
                window_id: window_id.to_string(),
                window_kind_id: O::WINDOW_KIND_ID.to_string(),
                generation: partition.store.generation(),
                revision: partition.store.content_revision_now(),
                snapshot: WindowConfigSnapshot {
                    window_id: window_id.to_string(),
                    window_kind_id: O::WINDOW_KIND_ID,
                    generation: partition.store.generation(),
                    revision: partition.store.content_revision_now(),
                    snapshot: partition.store.snapshot_root(),
                },
            })
        })
    }

    fn dispatch<'a, 'b>(&'a mut self,actor:&'a str,mutation:WindowConfigMutation,identity:&'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>)->Pin<Box<dyn Future<Output=Result<(),Fault>>+'a>>{
        Box::pin(async move{
            if !mutation.mutation.as_any().is::<O::Mutation>(){self.direct_ingress.original_mut().expect("live direct ingress remains original").mutations.push(mutation);return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.address"),"window config mutation owner differs from its registered window"));}
            if self.actor.as_ref().is_none_or(|opened|opened.0.as_str()!=actor){self.direct_ingress.original_mut().expect("live direct ingress remains original").mutations.push(mutation);return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.actor"),"window config mutation actor differs from its opened session"));}
            if let Err(error)=self.partition(&mutation.window_id).await{self.direct_ingress.original_mut().expect("live direct ingress remains original").mutations.push(mutation);return Err(error);}
            let window_id=mutation.window_id;let typed=mutation.mutation.into_any().downcast::<FactoryBoxedValue<O::Mutation>>().expect("registered mutation retains its original typed carrier");let(value,ingress)=typed.take_for_publication();self.direct_ingress.original_mut().expect("live direct ingress remains original").allocations.push(ingress);let command=store::ArtifactCommand::Apply{mutations:vec![value],transaction:None};self.partitions.get_mut(&window_id).expect("original window partition was admitted").store.dispatch(command,identity).await.map_err(|error|error.into_fault())?;Ok(())
        })
    }
    /// 🎟️ Admits one original typed frame and stops before the separately paid Store constructor.
    fn admit_begin(
        &mut self, operation: semio_framework_job::OperationId,
        actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations: &mut Vec<WindowConfigMutation>, authority: &WindowConfigAuthority, grant: RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn ErasedWindowConfigPublication>,RetainedCloneProgress)>,ValueError> {
        let original=mutations.last().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication requires its retained input"))?;
        if original.window_id!=authority.window_id||original.window_kind_id!=O::WINDOW_KIND_ID||authority.window_kind_id!=O::WINDOW_KIND_ID||!original.mutation.as_any().is::<O::Mutation>(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window input differs from its captured owner"));}
        let captured=actor.as_ref().filter(|owner|owner.original_is_untouched()).and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication requires its paid actor capture"))?;
        if self.actor.as_ref().is_none_or(|opened|opened.0.as_str()!=captured.as_str()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window actor differs from its opened session"));}
        let partition=self.partitions.get(&original.window_id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original captured window partition is absent"))?;
        if partition.store.generation()!=authority.generation||partition.store.content_revision_now()!=authority.revision{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window authority is stale"));}
        let frame=std::mem::size_of::<TypedWindowConfigPublication<O>>();
        let copied_bytes=[frame,std::mem::size_of::<WindowConfigMutation>(),std::mem::size_of::<FactoryBoxedValue<O::Mutation>>(),std::mem::size_of::<O::Mutation>()].into_iter().try_fold(0usize,usize::checked_add).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original window constructor copy overflow"))?;
        let retained_capacity_bytes=frame.checked_add(std::mem::size_of::<O::Mutation>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original window constructor capacity overflow"))?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_capacity_bytes<retained_capacity_bytes||grant.maximum_depth==0{return Ok(None);}
        let mut pending=Vec::new();pending.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original window typed input backing allocation failed"))?;
        let WindowConfigMutation{window_id,window_kind_id:_,mutation}=mutations.pop().expect("quoted original window input remains");
        let typed=mutation.into_any().downcast::<FactoryBoxedValue<O::Mutation>>().expect("quoted original native mutation carrier remains");
        let(original,ingress)=typed.take_for_publication();pending.push(original);
        let publication=TypedWindowConfigPublication::<O>{window_id:ControlledRetirement::new(window_id).expect("native window address supports controlled retirement"),operation,generation:authority.generation,revision:authority.revision,actor:actor.take(),pending:std::mem::ManuallyDrop::new(pending),transaction:None,publication:std::mem::ManuallyDrop::new(None),ingress:ControlledRetirement::new(ingress).map_err(|(error,_)|error).expect("native carrier has genuine controlled facets"),cancelled:None,closing:false};
        Ok(Some((Box::new(publication),RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes,released_bytes:0})))
    }

    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault> {
        let publication = publication
            .as_any_mut()
            .downcast_mut::<TypedWindowConfigPublication<O>>()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.publication-type"), "window config publication did not match its registered window owner"))?;
        if publication.closing{return Ok(store::ArtifactStoreOneItemAdvance::Blocked);}
        let address=publication.window_id.original().ok_or_else(||Fault::from("original window publication lost its captured address"))?;
        let partition=self.partitions.get_mut(address).ok_or_else(||Fault::from("original window publication lost its exact partition"))?;
        if publication.publication.is_none(){
            return match partition.store.admit_member_apply_batch(publication.operation,publication.generation,publication.revision,&mut publication.actor,&mut publication.pending,&mut publication.transaction,grant.retained_grant()).map_err(|error|error.into_fault())?{
                Some((owner,progress))=>{*publication.publication=Some(owner);Ok(store::ArtifactStoreOneItemAdvance::PreparationProgress(Default::default(),progress))},
                None=>Ok(store::ArtifactStoreOneItemAdvance::Blocked),
            };
        }
        let result=partition.store.advance_apply_batch(publication.publication.as_mut().expect("admitted original Store publication remains"),grant).map_err(|error|error.into_fault());
        if publication.publication.as_ref().is_some_and(|owner|owner.phase()==store::ArtifactStoreOneItemPublicationPhase::Closing){publication.closing=true;}
        result
    }

    fn refresh(&mut self, authority: &mut WindowConfigAuthority) -> Result<(), Fault> {
        let partition = self.partitions.get(&authority.window_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.partition"), "window config authority lost its exact partition"))?;
        authority.generation = partition.store.generation();
        authority.revision = partition.store.content_revision_now();
        Ok(())
    }

    fn packs<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WindowConfigPack>, Fault>> + 'a>> {
        Box::pin(async move {
            let mut packs = Vec::with_capacity(self.partitions.len());
            for (window_id, partition) in self.partitions.iter() {
                let files = store::print_document_pack(partition.store.envelope()).await.map_err(|error| error.into_fault())?;
                packs.push(WindowConfigPack { window_id: window_id.clone(), window_kind_id: O::WINDOW_KIND_ID.to_string(), files });
            }
            Ok(packs)
        })
    }

    fn begin_retained_load(&mut self, registry_lifetime: u64, pack: WindowConfigPack) -> WindowConfigPackLoad {
        let partition_generation = self.partitions.get(&pack.window_id).map(|partition| partition.store.generation());
        retained::begin_typed_window_config_pack_load::<O>(registry_lifetime, partition_generation, pack, self.actor.as_ref().expect("live window owner retains its opened actor").clone())
    }

    fn commit_retained_load(&mut self, registry_lifetime: u64, load: &mut dyn retained::ErasedWindowConfigPackLoad) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic> {
        retained::commit_typed_window_config_pack_load::<O>(registry_lifetime, &mut self.partitions, load)
    }

    fn snapshot(&self, window_id: &str) -> Option<WindowConfigSnapshot> {
        let partition = self.partitions.get(window_id)?;
        Some(WindowConfigSnapshot { window_id: window_id.to_string(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation(), revision: partition.store.content_revision_now(), snapshot: partition.store.snapshot_root() })
    }

    fn preview(&mut self, window_id: &str, mutations: &[&WindowConfigMutation]) -> Option<WindowConfigSnapshot> {
        let partition = self.partitions.get_mut(window_id)?;
        if partition.preview_retirement_pending(){return None;}
        let committed = partition.store.snapshot_owner();
        let (mut running, mut displaced) = (None, Vec::new());
        for mutation in mutations.iter().filter_map(|mutation| mutation.mutation.as_any().downcast_ref::<O::Mutation>()) {
            super::app::tool_machine::fold_leaf(&committed, &mut running, &mut displaced, mutation);
        }
        if !displaced.is_empty()||displaced.capacity()!=0{partition.pending_preview_displaced=Some(displaced);}
        let state: Arc<O::State> = running?;
        Some(WindowConfigSnapshot { window_id: window_id.to_string(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation(), revision: partition.store.content_revision_now(), snapshot: state })
    }

    fn preview_available(&self,window_id:&str)->bool{self.partitions.get(window_id).is_some_and(|partition|!partition.preview_retirement_pending())}
    fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool{preview.window_kind_id==O::WINDOW_KIND_ID&&preview.snapshot.is::<O::State>()&&self.partitions.get(&preview.window_id).is_some_and(|partition|partition.pending_preview.is_none()&&partition.pending_preview_alias.is_none()&&partition.pending_preview_address.is_none())}
    fn retire_preview(&mut self,preview:&mut Option<WindowConfigSnapshot>)->bool{
        let Some(original)=preview.as_ref()else{return true;};
        if !self.can_retire_preview(original){return false;}
        let partition=self.partitions.get_mut(&original.window_id).expect("checked original window partition remains");partition.pending_preview=preview.take();true
    }
    fn preview_retirement_pending(&self)->bool{self.partitions.values().any(WindowConfigPartition::preview_retirement_pending)}
    fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>{self.partitions.values().find(|partition|partition.preview_retirement_pending()).map_or(Ok(Default::default()),WindowConfigPartition::preview_retirement_demand)}
    fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}self.partitions.values_mut().find(|partition|partition.preview_retirement_pending()).map_or(Ok(RetainedCloneStep::Complete(Default::default())),|partition|partition.preview_retirement_step(grant))}

    fn pointer_values(&self, pointers: &[String]) -> Vec<(String, Vec<Option<semio_framework_value::DslValue>>)> {
        self.partitions
            .iter()
            .map(|(window_id, partition)| {
                let document = semio_framework_value::ToValue::to_value(partition.store.snapshot_owner().as_ref());
                (window_id.clone(), pointers.iter().map(|pointer| semio_framework_tool_run::tool_run_pointer_value(&document, pointer).cloned()).collect())
            })
            .collect()
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.direct_ingress.terminal_is_empty() { return self.direct_ingress_demands(body); }
        if let Some(owner) = self.partition_address_retirement.as_ref() { return controlled_owner_demands(owner, body); }
        if let Some(partition) = self.partitions.get_index(self.partition_close_cursor) {
            if partition.preview_retirement_pending() { return partition.preview_retirement_demand(); }
            return partition.disposer.as_ref().map_or(Ok(RetirementDemand { copy_bytes: std::mem::size_of::<usize>(), depth: 1, ..Default::default() }), |disposer| {
                if disposer.terminal_is_empty(&partition.store) { Ok(RetirementDemand { release_bytes: std::mem::size_of_val(disposer.as_ref()), depth: 1, ..Default::default() }) }
                else { disposer.retirement_demands(&partition.store, body) }
            });
        }
        if !self.partitions.is_empty() { return self.partitions.pop_demand(); }
        if !self.partitions.terminal_is_empty() { return self.partitions.backing_demand(); }
        window_actor_demands(&self.actor, &self.actor_retirement, body)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        if !registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if !self.direct_ingress.terminal_is_empty() { return self.direct_ingress.step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
        if let Some(owner) = self.partition_address_retirement.as_mut() {
            let step = owner.step(grant).map_err(ValueError::into_fault)?;
            if owner.terminal_is_empty() { self.partition_address_retirement = None; }
            return Ok(PluginLifecycleStep::retained(step, false));
        }
        if let Some(partition) = self.partitions.get_index_mut(self.partition_close_cursor) {
            if partition.preview_retirement_pending() { return partition.preview_retirement_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
            if let Some(disposer) = partition.disposer.as_mut() {
                if disposer.terminal_is_empty(&partition.store) {
                    drop(partition.disposer.take());
                    return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
                }
                let step = disposer.close_step(&mut partition.store, grant)?;
                return Ok(match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other });
            }
            self.partition_close_cursor += 1;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if !self.partitions.is_empty() {
            let Some(((address, partition), progress)) = self.partitions.pop_original(grant).map_err(ValueError::into_fault)? else { return Ok(PluginLifecycleStep::Progress(Default::default())); };
            assert!(partition.disposer.is_none() && !partition.preview_retirement_pending());
            drop(partition);
            self.partition_address_retirement = Some(ControlledRetirement::new(address).map_err(|(error, _)| error).expect("original address has controlled retirement"));
            return Ok(PluginLifecycleStep::Progress(progress));
        }
        if !self.partitions.terminal_is_empty() { return self.partitions.close_backing_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
        close_window_actor(&mut self.actor, &mut self.actor_retirement, grant)
    }

    fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:self.direct_ingress.next_copy_byte_demand()?,capacity_bytes:self.direct_ingress.next_capacity_byte_demand(body)?,release_bytes:self.direct_ingress.next_release_byte_demand()?,depth:self.direct_ingress.next_depth_demand()?})}
    fn close_direct_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.direct_ingress.step(grant)}
    fn direct_ingress_terminal_is_empty(&self)->bool{self.direct_ingress.terminal_is_empty()}
    fn terminal_is_empty(&self) -> bool {
        self.partitions.terminal_is_empty() && self.partition_address_retirement.is_none() && self.direct_ingress.terminal_is_empty() && self.actor.is_none() && self.actor_retirement.is_none()
    }

    /// 🧹️ Retires the displaced owners a LIVE partition accumulates — every coalesced amend (a
    /// playback tick, a gumball flag) displaces the previous snapshot, edit id and envelope into the
    /// partition store's fixed 1 024-slot retirement queue, and only a maintenance pump gives those
    /// slots back. One partition per step, the first that still owes work.
    fn maintenance_retirements_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.partitions.values().find(|partition| !partition.store.maintenance_retirements_terminal_is_empty()).map_or(Ok(Default::default()), |partition| partition.store.maintenance_retirements_demands(body))
    }
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let Some(partition) = self.partitions.values_mut().find(|partition| !partition.store.maintenance_retirements_terminal_is_empty()) else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        partition.store.maintenance_retirements_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|message| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.maintenance-retirement"), message.into_message()))
    }

    fn maintenance_retirements_terminal_is_empty(&self) -> bool {
        self.partitions.values().all(|partition| partition.store.maintenance_retirements_terminal_is_empty())
    }

    fn maintenance_retirements_under_pressure(&self) -> bool {
        self.partitions.values().any(|partition| partition.store.maintenance_retirements_under_pressure())
    }
}

/// ⏳️ Turns one registry-driven retained window config load may spend before it faults `window-config.load-bound`.
pub const WINDOW_CONFIG_PACK_LOAD_TURNS: usize = 1_048_576;

/// 🗂️ Runtime registry of heterogeneous persisted-local window config schemas.
pub struct WindowConfigOwnerRegistry {
    owners: WindowRegistry<&'static str, Box<dyn ErasedWindowConfigStoreOwner>>,
    owner_close_cursor: usize,
    retiring: semio_framework_value::list::PagedList<WindowConfigPackLoad, {usize::MAX}>,
    lifetime: u64,
    actor: Option<protocol::ActorId>,
    actor_retirement: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
}

fn controlled_owner_demands<T: semio_framework_value::retirement::RetireOwned>(owner: &ControlledRetirement<T>, body: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? })
}
fn window_actor_demands(actor: &Option<protocol::ActorId>, retirement: &Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, body: usize) -> Result<RetirementDemand, ValueError> {
    retirement.as_ref().map_or(Ok(RetirementDemand { copy_bytes: if actor.is_some() { std::mem::size_of::<protocol::ActorId>() } else { 0 }, depth: usize::from(actor.is_some()), ..Default::default() }), |owner| controlled_owner_demands(owner, body))
}
fn close_window_actor(actor: &mut Option<protocol::ActorId>, retirement: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
    if actor.is_none() && retirement.is_none() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
    let demand = window_actor_demands(actor, retirement, grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
    if !registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
    if let Some(actor) = actor.take() {
        *retirement = Some(ControlledRetirement::new(actor.0).map_err(|(error, _)| error).expect("original actor address has controlled retirement"));
        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
    }
    let owner = retirement.as_mut().expect("original actor retirement remains");
    let step = owner.step(grant).map_err(ValueError::into_fault)?;
    if owner.terminal_is_empty() { *retirement = None; }
    Ok(PluginLifecycleStep::retained(step, retirement.is_none()))
}

impl WindowConfigOwnerRegistry {
    pub fn new(actor: protocol::ActorId) -> Self {
        static NEXT_LIFETIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self { owners: WindowRegistry::new(), owner_close_cursor: 0, retiring: Default::default(), lifetime: NEXT_LIFETIME.fetch_add(1, std::sync::atomic::Ordering::Relaxed), actor: Some(actor), actor_retirement: None }
    }

    pub fn local_actor_id(&self) -> &protocol::ActorId {
        self.actor.as_ref().expect("live window registry retains its opened actor")
    }

    pub fn register<O: WindowConfigOwner>(&mut self) -> Result<(), Fault> {
        if O::WINDOW_KIND_ID.is_empty() || O::SCHEMA.is_empty() || self.owners.contains_key(O::WINDOW_KIND_ID) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config owner identity is empty or already registered"));
        }
        self.owners.insert(O::WINDOW_KIND_ID, Box::new(TypedWindowConfigStoreOwner::<O> { direct_ingress:ControlledRetirement::new(DirectIngress{mutations:Vec::new(),allocations:Vec::new()}).map_err(|(error,_)|error).expect("direct ingress owns genuine facets"),partitions: WindowRegistry::new(), partition_close_cursor: 0, partition_address_retirement: None, actor: Some(self.local_actor_id().clone()), actor_retirement: None }));
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    /// 📸️ The current config snapshot of window `window_id` of kind `window_kind_id`, without creating its partition.
    pub fn snapshot(&self, window_kind_id: &str, window_id: &str) -> Option<WindowConfigSnapshot> {
        self.owners.get(window_kind_id)?.snapshot(window_id)
    }

    /// 🪞️ Window `window_id`'s committed config with `mutations` folded on it and never published — a press's provisional
    /// window config (design §20.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); `None` when none applies or the window
    /// has no partition yet. Hand it back to [`Self::retire_preview`], never drop it plainly.
    pub(crate) fn preview(&mut self, window_kind_id: &str, window_id: &str, mutations: &[&WindowConfigMutation]) -> Option<WindowConfigSnapshot> {
        self.owners.get_mut(window_kind_id)?.preview(window_id, mutations)
    }

    /// 🧹️ Retires a [`Self::preview`] through its partition's store.
    pub(crate) fn retire_preview(&mut self,preview:&mut Option<WindowConfigSnapshot>)->bool{
        let Some(original)=preview.as_ref()else{return true;};
        self.owners.get_mut(original.window_kind_id).is_some_and(|owner|owner.retire_preview(preview))
    }
    pub(crate) fn preview_available(&self,kind:&str,window:&str)->bool{self.owners.get(kind).is_some_and(|owner|owner.preview_available(window))}
    pub(crate) fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool{self.owners.get(preview.window_kind_id).is_some_and(|owner|owner.can_retire_preview(preview))}
    pub(crate) fn preview_retirement_pending(&self)->bool{self.owners.values().any(|owner|owner.preview_retirement_pending())}
    pub(crate) fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>{self.owners.values().find(|owner|owner.preview_retirement_pending()).map_or(Ok(Default::default()),|owner|owner.preview_retirement_demand())}
    pub(crate) fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}self.owners.values_mut().find(|owner|owner.preview_retirement_pending()).map_or(Ok(RetainedCloneStep::Complete(Default::default())),|owner|owner.preview_retirement_step(grant))}

    /// ⏯️ The values `pointers` (RFC 6901) name in every window config partition of `window_kind_id`, by window id
    /// in window id order — what a tool run's declared window config reads compare across publications.
    pub fn pointer_values(&self, window_kind_id: &str, pointers: &[String]) -> Vec<(String, Vec<Option<semio_framework_value::DslValue>>)> {
        self.owners.get(window_kind_id).map_or_else(Vec::new, |owner| owner.pointer_values(pointers))
    }

    /// 🪟️ The window whose config this call speaks for: the addressed instance, and for a PANEL
    /// projection — which `ViewModel::for_panel` deliberately strips of `window_id` — the shell's
    /// focused pane. A panel's controls are addressed at `focused_window_id` (that is the only carrier of
    /// "which window is the user looking at" a panel is given), so handing the panel no window config at
    /// all made every panel render a per-window option from `Default` while writing to the focused pane:
    /// the puzzle3d Settings panel displayed spacing 10.0 and bumped it to 10.5 against a pane whose rail
    /// stood at 12.5 (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B36). A surface reads exactly the state
    /// it writes.
    pub(crate) async fn capture(&mut self, view_state: Option<&ViewModel>) -> Result<Option<WindowConfigAuthority>, Fault> {
        let Some(view_state) = view_state else { return Ok(None) };
        let Some(window_id) = view_state.window_id.as_deref().or(view_state.focused_window_id.as_deref()) else { return Ok(None) };
        let window = view_state
            .window_instances
            .iter()
            .find(|window| window.id == window_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.window-context"), "target window is absent from the exact ViewModel window instance roster"))?;
        let Some(owner) = self.owners.get_mut(window.window_kind_id.as_str()) else { return Ok(None) };
        owner.capture(window_id).await.map(Some)
    }

    pub(crate) async fn dispatch(&mut self, authority: &WindowConfigAuthority, actor: &str, mutation: WindowConfigMutation, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.validate_address(authority, &mutation)?;
        self.owners.get_mut(authority.window_kind_id.as_str()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config emission has no registered concrete window owner"))?.dispatch(actor, mutation, identity).await
    }

    pub(crate) fn admit_begin(
        &mut self, operation: semio_framework_job::OperationId,
        actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations: &mut Vec<WindowConfigMutation>, authority: &WindowConfigAuthority, grant: RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn ErasedWindowConfigPublication>,RetainedCloneProgress)>,ValueError> {
        let original=mutations.last().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication input is absent"))?;
        if original.window_id!=authority.window_id||original.window_kind_id!=authority.window_kind_id{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication address differs from its captured authority"));}
        let owner=self.owners.get_mut(authority.window_kind_id.as_str()).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original window publication has no installed native owner"))?;
        owner.admit_begin(operation,actor,mutations,authority,grant)
    }

    pub(crate) fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault> {
        self.owners.get_mut(publication.window_kind_id()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config publication lost its registered concrete window owner"))?.advance(publication, grant)
    }

    pub(crate) fn refresh(&mut self, authority: &mut WindowConfigAuthority) -> Result<(), Fault> {
        self.owners.get_mut(authority.window_kind_id.as_str()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config authority lost its registered concrete window owner"))?.refresh(authority)
    }

    pub async fn packs(&self) -> Result<Vec<WindowConfigPack>, Fault> {
        let mut packs = Vec::new();
        for owner in self.owners.values() {
            packs.extend(owner.packs().await?);
        }
        Ok(packs)
    }

    pub async fn load(&mut self, pack: WindowConfigPack) -> Result<(), Fault> {
        self.load_within(pack, WINDOW_CONFIG_PACK_LOAD_TURNS).await
    }

    /// ⏳️ Drives one retained load under exact per-turn demand grants for at most `turns` turns. Exhausting the bound
    /// is a typed `window-config.load-bound` fault, never `Ok`; a load that cannot retire within the bound is
    /// parked on the registry, whose bounded close retires it, so no load reaches `Drop` unretired.
    pub(crate) async fn load_within(&mut self, pack: WindowConfigPack, turns: usize) -> Result<(), Fault> {
        let mut load = self.begin_retained_load(pack)?;
        let mut rejected = None;
        let mut settled = false;
        for _ in 0..turns {
            let grant = load.next_grant();
            if rejected.is_some() || matches!(load.phase(), WindowConfigPackLoadPhase::RetiringDisplacedStore) {
                match self.close_retained_load_step(&mut load, grant) {
                    Ok(PluginLifecycleStep::Complete(_)) if load.terminal_is_empty() => {
                        settled = true;
                        break;
                    }
                    Ok(_) => continue,
                    Err(fault) => return Err(self.park_retained_load(load, fault)),
                }
            }
            match self.advance_retained_load(&mut load, grant) {
                WindowConfigPackLoadStep::Pending(_) => {}
                WindowConfigPackLoadStep::Ready => {
                    if let WindowConfigPackLoadStep::Rejected(diagnostic) = self.commit_retained_load(&mut load) {
                        rejected = Some(diagnostic);
                    }
                }
                WindowConfigPackLoadStep::Rejected(diagnostic) => rejected = Some(diagnostic),
                WindowConfigPackLoadStep::Complete => {
                    settled = true;
                    break;
                }
            }
        }
        let fault = rejected.map_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.load-bound"), "retained window config load exhausted its declared turn bound before it settled"), Self::load_fault);
        if !settled {
            load.request_cancel();
            for _ in 0..turns {
                let grant = load.next_grant();
                match self.close_retained_load_step(&mut load, grant) {
                    Ok(PluginLifecycleStep::Complete(_)) if load.terminal_is_empty() => break,
                    Ok(_) => {}
                    Err(retirement) => return Err(self.park_retained_load(load, retirement)),
                }
            }
            return Err(if load.terminal_is_empty() { fault } else { self.park_retained_load(load, fault) });
        }
        match rejected {
            Some(_) => Err(fault),
            None => Ok(()),
        }
    }

    fn park_retained_load(&mut self, load: WindowConfigPackLoad, fault: Fault) -> Fault {
        self.retiring.push(load);
        fault
    }

    /// 📥️ Begins a retained exact-partition Pack load without changing live authority.
    pub fn begin_retained_load(&mut self, pack: WindowConfigPack) -> Result<WindowConfigPackLoad, Fault> {
        let kind = pack.window_kind_id.clone();
        self.owners
            .get_mut(kind.as_str())
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config pack has no registered concrete window owner"))
            .map(|owner| owner.begin_retained_load(self.lifetime, pack))
    }

    /// 🧵️ Advances one bounded retained decode or hydration unit.
    pub fn advance_retained_load(&mut self, load: &mut WindowConfigPackLoad, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        if load.inner.registry_lifetime() != self.lifetime || !self.owners.contains_key(load.inner.window_kind_id()) {
            return load.inner.reject_stale();
        }
        load.inner.advance(grant)
    }

    /// 🔐️ Publishes a ready candidate only if its registry and exact-partition witnesses remain current.
    pub fn commit_retained_load(&mut self, load: &mut WindowConfigPackLoad) -> WindowConfigPackLoadStep {
        if load.inner.registry_lifetime() != self.lifetime {
            return load.inner.reject_stale();
        }
        let kind = load.inner.window_kind_id().to_string();
        let Some(owner) = self.owners.get_mut(kind.as_str()) else { return load.inner.reject_stale() };
        match owner.commit_retained_load(self.lifetime, load.inner.as_mut()) {
            Ok(step) => step,
            Err(_) => load.inner.reject_stale(),
        }
    }

    /// ♻️ Retires a cancelled, rejected, committed, or displaced load candidate under an exact grant.
    pub fn close_retained_load_step(&mut self, load: &mut WindowConfigPackLoad, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        load.inner.close_step(grant).map_err(|message| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.load-retirement"), message))
    }

    fn load_fault(diagnostic: WindowConfigPackLoadDiagnostic) -> Fault {
        let code = match diagnostic {
            WindowConfigPackLoadDiagnostic::EnvelopeIdentity => "window-config.pack-envelope",
            WindowConfigPackLoadDiagnostic::Pack => "window-config.pack",
            WindowConfigPackLoadDiagnostic::TypedState => "window-config.typed-state",
            WindowConfigPackLoadDiagnostic::History => "window-config.history",
            WindowConfigPackLoadDiagnostic::InnerIdentity => "window-config.inner-identity",
            WindowConfigPackLoadDiagnostic::Replay => "window-config.replay",
            WindowConfigPackLoadDiagnostic::Capacity => "window-config.capacity",
            WindowConfigPackLoadDiagnostic::Stale => "window-config.stale",
            WindowConfigPackLoadDiagnostic::Cancelled => "window-config.cancelled",
            WindowConfigPackLoadDiagnostic::Retirement => "window-config.retirement",
        };
        Fault::new(FaultOrigin::Framework, FaultCode::new(code), "retained exact window config Pack load was rejected")
    }

    /// 📏️ The exact physical release the next [`Self::close_step`] owes: the open owner's first partition store, or its opened actor's retirement. Parked loads pay their own demand.
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(load) = self.retiring.last() {
            if !load.terminal_is_empty() { return load.inner.retirement_demands(body); }
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<WindowConfigPackLoad>(), release_bytes: std::mem::size_of_val(load.inner.as_ref()), depth: self.retiring.next_pop_depth_demand().map_err(|error| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason))?, ..Default::default() });
        }
        if !self.retiring.terminal_is_empty() {
            return Ok(RetirementDemand { release_bytes: self.retiring.next_release_allocation_bytes().map_err(|error| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason))?, depth: self.retiring.next_release_depth_demand().map_err(|error| ValueError::literal(ValueRefusalKind::DepthLimit, error.reason))?, ..Default::default() });
        }
        if let Some(owner) = self.owners.get_index(self.owner_close_cursor) {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<usize>(), depth: 1, ..Default::default() }); }
            return owner.retirement_demands(body);
        }
        if let Some((_, owner)) = self.owners.last() {
            let mut demand = self.owners.pop_demand()?;
            demand.release_bytes = std::mem::size_of_val(owner.as_ref());
            return Ok(demand);
        }
        if !self.owners.terminal_is_empty() { return self.owners.backing_demand(); }
        window_actor_demands(&self.actor, &self.actor_retirement, body)
    }

    /// ♻️ Retires parked loads first, each under its own exact release demand (paying back credits it was admitted), then owners.
    pub(crate) fn direct_ingress_terminal_is_empty(&self)->bool{self.owners.values().all(|owner|owner.direct_ingress_terminal_is_empty())}
    pub(crate) fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owners.values().find(|owner|!owner.direct_ingress_terminal_is_empty()).map_or(Ok(Default::default()),|owner|owner.direct_ingress_demands(body))}
    pub(crate) fn close_direct_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match self.owners.values_mut().find(|owner|!owner.direct_ingress_terminal_is_empty()){Some(owner)=>owner.close_direct_ingress(grant).map(|step|RetainedCloneStep::Progress(step.progress())),None=>Ok(RetainedCloneStep::Complete(Default::default()))}}
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        if !registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if let Some(load) = self.retiring.last_mut() {
            if load.terminal_is_empty() {
                drop(self.retiring.pop());
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }));
            }
            return load.inner.close_step(grant).map(|step| match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other }).map_err(|message| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.load-retirement"), message));
        }
        if !self.retiring.terminal_is_empty() {
            let step = self.retiring.release_empty_page(grant.maximum_release_bytes).map_err(|error| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason).into_fault())?;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: usize::from(step.progressed), released_bytes: step.released_allocation_bytes, ..Default::default() }));
        }
        if let Some(owner) = self.owners.get_index_mut(self.owner_close_cursor) {
            if owner.terminal_is_empty() {
                self.owner_close_cursor += 1;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            return owner.close_step(grant).map(|step| match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other });
        }
        if !self.owners.is_empty() {
            let Some(((kind, owner), mut progress)) = self.owners.pop_original(grant).map_err(ValueError::into_fault)? else { return Ok(PluginLifecycleStep::Progress(Default::default())); };
            assert!(owner.terminal_is_empty());
            drop((kind, owner));
            progress.released_bytes = demand.release_bytes;
            return Ok(PluginLifecycleStep::Progress(progress));
        }
        if !self.owners.terminal_is_empty() { return self.owners.close_backing_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
        close_window_actor(&mut self.actor, &mut self.actor_retirement, grant)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty() && self.retiring.terminal_is_empty() && self.actor.is_none() && self.actor_retirement.is_none()
    }

    /// 🧹️ One maintenance step over the live partitions' displaced-owner queues — the first owner
    /// that still owes retirements advances; `Complete` means every partition is drained.
    pub(crate) fn maintenance_retirements_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.owners.values().find(|owner| !owner.maintenance_retirements_terminal_is_empty()).map_or(Ok(Default::default()), |owner| owner.maintenance_retirements_demands(body))
    }
    pub(crate) fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let Some(owner) = self.owners.values_mut().find(|owner| !owner.maintenance_retirements_terminal_is_empty()) else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        owner.maintenance_retirements_step(grant).map(|step| RetainedCloneStep::Progress(step.progress()))
    }

    pub(crate) fn maintenance_retirements_terminal_is_empty(&self) -> bool {
        self.owners.values().all(|owner| owner.maintenance_retirements_terminal_is_empty())
    }

    /// 🌡️ True while any live partition's displaced-owner queue sits at or above the store's pressure mark.
    pub(crate) fn maintenance_retirements_under_pressure(&self) -> bool {
        self.owners.values().any(|owner| owner.maintenance_retirements_under_pressure())
    }

    fn validate_address(&self, authority: &WindowConfigAuthority, mutation: &WindowConfigMutation) -> Result<(), Fault> {
        if mutation.window_id != authority.window_id || mutation.window_kind_id != authority.window_kind_id {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.address"), "window config emission does not match the operation's exact captured window authority"));
        }
        Ok(())
    }
}
