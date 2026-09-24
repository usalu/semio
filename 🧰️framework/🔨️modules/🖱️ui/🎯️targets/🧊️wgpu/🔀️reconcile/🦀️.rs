// #region reconcile
//! 🔁️ Keyed single-pass reconciliation: applies an incoming declarative `UiNode` tree to a retained
//! `UiTree`, matching children by key, diffing matched nodes, and marking the minimal dirty flags.
//! `Stack`/`Section`/`Field` recurse into their own literal `UiNode` children; `Select`/`Tree` recurse
//! into *synthesized* retained children (see `🔖️CompositeExpansion` below) built from their
//! non-`UiNode` payload (`items`/`sections`) since there is no dedicated `UiNode` variant for "one
//! Select option row" or "one Tree row" to reuse verbatim — the remaining 14 variants have no nested
//! `UiNode`/composite payload at all, so a diffed leaf is already their complete, correct treatment.
//! KNOWN GAP (wiring request, not fixable from this region alone — see `tree::WidgetState`'s own doc
//! comment): `Select`'s synthesized option rows are always built, unconditionally, regardless of
//! open/closed — `tree::WidgetState` is currently a zero-field marker with nowhere to record "is this
//! Select open", so this region can't gate the *rows' existence* on it. `NodeFlags::HAS_POPUP` is set
//! on the `Select` node itself (whenever it has ≥1 item) so a later events/paint milestone can find
//! the always-ready rows once `WidgetState` grows an `open`-like field to gate *showing*/hit-testing
//! them — no further reconcile-side change should be needed at that point.

use crate::wgpu::Label;
use crate::wgpu::UiTreeActionPlacement;
use dsl::DslValue;
#[cfg(any(test, feature = "testkit"))]
use std::borrow::Cow;
use std::collections::HashMap;
#[cfg(any(test, feature = "testkit"))]
use std::collections::HashSet;

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::layout::ActionDescriptor;
#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::component::ui::ui_control_to_node;
use crate::wgpu::component::ui::{
    SurfaceKind, UiButtonNode, UiComponentSceneNode, UiControlNode, UiDropOverlaySpec, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiMenuRef, UiNode, UiNumberStepperNode, UiPresence,
    UiProgressNode, UiRingNode, UiSectionNode, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiState, UiStatus, UiTextNode, UiToggleNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UiTreeWindow, UiTreeWindowRowExtent,
};
use crate::wgpu::tree::{Node, NodeFlags, NodeKey, UiDocumentPageRejection, UiDocumentTree, UiDocumentTreeFault, UiTree, WidgetSpec};
use crate::wgpu::{IconName, UiIntentAddress, UiIntentBindings};
use ui_contract::{UI_DOCUMENT_NODES, UiDocumentNodePage, UiNodeId, UiNodeRecord};

//#region 📄️DocumentPageReconcile
impl UiDocumentTree {
    #[expect(clippy::result_large_err, reason = "Fixed-capacity node admission returns the exact rejected node or page without allocating on refusal.")]
    pub fn try_push_page(&mut self, page: UiDocumentNodePage, expected_index: usize) -> Result<(), UiDocumentPageRejection> {
        let generation = page.generation();
        let revision = page.revision();
        let index = page.index();
        let fault = if generation != self.generation {
            Some(UiDocumentTreeFault::Generation)
        } else if revision != self.revision {
            Some(UiDocumentTreeFault::Revision)
        } else if index != expected_index {
            Some(UiDocumentTreeFault::PageOrder)
        } else if self.nodes.get(&page.record().id).is_some() {
            Some(UiDocumentTreeFault::DuplicateNode)
        } else {
            None
        };
        let record = page.into_record();
        if let Some(fault) = fault {
            return Err(UiDocumentPageRejection { fault, generation, revision, index, record });
        }
        self.nodes.try_insert(record).map(|_| ()).map_err(|record| UiDocumentPageRejection { fault: UiDocumentTreeFault::NodeCapacity, generation, revision, index, record })
    }
}
//#endregion 📄️DocumentPageReconcile

//#region 🌳️DocumentTreeReconcile
/// 🌳️ The deepest published document the reconcile walks in one pass — one frame per nesting level
/// of the DFS plan stack, sized to the same ceiling `mounted_layout`'s own walk carries
/// (`LAYOUT_DEPTH_CREDITS`), because a tree the layout cannot walk is not worth mounting.
pub const UI_DOCUMENT_RECONCILE_DEPTH: usize = 64;

