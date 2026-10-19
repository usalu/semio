//! 🎲️ Board2d scene: the `Board2dScene` payload with its pack wire, scene doc, lanes, serde defaults and value conversions. See [`super`].

use super::*;

//#region 🔖️Board2dScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board2dScene {
    pub snapshot_json: String,
    pub camera_json: String,
    #[serde(default = "board2d_default_glyph_catalogs_json")]
    pub glyph_catalogs_json: String,
    #[serde(default = "board2d_default_selection_json")]
    pub selection_json: String,
    /// 🔗️ The ids the board paints highlighted, as a JSON id array — what the open time-travel draft references
    /// (`InteractionView::draft_references`); `[]` highlights nothing.
    #[serde(default = "board2d_default_highlighted_ids_json", skip_serializing_if = "board2d_highlights_nothing")]
    pub highlighted_ids_json: String,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_utility: Option<String>,
    #[serde(default = "board2d_default_selection_method")]
    pub selection_method: String,
    /// 👁️ Whether the board host strokes its world grid at all — independent of snapping.
    #[serde(default = "board2d_default_true")]
    pub grid_visible: bool,
    #[serde(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "board2d_default_grid_factor")]
    pub grid_factor: f64,
    /// 🎯️ Which granularity a pick may reach — the app's own selectable-kind filter.
    #[serde(default = "board2d_default_true")]
    pub selectable_nodes: bool,
    #[serde(default = "board2d_default_true")]
    pub selectable_edges: bool,
    #[serde(default = "board2d_default_true")]
    pub selectable_handles: bool,
    #[serde(default)]
    pub suggestion_offset: f64,
    #[serde(default = "board2d_default_brush_weights_json")]
    pub brush_weights_json: String,
    #[serde(default = "board2d_default_placement_compatibility_json")]
    pub placement_compatibility_json: String,
    #[serde(default = "board2d_default_lod_mode")]
    pub lod_mode: String,
    /// 🕹️ Which selection-gumball handles this board's select utility composes, as
    /// `{"move":bool,"rotate":bool}` — `None` leaves the engine's own default (both on). Scale is
    /// deliberately absent, same law as the puzzle-3d gumball.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_flags: Option<String>,
    /// 🖍️ World extent one area-brush CLICK paints, as `{"width":f64,"height":f64}` — `None` leaves the
    /// engine's own default. A click-drag states its own rectangle and ignores this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_brush_size: Option<String>,
    /// 🎯️ The framework interaction domain this board's picks and hovers belong to — the owning app's
    /// declared `InteractionDefinition.id`. `None` means the app declares none and the host publishes
    /// no `interactionHover` at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// 🏷️ The owning domain's brush placement id prefixes in the board's wire form,
    /// `{"brushNodePrefix":…,"brushEdgePrefix":…}`, so a host the domain cannot configure directly (the os wgpu surface)
    /// mints the same ids as the domain's own boards. `None` leaves the host's installed naming.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_naming: Option<String>,
    /// 💡️ The one-shot handle-suggestions popup this window has open, as
    /// `{"open":true,"x":…,"y":…,"windowId":…,"handleId":…,"hoveredIndex":…,"candidates":[…]}` —
    /// `None` is the closed state. It is a bounded page (`BOARD2D_SUGGESTION_MENU_CANDIDATE_PAGE`
    /// rows), because the surface doc it rides is fixed-capacity and a richly catalogued handle
    /// resolves arbitrarily many candidates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_menu_json: Option<String>,
    /// ⏯️ The base64url `ToolRunTraceDelta` paged to this board — the board twin of
    /// [`Canvas2dScene::tool_run_trace`], carried outside the doc as [`Board2dSceneLane::ToolRunTrace`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_run_trace: Option<String>,
    /// 🚚️ The spine's lane manifest — see [`World3dScene::lanes`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

scene_pack_wire!(Board2dScenePack, Board2dScene {
    snapshot_json: String,
    camera_json: String,
    glyph_catalogs_json: String,
    selection_json: String,
    interactive: bool,
    hovered_id: Option<String>,
    active_utility: Option<String>,
    selection_method: String,
    grid_visible: bool,
    grid_snap_enabled: bool,
    grid_factor: f64,
    selectable_nodes: bool,
    selectable_edges: bool,
    selectable_handles: bool,
    suggestion_offset: f64,
    brush_weights_json: String,
    placement_compatibility_json: String,
    lod_mode: String,
    transform_flags: Option<String>,
    area_brush_size: Option<String>,
    domain_id: Option<String>,
    placement_naming: Option<String>,
    suggestion_menu_json: Option<String>,
    tool_run_trace: Option<String>,
    highlighted_ids_json: String,
    #[serde(default)]
    lanes: Vec<SceneLaneRef>,
});

