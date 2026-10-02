//! 🧠️ Unit tests of the behaviour module: what the modes allow, what an idle pet feels like doing, how long things last, how encounters turn out, how bonds and drives move, who is on stage, the committed vectors of the behavior-choice and bond-dynamics cases, and the arithmetic the twin is allowed to use.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/🧠️behavior-choice/🔣️.json
//! @see ../../../../🧫️fixtures/🤝️bond-dynamics/🔣️.json

use super::*;
use crate::randomness::{random_pick, random_unit};
use crate::schema::tests::{assert_same, entries, fixture, json, number, typed};
use crate::schema::{Bond, Face, Facing, Gait, Gaze, Locomotion, Palette, Repertoire, Size, Text, PET_MODES};
use serde_json::Value;

const TOLERANCE: f64 = 1e-9;
const SECOND: Ticks = TICKS_PER_SECOND;
const SOURCE: &str = include_str!("../../🦀️.rs");
const RESTED: Needs = Needs { energy: 0.9, sociability: 0.5, curiosity: 0.6 };
const WEARY: Needs = Needs { energy: 0.3, sociability: 0.5, curiosity: 0.6 };
const OPEN: Situation = Situation { mode: PetMode::Calm, quiet: false, movers: 0, fidgeters: 0, roam: true, hops: true, crowd: 0, watched: false };
const EVEN: Temperament = Temperament { energy: 0.5, sociability: 0.5, curiosity: 0.5 };
const HALF: Needs = Needs { energy: 0.5, sociability: 0.5, curiosity: 0.5 };

fn feeling(needs: Needs) -> Actor {
    Actor {
        species: "blob".to_string(),
        perch: None,
        x: 0.0,
        y: 0.0,
        vx: 0.0,
        vy: 0.0,
        facing: Facing::Right,
        faced: 0,
        activity: Activity::Idle,
        since: 0,
        until: 0,
        goal: 0.0,
        partner: None,
        clip: None,
        gaze: Gaze { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0 },
        blink: 0,
        mood: 0.0,
        needs,
        opacity: 1.0,
        leaving: false,
        draws: 0,
    }
}

fn kind(fidgets: bool) -> Species {
    let text = || Text { en: "Blob".to_string(), de: "Klecks".to_string() };
    Species {
        json_schema: None,
        id: "blob".to_string(),
        name: text(),
        thing: text(),
        grounds: Vec::new(),
        size: Size { width: 40.0, height: 40.0 },
        palette: Palette { body: "#000000".to_string(), accent: "#000000".to_string(), detail: "#000000".to_string() },
        bones: Vec::new(),
        parts: Vec::new(),
        face: Face { eyes: Vec::new(), mouth: None, above: None },
        clips: Vec::new(),
        repertoire: Repertoire { fidget: fidgets.then(|| vec!["fidget".to_string()]), ..Repertoire::default() },
        locomotion: Locomotion { gait: Gait::Walk, speed: 40.0, hover: None },
        temperament: EVEN,
    }
}

fn bonded(bonds: Vec<Bond>) -> Menagerie {
    Menagerie { json_schema: None, schema: crate::schema::MENAGERIE_SCHEMA.to_string(), id: "bonds".to_string(), title: Text { en: "Bonds".to_string(), de: "Bande".to_string() }, species: Vec::new(), bonds, casts: Vec::new() }
}

fn bond(a: &str, b: &str, affinity: f64) -> Bond {
    Bond { between: [a.to_string(), b.to_string()], affinity }
}

fn rapport(a: &str, b: &str, drift: f64) -> Rapport {
    Rapport { between: [a.to_string(), b.to_string()], drift }
}

fn weight_of(weights: &[f64], activity: Activity) -> f64 {
    weights[activity as usize]
}

fn weighed(vector: &Value) -> [f64; ACTIVITIES.len()] {
    let count = |field: &str| typed::<usize>(&vector[field]);
    let flag = |field: &str| typed::<bool>(&vector[field]);
    activity_weights(
        &feeling(typed(&vector["needs"])),
        &kind(flag("fidgets")),
        Situation { mode: typed(&vector["mode"]), quiet: flag("quiet"), movers: count("movers"), fidgeters: count("fidgeters"), roam: flag("roam"), hops: flag("hops"), crowd: count("crowd"), watched: flag("watched") },
    )
}

fn group<'a>(vectors: &'a Value, name: &str) -> &'a [Value] {
    let listed = entries(&vectors[name]);
    assert!(!listed.is_empty(), "the fixture carries no {name}");
    listed
}

fn id(vector: &Value) -> &str {
    vector["id"].as_str().unwrap_or_else(|| panic!("vector without id: {vector}"))
}

fn near(context: &str, produced: f64, expected: f64) {
    assert!((produced - expected).abs() <= TOLERANCE, "{context}: {produced} ≠ {expected}");
}

