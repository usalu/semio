//! ♿️ The accessibility projection this target publishes for an assistive technology to read.
//!
//! A DOM renderer gets accessibility for free: React's `Interpreter` turns each record's
//! [`ui_contract::AccessibilitySpec`] into real `aria-label`/`aria-describedby`/`aria-live`
//! attributes on an element whose tag already carries the role. A GPU canvas has no elements at
//! all, so it must SAY the same tree out loud — that is this module: one flat, ordered projection
//! built from the same retained document the paint walk consumes, published to the host, and
//! mirrored there into an offscreen ARIA subtree beside the canvas.
//!
//! The per-node semantics (the role a `Component` implies, whether it takes focus, the wire
//! spelling of a liveness level) deliberately live ONE level up, in `ui_contract`'s own
//! `♿️accessibility` region, so this target and every other answer the same shared fixture rather
//! than each inventing a role vocabulary. What lives here is only what needs the arena: tree order,
//! depth, live focus and the laid-out rect.
//!
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #3 of `📓️audit-wgpu-parity-2026-09-13.md`.

use semio_ui_retained_tree::tree::{NodeFlags, UiTree};
use ui_contract::{accessibility_projection_node, AccessibilityProjectionNode, UiNodeId, UI_DOCUMENT_NODES};

//#region ♿️Projection

/// 🪜️ The deepest document the projection walks in one pass — the same ceiling the document
/// reconcile itself carries (`UI_DOCUMENT_RECONCILE_DEPTH`), because a tree the reconcile cannot
/// mount has nothing laid out to announce.
pub const UI_ACCESSIBILITY_PROJECTION_DEPTH: usize = 64;

/// 🧭️ One pending step of the pre-order walk: the record to project, its depth, and the absolute
/// origin its own laid-out rect is relative to.
struct PendingProjection {
    id: UiNodeId,
    depth: usize,
    origin: (f32, f32),
    field_label: Option<ui_contract::Label>,
    row_description: Option<String>,
    place: Option<(usize, usize)>,
}

/// 🔢️ A slider's typed readout as the mirror's `<key>::editor` spinbutton: its text (the open draft, else the shown value),
/// whether a draft is open, and the hard range a typed value must keep, in display units.
struct SliderReadoutProjection {
    text: String,
    editing: bool,
    min: Option<f64>,
    max: Option<f64>,
}

/// 🪟️ The window a record's row children are a slice of, when it declares one.
fn record_row_window(record: &ui_contract::UiNodeRecord) -> Option<&ui_contract::TreeWindow> {
    match &record.component {
        ui_contract::Component::TreeSection(section) => section.window.as_ref(),
        ui_contract::Component::TreeItem(item) => item.window.as_ref(),
        ui_contract::Component::Table(table) => table.window.as_ref(),
        _ => None,
    }
}

/// 🧱️ Whether a record is a row of a tree or a table — the children a [`ui_contract::TreeWindow`] counts.
fn record_is_row(record: &ui_contract::UiNodeRecord) -> bool {
    matches!(record.component, ui_contract::Component::TreeItem(_) | ui_contract::Component::TableRow(_))
}

const SELECT_LISTBOX_KEY_SUFFIX: &str = "::listbox";
const SELECT_OPTION_KEY_INFIX: &str = "::option::";
const SLIDER_EDITOR_KEY_SUFFIX: &str = "::editor";

const INPUT_REFUSAL_KEY_SUFFIX: &str = "::refusal";
pub const ROW_ACTION_KEY_INFIX: &str = "::row-action::";

pub fn select_accessibility_option_value(record: &ui_contract::UiNodeRecord, key: &str) -> Option<String> {
    let ui_contract::Component::Select(select) = &record.component else { return None };
    let prefix = format!("{}{SELECT_OPTION_KEY_INFIX}", record.key.as_str());
    let value = key.strip_prefix(&prefix)?;
    select.items.iter().find(|item| item.value.as_str() == value).map(|item| item.value.as_str().to_string())
}

pub fn is_slider_accessibility_editor(record: &ui_contract::UiNodeRecord, key: &str) -> bool {
    matches!(record.component, ui_contract::Component::Slider(_)) && key == format!("{}{SLIDER_EDITOR_KEY_SUFFIX}", record.key.as_str())
}

/// 🎬️ A tree or table row record's `RowAction`s and the name React composes their buttons' names with (the tree item's label,
/// the table row's first cell).
fn record_row_actions(record: &ui_contract::UiNodeRecord) -> Option<(&str, &ui_contract::UiFixedList<ui_contract::RowAction>)> {
    match &record.component {
        ui_contract::Component::TreeItem(item) => Some((item.label.0.as_str(), &item.row_actions)),
        ui_contract::Component::TableRow(row) => Some((row.cells.get(0).map_or(record.key.as_str(), |cell| cell.as_str()), &row.row_actions)),
        _ => None,
    }
}

