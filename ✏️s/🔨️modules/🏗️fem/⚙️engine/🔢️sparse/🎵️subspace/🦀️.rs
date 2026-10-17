//! 🎵️ Subspace-iteration eigensolver for the generalized problem `Kφ=λBφ` (modal and buckling), with the modal
//! input construction that feeds it and the dense cyclic-Jacobi oracle of its tests.

use semio_framework_job::{OriginalTurnFailure, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RetainedJobPayloadWriter, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_s_fem_sparse::{numerical_close_original, CscSym, Csr, MountedScalarSlots, NUMERICAL_OWNER_PAGE_BYTES, NumericalBatchEnd, NumericalCheckpointFault, NumericalCheckpointIdentity, NumericalOutcomeDesk, NumericalOwnerRestoreCursor, NumericalPageCursor, NumericalPageView, NumericalRun, advance_f64_owner, advance_matrix_owner, advance_numerical_page_header, advance_owner_length, advance_paged_f64_owner, advance_paged_u32_owner, advance_pair_owner, advance_u64_owner, advance_u64_values, close_nested_vec_owner_step, close_vec_owner_step, declared_owner_length, numerical_batch_close, numerical_batch_step, numerical_close_gate, numerical_ladder_turn, numerical_restore_close_due, numerical_restore_frontier, numerical_restore_payload_close, numerical_rung_demand, numerical_writer_close, numerical_writer_demand, parse_numerical_page, read_checkpoint_u64, read_checkpoint_usize, restore_f64_entry, restore_numerical_owner, restore_paged_f64_entry, restore_paged_u32_entry, restore_pair_entry, restore_usize_entry, validate_restored_owner};
use semio_s_fem_sparse_ldlt::{LdltFactor};
use semio_s_fem_algebra::{MatD, VecD};

#[path = "🎼️modal-input/🦀️.rs"]
mod modal_input;
pub use modal_input::*;

#[path = "🌀️job-core/🦀️.rs"]
mod job_core;
pub use job_core::*;

#[path = "🚶️iteration-steps/🦀️.rs"]
mod iteration_steps;

#[path = "♻️restore-cursor/🦀️.rs"]
mod restore_cursor;
pub use restore_cursor::*;

#[path = "🏃️run-and-publish/🦀️.rs"]
mod run_and_publish;
pub use run_and_publish::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
