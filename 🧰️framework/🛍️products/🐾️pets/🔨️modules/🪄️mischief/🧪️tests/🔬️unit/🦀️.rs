//! 🪄️ Unit tests of the Rust twin of the mischief module: every committed vector of the mischief-choice case (matches, candidates, choices, leaks, gates exactly; stations, lifts and throws within a billionth where numpy computes them, everything else exactly), the proof that the pick reads nothing but keys and a draw, the laws of the gates, the station, the lift and the throw, and the ban on platform transcendentals.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/🪄️mischief-choice/🔣️.json — the answers of Python's strings and numpy (case 🪄️mischief-choice)
//! @see ../../🟦️.ts — the TypeScript twin, whose suite these tests follow

use super::*;
use crate::randomness::random_unit;
use crate::schema::tests::{entries, fixture, number, typed};
use serde_json::Value;
use std::cell::Cell;

const SEED: u32 = 20_261_003;
const QUIZZES: [&str; 4] = ["physics", "heating", "cooling", "demand"];
const TASKS: [&str; 6] = ["powers", "energies", "u-values", "heating-load", "heating-load-and-demand", "final-energy"];
const ITEMS: [&str; 6] = ["sun", "sunlight-on-earth", "kettle", "wall-geg", "wall", "kfw-40"];
const OPEN: Circumstances = Circumstances { permitted: true, fine: true, width: 1440.0, mode: PetMode::Calm, quiet: false, lifting: false, tick: 20000, stirred: 19000, rested: 8000 };

fn row() -> Fixture {
    Fixture { id: "row-2".to_string(), key: "heating/heating-load-and-demand".to_string(), x: 278.4, y: 221.2, width: 883.2, height: 38.0 }
}

fn wall(name: &str, side: Facing, x: f64) -> Pitch {
    Pitch { wall: name.to_string(), surface: "card".to_string(), side, x, y0: 150.0, y1: 420.0 }
}

fn keys() -> Vec<String> {
    QUIZZES.iter().flat_map(|quiz| std::iter::once(quiz.to_string()).chain(TASKS.iter().flat_map(move |task| std::iter::once(format!("{quiz}/{task}")).chain(ITEMS.iter().map(move |item| format!("{quiz}/{task}/{item}")))))).collect()
}

fn whole(stream: u32, counter: u32, below: u32) -> u32 {
    (random_unit(&[SEED, stream, counter]) * f64::from(below)).floor() as u32
}

fn strings(value: &Value) -> Vec<String> {
    typed(value)
}

#[derive(serde::Deserialize)]
struct Described {
    id: String,
    key: String,
    value: Value,
    correct: bool,
    answered: Value,
}

impl Keyed for Described {
    fn key(&self) -> &str {
        &self.key
    }
}

struct Watched {
    key: String,
    reads: Cell<usize>,
}

impl Keyed for Watched {
    fn key(&self) -> &str {
        self.reads.set(self.reads.get() + 1);
        &self.key
    }
}

fn picked(grounds: &[String], items: &[Described], unit: f64) -> Option<String> {
    chosen_fixture(&fixture_for(grounds, items), unit).map(|item| item.id.clone())
}

fn shuffled(items: &[Described], order: &[usize]) -> Vec<Described> {
    items.iter().enumerate().map(|(place, item)| Described { id: item.id.clone(), key: item.key.clone(), value: items[order[place]].value.clone(), correct: items[order[place]].correct, answered: items[order[place]].answered.clone() }).collect()
}

fn at(age: Ticks, side: Facing, room: f64, span: f64, unit: f64) -> Lift {
    lift_at(1000, 1000 + age, side, room, span, unit)
}

#[test]
fn every_committed_ground_covers_a_key_as_pythons_strings_say() {
    let vectors = fixture("mischief-choice");
    assert!(entries(&vectors["matches"]).len() > 20);
    for vector in entries(&vectors["matches"]) {
        assert_eq!(Value::Bool(fits(vector["ground"].as_str().unwrap_or_default(), vector["key"].as_str().unwrap_or_default())), vector["expected"], "{}", vector["id"]);
    }
}

