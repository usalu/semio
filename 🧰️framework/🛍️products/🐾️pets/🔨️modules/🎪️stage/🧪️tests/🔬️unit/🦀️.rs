//! 🎪️ Unit tests of the stage as a whole: the committed traces of the stage-trace case replayed digest for digest (the TypeScript twin recorded them, so every checkpoint is a proof of the same bits), the empty stage, the fold without events, the events of the learner's hand, what an actor arrives with and stands on, what a frame shows of it, the ways time can be cut, and the arithmetic the twin is allowed to use — in every part of the stage. What a part owns is tested in the suite of that part; all of them borrow the kit of this one: the troupe, a meadow to stand on, the events and a pet at rest.
//!
//! The digest is the one the adapters of the case define: FNV-1a (32 bit) over the little-endian bytes of every
//! number of every frame (an activity counts as its index in `ACTIVITIES`), running on from frame to frame.
//!
//! @see ../../🦀️.rs — the façade under test
//! @see ../../../../🧪️tests/🎪️stage-trace/🦀️.rs — the subject adapter that also checks the laws of the stage
//! @see ../../../../🧫️fixtures/🎪️stage-trace/🔣️.json

use super::*;
use crate::schema::tests::{assert_same, entries, fixture, json, typed};
use crate::feeling::{at_rest, settled, spirits_of};
use crate::gesture::{no_hover, COLD};
use crate::schema::{
    Actor, Facing, Fixture, Footing, Frame, Hushed, Needs, Over, PetMode, Permitted, Point, Pointed, Pointer, Pressed, Rate, Reclaimed, Rect, Released, Repertoire, Scrolled, Stirred, Summoned, Surface, Surveyed, Ticked, Ticks,
    Tuned, Wall,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const SOURCES: [(&str, &str); 11] = [
    ("stage", include_str!("../../🦀️.rs")),
    ("draft", include_str!("../../../📝️draft/🦀️.rs")),
    ("spacing", include_str!("../../../📏️spacing/🦀️.rs")),
    ("schedule", include_str!("../../../🗓️schedule/🦀️.rs")),
    ("attention", include_str!("../../../👀️attention/🦀️.rs")),
    ("locomotion", include_str!("../../../🚶️locomotion/🦀️.rs")),
    ("sociability", include_str!("../../../💞️sociability/🦀️.rs")),
    ("choice", include_str!("../../../🎯️choice/🦀️.rs")),
    ("population", include_str!("../../../👥️population/🦀️.rs")),
    ("clock", include_str!("../../../🕰️clock/🦀️.rs")),
    ("projection", include_str!("../../../🎥️projection/🦀️.rs")),
];
const FORBIDDEN: [&str; 21] =
    [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", ".max(", ".min(", "f64::max", "f64::min", "println!", "eprintln!", "dbg!", "static ", "thread_local!", "HashMap", "BTreeMap"];
pub(crate) const FAR: Ticks = 100_000_000;
pub(crate) const LEVEL: f64 = 400.0 - 30.0 * 0.6;
const FNV_OFFSET_BASIS: u32 = 2_166_136_261;
const FNV_PRIME: u32 = 16_777_619;
const CHUNKS: [Ticks; 8] = [1, 7, 64, 3, 500, 2, 19, 1000];
pub(crate) const TROUPE: [&str; 5] = ["mossy", "sparky", "thorny", "pebble", "misty"];

struct Script {
    id: String,
    seed: u32,
    ticks: Ticks,
    every: Ticks,
    events: BTreeMap<Ticks, Vec<StageEvent>>,
    expected: Value,
}

fn committed() -> (Menagerie, Vec<Script>) {
    let vectors = fixture("stage-trace");
    let scripts = entries(&vectors["scripts"])
        .iter()
        .map(|script| {
            let mut events: BTreeMap<Ticks, Vec<StageEvent>> = BTreeMap::new();
            for step in entries(&script["steps"]) {
                events.entry(typed(&step["at"])).or_default().extend(typed::<Vec<StageEvent>>(&step["events"]));
            }
            Script { id: typed(&script["id"]), seed: typed(&script["seed"]), ticks: typed(&script["ticks"]), every: typed(&script["every"]), events, expected: script["expected"].clone() }
        })
        .collect();
    (typed(&vectors["menagerie"]), scripts)
}

pub(crate) fn troupe() -> Menagerie {
    typed(&fixture("stage-trace")["menagerie"])
}

fn fold(hash: u32, value: f64) -> u32 {
    value.to_le_bytes().into_iter().fold(hash, |folded, byte| (folded ^ u32::from(byte)).wrapping_mul(FNV_PRIME))
}

fn fold_frame(hash: u32, menagerie: &Menagerie, frame: &Frame) -> u32 {
    let mut folded = [frame.tick as f64, f64::from(u8::from(frame.rate)), frame.wake.map_or(-1.0, |wake| wake as f64), frame.actors.len() as f64].into_iter().fold(hash, fold);
    for actor in &frame.actors {
        folded = fold(folded, menagerie.species.iter().position(|species| species.id == actor.species).map_or(-1.0, |index| index as f64));
        folded = [actor.x, actor.y, actor.facing.sign(), actor.activity as usize as f64, actor.opacity].into_iter().fold(folded, fold);
        folded = actor.bones.iter().copied().fold(folded, fold);
        folded = actor.eyes.iter().flat_map(|eye| [eye.x, eye.y, eye.lid]).fold(folded, fold);
        folded = fold(folded, actor.spirits);
    }
    folded
}

pub(crate) fn ticked(ticks: u64) -> [StageEvent; 1] {
    [StageEvent::Ticked(Ticked { ticks })]
}

fn replayed(menagerie: &Menagerie, script: &Script, seed: u32) -> (Value, Stage) {
    let mut stage = open_stage(seed);
    let mut hash = FNV_OFFSET_BASIS;
    let mut checkpoints = Vec::new();
    for tick in 1..=script.ticks {
        stage = advance(menagerie, stage, script.events.get(&(tick - 1)).map_or(&[], Vec::as_slice));
        stage = advance(menagerie, stage, &ticked(1));
        hash = fold_frame(hash, menagerie, &frame_of(menagerie, &stage));
        if tick % script.every != 0 && tick != script.ticks {
            continue;
        }
        let actors: Vec<Value> =
            stage.actors.iter().map(|actor| json!({ "species": actor.species, "perch": actor.perch, "x": actor.x, "y": actor.y, "activity": actor.activity, "partner": actor.partner, "leaving": actor.leaving, "opacity": actor.opacity })).collect();
        checkpoints.push(json!({ "tick": tick, "digest": hash, "mode": stage.mode, "actors": actors }));
    }
    (json!({ "frames": script.ticks, "digest": hash, "checkpoints": checkpoints }), stage)
}

fn chunked(menagerie: &Menagerie, script: &Script) -> Stage {
    let mut stage = open_stage(script.seed);
    let mut tick = 0;
    let mut turn = 0;
    while tick < script.ticks {
        stage = advance(menagerie, stage, script.events.get(&tick).map_or(&[], Vec::as_slice));
        let next = script.events.range(tick + 1..script.ticks).next().map_or(script.ticks, |(&at, _)| at);
        let ticks = Ticks::min(CHUNKS[turn % CHUNKS.len()], next - tick);
        stage = advance(menagerie, stage, &ticked(ticks.unsigned_abs()));
        tick += ticks;
        turn += 1;
    }
    stage
}

fn conforms(menagerie: &Menagerie, script: &Script) -> Stage {
    let (trace, stage) = replayed(menagerie, script, script.seed);
    for member in ["frames", "digest", "checkpoints"] {
        assert_same(&format!("stage-trace/{}/{member}", script.id), &trace[member], &script.expected[member]);
    }
    stage
}

pub(crate) fn surveyed(width: f64, height: f64, surfaces: &[(&str, f64, f64, f64)], keepouts: &[Rect]) -> StageEvent {
    StageEvent::Surveyed(Surveyed { width, height, surfaces: surfaces.iter().map(|&(id, x0, x1, y)| Surface { id: id.to_string(), x0, x1, y }).collect(), keepouts: keepouts.to_vec(), walls: Vec::new(), fixtures: Vec::new() })
}

pub(crate) fn clicked(x: f64, y: f64) -> [StageEvent; 2] {
    [StageEvent::Pressed(Pressed { x, y, pointer: Pointer::Mouse }), StageEvent::Released(Released { x, y })]
}

pub(crate) fn summoned(species: &[&str]) -> StageEvent {
    StageEvent::Summoned(Summoned { species: species.iter().map(|&id| id.to_string()).collect() })
}

pub(crate) fn meadow(menagerie: &Menagerie, seed: u32, species: &[&str]) -> Stage {
    advance(menagerie, open_stage(seed), &[surveyed(1280.0, 720.0, &[("card", 200.0, 900.0, 400.0), ("floor", 0.0, 1280.0, 720.0)], &[]), summoned(species)])
}

pub(crate) fn actor_of<'s>(stage: &'s Stage, species: &str) -> &'s Actor {
    stage.actors.iter().find(|actor| actor.species == species).unwrap_or_else(|| panic!("{species} is not on stage"))
}

pub(crate) fn run(menagerie: &Menagerie, mut stage: Stage, ticks: Ticks, mut watch: impl FnMut(&Stage)) -> Stage {
    for _ in 0..ticks {
        stage = advance(menagerie, stage, &ticked(1));
        watch(&stage);
    }
    stage
}

pub(crate) fn rested(menagerie: &Menagerie, curiosity: f64) -> Stage {
    let mut stage = meadow(menagerie, 6, &["mossy"]);
    let body = &mut stage.actors[0];
    (body.perch, body.x, body.y, body.goal, body.facing) = (Some("card".to_string()), 550.0, 400.0, 550.0, Facing::Right);
    (body.until, body.blink, body.opacity) = (FAR, FAR, 1.0);
    let mut stage = advance(menagerie, stage, &ticked(128));
    stage.actors[0].needs = Needs { energy: 1.0, sociability: 0.5, curiosity };
    stage
}

pub(crate) fn pointed(x: f64) -> StageEvent {
    StageEvent::Pointed(Pointed { x, y: LEVEL, over: Over::Free })
}

pub(crate) fn plain(menagerie: &Menagerie) -> Menagerie {
    let mut plain = menagerie.clone();
    for kind in &mut plain.species {
        kind.clips.clear();
        kind.repertoire = Repertoire::default();
    }
    plain
}

#[test]
fn a_stage_opens_empty_calm_and_at_tick_zero() {
    let stage = open_stage(7);
    let idle = json!({ "phase": "idle", "x": 0.0, "y": 0.0, "since": 0, "slop": 0.0 });
    let shaking = json!({ "live": false, "ax": 0.0, "ay": 0.0, "at": 0, "fx": 0.0, "fy": 0.0, "reached": 0, "count": 0, "mark1": 0, "mark2": 0, "mark3": 0, "rest": 0 });
    let mut expected = json!({ "seed": 7, "tick": 0, "mode": "calm", "quiet": false, "width": 0.0, "height": 0.0, "pointer": null, "pointed": 0, "over": "free", "glances": [], "surfaces": [], "keepouts": [], "walls": [], "fixtures": [], "perches": [], "pitches": [], "wanted": [], "actors": [], "rapports": [], "met": 0, "draws": 0 });
    let rest = json!({ "play": false, "mischief": false, "stirred": 0, "scrolled": 0, "press": idle, "touched": null, "shaking": shaking, "trail": [], "coolings": [], "pledges": [], "ladders": [], "lift": null, "rested": 0, "poofs": 0, "puffs": [], "claims": [], "courses": [], "trips": [], "origin": null });
    if let (Some(all), Some(more)) = (expected.as_object_mut(), rest.as_object()) {
        all.extend(more.clone());
    }
    assert_eq!(json(&stage), expected);
    let frame = frame_of(&troupe(), &stage);
    assert_eq!((frame.tick, frame.actors.len(), frame.rate, frame.wake), (0, 0, Rate::Rest, None));
    assert_eq!((frame.ladders.len(), frame.particles.len(), frame.lifts.len(), frame.held), (0, 0, 0, None));
    let later = advance(&troupe(), stage, &ticked(1000));
    assert_eq!((later.tick, later.draws), (1000, 0));
}

#[test]
fn a_reclaimed_fixture_changes_nothing_while_none_is_lifted_and_permissions_inputs_and_scrolls_are_kept() {
    let menagerie = troupe();
    let stage = advance(&menagerie, meadow(&menagerie, 9, &TROUPE), &ticked(64));
    assert_eq!(advance(&menagerie, stage.clone(), &[StageEvent::Reclaimed(Reclaimed { fixture: "card".to_string() })]), stage);
    let kept = advance(&menagerie, stage.clone(), &[StageEvent::Permitted(Permitted { play: true, mischief: false }), StageEvent::Stirred(Stirred {})]);
    assert_eq!((kept.play, kept.mischief, kept.stirred, kept.scrolled), (true, false, stage.tick, 0));
    let later = advance(&menagerie, advance(&menagerie, kept.clone(), &ticked(5)), &[StageEvent::Scrolled(Scrolled {}), StageEvent::Permitted(Permitted { play: false, mischief: true })]);
    assert_eq!((later.play, later.mischief, later.stirred, later.scrolled), (false, true, stage.tick, stage.tick + 5));
    assert_eq!(later.actors, advance(&menagerie, kept, &ticked(5)).actors);
    let walls = vec![Wall { id: "card-left".to_string(), surface: "card".to_string(), side: Facing::Left, x: 200.0, y0: 400.0, y1: 520.0 }];
    let fixtures = vec![Fixture { id: "task-1".to_string(), key: "quiz/task".to_string(), x: 320.0, y: 420.0, width: 200.0, height: 30.0 }];
    let StageEvent::Surveyed(plain) = surveyed(1280.0, 720.0, &[("card", 200.0, 900.0, 400.0), ("floor", 0.0, 1280.0, 720.0)], &[]) else {
        panic!("a survey is surveyed");
    };
    let walled = advance(&menagerie, stage.clone(), &[StageEvent::Surveyed(Surveyed { walls: walls.clone(), fixtures: fixtures.clone(), ..plain.clone() })]);
    assert_eq!((&walled.walls, &walled.fixtures), (&walls, &fixtures));
    assert_eq!(Stage { walls: Vec::new(), fixtures: Vec::new(), ..walled }, advance(&menagerie, stage, &[StageEvent::Surveyed(plain)]));
}

#[test]
fn an_actor_arrives_at_rest_on_its_perch_and_stands_on_a_perch_exactly_while_it_has_one() {
    let menagerie = troupe();
    let stage = meadow(&menagerie, 21, &TROUPE);
    assert!(!stage.actors.is_empty());
    for actor in &stage.actors {
        let kind = menagerie.species.iter().find(|kind| kind.id == actor.species).unwrap_or_else(|| panic!("{} is no species", actor.species));
        assert_eq!((actor.footing, actor.feeling, actor.state.as_str(), actor.state_since, actor.trick.as_deref()), (Footing::Perch, at_rest(kind.mood, stage.tick), kind.states[0].id.as_str(), stage.tick, None));
        assert_eq!((actor.warmth, actor.hover, actor.hang, actor.chute, actor.rope.as_ref(), actor.emitters.len()), (COLD, no_hover(0), None, None, None, 0));
    }
    let mut airborne = 0;
    let mut stage = advance(&menagerie, meadow(&menagerie, 13, &TROUPE), &[StageEvent::Tuned(Tuned { mode: PetMode::Lively })]);
    let floor = [("floor", 0.0, 1280.0, 720.0)];
    let world = [("card", 200.0, 900.0, 400.0), ("floor", 0.0, 1280.0, 720.0)];
    for second in 0..90 {
        let events = match second {
            20 => vec![surveyed(1280.0, 720.0, &floor, &[])],
            30 => vec![surveyed(1280.0, 720.0, &world, &[])],
            70 => vec![StageEvent::Tuned(Tuned { mode: PetMode::Still })],
            72 => vec![StageEvent::Tuned(Tuned { mode: PetMode::Lively })],
            _ => Vec::new(),
        };
        stage = run(&menagerie, advance(&menagerie, stage, &events), 64, |after| {
            for actor in &after.actors {
                assert_eq!(actor.footing == Footing::Perch, actor.perch.is_some(), "{} at {} by {:?}", actor.species, after.tick, actor.footing);
                airborne += usize::from(actor.perch.is_none());
            }
        });
    }
    assert!(airborne > 0);
}

#[test]
fn a_frame_draws_every_actor_upright_about_its_grip_with_its_spirits_and_its_size_box() {
    let menagerie = troupe();
    let mut stage = advance(&menagerie, meadow(&menagerie, 17, &TROUPE), &[StageEvent::Tuned(Tuned { mode: PetMode::Lively }), StageEvent::Ticked(Ticked { ticks: 1280 })]);
    for _ in 0..400 {
        stage = advance(&menagerie, stage, &ticked(1));
        let frame = frame_of(&menagerie, &stage);
        assert_eq!((frame.ladders.len(), frame.particles.len(), frame.lifts.len(), frame.held.as_deref()), (0, 0, 0, None));
        for drawn in &frame.actors {
            let actor = actor_of(&stage, &drawn.species);
            let kind = menagerie.species.iter().find(|kind| kind.id == drawn.species).unwrap_or_else(|| panic!("{} is no species", drawn.species));
            let feeling = settled(actor.feeling, kind.mood, stage.tick);
            assert_eq!((drawn.footing, drawn.state.as_str(), drawn.mood, drawn.intensity, drawn.spirits), (actor.footing, actor.state.as_str(), feeling.mood, feeling.intensity, spirits_of(feeling)));
            assert_eq!((drawn.tilt, drawn.pivot, drawn.tools.len()), (0.0, Point { x: 0.0, y: -kind.grip }, 0));
            assert_eq!(drawn.body, Rect { x: actor.x - kind.size.width / 2.0, y: actor.y - kind.size.height, width: kind.size.width, height: kind.size.height });
        }
    }
}

#[test]
fn advancing_without_events_answers_the_stage_untouched() {
    let menagerie = troupe();
    let mut stage = meadow(&menagerie, 9, &TROUPE);
    stage.actors.reverse();
    assert_eq!(advance(&menagerie, stage.clone(), &[]), stage);
    let sorted = advance(&menagerie, stage, &[StageEvent::Hushed(Hushed { quiet: false })]);
    assert_eq!(sorted.actors.iter().map(|actor| actor.species.as_str()).collect::<Vec<_>>(), TROUPE);
}

#[test]
fn the_same_seed_and_events_yield_the_same_stage_and_another_seed_another_story() {
    let (menagerie, scripts) = committed();
    let script = scripts.iter().find(|script| script.id == "narrow-stage").unwrap_or_else(|| panic!("no narrow-stage script"));
    let short = Script { id: script.id.clone(), seed: script.seed, ticks: 1280, every: 640, events: script.events.clone(), expected: Value::Null };
    let (first, stage) = replayed(&menagerie, &short, short.seed);
    let (again, same) = replayed(&menagerie, &short, short.seed);
    assert_eq!((&again, &same), (&first, &stage));
    let (other, _) = replayed(&menagerie, &short, short.seed.wrapping_add(1));
    assert_ne!(other["digest"], first["digest"]);
    assert_eq!(chunked(&menagerie, &short), stage);
}

#[test]
fn the_calm_home_replays_into_its_committed_trace() {
    let (menagerie, scripts) = committed();
    conforms(&menagerie, &scripts[0]);
}

#[test]
fn the_arithmetic_is_what_the_typescript_twin_can_reproduce() {
    for (part, source) in SOURCES {
        for forbidden in FORBIDDEN {
            assert!(!source.contains(forbidden), "{part}: {forbidden}");
        }
    }
}

mod quick {
    use super::*;

    #[test]
    fn every_script_replays_into_its_committed_trace_however_time_is_cut() {
        let (menagerie, scripts) = committed();
        assert!(scripts.len() >= 8);
        for script in &scripts {
            let stage = conforms(&menagerie, script);
            assert_eq!(chunked(&menagerie, script), stage, "{}", script.id);
        }
    }
}
