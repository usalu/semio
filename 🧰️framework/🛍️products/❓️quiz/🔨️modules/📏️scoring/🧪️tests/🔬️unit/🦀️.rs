//! 🎳️ Unit tests of the scoring: pair concordance, guesses with misses, partial answers on timed sheets, profile similarity, run means and points, and the shared scores.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::challenge::challenge_rules;
use crate::lifecycle::tests::perfect;
use crate::schema::{Challenge, MatchingAnswer, SheetDimension, SheetItem, SheetSortingTask, CHALLENGES};
use crate::sheet::sheet_of;
use crate::sheet::tests::{classification, matching, quantity, quiz, sorting, text};

const POWER: [f64; 5] = [1.0, 60.0, 2000.0, 1.0e6, 1.0e9];

fn items(ids: &[&str]) -> Vec<SheetItem> {
    ids.iter().map(|id| SheetItem { id: (*id).to_string(), label: text(id), short: None, icon: None }).collect()
}

fn ladder(task: &SortingTask, ids: &[&str]) -> Vec<f64> {
    let mut keys: Vec<f64> = ids.iter().filter_map(|id| task.items.iter().find(|item| item.id == *id).map(|item| item.value)).collect();
    keys.sort_by(f64::total_cmp);
    keys
}

fn sorting_sheet(task: &SortingTask, ids: &[&str], shown: bool, seconds: Option<u64>) -> SheetTask {
    SheetTask::Sorting(SheetSortingTask { id: task.id.clone(), title: task.title.clone(), prompt: task.prompt.clone(), icon: None, quantity: task.quantity.clone(), keys: shown.then(|| ladder(task, ids)), items: items(ids), seconds })
}

fn ordered(ids: &[&str]) -> Answer {
    Answer::Sorting(SortingAnswer { order: ids.iter().map(|id| (*id).to_string()).collect(), guesses: None })
}

fn guessed(guesses: &[(&str, Option<f64>)]) -> Answer {
    Answer::Sorting(SortingAnswer { order: guesses.iter().map(|(id, _)| (*id).to_string()).collect(), guesses: Some(guesses.iter().filter_map(|(id, guess)| guess.map(|guess| ((*id).to_string(), guess))).collect()) })
}

fn sort(task: &SortingTask, order: &[&str]) -> Option<TaskResult> {
    score_task(&Task::Sorting(task.clone()), &sorting_sheet(task, order, true, None), Some(&ordered(order)))
}

fn guess(task: &SortingTask, guesses: &[(&str, Option<f64>)], seconds: Option<u64>) -> Option<TaskResult> {
    let ids: Vec<&str> = guesses.iter().map(|(id, _)| *id).collect();
    score_task(&Task::Sorting(task.clone()), &sorting_sheet(task, &ids, false, seconds), Some(&guessed(guesses)))
}

fn score(result: Option<TaskResult>) -> f64 {
    result.map_or(f64::NAN, |result| result.score())
}

fn pairs(scale: Scale, values: &[f64], discordant: impl Fn(usize, usize) -> f64) -> f64 {
    let (mut total, mut lost) = (0.0, 0.0);
    for i in 0..values.len() {
        for j in i + 1..values.len() {
            let weight = (scaled(scale, values[i]) - scaled(scale, values[j])).abs();
            total += weight;
            lost += weight * discordant(i, j);
        }
    }
    1.0 - lost / total
}

fn sorted_items(result: Option<TaskResult>) -> Vec<(String, usize, Option<f64>, Option<bool>)> {
    match result {
        Some(TaskResult::Sorting { items, .. }) => items.into_iter().map(|item| (item.item, item.position, item.guess, item.miss)).collect(),
        _ => Vec::new(),
    }
}

#[test]
fn sorting_perfect_is_one_and_reversed_is_zero_exactly() {
    let task = sorting("power", &POWER, Scale::Logarithmic, None);
    assert_eq!(score(sort(&task, &["s0", "s1", "s2", "s3", "s4"])), 1.0);
    assert_eq!(score(sort(&task, &["s4", "s3", "s2", "s1", "s0"])), 0.0);
}

#[test]
fn sorting_weighs_pairs_by_scaled_distance() {
    let linear = sorting("linear", &[1.0, 2.0, 3.0], Scale::Linear, None);
    assert_eq!(score(sort(&linear, &["s1", "s0", "s2"])), 0.75);
    let logarithmic = sorting("log", &[1.0, 10.0, 1000.0], Scale::Logarithmic, None);
    assert!((score(sort(&logarithmic, &["s1", "s0", "s2"])) - 5.0 / 6.0).abs() < 1e-12);
    let tied = sorting("tied", &[4.0, 4.0], Scale::Linear, None);
    assert_eq!(score(sort(&tied, &["s1", "s0"])), 1.0);
}

#[test]
fn sorting_costs_little_for_neighbours_and_much_for_extremes() {
    let task = sorting("power", &POWER, Scale::Logarithmic, None);
    let neighbours = score(sort(&task, &["s0", "s1", "s2", "s4", "s3"]));
    let extremes = score(sort(&task, &["s4", "s1", "s2", "s3", "s0"]));
    assert!(neighbours > 0.9 && neighbours < 1.0, "{neighbours}");
    assert!(extremes < 0.2, "{extremes}");
}

#[test]
fn sorting_results_follow_the_learner_order_with_true_ranks() {
    let task = sorting("tie", &[5.0, 1.0, 5.0], Scale::Linear, None);
    let Some(TaskResult::Sorting { items, .. }) = sort(&task, &["s2", "s1", "s0"]) else { unreachable!() };
    let summary: Vec<(&str, usize, usize)> = items.iter().map(|item| (item.item.as_str(), item.position, item.rank)).collect();
    assert_eq!(summary, [("s2", 0, 2), ("s1", 1, 0), ("s0", 2, 1)]);
    assert!(items.iter().all(|item| item.guess.is_none() && item.miss.is_none()), "a sorting that shows the keys carries neither guesses nor misses");
}

#[test]
fn a_sorting_that_shows_the_keys_takes_no_guesses() {
    let task = sorting("power", &POWER, Scale::Logarithmic, None);
    let ids = ["s1", "s0", "s2", "s4", "s3"];
    let sheet = sorting_sheet(&task, &ids, true, None);
    assert!(score_task(&Task::Sorting(task.clone()), &sheet, Some(&ordered(&ids))).is_some());
    for guesses in [&[("s1", Some(60.0)), ("s0", Some(1.0e9)), ("s2", None), ("s4", None), ("s3", None)][..], &[("s1", None), ("s0", None), ("s2", None), ("s4", None), ("s3", None)]] {
        assert_eq!(score_task(&Task::Sorting(task.clone()), &sheet, Some(&guessed(guesses))), None, "{guesses:?}");
    }
}

#[test]
fn guesses_in_the_right_order_within_reach_score_one_without_being_exact() {
    let task = sorting("power", &POWER, Scale::Logarithmic, None);
    let near = [("s0", Some(3.0)), ("s1", Some(20.0)), ("s2", Some(900.0)), ("s3", Some(2.0e6)), ("s4", Some(9.0e11))];
    assert_eq!(score(guess(&task, &near, None)), 1.0);
    assert_eq!(sorted_items(guess(&task, &near, None)), near.iter().enumerate().map(|(position, (id, guess))| ((*id).to_string(), position, *guess, Some(false))).collect::<Vec<_>>());
    let exact: Vec<(&str, Option<f64>)> = ["s0", "s1", "s2", "s3", "s4"].into_iter().zip(POWER.map(Some)).collect();
    assert_eq!(score(guess(&task, &exact, None)), 1.0);
}

#[test]
fn counting_one_two_three_in_the_right_order_earns_no_perfect_score() {
    let task = sorting("power", &POWER, Scale::Logarithmic, None);
    let counted = [("s0", Some(1.0)), ("s1", Some(2.0)), ("s2", Some(3.0)), ("s3", Some(4.0)), ("s4", Some(5.0))];
    let result = guess(&task, &counted, None);
    assert_eq!(sorted_items(result.clone()).iter().map(|item| item.3).collect::<Vec<_>>(), [Some(false), Some(false), Some(false), Some(true), Some(true)]);
    let expected = pairs(Scale::Logarithmic, &POWER, |_, j| if j >= 3 { 1.0 } else { 0.0 });
    assert_eq!(score(result), expected);
    assert!(expected > 0.1 && expected < 0.2, "{expected}");
}

#[test]
fn a_miss_costs_every_pair_it_touches() {
    let task = sorting("power", &POWER, Scale::Logarithmic, None);
    let far = [("s0", Some(1.0)), ("s1", Some(60.0)), ("s2", Some(2.5e6)), ("s3", Some(3.0e6)), ("s4", Some(1.0e9))];
    assert_eq!(score(guess(&task, &far, None)), pairs(Scale::Logarithmic, &POWER, |i, j| if i == 2 || j == 2 { 1.0 } else { 0.0 }));
    let missing = [("s0", Some(1.0)), ("s1", Some(60.0)), ("s2", None), ("s3", Some(1.0e6)), ("s4", Some(1.0e9))];
    assert_eq!(guess(&task, &missing, None), None, "an untimed sheet wants a guess for every item");
    let timed = guess(&task, &missing, Some(90));
    assert_eq!(score(timed.clone()), pairs(Scale::Logarithmic, &POWER, |i, j| if i == 2 || j == 2 { 1.0 } else { 0.0 }));
    assert_eq!(sorted_items(timed)[2], ("s2".to_string(), 2, None, Some(true)));
    let all = [("s0", Some(1.0e9)), ("s1", Some(1.0e9)), ("s2", Some(1.0e9)), ("s3", Some(1.0e10)), ("s4", Some(1.0e14))];
    assert_eq!(score(guess(&task, &all, None)), 0.0);
}

