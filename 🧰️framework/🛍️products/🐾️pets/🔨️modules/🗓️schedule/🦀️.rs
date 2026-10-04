//! 🗓️ The schedule of the stage: the changes that are due at a known tick and depend on more than one actor — the next whole second on which a pair may be drawn (`pairing_tick`) and the next on which somebody who waits off stage may arrive (`arrival_tick`).
//!
//! The clock jumps to these ticks (`lull`) and the frame reports them (`wake`). A new time horizon of the stage is a function here and a line in both of them.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../🗓️schedule/🟦️.ts — the TypeScript twin

use crate::behavior::MODE_LIMITS;
use crate::draft::index_of;
use crate::schema::{Activity, Actor, Menagerie, PetMode, Stage, Ticks, TICKS_PER_SECOND};
use std::borrow::Borrow;

//#region 🔖️Constants
const WARMUP: Ticks = 1280;
//#endregion 🔖️Constants

//#region 🔖️Schedule
/// 🙌️ Whether an actor can be drawn into an encounter: standing idle and whole on a perch, without a partner, not leaving.
pub(crate) fn sociable(actor: &Actor) -> bool {
    actor.activity == Activity::Idle && actor.partner.is_none() && !actor.leaving && actor.opacity == 1.0 && actor.perch.is_some()
}

/// 📅️ The first whole second at or after tick `from` on which the stage may draw a pair, or −1 while it may not: the mode has encounters, it is not a quiet time, nobody has a partner, at least two actors are sociable, and the warm-up (20 s before the first encounter) or the gap of the mode since the last change of a rapport has passed.
pub(crate) fn pairing_tick<A: Borrow<Actor>>(stage: &Stage, actors: &[A], from: Ticks) -> Ticks {
    let limits = MODE_LIMITS[stage.mode];
    if limits.encounter_gap == 0 || stage.quiet {
        return -1;
    }
    let mut free = 0;
    for actor in actors {
        let actor: &Actor = actor.borrow();
        if actor.partner.is_some() {
            return -1;
        }
        if sociable(actor) {
            free += 1;
        }
    }
    if free < 2 {
        return -1;
    }
    let open = if stage.met == 0 { WARMUP } else { stage.met + limits.encounter_gap };
    let first = if from > open { from } else { open };
    (first + TICKS_PER_SECOND - 1).div_euclid(TICKS_PER_SECOND) * TICKS_PER_SECOND
}

/// 🚏️ The first whole second at or after tick `from` on which a species that is wanted but not on stage may arrive, or −1 while nobody waits, no perch exists or the stage is still (a still stage lets them arrive with its events).
pub(crate) fn arrival_tick<A: Borrow<Actor>>(menagerie: &Menagerie, stage: &Stage, actors: &[A], from: Ticks) -> Ticks {
    if stage.mode == PetMode::Still || stage.perches.is_empty() {
        return -1;
    }
    if menagerie.species.iter().any(|kind| stage.wanted.contains(&kind.id) && index_of(actors, &kind.id).is_none()) {
        return (from + TICKS_PER_SECOND - 1).div_euclid(TICKS_PER_SECOND) * TICKS_PER_SECOND;
    }
    -1
}
//#endregion 🔖️Schedule

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
