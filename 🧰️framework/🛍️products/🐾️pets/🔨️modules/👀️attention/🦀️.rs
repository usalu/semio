//! 👁️ Attention: what a pet does with its eyes, its head and the way it faces — how visible it wants to be under the pointer, the way it ought to face and the turn towards it, the gaze spring, the blink schedule, perking up at a pointer that has come to rest, and the lean and the squeeze a frame draws.
//!
//! Whatever the pointer does to one pet belongs here; its horizons (`POINTER_TICKS`, `TURN_REST`, `PERK_LINGER`) are read by the clock and by the frame.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../👀️attention/🟦️.ts — the TypeScript twin

use crate::animation::{spring_step, BLINK_TICKS, GAZE_DAMPING, GAZE_STIFFNESS};
use crate::behavior::{dwell_of, needs_after};
use crate::draft::{actor_key, blink_at, clip_at, facing_to, index_of, shift, Draft, ORIGIN};
use crate::randomness::{random_words, unit_of};
use crate::rig::{look_offset, BonePose, Pose};
use crate::schema::{Activity, Actor, Facing, Gaze, Point, Species, Stage, Ticks};
use crate::terrain::larger;
use crate::trigonometry::smoothstep;
use std::borrow::Borrow;

//#region 🔖️Constants
pub(crate) const POINTER_TICKS: Ticks = 256;
const GAZE_REACH: f64 = 32.0;
const GAZE_REST: f64 = 0.000244140625;
const GAZE_CALM: f64 = 0.015625;
const GAZE_AHEAD: f64 = 0.3;
const GAZE_SULK: f64 = 0.5;
const GAZE_FALL: f64 = 0.8;
const EYE_HEIGHT: f64 = 0.6;
const BLINK_AGAIN: Ticks = 7;
const WAKE_REACH: f64 = 1.5;
const SHY_OPACITY: f64 = 0.35;
const SHY_REACH: f64 = 4.0;
pub(crate) const TURN_TICKS: Ticks = 8;
pub(crate) const TURN_REST: Ticks = 56;
const TURN_CLEAR: f64 = 12.0;
const LEAN_TURN: f64 = 5.0;
const LEAN_REACH: f64 = 1.5;
const LEAN_NOD: f64 = 1.0;
pub(crate) const PERK_LINGER: Ticks = 32;
const PERK_URGE: f64 = 0.5;
const PERK_COST: f64 = 0.6;
//#endregion 🔖️Constants

//#region 🔖️Presence
/// 😴️ Whether the pointer is close enough to an actor to keep it awake: within 1.5 × its height of the middle of its body.
pub(crate) fn watched(pointer: Option<Point>, actor: &Actor, kind: &Species) -> bool {
    let Some(pointer) = pointer else {
        return false;
    };
    let height = kind.size.height;
    let dx = pointer.x - actor.x;
    let dy = pointer.y - (actor.y - height / 2.0);
    let reach = WAKE_REACH * height;
    dx * dx + dy * dy <= reach * reach
}

/// 🫣️ How visible an actor wants to be: see-through (0.35) while the pointer rests on it — inside its box, grown by 4 px — so that whoever points can see what lies beneath, else whole.
pub(crate) fn presence_of(pointer: Option<Point>, actor: &Actor, kind: &Species) -> f64 {
    let Some(pointer) = pointer else {
        return 1.0;
    };
    let reach = kind.size.width / 2.0 + SHY_REACH;
    let across = pointer.x - actor.x;
    if across > reach || across < 0.0 - reach {
        return 1.0;
    }
    if pointer.y >= actor.y - kind.size.height - SHY_REACH && pointer.y <= actor.y + SHY_REACH {
        SHY_OPACITY
    } else {
        1.0
    }
}
//#endregion 🔖️Presence

