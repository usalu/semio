//! 🧬️ Intrinsic collection clone producers compose actual generic clone and custody contracts.
pub use semio_framework_value_retained_clone::retained_clone::*;
pub use semio_framework_value_retained_map::ordered_map;
#[path="📦️paged/🦀️.rs"]
pub mod paged;

#[cfg(test)]
#[path="🔗️projection/🧪️tests/🔬️unit/🦀️.rs"]
mod original_projection_tests;
#[cfg(test)]
#[path="🔗️source/🧪️tests/🔬️unit/🦀️.rs"]
mod source_custody_tests;
#[cfg(test)]
#[path="🔗️source/🧪️tests/🏭️factory/🦀️.rs"]
mod original_factory_source_tests;
#[cfg(test)]
#[path="🔗️source/🧪️testing/🦀️.rs"]
mod original_source_fixture;
#[cfg(test)]
pub(crate) use original_source_fixture::{FixtureSource,FixtureSourceBirth};
