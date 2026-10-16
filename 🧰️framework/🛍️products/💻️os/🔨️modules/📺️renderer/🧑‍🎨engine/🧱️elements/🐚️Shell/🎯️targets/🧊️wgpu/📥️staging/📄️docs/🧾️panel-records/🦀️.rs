//! 📄️ `shell-docs` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🖥️ React's own empty/unavailable section for a Display leaf with nothing to show
/// (`buildDisplayWindowsTree`'s `framework.display.windows.empty` and the host-less
/// `{ sections: [{ id: "unavailable", … }] }` fallback, folded into one row here because this
/// renderer never has a "no host" state distinct from "no window kinds").
pub(crate) fn display_unavailable_section(id: &str, is_de: bool) -> UiTreeSectionNode {
    UiTreeSectionNode {
        header_toolbar: None,
        id: id.to_string(),
        label: None,
        default_open: Some(true),
        presence: UiPresence::default(),
        items: vec![UiTreeItemNode::base(format!("{id}.row"), Label::data(shell_chrome_string("display.unavailable", is_de)))],
        window: None,
    }
}

/// 🔄️ The `createWorldProjectionTemplates` taxonomy as nested Display rows, reconstructed from
/// [`WORLD_PROJECTION_TEMPLATES`]' own `depth` column: a row's id is its parent's id plus its template
/// id, composed through the element-id grammar exactly as React's `childElementId` composes it
/// (`framework.display.windows.puzzle3dMain.projection.parallel.axonometric.axonometricIsometric`).
/// `cursor` walks the flat table once; `depth` is the level this call materialises.
/// Each level is pre-reversed for the bottom-anchored Display tree's up-flow layout.
///
/// 🛡️ Unreachable for the declared table (a child always follows its own parent), and a
/// skip rather than a panic so a future row ordering cannot wedge the panel.
pub(crate) fn world_projection_template_rows(prefix: &str, window_kind_id: &str, depth: u8, cursor: &mut usize) -> Vec<UiTreeItemNode> {
    let mut rows = Vec::new();
    while let Some(template) = WORLD_PROJECTION_TEMPLATES.get(*cursor) {
        if template.depth < depth {
            break;
        }
        if template.depth > depth {
            *cursor += 1;
            continue;
        }
        let id = semio_framework::child_element_id(prefix, &[template.id]);
        *cursor += 1;
        let children = world_projection_template_rows(&id, window_kind_id, depth + 1, cursor);
        rows.push(UiTreeItemNode {
            id: id.clone(),
            label: Label::data(template.label),
            icon_id: IconName::from_str(template.icon_id),
            default_open: Some(false),
            draggable: Some(true),
            drag_data: Some(window_template_drag_data(window_kind_id, Some(template.id))),
            items: (!children.is_empty()).then_some(children),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
    }
    rows.reverse();
    rows
}

/// 🌳️ One React `TreeDataItem` property row with its Select in the inline control cell.
pub(crate) fn settings_tree_select_item(id: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str) -> UiTreeItemNode {
    settings_tree_select_item_with(id, label, value, options, action, None)
}

//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle

