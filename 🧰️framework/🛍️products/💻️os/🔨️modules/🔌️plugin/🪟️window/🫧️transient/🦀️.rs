//! 🫧️ Exact-window transient state with independent generation authority.

use super::app::{ArtifactOwnedDisposer, PluginCloseStep};
use super::transient_publication::{bounded_transient_preparation_factory, bounded_transient_root_retirement_factory, bounded_transient_store_disposer};
use crate::{protocol, store};
use semio_framework::{Fault, FaultCode, FaultOrigin, ViewModel};
use std::any::Any;
use std::collections::BTreeMap;
use std::sync::Arc;

/// 🪟️ Declares the transient schema and bounded owners of one concrete window kind.
pub trait WindowTransientOwner: Send + Sync + 'static {
    const WINDOW_KIND_ID: &'static str;
    type State: Clone + Default + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ArtifactDsl + store::ArtifactPack + 'static;
    type Mutation: protocol::Mutation<Self::State> + PartialEq + Send + protocol::OpText + protocol::OpBinary + 'static;

    fn build_one_item_preparation_factory() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::State, Self::Mutation>>;
    fn build_root_retirement_factory() -> Arc<dyn store::SnapshotRetirementFactory<Self::State>>;
    fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::State, Self::Mutation>>>;
}

pub fn bounded_window_transient_preparation_factory<O: WindowTransientOwner>() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<O::State, O::Mutation>> {
    bounded_transient_preparation_factory::<O::State, O::Mutation>()
}

pub fn bounded_window_transient_root_retirement_factory<O: WindowTransientOwner>() -> Arc<dyn store::SnapshotRetirementFactory<O::State>> {
    bounded_transient_root_retirement_factory::<O::State>()
}

pub fn bounded_window_transient_store_disposer<O: WindowTransientOwner>() -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<O::State, O::Mutation>>> {
    bounded_transient_store_disposer::<O::State, O::Mutation>()
}

/// 📬️ One typed mutation addressed to one exact concrete window instance.
pub struct WindowTransientMutation {
    window_id: String,
    window_kind_id: &'static str,
    mutation: Box<dyn Any + Send>,
}

impl std::fmt::Debug for WindowTransientMutation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("WindowTransientMutation").field("window_id", &self.window_id).field("window_kind_id", &self.window_kind_id).finish_non_exhaustive()
    }
}

