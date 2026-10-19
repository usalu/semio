//! 🌱️ The retained clone projection, source custody and factory laws.
pub use semio_framework_value::retained_clone::*;
pub use semio_framework_value_clone_source_testing::{FixtureSource, FixtureSourceBirth};

#[path = "../../../🧬️retained-clone/🔗️projection/🧪️tests/🔬️unit/🦀️.rs"]
mod original_projection_tests;

#[path = "../../../🧬️retained-clone/🔗️source/🧪️tests/🔬️unit/🦀️.rs"]
mod source_custody_tests;

#[path = "../../../🧬️retained-clone/🔗️source/🧪️tests/🏭️factory/🦀️.rs"]
mod original_factory_source_tests;

#[path = "📋️paged/🦀️.rs"]
pub mod paged;

#[path = "../../../🧬️retained-clone/♻️close/🎟️demand/🧪️tests/🦀️.rs"]
mod close_demand_tests;
