//! 🏃️ Locomotion: how a pet gets around and how it loses its ground — setting out for a goal on its perch, striding, hopping and gliding between perches, falling, touching down, being crowded out, waiting for a partner and leaving.
//!
//! A new footing or a new way to travel belongs here: what starts it, its tick of motion and where it ends.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../🚶️locomotion/🟦️.ts — the TypeScript twin

use crate::animation::clip_ticks;
use crate::behavior::dwell_of;
use crate::draft::{beat_of, clip_at, clip_of, hover_of, release, remove, settle, shift, shoulders, Draft, Launch, ORIGIN, RATE};
use crate::schema::{Activity, Footing, Gait, Perch, Point, Slug, Species, Ticks};
use crate::spacing::{clearway, hindered, soars, vacancy, COMFORT_GAP, MEET_GAP};
use crate::terrain::{fall_step, hop_landing, hop_of, hop_step, landing_of, larger, perch_at, smaller, stride_to, HOP_DISTANCE, HOP_HEIGHT};
use crate::trigonometry::clamp;

//#region 🔖️Constants
const PATIENCE: Ticks = 1920;
const LEAVE_REACH: f64 = 3.0;
const GLIDE_TICKS: Ticks = 256;
//#endregion 🔖️Constants

//#region 🔖️Footing
/// 🏡️ An actor that has nowhere left to be is gone at once (always `false`): it lets go of its partner and arrives anew, spaced from the others, once it is wanted and a perch has room.
pub(crate) fn vanish(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    release(draft, index, now);
    remove(draft, index);
    false
}

/// 🚷️ An actor has no room where it is: it lets go of its partner and of its perch and fades out on the spot. While it is wanted it arrives anew once a perch has room.
pub(crate) fn crowd_out(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    release(draft, index, now);
    if matches!(draft.stage.actors[index].activity, Activity::Walk | Activity::Hop | Activity::Fall) {
        shift(draft, index, Activity::Idle, now);
        draft.stage.actors[index].clip = clip_at(draft.kinds[index], Activity::Idle, 0.0);
    }
    let body = &mut draft.stage.actors[index];
    body.leaving = true;
    body.perch = None;
    body.footing = Footing::Air;
    body.goal = body.x;
    body.vx = 0.0;
    body.vy = 0.0;
}

/// 🕳️ An actor loses its perch and starts to fall.
pub(crate) fn drop(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    release(draft, index, now);
    shift(draft, index, Activity::Fall, now);
    let until = now + dwell_of(Activity::Fall, draft.stage.mode, 0.0);
    let clip = clip_at(draft.kinds[index], Activity::Fall, 0.0);
    let body = &mut draft.stage.actors[index];
    body.perch = None;
    body.footing = Footing::Air;
    body.vx = 0.0;
    body.vy = 0.0;
    body.until = until;
    body.clip = clip;
}

/// 🛬️ An actor touches down on a perch at `x`: it stands inside the stretch it fits on and lands for the length of its landing clip, or 0.3 s when that clip loops or is missing. When somebody stands there already it comes down beside them, at the nearest place that keeps a comfortable gap; when that place is farther away than its own width, or the perch is full, it lands where it fell and is crowded out.
fn touch(draft: &mut Draft<'_>, index: usize, perch: &Perch, x: f64, now: Ticks) {
    let kind = draft.kinds[index];
    let half = kind.size.width / 2.0;
    let aimed = clamp(x, perch.x0 + half, perch.x1 - half);
    let place = vacancy(draft, index, perch, aimed).filter(|place| (place - aimed).abs() <= kind.size.width);
    shift(draft, index, Activity::Land, now);
    let clip = clip_at(kind, Activity::Land, 0.0);
    let until = now + clip_of(kind, clip.as_deref()).filter(|clip| !clip.looping).map_or_else(|| dwell_of(Activity::Land, draft.stage.mode, 0.0), clip_ticks);
    let body = &mut draft.stage.actors[index];
    body.perch = Some(perch.surface.clone());
    body.footing = Footing::Perch;
    body.x = place.unwrap_or(aimed);
    body.y = perch.y - hover_of(kind);
    body.vx = 0.0;
    body.vy = 0.0;
    body.goal = body.x;
    body.clip = clip;
    body.until = until;
    if place.is_none() && !body.leaving {
        crowd_out(draft, index, now);
    }
}
//#endregion 🔖️Footing

//#region 🔖️Departure
/// 🥾️ Where a walk from `x` towards `goal` really ends: a hopping gait covers ground in whole hops of its clip (speed × the length of the clip), as many as fit before the goal; every other gait goes all the way.
pub(crate) fn paced(kind: &Species, clip: Option<&str>, x: f64, goal: f64) -> f64 {
    let beat = beat_of(kind, clip_of(kind, clip));
    if beat == 1 {
        return goal;
    }
    let hop = (larger(kind.locomotion.speed, 0.0) * beat as f64) / RATE;
    let leaps = hop > 0.0;
    if !leaps {
        return x;
    }
    let hops = ((goal - x).abs() / hop).floor();
    if goal >= x {
        x + hops * hop
    } else {
        x - hops * hop
    }
}