#[test]
fn a_ground_covers_a_key_that_is_it_or_continues_it_after_a_slash_over_the_whole_vocabulary() {
    let keys = keys();
    let mut covered = 0;
    for ground in &keys {
        for key in &keys {
            let fitting = fits(ground, key);
            assert_eq!(fitting, key == ground || key.starts_with(&format!("{ground}/")), "{ground} ← {key}");
            covered += usize::from(fitting);
        }
    }
    assert!(covered > keys.len());
    assert_eq!([fits("heating/heating-load", "heating/heating-load-and-demand"), fits("heating/u-values/wall", "heating/u-values/wall-geg"), fits("", "heating"), fits("", ""), fits("heating", "")], [false; 5]);
}

#[test]
fn every_committed_species_keeps_its_fitting_fixtures_in_the_surveys_order() {
    let vectors = fixture("mischief-choice");
    assert!(entries(&vectors["candidates"]).len() > 10);
    for vector in entries(&vectors["candidates"]) {
        let fixtures: Vec<Fixture> = typed(&vector["fixtures"]);
        let ids: Vec<&str> = fixture_for(&strings(&vector["grounds"]), &fixtures).into_iter().map(|fixture| fixture.id.as_str()).collect();
        assert_eq!(ids, strings(&vector["expected"]), "{}", vector["id"]);
    }
}

#[test]
fn the_pick_reads_every_key_once_and_nothing_of_any_candidate() {
    let watched: Vec<Watched> = keys().into_iter().map(|key| Watched { key, reads: Cell::new(0) }).collect();
    let fitting = fixture_for(&["physics".to_string(), "heating/u-values".to_string(), "cooling/powers/sun".to_string()], &watched);
    assert!(!fitting.is_empty());
    assert!(watched.iter().all(|item| item.reads.get() == 1));
    for counter in 0..50 {
        assert!(chosen_fixture(&watched, random_unit(&[SEED, 9, counter])).is_some());
    }
    assert!(watched.iter().all(|item| item.reads.get() == 1));
}

#[test]
fn every_committed_count_and_unit_picks_numpys_position() {
    let vectors = fixture("mischief-choice");
    for vector in entries(&vectors["choices"]) {
        let candidates: Vec<i64> = (0..number(&vector["count"]) as i64).collect();
        let chosen: Vec<f64> = entries(&vector["units"]).iter().map(|unit| chosen_fixture(&candidates, number(unit)).map_or(-1.0, |&place| place as f64)).collect();
        assert_eq!(chosen, entries(&vector["expected"]).iter().map(number).collect::<Vec<_>>(), "{}", vector["id"]);
    }
}

#[test]
fn the_pick_stays_inside_the_list_for_any_unit_and_gives_every_candidate_the_same_chance() {
    assert_eq!(chosen_fixture::<&str>(&[], 0.5), None);
    assert_eq!([f64::NAN, f64::NEG_INFINITY, f64::INFINITY].map(|unit| chosen_fixture(&["a", "b"], unit).copied()), [Some("a"), Some("a"), Some("b")]);
    assert_eq!([-3.0, 0.0, 0.34, 1.0 - 1.0 / 4_294_967_296.0, 1.0, 99.0].map(|unit| chosen_fixture(&["a", "b", "c"], unit).copied()), [Some("a"), Some("a"), Some("b"), Some("c"), Some("c"), Some("c")]);
    let mut counts = [0_u32; 7];
    let draws = 20_000;
    for counter in 0..draws {
        counts[chosen_fixture(&[0, 1, 2, 3, 4, 5, 6], random_unit(&[SEED, 8, counter])).copied().unwrap_or_default()] += 1;
    }
    assert!(counts.iter().all(|&count| (f64::from(count) / f64::from(draws) - 1.0 / 7.0).abs() < 0.35 / f64::from(draws).sqrt() + 0.004), "{counts:?}");
}

