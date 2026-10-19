//! 🪟️ Windows, utilities and interaction: the fill tool and window id helpers, the utility map hit, the interaction snapshot and the number edits and focus selection. See [`super`].

use super::*;

/// 🪣️ Whether the mode-level Fill tool currently authorizes fill planning and interaction.
pub fn puzzle3d_fill_tool_active(config: &Puzzle3dRuntime) -> bool {
    config.active_tool_id.as_deref() == Some(PUZZLE3D_FILL_TOOL_ID)
}

/// 🪟️ The window INSTANCE one render/dispatch is addressed at, in the order the host actually
/// carries it: an explicit `<body>:<instance>` body key, then `ViewModel::window_id` — the field
/// `ViewModel::for_window_instance` stamps and the ONLY per-call carrier of the instance identity —
/// then `ViewModel::focused_window_id`, then the caller's own fallback (a command's `windowId`
/// argument, which may still spell a bare window KIND), then the live roster's first pane.
///
/// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B15: the focused-pane hop is what an APP-LEVEL PANEL
/// resolves through. A panel body is never rendered FOR a window (`ViewModel::for_panel` clears
/// `window_id` by design), so before it this function fell straight to the roster's first entry —
/// which is the base window KIND, a pane nobody is looking at — and every Settings stepper baked THAT
/// into its args (B12 §5.1 measured `setGridSpacing window=Some("puzzle3d-main")` live).
///
/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B9: `ViewModel::active_utility_by_window_id`,
/// the window-owned config partitions and the per-window options are ALL keyed by instance
/// (`puzzle3d-main-perspective`), so resolving the roster's first pane before the host's own
/// `window_id` published the FIRST pane's armed utility into every other pane's world lane.
///
/// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B18: a kind-valued `window_id` (`puzzle3d-main`)
/// is not an addressing hit when an instance candidate exists — a window command otherwise looks the
/// utility map up under the kind (`map_hit=false`) and never sees the pane's armed utility.
pub fn puzzle3d_window_id_is_kind(id: &str) -> bool {
    id == semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID
}

fn puzzle3d_first_instance_id<'a>(ids: impl IntoIterator<Item = Option<&'a str>>) -> Option<&'a str> {
    ids.into_iter().flatten().find(|id| !id.is_empty() && !puzzle3d_window_id_is_kind(id))
}

fn puzzle3d_utility_map_hit<'a>(view: &'a semio_framework_plugin::ViewModel, wid: &str) -> Option<&'a String> {
    view.active_utility_by_window_id.get(wid).or_else(|| {
        if !puzzle3d_window_id_is_kind(wid) {
            return None;
        }
        view.focused_window_id
            .as_deref()
            .and_then(|focused| view.active_utility_by_window_id.get(focused))
            .or_else(|| {
                view.active_utility_by_window_id.iter().find_map(|(key, utility)| {
                    (key.starts_with(wid) && key.as_str() != wid && !utility.is_empty()).then_some(utility)
                })
            })
    })
}

pub fn puzzle3d_addressed_window_id<'a>(view_state: Option<&'a semio_framework_plugin::ViewModel>, keyed: Option<&'a str>, fallback: Option<&'a str>, roster: &'a [String]) -> &'a str {
    puzzle3d_first_instance_id([
        keyed,
        view_state.and_then(|view| view.window_id.as_deref()),
        view_state.and_then(|view| view.focused_window_id.as_deref()),
        fallback,
        roster.iter().map(String::as_str).find(|id| !id.is_empty() && !puzzle3d_window_id_is_kind(id)),
    ])
    .or_else(|| keyed.filter(|id| !id.is_empty()))
    .or_else(|| view_state.and_then(|view| view.window_id.as_deref()).filter(|id| !id.is_empty()))
    .or_else(|| view_state.and_then(|view| view.focused_window_id.as_deref()).filter(|id| !id.is_empty()))
    .or(fallback.filter(|id| !id.is_empty()))
    .or_else(|| roster.first().map(String::as_str).filter(|id| !id.is_empty()))
    .unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)
}

