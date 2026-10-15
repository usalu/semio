//! 🔬️ Introspection dumps and their wasm exports.
//!
//! The structure, frame, mesh and chrome dump types and builders, the accessibility dump and the wasm exports.

#[cfg(any(target_arch = "wasm32", test))]
use super::chrome_ledger::project_chrome_dump;
#[cfg(target_arch = "wasm32")]
use super::chrome_ledger::CHROME_LEDGER;
#[cfg(test)]
use super::chrome_ledger::{
    accessibility_window_is_visible, begin_accessibility_visible_documents, chrome_action_args,
    chrome_hit_row, ledger_publish_hits, ledger_publish_surfaces, ledger_push_action,
    note_accessibility_visible_document, note_chrome_accessibility,
    publish_accessibility_visible_documents, world_camera_ledger_row, ChromeLedger,
    CHROME_ACTION_ARGS_BYTES, CHROME_ACTION_CAPACITY, WORLD_CAMERA_LEDGER,
};
use super::chrome_ledger::{
    ACCESSIBILITY_VISIBLE_WINDOWS, CHROME_ACCESSIBILITY, SHELL_CHROME_ACCESSIBILITY_WINDOW_ID,
};
use super::ink_and_vfs_focus::focused_vfs_control;
#[cfg(any(target_arch = "wasm32", test))]
use super::mesh_and_board2d_stats::build_board2d_dump;
#[cfg(target_arch = "wasm32")]
use super::mesh_and_board2d_stats::build_mesh_stats;
#[cfg(test)]
use super::mesh_and_board2d_stats::{
    board2d_surface, mesh_stats_for_scene, walk_board2d, walk_mesh_stats,
};
#[cfg(target_arch = "wasm32")]
use super::retained_ui_engine::UI_ENGINE;
use super::text_and_table_focus::{
    focused_table_editable_text_address, focused_table_editable_text_draft, focused_text_editor,
    FOCUSED_TABLE_STEPPER,
};
#[cfg(test)]
use crate::element_contracts::fixture_scene_pointer_target;
#[cfg(any(target_arch = "wasm32", test))]
use serde_json::Value;
#[cfg(any(target_arch = "wasm32", test))]
use ui_wgpu::wgpu::UiState;
#[cfg(test)]
use ui_wgpu::wgpu::{ActionDescriptor, UiPresence};
use ui_wgpu::wgpu::{NodeId, Rect, Theme, UiNode};

//#region 🔬️Introspection
/** 🔬️ Structural + frame-stats dump for the wgpu↔React UI-parity headless test harness (see
 * `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`): walks the SAME `UI_ENGINE` retained façade
 * `render_ui_node` (above) already drives, so every dump reflects exactly what was last laid
 * out/painted — never a second, independent measurement pass. Deliberately scoped to ONE window's
 * content tree: shell chrome/navbar/footer/dock are rendered by this crate's own immediate-mode
 * widgets code directly into the composited canvas frame, never through `UI_ENGINE` at all, so
 * they're structurally unreachable from here — out of scope by construction, not by filtering.
 * Exported to JS the same way `semioWgpuMount`/`uploadIconAtlas` already are (bare
 * `#[wasm_bindgen(js_name = "...")]` free functions) — no new loading path invented for these two;
 * see the `🔬️IntrospectionExports` sub-region below for exactly how they end up reachable. */

//#region 🔬️IntrospectionTypes
// 🔬️ Live for real via the wasm32 `#[wasm_bindgen]` exports below (`🔬️IntrospectionExports`);
// also gated `test` since `introspection_tests` (bottom of this region) exercises this whole
// dump pipeline natively — neither cfg alone covers both compilations.
#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct DumpViewport {
    pub(super) w: f32,
    pub(super) h: f32,
    dpr: f32,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(serde::Serialize)]
