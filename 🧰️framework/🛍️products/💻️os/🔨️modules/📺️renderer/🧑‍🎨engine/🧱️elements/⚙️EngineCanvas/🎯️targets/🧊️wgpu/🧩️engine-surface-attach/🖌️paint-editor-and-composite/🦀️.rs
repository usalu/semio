//! 🖌️ Paint, editor and composite: the paint2d and text editor scene sync, the engine scene sync and paint staging, the raster composite progress and the engine raster key. See [`super`].

use super::*;

//#region 🧩️EngineSurfaceAttach
/** 🎨️ The canvas-theme document both `RasterHost::set_canvas_theme_from_json` and
 * `EditorHost::set_canvas_theme_from_json` read — the wgpu twin of `syncSessionCanvasTheme`
 * (`@semio-tech/ui-styling`), which reads the same CSS custom properties React's hosts do.
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `syncSessionCanvasTheme(session)` */
fn engine_canvas_theme_json(theme: &Theme) -> String {
    let rgba8 = |color: Rgba| json!([(color.r * 255.0).round() as u16, (color.g * 255.0).round() as u16, (color.b * 255.0).round() as u16, (color.a * 255.0).round() as u16]);
    json!({
        "rasterClear": rgba8(theme.canvas_clear),
        "gridMinorStroke": rgba8(theme.separator),
        "labelFill": rgba8(theme.text),
        "labelFillHovered": rgba8(theme.text_element),
        "labelHalo": rgba8(theme.panel),
        "nodeFillHovered": rgba8(theme.text_element),
        "nodeFillSelected": rgba8(theme.accent),
    })
    .to_string()
}

