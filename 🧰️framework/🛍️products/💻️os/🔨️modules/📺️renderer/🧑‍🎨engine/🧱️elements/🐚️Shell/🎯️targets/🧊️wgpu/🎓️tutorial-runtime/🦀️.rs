//! 🎓️ Tutorial runtime.
//!
//! Tutorial mode and runtime types, camera conversion, UI snapshots, the gesture overlay, recording and the
//! playhead/scrub helpers with their `impl ShellState`.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellChrome
//#region 🎬️Tutorial
// 🎬️ The wgpu-shell half of the Tutorial mechanism (sibling of `//#region 🔖️ChromeOverlaysAndTour`'s
// introduction tour, above) — see `framework/core/rs/lib.rs`'s `//#region 🔖️Tutorial` for the shared
// data model and pure engine fns (`tutorial_slice`/`compose_tutorial_ui`/`interpolate_tutorial_camera`/
// `tutorial_camera_at`/`apply_tutorial_ui_change`) this region calls directly rather than reimplementing.
// The React shell's own half lives in `framework/renderer/react/index.tsx`; both are built against the
// same core model but neither coordinates with the other beyond that shared semantics.

/// 🎬️ Live playback/recording state machine for `ShellState.tutorial`. `Deviated` is a distinct mode
/// (not just "paused") so the player remembers to converge the camera on resume (Design Decision 6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TutorialMode {
    Playing,
    Paused,
    Recording,
    Deviated,
}

/// 🎥️ A real-time (not timeline-time, not rate-scaled) camera glide from a live/deviated pose to the
/// recorded pose at the playhead the user pressed Play at — `TUTORIAL_CONVERGE_MS` long (Design
/// Decision 5). Reuses `interpolate_tutorial_camera` under the hood (see `tutorial_converge_pose`)
/// rather than reimplementing easing.
#[derive(Clone, Debug)]
pub struct TutorialCameraConverge {
    pub(crate) from: semio_framework::TutorialCameraState,
    pub(crate) to: semio_framework::TutorialCameraState,
    started_wall_ms: f64,
}

/// 🎬️ The active tutorial's full runtime state: the definition itself (recording IS a `TutorialDefinition`
/// — Design Decision 1), playback position/rate, the sandbox snapshot to restore on exit, and (for the
/// recorder) the bookkeeping needed to sample cameras/UI only on meaningful change.
#[derive(Clone)]
pub struct TutorialRuntime {
    pub definition: semio_framework::TutorialDefinition,
    pub mode: TutorialMode,
    pub playhead_ms: f64,
    pub rate: f32,
    /// ✂️ The playhead document/UI state was last synced to — `tutorial_slice(applied_ms, playhead_ms)`
    /// drives each tick's incremental apply; a seek instead jumps this straight to the target.
    applied_ms: f64,
    /// 📸️ The live document/UI as they stood the moment the tutorial sandboxed them — restored on exit
    /// (Design Decision 3). `None` for a recording, which is never sandboxed.
    pre_sandbox_document_dsl: Option<String>,
    pre_sandbox_ui: semio_framework::TutorialUiSnapshot,
    last_tick_wall_ms: f64,
    /// 🎥️ Active per-window convergence tweens (Deviated → Playing) — see `TutorialCameraConverge`.
    converge: HashMap<String, TutorialCameraConverge>,
    recorder_last_camera_wall_ms: HashMap<String, f64>,
    recorder_last_camera_pose: HashMap<String, semio_framework::TutorialCameraState>,
    recorder_last_ui: semio_framework::TutorialUiSnapshot,
    recorder_last_ui_sample_wall_ms: f64,
}

/// 🎬️ One document-track application queued by a tick/seek this frame — see
/// `ShellState::tutorial_pending_document_ops`'s own doc comment for why this has to be deferred rather
/// than applied inline (the plugin bridge's document calls are async, chrome rendering isn't).
#[derive(Clone, Debug)]
pub enum TutorialPendingDocOp {
    LoadArtifactDsl(String),
    ApplyOperations(Vec<String>),
    /// 🖋️ `Undo`/`Redo`/`Checkpoint`/`CheckoutCheckpoint`/`SwitchAlternative` all replay as a bare
    /// generic action dispatch (`"undo"`/`"redo"`/… against the session's `controller_id`) — the same
    /// convention `framework/plugin/rs`'s own `handle_action("undo", …)` test helpers already use, and
    /// the only "history action" mechanism reachable from here without inventing a second one.
    HistoryAction {
        action_id: String,
        args: Option<Value>,
    },
}

/// 🎬️ Reuses the navbar's own height token for the tutorial control bar (item 2's "reuse the existing
/// navbar height theme token" — no second style constant introduced).
pub(crate) fn tutorial_bar_height(theme: &Theme) -> f32 {
    theme.navbar_height
}

