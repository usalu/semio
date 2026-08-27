//! 🏊️ Sourcing curate app — the pool window: the full stock catalogue with filter chrome + drag source.

use crate::artifacts::curate::schema::{curated_count, filtered_stock};
use crate::artifacts::curate::{CurateSnapshot, Filters, SortDirection};
use crate::editor::sourcing::config::SourcingCurateConfig;
use crate::editor::sourcing::terminology::SourcingLabels;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SOURCING_CURATE_WINDOW_POOL: &str = "sourcing-pool";
pub const SOURCING_CURATE_BODY_POOL: &str = "sourcing.pool";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATE_WINDOW_POOL.into(),
        label: LocalizedLabel::native("Pool", "Pool"),
        body_key: SOURCING_CURATE_BODY_POOL.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "library".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        interactions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn pool_view(document: &CurateSnapshot, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> TableView {
    let mut filtered = filtered_stock(document, &cfg.filters);
    if let Some(sort) = &cfg.filters.sort {
        filtered.sort_by(|a, b| {
            let ordering = match sort.column_id.as_str() {
                "availability" => a.availability.cmp(&b.availability),
                _ => a.name.cmp(&b.name),
            };
            if sort.direction == SortDirection::Desc {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }
    let rows = filtered
        .iter()
        .map(|kind| vec![kind.name.clone(), kind.module_id.clone(), kind.typology_path.join(" / "), kind.availability.to_string(), curated_count(document, &kind.id).to_string()])
        .collect();
    TableView {
        columns: vec![
            labels.col_name.as_str().to_owned(),
            labels.col_module.as_str().to_owned(),
            labels.col_typology.as_str().to_owned(),
            labels.col_availability.as_str().to_owned(),
            labels.col_curated.as_str().to_owned(),
        ],
        rows,
    }
}

pub fn render(document: &CurateSnapshot, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    TableWindowKit::render(&pool_view(document, cfg, labels))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sourcing::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn pool_render_respects_query_filter() {
        let document = crate::artifacts::curate::schema::default_document();
        let cfg = SourcingCurateConfig { filters: Filters { query: "glulam".into(), ..Default::default() }, ..Default::default() };
        let view = pool_view(&document, &cfg, crate::editor::sourcing::terminology::sourcing_curate_labels(&SourcingCurateConfig::default()));
        assert!(view.rows.iter().flatten().any(|cell| cell.contains("Glulam")));
        assert!(!view.rows.iter().flatten().any(|cell| cell.contains("Hollow Core")));
    }

    #[semio_framework_async_macros::async_test]
    async fn pool_stepper_cell_max_equals_availability() {
        let document = crate::artifacts::curate::schema::default_document();
        let cfg = SourcingCurateConfig::default();
        let stock = crate::artifacts::curate::stock_of(&document);
        let kind = &stock[0];
        let view = pool_view(&document, &cfg, crate::editor::sourcing::terminology::sourcing_curate_labels(&SourcingCurateConfig::default()));
        let row = view.rows.iter().find(|row| row.first() == Some(&kind.name)).expect("stock row");
        assert_eq!(row[3], kind.availability.to_string());
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_table_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, SOURCING_CURATE_BODY_POOL);
        assert!(matches!(def.surface_kind, SurfaceKind::Table));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_pool_table_scene() {
        let mut app = new_app().await;
        assert!(render_body(&mut app, SOURCING_CURATE_BODY_POOL).await.contains("table"));
    }
}
//#endregion 🧪️Tests
