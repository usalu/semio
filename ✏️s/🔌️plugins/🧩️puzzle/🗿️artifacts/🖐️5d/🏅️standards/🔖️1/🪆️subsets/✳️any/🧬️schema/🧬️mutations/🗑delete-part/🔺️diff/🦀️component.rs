//! 🔺️ Sparse diff builder for `DeletePart` — a real cascade-aware removal (part + any fastener
//! that touches one of its grips), never a whole-snapshot capture. Grip full ids are `part_id:grip_id`.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dFastenersDelta, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeletePart, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part", payload.id), vec![payload.id.clone()]);
    };
    let grip_ids: Vec<String> = part.grips.iter().map(|grip| format!("{}:{}", part.id, grip.id)).collect();
    let severed: Vec<String> = base.fasteners.iter().filter(|fastener| grip_ids.contains(&fastener.source) || grip_ids.contains(&fastener.target)).map(|fastener| fastener.id.clone()).collect();
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        fasteners: if severed.is_empty() { None } else { Some(Puzzle5dFastenersDelta { removed: severed, ..Default::default() }) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
