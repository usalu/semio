use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, SemioImageMetadataEntry};

fn sample_semio() -> SemioImageSnapshot {
    SemioImageSnapshot {
        width: 2, height: 1, colorspace: crate::standards::v1::subsets::image::schema::snapshot::SemioColorspace::Rgba, bit_depth: 8,
        frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
        icc: None, metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "semio fixture".into() }],
        ..SemioImageSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn authors_a_valid_rgba8_png_with_metadata() {
    let semio = sample_semio();
    let png = ::semio_framework_async::poll::resolve_ready(SemioImageToPng::serialize(&semio)).unwrap();
    let projection = semio_s_artifact_stdio_png::io::project_png(&png.bytes).unwrap();
    assert_eq!((projection.width, projection.height, projection.bit_depth, projection.color_type), (2, 1, 8, PngColorType::Rgba));
    assert_eq!(projection.pixels, semio.frames[0].rgba8);
    assert_eq!((projection.text_chunks[0].keyword.as_str(), projection.text_chunks[0].value.as_str()), ("Title", "semio fixture"));
    assert_eq!(semio_s_artifact_stdio_png::io::encode_png(&png).unwrap(), png.bytes);
}

#[semio_framework_async_macros::async_test]
async fn refuses_unrepresentable_animation_and_icc() {
    let mut animated = sample_semio(); animated.frames.push(animated.frames[0].clone());
    assert!(::semio_framework_async::poll::resolve_ready(SemioImageToPng::serialize(&animated)).is_err());
    let mut profiled = sample_semio(); profiled.icc = Some(vec![1, 2, 3]);
    assert!(::semio_framework_async::poll::resolve_ready(SemioImageToPng::serialize(&profiled)).is_err());
}
