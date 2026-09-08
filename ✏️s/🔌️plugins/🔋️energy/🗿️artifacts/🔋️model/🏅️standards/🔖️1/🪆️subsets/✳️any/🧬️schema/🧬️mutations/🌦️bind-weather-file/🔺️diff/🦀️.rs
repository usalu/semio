//! 🔺️ Sparse diff builder for `BindWeatherFile` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyLinkSlotDelta;
use crate::diff::EnergyModelDiff;
use crate::mutations as vocabulary;

//#region 🔖️Diff
pub fn diff(payload: &super::BindWeatherFile, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Ok(target) = store::os_io::ArtifactRef::parse_uri(&payload.target_uri) else {
        return protocol::MutationOutcome::error("mutation.invariant", format!("{:?} is not an artifact reference URI.", payload.target_uri), [payload.target_uri.clone()]);
    };
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: vocabulary::WEATHER_LINK_ROLE.to_string() };
    if base.weather_link.as_ref() == Some(&link) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The weather file is already bound to {}.", payload.target_uri));
    }
    protocol::MutationOutcome::new(EnergyModelDiff { weather_link: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() })
}
//#endregion 🔖️Diff