fn near_needs(context: &str, produced: Needs, expected: &Value) {
    near(&format!("{context} energy"), produced.energy, number(&expected["energy"]));
    near(&format!("{context} sociability"), produced.sociability, number(&expected["sociability"]));
    near(&format!("{context} curiosity"), produced.curiosity, number(&expected["curiosity"]));
}

fn home() -> Cast {
    Cast { scene: "home".to_string(), core: ["sunny", "cloudy", "housy"].map(String::from).to_vec(), rotation: ["windy", "boily", "roofy", "insuly"].map(String::from).to_vec() }
}

#[test]
fn tables_are_read_in_activities_order() {
    for (index, activity) in ACTIVITIES.into_iter().enumerate() {
        assert_eq!(activity as usize, index, "{activity:?}");
    }
    assert_eq!(DWELL_LOW.len(), ACTIVITIES.len());
    assert_eq!(ENCOUNTERS, [Activity::Greet, Activity::Cuddle, Activity::Squabble]);
}

#[test]
fn still_allows_nothing_at_all() {
    let still = json(&MODE_LIMITS.still);
    assert_eq!(still.as_object().map(|limits| limits.len()), Some(11));
    for (name, value) in still.as_object().into_iter().flatten() {
        assert!(number(value) == 0.0, "{name}");
    }
}

#[test]
fn calm_lets_one_actor_move_and_lively_two() {
    assert_eq!(MODE_LIMITS.calm.movers, 1);
    assert_eq!(MODE_LIMITS.lively.movers, 2);
    assert_eq!([MODE_LIMITS.calm.idle_low, MODE_LIMITS.calm.idle_high], [6 * SECOND, 20 * SECOND]);
    assert_eq!([MODE_LIMITS.lively.idle_low, MODE_LIMITS.lively.idle_high], [3 * SECOND, 10 * SECOND]);
}

#[test]
fn encounters_keep_their_distance() {
    let (calm, lively) = (MODE_LIMITS[PetMode::Calm], MODE_LIMITS[PetMode::Lively]);
    assert!(calm.encounter_gap >= 90 * SECOND);
    assert!(lively.encounter_gap >= 30 * SECOND);
    assert!(calm.encounter_rate < lively.encounter_rate);
}

#[test]
fn calm_is_calmer_than_lively_in_every_weight() {
    let (calm, lively) = (MODE_LIMITS[PetMode::Calm], MODE_LIMITS[PetMode::Lively]);
    assert!(calm.fidget < lively.fidget);
    assert!(calm.walk < lively.walk);
    assert!(calm.hop < lively.hop);
    assert!(calm.stroll <= lively.stroll);
}

#[test]
fn every_mode_of_the_schema_has_limits_under_its_wire_name() {
    let record = json(&MODE_LIMITS);
    assert_eq!(record.as_object().map(|modes| modes.len()), Some(PET_MODES.len()));
    for mode in PET_MODES {
        let name = json(&mode);
        assert_eq!(record[name.as_str().unwrap_or_default()], json(&MODE_LIMITS[mode]), "{mode:?}");
    }
    for vector in group(&fixture("behavior-choice"), "limits") {
        assert_same(id(vector), &json(&MODE_LIMITS[typed::<PetMode>(&vector["id"])]), &vector["expected"]);
    }
}

#[test]
fn an_idle_pet_only_chooses_idle_fidget_walk_hop_or_sleep() {
    let weights = activity_weights(&feeling(WEARY), &kind(true), OPEN);
    assert_eq!(weights.len(), ACTIVITIES.len());
    for activity in [Activity::Fall, Activity::Land, Activity::Greet, Activity::Cuddle, Activity::Squabble, Activity::Sulk] {
        assert!(weight_of(&weights, activity) == 0.0, "{activity:?}");
    }
    for activity in [Activity::Idle, Activity::Fidget, Activity::Walk, Activity::Hop, Activity::Sleep] {
        assert!(weight_of(&weights, activity) > 0.0, "{activity:?}");
    }
}

#[test]
fn a_rested_pet_in_calm_is_slightly_active() {
    let weights = activity_weights(&feeling(RESTED), &kind(true), OPEN);
    let total: f64 = weights.iter().sum();
    assert!(weight_of(&weights, Activity::Idle) / total > 0.5);
    assert!(weight_of(&weights, Activity::Fidget) > weight_of(&weights, Activity::Walk));
    assert!(weight_of(&weights, Activity::Walk) > weight_of(&weights, Activity::Hop));
    assert!(weight_of(&weights, Activity::Sleep) == 0.0);
    let lively = activity_weights(&feeling(RESTED), &kind(true), Situation { mode: PetMode::Lively, ..OPEN });
    for activity in [Activity::Fidget, Activity::Walk, Activity::Hop] {
        assert!(weight_of(&lively, activity) > weight_of(&weights, activity), "{activity:?}");
    }
}

