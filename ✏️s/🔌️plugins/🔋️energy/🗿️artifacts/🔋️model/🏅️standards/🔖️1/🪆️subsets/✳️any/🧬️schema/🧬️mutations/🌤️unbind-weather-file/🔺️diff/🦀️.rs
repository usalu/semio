//! 🔺️ Sparse diff builder for `UnbindWeatherFile` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyLinkSlotDelta;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::UnbindWeatherFile, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = &base.weather_link else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No weather file is bound to this energy model.", Vec::<String>::new());
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff { weather_link: Some(EnergyLinkSlotDelta::Detached), ..Default::default() })
}
//#endregion 🔖️Diff
