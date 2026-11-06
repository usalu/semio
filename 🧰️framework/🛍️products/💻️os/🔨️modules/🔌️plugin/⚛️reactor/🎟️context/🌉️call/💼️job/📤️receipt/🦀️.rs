//! 💼️ The guest facade links the actual independent data-only job receipt owner.
pub use semio_framework_actor_receipt::{OriginalOwnedJobCall,OriginalOwnedJobChunk,OriginalOwnedJobStatus,OriginalOwnedJobFixedReceipt,OriginalOwnedJobReceiptLoan};
#[cfg(any(test,feature="original-actor-context-testing"))]
use super::{OriginalActorFixedCall,OriginalActorFixedKind,OriginalActorFixedReceipt,OriginalEscrowExtent,OriginalSliceHeaderWriter};
#[cfg(any(test,feature="original-actor-context-testing"))]
use semio_framework_actor::{OriginalJobCallFailure,OriginalJobOutcome,OriginalJobOutputState,OriginalJobStartReply,OriginalJobStepReply,OriginalJobOutputReply,OriginalJobOutputAckReply,OriginalJobCancelReply,RetainedTurnInput};
#[cfg(any(test,feature="original-actor-context-testing"))]
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
#[cfg(any(test,feature="original-actor-context-testing"))]
use semio_framework_job_return_escrow_publication::OriginalEscrowHeaderWrite;
#[cfg(any(test,feature="original-actor-context-testing"))]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;