#[test]
fn guesses_within_reach_in_the_wrong_order_lose_the_weight_of_their_pairs() {
    let task = sorting("power", &POWER, Scale::Logarithmic, None);
    let swapped = [("s0", Some(1.0)), ("s2", Some(100.0)), ("s1", Some(3000.0)), ("s3", Some(1.0e6)), ("s4", Some(1.0e9))];
    let values = [1.0, 2000.0, 60.0, 1.0e6, 1.0e9];
    let expected = pairs(Scale::Logarithmic, &values, |i, j| if (i, j) == (1, 2) { 1.0 } else { 0.0 });
    assert_eq!(score(guess(&task, &swapped, None)), expected);
    assert_eq!(score(sort(&task, &["s0", "s2", "s1", "s3", "s4"])), expected, "the same order scores the same where the keys show");
    assert!(sorted_items(guess(&task, &swapped, None)).iter().all(|item| item.3 == Some(false)));
}

#[test]
fn guesses_are_judged_by_half_the_spread_on_narrow_and_linear_sets() {
    let narrow = sorting("narrow", &[10.0, 12.0, 40.0], Scale::Logarithmic, None);
    assert_eq!(score(guess(&narrow, &[("s0", Some(11.0)), ("s1", Some(15.0)), ("s2", Some(60.0))], None)), 1.0);
    let far = guess(&narrow, &[("s0", Some(10.0)), ("s1", Some(12.0)), ("s2", Some(400.0))], None);
    assert_eq!(sorted_items(far.clone()).iter().map(|item| item.3).collect::<Vec<_>>(), [Some(false), Some(false), Some(true)], "a factor of ten misses a set spanning a factor of four");
    assert_eq!(score(far), pairs(Scale::Logarithmic, &[10.0, 12.0, 40.0], |_, j| if j == 2 { 1.0 } else { 0.0 }));
    let linear = sorting("linear", &[10.0, 40.0, 120.0], Scale::Linear, None);
    assert_eq!(score(guess(&linear, &[("s0", Some(-40.0)), ("s1", Some(90.0)), ("s2", Some(175.0))], None)), 1.0, "each guess lies within 55 of its value");
    assert_eq!(score(guess(&linear, &[("s0", Some(10.0)), ("s1", Some(96.0)), ("s2", Some(120.0))], None)), pairs(Scale::Linear, &[10.0, 40.0, 120.0], |i, j| if i == 1 || j == 1 { 1.0 } else { 0.0 }));
}

#[test]
fn tied_values_carry_no_weight_and_score_by_their_misses() {
    let tied = sorting("tied", &[5.0, 5.0], Scale::Linear, None);
    assert_eq!(score(guess(&tied, &[("s1", Some(-1.0e12)), ("s0", Some(1.0e12))], None)), 1.0, "values that do not spread reach infinitely far on a linear scale");
    assert_eq!(score(guess(&tied, &[("s1", None), ("s0", Some(5.0))], Some(54))), 0.0);
    let logarithmic = sorting("tied", &[5.0, 5.0], Scale::Logarithmic, None);
    assert_eq!(score(guess(&logarithmic, &[("s1", Some(0.006)), ("s0", Some(4000.0))], None)), 1.0);
    assert_eq!(score(guess(&logarithmic, &[("s1", Some(0.004)), ("s0", Some(5.0))], None)), 0.0, "beyond a factor of 1000 of a value shared by all");
    let equal = sorting("power", &POWER, Scale::Logarithmic, None);
    let level = [("s0", Some(1.0)), ("s2", Some(300.0)), ("s1", Some(300.0)), ("s4", Some(3.0e7)), ("s3", Some(3.0e7))];
    let values = [1.0, 2000.0, 60.0, 1.0e9, 1.0e6];
    assert_eq!(score(guess(&equal, &level, None)), pairs(Scale::Logarithmic, &values, |i, j| if values[i] > values[j] { 1.0 } else { 0.0 }), "equal guesses stand in the learner's order and lose the pairs it reverses");
    assert!(sorted_items(guess(&equal, &level, None)).iter().all(|item| item.3 == Some(false)));
}

#[test]
fn a_timed_sorting_without_an_answer_stands_in_sheet_order_and_all_its_items_miss() {
    let task = sorting("power", &POWER, Scale::Logarithmic, None);
    let ids = ["s3", "s0", "s4", "s1"];
    let timed = score_task(&Task::Sorting(task.clone()), &sorting_sheet(&task, &ids, false, Some(78)), None);
    assert_eq!(score(timed.clone()), 0.0);
    assert_eq!(sorted_items(timed), ids.iter().enumerate().map(|(position, id)| ((*id).to_string(), position, None, Some(true))).collect::<Vec<_>>());
    let Some(TaskResult::Sorting { items, .. }) = score_task(&Task::Sorting(task.clone()), &sorting_sheet(&task, &ids, false, Some(78)), None) else { unreachable!() };
    assert_eq!(items.iter().map(|item| (item.value, item.rank)).collect::<Vec<_>>(), [(1.0e6, 2), (1.0, 0), (1.0e9, 3), (60.0, 1)]);
    assert_eq!(score_task(&Task::Sorting(task.clone()), &sorting_sheet(&task, &ids, false, None), None), None);
    let unguessed = score_task(&Task::Sorting(task.clone()), &sorting_sheet(&task, &ids, false, Some(78)), Some(&ordered(&["s0", "s1", "s3", "s4"])));
    assert_eq!((score(unguessed.clone()), sorted_items(unguessed).iter().map(|item| (item.2, item.3)).collect::<Vec<_>>()), (0.0, vec![(None, Some(true)); 4]));
    let shown = score_task(&Task::Sorting(task.clone()), &sorting_sheet(&task, &ids, true, Some(78)), None);
    assert_eq!((score(shown.clone()), sorted_items(shown).iter().map(|item| (item.2, item.3)).collect::<Vec<_>>()), (0.0, vec![(None, None); 4]), "without an answer every pair is lost also where the keys show");
    assert_eq!(score_task(&Task::Sorting(task.clone()), &sorting_sheet(&task, &ids, false, Some(78)), Some(&ordered(&["s0", "s1", "s3"]))), None, "an invalid answer is never scored");
}

