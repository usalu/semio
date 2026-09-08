
use crate::RasterLayerNode;

#[semio_framework_async_macros::async_test]
async fn imports_dwg_polyline_into_raster_document() {
    let mut drawing = semio_s_artifact_stdio_dwg::DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    drawing.entities.push(semio_s_artifact_stdio_dwg::DwgEntity {
        layer,
        color: semio_s_artifact_stdio_dwg::DwgColor::ByLayer,
        geometry: semio_s_artifact_stdio_dwg::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
    });
    drawing.extmin = [0.0, 0.0, 0.0];
    drawing.extmax = [10.0, 10.0, 0.0];
    let document = crate::io::raster_document_json_from_dwg(&drawing).expect("dwg import");
    assert_eq!(document.layers.len(), 1);
    let RasterLayerNode::Pixel { image_key, .. } = &document.layers[0] else {
        panic!("expected pixel layer");
    };
    let asset_key = image_key.as_ref().expect("image key set");
    assert!(document.assets.contains_key(asset_key), "asset handle present");
    let asset = crate::raster_asset(&document.assets, asset_key).expect("asset content cached");
    assert_eq!(asset.mime, "image/png");
    assert!(!asset.data.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn imports_empty_dwg_into_blank_raster_document() {
    let drawing = semio_s_artifact_stdio_dwg::DwgDrawing::default();
    let document = crate::io::raster_document_json_from_dwg(&drawing).expect("empty dwg import");
    assert_eq!(document.layers.len(), 1);
    let RasterLayerNode::Pixel { image_key, width, height, .. } = &document.layers[0] else {
        panic!("expected pixel layer");
    };
    assert_eq!(*width, Some(1));
    assert_eq!(*height, Some(1));
    let asset_key = image_key.as_ref().expect("image key set");
    let asset = crate::raster_asset(&document.assets, asset_key).expect("asset content cached");
    assert!(!asset.data.is_empty());
}