//#region 🎥️CameraConversion
/// 🎥️ `OrbitController` → `TutorialCameraState::Orbit`. `fov` is in **degrees**, matching
/// `World3dScene.camera_json`'s own wire format (`infinite_world`'s `WorldCameraRecord.fov` — see that
/// crate's `camera.fov.unwrap_or(45.0) as f32 * PI / 180.0` conversion the other way), not
/// `OrbitController.fov_y`'s radians.
fn orbit_to_tutorial_camera(orbit: &ui_wgpu::wgpu::OrbitController) -> semio_framework::TutorialCameraState {
    let camera = orbit.to_camera();
    semio_framework::TutorialCameraState::Orbit {
        position: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        target: [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
        up: [camera.up.x as f64, camera.up.y as f64, camera.up.z as f64],
        fov: Some((camera.fov_y as f64).to_degrees()),
    }
}

/// 🎥️ `TutorialCameraState` → `OrbitController`. `Canvas` (the 2D infinite-canvas camera kind) has no
/// orbit-controller equivalent — `None` (see the ticket's own scope note on 2D camera tracks).
fn tutorial_camera_to_orbit(state: &semio_framework::TutorialCameraState) -> Option<ui_wgpu::wgpu::OrbitController> {
    match state {
        semio_framework::TutorialCameraState::Orbit { position, target, up, fov } => Some(ui_wgpu::wgpu::OrbitController::from_camera(&ui_wgpu::wgpu::Camera3d {
            position: ui_wgpu::wgpu::Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32),
            target: ui_wgpu::wgpu::Vec3::new(target[0] as f32, target[1] as f32, target[2] as f32),
            up: ui_wgpu::wgpu::Vec3::new(up[0] as f32, up[1] as f32, up[2] as f32),
            fov_y: (fov.unwrap_or(45.0) as f32).to_radians(),
            near: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_NEAR,
            far: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_MIN_FAR,
            projection: ui_wgpu::wgpu::CameraProjection3d::Perspective,
            zoom: 1.0,
        })),
        semio_framework::TutorialCameraState::Canvas { .. } => None,
    }
}

fn tutorial_capture_camera_pose(state: &ShellState, window_id: &str) -> Option<semio_framework::TutorialCameraState> {
    state.world3d_state_for_window(window_id).or_else(|| state.icon_render_states.get(window_id)).map(|world| orbit_to_tutorial_camera(&world.orbit))
}

fn tutorial_apply_camera_pose(state: &mut ShellState, window_id: &str, pose: &semio_framework::TutorialCameraState) {
    let Some(orbit) = tutorial_camera_to_orbit(pose) else {
        return;
    };
    if let Some(world) = state.world3d_state_for_window_mut(window_id) {
        world.orbit = orbit.clone();
    }
    if let Some(world) = state.icon_render_states.get_mut(window_id) {
        world.orbit = orbit;
    }
}

/// 🎥️ Unique window ids across both `base.cameras` and `tracks.camera`, in first-seen order.
fn tutorial_camera_window_ids(def: &semio_framework::TutorialDefinition) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for keyframe in def.base.cameras.iter().chain(def.tracks.camera.iter()) {
        if !ids.contains(&keyframe.window_id) {
            ids.push(keyframe.window_id.clone());
        }
    }
    ids
}

fn tutorial_camera_pose_close(a: &semio_framework::TutorialCameraState, b: &semio_framework::TutorialCameraState, epsilon: f64) -> bool {
    use semio_framework::TutorialCameraState::*;
    match (a, b) {
        (Orbit { position: p0, target: t0, .. }, Orbit { position: p1, target: t1, .. }) => {
            let dist = |x: [f64; 3], y: [f64; 3]| ((x[0] - y[0]).powi(2) + (x[1] - y[1]).powi(2) + (x[2] - y[2]).powi(2)).sqrt();
            dist(*p0, *p1) < epsilon && dist(*t0, *t1) < epsilon
        }
        (Canvas { x: x0, y: y0, zoom: z0 }, Canvas { x: x1, y: y1, zoom: z1 }) => (x0 - x1).abs() < epsilon && (y0 - y1).abs() < epsilon && (z0 - z1).abs() < epsilon,
        _ => false,
    }
}

/// 🎥️ Resolves the real-time convergence tween's pose at `elapsed_ms` since it started, by reusing
/// `interpolate_tutorial_camera` over two synthetic keyframes at `0`/`TUTORIAL_CONVERGE_MS` — rather than
/// reimplementing easing for this one caller.
fn tutorial_converge_pose(tween: &TutorialCameraConverge, elapsed_ms: f64) -> semio_framework::TutorialCameraState {
    let from_keyframe = semio_framework::TutorialCameraKeyframe { at: 0, window_id: String::new(), camera: tween.from.clone(), easing: semio_framework::TutorialEasing::Hold };
    let to_keyframe = semio_framework::TutorialCameraKeyframe { at: semio_framework::TUTORIAL_CONVERGE_MS, window_id: String::new(), camera: tween.to.clone(), easing: semio_framework::TutorialEasing::EaseInOut };
    semio_framework::interpolate_tutorial_camera(&from_keyframe, &to_keyframe, elapsed_ms)
}
//#endregion 🎥️CameraConversion

//#region 🧮️UiSnapshot
/// 🧮️ `ShellState` → `TutorialUiSnapshot` (Design Decision 4).
fn tutorial_capture_ui_snapshot(state: &ShellState) -> semio_framework::TutorialUiSnapshot {
    let mut active_panel_tab_by_group: HashMap<String, String> = HashMap::new();
    for anchor in PanelAnchor::ALL {
        if let Some(tab) = state.panel_anchor_snapshot(anchor).active_tab {
            active_panel_tab_by_group.insert(anchor.as_str().to_string(), tab);
        }
    }
    semio_framework::TutorialUiSnapshot {
        active_mode_id: state.session.as_ref().and_then(|s| s.view_state.active_mode_id.clone()),
        focused_window_id: state.active_window_id.clone(),
        active_utility_by_window_id: state.active_utility_by_window.iter().map(|(key,value)|(key.clone(),value.clone())).collect(),
        active_tool_id: state.session.as_ref().and_then(|s| s.view_state.active_tool_id.clone()),
        layout: Some(state.dock.to_window_layout()),
        active_panel_tab_by_group,
        panel_json: state.session.as_ref().and_then(|s| s.view_state.panel_json.clone()),
        interaction_selection: state.interaction_selection.clone(),
        open_dialog_id: state.chrome_build.dialog_stack.last().map(|dialog| dialog.id.clone()),
        expanded_tree_ids: state.tree_open_states.iter().filter_map(|(id, open)| open.then(|| id.clone())).collect(),
        command_panel_open: state.search_open,
    }
}

