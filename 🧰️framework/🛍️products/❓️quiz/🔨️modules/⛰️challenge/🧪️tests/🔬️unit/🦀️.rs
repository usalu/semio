//! 🧗️ Unit tests of the challenges: the rule table, points, reach and misses, task seconds, the instant a learner acted and every branch of the hints.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::schema::{Category, ClassificationAnswer, ClassificationItem, MatchingAnswer, SheetClassificationTask, SheetDimension, SheetItem, SheetMatchingTask, SheetSortingTask, SortingAnswer, CHALLENGES, MAX_TIMESTAMP};
use crate::sheet::tests::{classification, matching, quantity, sorting, text};
use std::collections::BTreeMap;
use Verdict::{Over, Reversed, Under};

fn items(ids: &[&str]) -> Vec<SheetItem> {
    ids.iter().map(|id| SheetItem { id: (*id).to_string(), label: text(id), short: None, icon: None }).collect()
}

fn familiar(task: Task, ids: &[&str]) -> Task {
    let Task::Sorting(mut task) = task else { unreachable!() };
    task.items.iter_mut().filter(|item| ids.contains(&item.id.as_str())).for_each(|item| item.familiar = Some(true));
    Task::Sorting(task)
}

fn ladder(scale: Scale, keys: Option<&[f64]>, ids: &[&str]) -> SheetTask {
    SheetTask::Sorting(SheetSortingTask { id: "power".to_string(), title: text("power"), prompt: text("sort"), icon: None, quantity: quantity("W", scale), keys: keys.map(<[f64]>::to_vec), items: items(ids), seconds: None })
}

fn order(ids: &[&str]) -> Answer {
    Answer::Sorting(SortingAnswer { order: ids.iter().map(|id| (*id).to_string()).collect(), guesses: None })
}

fn compare(item: &str, other: &str, dimension: Option<&str>, factor: Option<f64>, difference: Option<f64>, verdict: Verdict) -> Hint {
    Hint::Compare(CompareHint { item: item.to_string(), other: other.to_string(), dimension: dimension.map(str::to_string), factor, difference, verdict })
}

fn ratio(item: &str, other: &str, factor: f64, verdict: Verdict) -> Hint {
    compare(item, other, None, Some(factor), None, verdict)
}

fn offset(item: &str, other: &str, difference: f64, verdict: Verdict) -> Hint {
    compare(item, other, None, None, Some(difference), verdict)
}

fn profiled(item: &str, category: &str, axis: &str) -> Hint {
    Hint::Profile(ProfileHint { item: item.to_string(), category: category.to_string(), axis: axis.to_string(), other: None, above: None })
}

fn against(item: &str, category: &str, axis: &str, other: &str, above: bool) -> Hint {
    Hint::Profile(ProfileHint { item: item.to_string(), category: category.to_string(), axis: axis.to_string(), other: Some(other.to_string()), above: Some(above) })
}

fn grouped(item: &str, other: &str, together: bool) -> Hint {
    Hint::Group(GroupHint { item: item.to_string(), other: other.to_string(), together })
}

fn categorised(item: &str, category: &str) -> Hint {
    Hint::Category(CategoryHint { item: item.to_string(), category: category.to_string() })
}

fn standards(profiles: impl Fn(&str) -> Option<[f64; 3]>) -> (Task, SheetTask) {
    let mut definition = classification("standards", None);
    for category in &mut definition.categories {
        category.profile = profiles(&category.id).map(|[heat, cool, cost]| BTreeMap::from([("heat".to_string(), heat), ("cool".to_string(), cool), ("cost".to_string(), cost)]));
    }
    let sheet = SheetTask::Classification(SheetClassificationTask { id: "standards".to_string(), title: text("standards"), prompt: text("classify"), icon: None, axes: None, categories: definition.categories.clone(), items: items(&["c", "a", "b", "d"]), seconds: None });
    (Task::Classification(definition), sheet)
}

fn authored(id: &str) -> [f64; 3] {
    match id {
        "passive" => [1.0, 2.0, 9.0],
        "low" => [3.0, 3.0, 6.0],
        _ => [10.0, 6.0, 1.0],
    }
}

fn cards(load: Option<&[f64]>, demand: Option<&[f64]>) -> SheetTask {
    let dimension = |id: &str, scale: Scale, cards: Option<&[f64]>| SheetDimension { id: id.to_string(), quantity: quantity(id, scale), icon: None, cards: cards.map(<[f64]>::to_vec) };
    SheetTask::Matching(SheetMatchingTask { id: "buildings".to_string(), title: text("buildings"), prompt: text("match"), icon: None, dimensions: vec![dimension("load", Scale::Linear, load), dimension("demand", Scale::Logarithmic, demand)], items: items(&["m2", "m0", "m1"]), seconds: None })
}

fn assign(dimensions: &[(&str, &[(&str, usize)])]) -> Answer {
    Answer::Matching(MatchingAnswer { assignments: Some(dimensions.iter().map(|(dimension, cards)| ((*dimension).to_string(), cards.iter().map(|(item, card)| ((*item).to_string(), *card)).collect())).collect()), guesses: None })
}

fn classify(pairs: &[(&str, &str)]) -> Answer {
    Answer::Classification(ClassificationAnswer { assignments: pairs.iter().map(|(item, category)| ((*item).to_string(), (*category).to_string())).collect() })
}

#[test]
fn every_challenge_is_the_one_before_plus_one_step() {
    let rules = |keys, hints, timed, par| ChallengeRules { keys, hints, timed, par };
    assert_eq!(challenge_rules(Challenge::Easy), rules(true, true, false, 100.0));
    assert_eq!(challenge_rules(Challenge::Medium), rules(true, false, false, 200.0));
    assert_eq!(challenge_rules(Challenge::Hard), rules(false, false, false, 300.0));
    assert_eq!(challenge_rules(Challenge::Expert), rules(false, false, true, 400.0));
    assert_eq!(CHALLENGES.map(challenge_rules), CHALLENGE_RULES);
    assert_eq!(CHALLENGES.map(challenge_rank), [0, 1, 2, 3]);
    assert_eq!(serde_json::to_value(CHALLENGES).ok(), Some(serde_json::json!(["easy", "medium", "hard", "expert"])));
    assert_eq!(serde_json::to_value(challenge_rules(Challenge::Expert)).ok(), Some(serde_json::json!({"keys": false, "hints": false, "timed": true, "par": 400.0})));
    assert!(serde_json::from_value::<Challenge>(serde_json::json!("extreme")).is_err());
}

#[test]
fn a_challenge_meets_every_challenge_up_to_its_own() {
    for (rank, challenge) in CHALLENGES.into_iter().enumerate() {
        for (least_rank, least) in CHALLENGES.into_iter().enumerate() {
            assert_eq!(challenge_meets(challenge, least), rank >= least_rank, "{challenge:?} meets {least:?}");
        }
    }
}

#[test]
fn points_are_the_score_times_the_par_of_the_challenge() {
    for (challenge, par) in CHALLENGES.into_iter().zip([100.0, 200.0, 300.0, 400.0]) {
        assert_eq!(points(1.0, challenge), par);
        assert_eq!(points(0.0, challenge), 0.0);
        assert_eq!(points(0.87, challenge), 0.87 * par);
    }
    assert_eq!(points(0.5, Challenge::Hard), 150.0);
    assert!(points(0.75, Challenge::Easy) < points(0.75, Challenge::Medium) && points(0.5, Challenge::Expert) > points(0.6, Challenge::Hard));
}

#[test]
fn reach_is_the_root_of_the_ratio_capped_at_a_factor_of_a_thousand_on_a_logarithmic_scale() {
    assert_eq!(REACH_FACTOR, 1000.0);
    assert_eq!(reach(&[1.0, 60.0, 2000.0, 1.0e6, 1.0e9], Scale::Logarithmic), 1000.0);
    assert_eq!(reach(&[1.0, 1.0e7], Scale::Logarithmic), 1000.0);
    assert_eq!(reach(&[1.0, 1.0e6], Scale::Logarithmic), 1000.0);
    assert_eq!(reach(&[10.0, 1000.0], Scale::Logarithmic), 10.0);
    assert_eq!(reach(&[15.0, 90.0, 250.0], Scale::Logarithmic), (250.0f64 / 15.0).sqrt());
    assert_eq!(reach(&[250.0, 15.0, 90.0], Scale::Logarithmic), reach(&[15.0, 90.0, 250.0], Scale::Logarithmic));
    assert_eq!(reach(&[10.0, 12.0, 40.0], Scale::Logarithmic), 2.0);
}

#[test]
fn reach_is_always_half_the_spread_on_a_linear_scale() {
    assert_eq!(reach(&[10.0, 40.0, 120.0], Scale::Linear), 55.0);
    assert_eq!(reach(&[0.0, 1.0e9], Scale::Linear), 5.0e8);
    assert_eq!(reach(&[-3.0, 5.0], Scale::Linear), 4.0);
}

#[test]
fn values_that_do_not_spread_have_the_cap_as_reach() {
    assert_eq!(reach(&[5.0, 5.0, 5.0], Scale::Logarithmic), 1000.0);
    assert_eq!(reach(&[5.0], Scale::Logarithmic), 1000.0);
    assert_eq!(reach(&[], Scale::Logarithmic), 1000.0);
    for values in [&[5.0, 5.0][..], &[7.0], &[], &[0.0, -0.0]] {
        assert_eq!(reach(values, Scale::Linear), f64::INFINITY, "{values:?}");
    }
    assert_eq!(reach(&[f64::NAN, 1.0, 3.0], Scale::Linear), 1.0);
    assert_eq!(reach(&[f64::NAN, 10.0, 1000.0], Scale::Logarithmic), 10.0);
    assert_eq!(reach(&[f64::NAN], Scale::Linear), f64::INFINITY);
    assert_eq!(reach(&[f64::NAN], Scale::Logarithmic), 1000.0);
}

#[test]
fn a_value_misses_beyond_the_reach_of_its_set() {
    assert!(!misses(990.0, 1.0, Scale::Logarithmic, 1000.0));
    assert!(misses(1010.0, 1.0, Scale::Logarithmic, 1000.0));
    assert!(misses(1.0, 1010.0, Scale::Logarithmic, 1000.0));
    assert!(!misses(60.0, 60.0, Scale::Logarithmic, 1.0));
    assert!(!misses(1.0e9, 2.0e6, Scale::Logarithmic, 1000.0));
    assert!(misses(1.0e9, 999_000.0, Scale::Logarithmic, 1000.0));
    assert!(misses(40.0, 10.0, Scale::Logarithmic, reach(&[10.0, 12.0, 40.0], Scale::Logarithmic)));
    assert!(!misses(15.0, 10.0, Scale::Logarithmic, reach(&[10.0, 12.0, 40.0], Scale::Logarithmic)));
    assert!(!misses(65.0, 10.0, Scale::Linear, 55.0));
    assert!(misses(65.5, 10.0, Scale::Linear, 55.0));
    assert!(misses(10.0, 65.5, Scale::Linear, 55.0));
    for (value, truth) in [(1.0e300, -1.0e300), (0.0, 1.0), (f64::MAX, f64::MIN)] {
        assert!(!misses(value, truth, Scale::Linear, f64::INFINITY), "an infinite reach never misses");
    }
    assert!(!misses(f64::MAX, f64::MIN_POSITIVE, Scale::Logarithmic, f64::INFINITY));
    assert!(!misses(f64::NAN, 1.0, Scale::Linear, 0.0));
    assert!(misses(2.0, 1.0, Scale::Linear, 0.0) && !misses(1.0, 1.0, Scale::Linear, 0.0));
}

