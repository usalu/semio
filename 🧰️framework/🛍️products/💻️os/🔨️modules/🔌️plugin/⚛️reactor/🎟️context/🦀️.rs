//! 🌉️ OS guest entries compose the defining General Actor context with original job and wire ownership.
use semio_framework_actor::RetainedTurnInput;
use semio_framework_async::{CancelToken,CancelTokenRetirement,CancelTokenRetirementError};
use semio_framework_job::{Generation,JobPayloadAuthority,OperationId,OriginalCallerSourceParent,StepBudget,StepContext};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;

use semio_framework_actor::retained_turn::context::{OriginalActorAdmission,OriginalActorContextError,OriginalActorContext,OriginalActorCall,OriginalActorCallError,OriginalActorContextSlot,OriginalActorAdmissionReply,OriginalActorCloseReply};
fn metadata(complete:bool)->RetainedCloneStep{let progress=RetainedCloneProgress{copied_items:1,..Default::default()};if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

#[path="🌉️call/🦀️.rs"]
mod guest_entry;
pub use guest_entry::{OriginalActorEntryFailure,OriginalActorGuestEntry,OriginalActorFixedCall,OriginalActorFixedKind,OriginalActorFixedReceipt};
pub use guest_entry::{OriginalOwnedJobCall,OriginalOwnedJobChunk,OriginalOwnedJobFixedReceipt,OriginalOwnedJobStatus};

#[cfg(all(target_arch="wasm32",target_env="p2"))]
#[path="🌉️call/🎭️exports/🦀️.rs"]
pub mod exports;

/// 🧪️ Links narrow native tests to the same production owner and canonical neutral law bodies.
#[cfg(feature="original-actor-context-testing")]
pub mod original_laws{
 pub use super::guest_entry::{original_job_output_paged_source_identity_cpu_denials_system_and_paid_ack,original_owned_output_loan_keeps_paid_producer_and_exact_handback,original_owned_job_header_matches_portable_oracle_without_allocation,original_owned_job_metadata_loan_denies_overwrite_before_body_without_heap};
}
