//! 🧩️ Original-source and bounded record schema engine.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_intrinsic::{DslValue,Number};
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{retirement,OriginalRetirementOutcome,OriginalRetirementControl,retirement_fields_type,retirement_fields_seed};
pub use semio_framework_value_native_receiver_protocol::receiving_capacity;
#[path="../../🦀️.rs"]
pub mod record;
#[path="../../🌳️source/🦀️.rs"]
pub mod source;
#[path="../../🎟️reservation/🦀️.rs"]
pub mod reservation;
