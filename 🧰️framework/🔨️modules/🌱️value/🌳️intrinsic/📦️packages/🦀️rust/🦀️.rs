//! 🌳️ Intrinsic values and their own wire traits preserve exact admission and typed edits.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_native_decoding::{native_decoding,NativeDecodeControl,NativeDecodeProgress};
pub use semio_framework_value_native_encoding::{native_encoding,NativeEncodeControl};
pub use semio_framework_value_retirement::{retirement,artifact_retire_leaf};
pub use semio_framework_io_base64::{base64_standard_encode,base64_standard_decode};
pub use serde;
pub use serde_json;
#[path="../../🦀️.rs"]
pub mod value;
pub use value::*;
#[path="../../../🧬️bytes/🦀️.rs"]
pub mod bytes;
#[path="../../../♻️retirement/🌱️dynamic/🦀️.rs"]
mod dynamic_retirement;
#[path="../../../🧬️retained-clone/🌐️wire/🦀️.rs"]
mod retained_currency_wire;
#[cfg(test)]
pub use semio_framework_value_derive::{ToValue,FromValue};
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

#[path="../../../📋️list/🌐️wire/🦀️.rs"]
mod paged_list_wire;
pub use paged_list_wire::{append_paged_cold,paged_value_index};
#[path="../../../⚠️refusal/🔁️codec/🦀️.rs"]
mod refusal_wire;
