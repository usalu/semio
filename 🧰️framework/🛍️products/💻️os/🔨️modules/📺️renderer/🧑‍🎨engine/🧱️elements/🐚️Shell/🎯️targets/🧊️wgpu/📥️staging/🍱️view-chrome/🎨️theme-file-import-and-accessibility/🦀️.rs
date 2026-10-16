//! 🍱️ `shell-view-chrome` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ♿️ChromeAccessibleNames

/// 🔘️ Captures the painted pressed state of chrome buttons beside their accepted-frame label.
pub fn note_chrome_group_item(item: &ChromeGroupItem<'_>) {
    note_chrome_control_name(item.control_id, item.label);
    if item.kind == HitKind::PanelTab || item.control_id == "ui.fullscreen.toggle" || item.kind == HitKind::Toggle && item.control_id.starts_with("framework.utility.toggle.") {
        with_chrome_control_names(|names| {
            if let Some(presentation) = names.get_mut(item.control_id) {
                presentation.role = Some("button");
                presentation.pressed = Some(item.active);
            }
        });
    }
}

/// 🗣️ Localizes Dock actions while their title and pointer target belong to the same painted frame.
pub fn note_dock_control_name(control_id: &str, name: semio_framework_os_renderer_wgpu_dock::DockControlName<'_>, german: bool) {
        use semio_framework_os_renderer_wgpu_dock::DockControlName;
    let label = match name {
        DockControlName::Body(title) | DockControlName::Tab { title, .. } => title.to_string(),
        DockControlName::Focus => shell_chrome_string("common.focus", german).to_string(),
        DockControlName::Unfocus => shell_chrome_string("common.unfocus", german).to_string(),
        DockControlName::Close => shell_chrome_string("common.close", german).to_string(),
        DockControlName::Drag(title) => shell_chrome_string("tree.drag.sortTarget", german).replace("{{target}}", title),
    };
    note_chrome_control_name(control_id, Some(&label));
    with_chrome_control_names(|names| {
        if let Some(presentation) = names.get_mut(control_id) {
            match name {
                DockControlName::Body(_) => presentation.role = Some("tabpanel"),
                DockControlName::Tab { selected, panel, .. } => {
                    presentation.role = Some("tab");
                    presentation.selected = Some(selected);
                    presentation.controls = Some(panel.to_string());
                }
                _ => {}
            }
        }
    });
}

//#endregion ♿️ChromeAccessibleNames

