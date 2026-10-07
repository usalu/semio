//! 🧪️ Synthetic image-bearing raster document used exclusively by native tests.

use super::*;
use crate::RasterImageAsset;

pub fn raster_image_test_snapshot() -> RasterSnapshot {
    let mut assets = RasterOwnedMap::new();
    let emblem = RasterImageAsset { mime: "image/png".into(), data: base64_codec::base64_standard_decode("iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAEklEQVR42mP4z8DwHwyBNBgAAEnICfcD2WTxAAAAAElFTkSuQmCC").unwrap_or_default() };
    assets.insert("semio-emblem".into(), crate::mint_raster_asset_child("semio-emblem", &emblem)).expect("single fixture asset fits the owned map");
    let mut params = RasterOwnedMap::new();
    params.insert("brightness".into(), semio_framework_value::DslValue::float(0.12)).expect("first fixture adjustment fits the owned map");
    params.insert("contrast".into(), semio_framework_value::DslValue::float(0.08)).expect("second fixture adjustment has a distinct key and fits the owned map");
    RasterSnapshot {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "semio-demo".into(),
        title: Some("Semio Raster Demo".into()),
        layers: vec![
            RasterLayerNode::Pixel {
                id: "backdrop".into(),
                name: "Backdrop".into(),
                visible: true, locked: false,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(1024),
                height: Some(1024),
                image_key: Some("semio-emblem".into()),
            },
            RasterLayerNode::Adjustment { id: "brighten".into(), name: "Brighten".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "brightnessContrast".into(), params },
        ],
        assets,
    }
}