/** 🖌️ Feeds one `Paint2dScene` into a live [`RasterHost`], field by field, applying only what
 * changed — the wgpu twin of `Paint2dCanvasSurface`'s `syncAll` effect: document, then assets, then
 * utility/brush, then interaction, then the camera (fitted for the navigator view mode, taken from
 * the scene otherwise).
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `syncAll` */
fn sync_raster_engine(host: &mut RasterHost, cache: &mut RasterSyncCache, paint: &semio_framework_ui_declaration::ui::Paint2dScene, theme: &Theme, width: u32, height: u32) -> bool {
    let mut changed = false;
    let size_key = format!("{width}x{height}");
    if cache.size_key.as_deref() != Some(size_key.as_str()) {
        host.scene.set_size(width, height, 1.0);
        cache.size_key = Some(size_key);
        changed = true;
    }
    let theme_json = engine_canvas_theme_json(theme);
    if cache.theme_json.as_deref() != Some(theme_json.as_str()) {
        let _ = host.set_canvas_theme_from_json(&theme_json);
        cache.theme_json = Some(theme_json);
        changed = true;
    }
    if cache.document_sync_json.as_deref() != Some(paint.document_sync_json.as_str()) {
        let _ = host.sync_document_json(&paint.document_sync_json);
        cache.document_sync_json = Some(paint.document_sync_json.clone());
        changed = true;
    }
    if cache.assets_json.as_deref() != Some(paint.assets_json.as_str()) {
        for (key, bytes) in paint2d_asset_bytes(&paint.assets_json) {
            let _ = host.upload_raster_image_key(&key, &bytes);
        }
        cache.assets_json = Some(paint.assets_json.clone());
        changed = true;
    }
    if cache.active_utility.as_deref() != Some(paint.active_utility.as_str()) {
        host.set_active_utility(&paint.active_utility);
        cache.active_utility = Some(paint.active_utility.clone());
        changed = true;
    }
    if let Ok(color) = u32::from_str_radix(paint.brush_color.trim_start_matches('#'), 16) {
        let brush = (paint.brush_size, paint.brush_opacity, color, paint.brush_hardness);
        if cache.brush != Some(brush) {
            host.set_brush_size(paint.brush_size as f32);
            host.set_brush_opacity(paint.brush_opacity as f32);
            host.set_brush_color([(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]);
            host.set_brush_hardness(paint.brush_hardness as f32);
            cache.brush = Some(brush);
            changed = true;
        }
    }
    if let Some(target)=match paint.paint_target.as_str(){"pixels"=>Some(PaintTarget::Pixels),"mask"=>Some(PaintTarget::Mask),_=>None}{
        if cache.paint_target!=Some(target){host.set_paint_target(target);cache.paint_target=Some(target);changed=true;}
    }
    if let Ok(value)=u8::try_from(paint.mask_value){if cache.mask_value!=Some(value){host.set_mask_value(value);cache.mask_value=Some(value);changed=true;}}
    if cache.selection_json.as_deref() != Some(paint.selection_json.as_str()) || cache.hovered_id.as_deref() != paint.hovered_id.as_deref() {
        let selected: Vec<String> = serde_json::from_str(&paint.selection_json).unwrap_or_default();
        host.sync_interaction(&selected, paint.hovered_id.as_deref());
        cache.selection_json = Some(paint.selection_json.clone());
        cache.hovered_id = paint.hovered_id.clone();
        changed = true;
    }
    let navigator = paint.view_mode == "navigator";
    if cache.view_mode.as_deref() != Some(paint.view_mode.as_str()) {
        cache.view_mode = Some(paint.view_mode.clone());
        changed = true;
    }
    let camera_json = if navigator { host.navigator_fit_camera_json(f64::from(width), f64::from(height)) } else { paint.camera_json.clone() };
    if cache.camera_json.as_deref() != Some(camera_json.as_str()) {
        if let Some((x, y, zoom)) = engine_camera_from_json(&camera_json) {
            board_set_camera(&mut host.scene, &mut host.outbox, x, y, zoom);
        }
        cache.camera_json = Some(camera_json);
        changed = true;
    }
    changed
}

/** 🖼️ `Paint2dScene.assets_json`'s `{ key: { mime, data } }` rows decoded to raw bytes — the wgpu
 * twin of `base64ToBytes(asset.data)` in `syncAll`'s asset loop. */
fn paint2d_asset_bytes(assets_json: &str) -> Vec<(String, Vec<u8>)> {
    let Ok(Value::Object(rows)) = serde_json::from_str::<Value>(assets_json) else {
        return Vec::new();
    };
    rows.into_iter()
        .filter_map(|(key, row)| {
            let data = row.get("data").and_then(Value::as_str)?;
            let payload = data.rsplit_once("base64,").map_or(data, |(_, tail)| tail);
            semio_framework_io_base64::base64_standard_decode(payload).ok().map(|bytes| (key, bytes))
        })
        .collect()
}

/// 🖌️ Attaches — and re-feeds — the `RasterHost` behind one `SurfaceKind::Paint2d` scene.
pub fn sync_paint2d_scene(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme) -> bool {
    let Some(paint) = scene.paint_2d.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(_snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(entry) = registry.get_mut(&scene.host_id) else {
            return false;
        };
        let created = entry.raster_host.is_none();
        if created {
            entry.raster_host = Some(RasterHost::new());
        }
        let Some(host) = entry.raster_host.as_mut() else {
            return false;
        };
        if sync_raster_engine(host, &mut entry.raster_sync_cache, paint, theme, width, height) || created {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        true
    })
}

/** ✍️ Attaches — and re-feeds — the `EditorHost` behind one `SurfaceKind::TextEditor` scene. The
 * host is the SAME one `text_editor_pointer_button_into`/`text_editor_apply_key_into` mutate, so a
 * keystroke and the glyphs it produces are one object, not a render copy of a model. */
pub fn sync_text_editor_scene(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(_snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    let Ok(scene_json) = serde_json::to_string(editor) else {
        return false;
    };
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(entry) = registry.get_mut(&scene.host_id) else {
            return false;
        };
        let created = entry.editor.is_none();
        if created {
            entry.editor = Some(EditorHost::new());
        }
        let Some(host) = entry.editor.as_mut() else {
            return false;
        };
        let mut changed = created;
        let size_key = format!("{width}x{height}");
        if entry.editor_sync_cache.size_key.as_deref() != Some(size_key.as_str()) {
            host.scene.set_size(width, height, 1.0);
            entry.editor_sync_cache.size_key = Some(size_key);
            changed = true;
        }
        let theme_json = engine_canvas_theme_json(theme);
        if entry.editor_sync_cache.theme_json.as_deref() != Some(theme_json.as_str()) {
            let _ = host.set_canvas_theme_from_json(&theme_json);
            entry.editor_sync_cache.theme_json = Some(theme_json);
            changed = true;
        }
        if entry.editor_sync_cache.scene_json.as_deref() != Some(scene_json.as_str()) {
            let explicit = text_editor_explicit_draft_settings(scene).is_some();
            let external = if explicit && entry.editor_delivery.explicit_dirty {
                if editor.buffer == host.text() {
                    entry.editor_delivery.explicit_dirty = false;
                    entry.editor_delivery.explicit_base = Some(editor.buffer.clone());
                    entry.editor_delivery.explicit_changes = None;
                    entry.editor_delivery.explicit_conflict = false;
                    entry.editor_delivery.explicit_past.clear();
                    entry.editor_delivery.explicit_future.clear();
                    entry.editor_delivery.explicit_snapshot = Some(editor.buffer.clone());
                    entry.editor_delivery.explicit_publication = None;
                    entry.editor_delivery.explicit_active = None;
                    entry.editor_delivery.explicit_error = None;
                    true
                } else {
                    if entry.editor_delivery.explicit_base.as_deref() != Some(editor.buffer.as_str()) {
                        entry.editor_delivery.explicit_conflict = true;
                    }
                    false
                }
            } else if explicit {
                entry.editor_delivery.explicit_base = Some(editor.buffer.clone());
                entry.editor_delivery.explicit_changes = None;
                entry.editor_delivery.explicit_conflict = false;
                entry.editor_delivery.explicit_snapshot = Some(editor.buffer.clone());
                entry.editor_delivery.explicit_error = None;
                true
            } else {
                entry.editor_delivery.reconcile_buffer(&editor.buffer)
            };
            if let Err(fault) = synchronize_text_editor_host(host, editor, external) {
                engine_canvas_fault_log(&format!("[text-editor] surface {} refused its scene — {fault}", scene.host_id));
            }
            entry.editor_sync_cache.scene_json = Some(scene_json);
            changed = true;
        }
        if changed {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        true
    })
}

/// 🧩️ THE production attach entry: one call, every `SurfaceKind` whose engine host the wgpu shell
/// composites through a vello texture. Called from the retained scene paint
/// (`scenes::render_component_scene_step`), so a window's first painted frame after `SurfaceVisible`
/// is the frame its host is constructed on, and every later frame re-feeds only what moved.
/// `SurfaceKind::World3d` is NOT here: its host is `World3dState`, which lives in the shell's own
/// `world3d_states` (the pick/asset/authority ladders address it there) and paints a real 3-D pass
/// into the window draw list instead of a vello raster — `scenes` attaches it against that map and
/// records it through [`register_engine_surface`] like every other kind.
pub fn sync_engine_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, theme: &Theme) -> bool {
    match scene.component_kind {
        SurfaceKind::NodeGraph => sync_node_graph_scene(scene, window_id, bounds, theme.panel),
        SurfaceKind::TiledMap => sync_tiled_map_scene(scene, window_id, bounds, theme),
        SurfaceKind::Board2d => sync_board2d_scene(scene, window_id, bounds, theme),
        SurfaceKind::Paint2d => sync_paint2d_scene(scene, bounds, theme),
        SurfaceKind::TextEditor => sync_text_editor_scene(scene, bounds, theme),
        _ => false,
    }
}

