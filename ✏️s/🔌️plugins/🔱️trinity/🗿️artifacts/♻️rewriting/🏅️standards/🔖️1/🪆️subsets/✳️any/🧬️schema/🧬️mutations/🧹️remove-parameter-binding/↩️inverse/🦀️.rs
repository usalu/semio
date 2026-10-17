//! ↩️ Inverse for `RemoveParameterBinding` — the OLD value looked up from BASE, restored via
//! `change-parameter-binding`. Missing key ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::change_parameter_binding;

use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::RemoveParameterBinding, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::ChangeParameterBinding>,
{
    Ok((|| {
    base.parameter_bindings.get(&payload.key).map(|old| vec![change_parameter_binding(payload.key.clone(), old.clone())]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
