//! 🧾️ Dump types, paths and walk: the structure, accessibility, frame, mesh dump types, the node path grammar, the visual fields, the dump walk and the window selection. See [`super`].

use super::*;

//#region 🔬️Introspection
//#region 🔬️IntrospectionTypes
// 🔬️ Live for real via the wasm32 `#[wasm_bindgen]` exports below (`🔬️IntrospectionExports`);
// also gated `test` since `introspection_tests` (bottom of this region) exercises this whole
// dump pipeline natively — neither cfg alone covers both compilations.
#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
pub(super) struct DumpViewport {
    pub w: f32,
    pub h: f32,
    pub(super) dpr: f32,
}

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(serde::Serialize)]
struct DumpNodeState {
    pub(super) hovered: bool,
    pub(super) disabled: bool,
    pub selected: bool,
}

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpNode {
    pub path: String,
    pub kind: &'static str,
    pub rect: [f32; 4],
    pub text: Option<String>,
    color: Option<[f32; 4]>,
    bg: Option<[f32; 4]>,
    font_size: Option<f32>,
    font_weight: Option<u32>,
    pub visible: bool,
    pub state: DumpNodeState,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DumpStructure {
    pub viewport: DumpViewport,
    /// 🪟️ The window this dump answers for, and every window the engine could answer for — so a
    /// caller reading a two-pane mode layout can see both panes rather than guessing which one the
    /// zero-arg selection picked.
    pub window_id: Option<String>,
    pub(super) window_ids: Vec<String>,
    pub(super) focus_path: Option<String>,
    pub nodes: Vec<DumpNode>,
}

/// ♿️ One window's accessibility tree in reading order. `nodes` is `ui_contract`'s own
/// `AccessibilityProjectionNode` verbatim — the host mirror consumes the CONTRACT's shape, not a
/// renderer-private one, so a second renderer publishing the same tree needs no new consumer.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DumpAccessibilityWindow {
    pub window_id: String,
    pub window_generation: u64,
    pub nodes: Vec<ui_contract::AccessibilityProjectionNode>,
}

/// ♿️ The wire shape `dumpAccessibility()` answers.
///
/// 🩸️ Unlike its two `dumpStructure`/`dumpFrameStats` neighbours this is NOT a diagnostic that may
/// answer for "the window a caller probably meant". It is the production accessibility path, and an
/// app whose mode layout mounts several windows must announce ALL of them: a reader that can only
/// reach the largest pane cannot reach the preview beside it. A caller that NAMES a window still
/// gets exactly that window (`window_id` echoes it); a caller that names none gets every live
/// window, in the order the engine tracks them, which is the reading order across the dock.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DumpAccessibility {
    pub window_id: Option<String>,
    pub window_ids: Vec<String>,
    pub windows: Vec<DumpAccessibilityWindow>,
}

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpFrameStats {
    pub window_id: Option<String>,
    pub window_ids: Vec<String>,
    pub draw_calls: usize,
    pub quad_count: usize,
    pub glyph_count: usize,
    /// 🌍️ The 3d half — a `World3d` window paints `ScenePass3d`s and no quads at all, so these are
    /// the only numbers that can say whether its solid reached the frame.
    pub scene_passes: usize,
    pub scene_draws: usize,
    pub scene_instances: usize,
    pub frame_latency: semio_framework_os_renderer_wgpu_host_base::frame_latency::FrameLatencySnapshot,
}

/// 🧊️ One published preview mesh of one `World3d` surface, as the PAYLOAD states it — never as the
/// renderer's own mesh store counts it.
///
/// ⚖️ `state-meshes` in the surface census counts the mesh3d authority's store, which also holds the
/// face overlay, the placeholder and whatever a retirement has not yet dropped; it is therefore not
/// comparable with a producer's committed `delivery.meshes`. These rows are the producer's own
/// `meshes_json` entries, one per published mesh, so a probe can hold a live surface to the very
/// fixture the native lane asserts (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-mesh-oracle-2026-09-14.md`).
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpMesh {
    pub id: String,
    /// 🏷️ The producer's own `role` stamp (`solid`/`wire`/`point`/`vector`), or `(unstamped)` for a
    /// producer that publishes none — an absence a reader must see rather than have defaulted away.
    pub role: String,
    pub indices: usize,
    pub positions: usize,
    pub edge_positions: usize,
    /// 📦️ The mesh's own axis-aligned bounds over `positions`, falling back to `edgePositions` for a
    /// wire body that carries no vertices at all. `None` when the entry carries neither.
    pub bbox_min: Option<[f64; 3]>,
    pub bbox_max: Option<[f64; 3]>,
}

