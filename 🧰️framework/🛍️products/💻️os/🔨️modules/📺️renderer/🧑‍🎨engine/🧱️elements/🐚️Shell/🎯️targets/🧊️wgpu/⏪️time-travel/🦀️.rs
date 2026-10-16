//! ⏪️ The wgpu shell's time-travel chrome (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, packet W2-C).
//!
//! `HistoryPatch.timeTravel` is the whole input. While it is present the shell paints one persistent band above the
//! footer (stage, the edited mutation, replay progress with Cancel, the worst outcome in words, what blocks finalizing,
//! the accepted drafts, the last fault and the controls the stage offers), an indicator on every window pane saying
//! the pane shows time-travel state, and answers the remappable `ui.timeTravel.*` chords. Every control dispatches the
//! framework's reserved `historyEdit*` verb on the session controller, stamped with the session generation it was shown,
//! so a stale press is refused `timeTravel.stale` instead of acting on a session that moved on. Nothing here decides a
//! stage — the plugin runtime does. Escape is never a time-travel verb, so it never discards.
//!
//! The copy, the controls and the indicator are React's (`🛠️ShellHelpers/⏪️time-travel/🟦️.tsx`), pinned for both
//! shells by the language-neutral corpus `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json`. The history-edit
//! refusals (the hub's `history.*`, the session's `timeTravel.*`) are localized here too.

use super::*;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};
use semio_framework_time_travel::TimeTravelLabel;

//#region ⏪️TimeTravelControls

//#endregion ⏪️TimeTravelControls

//#region 🎯️TimeTravelFocus

//#endregion 🎯️TimeTravelFocus

//#region 🗣️TimeTravelCopy

//#endregion 🗣️TimeTravelCopy

//#region 📐️TimeTravelBand

//#endregion 📐️TimeTravelBand



//#region 👥️PeerHistoryEdits

//#endregion 👥️PeerHistoryEdits

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🧪️wgpu-time-travel/🦀️.rs"]
mod tests;
