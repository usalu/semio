//! 🔁️ Jacobi-preconditioned conjugate-gradient iterative solver: construction, checkpointed job and restore.

use semio_s_fem_sparse::{numerical_restore_admit, numerical_restore_control};
use semio_framework_value_persistent::list::PagedList;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::OriginalTurnFailure;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job::JobOutcomeDescriptor;
use semio_framework_job::JobOutcomeView;
use semio_framework_job_payload::JobPayloadStream;
use semio_framework_job::Operation;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_job::RetainedCloneProgress;
use semio_framework_job_payload::RetainedJobPayload;
use semio_framework_job_payload::RetainedJobPayloadWriter;
use semio_framework_job::StepContext;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use semio_s_fem_sparse::numerical_close_original;
use semio_s_fem_sparse::Csr;
use semio_s_fem_sparse::MountedScalarSlots;
use semio_s_fem_sparse::NumericalCheckpointFault;
use semio_s_fem_sparse::NumericalCheckpointIdentity;
use semio_s_fem_sparse::NumericalOutcomeDesk;
use semio_s_fem_sparse::NumericalOwnerRestoreCursor;
use semio_s_fem_sparse::NumericalPageCursor;
use semio_s_fem_sparse::NumericalRun;
use semio_s_fem_sparse::SPARSE_PAGE_BYTES;
use semio_s_fem_sparse::advance_f64_owner;
use semio_s_fem_sparse::advance_numerical_page_header;
use semio_s_fem_sparse::advance_owner_length;
use semio_s_fem_sparse::advance_paged_f64_owner;
use semio_s_fem_sparse::advance_paged_u32_owner;
use semio_s_fem_sparse::advance_u64_values;
use semio_s_fem_sparse::close_vec_owner_step;
use semio_s_fem_sparse::declared_owner_length;
use semio_s_fem_sparse::numerical_child_close;
use semio_s_fem_sparse::numerical_close_gate;
use semio_s_fem_sparse::numerical_close_result;
use semio_s_fem_sparse::numerical_ladder_turn;
use semio_s_fem_sparse::numerical_owner_turn;
use semio_s_fem_sparse::numerical_restore_close_due;
use semio_s_fem_sparse::numerical_restore_frontier;
use semio_s_fem_sparse::numerical_restore_payload_close;
use semio_s_fem_sparse::numerical_rung_demand;
use semio_s_fem_sparse::parse_numerical_page;
use semio_s_fem_sparse::read_checkpoint_u64;
use semio_s_fem_sparse::restore_f64_entry;
use semio_s_fem_sparse::restore_paged_f64_entry;
use semio_s_fem_sparse::restore_paged_u32_entry;
use semio_s_fem_sparse::write_numerical_entry;
use semio_s_fem_algebra::VecD;

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
#[cfg(test)]
#[global_allocator]
static ORIGINAL_PCG_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
// #endregion 🔖️Tests
