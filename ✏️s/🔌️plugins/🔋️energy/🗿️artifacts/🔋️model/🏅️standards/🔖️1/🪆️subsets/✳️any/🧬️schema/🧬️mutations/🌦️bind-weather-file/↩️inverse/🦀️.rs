//! ↩️ Inverse for `BindWeatherFile` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::BindWeatherFile, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Ok(target) = store::os_io::ArtifactRef::parse_uri(&payload.target_uri) else {
        return Vec::new();
    };
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: vocabulary::WEATHER_LINK_ROLE.to_string() };
    match &base.weather_link {
        Some(existing) if existing == &link => Vec::new(),
        Some(existing) => vec![vocabulary::bind_weather_file(existing.target.to_uri())],
        None => vec![vocabulary::unbind_weather_file()],
    }
}
//#endregion 🔖️Inverse