/// ⌨️ Which of a row's actions the virtual `<rowKey>::row-action::<i>` button of [`accessibility_projection`] activates.
pub fn row_accessibility_action(record: &ui_contract::UiNodeRecord, key: &str) -> Option<usize> {
    let (_, actions) = record_row_actions(record)?;
    let index = key.strip_prefix(record.key.as_str())?.strip_prefix(ROW_ACTION_KEY_INFIX)?.parse::<usize>().ok()?;
    actions.get(index).map(|_| index)
}

/// 🕸️ A row's actions as buttons — the trailing icons of its Row-placed actions and the context-menu entries of its
/// Menu-placed ones alike — named `"<label>: <row name>"` as React's `TableView` names them, reachable and activatable, so no
/// row action is pointer-only on this target.
fn row_action_accessibility_nodes(record: &ui_contract::UiNodeRecord, depth: usize, owner: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    let Some((name, actions)) = record_row_actions(record) else { return Vec::new() };
    let mut nodes = Vec::new();
    for (index, action) in actions.iter().enumerate() {
        let mut button = owner.clone();
        button.key = format!("{}{ROW_ACTION_KEY_INFIX}{index}", record.key.as_str());
        button.role = "button".to_string();
        button.depth = depth.saturating_add(1);
        button.label = Some(action.label.as_ref().map_or_else(|| name.to_string(), |label| format!("{}: {name}", label.0.as_str())));
        let disabled = record.disabled || action.disabled;
        button.description = action.reason.as_ref().filter(|reason| disabled && !reason.0.as_str().is_empty()).map(|reason| reason.0.as_str().to_string());
        button.shortcut = None;
        button.disabled = disabled;
        button.focusable = !disabled || button.description.is_some();
        button.tabbable = button.focusable;
        button.actionable = !disabled;
        button.focused = false;
        button.checked = None;
        button.pressed = None;
        button.selected = None;
        button.expanded = None;
        button.level = None;
        button.value_min = None;
        button.value_max = None;
        button.value_now = None;
        button.value_text = None;
        button.busy = false;
        button.value_step = None;
        button.invalid = false;
        button.set_size = None;
        button.pos_in_set = None;
        button.tone = None;
        nodes.push(button);
    }
    nodes
}

fn select_accessibility_nodes(record: &ui_contract::UiNodeRecord, depth: usize, owner: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    let ui_contract::Component::Select(select) = &record.component else { return Vec::new() };
    let mut nodes = Vec::with_capacity(select.items.len().saturating_add(1));
    let mut listbox = owner.clone();
    listbox.key = format!("{}{SELECT_LISTBOX_KEY_SUFFIX}", record.key.as_str());
    listbox.role = "listbox".to_string();
    listbox.depth = depth;
    listbox.description = None;
    listbox.shortcut = None;
    listbox.focusable = false;
    listbox.actionable = false;
    listbox.focused = false;
    listbox.checked = None;
    listbox.selected = None;
    listbox.expanded = None;
    listbox.level = None;
    listbox.rect = None;
    listbox.value_min = None;
    listbox.value_max = None;
    listbox.value_now = None;
    listbox.value_text = None;
    listbox.busy = false;
    listbox.value_step = None;
    listbox.invalid = false;
    listbox.set_size = None;
    listbox.pos_in_set = None;
    listbox.tone = None;
    nodes.push(listbox.clone());
    for item in select.items.iter() {
        let mut option = listbox.clone();
        option.key = format!("{}{SELECT_OPTION_KEY_INFIX}{}", record.key.as_str(), item.value.as_str());
        option.role = "option".to_string();
        option.depth = depth.saturating_add(1);
        option.label = Some(item.label.0.as_str().to_string());
        option.actionable = !record.disabled;
        option.selected = Some(item.value == select.value);
        option.value_text = Some(item.value.as_str().to_string());
        nodes.push(option);
    }
    nodes
}

/// 🔘️ A segmented select's options as the radios of its radio group — always projected, named by their labels, the chosen
/// one checked and the group's ONE Tab stop (the first while none is chosen); empty for every other record.
fn segmented_accessibility_nodes(record: &ui_contract::UiNodeRecord, depth: usize, owner: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    let ui_contract::Component::Select(select) = &record.component else { return Vec::new() };
    if select.appearance != ui_contract::SelectAppearance::Segmented {
        return Vec::new();
    }
    let any_chosen = select.items.iter().any(|item| item.value == select.value);
    select
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let chosen = item.value == select.value;
            let mut radio = owner.clone();
            radio.key = format!("{}{SELECT_OPTION_KEY_INFIX}{}", record.key.as_str(), item.value.as_str());
            radio.role = "radio".to_string();
            radio.depth = depth.saturating_add(1);
            radio.label = Some(item.label.0.as_str().to_string());
            radio.description = None;
            radio.shortcut = None;
            radio.focusable = !record.disabled;
            radio.tabbable = !record.disabled && (chosen || (!any_chosen && index == 0));
            radio.actionable = !record.disabled;
            radio.focused = false;
            radio.checked = Some(chosen);
            radio.pressed = None;
            radio.selected = None;
            radio.expanded = None;
            radio.level = None;
            radio.rect = None;
            radio.value_min = None;
            radio.value_max = None;
            radio.value_now = None;
            radio.value_text = None;
            radio.busy = false;
            radio.value_step = None;
            radio.invalid = false;
            radio.set_size = Some(select.items.len());
            radio.pos_in_set = Some(index + 1);
            radio.tone = None;
            radio
        })
        .collect()
}