fn matching_sheet(load: Option<&[f64]>, demand: Option<&[f64]>, seconds: Option<u64>) -> SheetTask {
    SheetTask::Matching(SheetMatchingTask {
        id: "m".to_string(),
        title: text("m"),
        prompt: text("m"),
        icon: None,
        dimensions: vec![SheetDimension { id: "load".to_string(), quantity: quantity("W", Scale::Linear), icon: None, cards: load.map(<[f64]>::to_vec) }, SheetDimension { id: "demand".to_string(), quantity: quantity("kWh", Scale::Logarithmic), icon: None, cards: demand.map(<[f64]>::to_vec) }],
        items: items(&["m0", "m1", "m2"]),
        seconds,
    })
}

fn matched(truth: &[f64], cards: &[f64], picks: &[usize]) -> Option<TaskResult> {
    let rows: Vec<(f64, f64)> = truth.iter().map(|&value| (value, value)).collect();
    let mut task = matching("m", &rows, None);
    task.dimensions.truncate(1);
    let ids: Vec<String> = (0..truth.len()).map(|index| format!("m{index}")).collect();
    let sheet = SheetTask::Matching(SheetMatchingTask { id: "m".to_string(), title: text("m"), prompt: text("m"), icon: None, dimensions: vec![SheetDimension { id: "load".to_string(), quantity: quantity("W", Scale::Linear), icon: None, cards: Some(cards.to_vec()) }], items: items(&ids.iter().map(String::as_str).collect::<Vec<_>>()), seconds: None });
    let answer = Answer::Matching(MatchingAnswer { assignments: Some(BTreeMap::from([("load".to_string(), ids.iter().cloned().zip(picks.iter().copied()).collect())])), guesses: None });
    score_task(&Task::Matching(task), &sheet, Some(&answer))
}

fn assigned(load: &[(&str, usize)], demand: &[(&str, usize)]) -> Answer {
    let cards = |picks: &[(&str, usize)]| picks.iter().map(|(item, card)| ((*item).to_string(), *card)).collect::<BTreeMap<_, _>>();
    Answer::Matching(MatchingAnswer { assignments: Some(BTreeMap::from([("load".to_string(), cards(load)), ("demand".to_string(), cards(demand))])), guesses: None })
}

fn estimated(load: &[(&str, f64)], demand: &[(&str, f64)]) -> Answer {
    let guesses = |picks: &[(&str, f64)]| picks.iter().map(|(item, guess)| ((*item).to_string(), *guess)).collect::<BTreeMap<_, _>>();
    Answer::Matching(MatchingAnswer { assignments: None, guesses: Some(BTreeMap::from([("load".to_string(), guesses(load)), ("demand".to_string(), guesses(demand))])) })
}

fn buildings() -> Task {
    Task::Matching(matching("m", &[(10.0, 15.0), (40.0, 90.0), (120.0, 250.0)], None))
}

type Matched = Vec<(f64, Vec<(Option<f64>, f64, Option<bool>)>)>;

fn dimensions(result: Option<TaskResult>) -> Matched {
    match result {
        Some(TaskResult::Matching { dimensions, .. }) => dimensions.into_iter().map(|dimension| (dimension.score, dimension.items.into_iter().map(|item| (item.assigned, item.correct, item.miss)).collect())).collect(),
        _ => Vec::new(),
    }
}

#[test]
fn matching_scores_pair_concordance_with_half_credit_ties() {
    assert_eq!(score(matched(&[1.0, 2.0, 3.0], &[3.0, 1.0, 2.0], &[1, 2, 0])), 1.0);
    assert_eq!(score(matched(&[1.0, 2.0, 3.0], &[3.0, 1.0, 2.0], &[0, 2, 1])), 0.0);
    assert_eq!(score(matched(&[1.0, 2.0, 3.0], &[3.0, 1.0, 2.0], &[2, 1, 0])), 0.75);
    assert_eq!(score(matched(&[1.0, 1.0, 3.0], &[1.0, 1.0, 3.0], &[0, 2, 1])), 0.25);
    assert!(matched(&[1.0, 2.0, 3.0], &[3.0, 1.0, 2.0], &[1, 2]).is_none());
}

