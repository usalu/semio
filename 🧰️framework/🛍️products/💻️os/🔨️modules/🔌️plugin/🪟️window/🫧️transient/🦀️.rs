//! 🫧️ Exact-window transient state with independent generation authority.

use super::app::{ArtifactOwnedDisposer, PluginCloseStep};
use super::transient_publication::transient_store_disposer;
use crate::{protocol, store};
use semio_framework::{Fault, FaultCode, FaultOrigin, ViewModel};
use std::any::Any;
use super::window_mutation::ErasedWindowMutationValue;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::collections::BTreeMap;
use std::sync::Arc;

/// 🪟️ Declares the transient schema and bounded owners of one concrete window kind.
pub trait WindowTransientOwner: Send + Sync + 'static {
    const WINDOW_KIND_ID: &'static str;
    type State: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + store::ArtifactDsl + store::ArtifactPack + 'static;
    type Mutation: protocol::Mutation<Self::State> + PartialEq + Send + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + 'static;

    fn build_mutation_retirement_factory()->Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>>{Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default())}
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
    window_id: String,
    window_kind_id: &'static str,
    mutation: Box<dyn ErasedWindowMutationValue>,
}

impl std::fmt::Debug for WindowTransientMutation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("WindowTransientMutation").field("window_id", &self.window_id).field("window_kind_id", &self.window_kind_id).finish_non_exhaustive()
    }
}

/// 🙅️ A window transient emission the registry refused, with its mutation handed back: a refusal is the caller's
/// typed fault, never a mutation silently dropped.
#[derive(Debug)]
pub(crate) struct RejectedWindowTransientEmission {
    pub mutation: WindowTransientMutation,
    pub fault: Fault,
}

impl WindowTransientMutation {
    pub fn of<O: WindowTransientOwner>(window_id: impl Into<String>, mutation: O::Mutation) -> Self {
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

/// 📖️ Immutable snapshot of one exact window-owned transient partition.
#[derive(Clone)]
pub struct WindowTransientSnapshot {
    window_id: String,
    window_kind_id: &'static str,
    generation: u64,
    document_generation: u64,
    snapshot: Arc<store::ErasedSnapshotRead>,
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

#[derive(Clone)]
pub(crate) struct WindowTransientAuthority {
    pub window_id: String,
    pub window_kind_id: String,
    pub generation: u64,
    pub snapshot: WindowTransientSnapshot,
}

pub(crate) trait ErasedWindowTransientPublication: Send {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn window_kind_id(&self) -> &str;
    fn document_generation(&self) -> u64;
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

struct TypedWindowTransientPublication<O: WindowTransientOwner> {
    window_id: String,
    document_generation: u64,
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

    fn phase(&self) -> store::ArtifactStoreOneItemPublicationPhase {
        self.publication.phase()
    }

    fn fault(&self) -> Option<&str> {
        self.publication.fault()
    }

    fn acknowledge(&mut self) -> bool {
        self.publication.acknowledge()
    }

    fn begin_close(&mut self) {
        self.publication.begin_close();
    }

    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, semio_framework_value::ValueError> {
        if !self.ingress.terminal_is_empty(){return self.ingress.step(grant).map(|step|semio_framework_value::RetainedCloneStep::Progress(step.progress()));}
        self.publication.close_step(grant)
    }

    fn ingress_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:self.ingress.next_copy_byte_demand()?,capacity_bytes:self.ingress.next_capacity_byte_demand(body)?,release_bytes:self.ingress.next_release_byte_demand()?,depth:self.ingress.next_depth_demand()?})}
    fn close_ingress(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.ingress.step(grant)}
    fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if !self.ingress.terminal_is_empty(){self.ingress_demands(body)}else{self.publication.retirement_demands(body)}}
    fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty()&&self.ingress.terminal_is_empty()
    }
}