//#region 🔖️Turning
/// 🧿️ The way an actor ought to face at tick `now`, or `None` when nothing says (the twin's 0): towards its goal while it walks or hops; towards its partner while it waits for it or acts with it, away from it while it sulks; and, standing idle by itself on a perch outside a time of concentration, towards the pointer while that is worth a look (it moved within the last 4 s) and clearly on one side — more than 12 px beyond its body — once the actor has rested for 56 ticks after its last turn, so a pointer that crosses over and back never makes it flip back and forth.
pub(crate) fn heading_of<A: Borrow<Actor>>(stage: &Stage, actors: &[A], kinds: &[&Species], index: usize, now: Ticks) -> Option<Facing> {
    let actor: &Actor = actors[index].borrow();
    let activity = actor.activity;
    if matches!(activity, Activity::Walk | Activity::Hop) {
        return if actor.goal > actor.x {
            Some(Facing::Right)
        } else if actor.goal < actor.x {
            Some(Facing::Left)
        } else {
            None
        };
    }
    if let Some(partner) = actor.partner.as_deref() {
        if !matches!(activity, Activity::Idle | Activity::Greet | Activity::Cuddle | Activity::Squabble | Activity::Sulk) {
            return None;
        }
        let other = index_of(actors, partner)?;
        let towards = facing_to(Borrow::<Actor>::borrow(&actors[other]).x, actor.x);
        return Some(if activity == Activity::Sulk { towards.reversed() } else { towards });
    }
    let pointer = stage.pointer?;
    if activity != Activity::Idle || actor.perch.is_none() || actor.leaving || stage.quiet || now - stage.pointed >= POINTER_TICKS || now < actor.faced + TURN_REST {
        return None;
    }
    let across = pointer.x - actor.x;
    let clear = kinds[index].size.width / 2.0 + TURN_CLEAR;
    if across > clear {
        Some(Facing::Right)
    } else if across < 0.0 - clear {
        Some(Facing::Left)
    } else {
        None
    }
}

/// 🔄️ An actor turns to face `way`: it faces that way at once, and its drawing follows — squeezed through a line — over the next 8 ticks, up to `faced`. An actor that is still turning turns back from where its drawing is: the turn back takes as long as the turn has run.
fn turn(body: &mut Actor, way: Facing, now: Ticks) {
    if way == body.facing {
        return;
    }
    let left = if body.faced > now { body.faced - now } else { 0 };
    body.facing = way;
    body.faced = now + TURN_TICKS - left;
}
//#endregion 🔖️Turning

//#region 🔖️Gaze
/// 👀️ Where an actor wants its pupils at tick `now`, between −1 and 1 on both axes of the screen.
///
/// Asleep: the centre. Sulking: ahead and down. Else the pointer while it moved within the last 4 s and the actor
/// neither walks nor flies; else its partner; else, standing, the nearest glance point; else down while falling and
/// straight ahead otherwise. Points are looked at from 0.6 of the height above the feet with `look_offset`.
fn gaze_goal<A: Borrow<Actor>>(stage: &Stage, actors: &[A], kinds: &[&Species], index: usize, now: Ticks) -> Point {
    let actor: &Actor = actors[index].borrow();
    let activity = actor.activity;
    if activity == Activity::Sleep {
        return ORIGIN;
    }
    if activity == Activity::Sulk {
        return Point { x: GAZE_AHEAD * actor.facing.sign(), y: GAZE_SULK };
    }
    let eye = Point { x: actor.x, y: actor.y - kinds[index].size.height * EYE_HEIGHT };
    let moving = matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall);
    if let Some(pointer) = stage.pointer.filter(|_| !moving && now - stage.pointed < POINTER_TICKS) {
        return look_offset(eye, pointer, GAZE_REACH);
    }
    if let Some(other) = actor.partner.as_deref().and_then(|partner| index_of(actors, partner)) {
        let partner: &Actor = actors[other].borrow();
        return look_offset(eye, Point { x: partner.x, y: partner.y - kinds[other].size.height * EYE_HEIGHT }, GAZE_REACH);
    }
    if !moving && !stage.glances.is_empty() {
        let mut nearest = stage.glances[0];
        let mut least = f64::INFINITY;
        for glance in &stage.glances {
            let dx = glance.x - eye.x;
            let dy = glance.y - eye.y;
            let distance = dx * dx + dy * dy;
            if distance < least {
                nearest = *glance;
                least = distance;
            }
        }
        return look_offset(eye, nearest, GAZE_REACH);
    }
    if activity == Activity::Fall {
        return Point { x: 0.0, y: GAZE_FALL };
    }
    Point { x: GAZE_AHEAD * actor.facing.sign(), y: 0.0 }
}

