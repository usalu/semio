//! 🏗️ Scene builders: the component scene constructor, the scene builders for each scene kind and the text identifier occurrences. See [`super`].

use super::*;

// #region ui
//#region 🔖️UiNode
#[allow(clippy::too_many_arguments, reason = "one arg per scene-kind payload; grouping into a struct is a T2 restructure, out of scope")]
fn component_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    component_kind: SurfaceKind,
    pane_id: Option<String>,
    binding_id: Option<String>,
    canvas_2d: Option<Canvas2dScene>,
    world_3d: Option<World3dScene>,
    node_graph: Option<NodeGraphScene>,
    text_editor: Option<TextEditorScene>,
    table: Option<TableScene>,
    paint_2d: Option<Paint2dScene>,
    virtual_file_system: Option<VirtualFileSystemScene>,
    tiled_map: Option<TiledMapScene>,
    board2d: Option<Board2dScene>,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        menu: None,
        host_id: String::new(),
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind,
        pane_id,
        binding_id,
        presence: UiPresence::default(),
        canvas_2d,
        world_3d,
        node_graph,
        text_editor,
        table,
        paint_2d,
        virtual_file_system,
        tiled_map,
        board2d,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
    })
}

pub fn build_canvas_2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Canvas2dScene) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::Canvas2d, None, None, Some(scene), None, None, None, None, None, None, None, None)
}

pub fn build_world_3d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: World3dScene) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::World3d, None, None, None, Some(scene), None, None, None, None, None, None, None)
}

pub fn build_node_graph_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: NodeGraphScene) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::NodeGraph, None, None, None, None, Some(scene), None, None, None, None, None, None)
}

pub fn build_text_editor_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TextEditorScene) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::TextEditor, None, None, None, None, None, Some(scene), None, None, None, None, None)
}

//#region 🔖️TextIdentifierOccurrences
/// 🔎️ Expands an offset in `text` to the bounds of the identifier (`[A-Za-z0-9_]+`) it falls in, if any.
pub fn text_identifier_bounds_at(text: &str, offset: usize) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let is_ident = |byte: u8| (byte as char).is_ascii_alphanumeric() || byte == b'_';
    let mut index = offset.min(bytes.len());
    while index > 0 && is_ident(bytes[index - 1]) {
        index -= 1;
    }
    let start = index;
    while index < bytes.len() && is_ident(bytes[index]) {
        index += 1;
    }
    if start == index {
        None
    } else {
        Some((start, index))
    }
}

/// 🔦️ JSON `{selection, hover}` occurrence ranges for the identifier under `cursor`, for editor cross-highlighting.
pub fn text_identifier_occurrences_json(text: &str, cursor: usize) -> Option<String> {
    let (start, end) = text_identifier_bounds_at(text, cursor)?;
    let needle = &text[start..end];
    if needle.is_empty() {
        return None;
    }
    let mut ranges = Vec::new();
    let mut scan = 0usize;
    while let Some(found) = text[scan..].find(needle) {
        let at = scan + found;
        let next_end = at + needle.len();
        if text_identifier_bounds_at(text, at) == Some((at, next_end)) {
            ranges.push(serde_json::json!({ "start": at, "end": next_end }));
        }
        scan = at + needle.len();
    }
    let ranges_json = serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into());
    Some(serde_json::json!({ "selection": ranges_json, "hover": ranges_json }).to_string())
}
//#endregion 🔖️TextIdentifierOccurrences

pub fn build_table_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TableScene) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::Table, None, None, None, None, None, None, Some(scene), None, None, None, None)
}

pub fn build_paint_2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Paint2dScene) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::Paint2d, None, None, None, None, None, None, None, Some(scene), None, None, None)
}

pub fn build_virtual_file_system_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: VirtualFileSystemScene, pane_id: Option<String>, binding_id: Option<String>) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::VirtualFileSystem, pane_id, binding_id, None, None, None, None, None, None, Some(scene), None, None)
}

pub fn build_tiled_map_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TiledMapScene) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::TiledMap, None, None, None, None, None, None, None, None, None, Some(scene), None)
}

pub fn build_board2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Board2dScene) -> UiNode {
    component_scene(surface_id, controller_id, SurfaceKind::Board2d, None, None, None, None, None, None, None, None, None, None, Some(scene))
}

pub fn build_icon_render_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: IconRenderScene) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::IconRender, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
    UiNode::ComponentScene(UiComponentSceneNode { icon_render: Some(scene), ..node })
}

pub fn build_ink_canvas_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: InkCanvasScene) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::InkCanvas, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
    UiNode::ComponentScene(UiComponentSceneNode { ink_canvas: Some(scene), ..node })
}

pub fn build_graph_timeline_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: GraphTimelineScene) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::GraphTimeline, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
    UiNode::ComponentScene(UiComponentSceneNode { graph_timeline: Some(scene), ..node })
}

pub fn build_block_list_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: BlockListScene) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::BlockList, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
    UiNode::ComponentScene(UiComponentSceneNode { block_list: Some(scene), ..node })
}

pub fn build_diff_view_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: DiffViewScene) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::DiffView, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
    UiNode::ComponentScene(UiComponentSceneNode { diff_view: Some(scene), ..node })
}

pub fn build_event_feed_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: EventFeedScene) -> UiNode {
    let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::EventFeed, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
    UiNode::ComponentScene(UiComponentSceneNode { event_feed: Some(scene), ..node })
}
//#endregion 🔖️UiNode
// #endregion ui
