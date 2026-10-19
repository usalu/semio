//! 🎵️ Subspace-iteration eigensolver for the generalized problem `Kφ=λBφ` (modal and buckling), with the modal
//! input construction that feeds it and the dense cyclic-Jacobi oracle of its tests.

use semio_s_fem_sparse::{NUMERICAL_CHECKPOINT_HEADER_BYTES, NumericalMatrixRestore, SPARSE_INDEX_SPACE, SparseColumns, advance_paged_pair_owner, close_paged_owner_step, numerical_grant_covers, numerical_restore_admit, numerical_restore_control, paged_owner_retirement_demand, restore_paged_pair_entry, restore_paged_usize_entry};
use semio_framework_job::OriginalTurnFailure;
use semio_framework_value::{list::PagedList, ValueRefusalKind};
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
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
use semio_framework_value::ValueError;
use semio_s_fem_sparse::numerical_close_original;
use semio_s_fem_sparse::CscSym;
use semio_s_fem_sparse::Csr;
use semio_s_fem_sparse::MountedScalarSlots;
use semio_s_fem_sparse::NUMERICAL_OWNER_PAGE_BYTES;
use semio_s_fem_sparse::NumericalCheckpointFault;
use semio_s_fem_sparse::NumericalCheckpointIdentity;
use semio_s_fem_sparse::NumericalOutcomeDesk;
use semio_s_fem_sparse::NumericalOwnerRestoreCursor;
use semio_s_fem_sparse::NumericalPageCursor;
use semio_s_fem_sparse::NumericalPageView;
use semio_s_fem_sparse::NumericalRun;
use semio_s_fem_sparse::advance_f64_owner;
use semio_s_fem_sparse::advance_matrix_owner;
use semio_s_fem_sparse::advance_numerical_page_header;
use semio_s_fem_sparse::advance_owner_length;
use semio_s_fem_sparse::advance_paged_f64_owner;
use semio_s_fem_sparse::advance_paged_u32_owner;
use semio_s_fem_sparse::advance_pair_owner;
use semio_s_fem_sparse::advance_u64_owner;
use semio_s_fem_sparse::advance_u64_values;
use semio_s_fem_sparse::close_nested_vec_owner_step;
use semio_s_fem_sparse::close_vec_owner_step;
use semio_s_fem_sparse::declared_owner_length;
use semio_s_fem_sparse::numerical_close_gate;
use semio_s_fem_sparse::numerical_ladder_turn;
use semio_s_fem_sparse::numerical_restore_close_due;
use semio_s_fem_sparse::numerical_restore_frontier;
use semio_s_fem_sparse::numerical_restore_payload_close;
use semio_s_fem_sparse::numerical_rung_demand;
use semio_s_fem_sparse::numerical_writer_close;
use semio_s_fem_sparse::numerical_writer_demand;
use semio_s_fem_sparse::parse_numerical_page;
use semio_s_fem_sparse::read_checkpoint_u64;
use semio_s_fem_sparse::read_checkpoint_usize;
use semio_s_fem_sparse::restore_f64_entry;
use semio_s_fem_sparse::restore_numerical_owner;
use semio_s_fem_sparse::restore_paged_f64_entry;
use semio_s_fem_sparse::restore_paged_u32_entry;
use semio_s_fem_sparse::restore_pair_entry;
use semio_s_fem_sparse::restore_usize_entry;
use semio_s_fem_sparse::validate_restored_owner;
use semio_s_fem_sparse_ldlt::LdltFactor;
use semio_s_fem_algebra::MatD;
use semio_s_fem_algebra::VecD;

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
#[cfg(test)]
#[path="🧪️tests/🎯️positive-spectrum/🦀️.rs"]
mod positive_spectrum_tests;
// #endregion 🔖️Tests

fn subspace_checkpoint_retirement_demands(state:&SubspaceCheckpoint)->Result<RetirementDemand,ValueError>{

        if !state.k_factor.terminal_is_empty(){return state.k_factor.retirement_demands()}
        for owner in [&state.prev_theta,&state.final_theta,&state.residuals]{if !owner.terminal_is_empty(){return paged_owner_retirement_demand(owner)}}
        if !state.x.data.terminal_is_empty(){return paged_owner_retirement_demand(&state.x.data)}
        if !state.b.vals.terminal_is_empty(){return paged_owner_retirement_demand(&state.b.vals)}
        for owner in [&state.b.indices,&state.b.indptr]{if !owner.terminal_is_empty(){return paged_owner_retirement_demand(owner)}}
        if !state.work.terminal_is_empty(){return state.work.retirement_demands()}
        if let Some(work)=state.retiring_work.as_ref(){return work.retirement_demands()}
        Ok(RetirementDemand{depth:1,..Default::default()})
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SubspaceSpectrum{Unresolved,Positive{available:usize},NoPositive}
