//! 🎲️ Board sync: the board engine sync, the area brush and transform flag defaults, the tool run trace and the board2d scene sync. See [`super`].

use super::*;

//#region 🧩️EngineSurfaceAttach
/// 🎲️ Feeds one `Board2dScene` into a live `BoardHost`, field by field, applying only what changed.
/// `load_board_snapshot_json` resets selection AND camera to the snapshot's own defaults, so a snapshot pass
/// re-applies both silently right after — the rule React's `applySnapshotToSession` states explicitly.
///
/// @see `🧱️elements/🖥️Board2dHost/🟦️.tsx` — `applySnapshotToSession`
///
/// 🎯️ Argument order on the normal port is `(nodes, edges, handles)`; the owning app's own
/// selectable-kind filter decides which granularity a pick may reach at all.
pub fn sync_board_engine(host: &mut infinite_canvas::BoardHost, cache: &mut BoardSyncCache, board: &semio_framework_ui_declaration::ui::Board2dScene, width: u32, height: u32) -> bool {
    let mut changed = false;
    let size_key = format!("{width}x{height}");
    if cache.size_key.as_deref() != Some(size_key.as_str()) {
        host.scene.set_size(width, height, 1.0);
        cache.size_key = Some(size_key);
        changed = true;
    }
    let snapshot_applied = cache.snapshot_json.as_deref() != Some(board.snapshot_json.as_str());
    if snapshot_applied {
        {let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);infinite_canvas::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board.snapshot_json,&mut control)};
        cache.granularity_by_id = board2d_granularity_by_id(&board.snapshot_json);
        cache.snapshot_json = Some(board.snapshot_json.clone());
        changed = true;
    }
    if cache.domain_id != board.domain_id {
        cache.domain_id = board.domain_id.clone();
        changed = true;
    }
    if cache.glyph_catalogs_json.as_deref() != Some(board.glyph_catalogs_json.as_str()) {
        let _ = host.style.set_board_kind_catalogs_from_json(&board.glyph_catalogs_json);
        cache.glyph_catalogs_json = Some(board.glyph_catalogs_json.clone());
        changed = true;
    }
    if cache.placement_naming != board.placement_naming {
        let _ = host.style.set_placement_naming_json(board.placement_naming.as_deref());
        cache.placement_naming = board.placement_naming.clone();
    }
    if cache.placement_compatibility_json.as_deref() != Some(board.placement_compatibility_json.as_str()) {
        let _ = host.style.set_handle_link_compat_from_json(&board.placement_compatibility_json);
        cache.placement_compatibility_json = Some(board.placement_compatibility_json.clone());
        changed = true;
    }
    let selectable_kinds = (board.selectable_nodes, board.selectable_edges, board.selectable_handles);
    if snapshot_applied || cache.selection_method.as_deref() != Some(board.selection_method.as_str()) || cache.selectable_kinds != Some(selectable_kinds) {
        host.focus.set_selection_options(&board.selection_method, "replace", selectable_kinds.0, selectable_kinds.1, selectable_kinds.2);
        cache.selection_method = Some(board.selection_method.clone());
        cache.selectable_kinds = Some(selectable_kinds);
        changed = true;
    }
    if cache.grid_visible != Some(board.grid_visible) {
        board_set_grid_visible(&mut host.scene, &mut host.style, board.grid_visible);
        cache.grid_visible = Some(board.grid_visible);
        changed = true;
    }
    if snapshot_applied || cache.selection_json.as_deref() != Some(board.selection_json.as_str()) {
        let ids = serde_json::from_str::<Vec<String>>(&board.selection_json).unwrap_or_default();
        set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &ids);
        cache.selection_json = Some(board.selection_json.clone());
        changed = true;
    }
    if snapshot_applied || cache.highlighted_ids_json.as_deref() != Some(board.highlighted_ids_json.as_str()) {
        host.focus.set_highlighted_ids(serde_json::from_str::<Vec<String>>(&board.highlighted_ids_json).unwrap_or_default());
        cache.highlighted_ids_json = Some(board.highlighted_ids_json.clone());
        changed = true;
    }
    if snapshot_applied || cache.camera_json.as_deref() != Some(board.camera_json.as_str()) {
        if let Some((x, y, zoom)) = engine_camera_from_json(&board.camera_json) {
            set_camera_silent(&mut host.scene, &mut host.outbox, x, y, zoom);
        }
        cache.camera_json = Some(board.camera_json.clone());
        changed = true;
    }
    if cache.hovered_id.as_deref() != board.hovered_id.as_deref() {
        host.focus.set_hovered_id_silent(board.hovered_id.clone());
        cache.hovered_id = board.hovered_id.clone();
        changed = true;
    }
    let active_utility = board.active_utility.clone().unwrap_or_else(|| "select".into());
    if cache.active_utility.as_deref() != Some(active_utility.as_str()) {
        host.set_active_utility(&active_utility);
        cache.active_utility = Some(active_utility);
        changed = true;
    }
    if cache.grid_snap_enabled != Some(board.grid_snap_enabled) {
        host.style.set_grid_snap_enabled(board.grid_snap_enabled);
        cache.grid_snap_enabled = Some(board.grid_snap_enabled);
        changed = true;
    }
    if cache.grid_factor != Some(board.grid_factor) {
        let _ = host.style.set_grid_factor(board.grid_factor);
        cache.grid_factor = Some(board.grid_factor);
        changed = true;
    }
    if board.suggestion_offset > 0.0 && cache.suggestion_offset != Some(board.suggestion_offset) {
        host.set_suggestion_offset(board.suggestion_offset);
        cache.suggestion_offset = Some(board.suggestion_offset);
        changed = true;
    }
    if cache.brush_weights_json.as_deref() != Some(board.brush_weights_json.as_str()) {
        host.set_brush_kind_weights(&board.brush_weights_json);
        cache.brush_weights_json = Some(board.brush_weights_json.clone());
        changed = true;
    }
    if cache.lod_mode.as_deref() != Some(board.lod_mode.as_str()) {
        if board.lod_mode == "automatic" {
            host.style.set_automatic_lod(true);
        } else {
            host.style.set_automatic_lod(false);
            host.style.set_forced_draw_lod_label(&board.lod_mode);
        }
        cache.lod_mode = Some(board.lod_mode.clone());
        changed = true;
    }
    let transform_flags = board.transform_flags.clone().unwrap_or_else(|| BOARD2D_DEFAULT_TRANSFORM_FLAGS_JSON.to_string());
    if cache.transform_flags.as_deref() != Some(transform_flags.as_str()) {
        let (move_enabled, rotate_enabled) = board2d_transform_flags_from_json(&transform_flags);
        set_transform_flags(&mut host.scene, &mut host.gestures, move_enabled, rotate_enabled);
        cache.transform_flags = Some(transform_flags);
        changed = true;
    }
    let area_brush_size = board.area_brush_size.clone().unwrap_or_else(|| BOARD2D_DEFAULT_AREA_BRUSH_SIZE_JSON.to_string());
    if cache.area_brush_size.as_deref() != Some(area_brush_size.as_str()) {
        let (width, height) = board2d_area_brush_size_from_json(&area_brush_size);
        host.gestures.set_area_brush_extent(width, height);
        cache.area_brush_size = Some(area_brush_size);
        changed = true;
    }
    changed
}