/// 🧩️ What a virtual key under a record that mounts synthesized children (an icon selector's four cells, an extension host's rows) names: the child control, and for a `<child>::option::<value>` key the option of that child's select it picks.
pub fn accessibility_cell(tree: &UiTree, owner: semio_ui_arena::NodeId, key: &str) -> Option<(semio_ui_arena::NodeId, Option<String>)> {
    if let Some((_, index)) = key.split_once(PICKER_KEY_INFIX) {
        let cell_key = format!("picker.{index}");
        let mut child = tree.node(owner)?.first_child;
        while let Some(candidate) = child {
            let node = tree.node(candidate)?;
            if matches!(&node.key, semio_ui_retained_tree::tree::NodeKey::Explicit(existing) if *existing == cell_key) {
                return Some((candidate, None));
            }
            child = node.next_sibling;
        }
        return None;
    }
    let (cell_key, option) = match key.split_once(SELECT_OPTION_KEY_INFIX) {
        Some((cell, value)) => (cell, Some(value)),
        None => (key, None),
    };
    let mut child = tree.node(owner)?.first_child;
    while let Some(candidate) = child {
        let node = tree.node(candidate)?;
        if matches!(&node.key, semio_ui_retained_tree::tree::NodeKey::Explicit(existing) if existing == cell_key) {
            let value = match (option, &node.spec.0) {
                (None, _) => None,
                (Some(value), semio_ui_declaration::ui::UiNode::Select(select)) if node.state.open && select.items.iter().any(|item| item.value == value) => Some(value.to_string()),
                _ => return None,
            };
            return Some((candidate, value));
        }
        child = node.next_sibling;
    }
    None
}

/// 🖼️ The synthesized children `keys` of `owner` as the controls React renders — a select as a combobox (with its listbox and options while open), an input as a textbox, a button by its text, text as text — in order, named by the owner's own label where React labels a field by its owner; children the sync pass has not minted yet are skipped.
fn cell_accessibility_nodes(tree: &UiTree, owner: semio_ui_arena::NodeId, keys: &[String], depth: usize, record: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    use semio_ui_declaration::ui::UiNode;
    let origin = record.rect.map_or((0.0, 0.0), |rect| (rect[0], rect[1]));
    let mut nodes = Vec::new();
    for key in keys {
        let Some((id, _)) = accessibility_cell(tree, owner, key) else { continue };
        let Some(cell) = tree.node(id) else { continue };
        let mut node = record.clone();
        node.key = key.clone();
        node.depth = depth.saturating_add(1);
        node.description = None;
        node.shortcut = None;
        node.value_text = None;
        node.rect = Some([origin.0 + cell.layout.x, origin.1 + cell.layout.y, cell.layout.width, cell.layout.height]);
        node.focused = cell.flags.contains(NodeFlags::FOCUSED);
        node.focusable = !record.disabled;
        node.tabbable = node.focusable;
        node.actionable = node.focusable;
        match &cell.spec.0 {
            UiNode::Select(select) => {
                node.role = "combobox".to_string();
                node.expanded = Some(cell.state.open);
                node.value_text = select.items.iter().find(|item| item.value == select.value).map(|item| item.label.as_str().to_string());
                nodes.push(node.clone());
                if cell.state.open {
                    let mut listbox = node.clone();
                    listbox.key = format!("{key}{SELECT_LISTBOX_KEY_SUFFIX}");
                    listbox.role = "listbox".to_string();
                    listbox.focusable = false;
                    listbox.tabbable = false;
                    listbox.actionable = false;
                    listbox.focused = false;
                    listbox.expanded = None;
                    listbox.value_text = None;
                    listbox.rect = None;
                    nodes.push(listbox.clone());
                    for item in &select.items {
                        let mut option = listbox.clone();
                        option.key = format!("{key}{SELECT_OPTION_KEY_INFIX}{}", item.value);
                        option.role = "option".to_string();
                        option.depth = listbox.depth.saturating_add(1);
                        option.label = Some(item.label.as_str().to_string());
                        option.actionable = !record.disabled;
                        option.selected = Some(item.value == select.value);
                        option.value_text = Some(item.value.clone());
                        nodes.push(option);
                    }
                }
                continue;
            }
            UiNode::Input(input) => {
                node.role = "textbox".to_string();
                node.editable = !record.disabled;
                node.multiline = input.input_kind == "longText";
                node.value_text = Some(cell.state.edit.as_ref().map_or_else(|| input.value.clone(), |edit| edit.text.clone()));
            }
            UiNode::Button(button) => {
                node.role = "button".to_string();
                node.label = Some(button.label.as_str().to_string());
            }
            UiNode::Text(text) => {
                node.role = "text".to_string();
                node.label = Some(text.value.as_str().to_string());
                node.focusable = false;
                node.tabbable = false;
                node.actionable = false;
            }
            _ => continue,
        }
        nodes.push(node);
    }
    nodes
}

