//! 🧺️ Sourcing curation app — the curated window: the currently-picked objects and their counts.

use crate::editor::sourcing::config::SourcingCurationConfig;
use crate::editor::sourcing::terminology::SourcingLabels;
use crate::editor::sourcing::{sourcing_table, sourcing_table_action, sourcing_table_row};
use crate::{CuratedItem, CurationSnapshot, ObjectKind, SortDirection};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, TableCell, UiAssemblyResult, UiTreeActionPlacement, UiTreeItemAction, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SOURCING_CURATION_WINDOW_CURATED: &str = "sourcing-curated";
pub const SOURCING_CURATION_BODY_CURATED: &str = "sourcing.curated";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATION_WINDOW_CURATED.into(),
        label: LocalizedLabel::native("Curated", "Kuratiert"),
        body_key: SOURCING_CURATION_BODY_CURATED.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "tags".into(),
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
const SOURCING_CURATION_SURFACE_CURATED: &str = "sourcing.curated.table";

/// ↕️ The curated table's own sortable columns. `Filters::sort` is ONE session-wide `TableSort`
/// shared with the pool, so each table applies it only to a column it actually has and otherwise
/// keeps document order — no second sort state model.
pub const SOURCING_CURATION_CURATED_SORT_COLUMNS: [&str; 3] = ["name", "availability", "count"];

/// 🧺️ The curated rows paired with the stock kind each one resolves to, in the order the table shows
/// them: document order, or [`SOURCING_CURATION_CURATED_SORT_COLUMNS`] order when the shared sort
/// names one of this table's columns.
fn curated_rows(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> Vec<(CuratedItem, ObjectKind)> {
    let stock = crate::stock_of(document);
    let mut rows: Vec<(CuratedItem, ObjectKind)> = document.curated.iter().filter_map(|item| stock.iter().find(|kind| kind.id == item.object_id).map(|kind| (item.clone(), kind.clone()))).collect();
    let Some(sort) = cfg.filters.sort.as_ref().filter(|sort| SOURCING_CURATION_CURATED_SORT_COLUMNS.contains(&sort.column_id.as_str())) else { return rows };
    rows.sort_by(|(left_item, left), (right_item, right)| {
        let ordering = match sort.column_id.as_str() {
            "availability" => left.availability.cmp(&right.availability),
            "count" => left_item.count.cmp(&right_item.count),
            _ => left.name.cmp(&right.name),
        };
        if sort.direction == SortDirection::Desc { ordering.reverse() } else { ordering }
    });
    rows
}

/// 🧺️ The curated row's ≤2 actions: curate one more (`curationAdd`, bounded by availability) and
/// uncurate the kind entirely (`curationRemove`).
fn curated_row_actions(item: &CuratedItem, kind: &ObjectKind, labels: &SourcingLabels) -> Vec<UiTreeItemAction> {
    let mut actions = Vec::new();
    if item.count < kind.availability {
        actions.push(UiTreeItemAction { icon_id: "plus".into(), label: Some(labels.curate.into()), action: sourcing_table_action("curationAdd", Some(&kind.id)), placement: Some(UiTreeActionPlacement::Row) });
    }
    actions.push(UiTreeItemAction { icon_id: "trash-2".into(), label: Some(labels.remove.into()), action: sourcing_table_action("curationRemove", Some(&kind.id)), placement: Some(UiTreeActionPlacement::Row) });
    actions
}

pub fn render(document: &CurationSnapshot, cfg: &SourcingCurationConfig, labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    let columns = [("name", labels.col_name.as_str(), true), ("availability", labels.col_availability.as_str(), true), ("count", labels.col_count.as_str(), true), ("actions", labels.col_actions.as_str(), false)];
    let rows = curated_rows(document, cfg)
        .into_iter()
        .map(|(item, kind)| {
            sourcing_table_row(
                &kind.id,
                vec![
                    ("name", TableCell::Text { value: kind.name.clone() }),
                    ("availability", TableCell::Number { value: kind.availability as f64 }),
                    ("count", TableCell::Stepper { value: item.count as f64, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_table_action("curationSetCount", Some(&kind.id)) }),
                    ("actions", TableCell::Buttons { buttons: curated_row_actions(&item, &kind, labels) }),
                ],
            )
        })
        .collect();
    sourcing_table(SOURCING_CURATION_SURFACE_CURATED, &columns, rows, "dropOnCurated", cfg.filters.sort.as_ref())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