struct DumpNodeState {
    hovered: bool,
    disabled: bool,
    pub(super) selected: bool,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpNode {
    path: String,
    pub(super) kind: &'static str,
    pub(super) rect: [f32; 4],
    pub(super) text: Option<String>,
    color: Option<[f32; 4]>,
    bg: Option<[f32; 4]>,
    font_size: Option<f32>,
    font_weight: Option<u32>,
    pub(super) visible: bool,
    pub(super) state: DumpNodeState,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpStructure {
    pub(super) viewport: DumpViewport,
    /// 🪟️ The window this dump answers for, and every window the engine could answer for — so a
    /// caller reading a two-pane mode layout can see both panes rather than guessing which one the
    /// zero-arg selection picked.
    pub(super) window_id: Option<String>,
    window_ids: Vec<String>,
    focus_path: Option<String>,
    pub(super) nodes: Vec<DumpNode>,
}

/// ♿️ One window's accessibility tree in reading order. `nodes` is `ui_contract`'s own
/// `AccessibilityProjectionNode` verbatim — the host mirror consumes the CONTRACT's shape, not a
/// renderer-private one, so a second renderer publishing the same tree needs no new consumer.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpAccessibilityWindow {
    pub(super) window_id: String,
    pub(super) window_generation: u64,
    pub(super) nodes: Vec<ui_contract::AccessibilityProjectionNode>,
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
struct DumpAccessibility {
    pub(super) window_id: Option<String>,
    window_ids: Vec<String>,
    pub(super) windows: Vec<DumpAccessibilityWindow>,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpFrameStats {
    pub(super) window_id: Option<String>,
    window_ids: Vec<String>,
    draw_calls: usize,
    quad_count: usize,
    glyph_count: usize,
    /// 🌍️ The 3d half — a `World3d` window paints `ScenePass3d`s and no quads at all, so these are
    /// the only numbers that can say whether its solid reached the frame.
    scene_passes: usize,
    scene_draws: usize,
    scene_instances: usize,
    frame_latency: crate::frame_latency::FrameLatencySnapshot,
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
#[cfg(any(target_arch = "wasm32", test))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DumpMesh {
    pub(super) id: String,
    /// 🏷️ The producer's own `role` stamp (`solid`/`wire`/`point`/`vector`), or `(unstamped)` for a
    /// producer that publishes none — an absence a reader must see rather than have defaulted away.
    pub(super) role: String,
    pub(super) indices: usize,
    pub(super) positions: usize,
    pub(super) edge_positions: usize,
    /// 📦️ The mesh's own axis-aligned bounds over `positions`, falling back to `edgePositions` for a
    /// wire body that carries no vertices at all. `None` when the entry carries neither.
    pub(super) bbox_min: Option<[f64; 3]>,
    pub(super) bbox_max: Option<[f64; 3]>,
}

/// 🎯️ One published instance of one mesh — the id every hover/select observation is addressed by,
/// beside the topology target it stands for.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DumpMeshInstance {
    pub(super) id: String,
    pub(super) mesh_id: String,
    pub(super) interaction_id: String,
}

/// 🌍️ One `World3d` surface's whole published payload: where it sits on the page, what it publishes,
/// and what the producer says is hovered and selected in it.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DumpMeshSurface {
    pub(super) surface_id: String,
    pub(super) pane_id: Option<String>,
    pub(super) rect: [f32; 4],
    pub(super) meshes: Vec<DumpMesh>,
    pub(super) instances: Vec<DumpMeshInstance>,
    /// 🏷️ How many published meshes carry each role — the row a fixture's own `delivery.meshRoles`
    /// is compared against.
    pub(super) roles: std::collections::BTreeMap<String, usize>,
    /// 📦️ The union of every published mesh's bounds, which is what a camera fit must frame.
    pub(super) bbox_min: Option<[f64; 3]>,
    pub(super) bbox_max: Option<[f64; 3]>,
    /// 🎥️ The WIRE camera — what the guest last published in `World3dScene.camera_json`.
    pub(super) camera: Option<Value>,
    /// 🎥️ The LIVE orbit — what this surface is actually looking through THIS frame, in the same
    /// `setCamera` shape as `camera` so the two are directly diffable, plus the projection family
    /// the orbit is in.
    ///
    /// 🩸️ Without it the dump answers the wire camera for both halves, so every local camera move —
    /// an orbit, a wheel, a `Projection` pane switch — is invisible to a probe: a switch to
    /// `Orthographic` reported the delivered perspective pose, unchanged, and looked like a dead
    /// control (`📓️w8b-orthographic-camera-and-3d-parity.md` §7.5).
    pub(super) live_camera: Option<Value>,
    pub(super) selected: Vec<String>,
    pub(super) hovered: Option<String>,
}

/// 🧊️ The wire shape `dumpMeshStats()` answers. Every live window is walked when no window is named,
/// because a mode layout paints its preview into a pane that is never the largest one.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DumpMeshStats {
    pub(super) window_id: Option<String>,
    pub(super) window_ids: Vec<String>,
    pub(super) surfaces: Vec<DumpMeshSurface>,
}

//#endregion 🔬️IntrospectionTypes

//#region 🔬️IntrospectionPathGrammar
/// 🏷️ The wire-format `type` tag for `node` — MUST byte-match `UiNode`'s own `#[serde(tag =
/// "type", rename_all = "camelCase")]` so a path segment built from this is comparable to the
/// React side's `data-ui-path` (built from the same serialized `UiNode` tree — see this ticket's
/// own path-grammar spec, shared verbatim between both sides). Exhaustive match: a new `UiNode`
/// variant fails to compile here until wired, same discipline as `UiNode::presence`'s own match.
#[cfg(any(target_arch = "wasm32", test))]
fn ui_node_kind_tag(node: &UiNode) -> &'static str {
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
#[cfg(any(target_arch = "wasm32", test))]
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
#[cfg(any(target_arch = "wasm32", test))]
fn ui_node_path_segment(node: &UiNode, sibling_index: usize) -> String {
    let kind = ui_node_kind_tag(node);
    match ui_node_declared_id(node) {
        Some(id) if !id.is_empty() => format!("{kind}[{sibling_index}]#{id}"),
        _ => format!("{kind}[{sibling_index}]"),
    }
}
//#endregion 🔬️IntrospectionPathGrammar

