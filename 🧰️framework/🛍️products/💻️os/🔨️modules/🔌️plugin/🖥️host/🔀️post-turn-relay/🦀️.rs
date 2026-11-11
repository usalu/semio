//! 🔀️ Source-owned guest operations and their original result recipients.
pub use semio_plugin_host_source_json::{SourceJsonEncoder,OriginalJobBudgetJson};
pub use semio_framework_actor::ActorId as RuntimeActorId;
use semio_plugin_host_guest_runtime::{GuestInstance,GuestRuntimes};
pub use semio_plugin_host_wake::{GuestRelayWakeDriver,GuestRelayAdmissionFailure};
#[path="📤️output/🦀️.rs"]
mod original_output;
#[path="📤️output/🧯️actor-failure.rs"]
mod actor_failure;
pub use actor_failure::ActorFailureCapsule;
pub use original_output::{GuestRelayOutput,GuestRelayOutputHome,GuestRelayFaultHome,GuestRelayFault,GuestRelayResult};
#[path="🎟️unit/🌱️canonical.rs"]
mod original_unit_flow;
pub use original_unit_flow::{GuestRelayUnitFlow,GuestRelayUnitStatus,GuestRelayJobInput,GuestRelayInferenceInput,GuestRelayInferenceSource,ColdJobCallPlan,GuestRelayJsonSource,GuestRelayByteSource,GuestRelayBorrowedJson,GuestRelayBorrowedBytes,GuestRelayJsonTransform,GuestRelayTransformedOutput,GuestRelayColdController};
#[path="🏠️native/🦀️.rs"]
mod native_home;
pub use native_home::{with_bootstrap_history_parts,attest_bootstrap_header_return,with_frozen_history_parts,attest_header_return,attest_actor_failure_if_needed,FrozenReceiptFrontier,GuestRelayNativeHome,GuestRelayNativeOrigin,GuestRelayNativeResolver,GuestRelayNativeCustody,GuestRelayNativeState,NativeInstanceHome,NativeInstanceRetirement};
#[path="🌱️native-admission/🦀️.rs"]
mod native_admission;
pub use native_home::origin_registry;
pub use native_admission::PluginInstanceHandleAdmission;
#[path="🌱️native-admission/🏠️generic.rs"]
mod native_home_admission;
pub use native_home_admission::NativeHomeAdmission;
#[path="🔌️plugin-instance-handle/🦀️.rs"]
mod plugin_instance_handle;
pub use plugin_instance_handle::PluginInstanceHandle;
#[path="🔌️plugin-instance-handle/📦️operation.rs"]
mod operation;
pub use operation::{GuestRelayOperation,GuestRelayOperationStatus};
#[path="🔌️plugin-instance-handle/🎫️operation-admission.rs"]
mod operation_admission;
pub use operation_admission::GuestRelayOperationAdmission;
