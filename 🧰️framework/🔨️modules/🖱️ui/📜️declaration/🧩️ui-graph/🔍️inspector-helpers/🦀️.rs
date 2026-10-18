//! 🔍️ Inspector helpers: the inspector field groups, the mixed value helpers, the read-only, stepper, toggle and vec3 fields and the declarative section to tree conversion. See [`super`].

use super::*;

// #region ui
//#region 🔖Primitives
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiInspectorFieldGroup {
    pub id: String,
    pub label: Label,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    pub fields: Vec<UiNode>,
}

pub const UI_INSPECTOR_MIXED_PLACEHOLDER: &str = "Mixed";
//#endregion 🔖️Primitives

//#region 🔖️InspectorHelpers
pub fn ui_inspector_all_equal<T: PartialEq>(values: &[T]) -> bool {
    if values.len() <= 1 {
        return true;
    }
    values.windows(2).all(|pair| pair[0] == pair[1])
}

pub struct UiInspectorMixedText {
    pub value: String,
    pub placeholder: Option<String>,
}

pub fn ui_inspector_mixed_text(values: &[String]) -> UiInspectorMixedText {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedText { value: if uniform { values.first().cloned().unwrap_or_default() } else { String::new() }, placeholder: if uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) } }
}

pub struct UiInspectorMixedNumber {
    pub value: f64,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_number(values: &[f64]) -> UiInspectorMixedNumber {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedNumber { value: if uniform { *values.first().unwrap_or(&0.0) } else { f64::NAN }, uniform }
}

pub fn ui_inspector_mixed_select(values: &[String]) -> UiInspectorMixedText {
    ui_inspector_mixed_text(values)
}

pub struct UiInspectorMixedToggle {
    pub pressed: bool,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_toggle(values: &[bool]) -> UiInspectorMixedToggle {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedToggle { pressed: uniform && values.first().copied().unwrap_or(false), uniform }
}

pub fn ui_inspector_mixed_slider(values: &[f64]) -> UiInspectorMixedNumber {
    ui_inspector_mixed_number(values)
}

pub fn ui_inspector_readonly_field(id: impl Into<String>, label: impl Into<Label>, value: impl Into<String>) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        menu: None,
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            menu: None,
            id,
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            accessibility_label: None,
            commit: None,
            on_change: ActionDescriptor { controller_id: String::new(), action: String::new(), args: None },
            min: None,
            max: None,
            step: None,
            accept: None,
            precision: None,
            snaps: Vec::new(),
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            ..Default::default()
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
    })
}

/** 🔢️ Builds an editable number-stepper field row, computing the mixed/uniform display from
     * `values` via {@link ui_inspector_mixed_number}. `action` is cloned into both `onAbsolute` (typed
     * entry, dispatched with `{value}` merged into `args`) and `onDelta` (nudge buttons, `{delta}`) —
     * callers' patch handlers branch on whichever key the dispatched action actually carries. */
pub fn ui_inspector_stepper_field(id: impl Into<String>, label: impl Into<Label>, values: &[f64], step: f64, action: ActionDescriptor) -> UiNode {
    let id = id.into();
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        menu: None,
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode { menu: None, id, value: mixed.value, step, uniform: mixed.uniform, min: None, max: None, precision: None, on_absolute: action.clone(), on_delta: action, presence: UiPresence::default(), ..Default::default() })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
    })
}

/** 🔘️ Builds an editable boolean toggle field row, computing the mixed/uniform display from
     * `values` via {@link ui_inspector_mixed_toggle}. */
pub fn ui_inspector_toggle_field(id: impl Into<String>, label: impl Into<Label>, icon_id: impl Into<IconName>, values: &[bool], action: ActionDescriptor) -> UiNode {
    let id = id.into();
    let mixed = ui_inspector_mixed_toggle(values);
    UiNode::Field(UiFieldNode {
        menu: None,
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, menu: None, id, icon_id: icon_id.into(), text: None, on_change: action, presence: UiPresence::selected(mixed.pressed) })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
    })
}

/** 📐️ Builds a nested `Origin`-style group: a parent tree item labeled `label` containing
     * three {@link ui_inspector_stepper_field} children (`X`/`Y`/`Z`), each computing its own per-axis
     * mixed state independently — a multi-selection that agrees on X but not Y shows only Y as "Mixed".
     * `axis_action(axis)` builds the per-axis `ActionDescriptor`; callers typically merge
     * `{"field": "<id>.x"}` etc. into its `args` so the patch handler can dot-path into the right
     * component with `value` (absolute) or `delta` (relative, offset-preserving across multi-select). */
pub fn ui_inspector_vec3_group(id: impl Into<String>, label: impl Into<Label>, values: &[[f64; 3]], step: f64, axis_action: impl Fn(&str) -> ActionDescriptor) -> UiNode {
    let id = id.into();
    let xs: Vec<f64> = values.iter().map(|v| v[0]).collect();
    let ys: Vec<f64> = values.iter().map(|v| v[1]).collect();
    let zs: Vec<f64> = values.iter().map(|v| v[2]).collect();
    UiNode::Group(UiGroupNode {
        menu: None,
        id: id.clone(),
        label: label.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![
            // 🔤️ Axis symbols (X/Y/Z) are mathematical notation, not translatable UI chrome.
            ui_inspector_stepper_field(format!("{id}.x"), Label::data("X"), &xs, step, axis_action("x")),
            ui_inspector_stepper_field(format!("{id}.y"), Label::data("Y"), &ys, step, axis_action("y")),
            ui_inspector_stepper_field(format!("{id}.z"), Label::data("Z"), &zs, step, axis_action("z")),
        ],
    })
}

