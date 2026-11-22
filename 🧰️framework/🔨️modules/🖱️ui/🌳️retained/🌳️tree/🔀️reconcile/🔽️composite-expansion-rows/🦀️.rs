//! 🔽️ Composite expansion rows: the select item rows, the item value arguments, the tree section and item rows and the own field and layout change checks. See [`super`].

use super::*;

// #region reconcile
//#region 🔖️CompositeExpansion
/// ☑️ Synthesizes one retained `Button` row per `Select` item, keyed by the item's own `value` (via
/// `explicit_id`'s `UiNode::Button` arm) — `UiSelectItem.value` is already Select's stable per-option
/// identity (it's what `UiSelectNode.value` itself holds to name the current choice), so reusing it as
/// the row's key needs no extra bookkeeping.
///
/// 🔓️ Ungated in ticket 26/09/17 packet W15a. It used to be `cfg(test, testkit)` alongside the
/// `apply_tree` reconciler that was its only caller, which meant PRODUCTION never materialized an
/// open popup's rows at all: `paint_select` (which draws the rows from the inline `items`) is itself
/// `cfg(test)`, the retained `UiNode::Select` paint arm draws only the trigger, and
/// `retained_hit_registration`'s `Select` arm mints one target for the trigger — so an open popup on
/// the live renderer painted nothing, hit nothing and projected nothing. The production minter is
/// `paint::sync_interactive_state_node_step`'s `SelectMint` phase, through
/// `UiTree::mint_composite_row`.
pub fn select_item_row(select: &UiSelectNode, item: &UiSelectItem) -> UiNode {
    select_item_row_committing(select, item, &item.value)
}

/// ☑️ [`select_item_row`] whose click commits `commit` instead of the item's own `value` — the row's key stays the item value, so two
/// rows may commit the same string (an icon selector's mode rows migrate the stored icon, which is empty for several modes).
pub fn select_item_row_committing(select: &UiSelectNode, item: &UiSelectItem, commit: &str) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(item.value.clone()), icon_id: IconName::CircleDot, label: item.label.clone(), action: with_item_value_arg(&select.on_change, commit), style: None, presence: UiPresence::default(), menu: None })
}

/// 🏷️ What choosing the item `item_value` of the retained select `id` commits: the item value itself, except for an icon selector's mode
/// select (a synthesized cell, see [`icon_select_cell`]) whose rows commit React's `migrateIconKindToIconSelectorMode` result. The ONE
/// answer the row builder, the keyboard commit and the assistive-technology option dispatch share.
pub fn select_commit_value(tree: &UiTree, id: NodeId, item_value: &str) -> String {
    let icon = tree.node(id).and_then(|node| node.parent).and_then(|parent| tree.node(parent)).and_then(|parent| match &parent.spec.0 {
        UiNode::IconSelect(icon) => Some(icon),
        _ => None,
    });
    match (icon, semio_ui_declaration::ui::IconSelectorMode::from_id(item_value)) {
        (Some(icon), Some(mode)) if tree.node(id).is_some_and(|node| matches!(&node.key, NodeKey::Explicit(key) if *key == icon_select_cell_key(icon, 0))) => UiIconSelectNode::migrate_mode(&icon.value, mode),
        _ => item_value.to_string(),
    }
}

//#region 🖼️IconSelectCells
/// 🧩️ How many cells an icon selector synthesizes: the mode select, the editor and the import and clear buttons.
pub const ICON_SELECT_CELLS: usize = 4;

/// 🔑️ The key of cell `index` (`mode`, `field`, `import`, `clear`) — React's `${id}.mode`, `${id}.field`; the buttons follow the same shape.
pub fn icon_select_cell_key(select: &UiIconSelectNode, index: usize) -> String {
    format!("{}.{}", select.id, ["mode", "field", "import", "clear"][index.min(ICON_SELECT_CELLS - 1)])
}

/// 🏷️ The localized mode names (`ui.iconSelector.mode.*`); empty while the host named no locale — there is no default language.
pub fn icon_select_mode_label(locale: Option<semio_ui_locale::Locale>, mode: semio_ui_declaration::ui::IconSelectorMode) -> &'static str {
    use semio_ui_declaration::ui::IconSelectorMode::*;
    locale.map_or("", |locale| {
        let labels = <semio_ui_locale::UiIconSelectorLabels as semio_ui_locale::AppLabels>::labels(locale, semio_ui_locale::Terminology::Native);
        match mode {
            Url => labels.url,
            Shortcode => labels.shortcode,
            Math => labels.math,
            Data => labels.data,
            Emoji => labels.emoji,
            Text => labels.text,
            Vector => labels.vector,
        }
        .as_str()
    })
}

