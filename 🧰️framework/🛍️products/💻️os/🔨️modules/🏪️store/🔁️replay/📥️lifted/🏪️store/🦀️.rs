//! 📥️ Trait impls of `🏪️store` whose trait or type belongs to `os-store-replay` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_vcs::ArtifactHistoryLedger;
use semio_framework_os_spr::Edit;
use std::collections::HashMap;
use semio_framework_os_spr::Mutation;
use crate::ReplayEdits;


impl<Mutation> ReplayEdits<Mutation> for HashMap<String, Edit<Mutation>> {
    fn replay_edit(&self, edit_id: &str) -> Option<&Edit<Mutation>> {
        self.get(edit_id)
    }
}

impl<Mutation> ReplayEdits<Mutation> for ArtifactHistoryLedger<Edit<Mutation>> {
    fn replay_edit(&self, edit_id: &str) -> Option<&Edit<Mutation>> {
        self.find_near(|edit| edit.id == edit_id)
    }
}
