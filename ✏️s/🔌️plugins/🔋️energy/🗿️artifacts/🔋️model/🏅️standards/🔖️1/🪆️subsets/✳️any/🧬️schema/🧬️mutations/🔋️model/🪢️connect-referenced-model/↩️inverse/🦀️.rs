//! ↩️ Inverse for `ConnectReferencedModel` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ConnectReferencedModel, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ConnectReferencedModel> + From<crate::DisconnectReferencedModel>,
{
    Ok((|| {
    if !::semio_s_artifact_energy_model_native_mutations_singletons::root::valid_link(&payload.link) { return Vec::new(); }
    let link = payload.link.clone();
    match &base.referenced_model {
        Some(existing) if existing == &link => Vec::new(),
        Some(existing) => vec![vocabulary::connect_referenced_model(existing.clone())],
        None => vec![vocabulary::disconnect_referenced_model()],
    }

    })())
}
//#endregion 🔖️Inverse
