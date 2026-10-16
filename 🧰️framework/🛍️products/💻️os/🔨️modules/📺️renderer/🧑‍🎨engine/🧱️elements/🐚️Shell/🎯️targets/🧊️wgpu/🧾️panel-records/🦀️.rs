//! 🧾️ Panel records assembler, tool-run panel and app-catalogue attempt.
//!
//! `panel_ui_records`, `PanelProjection`, the settings/staged-argument/agent-chat row helpers, the tool-run
//! panel reveal law and the app-catalogue attempt.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler

pub(super) fn panel_component_scene_props(scene: &ui_wgpu::wgpu::UiComponentSceneNode) -> Result<ui_contract::SurfaceProps, String> {
    let kind = match scene.component_kind {
        ui_wgpu::wgpu::SurfaceKind::Canvas2d => ui_contract::SurfaceKind::Canvas2d,
        ui_wgpu::wgpu::SurfaceKind::World3d => ui_contract::SurfaceKind::World3d,
        ui_wgpu::wgpu::SurfaceKind::NodeGraph => ui_contract::SurfaceKind::NodeGraph,
        ui_wgpu::wgpu::SurfaceKind::TextEditor => ui_contract::SurfaceKind::TextEditor,
        ui_wgpu::wgpu::SurfaceKind::Table => ui_contract::SurfaceKind::Table,
        ui_wgpu::wgpu::SurfaceKind::Paint2d => ui_contract::SurfaceKind::Paint2d,
        ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem => ui_contract::SurfaceKind::VirtualFileSystem,
        ui_wgpu::wgpu::SurfaceKind::TiledMap => ui_contract::SurfaceKind::TiledMap,
        ui_wgpu::wgpu::SurfaceKind::Board2d => ui_contract::SurfaceKind::Board2d,
        ui_wgpu::wgpu::SurfaceKind::IconRender => ui_contract::SurfaceKind::IconRender,
        ui_wgpu::wgpu::SurfaceKind::InkCanvas => ui_contract::SurfaceKind::InkCanvas,
        ui_wgpu::wgpu::SurfaceKind::GraphTimeline => ui_contract::SurfaceKind::GraphTimeline,
        ui_wgpu::wgpu::SurfaceKind::BlockList => ui_contract::SurfaceKind::BlockList,
        ui_wgpu::wgpu::SurfaceKind::DiffView => ui_contract::SurfaceKind::DiffView,
        ui_wgpu::wgpu::SurfaceKind::EventFeed => ui_contract::SurfaceKind::EventFeed,
    };
    let encoded = match scene.component_kind {
        ui_wgpu::wgpu::SurfaceKind::Canvas2d => scene.canvas_2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::World3d => scene.world_3d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::NodeGraph => scene.node_graph.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::TextEditor => scene.text_editor.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Table => scene.table.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Paint2d => scene.paint_2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem => scene.virtual_file_system.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::TiledMap => scene.tiled_map.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Board2d => scene.board2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::IconRender => scene.icon_render.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::InkCanvas => scene.ink_canvas.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::GraphTimeline => scene.graph_timeline.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::BlockList => scene.block_list.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::DiffView => scene.diff_view.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::EventFeed => scene.event_feed.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
    };
    encoded
        .ok_or_else(|| format!("panel ComponentScene '{}' has no {:?} payload", scene.surface_id, scene.component_kind))?
        .map_err(|error| format!("panel ComponentScene '{}' payload exceeds the retained surface contract: {error:?}", scene.surface_id))
}

/// 🧾️ Everything a panel record carries beside its key, component and layout.
#[derive(Default)]
pub(super) struct PanelRecord {
    pub(crate) children: ui_contract::UiNodeChildren,
    pub(crate) bindings: ui_contract::UiNodeBindings,
    pub(crate) activity: ui_contract::Activity,
    pub(crate) disabled: bool,
    pub(crate) label: Option<ui_contract::Label>,
    pub(crate) description: Option<ui_contract::Label>,
}

/// 🎛️ A nested tree leaf that carries only an Activate verb (no control, no further children) is
/// React's inline resolution/control slot, not another expandable row.
pub(super) fn tree_item_is_inline_action_leaf(item: &UiTreeItemNode) -> bool {
    item.action.is_some() && item.control.is_none() && item.default_open.is_none() && item.items.as_ref().map_or(true, |items| items.is_empty())
}

/// 🎛️ One [`UiControlNode`] as the [`UiNode`] it IS — the enum is a narrowed projection of the
/// same variants, so a tree row's inline control projects through the one assembler every other node
/// takes rather than a second, parallel arm table.
pub(super) fn control_ui_node(control: &ui_wgpu::wgpu::component::ui::UiControlNode) -> UiNode {
    use ui_wgpu::wgpu::component::ui::UiControlNode;
    match control.clone() {
        UiControlNode::Input(node) => UiNode::Input(node),
        UiControlNode::Select(node) => UiNode::Select(node),
        UiControlNode::Toggle(node) => UiNode::Toggle(node),
        UiControlNode::Button(node) => UiNode::Button(node),
        UiControlNode::KeyValue(node) => UiNode::KeyValue(node),
        UiControlNode::Slider(node) => UiNode::Slider(node),
        UiControlNode::NumberStepper(node) => UiNode::NumberStepper(node),
        UiControlNode::Ring(node) => UiNode::Ring(node),
        UiControlNode::IconSelect(node) => UiNode::IconSelect(node),
    }
}

/// ⚙️ One labelled `Select` row — React's `{ id, label, control: <Select …/> }` item, whose
/// `onValueChange` becomes the node's `on_change` over the `"framework"` controller.
fn settings_select_row(id: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str) -> UiNode {
    settings_select_row_with(id, label, value, options, action, None)
}

/// 🧾️ The variant name a refused `UiNode` reports under — the assembler's own error vocabulary.
pub(super) fn panel_ui_node_tag(node: &UiNode) -> &'static str {
    match node {
        UiNode::Stack(_) => "stack",
        UiNode::Text(_) => "text",
        UiNode::Button(_) => "button",
        UiNode::Separator(_) => "separator",
        UiNode::Input(_) => "input",
        UiNode::Select(_) => "select",
        UiNode::Toggle(_) => "toggle",
        UiNode::KeyValue(_) => "keyValue",
        UiNode::Slider(_) => "slider",
        UiNode::NumberStepper(_) => "numberStepper",
        UiNode::Ring(_) => "ring",
        UiNode::IconSelect(_) => "iconSelect",
        UiNode::Progress(_) => "progress",
        UiNode::Field(_) => "field",
        UiNode::Section(_) => "section",
        UiNode::Group(_) => "group",
        UiNode::Tree(_) => "tree",
        UiNode::Image(_) => "image",
        UiNode::ComponentScene(_) => "componentScene",
        UiNode::ExternalSlot(_) => "externalSlot",
    }
}
//#endregion 🧾️PanelRecordsAssembler

//#region ⏯️ToolRunPanel

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs"]
mod tool_run_panel_tests;
//#endregion ⏯️ToolRunPanel


#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🛍️app-catalogue-attempt/🦀️.rs"]
mod app_catalogue_attempt_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🪟️action-window-scope/🦀️.rs"]
mod action_window_scope_tests;
//#endregion ShellLifecycle
