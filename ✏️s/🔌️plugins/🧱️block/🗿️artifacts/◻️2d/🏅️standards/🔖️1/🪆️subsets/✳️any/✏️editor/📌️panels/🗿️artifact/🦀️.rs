//! 📄️ Block 2D play app panel — the document tree: handle-kind catalog + rim-handle templates,
//! selectable.

use crate::Block2dSnapshot;
use crate::editor::block2d::terminology::Block2dLabels;
use crate::editor::block2d::{ui_label, BLOCK2D_GRANULARITY_HANDLE, BLOCK2D_GRANULARITY_HANDLE_KIND, BLOCK2D_INTERACTION_HANDLE, BLOCK2D_PLAY_APP_ID};
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BLOCK2D_BODY_ARTIFACT: &str = "block2d.play.artifact";
pub const BLOCK2D_DOCUMENT_HANDLE_KINDS: &str = "block2d-play-document.handle-kinds";
pub const BLOCK2D_DOCUMENT_HANDLES: &str = "block2d-play-document.handles";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(BLOCK2D_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ One pick row of the `handle` domain: the raw canonical target id, its `granularity`, and no
/// binding of its own — the tree carries the single `interactionSelect` the whole panel picks through.
fn icon_item(id: String, label: Label, description: Option<String>, icon: &str, granularity: &str) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut node = tree_item_desc(id, label, description)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(ui_text(icon)?);
        props.granularity = Some(ui_text(granularity)?);
    }
    Ok(node)
}

fn ui_text(value: &str) -> semio_framework_plugin::UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "block2d tree text admission failed"))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `handleKind:{id}`/`handle:{id}` targets `Block2dPlayApp::interaction_topology` declares for the
/// `handle` domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology.
pub fn render(definition: &Block2dSnapshot, labels: &Block2dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    PanelTreeBuilder::new("block2d-play-document")?
        .window_section_or_placeholder(
            windows,
            BLOCK2D_DOCUMENT_HANDLE_KINDS,
            Some(ui_label(labels.handle_kinds.as_str())?),
            true,
            &definition.handle_kinds,
            |kind| icon_item(format!("handleKind:{}", kind.id), ui_label(&kind.label)?, Some(kind.color.clone()), "circle", BLOCK2D_GRANULARITY_HANDLE_KIND),
            ui_label(labels.no_handle_kinds.as_str())?,
        )?
        .window_section_or_placeholder(
            windows,
            BLOCK2D_DOCUMENT_HANDLES,
            Some(ui_label(labels.handles.as_str())?),
            true,
            &definition.handles,
            |handle| icon_item(format!("handle:{}", handle.id), ui_label(&handle.handle_kind)?, Some(format!("{:.2}", handle.angle)), "circle-dot", BLOCK2D_GRANULARITY_HANDLE),
            ui_label(labels.no_handles.as_str())?,
        )?
        .interaction_domain(BLOCK2D_PLAY_APP_ID, BLOCK2D_INTERACTION_HANDLE)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
