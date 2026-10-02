//! 🎪️ Unit tests of the stage: the committed traces of the stage-trace case replayed digest for digest (the TypeScript twin recorded them, so every checkpoint is a proof of the same bits), arrival and spreading out, stillness, pokes, turning round, attending to the pointer, the order of a frame, the ways time can be cut, and the arithmetic the twin is allowed to use.
//!
//! The digest is the one the adapters of the case define: FNV-1a (32 bit) over the little-endian bytes of every
//! number of every frame (an activity counts as its index in `ACTIVITIES`), running on from frame to frame.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧪️tests/🎪️stage-trace/🦀️.rs — the subject adapter that also checks the laws of the stage
//! @see ../../../../🧫️fixtures/🎪️stage-trace/🔣️.json

use super::*;
use crate::schema::tests::{assert_same, entries, fixture, json, typed};
use crate::schema::{Glanced, Hushed, Needs, Pointed, Poked, Rect, Repertoire, Summoned, Ticked, Tuned, Unpointed};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const SOURCE: &str = include_str!("../../🦀️.rs");
const FAR: Ticks = 100_000_000;
const LEVEL: f64 = 400.0 - 30.0 * 0.6;
const FNV_OFFSET_BASIS: u32 = 2_166_136_261;
const FNV_PRIME: u32 = 16_777_619;
const CHUNKS: [Ticks; 8] = [1, 7, 64, 3, 500, 2, 19, 1000];
const TROUPE: [&str; 5] = ["mossy", "sparky", "thorny", "pebble", "misty"];

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

fn troupe() -> Menagerie {
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
        folded = fold(folded, actor.mood);
    }
    folded
}

fn ticked(ticks: u64) -> [StageEvent; 1] {
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

fn surveyed(width: f64, height: f64, surfaces: &[(&str, f64, f64, f64)], keepouts: &[Rect]) -> StageEvent {
    StageEvent::Surveyed(Surveyed { width, height, surfaces: surfaces.iter().map(|&(id, x0, x1, y)| Surface { id: id.to_string(), x0, x1, y }).collect(), keepouts: keepouts.to_vec() })
}

fn summoned(species: &[&str]) -> StageEvent {
    StageEvent::Summoned(Summoned { species: species.iter().map(|&id| id.to_string()).collect() })
}

fn meadow(menagerie: &Menagerie, seed: u32, species: &[&str]) -> Stage {
    advance(menagerie, open_stage(seed), &[surveyed(1280.0, 720.0, &[("card", 200.0, 900.0, 400.0), ("floor", 0.0, 1280.0, 720.0)], &[]), summoned(species)])
}

fn actor_of<'s>(stage: &'s Stage, species: &str) -> &'s Actor {
    stage.actors.iter().find(|actor| actor.species == species).unwrap_or_else(|| panic!("{species} is not on stage"))
}

fn run(menagerie: &Menagerie, mut stage: Stage, ticks: Ticks, mut watch: impl FnMut(&Stage)) -> Stage {
    for _ in 0..ticks {
        stage = advance(menagerie, stage, &ticked(1));
        watch(&stage);
    }
    stage
}

fn rested(menagerie: &Menagerie, curiosity: f64) -> Stage {
    let mut stage = meadow(menagerie, 6, &["mossy"]);
    let body = &mut stage.actors[0];
    (body.perch, body.x, body.y, body.goal, body.facing) = (Some("card".to_string()), 550.0, 400.0, 550.0, Facing::Right);
    (body.until, body.blink, body.opacity) = (FAR, FAR, 1.0);
    let mut stage = advance(menagerie, stage, &ticked(128));
    stage.actors[0].needs = Needs { energy: 1.0, sociability: 0.5, curiosity };
    stage
}

fn pointed(x: f64) -> StageEvent {
    StageEvent::Pointed(Pointed { x, y: LEVEL })
}

fn plain(menagerie: &Menagerie) -> Menagerie {
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
    assert_eq!(
        json(&stage),
        json!({ "seed": 7, "tick": 0, "mode": "calm", "quiet": false, "width": 0.0, "height": 0.0, "pointer": null, "pointed": 0, "glances": [], "surfaces": [], "keepouts": [], "perches": [], "wanted": [], "actors": [], "rapports": [], "met": 0, "draws": 0 })
    );
    let frame = frame_of(&troupe(), &stage);
    assert_eq!((frame.tick, frame.actors.len(), frame.rate, frame.wake), (0, 0, Rate::Rest, None));
    let later = advance(&troupe(), stage, &ticked(1000));
    assert_eq!((later.tick, later.draws), (1000, 0));
}

#[test]
fn the_summoned_arrive_in_menagerie_order_and_fade_in() {
    let menagerie = troupe();
    let stage = meadow(&menagerie, 3, &["misty", "thorny", "mossy", "sparky", "pebble"]);
    assert_eq!(stage.actors.iter().map(|actor| actor.species.as_str()).collect::<Vec<_>>(), TROUPE);
    assert_eq!(stage.draws, 5);
    for (stream, actor) in stage.actors.iter().enumerate() {
        let kind = &menagerie.species[stream];
        let perch = stage.perches.iter().find(|perch| Some(&perch.surface) == actor.perch.as_ref() && perch.x0 <= actor.x && actor.x <= perch.x1).unwrap_or_else(|| panic!("{} stands on no perch", actor.species));
        assert!(actor.y == perch.y - hover_of(kind), "{}", actor.species);
        assert!(actor.x - kind.size.width / 2.0 >= perch.x0 && actor.x + kind.size.width / 2.0 <= perch.x1, "{}", actor.species);
        assert_eq!((actor.activity, actor.opacity, actor.leaving, actor.draws, actor.since), (Activity::Idle, 0.0, false, 1, 0), "{}", actor.species);
        assert_eq!(actor.needs, needs_of(kind.temperament), "{}", actor.species);
        assert!((BLINK_LOW..BLINK_HIGH).contains(&actor.blink), "{}", actor.species);
    }
    assert_eq!(frame_of(&menagerie, &stage).rate, Rate::Full);
    let whole = advance(&menagerie, stage, &ticked(16));
    assert!(whole.actors.iter().all(|actor| actor.opacity == 1.0));
}