/// 🧮️ `TutorialUiSnapshot` → `ShellState`, applied as a snap (single field writes — Design Decision 5).
pub(crate) fn tutorial_apply_ui_snapshot(state: &mut ShellState, snapshot: &semio_framework::TutorialUiSnapshot) {
    if let Some(session) = state.session.as_mut() {
        session.view_state.active_mode_id = snapshot.active_mode_id.clone();
        session.view_state.active_tool_id = snapshot.active_tool_id.clone();
        session.view_state.panel_json = snapshot.panel_json.clone();
    }
    state.active_window_id = snapshot.focused_window_id.clone();
    state.active_utility_by_window = snapshot.active_utility_by_window_id.clone();
    if let Some(layout) = &snapshot.layout {
        state.dock.apply_layout_diff(layout);
    }
    for anchor in PanelAnchor::ALL {
        match snapshot.active_panel_tab_by_group.get(anchor.as_str()) {
            Some(tab) => {
                let requested_path = state.dock_tabs.locate(tab).filter(|(located, _)| *located == anchor).map(|(_, path)| path).unwrap_or_else(|| vec![tab.clone()]);
                let path = state.dock_tabs.reconcile_path(anchor, &requested_path);
                let anchor_state = state.anchor_state_mut(anchor);
                anchor_state.path = path;
                anchor_state.visible = true;
            }
            None => state.anchor_state_mut(anchor).visible = false,
        }
    }
    state.set_interaction_selection_projection(snapshot.interaction_selection.clone());
    let known_tree_ids: Vec<String> = state.tree_open_states.keys().cloned().collect();
    for id in known_tree_ids {
        state.set_canonical_tree_open(&id, false);
        state.collapsed_sections.insert(format!("section.{id}"), true);
        state.collapsed_sections.insert(format!("tree.{id}"), true);
    }
    for id in &snapshot.expanded_tree_ids {
        state.set_canonical_tree_open(id, true);
        state.collapsed_sections.insert(format!("section.{id}"), false);
        state.collapsed_sections.insert(format!("tree.{id}"), false);
    }
    let requested_dialog = snapshot.open_dialog_id.as_deref().and_then(|id| state.chrome_dialog_request(id, None));
    state.chrome_build.dialog_stack.clear();
    if let Some(dialog) = requested_dialog {
        state.chrome_build.open_dialog(dialog);
    }
    state.set_search_open(snapshot.command_panel_open);
}

/// 🩹️ Applies one `TutorialUiChange` live — composed from `tutorial_capture_ui_snapshot` +
/// `apply_tutorial_ui_change` (core) + `tutorial_apply_ui_snapshot` rather than duplicating the change's
/// own per-field switch a second time.
fn tutorial_apply_ui_change_to_shell(state: &mut ShellState, change: &semio_framework::TutorialUiChange) {
    let mut snapshot = tutorial_capture_ui_snapshot(state);
    semio_framework::apply_tutorial_ui_change(&mut snapshot, change);
    tutorial_apply_ui_snapshot(state, &snapshot);
}

//#endregion 🧮️UiSnapshot

//#region 👻️GestureOverlay
/// 👻️ Resolves an `IntroductionPoint` to a viewport pixel through chrome geometry or the attached
/// surface owner's live camera and semantic geometry registry.
fn tutorial_resolve_gesture_point(state: &ShellState, point: &semio_framework::IntroductionPoint) -> Option<(f32, f32)> {
    use semio_framework::IntroductionPoint as P;
    match point {
        P::Screen { x, y } => Some((*x as f32, *y as f32)),
        P::ScreenNormalized { x, y } => Some((*x as f32 * state.screen_w, *y as f32 * state.screen_h)),
        P::Element { id, offset } => {
            let rect = state.chrome_build.resolve_element_rect(id)?;
            let [ox, oy] = offset.unwrap_or([0.5, 0.5]);
            Some((rect.x + rect.w * ox as f32, rect.y + rect.h * oy as f32))
        }
        P::Window { id, x, y } => {
            let rect = state.window_content_rects.get(id)?;
            Some((rect.x + *x as f32, rect.y + *y as f32))
        }
        P::WindowNormalized { id, x, y } => {
            let rect = state.window_content_rects.get(id)?;
            Some((rect.x + rect.w * (*x as f32), rect.y + rect.h * (*y as f32)))
        }
        P::Scene { id, position } => {
            let rect = state.window_content_rects.get(id).copied().or_else(|| state.world3d_state_for_window(id).map(|world| world.bounds))?;
            let local = infinite_world::world::world3d_tutorial_scene_point(state.world3d_state_for_window(id)?, *position)?;
            Some((rect.x + local[0], rect.y + local[1]))
        }
        P::Canvas { id, x, y } => {
            let rect = state.window_content_rects.get(id).copied().or_else(|| state.world3d_state_for_window(id).map(|world| world.bounds))?;
            let local = state
                .world3d_state_for_window(id)
                .and_then(|world| infinite_world::world::world3d_tutorial_scene_point(world, [*x, *y, 0.0]).map(|point| (point[0], point[1])))
                .or_else(|| state.engine_host_id_for_window(id).and_then(|host_id| crate::engine_canvas::resolve_tutorial_surface_point(host_id, point)))?;
            Some((rect.x + local.0, rect.y + local.1))
        }
        P::Entity { id, domain, entity, .. } => {
            let rect = state.window_content_rects.get(id).copied().or_else(|| state.world3d_state_for_window(id).map(|world| world.bounds))?;
            let local = state
                .world3d_state_for_window(id)
                .and_then(|world| infinite_world::world::world3d_tutorial_entity_geometry(world, domain, entity).map(|geometry| (geometry.point[0], geometry.point[1])))
                .or_else(|| state.engine_host_id_for_window(id).and_then(|host_id| crate::engine_canvas::resolve_tutorial_surface_point(host_id, point)))?;
            Some((rect.x + local.0, rect.y + local.1))
        }
        P::Curve { id, domain, entity, t } => {
            let rect = state.window_content_rects.get(id).copied().or_else(|| state.world3d_state_for_window(id).map(|world| world.bounds))?;
            let local = state
                .world3d_state_for_window(id)
                .and_then(|world| infinite_world::world::world3d_tutorial_entity_geometry(world, domain, entity))
                .and_then(|geometry| geometry.polyline.and_then(|polyline| crate::engine_canvas::tutorial_polyline_point(&polyline, *t)).map(|point| (point[0], point[1])))
                .or_else(|| state.engine_host_id_for_window(id).and_then(|host_id| crate::engine_canvas::resolve_tutorial_surface_point(host_id, point)))?;
            Some((rect.x + local.0, rect.y + local.1))
        }
        P::Domain { id, .. } => {
            let rect = *state.window_content_rects.get(id)?;
            let local = crate::engine_canvas::resolve_tutorial_surface_point(state.engine_host_id_for_window(id)?, point)?;
            Some((rect.x + local.0, rect.y + local.1))
        }
    }
}

