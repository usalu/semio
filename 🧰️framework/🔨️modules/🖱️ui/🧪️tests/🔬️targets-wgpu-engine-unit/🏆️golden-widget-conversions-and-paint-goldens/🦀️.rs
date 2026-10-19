// 🏆️ Golden widget conversions and paint goldens: the widget node conversions, the statistics helpers and the stack, text, button, separator, input, select, toggle, key value, slider, stepper, ring and icon select paint goldens.

//#region 🔖️GoldenHarness
/// 🏆️ Acceptance gate for this workstream: for a curated fixture of every `UiNode` variant, runs
/// the retained façade (`apply_tree` + `frame`) and the immediate-mode path (`render_widget` over
/// a hand-converted `WidgetNode`) and asserts they emit structurally equivalent `DrawList`s
/// (same instance/vector/raster counts — not bit-identical geometry, per this ticket's brief).
/// `to_widget_node`/`control_to_widget`/`tree_*_to_widget` below mirror
/// `framework/renderer/wgpu/rs/lib.rs`'s private `ui_node_to_widget` conversion; they're
/// duplicated here (test-only) rather than shared because that crate depends on `ui_wgpu`, never
/// the reverse — keeping the two in sync is this harness's job.
fn to_widget_node(node: &UiNode) -> WidgetNode<ActionDescriptor> {
    match node {
        UiNode::Stack(stack) => WidgetNode::Stack { direction: stack.direction.clone(), gap: stack.gap.clone(), padding: stack.padding.clone(), children: stack.children.iter().map(to_widget_node).collect() },
        UiNode::Text(text) => WidgetNode::Text { value: text.value.to_string(), emphasize: text.emphasize.unwrap_or(false) },
        UiNode::Separator(_) => WidgetNode::Separator,
        UiNode::Button(button) => WidgetNode::Button { id: button.id.clone(), icon_id: Some(button.icon_id.clone()), label: button.label.to_string(), event: Some(button.action.clone()) },
        UiNode::Input(input) => WidgetNode::Input {
            id: input.id.clone(),
            input_kind: input.input_kind.clone(),
            value: input.value.clone(),
            placeholder: input.placeholder.clone().map(|l| l.to_string()),
            commit: input.commit.clone(),
            min: input.min,
            max: input.max,
            step: input.step,
            accept: input.accept.clone(),
            on_change: Some(input.on_change.clone()),
        },
        UiNode::Select(select) => WidgetNode::Select {
            id: select.id.clone(),
            value: select.value.clone(),
            items: select.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
            placeholder: select.placeholder.clone().map(|l| l.to_string()),
            on_change: Some(select.on_change.clone()),
        },
        UiNode::Toggle(toggle) => WidgetNode::Toggle { id: toggle.id.clone(), icon_id: toggle.icon_id.clone(), pressed: toggle.presence.selected, text: toggle.text.clone().map(|l| l.to_string()), on_change: Some(toggle.on_change.clone()) },
        UiNode::KeyValue(kv) => WidgetNode::KeyValue { entries: kv.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
        UiNode::Slider(slider) => WidgetNode::Slider { id: slider.id.clone(), value: slider.value, min: slider.min, max: slider.max, step: slider.step, ready: None, disabled: false, on_change: Some(slider.on_change.clone()) },
        UiNode::NumberStepper(stepper) => {
            WidgetNode::NumberStepper { id: stepper.id.clone(), value: stepper.value, step: stepper.step, uniform: stepper.uniform, on_absolute: Some(stepper.on_absolute.clone()), on_delta: Some(stepper.on_delta.clone()) }
        }
        UiNode::Ring(ring) => WidgetNode::Ring { id: ring.id.clone(), t: ring.t, disabled: ring.presence.state == UiState::Disabled, on_change: Some(ring.on_change.clone()) },
        UiNode::IconSelect(select) => WidgetNode::IconSelect { id: select.id.clone(), value: select.value.clone(), uniform: select.uniform, classifier_kind: select.classifier_kind.clone(), on_change: Some(select.on_change.clone()) },
        UiNode::Field(field) => match ui_node_to_control(&field.child) {
            Some(control) => WidgetNode::Field { id: field.id.clone(), label: field.label.to_string(), child: control_to_widget(&control) },
            None => WidgetNode::Section { id: field.id.clone(), label: Some(field.label.to_string()), default_open: true, children: vec![to_widget_node(&field.child)] },
        },
        UiNode::Section(section) => {
            WidgetNode::Section { id: section.id.clone(), label: section.label.clone().map(|l| l.to_string()), default_open: section.default_open.unwrap_or(true), children: section.children.iter().map(to_widget_node).collect() }
        }
        UiNode::Group(group) => WidgetNode::Group { id: group.id.clone(), label: group.label.to_string(), default_open: group.default_open.unwrap_or(true), children: group.children.iter().map(to_widget_node).collect() },
        UiNode::Tree(tree) => WidgetNode::Tree {
            // 🧭️ Per-item `selected`/`highlighted` (see `tree_item_to_widget`) already carry the
            // full signal from `item.presence` — the tree-level id lists are gone, not re-derived.
            sections: tree.sections.iter().map(tree_section_to_widget).collect(),
            selected_ids: Vec::new(),
            highlighted_ids: Vec::new(),
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeNode.selectionChange`
            // is deleted — selection now flows through `interactionSelect`/`interaction_domain`, not yet
            // wired into this retained-mode engine.
            selection_change: None,
        },
        // 🧩️ CLOSED by ticket 26/09/17 packet W15a (was: "`WidgetNode<E>` has no Image/
        // ComponentScene/ExternalSlot/Progress variant at all", so all four collapsed to an empty
        // placeholder `Text` and no two-pipeline comparison was possible). All five kinds now have a
        // real arm in the kit, so this mapping is like-for-like and
        // `every_ui_node_kind_has_a_widget_kit_arm_that_paints` asserts each one paints.
        UiNode::Progress(progress) => WidgetNode::Progress { id: progress.id.clone(), completed: progress.completed, total: progress.total },
        UiNode::Image(image) => WidgetNode::Image { id: image.id.clone(), src: image.src.clone(), alt: image.alt.clone().map(|alt| alt.to_string()) },
        UiNode::ComponentScene(scene) => WidgetNode::component_scene(scene),
        UiNode::ExternalSlot(slot) => WidgetNode::ExternalSlot { body_key: slot.body_key.clone() },
    }
}

fn control_to_widget(control: &UiControlNode) -> ControlNode<ActionDescriptor> {
    match control {
        UiControlNode::Button(n) => ControlNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.to_string(), event: Some(n.action.clone()) },
        UiControlNode::Input(n) => ControlNode::Input {
            id: n.id.clone(),
            input_kind: n.input_kind.clone(),
            value: n.value.clone(),
            placeholder: n.placeholder.clone().map(|l| l.to_string()),
            commit: n.commit.clone(),
            min: n.min,
            max: n.max,
            step: n.step,
            accept: n.accept.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Select(n) => ControlNode::Select {
            id: n.id.clone(),
            value: n.value.clone(),
            items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
            placeholder: n.placeholder.clone().map(|l| l.to_string()),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Toggle(n) => ControlNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone().map(|l| l.to_string()), on_change: Some(n.on_change.clone()) },
        UiControlNode::KeyValue(n) => ControlNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
        UiControlNode::Slider(n) => ControlNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
        UiControlNode::NumberStepper(n) => ControlNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
        UiControlNode::Ring(n) => ControlNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
        UiControlNode::IconSelect(n) => ControlNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
    }
}

/// 🎛️ Same per-variant field mapping as `control_to_widget`, but into a `WidgetNode` instead of a
/// `ControlNode` — needed for `TreeItem::control: Option<Box<WidgetNode<E>>>`, which (unlike
/// `Field`'s `child: ControlNode<E>`) embeds a full widget, not a bare control payload.
fn control_to_widget_node(control: &UiControlNode) -> WidgetNode<ActionDescriptor> {
    match control {
        UiControlNode::Button(n) => WidgetNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.to_string(), event: Some(n.action.clone()) },
        UiControlNode::Input(n) => WidgetNode::Input {
            id: n.id.clone(),
            input_kind: n.input_kind.clone(),
            value: n.value.clone(),
            placeholder: n.placeholder.clone().map(|l| l.to_string()),
            commit: n.commit.clone(),
            min: n.min,
            max: n.max,
            step: n.step,
            accept: n.accept.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Select(n) => WidgetNode::Select {
            id: n.id.clone(),
            value: n.value.clone(),
            items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.to_string() }).collect(),
            placeholder: n.placeholder.clone().map(|l| l.to_string()),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Toggle(n) => WidgetNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone().map(|l| l.to_string()), on_change: Some(n.on_change.clone()) },
        UiControlNode::KeyValue(n) => WidgetNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.to_string(), value: entry.value.clone() }).collect() },
        UiControlNode::Slider(n) => WidgetNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
        UiControlNode::NumberStepper(n) => WidgetNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
        UiControlNode::Ring(n) => WidgetNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
        UiControlNode::IconSelect(n) => WidgetNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
    }
}

fn tree_action_to_widget(action: &UiTreeItemAction) -> TreeItemAction<ActionDescriptor> {
    TreeItemAction { icon_id: action.icon_id.clone(), label: action.label.clone().map(|l| l.to_string()), event: action.action.clone(), placement: action.placement() }
}

fn tree_item_to_widget(item: &UiTreeItemNode) -> TreeItem<ActionDescriptor> {
    TreeItem {
        id: item.id.clone(),
        label: item.label.to_string(),
        description: item.description.clone(),
        icon_id: item.icon_id.clone(),
        selected: item.presence.selected,
        highlighted: item.presence.state == UiState::Previewed,
        default_open: item.default_open.unwrap_or(false),
        dimmed: item.dimmed.unwrap_or(false),
        event: item.action.clone(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeItemNode.hoverAction`/
        // `unhoverAction` are deleted — hover is now framework-owned per `UiTreeNode.interactionDomain`.
        hover_event: None,
        unhover_event: None,
        actions: item.actions.as_ref().map(|actions| actions.iter().map(tree_action_to_widget).collect()).unwrap_or_default(),
        draggable: item.draggable.unwrap_or(false),
        drag_data: item.drag_data.clone().unwrap_or_default(),
        control: item.control.as_ref().map(|control| Box::new(control_to_widget_node(control))),
        children: item.items.as_ref().map(|items| items.iter().map(tree_item_to_widget).collect()).unwrap_or_default(),
        window: item.window,
    }
}

fn tree_section_to_widget(section: &UiTreeSectionNode) -> TreeSection<ActionDescriptor> {
    TreeSection { id: section.id.clone(), label: section.label.clone().map(|l| l.to_string()), default_open: section.default_open.unwrap_or(true), items: section.items.iter().map(tree_item_to_widget).collect(), window: section.window }
}

/// 📊️ Total (ui_instances incl. overlay, vector_vertices incl. overlay, raster_instances) across
/// every layer of a `DrawList` — the "structurally equivalent" signal this harness compares,
/// deliberately coarser than exact geometry per this ticket's tolerance allowance.
fn stats(draw: &DrawList) -> (usize, usize, usize) {
    let instances = draw.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum();
    let vectors = draw.layers.iter().map(|layer| layer.vector_vertices.len() + layer.overlay_vector_vertices.len()).sum();
    let raster = draw.layers.iter().map(|layer| layer.raster_instances.len()).sum();
    (instances, vectors, raster)
}

fn retained_stats(node: &UiNode) -> (usize, usize, usize) {
    let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("golden", node);
    drive_layout(&mut ui, "golden", 400.0, 400.0, &mut atlas);
    let draw = ui.frame::<RecordingSceneHost>("golden", 400.0, 400.0, &mut atlas, None, None).expect("apply_tree then frame must produce a draw list");
    stats(draw)
}

fn immediate_stats(node: &UiNode, bounds: Rect) -> (usize, usize, usize) {
    let widget = to_widget_node(node);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut scroll_offsets: StdHashMap<String, f32> = StdHashMap::new();
    let mut collapsed_sections: StdHashMap<String, bool> = StdHashMap::new();
    let mut open_selects: StdHashMap<String, bool> = StdHashMap::new();
    let mut ctx = WidgetContext {
        draw: &mut draw,
        overlay: None,
        atlas: &mut atlas,
        icons: None,
        input: &mut input,
        theme: &theme,
        scroll_offsets: &mut scroll_offsets,
        collapsed_sections: &mut collapsed_sections,
        open_selects: &mut open_selects,
        interaction_maps: None,
        pick_clip: None,
        viewport_height: 0.0,
    };
    render_widget(&widget, bounds, &mut ctx);
    stats(&draw)
}

const VIEWPORT: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

/// 🧱️ Wraps a leaf `UiNode` as the sole child of a gap-less/padding-less vertical `Stack`: on the
/// retained side, `flex::LayoutEngine` always forces the *root* to the full viewport size
/// (`compute`'s `root_style.size` override) and gives a `Stack`'s only child `flex_grow: 1.0`, so
/// the child's resolved `LayoutBucket` is exactly the full viewport. On the immediate side,
/// `layout::layout_vertical`/`layout_horizontal`'s `extra_per_child` gives a lone child the same
/// full bounds. Wrapping every leaf fixture this way guarantees both pipelines paint it at
/// identical bounds, which is what makes an exact instance/vector-count comparison meaningful
/// instead of an artifact of divergent layout math.
fn leaf(child: UiNode) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("none".into()),
        padding: Some("none".into()),
        id: None,
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![child],
        menu: None,
    })
}