#[test]
fn a_still_pet_and_a_quiet_pet_rest() {
    let only_idle = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    assert_eq!(activity_weights(&feeling(Needs { energy: 0.0, sociability: 1.0, curiosity: 1.0 }), &kind(true), Situation { mode: PetMode::Still, ..OPEN }), only_idle);
    assert_eq!(activity_weights(&feeling(RESTED), &kind(true), Situation { quiet: true, ..OPEN }), only_idle);
    let tired = activity_weights(&feeling(WEARY), &kind(true), Situation { quiet: true, ..OPEN });
    let awake = activity_weights(&feeling(WEARY), &kind(true), OPEN);
    assert!(weight_of(&tired, Activity::Sleep) > 0.0);
    assert!((weight_of(&tired, Activity::Sleep) - 3.0 * weight_of(&awake, Activity::Sleep)).abs() < 1e-12);
    assert_eq!(tired.iter().filter(|&&weight| weight > 0.0).count(), 2);
}

#[test]
fn budgets_room_reach_and_clips_gate_the_weights() {
    let busy = activity_weights(&feeling(RESTED), &kind(true), Situation { movers: 1, ..OPEN });
    assert!(weight_of(&busy, Activity::Walk) == 0.0 && weight_of(&busy, Activity::Hop) == 0.0);
    assert!(weight_of(&busy, Activity::Fidget) > 0.0);
    assert!(weight_of(&activity_weights(&feeling(RESTED), &kind(true), Situation { fidgeters: 1, ..OPEN }), Activity::Fidget) == 0.0);
    let lively = activity_weights(&feeling(RESTED), &kind(true), Situation { mode: PetMode::Lively, movers: 1, fidgeters: 1, ..OPEN });
    assert!(weight_of(&lively, Activity::Walk) > 0.0 && weight_of(&lively, Activity::Fidget) > 0.0);
    assert!(weight_of(&activity_weights(&feeling(RESTED), &kind(true), Situation { roam: false, ..OPEN }), Activity::Walk) == 0.0);
    assert!(weight_of(&activity_weights(&feeling(RESTED), &kind(true), Situation { hops: false, ..OPEN }), Activity::Hop) == 0.0);
    assert!(weight_of(&activity_weights(&feeling(RESTED), &kind(false), OPEN), Activity::Fidget) == 0.0);
    let mut empty = kind(false);
    empty.repertoire.fidget = Some(Vec::new());
    assert!(weight_of(&activity_weights(&feeling(RESTED), &empty, OPEN), Activity::Fidget) == 0.0);
}

#[test]
fn a_pet_gets_sleepier_the_less_energy_it_has() {
    let sleep = |energy: f64, watched: bool| weight_of(&activity_weights(&feeling(Needs { energy, sociability: 0.5, curiosity: 0.6 }), &kind(true), Situation { watched, ..OPEN }), Activity::Sleep);
    assert!(sleep(1.0, false) == 0.0);
    assert!(sleep(0.6, false) == 0.0);
    assert!(sleep(0.4, false) > 0.0);
    assert!(sleep(0.2, false) > sleep(0.4, false));
    assert!(sleep(0.0, false) == MODE_LIMITS.calm.sleep);
    assert!(sleep(0.0, true) == 0.0);
    let walk = |curiosity: f64| weight_of(&activity_weights(&feeling(Needs { energy: 0.8, sociability: 0.5, curiosity }), &kind(true), OPEN), Activity::Walk);
    assert!(walk(1.0) > walk(0.5) && walk(0.5) > walk(0.0) && walk(0.0) > 0.0);
}

#[test]
fn weights_answer_the_committed_situations() {
    for vector in group(&fixture("behavior-choice"), "weights") {
        let expected = entries(&vector["expected"]);
        let weights = weighed(vector);
        assert_eq!(weights.len(), expected.len(), "{}", id(vector));
        for (index, weight) in weights.into_iter().enumerate() {
            near(&format!("{}[{index}]", id(vector)), weight, number(&expected[index]));
        }
    }
}

#[test]
fn a_weighted_index_picks_in_proportion() {
    assert_eq!(weighted_index(&[1.0, 1.0, 2.0], 0.0), Some(0));
    assert_eq!(weighted_index(&[1.0, 1.0, 2.0], 0.2499), Some(0));
    assert_eq!(weighted_index(&[1.0, 1.0, 2.0], 0.25), Some(1));
    assert_eq!(weighted_index(&[1.0, 1.0, 2.0], 0.5), Some(2));
    assert_eq!(weighted_index(&[1.0, 1.0, 2.0], 0.999_999), Some(2));
    for step in 0..100 {
        let picked = weighted_index(&[-1.0, 2.0, 0.0, 4.0], f64::from(step) / 100.0);
        assert!(picked == Some(1) || picked == Some(3), "{picked:?}");
    }
    assert_eq!(weighted_index(&[], 0.5), None);
    assert_eq!(weighted_index(&[0.0, 0.0], 0.5), None);
    assert_eq!(weighted_index(&[-1.0], 0.5), None);
    assert_eq!(weighted_index(&[f64::NAN, 1.0], 0.5), Some(1));
    assert_eq!(weighted_index(&[1.0, 2.0], 1.5), Some(1));
}

