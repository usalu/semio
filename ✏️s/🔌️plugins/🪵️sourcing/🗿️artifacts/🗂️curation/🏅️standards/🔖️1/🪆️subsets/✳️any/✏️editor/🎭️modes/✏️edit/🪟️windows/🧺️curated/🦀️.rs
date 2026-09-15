//! 🧺️ Sourcing curation app — the curated window: the currently-picked objects and their counts.

use crate::CurationSnapshot;
use crate::editor::sourcing::terminology::SourcingLabels;
use crate::editor::sourcing::{sourcing_table, sourcing_table_action, sourcing_table_row};
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

pub fn render(document: &CurationSnapshot, labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    let stock = crate::stock_of(document);
    let columns = [("name", labels.col_name.as_str(), false), ("availability", labels.col_availability.as_str(), false), ("count", labels.col_count.as_str(), false), ("actions", "", false)];
    let rows = document
        .curated
        .iter()
        .filter_map(|item| {
            let kind = stock.iter().find(|kind| kind.id == item.object_id)?;
            let remove = UiTreeItemAction { icon_id: "trash-2".into(), label: Some(labels.remove.into()), action: sourcing_table_action("curationRemove", Some(&kind.id)), placement: Some(UiTreeActionPlacement::Row) };
            Some(sourcing_table_row(
                &kind.id,
                vec![
                    ("name", TableCell::Text { value: kind.name.clone() }),
                    ("availability", TableCell::Number { value: kind.availability as f64 }),
                    ("count", TableCell::Stepper { value: item.count as f64, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_table_action("curationSetCount", Some(&kind.id)) }),
                    ("actions", TableCell::Buttons { buttons: vec![remove] }),
                ],
            ))
        })
        .collect();
    sourcing_table(SOURCING_CURATION_SURFACE_CURATED, &columns, rows, "dropOnCurated", None)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
