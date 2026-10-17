//! ↩️ Inverse for `SetHeadingLevel`.

#[allow(unused_imports)]
use crate::schema::snapshot::DocBlock;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_heading_level;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetHeadingLevel, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_heading_level::SetHeadingLevel>,
{
    let super::SetHeadingLevel { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(DocBlock::Heading { level, .. }) => vec![Op::from(set_heading_level::SetHeadingLevel { path: path.clone(), level: *level })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