#[test]
fn a_weighted_index_is_what_random_pick_does_with_the_unit_of_its_key() {
    let weights = [0.5, 0.0, 1.25, 3.0, 0.0, 0.125];
    for counter in 0..500 {
        let key = [20_261_002, 3, counter];
        assert_eq!(weighted_index(&weights, random_unit(&key)), random_pick(&key, &weights), "{counter}");
    }
}

#[test]
fn picks_and_decisions_answer_the_committed_vectors() {
    let vectors = fixture("behavior-choice");
    for vector in group(&vectors, "picks") {
        let weights: Vec<f64> = typed(&vector["weights"]);
        let picked: Vec<i64> = entries(&vector["units"]).iter().map(|unit| weighted_index(&weights, number(unit)).map_or(-1, |index| index as i64)).collect();
        assert_eq!(picked, typed::<Vec<i64>>(&vector["expected"]), "{}", id(vector));
    }
    for vector in group(&vectors, "decisions") {
        let weights = weighed(vector);
        let (seed, stream, count): (u32, u32, u32) = (typed(&vector["seed"]), typed(&vector["stream"]), typed(&vector["count"]));
        let decided: Vec<Activity> = (0..count).map(|counter| ACTIVITIES[random_pick(&[seed, stream, counter], &weights).unwrap_or_else(|| panic!("{}: no weight is positive", id(vector)))]).collect();
        assert_eq!(decided, typed::<Vec<Activity>>(&vector["expected"]["activities"]), "{}", id(vector));
        for activity in ACTIVITIES {
            assert_eq!(decided.iter().filter(|&&chosen| chosen == activity).count(), typed::<usize>(&vector["expected"]["counts"][activity.as_str()]), "{} {activity:?}", id(vector));
        }
    }
}

#[test]
fn dwells_stay_inside_their_ranges() {
    for step in 0..200 {
        let unit = f64::from(step) / 200.0;
        assert!((6 * SECOND..20 * SECOND).contains(&dwell_of(Activity::Idle, PetMode::Calm, unit)));
        assert!((3 * SECOND..10 * SECOND).contains(&dwell_of(Activity::Idle, PetMode::Lively, unit)));
    }
    assert_eq!(dwell_of(Activity::Idle, PetMode::Still, 0.7), 0);
    for activity in ENCOUNTERS {
        assert_eq!(dwell_of(activity, PetMode::Calm, 0.0), 2 * SECOND);
        assert_eq!(dwell_of(activity, PetMode::Calm, 0.999_999), 5 * SECOND - 1);
    }
    assert_eq!([dwell_of(Activity::Sulk, PetMode::Calm, 0.0), dwell_of(Activity::Sulk, PetMode::Lively, 0.999_999)], [3 * SECOND, 6 * SECOND - 1]);
    assert_eq!([dwell_of(Activity::Sleep, PetMode::Calm, 0.0), dwell_of(Activity::Sleep, PetMode::Calm, 0.999_999)], [20 * SECOND, 60 * SECOND - 1]);
    assert_eq!(dwell_of(Activity::Land, PetMode::Calm, 0.5), 19);
    for activity in ACTIVITIES {
        for mode in PET_MODES {
            let mut previous = -1;
            for step in 0..64 {
                let dwell = dwell_of(activity, mode, f64::from(step) / 64.0);
                assert!(dwell >= previous, "{activity:?} {mode:?} {step}");
                previous = dwell;
            }
        }
    }
}

#[test]
fn every_activity_has_the_mood_of_the_design() {
    assert_eq!([mood_of(Activity::Cuddle), mood_of(Activity::Greet), mood_of(Activity::Idle), mood_of(Activity::Squabble), mood_of(Activity::Sulk), mood_of(Activity::Sleep)], [1.0, 0.7, 0.3, -0.8, -0.6, 0.1]);
    for activity in ACTIVITIES {
        assert!(mood_of(activity).abs() <= 1.0, "{activity:?}");
    }
}

#[test]
fn dwells_and_moods_answer_the_committed_vectors() {
    let vectors = fixture("behavior-choice");
    for vector in group(&vectors, "dwells") {
        let (activity, mode): (Activity, PetMode) = (typed(&vector["activity"]), typed(&vector["mode"]));
        let dwells: Vec<Ticks> = entries(&vector["units"]).iter().map(|unit| dwell_of(activity, mode, number(unit))).collect();
        assert_eq!(dwells, typed::<Vec<Ticks>>(&vector["expected"]), "{}", id(vector));
    }
    for vector in group(&vectors, "moods") {
        assert!(mood_of(typed(&vector["id"])) == number(&vector["expected"]), "{}", id(vector));
    }
}

