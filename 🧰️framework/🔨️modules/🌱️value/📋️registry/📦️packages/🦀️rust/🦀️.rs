//! 📋️ Fixed-key slot and request-owner registries with exact key lookup — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
use semio_framework_value_retained_clone::retained_clone::{self, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value_retirement::RetirementDemand;

#[path = "../../🎫️slots/🦀️.rs"]
pub mod fixed_key_slots;
#[path = "../../🎫️requests/🦀️.rs"]
pub mod fixed_request_owners;
#[path = "../../../🔑️lookup/🦀️.rs"]
pub mod exact_key_lookup;

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
