//! 🔺️ Sparse diff builder for `UnbindWeatherFile` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyLinkSlotDelta;
use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &super::UnbindWeatherFile, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.weather_link.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "No weather file is bound to this energy model.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(EnergyModelDiff { weather_link: Some(EnergyLinkSlotDelta::Detached), ..Default::default() })
}
//#endregion 🔖️Diff
