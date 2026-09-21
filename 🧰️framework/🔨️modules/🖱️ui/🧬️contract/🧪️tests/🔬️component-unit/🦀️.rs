use super::*;

#[test]
fn typed_wire_neutral_component_defaults_match_serde() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧵️retained/📦️wire/🧫️fixtures/🧾️typed/🔣️.json")).expect("typed fixture");
    let rows = fixture["components"].as_array().expect("component vectors");
    assert_eq!(rows.len(), 19);
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
    component_round_trips(Component::Input(InputProps { kind: InputKind::Number, value: ui_text("3"), placeholder: None, commit: Some(ui_text("blur")), min: Some(0.0), max: Some(10.0), step: Some(1.0), accept: None }));
    component_round_trips(Component::Select(SelectProps { value: ui_text("a"), items: crate::UiFixedList::default(), placeholder: None }));
    component_round_trips(Component::Toggle(ToggleProps { appearance: ToggleAppearance::Button, on: true, icon: ui_text("toggle-left"), text: Some(label("Enabled")) }));
    component_round_trips(Component::Toggle(ToggleProps { appearance: ToggleAppearance::Checkbox, on: true, icon: ui_text("check"), text: Some(label("Enabled")) }));
    component_round_trips(Component::KeyValueList(KeyValueListProps { entries: crate::UiFixedList::default() }));
    component_round_trips(Component::Slider(SliderProps { value: 0.5, min: 0.0, max: 1.0, step: 0.1, unit: Some(ui_text("m")) }));
    component_round_trips(Component::NumberStepper(NumberStepperProps { value: 2.0, step: 1.0, uniform: false }));
    component_round_trips(Component::Ring(RingProps { orb_id: ui_text("orb-1"), t: 0.25 }));
    component_round_trips(Component::IconSelect(IconSelectProps { value: ui_text("circle"), uniform: true, classifier_kind: ui_text("shape") }));
    component_round_trips(Component::Progress(ProgressProps { completed: 12.0, total: Some(100.0), value_text: label("12 of 100") }));
    component_round_trips(Component::Progress(ProgressProps { completed: 3.0, total: None, value_text: label("Preparing") }));
    component_round_trips(Component::Tree(TreeProps { presentation: Default::default(), interaction_domain: Some(ui_text("selection")) }));
    component_round_trips(Component::TreeSection(TreeSectionProps { label: Some(label("Section")), default_open: Some(true), window: Some(TreeWindow { total: 512, offset: 128 }) }));
    component_round_trips(Component::TreeItem(TreeItemProps {
        label: label("Item"),
        description: None,
        icon: Some(ui_text("file")),
        default_open: None,
        draggable: Some(true),
        drag_data: None,
        dimmed: Some(false),
        window: Some(TreeWindow { total: 4096, offset: 0 }),
        granularity: Some(ui_text("piece")),
        row_actions: crate::UiFixedList::default(),
    }));
    component_round_trips(Component::Image(ImageProps { src: ui_text("atlas://x"), alt: Some(label("alt")) }));
    component_round_trips(Component::Surface(Default::default()));
    component_round_trips(Component::Extension(ExtensionProps { extension: ui_text("plugin.app.slot"), props: Default::default() }));
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
