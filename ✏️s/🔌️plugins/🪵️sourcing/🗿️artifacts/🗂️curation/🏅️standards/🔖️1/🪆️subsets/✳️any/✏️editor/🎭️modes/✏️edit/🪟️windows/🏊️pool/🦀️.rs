//! 🏊️ Sourcing curation app — the pool window: the full stock catalogue with filter chrome + drag source.

use crate::schema::{curated_count, filtered_stock};
use crate::{CurationSnapshot, SortDirection};
use crate::editor::sourcing::config::SourcingCurationConfig;
use crate::editor::sourcing::terminology::SourcingLabels;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SOURCING_CURATION_WINDOW_POOL: &str = "sourcing-pool";
pub const SOURCING_CURATION_BODY_POOL: &str = "sourcing.pool";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATION_WINDOW_POOL.into(),
        label: LocalizedLabel::native("Pool", "Pool"),
        body_key: SOURCING_CURATION_BODY_POOL.into(),
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
fn pool_view(document: &CurationSnapshot, cfg: &SourcingCurationConfig, labels: &SourcingLabels) -> TableView {
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

pub fn render(document: &CurationSnapshot, cfg: &SourcingCurationConfig, labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    TableWindowKit::render(&pool_view(document, cfg, labels))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
