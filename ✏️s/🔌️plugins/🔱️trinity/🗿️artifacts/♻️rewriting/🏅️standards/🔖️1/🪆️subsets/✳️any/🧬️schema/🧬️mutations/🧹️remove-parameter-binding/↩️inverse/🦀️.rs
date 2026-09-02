//! ↩️ Inverse for `RemoveParameterBinding` — the OLD value looked up from BASE, restored via
//! `change-parameter-binding`. Missing key ⇒ `Vec::new()`.
use crate::artifacts::rewriting::mutations::{change_parameter_binding, RewriteRuleMutation};
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveParameterBinding, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    base.parameter_bindings.get(&payload.key).map(|old| vec![change_parameter_binding(payload.key.clone(), old.clone())]).unwrap_or_default()
}
//#endregion 🔖️Inverse