/// 🗓️ The infix of a picker cell's projection key: `<input key>::picker.<index>`, which `accessibility_cell` resolves back to the popup's `picker.<index>` child.
pub const PICKER_KEY_INFIX: &str = "::picker.";

/// 🗓️ An open date or colour popup as the roles the browser's pickers expose: a calendar as `grid` > `row` > `gridcell` (each day named by its ISO date, the field's day `selected`) with previous and next month buttons named in the host language; a colour popup as a `listbox` of `option` swatches (named by hex, the current colour `selected`) with the hex and channel fields as text boxes. Cells the sync pass has not minted yet are skipped.
fn picker_accessibility_nodes(tree: &UiTree, owner: semio_ui_arena::NodeId, depth: usize, record: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    use semio_ui_declaration::ui::UiNode;
    let Some(owner_node) = tree.node(owner) else { return Vec::new() };
    let UiNode::Input(input) = &owner_node.spec.0 else { return Vec::new() };
    if !owner_node.state.open || !semio_ui_wgpu_layout::date_picker::is_picker_kind(&input.input_kind) {
        return Vec::new();
    }
    let names = owner_node.state.picker_names.unwrap_or(["", "", ""]);
    let origin = record.rect.map_or((0.0, 0.0), |rect| (rect[0], rect[1]));
    let base = |suffix: &str, role: &str, extra: usize| {
        let mut node = record.clone();
        node.key = format!("{}{PICKER_KEY_INFIX}{suffix}", record.key);
        node.role = role.to_string();
        node.depth = depth.saturating_add(1 + extra);
        node.description = None;
        node.shortcut = None;
        node.value_text = None;
        node.expanded = None;
        node.selected = None;
        node.focused = false;
        node.editable = false;
        node.focusable = false;
        node.tabbable = false;
        node.actionable = false;
        node
    };
    let cell = |index: usize| {
        let mut child = tree.node(owner)?.first_child;
        while let Some(candidate) = child {
            let node = tree.node(candidate)?;
            if matches!(&node.key, semio_ui_retained_tree::tree::NodeKey::Explicit(existing) if *existing == format!("picker.{index}")) {
                return Some(node);
            }
            child = node.next_sibling;
        }
        None
    };
    let place = |node: &mut AccessibilityProjectionNode, cell: &semio_ui_retained_tree::tree::Node| {
        node.rect = Some([origin.0 + cell.layout.x, origin.1 + cell.layout.y, cell.layout.width, cell.layout.height]);
    };
    let label_of = |cell: &semio_ui_retained_tree::tree::Node| match &cell.spec.0 {
        UiNode::Button(button) => button.label.as_str().to_string(),
        UiNode::Input(input) => input.value.clone(),
        _ => String::new(),
    };
    let mut nodes = Vec::new();
    if input.input_kind == "date" {
        for (index, name) in [(1, names[0]), (2, names[1])] {
            let Some(cell_node) = cell(index) else { continue };
            let mut node = base(&index.to_string(), "button", 0);
            node.label = Some(name.to_string());
            node.focusable = !record.disabled;
            node.actionable = !record.disabled;
            place(&mut node, cell_node);
            nodes.push(node);
        }
        let mut grid = base("grid", "grid", 0);
        grid.label = cell(3).map(label_of);
        if let Some(panel) = cell(0) {
            place(&mut grid, panel);
        }
        nodes.push(grid);
        for row in 0..semio_ui_wgpu_layout::date_picker::DATE_GRID_ROWS {
            let mut row_node = base(&format!("row.{row}"), "row", 1);
            row_node.label = None;
            nodes.push(row_node);
            for column in 0..semio_ui_wgpu_layout::date_picker::DATE_GRID_COLUMNS {
                let index = 4 + row * semio_ui_wgpu_layout::date_picker::DATE_GRID_COLUMNS + column;
                let Some(cell_node) = cell(index) else { continue };
                let UiNode::Button(button) = &cell_node.spec.0 else { continue };
                let mut node = base(&index.to_string(), "gridcell", 2);
                node.label = Some(button.action.args.as_ref().and_then(|args| args.get("value")).and_then(semio_framework_value::DslValue::as_str).map_or_else(|| button.label.as_str().to_string(), str::to_string));
                node.selected = Some(button.presence.selected);
                node.focusable = !record.disabled;
                node.actionable = !record.disabled;
                place(&mut node, cell_node);
                nodes.push(node);
            }
        }
    } else {
        let mut listbox = base("listbox", "listbox", 0);
        listbox.label = Some(names[2].to_string());
        if let Some(panel) = cell(0) {
            place(&mut listbox, panel);
        }
        nodes.push(listbox);
        let current = ui_contract::parse_ui_color_hex(&input.value).map(|rgba| ui_contract::ui_color_hex(&rgba, false));
        for index in 2..2 + semio_ui_wgpu_layout::colour_picker::COLOUR_SWATCHES.len() {
            let Some(cell_node) = cell(index) else { continue };
            let mut node = base(&index.to_string(), "option", 1);
            node.label = Some(label_of(cell_node));
            node.selected = Some(current.as_deref() == Some(label_of(cell_node).as_str()));
            node.focusable = !record.disabled;
            node.actionable = !record.disabled;
            place(&mut node, cell_node);
            nodes.push(node);
        }
        for (index, name) in [(18, "Hex"), (19, "R"), (20, "G"), (21, "B")] {
            let Some(cell_node) = cell(index) else { continue };
            let mut node = base(&index.to_string(), "textbox", 0);
            node.label = Some(name.to_string());
            node.value_text = Some(cell_node.state.edit.as_ref().map_or_else(|| label_of(cell_node), |edit| edit.text.clone()));
            node.editable = !record.disabled;
            node.focusable = !record.disabled;
            node.tabbable = node.focusable;
            node.focused = cell_node.flags.contains(NodeFlags::FOCUSED);
            place(&mut node, cell_node);
            nodes.push(node);
        }
    }
    nodes
}

