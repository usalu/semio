//! 🔺️ Sparse diff builder for `DeleteObject` — a real cascade-aware removal (object + any
//! attraction that touches one of its vortices), never a whole-snapshot capture. Full vortex ids
//! are `object_id:vortex_id`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff, Puzzle3dObjectsDelta};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteObject, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some((at, object)) = base.objects.iter().enumerate().find(|(_, entry)| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object", payload.id), vec![payload.id.clone()]);
    };
    let vortex_ids: Vec<String> = object.vortices.iter().map(|vortex| format!("{}:{}", object.id, vortex.id)).collect();
    let severed: Vec<(String, usize)> = base.attractions.iter().enumerate().filter(|(_, attraction)| vortex_ids.contains(&attraction.attracting) || vortex_ids.contains(&attraction.attracted)).map(|(index, attraction)| (attraction.id.clone(), index)).collect();
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta::removal_by_id(payload.id.clone(), at)),
        attractions: if severed.is_empty() { None } else { Some(Puzzle3dAttractionsDelta::removals_by_id(severed)) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
