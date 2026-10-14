//! 🔺️ Sparse diff builder for `BindWeatherFile` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyLinkSlotDelta;
use crate::diff::EnergyModelDiff;
use crate::mutations as vocabulary;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::BindWeatherFile, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.target.artifact_id.is_empty() || payload.target.dialect.artifact_kind.is_empty() || payload.target.dialect.standard.is_empty() || payload.target.dialect.subset.is_empty() { return protocol::MutationOutcome::fatal("mutation.invariant", "Artifact reference identity components must be nonempty.", ["target"]); }
    let target = payload.target.clone();
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: vocabulary::WEATHER_LINK_ROLE.to_string() };
    if base.weather_link.as_ref() == Some(&link) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The weather file is already bound to {}.", payload.target.artifact_id));
    }
    protocol::MutationOutcome::new(EnergyModelDiff { weather_link: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() })
}
//#endregion 🔖️Diff