fn tutorial_gesture_endpoints(gesture: &semio_framework::IntroductionGesture) -> (semio_framework::IntroductionPoint, semio_framework::IntroductionPoint) {
    use semio_framework::IntroductionGesture as G;
    match gesture {
        G::LeftClick { at } | G::RightClick { at } | G::DoubleClick { at } => (at.clone(), at.clone()),
        G::Scroll { at, .. } => (at.clone(), at.clone()),
        G::Drag { from, to, .. } | G::Orbit { from, to, .. } => (from.clone(), to.clone()),
    }
}

/// 👻️ Paints a simple ghost-cursor dot at the active `TutorialGestureCue`'s interpolated position (linear
/// by playhead progress within the cue) — the minimal demonstration painter neither this shell nor its
/// introduction mechanism had before (verified: no `demonstration`/ghost-cursor renderer existed here).
pub(crate) fn render_tutorial_gesture_overlay_node(state: &ShellState, overlay: &mut DrawList, theme: &Theme) {
    let Some(runtime) = state.tutorial.as_ref() else {
        return;
    };
    if !matches!(runtime.mode, TutorialMode::Playing | TutorialMode::Paused | TutorialMode::Deviated) {
        return;
    }
    let playhead = runtime.playhead_ms;
    let Some(cue) = runtime.definition.tracks.gestures.iter().find(|cue| {
        let at = cue.at as f64;
        playhead >= at && playhead <= at + cue.duration_ms as f64
    }) else {
        return;
    };
    let (from, to) = tutorial_gesture_endpoints(&cue.gesture);
    let Some((fx, fy)) = tutorial_resolve_gesture_point(state, &from) else {
        return;
    };
    let (tx, ty) = tutorial_resolve_gesture_point(state, &to).unwrap_or((fx, fy));
    let t = if cue.duration_ms == 0 { 1.0 } else { ((playhead - cue.at as f64) / cue.duration_ms as f64).clamp(0.0, 1.0) as f32 };
    let x = fx + (tx - fx) * t;
    let y = fy + (ty - fy) * t;
    let size = 18.0;
    overlay.push_rounded([x - size * 0.5, y - size * 0.5, size, size], theme.selected, size * 0.5);
    let inner = size - 6.0;
    overlay.push_rounded([x - inner * 0.5, y - inner * 0.5, inner, inner], theme.background, inner * 0.5);
}
//#endregion 👻️GestureOverlay

