//! 📬️ Job payload custody glue — wiring only. Domain lives at the owner `🦀️.rs` files.

use std::mem::{ManuallyDrop, MaybeUninit, size_of, size_of_val};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use semio_framework_trace::{Generation, OperationId};
pub use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_retained_clone::retained_clone;

/// 🧭️ The funded step a payload custody call runs inside: its cancellation, yield, retained stock and ledger.
pub trait JobPayloadStep {
    fn payload_ledger(&self) -> &Arc<JobPayloadOperationLedger>;
    fn payload_page_granted(&self) -> bool;
    fn grant_payload_page(&mut self);
    fn is_cancelled(&self) -> bool;
    fn should_yield(&self) -> bool;
    fn retained_grant(&self) -> RetainedCloneGrant;
    fn consume_retained(&mut self, progress: RetainedCloneProgress) -> Result<(), ValueError>;
}

#[path = "../../🎟️step-outcome/🦀️.rs"]
mod original_step_outcome;
pub use original_step_outcome::{Checkpoint,CommitCandidate,JobFault,StepOutcome,InteractiveJobCloseStep};

#[path = "../../🪙️return-escrow/🦀️.rs"]
pub mod original_return_escrow;
pub use original_return_escrow::{original_retained_delta,original_retained_remaining,original_retained_min,original_retained_is_subset,OriginalReturnEscrowMessages,require_return_structure,OriginalReturnStructure,OriginalReturnEscrowAccounting,OriginalReturnRedemptionPlan,OriginalReturnClaimPlan,OriginalReturnBudgetSnapshot,OriginalReturnLedgerIssue,OriginalReturnEscrowAmounts,OriginalReturnEscrowIdentity,OriginalReturnEscrowRow,OriginalReturnEscrowStage,OriginalReturnEscrowTable,OriginalReturnGuardPermit,OriginalReturnGuardDemand,OriginalReturnGuardPlan,OriginalReturnGuardSettlementPlan,ORIGINAL_RETURN_ESCROW_CELLS,ORIGINAL_RETURN_ESCROW_HEADER_BYTES,ORIGINAL_RETURN_ESCROW_ROW_BYTES,ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES};

#[path = "../../../../📄️payload-pages-and-ledger/🦀️.rs"]
pub mod payload_pages_and_ledger;
pub use payload_pages_and_ledger::{JOB_PAYLOAD_OPERATION_BYTES, JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_PAGE_BYTES, JOB_PAYLOAD_PROCESS_BYTES, JOB_PAYLOAD_PROCESS_OWNED_BYTES, JobPayloadAdmissionFault, JobPayloadOperationLedger, JobPayloadPage, JobPayloadPageSource, JobPayloadRejectedPage, JobPayloadStream, RetainedJobPayload, RetainedJobPayloadReader};

#[path = "../../../📖️source/✍️bytes/🦀️.rs"]
pub mod staged_source;
pub use staged_source::{JobPayloadStagedSource,JobPayloadWriteProgress,JobPayloadWriteFailure};

#[path="../../../📖️source/📦️page-transfer/🦀️.rs"]
mod original_page_transfer;
pub use original_page_transfer::RetainedJobPageTransfer;

#[path = "../../../📖️source/🦀️.rs"]
mod original_payload_source;

#[path = "../../🦀️.rs"]
pub mod payload_builder;
pub use payload_builder::{JobPayloadAuthority, OriginalPayloadBuilderIdentity, OriginalPayloadSourceIdentity, RetainedPayloadBuilder};

#[path = "../../../../♻️retirement/📄️payload/🧱️payload-owner/🦀️.rs"]
mod payload_owner_retirement;
pub use payload_owner_retirement::{close_ledger, ledger_demands};

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
