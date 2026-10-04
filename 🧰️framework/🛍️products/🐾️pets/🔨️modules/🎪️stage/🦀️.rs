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
//!   decision `random_pick` on word 0, then `[·, dwell, clip, goal or target]`; blink `[gap, double]`; perking up
//!   `[dwell, clip]`; sulk `[dwell, clip]`;
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
//! PARTS. This file is the façade of the stage — `open_stage`, `advance` and `frame_of` are all the crate exports of
//! it — over ten modules that share the working copy of a stage (the draft): the gaps between actors, the schedule of
//! the stage, attention, locomotion, sociability, choice, population, the clock and the projection. The order above is
//! the order of the clock.
//!
//! @see ../📝️draft/🦀️.rs — the working copy of a stage, its lookups and the smallest changes of an actor
//! @see ../📏️spacing/🦀️.rs — the gaps between grounded actors
//! @see ../🗓️schedule/🦀️.rs — the whole seconds on which a pair may be drawn or somebody may arrive
//! @see ../👀️attention/🦀️.rs — eyes, head and facing: presence, turning round, gaze, blinks, mood, perking up
//! @see ../🚶️locomotion/🦀️.rs — walking, hopping, gliding, falling, landing, leaving
//! @see ../💞️sociability/🦀️.rs — pairing, encounters, sulks, rapport
//! @see ../🎯️choice/🦀️.rs — what an actor does next when its time is up
//! @see ../👥️population/🦀️.rs — surveys, arrivals, spreading out, summons, liveliness
//! @see ../🕰️clock/🦀️.rs — one tick in the normative order, the lulls, time passing
//! @see ../🎥️projection/🦀️.rs — poses, the rate and `frame_of`
//! @see ../🧠️behavior/🦀️.rs — limits, weights, dwells, encounters, needs, rapport
//! @see ../🏞️terrain/🦀️.rs — perches, strides, falls, hops
//! @see ../🎞️animation/🦀️.rs — clips, springs, blinks
//! @see ../🦴️rig/🦀️.rs — poses and matrices
//! @see ../🎲️randomness/🦀️.rs — counter-based draws
//! @see ../../🧬️schema/🦀️.rs — `Stage`, `Actor`, `StageEvent`, `Frame`
//! @see ../🎪️stage/🟦️.ts — the TypeScript twin

use crate::clock::pass;
use crate::draft::{draft_of, sealed, Draft};
use crate::gesture::{no_shaking, IDLE};
use crate::population::{summon, survey, tune};
use crate::schema::{Menagerie, Over, PetMode, Point, Stage, StageEvent};

pub use crate::projection::frame_of;

//#region 🔖️Stage
/// 🏟️ An empty stage for a seed: tick 0, mode `calm`, not quiet, nothing surveyed, nobody summoned, nothing permitted (the shell's `permitted` grants play and mischief), no press, no ladder, no lift.
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
        over: Over::Free,
        glances: Vec::new(),
        surfaces: Vec::new(),
        keepouts: Vec::new(),
        walls: Vec::new(),
        fixtures: Vec::new(),
        perches: Vec::new(),
        pitches: Vec::new(),
        wanted: Vec::new(),
        actors: Vec::new(),
        rapports: Vec::new(),
        met: 0,
        draws: 0,
        play: false,
        mischief: false,
        stirred: 0,
        scrolled: 0,
        press: IDLE,
        touched: None,
        shaking: no_shaking(0),
        trail: Vec::new(),
        coolings: Vec::new(),
        pledges: Vec::new(),
        ladders: Vec::new(),
        lift: None,
        rested: 0,
        poofs: 0,
        puffs: Vec::new(),
        claims: Vec::new(),
        courses: Vec::new(),
        trips: Vec::new(),
        origin: None,
    }
}

/// 📨️ One event folded into the draft. What the learner permits, what the pointer is over and the ticks of the learner's last input and of the last scroll are kept; a press, a drag, a release, a cancellation, a reclaimed fixture and a deed change nothing yet.
fn apply<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, event: &StageEvent) {
    match event {
        StageEvent::Ticked(event) => pass(menagerie, draft, event.ticks),
        StageEvent::Pointed(event) => {
            draft.stage.pointer = Some(Point { x: event.x, y: event.y });
            draft.stage.pointed = draft.stage.tick;
            draft.stage.over = event.over;
        }
        StageEvent::Unpointed(_) => draft.stage.pointer = None,
        StageEvent::Glanced(event) => draft.stage.glances.clone_from(&event.points),
        StageEvent::Surveyed(event) => survey(menagerie, draft, event),
        StageEvent::Summoned(event) => summon(menagerie, draft, &event.species),
        StageEvent::Tuned(event) => tune(menagerie, draft, event.mode),
        StageEvent::Hushed(event) => draft.stage.quiet = event.quiet,
        StageEvent::Permitted(event) => {
            draft.stage.play = event.play;
            draft.stage.mischief = event.mischief;
        }
        StageEvent::Stirred(_) => draft.stage.stirred = draft.stage.tick,
        StageEvent::Scrolled(_) => draft.stage.scrolled = draft.stage.tick,
        StageEvent::Pressed(_) | StageEvent::Dragged(_) | StageEvent::Released(_) | StageEvent::Cancelled(_) | StageEvent::Reclaimed(_) | StageEvent::Played(_) => {}
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
//#endregion 🔖️Stage

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
