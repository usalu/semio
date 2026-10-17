//! ↩️ Inverse for `RemoveNodeParam`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::param_index_at;
#[allow(unused_imports)]
use crate::schema::mutations::support::param_value_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_node_param;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveNodeParam, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_node_param::SetNodeParam>,
{
    let super::RemoveNodeParam { id, key } = payload;
    Ok(match param_value_at(base, id, key) {
        Some(value) => vec![Op::from(set_node_param::SetNodeParam { id: id.clone(), key: key.clone(), value: value.to_string(), at: param_index_at(base, id, key) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
