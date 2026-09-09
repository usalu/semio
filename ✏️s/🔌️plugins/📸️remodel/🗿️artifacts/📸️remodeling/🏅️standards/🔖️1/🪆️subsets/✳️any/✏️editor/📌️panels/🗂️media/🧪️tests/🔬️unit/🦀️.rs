use super::*;
use crate::editor::remodeling::commands::import_frame_payload::testkit_import_checker_stream;
use crate::editor::remodeling::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_media_panel_lists_every_imported_stream() {
    let mut app = app().await;
    testkit_import_checker_stream(&mut app, 2).await;
    let body = render_body(&mut app, REMODELING_PLAY_BODY_MEDIA).await;
    assert!(body.contains("frame-0.png"), "the stream's name is listed: {body}");
    assert!(body.contains("remodeling-media-drop"));
}
