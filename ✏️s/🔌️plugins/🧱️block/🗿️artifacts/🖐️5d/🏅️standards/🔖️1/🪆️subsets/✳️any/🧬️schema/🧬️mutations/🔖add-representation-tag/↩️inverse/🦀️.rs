//! ↩️ Inverse for `AddRepresentationTag`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddRepresentationTag, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_representation_tag::remove_representation_tag(payload.id.clone(), payload.tag.clone())]
}
//#endregion 🔖️Inverse
