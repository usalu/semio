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

//#region 🔖️NumberControls
fn action(name: &str) -> crate::ActionId {
    crate::ActionId::new(ui_text("history"), ui_text(name), 1)
}

#[test]
fn slider_snaps_accept_the_detent_law_and_refuse_breaking_it() {
    let slider = slider(5.0).min(0.0).max(10.0).step(0.5);
    let slider = slider.try_snap(2.5).unwrap_or_else(|_| panic!("first snap")).try_snap(5.0).unwrap_or_else(|_| panic!("second snap"));
    let slider = refused(slider.try_snap(5.0), "a repeated snap is refused");
    let slider = refused(slider.try_snap(11.0), "a snap past max is refused");
    let slider = refused(slider.try_snap(f64::NAN), "a non-finite snap is refused");
    let node = slider.try_snap(7.5).unwrap_or_else(|_| panic!("third snap")).try_build().unwrap_or_else(|_| panic!("slider build"));
    let crate::Component::Slider(props) = &node.component else { panic!("a slider") };
    assert_eq!(props.snaps.iter().copied().collect::<Vec<_>>(), vec![2.5, 5.0, 7.5]);
    assert!(props.snaps_are_valid());
    assert_eq!(serde_json::to_value(&node).expect("wire")["component"]["snaps"], serde_json::json!([2.5, 5.0, 7.5]));
    let bare = slider_without_snaps_wire();
    assert!(bare["component"].get("snaps").is_none(), "an undetented slider costs no snaps field");
}

fn refused<T>(result: Result<T, T>, reason: &str) -> T {
    match result {
        Ok(_) => panic!("{reason}"),
        Err(builder) => builder,
    }
}

fn slider_without_snaps_wire() -> serde_json::Value {
    serde_json::to_value(slider(0.5).try_build().unwrap_or_else(|_| panic!("slider build"))).expect("wire")
}

#[test]
fn number_stepper_builder_carries_bounds_precision_and_mixed_state() {
    let node = number_stepper(2.5).step(0.25).min(0.0).max(10.0).precision(2).mixed().try_build().unwrap_or_else(|_| panic!("stepper build"));
    assert_eq!(node.component, crate::Component::NumberStepper(crate::NumberStepperProps { value: 2.5, step: 0.25, uniform: false, min: Some(0.0), max: Some(10.0), precision: Some(2), snaps: Default::default(), unit: None, display_unit: None, display_factor: None, limits: None }));
    let capped = number_stepper(1.0).precision(40).try_build().unwrap_or_else(|_| panic!("stepper build"));
    let crate::Component::NumberStepper(props) = &capped.component else { panic!("a stepper") };
    assert_eq!(props.precision, Some(crate::UI_NUMBER_PRECISION_MAX));
}

#[test]
fn number_input_prints_its_value_at_its_precision() {
    let node = input(crate::InputKind::Number).precision(2).number(2.5).try_build().unwrap_or_else(|_| panic!("input build"));
    let crate::Component::Input(props) = &node.component else { panic!("an input") };
    assert_eq!(props.value.as_str(), "2.50");
    assert_eq!(props.precision, Some(2));
    assert_eq!(crate::accessibility_value(&node.component).text.as_deref(), Some("2.50"));
}

#[test]
fn vector_input_is_a_labelled_group_of_keyed_number_fields_sharing_unit_step_and_precision() {
    let bind = |name: &'static str| move |field: InputBuilder| field.try_on(crate::Trigger::Commit, action(name)).map_err(|(field, _)| field);
    let node = vector_input(label("Offset"))
        .unit(ui_text("mm"))
        .step(0.5)
        .precision(1)
        .commit(ui_text("blur"))
        .try_axis("dx", label("X"), 1.25, bind("setDx"))
        .unwrap_or_else(|_| panic!("x axis"))
        .try_axis("dy", label("Y"), -3.0, bind("setDy"))
        .unwrap_or_else(|_| panic!("y axis"))
        .try_build()
        .unwrap_or_else(|_| panic!("vector build"));
    let crate::Component::Container(group) = &node.component else { panic!("a container") };
    assert_eq!((group.role, group.label.clone()), (crate::ContainerRole::Group, Some(label("Offset"))));
    assert!(matches!(node.layout, crate::LayoutSpec::Stack(crate::StackLayout { axis: crate::Axis::Horizontal, wrap: true, .. })));
    let axes: Vec<(String, String, String, Option<f64>, Option<u16>, String)> = node
        .children
        .iter()
        .map(|axis| {
            let crate::Component::Container(field) = &axis.component else { panic!("a field") };
            assert_eq!(field.role, crate::ContainerRole::Field);
            assert_eq!(field.description.as_ref().map(|unit| unit.as_str()), Some("mm"));
            let number = &axis.children[0];
            let crate::Component::Input(props) = &number.component else { panic!("a number field") };
            assert_eq!(props.kind, crate::InputKind::Number);
            assert_eq!(props.commit.as_ref().map(|commit| commit.as_str()), Some("blur"));
            (axis.key.as_str().to_string(), number.key.as_str().to_string(), props.value.as_str().to_string(), props.step, props.precision, number.bindings.iter().next().expect("bound axis").action.to_string())
        })
        .collect();
    assert_eq!(
        axes,
        vec![
            ("dx.axis".to_string(), "dx".to_string(), "1.3".to_string(), Some(0.5), Some(1), "history.setDx@1".to_string()),
            ("dy.axis".to_string(), "dy".to_string(), "-3.0".to_string(), Some(0.5), Some(1), "history.setDy@1".to_string()),
        ]
    );
    assert_eq!(node.children[0].children[0].accessibility.label, Some(label("X")));
}

