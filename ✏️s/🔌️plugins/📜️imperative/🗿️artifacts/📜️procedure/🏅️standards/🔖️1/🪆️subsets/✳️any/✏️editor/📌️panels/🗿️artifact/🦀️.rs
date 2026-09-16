//! 📄️ Imperative play app panel — the document tree: the top-level steps of the current path.

use crate::editor::procedure::terminology::ImperativeLabels;
use crate::editor::procedure::{IMPERATIVE_INTERACTION_GRANULARITY, IMPERATIVE_INTERACTION_STEPS, IMPERATIVE_PLAY_APP_ID};
use crate::ProcedureSnapshot;
use semio_framework_plugin::{tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_BODY_ARTIFACT: &str = "imperative.play.artifact";
const IMPERATIVE_PLAY_DOCUMENT_NAMESPACE: &str = "imperative-play-document";
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ Canonical `steps` domain `InteractionTarget` id for a step — the SAME id this tree's own items
/// use, so the framework's post-render presence stamping (`stamp_and_cache_interaction_ui`)? can match
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
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(IMPERATIVE_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `step_row_id` targets `ImperativePlayApp::interaction_topology` declares for the `steps` domain —
/// the framework stamps this tree's selection/hover presence from that domain (`.interaction_domain`)
/// and prunes stale ids through that same topology, so no per-item click action is declared here
/// anymore (clicks are translated into `interactionSelect` generically)?.
///
/// 🪟️ A procedure with more steps than one node's fixed child capacity used to fail the whole render
/// with `ui.fixed-capacity`; the section is windowed now and always stamps the real step `total`.
pub fn render(document: &ProcedureSnapshot, labels: &ImperativeLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let path = crate::procedure_working_scene(document).path;
    let indexed: Vec<_> = path.steps.iter().enumerate().collect();
    PanelTreeBuilder::new(IMPERATIVE_PLAY_DOCUMENT_NAMESPACE)?
        .window_section_or_placeholder(
            windows,
            "imperative-play-document.steps",
            Some(crate::editor::procedure::ui_label(labels.document_title.as_str())?),
            true,
            &indexed,
            |(index, step)| {
                let mut node = tree_item_desc(step_row_id(&step.id), format!("{}. {}", index + 1, step.kind), Some(step.id.clone()))?;
                if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
                    props.granularity = Some(UiText::try_from_str(IMPERATIVE_INTERACTION_GRANULARITY).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "imperative step granularity admission failed"))?);
                }
                Ok(node)
            },
            labels.document_empty.as_str(),
        )?
        .interaction_domain(IMPERATIVE_PLAY_APP_ID, IMPERATIVE_INTERACTION_STEPS)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
