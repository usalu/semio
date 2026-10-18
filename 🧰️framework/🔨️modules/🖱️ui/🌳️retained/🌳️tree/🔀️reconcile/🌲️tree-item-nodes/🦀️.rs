//! 🌲️ Tree item nodes: the tree item node, its tone, detail, inline toolbar and toolbar, the row content lines and the item controls. See [`super`].

use super::*;

// #region reconcile
//#region 🌳️DocumentTreeReconcile
/// 🌳️ Assembles one `Component::TreeItem` record and its whole subtree into the inline
/// `UiTreeItemNode` the retained `Tree` spec carries. A child that projects to a control becomes the
/// row's `control`; the explicit toolbar/detail relations stay out of the nested item list; every
/// other child recurses as a nested item.
pub(super) fn tree_item(document: &UiDocumentTree, record: &UiNodeRecord, surface: &str, controller: &str, depth: usize) -> UiTreeItemNode {
    let ui_contract::Component::TreeItem(props) = &record.component else {
        return UiTreeItemNode {
            tone: None,
            window: None,
            granularity: None,
            id: record.key.as_str().to_string(),
            label: Label::data(record.key.as_str()),
            description: None,
            icon_id: None,
            presence: record_presence(record),
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
            menu: menu_ref(record),
        };
    };
    let inline_toolbar = tree_item_inline_toolbar(document, props, controller);
    let detail = tree_item_detail(document, props, surface, controller);
    let (items, control, content_lines) = if depth < UI_DOCUMENT_RECONCILE_DEPTH {
        let items: Vec<UiTreeItemNode> = record.children.iter().filter_map(|child| document.record(*child)).filter(|child| matches!(child.component, ui_contract::Component::TreeItem(_))).map(|child| tree_item(document, child, surface, controller, depth + 1)).collect();
        let content: Vec<&UiNodeRecord> = tree_row_content(document, record).collect();
        match content.as_slice() {
            [only] if is_tree_inline_control(&only.component) => (items, tree_item_control(only, controller), None),
            _ => (items, None, tree_row_content_lines(document, record).map(|lines| u16::try_from(lines).unwrap_or(u16::MAX))),
        }
    } else {
        (Vec::new(), None, None)
    };
    UiTreeItemNode {
        window: tree_window(props.window.as_ref()),
        granularity: props.granularity.as_ref().map(|value| value.as_str().to_string()),
        id: record.key.as_str().to_string(),
        label: contract_label(&props.label),
        description: props.description.as_ref().map(|value| value.as_str().to_string()),
        icon_id: props.icon.as_ref().map(icon_name),
        presence: UiPresence { selected: props.selected == Some(true), ..record_presence(record) },
        default_open: props.default_open,
        action: row_activation(record),
        actions: row_actions(&props.row_actions, props.target.as_ref()),
        draggable: props.draggable,
        drag_data: drag_data(props.drag_data.as_ref()),
        items: if items.is_empty() { None } else { Some(items) },
        control,
        content_lines,
        inline_toolbar,
        detail,
        dimmed: props.dimmed,
        menu: menu_ref(record),
        tone: tree_row_tone(record.style.tone),
    }
}

/// 🚦️ The semantic tone a tree row paints: the four outcome roles (React's `TREE_ROW_TONE_CLASSES`); the neutral tone and
/// the brand roles paint nothing.
pub(super) fn tree_row_tone(tone: ui_contract::Tone) -> Option<ui_contract::Tone> {
    matches!(tone, ui_contract::Tone::Info | ui_contract::Tone::Success | ui_contract::Tone::Warning | ui_contract::Tone::Danger).then_some(tone)
}

/// 🎞️ Rehydrates the Tree row's explicitly related engine Surface without treating it as a nested row.
fn tree_item_detail(document: &UiDocumentTree, props: &ui_contract::TreeItemProps, surface: &str, controller: &str) -> Option<UiComponentSceneNode> {
    let record = document.record(props.detail?)?;
    let ui_contract::Component::Surface(surface_props) = &record.component else { return None };
    Some(surface_scene_node(document, record, surface_props, surface, controller))
}

/// 🎛️ Rehydrates the explicit inline Toolbar relation without reinterpreting it as another
/// TreeItem. Its children stay ordinary Button nodes with their own ids and Activate bindings.
fn tree_item_inline_toolbar(document: &UiDocumentTree, props: &ui_contract::TreeItemProps, controller: &str) -> Option<UiStackNode> {
    tree_toolbar(document, props.inline_toolbar?, controller)
}

/// 🎛️ Rehydrates a Tree row or section header's explicit Toolbar relation.
pub(super) fn tree_toolbar(document: &UiDocumentTree, toolbar: UiNodeId, controller: &str) -> Option<UiStackNode> {
    let record = document.record(toolbar)?;
    let ui_contract::Component::Container(container) = &record.component else { return None };
    if container.role != ui_contract::ContainerRole::Toolbar {
        return None;
    }
    let ui_contract::LayoutSpec::Stack(layout) = &record.layout else { return None };
    if layout.axis != ui_contract::Axis::Horizontal {
        return None;
    }
    let children = record
        .children
        .iter()
        .map(|id| {
            let button = document.record(*id)?;
            matches!(&button.component, ui_contract::Component::Button(_)).then(|| button_node(button, controller))
        })
        .collect::<Option<Vec<_>>>()?;
    let (_, gap, padding) = stack_metrics(&record.layout);
    Some(UiStackNode {
        direction: "horizontal".into(),
        gap,
        padding,
        id: Some(record.key.as_str().to_string()),
        presence: record_presence(record),
        activate: record_action(record, ui_contract::Trigger::Activate, controller),
        drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
        drop_overlay: drop_overlay(container.drop_overlay.as_ref()),
        menu: menu_ref(record),
        children,
    })
}