#[test]
fn an_exact_factor_of_a_thousand_or_of_the_reach_never_misses_whatever_the_digits() {
    for truth in 1..=2000u32 {
        let truth = f64::from(truth);
        assert!(!misses(truth * 1000.0, truth, Scale::Logarithmic, REACH_FACTOR), "{truth} × 1000");
        assert!(!misses(truth, truth * 1000.0, Scale::Logarithmic, REACH_FACTOR), "{} / 1000", truth * 1000.0);
        assert!(!misses(truth / 1000.0, truth, Scale::Logarithmic, REACH_FACTOR), "{truth} / 1000 typed in decimal");
        assert!(misses(truth * 1000.0 * (1.0 + 2e-9), truth, Scale::Logarithmic, REACH_FACTOR), "beyond {truth} × 1000 and its slack");
    }
    assert!(!misses(18000.0, 18.0, Scale::Logarithmic, 1000.0));
    assert!(!misses(18000.000000000004, 18.0, Scale::Logarithmic, 1000.0), "one double beyond lies within the slack");
    assert!(misses(18000.0 * (1.0 + 2e-9), 18.0, Scale::Logarithmic, 1000.0));
    let within = reach(&[18.0, 72.0], Scale::Logarithmic);
    assert_eq!(within, 2.0);
    assert!(!misses(36.0, 18.0, Scale::Logarithmic, within));
    assert!(!misses(36.00000000000001, 18.0, Scale::Logarithmic, within));
    assert!(misses(36.0 * (1.0 + 2e-9), 18.0, Scale::Logarithmic, within));
    assert_eq!(18.0 / 0.018, 1000.0000000000001, "the double 0.018 lies just below 18 / 1000");
    assert!(!misses(0.018, 18.0, Scale::Logarithmic, 1000.0), "a decimal typed at exactly the reach is within");
    assert!(misses(0.0179, 18.0, Scale::Logarithmic, 1000.0));
}

#[test]
fn the_slack_widens_the_reach_by_a_billionth_and_not_one_double_more() {
    assert_eq!(REACH_SLACK, 1e-9);
    for within in [REACH_FACTOR, 2.0, reach(&[3.0, 7.0], Scale::Logarithmic)] {
        let bound = within * (1.0 + REACH_SLACK);
        let beyond = f64::from_bits(bound.to_bits() + 1);
        assert!(!misses(bound, 1.0, Scale::Logarithmic, within) && !misses(1.0, bound, Scale::Logarithmic, within), "a ratio of exactly the widened reach {bound}");
        assert!(misses(beyond, 1.0, Scale::Logarithmic, within) && misses(1.0, beyond, Scale::Logarithmic, within), "the next double {beyond}");
    }
    let bound = 55.0 * (1.0 + REACH_SLACK);
    assert!(!misses(bound, 0.0, Scale::Linear, 55.0) && !misses(0.0, bound, Scale::Linear, 55.0));
    assert!(misses(f64::from_bits(bound.to_bits() + 1), 0.0, Scale::Linear, 55.0));
}

#[test]
fn a_timed_task_allows_a_base_and_a_share_per_item_and_matching_dimension() {
    assert_eq!(TASK_SECONDS, TaskSeconds { base: 30, classification: 8, sorting: 12, matching: 12 });
    assert_eq!(task_seconds(TaskKind::Classification, 4, 1), 62);
    assert_eq!(task_seconds(TaskKind::Sorting, 4, 1), 78);
    assert_eq!(task_seconds(TaskKind::Matching, 3, 2), 102);
    assert_eq!(task_seconds(TaskKind::Matching, 3, 1), 66);
    assert_eq!((task_seconds(TaskKind::Classification, 4, 7), task_seconds(TaskKind::Sorting, 4, 0)), (62, 78));
    for kind in [TaskKind::Classification, TaskKind::Sorting, TaskKind::Matching] {
        assert_eq!(task_seconds(kind, 0, 3), 30);
    }
    assert_eq!(task_seconds(TaskKind::Matching, 5, 0), 30);
}

#[test]
fn the_instant_a_learner_acted_is_lowered_to_the_lead_and_raised_to_its_floor() {
    assert_eq!(acted(150, 100, 100), 150);
    assert_eq!(acted(50, 100, 100), 100);
    assert_eq!(acted(250, 100, 100), 250);
    assert_eq!((acted(100, 100, 0), acted(0, 300, 0)), (100, 300));
    assert_eq!((acted(0, 0, 0), acted(MAX_TIMESTAMP, 0, MAX_TIMESTAMP), acted(0, MAX_TIMESTAMP, 0)), (0, MAX_TIMESTAMP, MAX_TIMESTAMP));
    let now = 1_790_000_000_000;
    assert_eq!(CLOCK_LEAD, 300_000);
    assert_eq!((acted(now + 240_000, 0, now), acted(now + CLOCK_LEAD, 0, now), acted(now + CLOCK_LEAD + 1, 0, now)), (now + 240_000, now + CLOCK_LEAD, now + CLOCK_LEAD));
    assert_eq!((acted(now + 3_600_000, 0, now), acted(MAX_TIMESTAMP, now, now), acted(now + 3_600_000, now + 2 * CLOCK_LEAD, now)), (now + CLOCK_LEAD, now + CLOCK_LEAD, now + 2 * CLOCK_LEAD));
    assert_eq!(acted(u64::MAX, 0, u64::MAX), u64::MAX, "the lead saturates");
}

#[test]
fn nothing_hints_without_an_answer_a_matching_kind_or_shown_keys() {
    let power = Task::Sorting(sorting("power", &[1.0, 60.0, 2000.0, 1.0e6, 1.0e9], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[1.0, 60.0, 2000.0, 1.0e9]), &["s4", "s1", "s2", "s0"]);
    let reversed = order(&["s4", "s2", "s1", "s0"]);
    assert!(!hints_of(&power, &shown, Some(&reversed)).is_empty());
    assert_eq!(hints_of(&power, &shown, None), []);
    assert_eq!(hints_of(&power, &ladder(Scale::Logarithmic, None, &["s4", "s1", "s2", "s0"]), Some(&reversed)), []);
    assert_eq!(hints_of(&power, &shown, Some(&classify(&[("s0", "low")]))), []);
    assert_eq!(hints_of(&power, &cards(Some(&[1.0]), Some(&[1.0])), Some(&reversed)), []);
    assert_eq!(hints_of(&Task::Classification(classification("standards", None)), &shown, Some(&reversed)), []);
}

