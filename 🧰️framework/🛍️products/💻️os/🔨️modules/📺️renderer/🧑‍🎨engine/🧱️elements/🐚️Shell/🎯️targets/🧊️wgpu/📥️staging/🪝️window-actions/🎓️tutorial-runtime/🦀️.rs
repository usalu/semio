//! 🚶️ `shell-session` staging of `tutorial-runtime`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🎬️Tutorial
//#region 🎥️CameraConversion
/// 🎥️ Unique window ids across both `base.cameras` and `tracks.camera`, in first-seen order.
fn tutorial_camera_window_ids(def: &semio_framework::onboarding::TutorialDefinition) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for keyframe in def.base.cameras.iter().chain(def.tracks.camera.iter()) {
        if !ids.contains(&keyframe.window_id) {
            ids.push(keyframe.window_id.clone());
        }
    }
    ids
}

pub fn tutorial_camera_pose_close(a: &semio_framework::onboarding::TutorialCameraState, b: &semio_framework::onboarding::TutorialCameraState, epsilon: f64) -> bool {
    use semio_framework::onboarding::TutorialCameraState::*;
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
fn tutorial_converge_pose(tween: &TutorialCameraConverge, elapsed_ms: f64) -> semio_framework::onboarding::TutorialCameraState {
    let from_keyframe = semio_framework::onboarding::TutorialCameraKeyframe { at: 0, window_id: String::new(), camera: tween.from.clone(), easing: semio_framework::onboarding::TutorialEasing::Hold };
    let to_keyframe = semio_framework::onboarding::TutorialCameraKeyframe { at: semio_framework::onboarding::TUTORIAL_CONVERGE_MS, window_id: String::new(), camera: tween.to.clone(), easing: semio_framework::onboarding::TutorialEasing::EaseInOut };
    semio_framework::onboarding::interpolate_tutorial_camera(&from_keyframe, &to_keyframe, elapsed_ms)
}

//#endregion 🎥️CameraConversion
//#region 🧮️UiSnapshot
/// 🧮️ `ShellState` → `TutorialUiSnapshot` (Design Decision 4).
pub fn tutorial_capture_ui_snapshot(state: &WindowActionsCtx<'_>) -> semio_framework::onboarding::TutorialUiSnapshot {
    let mut active_panel_tab_by_group: HashMap<String, String> = HashMap::new();
    for anchor in PanelAnchor::ALL {
        if let Some(tab) = state.dock.panel_anchor_snapshot(anchor).active_tab {
            active_panel_tab_by_group.insert(anchor.as_str().to_string(), tab);
        }
    }
    semio_framework::onboarding::TutorialUiSnapshot {
        active_mode_id: state.docs.session.as_ref().and_then(|s| s.view_state.active_mode_id.clone()),
        focused_window_id: state.windows.active_window_id.clone(),
        active_utility_by_window_id: state.windows.active_utility_by_window.iter().map(|(key,value)|(key.clone(),value.clone())).collect(),
        active_tool_id: state.docs.session.as_ref().and_then(|s| s.view_state.active_tool_id.clone()),
        layout: Some(state.dock.dock.to_window_layout()),
        active_panel_tab_by_group,
        panel_json: state.docs.session.as_ref().and_then(|s| s.view_state.panel_json.clone()),
        interaction_selection: state.input.interaction_selection.clone(),
        open_dialog_id: state.chrome.chrome_build.dialog_stack.last().map(|dialog| dialog.id.clone()),
        expanded_tree_ids: state.input.tree_open_states.iter().filter_map(|(id, open)| open.then(|| id.clone())).collect(),
        command_panel_open: state.commands.search_open,
    }
}

/// 🧮️ `TutorialUiSnapshot` → `ShellState`, applied as a snap (single field writes — Design Decision 5).
pub fn tutorial_apply_ui_snapshot(state: &mut WindowActionsCtx<'_>, snapshot: &semio_framework::onboarding::TutorialUiSnapshot) {
    if let Some(session) = state.docs.session.as_mut() {
        session.view_state.active_mode_id = snapshot.active_mode_id.clone();
        session.view_state.active_tool_id = snapshot.active_tool_id.clone();
        session.view_state.panel_json = snapshot.panel_json.clone();
    }
    state.windows.active_window_id = snapshot.focused_window_id.clone();
    state.windows.active_utility_by_window = snapshot.active_utility_by_window_id.clone();
    if let Some(layout) = &snapshot.layout {
        state.dock.dock.apply_layout_diff(layout);
    }
    for anchor in PanelAnchor::ALL {
        match snapshot.active_panel_tab_by_group.get(anchor.as_str()) {
            Some(tab) => {
                let requested_path = state.dock.dock_tabs.locate(tab).filter(|(located, _)| *located == anchor).map(|(_, path)| path).unwrap_or_else(|| vec![tab.clone()]);
                let path = state.dock.dock_tabs.reconcile_path(anchor, &requested_path);
                let anchor_state = state.dock.anchor_state_mut(anchor);
                anchor_state.path = path;
                anchor_state.visible = true;
            }
            None => state.dock.anchor_state_mut(anchor).visible = false,
        }
    }
    state.input.set_interaction_selection_projection(snapshot.interaction_selection.clone());
    let known_tree_ids: Vec<String> = state.input.tree_open_states.keys().cloned().collect();
    for id in known_tree_ids {
        state.input.set_canonical_tree_open(&id, false);
        state.input.collapsed_sections.insert(format!("section.{id}"), true);
        state.input.collapsed_sections.insert(format!("tree.{id}"), true);
    }
    for id in &snapshot.expanded_tree_ids {
        state.input.set_canonical_tree_open(id, true);
        state.input.collapsed_sections.insert(format!("section.{id}"), false);
        state.input.collapsed_sections.insert(format!("tree.{id}"), false);
    }
    let requested_dialog = snapshot.open_dialog_id.as_deref().and_then(|id| state.docs.chrome_dialog_request(id, None, &state.prefs));
    state.chrome.chrome_build.dialog_stack.clear();
    if let Some(dialog) = requested_dialog {
        state.chrome.chrome_build.open_dialog(dialog);
    }
    state.commands.set_search_open(snapshot.command_panel_open, &mut state.input);
}

/// 🩹️ Applies one `TutorialUiChange` live — composed from `tutorial_capture_ui_snapshot` +
/// `apply_tutorial_ui_change` (core) + `tutorial_apply_ui_snapshot` rather than duplicating the change's
/// own per-field switch a second time.
pub fn tutorial_apply_ui_change_to_shell(state: &mut WindowActionsCtx<'_>, change: &semio_framework::onboarding::TutorialUiChange) {
    let mut snapshot = tutorial_capture_ui_snapshot(state);
    semio_framework::onboarding::apply_tutorial_ui_change(&mut snapshot, change);
    tutorial_apply_ui_snapshot(state, &snapshot);
}

//#endregion 🧮️UiSnapshot
//#region 👻️GestureOverlay
fn tutorial_gesture_endpoints(gesture: &semio_framework::onboarding::IntroductionGesture) -> (semio_framework::onboarding::IntroductionPoint, semio_framework::onboarding::IntroductionPoint) {
    use semio_framework::onboarding::IntroductionGesture as G;
    match gesture {
        G::LeftClick { at } | G::RightClick { at } | G::DoubleClick { at } => (at.clone(), at.clone()),
        G::Scroll { at, .. } => (at.clone(), at.clone()),
        G::Drag { from, to, .. } | G::Orbit { from, to, .. } => (from.clone(), to.clone()),
    }
}

/// 👻️ Paints a simple ghost-cursor dot at the active `TutorialGestureCue`'s interpolated position (linear
/// by playhead progress within the cue) — the minimal demonstration painter neither this shell nor its
/// introduction mechanism had before (verified: no `demonstration`/ghost-cursor renderer existed here).
pub fn render_tutorial_gesture_overlay_node(chrome: &ChromeState, surfaces: &SurfaceState, dock: &ShellDockState, tutorial: &TutorialState, overlay: &mut DrawList, theme: &Theme) {
    let Some(runtime) = tutorial.tutorial.as_ref() else {
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
    let Some((fx, fy)) = tutorial_resolve_gesture_point(chrome, surfaces, dock, &from) else {
        return;
    };
    let (tx, ty) = tutorial_resolve_gesture_point(chrome, surfaces, dock, &to).unwrap_or((fx, fy));
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
fn tutorial_pending_op_for_edit(entry: &semio_framework::onboarding::TutorialDocumentEvent, forward: bool) -> TutorialPendingDocOp {
    use semio_framework::onboarding::TutorialDocumentEventKind as K;
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
pub fn tutorial_advance_playhead(playhead_ms: f64, dt_ms: f64, rate: f32) -> f64 {
    playhead_ms + dt_ms * rate as f64
}

//#endregion 🎬️PureHelpers
impl<'ctx> WindowActionsCtx<'ctx> {
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
        let Some(session) = self.docs.session.clone() else {
            return;
        };
        let Some(definition) = session.app.tutorials.iter().find(|t| t.id == tutorial_id).cloned() else {
            return;
        };
        self.chrome.chrome_build.skip_introduction();
        let pre_sandbox_ui = tutorial_capture_ui_snapshot(self);
        let pre_sandbox_document_dsl = self.opening.last_envelope_dsl.clone();
        if let Some(document_dsl) = definition.base.document_dsl.clone() {
            self.tutorial.tutorial_pending_document_ops.push(TutorialPendingDocOp::LoadArtifactDsl(document_dsl));
        } else if let Some(example_id) = definition.base.example_id.as_ref() {
            debug_log(&format!("[TRACE] tutorial base.exampleId `{example_id}` sandbox load not wired (no base.documentJson) — playing over the live document"));
        }
        tutorial_apply_ui_snapshot(self, &definition.base.ui);
        for keyframe in &definition.base.cameras {
            tutorial_apply_camera_pose(&mut self.surfaces, &keyframe.window_id, &keyframe.camera);
        }
        self.tutorial.tutorial = Some(TutorialRuntime {
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
            recorder_last_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
            recorder_last_ui_sample_wall_ms: 0.0,
        });
    }

    /// ⏺️ Arms the recorder against the LIVE document (never sandboxed — a recording IS the user's work,
    /// Design Decision 2/3's sandbox note). Skips webcam/mic capture entirely (explicitly out of scope).
    pub fn tutorial_start_recording(&mut self) {
        if self.tutorial.tutorial.is_some() {
            return;
        }
        self.chrome.chrome_build.skip_introduction();
        let ui = tutorial_capture_ui_snapshot(self);
        let mut cameras = Vec::new();
        for (window_id, world) in self.surfaces.world3d_states.iter() {
            cameras.push(semio_framework::onboarding::TutorialCameraKeyframe { at: 0, window_id: window_id.clone(), camera: orbit_to_tutorial_camera(&world.orbit), easing: semio_framework::onboarding::TutorialEasing::Hold });
        }
        let now = chrome_now_ms();
        let definition = semio_framework::onboarding::TutorialDefinition {
            id: format!("recording-{}", tutorial_recorded_at_iso().replace(['-', ':'], "")),
            title: LocalizedLabel::native("Recording", "Aufnahme"),
            description: None,
            duration_ms: 0,
            chapters: Vec::new(),
            base: semio_framework::onboarding::TutorialBase { document_dsl: self.opening.last_envelope_dsl.clone(), example_id: self.plugins.active_example_id.clone(), ui: ui.clone(), cameras },
            tracks: semio_framework::onboarding::TutorialTracks::default(),
            recorded_at: None,
        };
        self.tutorial.tutorial = Some(TutorialRuntime {
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
        let Some(runtime) = self.tutorial.tutorial.take() else {
            return;
        };
        match runtime.mode {
            TutorialMode::Recording => {
                let mut definition = runtime.definition;
                definition.duration_ms = runtime.playhead_ms.max(0.0) as u64;
                definition.recorded_at = Some(tutorial_recorded_at_iso());
                match serde_json::to_string_pretty(&definition) {
                    Ok(json) => tutorial_save_recording(&definition.id, &json),
                    Err(err) => debug_log(&format!("[TRACE] tutorial recording serialize failed: {err}")),
                }
            }
            _ => {
                tutorial_apply_ui_snapshot(self, &runtime.pre_sandbox_ui);
                if let Some(document_dsl) = runtime.pre_sandbox_document_dsl {
                    self.tutorial.tutorial_pending_document_ops.push(TutorialPendingDocOp::LoadArtifactDsl(document_dsl));
                }
            }
        }
    }

    /// 🪄️ Deviation → Play: snap UI+document to the composed state at the current playhead, then
    /// start a real-time camera convergence tween per window (Design Decision 5/6).
    pub fn tutorial_toggle_play_pause(&mut self) {
        let Some(runtime) = self.tutorial.tutorial.clone() else {
            return;
        };
        match runtime.mode {
            TutorialMode::Playing => {
                if let Some(r) = self.tutorial.tutorial.as_mut() {
                    r.mode = TutorialMode::Paused;
                }
            }
            TutorialMode::Paused => {
                if let Some(r) = self.tutorial.tutorial.as_mut() {
                    r.mode = TutorialMode::Playing;
                    r.last_tick_wall_ms = chrome_now_ms();
                }
            }
            TutorialMode::Deviated => {
                let target_ms = runtime.playhead_ms;
                let ui_state = semio_framework::onboarding::compose_tutorial_ui(&runtime.definition, target_ms);
                tutorial_apply_ui_snapshot(self, &ui_state);
                let slice = semio_framework::onboarding::tutorial_slice(&runtime.definition, runtime.applied_ms, target_ms);
                for entry in &slice.document {
                    self.tutorial.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
                }
                let now = chrome_now_ms();
                let mut converge = HashMap::new();
                for window_id in tutorial_camera_window_ids(&runtime.definition) {
                    if let Some(to) = semio_framework::onboarding::tutorial_camera_at(&runtime.definition, &window_id, target_ms) {
                        if let Some(from) = tutorial_capture_camera_pose(&self.surfaces, &window_id) {
                            converge.insert(window_id, TutorialCameraConverge { from, to, started_wall_ms: now });
                        }
                    }
                }
                if let Some(r) = self.tutorial.tutorial.as_mut() {
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
        let Some(runtime) = self.tutorial.tutorial.clone() else {
            return;
        };
        if runtime.mode == TutorialMode::Recording {
            return;
        }
        let target_ms = target_ms.clamp(0.0, runtime.definition.duration_ms as f64);
        let ui_state = semio_framework::onboarding::compose_tutorial_ui(&runtime.definition, target_ms);
        tutorial_apply_ui_snapshot(self, &ui_state);
        let slice = semio_framework::onboarding::tutorial_slice(&runtime.definition, runtime.applied_ms, target_ms);
        for entry in &slice.document {
            self.tutorial.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
        }
        for window_id in tutorial_camera_window_ids(&runtime.definition) {
            if let Some(pose) = semio_framework::onboarding::tutorial_camera_at(&runtime.definition, &window_id, target_ms) {
                tutorial_apply_camera_pose(&mut self.surfaces, &window_id, &pose);
            }
        }
        if let Some(r) = self.tutorial.tutorial.as_mut() {
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
        let Some(mut runtime) = self.tutorial.tutorial.clone() else {
            return;
        };
        let dt_wall_ms = (now_wall_ms - runtime.last_tick_wall_ms).max(0.0);
        runtime.last_tick_wall_ms = now_wall_ms;

        if runtime.mode == TutorialMode::Recording {
            runtime.playhead_ms = tutorial_advance_playhead(runtime.playhead_ms, dt_wall_ms, 1.0);
            runtime.definition.duration_ms = runtime.playhead_ms.max(0.0) as u64;
            tutorial_recorder_sample(self, &mut runtime, now_wall_ms);
            self.tutorial.tutorial = Some(runtime);
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
            let slice = semio_framework::onboarding::tutorial_slice(&runtime.definition, from_ms, to_ms);
            for change in &slice.ui_changes {
                tutorial_apply_ui_change_to_shell(self, change);
            }
            for entry in &slice.document {
                self.tutorial.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
            }
            runtime.applied_ms = to_ms;
            if auto_paused {
                runtime.mode = TutorialMode::Paused;
            }
        }

        let mut converged: Vec<String> = Vec::new();
        for (window_id, tween) in runtime.converge.iter() {
            let elapsed_ms = (now_wall_ms - tween.started_wall_ms).max(0.0);
            let pose = tutorial_converge_pose(tween, elapsed_ms.min(semio_framework::onboarding::TUTORIAL_CONVERGE_MS as f64));
            tutorial_apply_camera_pose(&mut self.surfaces, window_id, &pose);
            if elapsed_ms >= semio_framework::onboarding::TUTORIAL_CONVERGE_MS as f64 {
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
                if let Some(pose) = semio_framework::onboarding::tutorial_camera_at(&runtime.definition, &window_id, runtime.playhead_ms) {
                    tutorial_apply_camera_pose(&mut self.surfaces, &window_id, &pose);
                }
            }
        }

        self.tutorial.tutorial = Some(runtime);
    }
    //#endregion 🎬️Lifecycle
}

/// 🎥️ Recorder-only sampling: the active window's camera on meaningful change (not every frame), and a
/// periodic full UI snapshot (Design Decision 7's "minimal recorder" scope).
fn tutorial_recorder_sample(state: &mut WindowActionsCtx<'_>, runtime: &mut TutorialRuntime, now_wall_ms: f64) {
    const CAMERA_SAMPLE_MIN_INTERVAL_MS: f64 = 150.0;
    const CAMERA_MOVE_EPSILON: f64 = 0.01;
    const UI_SAMPLE_INTERVAL_MS: f64 = 2000.0;

    if let Some(window_id) = state.windows.active_window_id.clone() {
        if let Some(pose) = tutorial_capture_camera_pose(&state.surfaces, &window_id) {
            let last_sample_ms = runtime.recorder_last_camera_wall_ms.get(&window_id).copied().unwrap_or(f64::NEG_INFINITY);
            let changed = runtime.recorder_last_camera_pose.get(&window_id).map(|prev| !tutorial_camera_pose_close(prev, &pose, CAMERA_MOVE_EPSILON)).unwrap_or(true);
            if changed && now_wall_ms - last_sample_ms >= CAMERA_SAMPLE_MIN_INTERVAL_MS {
                runtime.definition.tracks.camera.push(semio_framework::onboarding::TutorialCameraKeyframe { at: runtime.playhead_ms.max(0.0) as u64, window_id: window_id.clone(), camera: pose.clone(), easing: semio_framework::onboarding::TutorialEasing::EaseInOut });
                runtime.recorder_last_camera_wall_ms.insert(window_id.clone(), now_wall_ms);
                runtime.recorder_last_camera_pose.insert(window_id, pose);
            }
        }
    }
    if now_wall_ms - runtime.recorder_last_ui_sample_wall_ms >= UI_SAMPLE_INTERVAL_MS {
        let snapshot = tutorial_capture_ui_snapshot(state);
        if snapshot != runtime.recorder_last_ui {
            runtime.definition.tracks.ui.push(semio_framework::onboarding::TutorialUiKeyframe { at: runtime.playhead_ms.max(0.0) as u64, sample: semio_framework::onboarding::TutorialUiSample::Snapshot { state: Box::new(snapshot.clone()) } });
            runtime.recorder_last_ui = snapshot;
        }
        runtime.recorder_last_ui_sample_wall_ms = now_wall_ms;
    }
}

//#endregion 🎬️Tutorial
//#endregion ShellChrome


