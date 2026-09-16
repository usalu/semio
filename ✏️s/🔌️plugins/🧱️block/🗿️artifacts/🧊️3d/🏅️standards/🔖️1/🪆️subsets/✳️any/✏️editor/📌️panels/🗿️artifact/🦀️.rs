//! 📄️ Block 3D play app panel — the document tree: representation catalog + rim-vortex templates,
//! selectable.

use crate::Block3dSnapshot;
use crate::editor::block3d::terminology::Block3dLabels;
use crate::editor::block3d::{ui_label, BLOCK3D_GRANULARITY_SURFACE, BLOCK3D_GRANULARITY_VORTEX, BLOCK3D_INTERACTION_VORTEX, BLOCK3D_PLAY_APP_ID};
use semio_framework_plugin::{tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BLOCK3D_BODY_ARTIFACT: &str = "block3d.play.artifact";
pub const BLOCK3D_DOCUMENT_REPRESENTATIONS: &str = "block3d-play-document.representations";
pub const BLOCK3D_DOCUMENT_VORTICES: &str = "block3d-play-document.vortices";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(BLOCK3D_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ One pick row of the `vortex` domain: the raw canonical target id, its `granularity`, and no
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
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "block3d tree text admission failed"))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `surface:{id}`/`vortex:{id}` targets `Block3dPlayApp::interaction_topology` declares for the
/// `vortex` domain — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item click
/// action is declared here anymore (clicks are translated into `interactionSelect` generically)?.
pub fn render(definition: &Block3dSnapshot, labels: &Block3dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    PanelTreeBuilder::new("block3d-play-document")?
        .window_section_or_placeholder(
            windows,
            BLOCK3D_DOCUMENT_REPRESENTATIONS,
            Some(ui_label(labels.representations.as_str())?),
            true,
            &definition.representations,
            |representation| icon_item(format!("surface:{}", representation.id), &representation.name, representation.mesh_url.clone(), "box", BLOCK3D_GRANULARITY_SURFACE),
            ui_label(labels.no_representations.as_str())?,
        )?
        .window_section_or_placeholder(
            windows,
            BLOCK3D_DOCUMENT_VORTICES,
            Some(ui_label(labels.vortices.as_str())?),
            true,
            &definition.vortices,
            |vortex| icon_item(format!("vortex:{}", vortex.id), &vortex.vortex_kind, None, "circle-dot", BLOCK3D_GRANULARITY_VORTEX),
            ui_label(labels.no_vortices.as_str())?,
        )?
        .interaction_domain(BLOCK3D_PLAY_APP_ID, BLOCK3D_INTERACTION_VORTEX)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
