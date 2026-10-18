//! 🔤️ Vocabulary, census and digest of the Flow VCS: the declared limits, credits, resource fingerprint, census, source, authority, handle, grant, close phase, fault, page and poll types plus the fixture, widget and synapse census fns and the host snapshot scalar digest. See [`super`].

use super::*;

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
pub struct FlowVcsCensus {
    pub items: usize,
    pub bytes: usize,
    pub depth: usize,
}

impl FlowVcsCensus {
    pub(super) fn leaf(bytes: usize) -> Self {
        Self { items: 1, bytes, depth: 1 }
    }

    pub(super) fn include(&mut self, child: Self) {
        self.items = self.items.saturating_add(child.items);
        self.bytes = self.bytes.saturating_add(child.bytes);
        self.depth = self.depth.max(child.depth.saturating_add(1));
    }
}

#[derive(Debug)]
pub struct FlowVcsSource<T> {
    pub value: Option<T>,
}

impl<T> FlowVcsSource<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }

    pub fn retained(&self) -> bool {
        self.value.is_some()
    }

    pub(super) fn get(&self) -> Result<&T, FlowVcsFault> {
        self.value.as_ref().ok_or(FlowVcsFault::SourceExhausted)
    }

    pub(super) fn take(&mut self) -> T {
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
    pub retained: RetainedCloneGrant,
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
    pub(super) fn permits_work(self) -> bool {
        !self.interrupted && self.fuel > 0 && self.items > 0 && self.now_milliseconds < self.deadline_milliseconds && self.deadline_milliseconds.saturating_sub(self.now_milliseconds) <= FLOW_VCS_DEADLINE_MILLISECONDS
    }
}

/// 🪜️ The rungs of the retained VCS close ladder, in the order
/// [`FlowRetainedVcs::close_retired_step`] takes them.
///
/// `Backing` is the only rung that moves PAYLOAD: it drains the retirement frontier an owner at a
/// time. Every other rung retires one retained item and pushes its payload onto that frontier, so a
/// driver that anchors a turn on the non-`Backing` phase spends one turn per retained item whatever
/// that item weighs (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowVcsClosePhase {
    Operations,
    Backing,
    RetiredSurface,
    RetiredAction,
    History,
    DocumentSurface,
    DocumentVersion,
    Document,
    Complete,
}

impl FlowVcsClosePhase {
    pub fn is_backing(self) -> bool {
        matches!(self, Self::Backing)
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

pub(super) fn flow_vcs_fixture_census(host_snapshot: &FlowHostSnapshot) -> FlowVcsCensus {
    let items = 1usize.saturating_add(host_snapshot.widgets.len()).saturating_add(host_snapshot.synapses.len()).saturating_add(host_snapshot.layout.len());
    let bytes = size_of::<FlowHostSnapshot>()
        .saturating_add(host_snapshot.schema.len())
        .saturating_add(host_snapshot.widgets.len().saturating_mul(size_of::<Widget>()))
        .saturating_add(host_snapshot.synapses.len().saturating_mul(size_of::<SynapseSpec>()))
        .saturating_add(host_snapshot.layout.len().saturating_mul(size_of::<(String, WidgetLayout)>()));
    FlowVcsCensus { items, bytes, depth: FLOW_VCS_MAX_DEPTH }
}

pub(super) fn flow_vcs_widget_census(widget: &Widget) -> FlowVcsCensus {
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

pub(super) fn flow_vcs_synapse_census(synapse: &SynapseSpec) -> FlowVcsCensus {
    FlowVcsCensus::leaf(synapse.id.len() + synapse.from.len() + synapse.to.len() + synapse.from_port.len() + synapse.to_port.len())
}

pub fn flow_vcs_host_snapshot_scalar_digest(host_snapshot: &FlowHostSnapshot) -> u64 {
    14_695_981_039_346_656_037
        ^ u64::try_from(host_snapshot.schema.len()).unwrap_or(u64::MAX).rotate_left(3)
        ^ u64::try_from(host_snapshot.widgets.len()).unwrap_or(u64::MAX).rotate_left(11)
        ^ u64::try_from(host_snapshot.synapses.len()).unwrap_or(u64::MAX).rotate_left(23)
        ^ u64::try_from(host_snapshot.layout.len()).unwrap_or(u64::MAX).rotate_left(37)
        ^ host_snapshot.camera.x.to_bits()
        ^ host_snapshot.camera.y.to_bits().rotate_left(17)
        ^ host_snapshot.camera.zoom.to_bits().rotate_left(31)
}
//#endregion 🌊️RetainedVcs
