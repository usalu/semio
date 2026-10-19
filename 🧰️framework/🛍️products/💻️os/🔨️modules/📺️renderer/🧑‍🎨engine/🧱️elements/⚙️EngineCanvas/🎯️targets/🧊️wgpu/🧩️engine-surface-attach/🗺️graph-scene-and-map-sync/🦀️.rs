//! 🗺️ Graph scene and map sync: the node graph evaluation and scene sync, the surface registration, the map theme and the map engine and tiled map scene sync. See [`super`].

use super::*;

//#region 🧩️EngineSurfaceAttach
/// 🧮️ The evaluation half of the scene sync, kept in its own pass for the ordering React documents:
/// `applyEvalOutputsJson` clears computing chrome, so eval is applied BEFORE `statusJson`/
/// `computingJson`, never after.
pub(super) fn sync_node_graph_evaluation(engine: &mut NodeGraphEngine, cache: &mut NodeGraphSyncCache, graph: &semio_framework_ui_declaration::ui::NodeGraphScene) -> bool {
    let mut changed = false;
    if cache.eval_json.as_deref() != graph.eval_json.as_deref() {
        if let (NodeGraphEngine::Flow(host), Some(json)) = (&mut *engine, graph.eval_json.as_deref()) {
            host.apply_eval_outputs_json(json);
        }
        cache.eval_json = graph.eval_json.clone();
        changed = true;
    }
    if cache.status_json.as_deref() != graph.status_json.as_deref() {
        if let Some(json) = graph.status_json.as_deref() {
            let mut observe=|progress:semio_framework_value::NativeDecodeProgress|progress.total<=64*1024;
            let mut control=semio_framework_value::NativeDecodeControl::new(256*1024,&mut observe);
            if let Ok(statuses)=infinite_canvas::board::io::text::dag_input::decode_dag_node_statuses_json(json,&mut control){
                match engine {
                    NodeGraphEngine::Flow(host)=>host.set_node_statuses(&statuses),
                    NodeGraphEngine::Dag(host)=>set_node_statuses(&mut host.dag.graph, &mut host.dag.paint, &statuses),
                }
            }
        }
        cache.status_json = graph.status_json.clone();
        changed = true;
    }
    if cache.computing_json.as_deref() != graph.computing_json.as_deref() {
        if let (None, Some(json)) = (graph.status_json.as_deref(), graph.computing_json.as_deref()) {
            let value = serde_json::from_str::<Value>(json).unwrap_or(Value::Null);
            let active = value.get("active").and_then(Value::as_str);
            let stale: Vec<String> = value.get("stale").and_then(Value::as_array).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_owned)).collect()).unwrap_or_default();
            match engine {
                NodeGraphEngine::Flow(host) => host.set_computing_progress(active, &stale),
                NodeGraphEngine::Dag(host) => set_computing_progress(&mut host.dag.graph, &mut host.dag.paint, active, &stale),
            }
        }
        cache.computing_json = graph.computing_json.clone();
        changed = true;
    }
    changed
}

/// 🕸️ Attaches — and drives — the node-graph engine host behind one `SurfaceKind::NodeGraph` scene.
/// Called from the retained scene paint (`scenes::render_component_scene_step`), so a window's first
/// painted frame after `SurfaceVisible` is the frame the host is constructed on, and every later frame
/// re-feeds only the scene fields that moved. Returns whether a live host now backs the surface.
pub fn sync_node_graph_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, panel: Rgba) -> bool {
    let dark = 0.2126 * panel.r + 0.7152 * panel.g + 0.0722 * panel.b < 0.5;
    let Some(graph) = scene.node_graph.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    let created = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.host_id) else {
            return None;
        };
        let created = entry.node_graph.is_none();
        if created {
            entry.node_graph = Some(node_graph_engine_from_scene(graph, dark));
        }
        let Some(engine) = entry.node_graph.as_mut() else {
            return None;
        };
        let cache = &mut entry.sync_cache;
        let mut changed = created;
        if cache.viewport_pixels != Some((width, height)) {
            match engine {
                NodeGraphEngine::Flow(host) => host.set_viewport(width, height, 1.0),
                NodeGraphEngine::Dag(host) => host.set_viewport(width, height, 1.0),
            }
            cache.viewport_pixels = Some((width, height));
            changed = true;
        }
        if sync_node_graph_engine(engine, cache, graph) || changed {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        Some(created)
    });
    let Some(created) = created else {
        return false;
    };
    log_graph_geometry_census(&scene.host_id);
    register_engine_surface(scene, window_id, bounds, EngineSurfaceKindDetail::NodeGraph, created, Some(snapshot.identity.token));
    true
}

/// 🧩️ Records one attached surface for the shell's per-frame mirror. Every kind goes through it —
/// including `World3d`, whose host lives in the shell's own `world3d_states` — so the shell has ONE
/// drained witness of what the chrome walk actually painted this frame.
pub fn register_engine_surface(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, detail: EngineSurfaceKindDetail, created: bool, engine_token: Option<EngineSurfaceToken>) {
    ATTACHED_SURFACES.with(|cell| {
        cell.borrow_mut().upsert(EngineSurfaceRegistration { host_id: scene.host_id.clone(), surface_id: scene.surface_id.clone(), engine_token, window_id: window_id.to_string(), bounds, controller_id: scene.controller_id.clone(), detail, created })
    });
}