#[test]
fn reference_list_shows_chips_that_remove_a_use_selection_button_and_an_optional_candidate_list() {
    let labels = || ReferenceListLabels { use_selection: label("Use selection"), empty: label("Nothing selected") };
    let empty = reference_list(label("Targets"), labels()).try_use_selection(action("useSelection"), None, false).unwrap_or_else(|_| panic!("use selection")).try_build().unwrap_or_else(|_| panic!("empty build"));
    let keys: Vec<&str> = empty.children.iter().map(|child| child.key.as_str()).collect();
    assert_eq!(keys, vec!["empty", "actions"]);
    assert!(matches!(&empty.children[0].component, crate::Component::Text(props) if props.value == label("Nothing selected")));
    let use_selection = &empty.children[1].children[0];
    assert_eq!((use_selection.key.as_str(), use_selection.disabled), ("useSelection", true));
    assert_eq!(use_selection.bindings.iter().next().expect("bound").action.to_string(), "history.useSelection@1");

    let full = reference_list(label("Targets"), labels())
        .try_chip("piece-3", label("Piece 3"), "Remove Piece 3", action("removeTarget"), None)
        .unwrap_or_else(|_| panic!("chip"))
        .try_chip("piece-7", label("Piece 7"), "Remove Piece 7", action("removeTarget"), None)
        .unwrap_or_else(|_| panic!("chip"))
        .try_use_selection(action("useSelection"), None, true)
        .unwrap_or_else(|_| panic!("use selection"))
        .try_candidates(label("Add target"), action("addTarget"), [(ui_text("piece-9"), label("Piece 9"))])
        .unwrap_or_else(|_| panic!("candidates"))
        .try_build()
        .unwrap_or_else(|_| panic!("full build"));
    let crate::Component::Container(group) = &full.component else { panic!("a group") };
    assert_eq!(group.role, crate::ContainerRole::Group);
    let chips = &full.children[0];
    assert_eq!(chips.key.as_str(), "chips");
    assert!(matches!(&chips.component, crate::Component::Container(props) if props.role == crate::ContainerRole::Toolbar));
    let chip_names: Vec<(&str, Option<crate::Label>)> = chips.children.iter().map(|chip| (chip.key.as_str(), chip.accessibility.label.clone())).collect();
    assert_eq!(chip_names, vec![("piece-3", Some(label("Remove Piece 3"))), ("piece-7", Some(label("Remove Piece 7")))]);
    assert!(chips.children.iter().all(|chip| matches!(&chip.component, crate::Component::Button(props) if props.icon.as_str() == "x")));
    let actions = &full.children[1];
    let action_keys: Vec<&str> = actions.children.iter().map(|child| child.key.as_str()).collect();
    assert_eq!(action_keys, vec!["useSelection", "candidates"]);
    assert!(!actions.children[0].disabled);
    assert!(matches!(&actions.children[1].component, crate::Component::Select(props) if props.items.len() == 1));
}
/// 🌳️ The pre-order `(id, key, type, children)` shape a built tree flattens to — the corpus numbering.
fn flat_shape(node: &BuiltNode) -> Vec<serde_json::Value> {
    fn walk(node: &BuiltNode, next: &mut u64, out: &mut Vec<serde_json::Value>) -> u64 {
        let id = *next;
        *next += 1;
        let at = out.len();
        out.push(serde_json::Value::Null);
        let children: Vec<u64> = node.children.iter().map(|child| walk(child, next, out)).collect();
        let tag = serde_json::to_value(&node.component).expect("component wire")["type"].clone();
        out[at] = serde_json::json!({ "id": id, "key": node.key.as_str(), "type": tag, "children": children });
        id
    }
    let mut out = Vec::new();
    walk(node, &mut 0, &mut out);
    out
}

