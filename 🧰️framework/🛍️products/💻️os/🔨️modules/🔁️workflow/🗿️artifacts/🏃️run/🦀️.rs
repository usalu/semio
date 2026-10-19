//! 🏃️ Composable persisted workflow run artifact.
use semio_framework_os_kernel::ColdMutation;
use semio_framework_os_kernel::ColdMutationDiff;
use semio_framework_os_kernel::ColdDiffAlgebra;
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
pub use mutations::{AppendRunLog, FinishRunNode, RemoveRunLog, RemoveRunNode, RunMutation, SealRun, SetRunHeader, SetRunSeal, StartRun, StartRunNode};

#[path = "🚪️io/🦀️.rs"]
pub mod io;