/// 🎨️ Paints the attached host through its OWN vector renderer and stages the result as this frame's
/// engine packet — the one paint entry for every vello-composited surface kind.
pub fn stage_engine_scene_paint(scene: &UiComponentSceneNode, bounds: Rect, clear: Rgba) -> bool {
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let staged = ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let identity = registry.identity(&scene.host_id)?;
        let entry = registry.get_mut(&scene.host_id)?;
        let painted = match scene.component_kind {
            SurfaceKind::NodeGraph => match entry.node_graph.as_ref()? {
                NodeGraphEngine::Flow(host) => {
                    let mut painted = canvas::Scene::new();
                    host.paint_scene(&mut painted, width, height, 1.0);
                    painted
                }
                NodeGraphEngine::Dag(host) => {
                    let mut painted = canvas::Scene::new();
                    host.paint_scene(&mut painted, width, height, 1.0);
                    painted
                }
            },
            SurfaceKind::TiledMap => {
                let host = entry.map_host.as_mut()?;
                host.prepare_visible_tiles();
                host.build_vector_scene()
            }
            SurfaceKind::Board2d => {
                let host = entry.board_host.as_ref()?;
                let mut painted = host.build_vector_scene();
                append_board_tool_run_trace(&mut painted, host, entry);
                painted
            }
            SurfaceKind::Paint2d => entry.raster_host.as_mut()?.build_current_scene(),
            SurfaceKind::TextEditor => entry.editor.as_ref()?.build_scene(),
            _ => return None,
        };
        Some(StagedEngineScene {
            surface: EngineSurfaceSnapshot { identity, metrics_generation: entry.metrics_generation },
            document_generation: entry.document_generation,
            scene_revision: entry.scene_revision,
            scene: painted,
            clear: Color::new([clear.r, clear.g, clear.b, clear.a]),
            width,
            height,
        })
    });
    let Some(staged) = staged else {
        return false;
    };
    STAGED_ENGINE_SCENES.with(|cell| cell.borrow_mut().upsert(staged));
    true
}

