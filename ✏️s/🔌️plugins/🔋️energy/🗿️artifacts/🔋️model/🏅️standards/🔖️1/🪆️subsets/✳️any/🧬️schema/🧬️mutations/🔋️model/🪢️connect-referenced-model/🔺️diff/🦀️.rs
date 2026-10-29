//! 🔺️ Sparse diff builder for `ConnectReferencedModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyLinkSlotDelta;
use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectReferencedModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !::semio_s_artifact_energy_model_native_mutations_singletons::root::valid_link(&payload.link) { return protocol::MutationOutcome::fatal("mutation.invariant", "Artifact reference identity components must be nonempty.", ["link"]); }
    let link = payload.link.clone();
    if base.referenced_model.as_ref() == Some(&link) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The referenced model is already {}.", payload.link.target.artifact_id));
    }
    protocol::MutationOutcome::new(EnergyModelDiff { referenced_model: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() })
}
//#endregion 🔖️Diff