/// 🚨️ Why a published document could not be projected into the paintable arena. Never a panic and
/// never a dropped document: the surface keeps its previous arena content and the caller records the
/// fault, exactly the rule `ui_contract`'s `🗺️surface.rs` states for an unresolvable surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentReconcileFault {
    /// 🕳️ A record the document's own `children` list names is not in its node table.
    MissingRecord,
    /// 🪜️ The plan stack outgrew [`UI_DOCUMENT_RECONCILE_DEPTH`] or `UI_DOCUMENT_NODES`.
    Depth,
    /// 🌱️ A planned child's parent never mounted — structurally impossible on a pre-order plan, so
    /// this can only mean the arena was mutated under the cursor.
    Detached,
    /// 🎬️ The retained node exhausted its component-mount identity counter.
    ComponentGeneration,
    /// 🪙 The presented document could not acquire an independently credited candidate record.
    DocumentCredits,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentReconcileStep {
    Pending,
    Complete,
    Fault(UiDocumentReconcileFault),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiRetiredComponentScene {
    pub document_id: UiNodeId,
    pub host_id: String,
    pub window_id: String,
    pub window_generation: u64,
    pub component_generation: u64,
    pub node: NodeId,
    pub key: NodeKey,
    pub kind: SurfaceKind,
    pub surface_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiComponentSceneWitness<'a> {
    pub document_id: UiNodeId,
    pub host_id: &'a str,
    pub window_id: &'a str,
    pub window_generation: u64,
    pub component_generation: u64,
    pub key: &'a NodeKey,
    pub kind: SurfaceKind,
    pub surface_id: &'a str,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum UiDocumentReconcilePhase {
    /// 🔽️ Drains the tree's synthesized composite rows (an open `Select`'s option rows) one node per
    /// step BEFORE anything relinks the arena. `Unlink`/`Mount` below rebuild the sibling chains from
    /// the published record set alone, which would orphan every synthesized child and leak its arena
    /// slot on each republish; `paint::sync_interactive_state_node_step` re-mints whatever is still
    /// open on the next frame, so nothing is lost and the working set stays bounded
    /// (ticket 26/09/17 packet W15a).
    #[default]
    Compose,
    Adopt,
    Plan,
    Unlink,
    Mount,
    Retire,
    Publish,
    Complete,
    Fault,
}

#[derive(Clone, Copy, Debug)]
struct PlannedNode {
    id: UiNodeId,
    parent: Option<usize>,
    node: Option<NodeId>,
}

/// 🌳️ One surface's resumable document→arena reconcile.
///
/// Five phases, each advancing by exactly ONE unit per `step` so the whole pass fits inside the
/// interactive turn the frame build already prices (`📓️wgpu-blank-paint-2026-09-12.md` §3.2): plan
/// one record, unlink one mounted node, mount one record, retire one abandoned node, publish. A pass
/// that runs out of budget resumes at the same unit on the next opportunity, and a pass that faults
/// leaves the previous arena content standing.
///
/// Identity is the `UiNodeId`, not tree position: the producer mints one per `(parent, key)` and
/// reuses it for as long as that identity survives, so a node that merely moved or changed props
/// keeps its arena slot — and with it its `WidgetState` (a focused editor's live buffer, a scroll
/// offset, an open `Select`) and its interaction flags. This is the Rust twin of the React
/// `Interpreter`'s `uiChildReactKeys` identity rule.
#[derive(Default)]
pub struct UiDocumentReconcileCursor {
    phase: UiDocumentReconcilePhase,
    fault: Option<UiDocumentReconcileFault>,
    generation: u64,
    plan: Vec<PlannedNode>,
    planned_ids: Vec<UiNodeId>,
    stack: Vec<(UiNodeId, Option<usize>)>,
    sweep: usize,
    mount: usize,
}

impl UiDocumentReconcileCursor {
    /// 🧹️ Retires one reconcile index before releasing its fixed-lifetime window owner.
    pub(crate) fn close_step(&mut self) -> bool {
        if self.plan.pop().is_some() || self.planned_ids.pop().is_some() || self.stack.pop().is_some() {
            return false;
        }
        if self.plan.capacity() > 0 {
            self.plan = Vec::new();
            return false;
        }
        if self.planned_ids.capacity() > 0 {
            self.planned_ids = Vec::new();
            return false;
        }
        if self.stack.capacity() > 0 {
            self.stack = Vec::new();
            return false;
        }
        self.phase = UiDocumentReconcilePhase::Complete;
        self.fault = None;
        true
    }

    /// 🔁️ Rearms the cursor for `generation`, discarding any half-finished pass for an older one.
    pub fn rearm(&mut self, generation: u64) {
        if self.generation == generation && !matches!(self.phase, UiDocumentReconcilePhase::Complete | UiDocumentReconcilePhase::Fault) {
            return;
        }
        self.phase = UiDocumentReconcilePhase::Compose;
        self.fault = None;
        self.generation = generation;
        self.plan.clear();
        self.planned_ids.clear();
        self.stack.clear();
        self.sweep = 0;
        self.mount = 0;
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn terminal_is_complete(&self) -> bool {
        matches!(self.phase, UiDocumentReconcilePhase::Complete)
    }

    pub fn fault(&self) -> Option<UiDocumentReconcileFault> {
        self.fault
    }

    fn refuse(&mut self, fault: UiDocumentReconcileFault) -> UiDocumentReconcileStep {
        self.phase = UiDocumentReconcilePhase::Fault;
        self.fault = Some(fault);
        UiDocumentReconcileStep::Fault(fault)
    }

    fn remember_planned(&mut self, id: UiNodeId) {
        if let Err(index) = self.planned_ids.binary_search(&id) {
            self.planned_ids.insert(index, id);
        }
    }
}

/// 👶️ Whether a record's own subtree is CONSUMED by its projection instead of becoming arena
/// children of its own.
///
/// Exactly one component owns its subtree outright: `Surface` — "a surface's children are NOT
/// rendered chrome; a renderer draws `props` and nothing else" (`ui_contract`'s `🏗️builder.rs`
/// `SurfaceBuilder`). Its children are the out-of-doc payload lane carriers, which
/// [`surface_scene_node`] folds back into the scene itself.
///
/// `Tree` deliberately is NOT one, even though `UiTreeNode` carries its sections and items inline:
/// this engine's `paint::sync_interactive_state_node_step` resolves every section and item row to a
/// REAL arena child keyed by that row's id (`TreeApplyPrepare`/`TreeApplyScan`) and faults outright
/// when the row is missing. `reconcile::children_of`'s `Tree` arm synthesizes exactly those rows for
/// the declarative path; the document path does not have to, because the producer already published
/// them as `TreeSection`/`TreeItem` records — so they mount as themselves, projected to the same
/// keyed `Stack` rows that arm builds (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-document-reconcile-2026-09-12.md`).
fn record_consumes_subtree(record: &UiNodeRecord) -> bool {
    matches!(record.component, ui_contract::Component::Surface(_))
}

/// 📐️ A record's [`ui_contract::SpaceToken`] as the legacy `UiStackNode` string. It stamps the
/// token's OWN name (`"md"`, `"lg"`, …), not the old three-bucket `none`/`tight`/`loose` collapse
/// that rendered `Xs` and `Sm` identically and shrank `Md` fourfold: a record-mounted node's real
/// geometry now travels on `Node::layout_spec` (see `flex`'s header), and this channel stays
/// lossless so the reconcile diff still sees every token change.
fn space_token(token: ui_contract::SpaceToken) -> Option<String> {
    Some(crate::wgpu::layout::space_token_name(token).to_string())
}

/// 📐️ A record's [`ui_contract::EdgeSpace`] as one legacy string — every side, space-separated in
/// CSS shorthand order, so an asymmetric padding no longer collapses onto one sampled side.
fn edge_token(edge: ui_contract::EdgeSpace) -> Option<String> {
    let name = crate::wgpu::layout::space_token_name;
    Some(match edge {
        ui_contract::EdgeSpace::All(token) => name(token).to_string(),
        ui_contract::EdgeSpace::Symmetric { vertical, horizontal } => format!("{} {}", name(vertical), name(horizontal)),
        ui_contract::EdgeSpace::Each { top, right, bottom, left } => format!("{} {} {} {}", name(top), name(right), name(bottom), name(left)),
    })
}

/// 📐️ The `(direction, gap, padding)` triple `UiStackNode` carries, read off the record's own
/// `LayoutSpec` — the contract keeps geometry on the record, never on the component props. These
/// three strings are the LEGACY declarative dialect's vocabulary and the reconcile diff's baseline;
/// the layout engine itself reads the full `LayoutSpec` off `Node::layout_spec`, so `Grid`/`Scroll`/
/// `Overlay`/`Absolute` no longer lose their columns, axes, insets or sizing here.
fn stack_metrics(layout: &ui_contract::LayoutSpec) -> (String, Option<String>, Option<String>) {
    match layout {
        ui_contract::LayoutSpec::Stack(stack) => {
            let direction = if matches!(stack.axis, ui_contract::Axis::Horizontal) { "horizontal" } else { "vertical" };
            (direction.to_string(), space_token(stack.gap), edge_token(stack.padding))
        }
        ui_contract::LayoutSpec::Grid(grid) => ("grid".to_string(), space_token(grid.row_gap), edge_token(grid.padding)),
        ui_contract::LayoutSpec::Scroll(scroll) => ("scroll".to_string(), None, edge_token(scroll.padding)),
        ui_contract::LayoutSpec::Overlay(overlay) => ("overlay".to_string(), None, edge_token(overlay.inset)),
        ui_contract::LayoutSpec::Absolute(_) => ("absolute".to_string(), None, None),
        ui_contract::LayoutSpec::Leaf(_) => ("vertical".to_string(), None, None),
    }
}

/// 🖱️ The input-routing bits a `LayoutSpec` implies, which `events` already understands: a `Scroll`
/// container owns a scrollable, clipping viewport (`nearest_scrollable_ancestor` walks `SCROLLABLE`,
/// `hit_test` refuses a point outside a `CLIPS_CHILDREN` box), and an `Overlay` is hit-tested ahead
/// of its in-flow siblings — the same priority `EventRouter::open_overlay` grants a Select popup.
struct LayoutRoutingFlags {
    scrollable: bool,
    clips: bool,
    overlay: bool,
}

fn layout_routing_flags(layout: &ui_contract::LayoutSpec) -> LayoutRoutingFlags {
    match layout {
        ui_contract::LayoutSpec::Scroll(scroll) => LayoutRoutingFlags { scrollable: !matches!(scroll.axes, ui_contract::ScrollAxes::None), clips: true, overlay: false },
        ui_contract::LayoutSpec::Overlay(_) => LayoutRoutingFlags { scrollable: false, clips: false, overlay: true },
        _ => LayoutRoutingFlags { scrollable: false, clips: false, overlay: false },
    }
}

fn contract_label(label: &ui_contract::Label) -> Label {
    Label::data(label.0.as_str())
}

fn optional_contract_label(label: Option<&ui_contract::Label>) -> Option<Label> {
    label.map(contract_label)
}

fn icon_name(icon: &ui_contract::UiText) -> IconName {
    IconName::from_str(icon.as_str()).unwrap_or(IconName::CircleDot)
}

fn ui_value_to_dsl(value: &ui_contract::UiValue) -> Option<DslValue> {
    serde_json::to_value(value).ok().map(DslValue::from)
}

fn menu_ref(record: &UiNodeRecord) -> Option<UiMenuRef> {
    record.menu.as_ref().map(|menu| UiMenuRef { id: menu.id.as_str().to_string(), args: menu.args.as_ref().and_then(ui_value_to_dsl) })
}

/// 🧭️ The record's document-carried presence: `disabled` and `transition` are document state (the
/// contract's own `👥️presence.rs` header), while hover/selection/peer marks travel on the separate
/// coalesced presence channel and are stamped onto the arena node by `events`, never by this
/// projection — which is exactly why `sync_interactive_state_node_step` runs before every paint.
fn record_presence(record: &UiNodeRecord) -> UiPresence {
    let state = if record.disabled {
        UiState::Disabled
    } else {
        match record.transition {
            Some(ui_contract::TransitionHint::Introducing) => UiState::Introducing,
            Some(ui_contract::TransitionHint::Celebrating) => UiState::Celebrating,
            None => UiState::Normal,
        }
    };
    let status = match record.activity {
        ui_contract::Activity::Waiting => UiStatus::Waiting,
        ui_contract::Activity::Loading => UiStatus::Loading,
        ui_contract::Activity::Idle => UiStatus::Idle,
        ui_contract::Activity::Finished => UiStatus::Finished,
    };
    UiPresence { state, status, ..UiPresence::default() }
}

/// 🎬️ The record's binding for `trigger`, as the `ActionDescriptor` the retained spec carries for
/// paint and for the immediate-mode widget path. The published node normally dispatches through the
/// whole [`record_intent_bindings`] action id; synthesized children such as Select option rows have
/// no record of their own and dispatch this descriptor instead. It therefore keeps the binding's
/// authored scope rather than substituting the live document controller.
fn record_action(record: &UiNodeRecord, trigger: ui_contract::Trigger, _controller: &str) -> Option<ActionDescriptor> {
    record.bindings.iter().find(|binding| binding.trigger == trigger).map(|binding| ActionDescriptor {
        controller_id: binding.action.scope.as_str().to_string(),
        action: if binding.action.version == 1 { binding.action.name.as_str().to_string() } else { format!("{}@{}", binding.action.name.as_str(), binding.action.version) },
        args: binding.args.as_ref().and_then(ui_value_to_dsl),
    })
}

fn record_action_or_inert(record: &UiNodeRecord, trigger: ui_contract::Trigger, controller: &str) -> ActionDescriptor {
    record_action(record, trigger, controller).unwrap_or_else(|| ActionDescriptor { controller_id: controller.to_string(), action: String::new(), args: None })
}

/// 🎬️ The record's whole dispatch contract for the arena node it mounts as: its address on this
/// surface at this revision, plus every `(trigger, ActionId)` it binds. The twin of React's
/// `UiDocumentStore::buildIntent` inputs — `surface`/`revision` come from the store there and from
/// the published document header here, `node`/`node_key` from the record itself.
fn record_intent_bindings(record: &UiNodeRecord, surface: &str, revision: u64) -> UiIntentBindings {
    UiIntentBindings {
        address: UiIntentAddress { surface: surface.to_string(), revision, node: record.id.0, node_key: record.key.as_str().to_string() },
        bindings: record.bindings.iter().map(|binding| (binding.trigger, binding.action.clone())).collect(),
    }
}

/// 🔌️ The plugin that owns `surface`. A [`ui_contract::SurfaceId`] is a dotted address whose first
/// segment IS the owning plugin (`"note.play.navigator"` → `"note"`, that type's own docstring), and
/// `ExtensionProps` carries no plugin id of its own — so the slot's host is the publisher's host, and
/// nothing else in the record could name it. Previously hardcoded to `""`, which would have composed
/// empty plugin ids into every id built from it.
fn surface_plugin_id(surface: &str) -> String {
    surface.split('.').next().unwrap_or(surface).to_string()
}

fn drop_overlay(spec: Option<&ui_contract::DropOverlaySpec>) -> Option<UiDropOverlaySpec> {
    spec.map(|spec| UiDropOverlaySpec { title: contract_label(&spec.title), hint: contract_label(&spec.hint), accept: spec.accept.as_ref().map(|accept| accept.as_str().to_string()) })
}

fn input_kind(kind: ui_contract::InputKind) -> String {
    match kind {
        ui_contract::InputKind::Text => "text",
        ui_contract::InputKind::LongText => "longText",
        ui_contract::InputKind::Number => "number",
        ui_contract::InputKind::Date => "date",
        ui_contract::InputKind::Color => "color",
        ui_contract::InputKind::File => "file",
    }
    .to_string()
}

/// 🗺️ The contract's own `SurfaceKind` in this target's spelling. Both enums name the same fifteen
/// kinds and, since the `virtualFileSystem` -> `virtual-file-system` rename reached this target too,
/// the same fifteen wire tags — `ui_contract::SurfaceKind::as_wire`/`SurfaceKind::as_str` agree
/// verbatim and `🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format` pins that agreement.
fn surface_kind(kind: ui_contract::SurfaceKind) -> SurfaceKind {
    match kind {
        ui_contract::SurfaceKind::Canvas2d => SurfaceKind::Canvas2d,
        ui_contract::SurfaceKind::World3d => SurfaceKind::World3d,
        ui_contract::SurfaceKind::NodeGraph => SurfaceKind::NodeGraph,
        ui_contract::SurfaceKind::TextEditor => SurfaceKind::TextEditor,
        ui_contract::SurfaceKind::Table => SurfaceKind::Table,
        ui_contract::SurfaceKind::Paint2d => SurfaceKind::Paint2d,
        ui_contract::SurfaceKind::VirtualFileSystem => SurfaceKind::VirtualFileSystem,
        ui_contract::SurfaceKind::TiledMap => SurfaceKind::TiledMap,
        ui_contract::SurfaceKind::Board2d => SurfaceKind::Board2d,
        ui_contract::SurfaceKind::IconRender => SurfaceKind::IconRender,
        ui_contract::SurfaceKind::InkCanvas => SurfaceKind::InkCanvas,
        ui_contract::SurfaceKind::GraphTimeline => SurfaceKind::GraphTimeline,
        ui_contract::SurfaceKind::BlockList => SurfaceKind::BlockList,
        ui_contract::SurfaceKind::DiffView => SurfaceKind::DiffView,
        ui_contract::SurfaceKind::EventFeed => SurfaceKind::EventFeed,
    }
}

/// 🚚️ Depth-first concatenation of every packed `Text` leaf under `root` — the exact inverse of the
/// plugin host's `paged_text_carrier`, and the Rust twin of the React Interpreter's
/// `surfaceSceneLaneText`.
fn lane_payload(document: &UiDocumentTree, root: &UiNodeRecord) -> String {
    let mut payload = String::new();
    let mut stack: Vec<UiNodeId> = root.children.iter().rev().copied().collect();
    let mut visited = 0usize;
    while let Some(id) = stack.pop() {
        visited += 1;
        if visited > UI_DOCUMENT_NODES {
            break;
        }
        let Some(record) = document.record(id) else { continue };
        if let ui_contract::Component::Text(text) = &record.component {
            payload.push_str(&text.packed_payload());
        }
        for child in record.children.iter().rev() {
            stack.push(*child);
        }
    }
    payload
}

/// 🚚️ Reattaches a surface's out-of-doc payload lanes to the spine its `doc.bytes` decoded to.
/// `lane_name` resolves a carrier key to its declared lane name (`World3dSceneLane`,
/// `Canvas2dSceneLane`, `Board2dSceneLane`, `Paint2dSceneLane`). A lane the document declares but whose carrier has not fully arrived is left
/// at its spine value rather than guessed — a truncated `meshes` payload would parse to an EMPTY
/// scene, which is strictly worse than the previous frame's.
fn merge_scene_lanes<T: ui_scene::SceneDoc>(document: &UiDocumentTree, record: &UiNodeRecord, scene: &mut T, declared: &[ui_scene::SceneLaneRef], lane_name: impl Fn(&str) -> Option<&'static str>) {
    if declared.is_empty() {
        return;
    }
    for child_id in record.children.iter() {
        let Some(child) = document.record(*child_id) else { continue };
        let Some(name) = lane_name(child.key.as_str()) else { continue };
        let Some(reference) = declared.iter().find(|entry| entry.lane == name) else { continue };
        let payload = lane_payload(document, child);
        if payload.len() as u32 != reference.bytes {
            continue;
        }
        let _ = scene.merge_lane(child.key.as_str(), payload);
    }
}

/// 🪪️ Addresses one mounted component independently of its owning document and sibling hosts.
pub fn component_scene_host_id(window_generation: u64, component_generation: u64) -> String {
    format!("scene.{window_generation}.{component_generation}")
}

/// 🗺️ Projects one `Component::Surface` record onto the `UiComponentSceneNode` every engine-surface
/// host already reads — the Rust twin of the React Interpreter's `surfacePropsToComponentSceneNode`,
/// including its identity rule (`surfaceHostIdentityV1`): `surface_id` is the OWNING DOCUMENT's
/// surface (the window the host is mounted into, i.e. the plugin host's own `surface_contexts` key),
/// `controller_id` scopes the pane-independent per-app registries, and `pane_id` carries the
/// program's own authored surface id — the one identity `SurfaceProps` really did drop.
///
/// A schema this target cannot decode yields a scene node with NO payload rather than `None`: the
/// host paints its own empty viewport and the document keeps every sibling, which is the contract's
/// "an unrecognised `doc_schema` must never reject the surrounding patch" rule.
fn surface_scene_node(document: &UiDocumentTree, record: &UiNodeRecord, props: &ui_contract::SurfaceProps, surface: &str, controller: &str) -> UiComponentSceneNode {
    let mut node = UiComponentSceneNode {
        host_id: String::new(),
        surface_id: surface.to_string(),
        controller_id: controller.to_string(),
        component_kind: surface_kind(props.kind),
        pane_id: Some(record.key.as_str().to_string()),
        binding_id: None,
        presence: record_presence(record),
        menu: menu_ref(record),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
    };
    if ui_contract::parse_doc_schema(props.doc_schema.as_str()).is_err() || props.doc.bytes.is_empty() {
        return node;
    }
    match props.kind {
        ui_contract::SurfaceKind::World3d => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::World3dScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::World3dSceneLane::from_body_key(key).map(ui_scene::World3dSceneLane::name));
                scene.lanes = declared;
                node.world_3d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::NodeGraph => node.node_graph = ui_scene::decode::<ui_scene::NodeGraphScene>(props).ok(),
        ui_contract::SurfaceKind::Canvas2d => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::Canvas2dScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::Canvas2dSceneLane::from_body_key(key).map(ui_scene::Canvas2dSceneLane::name));
                scene.lanes = declared;
                node.canvas_2d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::TextEditor => node.text_editor = ui_scene::decode::<ui_scene::TextEditorScene>(props).ok(),
        ui_contract::SurfaceKind::Table => node.table = ui_scene::decode::<ui_scene::TableScene>(props).ok(),
        ui_contract::SurfaceKind::Paint2d => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::Paint2dScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::Paint2dSceneLane::from_body_key(key).map(ui_scene::Paint2dSceneLane::name));
                scene.lanes = declared;
                node.paint_2d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::VirtualFileSystem => node.virtual_file_system = ui_scene::decode::<ui_scene::VirtualFileSystemScene>(props).ok(),
        ui_contract::SurfaceKind::TiledMap => node.tiled_map = ui_scene::decode::<ui_scene::TiledMapScene>(props).ok(),
        ui_contract::SurfaceKind::Board2d => {
            if let Ok(mut scene) = ui_scene::decode::<ui_scene::Board2dScene>(props) {
                let declared = std::mem::take(&mut scene.lanes);
                merge_scene_lanes(document, record, &mut scene, &declared, |key| ui_scene::Board2dSceneLane::from_body_key(key).map(ui_scene::Board2dSceneLane::name));
                scene.lanes = declared;
                node.board2d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::IconRender => node.icon_render = ui_scene::decode::<ui_scene::IconRenderScene>(props).ok(),
        ui_contract::SurfaceKind::InkCanvas => node.ink_canvas = ui_scene::decode::<ui_scene::InkCanvasScene>(props).ok(),
        ui_contract::SurfaceKind::GraphTimeline => node.graph_timeline = ui_scene::decode::<ui_scene::GraphTimelineScene>(props).ok(),
        ui_contract::SurfaceKind::BlockList => node.block_list = ui_scene::decode::<ui_scene::BlockListScene>(props).ok(),
        ui_contract::SurfaceKind::DiffView => node.diff_view = ui_scene::decode::<ui_scene::DiffViewScene>(props).ok(),
        ui_contract::SurfaceKind::EventFeed => node.event_feed = ui_scene::decode::<ui_scene::EventFeedScene>(props).ok(),
    }
    node
}

fn drag_data(map: Option<&ui_contract::UiFixedMap<ui_contract::UiText>>) -> Option<HashMap<String, String>> {
    map.map(|entries| entries.iter().map(|(key, value)| (key.as_str().to_string(), value.as_str().to_string())).collect())
}

fn data_attributes(map: Option<&ui_contract::UiFixedMap<ui_contract::UiText>>) -> Option<HashMap<String, String>> {
    drag_data(map)
}

fn row_action(action: &ui_contract::RowAction, controller: &str) -> UiTreeItemAction {
    UiTreeItemAction {
        icon_id: icon_name(&action.icon),
        label: optional_contract_label(action.label.as_ref()),
        action: ActionDescriptor { controller_id: controller.to_string(), action: action.action.action.name.as_str().to_string(), args: action.action.args.as_ref().and_then(ui_value_to_dsl) },
        placement: Some(match action.placement {
            ui_contract::RowActionPlacement::Row => UiTreeActionPlacement::Row,
            ui_contract::RowActionPlacement::Menu => UiTreeActionPlacement::Menu,
        }),
    }
}

/// 🪟️ Projects the contract's [`ui_contract::TreeWindow`] onto the legacy wgpu tree node's own
/// mirror. The two structs are deliberately separate types: the legacy `UiNode` wire shape is rooted
/// in `dsl`'s `ToValue`/`FromValue`, the contract's in `protocol::value`.
fn tree_window(window: Option<&ui_contract::TreeWindow>) -> Option<UiTreeWindow> {
    window.map(|window| UiTreeWindow {
        total: window.total,
        offset: window.offset,
        row_extent: match window.row_extent {
            ui_contract::TreeWindowRowExtent::Standard => UiTreeWindowRowExtent::Standard,
            ui_contract::TreeWindowRowExtent::CompactText => UiTreeWindowRowExtent::CompactText,
            ui_contract::TreeWindowRowExtent::CompactSmallControl => UiTreeWindowRowExtent::CompactSmallControl,
            ui_contract::TreeWindowRowExtent::CompactControl => UiTreeWindowRowExtent::CompactControl,
        },
    })
}

/// 🌳️ Assembles one `Component::TreeItem` record and its whole subtree into the inline
/// `UiTreeItemNode` the retained `Tree` spec carries. A child that projects to a control becomes the
/// row's `control`; every other child recurses as a nested item.
fn tree_item(document: &UiDocumentTree, record: &UiNodeRecord, surface: &str, controller: &str, depth: usize) -> UiTreeItemNode {
    let ui_contract::Component::TreeItem(props) = &record.component else {
        return UiTreeItemNode {
            window: None,
            granularity: None,
            id: record.key.as_str().to_string(),
            label: Label::data(record.key.as_str()),
            description: None,
            icon_id: None,
            presence: record_presence(record),
            default_open: None,
            action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            dimmed: None,
            menu: menu_ref(record),
        };
    };
    let mut items: Vec<UiTreeItemNode> = Vec::new();
    let mut control: Option<UiControlNode> = None;
    if depth < UI_DOCUMENT_RECONCILE_DEPTH {
        for child_id in record.children.iter() {
            let Some(child) = document.record(*child_id) else { continue };
            match tree_item_control(child, controller) {
                Some(node) if control.is_none() => control = Some(node),
                _ => items.push(tree_item(document, child, surface, controller, depth + 1)),
            }
        }
    }
    let actions: Vec<UiTreeItemAction> = props.row_actions.iter().map(|action| row_action(action, controller)).collect();
    UiTreeItemNode {
        window: tree_window(props.window.as_ref()),
        granularity: props.granularity.as_ref().map(|value| value.as_str().to_string()),
        id: record.key.as_str().to_string(),
        label: contract_label(&props.label),
        description: props.description.as_ref().map(|value| value.as_str().to_string()),
        icon_id: props.icon.as_ref().map(icon_name),
        presence: record_presence(record),
        default_open: props.default_open,
        action: record_action(record, ui_contract::Trigger::Activate, controller),
        actions: if actions.is_empty() { None } else { Some(actions) },
        draggable: props.draggable,
        drag_data: drag_data(props.drag_data.as_ref()),
        items: if items.is_empty() { None } else { Some(items) },
        control,
        dimmed: props.dimmed,
        menu: menu_ref(record),
    }
}

/// 🎛️ The control a tree row embeds, when its child record is one of the nine control components.
fn tree_item_control(record: &UiNodeRecord, controller: &str) -> Option<UiControlNode> {
    Some(match record.component {
        ui_contract::Component::Input(_) => UiControlNode::Input(match input_node(record, controller) {
            UiNode::Input(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Select(_) => UiControlNode::Select(match select_node(record, controller) {
            UiNode::Select(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Toggle(_) => UiControlNode::Toggle(match toggle_node(record, controller) {
            UiNode::Toggle(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Button(_) => UiControlNode::Button(match button_node(record, controller) {
            UiNode::Button(node) => node,
            _ => return None,
        }),
        ui_contract::Component::KeyValueList(_) => UiControlNode::KeyValue(match key_value_node(record) {
            UiNode::KeyValue(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Slider(_) => UiControlNode::Slider(match slider_node(record, controller) {
            UiNode::Slider(node) => node,
            _ => return None,
        }),
        ui_contract::Component::NumberStepper(_) => UiControlNode::NumberStepper(match number_stepper_node(record, controller) {
            UiNode::NumberStepper(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Ring(_) => UiControlNode::Ring(match ring_node(record, controller) {
            UiNode::Ring(node) => node,
            _ => return None,
        }),
        ui_contract::Component::IconSelect(_) => UiControlNode::IconSelect(match icon_select_node(record, controller) {
            UiNode::IconSelect(node) => node,
            _ => return None,
        }),
        _ => return None,
    })
}

/// ⌨️ Which trigger an `Input`'s value commits through — `InputProps`' own doc names the pair
/// (`Trigger::Change`/`Trigger::Commit`), and `commit == "blur"` is what picks between them, exactly
/// as React's `InputView` picks `commitOnBlur ? "commit" : "change"`. Resolving it HERE keeps the
/// retained node's single `on_change` field the one descriptor `events`' commit authority
/// dispatches, whichever trigger the guest actually bound.
fn input_commit_action(record: &UiNodeRecord, props: &ui_contract::InputProps, controller: &str) -> ActionDescriptor {
    let trigger = if props.commit.as_ref().is_some_and(|value| value.as_str() == "blur") { ui_contract::Trigger::Commit } else { ui_contract::Trigger::Change };
    record_action_or_inert(record, trigger, controller)
}

fn input_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Input(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Input(UiInputNode {
        id: record.key.as_str().to_string(),
        input_kind: input_kind(props.kind),
        value: props.value.as_str().to_string(),
        placeholder: optional_contract_label(props.placeholder.as_ref()),
        accessibility_label: optional_contract_label(record.accessibility.label.as_ref()),
        commit: props.commit.as_ref().map(|value| value.as_str().to_string()),
        min: props.min,
        max: props.max,
        step: props.step,
        accept: props.accept.as_ref().map(|value| value.as_str().to_string()),
        on_change: input_commit_action(record, props, controller),
        // ⏎️⎋️🔁️ The three moments React's `SearchInput` binds BESIDE `onChange`. They are read
        // per-trigger (not resolved into the single `on_change` slot the way `input_commit_action`
        // resolves Change/Commit) because React fires them at the SAME time as `onChange`, never
        // instead of it: the window search line feeds the guest's autocomplete on every keystroke
        // and runs the verb on Enter (`🖥️ui/🎯️targets/⚛️react/🟦️.tsx`'s `Search`).
        on_submit: record_action(record, ui_contract::Trigger::Submit, controller),
        on_abort: record_action(record, ui_contract::Trigger::Abort, controller),
        on_repeat_last: record_action(record, ui_contract::Trigger::RepeatLast, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

fn select_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Select(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Select(UiSelectNode {
        id: record.key.as_str().to_string(),
        value: props.value.as_str().to_string(),
        items: props.items.iter().map(|item| UiSelectItem { value: item.value.as_str().to_string(), label: contract_label(&item.label) }).collect(),
        placeholder: optional_contract_label(props.placeholder.as_ref()),
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

fn toggle_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Toggle(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    let mut presence = record_presence(record);
    presence.selected = props.on;
    UiNode::Toggle(UiToggleNode {
        appearance: props.appearance,
        id: record.key.as_str().to_string(),
        icon_id: icon_name(&props.icon),
        text: optional_contract_label(props.text.as_ref()),
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence,
        menu: menu_ref(record),
    })
}

fn button_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Button(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Button(UiButtonNode {
        id: Some(record.key.as_str().to_string()),
        icon_id: icon_name(&props.icon),
        label: contract_label(&props.label),
        action: record_action_or_inert(record, ui_contract::Trigger::Activate, controller),
        style: None,
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

fn key_value_node(record: &UiNodeRecord) -> UiNode {
    let ui_contract::Component::KeyValueList(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::KeyValue(UiKeyValueNode { entries: props.entries.iter().map(|entry| UiKeyValueEntry { label: contract_label(&entry.label), value: entry.value.as_str().to_string() }).collect(), presence: record_presence(record), menu: menu_ref(record) })
}

fn slider_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Slider(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Slider(UiSliderNode {
        id: record.key.as_str().to_string(),
        value: props.value,
        min: props.min,
        max: props.max,
        step: props.step,
        unit: props.unit.as_ref().map(|value| value.as_str().to_string()),
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

fn number_stepper_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::NumberStepper(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::NumberStepper(UiNumberStepperNode {
        id: record.key.as_str().to_string(),
        value: props.value,
        step: props.step,
        uniform: props.uniform,
        on_absolute: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        on_delta: record_action_or_inert(record, ui_contract::Trigger::Delta, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

fn ring_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Ring(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Ring(UiRingNode {
        id: record.key.as_str().to_string(),
        orb_id: props.orb_id.as_str().to_string(),
        t: props.t,
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

fn icon_select_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::IconSelect(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::IconSelect(UiIconSelectNode {
        id: record.key.as_str().to_string(),
        value: props.value.as_str().to_string(),
        uniform: props.uniform,
        classifier_kind: props.classifier_kind.as_str().to_string(),
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

/// 📶️ `Component::Progress` → the retained bar; the record key is its identity, exactly like every
/// other leaf control.
fn progress_node(record: &UiNodeRecord, props: &ui_contract::ProgressProps, presence: UiPresence, menu: Option<UiMenuRef>) -> UiNode {
    UiNode::Progress(UiProgressNode { id: record.key.as_str().to_string(), completed: props.completed, total: props.total, value_text: contract_label(&props.value_text), presence, menu })
}

/// 🧩️ Projects ONE published record onto the retained `UiNode` this target paints — the missing half
/// of the retained-document pipeline (`📓️wgpu-blank-paint-2026-09-12.md` §5).
///
/// Children are deliberately left EMPTY on every container variant: the arena's own
/// parent/first-child/sibling links are the tree, and `mounted_layout`/`paint`/`scene_slots` all walk
/// those links, never a spec's inline `children`. The two components that do own their subtree
/// (`Surface`, `Tree` — see [`record_consumes_subtree`]) are the exception and carry it inline.
pub fn ui_node_from_record(document: &UiDocumentTree, record: &UiNodeRecord, surface: &str, controller: &str) -> UiNode {
    let presence = record_presence(record);
    let menu = menu_ref(record);
    match &record.component {
        ui_contract::Component::Container(props) => match props.role {
            ui_contract::ContainerRole::Section => UiNode::Section(UiSectionNode { id: record.key.as_str().to_string(), label: optional_contract_label(props.label.as_ref()), default_open: props.default_open, presence, menu, children: Vec::new() }),
            ui_contract::ContainerRole::Group => {
                UiNode::Group(UiGroupNode { id: record.key.as_str().to_string(), label: optional_contract_label(props.label.as_ref()).unwrap_or_else(|| Label::data("")), default_open: props.default_open, presence, menu, children: Vec::new() })
            }
            ui_contract::ContainerRole::Field => UiNode::Field(UiFieldNode {
                id: record.key.as_str().to_string(),
                label: optional_contract_label(props.label.as_ref()).unwrap_or_else(|| Label::data("")),
                description: props.description.as_ref().map(|value| value.as_str().to_string()),
                required: props.required,
                error: props.error.as_ref().map(|value| value.as_str().to_string()),
                // 🍃️ The field's real child is an ARENA child (the record's `children[0]`); this
                // placeholder only satisfies `UiFieldNode`'s non-optional shape and is never painted
                // — `paint_node_step`'s `Field` arm paints the label and nothing of `child`.
                child: Box::new(UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })),
                presence,
                menu,
            }),
            ui_contract::ContainerRole::Plain | ui_contract::ContainerRole::Form | ui_contract::ContainerRole::Toolbar => {
                let (direction, gap, padding) = stack_metrics(&record.layout);
                UiNode::Stack(UiStackNode {
                    direction,
                    gap,
                    padding,
                    id: Some(record.key.as_str().to_string()),
                    presence,
                    activate: record_action(record, ui_contract::Trigger::Activate, controller),
                    drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
                    drop_overlay: drop_overlay(props.drop_overlay.as_ref()),
                    menu,
                    children: Vec::new(),
                })
            }
        },
        ui_contract::Component::Text(props) => UiNode::Text(UiTextNode { value: contract_label(&props.value), emphasize: props.emphasize, data_attributes: data_attributes(props.data_attributes.as_ref()), presence, menu }),
        ui_contract::Component::Separator(_) => UiNode::Separator(UiSeparatorNode { presence, menu }),
        ui_contract::Component::Button(_) => button_node(record, controller),
        ui_contract::Component::Input(_) => input_node(record, controller),
        ui_contract::Component::Select(_) => select_node(record, controller),
        ui_contract::Component::Toggle(_) => toggle_node(record, controller),
        ui_contract::Component::KeyValueList(_) => key_value_node(record),
        ui_contract::Component::Slider(_) => slider_node(record, controller),
        ui_contract::Component::NumberStepper(_) => number_stepper_node(record, controller),
        ui_contract::Component::Ring(_) => ring_node(record, controller),
        ui_contract::Component::IconSelect(_) => icon_select_node(record, controller),
        ui_contract::Component::Progress(props) => progress_node(record, props, presence, menu),
        ui_contract::Component::Image(props) => UiNode::Image(UiImageNode { id: record.key.as_str().to_string(), src: props.src.as_str().to_string(), alt: optional_contract_label(props.alt.as_ref()), presence, menu }),
        ui_contract::Component::Tree(props) => {
            let sections = record
                .children
                .iter()
                .filter_map(|child_id| document.record(*child_id))
                .map(|child| match &child.component {
                    ui_contract::Component::TreeSection(section) => UiTreeSectionNode {
                        window: tree_window(section.window.as_ref()),
                        id: child.key.as_str().to_string(),
                        label: optional_contract_label(section.label.as_ref()),
                        default_open: section.default_open,
                        presence: record_presence(child),
                        items: child.children.iter().filter_map(|item_id| document.record(*item_id)).map(|item| tree_item(document, item, surface, controller, 1)).collect(),
                    },
                    _ => UiTreeSectionNode { window: None, id: child.key.as_str().to_string(), label: None, default_open: None, presence: record_presence(child), items: vec![tree_item(document, child, surface, controller, 1)] },
                })
                .collect();
            UiNode::Tree(UiTreeNode { presentation: props.presentation, sections, presence, drop_action: record_action(record, ui_contract::Trigger::Drop, controller), menu, interaction_domain: props.interaction_domain.as_ref().map(|value| value.as_str().to_string()) })
        }
        // 🌳️ A tree's section and item records mount as the keyed `Stack` ROWS this engine's
        // interactive sync resolves by id — the document-path twin of `children_of`'s `Tree` arm
        // (`tree_section_row`/`tree_item_row`). Their content is painted from the owning `Tree`'s own
        // inline spec; these rows carry identity, layout and drag/drop state. 🪟️ `TreeSectionProps.
        // window`/`TreeItemProps.window` are therefore NOT read here: the spacer pitch belongs to the
        // painted spec (`UiTreeSectionNode.window`/`UiTreeItemNode.window`, stamped in the `Tree` arm
        // above), not to the identity row, which has no extent of its own.
        ui_contract::Component::TreeSection(_) | ui_contract::Component::TreeItem(_) => UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some(record.key.as_str().to_string()),
            presence,
            activate: record_action(record, ui_contract::Trigger::Activate, controller),
            drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
            drop_overlay: None,
            menu,
            children: Vec::new(),
        }),
        ui_contract::Component::Table(_) => UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some(record.key.as_str().to_string()),
            presence,
            activate: None,
            drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
            drop_overlay: None,
            menu,
            children: Vec::new(),
        }),
        ui_contract::Component::TableRow(props) => UiNode::Button(UiButtonNode {
            id: Some(record.key.as_str().to_string()),
            icon_id: props.row_actions.get(0).map_or(IconName::ChevronRight, |action| icon_name(&action.icon)),
            label: Label::data(props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>().join(" · ")),
            action: record_action(record, ui_contract::Trigger::Activate, controller).or_else(|| props.row_actions.get(0).map(|action| row_action(action, controller).action)).unwrap_or_else(|| ActionDescriptor { controller_id: controller.to_string(), action: String::new(), args: None }),
            style: None,
            presence,
            menu,
        }),
        ui_contract::Component::Surface(props) => UiNode::ComponentScene(surface_scene_node(document, record, props, surface, controller)),
        ui_contract::Component::Extension(props) => UiNode::ExternalSlot(crate::wgpu::component::ui::UiExternalSlotNode {
            plugin_id: surface_plugin_id(surface),
            app_id: controller.to_string(),
            body_key: props.extension.as_str().to_string(),
            params_json: serde_json::to_string(&props.props).unwrap_or_else(|_| "null".to_string()),
            presence,
            menu,
        }),
    }
}

impl UiTree {
    /// 🌳️ Advances this surface's document→arena reconcile by exactly one unit. See
    /// [`UiDocumentReconcileCursor`] for the phase ladder and the identity rule.
    ///
    /// 📐️ The record's own `LayoutSpec` is the React-parity geometry channel
    /// (`layoutSpecStyle`'s input); a change to it alone — a grid gaining a
    /// column, a stack flipping to `justify: SpaceBetween` — moves every
    /// descendant's box without touching the projected `UiNode` at all, so it
    /// is re-stamped unconditionally. `Publish` below already re-dirties the
    /// whole surface's layout, so no per-node bubble is needed here.
    ///
    /// 🎬️ Always re-stamped, never diffed: the address carries the document's
    /// CURRENT revision, which moves on every published patch even when the
    /// node's own spec did not — and a node left at an old revision would
    /// start dropping its own live gestures as stale.
    ///
    /// 🪟️ Only ever RAISED here: `events::EventRouter::open_overlay`/`close_overlay`
    /// own this bit at runtime for popups the document knows nothing about, so a
    /// re-mount must not drop an open Select's hit-test priority.
    pub fn step_document_reconcile(&mut self, cursor: &mut UiDocumentReconcileCursor, surface: &str, controller: &str) -> UiDocumentReconcileStep {
        let mut component_mount_generation = self.document_bindings().iter().filter_map(|(_, node)| self.node(*node).map(Node::component_generation)).max().unwrap_or(0);
        self.step_document_reconcile_preserving(cursor, surface, controller, None, 0, None, true, &mut component_mount_generation, &mut |_| true)
    }

    /// 🔒️ Reconciles while preserving the synthesized rows owned by the Select whose option has
    /// pointer capture. The owner record itself keeps its arena identity, and its rows are relinked
    /// after the document links so the matching release still reaches the exact pressed row.
    pub(crate) fn step_document_reconcile_preserving(
        &mut self,
        cursor: &mut UiDocumentReconcileCursor,
        surface: &str,
        controller: &str,
        preserved_composite_owner: Option<NodeId>,
        window_generation: u64,
        presented: Option<&UiTree>,
        preserve_candidate_scene_identity: bool,
        component_mount_generation: &mut u64,
        retire_scene: &mut impl FnMut(UiRetiredComponentScene) -> bool,
    ) -> UiDocumentReconcileStep {
        match cursor.phase {
            UiDocumentReconcilePhase::Complete => return UiDocumentReconcileStep::Complete,
            UiDocumentReconcilePhase::Fault => return UiDocumentReconcileStep::Fault(cursor.fault.unwrap_or(UiDocumentReconcileFault::MissingRecord)),
            _ => {}
        }
        let Some(root_id) = self.document().map(UiDocumentTree::root_id) else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
        match cursor.phase {
            UiDocumentReconcilePhase::Compose => {
                if !self.retire_composite_row_except_step(preserved_composite_owner) {
                    return UiDocumentReconcileStep::Pending;
                }
                cursor.phase = UiDocumentReconcilePhase::Adopt;
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Adopt => {
                // 🧹️ A root this ledger never minted can only come from the testkit `apply_tree`
                // path; it is removed whole so the document owns the arena outright.
                let adopted = self.root.is_some_and(|root| self.document_bindings().iter().any(|(_, node)| *node == root));
                if let Some(root) = self.root.filter(|_| !adopted) {
                    self.remove(root);
                }
                self.root = None;
                cursor.stack.push((root_id, None));
                cursor.phase = UiDocumentReconcilePhase::Plan;
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Plan => {
                let Some((id, parent)) = cursor.stack.pop() else {
                    cursor.sweep = 0;
                    cursor.phase = UiDocumentReconcilePhase::Unlink;
                    return UiDocumentReconcileStep::Pending;
                };
                if cursor.plan.len() >= UI_DOCUMENT_NODES || cursor.stack.len() > UI_DOCUMENT_NODES {
                    return cursor.refuse(UiDocumentReconcileFault::Depth);
                }
                let children: Vec<UiNodeId> = {
                    let Some(document) = self.document() else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    let Some(record) = document.record(id) else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    if record_consumes_subtree(record) { Vec::new() } else { record.children.iter().rev().copied().collect() }
                };
                let index = cursor.plan.len();
                cursor.plan.push(PlannedNode { id, parent, node: None });
                cursor.remember_planned(id);
                for child in children {
                    cursor.stack.push((child, Some(index)));
                }
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Unlink => {
                let Some((_, node)) = self.document_bindings().get(cursor.sweep).copied() else {
                    cursor.mount = 0;
                    cursor.phase = UiDocumentReconcilePhase::Mount;
                    return UiDocumentReconcileStep::Pending;
                };
                self.clear_links(node);
                cursor.sweep += 1;
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Mount => {
                let Some(planned) = cursor.plan.get(cursor.mount).copied() else {
                    cursor.sweep = 0;
                    cursor.phase = UiDocumentReconcilePhase::Retire;
                    return UiDocumentReconcileStep::Pending;
                };
                let (key, mut spec, layout_spec, intent) = {
                    let Some(document) = self.document() else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    let Some(record) = document.record(planned.id) else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    (NodeKey::Explicit(record.key.as_str().to_string()), WidgetSpec(ui_node_from_record(document, record, surface, controller)), record.layout.clone(), record_intent_bindings(record, surface, document.revision().0))
                };
                let routing = layout_routing_flags(&layout_spec);
                let presented_component = presented.and_then(|tree| {
                    let node = tree.document_node(planned.id)?;
                    let retained = tree.node(node)?;
                    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                    Some((node, scene.host_id.clone(), retained.component_generation(), retained.key.clone(), scene.component_kind, scene.surface_id.clone()))
                });
                let presented_scene = match (&spec.0, presented_component.as_ref()) {
                    (UiNode::ComponentScene(next), Some((_, host_id, generation, presented_key, kind, surface_id)))
                        if presented_key == &key && *kind == next.component_kind && surface_id == &next.surface_id =>
                    {
                        Some((host_id.clone(), *generation))
                    }
                    _ => None,
                };
                let node = match self.document_node(planned.id).filter(|node| self.contains(*node)) {
                    Some(node) => {
                        let Some((old_scene, candidate_scene)) = self.node(node).map(|existing| {
                            let old_scene = match &existing.spec.0 {
                                UiNode::ComponentScene(scene) => Some(scene),
                                _ => None,
                            };
                            let same_logical_scene = matches!(
                                (old_scene, &spec.0),
                                (Some(old_scene), UiNode::ComponentScene(next_scene))
                                    if existing.key == key && old_scene.component_kind == next_scene.component_kind && old_scene.surface_id == next_scene.surface_id
                            );
                            let old_scene = old_scene.map(|old_scene| (old_scene.host_id.clone(), existing.component_generation(), existing.key.clone(), old_scene.component_kind, old_scene.surface_id.clone()));
                            let candidate_scene = if preserve_candidate_scene_identity && same_logical_scene { old_scene.as_ref().map(|(host_id, generation, ..)| (host_id.clone(), *generation)) } else { None };
                            (old_scene, candidate_scene)
                        }) else {
                            return cursor.refuse(UiDocumentReconcileFault::Detached);
                        };
                        let preserved_scene_identity = presented_scene.or(candidate_scene);
                        let retirement = old_scene.filter(|(host_id, _, _, _, _)| {
                            let owned = preserve_candidate_scene_identity || presented.is_some_and(|tree| tree.component_scene_host_is_mounted(host_id));
                            let survives = preserved_scene_identity.as_ref().is_some_and(|(preserved_host, _)| preserved_host == host_id);
                            owned && !survives
                        });
                        if let Some((host_id, generation, key, kind, surface_id)) = retirement {
                            if !retire_scene(UiRetiredComponentScene { document_id: planned.id, host_id, window_id: surface.to_owned(), window_generation, component_generation: generation, node, key, kind, surface_id }) {
                                return UiDocumentReconcileStep::Pending;
                            }
                        }
                        let scene_identity = if matches!(&spec.0, UiNode::ComponentScene(_)) {
                            match preserved_scene_identity {
                                Some(identity) => Some(identity),
                                None => {
                                    let Some(generation) = component_mount_generation.checked_add(1) else { return cursor.refuse(UiDocumentReconcileFault::ComponentGeneration) };
                                    *component_mount_generation = generation;
                                    Some((component_scene_host_id(window_generation, generation), generation))
                                }
                            }
                        } else {
                            None
                        };
                        if let UiNode::ComponentScene(scene) = &mut spec.0 {
                            scene.host_id = scene_identity.as_ref().expect("scene mount identity").0.clone();
                        }
                        if let Some(existing) = self.node_mut(node) {
                            if let Some((_, generation)) = scene_identity {
                                existing.component_generation = generation;
                            }
                            if existing.key != key {
                                existing.key = key;
                            }
                            if existing.spec != spec {
                                existing.spec = spec;
                            }
                            existing.layout_spec = Some(layout_spec);
                            existing.intent = Some(intent);
                        }
                        node
                    }
                    None => {
                        if presented_scene.is_none() {
                            if let Some((node, host_id, generation, key, kind, surface_id)) = presented_component {
                                if !retire_scene(UiRetiredComponentScene {
                                    document_id: planned.id,
                                    host_id,
                                    window_id: surface.to_owned(),
                                    window_generation,
                                    component_generation: generation,
                                    node,
                                    key,
                                    kind,
                                    surface_id,
                                }) {
                                    return UiDocumentReconcileStep::Pending;
                                }
                            }
                        }
                        let scene_identity = if matches!(&spec.0, UiNode::ComponentScene(_)) {
                            match presented_scene {
                                Some(identity) => Some(identity),
                                None => {
                                    let Some(generation) = component_mount_generation.checked_add(1) else { return cursor.refuse(UiDocumentReconcileFault::ComponentGeneration) };
                                    *component_mount_generation = generation;
                                    Some((component_scene_host_id(window_generation, generation), generation))
                                }
                            }
                        } else {
                            None
                        };
                        if let (UiNode::ComponentScene(scene), Some((host_id, _))) = (&mut spec.0, scene_identity.as_ref()) {
                            scene.host_id = host_id.clone();
                        }
                        let mut mounted = Node::new(key, spec);
                        if let Some((_, generation)) = scene_identity {
                            mounted.component_generation = generation;
                        }
                        mounted.layout_spec = Some(layout_spec);
                        mounted.intent = Some(intent);
                        let node = self.insert_detached(mounted);
                        self.bind_document_node(planned.id, node);
                        node
                    }
                };
                if let Some(mounted) = self.node_mut(node) {
                    mounted.flags.set(NodeFlags::SCROLLABLE, routing.scrollable);
                    mounted.flags.set(NodeFlags::CLIPS_CHILDREN, routing.clips);
                    if routing.overlay {
                        mounted.flags.set(NodeFlags::OVERLAY, true);
                    }
                }
                if let Some(entry) = cursor.plan.get_mut(cursor.mount) {
                    entry.node = Some(node);
                }
                match planned.parent {
                    None => self.root = Some(node),
                    Some(parent) => {
                        let Some(parent_node) = cursor.plan.get(parent).and_then(|entry| entry.node) else { return cursor.refuse(UiDocumentReconcileFault::Detached) };
                        self.attach_child(parent_node, node);
                    }
                }
                if Some(node) == preserved_composite_owner {
                    self.reattach_composite_rows(node);
                }
                cursor.mount += 1;
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Retire => {
                let Some((id, node)) = self.document_bindings().get(cursor.sweep).copied() else {
                    cursor.phase = UiDocumentReconcilePhase::Publish;
                    return UiDocumentReconcileStep::Pending;
                };
                if cursor.planned_ids.binary_search(&id).is_ok() {
                    cursor.sweep += 1;
                    return UiDocumentReconcileStep::Pending;
                }
                let retirement = self.node(node).and_then(|existing| {
                    let UiNode::ComponentScene(scene) = &existing.spec.0 else { return None };
                    Some(UiRetiredComponentScene {
                        document_id: id,
                        host_id: scene.host_id.clone(),
                        window_id: surface.to_owned(),
                        window_generation,
                        component_generation: existing.component_generation(),
                        node,
                        key: existing.key.clone(),
                        kind: scene.component_kind,
                        surface_id: scene.surface_id.clone(),
                    })
                });
                if retirement.is_some_and(|retirement| !retire_scene(retirement)) {
                    return UiDocumentReconcileStep::Pending;
                }
                self.unbind_document_node_at(cursor.sweep);
                self.remove_detached(node);
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Publish => {
                if let Some(root) = self.root {
                    self.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
                }
                cursor.phase = UiDocumentReconcilePhase::Complete;
                UiDocumentReconcileStep::Complete
            }
            UiDocumentReconcilePhase::Complete => UiDocumentReconcileStep::Complete,
            UiDocumentReconcilePhase::Fault => UiDocumentReconcileStep::Fault(cursor.fault.unwrap_or(UiDocumentReconcileFault::MissingRecord)),
        }
    }
}
//#endregion 🌳️DocumentTreeReconcile

#[cfg(any(test, feature = "testkit"))]
fn variant_discriminant(node: &UiNode) -> u32 {
    match node {
        UiNode::Stack(_) => 0,
        UiNode::Text(_) => 1,
        UiNode::Button(_) => 2,
        UiNode::Separator(_) => 3,
        UiNode::Input(_) => 4,
        UiNode::Select(_) => 5,
        UiNode::Toggle(_) => 6,
        UiNode::KeyValue(_) => 8,
        UiNode::Slider(_) => 9,
        UiNode::NumberStepper(_) => 10,
        UiNode::Ring(_) => 11,
        UiNode::IconSelect(_) => 12,
        UiNode::Progress(_) => 20,
        UiNode::Field(_) => 13,
        UiNode::Section(_) => 14,
        UiNode::Tree(_) => 15,
        UiNode::Image(_) => 16,
        UiNode::ComponentScene(_) => 17,
        UiNode::ExternalSlot(_) => 18,
        UiNode::Group(_) => 19,
    }
}

#[cfg(any(test, feature = "testkit"))]
fn explicit_id(node: &UiNode) -> Option<&str> {
    match node {
        UiNode::Stack(n) => n.id.as_deref(),
        UiNode::Button(n) => n.id.as_deref(),
        UiNode::Input(n) => Some(n.id.as_str()),
        UiNode::Select(n) => Some(n.id.as_str()),
        UiNode::Toggle(n) => Some(n.id.as_str()),
        UiNode::Slider(n) => Some(n.id.as_str()),
        UiNode::NumberStepper(n) => Some(n.id.as_str()),
        UiNode::Ring(n) => Some(n.id.as_str()),
        UiNode::IconSelect(n) => Some(n.id.as_str()),
        UiNode::Progress(n) => Some(n.id.as_str()),
        UiNode::Field(n) => Some(n.id.as_str()),
        UiNode::Section(n) => Some(n.id.as_str()),
        UiNode::Group(n) => Some(n.id.as_str()),
        UiNode::Image(n) => Some(n.id.as_str()),
        UiNode::ComponentScene(n) => Some(n.surface_id.as_str()),
        UiNode::ExternalSlot(n) => Some(n.body_key.as_str()),
        UiNode::Text(_) | UiNode::Separator(_) | UiNode::KeyValue(_) | UiNode::Tree(_) => None,
    }
}

#[cfg(any(test, feature = "testkit"))]
fn node_key(node: &UiNode, ordinal: u32) -> NodeKey {
    match explicit_id(node) {
        Some(id) if !id.is_empty() => NodeKey::Explicit(id.to_string()),
        _ => NodeKey::Positional(variant_discriminant(node), ordinal),
    }
}

/// 🌿️ The keyed-diffable children of `node`: `Stack`/`Section`'s own `children`, `Field`'s single
/// `child`, borrowed straight from `node` (no allocation); `Select`/`Tree`'s *synthesized* rows (see
/// `🔖️CompositeExpansion`), freshly built each call since they're derived from non-`UiNode` payload.
/// 🔽️ `open` is the parent's own `tree::WidgetState::open` — a CLOSED `Select` materializes no option
/// rows at all, matching React, whose `SelectContent` mounts only while the dropdown is open. The
/// rows used to be built unconditionally because `WidgetState` had nowhere to record the bit; it does
/// now (`events::toggle_select_popup`/`finish_close` own it), so the gate lives here.
/// Everything else has no nested `UiNode` payload to recurse into. `presence.state == Hidden`
/// children are dropped here — hidden means not rendered at all, so they get no retained node, no
/// layout, no paint, no hit-test; this is the one choke point every caller goes through.
#[cfg(any(test, feature = "testkit"))]
fn children_of(node: &UiNode, open: bool) -> Vec<Cow<'_, UiNode>> {
    let children = match node {
        UiNode::Stack(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Section(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Group(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Field(n) => vec![Cow::Borrowed(n.child.as_ref())],
        UiNode::Select(select) if open => select.items.iter().map(|item| Cow::Owned(select_item_row(select, item))).collect(),
        UiNode::Select(_) => Vec::new(),
        UiNode::Tree(tree_node) => tree_node.sections.iter().map(|section| Cow::Owned(tree_section_row(tree_node, section))).collect(),
        _ => Vec::new(),
    };
    children.into_iter().filter(|child: &Cow<'_, UiNode>| child.presence().visible()).collect()
}

//#region 🔖️CompositeExpansion
/// 🔽️ Synthesizes one retained `Button` row per `Select` item, keyed by the item's own `value` (via
/// `explicit_id`'s `UiNode::Button` arm) — `UiSelectItem.value` is already Select's stable per-option
/// identity (it's what `UiSelectNode.value` itself holds to name the current choice), so reusing it as
/// the row's key needs no extra bookkeeping.
///
/// 🔓️ Ungated in ticket 26/09/17 packet W15a. It used to be `cfg(test, testkit)` alongside the
/// `apply_tree` reconciler that was its only caller, which meant PRODUCTION never materialized an
/// open popup's rows at all: `paint_select` (which draws the rows from the inline `items`) is itself
/// `cfg(test)`, the retained `UiNode::Select` paint arm draws only the trigger, and
/// `retained_hit_registration`'s `Select` arm mints one target for the trigger — so an open popup on
/// the live renderer painted nothing, hit nothing and projected nothing. The production minter is
/// `paint::sync_interactive_state_node_step`'s `SelectMint` phase, through
/// `UiTree::mint_composite_row`.
pub(crate) fn select_item_row(select: &UiSelectNode, item: &UiSelectItem) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(item.value.clone()), icon_id: IconName::CircleDot, label: item.label.clone(), action: with_item_value_arg(&select.on_change, &item.value), style: None, presence: UiPresence::default(), menu: None })
}

/// 🏷️ Clones `action`, merging a `"value"` key into its JSON `args` object (creating one if absent)
/// so a click on one synthesized `Select` row is distinguishable from any other row once a later
/// events milestone dispatches it — `on_change.clone()` alone would fire an identical, valueless
/// action for every row. Ungated with [`select_item_row`] (packet W15a).
fn with_item_value_arg(action: &ActionDescriptor, value: &str) -> ActionDescriptor {
    let mut merged = action.clone();
    let mut entries = match merged.args.take() {
        Some(DslValue::Object(map)) => map,
        _ => Vec::new(),
    };
    entries.push(("value".to_string(), DslValue::String(value.to_string())));
    merged.args = Some(DslValue::Object(entries));
    merged
}

/// 🌳️ Synthesizes one retained `Stack` row per `Tree` section, keyed by `section.id`, wrapping its
/// `items` (recursively expanded by `tree_item_row`) as retained children.
#[cfg(any(test, feature = "testkit"))]
fn tree_section_row(tree_node: &UiTreeNode, section: &UiTreeSectionNode) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some(section.id.clone()),
        presence: section.presence.clone(),
        activate: None,
        drop_action: tree_node.drop_action.clone(),
        drop_overlay: None,
        children: section.items.iter().map(tree_item_row).collect(),
        menu: None,
    })
}

/// 🌳️ Synthesizes one retained `Stack` row per `Tree` item, keyed by `item.id`. Carries the item's own
/// `presence` (already the single source of truth for selected/previewed/status — no more union with
/// a tree-level id list) and `activate` (the row's click `action`) as a `UiStackNode`'s own fields,
/// plus its embedded `control` (via `ui_control_to_node`), trailing `actions` (via
/// `tree_item_action_row`), and nested `items` (recursively) as retained children.
/// `draggable`/`drag_data` have no matching `UiStackNode` field to carry them structurally —
/// `events::is_plain_stack_container`/`find_tree_item_spec` re-derive those straight from this row's
/// key (`item.id`) against the parent `Tree` node's still-fully-intact `spec.0` (reconcile never drops
/// fields, only clones them into `WidgetSpec`). ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM W3a: `hover_action`/`unhover_action` are deleted — hover is now framework-owned per
/// `UiTreeNode.interaction_domain`, never a per-item action.
#[cfg(any(test, feature = "testkit"))]
fn tree_item_row(item: &UiTreeItemNode) -> UiNode {
    let mut children: Vec<UiNode> = Vec::new();
    if let Some(control) = &item.control {
        children.push(ui_control_to_node(control.clone()));
    }
    for action in item.actions.iter().flatten() {
        if action.placement() == UiTreeActionPlacement::Menu {
            continue;
        }
        children.push(tree_item_action_row(action));
    }
    for nested in item.items.iter().flatten() {
        children.push(tree_item_row(nested));
    }
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some(item.id.clone()),
        presence: item.presence.clone(),
        activate: item.action.clone(),
        drop_action: None,
        drop_overlay: None,
        children,
        menu: item.menu.clone(),
    })
}

/// 🌳️ Synthesizes one retained `Button` row per `UiTreeItemAction` (a `Tree` item's trailing/
/// row-placement action buttons). No stable id exists on `UiTreeItemAction` itself (unlike items/
/// sections), so this leaves `UiButtonNode.id` unset — `node_key`'s positional fallback (keyed by the
/// action's ordinal within its parent row's `actions` list) is already stable across re-renders for a
/// fixed action set, matching every other id-less synthesized/leaf child in this module.
#[cfg(any(test, feature = "testkit"))]
fn tree_item_action_row(action: &UiTreeItemAction) -> UiNode {
    UiNode::Button(UiButtonNode { id: None, icon_id: action.icon_id, label: action.label.clone().unwrap_or_else(|| Label::data("")), action: action.action.clone(), style: None, presence: UiPresence::default(), menu: None })
}
//#endregion 🔖️CompositeExpansion

/// ⚖️ Whether the two nodes' *own* scalar fields (excluding nested `UiNode` children, which are
/// reconciled and dirtied independently) are equal.
#[cfg(any(test, feature = "testkit"))]
fn own_fields_equal(previous: &UiNode, next: &UiNode) -> bool {
    match (previous, next) {
        (UiNode::Stack(p), UiNode::Stack(n)) => {
            p.direction == n.direction && p.gap == n.gap && p.padding == n.padding && p.id == n.id && p.presence == n.presence && p.activate == n.activate && p.drop_action == n.drop_action && p.children.len() == n.children.len()
        }
        (UiNode::Section(p), UiNode::Section(n)) => p.id == n.id && p.label == n.label && p.default_open == n.default_open && p.presence == n.presence && p.children.len() == n.children.len(),
        (UiNode::Field(p), UiNode::Field(n)) => p.id == n.id && p.label == n.label && p.description == n.description && p.required == n.required && p.error == n.error && p.presence == n.presence,
        _ => previous == next,
    }
}

/// 📐️ Whether the field(s) that differ between `previous` and `next` affect measurement/layout (as
/// opposed to paint-only state like `selected`/`status`/`disabled`). `presence.visible()` flipping
/// (i.e. `state` crossing into/out of `Hidden`) always counts — a hidden element occupies no layout
/// space at all, so becoming hidden/unhidden must re-run layout for its parent, unlike every other
/// `presence` change (selected/status/hover/previewed/disabled), which is paint-only.
#[cfg(any(test, feature = "testkit"))]
fn layout_affecting_change(previous: &UiNode, next: &UiNode) -> bool {
    if previous.presence().visible() != next.presence().visible() {
        return true;
    }
    match (previous, next) {
        (UiNode::Stack(p), UiNode::Stack(n)) => p.direction != n.direction || p.gap != n.gap || p.padding != n.padding || p.children.len() != n.children.len(),
        (UiNode::Text(p), UiNode::Text(n)) => p.value != n.value,
        (UiNode::Field(p), UiNode::Field(n)) => p.label != n.label || p.description != n.description,
        (UiNode::Section(p), UiNode::Section(n)) => p.label != n.label || p.children.len() != n.children.len(),
        _ => false,
    }
}

#[cfg(any(test, feature = "testkit"))]
impl UiTree {
    /// 🔁️ Applies an incoming declarative `UiNode` tree to this retained tree: keyed single-pass
    /// child matching, minimal-dirty-flag diffing of matched nodes, insertion of unmatched incoming
    /// children, removal of unmatched existing children. Re-applying an identical tree sets zero
    /// dirty flags anywhere in the tree.
    pub fn apply_tree(&mut self, incoming: &UiNode) {
        let key = node_key(incoming, 0);
        match self.root {
            Some(root_id) if self.node(root_id).map(|n| &n.key) == Some(&key) => {
                self.diff_and_update(root_id, incoming);
                self.reconcile_children(root_id, incoming);
            }
            Some(root_id) => {
                self.remove(root_id);
                self.root = None;
                self.insert_new_root(key, incoming);
            }
            None => self.insert_new_root(key, incoming),
        }
    }

    fn insert_new_root(&mut self, key: NodeKey, incoming: &UiNode) {
        let id = self.insert_child(None, Node::new(key, WidgetSpec(incoming.clone())));
        self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        self.root = Some(id);
        self.reconcile_children(id, incoming);
    }

    fn diff_and_update(&mut self, id: NodeId, incoming: &UiNode) {
        let (needs_layout, needs_paint) = match self.node(id) {
            Some(node) if own_fields_equal(&node.spec.0, incoming) => (false, false),
            Some(node) if layout_affecting_change(&node.spec.0, incoming) => (true, true),
            Some(_) => (false, true),
            None => return,
        };
        if let Some(node) = self.node_mut(id) {
            node.spec = WidgetSpec(incoming.clone());
        }
        if needs_layout {
            self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        } else if needs_paint {
            self.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        }
    }

    /// 🚩️ Keeps structural `NodeFlags` that reflect `incoming`'s own shape (not its diff status) in
    /// sync — currently just `HAS_POPUP` on a `Select` with ≥1 item, so a later events/paint milestone
    /// can find "this Select has synthesized option rows ready under it" (see this module's own doc
    /// comment for the `WidgetState` open/closed wiring request that gates actually showing them)
    /// without re-deriving it from `spec.0` itself. Deliberately bypasses `mark_dirty` — direct flag
    /// mutation, no `SUBTREE_DIRTY` bubbling — since this is bookkeeping metadata, not a repaint signal.
    fn sync_composite_flags(&mut self, id: NodeId, incoming: &UiNode) {
        if let UiNode::Select(select) = incoming {
            if let Some(node) = self.node_mut(id) {
                node.flags.set(NodeFlags::HAS_POPUP, !select.items.is_empty());
            }
        }
    }

    fn reconcile_children(&mut self, parent: NodeId, incoming: &UiNode) {
        self.sync_composite_flags(parent, incoming);
        let open = self.node(parent).is_some_and(|node| node.state.open);
        let incoming_children = children_of(incoming, open);
        let existing_children: Vec<NodeId> = self.children(parent).collect();

        let mut existing_by_key: HashMap<NodeKey, NodeId> = HashMap::with_capacity(existing_children.len());
        for child_id in &existing_children {
            if let Some(node) = self.node(*child_id) {
                existing_by_key.insert(node.key.clone(), *child_id);
            }
        }

        let mut used_keys: HashSet<NodeKey> = HashSet::with_capacity(incoming_children.len());
        let mut matched_ids: HashSet<NodeId> = HashSet::with_capacity(incoming_children.len());
        for (ordinal, child) in incoming_children.iter().enumerate() {
            let key = node_key(child, ordinal as u32);
            let matched_id = match existing_by_key.get(&key) {
                Some(existing_id) if !used_keys.contains(&key) => {
                    used_keys.insert(key);
                    self.diff_and_update(*existing_id, child);
                    self.reconcile_children(*existing_id, child);
                    *existing_id
                }
                _ => {
                    let id = self.insert_child(Some(parent), Node::new(key, WidgetSpec(child.clone().into_owned())));
                    self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
                    self.reconcile_children(id, child);
                    id
                }
            };
            matched_ids.insert(matched_id);
        }

        for existing_id in existing_children {
            if !matched_ids.contains(&existing_id) {
                self.remove(existing_id);
            }
        }
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-reconcile-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🌳️document-tree-reconcile/🦀️.rs"]
mod document_tree_reconcile_tests;
// #endregion reconcile
