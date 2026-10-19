//! 🎛️ Widget nodes: the input commit action and the input, select, toggle, button, key value, slider, number stepper, ring, icon select and progress nodes. See [`super`].

use super::*;

// #region reconcile
//#region 🌳️DocumentTreeReconcile
/// ⌨️ Which trigger an `Input`'s value commits through — `InputProps`' own doc names the pair
/// (`Trigger::Change`/`Trigger::Commit`), and `commit == "blur"` is what picks between them, exactly
/// as React's `InputView` picks `commitOnBlur ? "commit" : "change"`. Resolving it HERE keeps the
/// retained node's single `on_change` field the one descriptor `events`' commit authority
/// dispatches, whichever trigger the guest actually bound.
fn input_commit_action(record: &UiNodeRecord, props: &ui_contract::InputProps, controller: &str) -> ActionDescriptor {
    let trigger = if props.commit.as_ref().is_some_and(|value| value.as_str() == "blur") { ui_contract::Trigger::Commit } else { ui_contract::Trigger::Change };
    record_action_or_inert(record, trigger, controller)
}

pub(super) fn input_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Input(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Input(UiInputNode {
        id: record.key.as_str().to_string(),
        input_kind: input_kind(props.kind),
        value: props.value.as_str().to_string(),
        placeholder: optional_contract_label(props.placeholder.as_ref()),
        accessibility_label: optional_contract_label(record.accessibility.label.as_ref()),
        commit: props.commit.as_ref().map(|value| value.as_str().to_string()),
        min: props.min,
        max: props.max,
        step: props.step.or_else(|| props.precision.filter(|_| props.kind == ui_contract::InputKind::Number).map(|precision| 10_f64.powi(-i32::from(precision.min(ui_contract::UI_NUMBER_PRECISION_MAX))) / props.display_factor.unwrap_or(1.0))),
        accept: props.accept.as_ref().map(|value| value.as_str().to_string()),
        precision: props.precision.filter(|_| props.kind == ui_contract::InputKind::Number),
        snaps: if props.kind == ui_contract::InputKind::Number { props.snaps.iter().copied().collect() } else { Vec::new() },
        display_factor: props.display_factor.filter(|_| props.kind == ui_contract::InputKind::Number),
        limits: props.limits.clone().filter(|_| props.kind == ui_contract::InputKind::Number),
        on_change: input_commit_action(record, props, controller),
        on_submit: record_action(record, ui_contract::Trigger::Submit, controller),
        on_abort: record_action(record, ui_contract::Trigger::Abort, controller),
        on_repeat_last: record_action(record, ui_contract::Trigger::RepeatLast, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

pub(super) fn select_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Select(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Select(UiSelectNode {
        id: record.key.as_str().to_string(),
        value: props.value.as_str().to_string(),
        items: props.items.iter().map(|item| UiSelectItem { value: item.value.as_str().to_string(), label: contract_label(&item.label) }).collect(),
        placeholder: optional_contract_label(props.placeholder.as_ref()),
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

pub(super) fn toggle_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Toggle(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    let mut presence = record_presence(record);
    presence.selected = props.on;
    UiNode::Toggle(UiToggleNode {
        appearance: props.appearance,
        id: record.key.as_str().to_string(),
        icon_id: icon_name(&props.icon),
        text: optional_contract_label(props.text.as_ref()),
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence,
        menu: menu_ref(record),
    })
}

pub(super) fn button_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Button(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Button(UiButtonNode {
        id: Some(record.key.as_str().to_string()),
        icon_id: icon_name(&props.icon),
        label: contract_label(&props.label),
        action: record_action_or_inert(record, ui_contract::Trigger::Activate, controller),
        style: None,
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

pub(super) fn key_value_node(record: &UiNodeRecord) -> UiNode {
    let ui_contract::Component::KeyValueList(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::KeyValue(UiKeyValueNode { entries: props.entries.iter().map(|entry| UiKeyValueEntry { label: contract_label(&entry.label), value: entry.value.as_str().to_string() }).collect(), presence: record_presence(record), menu: menu_ref(record) })
}

pub(super) fn slider_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Slider(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Slider(UiSliderNode {
        id: record.key.as_str().to_string(),
        value: props.value,
        min: props.min,
        max: props.max,
        step: props.step,
        unit: props.unit.as_ref().map(|value| value.as_str().to_string()),
        snaps: props.snaps.iter().copied().collect(),
        appearance: props.appearance,
        scale: props.scale,
        precision: props.precision,
        display_unit: props.display_unit.as_ref().map(|value| value.as_str().to_string()),
        display_factor: props.display_factor,
        limits: props.limits.clone(),
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

pub(super) fn number_stepper_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::NumberStepper(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::NumberStepper(UiNumberStepperNode {
        id: record.key.as_str().to_string(),
        value: props.value,
        step: props.step,
        uniform: props.uniform,
        min: props.min,
        max: props.max,
        precision: props.precision,
        snaps: props.snaps.iter().copied().collect(),
        unit: props.display_unit.as_ref().or(props.unit.as_ref()).map(|value| value.as_str().to_string()),
        display_factor: props.display_factor,
        limits: props.limits.clone(),
        on_absolute: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        on_delta: record_action_or_inert(record, ui_contract::Trigger::Delta, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

pub(super) fn ring_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::Ring(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::Ring(UiRingNode {
        id: record.key.as_str().to_string(),
        orb_id: props.orb_id.as_str().to_string(),
        t: props.t,
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

pub(super) fn icon_select_node(record: &UiNodeRecord, controller: &str) -> UiNode {
    let ui_contract::Component::IconSelect(props) = &record.component else { return UiNode::Separator(UiSeparatorNode { presence: record_presence(record), menu: menu_ref(record) }) };
    UiNode::IconSelect(UiIconSelectNode {
        id: record.key.as_str().to_string(),
        value: props.value.as_str().to_string(),
        uniform: props.uniform,
        classifier_kind: props.classifier_kind.as_str().to_string(),
        on_change: record_action_or_inert(record, ui_contract::Trigger::Change, controller),
        presence: record_presence(record),
        menu: menu_ref(record),
    })
}

/// 📶️ `Component::Progress` → the retained bar; the record key is its identity, exactly like every
/// other leaf control.
pub(super) fn progress_node(record: &UiNodeRecord, props: &ui_contract::ProgressProps, presence: UiPresence, menu: Option<UiMenuRef>) -> UiNode {
    UiNode::Progress(UiProgressNode { id: record.key.as_str().to_string(), completed: props.completed, total: props.total, value_text: contract_label(&props.value_text), presence, menu })
}
//#endregion 🌳️DocumentTreeReconcile
// #endregion reconcile
