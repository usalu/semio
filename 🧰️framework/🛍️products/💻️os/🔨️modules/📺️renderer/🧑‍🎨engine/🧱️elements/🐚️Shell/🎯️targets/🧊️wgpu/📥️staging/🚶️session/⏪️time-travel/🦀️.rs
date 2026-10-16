//! 🚶️ `shell-session` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::time_travel::*;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};

//#region ⏪️TimeTravelControls
/// ⏱️ How often the native shell re-reads the history while the runtime replays or finalizes: the replay advances in
/// reactor turns that answer no dispatch, so the band's progress is polled rather than pushed.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const TIME_TRAVEL_POLL_MS: f64 = 150.0;

//#endregion ⏪️TimeTravelControls
//#region 🎯️TimeTravelFocus
/// 🧭️ What one change of the focused program's session asks of the chrome — React's `TimeTravelTransitionV1`, pinned for
/// both shells by the band corpus's `transitions`: `reveal` opens the History panel, `focus` moves keyboard focus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TimeTravelTransition {
    pub reveal: bool,
    pub focus: Option<TimeTravelFocus>,
    pub scroll_to: Option<String>,
}

/// ⏱️ How long a focus target may take to reach the screen before the move is given up — React retries across 30
/// animation frames.
pub(crate) const TIME_TRAVEL_FOCUS_SETTLE_MS: f64 = 1000.0;

/// 🧭️ The chrome's answer to the session moving from `previous` to `next` (`None` = no session) — React's
/// `timeTravelTransitionV1`: the edge into a new session reveals the History panel (whoever began it — a person, a chord
/// or an agent); a draft that starts (Begin, Next problem, another mutation) puts focus on its first input; a replay or
/// a review puts it on the band; the finalize prompt takes it; a progress step, a draft edit, the commit and the close
/// move nothing. The edge into a review that names its first blocking mutation reveals the panel again and names that
/// mutation's row (`scroll_to`) to bring into view.
pub(crate) fn time_travel_transition(previous: Option<&HistoryTimeTravel>, next: Option<&HistoryTimeTravel>) -> TimeTravelTransition {
    let Some(next) = next else { return TimeTravelTransition { reveal: false, focus: None, scroll_to: None } };
    let same = previous.is_some_and(|previous| previous.session_id == next.session_id);
    let moved = !same || previous.is_some_and(|previous| previous.stage != next.stage);
    let focus = match next.stage {
        HistoryTimeTravelStage::Editing => (moved || previous.and_then(|previous| previous.target.as_ref()) != next.target.as_ref()).then_some(TimeTravelFocus::Editor),
        HistoryTimeTravelStage::Replaying | HistoryTimeTravelStage::Reviewing => moved.then_some(TimeTravelFocus::Band),
        HistoryTimeTravelStage::Choosing => moved.then_some(TimeTravelFocus::Dialog),
        HistoryTimeTravelStage::Finalizing => None,
    };
    let problem = next.next_problem.as_ref().filter(|_| moved && next.stage == HistoryTimeTravelStage::Reviewing);
    TimeTravelTransition { reveal: !same || problem.is_some(), focus, scroll_to: problem.map(|problem| format!("{HISTORY_MUTATION_ROW_KEY_PREFIX}{}", problem.mutation_id)) }
}

//#endregion 🎯️TimeTravelFocus
impl<'ctx> SessionCtx<'ctx> {
    /// ⏪️ Takes the session status one history patch carries (absent = no session) and answers its transition
    /// ([`time_travel_transition`]): the edge into a new session reveals the History panel tab — or, below the mobile
    /// breakpoint, opens the one mobile panel on it — whose Rust-built body holds the editor, and every edge that names a
    /// focus target leaves it waiting for [`Self::resolve_time_travel_focus`]. A closed session drops a focus still waiting.
    pub(crate) fn observe_history_time_travel(&mut self, time_travel: Option<&HistoryTimeTravel>) {
        let transition = time_travel_transition(self.history.history_time_travel.as_ref(), time_travel);
        if let Some(focus) = transition.focus {
            self.history.time_travel_focus = Some((focus, chrome_now_ms() + TIME_TRAVEL_FOCUS_SETTLE_MS));
        } else if time_travel.is_none() {
            self.history.time_travel_focus = None;
        }
        self.history.history_time_travel = time_travel.cloned();
        if transition.reveal {
            self.dock.reveal_dock_tab(FRAMEWORK_PANEL_TAB_HISTORY_ID);
            if self.chrome.mobile_panel_active() {
                self.reveal_mobile_panel_tab(FRAMEWORK_PANEL_TAB_HISTORY_ID);
            }
        }
    }

