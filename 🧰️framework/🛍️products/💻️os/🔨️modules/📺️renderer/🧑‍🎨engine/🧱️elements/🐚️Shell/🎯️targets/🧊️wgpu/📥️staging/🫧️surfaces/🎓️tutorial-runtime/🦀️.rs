//! 🫧️ `shell-surfaces` staging of `tutorial-runtime`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🎬️Tutorial
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
    pub(crate) from: semio_framework::onboarding::TutorialCameraState,
    pub(crate) to: semio_framework::onboarding::TutorialCameraState,
    pub(super) started_wall_ms: f64,
}

/// 🎬️ The active tutorial's full runtime state: the definition itself (recording IS a `TutorialDefinition`
/// — Design Decision 1), playback position/rate, the sandbox snapshot to restore on exit, and (for the
/// recorder) the bookkeeping needed to sample cameras/UI only on meaningful change.
#[derive(Clone)]
pub struct TutorialRuntime {
    pub definition: semio_framework::onboarding::TutorialDefinition,
    pub mode: TutorialMode,
    pub playhead_ms: f64,
    pub rate: f32,
    /// ✂️ The playhead document/UI state was last synced to — `tutorial_slice(applied_ms, playhead_ms)`
    /// drives each tick's incremental apply; a seek instead jumps this straight to the target.
    pub(super) applied_ms: f64,
    /// 📸️ The live document/UI as they stood the moment the tutorial sandboxed them — restored on exit
    /// (Design Decision 3). `None` for a recording, which is never sandboxed.
    pub(super) pre_sandbox_document_dsl: Option<String>,
    pub(super) pre_sandbox_ui: semio_framework::onboarding::TutorialUiSnapshot,
    pub(super) last_tick_wall_ms: f64,
    /// 🎥️ Active per-window convergence tweens (Deviated → Playing) — see `TutorialCameraConverge`.
    pub(super) converge: HashMap<String, TutorialCameraConverge>,
    pub(super) recorder_last_camera_wall_ms: HashMap<String, f64>,
    pub(super) recorder_last_camera_pose: HashMap<String, semio_framework::onboarding::TutorialCameraState>,
    pub(super) recorder_last_ui: semio_framework::onboarding::TutorialUiSnapshot,
    pub(super) recorder_last_ui_sample_wall_ms: f64,
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
pub(super) fn orbit_to_tutorial_camera(orbit: &ui_wgpu::wgpu::OrbitController) -> semio_framework::onboarding::TutorialCameraState {
    let camera = orbit.to_camera();
    semio_framework::onboarding::TutorialCameraState::Orbit {
        position: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        target: [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
        up: [camera.up.x as f64, camera.up.y as f64, camera.up.z as f64],
        fov: Some((camera.fov_y as f64).to_degrees()),
    }
}

/// 🎥️ `TutorialCameraState` → `OrbitController`. `Canvas` (the 2D infinite-canvas camera kind) has no
/// orbit-controller equivalent — `None` (see the ticket's own scope note on 2D camera tracks).
pub(super) fn tutorial_camera_to_orbit(state: &semio_framework::onboarding::TutorialCameraState) -> Option<ui_wgpu::wgpu::OrbitController> {
    match state {
        semio_framework::onboarding::TutorialCameraState::Orbit { position, target, up, fov } => Some(ui_wgpu::wgpu::OrbitController::from_camera(&ui_wgpu::wgpu::Camera3d {
            position: ui_wgpu::wgpu::Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32),
            target: ui_wgpu::wgpu::Vec3::new(target[0] as f32, target[1] as f32, target[2] as f32),
            up: ui_wgpu::wgpu::Vec3::new(up[0] as f32, up[1] as f32, up[2] as f32),
            fov_y: (fov.unwrap_or(45.0) as f32).to_radians(),
            near: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_NEAR,
            far: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_MIN_FAR,
            projection: ui_wgpu::wgpu::CameraProjection3d::Perspective,
            zoom: 1.0,
        })),
        semio_framework::onboarding::TutorialCameraState::Canvas { .. } => None,
    }
}

pub(super) fn tutorial_capture_camera_pose(surfaces: &SurfaceState, window_id: &str) -> Option<semio_framework::onboarding::TutorialCameraState> {
    surfaces.world3d_state_for_window(window_id).or_else(|| surfaces.icon_render_states.get(window_id)).map(|world| orbit_to_tutorial_camera(&world.orbit))
}

pub(super) fn tutorial_apply_camera_pose(surfaces: &mut SurfaceState, window_id: &str, pose: &semio_framework::onboarding::TutorialCameraState) {
    let Some(orbit) = tutorial_camera_to_orbit(pose) else {
        return;
    };
    if let Some(world) = surfaces.world3d_state_for_window_mut(window_id) {
        world.orbit = orbit.clone();
    }
    if let Some(world) = surfaces.icon_render_states.get_mut(window_id) {
        world.orbit = orbit;
    }
}

//#endregion 🎥️CameraConversion
//#region 🎬️PureHelpers
const TUTORIAL_FRAME_INTERVAL_MS: f64 = 1000.0 / 60.0;

//#endregion 🎬️PureHelpers
impl TutorialState {
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
        if let Some(converge_end) = runtime.converge.values().filter_map(|tween| tween.started_wall_ms.is_finite().then_some(tween.started_wall_ms + semio_framework::onboarding::TUTORIAL_CONVERGE_MS as f64)).min_by(f64::total_cmp) {
            deadline = deadline.min(converge_end);
        }
        deadline.is_finite().then_some(deadline)
    }

    //#region 🎬️Lifecycle
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
                runtime.definition.tracks.events.push(semio_framework::onboarding::TutorialEvent { at, kind: semio_framework::onboarding::TutorialEventKind::Action { action: action.action.clone(), args: action.args.clone() } });
            }
            TutorialMode::Paused | TutorialMode::Deviated => {}
        }
    }
    //#endregion 🎬️Lifecycle
}

//#endregion 🎬️Tutorial
//#endregion ShellChrome


