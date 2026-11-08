//! 🎛️ Public original retirement work laws exercise the canonical owner in a dedicated native binary.
pub use semio_framework_value_retirement::*;
pub use semio_framework_value_retirement::retirement::*;
pub use semio_framework_value_retirement::retirement::controlled::*;
pub use semio_framework_value_factory::{FactoryPayloadRetirement,FactoryRetirement,FactoryAuthority,FactoryChildTickets,FactoryChildSlot,FactoryRetirementTicket,factory_arc_birth_bytes,factory_retirement_frame_bytes,factory_constructor_birth_bytes};
pub use semio_framework_value_factory::owned::FactoryOwnedRetirement;
pub(crate) use semio_framework_value_testing_allocator as allocation_testing;
pub(crate) use std::mem::size_of;
pub use semio_framework_value::retained_clone;
#[path="../../../🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
#[path="🦀️.rs"]
mod original_retirement_work_tests;