/// 🎨️ The map palette this shell theme implies, in the rgba8 document `MapHost::set_map_theme_from_json`
/// reads — the wgpu twin of React's `serializeMapCanvasThemeJson`. Fields with no unambiguous shell
/// counterpart are left out, so `MapPalette`'s own defaults stand for them exactly as they do in React.
fn map_theme_json(theme: &Theme) -> String {
    let rgba8 = |color: Rgba| json!([(color.r * 255.0).round() as u16, (color.g * 255.0).round() as u16, (color.b * 255.0).round() as u16, (color.a * 255.0).round() as u16]);
    json!({
        "surfaceClear": rgba8(theme.canvas_clear),
        "labelFill": rgba8(theme.text),
        "labelHalo": rgba8(theme.panel),
        "selectionStroke": rgba8(theme.accent),
        "hoverStroke": rgba8(theme.text_element),
    })
    .to_string()
}

/// 🗺️ Feeds one `TiledMapScene` into a live `MapHost`, field by field, applying only what changed —
/// the wgpu twin of React's `TiledMapHost` effect ladder (descriptor, then render mode/vector style/
/// lod, then layer documents, then interaction, then camera).
///
/// @see `🧱️elements/🧭️TiledMapHost/🟦️.tsx`
fn sync_map_engine(host: &mut MapHost, cache: &mut MapSyncCache, map: &semio_framework_ui_declaration::ui::TiledMapScene, theme: &Theme, width: u32, height: u32) -> bool {
    let mut changed = false;
    let size_key = format!("{width}x{height}");
    if cache.size_key.as_deref() != Some(size_key.as_str()) {
        host.scene.set_size(width, height, 1.0);
        cache.size_key = Some(size_key);
        changed = true;
    }
    let theme_json = map_theme_json(theme);
    if cache.theme_json.as_deref() != Some(theme_json.as_str()) {
        let _ = host.set_map_theme_from_json(&theme_json);
        cache.theme_json = Some(theme_json);
        changed = true;
    }
    if cache.map_descriptor_json.as_deref() != Some(map.map_descriptor_json.as_str()) {
        let _ = host.sync_map_json(&map.map_descriptor_json);
        cache.map_descriptor_json = Some(map.map_descriptor_json.clone());
        changed = true;
    }
    if cache.render_mode.as_deref() != Some(map.render_mode.as_str()) {
        host.set_render_mode(&map.render_mode);
        cache.render_mode = Some(map.render_mode.clone());
        changed = true;
    }
    if cache.vector_style.as_deref() != Some(map.vector_style.as_str()) {
        host.set_vector_style(&map.vector_style);
        cache.vector_style = Some(map.vector_style.clone());
        changed = true;
    }
    if cache.lod_mode.as_deref() != Some(map.lod_mode.as_str()) {
        host.set_lod_mode(&map.lod_mode);
        cache.lod_mode = Some(map.lod_mode.clone());
        changed = true;
    }
    if cache.layer_visibility_json.as_deref() != Some(map.layer_visibility_json.as_str()) {
        let _ = host.set_layer_visibility_from_json(&map.layer_visibility_json);
        cache.layer_visibility_json = Some(map.layer_visibility_json.clone());
        changed = true;
    }
    if cache.layer_stroke_scale_json.as_deref() != Some(map.layer_stroke_scale_json.as_str()) {
        let _ = host.set_layer_stroke_scale_from_json(&map.layer_stroke_scale_json);
        cache.layer_stroke_scale_json = Some(map.layer_stroke_scale_json.clone());
        changed = true;
    }
    if cache.selection_json.as_deref() != Some(map.selection_json.as_str()) || cache.hover_json.as_deref() != Some(map.hover_json.as_str()) {
        host.sync_interaction_json(&map.selection_json, &map.hover_json);
        cache.selection_json = Some(map.selection_json.clone());
        cache.hover_json = Some(map.hover_json.clone());
        changed = true;
    }
    if cache.camera_json.as_deref() != Some(map.camera_json.as_str()) {
        match engine_camera_from_json(&map.camera_json) {
            Some((x, y, zoom)) => board_set_camera(&mut host.scene, &mut host.outbox, x, y, zoom),
            None => host.fit_world_camera(),
        }
        cache.camera_json = Some(map.camera_json.clone());
        changed = true;
    }
    changed
}

/// 🗺️ Attaches — and drives — the `MapHost` behind one `SurfaceKind::TiledMap` scene, on the same
/// production seam `sync_node_graph_scene` uses.
pub fn sync_tiled_map_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, theme: &Theme) -> bool {
    let Some(map) = scene.tiled_map.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    let created = ENGINE_SURFACES.with(|cell| {
        let mut map_registry = cell.borrow_mut();
        let entry = map_registry.get_mut(&scene.host_id)?;
        let created = entry.map_host.is_none();
        if created {
            entry.map_host = Some(MapHost::new());
        }
        let host = entry.map_host.as_mut()?;
        if sync_map_engine(host, &mut entry.map_sync_cache, map, theme, width, height) || created {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        let (host, cache) = (entry.map_host.as_ref()?, &mut entry.map_sync_cache);
        reserve_map_tile_fetches(&scene.host_id, host, cache, map);
        Some(created)
    });
    let Some(created) = created else {
        return false;
    };
    register_engine_surface(scene, window_id, bounds, EngineSurfaceKindDetail::TiledMap { selection_method: map.selection_method.clone() }, created, Some(snapshot.identity.token));
    true
}
//#endregion 🧩️EngineSurfaceAttach
