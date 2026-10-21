//! 🗂️ Immutable ordered roots own actual path copies, shared aliases and retirement.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_native_decoding::NativeDecodeControl;
pub use serde;
#[path="../../🦀️.rs"]
pub mod ordered;
pub use ordered::*;
use semio_framework_value_intrinsic::{DecodedValue,DslValue,FromValue,NativeEncodeControl,ToValue};
#[path="../../🌐️wire/🦀️.rs"]
mod ordered_value_wire;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
