//! 🎪️ The deterministic event fold and complete frame projection shared with the TypeScript stage.
//!
//! A tick expires claims, puffs and ladders, handles held gestures, seats simultaneous scoots,
//! advances actors, updates hover, chemistry, encounters and pairing, runs pranks, and admits arrivals.
//! Motion follows guarded claimed courses and gear trips; surveys ride bodies rigidly, release invalid
//! footholds, and reseat displaced actors within complete body clearance. Independent random streams
//! preserve decision, gear, wind and mischief determinism. Quiet stages jump only across ticks at
//! which no motion, gesture, state, feeling, chemistry, prank or population horizon can change.
//! Floating expressions retain the twin's order without reassociation or fused multiply-add.
//!
//! @see ../🕰️clock/🦀️.rs — tick order and safe lulls
//! @see ../🚶️locomotion/🦀️.rs — claimed courses, gear trips and hand motion
//! @see ../👥️population/🦀️.rs — rigid survey rides and clearance seating
//! @see ../🎥️projection/🦀️.rs — poses, tools, scenery and wake horizons
//! @see ../🎪️stage/🟦️.ts — the TypeScript twin
//! @see ../../🧬️schema/🦀️.rs — owned events, stage and frame

use crate::clock::pass;
use crate::attention::{grasp, tossed, touched_at, unhovered};
use crate::sociability::{clicked, played};
use crate::choice::reclaimed;
use crate::locomotion::unlifted;
use crate::draft::{draft_of, sealed, Draft};
use crate::population::{summon, survey, tune};
use semio_framework_pets_contract::schema::{Menagerie, Over, PetMode, Point, Stage, StageEvent};
use semio_framework_pets_interaction::gesture::{no_shaking, IDLE};

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

/// 📨️ Fold pointer, hand, consent, scenery and reclamation events in their normative order.
fn apply<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, event: &StageEvent) {
    match event {
        StageEvent::Ticked(event) => pass(menagerie, draft, event.ticks),
        StageEvent::Pointed(event) => {
            draft.stage.pointer = Some(Point { x: event.x, y: event.y });
            draft.stage.pointed = draft.stage.tick;
            draft.stage.over = event.over;
        }
        StageEvent::Unpointed(_) => { draft.stage.pointer = None; unhovered(draft, draft.stage.tick); },
        StageEvent::Glanced(event) => draft.stage.glances.clone_from(&event.points),
        StageEvent::Surveyed(event) => survey(menagerie, draft, event),
        StageEvent::Summoned(event) => summon(menagerie, draft, &event.species),
        StageEvent::Tuned(event) => tune(menagerie, draft, event.mode),
        StageEvent::Hushed(event) => draft.stage.quiet = event.quiet,
        StageEvent::Permitted(event) => {
            draft.stage.play = event.play;
            draft.stage.mischief = event.mischief;
            if !event.mischief { unlifted(draft, draft.stage.tick); }
            if !event.play && draft.stage.press.phase != PressPhase::Idle {
                let touched = draft.stage.touched.clone();
                grasp(draft, Some(PressSignal::Abort), touched.as_deref(), draft.stage.tick);
                draft.stage.press = IDLE;
                draft.stage.touched = None;
            }
        }
        StageEvent::Stirred(_) => draft.stage.stirred = draft.stage.tick,
        StageEvent::Scrolled(_) => draft.stage.scrolled = draft.stage.tick,
        StageEvent::Played(event) => { if event.deed == Deed::Toss { tossed(draft, &event.species, draft.stage.tick); } else { played(draft, &event.species, event.deed); } },
        StageEvent::Pressed(event) => {
            let step = press_step(draft.stage.press, PressInput::Pressed(*event), draft.stage.tick, Guards { quiet: draft.stage.quiet, still: draft.stage.mode == PetMode::Still, ..Guards::default() });
            let touched = draft.stage.touched.clone();
            grasp(draft, step.signal, touched.as_deref(), draft.stage.tick);
            draft.stage.press = step.state;
            draft.stage.touched = if step.state.phase == PressPhase::Idle { None } else { touched_at(draft, event.x, event.y) };
        }
        StageEvent::Dragged(event) => {
            draft.stage.pointer = Some(Point { x: event.x, y: event.y });
            draft.stage.pointed = draft.stage.tick;
            pressed_step(draft, PressInput::Dragged(*event));
        }
        StageEvent::Released(event) => {
            draft.stage.pointer = Some(Point { x: event.x, y: event.y });
            draft.stage.pointed = draft.stage.tick;
            let (signal, touched) = pressed_step(draft, PressInput::Released(*event));
            if signal == Some(PressSignal::Click) { if let Some(touched) = touched { clicked(draft, &touched, event.x); } }
        }
        StageEvent::Cancelled(_) => { pressed_step(draft, PressInput::Cancelled); },
        StageEvent::Reclaimed(event) => reclaimed(draft, &event.fixture, draft.stage.tick)
    }
}

/// 🖱️ Fold a follow-up pointer event through the press machine and its held actor.
fn pressed_step(draft: &mut Draft<'_>, input: PressInput) -> (Option<PressSignal>, Option<String>) {
    let step = press_step(draft.stage.press, input, draft.stage.tick, Guards { quiet: draft.stage.quiet, still: draft.stage.mode == PetMode::Still, ..Guards::default() });
    draft.stage.press = step.state;
    let touched = draft.stage.touched.clone();
    grasp(draft, step.signal, touched.as_deref(), draft.stage.tick);
    if step.state.phase == PressPhase::Idle { draft.stage.touched = None; }
    (step.signal, touched)
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
