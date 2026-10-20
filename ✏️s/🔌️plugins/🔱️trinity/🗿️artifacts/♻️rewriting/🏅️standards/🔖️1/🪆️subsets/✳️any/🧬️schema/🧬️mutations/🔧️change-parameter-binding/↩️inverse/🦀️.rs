//! ↩️ Inverse for `ChangeParameterBinding` — the OLD value looked up from BASE: `change` back to it
//! if the key existed, `remove` if it was previously absent.
use crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::change_parameter_binding;
use crate::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::remove_parameter_binding;

use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ChangeParameterBinding, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::ChangeParameterBinding> + From<crate::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::RemoveParameterBinding>,
{
    Ok((|| {
    match base.parameter_bindings.get(&payload.key) {
        Some(old) => vec![change_parameter_binding(payload.key.clone(), old.clone())],
        None => vec![remove_parameter_binding(payload.key.clone())],
    }

    })())
}
//#endregion 🔖️Inverse
