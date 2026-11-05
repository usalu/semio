//! 🎨️ Paint2d scene: the `Paint2dScene` payload with its scene doc, lanes and value conversions. See [`super`].

use super::*;

/// ⎋️ Exact admitted-operation authority scoped to the actor and mounted window.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,protocol::value::ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(crate="protocol::value",rename_all="camelCase")]
pub struct Paint2dSelectionCancellation {
    #[serde(deserialize_with="read_cancellation_scope")]
    pub actor:String,
    pub app_instance_id:u32,
    #[serde(deserialize_with="read_cancellation_scope")]
    pub window_id:String,
    #[serde(deserialize_with="read_cancellation_identity")]
    pub operation_id:String,
    #[serde(deserialize_with="read_cancellation_identity")]
    pub generation:String,
}

fn cancellation_identity(value:&str)->bool{value.len()==16&&value.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte))}
fn read_cancellation_scope<'de,D:serde::Deserializer<'de>>(input:D)->Result<String,D::Error>{let value=String::deserialize(input)?;if value.is_empty(){Err(serde::de::Error::custom("empty cancellation scope"))}else{Ok(value)}}
fn read_cancellation_identity<'de,D:serde::Deserializer<'de>>(input:D)->Result<String,D::Error>{let value=String::deserialize(input)?;if cancellation_identity(&value){Ok(value)}else{Err(serde::de::Error::custom("cancellation identity must be sixteen lowercase hexadecimal digits"))}}
impl FromValue for Paint2dSelectionCancellation{
    fn from_value(value:DslValue)->Result<Self,ValueError>{
        let entries=value.into_object()?;editing_fields(&entries,&["actor","appInstanceId","windowId","operationId","generation"])?;
        let authority=Self{actor:value_decode(&entries,"actor")?,app_instance_id:value_decode(&entries,"appInstanceId")?,window_id:value_decode(&entries,"windowId")?,operation_id:value_decode(&entries,"operationId")?,generation:value_decode(&entries,"generation")?};
        if authority.actor.is_empty()||authority.window_id.is_empty()||!cancellation_identity(&authority.operation_id)||!cancellation_identity(&authority.generation){return Err(editing_error("invalid scoped cancellation authority"));}Ok(authority)
    }
}

/// 📐️ A local crop/resize request carried without renderer-specific staging.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct Paint2dImageGrid {
    #[serde(deserialize_with="read_grid_coordinate")]
    pub x:u32,
    #[serde(deserialize_with="read_grid_coordinate")]
    pub y:u32,
    #[serde(deserialize_with="read_grid_extent")]
    pub width:u32,
    #[serde(deserialize_with="read_grid_extent")]
    pub height:u32,
    pub bilinear:bool,
}

/// 🎚️ Published local options shared by the native and React painting controls.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct Paint2dEditingOptions {
    #[serde(deserialize_with="read_filter_kind")]
    pub filter_kind:String,
    pub filter_amount:f64,
    pub image_grid:Paint2dImageGrid,
}

fn filter_supported(value:&str)->bool{matches!(value,"invert"|"grayscale"|"brightness"|"contrast"|"saturation"|"gamma"|"threshold"|"posterize"|"blur"|"sharpen"|"clear"|"flipHorizontal"|"flipVertical")}
fn read_filter_kind<'de,D:serde::Deserializer<'de>>(input:D)->Result<String,D::Error>{let value=String::deserialize(input)?;if filter_supported(&value){Ok(value)}else{Err(serde::de::Error::custom("unsupported paint filter"))}}
fn read_grid_number<'de,D:serde::Deserializer<'de>>(input:D,min:u32,max:u32)->Result<u32,D::Error>{let value=u32::deserialize(input)?;if(min..=max).contains(&value){Ok(value)}else{Err(serde::de::Error::custom("image grid value exceeds its bounds"))}}
fn read_grid_coordinate<'de,D:serde::Deserializer<'de>>(input:D)->Result<u32,D::Error>{read_grid_number(input,0,16384)}
fn read_grid_extent<'de,D:serde::Deserializer<'de>>(input:D)->Result<u32,D::Error>{read_grid_number(input,1,16384)}
fn editing_error(message:impl Into<String>)->ValueError{ValueError::new(protocol::value::ValueRefusalKind::InvalidValue,message)}
fn editing_fields(entries:&[(String,DslValue)],fields:&[&str])->Result<(),ValueError>{
    if let Some((field,_))=entries.iter().find(|(field,_)|!fields.contains(&field.as_str())){return Err(editing_error(format!("unknown field `{field}`")));}
    for field in fields.iter().copied(){if entries.iter().filter(|(candidate,_)|candidate==field).count()>1{return Err(editing_error(format!("duplicate field `{field}`")));}}
    Ok(())
}