/// 🖼️ An icon selector's four cells (`<id>.mode`, `.field`, `.import`, `.clear`) as [`cell_accessibility_nodes`].
fn icon_select_accessibility_nodes(tree: &UiTree, selector: semio_ui_arena::NodeId, depth: usize, owner: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    let Some(semio_ui_declaration::ui::UiNode::IconSelect(select)) = tree.node(selector).map(|node| &node.spec.0) else { return Vec::new() };
    let keys: Vec<String> = (0..semio_ui_retained_tree::reconcile::ICON_SELECT_CELLS).map(|index| semio_ui_retained_tree::reconcile::icon_select_cell_key(select, index)).collect();
    cell_accessibility_nodes(tree, selector, &keys, depth, owner)
}

/// 🔌️ A hosted extension's rows as [`cell_accessibility_nodes`], so what its host renders is reachable like any control.
fn extension_accessibility_nodes(tree: &UiTree, slot: semio_ui_arena::NodeId, depth: usize, owner: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    let Some(semio_ui_declaration::ui::UiNode::ExternalSlot(external)) = tree.node(slot).map(|node| &node.spec.0) else { return Vec::new() };
    let keys: Vec<String> = (0..semio_ui_retained_tree::reconcile::extension_host_rows(&external.body_key, &external.params_json).len()).map(|index| semio_ui_retained_tree::reconcile::extension_row_key(&external.body_key, index)).collect();
    cell_accessibility_nodes(tree, slot, &keys, depth, owner)
}