#[test]
fn the_recipes_build_exactly_the_conformance_corpus_shapes() {
    let vector = vector_input(label("Offset"))
        .unit(ui_text("mm"))
        .step(0.5)
        .precision(1)
        .commit(ui_text("blur"))
        .try_snap(0.0)
        .unwrap_or_else(|_| panic!("vector snap"))
        .try_axis("dx", label("X"), 1.25, |field| field.try_on(crate::Trigger::Commit, crate::ActionId::new(ui_text("app"), ui_text("setDx"), 1)).map_err(|(field, _)| field))
        .unwrap_or_else(|_| panic!("x axis"))
        .try_axis("dy", label("Y"), -3.0, |field| field.try_on(crate::Trigger::Commit, crate::ActionId::new(ui_text("app"), ui_text("setDy"), 1)).map_err(|(field, _)| field))
        .unwrap_or_else(|_| panic!("y axis"))
        .try_build()
        .unwrap_or_else(|_| panic!("vector build"));
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧪️conformance/🖥️composite/🧭️vector-input/🎯️expect.json")).expect("vector expectation");
    assert_eq!(serde_json::Value::from(flat_shape(&vector)), expected["tree"]["shape"]);
    let snapshot: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧪️conformance/🖥️composite/🧭️vector-input/📸️snapshot.json")).expect("vector snapshot");
    assert_eq!(flat_components(&vector), components_of(&snapshot), "each axis carries the shared step, precision and detents");
    let app = |name: &str| crate::ActionId::new(ui_text("app"), ui_text(name), 1);
    let references = reference_list(label("Targets"), ReferenceListLabels { use_selection: label("Use selection"), empty: label("Nothing selected") })
        .try_chip("piece-3", label("Piece 3"), "Remove Piece 3", app("removeTarget"), None)
        .unwrap_or_else(|_| panic!("chip"))
        .try_chip("piece-7", label("Piece 7"), "Remove Piece 7", app("removeTarget"), None)
        .unwrap_or_else(|_| panic!("chip"))
        .try_use_selection(app("useSelection"), None, true)
        .unwrap_or_else(|_| panic!("use selection"))
        .try_candidates(label("Add target"), app("addTarget"), [(ui_text("piece-9"), label("Piece 9"))])
        .unwrap_or_else(|_| panic!("candidates"))
        .try_build()
        .unwrap_or_else(|_| panic!("references build"));
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧪️conformance/🖥️composite/🧷️reference-list/🎯️expect.json")).expect("reference expectation");
    assert_eq!(serde_json::Value::from(flat_shape(&references)), expected["tree"]["shape"]);
    let color = color_input(label("Tint"), ColorInputLabels { hex: label("Hex"), alpha: label("Alpha") })
        .try_color(&[1.0, 0.5, 0.0, 0.5], true, |field| field.try_on(crate::Trigger::Change, app("setTint")).map_err(|(field, _)| field), |alpha| alpha.try_on(crate::Trigger::Change, app("setTintAlpha")).map_err(|(alpha, _)| alpha))
        .unwrap_or_else(|_| panic!("colour"))
        .try_build()
        .unwrap_or_else(|_| panic!("colour build"));
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧪️conformance/🖥️composite/🎨️color-input/🎯️expect.json")).expect("colour expectation");
    assert_eq!(serde_json::Value::from(flat_shape(&color)), expected["tree"]["shape"]);
    let snapshot: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧪️conformance/🖥️composite/🎨️color-input/📸️snapshot.json")).expect("colour snapshot");
    assert_eq!(flat_components(&color), components_of(&snapshot), "the swatch, hex field and alpha slider read the colour");
    let opaque = color_input(label("Tint"), ColorInputLabels { hex: label("Hex"), alpha: label("Alpha") }).try_color(&[0.2, 0.4, 0.6], false, Ok, Ok).unwrap_or_else(|_| panic!("opaque colour")).try_build().unwrap_or_else(|_| panic!("opaque build"));
    assert_eq!(opaque.children.len(), 2, "a colour without alpha has no alpha slider");
    assert_eq!(flat_components(&opaque)[2]["value"], "#336699");
}

/// 🧱️ Every node's component in pre-order — the order `flat_shape` numbers them.
fn flat_components(node: &BuiltNode) -> Vec<serde_json::Value> {
    std::iter::once(serde_json::to_value(&node.component).expect("component wire")).chain(node.children.iter().flat_map(flat_components)).collect()
}

/// 📸️ A corpus snapshot's components in node order.
fn components_of(snapshot: &serde_json::Value) -> Vec<serde_json::Value> {
    snapshot["nodes"].as_array().expect("nodes").iter().map(|node| node["component"].clone()).collect()
}
//#endregion 🔖️NumberControls
