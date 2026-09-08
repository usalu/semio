
use super::legacy_layout_job::LayoutJob;
use super::*;
use crate::wgpu::Label;
use crate::wgpu::component::ui::{UiFieldNode, UiPresence, UiSectionNode, UiStackNode, UiTextNode};

fn text(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn stack(direction: &str, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: direction.into(), gap: Some("none".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

#[test]
fn persistent_layout_job_is_worker_movable() {
    fn assert_send<T: Send>() {}
    assert_send::<LayoutJob>();
}

#[test]
fn vertical_stack_lays_children_top_to_bottom_with_correct_y_offsets() {
    let mut tree = UiTree::new();
    tree.apply_tree(&stack("vertical", vec![text("hello"), text("a longer line of text")]));
    let root = tree.root.unwrap();
    let mut engine = LayoutEngine::new();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();

    let ran = engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
    assert!(ran);

    let children: Vec<NodeId> = tree.children(root).collect();
    assert_eq!(children.len(), 2);
    let first = &tree.node(children[0]).unwrap().layout;
    let second = &tree.node(children[1]).unwrap().layout;
    assert_eq!(first.y, 0.0);
    assert!(second.y >= first.y + first.height);
}

#[test]
fn horizontal_stack_distributes_equal_leftover_width_across_children() {
    let mut tree = UiTree::new();
    let children = vec![
        UiNode::Separator(crate::wgpu::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
        UiNode::Separator(crate::wgpu::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
        UiNode::Separator(crate::wgpu::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
    ];
    tree.apply_tree(&stack("horizontal", children));
    let root = tree.root.unwrap();
    let mut engine = LayoutEngine::new();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();

    engine.compute(&mut tree, root, &mut atlas, &theme, 300.0, 100.0);

    let widths: Vec<f32> = tree.children(root).map(|id| tree.node(id).unwrap().layout.width).collect();
    assert_eq!(widths.len(), 3);
    for w in &widths {
        assert!((*w - 100.0).abs() < 0.5, "expected equal-thirds width, got {w}");
    }
}

#[test]
fn recomputing_with_nothing_dirty_is_a_no_operation() {
    let mut tree = UiTree::new();
    tree.apply_tree(&stack("vertical", vec![text("hello")]));
    let root = tree.root.unwrap();
    let mut engine = LayoutEngine::new();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();

    assert!(engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0));
    let first_pass = tree.node(root).unwrap().layout.clone();

    let ran_again = engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);
    assert!(!ran_again, "DIRTY_LAYOUT/SUBTREE_DIRTY are cleared after a pass, so a second call must early-out");
    let second_pass = tree.node(root).unwrap().layout.clone();
    assert_eq!((first_pass.x, first_pass.y, first_pass.width, first_pass.height), (second_pass.x, second_pass.y, second_pass.width, second_pass.height));
}

#[test]
fn text_measurement_is_cached_per_unchanged_width() {
    let mut atlas = FontAtlas::builtin();
    let first = atlas.measure("hello world", Some(120.0));
    let second = atlas.measure("hello world", Some(120.0));
    assert_eq!(first, second);

    let mut tree = UiTree::new();
    tree.apply_tree(&stack("vertical", vec![text("cache me")]));
    let root = tree.root.unwrap();
    let mut engine = LayoutEngine::new();
    let theme = Theme::default();
    engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);

    let child = tree.children(root).next().unwrap();
    let cached = tree.node(child).unwrap().layout.cached_text_measure.clone();
    assert!(cached.is_some(), "text node should have a cached measurement after layout");
}

//#region 🔖️FieldSectionGrowSemantics
#[test]
fn field_child_grows_to_fill_the_label_adjusted_remainder() {
    let mut tree = UiTree::new();
    let field = UiNode::Field(UiFieldNode { id: "f".into(), label: Label::data("Label"), description: None, required: None, error: None, child: Box::new(text("child")), presence: UiPresence::default(), menu: None });
    tree.apply_tree(&field);
    let root = tree.root.unwrap();
    let mut engine = LayoutEngine::new();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();

    engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 100.0);

    let child = tree.children(root).next().unwrap();
    let layout = &tree.node(child).unwrap().layout;
    let label_h = theme.font_size_small;
    let gap = gap_for_token(&theme, Some("standard"));
    assert!((layout.y - (label_h + gap)).abs() < 0.5, "child should start below the label, got y={}", layout.y);
    assert!((layout.height - (100.0 - label_h - gap)).abs() < 0.5, "child should fill the label-adjusted remainder, got height={}", layout.height);
    assert!((layout.width - 200.0).abs() < 0.5, "child should stretch to the field's full width, got width={}", layout.width);
}

#[test]
fn section_children_stack_below_the_header_at_their_own_intrinsic_height_with_gap() {
    let mut tree = UiTree::new();
    let section = UiNode::Section(UiSectionNode { id: "s".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), children: vec![text("a"), text("a longer line of text")], menu: None });
    tree.apply_tree(&section);
    let root = tree.root.unwrap();
    let mut engine = LayoutEngine::new();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();

    engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);

    let children: Vec<NodeId> = tree.children(root).collect();
    assert_eq!(children.len(), 2);
    let first = tree.node(children[0]).unwrap().layout.clone();
    let second = tree.node(children[1]).unwrap().layout.clone();
    assert!((first.y - SECTION_HEADER_HEIGHT).abs() < 0.5, "first child should start right below the header offset, got y={}", first.y);
    assert!((second.y - (first.y + first.height + theme.gap_standard)).abs() < 0.5, "second child should sit one gap below the first child's own intrinsic height, got y={} first.height={}", second.y, first.height);
    assert!((first.width - 200.0).abs() < 0.5, "children should stretch to the section's full width, got width={}", first.width);
}
//#endregion 🔖️FieldSectionGrowSemantics
