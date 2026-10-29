//! 🎫️ The original actor context vocabulary every plugin guest and host shares: the guest entry, the fixed call and job receipts, and the failure kinds.
//!
//! 🧭️ Owned by `semio-framework-plugin-guest-contract`; the call sources stay in `⚛️reactor/🎟️context/🌉️call` and the plugin reactor re-exports this module. The wasm guest export bindings are not part of the contract: they name plugin runtime types and stay with the reactor.
use semio_framework_actor::RetainedTurnInput;
use semio_framework_async::{CancelToken,CancelTokenRetirement,CancelTokenRetirementError};
use semio_framework_job::{Generation, OperationId, OriginalCallerSourceParent, StepBudget, StepContext};
use semio_framework_job_payload::{JobPayloadAuthority};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;

use semio_framework_actor::retained_context::{OriginalActorAdmission,OriginalActorContextError,OriginalActorContext,OriginalActorCall,OriginalActorCallError,OriginalActorContextSlot,OriginalActorAdmissionReply,OriginalActorCloseReply};
fn metadata(complete:bool)->RetainedCloneStep{let progress=RetainedCloneProgress{copied_items:1,..Default::default()};if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

#[path="../../⚛️reactor/🎟️context/🌉️call/🦀️.rs"]
mod guest_entry;
pub use guest_entry::{OriginalActorEntryFailure,OriginalActorGuestEntry,OriginalActorFixedCall,OriginalActorFixedKind,OriginalActorFixedReceipt};
pub use guest_entry::{OriginalOwnedJobCall,OriginalOwnedJobChunk,OriginalOwnedJobFixedReceipt,OriginalOwnedJobReceiptLoan,OriginalOwnedJobStatus};

/// 🧪️ Links narrow native tests to the same production owner and canonical neutral law bodies.
#[cfg(feature="original-actor-context-testing")]
pub mod original_laws{
 pub use super::guest_entry::{original_job_output_paged_source_identity_cpu_denials_system_and_paid_ack,original_owned_output_loan_keeps_paid_producer_and_exact_handback,original_owned_job_header_matches_portable_oracle_without_allocation,original_owned_job_metadata_loan_denies_overwrite_before_body_without_heap};
}
