//! 🎪️ The stage: the pure fold that makes pets alive. `advance` folds events into a stage, `frame_of` projects a stage into what a render target draws; the same seed and the same events yield the same frames, bit for bit, as the TypeScript twin.
//!
//! NORMATIVE ORDER (the twin's, step by step). Actors are kept in `menagerie.species` order. One tick:
//! 1. per actor, in that order: fade (out while leaving and not walking, gone at 0; otherwise towards its presence —
//!    whole, or see-through while the pointer rests on it) → turning round (an actor that does not face its heading
//!    faces it at once and its drawing follows over 8 ticks, up to `faced`; a walker strides only once it faces its
//!    goal squarely) → motion (`walk`: on every beat the end at the goal or before a neighbour, else `stride_to`;
//!    `hop`: `hop_step` + `landing_of`, a floating gait glides straight; `fall`: `fall_step` + `landing_of`; a sleeper
//!    wakes when the pointer is near; an idle one perks up when the pointer has come to rest beside it) → gaze spring
//!    → blink schedule → mood → the end of its activity when `tick ≥ until`;
//! 2. the stage: a pair whose partners both wait begins its encounter; on every whole second a new pair may be drawn,
//!    and then whoever is wanted but not on stage arrives when a perch has room.
//!
//! DISTANCE. Actors of one surface never stand in each other. Whoever arrives, lands or picks a goal keeps a
//! comfortable gap (8 px between the bodies); a walker stops before a neighbour (6 px); a ride on a perch that shrank
//! sets the actors apart again; and whoever does not fit — a perch holds as many as fit with the comfortable gap — is
//! crowded out: it fades where it stood and, while it is wanted, arrives anew on a perch with room. Newcomers spread
//! out: they arrive on a perch that holds the fewest, on the ground — the lowest perches of the stage — only when no
//! higher one is as empty. And when a survey brings new ground, the scenery has changed: whoever stood on ground that
//! vanished is gone with it and arrives anew (without new ground it falls), and whoever shares a perch moves to one
//! that holds nobody.
//!
//! THE POINTER. Outside a time of concentration a pet that stands idle by itself attends to the pointer while it is
//! worth a look (it moved within the last 4 s): its pupils follow it, the bone that carries its first eye leans
//! after the pupils, it turns round when the pointer is clearly behind it (at most once a second), and when the
//! pointer has come to rest beside it for half a second and it is curious enough it greets it, which costs curiosity.
//! Pupils and lean follow the gaze, which reaches half of its way at 32 px from the eyes: `look_offset(…, 32)`.
//!
//! DRAWS. One key per draw, `[seed, stream, counter]` as `u32` words, words in the order listed:
//! - actor (`stream` = index of the species, `counter` = `actor.draws`, which starts at the tick of its arrival and
//!   wraps at 2³²): arrival and thaw `[idle dwell, idle clip, first blink]`; coming to rest `[idle dwell, idle clip]`;
//!   decision `random_pick` on word 0, then `[·, dwell, clip, goal or target]`; blink `[gap, double]`; poke and
//!   perking up `[dwell, clip]`; sulk `[dwell, clip]`;
//! - stage (`stream` = `STAGE_STREAM`, `counter` = `stage.draws`): arrival `[perch, place, facing]`; pairing
//!   `random_pick` on word 0, then `[·, chance]`; encounter `[kind, span, clip of the first, clip of the second]`.
//!
//! LAZINESS. Needs are settled when an activity ends (`needs_after` over its whole span) and rapports when they are
//! touched (`rapport_faded` since `stage.met`, the tick of the last change of any rapport), so neither depends on how
//! time is cut into `ticked` events. Ticks in which nothing can change are jumped over: an actor at rest (no motion,
//! no turn, no fade, gaze and mood on their targets) only has scheduled changes — the end of its activity, the end of
//! its blink, the pointer losing its interest, the next whole second on which a pair may be drawn or somebody who
//! waits off stage may arrive.
//!
//! BITS. Every expression is the twin's, term for term and in its order (no reassociation, no `mul_add`); the larger
//! and the smaller of two numbers are JavaScript's (`larger`, `smaller`), whole ticks are `i64` and enter floating
//! point exactly, a word becomes a unit by one division, and `floor(x)` of a unit expression becomes ticks or an index
//! by one cast. `advance` takes the stage by value and answers it: a fold owns what it folds, and whoever wants to
//! keep the stage it had clones it.
//!
//! @see ../🧠️behavior/🦀️.rs — limits, weights, dwells, encounters, needs, rapport
//! @see ../🏞️terrain/🦀️.rs — perches, strides, falls, hops
//! @see ../🎞️animation/🦀️.rs — clips, springs, blinks
//! @see ../🦴️rig/🦀️.rs — poses and matrices
//! @see ../🎲️randomness/🦀️.rs — counter-based draws
//! @see ../../🧬️schema/🦀️.rs — `Stage`, `Actor`, `StageEvent`, `Frame`
//! @see ../🎪️stage/🟦️.ts — the TypeScript twin

use crate::animation::{blend_pose, clip_ticks, lid_at, sample_clip, spring_step, BLINK_TICKS, GAZE_DAMPING, GAZE_STIFFNESS};
use crate::behavior::{activity_weights, affinity_of, dwell_of, encounter_of, mood_of, needs_after, needs_of, rapport_after, rapport_faded, Situation, MODE_LIMITS};
use crate::randomness::{random_pick, random_words, unit_of, STAGE_STREAM};
use crate::rig::{look_offset, pupil_reach, rest_pose, solve_rig, BonePose, Pose};
use crate::schema::{Activity, Actor, ActorFrame, Clip, EyeFrame, Facing, Frame, Gait, Gaze, Menagerie, Perch, PetMode, Point, Rapport, Rate, Slug, Species, Stage, StageEvent, Surface, Surveyed, Ticks, ACTIVITIES, TICKS_PER_SECOND};
use crate::terrain::{fall_step, hop_landing, hop_of, hop_step, landing_of, larger, perch_at, perches_of, smaller, stride_to, Flight, Hop, HOP_DISTANCE, HOP_HEIGHT};
use crate::trigonometry::{clamp, smoothstep};
use std::borrow::Borrow;

//#region 🔖️Constants
const RATE: f64 = TICKS_PER_SECOND as f64;
const FADE_STEP: f64 = 0.0625;
const POINTER_TICKS: Ticks = 256;
const GAZE_REACH: f64 = 32.0;
const GAZE_REST: f64 = 0.000244140625;
const GAZE_CALM: f64 = 0.015625;
const GAZE_AHEAD: f64 = 0.3;
const GAZE_SULK: f64 = 0.5;
const GAZE_FALL: f64 = 0.8;
const EYE_HEIGHT: f64 = 0.6;
const BLINK_LOW: Ticks = 128;
const BLINK_HIGH: Ticks = 384;
const BLINK_AGAIN: Ticks = 7;
const MOOD_EASE: f64 = 0.03125;
const MOOD_REST: f64 = 0.0009765625;
const POKE_REACH: f64 = 1.2;
const POKE_CHEER: f64 = 0.3;
const WAKE_REACH: f64 = 1.5;
const BLEND_TICKS: Ticks = 8;
const BREATH_STAGGER: Ticks = 37;
const PATIENCE: Ticks = 1920;
const WARMUP: Ticks = 1280;
const ENCOUNTER_REACH: f64 = 12.0;
const ENCOUNTER_RISE: f64 = 3.0;
const PAIR_WEIGHT: f64 = 0.25;
const MEET_GAP: f64 = 6.0;
const COMFORT_GAP: f64 = 8.0;
const LEAVE_REACH: f64 = 3.0;
const STROLL_LEAST: f64 = 0.75;
const GLIDE_TICKS: Ticks = 256;
const SHY_OPACITY: f64 = 0.35;
const SHY_STEP: f64 = 0.03125;
const SHY_REACH: f64 = 4.0;
const TURN_TICKS: Ticks = 8;
const TURN_REST: Ticks = 56;
const TURN_CLEAR: f64 = 12.0;
const LEAN_TURN: f64 = 5.0;
const LEAN_REACH: f64 = 1.5;
const LEAN_NOD: f64 = 1.0;
const PERK_LINGER: Ticks = 32;
const PERK_URGE: f64 = 0.5;
const PERK_COST: f64 = 0.6;
const CONTACT: f64 = 0.0078125;
const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
const CENTRED: Gaze = Gaze { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0 };
//#endregion 🔖️Constants

