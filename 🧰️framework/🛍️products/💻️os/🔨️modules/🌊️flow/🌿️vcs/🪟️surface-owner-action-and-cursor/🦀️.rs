//! 🪟️ Surface owner, action and cursor of the Flow VCS: the surface owner, the action and stage enums and the history cursor with its phase and kind. See [`super`].

use super::*;

//#region 🌊️RetainedVcs
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
    pub fn from_host_snapshot(surface: u64, host: u64, generation: u64, host_snapshot: &FlowHostSnapshot) -> Self {
        let widget_slots = host_snapshot.widgets.len();
        Self { surface, host, generation, document: 1, widgets: widget_slots, synapses: host_snapshot.synapses.len(), previews: widget_slots, expanded: widget_slots, layout: host_snapshot.layout.len(), history: 1, edit: 1, conflict: 1, control: 1, output: 1 }
    }

    pub(super) fn close_one(&mut self) -> bool {
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

#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum FlowVcsAction {
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
    LayoutEntry(FlowLayoutEntry),
    ReplaceDocument(FlowHostSnapshot),
    Undo,
    Redo,
    Checkpoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowVcsStage {
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
pub enum FlowVcsCursorPhase {
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
pub enum FlowVcsCursorKind {
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
pub struct FlowVcsCursor {
    pub(super) phase: FlowVcsCursorPhase,
    pub(super) kind: FlowVcsCursorKind,
    pub scan: usize,
    pub(super) origin: usize,
    pub(super) current: usize,
    pub(super) target: usize,
    pub(super) history_mode: u8,
    pub history_loaded: bool,
    pub redo_retired: usize,
    pub history_transferred: bool,
    pub surface_transferred: bool,
    pub visibility_published: bool,
    pub(super) prior_generation: u64,
    pub(super) prior_digest: u64,
    pub owns_edit: bool,
    pub mutated: bool,
}

impl FlowVcsCursor {
    pub(super) fn new(action: &FlowVcsAction) -> Self {
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
            FlowVcsAction::LayoutEntry(_) => (FlowVcsCursorPhase::Mutate, FlowVcsCursorKind::Layout, 0),
        };
        let history_mode = match action {
            FlowVcsAction::Undo => 1,
            FlowVcsAction::Redo => 2,
            FlowVcsAction::Checkpoint | FlowVcsAction::ReplaceDocument(_) => 3,
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
//#endregion 🌊️RetainedVcs