#[test]
fn every_committed_host_description_picks_the_committed_fixture_however_its_answers_are_permuted() {
    let vectors = fixture("mischief-choice");
    assert!(entries(&vectors["leaks"]).len() > 6);
    for vector in entries(&vectors["leaks"]) {
        let (grounds, unit): (Vec<String>, f64) = (strings(&vector["grounds"]), number(&vector["unit"]));
        let items: Vec<Described> = typed(&vector["items"]);
        let plain = picked(&grounds, &items, unit);
        assert_eq!(Value::from(plain.clone()), vector["expected"]["plain"], "{}", vector["id"]);
        for (index, shuffle) in entries(&vector["shuffles"]).iter().enumerate() {
            let order: Vec<usize> = entries(shuffle).iter().map(|place| number(place) as usize).collect();
            assert_eq!(picked(&grounds, &shuffled(&items, &order), unit), plain, "{} shuffle {index}", vector["id"]);
        }
    }
}

#[test]
fn every_committed_occasion_gets_numpys_verdict_and_tick() {
    let vectors = fixture("mischief-choice");
    assert!(entries(&vectors["gates"]).len() > 60);
    for vector in entries(&vectors["gates"]) {
        let mut fields = vector.as_object().cloned().unwrap_or_default();
        fields.remove("id");
        fields.remove("expected");
        let circumstances: Circumstances = typed(&Value::Object(fields));
        assert_eq!(Value::Bool(allowed(circumstances)), vector["expected"]["allowed"], "{}", vector["id"]);
        assert_eq!(allowed_from(circumstances).map(|from| from as f64), vector["expected"]["from"].as_f64(), "{}", vector["id"]);
    }
}

#[test]
fn every_gate_alone_shuts_mischief_and_the_patience_and_cooldowns_are_the_designs() {
    assert!(allowed(OPEN));
    let shut = [
        Circumstances { permitted: false, ..OPEN },
        Circumstances { fine: false, ..OPEN },
        Circumstances { lifting: true, ..OPEN },
        Circumstances { mode: PetMode::Still, ..OPEN },
        Circumstances { width: MISCHIEF_WIDTH - 1.0, ..OPEN },
        Circumstances { width: f64::NAN, ..OPEN },
        Circumstances { stirred: OPEN.tick - MISCHIEF_PATIENCE + 1, ..OPEN },
        Circumstances { rested: OPEN.tick - MISCHIEF_COOLDOWN_CALM + 1, ..OPEN },
    ];
    assert!(shut.iter().all(|&circumstances| !allowed(circumstances)));
    let open = [
        Circumstances { width: MISCHIEF_WIDTH, ..OPEN },
        Circumstances { stirred: OPEN.tick - MISCHIEF_PATIENCE, ..OPEN },
        Circumstances { rested: OPEN.tick - MISCHIEF_COOLDOWN_CALM, ..OPEN },
        Circumstances { mode: PetMode::Lively, rested: OPEN.tick - MISCHIEF_COOLDOWN_LIVELY, ..OPEN },
    ];
    assert!(open.iter().all(|&circumstances| allowed(circumstances)));
    assert_eq!((MISCHIEF_WIDTH, MISCHIEF_PATIENCE / 64, MISCHIEF_PATIENCE_QUIET / 64, MISCHIEF_COOLDOWN_CALM / 64, MISCHIEF_COOLDOWN_LIVELY / 64), (1024.0, 12, 30, 180, 45));
    assert_eq!((patience_of(false), patience_of(true), cooldown_of(PetMode::Calm), cooldown_of(PetMode::Lively), cooldown_of(PetMode::Still)), (768, 1920, Some(11520), Some(2880), None));
    for counter in 0..400 {
        let occasion =
            Circumstances { mode: if whole(14, counter, 2) == 0 { PetMode::Calm } else { PetMode::Lively }, quiet: whole(15, counter, 2) == 0, stirred: Ticks::from(whole(16, counter, 30000)), rested: Ticks::from(whole(17, counter, 30000)), ..OPEN };
        let from = allowed_from(occasion).unwrap_or(Ticks::MIN);
        assert_eq!(Some(from), cooldown_of(occasion.mode).map(|cooldown| (occasion.stirred + patience_of(occasion.quiet)).max(occasion.rested + cooldown)));
        assert_eq!([from - 1, from, from + 100_000].map(|tick| allowed(Circumstances { tick, ..occasion })), [false, true, true]);
    }
}