//#region 🔖️Draft
/// 🗒️ A stage being folded: its actors in `menagerie.species` order, each beside its species and its stream.
struct Draft<'a> {
    stage: Stage,
    kinds: Vec<&'a Species>,
    streams: Vec<u32>,
}

/// 🛖️ A perch (its index among the perches of the stage) with the stretches (pairs of ends) a species can arrive on, their summed length and how many actors stand on it.
struct Room {
    perch: usize,
    stretches: Vec<[f64; 2]>,
    span: f64,
    crowd: usize,
}

/// 🚀️ A flight an actor could take: the x it ends at, its launch velocity and its ticks in the air.
#[derive(Clone, Copy)]
struct Launch {
    x: f64,
    vx: f64,
    vy: f64,
    ticks: Ticks,
}

/// 🏟️ An empty stage for a seed: tick 0, mode `calm`, not quiet, nothing surveyed, nobody summoned.
pub fn open_stage(seed: u32) -> Stage {
    Stage {
        seed,
        tick: 0,
        mode: PetMode::Calm,
        quiet: false,
        width: 0.0,
        height: 0.0,
        pointer: None,
        pointed: 0,
        glances: Vec::new(),
        surfaces: Vec::new(),
        keepouts: Vec::new(),
        perches: Vec::new(),
        wanted: Vec::new(),
        actors: Vec::new(),
        rapports: Vec::new(),
        met: 0,
        draws: 0,
    }
}

/// 📝️ A working copy of a stage: its actors in `menagerie.species` order, each beside its species and its stream; of two actors of one species the first stays, and an actor of a species the menagerie does not have is dropped.
fn draft_of(menagerie: &Menagerie, mut stage: Stage) -> Draft<'_> {
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
fn sealed(draft: Draft<'_>) -> Stage {
    draft.stage
}
//#endregion 🔖️Draft

//#region 🔖️Lookups
/// 🎈️ How high a species floats above its perch: its hover when its gait is `float`, else 0.
fn hover_of(kind: &Species) -> f64 {
    if kind.locomotion.gait == Gait::Float {
        kind.locomotion.hover.unwrap_or(0.0)
    } else {
        0.0
    }
}

/// 🔎️ The index of the actor of a species on stage, or `None` (the twin's −1).
fn index_of<A: Borrow<Actor>>(actors: &[A], species: &str) -> Option<usize> {
    actors.iter().position(|actor| Borrow::<Actor>::borrow(actor).species == species)
}

/// 🪵️ The surface with an id, or `None`.
fn surface_of<'s>(surfaces: &'s [Surface], id: &str) -> Option<&'s Surface> {
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
fn clip_at(kind: &Species, activity: Activity, unit: f64) -> Option<Slug> {
    let clips = clips_of(kind, activity);
    if clips.is_empty() {
        return None;
    }
    let index = (unit * clips.len() as f64).floor();
    Some(clips[if index < clips.len() as f64 { index as usize } else { clips.len() - 1 }].clone())
}

/// 📼️ The clip of a species with an id, or `None`.
fn clip_of<'s>(kind: &'s Species, id: Option<&str>) -> Option<&'s Clip> {
    let id = id?;
    kind.clips.iter().find(|clip| clip.id == id)
}

/// 🫁️ The first idle clip of a species, the loop that keeps running underneath everything else; `None` when it has none.
fn breath_of(kind: &Species) -> Option<&Clip> {
    let idle = kind.repertoire.idle.as_deref()?;
    clip_of(kind, Some(idle.first()?))
}

/// 🔑️ The key of the next draw of an actor; the draw is counted.
fn actor_key(draft: &mut Draft<'_>, index: usize) -> [u32; 3] {
    let body = &mut draft.stage.actors[index];
    let counter = body.draws;
    body.draws = counter.wrapping_add(1);
    [draft.stage.seed, draft.streams[index], counter]
}

/// 🗝️ The key of the next draw of the stage; the draw is counted.
fn stage_key(draft: &mut Draft<'_>) -> [u32; 3] {
    let counter = draft.stage.draws;
    draft.stage.draws = counter.wrapping_add(1);
    [draft.stage.seed, STAGE_STREAM, counter]
}

/// 😴️ Whether the pointer is close enough to an actor to keep it awake: within 1.5 × its height of the middle of its body.
fn watched(pointer: Option<Point>, actor: &Actor, kind: &Species) -> bool {
    let Some(pointer) = pointer else {
        return false;
    };
    let height = kind.size.height;
    let dx = pointer.x - actor.x;
    let dy = pointer.y - (actor.y - height / 2.0);
    let reach = WAKE_REACH * height;
    dx * dx + dy * dy <= reach * reach
}

/// 🧷️ The way an actor at `x` faces something at `towards`: right when that is not to its left (the twin's `towards >= x ? 1 : -1`), else left.
fn facing_to(towards: f64, x: f64) -> Facing {
    if towards >= x {
        Facing::Right
    } else {
        Facing::Left
    }
}

/// 🎰️ The blink of an actor that begins anew at tick `now`: 2…6 s ahead for a unit draw.
fn blink_at(now: Ticks, unit: f64) -> Ticks {
    now + BLINK_LOW + ((BLINK_HIGH - BLINK_LOW) as f64 * unit).floor() as Ticks
}

