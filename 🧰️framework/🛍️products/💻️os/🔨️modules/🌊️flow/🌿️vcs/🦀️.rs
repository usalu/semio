//! 🌿️ Flow document VCS: operations, DSL, store, and forms bridge.

use neural_engine as neural;

use neural::{Neuron, Synapse};
use crate::retained::{FlowOwner, FlowRetirement};
use protocol::value::ordered::{Grant as LayoutGrant, UpdateCursor as LayoutUpdate};

use semio_framework_artifact_flow_flow::*;
use crate::os_store::ErasedSnapshotRetirement;

//#region 🌊️RetainedVcs

pub const FLOW_VCS_MAX_OPERATIONS: usize = 4;
pub const FLOW_VCS_MAX_PAGES: usize = 4;
pub const FLOW_VCS_MAX_ITEMS: usize = 256;
pub const FLOW_VCS_MAX_BYTES: usize = 65_536;
pub const FLOW_VCS_MAX_OUTPUTS: usize = 4;
pub const FLOW_VCS_MAX_EVENTS: usize = 12;
pub const FLOW_VCS_MAX_CONTROLS: usize = 4;
pub const FLOW_VCS_MAX_HISTORY: usize = 256;
pub const FLOW_VCS_MAX_DEPTH: usize = 12;
pub const FLOW_VCS_DEADLINE_MILLISECONDS: u64 = 8;
pub const FLOW_VCS_FEATURES: [&str; 13] = ["addWidget", "removeWidget", "moveWidget", "patchWidget", "addSynapse", "removeSynapse", "moveSynapse", "patchSynapse", "setLayout", "replaceDocument", "undo", "redo", "checkpoint"];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlowVcsCredits {
    pub operations: usize,
    pub pages: usize,
    pub items: usize,
    pub bytes: usize,
    pub outputs: usize,
    pub events: usize,
    pub controls: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlowVcsResourceFingerprint {
    pub credits: FlowVcsCredits,
    pub active_operations: usize,
    pub leased_pages: usize,
    pub undo_owners: usize,
    pub redo_owners: usize,
    pub retired_action_owners: usize,
    pub retired_surface_owners: usize,
    pub revision: u64,
    pub parent_revision: u64,
    pub document_generation: u64,
    pub document_digest: u64,
    pub document_versions: usize,
    pub active_document_version: usize,
    pub edit_owner: Option<u64>,
    pub document_retained: bool,
    pub closing: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FlowVcsCensus {
    items: usize,
    bytes: usize,
    depth: usize,
}

impl FlowVcsCensus {
    fn leaf(bytes: usize) -> Self {
        Self { items: 1, bytes, depth: 1 }
    }

    fn include(&mut self, child: Self) {
        self.items = self.items.saturating_add(child.items);
        self.bytes = self.bytes.saturating_add(child.bytes);
        self.depth = self.depth.max(child.depth.saturating_add(1));
    }
}

#[derive(Debug)]
pub struct FlowVcsSource<T> {
    value: Option<T>,
}

impl<T> FlowVcsSource<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }

    pub fn retained(&self) -> bool {
        self.value.is_some()
    }

    fn get(&self) -> Result<&T, FlowVcsFault> {
        self.value.as_ref().ok_or(FlowVcsFault::SourceExhausted)
    }

    fn take(&mut self) -> T {
        self.value.take().expect("Flow VCS admission checked the retained source")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowVcsAuthority {
    pub session_generation: u32,
    pub base_revision: u64,
    pub parent_revision: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowVcsHandle {
    pub operation: u64,
    pub slot: u8,
    pub generation: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowVcsGrant {
    pub items: usize,
    pub bytes: usize,
    pub outputs: usize,
    pub events: usize,
    pub controls: usize,
    pub fuel: u32,
    pub now_milliseconds: u64,
    pub deadline_milliseconds: u64,
    pub interrupted: bool,
}

impl FlowVcsGrant {
    fn permits_work(self) -> bool {
        !self.interrupted && self.fuel > 0 && self.items > 0 && self.now_milliseconds < self.deadline_milliseconds && self.deadline_milliseconds.saturating_sub(self.now_milliseconds) <= FLOW_VCS_DEADLINE_MILLISECONDS
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowVcsFault {
    Closed,
    Full,
    Limit,
    Depth,
    SourceExhausted,
    WrongHandle,
    StaleHandle,
    StaleAuthority,
    DuplicateControl,
    InsufficientGrant,
    InvalidMutation,
    OutputNotReady,
    OutputAlreadyLeased,
    OutputNotLeased,
    WrongPage,
    Published,
    ClosePending,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowVcsPage {
    pub sequence: u64,
    pub operation: u64,
    pub session_generation: u32,
    pub revision: u64,
    pub parent_revision: u64,
    pub document_generation: u64,
    pub widget_count: u32,
    pub synapse_count: u32,
    pub layout_count: u32,
    pub semantic_digest: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowVcsPoll {
    Progress { completed: u32, total: u32 },
    Checkpoint { operation: u64, revision: u64 },
    Preview { widgets: u32, synapses: u32, layout: u32 },
    PageReady { sequence: u64 },
    Terminal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSurfaceOwner {
    pub surface: u64,
    pub host: u64,
    pub generation: u64,
    pub document: usize,
    pub widgets: usize,
    pub synapses: usize,
    pub previews: usize,
    pub expanded: usize,
    pub layout: usize,
    pub history: usize,
    pub edit: usize,
    pub conflict: usize,
    pub control: usize,
    pub output: usize,
}

impl FlowSurfaceOwner {
    fn from_fixture(surface: u64, host: u64, generation: u64, fixture: &FlowFixture) -> Self {
        let widget_slots = fixture.widgets.len();
        Self { surface, host, generation, document: 1, widgets: widget_slots, synapses: fixture.synapses.len(), previews: widget_slots, expanded: widget_slots, layout: fixture.layout.len(), history: 1, edit: 1, conflict: 1, control: 1, output: 1 }
    }

    fn close_one(&mut self) -> bool {
        if flow_vcs_release_count(&mut self.output) {
            return false;
        }
        if flow_vcs_release_count(&mut self.control) {
            return false;
        }
        if flow_vcs_release_count(&mut self.conflict) {
            return false;
        }
        if flow_vcs_release_count(&mut self.edit) {
            return false;
        }
        if flow_vcs_release_count(&mut self.history) {
            return false;
        }
        if flow_vcs_release_count(&mut self.expanded) {
            return false;
        }
        if flow_vcs_release_count(&mut self.previews) {
            return false;
        }
        if flow_vcs_release_count(&mut self.layout) {
            return false;
        }
        if flow_vcs_release_count(&mut self.synapses) {
            return false;
        }
        if flow_vcs_release_count(&mut self.widgets) {
            return false;
        }
        !flow_vcs_release_count(&mut self.document)
    }
}

fn flow_vcs_release_count(owner: &mut usize) -> bool {
    if *owner == 0 {
        return false;
    }
    *owner -= 1;
    true
}

#[derive(Debug)]
enum FlowVcsAction {
    InsertWidget { index: usize, item: Widget },
    RemoveWidget { id: String },
    RemoveWidgetAt { index: usize },
    MoveWidget { id: String, index: usize },
    PatchWidget { id: String, item: Widget },
    InsertSynapse { index: usize, item: SynapseSpec },
    RemoveSynapse { id: String },
    RemoveSynapseAt { index: usize },
    MoveSynapse { id: String, index: usize },
    PatchSynapse { id: String, item: SynapseSpec },
    SetLayout(FlowLayoutEntry),
    LayoutRoot(OrderedMap<WidgetLayout>),
    ReplaceDocument(FlowFixture),
    ActivateDocument { index: usize },
    Undo,
    Redo,
    Checkpoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowVcsStage {
    Admitted,
    Ready,
    PublishReady,
    PageReady,
    Complete,
    Cancelled,
    Faulted,
    Closing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowVcsCursorPhase {
    LoadHistory,
    Scan,
    Mutate,
    Shift,
    ReserveReplacement,
    ReplaceSchema,
    ReplaceCameraX,
    ReplaceCameraY,
    ReplaceCameraZoom,
    ReplaceWidgets,
    ReverseWidgets,
    ReplaceSynapses,
    ReverseSynapses,
    ReplaceLayout,
    RetireRedo,
    TransferHistory,
    TransferSurface,
    PublishVisibility,
    PublishPage,
    Rollback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowVcsCursorKind {
    None,
    InsertWidget,
    RemoveWidget,
    MoveWidget,
    PatchWidget,
    InsertSynapse,
    RemoveSynapse,
    MoveSynapse,
    PatchSynapse,
    Layout,
    ReplaceDocument,
}

#[derive(Clone, Copy, Debug)]
struct FlowVcsCursor {
    phase: FlowVcsCursorPhase,
    kind: FlowVcsCursorKind,
    scan: usize,
    origin: usize,
    current: usize,
    target: usize,
    history_mode: u8,
    history_loaded: bool,
    redo_retired: usize,
    history_transferred: bool,
    surface_transferred: bool,
    visibility_published: bool,
    prior_generation: u64,
    prior_digest: u64,
    owns_edit: bool,
    mutated: bool,
}

impl FlowVcsCursor {
    fn new(action: &FlowVcsAction) -> Self {
        let (phase, kind, target) = match action {
            FlowVcsAction::Undo | FlowVcsAction::Redo => (FlowVcsCursorPhase::LoadHistory, FlowVcsCursorKind::None, 0),
            FlowVcsAction::ReplaceDocument(_) => (FlowVcsCursorPhase::ReserveReplacement, FlowVcsCursorKind::ReplaceDocument, 0),
            FlowVcsAction::Checkpoint => (FlowVcsCursorPhase::TransferHistory, FlowVcsCursorKind::None, 0),
            FlowVcsAction::InsertWidget { index, .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::InsertWidget, *index),
            FlowVcsAction::RemoveWidget { .. } | FlowVcsAction::RemoveWidgetAt { .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::RemoveWidget, 0),
            FlowVcsAction::MoveWidget { index, .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::MoveWidget, *index),
            FlowVcsAction::PatchWidget { .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::PatchWidget, 0),
            FlowVcsAction::InsertSynapse { index, .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::InsertSynapse, *index),
            FlowVcsAction::RemoveSynapse { .. } | FlowVcsAction::RemoveSynapseAt { .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::RemoveSynapse, 0),
            FlowVcsAction::MoveSynapse { index, .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::MoveSynapse, *index),
            FlowVcsAction::PatchSynapse { .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::PatchSynapse, 0),
            FlowVcsAction::SetLayout(_) => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::Layout, 0),
            FlowVcsAction::LayoutRoot(_) => (FlowVcsCursorPhase::Mutate, FlowVcsCursorKind::Layout, 0),
            FlowVcsAction::ActivateDocument { index } => (FlowVcsCursorPhase::Mutate, FlowVcsCursorKind::ReplaceDocument, *index),
        };
        let history_mode = match action {
            FlowVcsAction::Undo => 1,
            FlowVcsAction::Redo => 2,
            FlowVcsAction::Checkpoint => 3,
            _ => 0,
        };
        Self {
            phase,
            kind,
            scan: 0,
            origin: 0,
            current: 0,
            target,
            history_mode,
            history_loaded: false,
            redo_retired: 0,
            history_transferred: false,
            surface_transferred: false,
            visibility_published: false,
            prior_generation: 0,
            prior_digest: 0,
            owns_edit: false,
            mutated: false,
        }
    }
}

struct FlowVcsOperation {
    handle: FlowVcsHandle,
    authority: FlowVcsAuthority,
    source: FlowVcsCensus,
    action: Option<FlowVcsAction>,
    rollback_owner: Option<FlowVcsAction>,
    layout_update: Option<LayoutUpdate<WidgetLayout>>,
    retirement: FlowRetirement,
    cursor: FlowVcsCursor,
    page: Option<FlowVcsPage>,
    page_leased: bool,
    delivery_held: bool,
    stage: FlowVcsStage,
    close_phase: u8,
}

struct FlowFixedOwners<T, const N: usize> {
    slots: [Option<T>; N],
    length: usize,
}

impl<T, const N: usize> FlowFixedOwners<T, N> {
    fn new() -> Self {
        Self { slots: [const { None }; N], length: 0 }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn is_full(&self) -> bool {
        self.length == N
    }

    fn remaining(&self) -> usize {
        N - self.length
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.slots[self.length] = Some(value);
        self.length += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        self.slots[self.length].take()
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        self.length.checked_sub(1).and_then(|index| self.slots[index].as_mut())
    }

    fn get(&self, index: usize) -> Option<&T> {
        (index < self.length).then(|| self.slots[index].as_ref()).flatten()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.length).then(|| self.slots[index].as_mut()).flatten()
    }
}

struct FlowVcsDocument {
    versions: FlowFixedOwners<FlowFixture, FLOW_VCS_MAX_HISTORY>,
    active: usize,
    revision: u64,
    parent_revision: u64,
    generation: u64,
    committed_digest: u64,
    edit_owner: Option<u64>,
    surface: Option<FlowSurfaceOwner>,
}

impl FlowVcsDocument {
    fn new(fixture: FlowFixture, revision: u64, parent_revision: u64) -> Self {
        let committed_digest = flow_vcs_fixture_scalar_digest(&fixture);
        let mut versions = FlowFixedOwners::new();
        let _ = versions.push(fixture);
        Self { versions, active: 0, revision, parent_revision, generation: 1, committed_digest, edit_owner: None, surface: None }
    }

    fn fixture(&self) -> &FlowFixture {
        self.versions.get(self.active).expect("active Flow VCS document version")
    }

    fn fixture_mut(&mut self) -> &mut FlowFixture {
        self.versions.get_mut(self.active).expect("active Flow VCS document version")
    }
}

pub struct FlowRetainedVcs {
    session_generation: u32,
    document: Option<FlowVcsDocument>,
    operations: [Option<FlowVcsOperation>; FLOW_VCS_MAX_OPERATIONS],
    slot_generations: [u32; FLOW_VCS_MAX_OPERATIONS],
    next_operation: u64,
    next_page: u64,
    credits: FlowVcsCredits,
    undo: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    redo: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    retired_actions: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    retired_surfaces: FlowFixedOwners<FlowSurfaceOwner, FLOW_VCS_MAX_HISTORY>,
    retirement: FlowRetirement,
    closing: bool,
}

impl FlowRetainedVcs {
    pub fn new(document: FlowFixture, session_generation: u32, revision: u64, parent_revision: u64) -> Self {
        Self {
            session_generation,
            document: Some(FlowVcsDocument::new(document, revision, parent_revision)),
            operations: [const { None }; FLOW_VCS_MAX_OPERATIONS],
            slot_generations: [1; FLOW_VCS_MAX_OPERATIONS],
            next_operation: 1,
            next_page: 1,
            credits: FlowVcsCredits::default(),
            undo: FlowFixedOwners::new(),
            redo: FlowFixedOwners::new(),
            retired_actions: FlowFixedOwners::new(),
            retired_surfaces: FlowFixedOwners::new(),
            retirement: FlowRetirement::default(),
            closing: false,
        }
    }

    pub fn authority(&self) -> FlowVcsAuthority {
        let document = self.document.as_ref().expect("open Flow VCS document");
        FlowVcsAuthority { session_generation: self.session_generation, base_revision: document.revision, parent_revision: document.parent_revision }
    }

    pub fn credits(&self) -> FlowVcsCredits {
        self.credits
    }

    pub fn resource_fingerprint(&self) -> FlowVcsResourceFingerprint {
        let active_operations = self.credits.operations;
        let leased_pages = usize::from(self.operations[0].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[1].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[2].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[3].as_ref().is_some_and(|operation| operation.page_leased));
        let (revision, parent_revision, document_generation, document_digest, document_versions, active_document_version, edit_owner, document_retained) = match &self.document {
            Some(document) => (document.revision, document.parent_revision, document.generation, document.committed_digest, document.versions.len(), document.active, document.edit_owner, true),
            None => (0, 0, 0, 0, 0, 0, None, false),
        };
        FlowVcsResourceFingerprint {
            credits: self.credits,
            active_operations,
            leased_pages,
            undo_owners: self.undo.len(),
            redo_owners: self.redo.len(),
            retired_action_owners: self.retired_actions.len(),
            retired_surface_owners: self.retired_surfaces.len(),
            revision,
            parent_revision,
            document_generation,
            document_digest,
            document_versions,
            active_document_version,
            edit_owner,
            document_retained,
            closing: self.closing,
        }
    }

    pub fn bind_surface(&mut self, surface: u64, host: u64, generation: u64) -> Result<(), FlowVcsFault> {
        if self.closing {
            return Err(FlowVcsFault::Closed);
        }
        let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
        if document.surface.is_some() || self.retired_surfaces.len() == FLOW_VCS_MAX_HISTORY {
            return Err(FlowVcsFault::Full);
        }
        document.surface = Some(FlowSurfaceOwner::from_fixture(surface, host, generation, document.fixture()));
        Ok(())
    }

    pub fn begin_add_widget(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<Widget>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = flow_vcs_widget_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::InsertWidget { index, item: source.take() })
    }

    pub fn begin_remove_widget(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::RemoveWidget { id: source.take() })
    }

    pub fn begin_move_widget(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::MoveWidget { id: source.take(), index })
    }

    pub fn begin_patch_widget(&mut self, authority: FlowVcsAuthority, id: &mut FlowVcsSource<String>, source: &mut FlowVcsSource<Widget>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let mut census = flow_vcs_widget_census(source.get()?);
        census.include(FlowVcsCensus::leaf(id.get()?.len()));
        if widget_id_for(source.get()?) != id.get()?.as_str() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::PatchWidget { id: id.take(), item: source.take() })
    }

    pub fn begin_add_synapse(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<SynapseSpec>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = flow_vcs_synapse_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::InsertSynapse { index, item: source.take() })
    }

    pub fn begin_remove_synapse(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::RemoveSynapse { id: source.take() })
    }

    pub fn begin_move_synapse(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::MoveSynapse { id: source.take(), index })
    }

    pub fn begin_patch_synapse(&mut self, authority: FlowVcsAuthority, id: &mut FlowVcsSource<String>, source: &mut FlowVcsSource<SynapseSpec>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let mut census = flow_vcs_synapse_census(source.get()?);
        census.include(FlowVcsCensus::leaf(id.get()?.len()));
        if source.get()?.id.as_str() != id.get()?.as_str() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::PatchSynapse { id: id.take(), item: source.take() })
    }

    pub fn begin_set_layout(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<FlowLayoutEntry>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.id.len() + 16);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::SetLayout(source.take()))
    }

    pub fn begin_replace_document(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<FlowFixture>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = flow_vcs_fixture_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::ReplaceDocument(source.take()))
    }

    pub fn begin_undo(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        if self.undo.is_empty() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Undo)
    }

    pub fn begin_redo(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        if self.redo.is_empty() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Redo)
    }

    pub fn begin_checkpoint(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Checkpoint)
    }

    pub fn poll(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        if !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let slot = self.slot(handle)?;
        match self.operations[slot].as_ref().expect("validated Flow VCS operation").stage {
            FlowVcsStage::Admitted => {
                self.operations[slot].as_mut().expect("validated Flow VCS operation").stage = FlowVcsStage::Ready;
                Ok(FlowVcsPoll::Progress { completed: 1, total: 3 })
            }
            FlowVcsStage::Ready => {
                self.operations[slot].as_mut().expect("validated Flow VCS operation").stage = FlowVcsStage::PublishReady;
                let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
                Ok(FlowVcsPoll::Checkpoint { operation: operation.handle.operation, revision: operation.authority.base_revision })
            }
            FlowVcsStage::PublishReady => self.step_action_cursor(slot, grant),
            FlowVcsStage::PageReady => Ok(FlowVcsPoll::PageReady { sequence: self.operations[slot].as_ref().and_then(|operation| operation.page.as_ref()).ok_or(FlowVcsFault::OutputNotReady)?.sequence }),
            FlowVcsStage::Complete => Ok(FlowVcsPoll::Terminal),
            _ => Err(FlowVcsFault::ClosePending),
        }
    }

    pub fn take_page(&mut self, handle: FlowVcsHandle) -> Result<FlowVcsPage, FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.stage != FlowVcsStage::PageReady {
            return Err(FlowVcsFault::OutputNotReady);
        }
        if operation.page_leased {
            return Err(FlowVcsFault::OutputAlreadyLeased);
        }
        let page = *operation.page.as_ref().ok_or(FlowVcsFault::OutputNotReady)?;
        operation.page_leased = true;
        Ok(page)
    }

    pub fn resume_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<(), FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
            return Err(FlowVcsFault::WrongPage);
        }
        if !operation.page_leased {
            return Err(FlowVcsFault::OutputNotLeased);
        }
        operation.page_leased = false;
        Ok(())
    }

    pub fn retry_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<FlowVcsPage, FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
            return Err(FlowVcsFault::WrongPage);
        }
        if operation.page_leased {
            return Err(FlowVcsFault::OutputAlreadyLeased);
        }
        operation.page_leased = true;
        Ok(*operation.page.as_ref().expect("validated Flow VCS page"))
    }

    pub fn acknowledge_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<(), FlowVcsFault> {
        {
            let operation = self.operation_mut(handle)?;
            if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
                return Err(FlowVcsFault::WrongPage);
            }
            if !operation.page_leased {
                return Err(FlowVcsFault::OutputNotLeased);
            }
            operation.page = None;
            operation.page_leased = false;
            operation.stage = FlowVcsStage::Complete;
            operation.delivery_held = false;
        }
        self.credits.pages -= 1;
        self.credits.outputs -= 1;
        self.credits.events -= 3;
        Ok(())
    }

    pub fn cancel(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let operation = self.operation_mut(handle)?;
        if !matches!(operation.stage, FlowVcsStage::Admitted | FlowVcsStage::Ready | FlowVcsStage::PublishReady) {
            return Err(if matches!(operation.stage, FlowVcsStage::PageReady | FlowVcsStage::Complete) { FlowVcsFault::Published } else { FlowVcsFault::DuplicateControl });
        }
        operation.stage = FlowVcsStage::Cancelled;
        if operation.cursor.mutated
            || operation.cursor.owns_edit
            || operation.cursor.redo_retired > 0
            || operation.cursor.history_transferred
            || operation.cursor.surface_transferred
            || operation.cursor.visibility_published
            || operation.cursor.history_loaded
        {
            operation.cursor.phase = FlowVcsCursorPhase::Rollback;
        }
        Ok(())
    }

    pub fn fault(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let operation = self.operation_mut(handle)?;
        if !matches!(operation.stage, FlowVcsStage::Admitted | FlowVcsStage::Ready | FlowVcsStage::PublishReady) {
            return Err(FlowVcsFault::DuplicateControl);
        }
        operation.stage = FlowVcsStage::Faulted;
        if operation.cursor.mutated
            || operation.cursor.owns_edit
            || operation.cursor.redo_retired > 0
            || operation.cursor.history_transferred
            || operation.cursor.surface_transferred
            || operation.cursor.visibility_published
            || operation.cursor.history_loaded
        {
            operation.cursor.phase = FlowVcsCursorPhase::Rollback;
        }
        Ok(())
    }

    pub fn panic_fault(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        self.fault(handle, grant)
    }

    pub fn rediscover(&self, operation: u64, generation: u32) -> Result<FlowVcsHandle, FlowVcsFault> {
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[0].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[1].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[2].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[3].as_ref(), operation, generation) {
            return Ok(handle);
        }
        Err(FlowVcsFault::StaleHandle)
    }

    pub fn close_operation_step(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<bool, FlowVcsFault> {
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let slot = self.slot(handle)?;
        let stage = self.operations[slot].as_ref().expect("validated Flow VCS operation").stage;
        if !matches!(stage, FlowVcsStage::Complete | FlowVcsStage::Cancelled | FlowVcsStage::Faulted | FlowVcsStage::Closing) {
            return Err(FlowVcsFault::ClosePending);
        }
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.stage = FlowVcsStage::Closing;
        if let Some(update) = operation.layout_update.as_mut() {
            update.begin_close();
            update.close_step(LayoutGrant { maximum_items: 1, maximum_bytes: grant.bytes });
            if update.terminal_is_empty() { operation.layout_update = None; }
            return Ok(false);
        }
        if !operation.retirement.is_empty() {
            operation.retirement.close_step(1, grant.bytes).map_err(|_| FlowVcsFault::ClosePending)?;
            return Ok(false);
        }
        if operation.cursor.phase == FlowVcsCursorPhase::Rollback {
            if operation.cursor.visibility_published {
                let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
                document.revision = operation.authority.base_revision;
                document.parent_revision = operation.authority.parent_revision;
                document.generation = operation.cursor.prior_generation;
                document.committed_digest = operation.cursor.prior_digest;
                operation.cursor.visibility_published = false;
                operation.stage = stage;
                return Ok(false);
            }
            if operation.cursor.surface_transferred {
                let surface = self.retired_surfaces.pop().ok_or(FlowVcsFault::InvalidMutation)?;
                self.document.as_mut().ok_or(FlowVcsFault::Closed)?.surface = Some(surface);
                operation.cursor.surface_transferred = false;
                operation.stage = stage;
                return Ok(false);
            }
            if operation.cursor.history_transferred {
                let action = match operation.cursor.history_mode {
                    0 | 2 => self.undo.pop(),
                    1 => self.redo.pop(),
                    _ => None,
                }
                .ok_or(FlowVcsFault::InvalidMutation)?;
                operation.action = Some(action);
                operation.cursor.history_transferred = false;
                operation.stage = stage;
                return Ok(false);
            }
            if operation.cursor.redo_retired > 0 {
                let action = self.retired_actions.pop().ok_or(FlowVcsFault::InvalidMutation)?;
                self.redo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.redo_retired -= 1;
                operation.stage = stage;
                return Ok(false);
            }
            let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if !flow_vcs_step_rollback(document, operation)? {
                operation.stage = stage;
                return Ok(false);
            }
            if operation.cursor.owns_edit {
                document.edit_owner = None;
                operation.cursor.owns_edit = false;
            }
            if operation.cursor.history_loaded && operation.cursor.history_mode == 1 {
                let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
                self.undo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.history_mode = 0;
                operation.cursor.history_loaded = false;
            } else if operation.cursor.history_loaded && operation.cursor.history_mode == 2 {
                let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
                self.redo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.history_mode = 0;
                operation.cursor.history_loaded = false;
            }
            operation.cursor.phase = FlowVcsCursorPhase::Scan;
            operation.stage = stage;
            return Ok(false);
        }
        match operation.close_phase {
            0 => {
                operation.page = None;
                operation.page_leased = false;
                operation.close_phase = 1;
                Ok(false)
            }
            1 => {
                if let Some(action) = operation.action.take() {
                    if self.retired_actions.len() == FLOW_VCS_MAX_HISTORY {
                        operation.action = Some(action);
                        return Err(FlowVcsFault::Full);
                    }
                    self.retired_actions.push(action).map_err(|action| {
                        operation.action = Some(action);
                        FlowVcsFault::Full
                    })?;
                }
                operation.close_phase = 2;
                Ok(false)
            }
            _ => {
                if let Some(action) = operation.rollback_owner.take() {
                    flow_vcs_retire_action(action, &mut operation.retirement);
                    return Ok(false);
                }
                let operation = self.operations[slot].take().expect("validated Flow VCS operation");
                self.hand_back(operation.source, operation.delivery_held);
                self.slot_generations[slot] = self.slot_generations[slot].checked_add(1).ok_or(FlowVcsFault::Limit)?;
                Ok(true)
            }
        }
    }

    pub fn close_retired_step(&mut self, grant: FlowVcsGrant) -> Result<bool, FlowVcsFault> {
        if self.terminal_is_empty() {
            return Ok(true);
        }
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        if self.credits.operations > 0 {
            return Err(FlowVcsFault::ClosePending);
        }
        if !self.retirement.is_empty() {
            self.retirement.close_step(1, grant.bytes).map_err(|_| FlowVcsFault::ClosePending)?;
            return Ok(false);
        }
        if let Some(surface) = self.retired_surfaces.last_mut() {
            if surface.close_one() {
                self.retired_surfaces.pop();
            }
            return Ok(false);
        }
        if let Some(action) = self.retired_actions.pop() {
            flow_vcs_retire_action(action, &mut self.retirement);
            return Ok(false);
        }
        if self.closing {
            if let Some(action) = self.undo.pop().or_else(|| self.redo.pop()) {
                flow_vcs_retire_action(action, &mut self.retirement);
                return Ok(false);
            }
        }
        if self.closing && self.credits.operations == 0 {
            let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if let Some(surface) = document.surface.take() {
                self.retired_surfaces.push(surface).map_err(|surface| {
                    document.surface = Some(surface);
                    FlowVcsFault::Full
                })?;
                return Ok(false);
            }
            if let Some(fixture) = document.versions.pop() {
                self.retirement.push(FlowOwner::Fixture(fixture));
                return Ok(false);
            }
            self.document = None;
        }
        Ok(true)
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.document.is_none() && self.credits.operations == 0 && self.credits == FlowVcsCredits::default() && self.undo.is_empty() && self.redo.is_empty() && self.retired_actions.is_empty() && self.retired_surfaces.is_empty() && self.retirement.is_empty()
    }

    fn preflight(&self, census: FlowVcsCensus) -> Result<(), FlowVcsFault> {
        if self.closing || self.document.is_none() {
            return Err(FlowVcsFault::Closed);
        }
        if census.depth > FLOW_VCS_MAX_DEPTH {
            return Err(FlowVcsFault::Depth);
        }
        if census.items > FLOW_VCS_MAX_ITEMS || census.bytes > FLOW_VCS_MAX_BYTES {
            return Err(FlowVcsFault::Limit);
        }
        if self.credits.operations == FLOW_VCS_MAX_OPERATIONS || !self.can_charge(census) {
            return Err(FlowVcsFault::Full);
        }
        if self.next_operation == u64::MAX || self.next_page == u64::MAX {
            return Err(FlowVcsFault::Limit);
        }
        Ok(())
    }

    fn admit(&mut self, authority: FlowVcsAuthority, source: FlowVcsCensus, action: FlowVcsAction) -> Result<FlowVcsHandle, FlowVcsFault> {
        let slot = if self.operations[0].is_none() {
            0
        } else if self.operations[1].is_none() {
            1
        } else if self.operations[2].is_none() {
            2
        } else if self.operations[3].is_none() {
            3
        } else {
            return Err(FlowVcsFault::Full);
        };
        let handle = FlowVcsHandle { operation: self.next_operation, slot: slot as u8, generation: self.slot_generations[slot] };
        self.charge(source);
        let cursor = FlowVcsCursor::new(&action);
        self.operations[slot] = Some(FlowVcsOperation { handle, authority, source, action: Some(action), rollback_owner: None, layout_update: None, retirement: FlowRetirement::default(), cursor, page: None, page_leased: false, delivery_held: true, stage: FlowVcsStage::Admitted, close_phase: 0 });
        self.next_operation += 1;
        Ok(handle)
    }

    fn can_charge(&self, source: FlowVcsCensus) -> bool {
        self.credits.operations < FLOW_VCS_MAX_OPERATIONS
            && self.credits.pages < FLOW_VCS_MAX_PAGES
            && self.credits.items.checked_add(source.items).is_some_and(|value| value <= FLOW_VCS_MAX_ITEMS)
            && self.credits.bytes.checked_add(source.bytes).is_some_and(|value| value <= FLOW_VCS_MAX_BYTES)
            && self.credits.outputs < FLOW_VCS_MAX_OUTPUTS
            && self.credits.events.checked_add(3).is_some_and(|value| value <= FLOW_VCS_MAX_EVENTS)
            && self.credits.controls < FLOW_VCS_MAX_CONTROLS
    }

    fn charge(&mut self, source: FlowVcsCensus) {
        self.credits.operations += 1;
        self.credits.pages += 1;
        self.credits.items += source.items;
        self.credits.bytes += source.bytes;
        self.credits.outputs += 1;
        self.credits.events += 3;
        self.credits.controls += 1;
    }

    fn hand_back(&mut self, source: FlowVcsCensus, delivery: bool) {
        self.credits.operations -= 1;
        self.credits.items -= source.items;
        self.credits.bytes -= source.bytes;
        self.credits.controls -= 1;
        if delivery {
            self.credits.pages -= 1;
            self.credits.outputs -= 1;
            self.credits.events -= 3;
        }
    }

    fn slot(&self, handle: FlowVcsHandle) -> Result<usize, FlowVcsFault> {
        let slot = usize::from(handle.slot);
        let operation = self.operations.get(slot).and_then(Option::as_ref).ok_or(FlowVcsFault::StaleHandle)?;
        if operation.handle.operation != handle.operation {
            return Err(FlowVcsFault::WrongHandle);
        }
        if operation.handle.generation != handle.generation {
            return Err(FlowVcsFault::StaleHandle);
        }
        Ok(slot)
    }

    fn operation_mut(&mut self, handle: FlowVcsHandle) -> Result<&mut FlowVcsOperation, FlowVcsFault> {
        let slot = self.slot(handle)?;
        Ok(self.operations[slot].as_mut().expect("validated Flow VCS operation"))
    }

    //#region 🌊️RetainedActionCursor
    fn step_action_cursor(&mut self, slot: usize, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
        let document = self.document.as_ref().ok_or(FlowVcsFault::Closed)?;
        if document.edit_owner.is_some_and(|owner| owner != operation.handle.operation) {
            return Err(FlowVcsFault::Full);
        }
        let authority_matches = if operation.cursor.visibility_published {
            operation.authority.base_revision.checked_add(1) == Some(document.revision) && document.parent_revision == operation.authority.base_revision
        } else {
            operation.authority.base_revision == document.revision && operation.authority.parent_revision == document.parent_revision
        };
        if operation.authority.session_generation != self.session_generation || !authority_matches {
            return Err(FlowVcsFault::StaleAuthority);
        }
        if !operation.cursor.surface_transferred && document.surface.is_some() && self.retired_surfaces.is_full() {
            return Err(FlowVcsFault::Full);
        }
        let history_mode = operation.cursor.history_mode;
        let history_capacity = match history_mode {
            0 => self.undo.remaining() >= 1 && self.retired_actions.remaining() >= self.redo.len(),
            1 => self.redo.remaining() >= 1,
            2 => self.undo.remaining() >= 1,
            _ => true,
        };
        if !operation.cursor.history_transferred && !history_capacity {
            return Err(FlowVcsFault::Full);
        }
        let phase = operation.cursor.phase;
        if flow_vcs_cursor_requires_edit(phase) && !operation.cursor.owns_edit {
            let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if document.edit_owner.is_some() {
                return Err(FlowVcsFault::Full);
            }
            document.edit_owner = Some(operation.handle.operation);
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.owns_edit = true;
        }
        if phase == FlowVcsCursorPhase::LoadHistory {
            self.load_history_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::RetireRedo {
            if let Some(action) = self.redo.pop() {
                self.retired_actions.push(action).map_err(|_| FlowVcsFault::Full)?;
                self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.redo_retired += 1;
                return Ok(self.cursor_progress(slot));
            }
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.phase = FlowVcsCursorPhase::TransferHistory;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::TransferHistory {
            self.transfer_history_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::TransferSurface {
            self.transfer_surface_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::PublishVisibility {
            self.publish_visibility_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::PublishPage {
            return self.publish_page_cursor(slot, grant);
        }
        let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        flow_vcs_step_cursor(document, operation, grant)?;
        if operation.cursor.phase == FlowVcsCursorPhase::TransferHistory && operation.cursor.history_mode == 0 && !self.redo.is_empty() {
            operation.cursor.phase = FlowVcsCursorPhase::RetireRedo;
        }
        Ok(self.cursor_progress(slot))
    }

    fn load_history_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        let command = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
        let (action, history_mode) = match command {
            FlowVcsAction::Undo => (self.undo.pop().ok_or(FlowVcsFault::InvalidMutation)?, 1),
            FlowVcsAction::Redo => (self.redo.pop().ok_or(FlowVcsFault::InvalidMutation)?, 2),
            _ => return Err(FlowVcsFault::InvalidMutation),
        };
        let mut cursor = FlowVcsCursor::new(&action);
        cursor.history_mode = history_mode;
        cursor.history_loaded = true;
        cursor.owns_edit = true;
        operation.action = Some(action);
        operation.cursor = cursor;
        Ok(())
    }

    fn cursor_progress(&self, slot: usize) -> FlowVcsPoll {
        let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
        FlowVcsPoll::Progress { completed: u32::try_from(operation.cursor.scan.saturating_add(2)).unwrap_or(u32::MAX), total: u32::try_from(operation.source.items.saturating_add(6)).unwrap_or(u32::MAX) }
    }

    fn transfer_history_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let history_mode = self.operations[slot].as_ref().expect("validated Flow VCS operation").cursor.history_mode;
        if history_mode == 0 && self.undo.is_full() || history_mode == 1 && self.redo.is_full() || history_mode == 2 && self.undo.is_full() {
            return Err(FlowVcsFault::Full);
        }
        let inverse = self.operations[slot].as_mut().expect("validated Flow VCS operation").action.take().ok_or(FlowVcsFault::InvalidMutation)?;
        match history_mode {
            0 => self.undo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            1 => self.redo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            2 => self.undo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            _ => drop(inverse),
        }
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.history_transferred = history_mode != 3;
        operation.cursor.phase = FlowVcsCursorPhase::TransferSurface;
        Ok(())
    }

    fn transfer_surface_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let document = self.document.as_mut().expect("open Flow VCS document");
        if let Some(surface) = document.surface.take() {
            self.retired_surfaces.push(surface).map_err(|surface| {
                document.surface = Some(surface);
                FlowVcsFault::Full
            })?;
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.surface_transferred = true;
        }
        self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.phase = FlowVcsCursorPhase::PublishVisibility;
        Ok(())
    }

    fn publish_visibility_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let document = self.document.as_mut().expect("open Flow VCS document");
        let revision = document.revision.checked_add(1).ok_or(FlowVcsFault::Limit)?;
        let generation = document.generation.checked_add(1).ok_or(FlowVcsFault::Limit)?;
        let widget_count = u32::try_from(document.fixture().widgets.len()).unwrap_or(u32::MAX);
        let synapse_count = u32::try_from(document.fixture().synapses.len()).unwrap_or(u32::MAX);
        let layout_count = u32::try_from(document.fixture().layout.len()).unwrap_or(u32::MAX);
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.prior_generation = document.generation;
        operation.cursor.prior_digest = document.committed_digest;
        operation.cursor.visibility_published = true;
        document.parent_revision = document.revision;
        document.revision = revision;
        document.generation = generation;
        document.committed_digest =
            document.committed_digest.rotate_left(13) ^ revision ^ u64::from(widget_count).rotate_left(7) ^ u64::from(synapse_count).rotate_left(17) ^ u64::from(layout_count).rotate_left(29) ^ u64::try_from(document.active).unwrap_or(u64::MAX);
        operation.cursor.phase = FlowVcsCursorPhase::PublishPage;
        Ok(())
    }

    fn publish_page_cursor(&mut self, slot: usize, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        if grant.outputs == 0 || grant.events == 0 || grant.bytes < size_of::<FlowVcsPage>() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let document = self.document.as_ref().ok_or(FlowVcsFault::Closed)?;
        let widget_count = u32::try_from(document.fixture().widgets.len()).unwrap_or(u32::MAX);
        let synapse_count = u32::try_from(document.fixture().synapses.len()).unwrap_or(u32::MAX);
        let layout_count = u32::try_from(document.fixture().layout.len()).unwrap_or(u32::MAX);
        let page = FlowVcsPage {
            sequence: self.next_page,
            operation: self.operations[slot].as_ref().expect("validated Flow VCS operation").handle.operation,
            session_generation: self.session_generation,
            revision: document.revision,
            parent_revision: document.parent_revision,
            document_generation: document.generation,
            widget_count,
            synapse_count,
            layout_count,
            semantic_digest: document.committed_digest,
        };
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.owns_edit = false;
        operation.page = Some(page);
        operation.stage = FlowVcsStage::PageReady;
        self.document.as_mut().expect("open Flow VCS document").edit_owner = None;
        self.next_page += 1;
        Ok(FlowVcsPoll::Preview { widgets: page.widget_count, synapses: page.synapse_count, layout: page.layout_count })
    }
}

fn flow_vcs_rediscovered_handle(operation: Option<&FlowVcsOperation>, expected_operation: u64, generation: u32) -> Option<FlowVcsHandle> {
    let handle = operation?.handle;
    (handle.operation == expected_operation && handle.generation == generation).then_some(handle)
}

fn flow_vcs_cursor_requires_edit(phase: FlowVcsCursorPhase) -> bool {
    matches!(
        phase,
        FlowVcsCursorPhase::LoadHistory
            | FlowVcsCursorPhase::Mutate
            | FlowVcsCursorPhase::Shift
            | FlowVcsCursorPhase::ReserveReplacement
            | FlowVcsCursorPhase::ReplaceSchema
            | FlowVcsCursorPhase::ReplaceCameraX
            | FlowVcsCursorPhase::ReplaceCameraY
            | FlowVcsCursorPhase::ReplaceCameraZoom
            | FlowVcsCursorPhase::ReplaceWidgets
            | FlowVcsCursorPhase::ReverseWidgets
            | FlowVcsCursorPhase::ReplaceSynapses
            | FlowVcsCursorPhase::ReverseSynapses
            | FlowVcsCursorPhase::ReplaceLayout
            | FlowVcsCursorPhase::RetireRedo
            | FlowVcsCursorPhase::TransferHistory
            | FlowVcsCursorPhase::TransferSurface
            | FlowVcsCursorPhase::PublishVisibility
            | FlowVcsCursorPhase::PublishPage
    )
}

fn flow_vcs_step_cursor(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
    match operation.cursor.phase {
        FlowVcsCursorPhase::Scan => flow_vcs_step_scan(document.fixture(), operation),
        FlowVcsCursorPhase::Mutate => flow_vcs_step_mutation(document, operation, grant),
        FlowVcsCursorPhase::Shift => flow_vcs_step_shift(document.fixture_mut(), operation),
        FlowVcsCursorPhase::ReserveReplacement
        | FlowVcsCursorPhase::ReplaceSchema
        | FlowVcsCursorPhase::ReplaceCameraX
        | FlowVcsCursorPhase::ReplaceCameraY
        | FlowVcsCursorPhase::ReplaceCameraZoom
        | FlowVcsCursorPhase::ReplaceWidgets
        | FlowVcsCursorPhase::ReverseWidgets
        | FlowVcsCursorPhase::ReplaceSynapses
        | FlowVcsCursorPhase::ReverseSynapses
        | FlowVcsCursorPhase::ReplaceLayout => flow_vcs_step_document_replacement(document, operation),
        _ => Err(FlowVcsFault::InvalidMutation),
    }
}

fn flow_vcs_step_scan(fixture: &FlowFixture, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let index = operation.cursor.scan;
    let action = operation.action.as_ref().ok_or(FlowVcsFault::InvalidMutation)?;
    match action {
        FlowVcsAction::InsertWidget { index: target, item } => {
            if *target > fixture.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if index == fixture.widgets.len() {
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
            if widget_id_for(&fixture.widgets[index]) == widget_id_for(item) {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsAction::RemoveWidgetAt { index: target } => {
            if *target >= fixture.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            operation.cursor.origin = *target;
            operation.cursor.current = *target;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            return Ok(());
        }
        FlowVcsAction::RemoveWidget { id } | FlowVcsAction::MoveWidget { id, .. } | FlowVcsAction::PatchWidget { id, .. } => {
            if index == fixture.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if widget_id_for(&fixture.widgets[index]) == id {
                operation.cursor.origin = index;
                operation.cursor.current = index;
                operation.cursor.phase = if matches!(action, FlowVcsAction::PatchWidget { .. }) { FlowVcsCursorPhase::Mutate } else { FlowVcsCursorPhase::Shift };
                return Ok(());
            }
        }
        FlowVcsAction::InsertSynapse { index: target, item } => {
            if *target > fixture.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if index == fixture.synapses.len() {
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
            if fixture.synapses[index].id == item.id {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsAction::RemoveSynapseAt { index: target } => {
            if *target >= fixture.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            operation.cursor.origin = *target;
            operation.cursor.current = *target;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            return Ok(());
        }
        FlowVcsAction::RemoveSynapse { id } | FlowVcsAction::MoveSynapse { id, .. } | FlowVcsAction::PatchSynapse { id, .. } => {
            if index == fixture.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if fixture.synapses[index].id == *id {
                operation.cursor.origin = index;
                operation.cursor.current = index;
                operation.cursor.phase = if matches!(action, FlowVcsAction::PatchSynapse { .. }) { FlowVcsCursorPhase::Mutate } else { FlowVcsCursorPhase::Shift };
                return Ok(());
            }
        }
        FlowVcsAction::SetLayout(entry) => {
            if index == fixture.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if widget_id_for(&fixture.widgets[index]) == entry.id {
                operation.cursor.origin = index;
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    operation.cursor.scan += 1;
    Ok(())
}

fn flow_vcs_step_shift(fixture: &mut FlowFixture, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let cursor = &mut operation.cursor;
    match cursor.kind {
        FlowVcsCursorKind::InsertWidget | FlowVcsCursorKind::InsertSynapse => {
            if cursor.current > cursor.target {
                if cursor.kind == FlowVcsCursorKind::InsertWidget {
                    fixture.widgets.swap(cursor.current, cursor.current - 1);
                } else {
                    fixture.synapses.swap(cursor.current, cursor.current - 1);
                }
                cursor.current -= 1;
                return Ok(());
            }
        }
        FlowVcsCursorKind::RemoveWidget | FlowVcsCursorKind::RemoveSynapse => {
            let length = if cursor.kind == FlowVcsCursorKind::RemoveWidget { fixture.widgets.len() } else { fixture.synapses.len() };
            if cursor.current + 1 < length {
                if cursor.kind == FlowVcsCursorKind::RemoveWidget {
                    fixture.widgets.swap(cursor.current, cursor.current + 1);
                } else {
                    fixture.synapses.swap(cursor.current, cursor.current + 1);
                }
                cursor.current += 1;
                cursor.mutated = true;
                return Ok(());
            }
        }
        FlowVcsCursorKind::MoveWidget | FlowVcsCursorKind::MoveSynapse => {
            if cursor.current < cursor.target {
                if cursor.kind == FlowVcsCursorKind::MoveWidget {
                    fixture.widgets.swap(cursor.current, cursor.current + 1);
                } else {
                    fixture.synapses.swap(cursor.current, cursor.current + 1);
                }
                cursor.current += 1;
                cursor.mutated = true;
                return Ok(());
            }
            if cursor.current > cursor.target {
                if cursor.kind == FlowVcsCursorKind::MoveWidget {
                    fixture.widgets.swap(cursor.current, cursor.current - 1);
                } else {
                    fixture.synapses.swap(cursor.current, cursor.current - 1);
                }
                cursor.current -= 1;
                cursor.mutated = true;
                return Ok(());
            }
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    cursor.phase = if matches!(cursor.kind, FlowVcsCursorKind::InsertWidget | FlowVcsCursorKind::InsertSynapse) { FlowVcsCursorPhase::TransferHistory } else { FlowVcsCursorPhase::Mutate };
    Ok(())
}

fn flow_vcs_step_mutation(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
    if let Some(update) = operation.layout_update.as_mut() {
        update.advance(LayoutGrant { maximum_items: 1, maximum_bytes: grant.bytes });
        if let Some(layout) = update.take_result() {
            let previous = std::mem::replace(&mut document.fixture_mut().layout, layout);
            operation.action = Some(FlowVcsAction::LayoutRoot(previous));
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
        }
        return Ok(());
    }
    let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
    let fixture = document.fixture_mut();
    operation.action = Some(match action {
        FlowVcsAction::InsertWidget { index, item } => {
            fixture.widgets.push(item);
            operation.cursor.current = fixture.widgets.len() - 1;
            operation.cursor.target = index;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            operation.action = Some(FlowVcsAction::RemoveWidgetAt { index });
            return Ok(());
        }
        action @ (FlowVcsAction::RemoveWidget { .. } | FlowVcsAction::RemoveWidgetAt { .. }) => {
            let item = fixture.widgets.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.rollback_owner = Some(action);
            operation.cursor.mutated = true;
            FlowVcsAction::InsertWidget { index: operation.cursor.origin, item }
        }
        FlowVcsAction::MoveWidget { id, .. } => FlowVcsAction::MoveWidget { id, index: operation.cursor.origin },
        FlowVcsAction::PatchWidget { id, mut item } => {
            std::mem::swap(&mut fixture.widgets[operation.cursor.origin], &mut item);
            operation.cursor.mutated = true;
            FlowVcsAction::PatchWidget { id, item }
        }
        FlowVcsAction::InsertSynapse { index, item } => {
            fixture.synapses.push(item);
            operation.cursor.current = fixture.synapses.len() - 1;
            operation.cursor.target = index;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            operation.action = Some(FlowVcsAction::RemoveSynapseAt { index });
            return Ok(());
        }
        action @ (FlowVcsAction::RemoveSynapse { .. } | FlowVcsAction::RemoveSynapseAt { .. }) => {
            let item = fixture.synapses.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.rollback_owner = Some(action);
            operation.cursor.mutated = true;
            FlowVcsAction::InsertSynapse { index: operation.cursor.origin, item }
        }
        FlowVcsAction::MoveSynapse { id, .. } => FlowVcsAction::MoveSynapse { id, index: operation.cursor.origin },
        FlowVcsAction::PatchSynapse { id, mut item } => {
            std::mem::swap(&mut fixture.synapses[operation.cursor.origin], &mut item);
            operation.cursor.mutated = true;
            FlowVcsAction::PatchSynapse { id, item }
        }
        FlowVcsAction::SetLayout(entry) => {
            operation.layout_update = Some(match entry.layout {
                Some(layout) => fixture.layout.begin_set(entry.id, layout),
                None => fixture.layout.begin_remove(entry.id),
            });
            return Ok(());
        }
        FlowVcsAction::LayoutRoot(layout) => {
            operation.cursor.mutated = true;
            FlowVcsAction::LayoutRoot(std::mem::replace(&mut fixture.layout, layout))
        }
        FlowVcsAction::ActivateDocument { index } => {
            if document.versions.get(index).is_none() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            let previous = document.active;
            document.active = index;
            operation.cursor.mutated = true;
            FlowVcsAction::ActivateDocument { index: previous }
        }
        FlowVcsAction::Checkpoint => FlowVcsAction::Checkpoint,
        action => {
            operation.action = Some(action);
            return Err(FlowVcsFault::InvalidMutation);
        }
    });
    operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
    Ok(())
}

fn flow_vcs_step_document_replacement(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let action = operation.action.as_mut().ok_or(FlowVcsFault::InvalidMutation)?;
    let source = match action {
        FlowVcsAction::ReplaceDocument(source) => source,
        _ => return Err(FlowVcsFault::InvalidMutation),
    };
    match operation.cursor.phase {
        FlowVcsCursorPhase::ReserveReplacement => {
            if document.versions.is_full() {
                return Err(FlowVcsFault::Full);
            }
            let empty = FlowFixture { schema: String::new(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 0.0 }, widgets: Vec::new(), synapses: Vec::new(), layout: OrderedMap::new() };
            document.versions.push(empty).map_err(|_| FlowVcsFault::Full)?;
            operation.cursor.target = document.versions.len() - 1;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceSchema;
        }
        FlowVcsCursorPhase::ReplaceSchema => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").schema = std::mem::take(&mut source.schema);
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraX;
        }
        FlowVcsCursorPhase::ReplaceCameraX => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.x = source.camera.x;
            source.camera.x = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraY;
        }
        FlowVcsCursorPhase::ReplaceCameraY => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.y = source.camera.y;
            source.camera.y = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraZoom;
        }
        FlowVcsCursorPhase::ReplaceCameraZoom => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.zoom = source.camera.zoom;
            source.camera.zoom = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceWidgets;
        }
        FlowVcsCursorPhase::ReplaceWidgets => {
            if let Some(widget) = source.widgets.pop() {
                document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").widgets.push(widget);
            } else {
                operation.cursor.scan = 0;
                operation.cursor.phase = FlowVcsCursorPhase::ReverseWidgets;
            }
        }
        FlowVcsCursorPhase::ReverseWidgets => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            if operation.cursor.scan < target.widgets.len() / 2 {
                let opposite = target.widgets.len() - operation.cursor.scan - 1;
                target.widgets.swap(operation.cursor.scan, opposite);
                operation.cursor.scan += 1;
            } else {
                operation.cursor.phase = FlowVcsCursorPhase::ReplaceSynapses;
            }
        }
        FlowVcsCursorPhase::ReplaceSynapses => {
            if let Some(synapse) = source.synapses.pop() {
                document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").synapses.push(synapse);
            } else {
                operation.cursor.scan = 0;
                operation.cursor.phase = FlowVcsCursorPhase::ReverseSynapses;
            }
        }
        FlowVcsCursorPhase::ReverseSynapses => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            if operation.cursor.scan < target.synapses.len() / 2 {
                let opposite = target.synapses.len() - operation.cursor.scan - 1;
                target.synapses.swap(operation.cursor.scan, opposite);
                operation.cursor.scan += 1;
            } else {
                operation.cursor.phase = FlowVcsCursorPhase::ReplaceLayout;
            }
        }
        FlowVcsCursorPhase::ReplaceLayout => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            std::mem::swap(&mut target.layout, &mut source.layout);
            let previous = document.active;
            document.active = operation.cursor.target;
            operation.action = Some(FlowVcsAction::ActivateDocument { index: previous });
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    Ok(())
}

fn flow_vcs_step_rollback(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation) -> Result<bool, FlowVcsFault> {
    if !operation.cursor.mutated {
        return Ok(true);
    }
    let cursor = &mut operation.cursor;
    match cursor.kind {
        FlowVcsCursorKind::InsertWidget => {
            let fixture = document.fixture_mut();
            if cursor.current + 1 < fixture.widgets.len() {
                fixture.widgets.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            let item = fixture.widgets.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.action = Some(FlowVcsAction::InsertWidget { index: cursor.target, item });
        }
        FlowVcsCursorKind::InsertSynapse => {
            let fixture = document.fixture_mut();
            if cursor.current + 1 < fixture.synapses.len() {
                fixture.synapses.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            let item = fixture.synapses.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.action = Some(FlowVcsAction::InsertSynapse { index: cursor.target, item });
        }
        FlowVcsCursorKind::RemoveWidget => {
            let fixture = document.fixture_mut();
            if matches!(operation.action.as_ref(), Some(FlowVcsAction::InsertWidget { .. })) {
                let FlowVcsAction::InsertWidget { item, .. } = operation.action.take().expect("retained widget inverse") else { unreachable!() };
                fixture.widgets.push(item);
                cursor.current = fixture.widgets.len() - 1;
                operation.action = Some(FlowVcsAction::Checkpoint);
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                fixture.widgets.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
        }
        FlowVcsCursorKind::RemoveSynapse => {
            let fixture = document.fixture_mut();
            if matches!(operation.action.as_ref(), Some(FlowVcsAction::InsertSynapse { .. })) {
                let FlowVcsAction::InsertSynapse { item, .. } = operation.action.take().expect("retained synapse inverse") else { unreachable!() };
                fixture.synapses.push(item);
                cursor.current = fixture.synapses.len() - 1;
                operation.action = Some(FlowVcsAction::Checkpoint);
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                fixture.synapses.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
        }
        FlowVcsCursorKind::MoveWidget => {
            let fixture = document.fixture_mut();
            if cursor.current < cursor.origin {
                fixture.widgets.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                fixture.widgets.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::MoveWidget { id, .. } = action {
                operation.action = Some(FlowVcsAction::MoveWidget { id, index: cursor.target });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::MoveSynapse => {
            let fixture = document.fixture_mut();
            if cursor.current < cursor.origin {
                fixture.synapses.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                fixture.synapses.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::MoveSynapse { id, .. } = action {
                operation.action = Some(FlowVcsAction::MoveSynapse { id, index: cursor.target });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::PatchWidget => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::PatchWidget { id, mut item } = action {
                std::mem::swap(&mut document.fixture_mut().widgets[cursor.origin], &mut item);
                operation.action = Some(FlowVcsAction::PatchWidget { id, item });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::PatchSynapse => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::PatchSynapse { id, mut item } = action {
                std::mem::swap(&mut document.fixture_mut().synapses[cursor.origin], &mut item);
                operation.action = Some(FlowVcsAction::PatchSynapse { id, item });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::Layout => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            match action {
                FlowVcsAction::LayoutRoot(layout) => {
                    operation.action = Some(FlowVcsAction::LayoutRoot(std::mem::replace(&mut document.fixture_mut().layout, layout)));
                }
                _ => return Err(FlowVcsFault::InvalidMutation),
            }
        }
        FlowVcsCursorKind::ReplaceDocument => {
            if document.active == cursor.target {
                if let Some(FlowVcsAction::ActivateDocument { index }) = operation.action.take() {
                    document.active = index;
                    operation.action = Some(FlowVcsAction::ActivateDocument { index: cursor.target });
                    return Ok(false);
                }
            }
            if cursor.target + 1 != document.versions.len() {
                return Err(FlowVcsFault::ClosePending);
            }
            let candidate = document.versions.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.retirement.push(FlowOwner::Fixture(candidate));
        }
        FlowVcsCursorKind::None => {}
    }
    if let Some(action) = operation.rollback_owner.take() {
        operation.action = Some(action);
    }
    cursor.mutated = false;
    Ok(true)
}
//#endregion 🌊️RetainedActionCursor

fn flow_vcs_retire_action(action: FlowVcsAction, retirement: &mut FlowRetirement) {
    match action {
        FlowVcsAction::ReplaceDocument(fixture) => retirement.push(FlowOwner::Fixture(fixture)),
        FlowVcsAction::LayoutRoot(layout) => retirement.push(FlowOwner::Layouts(layout)),
        FlowVcsAction::SetLayout(entry) => retirement.text(entry.id),
        FlowVcsAction::InsertWidget { item, .. } => retirement.push(FlowOwner::Widget(item)),
        FlowVcsAction::PatchWidget { id, item } => {
            retirement.text(id);
            retirement.push(FlowOwner::Widget(item));
        }
        FlowVcsAction::InsertSynapse { item, .. } => retirement.push(FlowOwner::Specs(vec![item])),
        FlowVcsAction::PatchSynapse { id, item } => {
            retirement.text(id);
            retirement.push(FlowOwner::Specs(vec![item]));
        }
        FlowVcsAction::RemoveWidget { id } | FlowVcsAction::MoveWidget { id, .. }
        | FlowVcsAction::RemoveSynapse { id } | FlowVcsAction::MoveSynapse { id, .. } => retirement.text(id),
        _ => {}
    }
}

fn flow_vcs_fixture_census(fixture: &FlowFixture) -> FlowVcsCensus {
    let items = 1usize.saturating_add(fixture.widgets.len()).saturating_add(fixture.synapses.len()).saturating_add(fixture.layout.len());
    let bytes = size_of::<FlowFixture>()
        .saturating_add(fixture.schema.len())
        .saturating_add(fixture.widgets.len().saturating_mul(size_of::<Widget>()))
        .saturating_add(fixture.synapses.len().saturating_mul(size_of::<SynapseSpec>()))
        .saturating_add(fixture.layout.len().saturating_mul(size_of::<(String, WidgetLayout)>()));
    FlowVcsCensus { items, bytes, depth: FLOW_VCS_MAX_DEPTH }
}

fn flow_vcs_widget_census(widget: &Widget) -> FlowVcsCensus {
    let depth = if matches!(widget, Widget::Cluster { .. }) { FLOW_VCS_MAX_DEPTH } else { 1 };
    let payload_bytes = match widget {
        Widget::Neuron { neuron_kind, input_ports, output_ports, .. } => {
            neuron_kind.len().saturating_add(input_ports.len().saturating_mul(size_of::<String>())).saturating_add(output_ports.len().saturating_mul(size_of::<String>()))
        }
        Widget::InputNote { text, .. } => text.len(),
        Widget::InputImage { src, .. } => src.len(),
        Widget::Variable { name, schema, .. } => name.len().saturating_add(schema.len()),
        Widget::OutputPreview { expanded, .. } => expanded.len().saturating_mul(size_of::<String>()),
        Widget::OutputAction { action, .. } => action.len(),
        Widget::OutputExport { format, .. } => format.len(),
        Widget::Cluster { name, tree, flow, .. } => name
            .len()
            .saturating_add(tree.neurons.len().saturating_mul(size_of::<Neuron>()))
            .saturating_add(tree.synapses.len().saturating_mul(size_of::<Synapse>()))
            .saturating_add(flow.nodes.len().saturating_mul(size_of::<(String, FlowNodeGui)>()))
            .saturating_add(flow.previews.len().saturating_mul(size_of::<FlowPreviewGui>())),
        Widget::InputSlider { label, .. } => label.len(),
    };
    FlowVcsCensus { items: 1, bytes: size_of::<Widget>().saturating_add(widget_id_for(widget).len()).saturating_add(payload_bytes), depth }
}

fn flow_vcs_synapse_census(synapse: &SynapseSpec) -> FlowVcsCensus {
    FlowVcsCensus::leaf(synapse.id.len() + synapse.from.len() + synapse.to.len() + synapse.from_port.len() + synapse.to_port.len())
}

fn flow_vcs_fixture_scalar_digest(fixture: &FlowFixture) -> u64 {
    14_695_981_039_346_656_037
        ^ u64::try_from(fixture.schema.len()).unwrap_or(u64::MAX).rotate_left(3)
        ^ u64::try_from(fixture.widgets.len()).unwrap_or(u64::MAX).rotate_left(11)
        ^ u64::try_from(fixture.synapses.len()).unwrap_or(u64::MAX).rotate_left(23)
        ^ u64::try_from(fixture.layout.len()).unwrap_or(u64::MAX).rotate_left(37)
        ^ fixture.camera.x.to_bits()
        ^ fixture.camera.y.to_bits().rotate_left(17)
        ^ fixture.camera.zoom.to_bits().rotate_left(31)
}

//#endregion 🌊️RetainedVcs

// #region 🔖️FormsBridge
pub mod forms_bridge {
    use super::{FlowFixture, Widget};
    use crate::playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookSpec, PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};

    fn humanize_widget_label(id: &str) -> String {
        let mut words = Vec::new();
        let mut current = String::new();
        for ch in id.chars() {
            if ch == '_' || ch == '-' || ch == ' ' {
                if !current.is_empty() {
                    words.push(current.clone());
                    current.clear();
                }
                continue;
            }
            if ch.is_uppercase() && !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            current.push(if ch.is_uppercase() { ch } else { ch.to_ascii_uppercase() });
        }
        if !current.is_empty() {
            words.push(current);
        }
        if words.is_empty() {
            return id.to_string();
        }
        words.join(" ")
    }

    /// 🔀️ Schema aliases treated as a single-choice question when generating a playbook block (anything else is free text).
    enum SchemaQuestionFamily {
        Choice,
    }

    impl SchemaQuestionFamily {
        fn parse(schema: &str) -> Option<Self> {
            match schema.trim().to_ascii_lowercase().as_str() {
                "enum" | "single" | "select" | "choice" => Some(Self::Choice),
                _ => None,
            }
        }
    }

    fn variable_question_kind(schema: &str) -> &'static str {
        match SchemaQuestionFamily::parse(schema) {
            Some(SchemaQuestionFamily::Choice) => "single",
            None => "text",
        }
    }

    fn widget_to_playbook_block(widget: &Widget) -> Option<PlaybookBlock> {
        match widget {
            Widget::InputSlider { id, label, value, min, max, step } => Some(PlaybookBlock {
                id: id.clone(),
                label: label.clone(),
                kind: "slider".into(),
                description: None,
                required: None,
                placeholder: None,
                default: Some(crate::os_dsl::DslValue::float(*value)),
                min: Some(*min),
                max: Some(*max),
                step: Some(*step),
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::InputNote { id, text, .. } => Some(PlaybookBlock {
                id: id.clone(),
                label: humanize_widget_label(id),
                kind: "note".into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: Some(text.clone()),
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::InputImage { id, src, .. } => Some(PlaybookBlock {
                id: id.clone(),
                label: humanize_widget_label(id),
                kind: "image".into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: Some(src.clone()),
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::Variable { id, name, schema, .. } => {
                let kind = variable_question_kind(schema);
                let options = if kind == "single" { Some(vec![PlaybookBlockOption { value: schema.clone(), label: humanize_widget_label(schema) }]) } else { None };
                Some(PlaybookBlock {
                    id: id.clone(),
                    label: humanize_widget_label(name),
                    kind: kind.into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(crate::os_dsl::DslValue::String(name.clone())),
                    min: None,
                    max: None,
                    step: None,
                    unit: None,
                    text: None,
                    options,
                    fields: None,
                    schema: Some(schema.clone()),
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: None,
                })
            }
            _ => None,
        }
    }

    pub fn flow_fixture_to_form_spec(fixture: &FlowFixture) -> PlaybookSpec {
        let blocks: Vec<PlaybookBlock> = fixture.widgets.iter().filter_map(widget_to_playbook_block).collect();
        PlaybookSpec { schema: PLAYBOOK_DOCUMENT_SCHEMA.into(), id: "flow-generate".into(), version: "1".into(), title: Some("Generate".into()), steps: vec![PlaybookStep { id: "inputs".into(), title: "Inputs".into(), description: None, blocks }] }
    }

    /// 🏷️ Widget "kind" tags recognized when patching a single generation value into a raw fixture-JSON widget.
    enum WidgetPatchKind {
        InputSlider,
        InputNote,
        InputImage,
        Variable,
    }

    impl WidgetPatchKind {
        fn parse(kind: &str) -> Option<Self> {
            match kind {
                "inputSlider" => Some(Self::InputSlider),
                "inputNote" => Some(Self::InputNote),
                "inputImage" => Some(Self::InputImage),
                "variable" => Some(Self::Variable),
                _ => None,
            }
        }
    }

    pub fn apply_generation_values_to_fixture(fixture_json: &str, values: &crate::os_pack::json::Object) -> String {
        let Ok(mut root) = crate::os_pack::json::parse(fixture_json) else {
            return fixture_json.to_string();
        };
        let Some(widgets) = root.get_mut("widgets").and_then(|entry| entry.as_array_mut()) else {
            return fixture_json.to_string();
        };
        for widget in widgets.iter_mut() {
            let Some(id) = widget.get("id").and_then(|entry| entry.as_str()) else {
                continue;
            };
            let Some(value) = values.get(id) else {
                continue;
            };
            let kind = widget.get("kind").and_then(|entry| entry.as_str()).unwrap_or_default();
            let patch_kind = WidgetPatchKind::parse(kind);
            let Some(object) = widget.as_object_mut() else {
                continue;
            };
            match patch_kind {
                Some(WidgetPatchKind::InputSlider) => {
                    if let Some(number) = value.as_f64() {
                        object.insert("value", crate::os_pack::json::Value::Number(number.into()));
                    }
                }
                Some(WidgetPatchKind::InputNote) => {
                    if let Some(text) = value.as_str() {
                        object.insert("text", crate::os_pack::json::Value::String(text.to_string()));
                    }
                }
                Some(WidgetPatchKind::InputImage) => {
                    if let Some(src) = value.as_str() {
                        object.insert("src", crate::os_pack::json::Value::String(src.to_string()));
                    }
                }
                Some(WidgetPatchKind::Variable) => {
                    if let Some(text) = value.as_str() {
                        object.insert("name", crate::os_pack::json::Value::String(text.to_string()));
                    }
                }
                None => {}
            }
        }
        crate::os_pack::json::to_string(&root)
    }
}
// #endregion 🔖️FormsBridge

#[cfg(test)]
#[path = "🧪️tests/🔬️flow-vcs/🦀️.rs"]
mod flow_vcs_tests;
// #endregion 🔖️ArtifactVcs
