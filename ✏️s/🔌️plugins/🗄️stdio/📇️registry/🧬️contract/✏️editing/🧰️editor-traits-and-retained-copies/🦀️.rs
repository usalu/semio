//! 🧰️ The snapshot editing editor traits, retained copies, close turns and native preparation routes.

use super::*;

pub trait SnapshotEditingEditor: ArtifactEditor {
    /// 📦️ Declares exact borrowed operation-wire authority for retained prestage host receipts.
    fn snapshot_operation_wire_source(_mutation: &Self::Mutation) -> Option<semio_framework_plugin::plugin_app_close_prelude::store::ArtifactPreparedOperationSource<'_>> { None }

    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent>;
    fn snapshot_edit_is_admitted(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> bool {
        snapshot_edit_value_is_admitted(event, snapshot)
    }
    /// 🧭️ The pointer → kind table of this artifact's details pane: which JSON-pointer edit raises which ONE concrete kind.
    fn snapshot_edit_rules() -> &'static EditRules;

    /// 🎯️ Edits the table cannot express because the kind's payload is computed from the edit rather than carried by it (a byte
    /// splice, a text range): answered here with the concrete kind of the gesture; `None` falls through to [`Self::snapshot_edit_rules`].
    fn snapshot_edit_special(_event: &SnapshotEditEvent, _snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        Ok(None)
    }

    /// ⚖️ What one snapshot edit publishes: replacing the whole source loads the document (no history row, no differencing), every other
    /// edit resolves to the concrete kind its pointer names or is refused naming the unsupported path.
    fn snapshot_edit_emit(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let SnapshotEditEvent::ReplaceSource { source } = event {
            let next = snapshot_from_edit_source::<Self::Snapshot>(source).map_err(snapshot_edit_fault)?;
            return Ok(Emit { effects: vec![load_example_effect(&next, Self::DOCUMENT_SCHEMA)], ..Default::default() });
        }
        let mutations = match Self::snapshot_edit_special(event, snapshot)? {
            Some(mutations) => mutations,
            None => Self::snapshot_edit_rules().resolve::<Self::Snapshot, Self::Mutation>(snapshot, event).map_err(snapshot_edit_fault)?,
        };
        check_publication_limits(snapshot, &mutations)?;
        Ok(Emit { artifact_mutations: mutations, ..Default::default() })
    }
}

/// 🧵️ Supplies native mutations to the same retained, cancelable execution lane as snapshot edits.
pub trait BoundedNativeEditingEditor: SnapshotEditingEditor {
    const NATIVE_TOOL_IDS: &'static [&'static str];
    const NATIVE_PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract];
    const NATIVE_PAYLOAD_SCHEMA: &'static str;
    const NATIVE_MAXIMUM_RAW_BYTES: usize = SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES;
    const NATIVE_MAXIMUM_WORK_ITEMS: usize = 2;
    const NATIVE_CHECKPOINT_RESUME: bool = false;

    fn native_edit_execution_contract() -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(Self::NATIVE_MAXIMUM_RAW_BYTES, 4_096, 1, Self::NATIVE_MAXIMUM_RAW_BYTES, 7_500)
    }

    fn native_edit_extent(command: &Self::Command, _snapshot: &Self::Snapshot, _interaction: &kernel::InteractionState) -> Option<usize> {
        Self::NATIVE_TOOL_IDS.contains(&Self::command_id(command)).then(|| command.encode_op().ok()).flatten().filter(|encoded| encoded.len() <= Self::NATIVE_MAXIMUM_RAW_BYTES).map(|_| Self::NATIVE_MAXIMUM_WORK_ITEMS)
    }

    fn native_edit_mutations(command: &Self::Command, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault>;

    /// 🧵️ Supplies the retained command cursor for one native edit. Editors whose reduction can
    /// traverse or copy artifact-sized values override this with an `ArtifactCommandWork` that
    /// advances one bounded semantic unit per step and owns its checkpoint/close lifecycle.
    fn native_edit_work(tool_id: &'static str) -> Box<dyn ArtifactCommandWork<EditorApp<Self>>>
    where
        Self: Sized,
    {
        Box::new(BoundedArtifactCommandWork::new(tool_id, bounded_native_edit_reduce::<Self>, Self::native_edit_extent))
    }

    /// 📬️ Supplies an exact mutation predicate and retained Store cursor for native edits whose
    /// inverse or post snapshot requires artifact-sized work. The surrounding router keeps shared
    /// snapshot mutations on their own preparation factory and never falls back after a recognized
    /// native mutation is refused.
    fn native_edit_preparation_route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<Self::Snapshot, Self::Mutation>>
    where
        Self: Sized,
    {
        None
    }
}

pub type ArtifactPreparationFactory<S, M> = std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<S, M>>;

pub(super) fn retained_copy_close_demands(bytes:&Vec<u8>,reserved:bool,complete:bool)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{release_bytes:bytes.capacity(),depth:usize::from(bytes.capacity()!=0||reserved||complete),..Default::default()}}
pub(super) fn retained_copy_close_step(bytes:&mut Vec<u8>,reserved:&mut bool,complete:&mut bool,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{
 use semio_framework_job::InteractiveJobCloseStep;use semio_framework_value::retained_clone::RetainedCloneProgress;let demand=retained_copy_close_demands(bytes,*reserved,*complete);
 if demand.depth==0{return InteractiveJobCloseStep::Complete{progress:Default::default()};}if grant.maximum_items==0||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return InteractiveJobCloseStep::Pending{progress:Default::default()};}
 if bytes.capacity()!=0{let original=std::mem::take(bytes);let released_bytes=original.capacity();drop(original);return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}};}
 *reserved=false;*complete=false;InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}}
}