impl SceneDoc for Board2dScene {
    const SCHEMA: &'static str = "board-2d@1";

    fn encode_pack(&self) -> Result<Vec<u8>, PackError> {
        to_bytes(&Board2dScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        from_bytes::<Board2dScenePack>(bytes).map(Into::into)
    }

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in Board2dSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = Board2dSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload);
        true
    }
}

/// 🚚️ The board-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — the board twin of
/// [`Canvas2dSceneLane`], pinned against `🧪️tests/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` on both sides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Board2dSceneLane {
    /// 🧩️ The whole fixture graph — the one payload that outgrows the 32 KiB surface doc (Nakagin's
    /// 180-node puzzle 2d board is ~100 KiB of JSON), so it always rides as its own paged carrier.
    Snapshot,
    ToolRunTrace,
}

/// 🚚️ Reserved carrier-key namespace of the board-2d lanes.
pub const BOARD2D_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.board2d.";

/// 🚚️ Wire name of each [`Board2dSceneLane`], in `Board2dSceneLane::ALL` order.
pub const BOARD2D_SCENE_LANE_NAMES: [&str; 2] = ["snapshot", "toolRunTrace"];

/// 🚚️ [`Board2dScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const BOARD2D_SCENE_LANE_FIELDS: [&str; 2] = ["snapshotJson", "toolRunTrace"];

/// 🚚️ Reserved carrier key of each lane.
pub const BOARD2D_SCENE_LANE_BODY_KEYS: [&str; 2] = ["framework.scene.board2d.snapshot", "framework.scene.board2d.toolRunTrace"];

/// 🚚️ Whether each lane's [`Board2dScene`] field is an `Option<String>`.
pub const BOARD2D_SCENE_LANE_OPTIONAL: [bool; 2] = [false, true];

impl Board2dSceneLane {
    pub const ALL: [Self; 2] = [Self::Snapshot, Self::ToolRunTrace];