#[test]
fn a_missed_key_is_questioned_against_the_anchor_whose_claimed_ratio_is_most_wrong() {
    let quadrupled = Task::Sorting(sorting("quadrupled", &[1.0, 4.0, 16.0, 256.0], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[1.0, 4.0, 16.0, 256.0]), &["s2", "s0", "s3", "s1"]);
    assert_eq!(reach(&[1.0, 4.0, 16.0, 256.0], Scale::Logarithmic), 16.0);
    assert_eq!(hints_of(&quadrupled, &shown, Some(&order(&["s3", "s0", "s1", "s2"]))), [ratio("s3", "s2", 1.0 / 256.0, Reversed)], "s2 off by exactly the reach is an anchor, and the claim points the other way");
    let (both, neither) = (familiar(quadrupled.clone(), &["s0", "s1"]), order(&["s3", "s0", "s1", "s2"]));
    assert_eq!(hints_of(&both, &shown, Some(&neither)), [ratio("s3", "s2", 1.0 / 256.0, Reversed)], "familiar anchors outside the tie window do not win");
    assert_eq!(hints_of(&quadrupled, &shown, Some(&order(&["s0", "s1", "s2", "s3"]))), [], "a right answer is questioned nowhere");
}

#[test]
fn anchors_are_preferred_and_tied_anchors_yield_to_the_smallest_claim() {
    let spread = Task::Sorting(sorting("spread", &[1.0, 2.0, 4.0, 1024.0], Scale::Logarithmic, None));
    let keys: &[f64] = &[1.0, 2.0, 4.0, 1024.0];
    let answer = order(&["s0", "s1", "s3", "s2"]);
    assert_eq!(hints_of(&spread, &ladder(Scale::Logarithmic, Some(keys), &["s3", "s1", "s2", "s0"]), Some(&answer)), [ratio("s3", "s1", 2.0, Under), ratio("s2", "s0", 1024.0, Over)], "the other miss, 65536 times off, is passed over for the anchors, tied at 256; s1 named, s2 takes s0");
    assert_eq!(hints_of(&spread, &ladder(Scale::Logarithmic, Some(keys), &["s3", "s0", "s2", "s1"]), Some(&answer)), [ratio("s3", "s1", 2.0, Under), ratio("s2", "s0", 1024.0, Over)], "of the tied anchors the smallest claim wins wherever it stands");
    assert_eq!(hints_of(&spread, &ladder(Scale::Logarithmic, Some(keys), &["s3", "s1", "s2", "s0"]), Some(&order(&["s0", "s1", "s3", "s2", "x"]))), [ratio("s3", "s1", 2.0, Under), ratio("s2", "s0", 1024.0, Over)], "an unknown item beyond the ladder changes nothing");
    assert_eq!(hints_of(&spread, &ladder(Scale::Logarithmic, Some(keys), &["s3", "s1", "s2", "s0"]), Some(&order(&["s0", "s1", "s3", "x"]))), [ratio("s3", "s1", 2.0, Under)], "an unplaced item has no key and is never questioned");
}

#[test]
fn of_the_tied_anchors_a_familiar_one_wins_before_the_smallest_claim() {
    let spread = Task::Sorting(sorting("spread", &[1.0, 2.0, 4.0, 1024.0], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[1.0, 2.0, 4.0, 1024.0]), &["s3", "s1", "s2", "s0"]);
    let answer = order(&["s0", "s1", "s3", "s2"]);
    assert_eq!(hints_of(&familiar(spread.clone(), &["s0"]), &shown, Some(&answer)), [ratio("s3", "s0", 4.0, Under), ratio("s2", "s1", 512.0, Over)], "s0 is familiar, s1 claims less; once s0 is named the unnamed s1 wins");
    assert_eq!(hints_of(&familiar(spread.clone(), &["s0", "s1"]), &shown, Some(&answer)), [ratio("s3", "s1", 2.0, Under), ratio("s2", "s0", 1024.0, Over)], "among familiar anchors the smallest claim wins");
    assert_eq!(hints_of(&familiar(spread, &["s2", "s3"]), &shown, Some(&answer)), [ratio("s3", "s1", 2.0, Under), ratio("s2", "s0", 1024.0, Over)], "a familiar miss is no anchor");
}

