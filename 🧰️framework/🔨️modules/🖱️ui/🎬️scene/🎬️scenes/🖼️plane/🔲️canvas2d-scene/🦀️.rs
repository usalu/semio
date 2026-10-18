//! 🔲️ Canvas2d scene: the `Canvas2dScene` payload with its pack wire, scene doc, value conversions and lanes, and the canvas framing value conversions. See [`super`].

use super::*;

//#region 🔖️Canvas2dScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas2dScene {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub layers_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framing: Option<Canvas2dFraming>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Canvas2dSnapshotLease>,
    /// ⏯️ The base64url `ToolRunTraceDelta` paged to this window — the 2d twin of
    /// [`World3dScene::tool_run_trace`], carried outside the doc as [`Canvas2dSceneLane::ToolRunTrace`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_run_trace: Option<String>,
    /// 🚚️ The spine's lane manifest — see [`World3dScene::lanes`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

impl Canvas2dScene {
    /** 🖼️ Builds a canvas-2d scene with every optional extension unset. */
    pub fn base(camera_x: f64, camera_y: f64, zoom: f64, layers_json: String) -> Self {
        Self { camera_x, camera_y, zoom, layers_json, framing: None, snapshot: None, tool_run_trace: None, lanes: Vec::new() }
    }
}

scene_pack_wire!(Canvas2dScenePack, Canvas2dScene {
    camera_x: f64,
    camera_y: f64,
    zoom: f64,
    layers_json: String,
    framing: Option<Canvas2dFraming>,
    snapshot: Option<Canvas2dSnapshotLease>,
    tool_run_trace: Option<String>,
    #[serde(default)]
    lanes: Vec<SceneLaneRef>,
});

impl SceneDoc for Canvas2dScene {
    const SCHEMA: &'static str = "canvas-2d@1";

    fn encode_pack(&self) -> Result<Vec<u8>, PackError> {
        to_bytes(&Canvas2dScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        from_bytes::<Canvas2dScenePack>(bytes).map(Into::into)
    }

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in Canvas2dSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = Canvas2dSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload);
        true
    }
}

impl ToValue for Canvas2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "cameraX", &self.camera_x);
        value_push(&mut entries, "cameraY", &self.camera_y);
        value_push(&mut entries, "zoom", &self.zoom);
        value_push(&mut entries, "layersJson", &self.layers_json);
        value_push_option(&mut entries, "framing", &self.framing);
        value_push_option(&mut entries, "snapshot", &self.snapshot);
        value_push_option(&mut entries, "toolRunTrace", &self.tool_run_trace);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for Canvas2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            camera_x: value_decode(&entries, "cameraX")?,
            camera_y: value_decode(&entries, "cameraY")?,
            zoom: value_decode(&entries, "zoom")?,
            layers_json: value_decode(&entries, "layersJson")?,
            framing: value_decode_option(&entries, "framing")?,
            snapshot: value_decode_option(&entries, "snapshot")?,
            tool_run_trace: value_decode_option(&entries, "toolRunTrace")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}

/// 🚚️ The canvas-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — the 2d twin of
/// [`World3dSceneLane`], pinned against `🧪️tests/🧫️fixtures/🚚️canvas2d-scene-lanes/🔣️.json` on both sides.
/// `layers_json` stays in the spine: it is a bounded per-frame descriptor, while a trace delta scales
/// with the run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Canvas2dSceneLane {
    ToolRunTrace,
}

/// 🚚️ Reserved carrier-key namespace of the canvas-2d lanes.
pub const CANVAS2D_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.canvas2d.";

/// 🚚️ Wire name of each [`Canvas2dSceneLane`], in `Canvas2dSceneLane::ALL` order.
pub const CANVAS2D_SCENE_LANE_NAMES: [&str; 1] = ["toolRunTrace"];

/// 🚚️ [`Canvas2dScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const CANVAS2D_SCENE_LANE_FIELDS: [&str; 1] = ["toolRunTrace"];

/// 🚚️ Reserved carrier key of each lane.
pub const CANVAS2D_SCENE_LANE_BODY_KEYS: [&str; 1] = ["framework.scene.canvas2d.toolRunTrace"];

/// 🚚️ Whether each lane's [`Canvas2dScene`] field is an `Option<String>`.
pub const CANVAS2D_SCENE_LANE_OPTIONAL: [bool; 1] = [true];

impl Canvas2dSceneLane {
    pub const ALL: [Self; 1] = [Self::ToolRunTrace];

    /// 🏷️ See [`CANVAS2D_SCENE_LANE_NAMES`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn name(self) -> &'static str {
        CANVAS2D_SCENE_LANE_NAMES[self as usize]
    }

    /// 🏷️ See [`CANVAS2D_SCENE_LANE_FIELDS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn field(self) -> &'static str {
        CANVAS2D_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`CANVAS2D_SCENE_LANE_BODY_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn body_key(self) -> &'static str {
        CANVAS2D_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// 🏷️ See [`CANVAS2D_SCENE_LANE_OPTIONAL`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn optional(self) -> bool {
        CANVAS2D_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// 🔎️ Resolves a lane wire name back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`; `None` when the optional lane is unset.
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn take(self, scene: &mut Canvas2dScene) -> Option<String> {
        match self {
            Self::ToolRunTrace => scene.tool_run_trace.take(),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`Canvas2dSceneLane::take`].
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn put(self, scene: &mut Canvas2dScene, payload: String) {
        match self {
            Self::ToolRunTrace => scene.tool_run_trace = Some(payload),
        }
    }
}
//#endregion 🔖️Canvas2dScene

impl ToValue for Canvas2dFraming {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("revision".into(),self.revision.to_value()),("bounds".into(),self.bounds.to_vec().to_value()),("padding".into(),self.padding.to_value())])
    }
}

impl FromValue for Canvas2dFraming {
    fn from_value(value: DslValue) -> Result<Self,ValueError> {
        let entries = value.into_object()?;
        let bounds: Vec<f64> = value_decode(&entries,"bounds")?;
        Ok(Self { revision: value_decode(&entries,"revision")?,bounds: bounds.try_into().map_err(|_| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "canvas framing requires four bounds"))?,padding: value_decode(&entries,"padding")? })
    }
}
