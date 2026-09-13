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
    SurfaceKind, UiButtonNode, UiComponentSceneNode, UiControlNode, UiDropOverlaySpec, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiMenuRef, UiNode,
    UiNumberStepperNode, UiPresence, UiRingNode, UiSectionNode, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiState, UiStatus, UiTextNode, UiToggleNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode,
    UiTreeSectionNode,
};
use crate::wgpu::tree::{Node, NodeFlags, NodeKey, UiDocumentPageRejection, UiDocumentTree, UiDocumentTreeFault, UiTree, WidgetSpec};
use crate::wgpu::IconName;
use ui_contract::{UiDocumentNodePage, UiNodeId, UiNodeRecord, UI_DOCUMENT_NODES};

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentReconcileStep {
    Pending,
    Complete,
    Fault(UiDocumentReconcileFault),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum UiDocumentReconcilePhase {
    #[default]
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
    /// 🔁️ Rearms the cursor for `generation`, discarding any half-finished pass for an older one.
    pub fn rearm(&mut self, generation: u64) {
        if self.generation == generation && !matches!(self.phase, UiDocumentReconcilePhase::Complete | UiDocumentReconcilePhase::Fault) {
            return;
        }
        self.phase = UiDocumentReconcilePhase::Adopt;
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

fn space_token(token: ui_contract::SpaceToken) -> Option<String> {
    match token {
        ui_contract::SpaceToken::None => Some("none".to_string()),
        ui_contract::SpaceToken::Xs | ui_contract::SpaceToken::Sm => Some("tight".to_string()),
        ui_contract::SpaceToken::Md => None,
        ui_contract::SpaceToken::Lg | ui_contract::SpaceToken::Xl | ui_contract::SpaceToken::Xxl => Some("loose".to_string()),
    }
}

fn edge_token(edge: ui_contract::EdgeSpace) -> Option<String> {
    match edge {
        ui_contract::EdgeSpace::All(token) => space_token(token),
        ui_contract::EdgeSpace::Symmetric { vertical, .. } => space_token(vertical),
        ui_contract::EdgeSpace::Each { top, .. } => space_token(top),
    }
}

/// 📐️ The `(direction, gap, padding)` triple `UiStackNode` carries, read off the record's own
/// `LayoutSpec` — the contract keeps geometry on the record, never on the component props.
fn stack_metrics(layout: &ui_contract::LayoutSpec) -> (String, Option<String>, Option<String>) {
    match layout {
        ui_contract::LayoutSpec::Stack(stack) => {
            let direction = if matches!(stack.axis, ui_contract::Axis::Horizontal) { "horizontal" } else { "vertical" };
            (direction.to_string(), space_token(stack.gap), edge_token(stack.padding))
        }
        ui_contract::LayoutSpec::Grid(grid) => ("vertical".to_string(), space_token(grid.row_gap), edge_token(grid.padding)),
        ui_contract::LayoutSpec::Scroll(scroll) => ("vertical".to_string(), None, edge_token(scroll.padding)),
        ui_contract::LayoutSpec::Overlay(overlay) => ("vertical".to_string(), None, edge_token(overlay.inset)),
        ui_contract::LayoutSpec::Leaf(_) | ui_contract::LayoutSpec::Absolute(_) => ("vertical".to_string(), None, None),
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

/// 🎬️ The record's binding for `trigger`, as the legacy `ActionDescriptor` the wgpu paint/event
/// path dispatches. `controller_id` is the OWNING APP's controller — the same value
/// `ShellState::queue_host_effects` stamps on every effect-borne descriptor — because a descriptor's
/// controller is who answers it, and the contract moved that identity off the node onto the session.
fn record_action(record: &UiNodeRecord, trigger: ui_contract::Trigger, controller: &str) -> Option<ActionDescriptor> {
    record
        .bindings
        .iter()
        .find(|binding| binding.trigger == trigger)
        .map(|binding| ActionDescriptor { controller_id: controller.to_string(), action: binding.action.name.as_str().to_string(), args: binding.args.as_ref().and_then(ui_value_to_dsl) })
}

fn record_action_or_inert(record: &UiNodeRecord, trigger: ui_contract::Trigger, controller: &str) -> ActionDescriptor {
    record_action(record, trigger, controller).unwrap_or_else(|| ActionDescriptor { controller_id: controller.to_string(), action: String::new(), args: None })
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
/// kinds; only `virtual-file-system` differs on the wire (see `ui_contract::SurfaceKind`'s own
/// docstring for why that rename landed there and not here).
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

/// 🚚️ Reattaches a world-3d surface's out-of-doc payload lanes to the spine its `doc.bytes` decoded
/// to. A lane the document declares but whose carrier has not fully arrived is left at its spine
/// value rather than guessed — a truncated `meshes` payload would parse to an EMPTY scene, which is
/// strictly worse than the previous frame's.
fn merge_world3d_lanes(document: &UiDocumentTree, record: &UiNodeRecord, scene: &mut ui_scene::World3dScene) {
    if scene.lanes.is_empty() {
        return;
    }
    let declared: Vec<ui_scene::World3dSceneLaneRef> = scene.lanes.clone();
    for child_id in record.children.iter() {
        let Some(child) = document.record(*child_id) else { continue };
        let Some(lane) = ui_scene::World3dSceneLane::from_body_key(child.key.as_str()) else { continue };
        let Some(reference) = declared.iter().find(|entry| entry.lane == lane.name()) else { continue };
        let payload = lane_payload(document, child);
        if payload.len() as u32 != reference.bytes {
            continue;
        }
        let _ = ui_scene::SceneDoc::merge_lane(scene, child.key.as_str(), payload);
    }
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
                merge_world3d_lanes(document, record, &mut scene);
                node.world_3d = Some(scene);
            }
        }
        ui_contract::SurfaceKind::NodeGraph => node.node_graph = ui_scene::decode::<ui_scene::NodeGraphScene>(props).ok(),
        ui_contract::SurfaceKind::Canvas2d => node.canvas_2d = ui_scene::decode::<ui_scene::Canvas2dScene>(props).ok(),
        ui_contract::SurfaceKind::TextEditor => node.text_editor = ui_scene::decode::<ui_scene::TextEditorScene>(props).ok(),
        ui_contract::SurfaceKind::Table => node.table = ui_scene::decode::<ui_scene::TableScene>(props).ok(),
        ui_contract::SurfaceKind::Paint2d => node.paint_2d = ui_scene::decode::<ui_scene::Paint2dScene>(props).ok(),
        ui_contract::SurfaceKind::VirtualFileSystem => node.virtual_file_system = ui_scene::decode::<ui_scene::VirtualFileSystemScene>(props).ok(),
        ui_contract::SurfaceKind::TiledMap => node.tiled_map = ui_scene::decode::<ui_scene::TiledMapScene>(props).ok(),
        ui_contract::SurfaceKind::Board2d => node.board2d = ui_scene::decode::<ui_scene::Board2dScene>(props).ok(),
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
        action: ActionDescriptor {
            controller_id: controller.to_string(),
            action: action.action.action.name.as_str().to_string(),
            args: action.action.args.as_ref().and_then(ui_value_to_dsl),
        },
        placement: Some(match action.placement {
            ui_contract::RowActionPlacement::Row => UiTreeActionPlacement::Row,
            ui_contract::RowActionPlacement::Menu => UiTreeActionPlacement::Menu,
        }),
    }
}

/// 🌳️ Assembles one `Component::TreeItem` record and its whole subtree into the inline
/// `UiTreeItemNode` the retained `Tree` spec carries. A child that projects to a control becomes the
/// row's `control`; every other child recurses as a nested item.
fn tree_item(document: &UiDocumentTree, record: &UiNodeRecord, surface: &str, controller: &str, depth: usize) -> UiTreeItemNode {
    let ui_contract::Component::TreeItem(props) = &record.component else {
        return UiTreeItemNode {
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
        commit: props.commit.as_ref().map(|value| value.as_str().to_string()),
        min: props.min,
        max: props.max,
        step: props.step,
        accept: props.accept.as_ref().map(|value| value.as_str().to_string()),
        on_change: input_commit_action(record, props, controller),
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
    UiNode::KeyValue(UiKeyValueNode {
        entries: props.entries.iter().map(|entry| UiKeyValueEntry { label: contract_label(&entry.label), value: entry.value.as_str().to_string() }).collect(),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
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
            ui_contract::ContainerRole::Section => UiNode::Section(UiSectionNode {
                id: record.key.as_str().to_string(),
                label: optional_contract_label(props.label.as_ref()),
                default_open: props.default_open,
                presence,
                menu,
                children: Vec::new(),
            }),
            ui_contract::ContainerRole::Group => UiNode::Group(UiGroupNode {
                id: record.key.as_str().to_string(),
                label: optional_contract_label(props.label.as_ref()).unwrap_or_else(|| Label::data("")),
                default_open: props.default_open,
                presence,
                menu,
                children: Vec::new(),
            }),
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
        ui_contract::Component::Image(props) => UiNode::Image(UiImageNode { id: record.key.as_str().to_string(), src: props.src.as_str().to_string(), alt: optional_contract_label(props.alt.as_ref()), presence, menu }),
        ui_contract::Component::Tree(props) => {
            let sections = record
                .children
                .iter()
                .filter_map(|child_id| document.record(*child_id))
                .map(|child| match &child.component {
                    ui_contract::Component::TreeSection(section) => UiTreeSectionNode {
                        id: child.key.as_str().to_string(),
                        label: optional_contract_label(section.label.as_ref()),
                        default_open: section.default_open,
                        presence: record_presence(child),
                        items: child.children.iter().filter_map(|item_id| document.record(*item_id)).map(|item| tree_item(document, item, surface, controller, 1)).collect(),
                    },
                    _ => UiTreeSectionNode { id: child.key.as_str().to_string(), label: None, default_open: None, presence: record_presence(child), items: vec![tree_item(document, child, surface, controller, 1)] },
                })
                .collect();
            UiNode::Tree(UiTreeNode { sections, presence, drop_action: record_action(record, ui_contract::Trigger::Drop, controller), menu, interaction_domain: props.interaction_domain.as_ref().map(|value| value.as_str().to_string()) })
        }
        // 🌳️ A tree's section and item records mount as the keyed `Stack` ROWS this engine's
        // interactive sync resolves by id — the document-path twin of `children_of`'s `Tree` arm
        // (`tree_section_row`/`tree_item_row`). Their content is painted from the owning `Tree`'s own
        // inline spec; these rows carry identity, layout and drag/drop state.
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
        ui_contract::Component::Surface(props) => UiNode::ComponentScene(surface_scene_node(document, record, props, surface, controller)),
        ui_contract::Component::Extension(props) => UiNode::ExternalSlot(crate::wgpu::component::ui::UiExternalSlotNode {
            plugin_id: String::new(),
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
    pub fn step_document_reconcile(&mut self, cursor: &mut UiDocumentReconcileCursor, surface: &str, controller: &str) -> UiDocumentReconcileStep {
        match cursor.phase {
            UiDocumentReconcilePhase::Complete => return UiDocumentReconcileStep::Complete,
            UiDocumentReconcilePhase::Fault => return UiDocumentReconcileStep::Fault(cursor.fault.unwrap_or(UiDocumentReconcileFault::MissingRecord)),
            _ => {}
        }
        let Some(root_id) = self.document().map(UiDocumentTree::root_id) else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
        match cursor.phase {
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
                let (key, spec) = {
                    let Some(document) = self.document() else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    let Some(record) = document.record(planned.id) else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    (NodeKey::Explicit(record.key.as_str().to_string()), WidgetSpec(ui_node_from_record(document, record, surface, controller)))
                };
                let node = match self.document_node(planned.id).filter(|node| self.contains(*node)) {
                    Some(node) => {
                        if let Some(existing) = self.node_mut(node) {
                            if existing.key != key {
                                existing.key = key;
                            }
                            if existing.spec != spec {
                                existing.spec = spec;
                            }
                        }
                        node
                    }
                    None => {
                        let node = self.insert_detached(Node::new(key, spec));
                        self.bind_document_node(planned.id, node);
                        node
                    }
                };
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
/// Everything else has no nested `UiNode` payload to recurse into. `presence.state == Hidden`
/// children are dropped here — hidden means not rendered at all, so they get no retained node, no
/// layout, no paint, no hit-test; this is the one choke point every caller goes through.
#[cfg(any(test, feature = "testkit"))]
fn children_of(node: &UiNode) -> Vec<Cow<'_, UiNode>> {
    let children = match node {
        UiNode::Stack(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Section(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Group(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Field(n) => vec![Cow::Borrowed(n.child.as_ref())],
        UiNode::Select(select) => select.items.iter().map(|item| Cow::Owned(select_item_row(select, item))).collect(),
        UiNode::Tree(tree_node) => tree_node.sections.iter().map(|section| Cow::Owned(tree_section_row(tree_node, section))).collect(),
        _ => Vec::new(),
    };
    children.into_iter().filter(|child: &Cow<'_, UiNode>| child.presence().visible()).collect()
}

//#region 🔖️CompositeExpansion
/// 🔽️ Synthesizes one retained `Button` row per `Select` item, keyed by the item's own `value` (via
/// `explicit_id`'s `UiNode::Button` arm) — `UiSelectItem.value` is already Select's stable per-option
/// identity (it's what `UiSelectNode.value` itself holds to name the current choice), so reusing it as
/// the row's key needs no extra bookkeeping. See this module's doc comment for the open/closed
/// `WidgetState` wiring request this groundwork is waiting on.
#[cfg(any(test, feature = "testkit"))]
fn select_item_row(select: &UiSelectNode, item: &UiSelectItem) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(item.value.clone()), icon_id: IconName::CircleDot, label: item.label.clone(), action: with_item_value_arg(&select.on_change, &item.value), style: None, presence: UiPresence::default(), menu: None })
}

/// 🏷️ Clones `action`, merging a `"value"` key into its JSON `args` object (creating one if absent)
/// so a click on one synthesized `Select` row is distinguishable from any other row once a later
/// events milestone dispatches it — `on_change.clone()` alone would fire an identical, valueless
/// action for every row.
#[cfg(any(test, feature = "testkit"))]
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
        let incoming_children = children_of(incoming);
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
