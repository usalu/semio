//! 🎳️ Unit tests of the scoring: pair concordance, profile similarity, run means and the shared scores.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::schema::{MatchingAnswer, SheetDimension, SheetItem, SheetSortingTask};
use crate::sheet::sheet_of;
use crate::sheet::tests::{classification, matching, quantity, quiz, sorting, text};

fn items(ids: &[&str]) -> Vec<SheetItem> {
    ids.iter().map(|id| SheetItem { id: (*id).to_string(), label: text(id) }).collect()
}

fn sort(task: &SortingTask, order: &[&str]) -> Option<TaskResult> {
    let sheet = SheetTask::Sorting(SheetSortingTask { id: task.id.clone(), title: task.title.clone(), prompt: task.prompt.clone(), quantity: task.quantity.clone(), items: items(order) });
    score_task(&Task::Sorting(task.clone()), &sheet, &Answer::Sorting(SortingAnswer { order: order.iter().map(|id| (*id).to_string()).collect() }))
}

fn score(result: Option<TaskResult>) -> f64 {
    result.map_or(f64::NAN, |result| result.score())
}

#[test]
fn sorting_perfect_is_one_and_reversed_is_zero_exactly() {
    let task = sorting("power", &[1.0, 60.0, 2000.0, 1.0e6, 1.0e9], Scale::Logarithmic, None);
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
    let task = sorting("power", &[1.0, 60.0, 2000.0, 1.0e6, 1.0e9], Scale::Logarithmic, None);
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
}

fn matched(truth: &[f64], cards: &[f64], picks: &[usize]) -> Option<TaskResult> {
    let rows: Vec<(f64, f64)> = truth.iter().map(|&value| (value, value)).collect();
    let mut task = matching("m", &rows, None);
    task.dimensions.truncate(1);
    let ids: Vec<String> = (0..truth.len()).map(|index| format!("m{index}")).collect();
    let sheet = SheetTask::Matching(SheetMatchingTask { id: "m".to_string(), title: text("m"), prompt: text("m"), dimensions: vec![SheetDimension { id: "load".to_string(), quantity: quantity("W", Scale::Linear), cards: cards.to_vec() }], items: items(&ids.iter().map(String::as_str).collect::<Vec<_>>()) });
    let answer = Answer::Matching(MatchingAnswer { assignments: BTreeMap::from([("load".to_string(), ids.iter().cloned().zip(picks.iter().copied()).collect())]) });
    score_task(&Task::Matching(task), &sheet, &answer)
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
    let task = matching("m", &[(10.0, 15.0), (40.0, 90.0), (120.0, 250.0)], None);
    let sheet = SheetTask::Matching(SheetMatchingTask {
        id: "m".to_string(),
        title: text("m"),
        prompt: text("m"),
        dimensions: vec![SheetDimension { id: "load".to_string(), quantity: quantity("W", Scale::Linear), cards: vec![10.0, 40.0, 120.0] }, SheetDimension { id: "demand".to_string(), quantity: quantity("kWh", Scale::Logarithmic), cards: vec![250.0, 90.0, 15.0] }],
        items: items(&["m0", "m1", "m2"]),
    });
    let answer = Answer::Matching(MatchingAnswer {
        assignments: BTreeMap::from([
            ("load".to_string(), BTreeMap::from([("m0".to_string(), 0), ("m1".to_string(), 1), ("m2".to_string(), 2)])),
            ("demand".to_string(), BTreeMap::from([("m0".to_string(), 0), ("m1".to_string(), 1), ("m2".to_string(), 2)])),
        ]),
    });
    let Some(TaskResult::Matching { score, dimensions, .. }) = score_task(&Task::Matching(task), &sheet, &answer) else { unreachable!() };
    assert_eq!((dimensions[0].score, dimensions[1].score, score), (1.0, 0.0, 0.5));
    assert_eq!(dimensions[1].items.iter().map(|item| (item.assigned, item.correct)).collect::<Vec<_>>(), [(250.0, 15.0), (90.0, 90.0), (15.0, 250.0)]);
}