/// 🌳️ The accessibility tree ONE window's retained document publishes, in pre-order — the reading
/// order an assistive technology walks.
///
/// Answers an empty projection rather than a fault for a window that has published no document yet:
/// "nothing to announce" and "not laid out" are the same thing to a reader, and a probe can tell
/// them apart from the window list instead.
///
/// A tree row whose own click is its target's activation (`RowTarget::activation`) is stamped `actionable`: that
/// activation is no record binding, so the contract's own projection reads the row as inert and the mirror forwarded
/// neither a click nor a key on it (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F18).
///
/// `focused` and `rect` are stamped from the ARENA (the retained node the record mounted to), since
/// they are live interaction state the published document itself does not carry: `rect` is absolute,
/// accumulated down the walk exactly the way `paint_node` accumulates its own origin, and comes from
/// the double-buffered `mounted_layout` the paint pass consumes — never the immediate-mode
/// `Node::layout` bucket, which stays zero on the retained path.
pub fn accessibility_projection(tree: &UiTree) -> Vec<AccessibilityProjectionNode> {
    let Some(document) = tree.document() else { return Vec::new() };
    let mut projection = Vec::new();
    let mut stack = vec![PendingProjection { id: document.root_id(), depth: 0, origin: (0.0, 0.0), field_label: None, row_description: None, place: None }];
    while let Some(pending) = stack.pop() {
        if projection.len() >= UI_DOCUMENT_NODES || pending.depth >= UI_ACCESSIBILITY_PROJECTION_DEPTH {
            continue;
        }
        let Some(record) = document.record(pending.id) else { continue };
        let mut node = accessibility_projection_node(record, pending.depth);
        if let ui_contract::Component::TreeItem(props) = &record.component {
            node.selected = props.selected;
            node.actionable |= !record.disabled && props.target.as_ref().is_some_and(|target| target.activation.is_some());
        }
        if let Some((position, size)) = pending.place {
            node.pos_in_set = Some(position);
            node.set_size = Some(size);
        }
        if let Some(note) = tree.presence_note(record.key.as_str()) {
            node.description = Some(match node.description.take() {
                Some(description) => format!("{description} · {note}"),
                None => note.to_string(),
            });
        }
        if let Some(row) = pending.row_description.as_deref().filter(|_| node.focusable || node.actionable) {
            node.description = Some(match node.description.take() {
                Some(own) => format!("{own} · {row}"),
                None => row.to_string(),
            });
        }
        let mut slider_readout = None;
        let mut refusal_alert = None;
        if node.label.is_none()
            && matches!(
                record.component,
                ui_contract::Component::Input(_)
                    | ui_contract::Component::Select(_)
                    | ui_contract::Component::Toggle(_)
                    | ui_contract::Component::Slider(_)
                    | ui_contract::Component::NumberStepper(_)
                    | ui_contract::Component::Ring(_)
                    | ui_contract::Component::IconSelect(_)
            )
        {
            node.label = pending.field_label.as_ref().map(|label| label.0.as_str().to_string());
        }
        let mut origin = pending.origin;
        let mut children_visible = true;
        if let Some(mounted) = tree.document_node(pending.id) {
            if let Some(arena_node) = tree.node(mounted) {
                node.focused = arena_node.flags.contains(NodeFlags::FOCUSED);
                if node.role == "treeitem" {
                    node.selected = node.selected.or(Some(arena_node.spec.0.presence().selected));
                }
                match &arena_node.spec.0 {
                    semio_ui_declaration::ui::UiNode::ExternalSlot(slot) if slot.host_status.is_some() => {
                        node.role = "status".to_string();
                        node.label = slot.host_status.clone();
                        node.focusable = false;
                        node.tabbable = false;
                        node.actionable = false;
                        node.editable = false;
                        node.focused = false;
                    }
                    semio_ui_declaration::ui::UiNode::Input(input) => {
                        node.value_text = Some(arena_node.state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_else(|| input.value.clone()));
                        if input.input_kind == "number" {
                            node.value_now = arena_node.state.edit.as_ref().and_then(|edit| edit.text.parse().ok()).or(node.value_now);
                        }
                        if semio_ui_wgpu_layout::date_picker::is_picker_kind(&input.input_kind) {
                            node.expanded = Some(arena_node.state.open);
                        }
                        if input.input_kind == "longText" {
                            node.multiline = true;
                            node.editable = !node.disabled;
                        }
                    }
                    semio_ui_declaration::ui::UiNode::Slider(slider) => {
                        let value = arena_node.state.slider_draft_value.unwrap_or(slider.value);
                        let shown = ui_contract::ui_number_display(value, slider.display_factor);
                        node.value_now = Some(if slider.display_factor.is_some() { ui_contract::format_ui_number(shown).parse().unwrap_or(shown) } else { value });
                        node.value_text = Some(semio_ui_wgpu_layout::layout::slider_unit_label(&slider.readout(value), slider.shown_unit()).unwrap_or_else(|| slider.readout(value)));
                        let draft = arena_node.state.edit.as_ref().map(|edit| edit.text.clone());
                        if draft.is_some() {
                            node.focused = false;
                        }
                        if draft.is_some() || (!node.disabled && node.actionable) {
                            let shown = |stored: f64| if slider.display_factor.is_some() { ui_contract::format_ui_number(ui_contract::ui_number_display(stored, slider.display_factor)).parse().unwrap_or(stored) } else { stored };
                            let (min, max) = match slider.limits.as_ref() {
                                Some(limits) => (limits.min.as_ref().map(|bound| shown(bound.value)), limits.max.as_ref().map(|bound| shown(bound.value))),
                                None => (node.value_min, node.value_max),
                            };
                            slider_readout = Some(SliderReadoutProjection { editing: draft.is_some(), text: draft.unwrap_or_else(|| slider.readout(value)), min, max });
                        }
                    }
                    semio_ui_declaration::ui::UiNode::NumberStepper(stepper) => {
                        node.value_text = Some(arena_node.state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_else(|| if stepper.uniform { stepper.value_text(stepper.value) } else { String::new() }));
                        node.value_now = arena_node.state.edit.as_ref().and_then(|edit| edit.text.parse().ok()).or_else(|| stepper.uniform.then(|| ui_contract::ui_number_display(stepper.value, stepper.display_factor)));
                    }
                    semio_ui_declaration::ui::UiNode::Select(_) => node.expanded = Some(arena_node.state.open),
                    semio_ui_declaration::ui::UiNode::Toggle(toggle) => match toggle.appearance {
                        ui_contract::ToggleAppearance::Button => node.pressed = Some(toggle.presence.selected),
                        ui_contract::ToggleAppearance::Checkbox => node.checked = Some(toggle.presence.selected),
                    },
                    _ => {}
                }
                node.invalid = arena_node.state.number_refusal.is_some() && arena_node.state.edit.is_some();
                refusal_alert = arena_node.state.number_refusal.clone().filter(|refusal| !refusal.is_empty() && arena_node.state.edit.is_some() && matches!(arena_node.spec.0, semio_ui_declaration::ui::UiNode::Input(_)));
                if let Some(refusal) = arena_node.state.number_refusal.as_deref().filter(|refusal| !refusal.is_empty() && arena_node.state.edit.is_some()) {
                    node.description = Some(node.description.take().map_or_else(|| refusal.to_string(), |description| format!("{refusal} · {description}")));
                }
                if let Some(open) = tree.disclosure_open(mounted) {
                    node.expanded = Some(open);
                    children_visible = open;
                }
            }
            if let Some((x, y, width, height)) = tree.mounted_layout(mounted) {
                origin = (pending.origin.0 + x, pending.origin.1 + y);
                node.rect = Some([origin.0, origin.1, width, height]);
            }
        }
        let select_open = tree.document_node(pending.id).and_then(|mounted| tree.node(mounted)).is_some_and(|arena_node| matches!(&arena_node.spec.0, semio_ui_declaration::ui::UiNode::Select(_)) && arena_node.state.open);
        let segmented = matches!(&record.component, ui_contract::Component::Select(select) if select.appearance == ui_contract::SelectAppearance::Segmented);
        if segmented {
            node.focusable = false;
            node.tabbable = false;
            node.expanded = None;
        }
        projection.push(node.clone());
        if let Some(refusal) = refusal_alert.filter(|_| projection.len() < UI_DOCUMENT_NODES) {
            let mut alert = ui_contract::AccessibilityProjectionNode { node_id: node.node_id, key: format!("{}{INPUT_REFUSAL_KEY_SUFFIX}", record.key.as_str()), role: "alert".to_string(), depth: node.depth + 1, label: Some(refusal), live: "assertive".to_string(), rect: node.rect.map(|[x, y, width, height]| [x, y + height, width, height]), ..Default::default() };
            alert.hidden = node.hidden;
            projection.push(alert);
        }
        if let Some(readout) = slider_readout {
            if projection.len() < UI_DOCUMENT_NODES {
                let mut editor = node.clone();
                editor.key = format!("{}{SLIDER_EDITOR_KEY_SUFFIX}", record.key.as_str());
                editor.role = "spinbutton".to_string();
                editor.focusable = true;
                editor.tabbable = readout.editing;
                editor.actionable = true;
                editor.focused = readout.editing;
                editor.editable = true;
                editor.value_min = readout.min;
                editor.value_max = readout.max;
                editor.value_now = readout.text.trim().parse().ok();
                editor.value_text = Some(readout.text);
                editor.set_size = None;
                editor.pos_in_set = None;
                projection.push(editor);
            }
        }
        if select_open && !segmented {
            for virtual_node in select_accessibility_nodes(record, pending.depth, &node) {
                if projection.len() >= UI_DOCUMENT_NODES {
                    break;
                }
                projection.push(virtual_node);
            }
        }
        if let Some(mounted) = tree.document_node(pending.id) {
            let cells = match record.component {
                ui_contract::Component::IconSelect(_) => icon_select_accessibility_nodes(tree, mounted, pending.depth, &node),
                ui_contract::Component::Extension(_) => extension_accessibility_nodes(tree, mounted, pending.depth, &node),
                ui_contract::Component::Input(_) => picker_accessibility_nodes(tree, mounted, pending.depth, &node),
                _ => Vec::new(),
            };
            for virtual_node in cells {
                if projection.len() >= UI_DOCUMENT_NODES {
                    break;
                }
                projection.push(virtual_node);
            }
        }
        for virtual_node in segmented_accessibility_nodes(record, pending.depth, &node) {
            if projection.len() >= UI_DOCUMENT_NODES {
                break;
            }
            projection.push(virtual_node);
        }
        for virtual_node in row_action_accessibility_nodes(record, pending.depth, &node) {
            if projection.len() >= UI_DOCUMENT_NODES {
                break;
            }
            projection.push(virtual_node);
        }
        if children_visible {
            let window = record_row_window(record);
            let mut row = record.children.iter().filter(|child| document.record(**child).is_some_and(record_is_row)).count();
            for index in (0..record.children.len()).rev() {
                if let Some(child) = record.children.get(index) {
                    let is_row = document.record(*child).is_some_and(record_is_row);
                    if is_row {
                        row = row.saturating_sub(1);
                    }
                    let place = window.filter(|_| is_row).map(|window| ((window.offset as usize).saturating_add(row).saturating_add(1), window.total as usize));
                    let field_label = match &record.component {
                        ui_contract::Component::Container(props) if props.role == ui_contract::ContainerRole::Field => {
                            document.record(*child).filter(|child| record.children.len() == 1 || child.key.as_str().strip_suffix(".control") == Some(record.key.as_str())).and(props.label.clone())
                        }
                        _ => None,
                    };
                    let row_description = match &record.component {
                        ui_contract::Component::TreeItem(item) => item.description.as_ref().filter(|_| Some(*child) != item.inline_toolbar && Some(*child) != item.detail).map(|description| description.as_str().to_string()),
                        _ => pending.row_description.clone(),
                    }
                    .filter(|_| document.record(*child).is_some_and(|child| !matches!(child.component, ui_contract::Component::TreeItem(_))));
                    stack.push(PendingProjection { id: *child, depth: pending.depth + 1, origin, field_label, row_description, place });
                }
            }
        }
    }
    tree_item_levels(&mut projection);
    roving_tab_stops(&mut projection);
    projection
}