#[test]
fn matching_task_score_is_the_mean_over_dimensions() {
    let sheet = matching_sheet(Some(&[10.0, 40.0, 120.0]), Some(&[250.0, 90.0, 15.0]), None);
    let identity = [("m0", 0), ("m1", 1), ("m2", 2)];
    let result = score_task(&buildings(), &sheet, Some(&assigned(&identity, &identity)));
    assert_eq!(score(result.clone()), 0.5);
    assert_eq!(dimensions(result), [(1.0, vec![(Some(10.0), 10.0, None), (Some(40.0), 40.0, None), (Some(120.0), 120.0, None)]), (0.0, vec![(Some(250.0), 15.0, None), (Some(90.0), 90.0, None), (Some(15.0), 250.0, None)])]);
    assert_eq!(score_task(&buildings(), &sheet, Some(&estimated(&[("m0", 10.0), ("m1", 40.0), ("m2", 120.0)], &[("m0", 15.0), ("m1", 90.0), ("m2", 250.0)]))), None, "a matching that shows the keys takes no guesses");
    assert_eq!(score_task(&buildings(), &sheet, None), None);
}

#[test]
fn guessed_matchings_score_one_within_reach_and_carry_the_guess_and_the_miss() {
    let sheet = matching_sheet(None, None, None);
    let exact = estimated(&[("m0", 10.0), ("m1", 40.0), ("m2", 120.0)], &[("m0", 15.0), ("m1", 90.0), ("m2", 250.0)]);
    let result = score_task(&buildings(), &sheet, Some(&exact));
    assert_eq!(score(result.clone()), 1.0);
    assert_eq!(dimensions(result), [(1.0, vec![(Some(10.0), 10.0, Some(false)), (Some(40.0), 40.0, Some(false)), (Some(120.0), 120.0, Some(false))]), (1.0, vec![(Some(15.0), 15.0, Some(false)), (Some(90.0), 90.0, Some(false)), (Some(250.0), 250.0, Some(false))])]);
    let near = estimated(&[("m0", -40.0), ("m1", 90.0), ("m2", 175.0)], &[("m0", 5.0), ("m1", 100.0), ("m2", 900.0)]);
    assert_eq!(score(score_task(&buildings(), &sheet, Some(&near))), 1.0);
    assert_eq!(score_task(&buildings(), &sheet, Some(&assigned(&[("m0", 0)], &[]))), None, "a matching that hides the keys takes no assignments");
}

#[test]
fn guessed_matchings_lose_the_pairs_of_misses_reversals_and_half_of_ties() {
    let sheet = matching_sheet(None, None, None);
    let loads = [10.0, 40.0, 120.0];
    let demands = [15.0, 90.0, 250.0];
    let scored = |load: [f64; 3], demand: [f64; 3]| dimensions(score_task(&buildings(), &sheet, Some(&estimated(&[("m0", load[0]), ("m1", load[1]), ("m2", load[2])], &[("m0", demand[0]), ("m1", demand[1]), ("m2", demand[2])]))));
    let missed = scored([10.0, 96.0, 120.0], [15.0, 90.0, 1500.0]);
    assert_eq!(missed[0].0, pairs(Scale::Linear, &loads, |i, j| if i == 1 || j == 1 { 1.0 } else { 0.0 }));
    assert_eq!(missed[0].1.iter().map(|item| item.2).collect::<Vec<_>>(), [Some(false), Some(true), Some(false)]);
    assert_eq!(missed[1].0, pairs(Scale::Logarithmic, &demands, |_, j| if j == 2 { 1.0 } else { 0.0 }));
    assert_eq!(missed[1].1[2], (Some(1500.0), 250.0, Some(true)));
    let reversed = scored([50.0, 20.0, 120.0], [60.0, 60.0, 250.0]);
    assert_eq!(reversed[0].0, pairs(Scale::Linear, &loads, |i, j| if (i, j) == (0, 1) { 1.0 } else { 0.0 }));
    assert_eq!(reversed[1].0, pairs(Scale::Logarithmic, &demands, |i, j| if (i, j) == (0, 1) { 0.5 } else { 0.0 }));
    assert!(reversed.iter().flat_map(|dimension| &dimension.1).all(|item| item.2 == Some(false)));
    let all = scored([500.0, 500.0, 500.0], [1.0e9, 1.0e9, 1.0e9]);
    assert_eq!((all[0].0, all[1].0), (0.0, 0.0));
}