type WindowTransientStore<O> = store::TransientStore<<O as WindowTransientOwner>::State, <O as WindowTransientOwner>::Mutation>;

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
    fn refresh(&mut self, authority: &mut WindowTransientAuthority) -> Result<(), Fault>;
    fn begin(&mut self, operation: semio_framework_job::OperationId, expected_generation: u64, document_generation: u64, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, RejectedWindowTransientEmission>;
    fn advance(&mut self, publication: &mut dyn ErasedWindowTransientPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault>;
    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>;
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>;
    fn terminal_is_empty(&self) -> bool;
}

struct TypedWindowTransientStoreOwner<O: WindowTransientOwner> {
    partitions: BTreeMap<String, WindowTransientPartition<O>>,
    owners: WindowTransientOwnerBundle<O::State, O::Mutation>,
    maintenance_cursor: Option<String>,
    retirement_cursor: Option<String>,
}

impl<O: WindowTransientOwner> TypedWindowTransientStoreOwner<O> {
    fn partition(&mut self, window_id: &str) -> &mut WindowTransientPartition<O> {
        let factory = self.owners.state_retirement.clone();
        self.partitions.entry(window_id.to_string()).or_insert_with(|| WindowTransientPartition::<O>::new(factory))
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
            snapshot: WindowTransientSnapshot { window_id: window_id.to_string(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation_now(), document_generation, snapshot: Arc::new(snapshot) },
        })
    }

    fn refresh(&mut self, authority: &mut WindowTransientAuthority) -> Result<(), Fault> {
        let partition = self.partition(&authority.window_id);
        authority.generation = partition.store.generation_now();
        let snapshot = partition.store.current_read_erased().map_err(Fault::from)?;
        authority.snapshot =
            WindowTransientSnapshot { window_id: authority.window_id.clone(), window_kind_id: O::WINDOW_KIND_ID, generation: authority.generation, document_generation: authority.snapshot.document_generation, snapshot: Arc::new(snapshot) };
        Ok(())
    }

    fn begin(&mut self, operation: semio_framework_job::OperationId, expected_generation: u64, document_generation: u64, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, RejectedWindowTransientEmission> {
        let WindowTransientMutation { window_id, window_kind_id, mutation } = mutation;
        if !mutation.as_any().is::<O::Mutation>(){return Err(RejectedWindowTransientEmission{mutation:WindowTransientMutation{window_id,window_kind_id,mutation},fault:Fault::new(FaultOrigin::Framework,FaultCode::new("window-transient.mutation-type"),"window transient mutation did not match its registered window owner")});}
        let typed=mutation.into_any().downcast::<FactoryBoxedValue<O::Mutation>>().expect("registered payload carrier preserves its typed owner");
        let (typed,ingress)=typed.take_for_publication();
        let preparation = self.owners.preparation.clone();
        let retirement = self.owners.state_retirement.clone();
        let partition = self.partition(&window_id);
        match partition.store.begin_publish_one_leased(operation, expected_generation, typed, preparation.as_ref(), retirement) {
            Ok(publication) => Ok(Box::new(TypedWindowTransientPublication::<O> { window_id, document_generation, publication,ingress:ControlledRetirement::new(ingress).map_err(|(error,_)|error).expect("typed ingress owns genuine facets") })),
            Err(rejected) => {
                let (reason, mutation) = rejected.into_owners();
                let fault = Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.admission"), reason);
                Err(RejectedWindowTransientEmission { mutation: WindowTransientMutation { window_id, window_kind_id, mutation: ingress.restore(mutation) }, fault })
            }
        }
    }

    fn advance(&mut self, publication: &mut dyn ErasedWindowTransientPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault> {
        let publication = publication
            .as_any_mut()
            .downcast_mut::<TypedWindowTransientPublication<O>>()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.publication-type"), "window transient publication did not match its registered window owner"))?;
        self.partition(&publication.window_id).store.advance_publish_one(&mut publication.publication, grant).map_err(|reason| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.publication"), reason))
    }

    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let next = self.maintenance_cursor.as_ref().and_then(|cursor| self.partitions.range::<str, _>((std::ops::Bound::Excluded(cursor.as_str()), std::ops::Bound::Unbounded)).next().map(|(id, _)| id));
        let Some(window_id) = next.or_else(|| self.partitions.keys().next()).cloned() else { return Ok(PluginCloseStep::Complete) };
        self.maintenance_cursor = Some(window_id.clone());
        let partition = self.partitions.get_mut(&window_id).expect("selected window transient partition remains owned");
        match partition.store.maintenance_returned_reads_step(&self.owners.state_retirement, maximum_items.min(1), maximum_bytes).map_err(|error| Fault::from(error.into_message()))? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
            store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "window transient returned read remains held" }),
            store::SnapshotRetirementStep::Complete => Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }),
        }
    }

    /// 🧹️ The partition's disposer gets the caller's WHOLE grant — the caller already decided how
    /// much a step may cost (one item while the app is live, a page while it is closing), and
    /// re-clamping it here meant a closing app paged a document's retained preview meshes one item
    /// at a time and then answered `Pending { 0, 0 }` once the retirement needed more than one
    /// (ticket 26/09/09: `plugin.internal.zero-progress` on an eight-document session).
    ///
    /// 🕰️ A retirement that released nothing is WAITING on something outside this ladder — the
    /// preview's own returned read lease, which comes back on a later reactor turn — not
    /// livelocked. Reported as `Pending { 0, 0 }` it is indistinguishable from a stuck ladder, and
    /// eight of them in a row kill the close with `plugin.internal.zero-progress`: measured
    /// intermittently on the close-cost fixture's eight-document session, always on the last
    /// remaining `procedural-preview` partition (ticket 26/09/09). `AwaitingInput` is the shape the
    /// runtime already treats as an external wait — it yields, names the authority through
    /// `runtime_close_pending_authority`, and spends no structural livelock credit.
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let next = self.retirement_cursor.as_ref().and_then(|cursor| self.partitions.range::<str, _>((std::ops::Bound::Excluded(cursor.as_str()), std::ops::Bound::Unbounded)).next().map(|(id, _)| id));
        let Some(window_id) = next.or_else(|| self.partitions.keys().next()).cloned() else { return Ok(PluginCloseStep::Complete) };
        self.retirement_cursor = Some(window_id.clone());
        let partition = self.partitions.get_mut(&window_id).expect("selected window transient partition remains owned");
        let disposer = partition.disposer.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.disposer"), "window transient partition lost its exact disposer"))?;
        let step = disposer.close_step(&mut partition.store, maximum_items, maximum_bytes)?;
        if matches!(step, PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }) {
            return Ok(PluginCloseStep::AwaitingInput { reason: "window transient retirement awaits its returned read" });
        }
        if step != PluginCloseStep::Complete {
            return Ok(step);
        }
        if !disposer.terminal_is_empty(&partition.store) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.terminal"), "window transient disposer reported complete without terminal emptiness"));
        }
        partition.disposer = None;
        self.partitions.remove(&window_id);
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.partitions.is_empty()
    }
}

