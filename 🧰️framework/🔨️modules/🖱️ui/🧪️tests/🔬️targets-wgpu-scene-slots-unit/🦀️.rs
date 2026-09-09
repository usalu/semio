
use super::*;
use crate::wgpu::Label;
use crate::wgpu::component::ui::{UiComponentSceneNode, UiGroupNode, UiPresence, UiStackNode, UiTextNode};
use crate::wgpu::flex::LayoutEngine;
use crate::wgpu::theme::Theme;

fn text(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn scene(surface_id: &str) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: "ctrl".into(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    })
}

fn image(id: &str) -> UiNode {
    UiNode::Image(UiImageNode { id: id.into(), src: "https://example.test/x.png".into(), alt: None, presence: UiPresence::default(), menu: None })
}

fn stack(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: Some("none".into()), padding: Some("standard".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

fn group(children: Vec<UiNode>) -> UiNode {
    UiNode::Group(UiGroupNode { id: "group".into(), label: Label::data("Group"), default_open: None, presence: UiPresence::default(), children, menu: None })
}

fn layout(node: &UiNode) -> UiTree {
    let mut tree = UiTree::new();
    tree.apply_tree(node);
    let root = tree.root.unwrap();
    let mut engine = LayoutEngine::new();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
    tree
}

#[test]
fn collects_a_scene_leaf_with_its_absolute_rect_accounting_for_ancestor_offsets() {
    let tree = layout(&stack(vec![text("above"), scene("surface.one")]));
    let root = tree.root.unwrap();

    let slots = collect_scene_slots(&tree, root);
    assert_eq!(slots.len(), 1);
    let slot = &slots[0];
    assert_eq!(slot.surface(), Some(("surface.one", SurfaceKind::World3d)));
    // The scene leaf is the stack's second child (below the text sibling plus the stack's own
    // top padding) -- a nonzero absolute y proves ancestor offsets were accumulated, not just
    // the leaf's own parent-relative `LayoutBucket` coordinates.
    assert!(slot.rect.y > 0.0, "expected the scene leaf offset below its text sibling, got y={}", slot.rect.y);
    assert!(slot.rect.w > 0.0 && slot.rect.h > 0.0);
}

#[test]
fn finds_no_slots_when_the_tree_has_no_scene_nodes() {
    let tree = layout(&stack(vec![text("only text")]));
    let root = tree.root.unwrap();
    assert!(collect_scene_slots(&tree, root).is_empty());
}

#[test]
fn collects_multiple_scene_leaves_in_document_order() {
    let tree = layout(&stack(vec![scene("surface.a"), scene("surface.b")]));
    let root = tree.root.unwrap();

    let slots = collect_scene_slots(&tree, root);
    let ids: Vec<&str> = slots.iter().filter_map(|slot| slot.surface().map(|(id, _)| id)).collect();
    assert_eq!(ids, vec!["surface.a", "surface.b"]);
}

#[test]
fn collects_an_image_leaf_alongside_a_scene_leaf() {
    let tree = layout(&stack(vec![image("img.one"), scene("surface.one")]));
    let root = tree.root.unwrap();

    let slots = collect_scene_slots(&tree, root);
    assert_eq!(slots.len(), 2);
    assert!(matches!(slots[0].content, SlotContent::Image(node) if node.id == "img.one"));
    assert!(matches!(slots[1].content, SlotContent::Scene(node) if node.surface_id == "surface.one"));
}

#[test]
fn collects_a_scene_leaf_nested_under_a_group_ancestor() {
    // 🌳️ Regression for the shadow-walk gap this bridge replaces: the legacy immediate-mode walk
    // it superseded only recursed into Stack/Section/Field, so a ComponentScene nested under a
    // Group never resolved to real content. `collect_scene_slots_node` recurses into every
    // node's `tree.children` unconditionally, so a Group ancestor is no different from a Stack.
    let tree = layout(&group(vec![text("label"), scene("surface.nested")]));
    let root = tree.root.unwrap();

    let slots = collect_scene_slots(&tree, root);
    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0].surface(), Some(("surface.nested", SurfaceKind::World3d)));
}

//#region 🎞️RetainedSceneLaws
#[test]
fn scene_paint_cursor_rejects_stale_node_without_consuming_owner() {
    let tree = layout(&stack(vec![scene("surface.a"), image("image.b")]));
    let Some(root) = tree.root else { panic!("scene root") };
    let mut children = tree.children(root);
    let Some(first) = children.next() else { panic!("first scene child") };
    let Some(second) = children.next() else { panic!("second scene child") };
    let mut cursor = ScenePaintCursor::default();
    assert_eq!(cursor.bind(first), Ok(false));
    assert_eq!(cursor.bind(second), Err(ScenePaintCursorError::NodeMismatch));
    assert_eq!(cursor.bind(first), Ok(true));
    assert!(!cursor.terminal_is_empty());
}

#[test]
fn scene_paint_cursor_advances_one_scalar_and_closes_one_bound_owner() {
    let tree = layout(&scene("surface.scalar"));
    let Some(root) = tree.root else { panic!("scene root") };
    let mut cursor = ScenePaintCursor::default();
    assert_eq!(cursor.bind(root), Ok(false));
    assert_eq!(cursor.byte(), 0);
    assert_eq!(cursor.advance_byte(), Ok(()));
    assert_eq!(cursor.byte(), 1);
    assert!(!cursor.close_step());
    assert!(cursor.terminal_is_empty());
    assert!(cursor.close_step());
}
//#endregion 🎞️RetainedSceneLaws