#[test]
fn every_activity_reaches_every_other_one() {
    for start in ACTIVITIES {
        let mut seen = vec![start];
        let mut head = 0;
        while head < seen.len() {
            for &follower in followers_of(seen[head]) {
                if !seen.contains(&follower) {
                    seen.push(follower);
                }
            }
            head += 1;
        }
        assert_eq!(seen.len(), ACTIVITIES.len(), "{start:?}");
    }
}

#[test]
fn followers_are_listed_in_activities_order_and_idle_follows_everything() {
    for activity in ACTIVITIES {
        let followers = followers_of(activity);
        let ordered: Vec<Activity> = ACTIVITIES.into_iter().filter(|candidate| followers.contains(candidate)).collect();
        assert_eq!(followers, ordered, "{activity:?}");
        assert!(followers.contains(&Activity::Idle), "{activity:?}");
    }
    let before = |follower: Activity| ACTIVITIES.into_iter().filter(|&activity| followers_of(activity).contains(&follower)).collect::<Vec<_>>();
    assert_eq!(before(Activity::Sulk), [Activity::Squabble]);
    assert_eq!(before(Activity::Land), [Activity::Hop, Activity::Fall]);
}

#[test]
fn the_graph_is_the_committed_graph() {
    let vectors = fixture("behavior-choice");
    let committed = &group(&vectors, "graph")[0]["expected"];
    for activity in ACTIVITIES {
        assert_eq!(json(&followers_of(activity)), committed["followers"][activity.as_str()], "{activity:?}");
    }
    assert_eq!(committed["components"], 1);
}

#[test]
fn encounter_shares_are_a_whole() {
    for step in -100..=100 {
        let shares = encounter_shares(f64::from(step) / 100.0);
        assert!((shares[0] + shares[1] + shares[2] - 1.0).abs() <= 1e-12, "{step}");
        assert!(shares.iter().all(|&share| share >= 0.0), "{step}");
    }
    for affinity in [0.4, 0.6, 0.9, 1.0] {
        let [greet, cuddle, squabble] = encounter_shares(affinity);
        assert!(cuddle > 0.5 && cuddle > greet && squabble == 0.0, "{affinity}");
    }
    for affinity in [-0.3, -0.45, AFFINITY_FLOOR, -1.0] {
        let [greet, cuddle, squabble] = encounter_shares(affinity);
        assert!(squabble > 0.5 && cuddle == 0.0 && greet > 0.0, "{affinity}");
    }
    for affinity in [-0.2, 0.0, 0.2, 0.39] {
        assert!(encounter_shares(affinity)[0] > 0.5, "{affinity}");
    }
    assert!(encounter_shares(0.0)[2] > 0.0 && encounter_shares(0.0)[2] < 0.15);
    assert!(encounter_shares(0.3)[1] > 0.0);
}

#[test]
fn the_kind_of_an_encounter_is_drawn_from_the_shares() {
    let tally = |affinity: f64, kind: Activity| (0..1000).filter(|&step| encounter_of(affinity, (f64::from(step) + 0.5) / 1000.0) == kind).count();
    assert_eq!(tally(0.8, Activity::Cuddle), 820);
    assert_eq!(tally(-0.5, Activity::Squabble), 800);
    assert_eq!(tally(0.0, Activity::Greet), 920);
    assert_eq!(encounter_of(0.8, 0.0), Activity::Greet);
    assert_eq!(encounter_of(-0.5, 0.999), Activity::Squabble);
}

#[test]
fn encounters_answer_the_committed_vectors() {
    for vector in group(&fixture("behavior-choice"), "encounters") {
        let affinity = number(&vector["affinity"]);
        for (index, share) in encounter_shares(affinity).into_iter().enumerate() {
            near(&format!("{}[{index}]", id(vector)), share, number(&vector["expected"]["shares"][index]));
        }
        let kinds: Vec<Encounter> = entries(&vector["units"]).iter().map(|unit| encounter_of(affinity, number(unit))).collect();
        assert_eq!(kinds, typed::<Vec<Encounter>>(&vector["expected"]["kinds"]), "{}", id(vector));
    }
}

#[test]
fn an_affinity_is_the_authored_bond_plus_the_drift() {
    let bonds = || bonded(vec![bond("sunny", "solary", 0.9), bond("sunny", "cloudy", -0.4), bond("solary", "cloudy", -0.7)]);
    assert!(affinity_of(&bonds(), &[], "sunny", "solary") == 0.9);
    assert!(affinity_of(&bonds(), &[], "solary", "sunny") == 0.9);
    assert!(affinity_of(&bonds(), &[], "sunny", "housy") == 0.0);
    assert!(affinity_of(&bonds(), &[rapport("sunny", "sunny", 0.3)], "sunny", "sunny") == 0.0);
    assert!((affinity_of(&bonds(), &[rapport("cloudy", "sunny", 0.1)], "sunny", "cloudy") + 0.3).abs() < 1e-12);
    assert!(affinity_of(&bonds(), &[rapport("sunny", "solary", 0.5)], "sunny", "solary") == 1.0);
    assert!(affinity_of(&bonds(), &[], "solary", "cloudy") == AFFINITY_FLOOR);
    assert!(affinity_of(&bonds(), &[rapport("solary", "cloudy", -0.5)], "cloudy", "solary") == AFFINITY_FLOOR);
    assert!(affinity_of(&bonded(Vec::new()), &[rapport("a", "b", -0.5)], "a", "b") == -0.5);
}

