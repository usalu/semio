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
pub use diff::{RunDiff, RunHeaderEdit, RunSealEdit, RunStep};
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::{AppendRunLog, FinishRunNode, RetractRunLog, RetractRunNode, RunMutation, SealRun, SetRunHeader, SetRunSeal, StartRun, StartRunNode};
#[path = "🧬️schema/🧬️mutations/⚡️apply/🦀️.rs"]
mod apply;
pub use apply::admit_run_operation;

/// 🔒️ The one real write seam for a `RunArtifact`: a refusal travels as the outcome's own messages (codes and levels
/// unchanged), and an apply-time rejection of the same `RunDiff` ordinary application uses joins them as the `Fatal`
/// `mutation.apply.*` message the store persists.
pub async fn apply_run_operation_checked(document: &RunArtifact, operation: RunMutation) -> Result<RunArtifact, Vec<protocol::MutationMessage>> {
    let (diff, messages) = admit_run_operation(document, &operation)?;
    protocol::apply_diff(&diff, document).map_err(|error| messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect())
}

#[cfg(test)]
#[path = "🧪️tests/🏃️run/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_semantic_tests;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