/// 🚶️ An actor sets out for a goal on its perch; it turns towards it first when it faces the other way.
pub(crate) fn stroll(draft: &mut Draft<'_>, index: usize, goal: f64, clip: Option<Slug>, now: Ticks) {
    shift(draft, index, Activity::Walk, now);
    let until = now + dwell_of(Activity::Walk, draft.stage.mode, 0.0);
    let body = &mut draft.stage.actors[index];
    body.goal = goal;
    body.until = until;
    body.clip = clip;
}

/// 🧍️ An actor waits for its partner: idle, turning towards it, for as long as the stage has patience.
pub(crate) fn attend(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    shift(draft, index, Activity::Idle, now);
    let clip = clip_at(draft.kinds[index], Activity::Idle, 0.0);
    let body = &mut draft.stage.actors[index];
    body.until = now + PATIENCE;
    body.clip = clip;
    body.goal = body.x;
}

/// 🦘️ Where an actor could hop to: per other perch (in perch order) the place nearest to it that keeps half a body from the ends and a comfortable gap from the actors there, when `hop_of` grants the hop, the flight really ends on that perch and its arc covers nothing that must stay free ([`soars`]); a floating gait glides there in a straight line at twice its speed when the place is within the reach of a hop and four seconds.
pub(crate) fn hops_of(draft: &Draft<'_>, index: usize) -> Vec<Launch> {
    let actors = &draft.stage.actors;
    let body = &actors[index];
    let kind = draft.kinds[index];
    let half = kind.size.width / 2.0;
    let hover = hover_of(kind);
    let mut launches = Vec::new();
    for perch in &draft.stage.perches {
        if body.perch.as_deref() == Some(perch.surface.as_str()) && perch.x0 <= body.x && body.x <= perch.x1 {
            continue;
        }
        let low = perch.x0 + half;
        let high = perch.x1 - half;
        if high < low {
            continue;
        }
        let inset = smaller(half, (high - low) / 2.0);
        let x = clamp(body.x, low + inset, high - inset);
        let mut taken = false;
        for (other, neighbour) in actors.iter().enumerate() {
            if other == index || neighbour.perch.as_deref() != Some(perch.surface.as_str()) {
                continue;
            }
            if (neighbour.x - x).abs() < shoulders(draft.kinds[other], kind) + COMFORT_GAP {
                taken = true;
            }
        }
        if taken {
            continue;
        }
        if kind.locomotion.gait == Gait::Float {
            let dx = x - body.x;
            let dy = perch.y - hover - body.y;
            if dx.abs() > HOP_DISTANCE || dy.abs() > HOP_HEIGHT {
                continue;
            }
            let ticks = (((dx * dx + dy * dy).sqrt() * RATE) / (2.0 * larger(kind.locomotion.speed, 1.0)) + 0.5).floor();
            if ticks < 1.0 || ticks > GLIDE_TICKS as f64 {
                continue;
            }
            launches.push(Launch { x, vx: (dx * RATE) / ticks, vy: (dy * RATE) / ticks, ticks: ticks as Ticks });
            continue;
        }
        let from = Point { x: body.x, y: body.y };
        let to = Point { x, y: perch.y };
        let Some(hop) = hop_of(from, to) else {
            continue;
        };
        let Some(landing) = hop_landing(&draft.stage.perches, from, to, hop) else {
            continue;
        };
        if landing.surface != perch.surface || landing.x0 != perch.x0 || !soars(draft, kind, from, to, hop) {
            continue;
        }
        launches.push(Launch { x, vx: hop.vx, vy: hop.vy, ticks: hop.ticks });
    }
    launches
}
//#endregion 🔖️Departure

//#region 🔖️Motion
/// 👣️ One tick of a walk. On every beat — every tick for a walking or floating gait, the start of every hop for a hopping one — the walk ends when the goal is no farther than one stride (the actor stands on it then) or when somebody is in the way of the next beat; otherwise a stride towards the goal follows. A hopping gait that reaches its goal within a hop, or finds somebody in its way in mid-hop, finishes that hop on the spot. The walk also ends when the stage loses patience. At its end a leaver starts to fade, an actor with a partner waits for it and anyone else comes to rest.
pub(crate) fn stride(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    let body = &draft.stage.actors[index];
    let beat = beat_of(kind, clip_of(kind, body.clip.as_deref()));
    let mut done = now >= body.until;
    if !done && (now - body.since) % beat == 0 {
        if (body.goal - body.x).abs() <= larger(kind.locomotion.speed, 0.0) / RATE {
            draft.stage.actors[index].x = body.goal;
            done = true;
        } else {
            done = hindered(draft, index, beat);
        }
    }
    if !done {
        let stops = beat > 1 && hindered(draft, index, 1);
        let body = &mut draft.stage.actors[index];
        if stops {
            body.goal = body.x;
        }
        body.x = stride_to(body.x, body.goal, kind.locomotion.speed);
        return;
    }
    let body = &mut draft.stage.actors[index];
    if body.leaving {
        shift(draft, index, Activity::Idle, now);
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Idle, 0.0);
        body.goal = body.x;
        return;
    }
    if body.partner.is_some() {
        attend(draft, index, now);
        return;
    }
    settle(draft, index, now);
}