#[test]
fn a_reference_no_earlier_hint_of_the_task_names_wins_the_tie_window_first() {
    let appliances = Task::Sorting(sorting("appliances", &[9.0, 40.0, 60.0, 2200.0, 11000.0, 1.6e9], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[9.0, 40.0, 60.0, 2200.0, 11000.0, 1.6e9]), &["s5", "s0", "s3", "s2", "s4", "s1"]);
    assert_eq!(hints_of(&appliances, &shown, Some(&order(&["s0", "s1", "s2", "s3", "s5", "s4"]))), [ratio("s5", "s3", 5.0, Under), ratio("s4", "s2", 1.6e9 / 60.0, Over)], "s3 claims less for s4 too, but s5 names it first");
    let twins = Task::Sorting(sorting("twins", &[9.0, 40.0, 2200.0, 2200.0, 11000.0, 1.6e9], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[9.0, 40.0, 2200.0, 2200.0, 11000.0, 1.6e9]), &["s5", "s0", "s2", "s3", "s4", "s1"]);
    let crossed = order(&["s0", "s1", "s2", "s3", "s5", "s4"]);
    assert_eq!(hints_of(&twins, &shown, Some(&crossed)), [ratio("s5", "s2", 5.0, Under), ratio("s4", "s3", 1.6e9 / 2200.0, Over)], "equal claims: the one not yet named");
    assert_eq!(hints_of(&familiar(twins.clone(), &["s3", "s4"]), &shown, Some(&crossed)), [ratio("s5", "s3", 5.0, Under), ratio("s4", "s2", 1.6e9 / 2200.0, Over)], "an unnamed reference wins over a familiar one already named");
    assert_eq!(hints_of(&twins, &shown, Some(&order(&["s4", "s5", "s0", "s2", "s1", "s3"]))), [ratio("s4", "s1", 9.0 / 11000.0, Reversed), ratio("s5", "s1", 40.0 / 11000.0, Reversed), ratio("s3", "s2", 1.6e9 / 2200.0, Reversed)], "inexact anchors: the window of both keys too low holds s1 alone, named twice");
    let eight = Task::Sorting(sorting("eight", &[9.0, 40.0, 60.0, 2200.0, 2200.0, 11000.0, 3.0e6, 1.6e9], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[9.0, 40.0, 60.0, 2200.0, 2200.0, 11000.0, 3.0e6, 1.6e9]), &["s7", "s0", "s3", "s4", "s5", "s1", "s2", "s6"]);
    assert_eq!(hints_of(&eight, &shown, Some(&order(&["s0", "s1", "s2", "s3", "s6", "s7", "s4", "s5"]))), [ratio("s6", "s3", 1.0, Under), ratio("s7", "s2", 11000.0 / 60.0, Under), ratio("s5", "s1", 1.6e9 / 40.0, Over)], "s4's hint is not given, so s1, which it would name, stays free for s5");
    let buildings = Task::Matching(matching("buildings", &[(0.0, 1.0), (10.0, 1.0e3), (20.0, 1.0e6), (31.0, 1.0e9)], None));
    let dimension = |id: &str, scale: Scale, cards: &[f64]| SheetDimension { id: id.to_string(), quantity: quantity(id, scale), icon: None, cards: Some(cards.to_vec()) };
    let shown = SheetTask::Matching(SheetMatchingTask { id: "buildings".to_string(), title: text("buildings"), prompt: text("match"), icon: None, dimensions: vec![dimension("load", Scale::Linear, &[0.0, 10.0, 20.0, 31.0]), dimension("demand", Scale::Logarithmic, &[1.0, 1.0e3, 1.0e6, 1.0e9])], items: items(&["m0", "m1", "m2", "m3"]), seconds: None });
    let mirrored = assign(&[("load", &[("m0", 3), ("m1", 1), ("m2", 2), ("m3", 0)]), ("demand", &[("m0", 3), ("m1", 1), ("m2", 2), ("m3", 0)])]);
    assert_eq!(hints_of(&buildings, &shown, Some(&mirrored)), [compare("m0", "m2", Some("load"), None, Some(11.0), Reversed), compare("m0", "m2", Some("demand"), Some(1.0e9 / 1.0e6), None, Reversed), compare("m3", "m1", Some("demand"), Some(1.0 / 1.0e3), None, Reversed)], "the two demand misses and the first load miss by error; m2, named in the load, is still free in the demand");
}

#[test]
fn errors_within_the_slack_tie_to_the_smallest_claim_then_the_first_in_sheet_order() {
    let tanks = |apart: f64| (Task::Sorting(sorting("tanks", &[0.0, 1000.0, 3000.0, 3000.0 + apart, 4000.0], Scale::Linear, None)), ladder(Scale::Linear, Some(&[0.0, 1000.0, 3000.0, 3000.0 + apart, 4000.0]), &["s2", "s4", "s1", "s0", "s3"]));
    let answer = order(&["s4", "s1", "s3", "s2", "s0"]);
    let (within, shown) = tanks(1.0e-6);
    assert_eq!(hints_of(&within, &shown, Some(&answer)), [offset("s4", "s1", -1000.0, Reversed), offset("s0", "s2", 4000.0 - (3000.0 + 1.0e-6), Reversed)], "errors a millionth apart in 4000 tie: the smallest claimed difference wins");
    let (beyond, shown) = tanks(1.0e-5);
    assert_eq!(hints_of(&beyond, &shown, Some(&answer)), [offset("s4", "s2", -(3000.0 + 1.0e-5), Reversed), offset("s0", "s3", 1000.0, Reversed)], "errors ten millionths apart in 4000 do not tie: the largest error wins");
    let twins = Task::Sorting(sorting("twins", &[1.0, 8.0, 8.0, 8192.0], Scale::Logarithmic, None));
    let keys: &[f64] = &[1.0, 8.0, 8.0, 8192.0];
    let swapped = order(&["s3", "s1", "s2", "s0"]);
    assert_eq!(hints_of(&twins, &ladder(Scale::Logarithmic, Some(keys), &["s2", "s0", "s1", "s3"]), Some(&swapped)), [ratio("s3", "s2", 0.125, Reversed), ratio("s0", "s1", 1024.0, Reversed)], "equal claims: the first in sheet order, then the one not yet named");
    assert_eq!(hints_of(&twins, &ladder(Scale::Logarithmic, Some(keys), &["s1", "s0", "s2", "s3"]), Some(&swapped)), [ratio("s3", "s1", 0.125, Reversed), ratio("s0", "s2", 1024.0, Reversed)]);
}

#[test]
fn without_anchors_every_other_keyed_item_is_a_candidate_and_without_one_nothing_hints() {
    let pair = Task::Sorting(sorting("pair", &[1.0, 1024.0], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[1.0, 1024.0]), &["s1", "s0"]);
    assert_eq!(hints_of(&pair, &shown, Some(&order(&["s1", "s0"]))), [ratio("s1", "s0", 1.0 / 1024.0, Reversed), ratio("s0", "s1", 1024.0, Reversed)]);
    assert_eq!(hints_of(&pair, &shown, Some(&order(&["s1"]))), [], "a lone key has nothing to be compared with");
}