impl ToValue for Paint2dImageGrid {
    fn to_value(&self)->DslValue{
        let mut entries=Vec::new();value_push(&mut entries,"x",&self.x);value_push(&mut entries,"y",&self.y);value_push(&mut entries,"width",&self.width);value_push(&mut entries,"height",&self.height);value_push(&mut entries,"bilinear",&self.bilinear);DslValue::Object(entries)
    }
}
impl FromValue for Paint2dImageGrid {
    fn from_value(value:DslValue)->Result<Self,ValueError>{
        let entries=value.into_object()?;editing_fields(&entries,&["x","y","width","height","bilinear"])?;
        let grid=Self{x:value_decode(&entries,"x")?,y:value_decode(&entries,"y")?,width:value_decode(&entries,"width")?,height:value_decode(&entries,"height")?,bilinear:value_decode(&entries,"bilinear")?};
        if grid.x>16384||grid.y>16384||!(1..=16384).contains(&grid.width)||!(1..=16384).contains(&grid.height){return Err(editing_error("image grid value exceeds its bounds"));}
        Ok(grid)
    }
}
impl ToValue for Paint2dEditingOptions {
    fn to_value(&self)->DslValue{
        let mut entries=Vec::new();value_push(&mut entries,"filterKind",&self.filter_kind);value_push(&mut entries,"filterAmount",&self.filter_amount);value_push(&mut entries,"imageGrid",&self.image_grid);DslValue::Object(entries)
    }
}
impl FromValue for Paint2dEditingOptions {
    fn from_value(value:DslValue)->Result<Self,ValueError>{
        let entries=value.into_object()?;editing_fields(&entries,&["filterKind","filterAmount","imageGrid"])?;
        let options=Self{filter_kind:value_decode(&entries,"filterKind")?,filter_amount:value_decode(&entries,"filterAmount")?,image_grid:value_decode(&entries,"imageGrid")?};
        if !filter_supported(&options.filter_kind)||!options.filter_amount.is_finite(){return Err(editing_error("invalid paint filter options"));}
        Ok(options)
    }
}

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
    pub editing_options:Paint2dEditingOptions,
    #[serde(skip_serializing_if="Option::is_none")]
    pub selection_cancellation:Option<Paint2dSelectionCancellation>,
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

/// 🖌️ The paint-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — the raster twin
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

/// 🔑️ Reserved carrier-key namespace of the paint-2d lanes.
pub const PAINT2D_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.paint2d.";

/// 🔤️ Wire name of each [`Paint2dSceneLane`], in `Paint2dSceneLane::ALL` order.
pub const PAINT2D_SCENE_LANE_NAMES: [&str; 3] = ["documentSync", "assets", "pixelSelection"];

/// 📋️ [`Paint2dScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const PAINT2D_SCENE_LANE_FIELDS: [&str; 3] = ["documentSyncJson", "assetsJson", "pixelSelectionJson"];

/// 🗝️ Reserved carrier key of each lane.
pub const PAINT2D_SCENE_LANE_BODY_KEYS: [&str; 3] = ["framework.scene.paint2d.documentSync", "framework.scene.paint2d.assets", "framework.scene.paint2d.pixelSelection"];

/// ❓️ Whether each lane's [`Paint2dScene`] field is an `Option<String>`.
pub const PAINT2D_SCENE_LANE_OPTIONAL: [bool; 3] = [false, false, true];

impl Paint2dSceneLane {
    pub const ALL: [Self; 3] = [Self::DocumentSync, Self::Assets, Self::PixelSelection];

    /// 🏷️ See [`PAINT2D_SCENE_LANE_NAMES`].
    pub fn name(self) -> &'static str {
        PAINT2D_SCENE_LANE_NAMES[self as usize]
    }

    /// 🔖️ See [`PAINT2D_SCENE_LANE_FIELDS`].
    pub fn field(self) -> &'static str {
        PAINT2D_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`PAINT2D_SCENE_LANE_BODY_KEYS`].
    pub fn body_key(self) -> &'static str {
        PAINT2D_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// ❔️ See [`PAINT2D_SCENE_LANE_OPTIONAL`].
    pub fn optional(self) -> bool {
        PAINT2D_SCENE_LANE_OPTIONAL[self as usize]
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
        value_push(&mut entries,"editingOptions",&self.editing_options);
        value_push_option(&mut entries,"selectionCancellation",&self.selection_cancellation);
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
            editing_options:value_decode(&entries,"editingOptions")?,
            selection_cancellation:value_decode_option(&entries,"selectionCancellation")?,
            view_mode: value_decode(&entries, "viewMode")?,
            composite_viewport_json: value_decode_option(&entries, "compositeViewportJson")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}
//#endregion 🔖️Paint2dScene
