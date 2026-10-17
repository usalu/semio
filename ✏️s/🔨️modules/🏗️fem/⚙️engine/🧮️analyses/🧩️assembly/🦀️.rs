//! 🧩️ Deterministic sparse stiffness assembly: the bounded construction, the checkpointed assembly job, the CSR
//! build and the one-shot assembled system with its load-case right-hand sides.

use replication::value::list::PagedList;
use semio_framework_job::{OriginalTurnFailure, InteractiveJob, InteractiveJobCloseStep, JobOutcomeDescriptor, JobOutcomeBorrow, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use semio_s_fem_analyses_model::{ASSEMBLY_TRIPLET_INDEX_SPACE, AnalysisModel, AnalysisModelCloseCursor, DofMap, LoadCase, MOUNTED_OWNER_PAGE_BYTES, MountedAnalysisModel, build_dof_map, build_rcm_permutation, close_analysis_model_step, close_paged_owner_step, close_vec_owner_step, cold_paged_owner, decode_value, element_global_indices, encode_value, node_index_of, positions_of, reserve_exact_owner_page, validate};
use semio_s_fem_algebra::VecD;
use semio_s_fem_model::{Dof, Element, ElementContext, FemError, Node};
use semio_s_fem_static::Elements;
use semio_s_fem_sparse::{numerical_close_original, numerical_batch_close, numerical_batch_step, numerical_close_gate, numerical_ladder_turn, numerical_rung_demand, NumericalBatchEnd, NumericalOutcomeDesk, NumericalRun, Coo, Csr};
use semio_s_fem_sparse_ldlt::{ldlt_factor, LdltFactor};

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
