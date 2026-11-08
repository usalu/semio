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

#[path="../../../🧬️retained-clone/🧬️contract/🔤️string-cursor/🧪️tests/🦀️.rs"]
mod string_output_tests;

/// 🧾️ Testkit admission of an owner that needs several advance turns inspects every Work verdict and settles.
#[test]
fn testkit_admission_of_a_multi_turn_owner_settles_without_a_stale_work_verdict() {
    let owner: Vec<String> = (0..8).map(|index| format!("owner-{index}")).collect();
    let (source, progress, work) = semio_framework_value::testing::admit_owned_source(owner, ());
    assert!(work > 1 && progress.copied_items >= 1);
    drop(source);
}
