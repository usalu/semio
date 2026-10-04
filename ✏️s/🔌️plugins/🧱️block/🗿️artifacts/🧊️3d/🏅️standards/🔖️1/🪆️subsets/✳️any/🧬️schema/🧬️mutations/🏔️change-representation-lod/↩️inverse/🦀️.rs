//! ↩️ Inverse for `ChangeRepresentationLod`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeRepresentationLod, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_lod::change_representation_lod(payload.id.clone(), existing.lod.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
