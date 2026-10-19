//! then re-`connect-vortices`es every attraction BASE shows touching one of its vortices (severed
//! ↩️ Inverse for `DeleteObject` — reconstructs a `create-object` of the captured BASE object,
//! cascade). Missing target ⇒ `Vec::new()`.
use super::mutation::DeleteObject;
use {semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::ConnectVortices, crate::create_object::mutation::CreateObject};
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DeleteObject> + From<ConnectVortices> + From<CreateObject>>(payload: &super::mutation::DeleteObject, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.objects.iter().position(|entry| entry.id == payload.id);
    let vortex_ids: Vec<String> = object.vortices.iter().map(|vortex| format!("{}:{}", object.id, vortex.id)).collect();
    let mut mutations = vec![crate::create_object::mutation::create_object(object.clone(), index)];
    for (at, attraction) in base.attractions.iter().enumerate().filter(|(_, attraction)| vortex_ids.contains(&attraction.attracting) || vortex_ids.contains(&attraction.attracted)) {
        mutations.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(
            attraction.id.clone(),
            attraction.attracting.clone(),
            attraction.attracted.clone(),
            attraction.gap,
            attraction.shift,
            attraction.rise,
            attraction.rotation,
            attraction.turn,
            attraction.tilt,
            attraction.x,
            attraction.y, Some(at),
        ));
    }
    mutations.reverse();
    mutations

    })())
}
//#endregion 🔖️Inverse
