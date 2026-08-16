//! 🔍️ Imperative play app panel — inspection: read-only summary of the document.

use crate::editor::imperative::terminology::ImperativeLabels;
use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::{ui_inspector_groups_to_tree, ui_inspector_readonly_field, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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
/// ⚠️ Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-selected-step field group
/// (id/kind/params, resolved from `ImperativeConfig::selected_step_ids`) this panel used to build is
/// deleted along with that field — selection is framework-owned state now and
/// `ArtifactApp::render(body_key, doc, cfg)` is never given an `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` are). Documented reduced-fidelity gap, same shape as
/// `🖍️draw`'s `📌️panels/🔍️properties/🦀️component.rs`: falls through to a step-count summary until a
/// resolved-selection render path exists.
pub fn render(document: &ImperativeSnapshot, labels: &ImperativeLabels) -> UiNode {
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "imperative-play-inspector.summary".into(),
        label: Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![ui_inspector_readonly_field("imperative-play-inspector.steps", labels.inspector_steps, path.steps.len().to_string())],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::imperative::testkit::{imperative_app, render as render_body};

    #[test]
    fn inspection_shows_step_count_summary() {
        let mut app = imperative_app();
        assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_INSPECTOR).contains("imperative-play-inspector.steps"));
    }
}
//#endregion 🧪️Tests
