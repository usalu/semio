//! ↩️ Inverse for `DeleteRepresentation`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteRepresentation, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_representation::create_representation(existing.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
