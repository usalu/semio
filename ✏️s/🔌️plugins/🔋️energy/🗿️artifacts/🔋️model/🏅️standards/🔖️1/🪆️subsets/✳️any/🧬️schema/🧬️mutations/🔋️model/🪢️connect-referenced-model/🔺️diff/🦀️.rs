//! 🔺️ Sparse diff builder for `ConnectReferencedModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyLinkSlotDelta;
use crate::diff::EnergyModelDiff;
use crate::mutations as vocabulary;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectReferencedModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.target.artifact_id.is_empty() || payload.target.dialect.artifact_kind.is_empty() || payload.target.dialect.standard.is_empty() || payload.target.dialect.subset.is_empty() { return protocol::MutationOutcome::fatal("mutation.invariant", "Artifact reference identity components must be nonempty.", ["target"]); }
    let target = payload.target.clone();
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: vocabulary::REFERENCED_MODEL_LINK_ROLE.to_string() };
    if base.referenced_model.as_ref() == Some(&link) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The referenced model is already {}.", payload.target.artifact_id));
    }
    protocol::MutationOutcome::new(EnergyModelDiff { referenced_model: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() })
}
//#endregion 🔖️Diff
