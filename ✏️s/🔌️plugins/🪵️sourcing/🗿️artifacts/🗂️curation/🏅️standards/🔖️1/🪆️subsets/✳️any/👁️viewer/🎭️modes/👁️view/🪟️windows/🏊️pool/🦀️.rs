//! 🏊️ Sourcing viewer — the pool window: a read-only table of the full stock catalogue, built on the
//! framework `TableWindowKit` (contract §2.6) rather than a bespoke render — a plain flat catalogue
//! table is exactly what the kit's `TableView { columns, rows }` shape already covers, unlike the
//! sibling editor pool window's richer `TableCell`/filter-chrome/drag-source rendering, which is
//! editor-only interaction, not something a read-only viewer needs. MUST NOT import anything from the
//! sibling `editor` module (`policyViewerPurityBreaches`).

use crate::{stock_of, CurationSnapshot};
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult, WindowKindDefinition};

pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧱️ Every stock object kind as one flat row — id/name/module/typology/availability, string cells
/// only (the framework `TableView` view-model has no typed-cell concept, unlike the editor's
/// `TableCell::{Text,Number,Stepper,Buttons}` — a viewer renders nothing interactive per cell).
pub fn view_model(document: &CurationSnapshot) -> TableView {
    let stock = stock_of(document);
    let rows = stock.iter().map(|kind| vec![kind.id.clone(), kind.name.clone(), kind.module_id.clone(), kind.typology_path.join(" / "), kind.availability.to_string()]).collect();
    TableView { columns: vec!["Id".into(), "Name".into(), "Module".into(), "Typology".into(), "Availability".into()], rows }
}

pub fn render(document: &CurationSnapshot) -> UiAssemblyResult<BuiltNode> {
    TableWindowKit::render(&view_model(document))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