#[test]
fn the_summoned_wait_while_there_is_no_perch() {
    let menagerie = troupe();
    let waiting = advance(&menagerie, open_stage(5), &[summoned(&TROUPE), StageEvent::Ticked(Ticked { ticks: 300 })]);
    assert!(waiting.actors.is_empty());
    assert_eq!((waiting.tick, waiting.draws, waiting.wanted.len()), (300, 0, 5));
    let arrived = advance(&menagerie, waiting, &[surveyed(800.0, 600.0, &[("floor", 0.0, 800.0, 600.0)], &[])]);
    assert_eq!(arrived.actors.len(), 5);
    assert!(arrived.actors.iter().all(|actor| actor.draws == 301 && actor.since == 300));
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
fn a_draft_keeps_one_actor_per_known_species() {
    let menagerie = troupe();
    let mut stage = meadow(&menagerie, 9, &["thorny", "mossy"]);
    let mut stranger = stage.actors[0].clone();
    stranger.species = "stranger".to_string();
    let mut twin = stage.actors[1].clone();
    twin.x += 100.0;
    let first = stage.actors[1].x;
    stage.actors.extend([stranger, twin]);
    let draft = draft_of(&menagerie, stage);
    assert_eq!(draft.stage.actors.iter().map(|actor| actor.species.as_str()).collect::<Vec<_>>(), ["mossy", "thorny"]);
    assert_eq!(draft.streams, [0, 2]);
    assert_eq!(draft.kinds.iter().map(|kind| kind.id.as_str()).collect::<Vec<_>>(), ["mossy", "thorny"]);
    assert!(draft.stage.actors[1].x == first);
}

#[test]
fn a_still_stage_rests() {
    let menagerie = troupe();
    let lively = advance(&menagerie, meadow(&menagerie, 11, &TROUPE), &[StageEvent::Pointed(Pointed { x: 400.0, y: 300.0 }), StageEvent::Ticked(Ticked { ticks: 200 })]);
    let still = advance(&menagerie, lively, &[StageEvent::Tuned(Tuned { mode: PetMode::Still })]);
    for actor in &still.actors {
        assert_eq!((actor.activity, actor.partner.as_ref(), actor.clip.as_ref(), actor.gaze, actor.opacity, actor.until), (Activity::Idle, None, None, CENTRED, 1.0, 200), "{}", actor.species);
        assert!(actor.mood == mood_of(Activity::Idle) && actor.perch.is_some());
    }
    let frame = frame_of(&menagerie, &still);
    assert_eq!((frame.rate, frame.wake, frame.actors.len()), (Rate::Rest, None, 5));
    for actor in &frame.actors {
        let kind = menagerie.species.iter().find(|kind| kind.id == actor.species).unwrap_or_else(|| panic!("{}", actor.species));
        assert_eq!(actor.bones, solve_rig(kind, &rest_pose(kind)), "{}", actor.species);
        assert!(actor.eyes.iter().all(|eye| eye.x == 0.0 && eye.y == 0.0 && eye.lid == 0.0), "{}", actor.species);
    }
    let draws: Vec<u32> = still.actors.iter().map(|actor| actor.draws).collect();
    let poked = advance(&menagerie, still.clone(), &[StageEvent::Poked(Poked { x: still.actors[0].x, y: still.actors[0].y - 10.0 }), StageEvent::Ticked(Ticked { ticks: 5000 })]);
    assert_eq!(poked.tick, 5200);
    assert_eq!(poked.actors, still.actors);
    let thawed = advance(&menagerie, poked, &[StageEvent::Tuned(Tuned { mode: PetMode::Lively })]);
    for (index, actor) in thawed.actors.iter().enumerate() {
        assert_eq!((actor.draws, actor.since), (draws[index].wrapping_add(1), 5200), "{}", actor.species);
        assert!(actor.until >= 5200 + 192 && actor.until < 5200 + 640 && actor.blink >= 5200 + BLINK_LOW && actor.blink < 5200 + BLINK_HIGH && actor.clip.is_some(), "{}", actor.species);
    }
}

#[test]
fn a_poke_makes_the_nearest_actor_greet_towards_it() {
    let menagerie = troupe();
    let stage = advance(&menagerie, meadow(&menagerie, 21, &["mossy", "thorny"]), &ticked(32));
    let before = actor_of(&stage, "mossy").clone();
    let far = advance(&menagerie, stage.clone(), &[StageEvent::Poked(Poked { x: before.x, y: before.y - 15.0 - 1.2 * 30.0 - 1.0 })]);
    assert_eq!(actor_of(&far, "mossy"), &before);
    let poked = advance(&menagerie, stage, &[StageEvent::Poked(Poked { x: before.x - 10.0, y: before.y - 15.0 })]);
    let after = actor_of(&poked, "mossy");
    assert_eq!((after.activity, after.facing, after.since, after.draws, after.partner.as_ref()), (Activity::Greet, Facing::Left, 32, before.draws + 1, None));
    assert_eq!(after.faced, if before.facing == Facing::Left { before.faced } else { 32 + TURN_TICKS });
    assert!(after.mood == smaller(1.0, before.mood + POKE_CHEER) && after.until >= 32 + 128 && after.until < 32 + 320);
    assert_eq!(after.clip.as_deref(), Some("wave"));
    assert_eq!(actor_of(&poked, "thorny"), actor_of(&far, "thorny"));
    let hushed = advance(&menagerie, far, &[StageEvent::Hushed(Hushed { quiet: true })]);
    assert_eq!(advance(&menagerie, hushed.clone(), &[StageEvent::Poked(Poked { x: before.x - 10.0, y: before.y - 15.0 })]), hushed);
}

#[test]
fn an_actor_that_is_asked_to_turn_again_in_the_middle_of_a_turn_turns_back_from_where_its_drawing_is() {
    let menagerie = plain(&troupe());
    let level = 400.0 - 15.0;
    let mut stage = rested(&menagerie, 0.0);
    stage.actors[0].clip = None;
    let stage = advance(&menagerie, stage, &[StageEvent::Poked(Poked { x: 530.0, y: level })]);
    assert_eq!((stage.actors[0].facing, stage.actors[0].faced, stage.actors[0].activity), (Facing::Left, stage.tick + 8, Activity::Greet));
    let stage = run(&menagerie, stage, 3, |_| {});
    let before = frame_of(&menagerie, &stage).actors[0].clone();
    assert!(before.bones[0] == 2.0 * smoothstep(3.0 / 8.0) - 1.0);
    let stage = advance(&menagerie, stage, &[StageEvent::Poked(Poked { x: 570.0, y: level })]);
    assert_eq!((stage.actors[0].facing, stage.actors[0].faced), (Facing::Right, stage.tick + 3));
    let after = frame_of(&menagerie, &stage).actors[0].clone();
    assert!((after.bones[0] * after.facing.sign() - before.bones[0] * before.facing.sign()).abs() < 1e-12);
    let stage = run(&menagerie, stage, 3, |_| {});
    assert!(frame_of(&menagerie, &stage).actors[0].bones[0] == 1.0);
}

#[test]
fn a_leaver_fades_and_is_gone_and_a_wanted_one_stays() {
    let menagerie = troupe();
    let stage = advance(&menagerie, meadow(&menagerie, 4, &["mossy", "pebble"]), &ticked(20));
    let leaving = advance(&menagerie, stage, &[summoned(&["pebble"])]);
    assert!(actor_of(&leaving, "mossy").leaving && !actor_of(&leaving, "pebble").leaving);
    let back = advance(&menagerie, leaving.clone(), &[summoned(&["pebble", "mossy"])]);
    assert!(!actor_of(&back, "mossy").leaving && actor_of(&back, "mossy").activity == Activity::Idle);
    let gone = advance(&menagerie, leaving, &ticked(64 * 40));
    assert_eq!(gone.actors.iter().map(|actor| actor.species.as_str()).collect::<Vec<_>>(), ["pebble"]);
}

#[test]
fn a_quiet_hour_of_sleep_is_jumped_over() {
    let menagerie = troupe();
    let mut stage = advance(&menagerie, meadow(&menagerie, 2, &["pebble"]), &[StageEvent::Hushed(Hushed { quiet: true }), StageEvent::Ticked(Ticked { ticks: 64 })]);
    let sleeper = &mut stage.actors[0];
    (sleeper.activity, sleeper.clip, sleeper.since, sleeper.until, sleeper.mood, sleeper.gaze) = (Activity::Sleep, Some("doze".to_string()), 0, 64 + 64 * 3600, mood_of(Activity::Sleep), CENTRED);
    let draft = draft_of(&menagerie, stage.clone());
    assert_eq!(lull(&menagerie, &draft, 1_000_000), 64 * 3600 - 1);
    assert_eq!(frame_of(&menagerie, &stage).rate, Rate::Quarter);
    let later = advance(&menagerie, stage.clone(), &ticked(64 * 3600 - 1));
    assert_eq!((later.tick, &later.actors), (64 * 3600 + 63, &stage.actors));
    let near = advance(&menagerie, stage.clone(), &[StageEvent::Pointed(Pointed { x: stage.actors[0].x, y: stage.actors[0].y }), StageEvent::Ticked(Ticked { ticks: 1 })]);
    assert_eq!(near.actors[0].activity, Activity::Idle);
    let away = advance(&menagerie, near, &[StageEvent::Unpointed(Unpointed {}), StageEvent::Glanced(Glanced { points: vec![Point { x: 0.0, y: 0.0 }] })]);
    assert_eq!((away.pointer, away.glances.len()), (None, 1));
}

#[test]
fn an_actor_turns_see_through_while_the_pointer_rests_on_it() {
    let menagerie = troupe();
    let kind = &menagerie.species[0];
    let stage = advance(&menagerie, meadow(&menagerie, 6, &["mossy"]), &ticked(32));
    let mossy = stage.actors[0].clone();
    let over = |dx: f64, dy: f64| Some(Point { x: mossy.x + dx, y: mossy.y + dy });
    assert!(presence_of(None, &mossy, kind) == 1.0 && presence_of(over(24.5, -10.0), &mossy, kind) == 1.0 && presence_of(over(0.0, -34.5), &mossy, kind) == 1.0 && presence_of(over(-24.5, 0.0), &mossy, kind) == 1.0);
    assert!(presence_of(over(24.0, -10.0), &mossy, kind) == SHY_OPACITY && presence_of(over(0.0, 4.0), &mossy, kind) == SHY_OPACITY && presence_of(over(-24.0, -34.0), &mossy, kind) == SHY_OPACITY);
    let pointed = StageEvent::Pointed(Pointed { x: mossy.x, y: mossy.y - 10.0 });
    let shy = advance(&menagerie, stage.clone(), &[pointed.clone(), StageEvent::Ticked(Ticked { ticks: 1 })]);
    assert!(shy.actors[0].opacity == 1.0 - SHY_STEP);
    assert_eq!(frame_of(&menagerie, &shy).rate, Rate::Full);
    let seen = advance(&menagerie, shy, &ticked(40));
    assert!(seen.actors[0].opacity == SHY_OPACITY);
    let back = advance(&menagerie, seen, &[StageEvent::Unpointed(Unpointed {}), StageEvent::Ticked(Ticked { ticks: 11 })]);
    assert!(back.actors[0].opacity == 1.0);
    let still = advance(&menagerie, stage, &[StageEvent::Tuned(Tuned { mode: PetMode::Still }), pointed]);
    let frame = frame_of(&menagerie, &still);
    assert!(still.actors[0].opacity == 1.0 && frame.actors[0].opacity == SHY_OPACITY);
    assert_eq!((frame.rate, frame.wake), (Rate::Rest, None));
}

#[test]
fn a_walker_turns_round_before_it_sets_out() {
    let menagerie = troupe();
    let mut draft = draft_of(&menagerie, advance(&menagerie, meadow(&menagerie, 6, &["mossy"]), &ticked(32)));
    let x = draft.stage.actors[0].x;
    draft.stage.actors[0].facing = Facing::Right;
    stroll(&mut draft, 0, x - 30.0, Some("stroll".to_string()), 32);
    assert_eq!(heading_of(&draft.stage, &draft.stage.actors, &draft.kinds, 0, 33), Some(Facing::Left));
    let begun = advance(&menagerie, sealed(draft), &ticked(1));
    assert_eq!((begun.actors[0].facing, begun.actors[0].faced, begun.actors[0].since), (Facing::Left, 41, 32));
    assert!(begun.actors[0].x == x && frame_of(&menagerie, &begun).actors[0].facing == Facing::Left);
    let halfway = advance(&menagerie, begun, &ticked(4));
    assert!(halfway.actors[0].x == x && halfway.actors[0].facing == Facing::Left);
    let frame = frame_of(&menagerie, &halfway);
    assert_eq!(frame.rate, Rate::Full);
    assert!(frame.actors[0].bones.iter().step_by(2).all(|entry| *entry == 0.0));
    let turned = advance(&menagerie, halfway, &ticked(4));
    assert_eq!((turned.actors[0].facing, turned.actors[0].faced, turned.actors[0].since), (Facing::Left, 41, 41));
    assert!(turned.actors[0].x == x && squeeze_of(&turned.actors[0], turned.tick) == 1.0);
    let striding = advance(&menagerie, turned, &ticked(1));
    assert!(striding.actors[0].x == x - 36.0 / 64.0);
}

#[test]
fn an_idle_actor_turns_round_to_a_pointer_that_is_clearly_behind_it() {
    let menagerie = troupe();
    let facings = |x: f64| {
        let mut seen = Vec::new();
        run(&menagerie, advance(&menagerie, rested(&menagerie, 0.0), &[pointed(x)]), 128, |after| seen.push(after.actors[0].facing));
        seen
    };
    assert!(facings(550.0 - 20.0 - 11.0).iter().all(|&facing| facing == Facing::Right));
    assert!(facings(550.0 + 300.0).iter().all(|&facing| facing == Facing::Right));
    assert!(facings(550.0 - 20.0 - 13.0).iter().all(|&facing| facing == Facing::Left));
    let mut stage = advance(&menagerie, rested(&menagerie, 0.0), &[pointed(100.0), StageEvent::Ticked(Ticked { ticks: 1 })]);
    assert_eq!((stage.actors[0].facing, stage.actors[0].faced, stage.actors[0].activity), (Facing::Left, stage.tick + 8, Activity::Idle));
    assert!(stage.actors[0].x == 550.0);
    let mut widths = Vec::new();
    for step in 0..=8 {
        let frame = frame_of(&menagerie, &stage);
        assert_eq!(frame.rate, if step < 8 { Rate::Full } else { Rate::Half }, "{step}");
        widths.push(frame.actors[0].bones[0]);
        stage = advance(&menagerie, stage, &ticked(1));
    }
    assert!(widths[0] < 0.0 && widths[4].abs() < 1e-12 && widths[8] > 0.0);
    assert!(widths.windows(2).all(|pair| pair[1] > pair[0]));
}

#[test]
fn a_pointer_that_crosses_over_and_back_turns_an_actor_once_a_second_at_most() {
    let menagerie = troupe();
    let mut stage = rested(&menagerie, 0.0);
    let mut turns = Vec::new();
    for tick in 0..256 {
        let before = stage.actors[0].facing;
        stage = advance(&menagerie, stage, &[pointed(if tick % 4 < 2 { 100.0 } else { 1000.0 }), StageEvent::Ticked(Ticked { ticks: 1 })]);
        if stage.actors[0].facing != before {
            turns.push(stage.tick);
        }
    }
    assert!((2..=4).contains(&turns.len()), "{turns:?}");
    assert!(turns.windows(2).all(|pair| pair[1] - pair[0] >= 64), "{turns:?}");
}

#[test]
fn a_pointer_turns_nobody_who_concentrates_stands_still_walks_or_sleeps_or_once_it_is_old_news() {
    let menagerie = troupe();
    let pointed_at = |stage: Stage| advance(&menagerie, stage, &[pointed(100.0), StageEvent::Ticked(Ticked { ticks: 128 })]);
    let standing = rested(&menagerie, 0.0);
    assert_eq!(pointed_at(standing.clone()).actors[0].facing, Facing::Left);
    let hushed = pointed_at(advance(&menagerie, standing.clone(), &[StageEvent::Hushed(Hushed { quiet: true })]));
    assert_eq!(hushed.actors[0].facing, Facing::Right);
    assert_eq!(pointed_at(advance(&menagerie, standing.clone(), &[StageEvent::Tuned(Tuned { mode: PetMode::Still })])).actors[0].facing, Facing::Right);
    let mut asleep = standing.clone();
    (asleep.actors[0].activity, asleep.actors[0].clip) = (Activity::Sleep, None);
    assert_eq!(pointed_at(asleep).actors[0].facing, Facing::Right);
    let mut walking = standing;
    (walking.actors[0].activity, walking.actors[0].goal) = (Activity::Walk, 780.0);
    let walked = pointed_at(walking);
    assert_eq!((walked.actors[0].facing, walked.actors[0].activity), (Facing::Right, Activity::Walk));
    let stale = advance(&menagerie, hushed, &[StageEvent::Ticked(Ticked { ticks: 192 }), StageEvent::Hushed(Hushed { quiet: false }), StageEvent::Ticked(Ticked { ticks: 128 })]);
    assert_eq!(stale.actors[0].facing, Facing::Right);
}

#[test]
fn the_bone_that_carries_the_first_eye_leans_after_the_gaze() {
    let menagerie = troupe();
    let kind = &menagerie.species[0];
    let bone = kind.bones.iter().position(|bone| bone.id == kind.face.eyes[0].bone).unwrap_or_else(|| panic!("the first eye of {} sits on no bone", kind.id));
    let bones = |menagerie: &Menagerie, stage: &Stage| frame_of(menagerie, stage).actors[0].bones.clone();
    let upright = |stage: &Stage| {
        let mut centred = stage.clone();
        centred.actors[0].gaze = CENTRED;
        bones(&menagerie, &centred)
    };
    let stage = advance(&menagerie, rested(&menagerie, 0.0), &[pointed(1100.0), StageEvent::Ticked(Ticked { ticks: 64 })]);
    let gaze = stage.actors[0].gaze;
    assert!(gaze.x > 0.8);
    assert_ne!(bones(&menagerie, &stage), upright(&stage));
    let bare = plain(&menagerie);
    let mut pose = rest_pose(kind);
    pose[bone] = BonePose { x: pose[bone].x + 1.5 * gaze.x, y: pose[bone].y + gaze.y, rotation: pose[bone].rotation + 5.0 * gaze.x, scale_x: 1.0, scale_y: 1.0 };
    let mut clipless = stage.clone();
    clipless.actors[0].clip = None;
    assert_eq!(bones(&bare, &clipless), solve_rig(kind, &pose));
    assert_eq!(bones(&menagerie, &advance(&menagerie, stage.clone(), &[StageEvent::Hushed(Hushed { quiet: true })])), upright(&stage));
    assert_eq!(bones(&menagerie, &advance(&menagerie, stage.clone(), &[StageEvent::Tuned(Tuned { mode: PetMode::Still })])), solve_rig(kind, &rest_pose(kind)));
    let (mut mirrored, mut settled) = (stage.clone(), stage);
    (mirrored.actors[0].facing, mirrored.actors[0].gaze) = (Facing::Left, Gaze { x: 0.0 - gaze.x, y: gaze.y, vx: 0.0, vy: 0.0 });
    settled.actors[0].gaze = Gaze { x: gaze.x, y: gaze.y, vx: 0.0, vy: 0.0 };
    assert_eq!(bones(&menagerie, &mirrored), bones(&menagerie, &settled));
}

#[test]
fn an_idle_actor_greets_a_pointer_that_has_come_to_rest_beside_it_and_pays_with_curiosity() {
    let menagerie = troupe();
    let beside = 550.0 + 20.0 + 20.0;
    let stage = advance(&menagerie, rested(&menagerie, 1.0), &[pointed(beside)]);
    let since = stage.tick;
    let waiting = advance(&menagerie, stage.clone(), &ticked(31));
    assert_eq!(waiting.actors[0].activity, Activity::Idle);
    let greeting = advance(&menagerie, waiting, &ticked(1));
    let body = &greeting.actors[0];
    assert_eq!((greeting.tick, body.activity, body.since, body.facing, body.partner.as_ref(), body.clip.as_deref()), (since + 32, Activity::Greet, since + 32, Facing::Right, None, Some("wave")));
    assert!(body.x == 550.0 && body.until - greeting.tick >= 128 && body.until - greeting.tick < 320 && (body.needs.curiosity - 0.4).abs() < 1e-9);
    assert_eq!(frame_of(&menagerie, &greeting).actors[0].activity, Activity::Greet);
    assert_eq!(advance(&menagerie, stage, &ticked(32)), greeting);
    let rests = |x: f64, y: f64, quiet: bool| advance(&menagerie, rested(&menagerie, 1.0), &[StageEvent::Hushed(Hushed { quiet }), StageEvent::Pointed(Pointed { x, y }), StageEvent::Ticked(Ticked { ticks: 40 })]).actors[0].activity;
    assert_eq!(rests(beside, LEVEL, false), Activity::Greet);
    assert_eq!(rests(550.0, LEVEL, false), Activity::Idle);
    assert_eq!(rests(550.0 + 4.0 * 30.0, LEVEL, false), Activity::Idle);
    assert_eq!(rests(beside, LEVEL, true), Activity::Idle);
    let dull = advance(&menagerie, rested(&menagerie, 0.45), &[pointed(beside), StageEvent::Ticked(Ticked { ticks: 40 })]);
    assert_eq!(dull.actors[0].activity, Activity::Idle);
    let spent = advance(&menagerie, rested(&menagerie, 0.5), &[pointed(beside), StageEvent::Ticked(Ticked { ticks: 32 })]);
    assert!(spent.actors[0].activity == Activity::Greet && spent.actors[0].needs.curiosity == 0.0);
    let mut moving = rested(&menagerie, 1.0);
    for tick in 0..128 {
        moving = advance(&menagerie, moving, &[pointed(beside + f64::from(tick % 20)), StageEvent::Ticked(Ticked { ticks: 1 })]);
        assert_eq!(moving.actors[0].activity, Activity::Idle, "{tick}");
    }
}

#[test]
fn a_resting_pointer_is_jumped_over_up_to_the_ticks_on_which_it_can_change_something() {
    let troupe = troupe();
    for menagerie in [plain(&troupe), troupe] {
        for (curiosity, x) in [(1.0, 590.0), (0.0, 100.0), (0.0, 590.0)] {
            let mut start = rested(&menagerie, curiosity);
            if menagerie.species[0].clips.is_empty() {
                start.actors[0].clip = None;
            }
            let start = advance(&menagerie, start, &[pointed(x)]);
            for chunk in [1, 31, 32, 33, 200, 2000] {
                assert_eq!(advance(&menagerie, start.clone(), &ticked(chunk)), run(&menagerie, start.clone(), chunk as Ticks, |_| {}), "{x} in one chunk of {chunk}");
            }
            let mut stage = start;
            let mut step = 0;
            while step < 384 {
                let frame = frame_of(&menagerie, &stage);
                let ticks = match frame.wake {
                    Some(wake) if frame.rate == Rate::Rest => Ticks::min(wake - stage.tick, 384 - step),
                    _ => 1,
                };
                assert!(ticks > 0);
                let woken = advance(&menagerie, stage.clone(), &ticked(ticks.unsigned_abs()));
                assert_eq!(woken, run(&menagerie, stage.clone(), ticks, |_| {}), "{x} woken {ticks} ticks after {}", stage.tick);
                if frame.rate == Rate::Rest {
                    assert_eq!(frame_of(&menagerie, &run(&menagerie, stage.clone(), ticks - 1, |_| {})).actors, frame.actors);
                }
                stage = woken;
                step += ticks;
            }
        }
    }
}

#[test]
fn newcomers_spread_out_and_take_the_ground_last() {
    let menagerie = troupe();
    let shelves = [("one", 40.0, 400.0, 300.0), ("two", 440.0, 800.0, 350.0), ("three", 840.0, 1240.0, 300.0), ("floor", 0.0, 1280.0, 720.0)];
    let mut first = Vec::new();
    for seed in 1..=40 {
        let few = advance(&menagerie, open_stage(seed), &[surveyed(1280.0, 720.0, &shelves, &[]), summoned(&TROUPE[..3])]);
        let mut perches: Vec<&str> = few.actors.iter().filter_map(|actor| actor.perch.as_deref()).collect();
        perches.sort_unstable();
        assert_eq!(perches, ["one", "three", "two"], "seed {seed}");
        if let Some(perch) = few.actors[0].perch.clone().filter(|perch| !first.contains(perch)) {
            first.push(perch);
        }
        let more = advance(&menagerie, few, &[summoned(&TROUPE[..4])]);
        assert_eq!(actor_of(&more, TROUPE[3]).perch.as_deref(), Some("floor"), "seed {seed}");
        let alone = advance(&menagerie, open_stage(seed), &[surveyed(1280.0, 720.0, &shelves[3..], &[]), summoned(&TROUPE[..1])]);
        assert_eq!(alone.actors[0].perch.as_deref(), Some("floor"), "seed {seed}");
    }
    assert_eq!(first.len(), 3);
}

#[test]
fn an_actor_is_gone_with_ground_that_vanishes_when_the_survey_brings_new_ground_and_falls_without() {
    let menagerie = troupe();
    let stage = rested(&menagerie, 0.0);
    let floor = ("floor", 0.0, 1280.0, 720.0);
    let shelf = ("shelf", 40.0, 400.0, 300.0);
    let fallen = advance(&menagerie, stage.clone(), &[surveyed(1280.0, 720.0, &[floor], &[])]);
    assert_eq!((fallen.actors[0].activity, fallen.actors[0].perch.as_deref()), (Activity::Fall, None));
    assert!(fallen.actors[0].x == 550.0);
    let moved = advance(&menagerie, stage.clone(), &[surveyed(1280.0, 720.0, &[shelf, floor], &[])]);
    assert_eq!((moved.actors[0].activity, moved.actors[0].perch.as_deref(), moved.actors[0].since), (Activity::Idle, Some("shelf"), stage.tick));
    assert!(moved.actors[0].opacity == 0.0 && moved.actors[0].x <= 400.0);
    let replaced = advance(&menagerie, stage.clone(), &[surveyed(1280.0, 720.0, &[("card-again", 200.0, 900.0, 400.0), floor], &[])]);
    assert_eq!((replaced.actors[0].activity, replaced.actors[0].perch.as_deref()), (Activity::Idle, Some("card-again")));
    assert!(replaced.actors[0].x == 550.0 && replaced.actors[0].opacity == 1.0);
    let hushed = advance(&menagerie, stage, &[StageEvent::Hushed(Hushed { quiet: true }), surveyed(1280.0, 720.0, &[shelf, floor], &[])]);
    assert!(hushed.actors[0].perch.as_deref() == Some("shelf") && hushed.actors[0].opacity == 0.0);
}

#[test]
fn a_company_spreads_out_over_new_ground() {
    let menagerie = troupe();
    let floor = [("floor", 0.0, 1280.0, 720.0)];
    let wide = [("card", 300.0, 800.0, 400.0), ("shelf", 40.0, 400.0, 300.0), ("floor", 0.0, 1280.0, 720.0)];
    let mut stage = advance(&menagerie, open_stage(1), &[surveyed(1280.0, 720.0, &floor, &[]), summoned(&TROUPE), StageEvent::Ticked(Ticked { ticks: 128 })]);
    for body in &mut stage.actors {
        (body.activity, body.until, body.partner) = (Activity::Idle, stage.tick + FAR, None);
    }
    assert!(stage.actors.iter().all(|actor| actor.perch.as_deref() == Some("floor")));
    assert!(advance(&menagerie, stage.clone(), &[surveyed(1280.0, 720.0, &floor, &[])]).actors.iter().all(|actor| !actor.leaving));
    let hushed = advance(&menagerie, stage.clone(), &[StageEvent::Hushed(Hushed { quiet: true }), surveyed(1280.0, 720.0, &wide, &[])]);
    assert!(hushed.actors.iter().all(|actor| !actor.leaving));
    let widened = advance(&menagerie, stage.clone(), &[surveyed(1280.0, 720.0, &wide, &[])]);
    assert_eq!(widened.actors.iter().filter(|actor| actor.leaving).map(|actor| actor.species.as_str()).collect::<Vec<_>>(), TROUPE[1..3]);
    assert_eq!(advance(&menagerie, widened.clone(), &[surveyed(1280.0, 720.0, &wide, &[])]), widened);
    let mut landed: Vec<(String, String)> = Vec::new();
    run(&menagerie, widened, 512, |after| {
        for actor in after.actors.iter().filter(|actor| !actor.leaving && TROUPE[1..3].contains(&actor.species.as_str())) {
            if landed.iter().all(|(species, _)| *species != actor.species) {
                landed.push((actor.species.clone(), actor.perch.clone().unwrap_or_default()));
            }
        }
    });
    let mut homes: Vec<&str> = landed.iter().map(|(_, perch)| perch.as_str()).collect();
    homes.sort_unstable();
    assert_eq!(homes, ["card", "shelf"]);
    let still = advance(&menagerie, stage, &[StageEvent::Tuned(Tuned { mode: PetMode::Still }), surveyed(1280.0, 720.0, &wide, &[])]);
    assert_eq!(still.actors.iter().map(|actor| actor.species.as_str()).collect::<Vec<_>>(), TROUPE);
    let mut moved: Vec<&str> = TROUPE[1..3].iter().map(|species| actor_of(&still, species)).filter(|actor| actor.opacity == 1.0 && !actor.leaving).filter_map(|actor| actor.perch.as_deref()).collect();
    moved.sort_unstable();
    assert_eq!(moved, ["card", "shelf"]);
}

#[test]
fn a_hopper_that_finds_somebody_in_its_way_in_mid_hop_finishes_the_hop_on_the_spot() {
    let menagerie = troupe();
    let floor = surveyed(1280.0, 400.0, &[("floor", 0.0, 1280.0, 400.0)], &[]);
    let mut draft = draft_of(&menagerie, advance(&menagerie, open_stage(3), &[floor, summoned(&["mossy", "sparky"]), StageEvent::Ticked(Ticked { ticks: 32 })]));
    assert_eq!(beat_of(draft.kinds[1], clip_of(draft.kinds[1], Some("bounce"))), 32);
    (draft.stage.actors[1].x, draft.stage.actors[1].facing) = (300.0, Facing::Right);
    draft.stage.actors[0].x = 300.0 + shoulders(draft.kinds[0], draft.kinds[1]) + MEET_GAP + 3.0;
    stroll(&mut draft, 1, 600.0, Some("bounce".to_string()), 32);
    assert!(!hindered(&draft, 1, 1) && hindered(&draft, 1, 32));
    let stopped = advance(&menagerie, sealed(draft), &ticked(5));
    let sparky = actor_of(&stopped, "sparky");
    assert!(sparky.x == 303.0 && sparky.goal == 303.0);
    assert_eq!((sparky.activity, sparky.since), (Activity::Walk, 32));
    let rested = advance(&menagerie, stopped, &ticked(27));
    let sparky = actor_of(&rested, "sparky");
    assert!(sparky.x == 303.0 && sparky.activity == Activity::Idle && sparky.since == 64);
}

#[test]
fn a_perch_that_shrank_holds_only_those_who_fit() {
    let menagerie = troupe();
    let floor = |width: f64| surveyed(width, 400.0, &[("floor", 0.0, width, 400.0)], &[]);
    let roomy = advance(&menagerie, open_stage(17), &[floor(1280.0), summoned(&TROUPE), StageEvent::Ticked(Ticked { ticks: 32 })]);
    assert_eq!(roomy.actors.len(), 5);
    let crowded = advance(&menagerie, roomy, &[floor(100.0)]);
    let seated: Vec<&Actor> = crowded.actors.iter().filter(|actor| !actor.leaving).collect();
    assert!((1..5).contains(&seated.len()));
    assert!(crowded.actors.iter().filter(|actor| actor.leaving).all(|actor| actor.perch.is_none() && actor.partner.is_none()));
    let width = |actor: &Actor| menagerie.species.iter().find(|kind| kind.id == actor.species).map_or(0.0, |kind| kind.size.width);
    let need: f64 = seated.iter().map(|actor| width(actor)).sum::<f64>() + (seated.len() - 1) as f64 * COMFORT_GAP;
    assert!(need <= 100.0);
    for one in &seated {
        assert!(one.x - width(one) / 2.0 >= 0.0 && one.x + width(one) / 2.0 <= 100.0, "{}", one.species);
        for other in &seated {
            assert!(one.species == other.species || (one.x - other.x).abs() >= (width(one) + width(other)) / 2.0, "{} in {}", one.species, other.species);
        }
    }
    let later = advance(&menagerie, crowded, &ticked(640));
    assert!(later.actors.len() < 5 && later.actors.iter().all(|actor| !actor.leaving));
    assert_eq!(later.wanted.len(), 5);
    assert_eq!(arrival_tick(&menagerie, &later, &later.actors, later.tick + 1), (later.tick + 64) / 64 * 64);
}

#[test]
fn stretches_are_carved_by_open_intervals() {
    assert_eq!(carve(&[[0.0, 10.0]], 3.0, 5.0), [[0.0, 3.0], [5.0, 10.0]]);
    assert_eq!(carve(&[[0.0, 10.0]], -5.0, 0.0), [[0.0, 10.0]]);
    assert_eq!(carve(&[[0.0, 10.0]], 10.0, 12.0), [[0.0, 10.0]]);
    assert_eq!(carve(&[[0.0, 10.0]], -1.0, 4.0), [[4.0, 10.0]]);
    assert_eq!(carve(&[[0.0, 10.0], [20.0, 30.0]], 8.0, 22.0), [[0.0, 8.0], [22.0, 30.0]]);
    assert!(carve(&[[0.0, 10.0]], 0.0, 10.0).is_empty());
}

#[test]
fn units_dwells_and_facings_are_the_expressions_of_the_twin() {
    assert!(unit_of(0) == 0.0 && unit_of(u32::MAX) == 4_294_967_295.0 / 4_294_967_296.0 && unit_of(0x8000_0000) == 0.5);
    assert_eq!((blink_at(100, 0.0), blink_at(100, 0.5), blink_at(100, unit_of(u32::MAX))), (228, 356, 483));
    assert_eq!((facing_to(1.0, 1.0), facing_to(0.0, 1.0), facing_to(f64::NAN, 1.0)), (Facing::Right, Facing::Left, Facing::Left));
    let kind = &troupe().species[0];
    assert_eq!(clip_at(kind, Activity::Fidget, 0.0).as_deref(), Some("wiggle"));
    assert_eq!(clip_at(kind, Activity::Fidget, 0.5).as_deref(), Some("stretch"));
    assert_eq!(clip_at(kind, Activity::Fidget, 1.0).as_deref(), Some("stretch"));
    assert_eq!(clip_at(kind, Activity::Hop, 0.3).as_deref(), Some("breathe"));
    assert_eq!(clips_of(&troupe().species[1], Activity::Walk), ["bounce"]);
    assert_eq!(breath_of(kind).map(|clip| clip.id.as_str()), Some("breathe"));
    assert!(hover_of(kind) == 0.0 && hover_of(&troupe().species[4]) == 14.0);
}

#[test]
fn pairs_are_drawn_on_whole_seconds_after_the_warm_up() {
    let menagerie = troupe();
    let stage = advance(&menagerie, meadow(&menagerie, 8, &["mossy", "sparky"]), &ticked(16));
    assert_eq!(pairing_tick(&stage, &stage.actors, 17), WARMUP);
    assert_eq!(pairing_tick(&stage, &stage.actors, WARMUP + 1), WARMUP + 64);
    let mut met = stage.clone();
    met.met = 1000;
    assert_eq!(pairing_tick(&met, &met.actors, 17), 6784);
    let mut quiet = stage.clone();
    quiet.quiet = true;
    assert_eq!(pairing_tick(&quiet, &quiet.actors, 17), -1);
    let mut alone = stage.clone();
    alone.actors.truncate(1);
    assert_eq!(pairing_tick(&alone, &alone.actors, 17), -1);
    let mut still = stage;
    still.mode = PetMode::Still;
    assert_eq!(pairing_tick(&still, &still.actors, 17), -1);
}

#[test]
fn a_frame_is_drawn_back_to_front() {
    let menagerie = troupe();
    let stage = advance(&menagerie, meadow(&menagerie, 13, &TROUPE), &ticked(40));
    let frame = frame_of(&menagerie, &stage);
    assert_eq!(frame.actors.len(), 5);
    for pair in frame.actors.windows(2) {
        assert!(pair[0].y < pair[1].y || (pair[0].y == pair[1].y && pair[0].species < pair[1].species), "{} before {}", pair[0].species, pair[1].species);
    }
    let mut reversed = stage.clone();
    reversed.actors.reverse();
    assert_eq!(frame_of(&menagerie, &reversed), frame);
    let mut one = stage.actors[0].clone();
    let mut other = stage.actors[0].clone();
    (one.species, other.species) = ("\u{ff5e}".to_string(), "\u{1f600}".to_string());
    assert!(behind(&other, &one) && !behind(&one, &other) && one.species < other.species);
    other.y = f64::NAN;
    assert!(behind(&other, &one));
    one.y -= 1.0;
    other.y = one.y + 1.0;
    assert!(behind(&one, &other) && !behind(&other, &one));
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
    for forbidden in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", ".max(", ".min(", "f64::max", "f64::min", "println!", "eprintln!", "dbg!", "static ", "thread_local!", "HashMap", "BTreeMap"] {
        assert!(!SOURCE.contains(forbidden), "{forbidden}");
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