/// 🗂️ Runtime registry of heterogeneous window-owned transient schemas.
#[derive(Default)]
pub struct WindowTransientOwnerRegistry {
    document_generation: u64,
    owners: BTreeMap<&'static str, Box<dyn ErasedWindowTransientStoreOwner>>,
    maintenance_cursor: Option<&'static str>,
    retirement_cursor: Option<&'static str>,
}

impl WindowTransientOwnerRegistry {
    pub(crate) fn for_document_generation(document_generation: u64) -> Self {
        Self { document_generation, owners: BTreeMap::new(), maintenance_cursor: None, retirement_cursor: None }
    }

    pub(crate) fn document_generation(&self) -> u64 {
        self.document_generation
    }

    pub fn register<O: WindowTransientOwner>(&mut self) -> Result<(), Fault> {
        if O::WINDOW_KIND_ID.is_empty() || self.owners.contains_key(O::WINDOW_KIND_ID) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner"), "window transient owner id is empty or already registered"));
        }
        self.owners.insert(O::WINDOW_KIND_ID, Box::new(TypedWindowTransientStoreOwner::<O> { partitions: BTreeMap::new(), owners: O::build_owners(), maintenance_cursor: None, retirement_cursor: None }));
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    pub(crate) fn refresh(&mut self, authority: &mut WindowTransientAuthority) -> Result<(), Fault> {
        if authority.snapshot.document_generation != self.document_generation {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.document-generation"), "window transient refresh belongs to a replaced document"));
        }
        self.owners.get_mut(authority.window_kind_id.as_str()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner"), "window transient refresh has no registered concrete window owner"))?.refresh(authority)
    }

    pub(crate) fn capture(&mut self, view_state: Option<&ViewModel>) -> Result<Option<WindowTransientAuthority>, Fault> {
        let Some(view_state) = view_state else { return Ok(None) };
        let Some(window_id) = view_state.window_id.as_deref() else { return Ok(None) };
        let window = view_state
            .window_instances
            .iter()
            .find(|window| window.id == window_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.window-context"), "target window is absent from the exact ViewModel window instance roster"))?;
        let Some(owner) = self.owners.get_mut(window.window_kind_id.as_str()) else { return Ok(None) };
        owner.capture(window_id, self.document_generation).map(Some)
    }

    /// 📬️ Begins publishing `mutation` under the operation's captured `authority`; a refusal hands the mutation back.
    pub(crate) fn begin(&mut self, operation: semio_framework_job::OperationId, authority: &WindowTransientAuthority, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, RejectedWindowTransientEmission> {
        let refuse = |mutation, code: &str, message: &str| Err(RejectedWindowTransientEmission { mutation, fault: Fault::new(FaultOrigin::Framework, FaultCode::new(code), message) });
        if authority.snapshot.document_generation != self.document_generation {
            return refuse(mutation, "window-transient.document-generation", "window transient emission belongs to a replaced document");
        }
        if mutation.window_id != authority.window_id || mutation.window_kind_id != authority.window_kind_id {
            return refuse(mutation, "window-transient.address", "window transient emission does not match the operation's exact captured window authority");
        }
        let document_generation = self.document_generation;
        match self.owners.get_mut(authority.window_kind_id.as_str()) {
            Some(owner) => owner.begin(operation, authority.generation, document_generation, mutation),
            None => refuse(mutation, "window-transient.owner", "window transient emission has no registered concrete window owner"),
        }
    }

    pub(crate) fn advance(&mut self, publication: &mut dyn ErasedWindowTransientPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault> {
        if publication.document_generation() != self.document_generation {
            publication.begin_close();
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.document-generation"), "window transient publication belongs to a replaced document"));
        }
        self.owners
            .get_mut(publication.window_kind_id())
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner"), "window transient publication lost its registered concrete window owner"))?
            .advance(publication, grant)
    }

    pub(crate) fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let next = self.maintenance_cursor.and_then(|cursor| self.owners.range::<str, _>((std::ops::Bound::Excluded(cursor), std::ops::Bound::Unbounded)).next().map(|(kind, _)| kind));
        let Some(kind) = next.or_else(|| self.owners.keys().next()).copied() else { return Ok(PluginCloseStep::Complete) };
        self.maintenance_cursor = Some(kind);
        self.owners.get_mut(kind).expect("selected window transient owner remains registered").maintenance_step(maximum_items.min(1), maximum_bytes)
    }

    pub(crate) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let next = self.retirement_cursor.and_then(|cursor| self.owners.range::<str, _>((std::ops::Bound::Excluded(cursor), std::ops::Bound::Unbounded)).next().map(|(kind, _)| kind));
        let Some(kind) = next.or_else(|| self.owners.keys().next()).copied() else { return Ok(PluginCloseStep::Complete) };
        self.retirement_cursor = Some(kind);
        let owner = self.owners.get_mut(kind).expect("selected window transient owner remains registered");
        let step = owner.close_step(maximum_items, maximum_bytes)?;
        if step == PluginCloseStep::Complete {
            if !owner.terminal_is_empty() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner-terminal"), "window transient owner reported complete without terminal emptiness"));
            }
            self.owners.remove(kind);
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(step)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.owners.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🪟️retained-window-input/🦀️.rs"]
mod retained_window_input_tests;

//#region 🔖️TransientRoot
/// 🫧️ Declares a whole-root transient state's one mutation and codecs (audit K3: the ~150 lines flow, fem, remodel, cad,
/// lowpoly, layout, forms, draw, raster, wfc each re-wrote): `$mutation::Snapshot { transient }` (wire
/// `{"kind":"snapshot","transient":…}`) replacing the whole root, its leaf descriptor (`$owner`, `$kind`, `$display`,
/// `$schema`), its sparse per-field diff `$diff` (`fields` lists the state's fields, see [`sparse_record_diff!`]: the slots where the
/// requested root differs from the base) and inverse (the root before), JSON op text and binary, and the state's DSL and pack in the semio envelope
/// `$envelope` (`$extension`). The state stays the plugin's own type: `Clone + Default + PartialEq + ToValue + FromValue`. An
/// artifact-level transient stops here; a window transient adds [`window_transient_owners!`].
///
/// 🎯️ The one exception to "no whole-record mutations": a transient root keeps its single `Snapshot { transient }` mutation. The
/// root is ephemeral and published through the ephemeral ownership transfer ([`window_transient_transfer!`]), which moves a whole
/// root (`footprint` sizes it, `into_state` moves it out), so per-field set variants cannot be transferred. Its diff is still the
/// sparse `$diff::changing(base, root)`, and its inverse restores the base root, whose diff sums to exactly the negative diff.
/// Persisted configs have no such variant (`config_record!` generates per-field set mutations only).
#[macro_export]
macro_rules! transient_root {
    (
        state: $state:ident,
        mutation: $mutation:ident,
        diff: $diff:ident,
        owner: $owner:literal,
        kind: $kind:literal,
        display_name: $display:literal,
        payload_schema: $schema:literal,
        envelope: $envelope:literal,
        extension: $extension:literal,
        $($fields:tt)+
    ) => {
        $crate::__kernel::sparse_record_diff! { record: $state, diff: $diff, $($fields)+ }

        #[doc = concat!("🫧️ The one mutation of [`", stringify!($state), "`]: replace the whole root.")]
        #[derive(Clone, Debug, PartialEq, $crate::ToValue, $crate::FromValue)]
        #[value(tag = "kind", rename_all = "kebab-case")]
        pub enum $mutation {
            Snapshot { transient: $state },
        }

        impl $crate::__kernel::Mutation<$state> for $mutation {
            type Diff = $diff;

            const DESCRIPTORS: &'static [$crate::__kernel::MutationLeafDescriptor] = &[$crate::__kernel::MutationLeafDescriptor {
                schema_version: 1,
                owner: $owner,
                semantic_kind: $kind,
                display_name: $display,
                emoji: "🫧️",
                aggregate_variant: "Snapshot",
                payload_schema: $schema,
                text_opcode: None,
                binary_tag: None,
                invertibility: $crate::__kernel::MutationInvertibility::ExplicitMutation,
                diff_participation: $crate::__kernel::MutationDiffParticipation::Detect,
                outcome_classes: &[$crate::__kernel::MutationOutcomeClass::Applied],
                composition: $crate::__kernel::MutationComposition::Atomic,
                required_language_surfaces: &[$crate::__kernel::MutationLanguageSurface::Rust, $crate::__kernel::MutationLanguageSurface::JsonSchema],
            }];

            fn descriptor(&self) -> &'static $crate::__kernel::MutationLeafDescriptor {
                &Self::DESCRIPTORS[0]
            }

            fn diff(&self, base: &$state) -> $crate::__kernel::MutationOutcome<$diff> {
                let Self::Snapshot { transient } = self;
                let changed = <$diff>::changing(base, transient);
                if changed == <$diff as Default>::default() {
                    return $crate::__kernel::MutationOutcome::empty();
                }
                $crate::__kernel::MutationOutcome::new(changed)
            }

            fn inverse(&self, base: &$state) -> Result<Vec<Self>, $crate::__value::ValueError> {
                Ok(vec![Self::Snapshot { transient: base.clone() }])
            }
        }

        impl $crate::__kernel::OpText for $mutation {
            fn parse_op(line: &str) -> Result<Self, $crate::__diagnostic::TextError> {
                $crate::__pack_json::from_json_str(line, $crate::__pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::__diagnostic::TextError::from_value_error(error, $crate::__diagnostic::TextSpan::at(1, 1)))
            }

            fn print_op(&self) -> String {
                $crate::__pack_json::to_json_string(self)
            }
        }

        impl $crate::__kernel::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, $crate::__kernel::ProtocolError> {
                Ok($crate::__pack_json::to_json_string(self).into_bytes())
            }

            fn decode_op(bytes: &[u8]) -> Result<Self, $crate::__kernel::ProtocolError> {
                let text = std::str::from_utf8(bytes).map_err(|error| $crate::__kernel::ProtocolError::from($crate::__kernel::PackError::from($crate::__value::ValueError::from(error))))?;
                $crate::__pack_json::from_json_str(text, $crate::__pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::__kernel::ProtocolError::from($crate::__kernel::PackError::from(error)))
            }
        }

        impl $crate::__kernel::ArtifactDsl for $state {
            const EXTENSION: &'static str = $extension;

            fn envelope_id() -> &'static str {
                $envelope
            }

            fn parse_dsl(text: &str) -> Result<Self, $crate::__diagnostic::TextError> {
                let body = $crate::__kernel::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
                if body.trim().is_empty() {
                    return Ok(Self::default());
                }
                $crate::__pack_json::from_json_str(body, $crate::__pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::__diagnostic::TextError::from_value_error(error, $crate::__diagnostic::TextSpan::at(1, 1)))
            }

            fn print_dsl(&self) -> String {
                let envelope = $crate::__kernel::semio_format::SemioEnvelope::from_envelope_id($envelope, $crate::__kernel::semio_format::Component::Dsl, 1).expect(concat!("valid ", $envelope, " envelope"));
                $crate::__kernel::semio_format::wrap_text(&envelope, &$crate::__pack_json::to_json_string(self))
            }
        }