//#region 📅️Provenance
/// 📅️ Howard Hinnant's `civil_from_days` algorithm (http://howardhinnant.github.io/date_algorithms.html)
/// — days-since-1970-01-01 → proleptic-Gregorian `(year, month, day)`, dependency-free (no date crate in
/// this crate's `Cargo.toml`) for the recorder's `recordedAt` provenance stamp.
fn tutorial_civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn tutorial_recorded_at_iso() -> String {
    let total_secs = (chrome_now_ms() / 1000.0).floor() as i64;
    let days = total_secs.div_euclid(86400);
    let secs_of_day = total_secs.rem_euclid(86400);
    let (hour, minute, second) = (secs_of_day / 3600, (secs_of_day % 3600) / 60, secs_of_day % 60);
    let (year, month, day) = tutorial_civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// 💾️ Reuses the existing "download a file from the browser"/"save file" mechanism (`download_media_export`,
/// already used for GLB/OBJ/image exports) rather than inventing a second one for tutorial recordings.
fn tutorial_save_recording(tutorial_id: &str, json: &str) {
    download_media_export(&format!("{tutorial_id}.tutorial.json"), "application/json", json, None);
}
//#endregion 📅️Provenance

//#region ✂️PendingDocOps
/// ✂️ One `TutorialDocumentEvent` → the pending op(s) needed to apply it in `direction` — `forward` uses
/// `Edit::forwards`/dispatches the named history action as-is; backward uses `Edit::backwards`/inverts
/// the history action (undo↔redo) per `TutorialDocumentEventKind::Edit`'s own doc comment on exact
/// bidirectional scrubbing.
fn tutorial_pending_op_for_edit(entry: &semio_framework::TutorialDocumentEvent, forward: bool) -> TutorialPendingDocOp {
    use semio_framework::TutorialDocumentEventKind as K;
    match &entry.kind {
        K::Edit { forwards, backwards, .. } => {
            let ops = if forward { forwards } else { backwards };
            TutorialPendingDocOp::ApplyOperations(ops.iter().filter_map(|v| serde_json::to_string(v).ok()).collect())
        }
        K::Undo => TutorialPendingDocOp::HistoryAction { action_id: (if forward { "undo" } else { "redo" }).into(), args: None },
        K::Redo => TutorialPendingDocOp::HistoryAction { action_id: (if forward { "redo" } else { "undo" }).into(), args: None },
        K::Checkpoint { message } => TutorialPendingDocOp::HistoryAction { action_id: "checkpoint".into(), args: message.as_ref().map(|m| serde_json::json!({ "message": m })) },
        K::CheckoutCheckpoint { checkpoint_id } => TutorialPendingDocOp::HistoryAction { action_id: "checkoutCheckpoint".into(), args: Some(serde_json::json!({ "checkpointId": checkpoint_id })) },
        K::SwitchAlternative { alternative_id } => TutorialPendingDocOp::HistoryAction { action_id: "switchAlternative".into(), args: Some(serde_json::json!({ "alternativeId": alternative_id })) },
        K::Load { document_dsl, previous_dsl } => TutorialPendingDocOp::LoadArtifactDsl(if forward { document_dsl.clone() } else { previous_dsl.clone() }),
    }
}
//#endregion ✂️PendingDocOps

//#region 🎬️PureHelpers
/// ⏱️ Pure playhead-advance math (per-tick real-time `dt` scaled by the UI-only `rate` — Design
/// Decision 6): factored out of `tutorial_tick` for unit testing.
fn tutorial_advance_playhead(playhead_ms: f64, dt_ms: f64, rate: f32) -> f64 {
    playhead_ms + dt_ms * rate as f64
}

/// 🎚️ Pure scrub-bar progress (0–1) for a given playhead/duration — `0.0` for a zero-length timeline
/// (the in-progress recording case) rather than dividing by zero.
pub(crate) fn tutorial_scrub_progress(playhead_ms: f64, duration_ms: u64) -> f32 {
    if duration_ms == 0 {
        0.0
    } else {
        (playhead_ms / duration_ms as f64).clamp(0.0, 1.0) as f32
    }
}

/// 🎚️ Pure scrub-bar hit math: pointer x within `track` → target playhead ms.
pub(crate) fn tutorial_scrub_target_ms(pointer_x: f32, track: Rect, duration_ms: u64) -> f64 {
    let t = ((pointer_x - track.x) / track.w.max(1.0)).clamp(0.0, 1.0);
    t as f64 * duration_ms as f64
}

const TUTORIAL_FRAME_INTERVAL_MS: f64 = 1000.0 / 60.0;
//#endregion 🎬️PureHelpers

impl ShellState {
    /// ⏱️ Next wall-clock tick owed by the active tutorial. Playback and recording mirror
    /// React's self-scheduling `createTutorialClock`; camera convergence remains live even if its
    /// timeline mode is not currently advancing. Exact playback/convergence ends win over cadence.
    pub(crate) fn next_tutorial_deadline_ms(&self, now_wall_ms: f64) -> Option<f64> {
        if !now_wall_ms.is_finite() {
            return None;
        }
        let runtime = self.tutorial.as_ref()?;
        let timeline_active = matches!(runtime.mode, TutorialMode::Playing | TutorialMode::Recording);
        if !timeline_active && runtime.converge.is_empty() {
            return None;
        }
        let last_tick = if runtime.last_tick_wall_ms.is_finite() { runtime.last_tick_wall_ms } else { now_wall_ms };
        let mut deadline = last_tick + TUTORIAL_FRAME_INTERVAL_MS;
        if runtime.mode == TutorialMode::Playing && runtime.rate > 0.0 {
            let remaining_timeline_ms = (runtime.definition.duration_ms as f64 - runtime.playhead_ms).max(0.0) / f64::from(runtime.rate);
            deadline = deadline.min(last_tick + remaining_timeline_ms);
        }
        if let Some(converge_end) = runtime.converge.values().filter_map(|tween| tween.started_wall_ms.is_finite().then_some(tween.started_wall_ms + semio_framework::TUTORIAL_CONVERGE_MS as f64)).min_by(f64::total_cmp) {
            deadline = deadline.min(converge_end);
        }
        deadline.is_finite().then_some(deadline)
    }

    //#region 🎬️Lifecycle
    /// 🎬️ Starts (or restarts) `tutorial_id` for the active app: sandboxes the live document behind
    /// `base`, snaps UI/camera to `base`, and begins playback from `t=0` (Design Decisions 2/3).
    ///
    /// 🎬️ Introductions and tutorials are mutually exclusive (Design Decision 8).
    ///
    /// 🚧️ `last_envelope_dsl` is this shell's own best-effort stand-in for "the live document's full
    /// `ArtifactEnvelope` JSON" — there is no other reachable accessor for it from here.
    ///
    /// 🚧️ No "load example by id" plugin-bridge primitive is reachable from here without
    /// duplicating `apply_shell_uri`'s example-switch machinery — scoped out; the tutorial plays
    /// over whichever document is already loaded instead of sandboxing a fresh example copy.
    pub fn tutorial_start(&mut self, tutorial_id: &str) {
        let Some(session) = self.session.clone() else {
            return;
        };
        let Some(definition) = session.app.tutorials.iter().find(|t| t.id == tutorial_id).cloned() else {
            return;
        };
        self.chrome_build.skip_introduction();
        let pre_sandbox_ui = tutorial_capture_ui_snapshot(self);
        let pre_sandbox_document_dsl = self.last_envelope_dsl.clone();
        if let Some(document_dsl) = definition.base.document_dsl.clone() {
            self.tutorial_pending_document_ops.push(TutorialPendingDocOp::LoadArtifactDsl(document_dsl));
        } else if let Some(example_id) = definition.base.example_id.as_ref() {
            Self::debug_log(&format!("[TRACE] tutorial base.exampleId `{example_id}` sandbox load not wired (no base.documentJson) — playing over the live document"));
        }
        tutorial_apply_ui_snapshot(self, &definition.base.ui);
        for keyframe in &definition.base.cameras {
            tutorial_apply_camera_pose(self, &keyframe.window_id, &keyframe.camera);
        }
        self.tutorial = Some(TutorialRuntime {
            definition,
            mode: TutorialMode::Playing,
            playhead_ms: 0.0,
            rate: 1.0,
            applied_ms: 0.0,
            pre_sandbox_document_dsl,
            pre_sandbox_ui,
            last_tick_wall_ms: chrome_now_ms(),
            converge: HashMap::new(),
            recorder_last_camera_wall_ms: HashMap::new(),
            recorder_last_camera_pose: HashMap::new(),
            recorder_last_ui: semio_framework::TutorialUiSnapshot::default(),
            recorder_last_ui_sample_wall_ms: 0.0,
        });
    }

    /// ⏺️ Arms the recorder against the LIVE document (never sandboxed — a recording IS the user's work,
    /// Design Decision 2/3's sandbox note). Skips webcam/mic capture entirely (explicitly out of scope).
    pub fn tutorial_start_recording(&mut self) {
        if self.tutorial.is_some() {
            return;
        }
        self.chrome_build.skip_introduction();
        let ui = tutorial_capture_ui_snapshot(self);
        let mut cameras = Vec::new();
        for (window_id, world) in self.world3d_states.iter() {
            cameras.push(semio_framework::TutorialCameraKeyframe { at: 0, window_id: window_id.clone(), camera: orbit_to_tutorial_camera(&world.orbit), easing: semio_framework::TutorialEasing::Hold });
        }
        let now = chrome_now_ms();
        let definition = semio_framework::TutorialDefinition {
            id: format!("recording-{}", tutorial_recorded_at_iso().replace(['-', ':'], "")),
            title: LocalizedLabel::native("Recording", "Aufnahme"),
            description: None,
            duration_ms: 0,
            chapters: Vec::new(),
            base: semio_framework::TutorialBase { document_dsl: self.last_envelope_dsl.clone(), example_id: self.active_example_id.clone(), ui: ui.clone(), cameras },
            tracks: semio_framework::TutorialTracks::default(),
            recorded_at: None,
        };
        self.tutorial = Some(TutorialRuntime {
            definition,
            mode: TutorialMode::Recording,
            playhead_ms: 0.0,
            rate: 1.0,
            applied_ms: 0.0,
            pre_sandbox_document_dsl: None,
            pre_sandbox_ui: ui.clone(),
            last_tick_wall_ms: now,
            converge: HashMap::new(),
            recorder_last_camera_wall_ms: HashMap::new(),
            recorder_last_camera_pose: HashMap::new(),
            recorder_last_ui: ui,
            recorder_last_ui_sample_wall_ms: now,
        });
    }

    /// 🛑️ Ends playback (restoring the sandboxed document/UI, Design Decision 3) or, if recording, ends
    /// the take and serializes it (Design Decision 7: JSON via the existing download/save mechanism).
    pub fn tutorial_stop(&mut self) {
        let Some(runtime) = self.tutorial.take() else {
            return;
        };
        match runtime.mode {
            TutorialMode::Recording => {
                let mut definition = runtime.definition;
                definition.duration_ms = runtime.playhead_ms.max(0.0) as u64;
                definition.recorded_at = Some(tutorial_recorded_at_iso());
                match serde_json::to_string_pretty(&definition) {
                    Ok(json) => tutorial_save_recording(&definition.id, &json),
                    Err(err) => Self::debug_log(&format!("[TRACE] tutorial recording serialize failed: {err}")),
                }
            }
            _ => {
                tutorial_apply_ui_snapshot(self, &runtime.pre_sandbox_ui);
                if let Some(document_dsl) = runtime.pre_sandbox_document_dsl {
                    self.tutorial_pending_document_ops.push(TutorialPendingDocOp::LoadArtifactDsl(document_dsl));
                }
            }
        }
    }

    /// 🪄️ Deviation → Play: snap UI+document to the composed state at the current playhead, then
    /// start a real-time camera convergence tween per window (Design Decision 5/6).
    pub fn tutorial_toggle_play_pause(&mut self) {
        let Some(runtime) = self.tutorial.clone() else {
            return;
        };
        match runtime.mode {
            TutorialMode::Playing => {
                if let Some(r) = self.tutorial.as_mut() {
                    r.mode = TutorialMode::Paused;
                }
            }
            TutorialMode::Paused => {
                if let Some(r) = self.tutorial.as_mut() {
                    r.mode = TutorialMode::Playing;
                    r.last_tick_wall_ms = chrome_now_ms();
                }
            }
            TutorialMode::Deviated => {
                let target_ms = runtime.playhead_ms;
                let ui_state = semio_framework::compose_tutorial_ui(&runtime.definition, target_ms);
                tutorial_apply_ui_snapshot(self, &ui_state);
                let slice = semio_framework::tutorial_slice(&runtime.definition, runtime.applied_ms, target_ms);
                for entry in &slice.document {
                    self.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
                }
                let now = chrome_now_ms();
                let mut converge = HashMap::new();
                for window_id in tutorial_camera_window_ids(&runtime.definition) {
                    if let Some(to) = semio_framework::tutorial_camera_at(&runtime.definition, &window_id, target_ms) {
                        if let Some(from) = tutorial_capture_camera_pose(self, &window_id) {
                            converge.insert(window_id, TutorialCameraConverge { from, to, started_wall_ms: now });
                        }
                    }
                }
                if let Some(r) = self.tutorial.as_mut() {
                    r.mode = TutorialMode::Playing;
                    r.applied_ms = target_ms;
                    r.last_tick_wall_ms = now;
                    r.converge = converge;
                }
            }
            TutorialMode::Recording => {}
        }
    }

    /// ⏩️ Seeks to `target_ms`: UI applies wholesale via `compose_tutorial_ui`, document track entries
    /// apply forward/backward via `tutorial_slice` since the last-applied playhead, camera sets exactly
    /// via `tutorial_camera_at` (Design Decision item 5 of "what to implement").
    pub fn tutorial_seek(&mut self, target_ms: f64) {
        let Some(runtime) = self.tutorial.clone() else {
            return;
        };
        if runtime.mode == TutorialMode::Recording {
            return;
        }
        let target_ms = target_ms.clamp(0.0, runtime.definition.duration_ms as f64);
        let ui_state = semio_framework::compose_tutorial_ui(&runtime.definition, target_ms);
        tutorial_apply_ui_snapshot(self, &ui_state);
        let slice = semio_framework::tutorial_slice(&runtime.definition, runtime.applied_ms, target_ms);
        for entry in &slice.document {
            self.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
        }
        for window_id in tutorial_camera_window_ids(&runtime.definition) {
            if let Some(pose) = semio_framework::tutorial_camera_at(&runtime.definition, &window_id, target_ms) {
                tutorial_apply_camera_pose(self, &window_id, &pose);
            }
        }
        if let Some(r) = self.tutorial.as_mut() {
            r.playhead_ms = target_ms;
            r.applied_ms = target_ms;
            r.converge.clear();
            if r.mode == TutorialMode::Deviated {
                r.mode = TutorialMode::Paused;
            }
        }
    }

    /// 🎬️ Per-frame tick — advances the playhead (Playing), samples the recorder (Recording), applies
    /// annotational-free document/UI deltas since the last applied playhead, resolves active camera
    /// convergence tweens, and otherwise drives the camera track exactly via `tutorial_camera_at`.
    ///
    /// 🎥️ Convergence tweens win over the recorded pose while active; once a window's tween completes
    /// it falls through to `tutorial_camera_at` at the live playhead next tick.
    pub fn tutorial_tick(&mut self, now_wall_ms: f64) {
        let Some(mut runtime) = self.tutorial.clone() else {
            return;
        };
        let dt_wall_ms = (now_wall_ms - runtime.last_tick_wall_ms).max(0.0);
        runtime.last_tick_wall_ms = now_wall_ms;

        if runtime.mode == TutorialMode::Recording {
            runtime.playhead_ms = tutorial_advance_playhead(runtime.playhead_ms, dt_wall_ms, 1.0);
            runtime.definition.duration_ms = runtime.playhead_ms.max(0.0) as u64;
            tutorial_recorder_sample(self, &mut runtime, now_wall_ms);
            self.tutorial = Some(runtime);
            return;
        }

        if runtime.mode == TutorialMode::Playing {
            let from_ms = runtime.applied_ms;
            let total_ms = runtime.definition.duration_ms as f64;
            let mut to_ms = tutorial_advance_playhead(runtime.playhead_ms, dt_wall_ms, runtime.rate);
            let auto_paused = to_ms >= total_ms;
            if auto_paused {
                to_ms = total_ms;
            }
            runtime.playhead_ms = to_ms;
            let slice = semio_framework::tutorial_slice(&runtime.definition, from_ms, to_ms);
            for change in &slice.ui_changes {
                tutorial_apply_ui_change_to_shell(self, change);
            }
            for entry in &slice.document {
                self.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
            }
            runtime.applied_ms = to_ms;
            if auto_paused {
                runtime.mode = TutorialMode::Paused;
            }
        }

        let mut converged: Vec<String> = Vec::new();
        for (window_id, tween) in runtime.converge.iter() {
            let elapsed_ms = (now_wall_ms - tween.started_wall_ms).max(0.0);
            let pose = tutorial_converge_pose(tween, elapsed_ms.min(semio_framework::TUTORIAL_CONVERGE_MS as f64));
            tutorial_apply_camera_pose(self, window_id, &pose);
            if elapsed_ms >= semio_framework::TUTORIAL_CONVERGE_MS as f64 {
                converged.push(window_id.clone());
            }
        }
        for window_id in converged {
            runtime.converge.remove(&window_id);
        }
        if matches!(runtime.mode, TutorialMode::Playing | TutorialMode::Paused) {
            for window_id in tutorial_camera_window_ids(&runtime.definition) {
                if runtime.converge.contains_key(&window_id) {
                    continue;
                }
                if let Some(pose) = semio_framework::tutorial_camera_at(&runtime.definition, &window_id, runtime.playhead_ms) {
                    tutorial_apply_camera_pose(self, &window_id, &pose);
                }
            }
        }

        self.tutorial = Some(runtime);
    }

    /// 🎬️ Deviation detection (Playing → Deviated on any real dispatch, Design Decision 6) + the
    /// recorder's annotational event tap (Design Decision 7) — called once at the very top of
    /// `dispatch_action` for every dispatch NOT already filtered as tutorial-internal.
    ///
    /// 🎥️ Camera changes are sampled directly from the orbit controller (`tutorial_recorder_sample`),
    /// never re-recorded as an annotational event too.
    ///
    /// 🕒️ Session-only command-history log rows, never part of a tutorial's own replayable
    /// action track (mirrors the `setCamera` skip just above).
    pub(crate) fn tutorial_note_real_dispatch(&mut self, action: &ActionDescriptor) {
        let Some(runtime) = self.tutorial.as_mut() else {
            return;
        };
        match runtime.mode {
            TutorialMode::Playing => {
                runtime.mode = TutorialMode::Deviated;
            }
            TutorialMode::Recording => {
                if action.action == "setCamera" {
                    return;
                }
                if action.action == "noteShellCommand" {
                    return;
                }
                let at = runtime.playhead_ms.max(0.0) as u64;
                runtime.definition.tracks.events.push(semio_framework::TutorialEvent { at, kind: semio_framework::TutorialEventKind::Action { action: action.action.clone(), args: action.args.clone() } });
            }
            TutorialMode::Paused | TutorialMode::Deviated => {}
        }
    }

    /// 🎬️ Drains `tutorial_pending_document_ops` (queued by a tick/seek this frame) and applies each
    /// through the existing plugin-bridge document mechanisms — the async counterpart to the sync
    /// tick/seek above, called from `AppRuntime::frame` right after `render_chrome` (mirrors how `frame`
    /// already defers `scene_events` the same way, for the same sync/async split).
    ///
    /// 🚧️ `TutorialBase.document_json` is always `None` fleet-wide today (no tutorial
    /// definition populates it, and `last_envelope_dsl` — its only non-`None` source — is
    /// itself never set past its `None` default); a real loader needs the tutorial-content
    /// dsl-text conversion this plan's B5 tutorial-track bullet scopes separately, not a
    /// whole-envelope JSON reader (deleted with `PluginApp::load_document`/`document_dsl`).
    pub async fn tutorial_flush_pending_document_ops(&mut self) {
        if self.tutorial_pending_document_ops.is_empty() {
            return;
        }
        let ops = std::mem::take(&mut self.tutorial_pending_document_ops);
        self.chrome_build.tutorial_dispatch_internal = true;
        for op in ops {
            match op {
                TutorialPendingDocOp::LoadArtifactDsl(_json) => {
                    Self::debug_log(&format!("[TRACE] tutorial load document (json) not wired to the pack-only plugin bridge"));
                }
                TutorialPendingDocOp::ApplyOperations(operations) => {
                    if let Err(err) = self.apply_mutations(&operations, UiDirtyScope::Full).await {
                        Self::debug_log(&format!("[TRACE] tutorial apply operations failed: {err}"));
                    }
                }
                TutorialPendingDocOp::HistoryAction { action_id, args } => {
                    if let Some(session) = self.session.clone() {
                        let descriptor = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: action_id, args: args.map(semio_framework::DslValue::from) };
                        if let Err(err) = self.dispatch_action(descriptor).await {
                            Self::debug_log(&format!("[TRACE] tutorial history action failed: {err}"));
                        }
                    }
                }
            }
        }
        self.chrome_build.tutorial_dispatch_internal = false;
    }
    //#endregion 🎬️Lifecycle

    //#region 🎬️Chrome

    //#endregion 🎬️Chrome
}

