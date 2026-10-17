//! 🧮️ Sparse linear algebra core: COO/CSR/CSC storage, reverse-Cuthill-McKee ordering, and the numerical job
//! protocol every sparse solver job shares (outcome desk, close ladder, checkpoint page writer and restore cursor).

use replication::value::list::PagedList;
use semio_framework_job::{close_original_retained_step, InteractiveJob, InteractiveJobCloseStep, OriginalTurnFailure, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPayload, RetainedJobPayloadWriter, StepBudget, StepContext};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::VecDeque;
use semio_s_fem_algebra::{MatD, VecD};

#[path = "🤝️numerical-protocol/🦀️.rs"]
mod numerical_protocol;
pub use numerical_protocol::*;

#[path = "🗃️storage-and-ordering/🦀️.rs"]
mod storage_and_ordering;
pub use storage_and_ordering::*;

#[path = "📄️checkpoint-pages/🦀️.rs"]
mod checkpoint_pages;
pub use checkpoint_pages::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
