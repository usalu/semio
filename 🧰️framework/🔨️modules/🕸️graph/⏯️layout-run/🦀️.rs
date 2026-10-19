//! ⏯️ Domain-neutral, stepped, cancellable force-directed layout run: the shared `runJob` every graph plugin's
//! `reorganize`/`forceLayout` tool drives through the ToolRun ledger.
//!
//! - One iteration per fuel unit; every iteration is sliced into resumable phases (tree, repulsion, springs,
//!   integration) that check the step deadline, so one step stays bounded for large graphs.
//! - Barnes-Hut repulsion above `pairwiseMaxBodies`, exact pairwise at or below it.
//! - Per-node trace records (`Entity` subjects) with displacement verdicts; counters iterations, energy, largest
//!   displacement and settled nodes; stages initialize, iterate, settle.
//! - Provisional move ops come from a caller encoder; the provisional list is compacted to one op per moved node at
//!   every checkpoint and at settle, so finalize publishes exactly the moved nodes.
//!
//! Schema of record: `🧬️schema/🔣️.json`. Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` §3.7, §5 W3-F.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_framework_value::{FromValue, ToValue};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, OriginalTurnFailure, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress, StepContext};
use semio_framework_job_payload::{JOB_PAYLOAD_PAGE_BYTES};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter,
    ToolRunTraceKind, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_PROVISIONAL_OPS_MAX,
};
use serde::{Deserialize, Serialize};
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Limits
/// 🚰️ Estimated tick bytes after which an iteration or compaction tick is flushed (one 16 KiB job payload page).
pub const LAYOUT_RUN_TICK_FLUSH_BYTES: usize = 12 * 1024;
/// 🧱️ Largest move op one encoder call may return.
pub const LAYOUT_RUN_OP_BYTES_MAX: usize = 2 * 1024;
/// 🔢️ Largest graph one run lays out: every node must fit one op in half of the provisional op cap.
pub const LAYOUT_RUN_NODES_MAX: usize = TOOL_RUN_PROVISIONAL_OPS_MAX as usize / 2;
/// ⏱️ Repulsion queries between two deadline checks.
pub const LAYOUT_RUN_REPULSION_CHUNK: usize = 4;
/// ⏱️ Linear body or edge updates between two deadline checks.
pub const LAYOUT_RUN_LINEAR_CHUNK: usize = 128;
/// 🌳️ Quadtree depth after which coincident bodies share one leaf.
pub const LAYOUT_RUN_TREE_DEPTH_MAX: u32 = 32;
/// 🪪️ Checkpoint magic, `LRC1` read as a little-endian `u32`.
pub const LAYOUT_RUN_CHECKPOINT_MAGIC: u32 = 0x4C52_4331;
const NONE: u32 = u32::MAX;
const MIN_DISTANCE: f64 = 1e-4;
//#endregion 🔖️Limits

#[path = "📖️vocabulary/🦀️.rs"]
mod layout_vocabulary;
pub use layout_vocabulary::*;

#[path = "🕸️input-graph/🦀️.rs"]
mod layout_graph;
pub use layout_graph::*;

#[path = "🎛️config/🦀️.rs"]
mod layout_config;
pub use layout_config::*;

#[path = "🏃️job/🦀️.rs"]
mod layout_job;
pub use layout_job::*;

//#region 🔖️Testing
/// 🔮️ Test support shared by this crate's and every consumer plugin's tests (`testing` feature): the third-party
/// layout-quality oracle (normalized stress: Euclidean distances optimally rescaled against hop distances, squared
/// relative error averaged over all connected node pairs; the `fdg-sim` Fruchterman-Reingold layout from the same
/// initial positions) and a driver that records a run the way the ToolRun ledger folds its ticks.
#[cfg(any(test, feature = "testing"))]
#[path = "🔮️testing/🦀️.rs"]
pub mod testing;
//#endregion 🔖️Testing

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️oracle/🦀️.rs"]
mod oracle_law;