impl WindowTransientMutation {
    pub fn of<O: WindowTransientOwner>(window_id: impl Into<String>, mutation: O::Mutation) -> Self {
        Self { window_id: window_id.into(), window_kind_id: O::WINDOW_KIND_ID, mutation: Box::new(mutation) }
    }

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
    snapshot: Arc<dyn Any + Send + Sync>,
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
        (self.window_kind_id == O::WINDOW_KIND_ID).then(|| self.snapshot.as_ref().downcast_ref::<O::State>()).flatten()
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
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

struct TypedWindowTransientPublication<O: WindowTransientOwner> {
    window_id: String,
    document_generation: u64,
    publication: store::ArtifactEphemeralOneItemPublication<O::State, O::Mutation>,
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

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        self.publication.close_step(grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty()
    }
}

type WindowTransientStore<O> = store::TransientStore<<O as WindowTransientOwner>::State, <O as WindowTransientOwner>::Mutation>;

struct WindowTransientPartition<O: WindowTransientOwner> {
    store: WindowTransientStore<O>,
    disposer: Option<Box<dyn ArtifactOwnedDisposer<WindowTransientStore<O>>>>,
}

impl<O: WindowTransientOwner> WindowTransientPartition<O> {
    fn new() -> Self {
        Self { store: store::TransientStore::new(O::State::default()), disposer: Some(O::build_store_disposer()) }
    }
}

trait ErasedWindowTransientStoreOwner: Send {
    fn capture(&mut self, window_id: &str, document_generation: u64) -> WindowTransientAuthority;
    fn begin(&mut self, operation: semio_framework_job::OperationId, expected_generation: u64, document_generation: u64, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, Fault>;
    fn advance(&mut self, publication: &mut dyn ErasedWindowTransientPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault>;
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>;
    fn terminal_is_empty(&self) -> bool;
}

struct TypedWindowTransientStoreOwner<O: WindowTransientOwner> {
    partitions: BTreeMap<String, WindowTransientPartition<O>>,
}

impl<O: WindowTransientOwner> TypedWindowTransientStoreOwner<O> {
    fn partition(&mut self, window_id: &str) -> &mut WindowTransientPartition<O> {
        self.partitions.entry(window_id.to_string()).or_insert_with(WindowTransientPartition::<O>::new)
    }
}

impl<O: WindowTransientOwner> ErasedWindowTransientStoreOwner for TypedWindowTransientStoreOwner<O> {
    fn capture(&mut self, window_id: &str, document_generation: u64) -> WindowTransientAuthority {
        let partition = self.partition(window_id);
        WindowTransientAuthority {
            window_id: window_id.to_string(),
            window_kind_id: O::WINDOW_KIND_ID.to_string(),
            generation: partition.store.generation_now(),
            snapshot: WindowTransientSnapshot { window_id: window_id.to_string(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation_now(), document_generation, snapshot: partition.store.current_root() },
        }
    }

    fn begin(&mut self, operation: semio_framework_job::OperationId, expected_generation: u64, document_generation: u64, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, Fault> {
        let window_id = mutation.window_id;
        let typed = mutation.mutation.downcast::<O::Mutation>().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.mutation-type"), "window transient mutation did not match its registered window owner"))?;
        let partition = self.partition(&window_id);
        let publication = partition
            .store
            .begin_publish_one(operation, expected_generation, *typed, Some(O::build_one_item_preparation_factory().as_ref()), Some(O::build_root_retirement_factory()))
            .map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.admission"), rejected.reason))?;
        Ok(Box::new(TypedWindowTransientPublication::<O> { window_id, document_generation, publication }))
    }

    fn advance(&mut self, publication: &mut dyn ErasedWindowTransientPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault> {
        let publication = publication
            .as_any_mut()
            .downcast_mut::<TypedWindowTransientPublication<O>>()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.publication-type"), "window transient publication did not match its registered window owner"))?;
        self.partition(&publication.window_id).store.advance_publish_one(&mut publication.publication, grant).map_err(|reason| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.publication"), reason))
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        let Some(window_id) = self.partitions.keys().next().cloned() else { return Ok(PluginCloseStep::Complete) };
        let partition = self.partitions.get_mut(&window_id).expect("selected window transient partition remains owned");
        let disposer = partition.disposer.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.disposer"), "window transient partition lost its exact disposer"))?;
        let step = disposer.close_step(&mut partition.store, maximum_items.min(1), maximum_bytes)?;
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
}

impl WindowTransientOwnerRegistry {
    pub(crate) fn for_document_generation(document_generation: u64) -> Self {
        Self { document_generation, owners: BTreeMap::new() }
    }

    pub(crate) fn document_generation(&self) -> u64 {
        self.document_generation
    }

    pub fn register<O: WindowTransientOwner>(&mut self) -> Result<(), Fault> {
        if O::WINDOW_KIND_ID.is_empty() || self.owners.contains_key(O::WINDOW_KIND_ID) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner"), "window transient owner id is empty or already registered"));
        }
        self.owners.insert(O::WINDOW_KIND_ID, Box::new(TypedWindowTransientStoreOwner::<O> { partitions: BTreeMap::new() }));
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
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
        Ok(Some(owner.capture(window_id, self.document_generation)))
    }

    pub(crate) fn begin(&mut self, operation: semio_framework_job::OperationId, authority: &WindowTransientAuthority, mutation: WindowTransientMutation) -> Result<Box<dyn ErasedWindowTransientPublication>, Fault> {
        if authority.snapshot.document_generation != self.document_generation {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.document-generation"), "window transient emission belongs to a replaced document"));
        }
        if mutation.window_id != authority.window_id || mutation.window_kind_id != authority.window_kind_id {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.address"), "window transient emission does not match the operation's exact captured window authority"));
        }
        self.owners.get_mut(authority.window_kind_id.as_str()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-transient.owner"), "window transient emission has no registered concrete window owner"))?.begin(
            operation,
            authority.generation,
            self.document_generation,
            mutation,
        )
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

    pub(crate) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(kind) = self.owners.keys().next().copied() else { return Ok(PluginCloseStep::Complete) };
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