fn classify(task: &ClassificationTask, pairs: &[(&str, &str)]) -> Option<TaskResult> {
    let sheet = SheetTask::Classification(SheetClassificationTask { id: task.id.clone(), title: task.title.clone(), prompt: task.prompt.clone(), axes: task.axes.clone(), categories: task.categories.clone(), items: items(&pairs.iter().map(|(item, _)| *item).collect::<Vec<_>>()) });
    let answer = Answer::Classification(ClassificationAnswer { assignments: pairs.iter().map(|(item, category)| ((*item).to_string(), (*category).to_string())).collect() });
    score_task(&Task::Classification(task.clone()), &sheet, &answer)
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
fn invalid_or_incomplete_answers_are_not_scored() {
    let task = classification("standards", None);
    assert!(classify(&task, &[("a", "future")]).is_none());
    let sorting_task = sorting("s", &[1.0, 2.0], Scale::Linear, None);
    let sheet = SheetTask::Sorting(SheetSortingTask { id: "s".to_string(), title: text("s"), prompt: text("s"), quantity: quantity("W", Scale::Linear), items: items(&["s0", "s1"]) });
    assert!(score_task(&Task::Sorting(sorting_task.clone()), &sheet, &Answer::Sorting(SortingAnswer { order: vec!["s0".to_string()] })).is_none());
    assert!(score_task(&Task::Sorting(sorting_task), &sheet, &Answer::Classification(ClassificationAnswer { assignments: BTreeMap::new() })).is_none());
}

#[test]
fn run_score_is_the_mean_of_task_scores_in_sheet_order() {
    let quiz = quiz();
    let sheet = sheet_of(&quiz, 11);
    let mut answers = BTreeMap::new();
    for task in &sheet.tasks {
        let answer = match task {
            SheetTask::Classification(task) => Answer::Classification(ClassificationAnswer { assignments: task.items.iter().map(|item| (item.id.clone(), "old".to_string())).collect() }),
            SheetTask::Sorting(task) => Answer::Sorting(SortingAnswer { order: task.items.iter().map(|item| item.id.clone()).collect() }),
            SheetTask::Matching(task) => Answer::Matching(MatchingAnswer { assignments: task.dimensions.iter().map(|dimension| (dimension.id.clone(), task.items.iter().enumerate().map(|(index, item)| (item.id.clone(), index)).collect())).collect() }),
        };
        answers.insert(task.id().clone(), answer);
    }
    let Some(result) = score_run(&quiz, &sheet, &answers) else { unreachable!() };
    assert_eq!(result.tasks.iter().map(|task| task.task().as_str()).collect::<Vec<_>>(), sheet.tasks.iter().map(|task| task.id().as_str()).collect::<Vec<_>>());
    let scores: Vec<f64> = result.tasks.iter().map(TaskResult::score).collect();
    assert_eq!(result.score, (scores[0] + scores[1] + scores[2]) / 3.0);
    assert!(scores.iter().all(|score| (0.0..=1.0).contains(score)));
    answers.remove(sheet.tasks[0].id());
    assert!(score_run(&quiz, &sheet, &answers).is_none());
}

#[test]
fn shared_scores_of_the_python_numpy_scipy_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    for case in ["sorting-concordance", "matching-concordance", "profile-similarity"] {
        let vectors = fixture(case);
        let tasks: Vec<Task> = entries(&vectors["tasks"]).iter().map(typed).collect();
        for vector in entries(&vectors["vectors"]) {
            let task = tasks.iter().find(|task| Some(task.id().as_str()) == vector["task"].as_str()).unwrap_or_else(|| panic!("{}", vector["task"]));
            let result = score_task(task, &typed(&vector["sheetTask"]), &typed(&vector["answer"])).unwrap_or_else(|| panic!("{case}/{} was not scored", vector["id"]));
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
}