/// 🛠️ The shared window owner resolves pane utilities; the mode-wide fill tool retains its viewport interaction.
pub fn puzzle3d_scene_active_utility(config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> String {
    if puzzle3d_fill_tool_active(config) {
        return fill_tool::TOOL_ID.to_string();
    }
    view_state
        .and_then(|view| view.window_utility(window_id))
        .map(str::to_string)
        .unwrap_or_else(|| PUZZLE3D_DEFAULT_UTILITY.to_string())
}

pub fn puzzle3d_brush_target_vortex(envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> Option<String> {
    if let Some(id) = interaction.selected_vortex_ids().first() {
        return Some(id.clone());
    }
    if let Some(id) = interaction.hovered_vortex_full_id(&envelope.scene_snapshot) {
        return Some(id.to_string());
    }
    let hovered_object = interaction.hovered_object_id(&envelope.scene_snapshot)?;
    let object = envelope.scene_snapshot.objects.iter().find(|object| object.id == hovered_object)?;
    object.vortices.first().map(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id))
}

/// 🕹️ One render pass' read of the framework-owned `vortex` interaction domain (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): the selected ids at the granularity they were
/// picked at, plus the pointer-channel hover. `DomainSelection` carries exactly one live granularity,
/// so this mirrors [`Puzzle3dActionCtx::selected_object_ids`] and friends verbatim — render and
/// reducer read the same shape, never two divergent projections.
///
/// 🐁️ Hover ids arrive WITHOUT a granularity (`protocol::DomainHover` has none), so classifying one
/// needs the document: `hovered_object_id`/`hovered_vortex_full_id` resolve against the live scene_snapshot
/// rather than guessing from the id's spelling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle3dInteractionSnapshot {
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
    pub referenced: Vec<String>,
}

impl Puzzle3dInteractionSnapshot {
    /// 🕹️ Reads the live `vortex` domain: its selection, its `"pointer"`-channel hover and the ids the open time-travel
    /// draft references (what the world paints highlighted while a history edit is open).
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let selection = interaction.selection(PUZZLE3D_INTERACTION_DOMAIN);
        let hover = interaction.hover(PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_HOVER_CHANNEL);
        let leftover_ids = interaction.leftover_selected_ids();
        let selected = if selection.ids.is_empty() { leftover_ids } else { selection.ids.clone() };
        let granularity = if !selection.granularity.is_empty() { selection.granularity.clone() } else if !selected.is_empty() { PUZZLE3D_GRANULARITY_OBJECT.to_string() } else { selection.granularity.clone() };
        Self { granularity, selected, hovered: hover.ids.clone(), referenced: interaction.draft_references(PUZZLE3D_INTERACTION_DOMAIN).to_vec() }
    }

    /// 🕹️ The retained-reducer twin — the same read from the raw `InteractionState`/hover map a
    /// retained tool job is handed, so a command reusing a render helper sees the same selection and
    /// hover the viewport painted.
    pub fn from_state(state: &protocol::InteractionState, hover: &semio_framework_plugin::app::InteractionHoverState) -> Self {
        let selection = state.selection.get(PUZZLE3D_INTERACTION_DOMAIN);
        let hovered = hover.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|hover| hover.channel == PUZZLE3D_HOVER_CHANNEL).map(|hover| hover.ids.clone()).unwrap_or_default();
        let leftover_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        let selected = selection.filter(|selection| !selection.ids.is_empty()).map(|selection| selection.ids.clone()).unwrap_or(leftover_ids);
        let granularity = selection.map(|selection| selection.granularity.clone()).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| if selected.is_empty() { String::new() } else { PUZZLE3D_GRANULARITY_OBJECT.to_string() });
        Self { granularity, selected, hovered, referenced: Vec::new() }
    }

    pub fn selected_ids(&self, granularity: &str) -> &[String] {
        if self.granularity == granularity {
            &self.selected
        } else {
            &[]
        }
    }
    pub fn selected_object_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_OBJECT)
    }
    pub fn selected_vortex_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_VORTEX)
    }
    pub fn selected_attraction_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_ATTRACTION)
    }
    pub fn selected_target_volume_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_TARGET_VOLUME)
    }
    pub fn selected_reference_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE3D_GRANULARITY_REFERENCE)
    }

    /// 🐁️ The hovered object id, resolved against the scene_snapshot's own object ids.
    pub fn hovered_object_id(&self, scene_snapshot: &Puzzle3dSceneSnapshot) -> Option<&str> {
        self.hovered.iter().find(|id| scene_snapshot.objects.iter().any(|object| &object.id == *id)).map(String::as_str)
    }

    /// 🐁️ The hovered vortex full id (`{objectId}:{vortexId}`), resolved against the scene_snapshot.
    pub fn hovered_vortex_full_id(&self, scene_snapshot: &Puzzle3dSceneSnapshot) -> Option<&str> {
        self.hovered.iter().find(|id| scene_snapshot.objects.iter().any(|object| object.vortices.iter().any(|vortex| &puzzle3d_vortex_full_id(&object.id, &vortex.id) == *id))).map(String::as_str)
    }

    /// 🐁️ The hovered reference id, resolved against the scene_snapshot.
    pub fn hovered_reference_id(&self, scene_snapshot: &Puzzle3dSceneSnapshot) -> Option<&str> {
        self.hovered.iter().find(|id| scene_snapshot.references.iter().any(|reference| &reference.id == *id)).map(String::as_str)
    }

    /// 👁️ Whether `object_id` is itself selected/hovered, or owns a selected/hovered vortex — the
    /// predicate the `PUZZLE3D_VORTEX_SHOW_SELECTED` marker mode is gated on.
    pub fn touches_object(&self, object: &Puzzle3dObject) -> bool {
        let mut marks = self.selected.iter().chain(self.hovered.iter());
        marks.any(|id| id == &object.id || object.vortices.iter().any(|vortex| &puzzle3d_vortex_full_id(&object.id, &vortex.id) == id))
    }

    /// 🕹️ Whether anything at all is marked — the cheap "is there a selection" test the gumball and
    /// the inspection panel's empty state both need.
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty() && self.hovered.is_empty()
    }
}

