//! ↩️ Inverse for `DisconnectReferencedModel` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DisconnectReferencedModel, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DisconnectReferencedModel> + From<crate::ConnectReferencedModel>,
{
    Ok((|| {
    let _ = payload;
    match &base.referenced_model {
        Some(existing) => vec![vocabulary::connect_referenced_model(existing.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
