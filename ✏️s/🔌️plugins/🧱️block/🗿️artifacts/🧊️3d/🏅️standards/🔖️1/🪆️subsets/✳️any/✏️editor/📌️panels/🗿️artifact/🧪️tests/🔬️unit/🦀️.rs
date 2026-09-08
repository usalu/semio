
use super::*;
use crate::editor::block3d::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_document_tree() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, BLOCK3D_BODY_DOCUMENT).await.contains("Representations"));
}
