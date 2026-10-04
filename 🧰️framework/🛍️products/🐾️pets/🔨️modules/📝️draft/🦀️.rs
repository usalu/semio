//! ✏️ The draft of the stage: the working copy that `advance` folds events into — the actors in `menagerie.species` order, each beside its species and its random stream — with what every part of the stage needs of it: the lookups of surfaces, clips and actors, the keys of the counter-based draws, the stretches of a perch, and the smallest changes of an actor (taking up an activity, coming to rest, letting go of its partner, leaving the draft).
//!
//! What the fold must know beside the stage itself belongs to `Draft`; what a new part of the stage needs of every other part belongs here, below all of them.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../📝️draft/🟦️.ts — the TypeScript twin

use crate::animation::clip_ticks;
use crate::behavior::{dwell_of, needs_after};
use crate::randomness::{random_words, unit_of, STAGE_STREAM};
use crate::schema::{Activity, Actor, Clip, Facing, Gait, Menagerie, Perch, Point, Slug, Species, Stage, Surface, Ticks, TICKS_PER_SECOND};
use std::borrow::Borrow;

//#region 🔖️Constants
pub(crate) const RATE: f64 = TICKS_PER_SECOND as f64;
pub(crate) const BLINK_LOW: Ticks = 128;
pub(crate) const BLINK_HIGH: Ticks = 384;
pub(crate) const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
//#endregion 🔖️Constants

//#region 🔖️Draft
/// 🗒️ A stage being folded: its actors in `menagerie.species` order, each beside its species and its stream.
pub(crate) struct Draft<'a> {
    pub(crate) stage: Stage,
    pub(crate) kinds: Vec<&'a Species>,
    pub(crate) streams: Vec<u32>,
}

/// 🛖️ A perch (its index among the perches of the stage) with the stretches (pairs of ends) a species can arrive on, their summed length and how many actors stand on it.
pub(crate) struct Room {
    pub(crate) perch: usize,
    pub(crate) stretches: Vec<[f64; 2]>,
    pub(crate) span: f64,
    pub(crate) crowd: usize,
}

/// 🚀️ A flight an actor could take: the x it ends at, its launch velocity and its ticks in the air.
#[derive(Clone, Copy)]
pub(crate) struct Launch {
    pub(crate) x: f64,
    pub(crate) vx: f64,
    pub(crate) vy: f64,
    pub(crate) ticks: Ticks,
}

/// 📝️ A working copy of a stage: its actors in `menagerie.species` order, each beside its species and its stream; of two actors of one species the first stays, and an actor of a species the menagerie does not have is dropped.
pub(crate) fn draft_of(menagerie: &Menagerie, mut stage: Stage) -> Draft<'_> {
    let mut waiting: Vec<Option<Actor>> = std::mem::take(&mut stage.actors).into_iter().map(Some).collect();
    let mut kinds = Vec::with_capacity(waiting.len());
    let mut streams = Vec::with_capacity(waiting.len());
    for (stream, kind) in menagerie.species.iter().enumerate() {
        let Some(actor) = waiting.iter_mut().find(|slot| slot.as_ref().is_some_and(|actor| actor.species == kind.id)).and_then(Option::take) else {
            continue;
        };
        stage.actors.push(actor);
        kinds.push(kind);
        streams.push(stream as u32);
    }
    Draft { stage, kinds, streams }
}

/// 📦️ The stage a draft has become.
pub(crate) fn sealed(draft: Draft<'_>) -> Stage {
    draft.stage
}
//#endregion 🔖️Draft

//#region 🔖️Lookups
/// 🎈️ How high a species floats above its perch: its hover when its gait is `float`, else 0.
pub(crate) fn hover_of(kind: &Species) -> f64 {
    if kind.locomotion.gait == Gait::Float {
        kind.locomotion.hover.unwrap_or(0.0)
    } else {
        0.0
    }
}

/// 🔎️ The index of the actor of a species on stage, or `None` (the twin's −1).
pub(crate) fn index_of<A: Borrow<Actor>>(actors: &[A], species: &str) -> Option<usize> {
    actors.iter().position(|actor| Borrow::<Actor>::borrow(actor).species == species)
}

/// 🪵️ The surface with an id, or `None`.
pub(crate) fn surface_of<'s>(surfaces: &'s [Surface], id: &str) -> Option<&'s Surface> {
    surfaces.iter().find(|surface| surface.id == id)
}

/// 🗃️ The clips a species plays for an activity: its own, else the hop clips for the walk of a hopping gait, else the idle clips (a pet that has no motion of its own for something keeps breathing), else none.
fn clips_of(kind: &Species, activity: Activity) -> &[Slug] {
    if let Some(own) = kind.repertoire.clips(activity).filter(|own| !own.is_empty()) {
        return own;
    }
    if activity == Activity::Walk && kind.locomotion.gait == Gait::Hop {
        if let Some(hops) = kind.repertoire.hop.as_deref().filter(|hops| !hops.is_empty()) {
            return hops;
        }
    }
    kind.repertoire.idle.as_deref().unwrap_or(&[])
}

/// 🎞️ The clip a unit draw picks for an activity, uniformly among [`clips_of`]; `None` when there is none.
pub(crate) fn clip_at(kind: &Species, activity: Activity, unit: f64) -> Option<Slug> {
    let clips = clips_of(kind, activity);
    if clips.is_empty() {
        return None;
    }
    let index = (unit * clips.len() as f64).floor();
    Some(clips[if index < clips.len() as f64 { index as usize } else { clips.len() - 1 }].clone())
}

