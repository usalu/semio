//! 🔺️ Sparse diff builder for `DeletePart` — a real cascade-aware removal (part + any fastener
//! that touches one of its grips), never a whole-snapshot capture. Grip full ids are `part_id:grip_id`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dFastenersDelta, Puzzle5dPartsDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeletePart, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some((at, part)) = base.parts.iter().enumerate().find(|(_, entry)| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part", payload.id), vec![payload.id.clone()]);
    };
    let grip_ids: Vec<String> = part.grips.iter().map(|grip| format!("{}:{}", part.id, grip.id)).collect();
    let severed: Vec<(String, usize)> = base.fasteners.iter().enumerate().filter(|(_, fastener)| grip_ids.contains(&fastener.source) || grip_ids.contains(&fastener.target)).map(|(index, fastener)| (fastener.id.clone(), index)).collect();
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta::removal_by_id(payload.id.clone(), at)),
        fasteners: if severed.is_empty() { None } else { Some(Puzzle5dFastenersDelta::removals_by_id(severed)) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
