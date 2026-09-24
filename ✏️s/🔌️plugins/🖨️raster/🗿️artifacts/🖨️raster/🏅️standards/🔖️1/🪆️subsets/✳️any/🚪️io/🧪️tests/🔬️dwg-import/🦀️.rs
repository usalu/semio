use crate::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes;
use crate::RasterLayerNode;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{encode_drawing, SemioDrawingFormat};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn dwg_bytes(children: Vec<DrawNode>) -> Vec<u8> {
    let drawing = SemioDrawingSnapshot {
        canvas: DrawCanvas { width: 40.0, height: 20.0, background: None },
        styles: vec![DrawStyle { name: "ink".into(), fill: None, stroke: Some(SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }), stroke_width: Some(1.0), opacity: None }],
        layers: vec![DrawLayer { id: "0".into(), name: "0".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
        ..SemioDrawingSnapshot::default()
    };
    encode_drawing(&drawing, SemioDrawingFormat::Dwg).expect("stdio writes the dwg")
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn rectangle(x0: f64, y0: f64, x1: f64, y1: f64) -> DrawNode {
    let p = |x: f64, y: f64| SemioPoint2 { x, y };
    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: p(x0, y0) }, PathSegment::LineTo { to: p(x1, y0) }, PathSegment::LineTo { to: p(x1, y1) }, PathSegment::LineTo { to: p(x0, y1) }, PathSegment::Close], style: Some("ink".into()) }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn only_pixel_layer(document: &crate::RasterSnapshot) -> (u32, u32, crate::RasterImageAsset) {
    assert_eq!(document.layers.len(), 1);
    let RasterLayerNode::Pixel { image_key, width, height, .. } = &document.layers[0] else {
        panic!("expected pixel layer");
    };
    let asset = crate::raster_asset(&document.assets, image_key.as_ref().expect("image key set")).expect("asset content cached");
    (width.expect("width"), height.expect("height"), asset)
}

#[semio_framework_async_macros::async_test]
async fn imports_real_dwg_bytes_into_a_page_of_the_drawing_bounds() {
    let document = deserialize_bytes(&dwg_bytes(vec![rectangle(100.0, 200.0, 130.0, 210.0)])).expect("dwg import");
    let (width, height, asset) = only_pixel_layer(&document);
    assert_eq!((width, height), (30, 10), "the page is the world bounds, not the file's canvas or origin");
    assert_eq!(asset.mime, "image/png");
    let image = crate::io::semio_image_from_png_bytes(&asset.data).expect("canonical png");
    assert!(image.frames[0].rgba8.chunks(4).any(|px| px[3] > 0), "the rectangle outline is painted");
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[semio_framework_async_macros::async_test]
async fn imports_empty_dwg_into_blank_raster_document() {
    let document = deserialize_bytes(&dwg_bytes(Vec::new())).expect("empty dwg import");
    let (width, height, asset) = only_pixel_layer(&document);
    assert_eq!((width, height), (1, 1));
    assert!(!asset.data.is_empty());
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[semio_framework_async_macros::async_test]
async fn rejects_bytes_that_are_not_dwg() {
    assert!(deserialize_bytes(b"not a dwg file").is_err());
}