//#region 🔬️IntrospectionVisualFields
#[cfg(any(target_arch = "wasm32", test))]
fn rgba_array(color: ui_wgpu::wgpu::Rgba) -> [f32; 4] {
    [color.r, color.g, color.b, color.a]
}

#[cfg(any(target_arch = "wasm32", test))]
fn dim(color: ui_wgpu::wgpu::Rgba, disabled: bool) -> ui_wgpu::wgpu::Rgba {
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
#[cfg(any(target_arch = "wasm32", test))]
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
            let color = dim(ui_wgpu::wgpu::item_text(theme, false, hovered), disabled);
            let bg = dim(ui_wgpu::wgpu::item_bg(theme, false, hovered), disabled);
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
            let color = ui_wgpu::wgpu::item_text(theme, pressed, hovered);
            let bg = ui_wgpu::wgpu::item_bg(theme, pressed, hovered);
            let font_size = toggle.text.is_some().then_some(theme.font_size_body);
            (toggle.text.as_ref().map(|text| text.as_str().to_string()), Some(rgba_array(color)), Some(rgba_array(bg)), font_size)
        }
        UiNode::KeyValue(_) => (None, None, None, None),
        UiNode::Slider(_) => (None, None, None, None),
        UiNode::NumberStepper(stepper) => {
            let (text, color) = if stepper.uniform { (format!("{:.3}", stepper.value), theme.text) } else { (ui_wgpu::wgpu::UI_INSPECTOR_MIXED_PLACEHOLDER.to_string(), theme.text_muted) };
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
#[cfg(any(target_arch = "wasm32", test))]
fn effective_hovered(node: &ui_wgpu::wgpu::Node, presence_hover: bool, disabled: bool) -> bool {
    let live = node.flags.contains(ui_wgpu::wgpu::NodeFlags::HOVERED);
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
#[cfg(any(target_arch = "wasm32", test))]
#[allow(clippy::too_many_arguments, reason = "one arg per walk-state accumulator; mirrors paint_node's own equally-wide signature")]
fn walk_dump(tree: &ui_wgpu::wgpu::UiTree, id: NodeId, origin_x: f32, origin_y: f32, parent_path: &str, sibling_index: usize, theme: &Theme, focus_path: &mut Option<String>, nodes: &mut Vec<DumpNode>) {
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
    if focus_path.is_none() && node.flags.contains(ui_wgpu::wgpu::NodeFlags::FOCUSED) {
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
#[cfg(any(target_arch = "wasm32", test))]
fn dump_window_id(engine: &ui_wgpu::wgpu::Ui, requested: Option<&str>) -> Option<String> {
    match requested {
        Some(id) if !id.is_empty() => engine.window_ids().find(|live| *live == id).map(str::to_string),
        _ => engine.window_ids().filter_map(|id| engine.viewport(id).map(|(w, h)| (id.to_string(), w * h))).max_by(|a, b| a.1.total_cmp(&b.1)).map(|(id, _)| id),
    }
}

/// 🪟️ Every window this engine currently tracks, so a dump that answers for one window can still
/// name the others a caller could ask for.
pub(super) fn dump_window_ids(engine: &ui_wgpu::wgpu::Ui) -> Vec<String> {
    engine.window_ids().map(str::to_string).collect()
}
//#endregion 🔬️IntrospectionWindowSelection

//#region 🔬️IntrospectionBuilders
#[cfg(target_arch = "wasm32")]
fn build_structure_dump(engine: &ui_wgpu::wgpu::Ui, dpr: f32, requested: Option<&str>) -> DumpStructure {
    let window_ids = dump_window_ids(engine);
    let Some(window_id) = dump_window_id(engine, requested) else {
        return DumpStructure { viewport: DumpViewport { w: 0.0, h: 0.0, dpr }, window_id: None, window_ids, focus_path: None, nodes: Vec::new() };
    };
    let (w, h) = engine.viewport(&window_id).unwrap_or((0.0, 0.0));
    let theme = engine.theme();
    let mut nodes = Vec::new();
    let mut focus_path = None;
    if let Some(tree) = engine.tree(&window_id) {
        if let Some(root) = tree.root {
            walk_dump(tree, root, 0.0, 0.0, "", 0, &theme, &mut focus_path, &mut nodes);
        }
    }
    DumpStructure { viewport: DumpViewport { w, h, dpr }, window_id: Some(window_id), window_ids, focus_path, nodes }
}

/// 🖼️ `drawCalls` = number of non-empty `DrawLayer`s (a reasonable, documented proxy — the
/// retained `DrawList` doesn't itself count submitted GPU draw calls anywhere, and each non-empty
/// layer submits as very few real draw calls). `quadCount` = every `UiInstance` across every layer
/// (glyphs included — a glyph is itself one `UiInstance`, see `draw::KIND_GLYPH`); `glyphCount` is
/// the `KIND_GLYPH` subset, for boot-triage (a booted-but-blank canvas has 0 of everything; text
/// that silently failed to shape has quads but 0 glyphs).
#[cfg(any(target_arch = "wasm32", test))]
fn layer_is_nonempty(layer: &ui_wgpu::wgpu::draw::DrawLayer) -> bool {
    !layer.ui_instances.is_empty() || !layer.raster_instances.is_empty() || !layer.vector_vertices.is_empty() || !layer.overlay_ui_instances.is_empty() || !layer.overlay_vector_vertices.is_empty()
}

#[cfg(any(target_arch = "wasm32", test))]
fn is_glyph_instance(instance: &ui_wgpu::wgpu::draw::UiInstance) -> bool {
    instance.params[2] == ui_wgpu::wgpu::draw::KIND_GLYPH
}

struct TableStepperAccessibilityEntry {
    pub(super) scene_node: NodeId,
    pub(super) host_id: String,
    pub(super) depth: usize,
    pub(super) cell: crate::scenes::TableStepperAccessibilityCell,
}

pub(super) fn scene_virtual_accessibility_node_id(key: &str) -> u64 {
    let hash = key.as_bytes().iter().fold(0xcbf29ce484222325_u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3));
    (1_u64 << 63) | (hash & !(1_u64 << 63))
}

fn collect_table_stepper_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, depth: usize, entries: &mut Vec<TableStepperAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::Table && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            entries.extend(crate::scenes::accepted_table_stepper_accessibility_cells(&scene.host_id).into_iter().map(|cell| TableStepperAccessibilityEntry { scene_node: node, host_id: scene.host_id.clone(), depth: depth.saturating_add(1), cell }));
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_table_stepper_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub(super) fn table_stepper_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree) -> Vec<TableStepperAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_table_stepper_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

fn append_table_stepper_accessibility_nodes(window_id: &str, tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let focus = FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().map(|focus| (focus.window_id.clone(), focus.node, focus.host_id.clone(), focus.row_id.clone(), focus.column_id.clone())));
    for entry in table_stepper_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let focused = focus.as_ref().is_some_and(|focus| focus.0 == window_id && focus.1 == entry.scene_node && focus.2 == entry.host_id && focus.3 == entry.cell.target.row_id && focus.4 == entry.cell.target.column_id);
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.cell.key),
            key: entry.cell.key,
            role: "spinbutton".into(),
            depth: entry.depth,
            label: (!entry.cell.label.trim().is_empty()).then_some(entry.cell.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.cell.rect.x, entry.cell.rect.y, entry.cell.rect.w, entry.cell.rect.h]),
            value_min: Some(entry.cell.min),
            value_max: Some(entry.cell.max),
            value_now: Some(entry.cell.value),
            value_text: Some(entry.cell.value.to_string()),
            busy: false,
        });
    }
}