/// 🧵️ Copies one immutable UTF-8 owner through fixed byte grants without cloning the whole value
/// in a scheduler turn.
#[derive(Default)]
pub struct RetainedTextCopy {
    pub(super) bytes: Vec<u8>,
    pub(super) reserved: bool,
    complete: bool,
}

impl RetainedTextCopy {
    pub fn advance(&mut self, source: &str, maximum_bytes: usize) -> Result<Option<usize>, String> {
        if maximum_bytes == 0 {
            return Ok(None);
        }
        if !self.reserved {
            self.bytes.try_reserve_exact(source.len()).map_err(|_| "retained text allocation admission failed")?;
            self.reserved = true;
            self.complete = source.is_empty();
            return Ok(Some(0));
        }
        if self.complete {
            return Ok(Some(0));
        }
        let start = self.bytes.len();
        let count = maximum_bytes.min(source.len().checked_sub(start).ok_or("retained text source changed during copy")?);
        self.bytes.extend_from_slice(&source.as_bytes()[start..start + count]);
        self.complete = self.bytes.len() == source.len();
        Ok(Some(count))
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn take(&mut self) -> Option<String> {
        if !self.complete {
            return None;
        }
        self.complete = false;
        self.reserved = false;
        String::from_utf8(std::mem::take(&mut self.bytes)).ok()
    }

    pub fn take_partial_bytes(&mut self) -> Vec<u8> {
        self.complete = false;
        self.reserved = false;
        std::mem::take(&mut self.bytes)
    }

    pub fn close_demands(&self)->semio_framework_value::RetirementDemand{retained_copy_close_demands(&self.bytes,self.reserved,self.complete)}
    pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{retained_copy_close_step(&mut self.bytes,&mut self.reserved,&mut self.complete,grant)}
    pub fn terminal_is_empty(&self) -> bool {
        self.bytes.is_empty() && self.bytes.capacity() == 0 && !self.reserved && !self.complete
    }
}

/// 🧵️ Copies one immutable byte owner through fixed grants and supports the same bounded
/// cancellation cleanup as [`RetainedTextCopy`].
#[derive(Default)]
pub struct RetainedBytesCopy {
    pub(super) bytes: Vec<u8>,
    pub(super) reserved: bool,
    complete: bool,
}

impl RetainedBytesCopy {
    pub fn advance(&mut self, source: &[u8], maximum_bytes: usize) -> Result<Option<usize>, String> {
        if maximum_bytes == 0 {
            return Ok(None);
        }
        if !self.reserved {
            self.bytes.try_reserve_exact(source.len()).map_err(|_| "retained byte allocation admission failed")?;
            self.reserved = true;
            self.complete = source.is_empty();
            return Ok(Some(0));
        }
        if self.complete {
            return Ok(Some(0));
        }
        let start = self.bytes.len();
        let count = maximum_bytes.min(source.len().checked_sub(start).ok_or("retained byte source changed during copy")?);
        self.bytes.extend_from_slice(&source[start..start + count]);
        self.complete = self.bytes.len() == source.len();
        Ok(Some(count))
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn take(&mut self) -> Option<Vec<u8>> {
        if !self.complete {
            return None;
        }
        self.complete = false;
        self.reserved = false;
        Some(std::mem::take(&mut self.bytes))
    }

    pub fn take_partial(&mut self) -> Vec<u8> {
        self.complete = false;
        self.reserved = false;
        std::mem::take(&mut self.bytes)
    }

    pub fn close_demands(&self)->semio_framework_value::RetirementDemand{retained_copy_close_demands(&self.bytes,self.reserved,self.complete)}
    pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{retained_copy_close_step(&mut self.bytes,&mut self.reserved,&mut self.complete,grant)}
    pub fn terminal_is_empty(&self) -> bool {
        self.bytes.is_empty() && self.bytes.capacity() == 0 && !self.reserved && !self.complete
    }
}

/// 🧪️ The mounted-owner authority registered fixtures close under: one item per turn, bounded copy, capacity and release.
pub const fn fixture_mounted_owner_policy() -> semio_framework_plugin::MountedOwnerPolicyV1 {
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 64 };
    semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant }
}

