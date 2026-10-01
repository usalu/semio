//! ↩️ Inverse for `ChangeParameterBinding` — the OLD value looked up from BASE: `change` back to it
//! if the key existed, `remove` if it was previously absent.
use crate::standards::v1::subsets::any::schema::mutations::{change_parameter_binding, remove_parameter_binding, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeParameterBinding, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    match base.parameter_bindings.get(&payload.key) {
        Some(old) => vec![change_parameter_binding(payload.key.clone(), old.clone())],
        None => vec![remove_parameter_binding(payload.key.clone())],
    }
}
//#endregion 🔖️Inverse
