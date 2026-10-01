use super::*;

#[test]
fn typed_wire_neutral_component_defaults_match_serde() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧵️retained/📦️wire/🧫️fixtures/🧾️typed/🔣️.json")).expect("typed fixture");
    let rows = fixture["components"].as_array().expect("component vectors");
    assert_eq!(rows.len(), 27);
    for row in rows {
        let sparse: Component = serde_json::from_value(row["wire"].clone()).expect("native sparse component");
        let normalized: Component = serde_json::from_value(row["expected"].clone()).expect("native normalized component");
        assert_eq!(sparse, normalized);
        assert_eq!(serde_json::to_value(sparse).expect("native sparse wire"), serde_json::to_value(normalized).expect("native normalized wire"));
    }
    let style: crate::StyleSpec = serde_json::from_value(fixture["defaults"]["style"].clone()).expect("normalized style");
    let accessibility: crate::AccessibilitySpec = serde_json::from_value(fixture["defaults"]["accessibility"].clone()).expect("normalized accessibility");
    assert_eq!(style, crate::StyleSpec::default());
    assert_eq!(accessibility, crate::AccessibilitySpec::default());
}

fn ui_text(value: &str) -> crate::UiText {
    crate::UiText::try_from_str(value).expect("bounded fixture text")
}

fn label(value: &str) -> Label {
    Label::try_from(value).expect("bounded fixture label")
}

#[allow(clippy::needless_pass_by_value)]
fn component_round_trips(component: Component) {
    let first = serde_json::to_string(&component).expect("serialize");
    let deserialized: Component = serde_json::from_str(&first).expect("deserialize");
    let second = serde_json::to_string(&deserialized).expect("re-serialize");
    assert_eq!(first, second);
    assert_eq!(component, deserialized);
}

#[test]
fn every_component_variant_round_trips() {
    component_round_trips(Component::Container(ContainerProps {
        role: ContainerRole::Section,
        label: Some(label("Section")),
        description: Some(ui_text("desc")),
        required: Some(true),
        error: None,
        default_open: Some(false),
        drop_overlay: Some(DropOverlaySpec { title: label("Drop"), hint: label("here"), accept: Some(ui_text("image/*")) }),
    }));
    component_round_trips(Component::Text(TextProps { value: label("hi"), emphasize: Some(true), data_attributes: None }));
    component_round_trips(Component::Button(ButtonProps { icon: ui_text("plus"), label: label("Add") }));
    component_round_trips(Component::Separator(SeparatorProps {}));
    component_round_trips(Component::Input(InputProps { kind: InputKind::Number, value: ui_text("3"), placeholder: None, commit: Some(ui_text("blur")), min: Some(0.0), max: Some(10.0), step: Some(1.0), accept: None, precision: Some(2), snaps: Default::default(), display_factor: None, limits: None }));
    component_round_trips(Component::Select(SelectProps { value: ui_text("a"), items: crate::UiFixedList::default(), placeholder: None }));
    component_round_trips(Component::Toggle(ToggleProps { appearance: ToggleAppearance::Button, on: true, icon: ui_text("toggle-left"), text: Some(label("Enabled")) }));
    component_round_trips(Component::Toggle(ToggleProps { appearance: ToggleAppearance::Checkbox, on: true, icon: ui_text("check"), text: Some(label("Enabled")) }));
    component_round_trips(Component::KeyValueList(KeyValueListProps { entries: crate::UiFixedList::default() }));
    component_round_trips(Component::Slider(SliderProps { value: 0.5, min: 0.0, max: 1.0, step: 0.1, unit: Some(ui_text("m")), snaps: snaps(&[0.25, 0.5, 0.75]), appearance: SliderAppearance::Track, scale: UiNumberScale::Linear, precision: None, display_unit: None, display_factor: None, limits: None }));
    component_round_trips(Component::Slider(SliderProps {
        value: 1.0,
        min: 0.1,
        max: 10.0,
        step: 0.01,
        unit: None,
        snaps: snaps(&[0.25, 0.5, 1.0, 2.0, 4.0]),
        appearance: SliderAppearance::Dial,
        scale: UiNumberScale::Log,
        precision: Some(2),
        display_unit: Some(ui_text("x")),
        display_factor: Some(2.0),
        limits: Some(UiNumberLimits { min: Some(UiNumberBound { value: 0.0, exclusive: true, refusal: Some(Label(ui_text("Must be greater than 0"))) }), max: None }),
    }));
    component_round_trips(Component::NumberStepper(NumberStepperProps { value: 2.0, step: 1.0, uniform: false, min: Some(0.0), max: Some(10.0), precision: Some(1), snaps: snaps(&[0.0, 5.0]), unit: Some(ui_text("rad")), display_unit: Some(ui_text("°")), display_factor: Some(57.29577951308232), limits: None }));
    component_round_trips(Component::Ring(RingProps { orb_id: ui_text("orb-1"), t: 0.25 }));
    component_round_trips(Component::IconSelect(IconSelectProps { value: ui_text("circle"), uniform: true, classifier_kind: ui_text("shape") }));
    component_round_trips(Component::Progress(ProgressProps { completed: 12.0, total: Some(100.0), value_text: label("12 of 100") }));
    component_round_trips(Component::Progress(ProgressProps { completed: 3.0, total: None, value_text: label("Preparing") }));
    component_round_trips(Component::Tree(TreeProps { presentation: Default::default(), interaction_domain: Some(ui_text("selection")) }));
    component_round_trips(Component::TreeSection(TreeSectionProps { label: Some(label("Section")), default_open: Some(true), header_toolbar: None, window: Some(TreeWindow { row_extent: Default::default(), total: 512, offset: 128 }) }));
    component_round_trips(Component::TreeItem(TreeItemProps {
        label: label("Item"),
        description: None,
        icon: Some(ui_text("file")),
        default_open: None,
        draggable: Some(true),
        drag_data: None,
        dimmed: Some(false),
        window: Some(TreeWindow { row_extent: Default::default(), total: 4096, offset: 0 }),
        granularity: Some(ui_text("piece")),
        inline_toolbar: Some(crate::UiNodeId(42)),
        detail: Some(crate::UiNodeId(43)),
        row_actions: crate::UiFixedList::default(),
        target: None,
    }));
    component_round_trips(Component::Image(ImageProps { src: ui_text("atlas://x"), alt: Some(label("alt")) }));
    component_round_trips(Component::Surface(Default::default()));
    component_round_trips(Component::Extension(ExtensionProps { extension: ui_text("plugin.app.slot"), props: Default::default() }));
}

