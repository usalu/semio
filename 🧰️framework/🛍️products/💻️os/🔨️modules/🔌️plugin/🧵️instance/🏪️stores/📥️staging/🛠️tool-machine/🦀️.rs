//! 🎸️ Staged slice of the `tool-machine` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🛠️ The instance's tool-machine turns: gesture, scrub and typing dispatch.
use super::*;
use crate::tool_jobs::tool_machine::*;
use crate::tool_jobs::tool_machine::gesture_issuer::{GestureOperation,reserve_live,issue_live};
use semio_framework_tool_machine::{
    ScrubPhase, ToolAbortReason, ToolStep, TypingCommit, TypingInput, TypingPhase, SCRUB_ABORT_ARG, SCRUB_COMMIT_ARG, SCRUB_GESTURE_ARG,
    TYPING_BUFFER_ARG, TYPING_COMMIT_ARG,
};
use std::collections::BTreeMap;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::OriginalAliasBatch as ToolOriginalAliasBatch;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

//#region 🔖️Driver
impl<A: ArtifactApp> InstanceStores<A> {
    /// 🧪️ The committed generations of the config lanes — the app config, and window `window_id` of kind `window_kind_id` once
    /// its partition exists — so a law counts the config edits a press published.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn config_generations(&self, window_kind_id: &str, window_id: &str) -> (u64, Option<u64>) {
        (self.config_store.generation(), self.window_config_store.snapshot(window_kind_id, window_id).map(|snapshot| snapshot.generation()))
    }

    /// 🧪️ The `TransactionRef` each edit of the log carries, oldest first (`None`: an edit no tool transaction authored) — a
    /// law counts edits and reads which run or press authored each.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn edit_transactions(&self) -> Vec<Option<protocol::TransactionRef>> {
        self.store.envelope().vcs.edits.iter().map(|edit| edit.mutation_meta.first().and_then(|meta| meta.transaction.clone())).collect()
    }
}
//#endregion 🔖️Driver
