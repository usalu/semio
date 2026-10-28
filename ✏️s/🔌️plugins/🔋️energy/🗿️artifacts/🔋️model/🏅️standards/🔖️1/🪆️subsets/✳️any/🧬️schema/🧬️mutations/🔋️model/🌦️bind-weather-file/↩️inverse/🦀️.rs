//! ↩️ Inverse for `BindWeatherFile` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::BindWeatherFile, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::BindWeatherFile> + From<crate::UnbindWeatherFile>,
{
    Ok((|| {
    if !::semio_s_artifact_energy_model_native_mutations::root::valid_link(&payload.link) { return Vec::new(); }
    let link = payload.link.clone();
    match &base.weather_link {
        Some(existing) if existing == &link => Vec::new(),
        Some(existing) => vec![vocabulary::bind_weather_file(existing.clone())],
        None => vec![vocabulary::unbind_weather_file()],
    }

    })())
}
//#endregion 🔖️Inverse