#[test]
fn row_target_fixture_dispatches_tree_and_table_rows_identically_and_refuses_disagreeing_rows() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️row-target/🔣️.json")).expect("row-target fixture");
    for row in fixture["rows"].as_array().expect("fixture rows") {
        let node = serde_json::json!({
            "id": 1, "key": row["case"], "component": row["component"], "bindings": row["bindings"],
            "layout": { "kind": "stack", "axis": "horizontal", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false },
            "style": {}, "activity": "idle", "accessibility": {}
        });
        let snapshot: crate::UiSnapshot = serde_json::from_value(serde_json::json!({ "surface": "row-target", "revision": 1, "root": 1, "nodes": [node], "layoutEpoch": 0 })).expect("row snapshot");
        let verdict = crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default());
        let Some(_) = row["violation"].as_str() else {
            assert_eq!(verdict, Ok(()), "{}: a targeted row is admitted", row["case"]);
            let (target, row_actions) = match &snapshot.nodes[0].component {
                Component::TreeItem(props) => (props.target.as_ref(), &props.row_actions),
                Component::TableRow(props) => (props.target.as_ref(), &props.row_actions),
                _ => panic!("{}: a row component", row["case"]),
            };
            let target = target.expect("a targeted row");
            let dispatched: Vec<serde_json::Value> = target
                .activation
                .iter()
                .map(|verb| serde_json::json!({ "verb": verb.as_str(), "binding": serde_json::to_value(target.binding(verb).expect("credited row binding")).expect("binding wire") }))
                .chain(row_actions.iter().map(|action| match target.action_binding(action) {
                    Ok(binding) => serde_json::json!({ "verb": action.verb.as_str(), "binding": serde_json::to_value(binding).expect("binding wire") }),
                    Err(crate::RowActionRefusal::Disabled) => serde_json::json!({ "verb": action.verb.as_str(), "refusal": "disabled" }),
                    Err(refusal) => panic!("{}: an enabled row action binds, got {refusal:?}", row["case"]),
                }))
                .collect();
            let disabled = row_actions.iter().find(|action| action.disabled).expect("the fixture's disabled row action");
            let wire = serde_json::to_value(disabled).expect("row action wire");
            assert_eq!(wire["disabled"], serde_json::json!(true), "{}: a disabled action carries its flag", row["case"]);
            assert!(row_actions.iter().filter(|action| !action.disabled).all(|action| serde_json::to_value(action).expect("row action wire").get("disabled").is_none()), "{}: an enabled action carries no flag at all", row["case"]);
            assert_eq!(serde_json::Value::from(dispatched), fixture["dispatch"], "{}: the activation and every row action dispatch as the fixture's bindings", row["case"]);
            assert_eq!(snapshot.nodes[0].credited_clone().expect("credited row copy").component, snapshot.nodes[0].component, "{}: a credited copy keeps target, activation and verbs", row["case"]);
            continue;
        };
        let violations = verdict.expect_err("a row whose verbs and target disagree is refused");
        assert!(violations.iter().any(|violation| matches!(violation, crate::UiContractViolation::InvalidRowTarget { node: crate::UiNodeId(1) })), "{}: refused as InvalidRowTarget", row["case"]);
    }
}

