
use super::*;
use crate::editor::block2d::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_document_tree() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, BLOCK2D_BODY_DOCUMENT).await.contains("Handle Kinds"));
}
