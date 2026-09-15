//! 🏊️ Sourcing curation app — the pool window: the full stock catalogue with filter chrome + drag source.

use crate::schema::{curated_count, filtered_stock};
use crate::{CurationSnapshot, ObjectKind, SortDirection};
use crate::editor::sourcing::config::SourcingCurationConfig;
use crate::editor::sourcing::terminology::SourcingLabels;
use crate::editor::sourcing::{sourcing_table, sourcing_table_action, sourcing_table_row};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, TableCell, UiAssemblyResult, WindowKindDefinition, WindowOptions};

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
const SOURCING_CURATION_SURFACE_POOL: &str = "sourcing.pool.table";

fn pool_kinds(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> Vec<ObjectKind> {
    let mut filtered = filtered_stock(document, &cfg.filters);
    if let Some(sort) = &cfg.filters.sort {
        filtered.sort_by(|a, b| {
            let ordering = match sort.column_id.as_str() {
                "availability" => a.availability.cmp(&b.availability),
                "module" => a.module_id.cmp(&b.module_id),
                _ => a.name.cmp(&b.name),
            };
            if sort.direction == SortDirection::Desc {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }
    filtered
}

fn pool_row(document: &CurationSnapshot, kind: &ObjectKind) -> protocol::DslValue {
    sourcing_table_row(
        &kind.id,
        vec![
            ("name", TableCell::Text { value: kind.name.clone() }),
            ("module", TableCell::Text { value: kind.module_id.clone() }),
            ("typology", TableCell::Text { value: kind.typology_path.join(" / ") }),
            ("availability", TableCell::Number { value: kind.availability as f64 }),
            ("curated", TableCell::Stepper { value: curated_count(document, &kind.id) as f64, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_table_action("curationSetCount", Some(&kind.id)) }),
        ],
    )
}

pub fn render(document: &CurationSnapshot, cfg: &SourcingCurationConfig, labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    let columns = [
        ("name", labels.col_name.as_str(), true),
        ("module", labels.col_module.as_str(), true),
        ("typology", labels.col_typology.as_str(), false),
        ("availability", labels.col_availability.as_str(), true),
        ("curated", labels.col_curated.as_str(), false),
    ];
    let rows = pool_kinds(document, cfg).iter().map(|kind| pool_row(document, kind)).collect();
    sourcing_table(SOURCING_CURATION_SURFACE_POOL, &columns, rows, "dropOnPool", cfg.filters.sort.as_ref())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