        impl $crate::__kernel::ArtifactPack for $state {
            fn encode_pack_with(&self, _options: &$crate::__kernel::PackEncodeOptions) -> Result<Vec<u8>, $crate::__kernel::PackError> {
                let envelope = $crate::__kernel::semio_format::SemioEnvelope::from_envelope_id($envelope, $crate::__kernel::semio_format::Component::Pack, 1).map_err(|error| $crate::__kernel::PackError::from(error.into_value_error()))?;
                Ok($crate::__kernel::semio_format::wrap_binary(&envelope, $crate::__pack_json::to_json_string(self).as_bytes()))
            }

            fn decode_pack_with(bytes: &[u8], _options: &$crate::__kernel::PackDecodeOptions) -> Result<Self, $crate::__kernel::PackError> {
                if bytes.is_empty() {
                    return Ok(Self::default());
                }
                let (envelope, inner) = $crate::__kernel::semio_format::unwrap_binary(bytes).map_err(|error| $crate::__kernel::PackError::from(error.into_value_error()))?;
                if !envelope.matches_identity($envelope, $crate::__kernel::semio_format::Component::Pack, 1) {
                    return Err($crate::__kernel::PackError::from($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, concat!($envelope, " pack envelope mismatch"))));
                }
                let text = std::str::from_utf8(&inner).map_err(|error| $crate::__kernel::PackError::from($crate::__value::ValueError::from(error)))?;
                $crate::__pack_json::from_json_str(text, $crate::__pack_json::JsonMemberPolicy::Reject).map_err($crate::__kernel::PackError::from)
            }
        }
    };
}

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
            pub fn footprint(&self) -> Result<$crate::__kernel::ArtifactStoreOneItemFootprint, String> {
                let Self::Snapshot { transient } = self;
                let footprint = $crate::__kernel::ArtifactStoreOneItemFootprint::for_ephemeral_item($crate::__pack_json::to_json_string(transient).len());
                footprint.is_admissible().then_some(footprint).ok_or_else(|| concat!(stringify!($state), " exceeds its retained publication envelope").to_string())
            }

            #[doc = "📦️ The root this mutation installs, moved out without a copy."]
            pub fn into_state(self) -> $state {
                let Self::Snapshot { transient } = self;
                transient
            }
        }

        impl $crate::__value::retirement::RetireOwned for $mutation {
            fn retirement(self) -> Box<dyn $crate::__value::retirement::RetirementCursor> {
                let Self::Snapshot { transient } = self;
                $crate::__value::retirement::sequence(vec![$crate::__value::retirement::leaf(0u8), $crate::__value::retirement::RetireOwned::retirement(transient)])
            }
        }
    };
}

