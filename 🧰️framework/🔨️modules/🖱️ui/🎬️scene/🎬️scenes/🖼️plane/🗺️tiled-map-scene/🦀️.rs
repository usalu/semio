//! 🗺️ Tiled map scene: the `TiledMapScene` payload with its scene doc, lanes, serde defaults and value conversions. See [`super`].

use super::*;

//#region 🔖️TiledMapScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiledMapScene {
    pub map_descriptor_json: String,
    pub camera_json: String,
    #[serde(default = "tiled_map_default_render_mode")]
    pub render_mode: String,
    #[serde(default = "tiled_map_default_vector_style")]
    pub vector_style: String,
    #[serde(default = "tiled_map_default_lod_mode")]
    pub lod_mode: String,
    #[serde(default = "tiled_map_default_tile_url_template")]
    pub tile_url_template: String,
    #[serde(default = "tiled_map_default_vector_tile_url_template")]
    pub vector_tile_url_template: String,
    #[serde(default = "tiled_map_default_layer_visibility_json")]
    pub layer_visibility_json: String,
    #[serde(default = "tiled_map_default_layer_stroke_scale_json")]
    pub layer_stroke_scale_json: String,
    #[serde(default = "tiled_map_default_selection_json")]
    pub selection_json: String,
    #[serde(default = "tiled_map_default_hover_json")]
    pub hover_json: String,
    #[serde(default = "tiled_map_default_selection_method")]
    pub selection_method: String,
    #[serde(default = "tiled_map_default_selection_mode")]
    pub selection_mode: String,
    /// 🚚️ The spine's lane manifest — see [`TiledMapSceneLane`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

impl SceneDoc for TiledMapScene {
    const SCHEMA: &'static str = "tiled-map@1";

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in TiledMapSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = TiledMapSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload);
        true
    }
}

/// 🎒️ The tiled-map payload field that rides OUTSIDE the fixed-capacity surface doc — the map twin of
/// [`Board2dSceneLane`]/[`Paint2dSceneLane`], pinned against
/// `🧪️tests/🧫️fixtures/🚚️tiledmap-scene-lanes/🔣️.json` on both sides.
///
/// `map_descriptor_json` is the whole map descriptor (`{ overlays, policy }`: the overlay features
/// per overlay layer id plus the app's map policy), so it scales with the DOCUMENT, not with the
/// frame: a map app's 300 overlay features can encode to about 60 000 bytes, nearly twice
/// `UI_FIXED_BYTES` (32 KiB). A surface doc cannot page, so `scene_surface.encode` would refuse the
/// whole surface and the window would never publish — `ui.fixed-capacity: fixed UI admission failed
/// at scene-surface.encode`.
/// Everything else (camera, render/vector/LOD mode, tile templates, layer visibility and stroke
/// scales, selection, hover, selection method/mode) is a bounded per-frame descriptor and stays in
/// the spine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiledMapSceneLane {
    MapDescriptor,
}

/// 🔑️ Reserved carrier-key namespace of the tiled-map lanes.
pub const TILEDMAP_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.tiledmap.";

/// 📛️ Wire name of each [`TiledMapSceneLane`], in `TiledMapSceneLane::ALL` order.
pub const TILEDMAP_SCENE_LANE_NAMES: [&str; 1] = ["mapDescriptor"];

/// 📋️ [`TiledMapScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const TILEDMAP_SCENE_LANE_FIELDS: [&str; 1] = ["mapDescriptorJson"];

/// 🗝️ Reserved carrier key of each lane.
pub const TILEDMAP_SCENE_LANE_BODY_KEYS: [&str; 1] = ["framework.scene.tiledmap.mapDescriptor"];

/// ❓️ Whether each lane's [`TiledMapScene`] field is an `Option<String>`.
pub const TILEDMAP_SCENE_LANE_OPTIONAL: [bool; 1] = [false];

impl TiledMapSceneLane {
    pub const ALL: [Self; 1] = [Self::MapDescriptor];