#[test]
fn every_committed_fixture_finds_numpys_station() {
    let vectors = fixture("mischief-choice");
    assert!(entries(&vectors["stations"]).len() > 25);
    for vector in entries(&vectors["stations"]) {
        let station = station_for(&typed(&vector["fixture"]), &typed::<Vec<Pitch>>(&vector["pitches"]), &typed::<Vec<Perch>>(&vector["perches"]), number(&vector["width"]));
        match (station, vector["expected"].is_null()) {
            (None, true) => {}
            (Some(station), false) => {
                let expected: Station = typed(&vector["expected"]);
                assert_eq!((&station.footing, &station.wall, &station.surface, station.x, station.side), (&expected.footing, &expected.wall, &expected.surface, expected.x, expected.side), "{}", vector["id"]);
                assert!((station.y - expected.y).abs() <= 1e-9 && (station.room - expected.room).abs() <= 1e-9, "{}: {station:?} instead of {expected:?}", vector["id"]);
            }
            (station, _) => panic!("{}: {station:?} instead of {}", vector["id"], vector["expected"]),
        }
    }
}

#[test]
fn a_pusher_works_from_the_side_with_room_a_perch_before_a_wall_and_never_without_room() {
    let row = row();
    let (left, right) = (wall("card:left", Facing::Left, 272.0), wall("card:right", Facing::Right, 1168.0));
    assert_eq!(
        station_for(&row, std::slice::from_ref(&left), &[], 1440.0),
        Some(Station { footing: Footing::Wall, wall: Some("card:left".to_string()), surface: "card".to_string(), x: 272.0, y: row.y + row.height, side: Facing::Right, room: 1440.0 - (row.x + row.width) })
    );
    assert_eq!(
        station_for(&row, std::slice::from_ref(&right), &[], 1440.0),
        Some(Station { footing: Footing::Wall, wall: Some("card:right".to_string()), surface: "card".to_string(), x: 1168.0, y: row.y + row.height, side: Facing::Left, room: row.x })
    );
    assert_eq!([1600.0, 1300.0].map(|width| station_for(&row, &[left.clone(), right.clone()], &[], width).map(|station| station.side)), [Some(Facing::Right), Some(Facing::Left)]);
    let shelf = Perch { surface: "shelf".to_string(), x0: 100.0, x1: 270.0, y: 259.2 };
    assert_eq!(station_for(&row, std::slice::from_ref(&left), std::slice::from_ref(&shelf), 1440.0).map(|station| (station.footing, station.x, station.side)), Some((Footing::Perch, 270.0, Facing::Right)));
    assert_eq!(station_for(&row, &[], &[], 1440.0), None);
    assert_eq!(station_for(&row, &[Pitch { x: row.x - STATION_GAP - 1.0, ..left.clone() }], &[], 1440.0), None);
    assert_eq!(station_for(&row, &[Pitch { side: Facing::Right, ..left.clone() }], &[], 1440.0), None);
    assert_eq!(station_for(&row, &[left], &[], row.x + row.width + LIFT_ROOM - 1.0), None);
    assert_eq!(station_for(&row, &[], &[Perch { y: row.y + row.height + STATION_STEP + 1.0, ..shelf.clone() }], 1440.0), None);
    assert_eq!(station_for(&row, &[], &[Perch { y: row.y, ..shelf }], 1440.0), None);
}