/// 🖍️ What a board that never declares `areaBrushSize` paints on a click — a non-positive axis is
/// refused by the engine itself, so the sentinel simply leaves the host's own default standing.
const BOARD2D_DEFAULT_AREA_BRUSH_SIZE_JSON: &str = "{\"width\":0,\"height\":0}";

/// 🖍️ Reads `{"width":f64,"height":f64}`; a malformed payload answers `(0, 0)`, which the engine
/// reads as "keep the extent you have" rather than collapsing the brush.
fn board2d_area_brush_size_from_json(json: &str) -> (f64, f64) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return (0.0, 0.0);
    };
    (value.get("width").and_then(serde_json::Value::as_f64).unwrap_or(0.0), value.get("height").and_then(serde_json::Value::as_f64).unwrap_or(0.0))
}

/// 🕹️ Both gumball handles on — what a board that never declares `transformFlags` composes.
const BOARD2D_DEFAULT_TRANSFORM_FLAGS_JSON: &str = "{\"move\":true,\"rotate\":true}";

/// 🕹️ Reads `{"move":bool,"rotate":bool}`; a malformed payload leaves both handles on rather than
/// silently disarming the gumball.
fn board2d_transform_flags_from_json(json: &str) -> (bool, bool) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return (true, true);
    };
    (value.get("move").and_then(serde_json::Value::as_bool).unwrap_or(true), value.get("rotate").and_then(serde_json::Value::as_bool).unwrap_or(true))
}

/// ⏯️ Feeds a board's `toolRunTrace` lane into its resident trace layer, refreshes the placement footprints when the kind
/// catalogs change and the verdict palette from `theme`, and remembers the window its cursor is echoed for. A malformed lane
/// keeps the previous records.
fn sync_board_tool_run_trace(entry: &mut EngineSurface, board: &semio_framework_ui_declaration::ui::Board2dScene, window_id: &str, theme: &Theme, catalogs_changed: bool) -> bool {
    let applied = entry.board_trace.apply_lane(board.tool_run_trace.as_deref()).is_ok_and(|applied| applied.pages > 0 || applied.cleared);
    if catalogs_changed || entry.board_trace_shapes.is_empty() {
        entry.board_trace_shapes = tool_run_trace::board2d_tool_run_trace_shapes(&board.glyph_catalogs_json);
    }
    entry.board_trace_palette = Some(tool_run_trace::ToolRunTracePalette::from_theme(theme));
    if entry.board_sync_cache.tool_run_trace_window_id.as_deref() != Some(window_id) {
        entry.board_sync_cache.tool_run_trace_window_id = Some(window_id.to_string());
    }
    applied
}

