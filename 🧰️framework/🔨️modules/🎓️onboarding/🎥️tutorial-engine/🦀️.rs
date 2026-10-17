//! 🎥️ Tutorial engine: the start and record actions, the validation, the camera interpolation, the interface composition and the tutorial slice. See [`super`].

use super::*;

//#region 🔖️Tutorial
/// 🎬️ The reserved action id apps dispatch to (re)start a tutorial — injected by the app builder as a
/// fully shell-intercepted View action into any `AppDefinition` that declares one (mirrors
/// `START_INTRODUCTION_ACTION_ID`). Distinct from an introduction: a tutorial takes a required
/// `tutorialId` argument since an app may declare more than one.
pub const START_TUTORIAL_ACTION_ID: &str = "startTutorial";

/// 🎬️ The reserved `startTutorial` View action: fully shell-intercepted, it sandboxes
/// the live document, loads the selected tutorial's `base`, and starts playback from t=0.
pub fn start_tutorial_action_definition(tutorials: &[TutorialDefinition]) -> ActionDefinition {
    let mut options = Vec::with_capacity(tutorials.len());
    for t in tutorials {
        options.push(ActionArgOption::new(t.id.clone(), t.title.clone()));
    }
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(START_TUTORIAL_ACTION_ID, LocalizedLabel::native("Play Tutorial", "Tutorial abspielen"), ActionKind::View, "eye") }.with_args([ActionArgDef::select(
        "tutorialId",
        LocalizedLabel::native("Tutorial", "Tutorial"),
        options,
    )
    .required()])
}

/// ⏺️ The reserved action id that opens the tutorial recorder chrome — injected by the app builder into
/// EVERY `AppDefinition` (recording needs no app-side declaration at all).
pub const RECORD_TUTORIAL_ACTION_ID: &str = "recordTutorial";

/// ⏺️ The reserved `recordTutorial` View action: fully shell-intercepted, arms the
/// recorder against the live document (never a sandboxed copy — a recording IS the user's work).
pub fn record_tutorial_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(RECORD_TUTORIAL_ACTION_ID, LocalizedLabel::native("Record Tutorial", "Tutorial aufzeichnen"), ActionKind::View, "eye") }
        .describe(LocalizedLabel::native("Opens the tutorial recorder on the open document so the next interactions are recorded as a tutorial.", "Öffnet den Tutorial-Rekorder für das geöffnete Dokument, damit die nächsten Interaktionen als Tutorial aufgezeichnet werden."))
}

/// ⏱️ Real-time (not timeline-time, not rate-scaled) duration of the camera glide the player performs
/// when the user presses Play after deviating from an active tutorial's recorded state.
pub const TUTORIAL_CONVERGE_MS: u64 = 600;

//#region 🔖️TutorialEngine
/// ✅️ Structural validation shared by the plugin builder and both recorders before save: every
/// track sorted ascending by `at`, every entry within `[0, durationMs]`, chapter/narration-cue ids
/// unique, `base.cameras` all at `at == 0`. Does NOT check that referenced action/command/element ids
/// exist — the plugin builder's validation (which has the full `AppDefinition` in scope) does that.
pub fn validate_tutorial(def: &TutorialDefinition) -> Result<(), String> {
    fn sorted_by_at<T>(label: &str, items: &[T], at: impl Fn(&T) -> u64, duration_ms: u64) -> Result<(), String> {
        let mut last: Option<u64> = None;
        for item in items {
            let at = at(item);
            if at > duration_ms {
                return Err(format!("tutorial track `{label}` has an entry at {at}ms beyond durationMs {duration_ms}"));
            }
            if let Some(last) = last {
                if at < last {
                    return Err(format!("tutorial track `{label}` is not sorted ascending by `at` ({last}ms then {at}ms)"));
                }
            }
            last = Some(at);
        }
        Ok(())
    }

    sorted_by_at("chapters", &def.chapters, |c| c.at, def.duration_ms)?;
    sorted_by_at("narration", &def.tracks.narration, |c| c.at, def.duration_ms)?;
    sorted_by_at("video", &def.tracks.video, |c| c.at, def.duration_ms)?;
    sorted_by_at("events", &def.tracks.events, |e| e.at, def.duration_ms)?;
    sorted_by_at("ui", &def.tracks.ui, |k| k.at, def.duration_ms)?;
    sorted_by_at("document", &def.tracks.document, |e| e.at, def.duration_ms)?;
    sorted_by_at("camera", &def.tracks.camera, |k| k.at, def.duration_ms)?;
    sorted_by_at("gestures", &def.tracks.gestures, |c| c.at, def.duration_ms)?;

    let mut chapter_ids = std::collections::HashSet::new();
    for chapter in &def.chapters {
        if !chapter_ids.insert(chapter.id.as_str()) {
            return Err(format!("duplicate tutorial chapter id `{}`", chapter.id));
        }
    }
    let mut cue_ids = std::collections::HashSet::new();
    for cue in &def.tracks.narration {
        if !cue_ids.insert(cue.id.as_str()) {
            return Err(format!("duplicate tutorial narration cue id `{}`", cue.id));
        }
    }
    for camera in &def.base.cameras {
        if camera.at != 0 {
            return Err(format!("tutorial base camera keyframe for window `{}` must have at == 0", camera.window_id));
        }
    }
    Ok(())
}