#[test]
fn a_rapport_moves_by_the_step_of_the_encounter_and_nothing_else() {
    assert!(rapport_after(0.0, Activity::Greet) == 0.05);
    assert!(rapport_after(0.0, Activity::Cuddle) == 0.1);
    assert!(rapport_after(0.0, Activity::Squabble) == -0.15);
    assert!((rapport_after(-0.15, Activity::Sulk) + 0.05).abs() < 1e-12);
    for activity in [Activity::Idle, Activity::Fidget, Activity::Walk, Activity::Hop, Activity::Fall, Activity::Land, Activity::Sleep] {
        assert!(rapport_after(0.2, activity) == 0.2, "{activity:?}");
    }
    assert!(rapport_after(0.45, Activity::Cuddle) == RAPPORT_SPAN);
    assert!(rapport_after(-0.45, Activity::Squabble) == -RAPPORT_SPAN);
    let rivals = bonded(vec![bond("a", "b", -0.5)]);
    let mut drift = 0.0;
    for round in 1..=20 {
        drift = rapport_after(rapport_after(drift, Activity::Squabble), Activity::Sulk);
        near(&format!("round {round}"), drift, (-0.05 * f64::from(round)).max(-RAPPORT_SPAN + 0.1));
        assert!(affinity_of(&rivals, &[rapport("a", "b", drift)], "a", "b") >= AFFINITY_FLOOR, "round {round}");
    }
}

#[test]
fn a_rapport_fades_to_nothing() {
    assert!(rapport_faded(0.1, 0) == 0.1);
    assert!((rapport_faded(0.1, 300 * SECOND) - 0.05).abs() < 1e-12);
    assert!(rapport_faded(0.1, 600 * SECOND) == 0.0);
    assert!((rapport_faded(-0.15, 600 * SECOND) + 0.05).abs() < 1e-12);
    assert!(rapport_faded(-0.15, 900 * SECOND) == 0.0);
    assert!(rapport_faded(0.5, 3000 * SECOND + 1) == 0.0);
    assert_eq!(rapport_faded(-0.1, 100_000 * SECOND).to_bits(), 0.0_f64.to_bits());
    for ticks in (0..40_000).step_by(997) {
        assert!(rapport_faded(0.3, ticks + 997).abs() <= rapport_faded(0.3, ticks).abs(), "{ticks}");
    }
}

#[test]
fn bonds_answer_the_committed_vectors() {
    let vectors = fixture("bond-dynamics");
    for vector in group(&vectors, "affinities") {
        let menagerie = bonded(typed(&vector["bonds"]));
        let rapports: Vec<Rapport> = typed(&vector["rapports"]);
        for (index, pair) in entries(&vector["pairs"]).iter().enumerate() {
            let [a, b]: [String; 2] = typed(pair);
            near(&format!("{} {a}–{b}", id(vector)), affinity_of(&menagerie, &rapports, &a, &b), number(&vector["expected"][index]));
        }
    }
    for vector in group(&vectors, "rapportSteps") {
        for activity in ACTIVITIES {
            near(&format!("{} {activity:?}", id(vector)), rapport_after(number(&vector["drift"]), activity), number(&vector["expected"][activity.as_str()]));
        }
    }
    for vector in group(&vectors, "rapportFading") {
        for (index, ticks) in entries(&vector["ticks"]).iter().enumerate() {
            near(&format!("{} {ticks}", id(vector)), rapport_faded(number(&vector["drift"]), typed(ticks)), number(&vector["expected"][index]));
        }
    }
    for vector in group(&vectors, "histories") {
        let menagerie = bonded(vec![bond("a", "b", number(&vector["affinity"]))]);
        let mut drift = 0.0;
        for (index, event) in entries(&vector["events"]).iter().enumerate() {
            drift = rapport_after(rapport_faded(drift, typed(&event["after"])), typed(&event["activity"]));
            near(&format!("{}[{index}] drift", id(vector)), drift, number(&vector["expected"]["drifts"][index]));
            near(&format!("{}[{index}] affinity", id(vector)), affinity_of(&menagerie, &[rapport("a", "b", drift)], "b", "a"), number(&vector["expected"]["affinities"][index]));
        }
    }
}