struct TableEditableTextAccessibilityEntry {
    pub(super) scene_node: NodeId,
    pub(super) host_id: String,
    pub(super) depth: usize,
    pub(super) cell: crate::scenes::TableEditableTextAccessibilityCell,
}

fn collect_table_editable_text_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, depth: usize, entries: &mut Vec<TableEditableTextAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::Table && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            entries.extend(crate::scenes::accepted_table_editable_text_accessibility_cells(&scene.host_id).into_iter().map(|cell| TableEditableTextAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                cell,
            }));
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_table_editable_text_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub(super) fn table_editable_text_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree) -> Vec<TableEditableTextAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_table_editable_text_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

fn append_table_editable_text_accessibility_nodes(window_id: &str, tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let focus = focused_table_editable_text_address();
    for entry in table_editable_text_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let focused = focus.as_ref().is_some_and(|focus| focus.0 == window_id && focus.2 == entry.scene_node && focus.3 == entry.host_id && focus.4 == entry.cell.target.row_id && focus.5 == entry.cell.target.column_id);
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.cell.key),
            key: entry.cell.key,
            role: "textbox".into(),
            depth: entry.depth,
            label: (!entry.cell.label.trim().is_empty()).then_some(entry.cell.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: true,
            multiline: true,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.cell.rect.x, entry.cell.rect.y, entry.cell.rect.w, entry.cell.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: Some(focused_table_editable_text_draft(window_id, entry.scene_node, &entry.host_id, &entry.cell.target.row_id, &entry.cell.target.column_id).unwrap_or(entry.cell.target.value)),
            busy: focus.as_ref().is_some_and(|focus| focus.0 == window_id && focus.2 == entry.scene_node && focus.3 == entry.host_id && focus.4 == entry.cell.target.row_id && focus.5 == entry.cell.target.column_id && focus.8.is_some()),
        });
    }
}

struct TableButtonAccessibilityEntry {
    pub(super) scene_node: NodeId,
    pub(super) host_id: String,
    pub(super) depth: usize,
    pub(super) cell: crate::scenes::TableButtonAccessibilityCell,
}