fn assert_equivalent(kind: &str, node: &UiNode) {
    let retained = retained_stats(node);
    let immediate = immediate_stats(node, VIEWPORT);
    assert_eq!(retained, immediate, "{kind}: retained (instances, vectors, raster) {retained:?} != immediate {immediate:?}");
}

// `action()` is shared with 🔖️FacadeTests above — both sub-regions live in the same `mod tests`.

#[test]
fn golden_stack() {
    let node = UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("none".into()),
        padding: Some("none".into()),
        id: None,
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![UiNode::Text(UiTextNode { value: Label::data("hello"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })],
        menu: None,
    });
    assert_equivalent("Stack", &node);
}

#[test]
fn golden_text() {
    assert_equivalent("Text", &leaf(UiNode::Text(UiTextNode { value: Label::data("hello world"), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_button() {
    assert_equivalent("Button", &leaf(UiNode::Button(UiButtonNode { id: Some("btn".into()), icon_id: IconName::CircleDot, label: Label::data("Go"), action: action(), style: None, presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_separator() {
    assert_equivalent("Separator", &leaf(UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_input() {
    assert_equivalent(
        "Input",
        &leaf(UiNode::Input(UiInputNode {
            id: "in".into(),
            input_kind: "text".into(),
            value: "abc".into(),
            placeholder: None,
            accessibility_label: None,
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            precision: None,
            snaps: Vec::new(),
            on_change: action(),
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            menu: None,
            ..Default::default()
        })),
    );
}

#[test]
fn golden_select() {
    assert_equivalent(
        "Select",
        &leaf(UiNode::Select(UiSelectNode {
            id: "sel".into(),
            value: "a".into(),
            items: vec![UiSelectItem { value: "a".into(), label: Label::data("Alpha") }, UiSelectItem { value: "b".into(), label: Label::data("Beta") }],
            placeholder: None,
            on_change: action(),
            presence: UiPresence::default(),
            menu: None,
        })),
    );
}

#[test]
fn checkbox_checked_state_does_not_paint_a_selection_outline() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/🧪️tests/🧫️fixtures/♿️retained-toggle-semantics/🔣️.json")).unwrap();
    let paint = &fixture["checkboxPaint"];
    let width = paint["availableWidth"].as_f64().unwrap() as f32;
    let height = paint["availableHeight"].as_f64().unwrap() as f32;
    let side = paint["controlSide"].as_f64().unwrap() as f32;
    for checked in paint["checkedStates"].as_array().unwrap().iter().map(|value| value.as_bool().unwrap()) {
        let mut ui = Ui::new(semio_framework_ui_locale::Locale::En);
        let mut atlas = FontAtlas::builtin();
        let toggle = UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Checkbox, id: "checkbox".into(), icon_id: IconName::CircleDot, text: None, on_change: action(), presence: UiPresence::selected(checked), menu: None });
        ui.apply_tree("checkbox-paint", &leaf(toggle));
        drive_layout(&mut ui, "checkbox-paint", width, height, &mut atlas);
        let draw = ui.frame::<RecordingSceneHost>("checkbox-paint", width, height, &mut atlas, None, None).expect("checkbox frame");
        let painted: Vec<_> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter().chain(layer.overlay_ui_instances.iter())).collect();
        assert!(!painted.is_empty());
        assert!(painted.iter().all(|instance| instance.rect[2] <= side + 0.01 && instance.rect[0] >= 0.0), "checked={checked}: a checkbox must not outline its {width}px allocation");
    }
}

#[test]
fn golden_toggle() {
    // 🚫️ `presence.selected` is intentionally NOT exercised here: the shared `presence_overlay`
    // now draws an outset accent ring for ANY selected element (see
    // `selected_presence_draws_an_outset_ring_on_any_element`, below) — a deliberate new
    // capability `widgets::render_toggle` (the immediate-mode reference this harness compares
    // against) never had, so a selected fixture would fail this equivalence check for the wrong
    // reason. This test stays scoped to the base (unselected) toggle's fill/label parity.
    assert_equivalent(
        "Toggle",
        &leaf(UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None })),
    );
}

/// ✨️ `presence.selected` draws its outset accent ring universally — proven here on `Toggle`, a
/// non-`Stack` variant, since `selected` used to be a `UiStackNode`-only field. Instance count
/// grows vs. the unselected fixture (the extra `push_chrome_border` edges), confirming the ring
/// is now a shared channel every element gets for free from `presence_overlay`.
#[test]
fn selected_presence_draws_an_outset_ring_on_any_element() {
    let unselected = UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None });
    let selected =
        UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::selected(true), menu: None });
    let (unselected_instances, _, _) = retained_stats(&leaf(unselected));
    let (selected_instances, _, _) = retained_stats(&leaf(selected));
    assert!(selected_instances > unselected_instances, "a selected element should paint more instances than an unselected one (the outset accent ring)");
}

#[test]
fn golden_key_value() {
    assert_equivalent("KeyValue", &leaf(UiNode::KeyValue(UiKeyValueNode { entries: vec![UiKeyValueEntry { label: Label::data("Name"), value: Label::data("Semio").to_string() }], presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_slider() {
    assert_equivalent("Slider", &leaf(UiNode::Slider(UiSliderNode { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: None, snaps: Vec::new(), on_change: action(), presence: UiPresence::default(), menu: None, ..Default::default() })));
}

/// 🔀️ Mixed values share the same placeholder and chrome in both WGPU painting paths.
#[test]
fn golden_mixed_number_stepper() {
    assert_equivalent(
        "MixedNumberStepper",
        &leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: false, min: None, max: None, precision: None, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None, ..Default::default() })),
    );
}

#[test]
fn golden_number_stepper() {
    assert_equivalent(
        "NumberStepper",
        &leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: true, min: None, max: None, precision: None, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None, ..Default::default() })),
    );
}

#[test]
fn golden_ring() {
    assert_equivalent("Ring", &leaf(UiNode::Ring(UiRingNode { id: "ring".into(), orb_id: "orb".into(), t: 0.25, on_change: action(), presence: UiPresence::default(), menu: None })));
}

#[test]
fn golden_icon_select() {
    assert_equivalent(
        "IconSelect",
        &leaf(UiNode::IconSelect(UiIconSelectNode { id: "ic".into(), value: IconName::Sparkles.to_string(), uniform: false, classifier_kind: "kind".into(), on_change: action(), presence: UiPresence::default(), menu: None })),
    );
}

#[test]
fn golden_tree() {
    let item = |id: &str, label: &str| UiTreeItemNode {
        tone: None,
        window: None,
        granularity: None,
        id: id.into(),
        label: Label::data(label),
        description: None,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        content_lines: None,
        inline_toolbar: None,
        detail: None,
        dimmed: None,
        menu: None,
    };
    let node = UiNode::Tree(UiTreeNode {
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item("i1", "Item One"), item("i2", "Item Two")] }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    });
    assert_equivalent("Tree", &node);
}
//#endregion 🔖️GoldenHarness
