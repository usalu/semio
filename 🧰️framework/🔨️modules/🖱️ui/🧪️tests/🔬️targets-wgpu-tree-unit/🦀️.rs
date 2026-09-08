
use super::*;
use crate::wgpu::Label;
use crate::wgpu::component::ui::{UiNode, UiPresence, UiTextNode};
use ui_contract::{Component, SeparatorProps};

fn text(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn leaf(discriminant: u32, ordinal: u32, value: &str) -> Node {
    Node::new(NodeKey::Positional(discriminant, ordinal), WidgetSpec(text(value)))
}

#[test]
fn insert_and_iterate_children_in_order() {
    let mut tree = UiTree::new();
    let root = tree.insert_child(None, leaf(0, 0, "root"));
    let a = tree.insert_child(Some(root), leaf(1, 0, "a"));
    let b = tree.insert_child(Some(root), leaf(1, 1, "b"));
    let grandchild = tree.insert_child(Some(b), leaf(1, 0, "c"));

    let children: Vec<NodeId> = tree.children(root).collect();
    assert_eq!(children, vec![a, b]);
    let grandchildren: Vec<NodeId> = tree.children(b).collect();
    assert_eq!(grandchildren, vec![grandchild]);
    assert_eq!(tree.children(grandchild).count(), 0);
}

#[test]
fn mark_dirty_sets_layout_and_paint_and_bubbles_subtree_dirty_to_root() {
    let mut tree = UiTree::new();
    let root = tree.insert_child(None, leaf(0, 0, "root"));
    let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
    let grandchild = tree.insert_child(Some(mid), leaf(1, 0, "leaf"));

    tree.mark_dirty(grandchild, NodeFlags::DIRTY_LAYOUT);

    let grandchild_flags = tree.node(grandchild).unwrap().flags;
    assert!(grandchild_flags.contains(NodeFlags::DIRTY_LAYOUT));
    assert!(grandchild_flags.contains(NodeFlags::DIRTY_PAINT));
    assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
}

#[test]
fn mark_dirty_stops_bubbling_once_it_hits_an_already_dirty_ancestor() {
    let mut tree = UiTree::new();
    let root = tree.insert_child(None, leaf(0, 0, "root"));
    let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
    let leaf_a = tree.insert_child(Some(mid), leaf(1, 0, "a"));
    let leaf_b = tree.insert_child(Some(mid), leaf(1, 1, "b"));

    tree.mark_dirty(leaf_a, NodeFlags::DIRTY_PAINT);
    assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));

    // mid and root already carry SUBTREE_DIRTY; this second call must still end up correct
    // (leaf_b itself dirtied, ancestors still dirtied) even though it stops bubbling at `mid`.
    tree.mark_dirty(leaf_b, NodeFlags::DIRTY_PAINT);
    assert!(tree.node(leaf_b).unwrap().flags.contains(NodeFlags::DIRTY_PAINT));
    assert!(!tree.node(leaf_a).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
    assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
}

#[test]
fn remove_detaches_node_and_frees_its_children_slots() {
    let mut tree = UiTree::new();
    let root = tree.insert_child(None, leaf(0, 0, "root"));
    let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
    let grandchild = tree.insert_child(Some(mid), leaf(1, 0, "leaf"));

    tree.remove(mid);

    assert!(!tree.contains(mid));
    assert!(!tree.contains(grandchild));
    assert!(tree.contains(root));
    assert_eq!(tree.children(root).count(), 0);
}

#[test]
fn retained_document_close_retires_one_record_per_step() {
    let id = UiNodeId(1);
    let header = UiDocumentLeaseHeader { generation: 1, surface: SurfaceId::try_from("test.surface").expect("bounded surface"), revision: UiRevision(1), root: id, layout_epoch: 1, node_count: 1 };
    let mut document = UiDocumentTree::new(header).expect("valid header");
    document
        .try_upsert_record(UiNodeRecord {
            id,
            key: "root".try_into().expect("bounded key"),
            component: Component::Separator(SeparatorProps {}),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: Default::default(),
            menu: None,
            children: Default::default(),
        })
        .expect("record admitted");

    assert!(!document.close_step());
    assert!(document.close_step());
}
