
use super::*;
use crate::editor::sourcing::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, SOURCING_CURATION_BODY_CURATED);
    assert!(matches!(def.surface_kind, SurfaceKind::Table));
}

#[semio_framework_async_macros::async_test]
async fn renders_curated_table_scene() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SOURCING_CURATION_BODY_CURATED).await.contains("table"));
}