/// 🖌️ Paints the board's placement2d trace over its vector scene: one filled footprint per visible record at the host's
/// live camera, the newest `testing` record outlined. Mirrors React's `ToolRunTrace2dLayer`.
pub(super) fn append_board_tool_run_trace(painted: &mut canvas::Scene, host: &infinite_canvas::BoardHost, entry: &EngineSurface) {
    let Some(palette) = entry.board_trace_palette.as_ref() else { return };
    if entry.board_trace.is_empty() {
        return;
    }
    let origin = host.scene.world_to_screen(canvas::Point::new(0.0, 0.0));
    let unit = host.scene.world_to_screen(canvas::Point::new(1.0, 0.0));
    let zoom = (unit.x - origin.x).hypot(unit.y - origin.y);
    let highlight = palette.testing.with_alpha(1.0);
    for draw in entry.board_trace.placement_draws(palette, tool_run_trace::ToolRunTraceVisibility::default()) {
        let Some(shape) = entry.board_trace_shapes.get(draw.shape as usize) else { continue };
        let center = host.scene.world_to_screen(canvas::Point::new(f64::from(draw.position[0]), f64::from(draw.position[1])));
        let (sin, cos) = f64::from(draw.rotation).sin_cos();
        let transform = canvas::Affine::new([zoom * cos, zoom * sin, -zoom * sin, zoom * cos, center.x, center.y]);
        let color = canvas::Color::new([draw.color.r, draw.color.g, draw.color.b, draw.color.a]);
        let outline = draw.newest.then(|| (canvas::Stroke::new(ui_styling::metrics::tool_run::TESTING_OUTLINE_WIDTH / zoom.max(f64::EPSILON)), canvas::Color::new([highlight.r, highlight.g, highlight.b, highlight.a])));
        match *shape {
            tool_run_trace::ToolRunTraceShape2d::Circle { radius } => {
                let circle = canvas::Circle::new(canvas::Point::new(0.0, 0.0), radius);
                painted.fill(canvas::FillRule::NonZero, transform, color, None, &circle);
                if let Some((stroke, stroke_color)) = outline.as_ref() {
                    painted.stroke(stroke, transform, *stroke_color, None, &circle);
                }
            }
            tool_run_trace::ToolRunTraceShape2d::Rectangle { width, height } => {
                let rect = canvas::Rect::new(-width * 0.5, -height * 0.5, width * 0.5, height * 0.5);
                painted.fill(canvas::FillRule::NonZero, transform, color, None, &rect);
                if let Some((stroke, stroke_color)) = outline.as_ref() {
                    painted.stroke(stroke, transform, *stroke_color, None, &rect);
                }
            }
        }
    }
}

/// 🧭️ The `toolRunTraceCursor` every board surface echoes, keyed by the window it paints into — merged into the live view
/// state next to `world3d_tool_run_trace_cursors`.
pub fn board2d_tool_run_trace_cursors() -> HashMap<String, semio_framework_tool_run::ToolRunTraceCursor> {
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().values_mut().filter(|entry| entry.board_host.is_some()).filter_map(|entry| Some((entry.board_sync_cache.tool_run_trace_window_id.clone()?, entry.board_trace.cursor()?))).collect())
}

/// 🔍️ Resident trace records and footprints of one board surface, for the attach law.
#[cfg(any(test, feature = "testkit"))]
pub fn board2d_tool_run_trace_state(surface_id: &str) -> Option<(usize, usize, usize)> {
    ENGINE_SURFACES.with(|cell| {
        let registry = cell.borrow_mut();
        let entry = registry.get(surface_id)?;
        let palette = entry.board_trace_palette?;
        Some((entry.board_trace.len(), entry.board_trace_shapes.len(), entry.board_trace.placement_draws(&palette, tool_run_trace::ToolRunTraceVisibility::default()).len()))
    })
}

/// 🎲️ Attaches — and drives — the `BoardHost` behind one `SurfaceKind::Board2d` scene.
pub fn sync_board2d_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, theme: &Theme) -> bool {
    let Some(board) = scene.board2d.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    let created = ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let entry = registry.get_mut(&scene.host_id)?;
        let created = entry.board_host.is_none();
        if created {
            entry.board_host = Some(ManuallyDrop::new(infinite_canvas::BoardHost::default()));
        }
        let catalogs_changed = entry.board_sync_cache.glyph_catalogs_json.as_deref() != Some(board.glyph_catalogs_json.as_str());
        let host = entry.board_host.as_mut()?;
        let engine_changed = sync_board_engine(host, &mut entry.board_sync_cache, board, width, height);
        let trace_changed = sync_board_tool_run_trace(entry, board, window_id, theme, catalogs_changed);
        if engine_changed || trace_changed || created {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        Some(created)
    });
    let Some(created) = created else {
        return false;
    };
    register_engine_surface(scene, window_id, bounds, EngineSurfaceKindDetail::Board2d { snapshot_json: board.snapshot_json.clone() }, created, Some(snapshot.identity.token));
    true
}
//#endregion 🧩️EngineSurfaceAttach
