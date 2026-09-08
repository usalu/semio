
use super::*;
use crate::wgpu::Label;
use crate::wgpu::component::ui::UiPresence;
use crate::wgpu::geometry::Rect;
use std::collections::HashMap;

fn hit(kind: HitKind, axis: Option<DragAxis>) -> HitTarget<()> {
    HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: None, kind, drag_axis: axis, drag_data: None }
}

#[test]
fn dock_split_horizontal_uses_ew_cursor() {
    let mut target = hit(HitKind::DockSplit, Some(DragAxis::Horizontal));
    target.control_id = Some("dock.split.0.0".into());
    let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
    assert_eq!(cursor, SemioCursor::EwResize);
}

#[test]
fn dock_join_corner_uses_move_cursor() {
    let target = hit(HitKind::DockJoinCorner, Some(DragAxis::Both));
    let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
    assert_eq!(cursor, SemioCursor::Move);
}

#[test]
fn dock_tab_uses_grab_cursor() {
    let cursor = resolve_semio_cursor(Some(&hit(HitKind::Window, None)), CursorDragState::default());
    assert_eq!(cursor, SemioCursor::Grab);
}

#[test]
fn panel_resize_uses_ew_cursor() {
    let cursor = resolve_semio_cursor(Some(&hit(HitKind::PanelResize, Some(DragAxis::Horizontal))), CursorDragState::default());
    assert_eq!(cursor, SemioCursor::EwResize);
}

#[test]
fn active_slider_drag_uses_grabbing() {
    let cursor = resolve_semio_cursor::<()>(None, CursorDragState { pointer_drag_active: true, pointer_drag_axis: Some(DragAxis::Horizontal), pointer_drag_kind: Some(HitKind::Slider), ..CursorDragState::default() });
    assert_eq!(cursor, SemioCursor::Grabbing);
}

#[test]
fn tree_draggable_label_uses_grab() {
    let mut target = hit(HitKind::TreeItem, Some(DragAxis::Both));
    target.drag_data = Some(HashMap::from([("id".into(), "x".into())]));
    let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
    assert_eq!(cursor, SemioCursor::Grab);
}

#[test]
fn dark_theme_cursor_urls_use_dark_assets() {
    assert_eq!(semio_cursor_css(SemioCursor::Default, true), "url(/🖼️assets/👆️cursor/🖱️default/🌙️dark.svg) 0 0, default");
    assert_eq!(semio_cursor_css(SemioCursor::Selectable, false), "url(/🖼️assets/👆️cursor/☑️selectable/☀️light.svg) 0 0, pointer");
}

//#region 🔖️RetainedTreeCursorTests
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiInputNode, UiStackNode, UiTextNode};
use crate::wgpu::events::ScrollAxis;
use crate::wgpu::tree::{Node, NodeKey, WidgetSpec};

fn leaf(node: UiNode) -> (UiTree, NodeId) {
    let mut tree = UiTree::new();
    let id = tree.insert_child(None, Node::new(NodeKey::Positional(0, 0), WidgetSpec(node)));
    (tree, id)
}

#[test]
fn hovering_an_input_uses_the_text_cursor() {
    let (tree, id) = leaf(UiNode::Input(UiInputNode {
        id: "name".into(),
        input_kind: "text".into(),
        value: String::new(),
        placeholder: None,
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        on_change: ActionDescriptor { controller_id: "c".into(), action: "a".into(), args: None },
        presence: UiPresence::default(),
        menu: None,
    }));
    assert_eq!(resolve_semio_cursor_from_tree(&tree, Some(id), None), SemioCursor::Text);
}

#[test]
fn hovering_a_drag_source_uses_the_grab_cursor() {
    let (mut tree, id) = leaf(UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None }));
    tree.node_mut(id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
    assert_eq!(resolve_semio_cursor_from_tree(&tree, Some(id), None), SemioCursor::Grab);
}

#[test]
fn an_active_drag_capture_overrides_whatever_is_merely_hovered() {
    let (tree, dragged) = leaf(UiNode::Text(UiTextNode { value: Label::data("x"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }));
    let cursor = resolve_semio_cursor_from_tree(&tree, None, Some((dragged, CaptureKind::Drag)));
    assert_eq!(cursor, SemioCursor::Grabbing);
}

#[test]
fn a_vertical_scroll_thumb_capture_uses_the_ns_resize_cursor() {
    let (tree, scrollable) = leaf(UiNode::Text(UiTextNode { value: Label::data("x"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }));
    let cursor = resolve_semio_cursor_from_tree(&tree, None, Some((scrollable, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));
    assert_eq!(cursor, SemioCursor::NsResize);
}
//#endregion 🔖️RetainedTreeCursorTests
