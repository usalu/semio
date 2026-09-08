
use super::*;

#[test]
fn typed_wire_neutral_component_defaults_match_serde() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧵️retained/📦️wire/🧪️fixtures/🧾️typed/🔣️.json")).expect("typed fixture");
    let rows = fixture["components"].as_array().expect("component vectors");
    assert_eq!(rows.len(), 18);
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
    component_round_trips(Component::Toggle(ToggleProps { on: true, icon: ui_text("toggle-left"), text: Some(label("Enabled")) }));
    component_round_trips(Component::KeyValueList(KeyValueListProps { entries: crate::UiFixedList::default() }));
    component_round_trips(Component::Slider(SliderProps { value: 0.5, min: 0.0, max: 1.0, step: 0.1, unit: Some(ui_text("m")) }));
    component_round_trips(Component::NumberStepper(NumberStepperProps { value: 2.0, step: 1.0, uniform: false }));
    component_round_trips(Component::Ring(RingProps { orb_id: ui_text("orb-1"), t: 0.25 }));
    component_round_trips(Component::IconSelect(IconSelectProps { value: ui_text("circle"), uniform: true, classifier_kind: ui_text("shape") }));
    component_round_trips(Component::Tree(TreeProps { interaction_domain: Some(ui_text("selection")) }));
    component_round_trips(Component::TreeSection(TreeSectionProps { label: Some(label("Section")), default_open: Some(true) }));
    component_round_trips(Component::TreeItem(TreeItemProps {
        label: label("Item"),
        description: None,
        icon: Some(ui_text("file")),
        default_open: None,
        draggable: Some(true),
        drag_data: None,
        dimmed: Some(false),
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
