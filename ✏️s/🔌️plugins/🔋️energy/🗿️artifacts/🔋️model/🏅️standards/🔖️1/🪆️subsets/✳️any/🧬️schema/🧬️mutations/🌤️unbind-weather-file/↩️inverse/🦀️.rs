//! ↩️ Inverse for `UnbindWeatherFile` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::UnbindWeatherFile, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let _ = payload;
    match &base.weather_link {
        Some(existing) => vec![vocabulary::bind_weather_file(existing.target.to_uri())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
