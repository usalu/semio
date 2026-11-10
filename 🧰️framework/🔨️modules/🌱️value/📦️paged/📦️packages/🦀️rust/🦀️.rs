//! 📦️ Paged owners sit below the value facade; the facade re-exports them as `paged`.
extern crate self as semio_framework_value;
pub use semio_framework_value_intrinsic::bytes;
pub use semio_framework_value_native_encoding::native_encoding;
pub use native_encoding::{NativeEncodeControl,NativeEncodeContinuation,NativeScopedEncodeContinuation,NativeScopedEncodePauseFailure};
pub mod refusal{pub use semio_framework_value_control_contract::{ValueError,ValueRefusalKind};}
pub use refusal::{ValueError,ValueRefusalKind};
pub use semio_framework_value_intrinsic::{equal_f32,equal_f64,DslValue,DslValueSource,Number,JsonInteger,json_integer,edit_through_value,ControlledValueHasher,DecodedValue,IntrinsicRetirement,FromValue,ToValue,ValueEdit,ValueShape};
pub use semio_framework_value_intrinsic::{controlled_sequence,path_index,dsl_value,NativeDecodeControl,NativeDecodeProgress};
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_retirement::retirement_contract::RetirementPollExit;
pub use semio_framework_value_retirement::retirement::RetireOwned;
pub use semio_framework_value_native_owned::native_decoding;
pub use native_decoding::OriginalReceivingPrimary;
pub use semio_framework_value_native_owned::{NativeReceive,NativeReceiver};
pub use semio_framework_value_retirement::{RetainedWorkOutcome,ErasedSnapshotRetirement,OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetirementDemand,close_factory_ticket,close_factory_ticket_original,close_factory_ticket_controlled};
pub use semio_framework_value_control_contract::{RetainedWorkPoll,RetainedAllocationEffect,OriginalEffectCallbackMode,NativeAllocationIdentity,OriginalWorkBindingIdentity,OriginalEffectInvocationIdentity};
pub use semio_framework_value_retirement::OriginalRetirementCloseLoan;
pub use semio_framework_value_retained_clone::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneSource,RetainedCloneStep};
pub use semio_framework_value_original_slot::initialization;
pub use semio_framework_value_factory as factory;
pub use semio_framework_value_factory::{FactoryPayloadRetirement,FactoryRetirement,FactoryAuthority,SnapshotRetirementFactory};
#[path="🧬️retained-clone/🦀️.rs"]
pub mod retained_clone;
#[path="../../🦀️.rs"]
pub mod paged;
pub use paged::*;
