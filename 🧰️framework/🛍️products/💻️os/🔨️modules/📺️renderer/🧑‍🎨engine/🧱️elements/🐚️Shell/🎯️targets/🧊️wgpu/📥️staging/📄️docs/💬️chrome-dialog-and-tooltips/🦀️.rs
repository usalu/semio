//! 📄️ `shell-docs` staging of `chrome-dialog-and-tooltips`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl DocumentState {
    /// 🎓️ The currently active introduction step (if a tour is running and its index still resolves) —
    /// shared by every wgpu tour touchpoint beyond painting (reveal, advance-by-doing, keyboard) so they
    /// can never drift on what "the active step" means.
    pub fn chrome_tour_active_step(&self, chrome: &ChromeState) -> Option<semio_framework::onboarding::IntroductionStepDefinition> {
        let step_index = chrome.chrome_build.tour_state.as_ref().map(|state| state.step_index)?;
        self.chrome_tour_definition(chrome)?.steps.get(step_index).cloned()
    }

    /// 🎓️ The definition the running tour walks — the hub first-run tour's, or the open app's. The hub tour
    /// is the same overlay fed a second definition, never a second onboarding surface.
    pub fn chrome_tour_definition(&self, chrome: &ChromeState) -> Option<&semio_framework::onboarding::IntroductionDefinition> {
        match chrome.chrome_build.tour_state.as_ref()?.source {
            semio_renderer_wgpu_shell_chrome_state::ChromeTourSource::HubFirstRun => Some(semio_renderer_wgpu_hub_workspace::hub_first_run::hub_first_run_introduction()),
            semio_renderer_wgpu_shell_chrome_state::ChromeTourSource::App => self.session.as_ref()?.app.introduction.as_ref(),
        }
    }

    /// 🎓️🏷️ The key the running tour's answer is filed under: `os.hub` for the hub tour (React's
    /// `markHubFirstRunSeenV1`, never brand-scoped), the app's brand-scoped key otherwise.
    pub fn chrome_tour_seen_key(&self, chrome: &ChromeState) -> Option<String> {
        match chrome.chrome_build.tour_state.as_ref()?.source {
            semio_renderer_wgpu_shell_chrome_state::ChromeTourSource::HubFirstRun => Some(semio_renderer_wgpu_hub_workspace::hub_first_run::hub_first_run_introduction_id().to_string()),
            semio_renderer_wgpu_shell_chrome_state::ChromeTourSource::App => self.introduction_seen_key(),
        }
    }

    /// 🎓️ Advance-by-doing (Part B) — called from the single funnel points a user/plugin action can take
    /// (`dispatch_action`'s successful program forward, [`Self::set_active_utility`]'s activation branch) so
    /// a step's matching `Action`/`Utility` interaction completes the instant the described behavior
    /// actually happens, mirroring the React shell's own advance-by-doing wiring. No-operations when no tour is
    /// active or nothing in the active step's `interactions` matches what was performed.
    pub fn chrome_tour_note_action_performed(&mut self, action_id: &str, chrome: &mut ChromeState) {
        let Some(step) = self.chrome_tour_active_step(chrome) else {
            return;
        };
        self.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::onboarding::IntroductionInteractionKind::Action(action) if action.as_str() == action_id), chrome);
    }

    pub fn chrome_tour_note_utility_performed(&mut self, utility_id: &str, chrome: &mut ChromeState) {
        let Some(step) = self.chrome_tour_active_step(chrome) else {
            return;
        };
        self.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::onboarding::IntroductionInteractionKind::Utility(utility) if utility.as_str() == utility_id), chrome);
    }

    /// ✅️ Shared completion path for interaction-gated steps: finds the first not-yet-completed
    /// interaction matching `matches` (respecting `step.ordered` — only the next in-order interaction may
    /// complete), records it, and advances the step once every interaction is done. Mirrors the React
    /// shell's `completeIntroductionInteraction`.
    pub fn chrome_tour_complete_interaction(&mut self, step: &semio_framework::onboarding::IntroductionStepDefinition, matches: impl Fn(&semio_framework::onboarding::IntroductionInteractionKind) -> bool, chrome: &mut ChromeState) {
        if step.interactions.is_empty() {
            return;
        }
        let completed = chrome.chrome_build.tour_state.as_ref().map(|state| state.completed_interactions.clone()).unwrap_or_default();
        let Some(index) = step.interactions.iter().enumerate().find(|(i, interaction)| !completed.contains(i) && matches(&interaction.on)).map(|(i, _)| i) else {
            return;
        };
        if step.ordered && index != completed.len() {
            return;
        }
        let completed_len = {
            if let Some(tour) = chrome.chrome_build.tour_state.as_mut() {
                if !tour.completed_interactions.contains(&index) {
                    tour.completed_interactions.push(index);
                }
                tour.completed_interactions.len()
            } else {
                0
            }
        };
        if completed_len >= step.interactions.len() {
            self.chrome_tour_advance_current_step(step, chrome);
        }
    }

    pub fn chrome_tour_advance_current_step(&mut self, step: &semio_framework::onboarding::IntroductionStepDefinition, chrome: &mut ChromeState) {
        let Some(intro) = self.chrome_tour_definition(chrome) else {
            return;
        };
        let step_index = intro.steps.iter().position(|candidate| candidate.id == step.id).unwrap_or(0);
        let step_count = intro.steps.len();
        if step_index + 1 >= step_count {
            if let Some(seen_key) = self.chrome_tour_seen_key(chrome) {
                chrome.chrome_build.mark_introduction_seen(&seen_key);
            }
        }
        chrome.chrome_build.advance_introduction(step_count);
    }
}

//#endregion ShellChrome

