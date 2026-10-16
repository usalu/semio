//! ⚓️ `shell-dock` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🖥️ React's `groupNamedLayoutsToTreeItems`: a layout with no `groupPath` is a root row, one with
/// a path folds into `framework.display.layout.group.<a/b>` folders minted on first use. A user
/// layout additionally carries React's `framework.display.delete.<id>` action — a row CONTROL here,
/// because this renderer's panel tree projects a row's control and not its `actions` menu.
pub(crate) fn named_layout_rows(layouts: &[ui_wgpu::wgpu::NamedLayout], deletable: bool, is_de: bool) -> Vec<UiTreeItemNode> {
    let mut root: Vec<UiTreeItemNode> = Vec::new();
    for entry in layouts {
        let leaf = UiTreeItemNode {
            id: format!("framework.display.layout.{}", entry.id),
            label: Label::data(entry.label.clone()),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: "applyNamedLayout".into(), args: crate::action_args_json!({ "layoutId": entry.id.clone() }) }),
            control: deletable.then(|| {
                ui_wgpu::wgpu::component::ui::UiControlNode::Button(UiButtonNode {
                    id: Some(format!("framework.display.delete.{}", entry.id)),
                    icon_id: IconName::Trash2,
                    label: Label::data(shell_chrome_string("display.deleteLayout", is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: "deleteUserLayout".into(), args: crate::action_args_json!({ "layoutId": entry.id.clone() }) },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                })
            }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        };
        let path = entry.group_path.clone().unwrap_or_default();
        place_named_layout_row(&mut root, &path, &mut String::new(), leaf);
    }
    root
}

/// 🖥️ One layout row into its folder chain, minting a `framework.display.layout.group.<a/b>`
/// folder on first use — the recursive half of {@link named_layout_rows}, kept separate so the
/// walk owns one `&mut Vec` at a time.
fn place_named_layout_row(rows: &mut Vec<UiTreeItemNode>, path: &[String], path_key: &mut String, leaf: UiTreeItemNode) {
    let Some((segment, rest)) = path.split_first() else {
        rows.push(leaf);
        return;
    };
    if !path_key.is_empty() {
        path_key.push('/');
    }
    path_key.push_str(segment);
    let folder_id = format!("framework.display.layout.group.{path_key}");
    if !rows.iter().any(|row| row.id == folder_id) {
        rows.push(UiTreeItemNode { id: folder_id.clone(), label: Label::data(segment.clone()), default_open: Some(false), items: Some(Vec::new()), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) });
    }
    let folder = rows.iter_mut().find(|row| row.id == folder_id).expect("the folder row was just ensured");
    place_named_layout_row(folder.items.get_or_insert_with(Vec::new), rest, path_key, leaf);
}

//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle

