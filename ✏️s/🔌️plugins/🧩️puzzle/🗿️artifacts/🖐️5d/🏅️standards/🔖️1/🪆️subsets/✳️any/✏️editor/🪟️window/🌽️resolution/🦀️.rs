//! 🌽️ Resolution: the window config and transient resolution from views, snapshots and runtimes. See [`super`].

use super::*;

/// 🎚️ Lifts ONE board pane's persisted options into the merged shape the runtime reads.
fn window_config_from_board(value: &Puzzle5dBoardWindowConfig) -> Puzzle5dWindowConfig {
    Puzzle5dWindowConfig {
        camera2d: value.camera2d.clone(),
        lod_mode: value.lod_mode.clone(),
        suggestion_offset: value.suggestion_offset,
        grid_snap_enabled: value.grid_snap_enabled,
        grid_factor: value.grid_factor,
        grid_visible: value.grid_visible,
        selectable_kinds: value.selectable_kinds.clone(),
        ..Default::default()
    }
}

/// 🎚️ Lifts ONE world pane's persisted options into the merged shape the runtime reads.
fn window_config_from_world(value: &Puzzle5dWorldWindowConfig) -> Puzzle5dWindowConfig {
    Puzzle5dWindowConfig {
        camera3d: value.camera3d.clone(),
        sun: value.sun.clone(),
        voxel_dims: value.voxel_dims,
        grid_visible: value.grid_visible,
        grid_snap_enabled: value.grid_snap_enabled,
        grid_spacing: value.grid_spacing,
        lod_automatic: value.lod_automatic,
        lod_depth_variable: value.lod_depth_variable,
        lod_manual: value.lod_manual,
        selectable_kinds: value.selectable_kinds.clone(),
        grip_show: value.grip_show.clone(),
        grip_direction: value.grip_direction.clone(),
        transform_move: value.transform_move,
        transform_rotate: value.transform_rotate,
        ..Default::default()
    }
}

pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, crate::editor::puzzle5d::config::Puzzle5dConfig>) -> Puzzle5dWindowConfig {
    if let Some(value) = view.window::<Puzzle5dBoardWindowConfigOwner>() {
        return window_config_from_board(value);
    }
    if let Some(value) = view.window::<Puzzle5dWorldWindowConfigOwner>() {
        return window_config_from_world(value);
    }
    Puzzle5dWindowConfig::default()
}

pub fn config_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Puzzle5dWindowConfig {
    if let Some(value) = snapshot.and_then(|snapshot| snapshot.get::<Puzzle5dBoardWindowConfigOwner>()) {
        return window_config_from_board(value);
    }
    if let Some(value) = snapshot.and_then(|snapshot| snapshot.get::<Puzzle5dWorldWindowConfigOwner>()) {
        return window_config_from_world(value);
    }
    Puzzle5dWindowConfig::default()
}

pub fn transient_from_view(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> Puzzle5dWindowTransient {
    view.window::<Puzzle5dBoardWindowTransientOwner>().or_else(|| view.window::<Puzzle5dWorldWindowTransientOwner>()).cloned().unwrap_or_default()
}

pub fn transient_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> Puzzle5dWindowTransient {
    snapshot.and_then(|value| value.get::<Puzzle5dBoardWindowTransientOwner>().or_else(|| value.get::<Puzzle5dWorldWindowTransientOwner>())).cloned().unwrap_or_default()
}

pub fn kind_for_view(view: &semio_framework_plugin::ViewModel) -> Option<&str> {
    let id = view.window_id.as_deref()?;
    view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str())
}

pub fn runtime(
    shared: &crate::editor::puzzle5d::config::Puzzle5dConfig,
    window: &Puzzle5dWindowConfig,
    transient: &Puzzle5dWindowTransient,
    window_id: &str,
) -> Puzzle5dRuntime {
    let mut runtime = Puzzle5dRuntime::default();
    runtime.contact_tolerance = shared.contact_tolerance;
    runtime.proximity_radius = shared.proximity_radius;
    runtime.chunk_size = shared.chunk_size;
    runtime.object_kind_weights = shared.object_kind_weights.clone();
    runtime.vortex_kind_weights = shared.vortex_kind_weights.clone();
    runtime.camera2d = window.camera2d.clone();
    runtime.camera3d = window.camera3d.clone();
    runtime.fill_count = shared.fill_count;
    runtime.lod_mode = window.lod_mode.clone();
    runtime.suggestion_offset = window.suggestion_offset;
    runtime.grid_snap_enabled = window.grid_snap_enabled;
    runtime.grid_factor = window.grid_factor;
    runtime.grid_visible = window.grid_visible;
    runtime.grid_spacing = window.grid_spacing;
    runtime.lod_automatic = window.lod_automatic;
    runtime.lod_depth_variable = window.lod_depth_variable;
    runtime.lod_manual = window.lod_manual;
    runtime.selectable_kinds = window.selectable_kinds.clone();
    runtime.grip_show = window.grip_show.clone();
    runtime.grip_direction = window.grip_direction.clone();
    runtime.transform_move = window.transform_move;
    runtime.transform_rotate = window.transform_rotate;
    runtime.sun = window.sun.clone();
    runtime.voxel_dims = window.voxel_dims;
    runtime.engagement_input_by_window.clear();
    runtime.engagement_input_by_window.insert(window_id.to_string(), transient.engagement_input.clone());
    runtime.brush_candidate_index = transient.brush_candidate_index;
    runtime.suggestion_menu = transient.suggestion_menu.clone();
    runtime
}

pub fn shared(runtime: &Puzzle5dRuntime) -> crate::editor::puzzle5d::config::Puzzle5dConfig {
    let mut config = crate::editor::puzzle5d::config::Puzzle5dConfig::default();
    config.fill_count = runtime.fill_count;
    config.contact_tolerance = runtime.contact_tolerance;
    config.proximity_radius = runtime.proximity_radius;
    config.chunk_size = runtime.chunk_size;
    config.object_kind_weights = runtime.object_kind_weights.clone();
    config.vortex_kind_weights = runtime.vortex_kind_weights.clone();
    config
}

pub fn config_from_runtime(runtime: &Puzzle5dRuntime) -> Puzzle5dWindowConfig {
    Puzzle5dWindowConfig {
        camera2d: runtime.camera2d.clone(),
        camera3d: runtime.camera3d.clone(),
        lod_mode: runtime.lod_mode.clone(),
        suggestion_offset: runtime.suggestion_offset,
        grid_snap_enabled: runtime.grid_snap_enabled,
        grid_factor: runtime.grid_factor,
        grid_visible: runtime.grid_visible,
        grid_spacing: runtime.grid_spacing,
        lod_automatic: runtime.lod_automatic,
        lod_depth_variable: runtime.lod_depth_variable,
        lod_manual: runtime.lod_manual,
        selectable_kinds: runtime.selectable_kinds.clone(),
        grip_show: runtime.grip_show.clone(),
        grip_direction: runtime.grip_direction.clone(),
        transform_move: runtime.transform_move,
        transform_rotate: runtime.transform_rotate,
        sun: runtime.sun.clone(),
        voxel_dims: runtime.voxel_dims,
    }
}

pub fn transient_from_runtime(runtime: &Puzzle5dRuntime, window_id: &str) -> Puzzle5dWindowTransient {
    Puzzle5dWindowTransient { suggestion_menu: runtime.suggestion_menu.clone(), engagement_input: runtime.engagement_input_by_window.get(window_id).cloned().unwrap_or_default(), brush_candidate_index: runtime.brush_candidate_index }
}

pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: Puzzle5dWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle5d-window-required"))?;
    match kind_for_view(view) {
        Some(PUZZLE5D_BOARD_WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle5dBoardWindowConfigOwner>(
            id,
            Puzzle5dBoardWindowConfigMutation::Set(Puzzle5dBoardWindowConfigMutationSet {
                patch: Puzzle5dBoardWindowConfigDiff::of(&Puzzle5dBoardWindowConfig {
                    camera2d: config.camera2d,
                    lod_mode: config.lod_mode,
                    suggestion_offset: config.suggestion_offset,
                    grid_snap_enabled: config.grid_snap_enabled,
                    grid_factor: config.grid_factor,
                    grid_visible: config.grid_visible,
                    selectable_kinds: config.selectable_kinds,
                }),
            }),
        )),
        Some(PUZZLE5D_WORLD_WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowConfigMutation::of::<Puzzle5dWorldWindowConfigOwner>(
            id,
            Puzzle5dWorldWindowConfigMutation::Set(Puzzle5dWorldWindowConfigMutationSet {
                patch: Puzzle5dWorldWindowConfigDiff::of(&Puzzle5dWorldWindowConfig {
                    camera3d: config.camera3d,
                    sun: config.sun,
                    voxel_dims: config.voxel_dims,
                    grid_visible: config.grid_visible,
                    grid_snap_enabled: config.grid_snap_enabled,
                    grid_spacing: config.grid_spacing,
                    lod_automatic: config.lod_automatic,
                    lod_depth_variable: config.lod_depth_variable,
                    lod_manual: config.lod_manual,
                    selectable_kinds: config.selectable_kinds,
                    grip_show: config.grip_show,
                    grip_direction: config.grip_direction,
                    transform_move: config.transform_move,
                    transform_rotate: config.transform_rotate,
                }),
            }),
        )),
        _ => Err(semio_framework_plugin::Fault::from("puzzle5d-window-kind-required")),
    }
}

pub fn addressed_transient(view: &semio_framework_plugin::ViewModel, transient: Puzzle5dWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("puzzle5d-window-required"))?;
    let mutation = Puzzle5dWindowTransientMutation::Snapshot { transient };
    match kind_for_view(view) {
        Some(PUZZLE5D_BOARD_WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowTransientMutation::of::<Puzzle5dBoardWindowTransientOwner>(id, mutation)),
        Some(PUZZLE5D_WORLD_WINDOW_KIND_ID) => Ok(semio_framework_plugin::WindowTransientMutation::of::<Puzzle5dWorldWindowTransientOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("puzzle5d-window-kind-required")),
    }
}
