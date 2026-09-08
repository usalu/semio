use super::*;

fn ui_text(value: &str) -> crate::UiText {
    crate::UiText::try_from_str(value).expect("bounded fixture text")
}

fn label(value: &str) -> crate::Label {
    crate::Label::try_from(value).expect("bounded fixture label")
}

//#region 🔖️WireCost
#[test]
fn button_serializes_to_minimal_json() {
    let node = button(label("Save")).try_build().unwrap_or_else(|_| panic!("button build"));
    let json = serde_json::to_value(&node).expect("serialize");
    assert!(json.get("layout").is_none());
    assert!(json.get("style").is_none());
    assert!(json.get("disabled").is_none());
    assert!(json.get("bindings").is_none());
    assert!(json.get("menu").is_none());
    assert!(json.get("children").is_none());
    assert_eq!(json.get("key").and_then(|v| v.as_str()), Some("#0"));
    assert_eq!(json.get("component").and_then(|c| c.get("type")).and_then(|t| t.as_str()), Some("button"));
}
//#endregion 🔖️WireCost

//#region 🔖️NestedShape
#[test]
fn nested_column_builds_expected_shape() {
    let node = column().try_children([text(label("A")), text(label("B"))]).unwrap_or_else(|_| panic!("bounded children")).try_build().unwrap_or_else(|_| panic!("column build"));
    assert!(matches!(node.component, crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Plain, .. })));
    assert!(matches!(node.layout, crate::LayoutSpec::Stack(crate::StackLayout { axis: crate::Axis::Vertical, .. })));
    assert_eq!(node.children.len(), 2);
    match &node.children[0].component {
        crate::Component::Text(props) => assert_eq!(props.value, label("A")),
        other => panic!("expected text, got {other:?}"),
    }
    match &node.children[1].component {
        crate::Component::Text(props) => assert_eq!(props.value, label("B")),
        other => panic!("expected text, got {other:?}"),
    }
}

#[test]
fn built_children_expose_bounded_shared_and_mutable_lookup() {
    let mut node = column().try_children([text(label("A")), text(label("B"))]).unwrap_or_else(|_| panic!("bounded children")).try_build().unwrap_or_else(|_| panic!("column build"));

    assert!(matches!(node.children.get(0).map(|child| &child.component), Some(crate::Component::Text(_))));
    node.children.get_mut(1).expect("second child").disabled = true;
    assert!(node.children.get(1).expect("second child").disabled);
    assert!(node.children.get(2).is_none());
}

#[test]
fn mixed_child_types_nest_through_child() {
    let node = row().try_child(text(label("A"))).unwrap_or_else(|_| panic!("first child")).try_child(button(label("Go"))).unwrap_or_else(|_| panic!("second child")).try_build().unwrap_or_else(|_| panic!("row build"));
    assert_eq!(node.children.len(), 2);
    assert!(matches!(node.children[0].component, crate::Component::Text(_)));
    assert!(matches!(node.children[1].component, crate::Component::Button(_)));
}
//#endregion 🔖️NestedShape

//#region 🔖️Bindings
#[test]
fn on_lands_in_bindings() {
    let action = crate::ActionId::try_v1("app", "save").expect("bounded action id");
    let node = button(label("Save")).try_on(crate::Trigger::Activate, action.clone()).unwrap_or_else(|_| panic!("bounded binding")).try_build().unwrap_or_else(|_| panic!("button build"));
    assert_eq!(node.bindings.len(), 1);
    let binding = node.bindings.get(0).expect("first binding");
    assert_eq!(binding.trigger, crate::Trigger::Activate);
    assert_eq!(binding.action, action);
    assert!(binding.args.is_none());
}

#[test]
fn on_with_carries_args() {
    let action = crate::ActionId::try_v1("app", "setValue").expect("bounded action id");
    let value = crate::UiText::try_from_str("hi").expect("bounded fixture text");
    let node = input(crate::InputKind::Text).try_on_with(crate::Trigger::Change, action, crate::UiValue::Text(value.clone())).unwrap_or_else(|_| panic!("bounded binding")).try_build().unwrap_or_else(|_| panic!("input build"));
    assert_eq!(node.bindings.get(0).expect("first binding").args, Some(crate::UiValue::Text(value)));
}
//#endregion 🔖️Bindings

//#region 🔖️Accessibility
#[test]
fn button_auto_derives_accessibility_label_from_visible_label() {
    let node = button(label("Save")).try_build().unwrap_or_else(|_| panic!("button build"));
    assert_eq!(node.accessibility.label, Some(label("Save")));
}

#[test]
fn explicit_label_overrides_auto_derived_accessibility_label() {
    let node = button(label("Save")).try_label("Save the document").unwrap_or_else(|_| panic!("bounded label")).try_build().unwrap_or_else(|_| panic!("button build"));
    assert_eq!(node.accessibility.label, Some(label("Save the document")));
}

/// 🚫️ `image(..)` without `.alt(..)`/`.decorative()` is a COMPILE error now (see the `compile_fail`
/// doctest on [`ImageBuilder`] itself), not a runtime panic — `ImageBuilder<NoAlt>` has no `build()`
/// at all, so there is nothing for a `#[test]` here to call. This comment stands in its place so the
/// next reader finds the negative case instead of assuming it was dropped.
#[test]
fn image_builder_no_alt_state_has_no_build_method_verified_by_the_type_doc_compile_fail_test() {
    let _: ImageBuilder<NoAlt> = image(ui_text("atlas://logo"));
}

#[test]
fn image_decorative_hides_from_accessibility_tree_and_omits_alt() {
    let node = image(ui_text("atlas://deco")).decorative().try_build().unwrap_or_else(|_| panic!("decorative image build"));
    assert!(node.accessibility.hidden);
    match node.component {
        crate::Component::Image(props) => assert!(props.alt.is_none()),
        other => panic!("expected image, got {other:?}"),
    }
}

#[test]
fn image_alt_populates_component_and_accessibility() {
    let node = image(ui_text("atlas://logo")).alt(label("Company logo")).try_build().unwrap_or_else(|_| panic!("image build"));
    assert_eq!(node.accessibility.label, Some(label("Company logo")));
    match node.component {
        crate::Component::Image(props) => assert_eq!(props.alt, Some(label("Company logo"))),
        other => panic!("expected image, got {other:?}"),
    }
}
//#endregion 🔖️Accessibility

//#region 🔖️Keys
#[test]
fn positional_keys_are_stable_and_distinct_among_siblings() {
    let build = || column().try_children([text(label("A")), text(label("B")), text(label("C"))]).unwrap_or_else(|_| panic!("bounded children")).try_build().unwrap_or_else(|_| panic!("column build"));
    let first = build();
    let second = build();
    let keys: Vec<&str> = first.children.iter().map(|child| child.key.as_str()).collect();
    assert_eq!(keys, vec!["#0", "#1", "#2"]);
    let second_keys: Vec<&str> = second.children.iter().map(|child| child.key.as_str()).collect();
    assert_eq!(keys, second_keys);
    let unique: std::collections::HashSet<&str> = keys.into_iter().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn explicit_id_overrides_positional_key() {
    let first = text(label("A")).try_id("first").unwrap_or_else(|_| panic!("bounded id"));
    let node = column().try_child(first).unwrap_or_else(|_| panic!("first child")).try_child(text(label("B"))).unwrap_or_else(|_| panic!("second child")).try_build().unwrap_or_else(|_| panic!("column build"));
    assert_eq!(node.children[0].key.as_str(), "first");
    assert_eq!(node.children[1].key.as_str(), "#1");
}
//#endregion 🔖️Keys
