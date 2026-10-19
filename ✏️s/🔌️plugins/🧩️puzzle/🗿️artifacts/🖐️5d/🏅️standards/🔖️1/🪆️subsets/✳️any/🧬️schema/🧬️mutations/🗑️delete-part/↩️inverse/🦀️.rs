//! 🔩️ re-`connect-grips`es every fastener BASE shows touching one of its grips (severed cascade).
//! ↩️ Inverse for `DeletePart` — reconstructs a `create-part` of the captured BASE part, then
//! Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::DeletePart, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_grips::ConnectGrips> + From<crate::standards::v1::subsets::any::schema::mutations::create_part::CreatePart>,
{
    Ok((|| {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.parts.iter().position(|entry| entry.id == payload.id);
    let grip_ids: Vec<String> = part.grips.iter().map(|grip| format!("{}:{}", part.id, grip.id)).collect();
    let mut mutations = vec![crate::standards::v1::subsets::any::schema::mutations::create_part::create_part(part.clone(), index)];
    for (at, fastener) in base.fasteners.iter().enumerate().filter(|(_, fastener)| grip_ids.contains(&fastener.source) || grip_ids.contains(&fastener.target)) {
        mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips::connect_grips(
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
            fastener.y, Some(at),
        ));
    }
    mutations.reverse();
    mutations

    })())
}
//#endregion 🔖️Inverse
