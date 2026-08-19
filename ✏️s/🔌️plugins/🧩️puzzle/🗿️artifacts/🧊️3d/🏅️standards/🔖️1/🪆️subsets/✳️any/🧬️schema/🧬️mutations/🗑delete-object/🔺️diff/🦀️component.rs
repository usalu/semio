//! 🔺️ Sparse diff builder for `DeleteObject` — a real cascade-aware removal (object + any
//! attraction that touches one of its vortices), never a whole-snapshot capture. Full vortex ids
//! are `object_id:vortex_id`.
use crate::artifacts::puzzle3d::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff, Puzzle3dObjectsDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteObject, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object", payload.id), vec![payload.id.clone()]);
    };
    let vortex_ids: Vec<String> = object.vortices.iter().map(|vortex| format!("{}:{}", object.id, vortex.id)).collect();
    let severed: Vec<String> = base
        .attractions
        .iter()
        .filter(|attraction| vortex_ids.contains(&attraction.attracting) || vortex_ids.contains(&attraction.attracted))
        .map(|attraction| attraction.id.clone())
        .collect();
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        attractions: if severed.is_empty() { None } else { Some(Puzzle3dAttractionsDelta { removed: severed, ..Default::default() }) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
