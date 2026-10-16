//! 📒️ `shell-directory` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};

//#region 🗣️TimeTravelCopy
pub const BAND_LABEL_PEER_EDITING_TARGET: &str = "ui.timeTravel.peer.editingTarget";

pub const BAND_LABEL_PEER_EDITING_HISTORY: &str = "ui.timeTravel.peer.editingHistory";

pub const BAND_LABEL_PEER_EDITING_ROW: &str = "ui.timeTravel.peer.editingRow";

//#endregion 🗣️TimeTravelCopy
//#region 👥️PeerHistoryEdits
/// ⏪️ The badge a peer's roster row wears while that peer edits the history — React's `timeTravelPeerPresenceV1`.
pub const TIME_TRAVEL_PEER_BADGE: &str = "⏪";

/// 🗣️ `ui.timeTravel.peer.editingTarget`, or `.editingHistory` without a target.
fn peer_editing_text(name: &str, target: Option<&str>, tongue: BandTongue) -> String {
    match target {
        Some(target) => band_label(BAND_LABEL_PEER_EDITING_TARGET, tongue).replace("{name}", name).replace("{target}", target),
        None => band_label(BAND_LABEL_PEER_EDITING_HISTORY, tongue).replace("{name}", name),
    }
}

/// 🗣️ `ui.timeTravel.peer.editingRow`.
fn peer_editing_row_text(name: &str, tongue: BandTongue) -> String {
    band_label(BAND_LABEL_PEER_EDITING_ROW, tongue).replace("{name}", name)
}

/// 👥️ What the shell shows about peers' open history edits: each editing peer's roster activity by actor, and the
/// notes by history-body node key, sorted by key with one key's lines joined by ` · `.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TimeTravelPeerPresence {
    pub activities: Vec<(String, ui_wgpu::wgpu::PresenceActivity)>,
    pub notes: Vec<(String, String)>,
}

/// 👥️ Peers' open history edits labelled from THIS replica's own history rows — React's `timeTravelPeerPresenceV1`
/// (the wire carries only the mutation id, the stage and the draft count, never locale text). `editing` is each editing
/// peer as `(actor, name, mutation id)`: its chip reads "Ada is editing Drag selection in time travel" (the history in
/// general when the mutation is not among the local rows), and the mutation's node and its history row get a note
/// naming who edits them.
pub fn time_travel_peer_presence<'a>(editing: impl IntoIterator<Item = (&'a str, &'a str, &'a str)>, entries: &BTreeMap<String, semio_framework::kernel::HistoryEntry>, terminology: Terminology, tongue: impl Into<BandTongue>) -> TimeTravelPeerPresence {
    let tongue: BandTongue = tongue.into();
    let locale = tongue.locale;
    let mut notes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let activities = editing
        .into_iter()
        .map(|(actor, name, mutation_id)| {
            let row = entries.values().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == mutation_id));
            let target = row.and_then(|entry| entry.mutations.iter().find(|mutation| mutation.mutation_id == mutation_id)).map(|mutation| mutation.label.resolve(terminology, locale).to_string()).filter(|target| !target.is_empty());
            let note = peer_editing_row_text(name, tongue);
            if let Some(entry) = row {
                notes.entry(format!("{HISTORY_ROW_KEY_PREFIX}{}", entry.seq)).or_default().push(note.clone());
            }
            notes.entry(format!("{HISTORY_MUTATION_ROW_KEY_PREFIX}{mutation_id}")).or_default().push(note);
            (actor.to_string(), ui_wgpu::wgpu::PresenceActivity { text: peer_editing_text(name, target.as_deref(), tongue), badge: TIME_TRAVEL_PEER_BADGE.to_string() })
        })
        .collect();
    TimeTravelPeerPresence { activities, notes: notes.into_iter().map(|(key, lines)| (key, lines.join(" · "))).collect() }
}

//#endregion 👥️PeerHistoryEdits
impl DirectoryState {
    /// 👥️ The open history edits of the peers on the roster this shell paints (the attached surface's, like
    /// `footer_presence_rows`), labelled from this replica's history rows on the shell's axes.
    pub fn peer_time_travel_presence(&self, chrome: &ChromeState, history: &HistoryState, prefs: &PrefsState) -> TimeTravelPeerPresence {
        let Some(surface) = self.presence_surface.as_deref() else { return TimeTravelPeerPresence::default() };
        let editing = self.presence_peers.iter().filter(|peer| peer.surface.as_deref() == Some(surface)).filter_map(|peer| Some((peer.actor.as_str(), peer.label.as_deref().unwrap_or(peer.actor.as_str()), peer.history_edit.as_ref()?.mutation_id.as_str())));
        time_travel_peer_presence(editing, &history.history_entries, prefs.active_terminology(), prefs.band_tongue(chrome))
    }

    /// 👥️ Hands the retained history body the peers' row notes — React's shell-root presence overlay. Runs every frame;
    /// with no peer editing it is one empty comparison per window. `true` when a window's notes changed.
    pub fn publish_peer_time_travel_notes(&self, chrome: &ChromeState, history: &HistoryState, prefs: &PrefsState) -> bool {
        semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::set_ui_presence_notes(&self.peer_time_travel_presence(chrome, history, prefs).notes)
    }
}

//#region 👥️PeerHistoryEdits
pub const HISTORY_MUTATION_ROW_KEY_PREFIX: &str = "framework.history.mutation.";

/// 🗝️ The node keys the framework history body gives a history row and a mutation child (`🔌️plugin/🦀️.rs`
/// `ui_history_panel`), which a peer's open history edit marks — React's `HISTORY_ROW_KEY_PREFIX` and
/// `HISTORY_MUTATION_ROW_KEY_PREFIX`.
pub const HISTORY_ROW_KEY_PREFIX: &str = "framework.history.entry.";
//#endregion 👥️PeerHistoryEdits
