//! 🌲️ Bounded original dynamic-value tree receiving engine.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue,Number};
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary};
pub use semio_framework_value_native_receiver_protocol::{NativeReceiver,receiving_capacity};
pub use semio_framework_value_native_schema_engine::source::{NativeValueSource,NativeTreeKind,fixed,invalid,receipt};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{retirement,OriginalRetirementOutcome};
#[path="../../🦀️.rs"]
mod bulk;
pub use bulk::{NativeTreeReceiver,NativeTreeValue,NATIVE_TREE_MAXIMUM_DEPTH};