#[test]
fn tree_detail_relation_validates_and_copies_a_direct_diff_view_surface() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🆚️diff-view-produced-surface/🔣️.json")).expect("DiffView fixture");
    let nodes = serde_json::json!([
        {
            "id": 1, "key": fixture["conflicts"][0]["id"],
            "component": { "type": "treeItem", "label": fixture["conflicts"][0]["message"], "detail": 2 },
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false },
            "style": {}, "activity": "idle", "accessibility": {}, "children": [2]
        },
        {
            "id": 2, "key": "conflict.preview", "component": { "type": "surface", "kind": "diff-view", "docSchema": "diff-view@1", "doc": { "bytes": [] } },
            "layout": { "kind": "leaf", "width": "fill", "height": "fill" }, "style": {}, "activity": "idle", "accessibility": {}
        }
    ]);
    let mut snapshot: crate::UiSnapshot = serde_json::from_value(serde_json::json!({ "surface": "diff-conflict", "revision": 1, "root": 1, "nodes": nodes, "layoutEpoch": 0 })).expect("detail snapshot");
    assert_eq!(crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()), Ok(()));
    let copied = snapshot.nodes[0].credited_clone().expect("bounded credited copy");
    let Component::TreeItem(copied) = copied.component else { panic!("copied root remains a TreeItem") };
    assert_eq!(copied.detail, Some(crate::UiNodeId(2)));
    assert_eq!(crate::accessibility_projection_node(&snapshot.nodes[0], 1).expanded, None, "detail content is not a disclosure subtree");
    snapshot.nodes[1].component = Component::Button(ButtonProps { icon: ui_text("file-diff"), label: label("Preview") });
    let violations = crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()).expect_err("a non-Surface detail is rejected");
    assert!(violations.iter().any(|violation| matches!(violation, crate::UiContractViolation::InvalidTreeDetail { node: crate::UiNodeId(1), detail: crate::UiNodeId(2) })));
}

