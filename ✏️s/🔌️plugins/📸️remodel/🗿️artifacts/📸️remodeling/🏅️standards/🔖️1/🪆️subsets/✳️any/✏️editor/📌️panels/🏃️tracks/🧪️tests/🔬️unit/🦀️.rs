
use super::*;
use crate::editor::remodeling::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn an_empty_track_list_renders_the_documented_gap_message() {
    let mut app = app().await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_TRACKS).await.contains("No motion tracks"));
}