/// 🧩️ Cell `index` of an icon selector as the retained node React's `IconSelector` renders there: the mode `Select` (its rows commit the
/// migrated icon through [`select_commit_value`]), the mono `longText` editor (its commit policy encodes the typed text under the stored
/// icon's mode), the Import button (the shell's file-picker verb) and the Clear button (`""`). A locked selector disables all four.
pub fn icon_select_cell(select: &UiIconSelectNode, index: usize, locale: Option<semio_ui_locale::Locale>) -> (String, UiNode) {
    use semio_ui_declaration::ui::IconSelectorMode;
    let key = icon_select_cell_key(select, index);
    let presence = UiPresence { state: if select.locked() { UiState::Disabled } else { UiState::Normal }, ..UiPresence::default() };
    let (mode, inner) = if select.uniform { UiIconSelectNode::decode_mode(&select.value) } else { (UiIconSelectNode::decode_mode(&select.value).0, String::new()) };
    let common = |pick: fn(&semio_ui_locale::UiCommonLabels) -> semio_ui_locale::LabelText| -> &'static str {
        locale.map_or("", |locale| pick(<semio_ui_locale::UiCommonLabels as semio_ui_locale::AppLabels>::labels(locale, semio_ui_locale::Terminology::Native)).as_str())
    };
    let node = match index {
        0 => UiNode::Select(UiSelectNode {
            appearance: Default::default(),
            id: key.clone(),
            value: mode.id().to_string(),
            items: IconSelectorMode::ALL.iter().map(|mode| UiSelectItem { value: mode.id().to_string(), label: Label::data(icon_select_mode_label(locale, *mode)) }).collect(),
            placeholder: None,
            on_change: select.on_change.clone(),
            presence,
            menu: None,
        }),
        1 => UiNode::Input(UiInputNode {
            id: key.clone(),
            input_kind: "longText".into(),
            value: inner,
            placeholder: Some(Label::data(icon_select_editor_placeholder(mode))),
            commit: Some(format!("{}{}", UiInputNode::ICON_MODE_COMMIT, mode.id())),
            on_change: select.on_change.clone(),
            presence,
            ..Default::default()
        }),
        2 => UiNode::Button(UiButtonNode { id: Some(key.clone()), icon_id: IconName::from("folder-open"), label: Label::data(common(|labels| labels.import_file)), action: select.import_action(), style: None, presence, menu: None }),
        _ => UiNode::Button(UiButtonNode { id: Some(key.clone()), icon_id: IconName::from("x"), label: Label::data(common(|labels| labels.clear)), action: with_item_value_arg(&select.on_change, ""), style: None, presence, menu: None }),
    };
    (key, node)
}

/// 💬️ React's editor placeholders, verbatim (`🎴️IconSelector/🟦️.tsx`).
fn icon_select_editor_placeholder(mode: semio_ui_declaration::ui::IconSelectorMode) -> &'static str {
    use semio_ui_declaration::ui::IconSelectorMode::*;
    match mode {
        Url => "https://example.com/icon.png",
        Shortcode => "Shortcode (emoji alias, UI icon, or catalog id — stored as :name:)",
        Math => "Typst markup (e.g. $x^2$)",
        Data => "data:image/png;base64,… or other data:… URL",
        Emoji => "Emoji or Typst emoji body (stored as emoji:…)",
        Text => "Short label (stored as text:…)",
        Vector => "Catalog id or inline <svg …>",
    }
}
//#endregion 🖼️IconSelectCells

/// 🏷️ Clones `action`, merging a `"value"` key into its JSON `args` object (creating one if absent)
/// so a click on one synthesized `Select` row is distinguishable from any other row once a later
/// events milestone dispatches it — `on_change.clone()` alone would fire an identical, valueless
/// action for every row. Ungated with [`select_item_row`] (packet W15a).
fn with_item_value_arg(action: &ActionDescriptor, value: &str) -> ActionDescriptor {
    let mut merged = action.clone();
    let mut entries = match merged.args.take() {
        Some(DslValue::Object(map)) => map,
        _ => Vec::new(),
    };
    entries.push(("value".to_string(), DslValue::String(value.to_string())));
    merged.args = Some(DslValue::Object(entries));
    merged
}

/// 🌳️ Synthesizes one retained `Stack` row per `Tree` section, keyed by `section.id`, wrapping its
/// `items` (recursively expanded by `tree_item_row`) as retained children.
#[cfg(any(test, feature = "testkit"))]
pub(super) fn tree_section_row(tree_node: &UiTreeNode, section: &UiTreeSectionNode) -> UiNode {
    let mut children = Vec::new();
    if let Some(toolbar) = &section.header_toolbar {
        children.push(UiNode::Stack(toolbar.clone()));
    }
    children.extend(section.items.iter().map(tree_item_row));
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some(section.id.clone()),
        presence: section.presence.clone(),
        activate: None,
        drop_action: tree_node.drop_action.clone(),
        drop_overlay: None,
        children,
        menu: None,
    })
}

