//! 📄️ `shell-docs` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl DocumentState {
    /// 🎓️ Coalesces one bounded introduction lookup for the shared I/O lane, against the BRAND-SCOPED
    /// key (see [`Self::introduction_seen_key`]).
    pub fn request_introduction_read(&mut self, chrome: &mut ChromeState) {
        if chrome.chrome_present.maintenance.introduction_read.is_some() {
            return;
        }
        let Some(seen_key) = self.introduction_seen_key() else { return };
        if seen_key.len() <= SHELL_CHROME_IO_FIELD_BYTES && !chrome.chrome_build.introduction_seen.contains_key(&seen_key) {
            chrome.chrome_present.maintenance.introduction_read = Some(seen_key);
        }
    }

    /// 🎓️🏷️ The key this boot's introduction-seen flag is filed under — React's `introductionSeenKey`
    /// (`🏛️ShellHost/🟦️.tsx:4159`): `"<brandId>:<appId>"` on a branded shell, the bare app id otherwise,
    /// so the BRANDED tour still plays on a device that already saw the unbranded one.
    ///
    /// 🩸️ What this replaces: the bare `session.app.id`, every time. The wgpu shell carried a brand ID
    /// and nothing else (`📓️w1d` gap 1), so on `aggregator` (6023/6123) it wrote
    /// `ui.introduction.seen.<appId>` where React writes `ui.introduction.seen.<brandId>:<appId>` — the
    /// one concretely demonstrated brand divergence (`📓️w5a-browser-prefs-persistence.md` §7.1).
    pub fn introduction_seen_key(&self) -> Option<String> {
        let app_id = self.session.as_ref().map(|session| session.app.id.clone())?;
        Some(match semio_renderer_wgpu_host_base::boot_brand_id() {
            Some(brand_id) => format!("{brand_id}:{app_id}"),
            None => app_id,
        })
    }

    /// 🎓️ Arms the open app's introduction the first time it launches on this device, the wgpu twin of
    /// `🏛️ShellHost/🟦️.tsx:4139-4166`'s auto-start effect. It runs from the ONE place the answer to
    /// "has this app been seen" becomes known — the `ui.introduction.seen.<appId>` read landing — so it
    /// can neither arm before the flag is readable nor re-arm afterwards: the read is requested only
    /// while `introduction_seen` lacks the key, and the key is written here.
    ///
    /// 🩸️ What this replaces: NOTHING called `start_introduction`. It was `#[cfg(test)]`, so the whole
    /// `🔖️ChromeOverlaysAndTour` tour — veil, info box, advance-by-doing, reveal, keyboard — was
    /// reachable only from its own unit tests, and every wgpu boot painted no tour at all where React
    /// painted `Welcome to Puzzle 3D`.
    ///
    /// 🏷️ React's `replayIntroductionOnLoad` brand flag ignores the device-local seen flag entirely,
    /// which is exactly what a kiosk brand needs. Carried by the brand ROW since this packet.
    pub fn auto_start_introduction(&mut self, seen_key: &str, seen: bool, chrome: &mut ChromeState, tutorial: &TutorialState) {
        if seen_key == semio_renderer_wgpu_hub_workspace::hub_first_run::hub_first_run_introduction_id() {
            return;
        }
        let has_introduction = self.session.as_ref().is_some_and(|session| session.app.introduction.is_some());
        let replay_on_load = semio_renderer_wgpu_host_base::boot_brand().replays_introduction();
        if !should_auto_start_introduction(seen_key, has_introduction, tutorial.tutorial.is_some(), seen, replay_on_load, semio_renderer_wgpu_host_base::host_introduction_suppressed()) {
            return;
        }
        chrome.chrome_build.start_introduction();
    }

    /// 🎓️ Arms and tends the hub first-run tour, the wgpu twin of `HubWorkspace`'s `tour` state: opening the
    /// hub surface reads the `os.hub` seen flag once and, when unseen and nothing else owns the overlay,
    /// starts the tour at the step the live hub state derives; closing the surface ends a running tour
    /// without answering it. A step completes by doing what it teaches, on the false-to-true edge.
    pub fn reconcile_hub_first_run(&mut self, chrome: &mut ChromeState, directory: &DirectoryState, tutorial: &TutorialState) {
        use semio_renderer_wgpu_hub_workspace::hub_first_run::{hub_first_run_introduction_id, hub_first_run_stage_done, hub_first_run_state_from, hub_first_run_step_index, HubFirstRunStage};
        use semio_renderer_wgpu_shell_chrome_state::ChromeTourSource;
        let workspace = &directory.hub_workspace;
        let rows: Vec<(&str, &str)> = workspace.rows.iter().map(|row| (row.id.as_str(), row.access.as_str())).collect();
        let state = hub_first_run_state_from(workspace.session.phase.as_str(), &rows, workspace.open_space_id.as_deref());
        let hub_tour = chrome.chrome_build.tour_state.as_ref().is_some_and(|tour| tour.source == ChromeTourSource::HubFirstRun);
        if !directory.hub_workspace_open {
            chrome.chrome_build.hub_first_run_latch = false;
            if hub_tour {
                chrome.chrome_build.skip_introduction();
            }
            return;
        }
        if hub_tour {
            let invite_ready = workspace.invite_capability.is_some();
            let advance = chrome.chrome_build.tour_state.as_mut().and_then(|tour| {
                let stage = *HubFirstRunStage::ALL.get(tour.step_index)?;
                let done = hub_first_run_stage_done(stage, &state, invite_ready);
                let edge = match tour.entry_done {
                    Some((index, at_entry)) if index == tour.step_index => done && !at_entry,
                    _ => {
                        tour.entry_done = Some((tour.step_index, done));
                        false
                    }
                };
                edge.then_some(())
            });
            if advance.is_some() {
                if let Some(step) = self.chrome_tour_active_step(chrome) {
                    self.chrome_tour_advance_current_step(&step, chrome);
                }
            }
            return;
        }
        if chrome.chrome_build.hub_first_run_latch {
            return;
        }
        let id = hub_first_run_introduction_id();
        let Some(seen) = chrome.chrome_build.introduction_seen.get(id).copied() else {
            if chrome.chrome_present.maintenance.introduction_read.is_none() {
                chrome.chrome_present.maintenance.introduction_read = Some(id.to_string());
            }
            return;
        };
        chrome.chrome_build.hub_first_run_latch = true;
        if seen || chrome.chrome_build.tour_is_open() || tutorial.tutorial.is_some() || semio_renderer_wgpu_host_base::host_introduction_suppressed() {
            return;
        }
        chrome.chrome_build.start_hub_first_run(hub_first_run_step_index(&state));
    }

    /// 🎓️ Reconciles a live host-policy transition without marking an unseen introduction as answered.
    pub fn reconcile_host_introduction_policy(&mut self, chrome: &mut ChromeState, tutorial: &TutorialState) {
        let suppressed = semio_renderer_wgpu_host_base::host_introduction_suppressed();
        if chrome.chrome_build.introduction_host_suppressed == suppressed {
            return;
        }
        chrome.chrome_build.introduction_host_suppressed = suppressed;
        if suppressed {
            chrome.chrome_build.skip_introduction();
        } else if let Some(key) = self.introduction_seen_key() {
            let seen = chrome.chrome_build.introduction_was_seen(&key);
            self.auto_start_introduction(&key, seen, chrome, tutorial);
        }
    }

    pub fn advance_chrome_preferences_load_step(&mut self, chrome: &mut ChromeState, prefs: &mut PrefsState) {
        let phase = chrome.chrome_present.maintenance.load_phase;
        match phase {
            0 => {
                let preferences = read_ui_preferences();
                let custom_themes = custom_themes_from(&preferences);
                prefs.appearance_id = resolve_appearance_id(&preferences);
                prefs.locale_id = resolve_locale_id(env_lock("SEMIO_LOCKED_LOCALE"), &preferences, prefs.active_locale());
                semio_renderer_wgpu_ui_engine::retained_ui_engine::install_ui_engine_locale(prefs.active_locale());
                prefs.terminology_id = env_lock("SEMIO_LOCKED_TERMINOLOGY").or(preferences.terminology).unwrap_or_else(|| prefs.active_terminology().as_str().to_string());
                prefs.driver_id = preferences.driver_id.unwrap_or_else(|| "default".to_string());
                chrome.chrome_build.preferences.custom_drivers = preferences.custom_drivers;
                chrome.chrome_build.driver = resolve_ui_driver_chrome(&prefs.driver_id, &chrome.chrome_build.preferences.custom_drivers);
                semio_renderer_wgpu_ui_engine::retained_ui_engine::set_ui_engine_driver(chrome.chrome_build.driver);
                chrome.chrome_build.preferences.ui_layout = match preferences.layout {
                    Some(OsUiChromeLayout::Tablet) => "tablet",
                    _ => "desktop",
                }
                .to_string();
                chrome.chrome_build.preferences.theme_id = env_lock("SEMIO_LOCKED_THEME").or(preferences.theme_id).unwrap_or_else(|| "semio".to_string());
                chrome.chrome_build.preferences.custom_themes = custom_themes;
                chrome.chrome_build.preferences.keybinding_overrides = preferences.keybinding_overrides;
                self.trace_resolved_locale("preferences", chrome, prefs);
                let loaded = chrome.chrome_build.preferences.clone();
                with_chrome_prefs(|current| *current = loaded);
            }
            1 => {
                chrome.chrome_build.preferences.worker_count = stored_field_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).and_then(|raw| raw.parse::<u32>().ok()).filter(|count| *count >= 1).unwrap_or_else(default_compute_worker_count);
            }
            _ => {
                chrome.chrome_present.preferences_loaded = true;
                chrome.chrome_present.maintenance.load_requested = false;
                chrome.chrome_present.maintenance.load_phase = 0;
                chrome.chrome_present.last_synced_preferences = Some(capture_ui_prefs_snapshot(chrome, prefs));
                return;
            }
        }
        chrome.chrome_present.maintenance.load_phase = phase.saturating_add(1);
    }

    pub fn has_display_tabs(&self) -> bool {
        self.session.as_ref().is_some_and(|s| !s.app.window_kinds.is_empty())
    }
}

//#endregion ShellChrome