    /// 🏷️ See [`BOARD2D_SCENE_LANE_NAMES`].
    pub fn name(self) -> &'static str {
        BOARD2D_SCENE_LANE_NAMES[self as usize]
    }

    /// 🏷️ See [`BOARD2D_SCENE_LANE_FIELDS`].
    pub fn field(self) -> &'static str {
        BOARD2D_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`BOARD2D_SCENE_LANE_BODY_KEYS`].
    pub fn body_key(self) -> &'static str {
        BOARD2D_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// 🏷️ See [`BOARD2D_SCENE_LANE_OPTIONAL`].
    pub fn optional(self) -> bool {
        BOARD2D_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to its lane.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// 🔎️ Resolves a lane wire name back to its lane.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`; `None` when the optional lane is unset.
    pub fn take(self, scene: &mut Board2dScene) -> Option<String> {
        match self {
            Self::Snapshot => Some(std::mem::take(&mut scene.snapshot_json)),
            Self::ToolRunTrace => scene.tool_run_trace.take(),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`Board2dSceneLane::take`].
    pub fn put(self, scene: &mut Board2dScene, payload: String) {
        match self {
            Self::Snapshot => scene.snapshot_json = payload,
            Self::ToolRunTrace => scene.tool_run_trace = Some(payload),
        }
    }
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_glyph_catalogs_json() -> String {
    "{}".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_selection_json() -> String {
    "[]".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_highlighted_ids_json() -> String {
    "[]".into()
}

// 🚫️async: E1 pure predicate consumed by external-trait impls (serde skip) — see R9
pub fn board2d_highlights_nothing(highlighted_ids_json: &str) -> bool {
    highlighted_ids_json == "[]"
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_selection_method() -> String {
    "rectangle".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_grid_factor() -> f64 {
    1.0
}

/// 👁️ A board scene an older producer wrote carries neither the grid-visibility nor the
/// selectable-kind flags; both default ON so the absent field reads as the pre-filter behaviour.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_true() -> bool {
    true
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_brush_weights_json() -> String {
    "{}".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_placement_compatibility_json() -> String {
    "[]".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_lod_mode() -> String {
    "automatic".into()
}

impl Board2dScene {
    /** 🧩️ Builds a 2D board scene with optional extensions unset. */
    pub fn base(snapshot_json: String, camera_json: String, interactive: bool) -> Self {
        Self {
            snapshot_json,
            camera_json,
            glyph_catalogs_json: board2d_default_glyph_catalogs_json(),
            selection_json: board2d_default_selection_json(),
            highlighted_ids_json: board2d_default_highlighted_ids_json(),
            interactive,
            hovered_id: None,
            active_utility: None,
            selection_method: board2d_default_selection_method(),
            grid_visible: board2d_default_true(),
            grid_snap_enabled: false,
            grid_factor: board2d_default_grid_factor(),
            selectable_nodes: board2d_default_true(),
            selectable_edges: board2d_default_true(),
            selectable_handles: board2d_default_true(),
            suggestion_offset: 0.0,
            brush_weights_json: board2d_default_brush_weights_json(),
            placement_compatibility_json: board2d_default_placement_compatibility_json(),
            lod_mode: board2d_default_lod_mode(),
            domain_id: None,
            placement_naming: None,
            transform_flags: None,
            area_brush_size: None,
            suggestion_menu_json: None,
            tool_run_trace: None,
            lanes: Vec::new(),
        }
    }
}

impl ToValue for Board2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "snapshotJson", &self.snapshot_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "glyphCatalogsJson", &self.glyph_catalogs_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        if !board2d_highlights_nothing(&self.highlighted_ids_json) {
            value_push(&mut entries, "highlightedIdsJson", &self.highlighted_ids_json);
        }
        value_push(&mut entries, "interactive", &self.interactive);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push_option(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "selectionMethod", &self.selection_method);
        value_push(&mut entries, "gridVisible", &self.grid_visible);
        value_push(&mut entries, "gridSnapEnabled", &self.grid_snap_enabled);
        value_push(&mut entries, "gridFactor", &self.grid_factor);
        value_push(&mut entries, "selectableNodes", &self.selectable_nodes);
        value_push(&mut entries, "selectableEdges", &self.selectable_edges);
        value_push(&mut entries, "selectableHandles", &self.selectable_handles);
        value_push(&mut entries, "suggestionOffset", &self.suggestion_offset);
        value_push(&mut entries, "brushWeightsJson", &self.brush_weights_json);
        value_push(&mut entries, "placementCompatibilityJson", &self.placement_compatibility_json);
        value_push(&mut entries, "lodMode", &self.lod_mode);
        value_push_option(&mut entries, "transformFlags", &self.transform_flags);
        value_push_option(&mut entries, "areaBrushSize", &self.area_brush_size);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        value_push_option(&mut entries, "placementNaming", &self.placement_naming);
        value_push_option(&mut entries, "suggestionMenuJson", &self.suggestion_menu_json);
        value_push_option(&mut entries, "toolRunTrace", &self.tool_run_trace);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for Board2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            snapshot_json: value_decode(&entries, "snapshotJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            glyph_catalogs_json: value_decode_default(&entries, "glyphCatalogsJson", board2d_default_glyph_catalogs_json)?,
            selection_json: value_decode_default(&entries, "selectionJson", board2d_default_selection_json)?,
            highlighted_ids_json: value_decode_default(&entries, "highlightedIdsJson", board2d_default_highlighted_ids_json)?,
            interactive: value_decode_default(&entries, "interactive", Default::default)?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode_option(&entries, "activeUtility")?,
            selection_method: value_decode_default(&entries, "selectionMethod", board2d_default_selection_method)?,
            grid_visible: value_decode_default(&entries, "gridVisible", board2d_default_true)?,
            grid_snap_enabled: value_decode_default(&entries, "gridSnapEnabled", Default::default)?,
            grid_factor: value_decode_default(&entries, "gridFactor", board2d_default_grid_factor)?,
            selectable_nodes: value_decode_default(&entries, "selectableNodes", board2d_default_true)?,
            selectable_edges: value_decode_default(&entries, "selectableEdges", board2d_default_true)?,
            selectable_handles: value_decode_default(&entries, "selectableHandles", board2d_default_true)?,
            suggestion_offset: value_decode_default(&entries, "suggestionOffset", Default::default)?,
            brush_weights_json: value_decode_default(&entries, "brushWeightsJson", board2d_default_brush_weights_json)?,
            placement_compatibility_json: value_decode_default(&entries, "placementCompatibilityJson", board2d_default_placement_compatibility_json)?,
            lod_mode: value_decode_default(&entries, "lodMode", board2d_default_lod_mode)?,
            transform_flags: value_decode_option(&entries, "transformFlags")?,
            area_brush_size: value_decode_option(&entries, "areaBrushSize")?,
            domain_id: value_decode_option(&entries, "domainId")?,
            placement_naming: value_decode_option(&entries, "placementNaming")?,
            suggestion_menu_json: value_decode_option(&entries, "suggestionMenuJson")?,
            tool_run_trace: value_decode_option(&entries, "toolRunTrace")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}
//#endregion 🔖️Board2dScene
