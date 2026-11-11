//! 🧫️ Neutral host component package glue.
extern crate semio_os_kernel as dsl;
#[path = "../../🧬️schema/📸️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::Snapshot;
#[path = "../../🧬️schema/🔺️diff/🦀️.rs"]
mod diff;
pub use diff::Diff;
#[path = "../../🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::Mutation;
#[path = "../../🦀️.rs"]
mod assembly;
semio_plugin::plugin_exports!(
    semio_plugin::MountedOwnerPolicyV1 {
        preparation: semio_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 },
        maintenance: semio_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 },
        close: semio_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 },
    },
    assembly::plugin,
    assembly::FixtureApps
);

#[cfg(test)]
#[path ="../../🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;