/// 🪜️ A tree row's `aria-level` is its nesting among tree rows: a section is a heading of the tree, never a level, so the rows of a section
/// start at level 1 (the contract's `depth + 1` also counts every container the row sits in).
fn tree_item_levels(projection: &mut [AccessibilityProjectionNode]) {
    let mut ancestors: Vec<usize> = Vec::new();
    for node in projection.iter_mut() {
        while ancestors.last().is_some_and(|depth| *depth >= node.depth) {
            ancestors.pop();
        }
        if node.role == "treeitem" {
            node.level = Some(ancestors.len() + 1);
            ancestors.push(node.depth);
        }
    }
}

/// 🎯️ Leaves ONE tab stop in each tree, as React's `applyRovingTabindexV1` does: the focused row, else the first selected row, else the first row.
/// Every other `treeitem` of that tree stays focusable — a key or a click reaches it — but is not `tabbable`.
fn roving_tab_stops(projection: &mut [AccessibilityProjectionNode]) {
    let mut start = 0;
    while start < projection.len() {
        if projection[start].role != "tree" {
            start += 1;
            continue;
        }
        let depth = projection[start].depth;
        let end = (start + 1..projection.len()).find(|&index| projection[index].depth <= depth).unwrap_or(projection.len());
        let rows: Vec<usize> = (start + 1..end).filter(|&index| projection[index].role == "treeitem" && projection[index].focusable && !projection[index].hidden).collect();
        let stop = rows.iter().copied().find(|&index| projection[index].focused).or_else(|| rows.iter().copied().find(|&index| projection[index].selected == Some(true))).or_else(|| rows.first().copied());
        for &index in &rows {
            projection[index].tabbable = Some(index) == stop;
        }
        start = end;
    }
}