    /// 📱️ Opens the one mobile panel on `tab_id` — the phone-width twin of the anchor reveal, since below the mobile
    /// breakpoint no anchor paints and the History body (the editor) lives in that panel.
    pub(super) fn reveal_mobile_panel_tab(&mut self, tab_id: &str) {
        let Some((_, path)) = self.dock.dock_tabs.locate(tab_id) else { return };
        self.dock.mobile_panel_path = mobile_panel_reconcile_path(&self.mobile_panel_tabs(), &path);
        self.dock.mobile_panel_visible = true;
    }

    /// 🧱️ The height of the subfooter — the shell's last layout row, under the footer, in which the session band, the
    /// reprojection band and the folder offer are stacked (React's `subfooter`); zero while none shows. The body and the footer
    /// end above it, so a band never covers content.
    pub(crate) fn subfooter_height(&self, theme: &Theme) -> f32 {
        let session = self.history.history_time_travel.as_ref().map(|status| self.prefs.time_travel_band_plan_for(status, theme, &self.chrome).band.y);
        let reprojection = self.prefs.history_reprojection_band_plan_for(theme, &self.chrome, &self.history).map(|(_, plan)| plan.band.y);
        let folder = self.docs.folder_reconnect_band_offer(&self.history, &self.sync).map(|offer| self.prefs.folder_reconnect_band_plan_for(&offer, theme, &self.chrome, &self.history).band.y);
        (self.chrome.screen_h - [session, reprojection, folder].into_iter().flatten().fold(self.chrome.screen_h, f32::min)).max(0.0)
    }

    /// ⏱️ While the runtime prepares, replays or finalizes a session, or a history change replays before adoption, re-reads the history at
    /// most every [`TIME_TRAVEL_POLL_MS`] and folds the snapshot — rows, cursor, session and reprojection status — so a reply older
    /// than it can no longer roll the bands back. The
    /// runtime also pushes throttled progress patches on uncorrelated `AppFrame::Invocation` frames, which reach
    /// [`ShellState::observe_invocation_history`] with the next exchange; this poll is what moves the band between two
    /// exchanges. `true` when the status changed.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn poll_time_travel_progress(&mut self) -> bool {
        let polling = self.history.history_time_travel.as_ref().is_some_and(|status| (matches!(status.stage, HistoryTimeTravelStage::Replaying | HistoryTimeTravelStage::Finalizing) || status.stage == HistoryTimeTravelStage::Editing && status.total.is_some())) || self.history.history_reprojection.as_ref().is_some_and(|reprojection| !reprojection.paused && reprojection.total > 0);
        let now = chrome_now_ms();
        if !polling || now - self.history.time_travel_polled_at_ms < TIME_TRAVEL_POLL_MS {
            return false;
        }
        self.history.time_travel_polled_at_ms = now;
        let Some((plugin, instance_id)) = self.docs.session.as_ref().and_then(|session| self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().map(|plugin| (plugin, session.instance_id))) else { return false };
        match plugin.read_history(instance_id).await {
            Ok(patch) if patch.cursor >= self.history.history_cursor => {
                let changed = patch.time_travel != self.history.history_time_travel || patch.reprojection != self.history.history_reprojection;
                fold_history_patch(&mut self.history.history_entries, &mut self.history.history_cursor, &patch, true);
                self.observe_history_time_travel(patch.time_travel.as_ref());
                self.history.observe_history_reprojection(patch.reprojection.as_ref());
                changed
            }
            Ok(_) => false,
            Err(error) => {
                debug_log(&format!("[TRACE] wgpu shell time-travel poll failed: {error}"));
                false
            }
        }
    }
}

