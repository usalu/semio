//! ↩️ Inverse for `ChangeSnapEnabled`.
use super::ChangeSnapEnabled;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeSnapEnabled, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::ChangeSnapEnabled>,
{
    Ok((|| {
    vec![Op::from(ChangeSnapEnabled { new_enabled: base.snap_enabled })]

    })())
}
//#endregion 🔖️Inverse