/// 🎥️ Recorder-only sampling: the active window's camera on meaningful change (not every frame), and a
/// periodic full UI snapshot (Design Decision 7's "minimal recorder" scope).
fn tutorial_recorder_sample(state: &mut ShellState, runtime: &mut TutorialRuntime, now_wall_ms: f64) {
    const CAMERA_SAMPLE_MIN_INTERVAL_MS: f64 = 150.0;
    const CAMERA_MOVE_EPSILON: f64 = 0.01;
    const UI_SAMPLE_INTERVAL_MS: f64 = 2000.0;

    if let Some(window_id) = state.active_window_id.clone() {
        if let Some(pose) = tutorial_capture_camera_pose(state, &window_id) {
            let last_sample_ms = runtime.recorder_last_camera_wall_ms.get(&window_id).copied().unwrap_or(f64::NEG_INFINITY);
            let changed = runtime.recorder_last_camera_pose.get(&window_id).map(|prev| !tutorial_camera_pose_close(prev, &pose, CAMERA_MOVE_EPSILON)).unwrap_or(true);
            if changed && now_wall_ms - last_sample_ms >= CAMERA_SAMPLE_MIN_INTERVAL_MS {
                runtime.definition.tracks.camera.push(semio_framework::TutorialCameraKeyframe { at: runtime.playhead_ms.max(0.0) as u64, window_id: window_id.clone(), camera: pose.clone(), easing: semio_framework::TutorialEasing::EaseInOut });
                runtime.recorder_last_camera_wall_ms.insert(window_id.clone(), now_wall_ms);
                runtime.recorder_last_camera_pose.insert(window_id, pose);
            }
        }
    }
    if now_wall_ms - runtime.recorder_last_ui_sample_wall_ms >= UI_SAMPLE_INTERVAL_MS {
        let snapshot = tutorial_capture_ui_snapshot(state);
        if snapshot != runtime.recorder_last_ui {
            runtime.definition.tracks.ui.push(semio_framework::TutorialUiKeyframe { at: runtime.playhead_ms.max(0.0) as u64, sample: semio_framework::TutorialUiSample::Snapshot { state: Box::new(snapshot.clone()) } });
            runtime.recorder_last_ui = snapshot;
        }
        runtime.recorder_last_ui_sample_wall_ms = now_wall_ms;
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-tutorial/🦀️.rs"]
mod tutorial_tests;
//#endregion 🎬️Tutorial
//#endregion ShellChrome
