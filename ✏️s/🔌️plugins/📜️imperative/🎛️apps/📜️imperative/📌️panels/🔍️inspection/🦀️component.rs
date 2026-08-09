//! 🔍️ Imperative play app panel — inspection: read-only detail fields for the selected step.

use crate::apps::imperative::terminology::ImperativeLabels;
use crate::artifacts::imperative::{ImperativeSnapshot, Step};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_BODY_INSPECTOR: &str = "imperative.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(IMPERATIVE_PLAY_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &ImperativeSnapshot, selected: &[String], labels: &ImperativeLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "imperative-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.inspector_empty_hint)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let steps: Vec<&Step> = selected.iter().filter_map(|id| document.path.steps.iter().find(|step| &step.id == id)).collect();
    if steps.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "imperative-play-inspector.missing".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.inspector_step_not_found)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "imperative-play-inspector.step".into(),
        label: Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("imperative-play-inspector.id", labels.inspector_id, steps[0].id.clone()),
            ui_inspector_readonly_field("imperative-play-inspector.kind", labels.inspector_kind, steps[0].kind.clone()),
            ui_inspector_readonly_field("imperative-play-inspector.params", labels.inspector_params, serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into())),
        ],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::imperative::testkit::{imperative_app, render as render_body};

    #[test]
    fn inspection_shows_empty_hint_without_selection() {
        let mut app = imperative_app();
        assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_INSPECTOR).contains("imperative-play-inspector.empty"));
    }
}
//#endregion 🧪️Tests
