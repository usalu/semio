
use super::*;
use ui_wgpu::wgpu::{Label, LayoutBucket, Node, NodeFlags, NodeKey, Theme, UiPresence, UiStackNode, UiTextNode, WidgetSpec};

fn text_node(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn stack_node(id: Option<&str>, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: id.map(String::from), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

#[test]
fn path_segments_use_kind_index_and_declared_id() {
    let root = stack_node(Some("root"), vec![text_node("a"), stack_node(None, vec![])]);
    assert_eq!(ui_node_path_segment(&root, 0), "stack[0]#root");
    let UiNode::Stack(stack) = &root else { unreachable!() };
    assert_eq!(ui_node_path_segment(&stack.children[0], 0), "text[0]");
    assert_eq!(ui_node_path_segment(&stack.children[1], 1), "stack[1]");
}

#[test]
fn walk_dump_accumulates_absolute_rects_and_builds_full_paths() {
    let mut tree = ui_wgpu::wgpu::UiTree::new();
    let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("root".into()), WidgetSpec(stack_node(Some("root"), vec![]))));
    let child_id = tree.insert_child(Some(root_id), Node::new(NodeKey::Positional(1, 0), WidgetSpec(text_node("hi"))));
    tree.node_mut(root_id).unwrap().layout = LayoutBucket { x: 10.0, y: 20.0, width: 200.0, height: 100.0, ..Default::default() };
    tree.node_mut(child_id).unwrap().layout = LayoutBucket { x: 5.0, y: 6.0, width: 50.0, height: 12.0, ..Default::default() };

    let theme = Theme::default();
    let mut nodes = Vec::new();
    let mut focus_path = None;
    walk_dump(&tree, root_id, 0.0, 0.0, "", 0, &theme, &mut focus_path, &mut nodes);

    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].path, "stack[0]#root");
    assert_eq!(nodes[0].rect, [10.0, 20.0, 200.0, 100.0]);
    assert_eq!(nodes[1].path, "stack[0]#root/text[0]");
    assert_eq!(nodes[1].rect, [15.0, 26.0, 50.0, 12.0], "child rect must be the root's absolute origin plus its own parent-relative offset");
}

#[test]
fn focus_path_is_recorded_for_the_focused_node() {
    let mut tree = ui_wgpu::wgpu::UiTree::new();
    let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("root".into()), WidgetSpec(stack_node(Some("root"), vec![]))));
    let child_id = tree.insert_child(Some(root_id), Node::new(NodeKey::Positional(1, 0), WidgetSpec(text_node("hi"))));
    tree.node_mut(child_id).unwrap().flags.set(NodeFlags::FOCUSED, true);

    let theme = Theme::default();
    let mut nodes = Vec::new();
    let mut focus_path = None;
    walk_dump(&tree, root_id, 0.0, 0.0, "", 0, &theme, &mut focus_path, &mut nodes);

    assert_eq!(focus_path, Some("stack[0]#root/text[0]".to_string()));
}

#[test]
fn kind_tags_match_the_ui_node_wire_format_tag() {
    // 🔒️ Guards path-grammar drift against `UiNode`'s own `#[serde(tag = "type")]` wire format.
    let node = text_node("x");
    let json = serde_json::to_value(&node).unwrap();
    assert_eq!(json.get("type").and_then(|v| v.as_str()), Some(ui_node_kind_tag(&node)));
}
