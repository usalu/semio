use super::*;
use crate::default_remodeling_scene;
use crate::editor::remodeling::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn an_unset_frame_cursor_renders_no_layers() {
    assert_eq!(frames_layers_json(&default_remodeling_scene(), &RemodelingFrameCursor::default()), "[]");
}

#[semio_framework_async_macros::async_test]
async fn renders_a_canvas_2d_surface() {
    let mut app = app().await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_FRAMES).await.contains(REMODELING_PLAY_SURFACE_FRAMES));
}