fn collect_table_button_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, depth: usize, entries: &mut Vec<TableButtonAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::Table && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            entries.extend(crate::scenes::accepted_table_button_accessibility_cells(&scene.host_id).into_iter().map(|cell| TableButtonAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                cell,
            }));
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_table_button_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub(super) fn table_button_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree) -> Vec<TableButtonAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_table_button_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

fn append_table_button_accessibility_nodes(tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in table_button_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.cell.key),
            key: entry.cell.key,
            role: "button".into(),
            depth: entry.depth,
            label: (!entry.cell.label.trim().is_empty()).then_some(entry.cell.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.cell.rect.x, entry.cell.rect.y, entry.cell.rect.w, entry.cell.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

struct VfsAccessibilityEntry {
    pub(super) scene_node: NodeId,
    pub(super) host_id: String,
    pub(super) depth: usize,
    pub(super) control: crate::scenes::VfsAccessibilityControl,
}

fn collect_vfs_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, depth: usize, entries: &mut Vec<VfsAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            entries.extend(crate::scenes::accepted_vfs_accessibility_controls(&scene.host_id).into_iter().map(|control| VfsAccessibilityEntry { scene_node: node, host_id: scene.host_id.clone(), depth: depth.saturating_add(1), control }));
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_vfs_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub(super) fn vfs_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree) -> Vec<VfsAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_vfs_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

fn append_vfs_accessibility_nodes(window_id: &str, tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let focus = focused_vfs_control();
    for entry in vfs_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let focused = focus.as_ref().is_some_and(|focus| focus.0 == window_id && focus.2 == entry.scene_node && focus.3 == entry.host_id && focus.4.row_id == entry.control.row_id && focus.4.kind == entry.control.kind);
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.control.key),
            key: entry.control.key,
            role: "button".into(),
            depth: entry.depth,
            label: Some(entry.control.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused,
            checked: None,
            pressed: None,
            selected: None,
            expanded: entry.control.expanded,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.control.rect.x, entry.control.rect.y, entry.control.rect.w, entry.control.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

struct BlockListAccessibilityEntry {
    pub(super) scene_node: NodeId,
    pub(super) host_id: String,
    pub(super) depth: usize,
    pub(super) control: crate::scenes::BlockListAccessibilityControl,
}

fn collect_block_list_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, depth: usize, entries: &mut Vec<BlockListAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::BlockList && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            entries.extend(crate::scenes::accepted_block_list_accessibility_controls(&scene.host_id).into_iter().map(|control| BlockListAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                control,
            }));
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_block_list_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub(super) fn block_list_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree) -> Vec<BlockListAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_block_list_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

fn append_block_list_accessibility_nodes(tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in block_list_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.control.key),
            key: entry.control.key,
            role: "button".into(),
            depth: entry.depth,
            label: Some(entry.control.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.control.rect.x, entry.control.rect.y, entry.control.rect.w, entry.control.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

struct EventFeedAccessibilityEntry {
    pub(super) scene_node: NodeId,
    pub(super) host_id: String,
    pub(super) depth: usize,
    pub(super) control: crate::scenes::EventFeedAccessibilityControl,
}

fn collect_event_feed_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, depth: usize, entries: &mut Vec<EventFeedAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::EventFeed && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            entries.extend(crate::scenes::accepted_event_feed_accessibility_controls(&scene.host_id).into_iter().map(|control| EventFeedAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                control,
            }));
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_event_feed_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub(super) fn event_feed_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree) -> Vec<EventFeedAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_event_feed_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

fn collect_world3d_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, depth: usize, entries: &mut Vec<(usize, crate::scenes::World3dAccessibilityControl)>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::World3d && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            entries.extend(crate::scenes::accepted_world3d_accessibility_controls(&scene.host_id).into_iter().map(|control| (depth.saturating_add(1), control)));
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_world3d_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

/// ♿️ Publishes the modelling overlays of every visible world surface (annotation list, scalar-field legend, mismatch status) as read-only nodes of the accessibility mirror;
/// the mismatch status is a polite live region so a screen reader announces it.
fn append_world3d_accessibility_nodes(tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_world3d_accessibility_entries(tree, root, 0, &mut entries);
    }
    for (depth, control) in entries {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&control.key),
            key: control.key,
            role: control.role.into(),
            depth: depth.saturating_add(usize::from(control.depth)),
            label: Some(control.label),
            description: None,
            live: ui_contract::liveness_name(if control.polite { ui_contract::Liveness::Polite } else { ui_contract::Liveness::Off }).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: false,
            tabbable: false,
            actionable: false,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: None,
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

