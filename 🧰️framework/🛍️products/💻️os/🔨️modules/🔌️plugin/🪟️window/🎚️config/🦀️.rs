//! 🎚️ Exact-window persisted-local configuration with independent event-stream authority.

use super::app::{ArtifactOwnedDisposer, PluginCloseStep};
use crate::{protocol, store};
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::any::Any;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// 🪟️ Declares one concrete window kind's persisted-local configuration owner.
pub trait WindowConfigOwner: Send + Sync + 'static {
    const WINDOW_KIND_ID: &'static str;
    const SCHEMA: &'static str;
    const MAXIMUM_PUBLICATION_BYTES: usize;
    type State: Clone + Default + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ConfigRecord + store::ArtifactPack + 'static;
    type Mutation: protocol::Mutation<Self::State> + PartialEq + Send + protocol::OpText + protocol::OpBinary + 'static;

    fn build_store_owners() -> store::MemberStoreOwners<Self::State, Self::Mutation>;
    fn build_one_item_preparation_factory() -> Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>>;
    fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>>;
}

/// 📏️ `retained_bytes` is THIS item's own encoded cost (forward op plus description), not the
/// owner's `MAXIMUM_PUBLICATION_BYTES` ceiling: the per-turn grant a publisher hands down is a work
/// budget for one turn (`TYPED_OPERATION_RESULT_PAGE_BYTES`), while the ceiling is the widest record
/// the schema may ever carry. Gating a turn on the ceiling made every owner whose ceiling exceeds
/// that grant permanently unpublishable — the publication spun in `Blocked` forever and the whole
/// typed operation behind it never quiesced. Same shape as the transient lane's
/// `BoundedTransientPreparation` (`🫧️transient/🧵️publication/🦀️.rs`).
struct BoundedWindowConfigPreparation<O: WindowConfigOwner> {
    base: Option<store::SnapshotRead<O::State>>,
    mutation: Option<O::Mutation>,
    description: Option<String>,
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<O::State, O::Mutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

struct BoundedWindowConfigPreparationFactory<O: WindowConfigOwner>(std::marker::PhantomData<fn() -> O>);

impl<O: WindowConfigOwner> Default for BoundedWindowConfigPreparationFactory<O> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<O: WindowConfigOwner> BoundedWindowConfigPreparationFactory<O> {
    /// 📏️ ONE item's exact encoded cost — the same quantity `preflight` bounds and the preparation
    /// gates its own turn on.
    fn item_retained_bytes(mutation: &O::Mutation, description: Option<&str>) -> Result<usize, String> {
        let retained_bytes = protocol::OpBinary::encode_op(mutation).map_err(|error| error.to_string())?.len().saturating_add(description.map_or(0, str::len));
        if retained_bytes > O::MAXIMUM_PUBLICATION_BYTES {
            return Err("window config mutation exceeds its owner-declared publication bound".into());
        }
        Ok(retained_bytes)
    }
}

impl<O: WindowConfigOwner> store::ArtifactStoreOneItemPreparationFactory<O::State, O::Mutation> for BoundedWindowConfigPreparationFactory<O> {
    fn preflight(&self, mutation: &O::Mutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || O::MAXIMUM_PUBLICATION_BYTES == 0 || O::MAXIMUM_PUBLICATION_BYTES > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err("window config publication has an invalid lane or byte bound".into());
        }
        let retained_bytes = Self::item_retained_bytes(mutation, description)?;
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<O::State, O::Mutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<O::State, O::Mutation>>, store::ArtifactStoreOneItemPreparationRequest<O::State, O::Mutation>> {
        if request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() {
            return Err(request);
        }
        if self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err() {
            return Err(request);
        }
        let Ok(retained_bytes) = Self::item_retained_bytes(&request.mutation, request.description.as_deref()) else { return Err(request) };
        Ok(Box::new(BoundedWindowConfigPreparation::<O> {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl<O: WindowConfigOwner> store::ArtifactStoreOneItemPreparation<O::State, O::Mutation> for BoundedWindowConfigPreparation<O> {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if self.cancelled || self.closing || !grant.permits_one() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if grant.maximum_bytes < self.retained_bytes {
            return Err("window config item cannot ever fit the publication turn's byte grant".into());
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "window config preparation lost its base".to_string())?;
        let mutation = self.mutation.as_ref().ok_or_else(|| "window config preparation lost its mutation".to_string())?;
        let outcome = protocol::Mutation::diff(mutation, base.get());
        if outcome.worst_level().is_some_and(|level| level >= protocol::Severity::Error) {
            return Err("window config mutation was rejected against its captured base".into());
        }
        let next = protocol::MutationDiff::apply(outcome.diff(), base.get()).map_err(|error| error.to_string())?;
        let inverse = protocol::Mutation::inverse(mutation, base.get());
        let encoded_bytes = store::ArtifactPack::encode_pack(&next)
            .len()
            .saturating_add(protocol::OpBinary::encode_op(mutation).map_err(|error| error.to_string())?.len())
            .saturating_add(inverse.iter().try_fold(0usize, |total, item| protocol::OpBinary::encode_op(item).map(|bytes| total.saturating_add(bytes.len())).map_err(|error| error.to_string()))?)
            .saturating_add(self.description.as_ref().map_or(0, String::len));
        if encoded_bytes > O::MAXIMUM_PUBLICATION_BYTES {
            return Err("window config prepared state or inverse exceeds its owner-declared publication bound".into());
        }
        let authority = self.authority.as_ref().ok_or_else(|| "window config preparation lost its live authority".to_string())?;
        let id = format!("window-config-{}-{}", authority.operation().0, authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![mutation.clone()],
            inverse,
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                dependencies: protocol::Mutation::dependencies(mutation),
                base_version: protocol::Mutation::base_version(mutation).map_or(authority.base_applied_edit_count() as u64, |version| version.0),
                author_id: protocol::Mutation::author_id(mutation).or_else(|| Some(protocol::ActorId(authority.actor().to_string()))),
                timestamp: protocol::Mutation::timestamp(mutation).unwrap_or_else(|| authority.next_clock()),
                undo_policy: protocol::Mutation::undo_policy(mutation),
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: Default::default(),
            }],
            description: self.description.clone(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, Arc::new(next))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: encoded_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<O::State, O::Mutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<O::State, O::Mutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes.min(grant.maximum_bytes) });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("window config base retirement was rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

pub fn bounded_window_config_store_owners<O: WindowConfigOwner>() -> store::MemberStoreOwners<O::State, O::Mutation> {
    super::app::bounded_config_store_owners::<O::State, O::Mutation>()
}

pub fn bounded_window_config_preparation_factory<O: WindowConfigOwner>() -> Arc<dyn store::ArtifactStoreOneItemPreparationFactory<O::State, O::Mutation>> {
    Arc::new(BoundedWindowConfigPreparationFactory::<O>::default())
}

pub fn bounded_window_config_store_disposer<O: WindowConfigOwner>() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<O::State, O::Mutation>>> {
    super::app::bounded_config_store_disposer::<O::State, O::Mutation>()
}

#[cfg(test)]
#[path = "🧪️tests/🪟️retained-window-config/🦀️.rs"]
mod tests;

/// 📬️ One typed config mutation addressed to one exact concrete window instance.
pub struct WindowConfigMutation {
    window_id: String,
    window_kind_id: &'static str,
    mutation: Box<dyn Any + Send>,
}

impl std::fmt::Debug for WindowConfigMutation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("WindowConfigMutation").field("window_id", &self.window_id).field("window_kind_id", &self.window_kind_id).finish_non_exhaustive()
    }
}

impl WindowConfigMutation {
    pub fn of<O: WindowConfigOwner>(window_id: impl Into<String>, mutation: O::Mutation) -> Self {
        Self { window_id: window_id.into(), window_kind_id: O::WINDOW_KIND_ID, mutation: Box::new(mutation) }
    }

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
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

struct TypedWindowConfigPublication<O: WindowConfigOwner> {
    window_id: String,
    publication: store::ArtifactStoreBatchPublication<O::State, O::Mutation>,
}

impl<O: WindowConfigOwner> ErasedWindowConfigPublication for TypedWindowConfigPublication<O> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn window_kind_id(&self) -> &str {
        O::WINDOW_KIND_ID
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

type WindowConfigStore<O> = store::ConfigStore<<O as WindowConfigOwner>::State, <O as WindowConfigOwner>::Mutation>;

struct WindowConfigPartition<O: WindowConfigOwner> {
    store: WindowConfigStore<O>,
    disposer: Option<Box<dyn ArtifactOwnedDisposer<WindowConfigStore<O>>>>,
}

trait ErasedWindowConfigStoreOwner: Send {
    fn capture<'a>(&'a mut self, window_id: &'a str) -> Pin<Box<dyn Future<Output = Result<WindowConfigAuthority, Fault>> + 'a>>;
    fn dispatch<'a>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation, description: Option<String>, coalesce_key: Option<String>) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>>;
    fn begin(&mut self, operation: semio_framework_job::OperationId, actor: String, authority: &WindowConfigAuthority, mutation: WindowConfigMutation) -> Result<Box<dyn ErasedWindowConfigPublication>, Fault>;
    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault>;
    fn refresh(&mut self, authority: &mut WindowConfigAuthority) -> Result<(), Fault>;
    fn packs<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WindowConfigPack>, Fault>> + 'a>>;
    fn load<'a>(&'a mut self, window_id: &'a str, files: store::ArtifactPackFiles) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>>;
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>;
    fn terminal_is_empty(&self) -> bool;
}

struct TypedWindowConfigStoreOwner<O: WindowConfigOwner> {
    partitions: BTreeMap<String, WindowConfigPartition<O>>,
}

impl<O: WindowConfigOwner> TypedWindowConfigStoreOwner<O> {
    async fn partition(&mut self, window_id: &str) -> Result<&mut WindowConfigPartition<O>, Fault> {
        if !self.partitions.contains_key(window_id) {
            let id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
            let envelope = store::create_config_envelope::<O::State, O::Mutation>(O::SCHEMA, &id, O::State::default(), None).await;
            let mut config = store::ConfigStore::new(envelope).await.map_err(|error| error.into_fault())?;
            config.install_member_store_owners_exact(O::build_store_owners());
            self.partitions.insert(window_id.to_string(), WindowConfigPartition { store: config, disposer: Some(O::build_store_disposer()) });
        }
        Ok(self.partitions.get_mut(window_id).expect("initialized window config partition remains owned"))
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

    fn dispatch<'a>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation, description: Option<String>, coalesce_key: Option<String>) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>> {
        Box::pin(async move {
            let window_id = mutation.window_id;
            let typed = mutation.mutation.downcast::<O::Mutation>().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.mutation-type"), "window config mutation did not match its registered window owner"))?;
            let partition = self.partition(&window_id).await?;
            partition.store.set_local_actor_id(Some(actor.to_string())).map_err(|error| error.into_fault())?;
            let command = match coalesce_key {
                Some(key) => store::ArtifactCommand::AmendLast { mutations: vec![*typed], coalesce_key: Some(format!("window:{window_id}:{key}")) },
                None => store::ArtifactCommand::Apply { mutations: vec![*typed], description },
            };
            partition.store.dispatch(command).await.map_err(|error| error.into_fault())?;
            Ok(())
        })
    }

    fn begin(&mut self, operation: semio_framework_job::OperationId, actor: String, authority: &WindowConfigAuthority, mutation: WindowConfigMutation) -> Result<Box<dyn ErasedWindowConfigPublication>, Fault> {
        let window_id = mutation.window_id;
        let typed = mutation.mutation.downcast::<O::Mutation>().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.mutation-type"), "window config mutation did not match its registered window owner"))?;
        let partition = self.partitions.get(&window_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.partition"), "captured window config partition is absent"))?;
        let factory = O::build_one_item_preparation_factory();
        let publication = partition
            .store
            .begin_apply_batch(operation, authority.generation, authority.revision, actor, vec![*typed], None, store::HistoryLane::Document, Some(&factory))
            .map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.admission"), rejected.reason))?;
        Ok(Box::new(TypedWindowConfigPublication::<O> { window_id, publication }))
    }

    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, Fault> {
        let publication = publication
            .as_any_mut()
            .downcast_mut::<TypedWindowConfigPublication<O>>()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.publication-type"), "window config publication did not match its registered window owner"))?;
        self.partitions
            .get_mut(&publication.window_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.partition"), "window config publication lost its exact partition"))?
            .store
            .advance_apply_batch(&mut publication.publication, grant)
            .map_err(|error| error.into_fault())
    }

    fn refresh(&mut self, authority: &mut WindowConfigAuthority) -> Result<(), Fault> {
        let partition = self.partitions.get(&authority.window_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.partition"), "window config authority lost its exact partition"))?;
        authority.generation = partition.store.generation();
        authority.revision = partition.store.content_revision_now();
        authority.snapshot = WindowConfigSnapshot { window_id: authority.window_id.clone(), window_kind_id: O::WINDOW_KIND_ID, generation: authority.generation, revision: authority.revision, snapshot: partition.store.snapshot_root() };
        Ok(())
    }

    fn packs<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WindowConfigPack>, Fault>> + 'a>> {
        Box::pin(async move {
            let mut packs = Vec::with_capacity(self.partitions.len());
            for (window_id, partition) in &self.partitions {
                let files = store::print_document_pack(partition.store.envelope()).await.map_err(|error| error.into_fault())?;
                packs.push(WindowConfigPack { window_id: window_id.clone(), window_kind_id: O::WINDOW_KIND_ID.to_string(), files });
            }
            Ok(packs)
        })
    }

    fn load<'a>(&'a mut self, window_id: &'a str, files: store::ArtifactPackFiles) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>> {
        Box::pin(async move {
            let parsed: store::ParsedDocumentText<O::State, O::Mutation> = store::parse_document_pack(&files.pack, &files.spr).await.map_err(|error| error.into_fault())?;
            if parsed.envelope.schema != O::SCHEMA {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.schema"), "window config pack schema does not match its registered concrete window owner"));
            }
            let (applied, redo) = match &parsed.envelope.cursor {
                Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
                None => (parsed.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), Vec::new()),
            };
            self.partition(window_id).await?.store.reset(parsed.into_envelope(), applied, redo).await.map(|_| ()).map_err(|error| error.into_fault())
        })
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        let Some(window_id) = self.partitions.keys().next().cloned() else { return Ok(PluginCloseStep::Complete) };
        let partition = self.partitions.get_mut(&window_id).expect("selected window config partition remains owned");
        let disposer = partition.disposer.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.disposer"), "window config partition lost its exact disposer"))?;
        let step = disposer.close_step(&mut partition.store, maximum_items.min(1), maximum_bytes)?;
        if step != PluginCloseStep::Complete {
            return Ok(step);
        }
        if !disposer.terminal_is_empty(&partition.store) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.terminal"), "window config disposer reported complete without terminal emptiness"));
        }
        partition.disposer = None;
        self.partitions.remove(&window_id);
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.partitions.is_empty()
    }
}

/// 🗂️ Runtime registry of heterogeneous persisted-local window config schemas.
#[derive(Default)]
pub struct WindowConfigOwnerRegistry {
    owners: BTreeMap<&'static str, Box<dyn ErasedWindowConfigStoreOwner>>,
}

impl WindowConfigOwnerRegistry {
    pub fn register<O: WindowConfigOwner>(&mut self) -> Result<(), Fault> {
        if O::WINDOW_KIND_ID.is_empty() || O::SCHEMA.is_empty() || self.owners.contains_key(O::WINDOW_KIND_ID) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config owner identity is empty or already registered"));
        }
        self.owners.insert(O::WINDOW_KIND_ID, Box::new(TypedWindowConfigStoreOwner::<O> { partitions: BTreeMap::new() }));
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
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

    pub(crate) async fn dispatch(&mut self, authority: &WindowConfigAuthority, actor: &str, mutation: WindowConfigMutation, description: Option<String>, coalesce_key: Option<String>) -> Result<(), Fault> {
        self.validate_address(authority, &mutation)?;
        self.owners
            .get_mut(authority.window_kind_id.as_str())
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config emission has no registered concrete window owner"))?
            .dispatch(actor, mutation, description, coalesce_key)
            .await
    }

    pub(crate) fn begin(&mut self, operation: semio_framework_job::OperationId, actor: String, authority: &WindowConfigAuthority, mutation: WindowConfigMutation) -> Result<Box<dyn ErasedWindowConfigPublication>, Fault> {
        self.validate_address(authority, &mutation)?;
        self.owners
            .get_mut(authority.window_kind_id.as_str())
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config emission has no registered concrete window owner"))?
            .begin(operation, actor, authority, mutation)
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
        self.owners.get_mut(pack.window_kind_id.as_str()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config pack has no registered concrete window owner"))?.load(&pack.window_id, pack.files).await
    }

    pub(crate) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        let Some(kind) = self.owners.keys().next().copied() else { return Ok(PluginCloseStep::Complete) };
        let owner = self.owners.get_mut(kind).expect("selected window config owner remains registered");
        let step = owner.close_step(maximum_items, maximum_bytes)?;
        if step == PluginCloseStep::Complete {
            if !owner.terminal_is_empty() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner-terminal"), "window config owner reported complete without terminal emptiness"));
            }
            self.owners.remove(kind);
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(step)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    fn validate_address(&self, authority: &WindowConfigAuthority, mutation: &WindowConfigMutation) -> Result<(), Fault> {
        if mutation.window_id != authority.window_id || mutation.window_kind_id != authority.window_kind_id {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.address"), "window config emission does not match the operation's exact captured window authority"));
        }
        Ok(())
    }
}
