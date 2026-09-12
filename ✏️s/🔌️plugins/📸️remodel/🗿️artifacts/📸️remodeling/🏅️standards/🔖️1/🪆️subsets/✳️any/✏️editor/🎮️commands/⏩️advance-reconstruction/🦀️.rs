//! ⏱️ Hidden bounded reconstruction continuation command.

use crate::editor::remodeling::commands::run_reconstruction;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️AdvanceReconstruction
pub use run_reconstruction::AdvanceReconstruction;

/// ⏱️ Delegates one generation-checked unit to the shared reconstruction session.
pub fn handle(payload: &AdvanceReconstruction, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    run_reconstruction::advance_reconstruction(payload, doc)
}
//#endregion 🔖️AdvanceReconstruction
