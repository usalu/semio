//! 🔺️ Envelope (skyline) sparse LDLT direct solver: the one-shot factor and its checkpointed interactive job.

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, OriginalTurnFailure, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RetainedJobPayloadWriter, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_s_fem_sparse::{numerical_close_original, CscSym, NUMERICAL_OWNER_PAGE_BYTES, NumericalCheckpointFault, NumericalCheckpointIdentity, NumericalOutcomeDesk, NumericalOwnerRestoreCursor, NumericalPageCursor, NumericalRun, advance_f64_owner, advance_numerical_page_header, advance_owner_length, advance_pair_owner, advance_u32_owner, advance_u64_owner, advance_u64_values, close_nested_vec_owner_step, close_vec_owner_step, declared_owner_length, numerical_close_gate, numerical_ladder_turn, numerical_owner_turn, numerical_restore_close_due, numerical_restore_frontier, numerical_restore_payload_close, numerical_rung_demand, numerical_writer_close, numerical_writer_demand, parse_numerical_page, read_checkpoint_u64, restore_f64_entry, restore_pair_entry, restore_u32_entry, restore_usize_entry, validate_restored_owner};
use semio_s_fem_algebra::{MatD, VecD};

#[path = "🔩️factor-and-job/🦀️.rs"]
mod factor_and_job;
pub use factor_and_job::*;

#[path = "🔄️restore-cursor/🦀️.rs"]
mod restore_cursor;
pub use restore_cursor::*;

#[path = "🚀️run-and-close/🦀️.rs"]
mod run_and_close;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