fn append_event_feed_accessibility_nodes(tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in event_feed_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let actionable = entry.control.action.is_some();
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.control.key),
            key: entry.control.key,
            role: if actionable { "button" } else { "paragraph" }.into(),
            depth: entry.depth,
            label: Some(entry.control.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: actionable,
            tabbable: actionable,
            actionable,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.control.rect.x, entry.control.rect.y, entry.control.rect.w, entry.control.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

struct GraphTimelineAccessibilityEntry {
    pub(super) scene_node: NodeId,
    pub(super) host_id: String,
    pub(super) depth: usize,
    pub(super) control: crate::scenes::GraphTimelineAccessibilityControl,
}

fn collect_graph_timeline_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, depth: usize, entries: &mut Vec<GraphTimelineAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::GraphTimeline && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            entries.extend(crate::scenes::accepted_graph_timeline_accessibility_controls(&scene.host_id).into_iter().map(|control| GraphTimelineAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                control,
            }));
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_graph_timeline_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub(super) fn graph_timeline_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree) -> Vec<GraphTimelineAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_graph_timeline_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

fn append_graph_timeline_accessibility_nodes(tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in graph_timeline_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.control.key),
            key: entry.control.key,
            role: "button".into(),
            depth: entry.depth,
            label: Some(entry.control.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.control.rect.x, entry.control.rect.y, entry.control.rect.w, entry.control.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

struct TextEditorAccessibilityEntry {
    pub(super) scene_node: NodeId,
    pub(super) host_id: String,
    pub(super) key: String,
    pub(super) label: String,
    pub(super) depth: usize,
    pub(super) rect: Rect,
    pub(super) value: String,
    read_only: bool,
}

fn collect_text_editor_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree, node: NodeId, origin: (f32, f32), depth: usize, entries: &mut Vec<TextEditorAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    let (x, y, width, height) = tree.mounted_layout(node).unwrap_or((retained.layout.x, retained.layout.y, retained.layout.width, retained.layout.height));
    let absolute = (origin.0 + x, origin.1 + y);
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == ui_wgpu::wgpu::SurfaceKind::TextEditor && scene.presence.visible() && !crate::scenes::scene_host_retiring(&scene.host_id) {
            if let Some(value) = crate::engine_canvas::text_editor_accessibility_value(scene) {
                entries.push(TextEditorAccessibilityEntry {
                    scene_node: node,
                    host_id: scene.host_id.clone(),
                    key: format!("{}.editor", scene.host_id),
                    label: scene.text_editor.as_ref().and_then(|editor| editor.language.as_ref()).map_or_else(|| "Editor".into(), |language| format!("{language} editor")),
                    depth: depth.saturating_add(1),
                    rect: Rect::new(absolute.0, absolute.1, width, height),
                    value,
                    read_only: crate::engine_canvas::text_editor_is_read_only(scene),
                });
            }
        }
    }
    if depth >= ui_wgpu::wgpu::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_text_editor_accessibility_entries(tree, child, absolute, depth.saturating_add(1), entries);
    }
}

pub(super) fn text_editor_accessibility_entries(tree: &ui_wgpu::wgpu::UiTree) -> Vec<TextEditorAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_text_editor_accessibility_entries(tree, root, (0.0, 0.0), 0, &mut entries);
    }
    entries
}

fn append_text_editor_accessibility_nodes(window_id: &str, tree: &ui_wgpu::wgpu::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let focus = focused_text_editor();
    for entry in text_editor_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let focused = focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.node == entry.scene_node && focus.host_id == entry.host_id);
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.key),
            key: entry.key,
            role: "textbox".into(),
            depth: entry.depth,
            label: Some(entry.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: !entry.read_only,
            multiline: true,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.rect.x, entry.rect.y, entry.rect.w, entry.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: Some(entry.value),
            busy: false,
        });
    }
}

/// ♿️ The accessibility tree the app publishes: the ONE window a caller named, or — when it named
/// none — EVERY live window, each with its own tree, in the order the engine tracks them.
///
/// 🩸️ This used to reuse `build_structure_dump`'s "largest viewport" selection. That rule is right
/// for a diagnostic and wrong for the accessibility path: on generation3d it announced
/// `procedural-main` and silently dropped `procedural-preview` and both measure panels, so 26 of the
/// app's 64 announced nodes were unreachable to a reader (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// ♿️ …and the chrome around those windows, which owns no retained tree of its own. Announced
/// only when the caller named no single window (or named the chrome itself), the same rule the
/// per-window filter above follows.
pub(super) fn build_accessibility_dump(engine: &ui_wgpu::wgpu::Ui, requested: Option<&str>) -> DumpAccessibility {
    let window_ids = dump_window_ids(engine);
    let announced: Vec<String> = match requested.filter(|id| !id.is_empty()) {
        Some(id) => window_ids.iter().filter(|live| live.as_str() == id).cloned().collect(),
        None => ACCESSIBILITY_VISIBLE_WINDOWS.borrow().published.iter().filter(|visible| window_ids.iter().any(|live| live == *visible)).cloned().collect(),
    };
    let mut windows: Vec<DumpAccessibilityWindow> = announced
        .into_iter()
        .map(|window_id| {
            let mut nodes = engine.tree(&window_id).map(ui_wgpu::wgpu::accessibility::accessibility_projection).unwrap_or_default();
            if let Some(tree) = engine.tree(&window_id) {
                append_table_stepper_accessibility_nodes(&window_id, tree, &mut nodes);
                append_table_editable_text_accessibility_nodes(&window_id, tree, &mut nodes);
                append_table_button_accessibility_nodes(tree, &mut nodes);
                append_vfs_accessibility_nodes(&window_id, tree, &mut nodes);
                append_block_list_accessibility_nodes(tree, &mut nodes);
                append_event_feed_accessibility_nodes(tree, &mut nodes);
                append_world3d_accessibility_nodes(tree, &mut nodes);
                append_graph_timeline_accessibility_nodes(tree, &mut nodes);
                append_text_editor_accessibility_nodes(&window_id, tree, &mut nodes);
            }
            let window_generation = engine.surface_generation(&window_id).unwrap_or_default();
            DumpAccessibilityWindow { window_id, window_generation, nodes }
        })
        .collect();
    let chrome = CHROME_ACCESSIBILITY.borrow().clone();
    if !chrome.nodes.is_empty() && requested.filter(|id| !id.is_empty()).is_none_or(|id| id == SHELL_CHROME_ACCESSIBILITY_WINDOW_ID) {
        windows.push(DumpAccessibilityWindow { window_id: SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: chrome.generation, nodes: chrome.nodes });
    }
    DumpAccessibility { window_id: requested.filter(|id| !id.is_empty()).map(str::to_string), window_ids, windows }
}