/// 🧘️ Whether the pupils of an actor rest on what it wants to look at at tick `now`: on the goal, without velocity.
pub(crate) fn gaze_rests<A: Borrow<Actor>>(stage: &Stage, actors: &[A], kinds: &[&Species], index: usize, now: Ticks) -> bool {
    let gaze = Borrow::<Actor>::borrow(&actors[index]).gaze;
    if gaze.vx != 0.0 || gaze.vy != 0.0 {
        return false;
    }
    let goal = gaze_goal(stage, actors, kinds, index, now);
    gaze.x == goal.x && gaze.y == goal.y
}

/// 🔭️ One tick of the gaze of an actor: both axes spring towards the goal; within 1/4096 of it and slower than 1/64 per second the pupils snap onto it and rest. A sleeper's gaze eases to the centre the same way: the lean of its head follows the gaze, so nothing may jump behind the shut lids either.
pub(crate) fn look(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let goal = gaze_goal(&draft.stage, &draft.stage.actors, &draft.kinds, index, now);
    let body = &mut draft.stage.actors[index];
    let gaze = body.gaze;
    if gaze.x == goal.x && gaze.y == goal.y && gaze.vx == 0.0 && gaze.vy == 0.0 {
        return;
    }
    let across = spring_step(gaze.x, gaze.vx, goal.x, GAZE_STIFFNESS, GAZE_DAMPING);
    let down = spring_step(gaze.y, gaze.vy, goal.y, GAZE_STIFFNESS, GAZE_DAMPING);
    let rests = (across.position - goal.x).abs() <= GAZE_REST && (down.position - goal.y).abs() <= GAZE_REST && across.velocity.abs() <= GAZE_CALM && down.velocity.abs() <= GAZE_CALM;
    body.gaze = if rests { Gaze { x: goal.x, y: goal.y, vx: 0.0, vy: 0.0 } } else { Gaze { x: across.position, y: down.position, vx: across.velocity, vy: down.velocity } };
}
//#endregion 🔖️Gaze

//#region 🔖️Manners
/// 😉️ The blink schedule of an actor: when a blink has run its 12 ticks the next one is drawn, 2…6 s ahead, or, one time in six, 7 ticks ahead (a double blink). A sleeper's lids stay shut and draw nothing.
pub(crate) fn wink(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    if body.activity == Activity::Sleep || now < body.blink + BLINK_TICKS {
        return;
    }
    let words = random_words(&actor_key(draft, index), 2);
    draft.stage.actors[index].blink = if unit_of(words[1]) * 6.0 < 1.0 { now + BLINK_AGAIN } else { blink_at(now, unit_of(words[0])) };
}

/// 🌀️ One tick of turning round: an actor that faces its way squarely and does not face its heading turns to it ([`turn`]). `true` while a walker must not stride yet: while it turns, and on the tick its turn ends — its walk begins anew at that tick, so its clip starts with its first stride.
pub(crate) fn swivel(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let body = &mut draft.stage.actors[index];
    if now < body.faced {
        return true;
    }
    if now == body.faced {
        if body.activity == Activity::Walk {
            body.since = now;
        }
        return true;
    }
    let Some(heading) = heading_of(&draft.stage, &draft.stage.actors, &draft.kinds, index, now) else {
        return false;
    };
    let body = &mut draft.stage.actors[index];
    if heading == body.facing {
        return false;
    }
    turn(body, heading, now);
    true
}