#[test]
fn inline_tree_toolbar_relation_validates_and_copies_from_the_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🎛️inline-tree-controls/🔣️.json")).expect("inline controls fixture");
    let controls = fixture["controls"].as_array().expect("fixture controls");
    let nodes = serde_json::json!([
        {
            "id": 1, "key": fixture["rowId"],
            "component": { "type": "treeItem", "label": fixture["conflict"]["message"], "inlineToolbar": 2 },
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false }, "style": {}, "activity": "idle", "accessibility": {}, "children": [2]
        },
        {
            "id": 2, "key": "conflict.toolbar", "component": { "type": "container", "role": fixture["expected"]["containerRole"] },
            "layout": { "kind": "stack", "axis": fixture["expected"]["axis"], "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false }, "style": {}, "activity": "idle", "accessibility": {}, "children": [3, 4]
        },
        {
            "id": 3, "key": "conflict.accept", "component": { "type": fixture["expected"]["controlComponent"], "icon": controls[0]["icon"], "label": controls[0]["label"] },
            "layout": { "kind": "leaf", "width": "hug", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {}
        },
        {
            "id": 4, "key": "conflict.discard", "component": { "type": fixture["expected"]["controlComponent"], "icon": controls[1]["icon"], "label": controls[1]["label"] },
            "layout": { "kind": "leaf", "width": "hug", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {}
        }
    ]);
    let mut snapshot: crate::UiSnapshot = serde_json::from_value(serde_json::json!({ "surface": "inline-conflict", "revision": 1, "root": 1, "nodes": nodes, "layoutEpoch": 0 })).expect("fixture snapshot");
    assert_eq!(crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()), Ok(()));
    let copied = snapshot.nodes[0].credited_clone().expect("bounded credited copy");
    let Component::TreeItem(copied) = copied.component else { panic!("copied root remains a TreeItem") };
    assert_eq!(copied.inline_toolbar, Some(crate::UiNodeId(2)));
    let projected_row = crate::accessibility_projection_node(&snapshot.nodes[0], 1);
    assert_eq!(projected_row.expanded, None, "an inline toolbar is not a disclosure subtree");
    for (record, expected) in snapshot.nodes.iter().skip(2).zip(controls) {
        let projected = crate::accessibility_projection_node(record, 3);
        assert_eq!((projected.role.as_str(), projected.label.as_deref()), ("button", expected["label"].as_str()));
    }
    snapshot.nodes[1].layout = crate::LayoutSpec::Stack(crate::StackLayout { axis: crate::Axis::Vertical, ..Default::default() });
    let violations = crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()).expect_err("a vertical inline toolbar is rejected");
    assert!(violations.iter().any(|violation| matches!(violation, crate::UiContractViolation::InvalidTreeInlineToolbar { node: crate::UiNodeId(1), toolbar: crate::UiNodeId(2) })));
}

#[test]
fn tree_section_header_toolbar_is_a_direct_horizontal_button_toolbar() {
    let nodes = serde_json::json!([
        {
            "id": 1, "key": "command.form", "component": { "type": "treeSection", "label": "Set Theme", "headerToolbar": 2 },
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false }, "style": {}, "activity": "idle", "accessibility": {}, "children": [2]
        },
        {
            "id": 2, "key": "command.form.toolbar", "component": { "type": "container", "role": "toolbar" },
            "layout": { "kind": "stack", "axis": "horizontal", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": false, "wrap": false }, "style": {}, "activity": "idle", "accessibility": {}, "children": [3, 4]
        },
        {
            "id": 3, "key": "command.execute", "component": { "type": "button", "icon": "check", "label": "Execute" },
            "layout": { "kind": "leaf", "width": "hug", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {}
        },
        {
            "id": 4, "key": "command.reset", "component": { "type": "button", "icon": "undo", "label": "Reset" },
            "layout": { "kind": "leaf", "width": "hug", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {}
        }
    ]);
    let mut snapshot: crate::UiSnapshot = serde_json::from_value(serde_json::json!({ "surface": "command-form", "revision": 1, "root": 1, "nodes": nodes, "layoutEpoch": 0 })).expect("section toolbar snapshot");
    assert_eq!(crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()), Ok(()));
    snapshot.nodes[1].layout = crate::LayoutSpec::Stack(crate::StackLayout { axis: crate::Axis::Vertical, ..Default::default() });
    let violations = crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()).expect_err("vertical section toolbar rejected");
    assert!(violations.iter().any(|violation| matches!(violation, crate::UiContractViolation::InvalidTreeSectionHeaderToolbar { node: crate::UiNodeId(1), toolbar: crate::UiNodeId(2) })));
}

#[test]
fn label_conversions_and_display() {
    let from_str = Label::try_from("hello").expect("bounded fixture label");
    let from_string = Label::try_from(String::from("hello")).expect("bounded fixture label");
    assert_eq!(from_str, from_string);
    assert_eq!(from_str.to_string(), "hello");
}

//#region 📶️ProgressWireLaw

const PROGRESS_FIXTURE: &str = include_str!("../../🧫️fixtures/📶️progress.json");