/// 🌲️ Synthesizes one retained `Stack` row per `Tree` item, keyed by `item.id`. Carries the item's own
/// `presence` (already the single source of truth for selected/previewed/status — no more union with
/// a tree-level id list) and `activate` (the row's click `action`) as a `UiStackNode`'s own fields,
/// plus its embedded `control` (via `ui_control_to_node`), trailing `actions` (via
/// `tree_item_action_row`), and nested `items` (recursively) as retained children.
/// `draggable`/`drag_data` have no matching `UiStackNode` field to carry them structurally —
/// `events::is_plain_stack_container`/`find_tree_item_spec` re-derive those straight from this row's
/// key (`item.id`) against the parent `Tree` node's still-fully-intact `spec.0` (reconcile never drops
/// fields, only clones them into `WidgetSpec`). ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM W3a: `hover_action`/`unhover_action` are deleted — hover is now framework-owned per
/// `UiTreeNode.interaction_domain`, never a per-item action.
#[cfg(any(test, feature = "testkit"))]
fn tree_item_row(item: &UiTreeItemNode) -> UiNode {
    let mut children: Vec<UiNode> = Vec::new();
    if let Some(control) = &item.control {
        children.push(ui_control_to_node(control.clone()));
    }
    if let Some(toolbar) = &item.inline_toolbar {
        children.push(UiNode::Stack(toolbar.clone()));
    }
    if let Some(detail) = &item.detail {
        children.push(UiNode::ComponentScene(detail.clone()));
    }
    for action in item.actions.iter().flatten() {
        if action.placement() == UiTreeActionPlacement::Menu {
            continue;
        }
        children.push(tree_item_action_row(action));
    }
    for nested in item.items.iter().flatten() {
        children.push(tree_item_row(nested));
    }
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some(item.id.clone()),
        presence: item.presence.clone(),
        activate: item.action.clone(),
        drop_action: None,
        drop_overlay: None,
        children,
        menu: item.menu.clone(),
    })
}

/// 🔘️ Synthesizes one retained `Button` row per `UiTreeItemAction` (a `Tree` item's trailing/
/// row-placement action buttons). No stable id exists on `UiTreeItemAction` itself (unlike items/
/// sections), so this leaves `UiButtonNode.id` unset — `node_key`'s positional fallback (keyed by the
/// action's ordinal within its parent row's `actions` list) is already stable across re-renders for a
/// fixed action set, matching every other id-less synthesized/leaf child in this module.
#[cfg(any(test, feature = "testkit"))]
fn tree_item_action_row(action: &UiTreeItemAction) -> UiNode {
    let presence = UiPresence { state: if action.disabled { UiState::Disabled } else { UiState::Normal }, ..UiPresence::default() };
    UiNode::Button(UiButtonNode { id: None, icon_id: action.icon_id, label: action.label.clone().unwrap_or_else(|| Label::data("")), action: action.action.clone(), style: None, presence, menu: None })
}
//#endregion 🔖️CompositeExpansion

/// ⚖️ Whether the two nodes' *own* scalar fields (excluding nested `UiNode` children, which are
/// reconciled and dirtied independently) are equal.
#[cfg(any(test, feature = "testkit"))]
pub(super) fn own_fields_equal(previous: &UiNode, next: &UiNode) -> bool {
    match (previous, next) {
        (UiNode::Stack(p), UiNode::Stack(n)) => {
            p.direction == n.direction && p.gap == n.gap && p.padding == n.padding && p.id == n.id && p.presence == n.presence && p.activate == n.activate && p.drop_action == n.drop_action && p.children.len() == n.children.len()
        }
        (UiNode::Section(p), UiNode::Section(n)) => p.id == n.id && p.label == n.label && p.default_open == n.default_open && p.presence == n.presence && p.children.len() == n.children.len(),
        (UiNode::Field(p), UiNode::Field(n)) => p.id == n.id && p.label == n.label && p.description == n.description && p.required == n.required && p.error == n.error && p.presence == n.presence,
        _ => previous == next,
    }
}

/// 📐️ Whether the field(s) that differ between `previous` and `next` affect measurement/layout (as
/// opposed to paint-only state like `selected`/`status`/`disabled`). `presence.visible()` flipping
/// (i.e. `state` crossing into/out of `Hidden`) always counts — a hidden element occupies no layout
/// space at all, so becoming hidden/unhidden must re-run layout for its parent, unlike every other
/// `presence` change (selected/status/hover/previewed/disabled), which is paint-only.
#[cfg(any(test, feature = "testkit"))]
pub(super) fn layout_affecting_change(previous: &UiNode, next: &UiNode) -> bool {
    if previous.presence().visible() != next.presence().visible() {
        return true;
    }
    match (previous, next) {
        (UiNode::Stack(p), UiNode::Stack(n)) => p.direction != n.direction || p.gap != n.gap || p.padding != n.padding || p.children.len() != n.children.len(),
        (UiNode::Text(p), UiNode::Text(n)) => p.value != n.value,
        (UiNode::Slider(p), UiNode::Slider(n)) => p.unit_label() != n.unit_label(),
        (UiNode::Field(p), UiNode::Field(n)) => p.label != n.label || p.description != n.description || p.error != n.error,
        (UiNode::Section(p), UiNode::Section(n)) => p.label != n.label || p.children.len() != n.children.len(),
        (UiNode::OverviewCard(p), UiNode::OverviewCard(n)) => p.title != n.title || p.footer != n.footer,
        _ => false,
    }
}
// #endregion reconcile
