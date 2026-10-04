//! 📽️ The projection of a stage into a frame: the pose of every actor (the idle loop underneath, the clip of its activity on top of it or in its place), the rate its motion needs (`pace_of`), and `frame_of` — the actors back to front with their matrices, eyes, spirits and mood, the rate of the frame and the tick it must wake at.
//!
//! Whatever a render target is to draw is projected here, from the stage alone.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../🎥️projection/🟦️.ts — the TypeScript twin

use crate::animation::{blend_pose, clip_ticks, lid_at, sample_clip, BLINK_TICKS};
use crate::attention::{gaze_rests, leant, presence_of, squeeze_of, PERK_LINGER, POINTER_TICKS, TURN_REST};
use crate::draft::{breath_of, clip_of, RATE};
use crate::feeling::{settled, settles_at, spirits_of};
use crate::rig::{pupil_reach, rest_pose, solve_rig, BonePose, Pose};
use crate::schedule::{arrival_tick, pairing_tick};
use crate::schema::{Activity, Actor, ActorFrame, EyeFrame, Facing, Frame, Menagerie, PetMode, Point, Rate, Rect, Species, Stage, Ticks};
use crate::terrain::{larger, smaller};
use crate::trigonometry::clamp;
use std::borrow::Borrow;

//#region 🔖️Constants
const BLEND_TICKS: Ticks = 8;
const BREATH_STAGGER: Ticks = 37;
//#endregion 🔖️Constants

//#region 🔖️Frame
/// 🧱️ Whether the clip of an activity replaces the idle loop underneath it (cross-fade) instead of being added on top of it: the gaits, the flights, the landing and sleep bring their own body motion.
fn replaces(activity: Activity) -> bool {
    matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall | Activity::Land | Activity::Sleep)
}

/// 🎛️ How much of its clip an actor shows: it fades in over the first 8 ticks of the activity and out over the last 8 — of its span, or of the way to its goal when it walks; flights end hard.
fn weight_of(kind: &Species, actor: &Actor, tick: Ticks) -> f64 {
    let activity = actor.activity;
    let into = (tick - actor.since) as f64 / BLEND_TICKS as f64;
    let out = if matches!(activity, Activity::Hop | Activity::Fall) {
        1.0
    } else if activity == Activity::Walk {
        ((actor.goal - actor.x).abs() * RATE) / (larger(kind.locomotion.speed, 1.0) * BLEND_TICKS as f64)
    } else {
        (actor.until - tick) as f64 / BLEND_TICKS as f64
    };
    clamp(smaller(into, out), 0.0, 1.0)
}

/// 🧩️ One pose on top of another: offsets and rotations add, scale factors multiply.
fn layer(under: &[BonePose], over: &[BonePose]) -> Pose {
    under
        .iter()
        .enumerate()
        .map(|(index, bone)| BonePose { x: bone.x + over[index].x, y: bone.y + over[index].y, rotation: bone.rotation + over[index].rotation, scale_x: bone.scale_x * over[index].scale_x, scale_y: bone.scale_y * over[index].scale_y })
        .collect()
}

/// 🤸️ The pose of an actor at a tick: the first idle clip of its species loops underneath on the clock of the stage (staggered by 37 ticks per stream, so a pet that stands still is never frozen and no two breathe in step), and the clip of its activity, weighted by [`weight_of`] from the rest pose, lies on top of it or takes its place. A walker that still turns round has not set out yet: its walk weighs nothing.
fn pose_of(kind: &Species, actor: &Actor, tick: Ticks, stream: Ticks) -> Pose {
    let rest = rest_pose(kind);
    let breath = breath_of(kind);
    let Some(top) = clip_of(kind, actor.clip.as_deref()).filter(|top| breath.is_none_or(|breath| top.id != breath.id)) else {
        return breath.map_or(rest, |breath| sample_clip(kind, breath, tick + stream * BREATH_STAGGER));
    };
    let weight = if tick < actor.faced && actor.activity == Activity::Walk { 0.0 } else { weight_of(kind, actor, tick) };
    let over = blend_pose(&rest, &sample_clip(kind, top, tick - actor.since), weight);
    let Some(breath) = breath else {
        return over;
    };
    let under = sample_clip(kind, breath, tick + stream * BREATH_STAGGER);
    layer(&if replaces(actor.activity) { blend_pose(&under, &rest, weight) } else { under }, &over)
}

