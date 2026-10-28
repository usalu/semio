//! 🔺️ Sparse diff builder for `BindWeatherFile` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyLinkSlotDelta;
use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::BindWeatherFile, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !::semio_s_artifact_energy_model_native_mutations::root::valid_link(&payload.link) { return protocol::MutationOutcome::fatal("mutation.invariant", "Artifact reference identity components must be nonempty.", ["link"]); }
    let link = payload.link.clone();
    if base.weather_link.as_ref() == Some(&link) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The weather file is already bound to {}.", payload.link.target.artifact_id));
    }
    protocol::MutationOutcome::new(EnergyModelDiff { weather_link: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() })
}
//#endregion 🔖️Diff