/// 🎯️ One published instance of one mesh — the id every hover/select observation is addressed by,
/// beside the topology target it stands for.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpMeshInstance {
    pub id: String,
    pub mesh_id: String,
    pub interaction_id: String,
}

/// 🌍️ One `World3d` surface's whole published payload: where it sits on the page, what it publishes,
/// and what the producer says is hovered and selected in it.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpMeshSurface {
    pub surface_id: String,
    pub pane_id: Option<String>,
    pub rect: [f32; 4],
    pub meshes: Vec<DumpMesh>,
    pub instances: Vec<DumpMeshInstance>,
    /// 🏷️ How many published meshes carry each role — the row a fixture's own `delivery.meshRoles`
    /// is compared against.
    pub roles: std::collections::BTreeMap<String, usize>,
    /// 📦️ The union of every published mesh's bounds, which is what a camera fit must frame.
    pub bbox_min: Option<[f64; 3]>,
    pub bbox_max: Option<[f64; 3]>,
    /// 🎥️ The WIRE camera — what the guest last published in `World3dScene.camera_json`.
    pub camera: Option<Value>,
    /// 🎥️ The LIVE orbit — what this surface is actually looking through THIS frame, in the same
    /// `setCamera` shape as `camera` so the two are directly diffable, plus the projection family
    /// the orbit is in.
    ///
    /// 🩸️ Without it the dump answers the wire camera for both halves, so every local camera move —
    /// an orbit, a wheel, a `Projection` pane switch — is invisible to a probe: a switch to
    /// `Orthographic` reported the delivered perspective pose, unchanged, and looked like a dead
    /// control (`📓️w8b-orthographic-camera-and-3d-parity.md` §7.5).
    pub live_camera: Option<Value>,
    pub selected: Vec<String>,
    pub hovered: Option<String>,
}

/// 🧊️ The wire shape `dumpMeshStats()` answers. Every live window is walked when no window is named,
/// because a mode layout paints its preview into a pane that is never the largest one.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpMeshStats {
    pub window_id: Option<String>,
    pub window_ids: Vec<String>,
    pub surfaces: Vec<DumpMeshSurface>,
}
//#endregion 🔬️IntrospectionTypes

//#region 🔬️IntrospectionPathGrammar
/// 🏷️ The wire-format `type` tag for `node` — MUST byte-match `UiNode`'s own `#[serde(tag =
/// "type", rename_all = "camelCase")]` so a path segment built from this is comparable to the
/// React side's `data-ui-path` (built from the same serialized `UiNode` tree — see this ticket's
/// own path-grammar spec, shared verbatim between both sides). Exhaustive match: a new `UiNode`
/// variant fails to compile here until wired, same discipline as `UiNode::presence`'s own match.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn ui_node_kind_tag(node: &UiNode) -> &'static str {
    match node {
        UiNode::Stack(_) => "stack",
        UiNode::Text(_) => "text",
        UiNode::Button(_) => "button",
        UiNode::Separator(_) => "separator",
        UiNode::Input(_) => "input",
        UiNode::Select(_) => "select",
        UiNode::Toggle(_) => "toggle",
        UiNode::KeyValue(_) => "keyValue",
        UiNode::Slider(_) => "slider",
        UiNode::NumberStepper(_) => "numberStepper",
        UiNode::Ring(_) => "ring",
        UiNode::IconSelect(_) => "iconSelect",
        UiNode::Progress(_) => "progress",
        UiNode::Field(_) => "field",
        UiNode::Section(_) => "section",
        UiNode::Group(_) => "group",
        UiNode::Tree(_) => "tree",
        UiNode::Image(_) => "image",
        UiNode::ComponentScene(_) => "componentScene",
        UiNode::ExternalSlot(_) => "externalSlot",
    }
}