/// 🎼️ How many ticks per second the motion of one actor needs at a tick: 64 while it fades (in, out, or see-through under the pointer and back), turns round, walks, flies, lands, plays a clip that does not loop or blends a clip in or out; 32 while a loop, a blink, its pupils or its feeling move (until it `settles_at` its rest); 16 while it only sleeps; 0 when nothing of it moves.
fn pace_of<A: Borrow<Actor>>(stage: &Stage, actors: &[A], kinds: &[&Species], index: usize) -> Rate {
    let actor: &Actor = actors[index].borrow();
    let kind = kinds[index];
    let tick = stage.tick;
    let activity = actor.activity;
    if actor.leaving || actor.opacity != presence_of(stage.pointer, actor, kind) || tick < actor.faced || matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall | Activity::Land) {
        return Rate::Full;
    }
    let breath = breath_of(kind);
    let layered = clip_of(kind, actor.clip.as_deref()).filter(|top| breath.is_none_or(|breath| top.id != breath.id));
    if layered.is_some_and(|top| (!top.looping && tick - actor.since < clip_ticks(top)) || tick - actor.since < BLEND_TICKS || actor.until - tick < BLEND_TICKS) {
        return Rate::Full;
    }
    let restless = settles_at(actor.feeling, kind.mood) > tick || !gaze_rests(stage, actors, kinds, index, tick + 1);
    if activity == Activity::Sleep {
        return if restless {
            Rate::Half
        } else if layered.is_some() || breath.is_some() {
            Rate::Quarter
        } else {
            Rate::Rest
        };
    }
    if restless || (tick >= actor.blink && tick < actor.blink + BLINK_TICKS) {
        return Rate::Half;
    }
    if layered.is_some_and(|top| top.looping) || breath.is_some() {
        Rate::Half
    } else {
        Rate::Rest
    }
}

/// 🪜️ Whether one actor is drawn before another: the one higher up (the smaller `y`) first, at the same height the species id that is smaller by UTF-16 code units, as the twin compares strings.
fn behind(one: &Actor, other: &Actor) -> bool {
    let rise = one.y - other.y;
    if rise < 0.0 {
        return true;
    }
    if rise > 0.0 {
        return false;
    }
    one.species.encode_utf16().lt(other.species.encode_utf16())
}