/// 📶️ The shared progress fixture's exact wire spelling, answered by serde_json (the third-party
/// oracle) and by the first-party `ToValue`/`FromValue` codec, which must agree on every case.
#[test]
fn progress_wire_shape_matches_the_serde_oracle_and_the_value_codec() {
    use protocol::value::{FromValue, ToValue};
    let fixture: serde_json::Value = serde_json::from_str(PROGRESS_FIXTURE).expect("📶️ the progress fixture parses");
    let cases = fixture["cases"].as_array().expect("progress cases");
    for case in cases {
        let id = case["id"].as_str().expect("case id");
        let component: Component = serde_json::from_value(case["component"].clone()).unwrap_or_else(|error| panic!("{id}: fixture component deserializes: {error}"));
        let Component::Progress(props) = &component else { panic!("{id}: fixture component is a progress bar") };
        assert_eq!(serde_json::to_string(&component).expect("serialize"), case["wire"].as_str().expect("fixture wire"), "{id}: exact serde wire");
        assert_eq!(serde_json::from_str::<Component>(case["wire"].as_str().expect("fixture wire")).expect("wire decodes"), component, "{id}: wire decodes back");
        assert_eq!(ProgressProps::from_value(props.to_value()).expect("value codec decodes"), *props, "{id}: first-party value codec round trip");
        assert_eq!(props.is_determinate(), case["component"].get("total").is_some(), "{id}: determinate iff a total is present");
        assert_eq!(progress_fraction(props.completed, props.total), case["fraction"].as_f64(), "{id}: filled fraction");
    }
}

/// 🛡️ A progress bar whose numbers are not finite is a contract violation, not a renderer's guess.
#[test]
fn progress_with_a_non_finite_number_is_rejected_by_validation() {
    for props in [ProgressProps { completed: f64::NAN, total: Some(1.0), value_text: label("x") }, ProgressProps { completed: 0.0, total: Some(f64::INFINITY), value_text: label("x") }] {
        let mut snapshot = crate::UiSnapshot { surface: crate::SurfaceId::try_from("progress").expect("bounded surface"), revision: crate::UiRevision(0), root: crate::UiNodeId(0), nodes: Default::default(), layout_epoch: 0 };
        let record = crate::UiNodeRecord {
            id: crate::UiNodeId(0),
            key: ui_text("#progress"),
            component: Component::Progress(props),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: Default::default(),
            menu: None,
            children: Default::default(),
        };
        snapshot.nodes.try_push(record).expect("fixed record");
        let violations = crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()).expect_err("a non-finite progress number must not validate");
        assert!(violations.iter().any(|violation| matches!(violation, crate::UiContractViolation::NonFiniteNumber { .. })), "a non-finite progress number is a NonFiniteNumber violation");
    }
}

//#endregion 📶️ProgressWireLaw

fn snaps(values: &[f64]) -> crate::UiFixedList<f64> {
    let mut list = crate::UiFixedList::default();
    for value in values {
        list.try_push(*value).expect("bounded fixture snaps");
    }
    list
}

fn fixture_scale(value: &serde_json::Value) -> crate::UiNumberScale {
    match value.as_str() {
        Some("log") => crate::UiNumberScale::Log,
        Some("linear") | None => crate::UiNumberScale::Linear,
        Some(other) => panic!("unknown number scale {other}"),
    }
}

fn fixture_snaps(value: &serde_json::Value) -> Vec<f64> {
    value.as_array().expect("fixture snaps").iter().map(|snap| snap.as_f64().expect("numeric snap")).collect()
}

