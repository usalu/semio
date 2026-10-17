//! ↩️ Inverse for `RemoveMetadataEntry`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_metadata_entry;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveMetadataEntry, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_metadata_entry::SetMetadataEntry>,
{
    let super::RemoveMetadataEntry { key } = payload;
    Ok(vec![match base.metadata.iter().position(|e| &e.key == key) {
        Some(at) => Op::from(set_metadata_entry::SetMetadataEntry { key: key.clone(), value: base.metadata[at].value.clone(), at: Some(at) }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