pub fn ui_inspector_groups_to_tree(groups: &[UiInspectorFieldGroup]) -> UiNode {
    let sections: Vec<UiSectionNode> = groups
        .iter()
        .filter(|group| !group.fields.is_empty())
        .map(|group| UiSectionNode { menu: None, id: group.id.clone(), label: Some(group.label.clone()), default_open: Some(group.default_open.unwrap_or(true)), presence: UiPresence::default(), children: group.fields.clone() })
        .collect();
    ui_declarative_sections_to_tree(&sections)
}

pub fn ui_declarative_sections_to_tree(sections: &[UiSectionNode]) -> UiNode {
    let mut tree_sections: Vec<UiTreeSectionNode> = Vec::with_capacity(sections.len());
    for section in sections {
        let mut items = Vec::with_capacity(section.children.len());
        for (index, child) in section.children.iter().enumerate() {
            items.push(ui_declarative_child_to_tree_item(child, format!("{}.{}", section.id, index)));
        }
        tree_sections.push(UiTreeSectionNode { header_toolbar: None, window: None, id: section.id.clone(), label: section.label.clone(), default_open: Some(section.default_open.unwrap_or(true)), presence: section.presence.clone(), items });
    }
    UiNode::Tree(if tree_sections.is_empty() {
        UiTreeNode {
            presentation: Default::default(),
            menu: None,
            sections: vec![UiTreeSectionNode {
                header_toolbar: None,
                window: None,
                id: "empty".into(),
                label: None,
                default_open: None,
                presence: UiPresence::default(),
                items: vec![UiTreeItemNode {
                    tone: None,
                    window: None,
                    granularity: None,
                    id: "empty".into(),
                    label: Label::data("—"),
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
                }],
            }],
            presence: UiPresence::default(),
            interaction_domain: None,
            drop_action: None,
        }
    } else {
        UiTreeNode { presentation: Default::default(), menu: None, sections: tree_sections, presence: UiPresence::default(), drop_action: None, interaction_domain: None }
    })
}

fn ui_declarative_child_to_tree_item(node: &UiNode, fallback_id: String) -> UiTreeItemNode {
    match node {
        UiNode::Text(text) => UiTreeItemNode {
            tone: None,
            window: None,
            granularity: None,
            menu: None,
            id: format!("{}.text", fallback_id),
            label: text.value.clone(),
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
        },
        UiNode::Field(field) => {
            let description = if let UiNode::Input(input) = field.child.as_ref() { input.placeholder.clone().map(Label::into_string).or_else(|| if input.value.is_empty() { None } else { Some(input.value.clone()) }) } else { None };
            UiTreeItemNode {
                tone: None,
                window: None,
                granularity: None,
                menu: None,
                id: field.id.clone(),
                label: field.label.clone(),
                description,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: ui_node_to_control(&field.child),
                content_lines: None,
                inline_toolbar: None,
                detail: None,
                dimmed: None,
            }
        }
        UiNode::Button(button) => UiTreeItemNode {
            tone: None,
            window: None,
            granularity: None,
            menu: None,
            id: button.id.clone().unwrap_or(fallback_id),
            label: button.label.clone(),
            description: None,
            icon_id: None,
            presence: UiPresence::default(),
            default_open: None,
            action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: Some(UiControlNode::Button(button.clone())),
            content_lines: None,
            inline_toolbar: None,
            detail: None,
            dimmed: None,
        },
        UiNode::Input(input) => tree_control_item(input.id.clone(), UiControlNode::Input(input.clone())),
        UiNode::Select(select) => tree_control_item(select.id.clone(), UiControlNode::Select(select.clone())),
        UiNode::Toggle(toggle) => tree_control_item(toggle.id.clone(), UiControlNode::Toggle(toggle.clone())),
        UiNode::Group(group) => {
            let mut items = Vec::with_capacity(group.children.len());
            for (index, child) in group.children.iter().enumerate() {
                items.push(ui_declarative_child_to_tree_item(child, format!("{}.{}", group.id, index)));
            }
            UiTreeItemNode {
                tone: None,
                window: None,
                granularity: None,
                menu: None,
                id: group.id.clone(),
                label: group.label.clone(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: group.default_open,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: Some(items),
                control: None,
                content_lines: None,
                inline_toolbar: None,
                detail: None,
                dimmed: None,
            }
        }
        UiNode::KeyValue(key_value) => tree_control_item(fallback_id, UiControlNode::KeyValue(key_value.clone())),
        UiNode::Slider(slider) => tree_control_item(slider.id.clone(), UiControlNode::Slider(slider.clone())),
        UiNode::NumberStepper(stepper) => tree_control_item(stepper.id.clone(), UiControlNode::NumberStepper(stepper.clone())),
        UiNode::Ring(ring) => tree_control_item(ring.id.clone(), UiControlNode::Ring(ring.clone())),
        UiNode::IconSelect(icon_select) => tree_control_item(icon_select.id.clone(), UiControlNode::IconSelect(icon_select.clone())),
        UiNode::Separator(_) => UiTreeItemNode {
            tone: None,
            window: None,
            granularity: None,
            menu: None,
            id: format!("{}.sep", fallback_id),
            label: Label::data("—"),
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
        },
        other => UiTreeItemNode {
            tone: None,
            window: None,
            granularity: None,
            menu: None,
            id: fallback_id,
            label: Label::data(format!("{other:?}")),
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
        },
    }
}

fn tree_control_item(id: String, control: UiControlNode) -> UiTreeItemNode {
    UiTreeItemNode {
        tone: None,
        window: None,
        granularity: None,
        menu: None,
        id,
        label: Label::data(String::new()),
        description: None,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: Some(control),
        content_lines: None,
        inline_toolbar: None,
        detail: None,
        dimmed: None,
    }
}
//#endregion 🔖️InspectorHelpers
// #endregion ui