#[test]
fn an_item_without_a_guess_misses_on_a_timed_matching_and_keeps_an_untimed_one_unscored() {
    let partial = estimated(&[("m0", 10.0), ("m2", 120.0)], &[]);
    assert_eq!(score_task(&buildings(), &matching_sheet(None, None, None), Some(&partial)), None);
    let timed = matching_sheet(None, None, Some(102));
    let result = score_task(&buildings(), &timed, Some(&partial));
    let scored = dimensions(result.clone());
    assert_eq!(scored[0], (pairs(Scale::Linear, &[10.0, 40.0, 120.0], |i, j| if i == 1 || j == 1 { 1.0 } else { 0.0 }), vec![(Some(10.0), 10.0, Some(false)), (None, 40.0, Some(true)), (Some(120.0), 120.0, Some(false))]));
    assert_eq!(scored[1], (0.0, vec![(None, 15.0, Some(true)), (None, 90.0, Some(true)), (None, 250.0, Some(true))]));
    assert_eq!(score(result), (scored[0].0 + 0.0) / 2.0);
    for unanswered in [None, Some(Answer::Matching(MatchingAnswer { assignments: None, guesses: None }))] {
        let result = score_task(&buildings(), &timed, unanswered.as_ref());
        assert_eq!((score(result.clone()), dimensions(result).iter().flat_map(|dimension| dimension.1.iter().map(|item| (item.0, item.2))).collect::<Vec<_>>()), (0.0, vec![(None, Some(true)); 6]));
    }
    let json = serde_json::to_value(score_task(&buildings(), &timed, None)).unwrap_or_default();
    assert_eq!(json["dimensions"][0]["items"][0], serde_json::json!({"item": "m0", "correct": 10.0, "miss": true}));
}

#[test]
fn an_unassigned_card_takes_part_as_a_miss_on_a_timed_matching_that_shows_the_keys() {
    let timed = matching_sheet(Some(&[10.0, 40.0, 120.0]), Some(&[15.0, 90.0, 250.0]), Some(102));
    let partial = assigned(&[("m0", 0), ("m2", 2)], &[("m0", 0), ("m1", 1), ("m2", 2)]);
    let scored = dimensions(score_task(&buildings(), &timed, Some(&partial)));
    assert_eq!(scored[0], (pairs(Scale::Linear, &[10.0, 40.0, 120.0], |i, j| if i == 1 || j == 1 { 1.0 } else { 0.0 }), vec![(Some(10.0), 10.0, None), (None, 40.0, None), (Some(120.0), 120.0, None)]));
    assert_eq!(scored[1].0, 1.0);
    assert_eq!(score_task(&buildings(), &matching_sheet(Some(&[10.0, 40.0, 120.0]), Some(&[15.0, 90.0, 250.0]), None), Some(&partial)), None);
    let tied = Task::Matching(matching("m", &[(7.0, 3.0), (7.0, 3.0), (7.0, 3.0)], None));
    let level = dimensions(score_task(&tied, &matching_sheet(None, None, Some(102)), Some(&estimated(&[("m0", 1.0), ("m1", 2.0), ("m2", 3.0)], &[("m0", 3.0), ("m1", 3.0)]))));
    assert_eq!((level[0].0, level[1].0), (1.0, 0.0), "without any weight a dimension scores 0 with a miss and 1 without");
}

fn classification_sheet(task: &ClassificationTask, ids: &[&str], seconds: Option<u64>) -> SheetTask {
    SheetTask::Classification(SheetClassificationTask { id: task.id.clone(), title: task.title.clone(), prompt: task.prompt.clone(), icon: None, axes: None, categories: task.categories.clone(), items: items(ids), seconds })
}

fn classify(task: &ClassificationTask, pairs: &[(&str, &str)]) -> Option<TaskResult> {
    let answer = Answer::Classification(ClassificationAnswer { assignments: pairs.iter().map(|(item, category)| ((*item).to_string(), (*category).to_string())).collect() });
    score_task(&Task::Classification(task.clone()), &classification_sheet(task, &pairs.iter().map(|(item, _)| *item).collect::<Vec<_>>(), None), Some(&answer))
}

#[test]
fn classification_credits_profile_similarity() {
    let task = classification("standards", None);
    assert_eq!(score(classify(&task, &[("a", "passive"), ("b", "low"), ("c", "old"), ("d", "low")])), 1.0);
    let Some(TaskResult::Classification { items, score, .. }) = classify(&task, &[("a", "low"), ("b", "low"), ("c", "passive"), ("d", "old")]) else { unreachable!() };
    let largest = (0.81f64 + 0.16 + 0.64).sqrt();
    let expected = [1.0 - (0.04f64 + 0.01 + 0.09).sqrt() / largest, 1.0, 0.0, 1.0 - (0.49f64 + 0.09 + 0.25).sqrt() / largest];
    for (item, expected) in items.iter().zip(expected) {
        assert!((item.credit - expected).abs() < 1e-12, "{}: {} vs {expected}", item.item, item.credit);
    }
    assert!((score - expected.iter().sum::<f64>() / 4.0).abs() < 1e-12);
    assert_eq!(items[0].explanation, Some(text("a is passive")));
    assert_eq!(items.iter().map(|item| item.assigned.as_deref()).collect::<Vec<_>>(), [Some("low"), Some("low"), Some("passive"), Some("old")]);
}

#[test]
fn classification_without_profiles_is_all_or_nothing() {
    let mut task = classification("plain", None);
    task.axes = None;
    for category in &mut task.categories {
        category.profile = None;
    }
    assert_eq!(score(classify(&task, &[("a", "low"), ("b", "low"), ("c", "old"), ("d", "passive")])), 0.5);
}

