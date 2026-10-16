//! 🎛️ `shell-prefs` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
pub fn display_panel_body(panel_id: &str, sections: Vec<UiTreeSectionNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "column".into(),
        gap: None,
        padding: None,
        id: Some(panel_id.to_string()),
        children: vec![UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })],
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        menu: None,
    })
}

/// ⚙️ One collapsible settings section — React's `TreeDataSection` with `defaultOpen: true`.
pub fn settings_section(id: &str, label: &str, children: Vec<UiNode>) -> UiNode {
    UiNode::Section(UiSectionNode { id: id.to_string(), label: Some(Label::data(label)), default_open: Some(true), presence: UiPresence::default(), menu: None, children })
}

/// ⚙️ One control-less text row — React's `TreeDataItem` carrying only a `label`.
pub fn settings_text_row(text: &str) -> UiNode {
    UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(text), emphasize: Some(false), data_attributes: None, menu: None })
}

/// ⚙️ {@link settings_select_row} plus authored action args — the row's own coordinate, for a table
/// whose action needs to know WHICH row changed (the default-apps `(dialect × role)` pairing).
pub fn settings_select_row_with(id: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str, args: Option<DslValue>) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: format!("{id}.field"),
        label: Label::data(label),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Select(UiSelectNode {
            presence: UiPresence::default(),
            id: id.to_string(),
            value: value.to_string(),
            items: options.into_iter().map(|(value, label)| UiSelectItem { value, label: Label::data(label) }).collect(),
            placeholder: None,
            on_change: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    })
}

/// 🌳️ One inline settings Select whose action keeps its authored row arguments.
pub fn settings_tree_select_item_with(id: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str, args: Option<DslValue>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.to_string(),
        label: Label::data(label),
        control: Some(UiControlNode::Select(UiSelectNode {
            presence: UiPresence::default(),
            id: id.to_string(),
            value: value.to_string(),
            items: options.into_iter().map(|(value, label)| UiSelectItem { value, label: Label::data(label) }).collect(),
            placeholder: None,
            on_change: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            menu: None,
        })),
        ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
    }
}

//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle

