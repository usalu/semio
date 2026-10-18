//! 🔁️ Host sync and ui scopes: the board host content and runtime sync and the window and selection ui dirty scopes. See [`super`].

use super::*;

/// 🧱️ The expensive half of syncing `host` from `envelope`: a full `clear_scene()` + rebuild of
/// every node/handle/edge plus the kind-catalog/kind-compat re-push. Only needed when the snapshot
/// content actually changed — gated by `last_synced_fixture` in `handle`.
pub fn sync_host_fixture_content(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    let _ = {let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);semio_framework_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&envelope.board_snapshot.to_string(),&mut control)};
    if let Some(json) = board_kind_catalogs_json_or_inferred(&envelope.board_snapshot) {
        let _ = host.style.set_board_kind_catalogs_from_json(&json);
    }
    if let Some(compat) = envelope.board_snapshot.get("meta").and_then(|value| value.get("kindCompatibility")).or_else(|| envelope.board_snapshot.get("kindCompatibility")) {
        let json = semio_framework_pack_json::to_json_string(compat); {
            let _ = host.style.set_handle_link_compat_from_json(&json);
        }
    }
}

/// 🪶️ The cheap half of syncing `host` from `envelope`: plain setters mirroring ephemeral view state
/// (selection/utility/grid/LOD/…) — must run on every action regardless of whether the snapshot
/// content changed, since this state itself changes every action.
pub fn sync_host_runtime_state(host: &mut BoardHost, envelope: &Puzzle2dScene, selected_ids: &[String]) {
    host.scene.set_size(BOARD_DEFAULT_WIDTH, BOARD_DEFAULT_HEIGHT, 1.0);
    set_selection_ids(&mut host.scene, &mut host.focus, &mut host.outbox, selected_ids);
    host.set_active_utility(&envelope.active_utility);
    let overview_lod_mode = envelope.runtime.lod_mode_by_pane.get(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID).map_or(PUZZLE2D_LOD_MODE_AUTOMATIC, String::as_str);
    if overview_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
        host.style.set_automatic_lod(true);
    } else {
        host.style.set_automatic_lod(false);
        host.style.set_forced_draw_lod_label(overview_lod_mode);
    }
    board_set_grid_visible(&mut host.scene, &mut host.style, envelope.runtime.grid_visible);
    host.style.set_grid_snap_enabled(envelope.runtime.grid_snap_enabled);
    set_transform_flags(&mut host.scene, &mut host.gestures, envelope.runtime.transform_move, envelope.runtime.transform_rotate);
    let _ = host.style.set_grid_factor(envelope.runtime.grid_factor);
    let (brush_width, brush_height) = puzzle2d_area_brush_extent_world(&envelope.runtime);
    host.gestures.set_area_brush_extent(brush_width, brush_height);
    host.set_suggestion_offset(envelope.runtime.suggestion_offset);
    let weights_json = semio_framework_pack_json::to_json_string(&json!({
        "nodeWeights": envelope.runtime.node_kind_weights,
        "handleWeights": envelope.runtime.handle_kind_weights,
    })); {
        host.set_brush_kind_weights(&weights_json);
    }
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the marquee method is
    // framework-owned now (`interactionSelect`'s `method` arg) — the board engine still needs SOME
    // default to hit-test with, so it keeps "rectangle" rather than reading a deleted config field.
    // 🎯️ …but WHICH granularities a hit-test may return is app config now (`setSelectableKind`), so
    // a filtered pane cannot silently pick the kind the user switched off. Argument order on the
    // normal port is `(nodes, edges, handles)`.
    let kinds = envelope.runtime.selectable_kinds;
    host.focus.set_selection_options("rectangle", "replace", kinds.nodes, kinds.edges, kinds.handles);
}

fn sync_host_from_envelope(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    sync_host_fixture_content(host, envelope);
    sync_host_runtime_state(host, envelope, &[]);
}

/// 🐢️ Narrow `UiDirtyScope` shared by pure view/selection/camera actions that only touch the 3
/// canvas panes (never a panel or engagement/measure/utility refresh).
pub fn puzzle2d_window_only_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: crate::editor::puzzle2d::config::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for actions that additionally change the engagement bar (active utility,
/// brush weights, LOD/grid settings, engagement text input) but never touch document content.
pub fn puzzle2d_window_and_engagements_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: crate::editor::puzzle2d::config::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: true,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for settings surfaced in the measures sidebar (LOD mode, grid, brush
/// weights, suggestion offset) but that never touch document content or the engagement bar.
pub fn puzzle2d_window_and_measures_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: crate::editor::puzzle2d::config::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: true,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for a runtime-only selection change: the 3 canvas panes plus the
/// layers/properties panels (which highlight the selection) and the engagement bar.
pub fn puzzle2d_select_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: crate::editor::puzzle2d::config::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: vec![crate::editor::puzzle2d::config::PUZZLE2D_PLAY_BODY_LAYERS.to_string(), crate::editor::puzzle2d::config::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()],
        utilities: false,
        tools: false,
        engagements: true,
        measures: false,
        labels: false,
    }
}
