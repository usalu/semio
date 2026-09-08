
use super::*;
use crate::Filters;
use crate::editor::sourcing::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn pool_render_respects_query_filter() {
    let document = crate::schema::default_document();
    let cfg = SourcingCurationConfig { filters: Filters { query: "glulam".into(), ..Default::default() }, ..Default::default() };
    let view = pool_view(&document, &cfg, crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default()));
    assert!(view.rows.iter().flatten().any(|cell| cell.contains("Glulam")));
    assert!(!view.rows.iter().flatten().any(|cell| cell.contains("Hollow Core")));
}

#[semio_framework_async_macros::async_test]
async fn pool_stepper_cell_max_equals_availability() {
    let document = crate::schema::default_document();
    let cfg = SourcingCurationConfig::default();
    let stock = crate::stock_of(&document);
    let kind = &stock[0];
    let view = pool_view(&document, &cfg, crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default()));
    let row = view.rows.iter().find(|row| row.first() == Some(&kind.name)).expect("stock row");
    assert_eq!(row[3], kind.availability.to_string());
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, SOURCING_CURATION_BODY_POOL);
    assert!(matches!(def.surface_kind, SurfaceKind::Table));
}

#[semio_framework_async_macros::async_test]
async fn renders_pool_table_scene() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, SOURCING_CURATION_BODY_POOL).await.contains("table"));
}