#[test]
fn an_unassigned_item_earns_nothing_on_a_timed_classification() {
    let task = classification("standards", None);
    let ids = ["a", "b", "c", "d"];
    let partial = Answer::Classification(ClassificationAnswer { assignments: BTreeMap::from([("a".to_string(), "passive".to_string()), ("c".to_string(), "old".to_string())]) });
    assert_eq!(score_task(&Task::Classification(task.clone()), &classification_sheet(&task, &ids, None), Some(&partial)), None);
    let Some(TaskResult::Classification { items, score, .. }) = score_task(&Task::Classification(task.clone()), &classification_sheet(&task, &ids, Some(62)), Some(&partial)) else { unreachable!() };
    assert_eq!(items.iter().map(|item| (item.item.as_str(), item.assigned.as_deref(), item.correct.as_str(), item.credit)).collect::<Vec<_>>(), [("a", Some("passive"), "passive", 1.0), ("b", None, "low", 0.0), ("c", Some("old"), "old", 1.0), ("d", None, "low", 0.0)]);
    assert_eq!(score, 0.5);
    assert_eq!(serde_json::to_value(&items[1]).unwrap_or_default(), serde_json::json!({"item": "b", "correct": "low", "credit": 0.0, "explanation": {"en": "b is low", "de": "b is low (de)"}}));
    let Some(TaskResult::Classification { items, score, .. }) = score_task(&Task::Classification(task.clone()), &classification_sheet(&task, &ids, Some(62)), None) else { unreachable!() };
    assert_eq!((score, items.iter().all(|item| item.assigned.is_none() && item.credit == 0.0), items.len()), (0.0, true, 4));
    assert_eq!(score_task(&Task::Classification(task.clone()), &classification_sheet(&task, &ids, None), None), None);
}

#[test]
fn invalid_or_incomplete_answers_are_not_scored() {
    let task = classification("standards", None);
    assert!(classify(&task, &[("a", "future")]).is_none());
    let sorting_task = sorting("s", &[1.0, 2.0], Scale::Linear, None);
    for seconds in [None, Some(54)] {
        let sheet = sorting_sheet(&sorting_task, &["s0", "s1"], true, seconds);
        assert!(score_task(&Task::Sorting(sorting_task.clone()), &sheet, Some(&ordered(&["s0"]))).is_none());
        assert!(score_task(&Task::Sorting(sorting_task.clone()), &sheet, Some(&Answer::Classification(ClassificationAnswer { assignments: BTreeMap::new() }))).is_none());
        assert!(score_task(&Task::Sorting(sorting("other", &[1.0, 2.0], Scale::Linear, None)), &sheet, Some(&ordered(&["s0", "s1"]))).is_none());
    }
}

fn answers(quiz: &Quiz, sheet: &Sheet) -> BTreeMap<Slug, Answer> {
    sheet.tasks.iter().map(|task| (task.id().clone(), perfect(quiz, task))).collect()
}

#[test]
fn run_score_is_the_mean_of_task_scores_in_sheet_order() {
    let quiz = quiz();
    let sheet = sheet_of(&quiz, 11, Challenge::Medium);
    let mut answers = BTreeMap::new();
    for task in &sheet.tasks {
        let answer = match task {
            SheetTask::Classification(task) => Answer::Classification(ClassificationAnswer { assignments: task.items.iter().map(|item| (item.id.clone(), "old".to_string())).collect() }),
            SheetTask::Sorting(task) => Answer::Sorting(SortingAnswer { order: task.items.iter().map(|item| item.id.clone()).collect(), guesses: None }),
            SheetTask::Matching(task) => Answer::Matching(MatchingAnswer { assignments: Some(task.dimensions.iter().map(|dimension| (dimension.id.clone(), task.items.iter().enumerate().map(|(index, item)| (item.id.clone(), index)).collect())).collect()), guesses: None }),
        };
        answers.insert(task.id().clone(), answer);
    }
    let Some(result) = score_run(&quiz, &sheet, &answers) else { unreachable!() };
    assert_eq!(result.tasks.iter().map(|task| task.task().as_str()).collect::<Vec<_>>(), sheet.tasks.iter().map(|task| task.id().as_str()).collect::<Vec<_>>());
    let scores: Vec<f64> = result.tasks.iter().map(TaskResult::score).collect();
    assert_eq!(result.score, (scores[0] + scores[1] + scores[2]) / 3.0);
    assert_eq!((result.challenge, result.points), (Challenge::Medium, result.score * 200.0));
    assert!(scores.iter().all(|score| (0.0..=1.0).contains(score)));
    answers.remove(sheet.tasks[0].id());
    assert!(score_run(&quiz, &sheet, &answers).is_none());
}

#[test]
fn a_perfect_run_earns_the_par_of_its_challenge() {
    let quiz = quiz();
    for challenge in CHALLENGES {
        for seed in [3, 11, 5489] {
            let sheet = sheet_of(&quiz, seed, challenge);
            let Some(result) = score_run(&quiz, &sheet, &answers(&quiz, &sheet)) else { panic!("seed {seed} at {challenge:?} was not scored") };
            assert_eq!((result.score, result.points, result.challenge), (1.0, challenge_rules(challenge).par, challenge), "seed {seed} at {challenge:?}");
            let json = serde_json::to_value(&result).unwrap_or_default();
            let mut members: Vec<&str> = json.as_object().into_iter().flat_map(|object| object.keys().map(String::as_str)).collect();
            members.sort_unstable();
            assert_eq!(members, ["challenge", "points", "quiz", "score", "tasks"]);
            let hidden = !challenge_rules(challenge).keys;
            assert_eq!(serde_json::to_string(&result).unwrap_or_default().contains("\"miss\":false"), hidden, "{challenge:?}");
        }
    }
}