#[derive(Clone, Copy)]
pub(crate) struct RasterCompositeProgress {
    pub completed: u64,
    pub done: bool,
}

/// 🧩️ Advances raster preparation within one frame-build grant, retaining the previous presented image.
pub fn advance_raster_composite(scene: &UiComponentSceneNode) -> RasterCompositeProgress {
    if scene.component_kind != SurfaceKind::Paint2d {
        return RasterCompositeProgress { completed: 0, done: true };
    }
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(host) = registry.get_mut(&scene.host_id).and_then(|entry| entry.raster_host.as_mut()) else {
            return RasterCompositeProgress { completed: 0, done: true };
        };
        if let Some(paint) = &scene.paint_2d {
            let selection: Vec<String> = serde_json::from_str(&paint.selection_json).unwrap_or_default();
            host.set_composite_view(&paint.view_mode, selection.first().map(String::as_str));
        }
        match host.advance_composite(65536) {
            Ok(progress) => RasterCompositeProgress { completed: progress.completed as u64, done: progress.done },
            Err(error) => {
                engine_canvas_debug_log(&format!("Raster composite failed: {error}"));
                RasterCompositeProgress { completed: 0, done: true }
            }
        }
    })
}

/// 🔑️ The raster key an engine surface's composited texture is published under — the same key
/// `EngineGpuCandidate` reserves through `GpuContext::reserve_engine_texture`, so a retained
/// `push_raster_quad` under it composites the vello-rendered graph into the window's draw list.
pub fn engine_raster_key(surface_id: &str) -> Option<String> {
    EngineSurfaceId::try_from_str(surface_id).ok().map(|id| id.with_raster_key(str::to_owned))
}
//#endregion 🧩️EngineSurfaceAttach
