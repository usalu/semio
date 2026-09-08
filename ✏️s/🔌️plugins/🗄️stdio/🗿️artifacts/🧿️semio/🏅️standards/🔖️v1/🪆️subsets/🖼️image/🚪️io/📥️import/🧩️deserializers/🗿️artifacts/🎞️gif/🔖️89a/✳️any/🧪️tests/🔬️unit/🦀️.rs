
use super::*;
use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifFrame, GifRgb};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_gif() -> GifSnapshot {
    GifSnapshot {
        width: 2,
        height: 1,
        gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 255, g: 0, b: 0 }, GifRgb { r: 0, g: 255, b: 0 }] }),
        loop_count: Some(0),
        comments: vec!["semio fixture".into()],
        frames: vec![GifFrame { left: 0, top: 0, width: 2, height: 1, indices: vec![0, 1], delay_cs: 10, ..GifFrame::default() }],
        ..GifSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn decodes_indices_through_gct_and_maps_comments() {
    let semio = semio_framework_plugin::resolve_ready(SemioImageFromGif::deserialize(&sample_gif())).expect("deserialize");
    assert_eq!(semio.width, 2);
    assert_eq!(semio.height, 1);
    assert_eq!(semio.colorspace, SemioColorspace::Indexed);
    assert_eq!(semio.frames.len(), 1);
    assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    assert_eq!(semio.frames[0].delay_ms, 100);
    assert!(semio.metadata.iter().any(|m| m.key == "comment" && m.value == "semio fixture"));
    assert!(semio.metadata.iter().any(|m| m.key == "loopCount" && m.value == "0"));
}
