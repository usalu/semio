//! 🗨️ `shell-view-overlays` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ♿️ChromeAccessibleNames

/// 🎛️ Records a dialog control's role, pressed state and value (a slider's range and spoken value, a spin
/// button's text) beside its painted name.
pub fn note_chrome_control_semantics(control_id: &str, semantics: &ChromeDialogSemantics) {
    with_chrome_control_names(|names| {
        if let Some(presentation) = names.get_mut(control_id) {
            presentation.role = semantics.role.or(presentation.role);
            presentation.pressed = semantics.pressed.or(presentation.pressed);
            presentation.value_text.clone_from(&semantics.value_text);
            presentation.value_range = semantics.value_range;
        }
    });
}

/// 🔊️ One non-interactive chrome status node: named, politely live, never focusable nor actionable.
pub fn chrome_status_accessibility_node(node_id: u64, key: &str, label: String) -> ui_contract::AccessibilityProjectionNode {
    ui_contract::AccessibilityProjectionNode {
        value_step: None,
        invalid: false,
        set_size: None,
        pos_in_set: None,
        tone: None,
        node_id,
        key: key.to_string(),
        role: "status".to_string(),
        depth: 0,
        label: Some(label),
        description: None,
        live: ui_contract::liveness_name(ui_contract::Liveness::Polite).to_string(),
        shortcut: None,
        hidden: false,
        disabled: false,
        focusable: false,
        tabbable: false,
        actionable: false,
        focused: false,
        checked: None,
        pressed: None,
        selected: None,
        expanded: None,
        editable: false,
        multiline: false,
        controls: None,
        active_descendant: None,
        level: None,
        rect: None,
        value_min: None,
        value_max: None,
        value_now: None,
        value_text: None,
        busy: false,
    }
}

//#endregion ♿️ChromeAccessibleNames

