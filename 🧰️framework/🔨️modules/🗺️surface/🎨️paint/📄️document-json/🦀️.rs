//! 📄️ Document records of surface paint: the layer, transform, mask and document wire records, the layer node and mask state and the document parser. See [`super`].

use super::*;

// #region 🔖️Document
#[derive(Clone, Debug, Deserialize, semio_framework_value::FromValue)]
#[serde(tag = "kind")]
#[value(tag = "kind")]
pub(super) enum LayerNodeJson {
    #[serde(rename = "pixel", rename_all = "camelCase")]
    #[value(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        #[serde(default = "default_true")]
        #[value(default = "default_true")]
        visible: bool,
        #[serde(default)]
        #[value(default)]
        locked: bool,
        #[serde(default = "default_opacity")]
        #[value(default = "default_opacity")]
        opacity: f32,
        #[serde(default)]
        #[value(default)]
        blend_mode: String,
        transform: TransformJson,
        mask: Option<MaskJson>,
        width: Option<u32>,
        height: Option<u32>,
        #[serde(default)]
        #[value(default)]
        image_key: Option<String>,
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    #[value(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        #[serde(default = "default_true")]
        #[value(default = "default_true")]
        visible: bool,
        #[serde(default)]
        #[value(default)]
        locked: bool,
        #[serde(default = "default_opacity")]
        #[value(default = "default_opacity")]
        opacity: f32,
        #[serde(default)]
        #[value(default)]
        blend_mode: String,
        transform: TransformJson,
        mask: Option<MaskJson>,
        children: Vec<LayerNodeJson>,
    },
    #[serde(rename = "adjustment", rename_all = "camelCase")]
    #[value(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {
        id: String,
        transform: TransformJson,
        #[serde(default = "default_true")]
        #[value(default = "default_true")]
        visible: bool,
        #[serde(default = "default_opacity")]
        #[value(default = "default_opacity")]
        opacity: f32,
        #[serde(default)]
        #[value(default)]
        blend_mode: String,
        adjustment_kind: String,
        #[serde(default)]
        #[value(default)]
        params: AdjustmentParamsJson,
    },
}

fn default_true() -> bool {
    true
}

fn default_opacity() -> f32 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(super) struct TransformJson {pub(super) x:f64,pub(super) y:f64,pub(super) a:f64,pub(super) b:f64,pub(super) c:f64,pub(super) d:f64}

#[derive(Clone, Debug, PartialEq, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(super) struct MaskJson {
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    pub(super) enabled: bool,
    #[serde(default)]
    #[value(default)]
    pub(super) invert: bool,
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    pub(super) linked: bool,
    pub(super) image_key: Option<String>,
    #[serde(default = "default_transform")]
    #[value(default = "default_transform")]
    pub(super) transform: TransformJson,
    pub(super) width: Option<u32>,
    pub(super) height: Option<u32>,
}

fn default_transform()->TransformJson {TransformJson {x:0.0,y:0.0,a:1.0,b:0.0,c:0.0,d:1.0}}

#[derive(Clone, Debug, Default, Deserialize, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct AdjustmentParamsJson {
    pub(super) brightness: Option<f32>,
    pub(super) contrast: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct DocumentJson {
    pub(super) schema: String,
    pub(super) layers: Vec<LayerNodeJson>,
}

#[derive(Clone)]
pub(super) enum LayerNode {
    Pixel { id: String, visible: bool, locked: bool, opacity: f32, blend: BlendMode, transform: Affine, width: u32, height: u32, image_key: Option<String>, mask: Option<MaskState> },
    Group { id: String, visible: bool, locked: bool, opacity: f32, blend: BlendMode, transform: Affine, children: Vec<LayerNode>, mask: Option<MaskState> },
    Adjustment { visible: bool, opacity: f32, blend: BlendMode, kind: String, params: AdjustmentParamsJson },
}

#[derive(Clone)]
pub(super) struct MaskState {
    pub(super) descriptor: MaskJson,
    pub(super) width: u32,
    pub(super) height: u32,
}

#[derive(Clone)]
pub(super) struct RasterDocument {
    pub(super) layers: Vec<LayerNode>,
    pub(super) stack: Vec<RasterStackLayer>,
}

pub(super) fn blend_from_str(raw: &str) -> BlendMode {
    match raw {
        "multiply" => BlendMode::Multiply,
        "screen" => BlendMode::Screen,
        "overlay" => BlendMode::Overlay,
        "darken" => BlendMode::Darken,
        "lighten" => BlendMode::Lighten,
        "colorDodge" => BlendMode::ColorDodge,
        "colorBurn" => BlendMode::ColorBurn,
        "hardLight" => BlendMode::HardLight,
        "softLight" => BlendMode::SoftLight,
        "difference" => BlendMode::Difference,
        "exclusion" => BlendMode::Exclusion,
        "hue" => BlendMode::Hue,
        "saturation" => BlendMode::Saturation,
        "color" => BlendMode::Color,
        "luminosity" => BlendMode::Luminosity,
        _ => BlendMode::Normal,
    }
}

pub(super) fn affine_from_json(t: &TransformJson) -> Affine {
    Affine::new([t.a,t.b,t.c,t.d,t.x,t.y])
}

fn parse_mask(descriptor:MaskJson,width:u32,height:u32)->MaskState {
    MaskState {width:descriptor.width.unwrap_or(width),height:descriptor.height.unwrap_or(height),descriptor}
}

fn parse_layer(raw: LayerNodeJson) -> LayerNode {
    match raw {
        LayerNodeJson::Pixel { id, visible, locked, opacity, blend_mode, transform, mask, width, height, image_key, .. } => LayerNode::Pixel {
            id,
            visible,
            locked,
            opacity: opacity.clamp(0.0, 1.0),
            blend: blend_from_str(&blend_mode),
            transform: affine_from_json(&transform),
            width: width.unwrap_or(512),
            height: height.unwrap_or(512),
            image_key,
            mask: mask.map(|m| parse_mask(m,width.unwrap_or(512),height.unwrap_or(512))),
        },
        LayerNodeJson::Group { id, visible, locked, opacity, blend_mode, transform, mask, children, .. } => {
            LayerNode::Group { id, visible, locked, opacity: opacity.clamp(0.0, 1.0), blend: blend_from_str(&blend_mode), transform: affine_from_json(&transform), children: children.into_iter().map(parse_layer).collect(), mask: mask.map(|m| parse_mask(m,512,512)) }
        }
        LayerNodeJson::Adjustment { visible, opacity, blend_mode, adjustment_kind, params, .. } => LayerNode::Adjustment { visible, opacity: opacity.clamp(0.0, 1.0), blend: blend_from_str(&blend_mode), kind: adjustment_kind, params },
    }
}

pub(super) fn parse_document(json: &str) -> Result<RasterDocument, FrameworkSurfacePaintError> {
    let doc: DocumentJson = serde_json::from_str(json)?;
    if doc.schema != "raster.document" {
        return Err(FrameworkSurfacePaintError::UnsupportedSchema(doc.schema));
    }
    let stack=compositing::layers(&doc.layers).map_err(FrameworkSurfacePaintError::Composite)?;
    Ok(RasterDocument { layers: doc.layers.into_iter().map(parse_layer).collect(),stack })
}
// #endregion 🔖️Document
