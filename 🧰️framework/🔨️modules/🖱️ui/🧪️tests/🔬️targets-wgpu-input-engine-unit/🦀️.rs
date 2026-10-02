use super::*;

#[test]
fn an_up_flow_tree_section_registers_its_header_at_the_painted_bottom_edge() {
    let section = Rect::new(100.0, 200.0, 300.0, 240.0);
    assert_eq!(retained_tree_section_header_band(section, 24.0, true), Rect::new(100.0, 416.0, 300.0, 24.0));
    assert_eq!(retained_tree_section_header_band(section, 24.0, false), Rect::new(100.0, 200.0, 300.0, 24.0));
}

#[test]
fn a_childful_up_flow_tree_item_registers_label_and_gutter_on_its_painted_bottom_row() {
    use crate::wgpu::chrome::UiDriverDrag;
    use crate::wgpu::component::ui::{UiNode, UiPresence, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
    use crate::wgpu::layout::TreeRowMetrics;
    use crate::wgpu::tree::{Node, NodeKey, UiTree, WidgetSpec};
    use crate::wgpu::Label;

    let child = UiTreeItemNode::base("child", Label::data("Child"));
    let mut branch = UiTreeItemNode::base("branch", Label::data("Branch"));
    branch.items = Some(vec![child]);
    let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "section".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), items: vec![branch] };
    let mut tree = UiTree::new();
    let owner = tree.insert_child(
        None,
        Node::new(NodeKey::Explicit("tree".into()), WidgetSpec(UiNode::Tree(UiTreeNode { presentation: Default::default(), sections: vec![section], presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None }))),
    );
    let row =
        |id: &str| UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
    let section_row = tree.insert_child(Some(owner), Node::new(NodeKey::Explicit("section".into()), WidgetSpec(row("section"))));
    let branch_row = tree.insert_child(Some(section_row), Node::new(NodeKey::Explicit("branch".into()), WidgetSpec(row("branch"))));
    let _child_row = tree.insert_child(Some(branch_row), Node::new(NodeKey::Explicit("child".into()), WidgetSpec(row("child"))));
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
    let subtree = Rect::new(100.0, 200.0, 300.0, metrics.row_height * 2.0);

    let label = retained_hit_registration(&tree, branch_row, subtree, &metrics, UiDriverDrag::Handle, true).expect("branch label");
    let gutter = retained_tree_chevron_registration(&tree, branch_row, subtree, &metrics, true).expect("branch gutter");
    assert_eq!(label.control_id, "tree.label.branch");
    assert_eq!(gutter.control_id, "tree.chevron.branch");
    assert_eq!(label.rect.y, subtree.y + metrics.row_height);
    assert_eq!(gutter.rect.y, label.rect.y);
    assert_eq!(gutter.rect.h, metrics.row_height);
    assert!(gutter.rect.w > 0.0 && gutter.rect.w < label.rect.w);
}