/// 🫡️ An actor greets towards `x`: one actor draw picks the span and the clip, it turns to face `x` and lets go of any goal and partner.
fn hail(draft: &mut Draft<'_>, index: usize, x: f64, now: Ticks) {
    let words = random_words(&actor_key(draft, index), 2);
    shift(draft, index, Activity::Greet, now);
    let mode = draft.stage.mode;
    let clip = clip_at(draft.kinds[index], Activity::Greet, unit_of(words[1]));
    let body = &mut draft.stage.actors[index];
    turn(body, facing_to(x, body.x), now);
    body.until = now + dwell_of(Activity::Greet, mode, unit_of(words[0]));
    body.clip = clip;
    body.goal = body.x;
    body.partner = None;
}

/// 🐿️ An idle actor perks up when the pointer has come to rest beside it: outside a time of concentration, on the tick the pointer has not moved for half a second while it is within 1.5 × the actor's height of the middle of its body but not on it (there the actor turns see-through instead: whoever points wants to see what lies beneath), an actor that stands by itself on a perch and whose curiosity — as it stands now — is at least 0.5 greets the pointer ([`hail`]) and spends 0.6 of its curiosity on it (as far as it has any): more than the 0.5 it can have left to spare, so no pet greets twice in a row. Curiosity grows back at rest, faster for a curious species — ten seconds to a minute: that is the pause between two greetings, and a species that is not curious by temperament needs as long before its first one.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub(crate) fn perk(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let Some(pointer) = draft.stage.pointer else {
        return;
    };
    if draft.stage.quiet || now != draft.stage.pointed + PERK_LINGER || body.partner.is_some() || body.leaving || body.perch.is_none() || !watched(Some(pointer), body, kind) || presence_of(Some(pointer), body, kind) != 1.0 {
        return;
    }
    if !(needs_after(body.needs, Activity::Idle, now - body.since, kind.temperament).curiosity >= PERK_URGE) {
        return;
    }
    hail(draft, index, pointer.x, now);
    let body = &mut draft.stage.actors[index];
    body.needs.curiosity = larger(body.needs.curiosity - PERK_COST, 0.0);
}
//#endregion 🔖️Manners

//#region 🔖️Drawing
/// 🌾️ A pose in which the head leans after the eyes: the bone that carries the first eye of the species turns by 5° and shifts by 1.5 px per unit of the gaze `across` (in the actor's own orientation: ahead is positive) and sinks by 1 px per unit of the gaze `down`. The gaze is a spring, so the lean eases with it. A species without eyes does not lean.
pub(crate) fn leant(kind: &Species, mut pose: Pose, across: f64, down: f64) -> Pose {
    let Some(eye) = kind.face.eyes.first() else {
        return pose;
    };
    for (index, bone) in pose.iter_mut().enumerate() {
        if kind.bones[index].id == eye.bone {
            *bone = BonePose { x: bone.x + LEAN_REACH * across, y: bone.y + LEAN_NOD * down, rotation: bone.rotation + LEAN_TURN * across, scale_x: bone.scale_x, scale_y: bone.scale_y };
        }
    }
    pose
}

/// 🪞️ The factor the drawing of an actor is scaled by across at a tick: 1 once it faces its way squarely, and while it turns round an eased sweep from −1 (the mirror image, which is the way it faced before) through 0 (a line) to 1 over the 8 ticks before `faced`.
pub(crate) fn squeeze_of(actor: &Actor, tick: Ticks) -> f64 {
    if tick < actor.faced {
        2.0 * smoothstep((TURN_TICKS - (actor.faced - tick)) as f64 / TURN_TICKS as f64) - 1.0
    } else {
        1.0
    }
}
//#endregion 🔖️Drawing

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