/// 🎥️ What a render target draws for a stage: the actors back to front (by `y`, then species id), each with its feet, facing, activity, opacity, one matrix per bone (`solve_rig` of [`pose_of`], leaning after the gaze ([`leant`]) outside a time of concentration, squeezed across by [`squeeze_of`] while the actor turns round), its eyes (pupil offset = gaze × `pupil_reach`: the pupil travels up to the outline of the white; mirrored with the actor as drawn; lid from the blink, shut asleep), its footing and the state of its species, the mood it feels with its intensity and its spirits (`spirits_of` its feeling as it stands at the tick), upright about its grip with nothing in its hands, and its size box as its body; the rate the motion needs and, at rate 0, the tick of the next scheduled change. No ladder stands, no particle flies, no fixture is lifted and nobody is held.
///
/// A still stage shows every actor in its rest pose with open eyes and centred pupils at rate 0 without a wake tick —
/// see-through at once while the pointer rests on it, since nothing eases there — and so does an empty one that nobody
/// waits to enter. At rate 0 otherwise (species without an idle loop, pupils at rest, between blinks) `wake` is the
/// earliest of the next blink, the end of an activity, the pointer losing its interest, having come to rest for half
/// a second or being allowed to turn an actor again, the next whole second on which a pair may be drawn and the next
/// whole second on which somebody who waits off stage may arrive.
pub fn frame_of(menagerie: &Menagerie, stage: &Stage) -> Frame {
    let mut actors: Vec<&Actor> = Vec::with_capacity(stage.actors.len());
    let mut kinds: Vec<&Species> = Vec::with_capacity(stage.actors.len());
    let mut streams: Vec<Ticks> = Vec::with_capacity(stage.actors.len());
    for (stream, kind) in menagerie.species.iter().enumerate() {
        if let Some(actor) = stage.actors.iter().find(|actor| actor.species == kind.id) {
            actors.push(actor);
            kinds.push(kind);
            streams.push(stream as Ticks);
        }
    }
    let tick = stage.tick;
    let still = stage.mode == PetMode::Still;
    let mut order: Vec<usize> = (0..actors.len()).collect();
    for sorted in 1..order.len() {
        let mut at = sorted;
        while at > 0 && behind(actors[order[at]], actors[order[at - 1]]) {
            order.swap(at, at - 1);
            at -= 1;
        }
    }
    let frames: Vec<ActorFrame> = order
        .into_iter()
        .map(|index| {
            let actor = actors[index];
            let kind = kinds[index];
            let lid = if still {
                0.0
            } else if actor.activity == Activity::Sleep {
                1.0
            } else {
                lid_at(tick - actor.blink)
            };
            let squeeze = if still { 1.0 } else { squeeze_of(actor, tick) };
            let forward = if squeeze < 0.0 { actor.facing == Facing::Left } else { actor.facing == Facing::Right };
            let across = if forward { actor.gaze.x } else { 0.0 - actor.gaze.x };
            let eyes = kind
                .face
                .eyes
                .iter()
                .map(|eye| {
                    let span = pupil_reach(eye);
                    EyeFrame { x: across * span, y: actor.gaze.y * span, lid }
                })
                .collect();
            let pose = if still { rest_pose(kind) } else { pose_of(kind, actor, tick, streams[index]) };
            let mut bones = solve_rig(kind, &if still || stage.quiet { pose } else { leant(kind, pose, across, actor.gaze.y) });
            if squeeze != 1.0 {
                for entry in bones.iter_mut().step_by(2) {
                    *entry *= squeeze;
                }
            }
            let presence = presence_of(stage.pointer, actor, kind);
            let feeling = settled(actor.feeling, kind.mood, tick);
            ActorFrame {
                species: actor.species.clone(),
                x: actor.x,
                y: actor.y,
                facing: actor.facing,
                activity: actor.activity,
                opacity: if still && presence < actor.opacity { presence } else { actor.opacity },
                bones,
                eyes,
                footing: actor.footing,
                state: actor.state.clone(),
                mood: feeling.mood,
                intensity: feeling.intensity,
                spirits: spirits_of(feeling),
                tilt: 0.0,
                pivot: Point { x: 0.0, y: -kind.grip },
                tools: Vec::new(),
                body: Rect { x: actor.x - kind.size.width / 2.0, y: actor.y - kind.size.height, width: kind.size.width, height: kind.size.height },
            }
        })
        .collect();
    let mut rate = Rate::Rest;
    if !still {
        for index in 0..actors.len() {
            let pace = pace_of(stage, &actors, &kinds, index);
            if pace > rate {
                rate = pace;
            }
        }
    }
    let mut wake = None;
    let arrival = arrival_tick(menagerie, stage, &actors, tick + 1);
    if rate == Rate::Rest && !still && (!actors.is_empty() || arrival >= 0) {
        let mut horizon = Ticks::MAX;
        for actor in &actors {
            horizon = Ticks::min(horizon, actor.until);
            if actor.activity != Activity::Sleep {
                horizon = Ticks::min(horizon, if actor.blink > tick { actor.blink } else { actor.blink + BLINK_TICKS });
            }
            if stage.pointer.is_some() && tick + 1 < actor.faced + TURN_REST {
                horizon = Ticks::min(horizon, actor.faced + TURN_REST);
            }
        }
        if !actors.is_empty() && stage.pointer.is_some() && tick < stage.pointed + PERK_LINGER {
            horizon = Ticks::min(horizon, stage.pointed + PERK_LINGER);
        }
        if !actors.is_empty() && stage.pointer.is_some() && tick + 1 - stage.pointed < POINTER_TICKS {
            horizon = Ticks::min(horizon, stage.pointed + POINTER_TICKS);
        }
        let pairing = pairing_tick(stage, &actors, tick + 1);
        if pairing >= 0 {
            horizon = Ticks::min(horizon, pairing);
        }
        if arrival >= 0 {
            horizon = Ticks::min(horizon, arrival);
        }
        wake = Some(if horizon > tick { horizon } else { tick + 1 });
    }
    Frame { tick, actors: frames, rate, wake, ladders: Vec::new(), particles: Vec::new(), lifts: Vec::new(), puffs: Vec::new(), held: None }
}
//#endregion 🔖️Frame

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
