//! 🌳️ Prepared branch headers stage and exchange through the shared original sparse patch application.
use super::{EnergyModelState,EnergyModelStatePatch};
use protocol::{RetainedDiffFragment,retained::RetainedPatchDiffApply};

/// 🌳️ A native sparse path stage supplies sealed branch candidates; only the central capability publishes their headers.
pub type PreparedEnergyBranchesApply=RetainedPatchDiffApply<EnergyModelState,EnergyModelStatePatch>;
impl RetainedDiffFragment<EnergyModelState> for EnergyModelStatePatch{type FragmentApply=PreparedEnergyBranchesApply;}
