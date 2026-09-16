//! 📄️ Block 5D play app panel — the document tree: grip-kind catalog + rim-grip templates, selectable.

use crate::Block5dSnapshot;
use crate::editor::block5d::terminology::Block5dLabels;
use crate::editor::block5d::{ui_label, BLOCK5D_GRANULARITY_GRIP, BLOCK5D_GRANULARITY_GRIP_KIND, BLOCK5D_INTERACTION_GRIP, BLOCK5D_PLAY_APP_ID};
use semio_framework_plugin::{tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BLOCK5D_BODY_ARTIFACT: &str = "block5d.play.artifact";
pub const BLOCK5D_DOCUMENT_GRIP_KINDS: &str = "block5d-play-document.grip-kinds";
pub const BLOCK5D_DOCUMENT_GRIPS: &str = "block5d-play-document.grips";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(BLOCK5D_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ One pick row of the `grip` domain: the raw canonical target id, its `granularity`, and no
/// binding of its own — the tree carries the single `interactionSelect` the whole panel picks through.
fn icon_item(id: String, label: &str, description: Option<String>, icon: &str, granularity: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut node = tree_item_desc(id, ui_label(label)?, description)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(ui_text(icon)?);
        props.granularity = Some(ui_text(granularity)?);
    }
    Ok(node)
}

fn ui_text(value: &str) -> semio_framework_plugin::UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "block5d tree text admission failed"))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `gripKind:{id}`/`grip:{id}` targets `Block5dPlayApp::interaction_topology` declares for the `grip`
/// domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology.
pub fn render(definition: &Block5dSnapshot, labels: &Block5dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    PanelTreeBuilder::new("block5d-play-document")?
        .window_section_or_placeholder(
            windows,
            BLOCK5D_DOCUMENT_GRIP_KINDS,
            Some(ui_label(labels.grip_kinds.as_str())?),
            true,
            &definition.grip_kinds,
            |kind| icon_item(format!("gripKind:{}", kind.id), &kind.label, Some(kind.color.clone()), "circle", BLOCK5D_GRANULARITY_GRIP_KIND),
            ui_label(labels.no_grip_kinds.as_str())?,
        )?
        .window_section_or_placeholder(
            windows,
            BLOCK5D_DOCUMENT_GRIPS,
            Some(ui_label(labels.grips.as_str())?),
            true,
            &definition.grips,
            |grip| icon_item(format!("grip:{}", grip.id), &grip.grip_kind, Some(format!("{:.2}", grip.angle)), "circle-dot", BLOCK5D_GRANULARITY_GRIP),
            ui_label(labels.no_grips.as_str())?,
        )?
        .interaction_domain(BLOCK5D_PLAY_APP_ID, BLOCK5D_INTERACTION_GRIP)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