/// 📊️ The production paint entry is `frame_into_step`, which appends into the CALLER's draw list
/// and never publishes into the window's own — so `draw_list` answers "empty" no matter how much
/// the window painted, which is why `drawCalls: 0` survived every earlier lane. The engine's own
/// per-window paint census measures the delta that paint appended, for either entry; the retained
/// `draw_list` is still read when it carries one (the `frame_step` path).
/// 🩸️ `layers > 0` alone disqualified a window whose whole paint IS its scene pass: the wgpu
/// preview appends one `push_scene_pass` and no new draw LAYER, so its real census
/// (`scene_passes 1, scene_draws 2, scene_instances 1`) was thrown away and the empty retained
/// `draw_list` answered zeros — the hexagonal column was on screen while `dumpFrameStats` said
/// nothing had been drawn (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(any(target_arch = "wasm32", test))]
fn build_frame_stats(engine: &ui_wgpu::wgpu::Ui, requested: Option<&str>) -> DumpFrameStats {
    let window_ids = dump_window_ids(engine);
    let Some(window_id) = dump_window_id(engine, requested) else {
        return DumpFrameStats { window_id: None, window_ids, draw_calls: 0, quad_count: 0, glyph_count: 0, scene_passes: 0, scene_draws: 0, scene_instances: 0, frame_latency: crate::frame_latency::snapshot() };
    };
    if let Some(census) = engine.paint_census(&window_id).filter(|census| census.layers > 0 || census.scene_passes > 0) {
        return DumpFrameStats {
            window_id: Some(window_id),
            window_ids,
            draw_calls: census.layers,
            quad_count: census.quads,
            glyph_count: census.glyphs,
            scene_passes: census.scene_passes,
            scene_draws: census.scene_draws,
            scene_instances: census.scene_instances,
            frame_latency: crate::frame_latency::snapshot(),
        };
    }
    let Some(draw) = engine.draw_list(&window_id) else {
        return DumpFrameStats { window_id: Some(window_id), window_ids, draw_calls: 0, quad_count: 0, glyph_count: 0, scene_passes: 0, scene_draws: 0, scene_instances: 0, frame_latency: crate::frame_latency::snapshot() };
    };
    let draw_calls = draw.layers.iter().filter(|layer| layer_is_nonempty(layer)).count();
    let all_instances = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter().chain(layer.overlay_ui_instances.iter()));
    let mut quad_count = 0usize;
    let mut glyph_count = 0usize;
    for instance in all_instances {
        quad_count += 1;
        if is_glyph_instance(instance) {
            glyph_count += 1;
        }
    }
    let scene_passes = draw.scene_passes.len();
    let scene_draws = draw.scene_passes.iter().map(|pass| pass.draws.len() + pass.translucent_draws.len() + pass.material_draws.len() + pass.textured_draws.len() + pass.line_draws.len()).sum();
    let scene_instances = draw
        .scene_passes
        .iter()
        .map(|pass| {
            pass.draws.iter().chain(pass.translucent_draws.iter()).map(|scene_draw| scene_draw.instances.len()).sum::<usize>()
                + pass.material_draws.iter().map(|scene_draw| scene_draw.instances.len()).sum::<usize>()
                + pass.textured_draws.iter().map(|scene_draw| scene_draw.instances.len()).sum::<usize>()
        })
        .sum();
    DumpFrameStats { window_id: Some(window_id), window_ids, draw_calls, quad_count, glyph_count, scene_passes, scene_draws, scene_instances, frame_latency: crate::frame_latency::snapshot() }
}

//#endregion 🔬️IntrospectionBuilders