/// 🪟️ Declares the window-transient owners of one whole-root state ([`transient_root!`]) — its ephemeral transfer
/// ([`window_transient_transfer!`]), one [`WindowTransientOwner`] per window kind — and, in the invoking module, `register`
/// (every owner), `from_snapshot` / `current` (the window's root, else the default) and `addressed` (the mutation installing a
/// root in the view's own window, refused `window-transient.window-required` / `.window-stale` / `.kind-unknown`), audit K3.
#[macro_export]
macro_rules! window_transient_owners {
    (state: $state:ident, mutation: $mutation:ident, windows: { $($owner:ident => $window:expr),+ $(,)? } $(,)?) => {
        $crate::window_transient_transfer! { state: $state, mutation: $mutation }

        $(
            #[doc = concat!("🪟️ The [`", stringify!($state), "`] owner of window kind `", stringify!($window), "`.")]
            pub struct $owner;

            impl $crate::WindowTransientOwner for $owner {
                const WINDOW_KIND_ID: &'static str = $window;
                type State = $state;
                type Mutation = $mutation;

                fn build_owners() -> $crate::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                    let state = std::sync::Arc::new($crate::__value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                    let mutation = std::sync::Arc::new($crate::__value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
                    let preparation = std::sync::Arc::new($crate::__kernel::ArtifactEphemeralTransferPreparationFactory::new($mutation::footprint, $mutation::into_state, state.clone(), mutation.clone()));
                    $crate::WindowTransientOwnerBundle::new(preparation, state, mutation)
                }
            }
        )+

        #[doc = "🗂️ Registers every window-transient owner of this state."]
        pub fn register(registry: &mut $crate::WindowTransientOwnerRegistry) -> Result<(), $crate::Fault> {
            $(registry.register::<$owner>()?;)+
            Ok(())
        }

        #[doc = "📖️ The root a window-transient snapshot holds for this state, else the default root."]
        pub fn from_snapshot(snapshot: Option<&$crate::WindowTransientSnapshot>) -> $state {
            snapshot.and_then(|snapshot| None $(.or_else(|| snapshot.get::<$owner>()))+).cloned().unwrap_or_default()
        }

        #[doc = "📖️ The root of the window a transient view reads, else the default root."]
        pub fn current<T>(view: &$crate::TransientView<'_, T>) -> $state {
            from_snapshot(view.window)
        }

        #[doc = "📬️ The mutation installing `transient` as the root of the view's own window."]
        pub fn addressed(view: &$crate::ViewModel, transient: $state) -> Result<$crate::WindowTransientMutation, $crate::Fault> {
            let refuse = |code: &str, message: &str| $crate::Fault::new($crate::FaultOrigin::Framework, $crate::FaultCode::new(code), message);
            let id = view.window_id.as_deref().ok_or_else(|| refuse("window-transient.window-required", "a window transient needs the view's own window"))?;
            let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| refuse("window-transient.window-stale", "the view's window is absent from its window roster"))?;
            let mutation = $mutation::Snapshot { transient };
            $(
                if kind == <$owner as $crate::WindowTransientOwner>::WINDOW_KIND_ID {
                    return Ok($crate::WindowTransientMutation::of::<$owner>(id, mutation));
                }
            )+
            Err(refuse("window-transient.kind-unknown", "the view's window kind holds no transient of this state"))
        }
    };
}
//#endregion 🔖️TransientRoot

#[cfg(test)]
#[path = "🧪️tests/🧪️transient-root/🦀️.rs"]
mod transient_root_tests;
