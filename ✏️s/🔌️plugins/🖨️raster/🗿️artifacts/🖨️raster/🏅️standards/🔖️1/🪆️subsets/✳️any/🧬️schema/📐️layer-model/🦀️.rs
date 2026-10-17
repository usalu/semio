//! 📐️ Viewport, camera, transform, mask and the layer node tree of the raster document.

use super::*;

pub fn default_one() -> f64 {
    1.0
}

pub fn default_true() -> bool {
    true
}

/// 🎞️ Non-destructive raster document: a nested layer tree (pixel/group/adjustment) plus embedded
/// image assets. This is the authoritative projection shared by the wasm compositor bridge and the
/// `raster-plugin` `ArtifactApp`. Ephemeral tool/brush/selection/camera state lives in the app's
/// `RasterConfig`, never here.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterViewportSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterCamera {
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default = "default_one")]
    pub zoom: f64,
}

impl Default for RasterCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub fn one_f32() -> f32 {
    1.0
}

pub fn default_blend() -> String {
    "normal".into()
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterTransform {
    pub x: f64,
    pub y: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Default for RasterTransform {
    fn default() -> Self { Self { x:0.0,y:0.0,a:1.0,b:0.0,c:0.0,d:1.0 } }
}

impl RasterTransform {
    /// 📐️ Coefficients in the shared compositor's affine order.
    pub fn as_affine(&self)->semio_framework_pixels::compositing::CompositeAffine { [self.a,self.b,self.c,self.d,self.x,self.y] }
    /// 🧭️ Keeps the full placement returned by coordinate-frame conversion.
    pub fn from_affine(value:semio_framework_pixels::compositing::CompositeAffine)->Self {
        let [a,b,c,d,x,y]=value;Self {x,y,a,b,c,d}
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerMask {
    #[value(default = "default_true")]
    pub enabled: bool,
    #[value(default = "default_true")]
    pub linked: bool,
    #[value(default)]
    pub invert: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub image_key: Option<String>,
    #[dsl(block)]
    pub transform: RasterTransform,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RasterLayerNode {
    #[value(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        name: String,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[value(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[value(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[value(default)]
        transform: RasterTransform,
        #[dsl(block)]
        mask: Option<RasterLayerMask>,
        width: Option<u32>,
        height: Option<u32>,
        #[dsl(key = "image")]
        image_key: Option<String>,
    },
    #[value(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[value(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[value(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[value(default)]
        transform: RasterTransform,
        #[dsl(block)]
        mask: Option<RasterLayerMask>,
        #[dsl(statements, block)]
        children: Vec<RasterLayerNode>,
    },
    #[value(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {
        id: String,
        name: String,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[value(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[value(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[value(default)]
        transform: RasterTransform,
        #[dsl(key = "kind")]
        adjustment_kind: String,
        #[value(default)]
        params: RasterOwnedMap<semio_framework_value::DslValue>,
    },
}

/// 🧽️ Closes one layer subtree's owned roots. An `Adjustment` layer carries a
/// `RasterOwnedMap<dsl::DslValue>` of parameters whose `Drop` is fail-closed, and a `Group` hides
/// arbitrarily many of them below it, so every seam that throws a layer away — a displaced replay
/// projection, a cold diff, an inverted `create-layer` — closes it through here instead of dropping
/// it and aborting the guest.
pub fn retire_raster_layer(layer: RasterLayerNode) {
    match layer {
        RasterLayerNode::Adjustment { mut params, .. } => params.retire(),
        RasterLayerNode::Group { children, .. } => retire_raster_layers(children),
        RasterLayerNode::Pixel { .. } => {}
    }
}

/// 🧼️ [`retire_raster_layer`] over a whole forest, in document order.
pub fn retire_raster_layers(layers: Vec<RasterLayerNode>) {
    for layer in layers {
        retire_raster_layer(layer);
    }
}
