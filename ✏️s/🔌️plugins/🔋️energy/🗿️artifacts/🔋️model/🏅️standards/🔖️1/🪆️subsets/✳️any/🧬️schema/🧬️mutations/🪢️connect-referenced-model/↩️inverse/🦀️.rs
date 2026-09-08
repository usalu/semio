//! ↩️ Inverse for `ConnectReferencedModel` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ConnectReferencedModel, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Ok(target) = store::os_io::ArtifactRef::parse_uri(&payload.target_uri) else {
        return Vec::new();
    };
    let link = store::ArtifactLink { target, pin: store::LinkPin::Head, role: vocabulary::REFERENCED_MODEL_LINK_ROLE.to_string() };
    match &base.referenced_model {
        Some(existing) if existing == &link => Vec::new(),
        Some(existing) => vec![vocabulary::connect_referenced_model(existing.target.to_uri())],
        None => vec![vocabulary::disconnect_referenced_model()],
    }
}
//#endregion 🔖️Inverse
