use super::*;
use crate::editor::procedure::unit_tests::context::{imperative_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_script_editor() {
    let mut app = imperative_app().await;
    assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_SCRIPT).await.contains("text-editor"));
}
