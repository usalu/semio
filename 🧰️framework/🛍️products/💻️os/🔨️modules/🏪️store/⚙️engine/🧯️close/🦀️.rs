//! 🧯️ Close witnesses and cursor admission over the StoreParts bundle.
//!
//! 🧭️ Layering and decomposition: `PLAN-OS-KERNEL-CRATES.md` sections 6 and 7 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`).

use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ErasedSnapshotRetirement, ValueError};

#[path = "📥️lifted/🏪️store/🦀️.rs"]
mod lifted_store;

/// 🧯️ Admits the owned roots of one store to the installed close cursor one phase at a time.
pub trait StoreCloseAdmission {
    /// 🧲️ Takes the attached backbone and its inbox under the caller's retirement grant.
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
    /// 🚪️ Moves one owned root of the addressed phase into its retirement owner.
    fn close_cursor_admit(&mut self, phase: semio_framework_os_store_read::ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
}

/// 🪹️ Reads the terminal-empty witnesses of one store's owned roots.
pub trait StoreCloseWitness {
    /// 🔎️ Whether every snapshot read lease has closed.
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool;
    /// 🫙️ Whether the structural shells hold no payload owner.
    fn close_structural_owners_terminal_is_empty(&self) -> bool;
    /// 🪦️ Whether every owned root has been detached.
    fn owned_roots_terminal_is_empty(&self) -> bool;
}
