//! ↩️ Inverse for `SetMetadataEntry`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_metadata_entry;
#[allow(unused_imports)]
use crate::schema::mutations::set_metadata_entry;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetMetadataEntry, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_metadata_entry::RemoveMetadataEntry> + From<crate::schema::mutations::set_metadata_entry::SetMetadataEntry>,
{
    let super::SetMetadataEntry { key, .. } = payload;
    Ok(vec![match base.metadata.iter().find(|e| &e.key == key) {
        Some(entry) => Op::from(set_metadata_entry::SetMetadataEntry { key: key.clone(), value: entry.value.clone(), at: None }),
        None => Op::from(remove_metadata_entry::RemoveMetadataEntry { key: key.clone() }),
    }])
}
//#endregion 🔖️Inverse