/// 🎟️ Funds one generously bounded single-item close turn for drains that already hold their quoted demand.
pub const fn ample_close_grant() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: 64 }
}

/// 📏️ Quotes the exact backing release of one optional owned revision string.
pub fn revision_close_demand(revision: &Option<String>) -> semio_framework_value::RetirementDemand {
    revision.as_ref().map_or_else(Default::default, |value| semio_framework_value::RetirementDemand { release_bytes: value.capacity(), depth: 1, ..Default::default() })
}

/// 🧹️ Releases one optional owned revision string in a single exactly granted close turn.
pub fn close_revision_turn(revision: &mut Option<String>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
    use semio_framework_job::InteractiveJobCloseStep;
    let demand = revision_close_demand(revision);
    if demand == semio_framework_value::RetirementDemand::default() {
        return InteractiveJobCloseStep::Complete { progress: Default::default() };
    }
    if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes {
        return InteractiveJobCloseStep::Pending { progress: Default::default() };
    }
    drop(revision.take());
    InteractiveJobCloseStep::Complete { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() } }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct NativeEditPreparationRoute<S, M> {
    recognizes: fn(&M) -> bool,
    #[factory_child]
    pub(super) factory: std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<S, M>>,
}

impl<S, M> NativeEditPreparationRoute<S, M> {
    pub fn new(recognizes: fn(&M) -> bool, factory: ArtifactPreparationFactory<S, M>) -> Self {
        Self { recognizes, factory }
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RoutedNativeEditPreparationFactory<S, M> {
    #[factory_owned]
    route: NativeEditPreparationRoute<S, M>,
    #[factory_child]
    pub(super) fallback: std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<S, M>>,
}

impl<S, M> semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<S, M> for RoutedNativeEditPreparationFactory<S, M>
where
    S: Send + Sync + 'static,
    M: Send + Sync + 'static,
{
    fn preflight(&self, mutation: &M, lane: semio_framework_plugin::plugin_app_close_prelude::store::os_store::store_schema::HistoryLane) -> Result<semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if (self.route.recognizes)(mutation) {
            self.route.factory.preflight(mutation, lane)
        } else {
            self.fallback.preflight(mutation, lane)
        }
    }

    fn begin_batch_digest(&self,edit:&mut Option<Box<semio_framework_plugin::plugin_app_close_prelude::store::Edit<M>>>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreBatchDigest<M>>,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>{self.fallback.begin_batch_digest(edit,grant)}

    fn begin_demand(&self,mutation:&M,lane:semio_framework_plugin::plugin_app_close_prelude::store::os_store::store_schema::HistoryLane)->Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand,semio_framework_value::ValueError>{if(self.route.recognizes)(mutation){self.route.factory.begin_demand(mutation,lane)}else{self.fallback.begin_demand(mutation,lane)}}
    fn begin(
        &self,
        request: semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<S, M, M>,
        grant:semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<S, M>>,semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError,semio_framework_plugin::plugin_app_close_prelude::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<S, M, M>)> {
        if (self.route.recognizes)(&request.mutation) {
            self.route.factory.begin(request,grant)
        } else {
            self.fallback.begin(request,grant)
        }
    }
}

/// 🌱️ Prices the concrete routed wrapper around its two actual original child factory trees.
pub fn routed_native_edit_preparation_factory_birth_bytes<S: 'static, M: 'static>(route_birth: Option<usize>, fallback_birth: usize) -> usize {
    route_birth.map_or(fallback_birth, |bytes| semio_framework_value::factory_constructor_birth_bytes::<RoutedNativeEditPreparationFactory<S, M>>(bytes.checked_add(fallback_birth).expect("routed child constructor layout")))
}

pub fn routed_native_edit_preparation_factory<S, M>(route: Option<NativeEditPreparationRoute<S, M>>, fallback: ArtifactPreparationFactory<S, M>) -> ArtifactPreparationFactory<S, M>
where
    S: Send + Sync + 'static,
    M: Send + Sync + 'static,
{
    match route {
        Some(route) => std::sync::Arc::new(RoutedNativeEditPreparationFactory { route, fallback }),
        None => fallback,
    }
}
