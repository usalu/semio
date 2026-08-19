//! 🏊️ Sourcing viewer — the pool window: a read-only table of the full stock catalogue, built on the
//! framework `TableWindowKit` (contract §2.6) rather than a bespoke render — a plain flat catalogue
//! table is exactly what the kit's `TableView { columns, rows }` shape already covers, unlike the
//! sibling editor pool window's richer `TableCell`/filter-chrome/drag-source rendering, which is
//! editor-only interaction, not something a read-only viewer needs. MUST NOT import anything from the
//! sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::curate::{stock_of, CurateSnapshot};
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition};

pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧱️ Every stock object kind as one flat row — id/name/module/typology/availability, string cells
/// only (the framework `TableView` view-model has no typed-cell concept, unlike the editor's
/// `TableCell::{Text,Number,Stepper,Buttons}` — a viewer renders nothing interactive per cell).
pub async fn view_model(document: &CurateSnapshot) -> TableView {
    let stock = stock_of(document);
    let rows = stock
        .iter()
        .map(|kind| vec![kind.id.clone(), kind.name.clone(), kind.module_id.clone(), kind.typology_path.join(" / "), kind.availability.to_string()])
        .collect();
    TableView { columns: vec!["Id".into(), "Name".into(), "Module".into(), "Typology".into(), "Availability".into()], rows }
}

pub async fn render(document: &CurateSnapshot) -> UiNode {
    TableWindowKit::render(&view_model(document))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_uses_the_framework_table_window_kit() {
        let def = definition();
        assert_eq!(def.id, TableWindowKit::KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn view_model_lists_every_stock_row_with_five_columns() {
        let document = crate::artifacts::curate::schema::default_document();
        let stock = stock_of(&document);
        let view = view_model(&document);
        assert_eq!(view.columns.len(), 5);
        assert_eq!(view.rows.len(), stock.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_table_ui_node() {
        let document = crate::artifacts::curate::schema::default_document();
        let json = serde_json::to_string(&render(&document)).expect("render json");
        assert!(json.contains("table"), "expected a table UiNode: {json}");
    }
}
//#endregion 🧪️Tests