/// 🧺️ A tree item's content: every child record that is not a nested row, its inline toolbar or its detail, in order.
pub fn tree_row_content<'a>(document: &'a UiDocumentTree, record: &'a UiNodeRecord) -> impl Iterator<Item = &'a UiNodeRecord> + 'a {
    let (toolbar, detail) = match &record.component {
        ui_contract::Component::TreeItem(props) => (props.inline_toolbar, props.detail),
        _ => (None, None),
    };
    record
        .children
        .iter()
        .filter(move |child| Some(**child) != toolbar && Some(**child) != detail)
        .filter_map(|child| document.record(*child))
        .filter(|child| !matches!(child.component, ui_contract::Component::TreeItem(_)) && record_presence(child).visible())
}

/// 🧮️ The lines a tree item's content fills in its value column: `None` when it has no content or exactly one of the
/// nine inline controls (its `control`), else every content child's [`tree_content_lines`] stacked — so every leaf of a
/// recipe is a line of its own that a pointer, the keyboard and a reader reach.
pub fn tree_row_content_lines(document: &UiDocumentTree, record: &UiNodeRecord) -> Option<u32> {
    let content: Vec<&UiNodeRecord> = tree_row_content(document, record).collect();
    match content.as_slice() {
        [] => None,
        [only] if is_tree_inline_control(&only.component) => None,
        content => Some(content.iter().map(|child| tree_content_lines(document, child, 0)).sum()),
    }
}

/// 📏️ The lines one tree-row content record fills: a container its label lines ([`tree_content_label_lines`]) and its
/// visible children stacked, a nested row none (it is a row of its own), and every other node — a control, a button,
/// text, progress — one line.
pub fn tree_content_lines(document: &UiDocumentTree, record: &UiNodeRecord, depth: usize) -> u32 {
    if !record_presence(record).visible() || depth >= UI_DOCUMENT_RECONCILE_DEPTH {
        return 0;
    }
    let children = || record.children.iter().filter_map(|child| document.record(*child)).map(|child| tree_content_lines(document, child, depth + 1)).sum::<u32>();
    match &record.component {
        ui_contract::Component::TreeItem(_) => 0,
        ui_contract::Component::Container(props) => {
            let (above, below) = tree_content_label_lines(props);
            above + below + children()
        }
        _ => 1,
    }
}

/// 🪧️ The label lines a content container paints above and below its children, one line each: a Field's label and
/// description above and its error below, a labelled Section's title above; every other container none.
pub fn tree_content_label_lines(props: &ui_contract::ContainerProps) -> (u32, u32) {
    match props.role {
        ui_contract::ContainerRole::Field => (1 + u32::from(props.description.is_some()), u32::from(props.error.is_some())),
        ui_contract::ContainerRole::Section => (u32::from(props.label.is_some()), 0),
        _ => (0, 0),
    }
}

/// 🕹️ Whether `component` is one of the nine controls a tree row embeds inline as its `control`.
fn is_tree_inline_control(component: &ui_contract::Component) -> bool {
    matches!(
        component,
        ui_contract::Component::Input(_) | ui_contract::Component::Select(_) | ui_contract::Component::Toggle(_) | ui_contract::Component::Button(_) | ui_contract::Component::KeyValueList(_) | ui_contract::Component::Slider(_) | ui_contract::Component::NumberStepper(_) | ui_contract::Component::Ring(_) | ui_contract::Component::IconSelect(_)
    )
}

/// 🎛️ The control a tree row embeds, when its child record is one of the nine control components.
fn tree_item_control(record: &UiNodeRecord, controller: &str) -> Option<UiControlNode> {
    Some(match record.component {
        ui_contract::Component::Input(_) => UiControlNode::Input(match input_node(record, controller) {
            UiNode::Input(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Select(_) => UiControlNode::Select(match select_node(record, controller) {
            UiNode::Select(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Toggle(_) => UiControlNode::Toggle(match toggle_node(record, controller) {
            UiNode::Toggle(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Button(_) => UiControlNode::Button(match button_node(record, controller) {
            UiNode::Button(node) => node,
            _ => return None,
        }),
        ui_contract::Component::KeyValueList(_) => UiControlNode::KeyValue(match key_value_node(record) {
            UiNode::KeyValue(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Slider(_) => UiControlNode::Slider(match slider_node(record, controller) {
            UiNode::Slider(node) => node,
            _ => return None,
        }),
        ui_contract::Component::NumberStepper(_) => UiControlNode::NumberStepper(match number_stepper_node(record, controller) {
            UiNode::NumberStepper(node) => node,
            _ => return None,
        }),
        ui_contract::Component::Ring(_) => UiControlNode::Ring(match ring_node(record, controller) {
            UiNode::Ring(node) => node,
            _ => return None,
        }),
        ui_contract::Component::IconSelect(_) => UiControlNode::IconSelect(match icon_select_node(record, controller) {
            UiNode::IconSelect(node) => node,
            _ => return None,
        }),
        _ => return None,
    })
}
//#endregion 🌳️DocumentTreeReconcile
// #endregion reconcile