#[test]
fn every_committed_lift_follows_numpys_branch_free_path() {
    let vectors = fixture("mischief-choice");
    assert!(entries(&vectors["lifts"]).len() > 5);
    for vector in entries(&vectors["lifts"]) {
        let since = number(&vector["since"]) as Ticks;
        let (side, room, span, unit): (Facing, f64, f64, f64) = (typed(&vector["side"]), number(&vector["room"]), number(&vector["span"]), number(&vector["unit"]));
        let expected = &vector["expected"];
        assert_eq!(lift_ends(since) as f64, number(&expected["ends"]), "{}", vector["id"]);
        let ticks: Vec<Ticks> = (number(&vector["first"]) as Ticks..=number(&vector["last"]) as Ticks).step_by(number(&vector["step"]) as usize).collect();
        assert_eq!(ticks.len(), entries(&expected["dx"]).len(), "{}", vector["id"]);
        for (place, &tick) in ticks.iter().enumerate() {
            let lift = lift_at(since, tick, side, room, span, unit);
            for (field, value) in [("dx", lift.dx), ("dy", lift.dy), ("tilt", lift.tilt), ("opacity", lift.opacity)] {
                assert!((value - number(&expected[field][place])).abs() <= 1e-9, "{} {field} at {tick}: {value} instead of {}", vector["id"], expected[field][place]);
            }
        }
    }
}

#[test]
fn a_lift_lasts_688_ticks_appears_and_vanishes_on_its_element_and_rests_exactly() {
    assert_eq!([LIFT_BRACE, LIFT_SHOVE, LIFT_WOBBLE, LIFT_HOLD, LIFT_RETURN, LIFT_FADE, LIFT_RETURNS, LIFT_TICKS, LIFT_LIMIT], [24, 40, 48, 512, 56, 8, 624, 688, 1280]);
    assert_eq!(lift_ends(1000), 1688);
    assert!([-100, -1, LIFT_TICKS, LIFT_TICKS + 1, 5000].iter().all(|&age| at(age, Facing::Right, 40.0, 883.2, 0.5) == NO_LIFT));
    for age in 0..=LIFT_FADE {
        let share = age as f64 / LIFT_FADE as f64;
        assert_eq!(at(age, Facing::Right, 40.0, 883.2, 0.5), Lift { dx: 0.0, dy: 0.0, tilt: 0.0, opacity: if age == LIFT_FADE { 1.0 } else { share * share * (3.0 - 2.0 * share) } });
    }
    let travel = 40.0 * (LIFT_LEAST + (1.0 - LIFT_LEAST) * 0.5);
    for age in LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE..=LIFT_RETURNS {
        assert_eq!(at(age, Facing::Right, 40.0, 883.2, 0.5), Lift { dx: travel, dy: 0.0, tilt: 0.0, opacity: 1.0 });
    }
    for age in LIFT_TICKS - LIFT_FADE..LIFT_TICKS {
        let lift = at(age, Facing::Right, 40.0, 883.2, 0.5);
        assert_eq!((lift.dx, lift.dy, lift.tilt), (0.0, 0.0, 0.0));
    }
    assert_eq!((at(LIFT_BRACE, Facing::Right, 40.0, 883.2, 0.5).dx, at(LIFT_RETURNS + LIFT_RETURN, Facing::Right, 40.0, 883.2, 0.5).dx), (0.0, 0.0));
}

#[test]
fn a_lift_never_jumps_never_leaves_its_row_and_mirrors_a_shove_to_the_left_exactly() {
    for counter in 0..40 {
        let side = if counter % 2 == 0 { Facing::Right } else { Facing::Left };
        let (room, span, unit) = (f64::from(whole(32, counter, 60)), 20.0 + f64::from(whole(33, counter, 1200)), random_unit(&[SEED, 34, counter]));
        let lean = LIFT_TILT.min(LIFT_RISE / (HALF_TURN_RADIANS * span));
        let mut previous = lift_at(0, -1, side, room, span, unit);
        for tick in 0..=LIFT_TICKS {
            let lift = lift_at(0, tick, side, room, span, unit);
            assert!((lift.dx - previous.dx).abs() <= 0.6 + 1.5 * room / LIFT_SHOVE as f64 && (lift.dy - previous.dy).abs() <= 0.55 && (lift.opacity - previous.opacity).abs() <= 0.2, "{counter} at {tick}");
            assert!(lift.dy.abs() <= LIFT_RISE && lift.tilt.abs() <= lean + 1e-15 && (0.0..=1.0).contains(&lift.opacity), "{counter} at {tick}");
            assert!(lift.dx * side.sign() >= -LIFT_GIVE && lift.dx * side.sign() <= room + LIFT_GIVE, "{counter} at {tick}");
            previous = lift;
        }
    }
    for age in -2..LIFT_TICKS + 2 {
        let (right, left) = (at(age, Facing::Right, 37.0, 410.0, 0.3), at(age, Facing::Left, 37.0, 410.0, 0.3));
        assert_eq!([left.dx + right.dx, left.tilt + right.tilt, left.dy - right.dy, left.opacity - right.opacity], [0.0; 4], "{age}");
    }
}

