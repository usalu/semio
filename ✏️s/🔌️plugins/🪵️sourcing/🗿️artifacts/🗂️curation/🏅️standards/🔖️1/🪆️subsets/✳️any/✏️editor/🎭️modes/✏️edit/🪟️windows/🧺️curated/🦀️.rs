//! 🧺️ Sourcing curation app — the curated window: the currently-picked objects and their counts.

use crate::artifacts::curation::CurationSnapshot;
use crate::editor::sourcing::terminology::SourcingLabels;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

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
fn view_model(document: &CurationSnapshot, labels: &SourcingLabels) -> TableView {
    let stock = crate::artifacts::curation::stock_of(document);
    let rows = document
        .curated
        .iter()
        .filter_map(|item| {
            let kind = stock.iter().find(|kind| kind.id == item.object_id)?;
            Some(vec![kind.name.clone(), kind.availability.to_string(), item.count.to_string()])
        })
        .collect();
    TableView {
        columns: vec![labels.col_name.as_str().to_owned(), labels.col_availability.as_str().to_owned(), labels.col_count.as_str().to_owned()],
        rows,
    }
}

pub fn render(document: &CurationSnapshot, labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    TableWindowKit::render(&view_model(document, labels))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
}
//#endregion 🧪️Tests
