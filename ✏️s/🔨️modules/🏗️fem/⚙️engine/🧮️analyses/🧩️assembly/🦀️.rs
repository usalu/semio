//! build and the one-shot assembled system with its load-case right-hand sides.
//! 🧩️ Deterministic sparse stiffness assembly: the bounded construction, the checkpointed assembly job, the CSR

use replication::value::list::PagedList;
use semio_framework_job::OriginalTurnFailure;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::JobOutcomeDescriptor;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job::JobPayloadStream;
use semio_framework_job::Operation;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_job::RetainedCloneProgress;
use semio_framework_job::StepContext;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use semio_s_fem_analyses_model::ASSEMBLY_TRIPLET_INDEX_SPACE;
use semio_s_fem_analyses_model::AnalysisModel;
use semio_s_fem_analyses_model::AnalysisModelCloseCursor;
use semio_s_fem_analyses_model::DofMap;
use semio_s_fem_analyses_model::LoadCase;
use semio_s_fem_analyses_model::MOUNTED_OWNER_PAGE_BYTES;
use semio_s_fem_analyses_model::MountedAnalysisModel;
use semio_s_fem_analyses_model::build_dof_map;
use semio_s_fem_analyses_model::build_rcm_permutation;
use semio_s_fem_analyses_model::close_analysis_model_step;
use semio_s_fem_analyses_model::close_paged_owner_step;
use semio_s_fem_analyses_model::close_vec_owner_step;
use semio_s_fem_analyses_model::cold_paged_owner;
use semio_s_fem_analyses_model::decode_value;
use semio_s_fem_analyses_model::element_global_indices;
use semio_s_fem_analyses_model::encode_value;
use semio_s_fem_analyses_model::node_index_of;
use semio_s_fem_analyses_model::positions_of;
use semio_s_fem_analyses_model::reserve_exact_owner_page;
use semio_s_fem_analyses_model::validate;
use semio_s_fem_algebra::VecD;
use semio_s_fem_model::Dof;
use semio_s_fem_model::Element;
use semio_s_fem_model::ElementContext;
use semio_s_fem_model::FemError;
use semio_s_fem_model::Node;
use semio_s_fem_static::Elements;
use semio_s_fem_sparse::numerical_close_original;
use semio_s_fem_sparse::numerical_close_gate;
use semio_s_fem_sparse::numerical_ladder_turn;
use semio_s_fem_sparse::numerical_rung_demand;
use semio_s_fem_sparse::NumericalOutcomeDesk;
use semio_s_fem_sparse::NumericalRun;
use semio_s_fem_sparse::Coo;
use semio_s_fem_sparse::Csr;
use semio_s_fem_sparse_ldlt::ldlt_factor;
use semio_s_fem_sparse_ldlt::LdltFactor;

#[path = "🧷️job-state-and-construction/🦀️.rs"]
mod job_state_and_construction;
pub use job_state_and_construction::*;

#[path = "🪡️element-build/🦀️.rs"]
mod element_build;
pub use element_build::*;

#[path = "🔶️csr-build-and-run/🦀️.rs"]
mod csr_build_and_run;
pub use csr_build_and_run::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