#[test]
fn a_pet_arrives_rested() {
    assert_eq!(needs_of(Temperament { energy: 0.6, sociability: 0.7, curiosity: 0.2 }), Needs { energy: 0.8, sociability: 0.7, curiosity: 0.2 });
    assert!(needs_of(Temperament { energy: 0.0, sociability: 0.0, curiosity: 0.0 }).energy == 0.5);
    assert!(needs_of(Temperament { energy: 1.0, sociability: 1.0, curiosity: 1.0 }).energy == 1.0);
    let needs = Needs { energy: 0.4, sociability: 0.6, curiosity: 0.8 };
    for activity in ACTIVITIES {
        assert_eq!(needs_after(needs, activity, 0, EVEN), needs, "{activity:?}");
    }
}

#[test]
fn needs_move_with_what_a_pet_does() {
    let energy = |activity: Activity| needs_after(HALF, activity, 10 * SECOND, EVEN).energy;
    assert!(energy(Activity::Idle) < 0.5);
    assert!(energy(Activity::Walk) < energy(Activity::Idle));
    assert!(energy(Activity::Hop) < energy(Activity::Walk));
    assert!((energy(Activity::Sleep) - 0.7).abs() < 1e-12);
    assert!(needs_after(HALF, Activity::Idle, 10 * SECOND, Temperament { energy: 0.9, ..EVEN }).energy > needs_after(HALF, Activity::Idle, 10 * SECOND, Temperament { energy: 0.1, ..EVEN }).energy);
    for activity in ENCOUNTERS {
        assert!((needs_after(HALF, activity, 3 * SECOND, EVEN).sociability - 0.14).abs() < 1e-12, "{activity:?}");
    }
    assert!(needs_after(HALF, Activity::Idle, 60 * SECOND, EVEN).sociability > 0.5);
    for activity in [Activity::Walk, Activity::Fidget, Activity::Hop] {
        assert!(needs_after(HALF, activity, 2 * SECOND, EVEN).curiosity < 0.5, "{activity:?}");
    }
    assert!(needs_after(HALF, Activity::Idle, 10 * SECOND, EVEN).curiosity > 0.5);
    for activity in ACTIVITIES {
        for start in [0.0, 0.5, 1.0] {
            let needs = needs_after(Needs { energy: start, sociability: start, curiosity: start }, activity, 1_000_000, EVEN);
            for need in [needs.energy, needs.sociability, needs.curiosity] {
                assert!((0.0..=1.0).contains(&need), "{activity:?} {start}");
            }
        }
    }
}

#[test]
fn needs_answer_the_committed_vectors() {
    let vectors = fixture("bond-dynamics");
    for vector in group(&vectors, "needs") {
        near_needs(id(vector), needs_after(typed(&vector["needs"]), typed(&vector["activity"]), typed(&vector["ticks"]), typed(&vector["temperament"])), &vector["expected"]);
    }
    for vector in group(&vectors, "days") {
        let temperament: Temperament = typed(&vector["temperament"]);
        let mut needs = needs_of(temperament);
        near_needs(id(vector), needs, &vector["expected"][0]);
        for (index, span) in entries(&vector["spans"]).iter().enumerate() {
            needs = needs_after(needs, typed(&span["activity"]), typed(&span["ticks"]), temperament);
            near_needs(&format!("{}[{index}]", id(vector)), needs, &vector["expected"][index + 1]);
        }
    }
}

#[test]
fn a_cast_shows_its_core_and_fills_up_with_the_rotation() {
    let cast = home();
    let troupe = cast_of(&cast, 5, 0, 7);
    assert_eq!(troupe[..3], cast.core[..]);
    assert_eq!(troupe.len(), 5);
    assert!(troupe[3..].iter().all(|species| cast.rotation.contains(species)));
    assert!(cast_of(&cast, 0, 0, 7).is_empty());
    assert_eq!(cast_of(&cast, 100, 3, 7).len(), 7);
    assert_eq!(cast_of(&cast, 4, 5, 7)[..3], cast.core[..]);
}

#[test]
fn a_visitor_keeps_one_seat_from_two_seats_on_and_the_core_takes_turns_for_the_others() {
    let cast = home();
    let round = (cast.core.len() * cast.rotation.len()) as i64;
    for capacity in [2usize, 3] {
        let mut seen: Vec<Slug> = Vec::new();
        for epoch in 0..round {
            let troupe = cast_of(&cast, capacity, epoch, 7);
            assert_eq!(troupe.len(), capacity);
            for (index, species) in troupe.iter().enumerate() {
                assert!(!troupe[..index].contains(species), "{capacity}/{epoch}: {species} twice");
                assert_eq!(cast.core.contains(species), index + 1 < capacity, "{capacity}/{epoch}: {species}");
                assert_eq!(cast.rotation.contains(species), index + 1 == capacity, "{capacity}/{epoch}: {species}");
                if !seen.contains(species) {
                    seen.push(species.clone());
                }
            }
            assert_eq!(cast_of(&cast, capacity, epoch + round, 7), troupe);
            assert_eq!(cast_of(&cast, capacity, epoch - round, 7), troupe);
        }
        assert_eq!(seen.len(), cast.core.len() + cast.rotation.len());
    }
    assert_eq!(cast_of(&cast, 3, 1, 7)[0], cast_of(&cast, 3, 0, 7)[1]);
    assert_eq!(cast_of(&cast, 1, 0, 7).len(), 1);
    let mut alone: Vec<Slug> = (0..3).map(|epoch| cast_of(&cast, 1, epoch, 7)[0].clone()).collect();
    alone.sort();
    let mut core = cast.core;
    core.sort();
    assert_eq!(alone, core);
}

