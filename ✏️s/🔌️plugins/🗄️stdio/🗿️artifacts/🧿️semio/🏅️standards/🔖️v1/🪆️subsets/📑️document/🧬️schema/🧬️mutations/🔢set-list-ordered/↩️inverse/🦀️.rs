//! ↩️ Inverse for `SetListOrdered`.

#[allow(unused_imports)]
use crate::schema::snapshot::DocBlock;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_list_ordered;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetListOrdered, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_list_ordered::SetListOrdered>,
{
    let super::SetListOrdered { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(DocBlock::List { ordered, .. }) => vec![Op::from(set_list_ordered::SetListOrdered { path: path.clone(), ordered: *ordered })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
