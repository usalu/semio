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
    let mut source = source;
    for _ in 0..10_000 {
        if source.terminal_is_empty() { break; }
        let demand = source.next_close_demand(4096).unwrap();
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        semio_framework_value_clone_source_testing::drive::warm();
        semio_framework_value_clone_source_testing::drive::drive_reply::<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value_retirement::OriginalRetirementFailure>(grant, crate::effect::CPU, |result| match result { Ok(step) => step.progress(), Err(error) => error.retained_progress() }, |effect, reply, recover| if recover { source.recover_close_into(4096, effect, reply) } else { source.close_into(4096, effect, reply) }).result.unwrap();
    }
    assert!(source.terminal_is_empty());
    drop(source);
}
