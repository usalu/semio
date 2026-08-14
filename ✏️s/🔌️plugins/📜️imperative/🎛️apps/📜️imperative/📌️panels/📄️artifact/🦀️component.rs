//! 📄️ Imperative play app panel — the document tree: the top-level steps of the current path.

use crate::apps::imperative::terminology::ImperativeLabels;
use crate::apps::imperative::IMPERATIVE_INTERACTION_STEPS;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_BODY_DOCUMENT: &str = "imperative.play.document";
const IMPERATIVE_PLAY_DOCUMENT_NAMESPACE: &str = "imperative-play-document";
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ Canonical `steps` domain `InteractionTarget` id for a step — the SAME id this tree's own items
/// use, so the framework's post-render presence stamping (`stamp_and_cache_interaction_ui`) can match
/// tree items to their live selection/hover state; also reused by
/// `ImperativePlayApp::interaction_topology` so the topology walks the identical id space (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn step_row_id(id: &str) -> String {
    format!("{IMPERATIVE_PLAY_DOCUMENT_NAMESPACE}.step.{id}")
}
//#endregion 🔖️Interaction

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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `step_row_id` targets `ImperativePlayApp::interaction_topology` declares for the `steps` domain —
/// the framework stamps this tree's selection/hover presence from that domain (`.interaction_domain`)
/// and prunes stale ids through that same topology, so no per-item click action is declared here
/// anymore (clicks are translated into `interactionSelect` generically).
pub fn render(document: &ImperativeSnapshot, labels: &ImperativeLabels) -> UiNode {
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    let step_items: Vec<UiTreeItemNode> = path
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| tree_item_desc(step_row_id(&step.id), Label::data(format!("{}. {}", index + 1, step.kind)), Some(step.id.clone())))
        .collect();
    PanelTreeBuilder::new(IMPERATIVE_PLAY_DOCUMENT_NAMESPACE)
        .section_or_placeholder("imperative-play-document.steps", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, step_items, labels.document_empty)
        .interaction_domain(IMPERATIVE_INTERACTION_STEPS)
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
