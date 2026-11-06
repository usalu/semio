//! 📋️ Original vector ownership, paid extraction and cancellation.
pub use semio_framework_value_retirement::*;
pub use semio_framework_value_retirement::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled,erased_cursor_turn};
#[path="../../🦀️.rs"]
mod original_vector;
pub use original_vector::{OriginalVectorCursor,OriginalVectorRetirementSeed};
#[cfg(test)]
pub use semio_framework_value_testing_allocator as allocation_testing;
