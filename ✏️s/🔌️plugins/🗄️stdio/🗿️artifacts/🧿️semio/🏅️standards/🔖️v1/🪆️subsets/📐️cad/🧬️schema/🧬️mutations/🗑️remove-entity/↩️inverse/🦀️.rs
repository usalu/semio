//! ↩️ Inverse for `RemoveEntity`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::add_entity;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_entity;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveEntity, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::add_entity::AddEntity>,
{
    let super::RemoveEntity { handle } = payload;
    Ok(match find_entity(base, handle) {
        Some(e) => vec![Op::from(add_entity::AddEntity { entity: e.clone(), at: base.entities.iter().position(|entity| entity.handle == *handle) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
