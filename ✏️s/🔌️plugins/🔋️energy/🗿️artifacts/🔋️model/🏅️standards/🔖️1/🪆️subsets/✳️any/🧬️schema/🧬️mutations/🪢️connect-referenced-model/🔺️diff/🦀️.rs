//! 🔺️ Sparse diff builder for `ConnectReferencedModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyLinkSlotDelta;
use crate::diff::EnergyModelDiff;
use crate::mutations as vocabulary;

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectReferencedModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Ok(target) = store::os_io::ArtifactRef::parse_uri(&payload.target_uri) else {
        return protocol::MutationOutcome::error("mutation.invariant", format!("{:?} is not an artifact reference URI.", payload.target_uri), [payload.target_uri.clone()]);
    };
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: vocabulary::REFERENCED_MODEL_LINK_ROLE.to_string() };
    if base.referenced_model.as_ref() == Some(&link) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The referenced model is already {}.", payload.target_uri));
    }
    protocol::MutationOutcome::new(EnergyModelDiff { referenced_model: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() })
}
//#endregion 🔖️Diff