/// 📼️ The clip of a species with an id, or `None`.
pub(crate) fn clip_of<'s>(kind: &'s Species, id: Option<&str>) -> Option<&'s Clip> {
    let id = id?;
    kind.clips.iter().find(|clip| clip.id == id)
}

/// 🫁️ The first idle clip of a species, the loop that keeps running underneath everything else; `None` when it has none.
pub(crate) fn breath_of(kind: &Species) -> Option<&Clip> {
    let idle = kind.repertoire.idle.as_deref()?;
    clip_of(kind, Some(idle.first()?))
}

/// 🔑️ The key of the next draw of an actor; the draw is counted.
pub(crate) fn actor_key(draft: &mut Draft<'_>, index: usize) -> [u32; 3] {
    let body = &mut draft.stage.actors[index];
    let counter = body.draws;
    body.draws = counter.wrapping_add(1);
    [draft.stage.seed, draft.streams[index], counter]
}

/// 🗝️ The key of the next draw of the stage; the draw is counted.
pub(crate) fn stage_key(draft: &mut Draft<'_>) -> [u32; 3] {
    let counter = draft.stage.draws;
    draft.stage.draws = counter.wrapping_add(1);
    [draft.stage.seed, STAGE_STREAM, counter]
}

/// 🧷️ The way an actor at `x` faces something at `towards`: right when that is not to its left (the twin's `towards >= x ? 1 : -1`), else left.
pub(crate) fn facing_to(towards: f64, x: f64) -> Facing {
    if towards >= x {
        Facing::Right
    } else {
        Facing::Left
    }
}

/// 🎰️ The blink of an actor that begins anew at tick `now`: 2…6 s ahead for a unit draw.
pub(crate) fn blink_at(now: Ticks, unit: f64) -> Ticks {
    now + BLINK_LOW + ((BLINK_HIGH - BLINK_LOW) as f64 * unit).floor() as Ticks
}

/// 🫂️ How far apart the feet of two actors on one perch must stay for their bodies not to overlap: half of both widths.
pub(crate) fn shoulders(one: &Species, other: &Species) -> f64 {
    (one.size.width + other.size.width) / 2.0
}

/// 🎵️ The ticks of one hop of a hopping gait while it walks with `clip` — the gait covers ground in whole hops — and 1 for every other gait.
pub(crate) fn beat_of(kind: &Species, clip: Option<&Clip>) -> Ticks {
    match clip {
        Some(clip) if kind.locomotion.gait == Gait::Hop => clip_ticks(clip),
        _ => 1,
    }
}
//#endregion 🔖️Lookups

//#region 🔖️Activities
/// 🔀️ An actor takes up an activity at tick `now`: its needs are settled over the span of the one it leaves.
pub(crate) fn shift(draft: &mut Draft<'_>, index: usize, activity: Activity, now: Ticks) {
    let temperament = draft.kinds[index].temperament;
    let body = &mut draft.stage.actors[index];
    body.needs = needs_after(body.needs, body.activity, now - body.since, temperament);
    body.activity = activity;
    body.since = now;
}

/// 🛋️ An actor comes to rest: idle where it stands for a drawn dwell with a drawn idle clip, without a partner.
pub(crate) fn settle(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let words = random_words(&actor_key(draft, index), 2);
    shift(draft, index, Activity::Idle, now);
    let until = now + dwell_of(Activity::Idle, draft.stage.mode, unit_of(words[0]));
    let clip = clip_at(draft.kinds[index], Activity::Idle, unit_of(words[1]));
    let body = &mut draft.stage.actors[index];
    body.until = until;
    body.clip = clip;
    body.partner = None;
    body.goal = body.x;
    body.vx = 0.0;
    body.vy = 0.0;
}

/// ✂️ An actor lets go of its partner: both links are cut, and a partner that was on its way, waiting or in the middle of the encounter comes to rest (one that sulks keeps sulking).
pub(crate) fn release(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let Some(partner) = draft.stage.actors[index].partner.take() else {
        return;
    };
    let Some(other) = index_of(&draft.stage.actors, &partner) else {
        return;
    };
    if draft.stage.actors[other].partner.as_deref() != Some(draft.stage.actors[index].species.as_str()) {
        return;
    }
    draft.stage.actors[other].partner = None;
    if draft.stage.actors[other].activity != Activity::Sulk && !draft.stage.actors[other].leaving {
        settle(draft, other, now);
    }
}

/// 🚪️ An actor leaves the stage for good: it is taken out of the draft.
pub(crate) fn remove(draft: &mut Draft<'_>, index: usize) {
    draft.stage.actors.remove(index);
    draft.kinds.remove(index);
    draft.streams.remove(index);
}
//#endregion 🔖️Activities

//#region 🔖️Stretches
/// 🔪️ Stretches (pairs of ends) without the interval `(low, high)`.
pub(crate) fn carve(stretches: &[[f64; 2]], low: f64, high: f64) -> Vec<[f64; 2]> {
    let mut kept = Vec::with_capacity(stretches.len() + 1);
    for &[x0, x1] in stretches {
        if high <= x0 || low >= x1 {
            kept.push([x0, x1]);
        } else {
            if low > x0 {
                kept.push([x0, low]);
            }
            if high < x1 {
                kept.push([high, x1]);
            }
        }
    }
    kept
}

/// 👪️ How many actors stand on a perch: on its surface, within its ends.
pub(crate) fn crowd_on(draft: &Draft<'_>, perch: &Perch) -> usize {
    let mut crowd = 0;
    for body in &draft.stage.actors {
        if body.perch.as_deref() == Some(perch.surface.as_str()) && body.x >= perch.x0 && body.x <= perch.x1 {
            crowd += 1;
        }
    }
    crowd
}
//#endregion 🔖️Stretches

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