#[test]
fn a_timed_run_is_scored_with_what_is_missing_as_a_miss() {
    let quiz = quiz();
    let expert = sheet_of(&quiz, 11, Challenge::Expert);
    let whole = answers(&quiz, &expert);
    let Some(empty) = score_run(&quiz, &expert, &BTreeMap::new()) else { unreachable!() };
    assert_eq!((empty.score, empty.points, empty.tasks.len()), (0.0, 0.0, 3));
    for missing in expert.tasks.iter().map(|task| task.id()) {
        let mut partial = whole.clone();
        partial.remove(missing);
        let Some(result) = score_run(&quiz, &expert, &partial) else { unreachable!() };
        assert_eq!((result.score, result.points), (2.0 / 3.0, 2.0 / 3.0 * 400.0), "without {missing}");
        assert_eq!(result.tasks.iter().map(|task| (task.task(), task.score())).collect::<Vec<_>>(), expert.tasks.iter().map(|task| (task.id(), if task.id() == missing { 0.0 } else { 1.0 })).collect::<Vec<_>>());
    }
    let hard = sheet_of(&quiz, 11, Challenge::Hard);
    let mut partial = answers(&quiz, &hard);
    assert!(score_run(&quiz, &hard, &partial).is_some());
    partial.remove(hard.tasks[1].id());
    assert_eq!(score_run(&quiz, &hard, &partial), None, "an untimed run is scored only when every answer is complete");
    assert_eq!(score_run(&quiz, &hard, &whole).map(|result| (result.score, result.points)), Some((1.0, 300.0)), "hard and expert sheets of one seed take the same answers");
}

#[test]
fn shared_scores_of_the_python_numpy_scipy_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    for case in ["sorting-concordance", "matching-concordance", "profile-similarity"] {
        let vectors = fixture(case);
        let tasks: Vec<Task> = entries(&vectors["tasks"]).iter().map(typed).collect();
        for vector in entries(&vectors["vectors"]) {
            let task = tasks.iter().find(|task| Some(task.id().as_str()) == vector["task"].as_str()).unwrap_or_else(|| panic!("{}", vector["task"]));
            let answer: Option<Answer> = vector.get("answer").map(typed);
            let result = score_task(task, &typed(&vector["sheetTask"]), answer.as_ref()).unwrap_or_else(|| panic!("{case}/{} was not scored", vector["id"]));
            assert_close(&format!("{case}/{}", vector["id"]), &json(&result), &vector["expected"]);
        }
    }
}

mod quick {
    use super::*;
    use crate::randomness::{shuffle, Mt19937};

    #[test]
    fn sorting_scores_stay_in_the_unit_interval_and_perfect_is_unique() {
        let values = [0.5, 3.0, 40.0, 700.0, 9_000.0, 1.0e5, 2.0e6];
        let task = sorting("many", &values, Scale::Logarithmic, None);
        let ids: Vec<String> = (0..values.len()).map(|index| format!("s{index}")).collect();
        let mut random = Mt19937::new(77);
        for _ in 0..2_000 {
            let order = shuffle(&mut random, &ids);
            let score = score(sort(&task, &order.iter().map(String::as_str).collect::<Vec<_>>()));
            assert!((0.0..=1.0).contains(&score));
            assert_eq!(score == 1.0, order == ids);
        }
    }

    #[test]
    fn guessed_scores_stay_in_the_unit_interval_and_never_beat_the_same_order_with_shown_keys() {
        let values = [0.5, 3.0, 40.0, 700.0, 9_000.0, 1.0e5, 2.0e6];
        let task = sorting("many", &values, Scale::Logarithmic, None);
        let ids: Vec<usize> = (0..values.len()).collect();
        let mut random = Mt19937::new(78);
        for round in 0..2_000u32 {
            let order = shuffle(&mut random, &ids);
            let factor = [1.0, 30.0, 0.02, 4000.0][(round % 4) as usize];
            let names: Vec<String> = order.iter().map(|index| format!("s{index}")).collect();
            let mut guesses: Vec<f64> = order.iter().map(|&index| values[index] * if index % 2 == 0 { factor } else { 1.0 }).collect();
            guesses.sort_by(f64::total_cmp);
            let guessed = score(guess(&task, &names.iter().map(String::as_str).zip(guesses.into_iter().map(Some)).collect::<Vec<_>>(), None));
            let shown = score(sort(&task, &names.iter().map(String::as_str).collect::<Vec<_>>()));
            assert!((0.0..=1.0).contains(&guessed) && guessed <= shown, "{guessed} vs {shown}");
        }
    }
}
