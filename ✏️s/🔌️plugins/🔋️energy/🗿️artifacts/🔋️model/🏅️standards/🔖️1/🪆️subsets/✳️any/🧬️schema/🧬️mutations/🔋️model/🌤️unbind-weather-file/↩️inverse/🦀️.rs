//! ↩️ Inverse for `UnbindWeatherFile` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::UnbindWeatherFile, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::UnbindWeatherFile> + From<crate::BindWeatherFile>,
{
    Ok((|| {
    let _ = payload;
    match &base.weather_link {
        Some(existing) => vec![vocabulary::bind_weather_file(existing.target.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
