//! ⏱️ Hidden bounded reconstruction continuation command.

use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::commands::run_reconstruction;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️AdvanceReconstruction
pub use run_reconstruction::AdvanceReconstruction;

/// ⏱️ Delegates one generation-checked unit to the shared reconstruction session.
pub async fn handle(payload: &AdvanceReconstruction, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    run_reconstruction::advance_reconstruction(payload, doc)
}
//#endregion 🔖️AdvanceReconstruction
