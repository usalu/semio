//! ↩️ Inverse for `RemovePartGrip` — reconstructs an `add-part-grip` of the captured BASE grip,
//! then re-`connect-grips`es every fastener BASE shows touching it (severed cascade). Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemovePartGrip, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return Vec::new();
    };
    let Some(grip) = part.grips.iter().find(|grip| grip.id == payload.grip_id) else {
        return Vec::new();
    };
    let index = part.grips.iter().position(|g| g.id == payload.grip_id);
    let full_id = format!("{}:{}", payload.part_id, payload.grip_id);
    let mut mutations = vec![crate::artifacts::puzzle5d::mutations::add_part_grip::mutation::add_part_grip(payload.part_id.clone(), grip.clone(), index)];
    for fastener in base.fasteners.iter().filter(|fastener| fastener.source == full_id || fastener.target == full_id) {
        mutations.push(crate::artifacts::puzzle5d::mutations::connect_grips::mutation::connect_grips(
            fastener.id.clone(), fastener.source.clone(), fastener.target.clone(), fastener.fastener_kind.clone(),
            fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y,
        ));
    }
    mutations
}
//#endregion 🔖️Inverse
