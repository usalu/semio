//! 🎛️ `shell-prefs` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl PrefsState {
    /// 💾️ Coalesces CHANGED preference fields for the shared I/O lane.
    ///
    /// 🩸️ The three arms above used to fire unconditionally at the end of EVERY chrome walk, so a
    /// settled shell asked the deferred I/O lane for five steps per frame that all had nothing to do.
    /// Each of those steps is a full `check_out_interaction("frame-deferred")` plus an async hand-back,
    /// and `FrameTransaction::step` refuses to run at all while the interaction state is out
    /// (`AppFrameTransactionStep::Superseded`) — so every frame build re-armed the lane, the lane took
    /// the state, the build was superseded, and the next build re-armed it again. Measured on the
    /// puzzle3d wgpu playground as ONE presented frame per boot: the canvas was byte-identical from
    /// t≈9 s to t≈45 s, and the concrete-forest GLB that becomes resident at t≈12 s could never reach
    /// a presented packet (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W5c). Maintenance is
    /// pressure-driven: no pressure, no checkout.
    pub(super) fn request_chrome_preferences_persist(&mut self, chrome: &mut ChromeState) {
        if chrome.chrome_present.last_synced_preferences.as_ref() != Some(&UiPrefsSnapshot::capture(chrome, self)) {
            chrome.chrome_present.maintenance.persist_requested = true;
        }
    }

    pub(super) fn advance_chrome_preferences_persist_step(&mut self, chrome: &mut ChromeState) {
        let phase = chrome.chrome_present.maintenance.persist_phase;
        let snapshot = UiPrefsSnapshot::capture(chrome, self);
        match phase {
            0 if chrome.chrome_present.last_synced_preferences.as_ref() != Some(&snapshot) => {
                persist_ui_preferences(chrome, self, chrome.chrome_present.last_synced_preferences.as_ref());
            }
            1 if chrome.chrome_present.last_synced_preferences.as_ref().map(|synced| synced.worker_count) != Some(snapshot.worker_count) => {
                stored_field_set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, &snapshot.worker_count.to_string());
            }
            0..=1 => {}
            _ => {
                chrome.chrome_present.last_synced_preferences = Some(snapshot);
                chrome.chrome_present.maintenance.persist_requested = false;
                chrome.chrome_present.maintenance.persist_phase = 0;
                return;
            }
        }
        chrome.chrome_present.maintenance.persist_phase = phase.saturating_add(1);
    }

    pub(crate) fn fullscreen_item_width(&self, atlas: &mut FontAtlas, theme: &Theme, chrome: &ChromeState) -> f32 {
        let is_de = self.locale_id == "de";
        let item = ChromeGroupItem {
            control_id: "ui.fullscreen.toggle",
            icon_id: Some(if chrome.fullscreen_active { "minimize-2" } else { "maximize-2" }),
            label: Some(if chrome.fullscreen_active { shell_chrome_string("fullscreen.exit", is_de) } else { shell_chrome_string("fullscreen.toggle", is_de) }),
            active: chrome.fullscreen_active,
            disabled: false,
            kind: HitKind::Toggle,
        };
        retained_chrome_group_item_width(atlas, theme, &item).unwrap_or(theme.control_height)
    }
}

//#endregion ShellChrome