fn tutorial_ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

fn tutorial_lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// 🎥️ Interpolates between two camera keyframes at timeline offset `at_ms` (clamped into
/// `[prev.at, next.at]`). Position/target/up/fov lerp componentwise; `Canvas.zoom` interpolates in log
/// space so zooming reads as constant visual speed. `next.easing` governs the curve; `Hold` snaps to
/// `prev` until `next.at`, then jumps. Mismatched camera kinds between the two keyframes (`Orbit` vs
/// `Canvas` on the same window) never interpolate — the result snaps to whichever side `t` is closer to.
pub fn interpolate_tutorial_camera(prev: &TutorialCameraKeyframe, next: &TutorialCameraKeyframe, at_ms: f64) -> TutorialCameraState {
    let span = (next.at as f64 - prev.at as f64).max(1.0);
    let raw = ((at_ms - prev.at as f64) / span).clamp(0.0, 1.0);
    let t = match next.easing {
        TutorialEasing::Linear => raw,
        TutorialEasing::EaseInOut => tutorial_ease_in_out(raw),
        TutorialEasing::Hold => {
            if raw >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
    };
    match (&prev.camera, &next.camera) {
        (TutorialCameraState::Orbit { position: p0, target: t0, up: u0, fov: f0 }, TutorialCameraState::Orbit { position: p1, target: t1, up: u1, fov: f1 }) => TutorialCameraState::Orbit {
            position: tutorial_lerp3(*p0, *p1, t),
            target: tutorial_lerp3(*t0, *t1, t),
            up: tutorial_lerp3(*u0, *u1, t),
            fov: match (f0, f1) {
                (Some(a), Some(b)) => Some(a + (b - a) * t),
                (Some(a), None) => Some(*a),
                (None, Some(b)) => Some(*b),
                (None, None) => None,
            },
        },
        (TutorialCameraState::Canvas { x: x0, y: y0, zoom: z0 }, TutorialCameraState::Canvas { x: x1, y: y1, zoom: z1 }) => TutorialCameraState::Canvas { x: x0 + (x1 - x0) * t, y: y0 + (y1 - y0) * t, zoom: (z0.ln() + (z1.ln() - z0.ln()) * t).exp() },
        _ => {
            if t < 0.5 {
                prev.camera.clone()
            } else {
                next.camera.clone()
            }
        }
    }
}

/// 🎥️ Finds the camera pose for `window_id` at `at_ms`: exact if `at_ms` lands on or before the
/// first keyframe (falling back to `base.cameras`), interpolated between the bracketing pair otherwise,
/// held at the last pose past the final keyframe. `None` when the window has no camera keyframes at all.
pub fn tutorial_camera_at(def: &TutorialDefinition, window_id: &str, at_ms: f64) -> Option<TutorialCameraState> {
    let keyframes: Vec<&TutorialCameraKeyframe> = def.base.cameras.iter().chain(def.tracks.camera.iter()).filter(|k| k.window_id == window_id).collect();
    let first = keyframes.first()?;
    if at_ms <= first.at as f64 {
        return Some(first.camera.clone());
    }
    for pair in keyframes.windows(2) {
        let (prev, next) = (pair[0], pair[1]);
        if at_ms <= next.at as f64 {
            return Some(interpolate_tutorial_camera(prev, next, at_ms));
        }
    }
    Some(keyframes.last().unwrap().camera.clone())
}

/// 🩹️ Applies one `TutorialUiChange` onto a `TutorialUiSnapshot` in place — the pure core both
/// `compose_tutorial_ui` and each shell's live director share.
pub fn apply_tutorial_ui_change(state: &mut TutorialUiSnapshot, change: &TutorialUiChange) {
    match change {
        TutorialUiChange::ActiveMode { id } => state.active_mode_id = Some(id.clone()),
        TutorialUiChange::FocusedWindow { id } => state.focused_window_id = id.clone(),
        TutorialUiChange::ActiveUtility { window_id, utility_id } => match utility_id {
            Some(id) => {
                state.active_utility_by_window_id.insert(window_id.clone(), id.clone());
            }
            None => {
                state.active_utility_by_window_id.remove(window_id);
            }
        },
        TutorialUiChange::ActiveTool { id } => state.active_tool_id = id.clone(),
        TutorialUiChange::Layout { layout } => state.layout = Some(layout.clone()),
        TutorialUiChange::PanelTab { group, tab_id } => match tab_id {
            Some(id) => {
                state.active_panel_tab_by_group.insert(group.clone(), id.clone());
            }
            None => {
                state.active_panel_tab_by_group.remove(group);
            }
        },
        TutorialUiChange::PanelState { panel_json } => state.panel_json = Some(panel_json.clone()),
        TutorialUiChange::Selection { domain_id, granularity, ids } => {
            if ids.is_empty() {
                state.interaction_selection.remove(domain_id);
            } else {
                state.interaction_selection.insert(domain_id.clone(), DomainSelection { granularity: granularity.clone(), ids: ids.clone(), anchor_id: None });
            }
        }
        TutorialUiChange::Dialog { id, .. } => state.open_dialog_id = id.clone(),
        TutorialUiChange::TreeExpansion { id, expanded } => {
            if *expanded {
                if !state.expanded_tree_ids.iter().any(|existing| existing == id) {
                    state.expanded_tree_ids.push(id.clone());
                }
            } else {
                state.expanded_tree_ids.retain(|existing| existing != id);
            }
        }
        TutorialUiChange::CommandPanel { open } => state.command_panel_open = *open,
    }
}

/// 🧮️ Reconstructs the full `TutorialUiSnapshot` at `at_ms`: starts from `base.ui`, then the
/// latest `Snapshot` sample with `at <= at_ms` (if any, replacing the base), then replays every `Delta`
/// sample after that snapshot up to and including `at_ms`, in order. This is the one place seeking (and
/// the deviation-then-play converge step) source their target UI state.
pub fn compose_tutorial_ui(def: &TutorialDefinition, at_ms: f64) -> TutorialUiSnapshot {
    let mut state = def.base.ui.clone();
    let mut deltas: Vec<&TutorialUiChange> = Vec::new();
    for keyframe in &def.tracks.ui {
        if keyframe.at as f64 > at_ms {
            break;
        }
        match &keyframe.sample {
            TutorialUiSample::Snapshot { state: snapshot } => {
                state = snapshot.as_ref().clone();
                deltas.clear();
            }
            TutorialUiSample::Delta { changes } => {
                deltas.extend(changes.iter());
            }
        }
    }
    for change in deltas {
        apply_tutorial_ui_change(&mut state, change);
    }
    state
}

/// ✂️ Everything a live director's tick from `from_ms` to `to_ms` must apply: annotational
/// events, document edits, and UI deltas within the half-open interval on the crossing direction (empty
/// when `from_ms == to_ms`). Backward direction (scrubbing left) reverses entry order so callers apply
/// each `TutorialDocumentEventKind::Edit`'s `backwards` ops from most-recent to least-recent. Plain Rust
/// struct (not owned schema mirrored) — the TS port lives in `framework/renderer/react/index.tsx` and is pinned
/// to this one via shared golden fixtures, not a wasm call per frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TutorialSlice {
    pub forward: bool,
    pub events: Vec<TutorialEvent>,
    pub document: Vec<TutorialDocumentEvent>,
    pub ui_changes: Vec<TutorialUiChange>,
}

