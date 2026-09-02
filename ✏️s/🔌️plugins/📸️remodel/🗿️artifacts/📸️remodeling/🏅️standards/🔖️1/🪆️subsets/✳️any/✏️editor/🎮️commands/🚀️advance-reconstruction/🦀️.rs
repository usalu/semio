//! ⏱️ Hidden bounded reconstruction continuation command.

use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::commands::run_reconstruction;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️AdvanceReconstruction
pub use run_reconstruction::AdvanceReconstruction;

/// ⏱️ Delegates one generation-checked unit to the shared reconstruction session.
pub async fn handle(payload: &AdvanceReconstruction, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    run_reconstruction::advance_reconstruction(payload, doc)
}
//#endregion 🔖️AdvanceReconstruction