pub fn value_as_vec3(value: &Value) -> Option<[f64; 3]> {
    let array = value.as_array()?;
    Some([array.first()?.as_f64()?, array.get(1)?.as_f64()?, array.get(2)?.as_f64()?])
}

/** 📐️ Resolves one numeric-field edit: an absolute `value` (typed entry) wins when present,
 * otherwise a `delta` (stepper nudge) is added to `current` — offset-preserving across a multi-select
 * where `current` differs per entity. `None` when neither parses. */
pub fn puzzle3d_resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

/** 📐️ Settings counterpart to `puzzle3d_resolve_number_edit`: reads `value`/`delta` directly
 * out of an action's `args`, for single global settings (not per-entity multi-select) whose stepper
 * dispatches straight to their own dedicated action. */
pub fn puzzle3d_absolute_or_delta(args: Option<&Value>, current: f64) -> Option<f64> {
    puzzle3d_resolve_number_edit(current, args.and_then(|value| value.get("value")), args.and_then(|value| value.get("delta")))
}

/** 📐️ Parses a nested stepper-group field id as `"<base>.<axis>"` (`x`/`y`/`z`/`w`), returning
 * the axis index when `field` names a component of `base` — the dot-path convention
 * `ui_inspector_vec3_group`/the inspector's quaternion group use for their per-axis actions. */
pub fn puzzle3d_axis_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        "w" => Some(3),
        _ => None,
    }
}

/// 🧲️ The world offset an inspector stepper nudge states — `patchInspector{entity: object|targetVolume,
/// field: origin.x|y|z, delta}` without an absolute `value` — the one inspector edit that is a gesture, so it
/// commits the transform tool's `drag-selection` instead of an absolute origin. `None` for every other edit.
pub fn puzzle3d_inspector_origin_nudge(args: Option<&Value>) -> Option<[f64; 3]> {
    let args = args?;
    if !matches!(args.get("entity").and_then(Value::as_str), Some("object" | "targetVolume")) || args.get("value").is_some_and(|value| !value.is_null()) {
        return None;
    }
    let axis = puzzle3d_axis_index(args.get("field").and_then(Value::as_str)?, "origin").filter(|axis| *axis < 3)?;
    let delta = args.get("delta").and_then(Value::as_f64).filter(|delta| delta.is_finite())?;
    let mut offset = [0.0; 3];
    offset[axis] = delta;
    Some(offset)
}

/// 🎯️ Mirrors the host's client-side zoom-to-selection framing math so a keybinding/engagement-token
/// driven focus (which bypasses that host interception) still produces a sensible camera. Camera-only:
/// writes `envelope.runtime.camera` (session-only per-window state), never the shared `scene_snapshot`. An
/// EMPTY `selected_object_ids` frames the whole document — same subject rule as
/// `Puzzle3dFocusSelectionWork::frames`, so the retained lane and this one can never disagree.
pub fn apply_puzzle3d_focus_selection(envelope: &mut Puzzle3dScene, selected_object_ids: &[String]) {
    let selected_origins: Vec<[f64; 3]> = envelope.scene_snapshot.objects.iter().filter(|object| selected_object_ids.is_empty() || selected_object_ids.contains(&object.id)).map(|object| object.origin).collect();
    if selected_origins.is_empty() {
        return;
    }
    let count = selected_origins.len() as f64;
    let mut center = [0.0, 0.0, 0.0];
    for origin in &selected_origins {
        center[0] += origin[0];
        center[1] += origin[1];
        center[2] += origin[2];
    }
    center = [center[0] / count, center[1] / count, center[2] / count];
    let max_distance = selected_origins
        .iter()
        .map(|origin| {
            let dx = origin[0] - center[0];
            let dy = origin[1] - center[1];
            let dz = origin[2] - center[2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        })
        .fold(1.0_f64, f64::max);
    let distance = max_distance * 3.0 + 2.0;
    envelope.runtime.camera.position = [center[0] + distance * 0.6, center[1] - distance * 0.6, center[2] + distance * 0.5];
    envelope.runtime.camera.target = center;
}
