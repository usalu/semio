//! ↩️ Inverse for `ChangeObjectKind` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ChangeObjectKind;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeObjectKind>>(payload: &super::mutation::ChangeObjectKind, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::change_object_kind::mutation::change_object_kind(item.id.clone(), item.object_kind.clone())]

    })())
}
//#endregion 🔖️Inverse
