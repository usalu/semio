//! 🎟️ Original construction, performed work and failure custody use the shared physical contract.
pub use semio_framework_value_control_physical::{RETAINED_CLONE_BULK_PAGE_BYTES,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneBirthDemand,RETAINED_CLONE_GRANT_SCHEMA,RETAINED_CLONE_PROGRESS_SCHEMA,admit_retained_clone_progress,admit_retained_clone_close,ValueRefusalKind,ValueError,RetirementDemand,retained_clone_grant_admits,RetainedWorkOutcome,RetainedWorkPoll};
#[path="🏭️construction/🦀️.rs"]
pub mod construction;
pub use construction::{OriginalConstructionIssuerIdentity,OriginalConstructionActionIdentity,OriginalConstructionPhase,OriginalConstructionAction,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionRecipient,OriginalConstructionAuthority,OriginalConstructionLoan};
pub use semio_framework_value_control_physical::allocation;
#[path="📦️effect/🦀️.rs"]
mod allocation_effect;
#[path="🎟️authoring/🦀️.rs"]
mod authoring;
pub use authoring::{AuthoredEffect,AuthoringWallet,run_original_effect,OriginalEffectRun};
#[path="🎟️lent/🦀️.rs"]
mod lent;
pub use lent::{EffectLend,LentEffectSlot};
#[path="🎟️gate/🦀️.rs"]
mod gate;
pub use gate::{RetainedOwnerGate,RetainedOwnerGateBusy,RetainedOwnerGateGuard};
pub use allocation_effect::{OriginalWorkPerformedBoundary,RetainedAllocationEffect};
pub use allocation::{NATIVE_ALLOCATION_SCHEMA,NativeAllocationAuthority,NativeAllocationAuthorityLoan,NativeAllocationAuthorityPort,NativeAllocationReply,NativeAllocationReplyComplete,NativeAllocationInvocation,NativeAllocationInvocationResult,NativeAllocationLoan,NativeSendAllocationLoan,NativeAllocationSendCallback,NativeAllocationTransaction,NativeAllocationTransactionPhase,NativeAllocationBudget,NativeAllocationEvent,NativeAllocationIdentity,NativeAllocationRefusal,NativeAllocationRefusalHandle,NativeAllocationRefusalVault,NativeAllocationFailure,NativeAllocationProgress,NativeAllocationRequest};
#[path="♻️failure/🦀️.rs"]mod original_failure;
pub use original_failure::{OriginalFailureStorageBirth,OriginalFailureStorageBirthPhase,OriginalFailurePerformedScope,OriginalFailureStorageStep,OriginalFailureFunding,refill_original_failure_storage,OriginalRetirementFailure,OriginalFailureStorage,OriginalFailureStoragePool,run_with_failure_storage,run_with_failure_storage_transaction};
#[path="♻️original-close/🦀️.rs"]mod original_close;
pub use original_close::{OriginalRetirementCloseLoan,OriginalRetirementClosePort};
#[path="../../📦️paged/📖️read/🎟️.rs"]
mod byte_source;
pub use byte_source::{ByteReadSource,ByteReadRange};

#[path="📤️output/🦀️.rs"]
mod output_slot;
pub use output_slot::{retained_clone_cursor_take_demand,retained_clone_take_demand,take_retained_clone_output};

#[path="♻️close-start/🦀️.rs"]
mod close_start;
pub use close_start::{retained_close_start_demand,retained_close_start_work_demand,start_retained_close};

#[path="🎟️work-action/🦀️.rs"]
mod work_action;
pub use work_action::{OriginalWork,OriginalWorkState,OriginalWorkTurn,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkAuthority,OriginalWorkLoan,OriginalWorkRecipient,OriginalWorkScope,OriginalWorkScopeIdentity,OriginalWorkIssuerIdentity,OriginalWorkActionIdentity,OriginalWorkActionReservation,OriginalWorkActionProof,OriginalWorkActionIssuer,OriginalWorkPerformedReceipt,OriginalWorkPerformedIdentity,OriginalWorkSink,OriginalWorkPartialOutcome,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete};

pub use semio_framework_value_source_turn::{OriginalSourceTurnIssuerIdentity,OriginalSourceTurnIdentity,OriginalSourceTurnError,OriginalSourceTurnIssuer,OriginalSourceTurn,OriginalSourceTurnWitness};

#[path="📨️reply/🦀️.rs"]
mod borrowed_reply;
pub use borrowed_reply::{OriginalEffectReply,OriginalEffectReplyComplete};
