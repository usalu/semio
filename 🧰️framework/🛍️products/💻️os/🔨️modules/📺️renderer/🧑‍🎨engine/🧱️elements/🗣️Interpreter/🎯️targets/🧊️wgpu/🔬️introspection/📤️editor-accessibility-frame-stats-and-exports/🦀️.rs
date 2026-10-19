//! 📤️ Editor accessibility, frame stats and exports: the text editor accessibility entries, the accessibility dump, the frame stats and the wasm dump exports. See [`super`].

use super::*;

//#region 🔬️Introspection
//#region 🔬️IntrospectionBuilders
struct TextEditorAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub key: String,
    pub label: String,
    pub depth: usize,
    pub rect: Rect,
    pub value: String,
    read_only: bool,
}

fn collect_text_editor_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, origin: (f32, f32), depth: usize, entries: &mut Vec<TextEditorAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    let (x, y, width, height) = tree.mounted_layout(node).unwrap_or((retained.layout.x, retained.layout.y, retained.layout.width, retained.layout.height));
    let absolute = (origin.0 + x, origin.1 + y);
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::TextEditor && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            if let Some(value) = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_accessibility_value(scene) {
                entries.push(TextEditorAccessibilityEntry {
                    scene_node: node,
                    host_id: scene.host_id.clone(),
                    key: format!("{}.editor", scene.host_id),
                    label: scene.text_editor.as_ref().and_then(|editor| editor.language.as_ref()).map_or_else(|| "Editor".into(), |language| format!("{language} editor")),
                    depth: depth.saturating_add(1),
                    rect: Rect::new(absolute.0, absolute.1, width, height),
                    value,
                    read_only: semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_is_read_only(scene),
                });
            }
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_text_editor_accessibility_entries(tree, child, absolute, depth.saturating_add(1), entries);
    }
}

pub fn text_editor_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Vec<TextEditorAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_text_editor_accessibility_entries(tree, root, (0.0, 0.0), 0, &mut entries);
    }
    entries
}

fn append_text_editor_accessibility_nodes(window_id: &str, tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
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
pub fn build_accessibility_dump(engine: &semio_framework_ui_wgpu_engine::Ui, requested: Option<&str>) -> DumpAccessibility {
    let window_ids = dump_window_ids(engine);
    let announced: Vec<String> = match requested.filter(|id| !id.is_empty()) {
        Some(id) => window_ids.iter().filter(|live| live.as_str() == id).cloned().collect(),
        None => ACCESSIBILITY_VISIBLE_WINDOWS.borrow().published.iter().filter(|visible| window_ids.iter().any(|live| live == *visible)).cloned().collect(),
    };
    let mut windows: Vec<DumpAccessibilityWindow> = announced
        .into_iter()
        .map(|window_id| {
            let mut nodes = engine.tree(&window_id).map(semio_framework_ui_wgpu_hosting::accessibility::accessibility_projection).unwrap_or_default();
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
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn build_frame_stats(engine: &semio_framework_ui_wgpu_engine::Ui, requested: Option<&str>) -> DumpFrameStats {
    let window_ids = dump_window_ids(engine);
    let Some(window_id) = dump_window_id(engine, requested) else {
        return DumpFrameStats { window_id: None, window_ids, draw_calls: 0, quad_count: 0, glyph_count: 0, scene_passes: 0, scene_draws: 0, scene_instances: 0, frame_latency: semio_framework_os_renderer_wgpu_host_base::frame_latency::snapshot() };
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
            frame_latency: semio_framework_os_renderer_wgpu_host_base::frame_latency::snapshot(),
        };
    }
    let Some(draw) = engine.draw_list(&window_id) else {
        return DumpFrameStats { window_id: Some(window_id), window_ids, draw_calls: 0, quad_count: 0, glyph_count: 0, scene_passes: 0, scene_draws: 0, scene_instances: 0, frame_latency: semio_framework_os_renderer_wgpu_host_base::frame_latency::snapshot() };
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
    DumpFrameStats { window_id: Some(window_id), window_ids, draw_calls, quad_count, glyph_count, scene_passes, scene_draws, scene_instances, frame_latency: semio_framework_os_renderer_wgpu_host_base::frame_latency::snapshot() }
}
//#endregion 🔬️IntrospectionBuilders

//#region 🔬️IntrospectionExports
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
//#endregion 🔬️Introspection