/// 🔑️ `node`'s own declared identity field, if it has one — mirrors `ui_wgpu`'s private
/// `reconcile::explicit_id` (same variant→field mapping, the one `NodeKey::Explicit` itself keys
/// retained children by, so this names the exact same identity the retained tree already
/// reconciles against) since that function isn't part of `ui_wgpu`'s public surface.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
fn ui_node_declared_id(node: &UiNode) -> Option<&str> {
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
        UiNode::Progress(n) => Some(n.id.as_str()),
        UiNode::Image(n) => Some(n.id.as_str()),
        UiNode::ComponentScene(n) => Some(n.surface_id.as_str()),
        UiNode::ExternalSlot(n) => Some(n.body_key.as_str()),
        UiNode::Text(_) | UiNode::Separator(_) | UiNode::KeyValue(_) | UiNode::Tree(_) => None,
    }
}

/// 🧭️ One path segment: `${kind}[${i}]` or `${kind}[${i}]#${id}` when `node` carries a non-empty
/// declared id — the exact grammar the React side's `data-ui-path` mirrors.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn ui_node_path_segment(node: &UiNode, sibling_index: usize) -> String {
    let kind = ui_node_kind_tag(node);
    match ui_node_declared_id(node) {
        Some(id) if !id.is_empty() => format!("{kind}[{sibling_index}]#{id}"),
        _ => format!("{kind}[{sibling_index}]"),
    }
}
//#endregion 🔬️IntrospectionPathGrammar

//#region 🔬️IntrospectionVisualFields
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
fn rgba_array(color: semio_framework_ui_theme::Rgba) -> [f32; 4] {
    [color.r, color.g, color.b, color.a]
}

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
fn dim(color: semio_framework_ui_theme::Rgba, disabled: bool) -> semio_framework_ui_theme::Rgba {
    if disabled {
        color.with_alpha(color.a * 0.5)
    } else {
        color
    }
}