#[test]
fn shared_tree_drag_fixture_projects_driver_specific_row_and_handle_targets() {
    use crate::wgpu::chrome::UiDriverDrag;
    use crate::wgpu::component::ui::{UiNode, UiPresence, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
    use crate::wgpu::layout::TreeRowMetrics;
    use crate::wgpu::tree::{Node, NodeKey, UiTree, WidgetSpec};
    use crate::wgpu::Label;
    use std::collections::HashMap;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌳️tree-drag-handles/🔣️.json")).expect("tree drag fixture");
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
    let row_width = fixture["rowWidth"].as_f64().expect("row width") as f32;
    for row in fixture["rows"].as_array().expect("rows") {
        let id = row["id"].as_str().expect("row id");
        let drag_data = row["dragData"].as_object().map(|map| map.iter().map(|(key, value)| (key.clone(), value.as_str().expect("payload").to_string())).collect::<HashMap<_, _>>());
        let mut item = UiTreeItemNode::base(id, Label::data(row["label"].as_str().expect("label")));
        item.draggable = row["draggable"].as_bool();
        item.drag_data = drag_data;
        let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "fixture".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };
        let mut tree = UiTree::new();
        let owner = tree.insert_child(
            None,
            Node::new(NodeKey::Explicit("tree".into()), WidgetSpec(UiNode::Tree(UiTreeNode { presentation: Default::default(), sections: vec![section], presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None }))),
        );
        let stack = UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None };
        let row_node = tree.insert_child(Some(owner), Node::new(NodeKey::Explicit(id.into()), WidgetSpec(UiNode::Stack(stack))));
        let rect = Rect::new(0.0, 0.0, row_width, metrics.row_height);

        for driver in fixture["drivers"].as_array().expect("drivers") {
            let mode = if driver["drag"].as_str() == Some("surface") { UiDriverDrag::Surface } else { UiDriverDrag::Handle };
            let row_hit = retained_hit_registration(&tree, row_node, rect, &metrics, mode, false).expect("row hit");
            let handle_hit = retained_tree_drag_handle_registration(&tree, row_node, rect, &metrics, mode);
            let visible = driver["visibleHandles"].as_array().expect("visible handles").iter().any(|value| value.as_str() == Some(id));
            let label_starts = driver["labelStarts"].as_array().expect("label starts").iter().any(|value| value.as_str() == Some(id));
            let handle_starts = driver["handleStarts"].as_array().expect("handle starts").iter().any(|value| value.as_str() == Some(id));
            assert_eq!(row_hit.drag_data.is_some(), label_starts, "{id}: row initiation under {:?}", mode);
            assert_eq!(handle_hit.is_some(), visible, "{id}: handle visibility under {:?}", mode);
            assert_eq!(handle_hit.as_ref().is_some_and(|hit| hit.drag_data.is_some()), handle_starts, "{id}: handle initiation under {:?}", mode);
            if let (Some(hit), Some(role)) = (handle_hit, row["role"].as_str()) {
                assert_eq!(hit.kind, HitKind::TreeDragHandle);
                assert_eq!(hit.control_id, format!("tree.drag.{role}.{id}"));
                assert!(rect.contains(hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5));
            }
        }
    }
}

#[test]
fn number_constraints_clamp_and_snap_exactly_once_each() {
    use crate::wgpu::events::constrain_number_input;
    assert_eq!(constrain_number_input(999.0, Some(0.0), Some(10.0), None), 10.0);
    assert_eq!(constrain_number_input(-4.0, Some(2.0), Some(10.0), None), 2.0);
    assert_eq!(constrain_number_input(3.3, None, None, Some(0.5)), 3.5, "snapping is to the nearest step, not a floor");
    assert_eq!(constrain_number_input(3.4, Some(1.0), None, Some(2.0)), 3.0, "the step ladder starts at `min`, not at zero");
    assert_eq!(constrain_number_input(7.0, None, None, Some(0.0)), 7.0, "a zero step is no step, never a division");
    assert_eq!(constrain_number_input(7.25, None, None, None), 7.25);
    assert!(constrain_number_input(f64::NAN, Some(0.0), Some(10.0), Some(1.0)).is_nan(), "an unparseable buffer stays a refusal — never an invented in-range number");
}

#[test]
fn an_input_metas_commit_value_carries_its_own_constraints() {
    let text = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "text".into(), min: None, max: None, step: None, accept: None };
    assert_eq!(text.commit_value("12abc"), Some(semio_framework_value::DslValue::String("12abc".into())), "a text field commits its text verbatim");

    let number = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "number".into(), min: Some(0.0), max: Some(10.0), step: Some(0.5), accept: None };
    assert_eq!(number.commit_value("99"), Some(semio_framework_value::DslValue::float(10.0)));
    assert_eq!(number.commit_value("3.3"), Some(semio_framework_value::DslValue::float(3.5)));
    assert_eq!(number.commit_value(""), None, "an empty number buffer commits nothing at all");

    let file = crate::wgpu::widgets::InputMeta { on_change: (), commit: None, value: String::new(), input_kind: "file".into(), min: None, max: None, step: None, accept: Some("image/*".into()) };
    assert_eq!(file.accept.as_deref(), Some("image/*"), "a file field's `accept` reaches the host picker instead of being dropped at the render call site");
}
