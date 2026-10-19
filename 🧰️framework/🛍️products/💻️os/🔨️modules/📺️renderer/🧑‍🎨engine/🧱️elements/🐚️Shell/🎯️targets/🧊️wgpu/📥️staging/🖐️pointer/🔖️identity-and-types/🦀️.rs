//! 🖐️ `shell-pointer` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

/// 🗂️ `ui_wgpu::wgpu::ShellMenuAction.kind` wire string for an `ActionDefinition.kind` — host-side styling
/// parity only, unused by `build_shell_context_menu_specs` itself.
pub fn context_menu_action_kind_str(kind: semio_framework::ActionKind) -> String {
    match kind {
        semio_framework::ActionKind::Mutation => "mutation",
        semio_framework::ActionKind::View => "view",
        semio_framework::ActionKind::History => "history",
        semio_framework::ActionKind::Clipboard => "clipboard",
        semio_framework::ActionKind::Shell => "shell",
        semio_framework::ActionKind::Interaction => "interaction",
    }
    .to_string()
}

/// 🖱️ Maps an on-demand plugin context-menu spec into the wgpu shell menu row — `menu.group.<category>`
/// rows (D5's `organize_context_menu` folds, see `ui_wgpu::wgpu::ContextMenuOrganizer`) resolve their label via
/// `context_menu_group_label` (falling back to the spec's own label if the category is unrecognized) and get
/// a default folder icon when the spec left `icon` unset.
///
/// 🗂️ `context_menu_group_label`, not `ribbon_parent_label`: the overflow row `organize_context_menu`
/// synthesizes (`menu.group.more`) is the one group id outside the closed 20-id ribbon-parent taxonomy,
/// so resolving through the taxonomy alone rendered it with an EMPTY label. Its React twin resolves the
/// same id through `ui.contextMenu.more` (`🛠️ShellHelpers/🟦️.tsx`'s `contextMenuGroupLabel`).
pub fn shell_context_menu_item_from_spec(spec: semio_framework_ui_declaration::ui::ContextMenuItemSpec, controller_id: &str, is_de: bool) -> ContextMenuItem {
    let semio_framework_ui_declaration::ui::ContextMenuItemSpec { id, label, icon, shortcut, disabled, reason, separator, checked, destructive, action, args, children, .. } = spec;
    let is_group_row = id.starts_with(semio_framework_ui_declaration::ui::CONTEXT_MENU_GROUP_ID_PREFIX);
    let label = semio_framework_ui_declaration::ui::context_menu_group_label(&id, is_de).map(str::to_string).or(label);
    let icon = icon.or_else(|| is_group_row.then(|| "folder".to_string()));
    ContextMenuItem {
        id,
        label: label.unwrap_or_default(),
        icon,
        shortcut,
        destructive: destructive.unwrap_or(false),
        action: action.map(|action| ActionDescriptor { controller_id: controller_id.into(), action, args }),
        children: children.unwrap_or_default().into_iter().map(|child| shell_context_menu_item_from_spec(child, controller_id, is_de)).collect(),
        disabled: disabled.unwrap_or(false),
        reason,
        separator: separator.unwrap_or(false),
        checked: checked.unwrap_or(false),
    }
}

pub fn scope_context_menu_items(items: &mut [ContextMenuItem], window_id: &str) {
    for item in items {
        if let Some(action) = item.action.as_mut() {
            scope_action_to_window(action, window_id);
        }
        scope_context_menu_items(&mut item.children, window_id);
    }
}

//#region ShellTypes
/// ⌨️ The shell's OWN overlay query fields. Keyboard focus on one of these is the shell typing into
/// its own chrome, NOT the user typing into app content, so the hardcoded shell chords (and the
/// focused-input Enter/Escape commit) must look straight through them — otherwise the chord that
/// opens the quick-search palette disables every route out of it, including its own toggle and
/// Escape (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const SHELL_OVERLAY_INPUT_IDS: [&str; 2] = ["ui.search.input", "ui.find.input"];

//#endregion ShellTypes

