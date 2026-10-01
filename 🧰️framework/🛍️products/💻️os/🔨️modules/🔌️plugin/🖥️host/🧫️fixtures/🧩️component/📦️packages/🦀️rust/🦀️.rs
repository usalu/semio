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
semio_framework_plugin::plugin_exports!(assembly::plugin,assembly::FixtureApps);