#[test]
fn a_lift_wakes_the_stage_for_every_tick_the_copy_changes_and_lets_it_sleep_while_it_rests() {
    let mut tick = 990;
    let mut wakes = 0;
    while let Some(next) = lift_wake(1000, tick) {
        assert!(next > tick);
        let here = lift_at(1000, tick, Facing::Right, 40.0, 883.2, 0.5);
        assert!((tick + 1..next).all(|between| lift_at(1000, between, Facing::Right, 40.0, 883.2, 0.5) == here), "{tick} → {next}");
        tick = next;
        wakes += 1;
    }
    assert_eq!((tick, wakes), (lift_ends(1000), 1 + LIFT_TICKS - (LIFT_HOLD - 1)));
    assert_eq!(
        [lift_wake(1000, 1000 + LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE), lift_wake(1000, 1000 + LIFT_RETURNS - 1), lift_wake(1000, 1000 + LIFT_RETURNS), lift_wake(1000, 2000)],
        [Some(1000 + LIFT_RETURNS), Some(1000 + LIFT_RETURNS), Some(1001 + LIFT_RETURNS), None]
    );
}

#[test]
fn every_committed_pusher_is_thrown_as_numpy_says_away_from_the_middle_and_upwards() {
    let vectors = fixture("mischief-choice");
    for vector in entries(&vectors["throws"]) {
        let toss = thrown_off(typed(&vector["pusher"]), &typed(&vector["fixture"]), number(&vector["unit"]));
        assert!((toss.vx - number(&vector["expected"]["vx"])).abs() <= 1e-9 && toss.vy == number(&vector["expected"]["vy"]), "{}: {toss:?}", vector["id"]);
    }
    let row = row();
    let middle = row.x + row.width / 2.0;
    for counter in 0..400 {
        let (x, unit) = (f64::from(whole(35, counter, 1440)), random_unit(&[SEED, 36, counter]));
        let toss = thrown_off(Point { x, y: 259.2 }, &row, unit);
        assert_eq!((toss.vx.signum(), toss.vx.abs(), toss.vy), (if x < middle { -1.0 } else { 1.0 }, THROW_SPEED + THROW_SPREAD * unit, -THROW_LIFT));
    }
    assert_eq!(thrown_off(Point { x: middle, y: 0.0 }, &row, 0.0), Toss { vx: 120.0, vy: -220.0 });
}

#[test]
fn the_constants_are_the_twins_and_the_module_calls_no_platform_transcendental() {
    assert_eq!((HALF_TURN_RADIANS.to_bits(), NO_LIFT), (0x4009_21fb_5444_2d18, Lift { dx: 0.0, dy: 0.0, tilt: 0.0, opacity: 0.0 }));
    assert_eq!([STATION_GAP, STATION_SLACK, STATION_STEP, LIFT_ROOM, LIFT_LEAST, LIFT_GIVE, LIFT_RISE, LIFT_TILT, LIFT_WOBBLES, THROW_SPEED, THROW_SPREAD, THROW_LIFT], [24.0, 2.0, 12.0, 12.0, 0.6, 1.5, 2.0, 0.004, 2.0, 120.0, 80.0, 220.0]);
    let source = include_str!("../../🦀️.rs");
    for call in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", "f64::max", "f64::min", ".max(0.0", ".min(LIFT", "SystemTime", "Instant", "println!", "static "] {
        assert!(!source.contains(call), "{call}");
    }
}