#[test]
fn number_controls_fixture_pins_the_detent_pointer_key_and_precision_laws() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️number-controls/🔣️.json")).expect("number-controls fixture");
    for row in fixture["validity"].as_array().expect("validity rows") {
        let verdict = crate::snaps_are_valid(fixture_snaps(&row["snaps"]), row["min"].as_f64().expect("min"), row["max"].as_f64().expect("max"));
        assert_eq!(verdict, row["valid"].as_bool().expect("valid"), "{}", row["case"]);
    }
    for row in fixture["pointer"].as_array().expect("pointer rows") {
        let value = crate::slider_pointer_value(row["value"].as_f64().expect("value"), row["min"].as_f64().expect("min"), row["max"].as_f64().expect("max"), row["step"].as_f64().expect("step"), fixture_snaps(&row["snaps"]), fixture_scale(&row["scale"]));
        assert_eq!(value, row["expected"].as_f64().expect("expected"), "{}", row["case"]);
    }
    for row in fixture["adjacent"].as_array().expect("adjacent rows") {
        let snap = crate::slider_adjacent_snap(row["current"].as_f64().expect("current"), fixture_snaps(&row["snaps"]), row["forward"].as_bool().expect("forward"));
        assert_eq!(snap, row["expected"].as_f64(), "{}", row["case"]);
    }
    for row in fixture["keys"].as_array().expect("key rows") {
        let key = match row["key"].as_str().expect("key") {
            "decrement" => crate::SliderKey::Decrement,
            "increment" => crate::SliderKey::Increment,
            "pageDown" => crate::SliderKey::PageDown,
            "pageUp" => crate::SliderKey::PageUp,
            "home" => crate::SliderKey::Home,
            "end" => crate::SliderKey::End,
            other => panic!("unknown slider key {other}"),
        };
        let (current, min, max, step, large) = (row["current"].as_f64().expect("current"), row["min"].as_f64(), row["max"].as_f64(), row["step"].as_f64().expect("step"), row["large"].as_bool().expect("large"));
        let value = crate::ui_number_key_value(current, min, max, step, fixture_snaps(&row["snaps"]), key, large);
        assert_eq!(value, row["expected"].as_f64().expect("expected"), "{}", row["case"]);
        if let (Some(min), Some(max)) = (min, max) {
            assert_eq!(crate::slider_key_value(current, min, max, step, fixture_snaps(&row["snaps"]), key, large), value, "{}: the slider law is the bounded number law", row["case"]);
        }
    }
    for row in fixture["fixed"].as_array().expect("fixed rows") {
        let (value, precision) = (row["value"].as_f64().expect("value"), row["precision"].as_u64().expect("precision") as u16);
        assert_eq!(crate::format_ui_number_fixed(value, precision), row["expected"].as_str().expect("expected"), "{}", row["case"]);
        assert_eq!(crate::round_ui_number(value, precision), row["rounded"].as_f64().expect("rounded"), "{}", row["case"]);
    }
    for row in fixture["valueTexts"].as_array().expect("value text rows") {
        let component: Component = serde_json::from_value(row["component"].clone()).expect("value text component");
        let value = crate::accessibility_value(&component);
        assert_eq!(value.text.as_deref(), row["valueText"].as_str(), "{}", row["case"]);
        for (field, spoken) in [("valueNow", value.now), ("valueMin", value.min), ("valueMax", value.max)] {
            if let Some(expected) = row.get(field) {
                assert_eq!(spoken, expected.as_f64(), "{}: {field}", row["case"]);
            }
        }
    }
    for row in fixture["axis"].as_array().expect("axis rows") {
        let (value, min, max, scale) = (row["value"].as_f64().expect("value"), row["min"].as_f64().expect("min"), row["max"].as_f64().expect("max"), fixture_scale(&row["scale"]));
        let position = crate::slider_axis_position(value, min, max, scale);
        assert!((position - row["position"].as_f64().expect("position")).abs() <= 1e-12, "{}: position {position}", row["case"]);
        if (min..=max).contains(&value) {
            let back = crate::slider_axis_value(position, min, max, scale);
            assert!((back - value).abs() <= 1e-12 * value.abs().max(1.0), "{}: value {back}", row["case"]);
        }
    }
    for row in fixture["dial"].as_array().expect("dial rows") {
        let angle = crate::dial_angle(row["position"].as_f64().expect("position"));
        assert!((angle - row["angle"].as_f64().expect("angle")).abs() <= 1e-12, "{}: angle {angle}", row["case"]);
        assert!((crate::dial_position(angle) - row["back"].as_f64().expect("back")).abs() <= 1e-12, "{}: back", row["case"]);
    }
    for row in fixture["display"].as_array().expect("display rows") {
        let precision = row["precision"].as_u64().map(|precision| precision as u16);
        assert_eq!(crate::ui_number_display_text(row["stored"].as_f64().expect("stored"), row["factor"].as_f64(), precision), row["text"].as_str().expect("text"), "{}", row["case"]);
    }
    for row in fixture["typed"].as_array().expect("typed rows") {
        let precision = row["precision"].as_u64().map(|precision| precision as u16);
        let stored = crate::ui_number_typed_value(row["typed"].as_f64().expect("typed"), row["factor"].as_f64(), precision, fixture_snaps(&row["candidates"]));
        assert_eq!(stored, row["expected"].as_f64().expect("expected"), "{}", row["case"]);
    }
    for row in fixture["limits"].as_array().expect("limit rows") {
        let limits: Option<crate::UiNumberLimits> = serde_json::from_value(row["limits"].clone()).expect("fixture limits");
        let (min, max) = (row["min"].as_f64(), row["max"].as_f64());
        let crossed = crate::ui_number_crossed_bound(row["value"].as_f64().expect("value"), min, max, limits.as_ref());
        let lower = limits.as_ref().map_or(min, |limits| limits.min.as_ref().map(|bound| bound.value));
        let side = crossed.map(|bound| if lower == Some(bound.value) && limits.as_ref().is_none_or(|limits| limits.min.as_ref() == Some(&bound)) { "min" } else { "max" });
        assert_eq!(side, row["crossed"].as_str(), "{}", row["case"]);
    }
    for row in fixture["documents"].as_array().expect("document rows") {
        let node = serde_json::json!({ "id": 1, "key": row["case"], "component": row["component"], "layout": { "kind": "leaf", "width": "hug", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {} });
        let snapshot: crate::UiSnapshot = serde_json::from_value(serde_json::json!({ "surface": "number-controls", "revision": 1, "root": 1, "nodes": [node], "layoutEpoch": 0 })).expect("number-control snapshot");
        let verdict = crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default());
        match row["violation"].as_str() {
            None => assert_eq!(verdict, Ok(()), "{}: admitted", row["case"]),
            Some(violation) => {
                let violations = verdict.expect_err("refused");
                assert!(violations.iter().any(|found| serde_json::to_value(found).expect("violation wire")["type"] == violation), "{}: refused as {violation}", row["case"]);
            }
        }
        component_round_trips(snapshot.nodes[0].component.credited_clone().expect("credited copy"));
    }
}

