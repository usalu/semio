//! 📄️ Imperative play app panel — the document tree: the top-level steps of the current path.

use crate::apps::imperative::terminology::ImperativeLabels;
use crate::apps::imperative::imperative_action;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_BODY_DOCUMENT: &str = "imperative.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(IMPERATIVE_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &ImperativeSnapshot, selected: &[String], labels: &ImperativeLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("imperative-play-document");
    let step_items: Vec<UiTreeItemNode> = document
        .path
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| tree_item_with_action(builder.item_id("step", &step.id), Label::data(format!("{}. {}", index + 1, step.kind)), Some(step.id.clone()), imperative_action("setSelection", Some(json!({ "ids": [step.id.clone()] })))))
        .collect();
    builder
        .section_or_placeholder("imperative-play-document.steps", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, step_items, labels.document_empty)
        .selected(selected.iter().map(|id| format!("imperative-play-document.step.{id}")).collect())
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::imperative::testkit::{imperative_app, render as render_body};

    #[test]
    fn document_lists_steps() {
        let mut app = imperative_app();
        assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_DOCUMENT).contains("imperative-play-document.steps"));
    }

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(IMPERATIVE_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
