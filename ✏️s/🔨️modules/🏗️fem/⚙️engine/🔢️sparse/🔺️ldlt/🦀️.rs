//! 🔺️ Envelope (skyline) sparse LDLT direct solver: the one-shot factor and its checkpointed interactive job.

use semio_s_fem_sparse::{RowContributor, RowContributors, SPARSE_INDEX_SPACE, SparseColumns, advance_contributor_owner, advance_paged_f64_owner, advance_paged_pair_owner, advance_paged_u32_owner, advance_paged_u64_owner, close_paged_owner_step, numerical_grant_covers, numerical_restore_admit, numerical_restore_control, paged_from_values, paged_owner_retirement_demand, rcm_order, restore_contributor_entry, restore_paged_f64_entry, restore_paged_pair_entry, restore_paged_u32_entry, restore_paged_u64_entry, restore_paged_usize_entry, write_numerical_entry};
use semio_framework_job::InteractiveJob;
use semio_framework_value::{list::PagedList, HeapSortCursor, HeapSortWork, ValueRefusalKind};

use semio_framework_job::InteractiveJobCloseStep;

use semio_framework_job::OriginalTurnFailure;

use semio_framework_job::JobOutcomeBorrow;

use semio_framework_job::JobOutcomeDescriptor;

use semio_framework_job::JobOutcomeView;


use semio_framework_job::JobPayloadStream;

use semio_framework_job::Operation;

use semio_framework_job::RetainedCloneGrant;

use semio_framework_job::RetainedCloneProgress;

use semio_framework_job::RetainedJobPayload;

use semio_framework_job::RetainedJobPayloadWriter;

use semio_framework_job::StepContext;
use semio_framework_value::RetirementDemand;
use semio_s_fem_algebra::VecD;


























use semio_framework_value::ValueError;
use semio_s_fem_sparse::numerical_close_original;
use semio_s_fem_sparse::CscSym;
use semio_s_fem_sparse::NUMERICAL_OWNER_PAGE_BYTES;
use semio_s_fem_sparse::NumericalCheckpointFault;
use semio_s_fem_sparse::NumericalCheckpointIdentity;
use semio_s_fem_sparse::NumericalOutcomeDesk;
use semio_s_fem_sparse::NumericalOwnerRestoreCursor;
use semio_s_fem_sparse::NumericalPageCursor;
use semio_s_fem_sparse::NumericalRun;
use semio_s_fem_sparse::advance_f64_owner;
use semio_s_fem_sparse::advance_numerical_page_header;
use semio_s_fem_sparse::advance_owner_length;
use semio_s_fem_sparse::advance_pair_owner;
use semio_s_fem_sparse::advance_u32_owner;
use semio_s_fem_sparse::advance_u64_owner;
use semio_s_fem_sparse::advance_u64_values;
use semio_s_fem_sparse::close_nested_vec_owner_step;
use semio_s_fem_sparse::close_vec_owner_step;
use semio_s_fem_sparse::declared_owner_length;
use semio_s_fem_sparse::numerical_close_gate;
use semio_s_fem_sparse::numerical_ladder_turn;
use semio_s_fem_sparse::numerical_owner_turn;
use semio_s_fem_sparse::numerical_restore_close_due;
use semio_s_fem_sparse::numerical_restore_frontier;
use semio_s_fem_sparse::numerical_restore_payload_close;
use semio_s_fem_sparse::numerical_rung_demand;
use semio_s_fem_sparse::numerical_writer_close;
use semio_s_fem_sparse::numerical_writer_demand;
use semio_s_fem_sparse::parse_numerical_page;
use semio_s_fem_sparse::read_checkpoint_u64;
use semio_s_fem_sparse::restore_f64_entry;
use semio_s_fem_sparse::restore_pair_entry;
use semio_s_fem_sparse::restore_u32_entry;
use semio_s_fem_sparse::restore_usize_entry;
use semio_s_fem_sparse::validate_restored_owner;
use semio_s_fem_algebra::MatD;


#[path = "🔩️factor-and-job/🦀️.rs"]
mod factor_and_job;
pub use factor_and_job::*;

#[path = "🔄️restore-cursor/🦀️.rs"]
mod restore_cursor;
#[cfg(test)]
    use restore_cursor::*;

#[path = "🚀️run-and-close/🦀️.rs"]
mod run_and_close;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

fn ldlt_checkpoint_retirement_demand(state: &LdltCheckpoint) -> Result<Option<RetirementDemand>, ValueError> {
    let mut demand = if !state.contributors.terminal_is_empty() { state.contributors.retirement_demands()? }
    else if !state.columns.terminal_is_empty() { state.columns.next_retirement_demands()? }
    else if !state.workspace.candidate.terminal_is_empty() { paged_owner_retirement_demand(&state.workspace.candidate)? }
    else if !state.workspace.touched.terminal_is_empty() { paged_owner_retirement_demand(&state.workspace.touched)? }
    else if !state.workspace.marks.terminal_is_empty() { paged_owner_retirement_demand(&state.workspace.marks)? }
    else if !state.a.rowind.terminal_is_empty() { paged_owner_retirement_demand(&state.a.rowind)? }
    else if !state.a.colptr.terminal_is_empty() { paged_owner_retirement_demand(&state.a.colptr)? }
    else if !state.d.terminal_is_empty() { paged_owner_retirement_demand(&state.d)? }
    else if !state.a.vals.terminal_is_empty() { paged_owner_retirement_demand(&state.a.vals)? }
    else if !state.workspace.values.terminal_is_empty() { paged_owner_retirement_demand(&state.workspace.values)? }
    else { return Ok(None); };
    demand.copy_bytes = size_of::<LdltCheckpoint>() * 2;
    Ok(Some(demand))
}

#[cfg(test)]
#[path="🧪️tests/📚️paged-ldlt/🦀️.rs"]
mod paged_ldlt_tests;
