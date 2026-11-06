//! 📝️ Original semantic UTF8 and inline text own exact physical payload admission.
#[cfg(test)] extern crate self as semio_framework_value_paged_text;
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,retirement};
pub use semio_framework_value_paged_list::list;
pub use semio_framework_value_native_encoding::{native_encoding,NativeEncodeControl};
pub use semio_framework_value_native_decoding::{native_decoding,NativeDecodeControl};
#[path="../../🦀️.rs"]
pub mod paged_text;
pub use paged_text::*;
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
#[path="../../🧪️tests/native-rig.rs"]
mod staged_rig;
#[cfg(test)]
#[path="../../../../🛫️encode/🧪️tests/📋️paged/🦀️.rs"]
mod original_paged_encoding_tests;
#[cfg(test)]
#[path="../../../../🛬️decode/🧪️tests/📝️paged/🦀️.rs"]
mod original_paged_decoding_tests;