/// 🎨️ Best-effort `(text, color, bg, fontSize)` per `UiNode` kind, read straight off `theme`'s
/// already-`pub` fields (or the re-exported `ui_wgpu::wgpu::item_bg`/`item_text` helpers
/// `paint::paint_button`/`paint_toggle` themselves call) rather than duplicating `paint`'s private
/// per-widget geometry — `null` wherever a kind genuinely carries no single rendered text/color
/// (e.g. `KeyValue`'s multiple entries, `Slider`'s numeric value, any purely-visual kind). Colors
/// are `Theme`'s own LINEAR-space floats (see `Rgba::from_srgb8`), NOT sRGB 0..255 — a caller
/// comparing these against the DOM's `rgb()` CSS colors must gamma-correct first; a known,
/// documented unit mismatch, not a bug. `fontWeight` is always `None`: this renderer's `Theme`/
/// `paint` layer has no font-weight concept at all (only `UiTextNode.emphasize`, which changes
/// size+color, never weight) — a genuine gap, not a guess.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
fn dump_visual_fields(node: &UiNode, theme: &Theme, hovered: bool) -> (Option<String>, Option<[f32; 4]>, Option<[f32; 4]>, Option<f32>) {
    match node {
        UiNode::Stack(stack) => {
            if stack.activate.is_some() {
                let bg = if hovered { theme.button_hover } else { theme.panel };
                (None, None, Some(rgba_array(bg)), None)
            } else {
                (None, None, None, None)
            }
        }
        UiNode::Text(text) => {
            let emphasize = text.emphasize.unwrap_or(false);
            let color = if emphasize { theme.text } else { theme.text_muted };
            let size = if emphasize { theme.font_size_emphasized } else { theme.font_size_body };
            (Some(text.value.as_str().to_string()), Some(rgba_array(color)), None, Some(size))
        }
        UiNode::Button(button) => {
            let disabled = button.presence.state == UiState::Disabled;
            let color = dim(semio_framework_ui_wgpu_layout::chrome::item_text(theme, false, hovered), disabled);
            let bg = dim(semio_framework_ui_wgpu_layout::chrome::item_bg(theme, false, hovered), disabled);
            (Some(button.label.as_str().to_string()), Some(rgba_array(color)), Some(rgba_array(bg)), Some(theme.font_size_body))
        }
        UiNode::Separator(_) => (None, Some(rgba_array(theme.separator)), None, None),
        UiNode::Input(input) => {
            let color = if input.value.is_empty() { theme.text_muted } else { theme.text };
            (Some(input.value.clone()), Some(rgba_array(color)), Some(rgba_array(theme.input_bg)), Some(theme.font_size_body))
        }
        UiNode::Select(select) => {
            let label = select.items.iter().find(|item| item.value == select.value).map(|item| item.label.as_str().to_string()).or_else(|| select.placeholder.as_ref().map(|label| label.as_str().to_string()));
            let bg = if hovered { theme.button_hover } else { theme.input_bg };
            (label, Some(rgba_array(theme.text)), Some(rgba_array(bg)), Some(theme.font_size_body))
        }
        UiNode::Toggle(toggle) => {
            let pressed = toggle.presence.selected;
            let color = semio_framework_ui_wgpu_layout::chrome::item_text(theme, pressed, hovered);
            let bg = semio_framework_ui_wgpu_layout::chrome::item_bg(theme, pressed, hovered);
            let font_size = toggle.text.is_some().then_some(theme.font_size_body);
            (toggle.text.as_ref().map(|text| text.as_str().to_string()), Some(rgba_array(color)), Some(rgba_array(bg)), font_size)
        }
        UiNode::KeyValue(_) => (None, None, None, None),
        UiNode::Slider(_) => (None, None, None, None),
        UiNode::NumberStepper(stepper) => {
            let (text, color) = if stepper.uniform { (format!("{:.3}", stepper.value), theme.text) } else { (semio_framework_ui_declaration::ui::UI_INSPECTOR_MIXED_PLACEHOLDER.to_string(), theme.text_muted) };
            (Some(text), Some(rgba_array(color)), Some(rgba_array(theme.input_bg)), Some(theme.font_size_body))
        }
        UiNode::Ring(_) => (None, None, None, None),
        UiNode::IconSelect(select) => (Some(select.value.clone()), Some(rgba_array(theme.text)), None, Some(theme.font_size_body)),
        UiNode::Progress(progress) => (Some(progress.value_text.as_str().to_string()), Some(rgba_array(theme.progress)), Some(rgba_array(theme.separator)), None),
        UiNode::Field(field) => (Some(field.label.as_str().to_string()), Some(rgba_array(theme.text_muted)), None, Some(theme.font_size_small)),
        UiNode::Section(section) => match &section.label {
            Some(label) => (Some(label.as_str().to_string()), Some(rgba_array(theme.text)), None, Some(theme.font_size_body)),
            None => (None, None, None, None),
        },
        UiNode::Group(group) => (Some(group.label.as_str().to_string()), Some(rgba_array(theme.text)), None, Some(theme.font_size_body)),
        UiNode::Tree(_) => (None, None, None, None),
        UiNode::Image(image) => (image.alt.as_ref().map(|alt| alt.as_str().to_string()), None, None, None),
        UiNode::ComponentScene(_) => (None, None, None, None),
        UiNode::ExternalSlot(_) => (None, None, None, None),
    }
}
//#endregion 🔬️IntrospectionVisualFields

//#region 🔬️IntrospectionWalk
/// 🖱️ Same authored-hover-folds-into-live-hover rule `paint::paint_node` applies (private to
/// `ui_wgpu::wgpu::paint`, so re-derived here from the same two already-`pub` inputs it reads):
/// `presence.hover` counts as hovered too, unless the node is disabled.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
fn effective_hovered(node: &semio_framework_ui_retained_tree::tree::Node, presence_hover: bool, disabled: bool) -> bool {
    let live = node.flags.contains(semio_framework_ui_retained_tree::tree::NodeFlags::HOVERED);
    if disabled {
        live
    } else {
        live || presence_hover
    }
}

