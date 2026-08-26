//! ↩️ Inverse for `ChangeParameterBinding` — the OLD value looked up from BASE: `change` back to it
//! if the key existed, `remove` if it was previously absent.
use crate::artifacts::rewrite::mutations::{change_parameter_binding, remove_parameter_binding, RewriteRuleMutation};
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeParameterBinding, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
    match base.parameter_bindings.get(&payload.key) {
        Some(old) => vec![change_parameter_binding(payload.key.clone(), old.clone())],
        None => vec![remove_parameter_binding(payload.key.clone())],
    }
}
//#endregion 🔖️Inverse
