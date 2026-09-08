//! 🏃️ Composable persisted workflow run artifact.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

/// 🪪️ Persisted workflow run schema identifier.
pub const S_RUN_SCHEMA: &str = "os.run";

#[path = "🧬️schema/📸️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::*;
#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod diff;
pub use diff::RunDiff;
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::{AppendRunLog, FinishRunNode, RunMutation, SealRun, StartRun, StartRunNode};
#[path = "🧬️schema/🧬️mutations/⚡️apply/🦀️.rs"]
mod apply;
pub use apply::{apply_run_operation, apply_run_operation_checked};

#[cfg(test)]
#[path = "🧪️tests/🏃️run/🦀️.rs"]
mod tests;
