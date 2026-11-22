//! 🌳️ Tree roving and keys: the visible and ordered rows of a mounted `Tree`, its roving tab stop, the WAI-ARIA keys, the pointer press with its modifier and double-click rules and the `interactionSelect` pick a row sends. See [`super`].

use super::*;
use semio_ui_wgpu_events_focus::{tree_row_owner, tree_visible_rows, TREE_NAV_DEPTH_CAP, TREE_NAV_ROW_CAP};
use semio_ui_wgpu_light::tree_roving::{tree_key, tree_pointer, tree_selection, tree_typeahead, TreeIntent, TreeMerge, TreeNavModifiers, TreeNavRow, TreeSelection};

// #region events
//#region 🔖️TreeRoving
/// 🎬️ The framework-owned verb a renderer dispatches to change a domain's selection (`INTERACTION_SELECT_ACTION_ID`, `🛂️manifest`).
const INTERACTION_SELECT_VERB: &str = "interactionSelect";

/// 👆️ The framework-owned verb a bound row answers pointer motion with; a row that carries it is a pick target (`INTERACTION_HOVER_ACTION_ID`).
const INTERACTION_HOVER_VERB: &str = "interactionHover";

/// 🧮️ Every authored item of the tree in order, folded rows included, with the granularity it picks under and whether it is selected —
/// the order React ranges over (`getTreeItemOrderedIds`).
pub(super) fn tree_ordered_items(spec: &semio_ui_declaration::ui::UiTreeNode) -> Vec<(TreeNavRow, Option<String>, bool)> {
    fn walk(items: &[UiTreeItemNode], level: usize, out: &mut Vec<(TreeNavRow, Option<String>, bool)>) {
        if level > TREE_NAV_DEPTH_CAP {
            return;
        }
        for item in items {
            if out.len() >= TREE_NAV_ROW_CAP {
                return;
            }
            out.push((TreeNavRow { id: item.id.clone(), level, expanded: None, label: item.label.as_str().to_owned() }, item.granularity.clone(), item.presence.selected));
            if let Some(children) = item.items.as_deref() {
                walk(children, level + 1, out);
            }
        }
    }
    let mut out = Vec::new();
    for section in &spec.sections {
        walk(&section.items, 1, &mut out);
    }
    out
}

fn tree_spec(tree: &UiTree, owner: NodeId) -> Option<&semio_ui_declaration::ui::UiTreeNode> {
    match &tree.node(owner)?.spec.0 {
        UiNode::Tree(spec) => Some(spec),
        _ => None,
    }
}

/// ☑️ Whether the tree selects more than one row: it carries an interaction domain, whose pick merges are replace, range and invert.
pub(super) fn tree_is_multiple(tree: &UiTree, owner: NodeId) -> bool {
    tree_spec(tree, owner).is_some_and(|spec| spec.interaction_domain.is_some())
}

fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if (control as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", control as u32)),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

impl EventRouter {
    /// ⌨️ Routes one key at a focused tree row, returning whether it was consumed — [`tree_key`] answers which intents the key means (the
    /// WAI-ARIA tree pattern: moves, fold and unfold, Enter and Space, range and toggle selection) and a printable key is the typeahead.
    /// Left and Right are never mirrored: React forces its `Tree` to LTR.
    pub(super) fn route_tree_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers, out: &mut Vec<UiCommand>) -> bool {
        let Some(row) = self.focus.focused else { return false };
        let Some(owner) = tree_row_owner(tree, row) else { return false };
        let rows = tree_visible_rows(tree, owner);
        let Some(current) = rows.iter().position(|(id, _)| *id == row) else { return false };
        let nav: Vec<TreeNavRow> = rows.iter().map(|(_, row)| row.clone()).collect();
        let held = TreeNavModifiers { shift: modifiers.shift, ctrl: modifiers.ctrl, meta: modifiers.meta, alt: modifiers.alt };
        let intents = match tree_key(key, held, &nav, current, tree_is_multiple(tree, owner)) {
            Some(intents) => {
                self.select_typeahead = None;
                intents
            }
            None => {
                let mut characters = key.chars();
                let (Some(typed), None) = (characters.next(), characters.next()) else { return false };
                if held.ctrl || held.meta || held.alt || typed.is_control() {
                    return false;
                }
                let query = self.extend_select_typeahead(typed);
                match tree_typeahead(&nav, current, &query) {
                    Some(index) => vec![TreeIntent::Focus(index)],
                    None => return false,
                }
            }
        };
        self.focus_visible = true;
        self.apply_tree_intents(tree, owner, &rows, current, &intents, out);
        true
    }

    /// 🖱️ A primary press released on tree row `row`: focus it, then click it with the merge the modifiers name. A press that continues a click
    /// chain ([`Self::click_count`] above 1) yields to the row's double-click and clicks nothing — React's `event.detail > 1` rule.
    pub(super) fn route_tree_press(&mut self, tree: &mut UiTree, row: NodeId, modifiers: EventModifiers, out: &mut Vec<UiCommand>) -> bool {
        let Some(owner) = tree_row_owner(tree, row) else { return false };
        let rows = tree_visible_rows(tree, owner);
        let Some(index) = rows.iter().position(|(id, _)| *id == row) else { return false };
        let repeated = self.click_count() > 1;
        let held = TreeNavModifiers { shift: modifiers.shift, ctrl: modifiers.ctrl, meta: modifiers.meta, alt: modifiers.alt };
        let intents = tree_pointer(held, index, if repeated { 2 } else { 1 });
        self.apply_tree_intents(tree, owner, &rows, index, &intents, out);
        true
    }

    /// 📊️ A primary click released on a document-table row. React's `TableView` focuses the row on the single click (`moveTo`, which the press already did) and activates it on the double click (`onDoubleClick` calls `activateRow`): the second click of a chain fires the row's own `Trigger::Activate`, once.
    pub(super) fn route_table_row_click(&mut self, tree: &mut UiTree, row: NodeId, out: &mut Vec<UiCommand>) {
        if self.click_count() != 2 {
            return;
        }
        let fired = tree.node(row).and_then(|node| match &node.spec.0 {
            UiNode::Stack(stack) => stack.activate.as_ref().and_then(|action| bare_action(action, Trigger::Activate)),
            _ => None,
        });
        if let Some(fired) = fired {
            self.push_app_command(tree, row, fired, out);
        }
    }

    fn apply_tree_intents(&mut self, tree: &mut UiTree, owner: NodeId, rows: &[(NodeId, TreeNavRow)], before: usize, intents: &[TreeIntent], out: &mut Vec<UiCommand>) {
        let focus_before = rows.get(before).map(|(_, row)| row.id.clone());
        for intent in intents {
            match *intent {
                TreeIntent::Focus(index) => self.focus_tree_row(tree, rows[index].0, out),
                TreeIntent::Click { index, merge } => {
                    if rows[index].1.expanded.is_some() {
                        let _ = self.toggle_disclosure(tree, rows[index].0);
                    }
                    self.select_tree_row(tree, owner, rows[index].0, focus_before.as_deref(), merge, true, out);
                }
                TreeIntent::Pick { index, merge } => self.select_tree_row(tree, owner, rows[index].0, focus_before.as_deref(), merge, false, out),
                TreeIntent::DoubleClick(_) => {}
            }
        }
    }

    fn focus_tree_row(&mut self, tree: &mut UiTree, row: NodeId, out: &mut Vec<UiCommand>) {
        if self.focus.focused == Some(row) {
            return;
        }
        if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(row), true) {
            self.push_app_command(tree, blurred, fired, out);
        }
        out.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: Some(row) });
    }

    /// ☑️ What picking `row` sends. A row with its own activation fires it (React: `activateBinding ? dispatchTrigger : pickClick`); a row of a
    /// domain-bound tree sends `interactionSelect` with the selection the merge makes of the current one; a plain row sends nothing.
    #[allow(clippy::too_many_arguments, reason = "one pick: where, which row, how it merges and whether it is a click")]
    fn select_tree_row(&mut self, tree: &mut UiTree, owner: NodeId, row: NodeId, focus_before: Option<&str>, merge: TreeMerge, activating: bool, out: &mut Vec<UiCommand>) {
        let activation = tree.node(row).and_then(|node| match &node.spec.0 {
            UiNode::Stack(stack) => stack.activate.as_ref().and_then(|action| bare_action(action, Trigger::Activate)),
            _ => None,
        });
        if let (true, Some(fired)) = (activating, activation) {
            self.push_app_command(tree, row, fired, out);
            return;
        }
        let Some(target) = tree.authored_tree_item(row).map(|item| item.id.clone()) else { return };
        let Some(spec) = tree_spec(tree, owner) else { return };
        let items = tree_ordered_items(spec);
        let ordered: Vec<TreeNavRow> = items.iter().map(|(row, _, _)| row.clone()).collect();
        let current = TreeSelection { ids: items.iter().filter(|(_, _, selected)| *selected).map(|(row, _, _)| row.id.clone()).collect(), anchor: self.focus.tree_anchor.get(&owner).cloned() };
        let multiple = tree_is_multiple(tree, owner);
        let next = tree_selection(&ordered, &current, focus_before, &target, merge, multiple);
        if let Some(anchor) = next.anchor.clone() {
            self.focus.tree_anchor.insert(owner, anchor);
        }
        let (verb, ids) = match merge {
            TreeMerge::Invertive if multiple => ("invertive", vec![target]),
            _ => ("replace", next.ids),
        };
        let targets: Vec<(String, String)> = ids.iter().filter_map(|id| items.iter().find(|(row, _, _)| row.id == *id).and_then(|(row, granularity, _)| granularity.clone().map(|granularity| (row.id.clone(), granularity)))).collect();
        if let Some(command) = self.tree_pick_command(tree, row, verb, &targets) {
            out.push(command);
        }
    }

    /// 🎬️ The `interactionSelect` intent of a pick on `row`: the row's own address, and the scope and version of the `interactionHover`
    /// binding that makes it a domain-bound row; its input is React's `treePickIntentInputV1` (`merge`, `method: "pick"`, JSON `targets`).
    fn tree_pick_command(&mut self, tree: &UiTree, row: NodeId, merge: &str, targets: &[(String, String)]) -> Option<UiCommand> {
        if targets.is_empty() {
            return None;
        }
        let bindings = tree.node(row)?.intent.as_ref()?;
        let hover = bindings.action_for(Trigger::HoverPreview).filter(|action| action.name.as_str() == INTERACTION_HOVER_VERB)?;
        if intent_is_stale(bindings.address.revision, tree.document().map_or(0, |document| document.revision().0)) {
            return None;
        }
        let action = ui_contract::ActionId::new(hover.scope.clone(), ui_contract::UiText::try_from_str(INTERACTION_SELECT_VERB)?, hover.version);
        let listed: Vec<String> = targets.iter().map(|(id, granularity)| format!("{{\"granularity\":{},\"id\":{}}}", json_string(granularity), json_string(id))).collect();
        let input = DslValue::Object(vec![
            ("merge".to_string(), DslValue::String(merge.to_string())),
            ("method".to_string(), DslValue::String("pick".to_string())),
            ("targets".to_string(), DslValue::String(format!("[{}]", listed.join(",")))),
        ]);
        let args = bindings.args_for(Trigger::HoverPreview).cloned();
        let address = bindings.address.clone();
        let seq = self.intents.next(&address.surface);
        Some(UiCommand::App { window_id: self.window_id.clone(), intent: UiIntentCommand { address, trigger: Trigger::Activate, action, args, input: Some(input), seq } })
    }
}
//#endregion 🔖️TreeRoving
// #endregion events
