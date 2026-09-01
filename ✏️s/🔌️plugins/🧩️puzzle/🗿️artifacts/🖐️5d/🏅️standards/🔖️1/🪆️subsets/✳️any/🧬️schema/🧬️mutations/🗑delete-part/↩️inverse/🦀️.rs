//! ↩️ Inverse for `DeletePart` — reconstructs a `create-part` of the captured BASE part, then
//! re-`connect-grips`es every fastener BASE shows touching one of its grips (severed cascade).
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeletePart, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.parts.iter().position(|entry| entry.id == payload.id);
    let grip_ids: Vec<String> = part.grips.iter().map(|grip| format!("{}:{}", part.id, grip.id)).collect();
    let mut mutations = vec![crate::artifacts::puzzle5d::mutations::create_part::create_part(part.clone(), index)];
    for fastener in base.fasteners.iter().filter(|fastener| grip_ids.contains(&fastener.source) || grip_ids.contains(&fastener.target)) {
        mutations.push(crate::artifacts::puzzle5d::mutations::connect_grips::connect_grips(
            fastener.id.clone(),
            fastener.source.clone(),
            fastener.target.clone(),
            fastener.fastener_kind.clone(),
            fastener.gap,
            fastener.shift,
            fastener.rise,
            fastener.rotation,
            fastener.turn,
            fastener.tilt,
            fastener.x,
            fastener.y,
        ));
    }
    mutations
}
//#endregion 🔖️Inverse