#[test]
fn color_input_fixture_pins_the_hex_format_and_parse_laws() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️color-input/🔣️.json")).expect("color-input fixture");
    for row in fixture["hex"].as_array().expect("hex rows") {
        assert_eq!(crate::ui_color_hex(&fixture_snaps(&row["rgba"]), row["alpha"].as_bool().expect("alpha")), row["expected"].as_str().expect("expected"), "{}", row["case"]);
    }
    for row in fixture["parse"].as_array().expect("parse rows") {
        let expected = row["expected"].as_array().map(|rgba| rgba.iter().map(|component| component.as_f64().expect("component")).collect::<Vec<_>>());
        assert_eq!(crate::parse_ui_color_hex(row["text"].as_str().expect("text")).map(Vec::from), expected, "{}", row["case"]);
    }
    assert_eq!(crate::ui_color_hex(&[f64::NAN, f64::INFINITY, 0.5, f64::NAN], true), "#00008000", "a non-finite component reads 0");
}

#[test]
fn a_non_finite_snap_is_a_non_finite_number_not_a_detent_violation() {
    let node = serde_json::json!({ "id": 1, "key": "nan", "component": { "type": "slider", "value": 1, "min": 0, "max": 2, "step": 0.5, "snaps": [1] }, "layout": { "kind": "leaf", "width": "hug", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {} });
    let mut snapshot: crate::UiSnapshot = serde_json::from_value(serde_json::json!({ "surface": "nan", "revision": 1, "root": 1, "nodes": [node], "layoutEpoch": 0 })).expect("slider snapshot");
    let Some(Component::Slider(props)) = snapshot.nodes.get_mut(0).map(|record| &mut record.component) else { panic!("a slider record") };
    props.snaps = snaps(&[f64::NAN]);
    let violations = crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()).expect_err("a NaN snap is refused");
    assert!(violations.iter().any(|violation| matches!(violation, crate::UiContractViolation::NonFiniteNumber { .. })));
    assert!(!violations.iter().any(|violation| matches!(violation, crate::UiContractViolation::InvalidSnaps { .. })));
}
