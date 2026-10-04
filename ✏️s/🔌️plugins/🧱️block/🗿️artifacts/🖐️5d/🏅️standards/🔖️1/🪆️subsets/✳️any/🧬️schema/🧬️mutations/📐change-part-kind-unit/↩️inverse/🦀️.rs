//! ↩️ Inverse for `ChangePartKindUnit`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangePartKindUnit, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_part_kind_unit::change_part_kind_unit(base.part_kind.unit.clone())]

    })())
}
//#endregion 🔖️Inverse
