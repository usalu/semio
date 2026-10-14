//! ↩️ Inverse for `ChangeObjectAnchor` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ChangeObjectAnchor;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeObjectAnchor>>(payload: &super::mutation::ChangeObjectAnchor, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::change_object_anchor::mutation::change_object_anchor(item.id.clone(), item.anchor)]

    })())
}
//#endregion 🔖️Inverse
