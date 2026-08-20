//! 📊 BCF editor — the Main window: the SAME topic table as the sibling viewer window,
//! built with the shared `TableWindowKit`'s EDITABLE variant (contract §2.6, action id `set-cell`).
//! Render is identical to the viewer's read; mutation is the surface root's `handle()` responsibility.

use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;
use semio_framework_plugin::{UiNode, WindowKindDefinition, WindowKit};
use semio_framework_plugin::app::{TableView, TableWindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    TableWindowKit::editable_window_kind().await
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn columns_and_rows(document: &BcfSnapshot) -> (Vec<String>, Vec<Vec<String>>) {
    let columns = vec!["GUID".to_string(), "Title".to_string(), "Status".to_string(), "Priority".to_string(), "Author".to_string()];
    let rows = document.topics.iter().map(|topic| vec![topic.guid.clone(), topic.title.clone(), topic.status.clone(), topic.priority.clone(), topic.creation_author.clone()]).collect();
    (columns, rows)
}

pub async fn render(document: &BcfSnapshot) -> UiNode {
    let (columns, rows) = columns_and_rows(document).await;
    TableWindowKit::render(&TableView { columns, rows }).await
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_editable_table_window_kit() {
        let def = definition();
        assert_eq!(def.id, TableWindowKit::KIND_ID);
        assert!(def.actions.iter().any(|action| action.id == "set-cell"));
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_table_node_for_the_default_document() {
        let document = BcfSnapshot::default();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
