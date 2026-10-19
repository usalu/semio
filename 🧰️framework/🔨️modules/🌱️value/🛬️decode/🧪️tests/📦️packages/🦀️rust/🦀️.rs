//! 🧪️ Companion laws of the native decoding core, restated against its public vocabulary.
#![cfg(test)]

pub use semio_framework_value_native_decoding::*;
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_shared_issuance::{SealedShared, SealedSharedBirth, SharedIssuer};
use semio_framework_value_testing_allocator as allocation_testing;
pub(crate) use allocation_testing::observe_retirement_allocations;

#[path = "../../../📏️maximum/🧪️tests/🦀️.rs"]
mod maximum_tests;

#[path = "../../🦀️.rs"]
mod root_tests;

#[path = "../../../⚠️refusal/🧪️tests/🦀️.rs"]
mod literal_refusal_tests;

#[path = "../../../🔁️ledger/🧪️tests/🦀️.rs"]
mod original_ledger_tests;

#[path = "../../../🛂️allocation/🧪️tests/🦀️.rs"]
mod allocation_tests;

#[path = "../../../🏭️construction/🧪️tests/🦀️.rs"]
mod construction_tests;

#[path = "../../../♻️retirement/🧪️tests/🦀️.rs"]
mod retirement_tests;

#[path = "../../../../🫴️receiving/🧪️tests/🦀️.rs"]
mod original_receiving_tests;

