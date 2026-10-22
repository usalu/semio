//! 📬️ Job payload custody glue — wiring only. Domain lives at the owner `🦀️.rs` files.

use std::mem::{ManuallyDrop, MaybeUninit, size_of_val};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use semio_framework_trace::{Generation, OperationId};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

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

#[path = "../../../../📄️payload-pages-and-ledger/🦀️.rs"]
pub mod payload_pages_and_ledger;
pub use payload_pages_and_ledger::{JOB_PAYLOAD_OPERATION_BYTES, JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_PAGE_BYTES, JOB_PAYLOAD_PROCESS_BYTES, JOB_PAYLOAD_PROCESS_OWNED_BYTES, JobPayloadAdmissionFault, JobPayloadOperationLedger, JobPayloadPage, JobPayloadPageSource, JobPayloadRejectedPage, JobPayloadStream, RetainedJobPayload, RetainedJobPayloadReader};

#[path = "../../../../✍️payload-writer-and-grant/🦀️.rs"]
pub mod payload_writer_and_grant;
pub use payload_writer_and_grant::{JobPayloadPageGrant, JobPayloadStagedSource, RetainedJobPayloadWriter};

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
