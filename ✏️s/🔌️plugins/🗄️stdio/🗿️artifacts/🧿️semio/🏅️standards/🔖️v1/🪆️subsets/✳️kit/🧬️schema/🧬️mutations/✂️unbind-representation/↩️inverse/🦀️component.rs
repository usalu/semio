//! ↩️ `unbind-representation` — undo re-declares the pool's TAIL, not just the escrowed link.
//!
//! `bind-representation` can only APPEND (its `🔺️diff` pushes onto the end of `representations`),
//! so a lone `bind` of the escrowed link puts it back at the end of the pool rather than at the
//! index it was removed from. That reads as a pass only when the removed link happened to be the
//! last one; unbinding index 0 of a two-link pool returns `[beam-section, beam-plan]` where the
//! document began `[beam-plan, beam-section]`, and `mutate-semio-kit`'s `inverse-unbind-
//! representation` scenario is exactly that case. Removing index `i` closes the whole index space
//! above it, so the tail has to be lifted off and re-declared in order — the same remedy `🧊️obj`'s
//! `RemoveFace` needed for its membership lists (ticket 26/08/23/END-TO-END-TESTING-REFACTOR).
//!
//! The steps are therefore: `unbind` at `i` as many times as there are links after `i` (each
//! removal shifts the next one down into `i`, so one index does the whole tail), then `bind` every
//! link from `i` to the end of BASE in order — which lands the escrowed link back at `i` with its
//! successors behind it. Empty when the index was already out of range, since nothing was removed.

use super::mutation::UnbindRepresentation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{bind_representation, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &UnbindRepresentation, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    if payload.index >= base.representations.len() {
        return Vec::new();
    }
    let tail = &base.representations[payload.index..];
    let mut undo: Vec<SemioKitMutation> = (1..tail.len()).map(|_| SemioKitMutation::UnbindRepresentation(UnbindRepresentation { index: payload.index })).collect();
    undo.extend(tail.iter().map(|link| SemioKitMutation::BindRepresentation(bind_representation::mutation::BindRepresentation { target: link.target.clone(), pin: link.pin.clone(), role: link.role.clone() })));
    undo
}
//#endregion 🔖️Inverse
