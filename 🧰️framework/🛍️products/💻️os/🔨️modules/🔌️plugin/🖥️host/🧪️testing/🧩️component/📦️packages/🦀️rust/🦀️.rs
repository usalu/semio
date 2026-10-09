//! 🧫️ Neutral host component package glue.
extern crate semio_framework_os_kernel as dsl;
#[path="../../🧬️schema/📸️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::Snapshot;
#[path="../../🧬️schema/🔺️diff/🦀️.rs"]
mod diff;
pub use diff::Diff;
#[path="../../🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::Mutation;
#[path="../../🦀️.rs"]
mod assembly;
semio_framework_plugin::plugin_exports!(
    semio_framework_plugin::MountedOwnerPolicyV1 {
        preparation: semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 },
        maintenance: semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 },
        close: semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 },
    },
    assembly::plugin,
    assembly::FixtureApps
);

#[cfg(test)]
#[path="../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
#[cfg(test)]
#[path="../../../../../../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;
