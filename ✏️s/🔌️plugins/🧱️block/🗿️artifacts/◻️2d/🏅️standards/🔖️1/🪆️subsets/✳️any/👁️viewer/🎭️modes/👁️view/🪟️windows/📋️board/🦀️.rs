//! 📋️ Block 2D viewer — the board window: a read-only rim-geometry summary (the viewer's only
//! window kind, mirroring the editor's single `📋️board` window). Never calls into the sibling
//! editor module — built directly from the shared artifact-level `Block2dSnapshot`.

use crate::artifacts::block2d::Block2dSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::{column, text, Buildable, HasChildren, Label};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, PluginAssemblyError, SurfaceKind, UiAssemblyResult, UiFixedList, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "block2d-view-board";
pub const BODY_KEY: &str = "block2d.view.board";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::block2d::create_block2d_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Node Kind", "Knotenart"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Board2d,
        icon_id: "layout-grid".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Read-only mirror of the editor's board window, but richer: real per-handle-kind and
/// per-handle-template geometry (id/color, angle/radius) read straight off the shared
/// `Block2dSnapshot` — not a text-summary stub. Built with the same fixed-capacity contract
/// builders the editor's own board window uses (`column`/`text`) — duplicated on purpose rather than
/// imported from the sibling editor module, which viewer purity forbids; block2d's board surface is
/// UI-node-based, not a 3D mesh/world scene, so there is no `world2d_*` helper to reuse here.
fn line(value: String) -> UiAssemblyResult<BuiltNode> {
    let label = Label::try_from(value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d viewer board label admission failed"))?;
    text(label).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d viewer board line admission failed"))
}

pub fn render(document: &Block2dSnapshot) -> UiAssemblyResult<BuiltNode> {
    let mut lines = vec![line(format!("{}: {}", "Node kind", if document.node_kind.label.is_empty() { "\u{2014}" } else { &document.node_kind.label }))?];
    lines.push(line(format!("{} handle kind(s)", document.handle_kinds.len()))?);
    for kind in &document.handle_kinds {
        lines.push(line(format!("  \u{25e6} {} ({}) \u{2014} {}", kind.label, kind.id, kind.color))?);
    }
    lines.push(line(format!("{} handle(s)", document.handles.len()))?);
    for handle in &document.handles {
        lines.push(line(format!("  \u{25e6} {} \u{2014} kind {}, angle {:.1}\u{b0}, radius {:.2}", handle.id, handle.handle_kind, handle.angle.to_degrees(), handle.radius))?);
    }
    let mut children = UiFixedList::<BuiltNode>::default();
    for node in lines {
        children.try_push(node).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d viewer board child admission failed"))?;
    }
    column()
        .try_children(children)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d viewer board children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "block2d viewer board admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_board_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, BODY_KEY);
        assert!(matches!(definition.surface_kind, SurfaceKind::Board2d));
    }

    #[semio_framework_async_macros::async_test]
    async fn render_lists_real_handle_kind_and_handle_geometry() {
        use crate::artifacts::block2d::{Block2dHandleKind, Block2dHandleTemplate};
        let mut document = crate::artifacts::block2d::schema::empty_block2d_snapshot();
        document.handle_kinds.push(Block2dHandleKind { id: "k1".into(), name: "k1".into(), label: "Cable".into(), color: "#ff0000".into(), default_wire_kind: "cable.link".into() });
        document.handles.push(Block2dHandleTemplate { id: "h1".into(), handle_kind: "k1".into(), angle: std::f64::consts::PI, radius: 0.5 });
        let json = serde_json::to_string(&render(&document).expect("board render")).expect("render json");
        assert!(json.contains("Cable"), "board render must surface real handle-kind geometry: {json}");
        assert!(json.contains("radius 0.50"), "board render must surface real handle-template geometry: {json}");
    }
}
//#endregion 🧪️Tests