/// 🎯️ The perch a flight towards `x` ends on: among the perches that carry `x`, the one nearest to the height `y`; `None` when the target is gone.
fn aim(perches: &[Perch], x: f64, y: f64) -> Option<&Perch> {
    let mut nearest = None;
    let mut least = f64::INFINITY;
    for perch in perches {
        if x < perch.x0 || x > perch.x1 {
            continue;
        }
        let distance = (perch.y - y).abs();
        if distance < least {
            nearest = Some(perch);
            least = distance;
        }
    }
    nearest
}

/// ⬇️ One tick of a fall: `fall_step`, then the swept landing at the height of the feet plus the hover; below the stage box the actor is gone and arrives anew. `false` when the actor is gone.
pub(crate) fn plunge(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let kind = draft.kinds[index];
    let hover = hover_of(kind);
    let body = &mut draft.stage.actors[index];
    let fallen = fall_step(body.y, body.vy);
    if let Some(landing) = landing_of(&draft.stage.perches, body.x, body.y + hover, fallen.y + hover).cloned() {
        let x = body.x;
        touch(draft, index, &landing, x, now);
        return true;
    }
    body.y = fallen.y;
    body.vy = fallen.vy;
    if body.y - kind.size.height <= draft.stage.height {
        return true;
    }
    vanish(draft, index, now)
}

/// 🛫️ One tick of a hop: a floating gait glides straight, any other flies the arc of `hop_step` and lands on the first perch it crosses on its way down; on its last tick it touches down on the perch it aimed at, and falls when that perch is gone. `false` when the actor is gone.
pub(crate) fn fly(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let kind = draft.kinds[index];
    let floats = kind.locomotion.gait == Gait::Float;
    let body = &mut draft.stage.actors[index];
    let left = body.until - now + 1;
    if left > 1 {
        if floats {
            body.x += body.vx / RATE;
            body.y += body.vy / RATE;
            return true;
        }
        let next = hop_step(body.x, body.y, body.vx, body.vy, ORIGIN, left);
        if let Some(landing) = landing_of(&draft.stage.perches, next.x, body.y, next.y).cloned() {
            touch(draft, index, &landing, next.x, now);
            return true;
        }
        body.x = next.x;
        body.y = next.y;
        body.vy = next.vy;
        return true;
    }
    let goal = body.goal;
    if let Some(target) = aim(&draft.stage.perches, goal, if floats { body.y + hover_of(kind) } else { fall_step(body.y, body.vy).y }).cloned() {
        touch(draft, index, &target, goal, now);
        return true;
    }
    shift(draft, index, Activity::Fall, now);
    let until = now + dwell_of(Activity::Fall, draft.stage.mode, 0.0);
    let body = &mut draft.stage.actors[index];
    body.vx = 0.0;
    body.vy = if floats { 0.0 } else { body.vy };
    body.until = until;
    body.clip = clip_at(kind, Activity::Fall, 0.0);
    plunge(draft, index, now)
}
//#endregion 🔖️Motion

//#region 🔖️Leaving
/// 👋️ An actor starts to leave: it lets go of its partner and walks to the nearer end of its perch when that is within three of its widths and nobody stands in the way, then fades; farther away, hemmed in or in the air, it fades where it is.
pub(crate) fn leave(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    release(draft, index, now);
    draft.stage.actors[index].leaving = true;
    let body = &draft.stage.actors[index];
    let Some(surface) = body.perch.as_deref() else {
        return;
    };
    let perch = perch_at(&draft.stage.perches, surface, body.x);
    let half = kind.size.width / 2.0;
    let low = perch.map_or(body.x, |perch| perch.x0 + half);
    let high = perch.map_or(body.x, |perch| perch.x1 - half);
    let end = if body.x - low <= high - body.x { low } else { high };
    let [clear_low, clear_high] = clearway(draft, index, MEET_GAP, None);
    let clip = clip_at(kind, Activity::Walk, 0.0);
    let goal = paced(kind, clip.as_deref(), body.x, end);
    if goal != body.x && end >= clear_low && end <= clear_high && (end - body.x).abs() <= LEAVE_REACH * kind.size.width {
        stroll(draft, index, goal, clip, now);
        return;
    }
    shift(draft, index, Activity::Idle, now);
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(kind, Activity::Idle, 0.0);
    body.goal = body.x;
}
//#endregion 🔖️Leaving

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