/// 🚶️ Depth-first walk mirroring `paint::paint_node`'s own recursion exactly (same `tree.children`
/// order, same parent-relative-`LayoutBucket`-offset accumulation into absolute `(origin_x +
/// node.layout.x, origin_y + node.layout.y)`), building one `DumpNode` per visited node and
/// recording the first node found with `NodeFlags::FOCUSED` set as `focus_path`.
///
/// 📐️ `Node::layout` is the immediate-mode bucket and stays zero on the retained path; the paint
/// walk consumes the double-buffered `mounted_layout`, so a probe reading the former reports an
/// unlaid-out tree even when layout published (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[allow(clippy::too_many_arguments, reason = "one arg per walk-state accumulator; mirrors paint_node's own equally-wide signature")]
pub fn walk_dump(tree: &semio_framework_ui_retained_tree::tree::UiTree, id: NodeId, origin_x: f32, origin_y: f32, parent_path: &str, sibling_index: usize, theme: &Theme, focus_path: &mut Option<String>, nodes: &mut Vec<DumpNode>) {
    let Some(node) = tree.node(id) else { return };
    let ui_node = &node.spec.0;
    let presence = ui_node.presence();
    let segment = ui_node_path_segment(ui_node, sibling_index);
    let path = if parent_path.is_empty() { segment } else { format!("{parent_path}/{segment}") };

    let (layout_x, layout_y, layout_w, layout_h) = tree.mounted_layout(id).unwrap_or((node.layout.x, node.layout.y, node.layout.width, node.layout.height));
    let abs_x = origin_x + layout_x;
    let abs_y = origin_y + layout_y;
    let disabled = presence.state == UiState::Disabled;
    let hovered = effective_hovered(node, presence.hover, disabled);
    if focus_path.is_none() && node.flags.contains(semio_framework_ui_retained_tree::tree::NodeFlags::FOCUSED) {
        *focus_path = Some(path.clone());
    }

    let (text, color, bg, font_size) = dump_visual_fields(ui_node, theme, hovered);
    nodes.push(DumpNode {
        path: path.clone(),
        kind: ui_node_kind_tag(ui_node),
        rect: [abs_x, abs_y, layout_w, layout_h],
        text,
        color,
        bg,
        font_size,
        font_weight: None,
        visible: presence.visible(),
        state: DumpNodeState { hovered, disabled, selected: presence.selected },
    });

    for (index, child) in tree.children(id).enumerate() {
        walk_dump(tree, child, abs_x, abs_y, &path, index, theme, focus_path, nodes);
    }
}
//#endregion 🔬️IntrospectionWalk

//#region 🔬️IntrospectionWindowSelection
/// 🪟️ Which window a dump answers for. `UI_ENGINE` tracks one entry per live window — the dock's
/// own panes (`procedural-main`, `procedural-preview`, …), `"spawned"` for a studio surface, and one
/// per open floating panel tab — so an app whose authored mode layout places more than one window
/// has more than one candidate. A caller that names a window gets exactly that window (and `None`
/// when it is not live, so a probe can tell "not laid out" from "laid out and empty"); a caller that
/// names none keeps the previous rule, the largest last-known viewport area. The optional argument
/// is what makes the smaller pane of a two-pane mode layout measurable at all — the zero-arg export
/// could only ever answer for the larger one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub(super) fn dump_window_id(engine: &semio_framework_ui_wgpu_engine::Ui, requested: Option<&str>) -> Option<String> {
    match requested {
        Some(id) if !id.is_empty() => engine.window_ids().find(|live| *live == id).map(str::to_string),
        _ => engine.window_ids().filter_map(|id| engine.viewport(id).map(|(w, h)| (id.to_string(), w * h))).max_by(|a, b| a.1.total_cmp(&b.1)).map(|(id, _)| id),
    }
}

/// 🪟️ Every window this engine currently tracks, so a dump that answers for one window can still
/// name the others a caller could ask for.
pub fn dump_window_ids(engine: &semio_framework_ui_wgpu_engine::Ui) -> Vec<String> {
    engine.window_ids().map(str::to_string).collect()
}
//#endregion 🔬️IntrospectionWindowSelection
//#endregion 🔬️Introspection