#[test]
fn the_truth_further_out_than_the_claim_is_under_in_both_directions() {
    let wide = Task::Sorting(sorting("wide", &[1.0, 2048.0, 4096.0], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[1.0, 2048.0, 4096.0]), &["s2", "s1", "s0"]);
    assert_eq!(hints_of(&wide, &shown, Some(&order(&["s1", "s0", "s2"]))), [ratio("s1", "s2", 1.0 / 4096.0, Over), ratio("s0", "s2", 0.5, Under)], "s0 claimed half of s2 is only a 4096th of it; s1 claimed a 4096th of s2 is half of it");
    let linear = Task::Sorting(sorting("linear", &[0.0, 10.0, 20.0, 100.0], Scale::Linear, None));
    let shown = ladder(Scale::Linear, Some(&[0.0, 10.0, 20.0, 100.0]), &["s3", "s2", "s1", "s0"]);
    assert_eq!(hints_of(&linear, &shown, Some(&order(&["s0", "s1", "s3", "s2"]))), [offset("s3", "s1", 10.0, Under), offset("s2", "s0", 100.0, Over)], "differences on a linear scale, the anchors tied at 80");
    let below = Task::Sorting(sorting("below", &[0.0, 60.0, 100.0], Scale::Linear, None));
    let shown = ladder(Scale::Linear, Some(&[0.0, 60.0, 100.0]), &["s0", "s1", "s2"]);
    assert_eq!(hints_of(&below, &shown, Some(&order(&["s1", "s0", "s2"]))), [offset("s1", "s2", -100.0, Over), offset("s0", "s2", -40.0, Under)], "40 below is claimed where 100 below is true");
    let tied = Task::Sorting(sorting("tied", &[5.0, 5.0], Scale::Linear, None));
    assert_eq!(hints_of(&tied, &ladder(Scale::Linear, Some(&[5.0, 5.0]), &["s1", "s0"]), Some(&order(&["s1", "s0"]))), []);
}

#[test]
fn a_claim_pointing_the_other_way_or_off_an_equal_truth_is_reversed_and_an_equal_claim_never_is() {
    let pair = Task::Sorting(sorting("pair", &[1.0, 1024.0], Scale::Logarithmic, None));
    assert_eq!(hints_of(&pair, &ladder(Scale::Logarithmic, Some(&[1.0, 1024.0]), &["s0", "s1"]), Some(&order(&["s1", "s0"]))), [ratio("s1", "s0", 1.0 / 1024.0, Reversed), ratio("s0", "s1", 1024.0, Reversed)]);
    let level = Task::Sorting(sorting("level", &[1.0, 1024.0, 1024.0], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[1.0, 1024.0, 1024.0]), &["s0", "s1", "s2"]);
    assert_eq!(hints_of(&level, &shown, Some(&order(&["s1", "s0", "s2"]))), [ratio("s1", "s2", 1.0 / 1024.0, Reversed), ratio("s0", "s2", 1.0, Over)], "s1 claimed below its equal is reversed; s0 claimed level with s2, a 1024th of it, is over: an equal claim is never reversed");
    assert_eq!(hints_of(&level, &shown, Some(&order(&["s0", "s2", "s1"]))), [], "equal truths swapped are no miss");
    let span = Task::Sorting(sorting("span", &[0.0, 0.0, 100.0], Scale::Linear, None));
    let shown = ladder(Scale::Linear, Some(&[0.0, 0.0, 100.0]), &["s0", "s1", "s2"]);
    assert_eq!(hints_of(&span, &shown, Some(&order(&["s2", "s0", "s1"]))), [offset("s2", "s0", 0.0, Under), offset("s1", "s0", 100.0, Reversed)], "s2 claimed level with s0 lies 100 above it; s1 claimed 100 above its equal is reversed");
    assert_eq!(hints_of(&span, &shown, Some(&order(&["s2", "s1", "s0"]))), [offset("s2", "s1", 0.0, Under), offset("s0", "s1", 100.0, Reversed)]);
    let verdicts = [(2.0, 4.0, Under), (2.0, 1.5, Over), (2.0, 1.0, Reversed), (2.0, 0.5, Reversed), (0.5, 0.25, Under), (0.5, 0.75, Over), (0.5, 1.0, Reversed), (1.0, 2.0, Under), (1.0, 0.5, Over), (1.0, 1.0, Over)];
    for (claim, truth, verdict) in verdicts {
        assert_eq!((verdict_of(claim, truth, 1.0), verdict_of(claim - 1.0, truth - 1.0, 0.0)), (verdict, verdict), "{claim} against {truth}");
    }
}

#[test]
fn a_matched_item_is_questioned_per_dimension_in_sheet_order() {
    let buildings = Task::Matching(matching("buildings", &[(10.0, 15.0), (40.0, 90.0), (120.0, 250.0)], None));
    let shown = cards(Some(&[40.0, 120.0, 10.0]), Some(&[250.0, 15.0, 90.0]));
    let swapped = assign(&[("load", &[("m0", 1), ("m1", 0), ("m2", 2)]), ("demand", &[("m0", 0), ("m1", 2), ("m2", 1)])]);
    let demand = [compare("m2", "m1", Some("demand"), Some(15.0 / 90.0), None, Reversed), compare("m0", "m1", Some("demand"), Some(250.0 / 90.0), None, Reversed)];
    assert_eq!(hints_of(&buildings, &shown, Some(&swapped)), [compare("m2", "m1", Some("load"), None, Some(-30.0), Reversed), compare("m0", "m1", Some("load"), None, Some(80.0), Reversed), demand[0].clone()], "the cap keeps the three largest errors, the first of equals, in their order");
    let near = assign(&[("load", &[("m0", 0), ("m1", 2), ("m2", 1)]), ("demand", &[("m0", 1), ("m2", 0)])]);
    assert_eq!(hints_of(&buildings, &shown, Some(&near)), [], "30 beside a reach of 55 and correct cards hint nothing");
    let partial = assign(&[("demand", &[("m1", 1), ("m0", 9)]), ("area", &[("m0", 0)])]);
    assert_eq!(hints_of(&buildings, &shown, Some(&partial)), [], "a lone card in range has nothing to be compared with; unknown dimensions and cards out of range give no hint");
    assert_eq!(hints_of(&buildings, &cards(None, Some(&[250.0, 15.0, 90.0])), Some(&swapped)), demand);
    assert_eq!(hints_of(&buildings, &cards(None, None), Some(&swapped)), []);
    let guessed = Answer::Matching(MatchingAnswer { assignments: None, guesses: Some(BTreeMap::from([("load".to_string(), BTreeMap::from([("m0".to_string(), 1.0e6)]))])) });
    assert_eq!(hints_of(&buildings, &shown, Some(&guessed)), []);
}

