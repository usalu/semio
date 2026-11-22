//! 🔭️ Derived history previews, report replays and derived operations over the Store frontier: borrowed views of one Store's components that never move its cursor.
//!
//! 🧭️ Split out of `os-store-engine` so the composer crate keeps real headroom under the source boundary; the composer re-exports the public paths and delegates to [`StoreDerived`].

use semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;
use semio_os_store_read::HistoryReadPlan;
use semio_os_store_replay::ReplayConvergence;
use semio_os_store_ledger::edit_retirement;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::ValueError;
use semio_os_spr::MutationId;
use semio_os_spr::{Edit, Mutation};
use semio_os_spr::{ColdMutation, MutationDiff};
use std::marker::PhantomData;
use std::sync::Arc;
use semio_os_store_read::{ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ErasedSnapshotRead, SnapshotRead};
use semio_os_vcs::VcsError;
use semio_os_store_ledger::CursorRevisionAccumulator;
use semio_os_store_operation_wire::ArtifactOperationWire;
use semio_os_store_replay::{EditReplay, IndexedAppliedReplayEdits, ReplayMode, ReplayProgress, ReplayStep};
use semio_os_store_engine_core::lifted_store::StoreWire;
use semio_os_store_engine_core::lifted_store::StoreCursor;
use semio_os_store_engine_live::lifted_store::StoreLive;
use semio_os_store_replay_operation::{ArtifactDerivedOperation, ArtifactDerivedOperationMode, ArtifactReplayPreparationContext, ArtifactReplayPreparationMode, ArtifactReplayPreparationRequest, ArtifactReplayPrepared};
use semio_os_spr as protocol;
use semio_os_store_engine_core::lifted_store::{StoreLog, StoreOwners};

/// ⛽️ Computational work one completed-planner retirement step may spend in its one original effect.
const ARTIFACT_STORE_PLAN_RETIREMENT_WORK: u64 = 64;

/// 🪢️ Funds one completed-planner retirement step from the decoder's own Work recipient; a pending or unadmitted turn keeps the planner and reports no progress.
fn funded_plan_retirement(control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneProgress,VcsError>{
    use semio_framework_value::RetainedWorkPoll;
    let Some(poll)=control.with_effect(grant,ARTIFACT_STORE_PLAN_RETIREMENT_WORK,|_,effect|run(effect)).map_err(VcsError::InverseRefused)?else{return Ok(Default::default())};
    match poll{RetainedWorkPoll::Pending{..}=>Ok(Default::default()),RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|step.progress()).map_err(|failure|VcsError::InverseRefused(failure.into_value_error()))}
}

#[path = "🔭️history-preview/🦀️.rs"]
mod history_preview;
#[path = "🧩️operation/🦀️.rs"]
mod operation;
#[path = "📜️report-replay/🦀️.rs"]
mod report_replay;
#[path = "📸️checkpoint-read/🦀️.rs"]
mod checkpoint_read;
pub use checkpoint_read::CheckpointSource;

/// 🔭️ The components one derived read or replay step borrows from its Store; `Log` is the shared [`StoreLog`] for reads and the exclusive one when the history ledger is stepped or retired.
pub struct StoreDerived<'a, P, Mutation, Log>
where
    P: semio_os_spr::StoreState,
    Mutation: semio_os_spr::StoreMutationFor<P>,
{
    pub log: Log,
    pub owners: &'a StoreOwners<P, Mutation>,
    pub wire: &'a StoreWire<P, Mutation>,
    pub cursor: &'a StoreCursor<P, Mutation>,
    pub live: &'a StoreLive<P, Mutation>,
}
