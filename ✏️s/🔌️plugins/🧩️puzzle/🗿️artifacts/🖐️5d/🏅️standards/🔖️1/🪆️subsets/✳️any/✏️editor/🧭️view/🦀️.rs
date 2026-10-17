//! 🧭️ View of the puzzle 5d editor: the transient scene bundle, the interaction snapshot, the scene projection and gumball helpers and the engine grip kind. See [`super`].

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::entities::{PUZZLE5D_FILL_TOOL_ID, PUZZLE5D_TRANSFORM_UTILITY_ID};
use semio_framework_plugin::app::InteractionView;
use semio_framework_value::FromValue;
use semio_framework_pack_json::Value;
use crate::editor::puzzle5d::entities::{PUZZLE5D_DEFAULT_UTILITY, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRANULARITY_TARGET_VOLUME, PUZZLE5D_HOVER_CHANNEL, PUZZLE5D_INTERACTION_DOMAIN, Puzzle5dDocument, Puzzle5dGrip, Puzzle5dPart, empty_document, puzzle5d_grip_full_id};

//#region 🔖️Scene
/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle5dDocument`
/// json) with the app's view state. Never persisted — the `VcsArtifactApp` store owns the document
/// and the wrapping store owns the VCS-tracked `Puzzle5dConfig` — but rebuilt per call so the
/// board/world/engagement helpers keep their `&scene` signatures.
#[derive(Clone)]
pub struct Puzzle5dScene {
    pub document: Puzzle5dDocument,
    pub runtime: Puzzle5dRuntime,
    /// 🧰️ The active utility for this window — transient, never persisted.
    pub active_utility: String,
    /// 🕹️ The framework-owned `vortex` selection AND hover this render/mutation reads. ONE domain for
    /// BOTH panes: a pick in the board pane and a pick in the world pane write the same domain, so
    /// each pane paints what the other selected or hovered without any cross-pane bus.
    pub interaction: Puzzle5dInteractionSnapshot,
}

/// 🕹️ One render pass' read of the framework-owned `vortex` interaction domain (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): the selected ids at the granularity they were
/// picked at plus the pointer-channel hover — the 5d twin of `Puzzle3dInteractionSnapshot` /
/// `Puzzle2dInteractionSnapshot`, in this artifact's part/grip/fastener vocabulary.
///
/// 🐁️ Hover ids arrive WITHOUT a granularity (`protocol::DomainHover` has none), so classifying one
/// needs the document: `hovered_part_id`/`hovered_grip_full_id` resolve against the live document
/// rather than guessing from the id's spelling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle5dInteractionSnapshot {
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
    pub referenced: Vec<String>,
}

impl Puzzle5dInteractionSnapshot {
    /// 🕹️ Reads the live `vortex` domain: its selection, its `"pointer"`-channel hover and the ids the open time-travel
    /// draft references (what both panes paint highlighted while a history edit is open).
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let selection = interaction.selection(PUZZLE5D_INTERACTION_DOMAIN);
        let hover = interaction.hover(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_HOVER_CHANNEL);
        let leftover_ids = interaction.leftover_selected_ids();
        let selected = if selection.ids.is_empty() { leftover_ids } else { selection.ids.clone() };
        let granularity = if !selection.granularity.is_empty() {
            selection.granularity.clone()
        } else if selected.is_empty() {
            String::new()
        } else {
            PUZZLE5D_GRANULARITY_PART.to_string()
        };
        Self { granularity, selected, hovered: hover.ids.clone(), referenced: interaction.draft_references(PUZZLE5D_INTERACTION_DOMAIN).to_vec() }
    }

    /// 🕹️ The retained-reducer twin — the same read from the raw `InteractionState`/hover map a
    /// retained tool job is handed, so a command reusing a render helper sees the selection and hover
    /// both panes painted.
    pub fn from_state(state: &protocol::InteractionState, hover: &semio_framework_plugin::app::InteractionHoverState) -> Self {
        let selection = state.selection.get(PUZZLE5D_INTERACTION_DOMAIN);
        let hovered = hover.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|hover| hover.channel == PUZZLE5D_HOVER_CHANNEL).map(|hover| hover.ids.clone()).unwrap_or_default();
        let leftover_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        let selected = selection.filter(|selection| !selection.ids.is_empty()).map(|selection| selection.ids.clone()).unwrap_or(leftover_ids);
        let granularity = selection.map(|selection| selection.granularity.clone()).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| if selected.is_empty() { String::new() } else { PUZZLE5D_GRANULARITY_PART.to_string() });
        Self { granularity, selected, hovered, referenced: Vec::new() }
    }

    /// 🔗️ The ids the board paints highlighted while a time-travel draft references them, as the scene's JSON id array.
    pub fn referenced_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&self.referenced)
    }

    pub fn selected_ids(&self, granularity: &str) -> &[String] {
        if self.granularity == granularity {
            &self.selected
        } else {
            &[]
        }
    }
    pub fn selected_part_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE5D_GRANULARITY_PART)
    }
    pub fn selected_grip_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE5D_GRANULARITY_GRIP)
    }
    pub fn selected_fastener_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE5D_GRANULARITY_FASTENER)
    }
    pub fn selected_target_volume_ids(&self) -> &[String] {
        self.selected_ids(PUZZLE5D_GRANULARITY_TARGET_VOLUME)
    }

    /// 🎨️ Every marked id regardless of granularity — what the board pane paints as its selection and
    /// the world pane matches instances against.
    pub fn selection_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&self.selected)
    }

    /// 🐁️ The hovered part id, resolved against the document's own part ids.
    pub fn hovered_part_id(&self, document: &Puzzle5dDocument) -> Option<&str> {
        self.hovered.iter().find(|id| document.parts.iter().any(|part| &part.id == *id)).map(String::as_str)
    }

    /// 🐁️ The hovered grip full id (`{partId}:{gripId}`), resolved against the document.
    pub fn hovered_grip_full_id(&self, document: &Puzzle5dDocument) -> Option<&str> {
        self.hovered.iter().find(|id| document.parts.iter().any(|part| part.grips.iter().any(|grip| &puzzle5d_grip_full_id(&part.id, &grip.id) == *id))).map(String::as_str)
    }

    /// 🐁️ The first hovered id of any granularity — what `Board2dScene::hovered_id` paints.
    pub fn hovered_id(&self) -> Option<&str> {
        self.hovered.first().map(String::as_str)
    }

    /// 👁️ Whether `part` is itself selected/hovered or owns a selected/hovered grip — the predicate
    /// `PUZZLE5D_GRIP_SHOW_SELECTED` gates grip-marker emission on.
    pub fn touches_part(&self, part: &Puzzle5dPart) -> bool {
        let mut marks = self.selected.iter().chain(self.hovered.iter());
        marks.any(|id| id == &part.id || part.grips.iter().any(|grip| &puzzle5d_grip_full_id(&part.id, &grip.id) == id))
    }

    /// 🕹️ Whether anything at all is marked.
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty() && self.hovered.is_empty()
    }
}

/// 🧾️ Materializes the transient scene from the persisted projection (bare document json) and the
/// app's current view state; an unparseable projection degrades to an empty document. The interaction
/// read stays empty — call [`scene_from_projection_with_interaction`] from any path that holds one.
pub fn scene_from_projection(projection: &Value, runtime: Puzzle5dRuntime, active_utility: &str) -> Puzzle5dScene {
    scene_from_projection_with_interaction(projection, runtime, active_utility, Puzzle5dInteractionSnapshot::default())
}

/// 🧾️ The interaction-aware twin: the render paths that are handed an `InteractionView` build the
/// scene through this one so both panes project the SAME live selection and hover.
pub fn scene_from_projection_with_interaction(projection: &Value, runtime: Puzzle5dRuntime, active_utility: &str, interaction: Puzzle5dInteractionSnapshot) -> Puzzle5dScene {
    let document = <Puzzle5dDocument as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(projection)).unwrap_or_else(|_| empty_document());
    Puzzle5dScene { document, runtime, active_utility: active_utility.to_string(), interaction }
}

/// 🛠️ Whether the host has the mode-level fill TOOL armed (`ViewModel::active_tool_id`), which outranks every
/// window utility: fill keeps its viewport interaction in both panes while it is the armed tool.
pub fn puzzle5d_fill_tool_active(view_state: Option<&semio_framework_plugin::ViewModel>) -> bool {
    view_state.and_then(|state| state.active_tool_id.as_deref()) == Some(PUZZLE5D_FILL_TOOL_ID)
}

/// 🧰️ Resolves the host-owned armed gesture for a concrete window: the mode-level tool first (a tool and a
/// utility are mutually exclusive in the shell, and fill wins), then that window's utility, then the surface's.
pub fn puzzle5d_scene_active_utility(view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> String {
    if puzzle5d_fill_tool_active(view_state) {
        return PUZZLE5D_FILL_TOOL_ID.to_string();
    }
    window_id
        .and_then(|window_id| view_state.and_then(|state| state.active_utility_by_window_id.get(window_id)))
        .or_else(|| view_state.and_then(|state| state.active_utility_id.as_ref()))
        .filter(|utility| !utility.is_empty())
        .cloned()
        .unwrap_or_else(|| PUZZLE5D_DEFAULT_UTILITY.to_string())
}

/// 🧭️ The select/brush/fill/volume-brush interaction mode the world engine reads, derived from the flat
/// active utility (the transform gumball and `worldRelocate` both present as `select`) — puzzle 3d's
/// `main::scene_mode` twin.
pub fn puzzle5d_scene_mode(active_utility: &str) -> &str {
    match active_utility {
        "brush" => "brush",
        "fill" => "fill",
        "volumeBrush" => "volumeBrush",
        _ => "select",
    }
}

/// 🎚️ The gumball handle the world engine draws when the transform utility is active.
pub fn puzzle5d_transform_handle(active_utility: &str) -> Option<&'static str> {
    (active_utility == PUZZLE5D_TRANSFORM_UTILITY_ID).then_some("transform")
}

/// 🧭️ Whether the active utility is the transform gumball.
pub fn puzzle5d_transform_utility_active(active_utility: &str) -> bool {
    puzzle5d_transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render: the transform utility is active, at least one handle
/// flag is on (`setTransformGumballFlag` — an all-off gumball would draw nothing to grab), and the
/// live `vortex` selection holds at least one part or target volume to move. Selection comes from the
/// framework-owned domain via [`Puzzle5dInteractionSnapshot`], never from stored app state.
pub fn puzzle5d_gumball_active(runtime: &Puzzle5dRuntime, active_utility: &str, interaction: &Puzzle5dInteractionSnapshot) -> bool {
    puzzle5d_transform_utility_active(active_utility) && (runtime.transform_move || runtime.transform_rotate) && !(interaction.selected_part_ids().is_empty() && interaction.selected_target_volume_ids().is_empty())
}

pub fn gumball_target_world(envelope: &Puzzle5dScene, selected_part_ids: &[String]) -> Option<[f64; 3]> {
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| selected_part_ids.contains(&part.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut sum = [0.0, 0.0, 0.0];
    for part in &selected {
        sum[0] += part.part_3d.origin[0];
        sum[1] += part.part_3d.origin[1];
        sum[2] += part.part_3d.origin[2];
    }
    let count = selected.len() as f64;
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}
//#endregion 🔖️Scene

//#region 🔖️Engine
/// 🔘️ The kind a grip gates attraction compatibility with: its own kind, else its flat aspect's kind.
pub fn engine_grip_kind(grip: &Puzzle5dGrip) -> String {
    if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() }
}
//#endregion 🔖️Engine