    /// 🏷️ See [`TILEDMAP_SCENE_LANE_NAMES`].
    pub fn name(self) -> &'static str {
        TILEDMAP_SCENE_LANE_NAMES[self as usize]
    }

    /// 🔖️ See [`TILEDMAP_SCENE_LANE_FIELDS`].
    pub fn field(self) -> &'static str {
        TILEDMAP_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`TILEDMAP_SCENE_LANE_BODY_KEYS`].
    pub fn body_key(self) -> &'static str {
        TILEDMAP_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// ❔️ See [`TILEDMAP_SCENE_LANE_OPTIONAL`].
    pub fn optional(self) -> bool {
        TILEDMAP_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to its lane.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// ↩️ Resolves a lane wire name back to its lane.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`.
    pub fn take(self, scene: &mut TiledMapScene) -> Option<String> {
        match self {
            Self::MapDescriptor => Some(std::mem::take(&mut scene.map_descriptor_json)),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`TiledMapSceneLane::take`].
    pub fn put(self, scene: &mut TiledMapScene, payload: String) {
        match self {
            Self::MapDescriptor => scene.map_descriptor_json = payload,
        }
    }
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_render_mode() -> String {
    "combined".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_vector_style() -> String {
    "colored".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_lod_mode() -> String {
    "automatic".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_tile_url_template() -> String {
    "/osm/{z}/{x}/{y}.png".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_vector_tile_url_template() -> String {
    "/vt/{z}/{x}/{y}.pbf".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_layer_visibility_json() -> String {
    "{}".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_layer_stroke_scale_json() -> String {
    "{}".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_json() -> String {
    "{}".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_hover_json() -> String {
    "null".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_method() -> String {
    "rectangle".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_mode() -> String {
    "default".into()
}

impl TiledMapScene {
    /** 🧱️ Builds a tiled map scene with optional extensions unset. */
    pub fn base(map_descriptor_json: String, camera_json: String) -> Self {
        Self {
            map_descriptor_json,
            camera_json,
            render_mode: tiled_map_default_render_mode(),
            vector_style: tiled_map_default_vector_style(),
            lod_mode: tiled_map_default_lod_mode(),
            tile_url_template: tiled_map_default_tile_url_template(),
            vector_tile_url_template: tiled_map_default_vector_tile_url_template(),
            layer_visibility_json: tiled_map_default_layer_visibility_json(),
            layer_stroke_scale_json: tiled_map_default_layer_stroke_scale_json(),
            selection_json: tiled_map_default_selection_json(),
            hover_json: tiled_map_default_hover_json(),
            selection_method: tiled_map_default_selection_method(),
            selection_mode: tiled_map_default_selection_mode(),
            lanes: Vec::new(),
        }
    }
}

impl ToValue for TiledMapScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "mapDescriptorJson", &self.map_descriptor_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "renderMode", &self.render_mode);
        value_push(&mut entries, "vectorStyle", &self.vector_style);
        value_push(&mut entries, "lodMode", &self.lod_mode);
        value_push(&mut entries, "tileUrlTemplate", &self.tile_url_template);
        value_push(&mut entries, "vectorTileUrlTemplate", &self.vector_tile_url_template);
        value_push(&mut entries, "layerVisibilityJson", &self.layer_visibility_json);
        value_push(&mut entries, "layerStrokeScaleJson", &self.layer_stroke_scale_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push(&mut entries, "hoverJson", &self.hover_json);
        value_push(&mut entries, "selectionMethod", &self.selection_method);
        value_push(&mut entries, "selectionMode", &self.selection_mode);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for TiledMapScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            map_descriptor_json: value_decode(&entries, "mapDescriptorJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            render_mode: value_decode_default(&entries, "renderMode", tiled_map_default_render_mode)?,
            vector_style: value_decode_default(&entries, "vectorStyle", tiled_map_default_vector_style)?,
            lod_mode: value_decode_default(&entries, "lodMode", tiled_map_default_lod_mode)?,
            tile_url_template: value_decode_default(&entries, "tileUrlTemplate", tiled_map_default_tile_url_template)?,
            vector_tile_url_template: value_decode_default(&entries, "vectorTileUrlTemplate", tiled_map_default_vector_tile_url_template)?,
            layer_visibility_json: value_decode_default(&entries, "layerVisibilityJson", tiled_map_default_layer_visibility_json)?,
            layer_stroke_scale_json: value_decode_default(&entries, "layerStrokeScaleJson", tiled_map_default_layer_stroke_scale_json)?,
            selection_json: value_decode_default(&entries, "selectionJson", tiled_map_default_selection_json)?,
            hover_json: value_decode_default(&entries, "hoverJson", tiled_map_default_hover_json)?,
            selection_method: value_decode_default(&entries, "selectionMethod", tiled_map_default_selection_method)?,
            selection_mode: value_decode_default(&entries, "selectionMode", tiled_map_default_selection_mode)?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}
//#endregion 🔖️TiledMapScene