#[test]
fn a_core_without_a_rotation_gets_every_seat_and_so_does_a_rotation_without_a_core() {
    let alone = Cast { scene: "s".to_string(), core: ["a", "b", "c"].map(String::from).to_vec(), rotation: Vec::new() };
    assert_eq!(cast_of(&alone, 3, 4, 7), ["a", "b", "c"]);
    assert_eq!(cast_of(&alone, 2, 0, 7).len(), 2);
    assert_eq!(cast_of(&alone, 2, 1, 7)[0], cast_of(&alone, 2, 0, 7)[1]);
    let guests = Cast { scene: "s".to_string(), core: Vec::new(), rotation: ["x", "y", "z"].map(String::from).to_vec() };
    let mut everyone = cast_of(&guests, 5, 0, 7);
    everyone.sort();
    assert_eq!(everyone, ["x", "y", "z"]);
    assert_eq!(cast_of(&guests, 1, 0, 7).len(), 1);
}

#[test]
fn a_stage_of_six_shows_five_of_nine_in_turn_and_one_of_eleven_visitors() {
    let home = Cast { scene: "home".to_string(), core: (1..=9).map(|number| format!("a{number}")).collect(), rotation: (1..=11).map(|number| format!("b{number}")).collect() };
    let mut seen: Vec<Slug> = Vec::new();
    for epoch in 0..11 {
        let troupe = cast_of(&home, 6, epoch, 20_261_002);
        assert_eq!(troupe.len(), 6);
        assert_eq!(troupe.iter().filter(|species| home.core.contains(species)).count(), 5);
        assert!(home.rotation.contains(&troupe[5]));
        let next = cast_of(&home, 6, epoch + 1, 20_261_002);
        assert_eq!(next.iter().filter(|species| !troupe.contains(species)).count(), 2);
        for species in troupe {
            if !seen.contains(&species) {
                seen.push(species);
            }
        }
    }
    assert_eq!(seen.len(), 20);
    let words = random_words(&[20_261_002, CAST_STREAM, 0], 2);
    assert_eq!(cast_of(&home, 6, 0, 20_261_002)[0], home.core[words[0] as usize % 9]);
    assert_eq!(cast_of(&home, 6, 0, 20_261_002)[5], home.rotation[words[1] as usize % 11]);
}

#[test]
fn the_rotation_moves_on_by_one_per_epoch() {
    let cast = home();
    let shown = |epoch: i64| cast_of(&cast, 5, epoch, 7)[3..].to_vec();
    for epoch in 0..8 {
        assert_eq!(shown(epoch + 1)[0], shown(epoch)[1]);
        assert_eq!(shown(epoch + cast.rotation.len() as i64), shown(epoch));
    }
    let mut everyone: Vec<Slug> = (0..4).flat_map(shown).collect();
    everyone.sort();
    everyone.dedup();
    assert_eq!(everyone.len(), cast.rotation.len());
    assert_eq!(cast_of(&cast, 5, 2, 99), cast_of(&cast, 5, 2, 99));
    let mut starts: Vec<Slug> = (0..40).map(|seed| cast_of(&cast, 4, 0, seed)[3].clone()).collect();
    starts.sort();
    starts.dedup();
    assert_eq!(starts.len(), cast.rotation.len());
}

#[test]
fn a_cast_lists_a_species_once() {
    let cast = Cast { scene: "s".to_string(), core: ["a", "b", "a"].map(String::from).to_vec(), rotation: ["b", "c", "d", "c"].map(String::from).to_vec() };
    let mut troupe = cast_of(&cast, 6, 1, 3);
    assert_eq!(troupe[..2], ["a", "b"]);
    troupe.sort();
    assert_eq!(troupe, ["a", "b", "c", "d"]);
}

#[test]
fn casts_answer_the_committed_vectors() {
    for vector in group(&fixture("behavior-choice"), "casts") {
        let cast: Cast = typed(&vector["cast"]);
        let troupes: Vec<Vec<Slug>> = entries(&vector["epochs"]).iter().map(|epoch| cast_of(&cast, typed(&vector["capacity"]), typed(epoch), typed(&vector["seed"]))).collect();
        assert_eq!(json(&troupes), vector["expected"], "{}", id(vector));
    }
}

#[test]
fn the_arithmetic_is_what_the_typescript_twin_can_reproduce() {
    for forbidden in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", ".max(", ".min(", "println!", "eprintln!", "dbg!"] {
        assert!(!SOURCE.contains(forbidden), "{forbidden}");
    }
}