/// 🏷️ Every node a reader can actually reach and name — the subset an ARIA mirror must expose, and
/// the exact subset this target's law measures: focusable or actionable, not hidden, carrying a
/// label of its own.
pub fn accessibility_announced(projection: &[AccessibilityProjectionNode]) -> Vec<&AccessibilityProjectionNode> {
    projection.iter().filter(|node| !node.hidden && (node.focusable || node.actionable) && node.label.is_some()).collect()
}

//#endregion ♿️Projection

#[cfg(test)]
#[path = "../../../🧪️tests/♿️retained-section-accessibility/🦀️.rs"]
mod retained_section_collapse_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-accessibility-projection/🦀️.rs"]
mod tests;

/// 🔒️ React's modal isolation for assistive tech (`aria-hidden` on the siblings of an open `Dialog`): in the flat, depth-ordered `projection`, every node outside the subtree of the modal `modal_node_id` and not one of its ancestors is hidden. Without that node in the projection nothing changes.
pub fn hide_outside_modal(projection: &mut [AccessibilityProjectionNode], modal_node_id: u64) {
    let Some(modal) = projection.iter().position(|node| node.node_id == modal_node_id) else { return };
    let depth = projection[modal].depth;
    let end = projection[modal + 1..].iter().position(|node| node.depth <= depth).map_or(projection.len(), |offset| modal + 1 + offset);
    let mut ancestors = Vec::new();
    let mut shallowest = depth;
    for index in (0..modal).rev() {
        if projection[index].depth < shallowest {
            shallowest = projection[index].depth;
            ancestors.push(index);
        }
    }
    for (index, node) in projection.iter_mut().enumerate() {
        if !(modal..end).contains(&index) && !ancestors.contains(&index) {
            node.hidden = true;
        }
    }
}