#[test]
fn a_far_off_profile_names_the_axis_with_the_largest_gap_relative_to_its_reach() {
    let (task, sheet) = standards(|id| Some(authored(id)));
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "old"), ("b", "low"), ("c", "low"), ("d", "low")]))), [profiled("c", "low", "heat"), against("a", "old", "heat", "b", true)], "in sheet order; a is off by twice the reach on every axis and takes the first, claimed above b; c claimed level with b names no anchor");
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "low"), ("b", "passive")]))), [], "near misses within every axis's reach give no hint");
    let (task, sheet) = standards(|id| Some(if id == "old" { [4.0, 6.0, 1.0] } else { authored(id) }));
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("b", "old")]))), [profiled("b", "old", "cool")], "3 of a reach of 2 beats 5 of 4 and 1 of 1.5");
    let (task, sheet) = standards(|id| {
        let [heat, _, cost] = authored(id);
        Some([heat, 4.0, cost])
    });
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("d", "old")]))), [profiled("d", "old", "heat")], "an axis without spread takes no part");
    let (task, sheet) = standards(|id| Some(authored(id)));
    let SheetTask::Classification(mut two) = sheet else { unreachable!() };
    two.categories.retain(|category| category.id != "old");
    assert_eq!(hints_of(&task, &SheetTask::Classification(two), Some(&classify(&[("b", "passive")]))), [profiled("b", "passive", "heat")], "the reach spans the presented categories only");
}

#[test]
fn a_far_off_profile_is_questioned_against_an_anchor_on_the_other_side_of_its_own() {
    let (task, sheet) = standards(|id| Some(authored(id)));
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("c", "passive"), ("a", "passive"), ("b", "low"), ("d", "low")]))), [against("c", "passive", "heat", "b", false)], "c claimed below b at 1 lies above it at 10; a at its own 1 is level with the claim");
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("c", "low"), ("a", "passive")]))), [profiled("c", "low", "heat")], "a lies below both the claim and the truth: no anchor on the other side");
    let Task::Classification(mut definition) = task else { unreachable!() };
    definition.categories.push(Category { id: "mid".to_string(), label: text("mid"), short: None, icon: None, description: None, profile: Some(BTreeMap::from([("heat".to_string(), 6.0), ("cool".to_string(), 4.0), ("cost".to_string(), 4.0)])) });
    definition.items.push(ClassificationItem { id: "e".to_string(), label: text("e"), short: None, icon: None, category: "mid".to_string(), explanation: None });
    let sheet = SheetTask::Classification(SheetClassificationTask { id: "standards".to_string(), title: text("standards"), prompt: text("classify"), icon: None, axes: None, categories: definition.categories.clone(), items: items(&["e", "c", "d", "b", "a"]), seconds: None });
    let task = Task::Classification(definition);
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("c", "passive"), ("e", "mid"), ("d", "low"), ("b", "low")]))), [against("c", "passive", "heat", "d", false)], "the anchor farthest from the truth first (d and b at 7 before e at 4), then sheet order");
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("c", "low"), ("e", "mid"), ("d", "low"), ("b", "low")]))), [against("c", "low", "heat", "e", false)], "only e at 6 lies between the claim 3 and the truth 10");
}

#[test]
fn a_task_gives_at_most_three_hints_those_with_the_largest_error() {
    assert_eq!(HINTS_PER_TASK, 3);
    let decades = Task::Sorting(sorting("decades", &[1.0, 1.0e3, 1.0e6, 1.0e9, 1.0e12], Scale::Logarithmic, None));
    let shown = ladder(Scale::Logarithmic, Some(&[1.0, 1.0e3, 1.0e6, 1.0e9, 1.0e12]), &["s2", "s0", "s4", "s1", "s3"]);
    assert_eq!(hints_of(&decades, &shown, Some(&order(&["s4", "s3", "s2", "s1", "s0"]))), [ratio("s4", "s2", 1.0 / 1.0e6, Reversed), ratio("s3", "s2", 1.0e3 / 1.0e6, Reversed), ratio("s0", "s2", 1.0e12 / 1.0e6, Reversed)], "s4 and s0 off by 10¹², then s3 before s1 tied at 10⁶; kept in the learner's order");
    let (task, sheet) = standards(|id| Some(authored(id)));
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("c", "passive"), ("a", "old"), ("b", "old"), ("d", "old")]))), [profiled("c", "passive", "heat"), profiled("a", "old", "heat"), profiled("b", "old", "heat")], "c and a at twice their reach before b and d at 7 of 4.5; the first of equals stays");
    let (task, sheet) = standards(|id| (id != "old").then(|| authored(id)));
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "old"), ("b", "passive"), ("c", "low"), ("d", "passive")]))), [categorised("c", "low"), profiled("b", "passive", "heat"), profiled("d", "passive", "heat")], "profile hints first, then the first of the hints without an error");
}

#[test]
fn without_profiles_a_misplaced_item_is_questioned_against_another_item_or_its_category() {
    let (task, sheet) = standards(|_| None);
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "low"), ("b", "low"), ("c", "old"), ("d", "low")]))), [grouped("a", "b", true)], "a beside b although their own categories differ");
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "old"), ("b", "low"), ("c", "old"), ("d", "passive")]))), [grouped("a", "c", true), grouped("d", "b", false)], "d apart from b although both are low");
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "old"), ("d", "passive")]))), [categorised("a", "old"), categorised("d", "passive")], "nobody beside it and none of its own placed elsewhere");
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "passive"), ("b", "low"), ("c", "old"), ("d", "low")]))), []);
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[]))), []);
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "attic"), ("b", "attic")]))), [], "an unknown category gives no hint");
    let (task, sheet) = standards(|id| (id != "old").then(|| authored(id)));
    assert_eq!(hints_of(&task, &sheet, Some(&classify(&[("a", "old"), ("b", "passive"), ("c", "low"), ("d", "low")]))), [grouped("c", "d", true), categorised("a", "old"), profiled("b", "passive", "heat")], "a profile missing on either side asks without one; a category without one gives no reach, so b is far off beside the two that have one");
    let Task::Classification(definition) = &task else { unreachable!() };
    let drawn = SheetTask::Classification(SheetClassificationTask { id: "standards".to_string(), title: text("standards"), prompt: text("classify"), icon: None, axes: None, categories: definition.categories.clone(), items: items(&["a", "ghost"]), seconds: None });
    assert_eq!(hints_of(&task, &drawn, Some(&classify(&[("a", "old"), ("c", "old"), ("ghost", "old")]))), [categorised("a", "old")], "only presented items of the task are questioned or questioned against");
}

