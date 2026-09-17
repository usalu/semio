use super::*;
use crate::default_remodeling_scene;
use crate::editor::remodeling::commands::set_active_example::SetActiveExample;
use crate::editor::remodeling::unit_tests::context::{app, dispatch, render as render_body};
use crate::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn an_unset_frame_cursor_renders_no_layers_for_a_document_without_streams() {
    assert_eq!(frames_layers_json(&default_remodeling_scene(), &RemodelingFrameCursor::default()), "[]");
}

/// 🎞️ The committed Synthetic Orbit example carries one stream of ten frames: once it is the open
/// document, an unset frame cursor shows its first frame (the asset the first frame names is drawn),
/// and a cursor naming a frame the stream lacks falls back to that first frame too.
#[semio_framework_async_macros::async_test]
async fn an_unset_frame_cursor_falls_back_to_the_first_stream_and_frame() {
    let mut app = app().await;
    assert!(dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::synthetic_orbit::ID.into() })).await.edited_document(), "the example load publishes the document lane");
    let scene = app.snapshot().expect("snapshot");
    let stream = scene.streams.first().expect("the example carries a stream");
    let first = stream.frames.first().expect("the example carries frames");
    let layers = frames_layers_json(&scene, &RemodelingFrameCursor::default());
    assert!(layers.contains(&format!("\"id\":\"{}\"", first.asset_id)), "the first frame is drawn: {}", &layers[..layers.len().min(200)]);
    let beyond = frames_layers_json(&scene, &RemodelingFrameCursor { stream_id: Some(stream.id.clone()), frame_index: u32::MAX });
    assert!(beyond.contains(&format!("\"id\":\"{}\"", first.asset_id)), "a frame the stream lacks falls back to its first frame");
    assert_eq!(frames_layers_json(&scene, &RemodelingFrameCursor { stream_id: Some("missing-stream".into()), frame_index: 0 }), "[]", "a cursor naming a stream the document lacks stays empty");
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_FRAMES).await.contains(REMODELING_PLAY_SURFACE_FRAMES));
}

#[semio_framework_async_macros::async_test]
async fn renders_a_canvas_2d_surface() {
    let mut app = app().await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_FRAMES).await.contains(REMODELING_PLAY_SURFACE_FRAMES));
}
