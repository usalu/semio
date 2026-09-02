//! 🔍️ Imperative play app panel — inspection: read-only summary of the document.

use crate::artifacts::imperative::ImperativeSnapshot;
use crate::editor::imperative::terminology::ImperativeLabels;
use semio_framework_plugin::{tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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
/// `🖍️draw`'s `📌️panels/🔍️properties/🦀️.rs`: falls through to a step-count summary until a
/// resolved-selection render path exists.
pub fn render(document: &ImperativeSnapshot, labels: &ImperativeLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    let field = tree_item_desc("imperative-play-inspector.steps", labels.inspector_steps.as_str(), Some(path.steps.len().to_string()))?;
    let mut fields = semio_framework_plugin::UiFixedList::default();
    fields
        .try_push(field)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed inspector field admission failed"))?;
    PanelTreeBuilder::new("imperative-play-inspector")?.section("imperative-play-inspector.summary", Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()), true, fields)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::imperative::testkit::{imperative_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn inspection_shows_step_count_summary() {
        let mut app = imperative_app().await;
        assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_INSPECTOR).await.contains("imperative-play-inspector.steps"));
    }
}
//#endregion 🧪️Tests