/// 🫣️ How visible an actor wants to be: see-through (0.35) while the pointer rests on it — inside its box, grown by 4 px — so that whoever points can see what lies beneath, else whole.
fn presence_of(pointer: Option<Point>, actor: &Actor, kind: &Species) -> f64 {
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

/// 🧿️ The way an actor ought to face at tick `now`, or `None` when nothing says (the twin's 0): towards its goal while it walks or hops; towards its partner while it waits for it or acts with it, away from it while it sulks; and, standing idle by itself on a perch outside a time of concentration, towards the pointer while that is worth a look (it moved within the last 4 s) and clearly on one side — more than 12 px beyond its body — once the actor has rested for 56 ticks after its last turn, so a pointer that crosses over and back never makes it flip back and forth.
fn heading_of<A: Borrow<Actor>>(stage: &Stage, actors: &[A], kinds: &[&Species], index: usize, now: Ticks) -> Option<Facing> {
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

/// 🫂️ How far apart the feet of two actors on one perch must stay for their bodies not to overlap: half of both widths.
fn shoulders(one: &Species, other: &Species) -> f64 {
    (one.size.width + other.size.width) / 2.0
}

/// 🎵️ The ticks of one hop of a hopping gait while it walks with `clip` — the gait covers ground in whole hops — and 1 for every other gait.
fn beat_of(kind: &Species, clip: Option<&Clip>) -> Ticks {
    match clip {
        Some(clip) if kind.locomotion.gait == Gait::Hop => clip_ticks(clip),
        _ => 1,
    }
}
//#endregion 🔖️Lookups

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
fn gaze_rests<A: Borrow<Actor>>(stage: &Stage, actors: &[A], kinds: &[&Species], index: usize, now: Ticks) -> bool {
    let gaze = Borrow::<Actor>::borrow(&actors[index]).gaze;
    if gaze.vx != 0.0 || gaze.vy != 0.0 {
        return false;
    }
    let goal = gaze_goal(stage, actors, kinds, index, now);
    gaze.x == goal.x && gaze.y == goal.y
}

/// 🔭️ One tick of the gaze of an actor: both axes spring towards the goal; within 1/4096 of it and slower than 1/64 per second the pupils snap onto it and rest. A sleeper's gaze eases to the centre the same way: the lean of its head follows the gaze, so nothing may jump behind the shut lids either.
fn look(draft: &mut Draft<'_>, index: usize, now: Ticks) {
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

//#region 🔖️Activities
/// 🔀️ An actor takes up an activity at tick `now`: its needs are settled over the span of the one it leaves.
fn shift(draft: &mut Draft<'_>, index: usize, activity: Activity, now: Ticks) {
    let temperament = draft.kinds[index].temperament;
    let body = &mut draft.stage.actors[index];
    body.needs = needs_after(body.needs, body.activity, now - body.since, temperament);
    body.activity = activity;
    body.since = now;
}

/// 🛋️ An actor comes to rest: idle where it stands for a drawn dwell with a drawn idle clip, without a partner.
fn settle(draft: &mut Draft<'_>, index: usize, now: Ticks) {
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
fn release(draft: &mut Draft<'_>, index: usize, now: Ticks) {
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
fn remove(draft: &mut Draft<'_>, index: usize) {
    draft.stage.actors.remove(index);
    draft.kinds.remove(index);
    draft.streams.remove(index);
}

/// 🏡️ An actor that has nowhere left to be is gone at once (always `false`): it lets go of its partner and arrives anew, spaced from the others, once it is wanted and a perch has room.
fn vanish(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    release(draft, index, now);
    remove(draft, index);
    false
}

/// 🚷️ An actor has no room where it is: it lets go of its partner and of its perch and fades out on the spot. While it is wanted it arrives anew once a perch has room.
fn crowd_out(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    release(draft, index, now);
    if matches!(draft.stage.actors[index].activity, Activity::Walk | Activity::Hop | Activity::Fall) {
        shift(draft, index, Activity::Idle, now);
        draft.stage.actors[index].clip = clip_at(draft.kinds[index], Activity::Idle, 0.0);
    }
    let body = &mut draft.stage.actors[index];
    body.leaving = true;
    body.perch = None;
    body.goal = body.x;
    body.vx = 0.0;
    body.vy = 0.0;
}

/// 🪑️ The place nearest to `x` on a perch where an actor stands a comfortable gap (8 px between the bodies) away from everybody else on that surface, or `None` when the perch has no such place.
fn vacancy(draft: &Draft<'_>, index: usize, perch: &Perch, x: f64) -> Option<f64> {
    let kind = draft.kinds[index];
    let half = kind.size.width / 2.0;
    if perch.x1 - half < perch.x0 + half {
        return None;
    }
    let mut stretches = vec![[perch.x0 + half, perch.x1 - half]];
    for (other, neighbour) in draft.stage.actors.iter().enumerate() {
        if other == index || neighbour.perch.as_deref() != Some(perch.surface.as_str()) {
            continue;
        }
        let room = shoulders(draft.kinds[other], kind) + COMFORT_GAP;
        stretches = carve(&stretches, neighbour.x - room, neighbour.x + room);
    }
    let mut nearest = None;
    let mut least = f64::INFINITY;
    for &[low, high] in &stretches {
        let place = clamp(x, low, high);
        let distance = (place - x).abs();
        if distance < least {
            nearest = Some(place);
            least = distance;
        }
    }
    nearest
}

/// 🕳️ An actor loses its perch and starts to fall.
fn drop(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    release(draft, index, now);
    shift(draft, index, Activity::Fall, now);
    let until = now + dwell_of(Activity::Fall, draft.stage.mode, 0.0);
    let clip = clip_at(draft.kinds[index], Activity::Fall, 0.0);
    let body = &mut draft.stage.actors[index];
    body.perch = None;
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

/// 🥾️ Where a walk from `x` towards `goal` really ends: a hopping gait covers ground in whole hops of its clip (speed × the length of the clip), as many as fit before the goal; every other gait goes all the way.
fn paced(kind: &Species, clip: Option<&str>, x: f64, goal: f64) -> f64 {
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
fn stroll(draft: &mut Draft<'_>, index: usize, goal: f64, clip: Option<Slug>, now: Ticks) {
    shift(draft, index, Activity::Walk, now);
    let until = now + dwell_of(Activity::Walk, draft.stage.mode, 0.0);
    let body = &mut draft.stage.actors[index];
    body.goal = goal;
    body.until = until;
    body.clip = clip;
}

/// 🧍️ An actor waits for its partner: idle, turning towards it, for as long as the stage has patience.
fn attend(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    shift(draft, index, Activity::Idle, now);
    let clip = clip_at(draft.kinds[index], Activity::Idle, 0.0);
    let body = &mut draft.stage.actors[index];
    body.until = now + PATIENCE;
    body.clip = clip;
    body.goal = body.x;
}

/// 😤️ An actor sulks for a drawn span, turning its back on its partner.
fn sulk(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let words = random_words(&actor_key(draft, index), 2);
    shift(draft, index, Activity::Sulk, now);
    let until = now + dwell_of(Activity::Sulk, draft.stage.mode, unit_of(words[0]));
    let clip = clip_at(draft.kinds[index], Activity::Sulk, unit_of(words[1]));
    let body = &mut draft.stage.actors[index];
    body.until = until;
    body.clip = clip;
}
//#endregion 🔖️Activities

//#region 🔖️Rapport
/// 🍂️ Every rapport fades over the ticks since the last change of any of them; what has faded to 0 is forgotten. `met` becomes `now`.
fn recall(draft: &mut Draft<'_>, now: Ticks) {
    let elapsed = now - draft.stage.met;
    if elapsed > 0 {
        draft.stage.rapports.retain_mut(|rapport| {
            rapport.drift = rapport_faded(rapport.drift, elapsed);
            rapport.drift != 0.0
        });
    }
    draft.stage.met = now;
}

/// 🪢️ The rapport of two actors after something between them (`rapport_after`); a new pair is listed last, the earlier species of the menagerie first, and a drift of 0 is forgotten.
fn bond(draft: &mut Draft<'_>, first: usize, second: usize, activity: Activity) {
    let early = if first < second { first } else { second };
    let late = if first < second { second } else { first };
    let a = &draft.stage.actors[early].species;
    let b = &draft.stage.actors[late].species;
    let mut found = false;
    draft.stage.rapports.retain_mut(|rapport| {
        if rapport.between[0] != *a || rapport.between[1] != *b {
            return true;
        }
        found = true;
        rapport.drift = rapport_after(rapport.drift, activity);
        rapport.drift != 0.0
    });
    if !found {
        let drift = rapport_after(0.0, activity);
        if drift != 0.0 {
            draft.stage.rapports.push(Rapport { between: [a.clone(), b.clone()], drift });
        }
    }
}

/// 🕊️ The end of a sulk: once the partner has stopped sulking too, the two mend their rapport by a little; the actor lets go of its partner either way.
fn reconcile(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let Some(partner) = body.partner.as_deref() else {
        return;
    };
    let other = index_of(&draft.stage.actors, partner);
    let pending = other.is_some_and(|other| draft.stage.actors[other].activity == Activity::Sulk && draft.stage.actors[other].partner.as_deref() == Some(body.species.as_str()));
    if let Some(other) = other.filter(|_| !pending) {
        recall(draft, now);
        bond(draft, index, other, Activity::Sulk);
    }
    draft.stage.actors[index].partner = None;
}
//#endregion 🔖️Rapport

//#region 🔖️Decision
/// 🪂️ Whether the arc of a hop keeps clear of what must stay free: on every tick on which the feet are above both ends of the hop (lower down the body is beside the things it hops between), the box of the body touches no keep-out and stays below the top of the stage.
fn soars(draft: &Draft<'_>, kind: &Species, from: Point, to: Point, hop: Hop) -> bool {
    let half = kind.size.width / 2.0;
    let ridge = smaller(from.y, to.y);
    let mut flight = Flight { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
    for left in (2..=hop.ticks).rev() {
        flight = hop_step(flight.x, flight.y, flight.vx, flight.vy, to, left);
        if flight.y >= ridge {
            continue;
        }
        let top = flight.y - kind.size.height;
        if top < 0.0 {
            return false;
        }
        for keepout in &draft.stage.keepouts {
            if keepout.width > 0.0 && keepout.height > 0.0 && larger(flight.x - half, keepout.x) < smaller(flight.x + half, keepout.x + keepout.width) && larger(top, keepout.y) < smaller(flight.y, keepout.y + keepout.height) {
                return false;
            }
        }
    }
    true
}

/// 🦘️ Where an actor could hop to: per other perch (in perch order) the place nearest to it that keeps half a body from the ends and a comfortable gap from the actors there, when `hop_of` grants the hop, the flight really ends on that perch and its arc covers nothing that must stay free ([`soars`]); a floating gait glides there in a straight line at twice its speed when the place is within the reach of a hop and four seconds.
fn hops_of(draft: &Draft<'_>, index: usize) -> Vec<Launch> {
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

/// 🛣️ The stretch `[low, high]` of its perch an actor can walk on without its body coming closer than `gap` to anybody else on that surface (`except` aside; a walker counts for the whole way it still has to go). The stretch always holds the place where the actor stands, and is that place alone for an actor without a perch.
fn clearway(draft: &Draft<'_>, index: usize, gap: f64, except: Option<usize>) -> [f64; 2] {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let Some(perch) = body.perch.as_deref().and_then(|surface| perch_at(&draft.stage.perches, surface, body.x)) else {
        return [body.x, body.x];
    };
    let mut low = perch.x0 + kind.size.width / 2.0;
    let mut high = perch.x1 - kind.size.width / 2.0;
    for (other, neighbour) in draft.stage.actors.iter().enumerate() {
        if other == index || Some(other) == except || neighbour.perch != body.perch {
            continue;
        }
        let room = shoulders(draft.kinds[other], kind) + gap;
        let walks = neighbour.activity == Activity::Walk;
        let left = if walks && neighbour.goal < neighbour.x { neighbour.goal } else { neighbour.x };
        let right = if walks && neighbour.goal > neighbour.x { neighbour.goal } else { neighbour.x };
        if right <= body.x {
            low = larger(low, right + room);
        } else if left >= body.x {
            high = smaller(high, left - room);
        } else {
            low = body.x;
            high = body.x;
        }
    }
    [smaller(low, body.x), larger(high, body.x)]
}

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
        Situation { mode, quiet: draft.stage.quiet, movers, fidgeters, roam: larger(at - low, high - at) >= STROLL_LEAST * width, hops: !launches.is_empty() || elsewhere, crowd, watched: watched(draft.stage.pointer, body, kind) },
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
fn conclude(draft: &mut Draft<'_>, index: usize, now: Ticks) {
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

//#region 🔖️Motion
/// 🚧️ Whether somebody stands in the way of the next `ticks` strides of a walker: another actor on its surface, ahead of it, whose body its own would come closer to than 6 px (bodies that are that close already count as touching within 1/128 px).
fn hindered(draft: &Draft<'_>, index: usize, ticks: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let way = if body.goal > body.x { 1.0 } else { -1.0 };
    let far = (larger(kind.locomotion.speed, 0.0) * ticks as f64) / RATE;
    let ahead = clamp(body.goal, body.x - far, body.x + far);
    for (other, neighbour) in draft.stage.actors.iter().enumerate() {
        if other == index || neighbour.perch != body.perch || (neighbour.x - body.x) * way <= 0.0 {
            continue;
        }
        if (neighbour.x - ahead) * way < shoulders(draft.kinds[other], kind) + MEET_GAP - CONTACT {
            return true;
        }
    }
    false
}

/// 👣️ One tick of a walk. On every beat — every tick for a walking or floating gait, the start of every hop for a hopping one — the walk ends when the goal is no farther than one stride (the actor stands on it then) or when somebody is in the way of the next beat; otherwise a stride towards the goal follows. A hopping gait that reaches its goal within a hop, or finds somebody in its way in mid-hop, finishes that hop on the spot. The walk also ends when the stage loses patience. At its end a leaver starts to fade, an actor with a partner waits for it and anyone else comes to rest.
fn stride(draft: &mut Draft<'_>, index: usize, now: Ticks) {
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
fn plunge(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
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
fn fly(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
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

/// 😉️ The blink schedule of an actor: when a blink has run its 12 ticks the next one is drawn, 2…6 s ahead, or, one time in six, 7 ticks ahead (a double blink). A sleeper's lids stay shut and draw nothing.
fn wink(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    if body.activity == Activity::Sleep || now < body.blink + BLINK_TICKS {
        return;
    }
    let words = random_words(&actor_key(draft, index), 2);
    draft.stage.actors[index].blink = if unit_of(words[1]) * 6.0 < 1.0 { now + BLINK_AGAIN } else { blink_at(now, unit_of(words[0])) };
}

/// 🎭️ One tick of the mood of an actor: a thirty-second of the way to the mood of its activity, onto it once closer than 1/1024.
fn cheer(body: &mut Actor) {
    let want = mood_of(body.activity);
    if body.mood == want {
        return;
    }
    let next = body.mood + (want - body.mood) * MOOD_EASE;
    body.mood = if (want - next).abs() < MOOD_REST { want } else { next };
}

/// 🌀️ One tick of turning round: an actor that faces its way squarely and does not face its heading turns to it ([`turn`]). `true` while a walker must not stride yet: while it turns, and on the tick its turn ends — its walk begins anew at that tick, so its clip starts with its first stride.
fn swivel(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
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
fn perk(draft: &mut Draft<'_>, index: usize, now: Ticks) {
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
    let body = &mut draft.stage.actors[index];
    cheer(body);
    if !body.leaving && now >= body.until {
        conclude(draft, index, now);
    }
    true
}
//#endregion 🔖️Motion

//#region 🔖️Encounters
/// 🙌️ Whether an actor can be drawn into an encounter: standing idle and whole on a perch, without a partner, not leaving.
fn sociable(actor: &Actor) -> bool {
    actor.activity == Activity::Idle && actor.partner.is_none() && !actor.leaving && actor.opacity == 1.0 && actor.perch.is_some()
}

/// 📅️ The first whole second at or after tick `from` on which the stage may draw a pair, or −1 while it may not: the mode has encounters, it is not a quiet time, nobody has a partner, at least two actors are sociable, and the warm-up (20 s before the first encounter) or the gap of the mode since the last change of a rapport has passed.
fn pairing_tick<A: Borrow<Actor>>(stage: &Stage, actors: &[A], from: Ticks) -> Ticks {
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

/// 💌️ On a whole second the stage may pair two sociable actors that stand near each other (no farther apart than 12 of their mean widths, no more than 3 in height): one stage draw picks a pair, weighted `(0.25 + |authored affinity|) × mean sociability` (pairs that feel something for each other meet more often than strangers, and whoever has just had company lets others go first), and decides with the chance `rate × mean sociability`. Sociability is the need as it stands now. The pair then approaches; when the mode has room for one mover only, the more sociable of the two walks.
fn pair(menagerie: &Menagerie, draft: &mut Draft<'_>, now: Ticks) {
    if pairing_tick(&draft.stage, &draft.stage.actors, now) != now {
        return;
    }
    let limits = MODE_LIMITS[draft.stage.mode];
    let actors = &draft.stage.actors;
    let movers = actors.iter().filter(|body| matches!(body.activity, Activity::Walk | Activity::Hop)).count();
    if movers >= limits.movers {
        return;
    }
    let mut firsts: Vec<usize> = Vec::new();
    let mut seconds: Vec<usize> = Vec::new();
    let mut drives: Vec<u8> = Vec::new();
    let mut socials: Vec<f64> = Vec::new();
    let mut weights: Vec<f64> = Vec::new();
    for (first, one) in actors.iter().enumerate() {
        if !sociable(one) {
            continue;
        }
        let eager = needs_after(one.needs, Activity::Idle, now - one.since, draft.kinds[first].temperament).sociability;
        for (second, two) in actors.iter().enumerate().skip(first + 1) {
            if !sociable(two) {
                continue;
            }
            let width = (draft.kinds[first].size.width + draft.kinds[second].size.width) / 2.0;
            if (one.x - two.x).abs() > ENCOUNTER_REACH * width || (one.y - two.y).abs() > ENCOUNTER_RISE * width || parted(draft, first, second) {
                continue;
            }
            let keen = needs_after(two.needs, Activity::Idle, now - two.since, draft.kinds[second].temperament).sociability;
            let social = (eager + keen) / 2.0;
            firsts.push(first);
            seconds.push(second);
            drives.push(if eager >= keen { 1 } else { 2 });
            socials.push(social);
            weights.push((PAIR_WEIGHT + affinity_of(menagerie, &[], &one.species, &two.species).abs()) * social);
        }
    }
    if firsts.is_empty() {
        return;
    }
    let key = stage_key(draft);
    let Some(chosen) = random_pick(&key, &weights) else {
        return;
    };
    let lucky = unit_of(random_words(&key, 2)[1]) < limits.encounter_rate * socials[chosen];
    if !lucky {
        return;
    }
    approach(draft, firsts[chosen], seconds[chosen], if limits.movers - movers >= 2 { 0 } else { drives[chosen] }, now);
}

/// 🚻️ Whether somebody stands between two actors of one surface, so that they could not come together without walking through it.
fn parted(draft: &Draft<'_>, first: usize, second: usize) -> bool {
    let one = &draft.stage.actors[first];
    let two = &draft.stage.actors[second];
    if one.perch != two.perch {
        return false;
    }
    let low = smaller(one.x, two.x);
    let high = larger(one.x, two.x);
    draft.stage.actors.iter().enumerate().any(|(other, between)| other != first && other != second && between.perch == one.perch && between.x >= low && between.x <= high)
}

/// 🧭️ Where an actor may walk to on its perch to meet its partner (the actor at `partner`), as close to `x` as its clear way and its gait allow.
fn reachable(draft: &Draft<'_>, index: usize, partner: usize, x: f64) -> f64 {
    let kind = draft.kinds[index];
    let [low, high] = clearway(draft, index, COMFORT_GAP, Some(partner));
    paced(kind, clip_at(kind, Activity::Walk, 0.0).as_deref(), draft.stage.actors[index].x, clamp(x, low, high))
}

/// 🤝️ Two actors become partners and come together until their bodies are 6 px apart (what reaches beyond the box of a body — an arm, a ray — then just touches): both walk to the middle (`walker` 0), or only the first (1) or the second (2) walks while the other waits, and whoever need not move waits at once. Each stays on its own perch.
fn approach(draft: &mut Draft<'_>, first: usize, second: usize, walker: u8, now: Ticks) {
    let partner = draft.stage.actors[second].species.clone();
    draft.stage.actors[first].partner = Some(partner);
    let partner = draft.stage.actors[first].species.clone();
    draft.stage.actors[second].partner = Some(partner);
    let one = draft.stage.actors[first].x;
    let two = draft.stage.actors[second].x;
    let apart = (draft.kinds[first].size.width + draft.kinds[second].size.width) / 2.0 + MEET_GAP;
    let side = if two >= one { 1.0 } else { -1.0 };
    let close = (two - one).abs() <= apart;
    let middle = (one + two) / 2.0;
    let near = if close || walker == 2 { one } else { reachable(draft, first, second, if walker == 1 { two - side * apart } else { middle - (side * apart) / 2.0 }) };
    let far = if close || walker == 1 { two } else { reachable(draft, second, first, if walker == 2 { one + side * apart } else { middle + (side * apart) / 2.0 }) };
    if near == one {
        attend(draft, first, now);
    } else {
        stroll(draft, first, near, clip_at(draft.kinds[first], Activity::Walk, 0.0), now);
    }
    if far == two {
        attend(draft, second, now);
    } else {
        stroll(draft, second, far, clip_at(draft.kinds[second], Activity::Walk, 0.0), now);
    }
}

/// 🎉️ Partners that both wait begin their encounter: the rapports fade up to now, one stage draw picks the kind from their affinity (`encounter_of`), the span they share and a clip for each; they face each other and their rapport moves.
fn meet(menagerie: &Menagerie, draft: &mut Draft<'_>, now: Ticks) {
    for first in 0..draft.stage.actors.len() {
        let one = &draft.stage.actors[first];
        if one.activity != Activity::Idle {
            continue;
        }
        let Some(second) = one.partner.as_deref().and_then(|partner| index_of(&draft.stage.actors, partner)).filter(|&second| second > first) else {
            continue;
        };
        let two = &draft.stage.actors[second];
        if two.activity != Activity::Idle || two.partner.as_deref() != Some(one.species.as_str()) {
            continue;
        }
        recall(draft, now);
        let words = random_words(&stage_key(draft), 4);
        let one = &draft.stage.actors[first];
        let two = &draft.stage.actors[second];
        let kind = encounter_of(affinity_of(menagerie, &draft.stage.rapports, &one.species, &two.species), unit_of(words[0]));
        let until = now + dwell_of(kind, draft.stage.mode, unit_of(words[1]));
        shift(draft, first, kind, now);
        shift(draft, second, kind, now);
        let clip = clip_at(draft.kinds[first], kind, unit_of(words[2]));
        let one = &mut draft.stage.actors[first];
        one.until = until;
        one.clip = clip;
        let clip = clip_at(draft.kinds[second], kind, unit_of(words[3]));
        let two = &mut draft.stage.actors[second];
        two.until = until;
        two.clip = clip;
        bond(draft, first, second, kind);
    }
}
//#endregion 🔖️Encounters

//#region 🔖️Time
/// 🚏️ The first whole second at or after tick `from` on which a species that is wanted but not on stage may arrive, or −1 while nobody waits, no perch exists or the stage is still (a still stage lets them arrive with its events).
fn arrival_tick<A: Borrow<Actor>>(menagerie: &Menagerie, stage: &Stage, actors: &[A], from: Ticks) -> Ticks {
    if stage.mode == PetMode::Still || stage.perches.is_empty() {
        return -1;
    }
    if menagerie.species.iter().any(|kind| stage.wanted.contains(&kind.id) && index_of(actors, &kind.id).is_none()) {
        return (from + TICKS_PER_SECOND - 1).div_euclid(TICKS_PER_SECOND) * TICKS_PER_SECOND;
    }
    -1
}

/// 💤️ How many of the next `left` ticks change nothing but the tick: 0 while any actor moves, turns or is about to, fades or has pupils or a mood off their targets, else the ticks before the earliest scheduled change — among them, while a pointer is on stage, the tick an actor may turn to it again and the tick it has come to rest for half a second.
fn lull(menagerie: &Menagerie, draft: &Draft<'_>, left: Ticks) -> Ticks {
    let stage = &draft.stage;
    let next = stage.tick + 1;
    let mut horizon = next + left;
    for (index, body) in stage.actors.iter().enumerate() {
        let activity = body.activity;
        if body.leaving || body.opacity != presence_of(stage.pointer, body, draft.kinds[index]) || matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall) {
            return 0;
        }
        if body.mood != mood_of(activity) || next <= body.faced || !gaze_rests(stage, &stage.actors, &draft.kinds, index, next) {
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
fn pass<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, ticks: u64) {
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

//#region 🔖️Events
/// 📐️ The perches of the stage for everyone who is on it or wanted: clearance = the tallest of them plus its hover, minimum = the widest, so that every perch carries any of them.
fn measure(menagerie: &Menagerie, draft: &mut Draft<'_>) {
    let mut tallest = 0.0;
    let mut widest = 0.0;
    for kind in &menagerie.species {
        if !draft.stage.wanted.contains(&kind.id) && index_of(&draft.stage.actors, &kind.id).is_none() {
            continue;
        }
        tallest = larger(tallest, kind.size.height + hover_of(kind));
        widest = larger(widest, kind.size.width);
    }
    draft.stage.perches = perches_of(&draft.stage.surfaces, &draft.stage.keepouts, draft.stage.width, draft.stage.height, tallest, widest);
}

/// 🚚️ How far the surface an actor stands on moved its left end from where the surfaces were `before` (`None` when they did not change): 0 for an actor in the air and for a surface that is new or gone.
fn haul(draft: &Draft<'_>, index: usize, before: Option<&[Surface]>) -> f64 {
    let Some(surface) = draft.stage.actors[index].perch.as_deref() else {
        return 0.0;
    };
    match (surface_of(before.unwrap_or(&draft.stage.surfaces), surface), surface_of(&draft.stage.surfaces, surface)) {
        (Some(old), Some(fresh)) => fresh.x0 - old.x0,
        _ => 0.0,
    }
}

/// 🚡️ A grounded actor rides its surface: it moves with the surface's left end and height, and is held inside the nearest perch of that surface (fading in anew when that carries it farther than its own width). When that surface has no perch left it stays on a perch of another surface that lies exactly under its feet (an element that was replaced by its like); without one it falls — unless the survey also brought new ground (`uprooted`: the scenery changed, and a fall would pass in front of whatever is there now) or the stage is still: then it is gone with its ground and arrives anew. `before` are the surfaces as they were, `None` when they did not change. Answers how far the perch pulled the actor from where its surface carried it (0 for an actor in the air or falling), or `None` when the actor is gone.
fn carry(draft: &mut Draft<'_>, index: usize, before: Option<&[Surface]>, uprooted: bool, now: Ticks) -> Option<f64> {
    let kind = draft.kinds[index];
    let moved = haul(draft, index, before);
    let body = &draft.stage.actors[index];
    let Some(surface) = body.perch.as_deref() else {
        return Some(0.0);
    };
    let x = body.x + moved;
    let half = kind.size.width / 2.0;
    let mut best: Option<&Perch> = None;
    let mut least = f64::INFINITY;
    for perch in &draft.stage.perches {
        if perch.surface != surface {
            continue;
        }
        let low = perch.x0 + half;
        let high = perch.x1 - half;
        let gap = if x < low {
            low - x
        } else if x > high {
            x - high
        } else {
            0.0
        };
        if gap < least {
            best = Some(perch);
            least = gap;
        }
    }
    if best.is_none() {
        best = landing_of(&draft.stage.perches, body.x, body.y + hover_of(kind), body.y + hover_of(kind));
        least = 0.0;
    }
    let Some(best) = best else {
        if uprooted || draft.stage.mode == PetMode::Still {
            vanish(draft, index, now);
            return None;
        }
        drop(draft, index, now);
        return Some(0.0);
    };
    let elsewhere = (best.surface != surface).then(|| best.surface.clone());
    let (low, high, y) = (best.x0 + half, best.x1 - half, best.y - hover_of(kind));
    let fades = least > kind.size.width && draft.stage.mode != PetMode::Still;
    let body = &mut draft.stage.actors[index];
    if elsewhere.is_some() {
        body.perch = elsewhere;
    }
    body.x = clamp(x, low, high);
    body.y = y;
    body.goal = clamp(body.goal + moved, low, high);
    if fades && !body.leaving {
        body.opacity = 0.0;
    }
    Some(least)
}

/// 💺️ Nobody stands in anybody after a ride. Per perch, the actors it holds (leavers aside) must fit with a comfortable gap between their bodies (the sum of their widths plus 8 px per neighbour pair within the width of the perch; one actor always fits): while they do not, the one its perch pulled farthest (`strains`) — among equals the last in `menagerie.species` order — is crowded out where it stood (`places`), as visible as it was (`shown`), and arrives anew once a perch has room. The others are then set apart, each moved as little as possible, left to right and back: the feet of neighbours end at least as far apart as before the ride, held between half of both widths (bodies that touch) and 8 px more. On a still stage whoever is crowded out is gone at once.
fn seat(draft: &mut Draft<'_>, places: &[f64], strains: &[f64], shown: &[f64]) {
    let now = draft.stage.tick;
    let mut gone: Vec<usize> = Vec::new();
    for at in 0..draft.stage.perches.len() {
        let perch = &draft.stage.perches[at];
        let (x0, x1) = (perch.x0, perch.x1);
        let mut members: Vec<usize> = Vec::new();
        for (index, body) in draft.stage.actors.iter().enumerate() {
            if body.leaving || body.perch.as_deref() != Some(perch.surface.as_str()) || body.x < x0 || body.x > x1 {
                continue;
            }
            let mut slot = members.len();
            while slot > 0 && places[members[slot - 1]] > places[index] {
                slot -= 1;
            }
            members.insert(slot, index);
        }
        while members.len() > 1 {
            let mut need = (members.len() - 1) as f64 * COMFORT_GAP;
            for &member in &members {
                need += draft.kinds[member].size.width;
            }
            if need <= x1 - x0 {
                break;
            }
            let mut worst = 0;
            for slot in 1..members.len() {
                let strain = strains[members[slot]];
                let most = strains[members[worst]];
                if strain > most || (strain == most && members[slot] > members[worst]) {
                    worst = slot;
                }
            }
            let index = members.remove(worst);
            draft.stage.actors[index].x = places[index];
            draft.stage.actors[index].opacity = shown[index];
            crowd_out(draft, index, now);
            gone.push(index);
        }
        for slot in 1..members.len() {
            let (before, member) = (members[slot - 1], members[slot]);
            let hard = shoulders(draft.kinds[before], draft.kinds[member]);
            let apart = clamp(places[member] - places[before], hard, hard + COMFORT_GAP);
            let held = draft.stage.actors[before].x;
            if draft.stage.actors[member].x - held < apart {
                draft.stage.actors[member].x = held + apart;
            }
        }
        for slot in (0..members.len()).rev() {
            let member = members[slot];
            let high = x1 - draft.kinds[member].size.width / 2.0;
            if draft.stage.actors[member].x > high {
                draft.stage.actors[member].x = high;
            }
            if slot < members.len() - 1 {
                let after = members[slot + 1];
                let hard = shoulders(draft.kinds[after], draft.kinds[member]);
                let apart = clamp(places[after] - places[member], hard, hard + COMFORT_GAP);
                let held = draft.stage.actors[after].x;
                if held - draft.stage.actors[member].x < apart {
                    draft.stage.actors[member].x = held - apart;
                }
            }
            let body = &mut draft.stage.actors[member];
            if body.activity != Activity::Walk {
                body.goal = body.x;
            }
        }
    }
    if draft.stage.mode != PetMode::Still {
        return;
    }
    gone.sort_unstable_by(|one, other| other.cmp(one));
    for index in gone {
        remove(draft, index);
    }
}

/// 🆕️ Whether the surfaces of the stage hold one that was not there `before`: new ground.
fn widened(draft: &Draft<'_>, before: &[Surface]) -> bool {
    draft.stage.surfaces.iter().any(|surface| surface_of(before, &surface.id).is_none())
}

/// 🎠️ Every grounded actor rides its surface from where the surfaces were `before` (`uprooted`: new ground came with them), and is then seated so that nobody stands in anybody.
fn ride(draft: &mut Draft<'_>, before: Option<&[Surface]>, uprooted: bool) {
    let now = draft.stage.tick;
    let mut places: Vec<f64> = Vec::with_capacity(draft.stage.actors.len());
    let mut strains: Vec<f64> = Vec::with_capacity(draft.stage.actors.len());
    let mut shown: Vec<f64> = Vec::with_capacity(draft.stage.actors.len());
    let mut index = 0;
    while index < draft.stage.actors.len() {
        let place = draft.stage.actors[index].x + haul(draft, index, before);
        let opacity = draft.stage.actors[index].opacity;
        let Some(strain) = carry(draft, index, before, uprooted, now) else {
            continue;
        };
        places.push(place);
        strains.push(strain);
        shown.push(opacity);
        index += 1;
    }
    seat(draft, &places, &strains, &shown);
}

/// 🔪️ Stretches (pairs of ends) without the interval `(low, high)`.
fn carve(stretches: &[[f64; 2]], low: f64, high: f64) -> Vec<[f64; 2]> {
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
fn crowd_on(draft: &Draft<'_>, perch: &Perch) -> usize {
    let mut crowd = 0;
    for body in &draft.stage.actors {
        if body.perch.as_deref() == Some(perch.surface.as_str()) && body.x >= perch.x0 && body.x <= perch.x1 {
            crowd += 1;
        }
    }
    crowd
}

/// 🛏️ Where a species could arrive: per perch that carries it the stretches it can stand on a comfortable gap away from the actors already there (half of both widths plus 8 px), and how many stand there. A perch without such a stretch is full.
fn rooms_for(draft: &Draft<'_>, kind: &Species) -> Vec<Room> {
    let mut rooms = Vec::new();
    let half = kind.size.width / 2.0;
    for (at, perch) in draft.stage.perches.iter().enumerate() {
        if perch.x1 - half < perch.x0 + half {
            continue;
        }
        let mut stretches = vec![[perch.x0 + half, perch.x1 - half]];
        for (other, neighbour) in draft.stage.actors.iter().enumerate() {
            if neighbour.perch.as_deref() != Some(perch.surface.as_str()) {
                continue;
            }
            let gap = shoulders(draft.kinds[other], kind) + COMFORT_GAP;
            stretches = carve(&stretches, neighbour.x - gap, neighbour.x + gap);
        }
        if stretches.is_empty() {
            continue;
        }
        let mut span = 0.0;
        for &[x0, x1] in &stretches {
            span += x1 - x0;
        }
        rooms.push(Room { perch: at, stretches, span, crowd: crowd_on(draft, perch) });
    }
    rooms
}

/// 🏘️ The rooms a newcomer chooses among, so that a company spreads out over what the stage offers: those that hold the fewest actors, and of these the ones on the ground — the lowest perches of the stage — only when no higher one is as empty. Pets stand on things before they stand beneath them.
fn quarters(draft: &Draft<'_>, rooms: Vec<Room>) -> Vec<Room> {
    let mut fewest = usize::MAX;
    for room in &rooms {
        if room.crowd < fewest {
            fewest = room.crowd;
        }
    }
    let mut ground = f64::NEG_INFINITY;
    for perch in &draft.stage.perches {
        if perch.y > ground {
            ground = perch.y;
        }
    }
    let raised = |room: &Room| draft.stage.perches[room.perch].y < ground;
    let mut emptiest: Vec<Room> = rooms.into_iter().filter(|room| room.crowd == fewest).collect();
    if emptiest.iter().any(raised) {
        emptiest.retain(raised);
    }
    emptiest
}

/// 🌟️ A wanted species arrives: one stage draw picks one of its [`quarters`] (uniformly), a place on it a comfortable gap away from everybody there and the way it faces; one actor draw its first idle dwell, idle clip and blink. It fades in (on a still stage it is simply there). While no perch has room it waits off stage: nothing is drawn, and it is tried again with every survey, every summons and every whole second.
fn arrive<'a>(draft: &mut Draft<'a>, kind: &'a Species, stream: u32) {
    let rooms = quarters(draft, rooms_for(draft, kind));
    if rooms.is_empty() {
        return;
    }
    let words = random_words(&stage_key(draft), 3);
    let chosen = (unit_of(words[0]) * rooms.len() as f64).floor();
    let room = &rooms[if chosen < rooms.len() as f64 { chosen as usize } else { rooms.len() - 1 }];
    let mut along = unit_of(words[1]) * room.span;
    let mut x = room.stretches[0][0];
    for &[x0, x1] in &room.stretches {
        let length = x1 - x0;
        x = x0 + smaller(along, length);
        if along <= length {
            break;
        }
        along -= length;
    }
    let now = draft.stage.tick;
    let counter = now as u32;
    let draws = random_words(&[draft.stage.seed, stream, counter], 3);
    let perch = &draft.stage.perches[room.perch];
    let mode = draft.stage.mode;
    let body = Actor {
        species: kind.id.clone(),
        perch: Some(perch.surface.clone()),
        x,
        y: perch.y - hover_of(kind),
        vx: 0.0,
        vy: 0.0,
        facing: if unit_of(words[2]) < 0.5 { Facing::Right } else { Facing::Left },
        faced: now,
        activity: Activity::Idle,
        since: now,
        until: now + dwell_of(Activity::Idle, mode, unit_of(draws[0])),
        goal: x,
        partner: None,
        clip: clip_at(kind, Activity::Idle, unit_of(draws[1])),
        gaze: CENTRED,
        blink: blink_at(now, unit_of(draws[2])),
        mood: mood_of(Activity::Idle),
        needs: needs_of(kind.temperament),
        opacity: if mode == PetMode::Still { 1.0 } else { 0.0 },
        leaving: false,
        draws: counter.wrapping_add(1),
    };
    let at = draft.streams.iter().take_while(|&&earlier| earlier < stream).count();
    draft.stage.actors.insert(at, body);
    draft.kinds.insert(at, kind);
    draft.streams.insert(at, stream);
}

/// 📣️ Every wanted species that is not on stage arrives, in `menagerie.species` order — as long as the company on stage, leavers included, is smaller than the wanted one: a newcomer waits until whoever it replaces is gone, so a stage never holds more actors than were summoned.
fn spawn<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>) {
    if draft.stage.perches.is_empty() {
        return;
    }
    for (stream, kind) in menagerie.species.iter().enumerate() {
        if draft.stage.actors.len() >= draft.stage.wanted.len() {
            break;
        }
        if draft.stage.wanted.contains(&kind.id) && index_of(&draft.stage.actors, &kind.id).is_none() {
            arrive(draft, kind, stream as u32);
        }
    }
}

/// 🌬️ The company spreads out over new ground (a survey brought a surface that was not there before). Every perch that holds more than one actor gives up all but the first of them (in `menagerie.species` order), as far as perches that hold nobody can carry them — each such perch takes one: whoever stands idle by itself on the shared perch leaves (on a still stage it is gone at once) and, still wanted, arrives anew where nobody stands. So a company that gathered on the only edge a screen offered — the footer line under an introduction — does not stay parked there when the next screen brings cards. A time of concentration leaves everybody where they are.
fn spread(draft: &mut Draft<'_>) {
    if draft.stage.quiet {
        return;
    }
    let now = draft.stage.tick;
    let mut vacant: Vec<usize> = Vec::new();
    for (at, perch) in draft.stage.perches.iter().enumerate() {
        if crowd_on(draft, perch) == 0 {
            vacant.push(at);
        }
    }
    let mut gone: Vec<usize> = Vec::new();
    for shared in 0..draft.stage.perches.len() {
        let mut first = true;
        let mut index = 0;
        while index < draft.stage.actors.len() && !vacant.is_empty() {
            let at = index;
            index += 1;
            let perch = &draft.stage.perches[shared];
            let body = &draft.stage.actors[at];
            if body.leaving || body.perch.as_deref() != Some(perch.surface.as_str()) || body.x < perch.x0 || body.x > perch.x1 {
                continue;
            }
            if first {
                first = false;
                continue;
            }
            if body.activity != Activity::Idle || body.partner.is_some() {
                continue;
            }
            let width = draft.kinds[at].size.width;
            let Some(home) = vacant.iter().position(|&free| draft.stage.perches[free].x1 - draft.stage.perches[free].x0 >= width) else {
                continue;
            };
            vacant.remove(home);
            if draft.stage.mode == PetMode::Still {
                gone.push(at);
            } else {
                leave(draft, at, now);
            }
        }
    }
    gone.sort_unstable_by(|one, other| other.cmp(one));
    for index in gone {
        if index < draft.stage.actors.len() {
            remove(draft, index);
        }
    }
}

/// 🗺️ The stage was measured anew: perches are cut again, grounded actors ride their surfaces, fall or — when the survey brought new ground — are gone with ground that vanished, and whoever waits for a perch arrives; over new ground the company then spreads out ([`spread`]), and on a still stage whoever it moved arrives at once.
fn survey<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, event: &Surveyed) {
    let before = std::mem::replace(&mut draft.stage.surfaces, event.surfaces.clone());
    draft.stage.width = event.width;
    draft.stage.height = event.height;
    draft.stage.keepouts.clone_from(&event.keepouts);
    let uprooted = widened(draft, &before);
    measure(menagerie, draft);
    ride(draft, Some(&before), uprooted);
    spawn(menagerie, draft);
    if !uprooted {
        return;
    }
    spread(draft);
    spawn(menagerie, draft);
}

/// 👋️ An actor starts to leave: it lets go of its partner and walks to the nearer end of its perch when that is within three of its widths and nobody stands in the way, then fades; farther away, hemmed in or in the air, it fades where it is.
fn leave(draft: &mut Draft<'_>, index: usize, now: Ticks) {
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

/// 🎟️ The species that belong on stage changed: whoever is no longer wanted leaves (on a still stage it is gone at once), a leaver that is wanted again stays when it still stands on a perch (one that was crowded out goes on fading and arrives anew), perches are cut for the new company and the newly wanted arrive.
fn summon<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, wanted: &[Slug]) {
    let now = draft.stage.tick;
    draft.stage.wanted = wanted.to_vec();
    let mut index = 0;
    while index < draft.stage.actors.len() {
        let body = &mut draft.stage.actors[index];
        let stays = wanted.contains(&body.species);
        if stays && body.leaving && body.perch.is_some() {
            body.leaving = false;
            settle(draft, index, now);
        }
        if !stays && !draft.stage.actors[index].leaving {
            if draft.stage.mode == PetMode::Still {
                remove(draft, index);
                continue;
            }
            leave(draft, index, now);
        }
        index += 1;
    }
    measure(menagerie, draft);
    ride(draft, None, false);
    spawn(menagerie, draft);
}

/// 🧊️ The stage turns still: leavers and whoever is in the air are gone (the wanted among them arrive anew at once, on a perch with room), and everyone else is idle at rest, whole, without a partner, with centred pupils.
fn freeze(draft: &mut Draft<'_>, now: Ticks) {
    let mut index = 0;
    while index < draft.stage.actors.len() {
        if draft.stage.actors[index].leaving || draft.stage.actors[index].perch.is_none() {
            remove(draft, index);
            continue;
        }
        shift(draft, index, Activity::Idle, now);
        let body = &mut draft.stage.actors[index];
        body.faced = now;
        body.until = now;
        body.clip = None;
        body.partner = None;
        body.vx = 0.0;
        body.vy = 0.0;
        body.goal = body.x;
        body.gaze = CENTRED;
        body.mood = mood_of(Activity::Idle);
        body.opacity = 1.0;
        index += 1;
    }
}

/// 🎚️ The liveliness changed. Still freezes the stage; leaving still gives every actor a fresh idle dwell, idle clip and blink; a mode that allows fewer movers lets the walkers beyond its limit come to rest (hops in flight count first).
fn tune<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, mode: PetMode) {
    if mode == draft.stage.mode {
        return;
    }
    let before = draft.stage.mode;
    let now = draft.stage.tick;
    draft.stage.mode = mode;
    if mode == PetMode::Still {
        freeze(draft, now);
        spawn(menagerie, draft);
        return;
    }
    if before == PetMode::Still {
        for index in 0..draft.stage.actors.len() {
            let words = random_words(&actor_key(draft, index), 3);
            let clip = clip_at(draft.kinds[index], Activity::Idle, unit_of(words[1]));
            let body = &mut draft.stage.actors[index];
            body.since = now;
            body.until = now + dwell_of(Activity::Idle, mode, unit_of(words[0]));
            body.clip = clip;
            body.blink = blink_at(now, unit_of(words[2]));
        }
        return;
    }
    let mut movers = draft.stage.actors.iter().filter(|body| !body.leaving && body.activity == Activity::Hop).count();
    for index in 0..draft.stage.actors.len() {
        let body = &draft.stage.actors[index];
        if body.leaving || body.activity != Activity::Walk {
            continue;
        }
        movers += 1;
        if movers <= MODE_LIMITS[mode].movers {
            continue;
        }
        release(draft, index, now);
        settle(draft, index, now);
    }
}

/// 👆️ Someone tapped the stage: the nearest grounded actor within 1.2 × its height of the tap (measured from the middle of its body) cheers up and, unless it is busy with its partner, greets towards the tap ([`hail`]); a sulker is reconciled first. A still stage ignores it, and so does a time of concentration: hushed actors rest.
fn poke(draft: &mut Draft<'_>, x: f64, y: f64) {
    if draft.stage.mode == PetMode::Still || draft.stage.quiet {
        return;
    }
    let now = draft.stage.tick;
    let mut nearest = None;
    let mut least = f64::INFINITY;
    for (index, body) in draft.stage.actors.iter().enumerate() {
        if body.leaving || body.perch.is_none() {
            continue;
        }
        let height = draft.kinds[index].size.height;
        let dx = x - body.x;
        let dy = y - (body.y - height / 2.0);
        let distance = dx * dx + dy * dy;
        let reach = POKE_REACH * height;
        if distance > reach * reach || distance >= least {
            continue;
        }
        nearest = Some(index);
        least = distance;
    }
    let Some(nearest) = nearest else {
        return;
    };
    let body = &mut draft.stage.actors[nearest];
    body.mood = smaller(1.0, body.mood + POKE_CHEER);
    if body.partner.is_some() && body.activity != Activity::Sulk {
        return;
    }
    if body.activity == Activity::Sulk {
        reconcile(draft, nearest, now);
    }
    hail(draft, nearest, x, now);
}

/// 📨️ One event folded into the draft.
fn apply<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, event: &StageEvent) {
    match event {
        StageEvent::Ticked(event) => pass(menagerie, draft, event.ticks),
        StageEvent::Pointed(event) => {
            draft.stage.pointer = Some(Point { x: event.x, y: event.y });
            draft.stage.pointed = draft.stage.tick;
        }
        StageEvent::Unpointed(_) => draft.stage.pointer = None,
        StageEvent::Glanced(event) => draft.stage.glances.clone_from(&event.points),
        StageEvent::Surveyed(event) => survey(menagerie, draft, event),
        StageEvent::Summoned(event) => summon(menagerie, draft, &event.species),
        StageEvent::Tuned(event) => tune(menagerie, draft, event.mode),
        StageEvent::Hushed(event) => draft.stage.quiet = event.quiet,
        StageEvent::Poked(event) => poke(draft, event.x, event.y),
    }
}

/// 🧵️ The stage after the events, folded in order. The stage is taken by value and answered: without events it comes back untouched, with events its actors come back in `menagerie.species` order.
pub fn advance(menagerie: &Menagerie, stage: Stage, events: &[StageEvent]) -> Stage {
    if events.is_empty() {
        return stage;
    }
    let mut draft = draft_of(menagerie, stage);
    for event in events {
        apply(menagerie, &mut draft, event);
    }
    sealed(draft)
}
//#endregion 🔖️Events

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

/// 🌾️ A pose in which the head leans after the eyes: the bone that carries the first eye of the species turns by 5° and shifts by 1.5 px per unit of the gaze `across` (in the actor's own orientation: ahead is positive) and sinks by 1 px per unit of the gaze `down`. The gaze is a spring, so the lean eases with it. A species without eyes does not lean.
fn leant(kind: &Species, mut pose: Pose, across: f64, down: f64) -> Pose {
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
fn squeeze_of(actor: &Actor, tick: Ticks) -> f64 {
    if tick < actor.faced {
        2.0 * smoothstep((TURN_TICKS - (actor.faced - tick)) as f64 / TURN_TICKS as f64) - 1.0
    } else {
        1.0
    }
}

/// 🎼️ How many ticks per second the motion of one actor needs at a tick: 64 while it fades (in, out, or see-through under the pointer and back), turns round, walks, flies, lands, plays a clip that does not loop or blends a clip in or out; 32 while a loop, a blink, its pupils or its mood move; 16 while it only sleeps; 0 when nothing of it moves.
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
    let restless = actor.mood != mood_of(activity) || !gaze_rests(stage, actors, kinds, index, tick + 1);
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

/// 🎥️ What a render target draws for a stage: the actors back to front (by `y`, then species id), each with its feet, facing, activity, opacity, one matrix per bone (`solve_rig` of [`pose_of`], leaning after the gaze ([`leant`]) outside a time of concentration, squeezed across by [`squeeze_of`] while the actor turns round), its eyes (pupil offset = gaze × `pupil_reach`: the pupil travels up to the outline of the white; mirrored with the actor as drawn; lid from the blink, shut asleep) and its mood; the rate the motion needs and, at rate 0, the tick of the next scheduled change.
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
            ActorFrame { species: actor.species.clone(), x: actor.x, y: actor.y, facing: actor.facing, activity: actor.activity, opacity: if still && presence < actor.opacity { presence } else { actor.opacity }, bones, eyes, mood: actor.mood }
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
    Frame { tick, actors: frames, rate, wake }
}
//#endregion 🔖️Frame

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
