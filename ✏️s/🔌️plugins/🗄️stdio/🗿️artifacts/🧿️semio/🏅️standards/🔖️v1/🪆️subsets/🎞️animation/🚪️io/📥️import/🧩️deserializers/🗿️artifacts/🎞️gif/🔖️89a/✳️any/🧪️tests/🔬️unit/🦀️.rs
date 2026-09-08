
use super::*;
use semio_s_artifact_stdio_gif::schema::snapshot::GifFrame;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_gif() -> GifSnapshot {
    GifSnapshot {
        schema: "stdio.gif.89a".into(),
        width: 10,
        height: 10,
        gct: None,
        background_color_index: 0,
        pixel_aspect_ratio: 0,
        loop_count: Some(0),
        frames: vec![
            GifFrame { left: 0, top: 0, width: 10, height: 10, delay_cs: 10, indices: vec![0; 100], ..GifFrame::default() },
            GifFrame { left: 0, top: 0, width: 10, height: 10, delay_cs: 20, indices: vec![1; 100], ..GifFrame::default() },
            GifFrame { left: 0, top: 0, width: 10, height: 10, delay_cs: 15, indices: vec![2; 100], ..GifFrame::default() },
        ],
        comments: vec![],
        app_extensions: vec![],
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_derives_real_cumulative_delay_as_step_scalar_keyframes() {
    let anim = semio_framework_plugin::resolve_ready(SemioAnimationFromGif::deserialize(&real_world_gif())).expect("deserialize");
    assert_eq!(anim.timelines.len(), 1);
    let ch = &anim.timelines[0].channels[0];
    assert_eq!(ch.target.node, GIF_FRAME_NODE);
    assert_eq!(ch.interpolation, AnimInterpolation::Step);
    assert_eq!(ch.keyframes.len(), 3);
    assert_eq!(ch.keyframes[0].t, 0.0);
    assert_eq!(ch.keyframes[1].t, 0.10);
    assert_eq!(ch.keyframes[2].t, 0.30);
    assert_eq!(ch.keyframes[2].value, AnimValue::Scalar { value: 2.0 });
}

#[semio_framework_async_macros::async_test]
async fn zero_frames_yields_zero_timelines() {
    let mut gif = real_world_gif();
    gif.frames.clear();
    let anim = semio_framework_plugin::resolve_ready(SemioAnimationFromGif::deserialize(&gif)).expect("deserialize");
    assert!(anim.timelines.is_empty());
}
