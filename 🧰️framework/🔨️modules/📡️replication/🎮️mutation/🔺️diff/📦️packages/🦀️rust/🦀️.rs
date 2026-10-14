//! 🔺️ Genuine lower owner of the mutation diff algebra and its sealed apply capability.
pub use semio_framework_value_intrinsic as value;
pub use semio_framework_value_intrinsic as __value;
pub use semio_framework_value_intrinsic::{DslValue,ToValue,FromValue,ValueError,ValueRefusalKind};
#[path="../../🦀️.rs"]
pub mod diff;
pub use diff::*;
#[cfg(test)]
#[path="../../🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="../../../../../🌱️value/🧪️testing/💾️allocator/🦀️.rs"]
mod allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
