// Generated from draw-🛂manifest.jsonlayers.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const DRAWLAYERS_LAYER_SHAPE: &str = "shape";
pub const DRAWLAYERS_LAYER_PATH: &str = "path";
pub const DRAWLAYERS_LAYER_TEXT: &str = "text";
pub const DRAWLAYERS_LAYER_IMAGE: &str = "image";
pub const DRAWLAYERS_LAYER_GROUP: &str = "group";
pub const DRAWLAYERS_LAYER_BOOLEAN: &str = "boolean";
pub const DRAWLAYERS_LAYER_TRACE: &str = "trace";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DrawLayersLayerKind {
    #[serde(rename = "shape")]
    Shape,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "trace")]
    Trace,
}

impl DrawLayersLayerKind {
    pub const ALL: &'static [Self] = &[DrawLayersLayerKind::Shape, DrawLayersLayerKind::Path, DrawLayersLayerKind::Text, DrawLayersLayerKind::Image, DrawLayersLayerKind::Group, DrawLayersLayerKind::Boolean, DrawLayersLayerKind::Trace];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shape => "shape",
            Self::Path => "path",
            Self::Text => "text",
            Self::Image => "image",
            Self::Group => "group",
            Self::Boolean => "boolean",
            Self::Trace => "trace",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "shape" => Ok(Self::Shape),
            "path" => Ok(Self::Path),
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            "group" => Ok(Self::Group),
            "boolean" => Ok(Self::Boolean),
            "trace" => Ok(Self::Trace),
            other => Err(format!("unknown layer kind {other:?} for DrawLayers")),
        }
    }
}

pub const DRAWLAYERS_LAYER_IDS: &[&str] = &["shape", "path", "text", "image", "group", "boolean", "trace"];
pub const DRAWLAYERS_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"draw-layers\",\"name\":\"Draw Document Layers\",\"layerKinds\":[{\"id\":\"shape\",\"name\":\"Shape\",\"properties\":[{\"name\":\"shapeKind\",\"kind\":\"data\",\"valueType\":{\"kind\":\"text\"}}]},{\"id\":\"path\",\"name\":\"Path\"},{\"id\":\"text\",\"name\":\"Text\"},{\"id\":\"image\",\"name\":\"Image\"},{\"id\":\"group\",\"name\":\"Group\"},{\"id\":\"boolean\",\"name\":\"Boolean\"},{\"id\":\"trace\",\"name\":\"Trace\"}]}";

pub fn draw_layers_manifest() -> Manifest {
    serde_json::from_str(DRAWLAYERS_MANIFEST_JSON).expect("manifest json")
}
