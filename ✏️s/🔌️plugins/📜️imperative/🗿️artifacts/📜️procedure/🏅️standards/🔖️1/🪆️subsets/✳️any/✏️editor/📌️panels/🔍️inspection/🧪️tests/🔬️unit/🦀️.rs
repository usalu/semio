use super::*;
use crate::editor::procedure::testkit::{imperative_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn inspection_shows_step_count_summary() {
    let mut app = imperative_app().await;
    assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_INSPECTOR).await.contains("imperative-play-inspector.steps"));
}