/// ✂️ Computes the `TutorialSlice` for advancing the playhead from `from_ms` to `to_ms` (`to_ms`
/// may be less than `from_ms` when scrubbing backward).
///
/// 🐢️ A `TutorialUiSample::Snapshot` crossed mid-slice is intentionally NOT flattened into deltas here:
/// recomposing state across a snapshot boundary is exactly what `compose_tutorial_ui` already does
/// correctly and cheaply. This function is for the live per-tick advance, which never spans a snapshot
/// in practice (ticks run far more often than the multi-second snapshot cadence); any caller that jumps
/// across a snapshot boundary (a seek/scrub) should call `compose_tutorial_ui` wholesale instead of
/// accumulating through this slice.
pub fn tutorial_slice(def: &TutorialDefinition, from_ms: f64, to_ms: f64) -> TutorialSlice {
    let forward = to_ms >= from_ms;
    let (lo, hi) = if forward { (from_ms, to_ms) } else { (to_ms, from_ms) };
    let in_range = |at: u64| (at as f64) > lo && (at as f64) <= hi;

    let mut events: Vec<TutorialEvent> = def.tracks.events.iter().filter(|e| in_range(e.at)).cloned().collect();
    let mut document: Vec<TutorialDocumentEvent> = def.tracks.document.iter().filter(|e| in_range(e.at)).cloned().collect();
    let mut ui_changes: Vec<TutorialUiChange> = Vec::new();
    for keyframe in def.tracks.ui.iter().filter(|k| in_range(k.at)) {
        if let TutorialUiSample::Delta { changes } = &keyframe.sample {
            ui_changes.extend(changes.iter().cloned());
        }
    }
    if !forward {
        events.reverse();
        document.reverse();
        ui_changes.reverse();
    }
    TutorialSlice { forward, events, document, ui_changes }
}
//#endregion 🔖️TutorialEngine
//#endregion 🔖️Tutorial
