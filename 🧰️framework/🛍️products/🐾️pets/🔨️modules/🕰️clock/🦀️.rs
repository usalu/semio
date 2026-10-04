//! 🕰️ The clock of the stage: one tick of one actor and one tick of the stage in the normative order (`act`, `step`), the lull in which nothing changes but the tick (`lull`), and time passing over both (`pass`).
//!
//! Whatever moves, fades, turns or is scheduled must be known in three places, or it is drawn at the wrong rate or jumped over: `lull` here, and `pace_of` and the `wake` of `frame_of` in the projection.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../🕰️clock/🟦️.ts — the TypeScript twin

use crate::animation::BLINK_TICKS;
use crate::attention::{gaze_rests, heading_of, look, perk, presence_of, swivel, watched, wink, PERK_LINGER, POINTER_TICKS, TURN_REST};
use crate::choice::conclude;
use crate::draft::{remove, settle, Draft};
use crate::locomotion::{fly, plunge, stride};
use crate::population::spawn;
use crate::schedule::{arrival_tick, pairing_tick};
use crate::schema::{Activity, Menagerie, PetMode, Ticks};
use crate::sociability::{meet, pair};

//#region 🔖️Constants
const FADE_STEP: f64 = 0.0625;
pub(crate) const SHY_STEP: f64 = 0.03125;
//#endregion 🔖️Constants

//#region 🔖️Time
/// 🎬️ One tick of one actor, in the normative order; `false` when the actor is gone.
fn act(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let presence = presence_of(draft.stage.pointer, &draft.stage.actors[index], draft.kinds[index]);
    let body = &mut draft.stage.actors[index];
    if body.leaving {
        if body.activity != Activity::Walk {
            let opacity = body.opacity - FADE_STEP;
            if opacity <= 0.0 {
                remove(draft, index);
                return false;
            }
            body.opacity = opacity;
        }
    } else if body.opacity < presence {
        body.opacity = if body.opacity + FADE_STEP < presence { body.opacity + FADE_STEP } else { presence };
    } else if body.opacity > presence {
        body.opacity = if body.opacity - SHY_STEP > presence { body.opacity - SHY_STEP } else { presence };
    }
    let turns = swivel(draft, index, now);
    match draft.stage.actors[index].activity {
        Activity::Walk => {
            if !turns {
                stride(draft, index, now);
            }
        }
        Activity::Hop => {
            if !fly(draft, index, now) {
                return false;
            }
        }
        Activity::Fall => {
            if !plunge(draft, index, now) {
                return false;
            }
        }
        Activity::Sleep if watched(draft.stage.pointer, &draft.stage.actors[index], draft.kinds[index]) => settle(draft, index, now),
        Activity::Idle => perk(draft, index, now),
        _ => {}
    }
    look(draft, index, now);
    wink(draft, index, now);
    let body = &draft.stage.actors[index];
    if !body.leaving && now >= body.until {
        conclude(draft, index, now);
    }
    true
}

/// 💤️ How many of the next `left` ticks change nothing but the tick: 0 while any actor moves, turns or is about to, fades or has pupils off their target (a feeling is stored as an anchor and read in closed form, so it never stops the jump), else the ticks before the earliest scheduled change — among them, while a pointer is on stage, the tick an actor may turn to it again and the tick it has come to rest for half a second.
fn lull(menagerie: &Menagerie, draft: &Draft<'_>, left: Ticks) -> Ticks {
    let stage = &draft.stage;
    let next = stage.tick + 1;
    let mut horizon = next + left;
    for (index, body) in stage.actors.iter().enumerate() {
        let activity = body.activity;
        if body.leaving || body.opacity != presence_of(stage.pointer, body, draft.kinds[index]) || matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall) {
            return 0;
        }
        if next <= body.faced || !gaze_rests(stage, &stage.actors, &draft.kinds, index, next) {
            return 0;
        }
        if heading_of(stage, &stage.actors, &draft.kinds, index, next).is_some_and(|heading| heading != body.facing) {
            return 0;
        }
        if activity == Activity::Sleep {
            if watched(stage.pointer, body, draft.kinds[index]) {
                return 0;
            }
        } else {
            horizon = Ticks::min(horizon, body.blink + BLINK_TICKS);
        }
        horizon = Ticks::min(horizon, body.until);
        if stage.pointer.is_some() && next < body.faced + TURN_REST {
            horizon = Ticks::min(horizon, body.faced + TURN_REST);
        }
    }
    if stage.pointer.is_some() && next <= stage.pointed + PERK_LINGER {
        horizon = Ticks::min(horizon, stage.pointed + PERK_LINGER);
    }
    if stage.pointer.is_some() && next - stage.pointed < POINTER_TICKS {
        horizon = Ticks::min(horizon, stage.pointed + POINTER_TICKS);
    }
    let pairing = pairing_tick(stage, &stage.actors, next);
    if pairing >= 0 {
        horizon = Ticks::min(horizon, pairing);
    }
    let arrival = arrival_tick(menagerie, stage, &stage.actors, next);
    if arrival >= 0 {
        horizon = Ticks::min(horizon, arrival);
    }
    let skip = horizon - next;
    if skip > 0 {
        Ticks::min(skip, left)
    } else {
        0
    }
}

/// 🥁️ One tick of the stage, in the normative order.
fn step<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>) {
    let now = draft.stage.tick + 1;
    draft.stage.tick = now;
    let mut index = 0;
    while index < draft.stage.actors.len() {
        if act(draft, index, now) {
            index += 1;
        }
    }
    meet(menagerie, draft, now);
    pair(menagerie, draft, now);
    if arrival_tick(menagerie, &draft.stage, &draft.stage.actors, now) == now {
        spawn(menagerie, draft);
    }
}

/// ⏭️ Time passes: tick by tick while something can change, in one jump over every lull, over a still stage and over an empty one that nobody waits to enter.
pub(crate) fn pass<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, ticks: u64) {
    let mut left = Ticks::try_from(ticks).unwrap_or(Ticks::MAX);
    while left > 0 {
        if draft.stage.mode == PetMode::Still || (draft.stage.actors.is_empty() && arrival_tick(menagerie, &draft.stage, &draft.stage.actors, draft.stage.tick + 1) < 0) {
            draft.stage.tick += left;
            return;
        }
        let skip = lull(menagerie, draft, left);
        if skip > 0 {
            draft.stage.tick += skip;
            left -= skip;
            continue;
        }
        step(menagerie, draft);
        left -= 1;
    }
}
//#endregion 🔖️Time

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
