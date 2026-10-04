//! 👥️ Unit tests of the population: the summoned arrive in menagerie order and fade in, wait while there is no perch, and spread out with the ground last; a still stage rests; a leaver fades and a wanted one stays; ground that vanishes takes its actor along when the survey brings new ground and drops it otherwise; a company spreads out over new ground; and a perch that shrank holds only those who fit.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../🎪️stage/🧪️tests/🔬️unit/🦀️.rs — the kit: the troupe, a meadow to stand on, the events and a pet at rest

use super::*;
use crate::draft::{BLINK_HIGH, BLINK_LOW};
use crate::rig::{rest_pose, solve_rig};
use crate::schedule::arrival_tick;
use crate::schema::{Hushed, Over, Pointed, Rate, StageEvent, Ticked, Tuned};
use crate::stage::tests::{actor_of, clicked, meadow, rested, run, summoned, surveyed, ticked, troupe, FAR, TROUPE};
use crate::stage::{advance, frame_of, open_stage};

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
fn a_still_stage_rests() {
    let menagerie = troupe();
    let lively = advance(&menagerie, meadow(&menagerie, 11, &TROUPE), &[StageEvent::Pointed(Pointed { x: 400.0, y: 300.0, over: Over::Free }), StageEvent::Ticked(Ticked { ticks: 200 })]);
    let still = advance(&menagerie, lively, &[StageEvent::Tuned(Tuned { mode: PetMode::Still })]);
    for actor in &still.actors {
        assert_eq!((actor.activity, actor.partner.as_ref(), actor.clip.as_ref(), actor.gaze, actor.opacity, actor.until), (Activity::Idle, None, None, CENTRED, 1.0, 200), "{}", actor.species);
        let kind = menagerie.species.iter().find(|kind| kind.id == actor.species).unwrap_or_else(|| panic!("{}", actor.species));
        assert!(actor.feeling == at_rest(kind.mood, 200) && actor.warmth == COLD && actor.perch.is_some(), "{}", actor.species);
    }
    let frame = frame_of(&menagerie, &still);
    assert_eq!((frame.rate, frame.wake, frame.actors.len()), (Rate::Rest, None, 5));
    for actor in &frame.actors {
        let kind = menagerie.species.iter().find(|kind| kind.id == actor.species).unwrap_or_else(|| panic!("{}", actor.species));
        assert_eq!(actor.bones, solve_rig(kind, &rest_pose(kind)), "{}", actor.species);
        assert!(actor.eyes.iter().all(|eye| eye.x == 0.0 && eye.y == 0.0 && eye.lid == 0.0), "{}", actor.species);
    }
    let draws: Vec<u32> = still.actors.iter().map(|actor| actor.draws).collect();
    let clicked = advance(&menagerie, advance(&menagerie, still.clone(), &clicked(still.actors[0].x, still.actors[0].y - 10.0)), &ticked(5000));
    assert_eq!(clicked.tick, 5200);
    assert_eq!(clicked.actors, still.actors);
    let thawed = advance(&menagerie, clicked, &[StageEvent::Tuned(Tuned { mode: PetMode::Lively })]);
    for (index, actor) in thawed.actors.iter().enumerate() {
        assert_eq!((actor.draws, actor.since), (draws[index].wrapping_add(1), 5200), "{}", actor.species);
        assert!(actor.until >= 5200 + 192 && actor.until < 5200 + 640 && actor.blink >= 5200 + BLINK_LOW && actor.blink < 5200 + BLINK_HIGH && actor.clip.is_some(), "{}", actor.species);
    }
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
