
use super::*;
use crate::editor::block2d::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_inspector_fields() {
    let mut app = new_app().await;
    let json = render_body(&mut app, BLOCK2D_BODY_INSPECTOR).await;
    assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
    assert!(json.contains("Name"));
    assert!(!json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
}
