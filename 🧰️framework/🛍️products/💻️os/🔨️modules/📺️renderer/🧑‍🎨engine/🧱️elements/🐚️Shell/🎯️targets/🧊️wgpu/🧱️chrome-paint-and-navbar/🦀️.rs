//! 🧱️ Chrome paint steps, navbar/footer layout, utilities and window pane chrome.
//!
//! Immediate-mode chrome text and icons, group painting, bands, navbar/footer layouts, the sync pill, hub
//! projections, utility rendering, keybinding formatting, navbar controls, World3d status, the safe area and the
//! window pane chrome.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;



#[cfg(test)]
fn partition_utilities_by_category(utilities: &[UtilityNode]) -> [Vec<UtilityNode>; 4] {
    let mut buckets: [Vec<UtilityNode>; 4] = [vec![], vec![], vec![], vec![]];
    for utility in utilities {
        let idx = match utility.category() {
            semio_framework_ui_layout::utilities::UtilityCategory::Selection => 0,
            semio_framework_ui_layout::utilities::UtilityCategory::Utilities => 1,
            semio_framework_ui_layout::utilities::UtilityCategory::History => 2,
            semio_framework_ui_layout::utilities::UtilityCategory::Sync => 3,
        };
        buckets[idx].push(utility.clone());
    }
    buckets
}



//#region ActionPanelAndUtilities

/// 📇️ The first window kind whose resolved actions include `action_id` — the window the palette/keybinding
/// redirect focuses to open an arg-carrying action's form (Architecture Decision 8, P3/P4).
// 🎯️ Exercised today only by `🛰️Dock`'s own `action_host_window_id_finds_scoping_window` test — the
// palette/keybinding redirect call site itself hasn't landed yet, hence the matching test-only cfg.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn action_host_window_id(app: &AppDefinition, action_id: &str) -> Option<String> {
    app.window_kinds.iter().find(|kind| semio_framework::resolve_window_actions(app, kind).iter().any(|action| action.id == action_id)).map(|kind| kind.id.clone())
}

//#region 🔀️ChromeParity

/// 👁️✏️🔁️ **Dedup note**: the surface-role (`mod+alt+e`/`mod+alt+v`) and mode-cycle
/// (`mod+alt+→`/`mod+alt+←`) chords used to be hand-rolled predicates run AHEAD of the shortcut
/// table, which made both rows unremappable by construction (an override could add a chord but never
/// retire the hardcoded one) and matched a chord held with Shift that React's own
/// `keyboardEventMatchesChord` refuses. They are now ordinary [`SHELL_SHORTCUT_ROWS`] rows resolved
/// by [`shell_shortcut_for_in`] like every other verb.

//#endregion 🔀️ChromeParity

//#region 🪟️WindowPaneChrome
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs"]
mod navbar_footer_parity_tests;

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs"]
mod shell_chrome_parity_tests;
