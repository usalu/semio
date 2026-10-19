//! 🧾️ Record routing and actions: the subtree consumption, the space, edge and stack metrics, the layout routing flags, the labels, icons and menus, the presence, action and intent bindings and the scene lane merge. See [`super`].

use super::*;

// #region reconcile
//#region 🌳️DocumentTreeReconcile
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
pub(super) fn record_consumes_subtree(record: &UiNodeRecord) -> bool {
    matches!(record.component, ui_contract::Component::Surface(_))
}

/// 📐️ A record's [`ui_contract::SpaceToken`] as the legacy `UiStackNode` string. It stamps the
/// token's OWN name (`"md"`, `"lg"`, …), not the old three-bucket `none`/`tight`/`loose` collapse
/// that rendered `Xs` and `Sm` identically and shrank `Md` fourfold: a record-mounted node's real
/// geometry now travels on `Node::layout_spec` (see `flex`'s header), and this channel stays
/// lossless so the reconcile diff still sees every token change.
fn space_token(token: ui_contract::SpaceToken) -> Option<String> {
    Some(semio_framework_ui_layout::space_token_name(token).to_string())
}

/// 📐️ A record's [`ui_contract::EdgeSpace`] as one legacy string — every side, space-separated in
/// CSS shorthand order, so an asymmetric padding no longer collapses onto one sampled side.
fn edge_token(edge: ui_contract::EdgeSpace) -> Option<String> {
    let name = semio_framework_ui_layout::space_token_name;
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
pub(super) fn stack_metrics(layout: &ui_contract::LayoutSpec) -> (String, Option<String>, Option<String>) {
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
pub(super) struct LayoutRoutingFlags {
    pub(super) scrollable: bool,
    pub(super) clips: bool,
    pub(super) overlay: bool,
}

pub(super) fn layout_routing_flags(layout: &ui_contract::LayoutSpec) -> LayoutRoutingFlags {
    match layout {
        ui_contract::LayoutSpec::Scroll(scroll) => LayoutRoutingFlags { scrollable: !matches!(scroll.axes, ui_contract::ScrollAxes::None), clips: true, overlay: false },
        ui_contract::LayoutSpec::Overlay(_) => LayoutRoutingFlags { scrollable: false, clips: false, overlay: true },
        _ => LayoutRoutingFlags { scrollable: false, clips: false, overlay: false },
    }
}

pub(super) fn contract_label(label: &ui_contract::Label) -> Label {
    Label::data(label.0.as_str())
}

pub(super) fn optional_contract_label(label: Option<&ui_contract::Label>) -> Option<Label> {
    label.map(contract_label)
}

pub(super) fn icon_name(icon: &ui_contract::UiText) -> IconName {
    IconName::from_str(icon.as_str()).unwrap_or(IconName::CircleDot)
}

fn ui_value_to_dsl(value: &impl serde::Serialize) -> Option<DslValue> {
    serde_json::to_value(value).ok().map(DslValue::from)
}

pub(super) fn menu_ref(record: &UiNodeRecord) -> Option<UiMenuRef> {
    record.menu.as_ref().map(|menu| UiMenuRef { id: menu.id.as_str().to_string(), args: menu.args.as_ref().and_then(ui_value_to_dsl) })
}

/// 🧭️ The record's document-carried presence: `disabled` and `transition` are document state (the
/// contract's own `👥️presence.rs` header), while hover/selection/peer marks travel on the separate
/// coalesced presence channel and are stamped onto the arena node by `events`, never by this
/// projection — which is exactly why `sync_interactive_state_node_step` runs before every paint.
pub(super) fn record_presence(record: &UiNodeRecord) -> UiPresence {
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
pub(super) fn record_action(record: &UiNodeRecord, trigger: ui_contract::Trigger, _controller: &str) -> Option<ActionDescriptor> {
    record.bindings.iter().find(|binding| binding.trigger == trigger).map(|binding| action_descriptor(&binding.action.scope, &binding.action.name, binding.action.version, binding.args.as_ref().and_then(ui_value_to_dsl)))
}

/// 🎬️ One versioned action as the `ActionDescriptor` the retained spec carries: the authored scope, and `name@version` past
/// version 1 — the one spelling a record binding and a row verb share.
fn action_descriptor(scope: &ui_contract::UiText, name: &ui_contract::UiText, version: u16, args: Option<DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: scope.as_str().to_string(), action: if version == 1 { name.as_str().to_string() } else { format!("{}@{}", name.as_str(), version) }, args }
}

/// 🎯️ The ONE target a tree or table row's verbs fire on.
fn row_target(record: &UiNodeRecord) -> Option<&ui_contract::RowTarget> {
    match &record.component {
        ui_contract::Component::TreeItem(props) => props.target.as_ref(),
        ui_contract::Component::TableRow(props) => props.target.as_ref(),
        _ => None,
    }
}

/// 🎬️ `verb` on a row's target as the retained spec's `ActionDescriptor` — scope, version and argument map inherited.
pub(super) fn row_verb_action(target: &ui_contract::RowTarget, verb: &ui_contract::UiText) -> ActionDescriptor {
    action_descriptor(&target.scope, verb, target.version, target.args.as_ref().and_then(ui_value_to_dsl))
}

/// ▶️ A row's activation: the target's activation verb, never a record `Trigger::Activate` binding.
pub(super) fn row_activation(record: &UiNodeRecord) -> Option<ActionDescriptor> {
    row_target(record).and_then(|target| target.activation.as_ref().map(|verb| row_verb_action(target, verb)))
}

pub(super) fn record_action_or_inert(record: &UiNodeRecord, trigger: ui_contract::Trigger, controller: &str) -> ActionDescriptor {
    record_action(record, trigger, controller).unwrap_or_else(|| ActionDescriptor { controller_id: controller.to_string(), action: String::new(), args: None })
}

/// 🎬️ The record's whole dispatch contract for the arena node it mounts as: its address on this
/// surface at this revision, plus every `(trigger, ActionId)` it binds. The twin of React's
/// `UiDocumentStore::buildIntent` inputs — `surface`/`revision` come from the store there and from
/// the published document header here, `node`/`node_key` from the record itself.
pub(super) fn record_intent_bindings(record: &UiNodeRecord, surface: &str, revision: u64) -> UiIntentBindings {
    UiIntentBindings {
        address: UiIntentAddress { surface: surface.to_string(), revision, node: record.id.0, node_key: record.key.as_str().to_string() },
        bindings: record
            .bindings
            .iter()
            .map(|binding| (binding.trigger, binding.action.clone()))
            .chain(row_target(record).and_then(|target| target.activation.as_ref().map(|verb| (ui_contract::Trigger::Activate, ui_contract::ActionId::new(target.scope.clone(), verb.clone(), target.version)))))
            .collect(),
    }
}

/// 🔌️ The plugin that owns `surface`. A [`ui_contract::SurfaceId`] is a dotted address whose first
/// segment IS the owning plugin (`"note.play.navigator"` → `"note"`, that type's own docstring), and
/// `ExtensionProps` carries no plugin id of its own — so the slot's host is the publisher's host, and
/// nothing else in the record could name it. Previously hardcoded to `""`, which would have composed
/// empty plugin ids into every id built from it.
pub(super) fn surface_plugin_id(surface: &str) -> String {
    surface.split('.').next().unwrap_or(surface).to_string()
}

pub(super) fn drop_overlay(spec: Option<&ui_contract::DropOverlaySpec>) -> Option<UiDropOverlaySpec> {
    spec.map(|spec| UiDropOverlaySpec { title: contract_label(&spec.title), hint: contract_label(&spec.hint), accept: spec.accept.as_ref().map(|accept| accept.as_str().to_string()) })
}

pub(super) fn input_kind(kind: ui_contract::InputKind) -> String {
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
pub(super) fn surface_kind(kind: ui_contract::SurfaceKind) -> SurfaceKind {
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
pub(super) fn merge_scene_lanes<T: ui_scene::SceneDoc>(document: &UiDocumentTree, record: &UiNodeRecord, scene: &mut T, declared: &[ui_scene::SceneLaneRef], lane_name: impl Fn(&str) -> Option<&'static str>) {
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
//#endregion 🌳️DocumentTreeReconcile
// #endregion reconcile
