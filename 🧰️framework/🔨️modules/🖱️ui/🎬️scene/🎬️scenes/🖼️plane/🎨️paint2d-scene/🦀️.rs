//! 🎨️ Paint2d scene: the `Paint2dScene` payload with its scene doc, lanes and value conversions. See [`super`].

use super::*;

//#region 🔖️Paint2dScene
/** 🖼️ Paint-2d scene: WASM `RasterSession` sync channels for the composite/navigator windows. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paint2dScene {
    pub document_sync_json: String,
    pub assets_json: String,
    pub camera_json: String,
    pub selection_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_selection_json:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    pub active_utility: String,
    pub brush_size: f64,
    pub brush_opacity: f64,
    pub brush_color: String,
    pub brush_hardness: f64,
    pub paint_target:String,
    pub mask_value:u32,
    /// 🪣️ The session colour tolerance (0..255) every host's bucket and wand floods with.
    pub fill_tolerance:u32,
    pub view_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_viewport_json: Option<String>,
    /// 🚚️ The spine's lane manifest — see [`Paint2dSceneLane`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

impl SceneDoc for Paint2dScene {
    const SCHEMA: &'static str = "paint-2d@1";

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in Paint2dSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = Paint2dSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload);
        true
    }
}

/// 🚚️ The paint-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — the raster twin
/// of [`Canvas2dSceneLane`]/[`Board2dSceneLane`], pinned against
/// `🧪️tests/🧫️fixtures/🚚️paint2d-scene-lanes/🔣️.json` on both sides.
///
/// Document, assets and completed pixel coverage can exceed the fixed scene header capacity.
/// Camera, layer selection and brush settings remain in the bounded header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paint2dSceneLane {
    DocumentSync,
    Assets,
    PixelSelection,
}

/// 🚚️ Reserved carrier-key namespace of the paint-2d lanes.
pub const PAINT2D_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.paint2d.";

/// 🚚️ Wire name of each [`Paint2dSceneLane`], in `Paint2dSceneLane::ALL` order.
pub const PAINT2D_SCENE_LANE_NAMES: [&str; 3] = ["documentSync", "assets", "pixelSelection"];

/// 🚚️ [`Paint2dScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const PAINT2D_SCENE_LANE_FIELDS: [&str; 3] = ["documentSyncJson", "assetsJson", "pixelSelectionJson"];

/// 🚚️ Reserved carrier key of each lane.
pub const PAINT2D_SCENE_LANE_BODY_KEYS: [&str; 3] = ["framework.scene.paint2d.documentSync", "framework.scene.paint2d.assets", "framework.scene.paint2d.pixelSelection"];

/// 🚚️ Whether each lane's [`Paint2dScene`] field is an `Option<String>`.
pub const PAINT2D_SCENE_LANE_OPTIONAL: [bool; 3] = [false, false, true];

impl Paint2dSceneLane {
    pub const ALL: [Self; 3] = [Self::DocumentSync, Self::Assets, Self::PixelSelection];

    /// 🏷️ See [`PAINT2D_SCENE_LANE_NAMES`].
    pub fn name(self) -> &'static str {
        PAINT2D_SCENE_LANE_NAMES[self as usize]
    }

    /// 🏷️ See [`PAINT2D_SCENE_LANE_FIELDS`].
    pub fn field(self) -> &'static str {
        PAINT2D_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`PAINT2D_SCENE_LANE_BODY_KEYS`].
    pub fn body_key(self) -> &'static str {
        PAINT2D_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// 🏷️ See [`PAINT2D_SCENE_LANE_OPTIONAL`].
    pub fn optional(self) -> bool {
        PAINT2D_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to its lane.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// 🔎️ Resolves a lane wire name back to its lane.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`.
    pub fn take(self, scene: &mut Paint2dScene) -> Option<String> {
        match self {
            Self::DocumentSync => Some(std::mem::take(&mut scene.document_sync_json)),
            Self::Assets => Some(std::mem::take(&mut scene.assets_json)),
            Self::PixelSelection=>scene.pixel_selection_json.take(),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`Paint2dSceneLane::take`].
    pub fn put(self, scene: &mut Paint2dScene, payload: String) {
        match self {
            Self::DocumentSync => scene.document_sync_json = payload,
            Self::Assets => scene.assets_json = payload,
            Self::PixelSelection=>scene.pixel_selection_json=Some(payload),
        }
    }
}

impl ToValue for Paint2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "documentSyncJson", &self.document_sync_json);
        value_push(&mut entries, "assetsJson", &self.assets_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries,"pixelSelectionJson",&self.pixel_selection_json);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "brushSize", &self.brush_size);
        value_push(&mut entries, "brushOpacity", &self.brush_opacity);
        value_push(&mut entries, "brushColor", &self.brush_color);
        value_push(&mut entries, "brushHardness", &self.brush_hardness);
        value_push(&mut entries,"paintTarget",&self.paint_target);
        value_push(&mut entries,"maskValue",&self.mask_value);
        value_push(&mut entries, "fillTolerance", &self.fill_tolerance);
        value_push(&mut entries, "viewMode", &self.view_mode);
        value_push_option(&mut entries, "compositeViewportJson", &self.composite_viewport_json);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for Paint2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            document_sync_json: value_decode(&entries, "documentSyncJson")?,
            assets_json: value_decode(&entries, "assetsJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            selection_json: value_decode(&entries, "selectionJson")?,
            pixel_selection_json:value_decode_option(&entries,"pixelSelectionJson")?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode(&entries, "activeUtility")?,
            brush_size: value_decode(&entries, "brushSize")?,
            brush_opacity: value_decode(&entries, "brushOpacity")?,
            brush_color: value_decode(&entries, "brushColor")?,
            brush_hardness: value_decode(&entries, "brushHardness")?,
            paint_target:value_decode(&entries,"paintTarget")?,
            mask_value:value_decode(&entries,"maskValue")?,
            fill_tolerance: value_decode(&entries, "fillTolerance")?,
            view_mode: value_decode(&entries, "viewMode")?,
            composite_viewport_json: value_decode_option(&entries, "compositeViewportJson")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}
//#endregion 🔖️Paint2dScene
