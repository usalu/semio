//! 🎟️ Original construction, performed work and failure custody use the shared physical contract.
pub use semio_framework_value_control_physical::{RETAINED_CLONE_BULK_PAGE_BYTES,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneBirthDemand,RETAINED_CLONE_GRANT_SCHEMA,RETAINED_CLONE_PROGRESS_SCHEMA,admit_retained_clone_progress,admit_retained_clone_close,ValueRefusalKind,ValueError,RetirementDemand,retained_clone_grant_admits,RetainedWorkOutcome,RetainedWorkPoll};
#[path="🏭️construction/🦀️.rs"]
pub mod construction;
pub use construction::{OriginalConstructionIssuerIdentity,OriginalConstructionActionIdentity,OriginalConstructionPhase,OriginalConstructionAction,OriginalConstructionAcceptStatus,OriginalConstructionCompletion,OriginalConstructionRetryWitness,OriginalConstructionActionWitness,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionRecipient,OriginalConstructionAuthority,OriginalConstructionLoan};
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
pub use allocation_effect::{OriginalWorkActionBoundary,OriginalWorkPerformedBoundary,RetainedAllocationEffect};
pub use allocation::{NATIVE_ALLOCATION_SCHEMA,NativeAllocationAuthority,NativeAllocationAuthorityLoan,NativeAllocationAuthorityPort,NativeAllocationReply,NativeAllocationReplyComplete,NativeAllocationInvocation,NativeAllocationInvocationResult,NativeAllocationLoan,NativeSendAllocationLoan,NativeAllocationSendCallback,NativeAllocationTransaction,NativeAllocationTransactionPhase,NativeAllocationBudget,NativeAllocationEvent,NativeAllocationIdentity,NativeAllocationRefusal,NativeAllocationRefusalHandle,NativeAllocationRefusalVault,NativeAllocationFailure,NativeAllocationProgress,NativeAllocationRequest};
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
pub use work_action::{OriginalWorkBindingIdentity,OriginalWorkBorrow,OriginalWorkPerformedScope,OriginalWork,OriginalWorkState,OriginalWorkTurn,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkAuthority,OriginalWorkLoan,OriginalWorkRecipient,OriginalWorkScope,OriginalWorkScopeIdentity,OriginalWorkIssuerIdentity,OriginalWorkActionIdentity,OriginalWorkActionReservation,OriginalWorkActionProof,OriginalWorkActionIssuer,OriginalWorkPerformedReceipt,OriginalWorkPerformedIdentity,OriginalWorkSink,OriginalWorkLender,OriginalWorkLenderLoan,OriginalWorkPartialOutcome,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete};

pub use semio_framework_value_source_turn::{OriginalSourceTurnIssuerIdentity,OriginalSourceTurnIdentity,OriginalSourceTurnError,OriginalSourceTurnIssuer,OriginalSourceTurn,OriginalSourceTurnWitness};

#[path="🏭️producer/🦀️.rs"] mod producer;
pub use producer::{OriginalEffectBirthContext,OriginalEffectBirthProducer,OriginalEffectInvocation,OriginalEffectInvocationIdentity,OriginalEffectAction,OriginalConstructionEffectProof,OriginalEffectSuccessProof,OriginalEffectAuthority,OriginalEffectCallbackMode,OriginalEffectProducer};

pub use semio_framework_value_control_reply::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalEffectReplyScope};
