//! 🏊️ Sourcing viewer — the pool window: a read-only table of the full stock catalogue, built on the
//! framework `TableWindowKit` (contract §2.6) rather than a bespoke render — a plain flat catalogue
//! table is exactly what the kit's `TableView { columns, rows }` shape already covers, unlike the
//! sibling editor pool window's richer `TableCell`/filter-chrome/drag-source rendering, which is
//! editor-only interaction, not something a read-only viewer needs. MUST NOT import anything from the
//! sibling `editor` module (`policyViewerPurityBreaches`).

use crate::schema::{catalog_kind_name, catalog_module_label, catalog_typology_path, curated_count};
use crate::{stock_of, CurationSnapshot};
use semio_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_plugin::{BuiltNode, UiAssemblyResult, WindowKindDefinition};

pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧱️ Every stock object kind as one flat row — name, module, typology, availability and the curated count read from the same document, localized in `locale`, string cells only (the framework `TableView` view-model has no typed-cell concept, a viewer renders nothing interactive per cell).
pub fn view_model(document: &CurationSnapshot, locale: semio_plugin::Locale) -> TableView {
    use semio_plugin::surface_name as pick;
    let rows = stock_of(document).iter().map(|kind| vec![catalog_kind_name(kind, locale), catalog_module_label(&kind.module_id, &kind.module_id, locale), catalog_typology_path(kind, locale), kind.availability.to_string(), curated_count(document, &kind.id).to_string()]).collect();
    TableView { columns: ["Name", "Module", "Typology", "Availability", "Curated"].iter().zip(["Name", "Modul", "Typologie", "Verfügbarkeit", "Kuratiert"]).map(|(en, de)| pick(locale, en, de).to_string()).collect(), rows }
}

pub fn render(document: &CurationSnapshot, locale: semio_plugin::Locale) -> UiAssemblyResult<BuiltNode> {
    TableWindowKit::render(&view_model(document, locale), locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
