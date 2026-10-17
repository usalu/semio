//! 🔁️ Jacobi-preconditioned conjugate-gradient iterative solver: construction, checkpointed job and restore.

use replication::value::list::PagedList;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, OriginalTurnFailure, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RetainedJobPayloadWriter, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_fem_sparse::{numerical_close_original, Csr, MountedScalarSlots, NumericalBatchEnd, NumericalCheckpointFault, NumericalCheckpointIdentity, NumericalOutcomeDesk, NumericalOwnerRestoreCursor, NumericalPageCursor, NumericalRun, SPARSE_PAGE_BYTES, advance_f64_owner, advance_numerical_page_header, advance_owner_length, advance_paged_f64_owner, advance_paged_u32_owner, advance_u64_values, close_vec_owner_step, declared_owner_length, numerical_batch_close, numerical_batch_step, numerical_child_close, numerical_close_gate, numerical_close_result, numerical_ladder_turn, numerical_owner_turn, numerical_restore_close_due, numerical_restore_frontier, numerical_restore_payload_close, numerical_rung_demand, parse_numerical_page, read_checkpoint_u64, restore_f64_entry, restore_paged_f64_entry, restore_paged_u32_entry, write_numerical_entry};
use semio_s_fem_algebra::{VecD};

#[path = "🪜️job-and-steps/🦀️.rs"]
mod job_and_steps;
pub use job_and_steps::*;

#[path = "🌱️construction-and-restore/🦀️.rs"]
mod construction_and_restore;
pub use construction_and_restore::*;

#[path = "🏁️run-and-entry/🦀️.rs"]
mod run_and_entry;
pub use run_and_entry::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
