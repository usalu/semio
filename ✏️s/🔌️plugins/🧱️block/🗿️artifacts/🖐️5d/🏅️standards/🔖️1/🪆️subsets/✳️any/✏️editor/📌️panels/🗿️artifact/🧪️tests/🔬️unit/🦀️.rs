
use super::*;
use crate::editor::block5d::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_document_tree() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, BLOCK5D_BODY_DOCUMENT).await.contains("Grip Kinds"));
}
