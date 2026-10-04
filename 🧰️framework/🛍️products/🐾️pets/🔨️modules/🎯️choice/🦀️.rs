//! 🎯️ Choice: what an actor does next when its time is up — an idle one picks an activity by its weights, with its dwell, its clip and its goal or target (`decide`); everything else ends the way its activity ends (`conclude`).
//!
//! A new activity an actor may choose by itself gets its branch in `decide`, and its end in `conclude`.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../🎯️choice/🟦️.ts — the TypeScript twin

use crate::animation::clip_ticks;
use crate::attention::watched;
use crate::behavior::{activity_weights, dwell_of, Situation, MODE_LIMITS};
use crate::draft::{actor_key, clip_at, clip_of, release, settle, shift, Draft};
use crate::locomotion::{hops_of, leave, paced, stroll};
use crate::randomness::{random_pick, random_words, unit_of};
use crate::schema::{Activity, Footing, Gait, Ticks, ACTIVITIES};
use crate::sociability::{reconcile, sulk};
use crate::spacing::{clearway, rooms_for, COMFORT_GAP};
use crate::terrain::{larger, perch_at};
use crate::trigonometry::clamp;

//#region 🔖️Constants
const STROLL_LEAST: f64 = 0.75;
//#endregion 🔖️Constants

//#region 🔖️Decision
/// 🙋️ An idle actor decides what to do next: one `random_pick` over `activity_weights`, then its dwell, its clip and, for a walk, a goal on the clear way of its perch (within the stroll of the mode, at least three quarters of a body away — a hopping gait at least one whole hop —, never past or into anybody; without such a goal it stays idle instead) or, for a hop, one of its launches. An actor that feels like a hop where no perch is in reach wanders off instead when another perch has room for it: it leaves, and arrives anew a moment later.
fn decide(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    let mode = draft.stage.mode;
    let limits = MODE_LIMITS[mode];
    let width = kind.size.width;
    shift(draft, index, Activity::Idle, now);
    let body = &draft.stage.actors[index];
    let mut movers = 0;
    let mut fidgeters = 0;
    let mut crowd = 0;
    for (other, actor) in draft.stage.actors.iter().enumerate() {
        if other == index {
            continue;
        }
        if matches!(actor.activity, Activity::Walk | Activity::Hop) {
            movers += 1;
        }
        if actor.activity == Activity::Fidget {
            fidgeters += 1;
        }
        if body.perch.is_some() && actor.perch == body.perch && !actor.leaving {
            crowd += 1;
        }
    }
    let at = body.x;
    let perch = body.perch.as_deref().and_then(|surface| perch_at(&draft.stage.perches, surface, at));
    let [low, high] = clearway(draft, index, COMFORT_GAP, None);
    let restless = !draft.stage.quiet && movers < limits.movers && limits.hop > 0.0 && perch.is_some();
    let launches = if restless { hops_of(draft, index) } else { Vec::new() };
    let elsewhere = restless && launches.is_empty() && rooms_for(draft, kind).iter().any(|room| !perch.is_some_and(|perch| std::ptr::eq(perch, &draft.stage.perches[room.perch])));
    let weights = activity_weights(
        body,
        kind,
        Situation { mode, quiet: draft.stage.quiet, movers, fidgeters, roam: larger(at - low, high - at) >= STROLL_LEAST * width, hops: !launches.is_empty() || elsewhere, crowd, watched: watched(draft.stage.pointer, body, kind), whims: false },
    );
    let key = actor_key(draft, index);
    let pick = random_pick(&key, &weights);
    let words = random_words(&key, 4);
    let activity = pick.map_or(Activity::Idle, |pick| ACTIVITIES[pick]);
    let dwell = unit_of(words[1]);
    let clip = clip_at(kind, activity, unit_of(words[2]));
    let place = unit_of(words[3]);
    if activity == Activity::Walk {
        let reach = limits.stroll * width;
        let least = STROLL_LEAST * width;
        let mut goal = clamp(low + (high - low) * place, at - reach, at + reach);
        if (goal - at).abs() < least {
            goal = if at - low > high - at { at - least } else { at + least };
        }
        goal = paced(kind, clip.as_deref(), at, clamp(goal, low, high));
        let sets_out = if kind.locomotion.gait == Gait::Hop { goal != at } else { (goal - at).abs() >= least };
        if sets_out {
            stroll(draft, index, goal, clip, now);
            return;
        }
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Idle, unit_of(words[2]));
        body.until = now + dwell_of(Activity::Idle, mode, dwell);
        return;
    }
    if activity == Activity::Hop {
        if launches.is_empty() {
            leave(draft, index, now);
            return;
        }
        let chosen = (place * launches.len() as f64).floor();
        let launch = launches[if chosen < launches.len() as f64 { chosen as usize } else { launches.len() - 1 }];
        shift(draft, index, Activity::Hop, now);
        let body = &mut draft.stage.actors[index];
        body.perch = None;
        body.footing = Footing::Air;
        body.vx = launch.vx;
        body.vy = launch.vy;
        body.goal = launch.x;
        body.until = now + launch.ticks;
        body.clip = clip;
        return;
    }
    let until = now + clip_of(kind, clip.as_deref()).filter(|played| activity == Activity::Fidget && !played.looping).map_or_else(|| dwell_of(activity, mode, dwell), clip_ticks);
    let body = &mut draft.stage.actors[index];
    body.activity = activity;
    body.clip = clip;
    body.until = until;
}

/// 🏁️ The end of what an actor does when its time is up: an idle one decides (or gives up waiting for its partner), a squabbler sulks, a sulker reconciles, and everyone else comes to rest. Walks, hops and falls end by their motion.
pub(crate) fn conclude(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let activity = draft.stage.actors[index].activity;
    let partnered = draft.stage.actors[index].partner.is_some();
    if matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall) {
        return;
    }
    if activity == Activity::Idle && !partnered {
        decide(draft, index, now);
        return;
    }
    if activity == Activity::Squabble && partnered {
        sulk(draft, index, now);
        return;
    }
    if activity == Activity::Idle {
        release(draft, index, now);
    }
    if activity == Activity::Sulk {
        reconcile(draft, index, now);
    }
    settle(draft, index, now);
}
//#endregion 🔖️Decision
