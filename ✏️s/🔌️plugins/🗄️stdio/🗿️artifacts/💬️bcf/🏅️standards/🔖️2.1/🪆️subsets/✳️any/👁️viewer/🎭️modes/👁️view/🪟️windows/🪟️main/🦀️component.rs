//! 📊 BCF viewer — the Main window: a read-only table of BCF issue topics
//! (guid/title/status/priority/author), built with the shared `TableWindowKit`. `💬️bcf` is
//! issue/markup data, not geometry — `TableWindowKit` fits its natural shape (a flat guid-keyed
//! list of topics) better than `MeshWindowKit`, and better than `TreeWindowKit` too: topics carry
//! no parent/child nesting in the snapshot (comments/viewpoints are per-topic DETAIL, not a tree of
//! topics), so uniform columns are the honest fit, not a recursive node shape. Reads
//! `BcfSnapshot.topics` directly — this file imports nothing from the sibling editor module.

use crate::artifacts::bcf::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;
use semio_framework_plugin::{UiNode, WindowKindDefinition, WindowKit};
use semio_framework_plugin::app::{TableView, TableWindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `BcfSnapshot -> UiNode` read: one row per topic, real fields
/// (guid/title/status/priority/creation author) straight off the document.
pub fn render(document: &BcfSnapshot) -> UiNode {
    let columns = vec!["GUID".to_string(), "Title".to_string(), "Status".to_string(), "Priority".to_string(), "Author".to_string()];
    let rows: Vec<Vec<String>> = document.topics.iter().map(|topic| vec![topic.guid.clone(), topic.title.clone(), topic.status.clone(), topic.priority.clone(), topic.creation_author.clone()]).collect();
    let view = TableView { columns, rows };
    TableWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_the_shared_table_window_kit() {
        assert_eq!(definition().id, TableWindowKit::KIND_ID);
    }

    #[test]
    fn render_produces_a_table_node_for_the_default_document() {
        let document = BcfSnapshot::default();
        let _node = render(&document);
    }

    #[test]
    fn render_lists_one_row_per_topic() {
        let mut document = BcfSnapshot::default();
        document.topics.push(crate::artifacts::bcf::schema::snapshot::BcfTopic { guid: "g1".into(), title: "Clash".into(), description: String::new(), status: "open".into(), priority: "high".into(), labels: Vec::new(), creation_date: String::new(), creation_author: "tester".into(), comments: Vec::new(), viewpoints: Vec::new() });
        let _node = render(&document);
        assert_eq!(document.topics.len(), 1);
    }
}
//#endregion 🧪️Tests