//#region 🔬️IntrospectionExports
/// 📤️ `dumpStructure()`/`dumpFrameStats()` — reachable exactly like `semioWgpuMount`/
/// `uploadIconAtlas` already are: Trunk's dev-server boot glue (`framework/renderer/wgpu/js/
/// 🟦️.ts`) waits for `window.wasmBindings` then calls exports straight off it, so these land at
/// `window.wasmBindings.dumpStructure()`/`window.wasmBindings.dumpFrameStats()` there; the library
/// boot path (`framework/renderer/wgpu/index.ts`'s `bootFrameworkOsWgpu`) instead calls them on its
/// own dynamically-imported module object. No new loading path invented for either.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = dumpStructure)]
pub fn dump_structure(window_id: Option<String>) -> String {
    let dpr = web_sys::window().map(|window| window.device_pixel_ratio() as f32).unwrap_or(1.0);
    let dump = UI_ENGINE.with(|cell| build_structure_dump(&cell.borrow(), dpr, window_id.as_deref()));
    serde_json::to_string(&dump).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = dumpFrameStats)]
pub fn dump_frame_stats(window_id: Option<String>) -> String {
    let stats = UI_ENGINE.with(|cell| build_frame_stats(&cell.borrow(), window_id.as_deref()));
    serde_json::to_string(&stats).unwrap_or_else(|_| "{}".to_string())
}

/// 🧊️📤️ `dumpMeshStats()` — every `World3d` surface's PUBLISHED mesh payload, per window: one row
/// per `meshes_json` entry with its producer-stamped role, its array lengths and its own bounds,
/// plus the instances, the camera, the selection and the surface's page rect.
///
/// ⚖️ Its two neighbours cannot answer the question this one exists for. `dumpFrameStats` counts
/// what the frame DREW (`sceneInstances`), and the World3d surface census counts what the renderer's
/// mesh store HOLDS (`state-meshes`, which also carries the face overlay and the placeholder) — so
/// neither is comparable with a producer's committed `delivery.meshes`/`delivery.meshRoles`. This
/// export answers the producer's own publication, which is the only number both sides state
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-mesh-oracle-2026-09-14.md`).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = dumpMeshStats)]
pub fn dump_mesh_stats(window_id: Option<String>) -> String {
    let stats = UI_ENGINE.with(|cell| build_mesh_stats(&cell.borrow(), window_id.as_deref()));
    serde_json::to_string(&stats).unwrap_or_else(|_| "{}".to_string())
}

/// 🎲️📤️ `dumpBoard2d()` — every `Board2d` surface's published board (page rect, camera, node positions, selection, and
/// the node, edge and handle counts), the wgpu twin of React's Board2dHost `data-board-*` vitals a browser probe reads.
/// Read-only and answered on request only: nothing is collected per frame, so it costs production nothing.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = dumpBoard2d)]
pub fn dump_board2d(window_id: Option<String>) -> String {
    let dump = UI_ENGINE.with(|cell| build_board2d_dump(&cell.borrow(), window_id.as_deref()));
    serde_json::to_string(&dump).unwrap_or_else(|_| "{}".to_string())
}

/// ♿️📤️ `dumpAccessibility()` — the app's accessibility tree (every live window, or the one a caller
/// names), for the host to MIRROR into a real ARIA subtree beside the canvas
/// (`🚀️browser-boot/🟦️.ts`'s `accessibilityMirror`).
///
/// This is not a test probe like its two neighbours: it is the wgpu target's production
/// accessibility path. A DOM renderer gets `aria-label`/`aria-describedby`/`aria-live` for free,
/// because React's Interpreter writes them onto the element it renders per record; a GPU canvas has
/// no elements, so the same information has to leave the isolate as data and be given elements on
/// the other side. `ui_wgpu::wgpu::accessibility::accessibility_projection` builds it from the very
/// retained document the paint walk consumes, so what is announced is always what is on screen.
///
/// Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #3 of `📓️audit-wgpu-parity-2026-09-13.md`.
/// 🎯️📤️ `dumpChrome()` — the shell chrome's pointer registry and its dispatched-action ledger, the two
/// things a DOM renderer publishes for free (an element to click, a handler that ran) and a GPU canvas
/// otherwise publishes not at all. Its three neighbours answer the retained DOCUMENT; this one answers
/// the chrome around it, which is where a behavioural-parity journey spends most of its steps
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, packet W5b).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = dumpChrome)]
pub fn dump_chrome(window_id: Option<String>) -> String {
    let dump = project_chrome_dump(&CHROME_LEDGER.borrow(), window_id.as_deref(), semio_framework_trace::runtime_diagnostics_enabled());
    serde_json::to_string(&dump).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = dumpAccessibility)]
pub fn dump_accessibility(window_id: Option<String>) -> String {
    let dump = UI_ENGINE.with(|cell| build_accessibility_dump(&cell.borrow(), window_id.as_deref()));
    serde_json::to_string(&dump).unwrap_or_else(|_| "{}".to_string())
}
//#endregion 🔬️IntrospectionExports

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-introspection/🦀️.rs"]
mod introspection_tests;
//#endregion 🔬️Introspection