#[test]
fn shared_challenge_rules_of_the_python_numpy_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    let vectors = fixture("challenge-rules");
    let challenges: Vec<Challenge> = entries(&vectors["rules"]["challenges"]).iter().map(typed).collect();
    assert_eq!(challenges, CHALLENGES);
    let expected = &vectors["rules"]["expected"];
    for challenge in CHALLENGES {
        let name = json(&challenge).as_str().unwrap_or_default().to_string();
        assert_close(&format!("rules/table/{name}"), &json(&challenge_rules(challenge)), &expected["table"][&name]);
        assert_eq!(json(&challenge_rank(challenge)), expected["ranks"][&name], "rules/ranks/{name}");
        for least in CHALLENGES {
            let other = json(&least).as_str().unwrap_or_default().to_string();
            assert_eq!(json(&challenge_meets(challenge, least)), expected["meets"][&name][&other], "rules/meets/{name}/{other}");
        }
    }
    for vector in entries(&vectors["points"]) {
        assert_close(&format!("points/{}", vector["id"]), &json(&points(typed(&vector["score"]), typed(&vector["challenge"]))), &vector["expected"]);
    }
    for vector in entries(&vectors["reaches"]) {
        let values: Vec<f64> = typed(&vector["values"]);
        assert_close(&format!("reaches/{}", vector["id"]), &json(&reach(&values, typed(&vector["scale"]))), &vector["expected"]);
    }
    for vector in entries(&vectors["misses"]) {
        let (values, scale): (Vec<f64>, Scale) = (typed(&vector["values"]), typed(&vector["scale"]));
        let within = reach(&values, scale);
        let produced = serde_json::json!({"reach": json(&within), "miss": misses(typed(&vector["value"]), typed(&vector["truth"]), scale, within)});
        assert_close(&format!("misses/{}", vector["id"]), &produced, &vector["expected"]);
    }
    for vector in entries(&vectors["seconds"]) {
        let count = |member: &str| vector[member].as_u64().and_then(|count| usize::try_from(count).ok()).unwrap_or_else(|| panic!("{}", vector[member]));
        assert_eq!(json(&task_seconds(typed(&vector["kind"]), count("items"), count("dimensions"))), vector["expected"], "seconds/{}", vector["id"]);
    }
    for vector in entries(&vectors["instants"]) {
        let instant = |member: &str| vector[member].as_u64().unwrap_or_else(|| panic!("{}", vector[member]));
        assert_eq!(json(&acted(instant("at"), instant("floor"), instant("now"))), vector["expected"], "instants/{}", vector["id"]);
    }
    let tasks: Vec<Task> = entries(&vectors["tasks"]).iter().map(typed).collect();
    assert!(!entries(&vectors["hints"]).is_empty());
    for vector in entries(&vectors["hints"]) {
        let task = tasks.iter().find(|task| Some(task.id().as_str()) == vector["task"].as_str()).unwrap_or_else(|| panic!("{}", vector["task"]));
        let answer: Option<Answer> = vector.get("answer").map(typed);
        assert_close(&format!("hints/{}", vector["id"]), &json(&hints_of(task, &typed(&vector["sheetTask"]), answer.as_ref())), &vector["expected"]);
    }
}

#[test]
fn hints_use_their_wire_shapes() {
    assert_eq!(serde_json::to_value(crate::schema::VERDICTS).ok(), Some(serde_json::json!(["under", "over", "reversed"])));
    let shapes = [
        (ratio("s4", "s1", 0.5, Under), serde_json::json!({"kind": "compare", "item": "s4", "other": "s1", "factor": 0.5, "verdict": "under"})),
        (ratio("s4", "s1", 4.0, Over), serde_json::json!({"kind": "compare", "item": "s4", "other": "s1", "factor": 4.0, "verdict": "over"})),
        (compare("m0", "m1", Some("load"), None, Some(-30.0), Reversed), serde_json::json!({"kind": "compare", "item": "m0", "other": "m1", "dimension": "load", "difference": -30.0, "verdict": "reversed"})),
        (profiled("a", "old", "heat"), serde_json::json!({"kind": "profile", "item": "a", "category": "old", "axis": "heat"})),
        (against("a", "old", "heat", "b", false), serde_json::json!({"kind": "profile", "item": "a", "category": "old", "axis": "heat", "other": "b", "above": false})),
        (grouped("a", "b", false), serde_json::json!({"kind": "group", "item": "a", "other": "b", "together": false})),
        (categorised("a", "old"), serde_json::json!({"kind": "category", "item": "a", "category": "old"})),
    ];
    for (hint, wire) in shapes {
        assert_eq!(serde_json::to_value(&hint).ok(), Some(wire.clone()));
        assert_eq!(serde_json::from_value::<Hint>(wire).ok(), Some(hint));
    }
    for refused in [
        serde_json::json!({"kind": "compare", "item": "s4", "other": "s1", "factor": 2.0}),
        serde_json::json!({"kind": "compare", "item": "s4", "other": "s1", "factor": 2.0, "under": true}),
        serde_json::json!({"kind": "compare", "item": "s4", "other": "s1", "factor": 2.0, "verdict": "wrong"}),
        serde_json::json!({"kind": "compare", "item": "s4", "factor": 2.0, "verdict": "under"}),
        serde_json::json!({"kind": "profile", "item": "a", "category": "old", "axis": "heat", "above": "yes"}),
        serde_json::json!({"kind": "profile", "item": "a", "category": "old"}),
        serde_json::json!({"kind": "group", "item": "a", "other": "b", "together": true, "category": "old"}),
        serde_json::json!({"kind": "magnitude", "item": "s4", "direction": "low"}),
        serde_json::json!({"kind": "misplaced", "count": 1}),
    ] {
        assert!(serde_json::from_value::<Hint>(refused.clone()).is_err(), "{refused}");
    }
}
