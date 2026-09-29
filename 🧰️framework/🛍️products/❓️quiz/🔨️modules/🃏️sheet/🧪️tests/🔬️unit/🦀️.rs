//! 📑️ Unit tests of the sheet: RNG consumption order, draws, rotation and the shared sheets; also the shared quiz builders.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::randomness::uniform_index;
use crate::schema::{Axis, Category, ClassificationItem, ClassificationTask, Dimension, MatchingItem, Quantity, Scale, SortingTask};
use std::collections::BTreeMap;

pub(crate) fn text(value: &str) -> Text {
    Text { en: value.to_string(), de: format!("{value} (de)") }
}

pub(crate) fn quantity(unit: &str, scale: Scale) -> Quantity {
    Quantity { label: text(unit), unit: unit.to_string(), scale, prefixed: false }
}

pub(crate) fn classification(id: &str, draw: Option<usize>) -> ClassificationTask {
    let axis = |id: &str| Axis { id: id.to_string(), label: text(id), unit: "u".to_string(), min: 0.0, max: 10.0 };
    let category = |id: &str, profile: [f64; 3]| Category { id: id.to_string(), label: text(id), description: None, profile: Some(BTreeMap::from([("heat".to_string(), profile[0]), ("cool".to_string(), profile[1]), ("cost".to_string(), profile[2])])) };
    let item = |id: &str, category: &str| ClassificationItem { id: id.to_string(), label: text(id), category: category.to_string(), explanation: Some(text(&format!("{id} is {category}"))) };
    ClassificationTask {
        id: id.to_string(),
        title: text(id),
        prompt: text("classify"),
        axes: Some(vec![axis("heat"), axis("cool"), axis("cost")]),
        categories: vec![category("passive", [1.0, 2.0, 9.0]), category("low", [3.0, 3.0, 6.0]), category("old", [10.0, 6.0, 1.0])],
        items: vec![item("a", "passive"), item("b", "low"), item("c", "old"), item("d", "low")],
        draw,
    }
}

pub(crate) fn sorting(id: &str, values: &[f64], scale: Scale, draw: Option<usize>) -> SortingTask {
    SortingTask {
        id: id.to_string(),
        title: text(id),
        prompt: text("sort"),
        quantity: quantity("W", scale),
        items: values.iter().enumerate().map(|(index, &value)| SortingItem { id: format!("s{index}"), label: text(&format!("s{index}")), value, explanation: None }).collect(),
        draw,
    }
}

pub(crate) fn matching(id: &str, rows: &[(f64, f64)], draw: Option<usize>) -> MatchingTask {
    MatchingTask {
        id: id.to_string(),
        title: text(id),
        prompt: text("match"),
        dimensions: vec![Dimension { id: "load".to_string(), quantity: quantity("W/m²", Scale::Linear) }, Dimension { id: "demand".to_string(), quantity: quantity("kWh/(m²a)", Scale::Logarithmic) }],
        items: rows.iter().enumerate().map(|(index, &(load, demand))| MatchingItem { id: format!("m{index}"), label: text(&format!("m{index}")), values: BTreeMap::from([("load".to_string(), load), ("demand".to_string(), demand)]), explanation: None }).collect(),
        draw,
    }
}

pub(crate) fn quiz() -> Quiz {
    Quiz {
        json_schema: None,
        schema: "semio.quiz/v1".to_string(),
        id: "energy".to_string(),
        emoji: "⚡".to_string(),
        title: text("Energy"),
        description: text("About energy"),
        tasks: vec![
            Task::Classification(classification("standards", None)),
            Task::Sorting(sorting("power", &[1.0, 60.0, 2000.0, 1.0e6, 1.0e9], Scale::Logarithmic, Some(4))),
            Task::Matching(matching("buildings", &[(10.0, 15.0), (40.0, 90.0), (120.0, 250.0)], None)),
        ],
    }
}

fn ids(items: &[SheetItem]) -> Vec<&str> {
    items.iter().map(|item| item.id.as_str()).collect()
}

#[test]
fn sheet_is_a_pure_function_of_quiz_and_seed() {
    assert_eq!(sheet_of(&quiz(), 17), sheet_of(&quiz(), 17));
    let sheets: Vec<Sheet> = (0..32).map(|seed| sheet_of(&quiz(), seed)).collect();
    assert!(sheets.iter().any(|sheet| sheet != &sheets[0]));
}

#[test]
fn sheet_copies_the_quiz_header_and_permutes_the_tasks() {
    let sheet = sheet_of(&quiz(), 3);
    assert_eq!((sheet.quiz.as_str(), sheet.seed, &sheet.title, &sheet.description), ("energy", 3, &text("Energy"), &text("About energy")));
    let mut presented: Vec<&str> = sheet.tasks.iter().map(|task| task.id().as_str()).collect();
    presented.sort_unstable();
    assert_eq!(presented, ["buildings", "power", "standards"]);
}

#[test]
fn sheet_follows_the_rng_consumption_order() {
    let quiz = quiz();
    let seed = 12_345;
    let mut random = Mt19937::new(seed);
    let order = shuffle(&mut random, &[0usize, 1, 2]);
    let classification_items = shuffle(&mut random, &[0usize, 1, 2, 3]);
    let categories = shuffle(&mut random, &["passive", "low", "old"]);
    let sorting_items = shuffle(&mut random, &[0usize, 1, 2, 3, 4]);
    let matching_items = shuffle(&mut random, &[0usize, 1, 2]);
    let loads = [10.0, 40.0, 120.0];
    let demands = [15.0, 90.0, 250.0];
    let load_cards = shuffle(&mut random, &matching_items.iter().map(|&i| loads[i]).collect::<Vec<_>>());
    let demand_cards = shuffle(&mut random, &matching_items.iter().map(|&i| demands[i]).collect::<Vec<_>>());
    let sheet = sheet_of(&quiz, seed);
    let expected_order: Vec<&str> = order.iter().map(|&index| quiz.tasks[index].id().as_str()).collect();
    assert_eq!(sheet.tasks.iter().map(|task| task.id().as_str()).collect::<Vec<_>>(), expected_order);
    for task in &sheet.tasks {
        match task {
            SheetTask::Classification(task) => {
                assert_eq!(ids(&task.items), classification_items.iter().map(|&i| ["a", "b", "c", "d"][i]).collect::<Vec<_>>());
                assert_eq!(task.categories.iter().map(|category| category.id.as_str()).collect::<Vec<_>>(), categories);
            }
            SheetTask::Sorting(task) => {
                let mut drawn: Vec<usize> = sorting_items[..4].to_vec();
                if drawn.windows(2).all(|pair| pair[0] < pair[1]) {
                    drawn.rotate_left(1);
                }
                assert_eq!(ids(&task.items), drawn.iter().map(|i| format!("s{i}")).collect::<Vec<_>>());
            }
            SheetTask::Matching(task) => {
                assert_eq!(ids(&task.items), matching_items.iter().map(|i| format!("m{i}")).collect::<Vec<_>>());
                assert_eq!(task.dimensions[0].cards, load_cards);
                assert_eq!(task.dimensions[1].cards, demand_cards);
            }
        }
    }
}

#[test]
fn sheet_draws_and_never_presents_a_sorting_ascending() {
    for seed in 0..500 {
        let sheet = sheet_of(&quiz(), seed);
        for task in &sheet.tasks {
            if let SheetTask::Sorting(task) = task {
                assert_eq!(task.items.len(), 4);
                let indices: Vec<usize> = task.items.iter().map(|item| item.id[1..].parse().unwrap_or(usize::MAX)).collect();
                assert!(!indices.windows(2).all(|pair| pair[0] < pair[1]), "seed {seed}: {indices:?}");
            }
        }
    }
}

#[test]
fn sorting_ties_break_by_definition_index_before_rotating() {
    let tied = Quiz { tasks: vec![Task::Sorting(sorting("tied", &[5.0, 5.0], Scale::Linear, None))], ..quiz() };
    for seed in 0..64 {
        let sheet = sheet_of(&tied, seed);
        let SheetTask::Sorting(task) = &sheet.tasks[0] else { unreachable!() };
        assert_eq!(ids(&task.items), ["s1", "s0"]);
    }
}

#[test]
fn matching_cards_are_the_drawn_values() {
    let drawn = Quiz { tasks: vec![Task::Matching(matching("m", &[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0), (7.0, 8.0)], Some(2)))], ..quiz() };
    let sheet = sheet_of(&drawn, 99);
    let SheetTask::Matching(task) = &sheet.tasks[0] else { unreachable!() };
    assert_eq!(task.items.len(), 2);
    let expect: Vec<f64> = task.items.iter().map(|item| [1.0, 3.0, 5.0, 7.0][item.id[1..].parse::<usize>().unwrap_or(0)]).collect();
    let mut cards = task.dimensions[0].cards.clone();
    let mut expect_sorted = expect;
    cards.sort_by(f64::total_cmp);
    expect_sorted.sort_by(f64::total_cmp);
    assert_eq!(cards, expect_sorted);
}

#[test]
fn sheet_is_solution_free() {
    let json = serde_json::to_value(sheet_of(&quiz(), 5)).unwrap_or_default();
    for task in json["tasks"].as_array().into_iter().flatten() {
        for item in task["items"].as_array().into_iter().flatten() {
            let mut keys: Vec<&str> = item.as_object().into_iter().flat_map(|object| object.keys().map(String::as_str)).collect();
            keys.sort_unstable();
            assert_eq!(keys, ["id", "label"]);
        }
    }
}

#[test]
fn shared_sheets_of_the_python_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    let vectors = fixture("sheet-assembly");
    let quizzes: Vec<Quiz> = entries(&vectors["quizzes"]).iter().map(typed).collect();
    for vector in entries(&vectors["sheets"]) {
        let quiz = quizzes.iter().find(|quiz| Some(quiz.id.as_str()) == vector["quiz"].as_str()).unwrap_or_else(|| panic!("{}", vector["quiz"]));
        let seed = vector["seed"].as_u64().and_then(|seed| u32::try_from(seed).ok()).unwrap_or_else(|| panic!("{}", vector["seed"]));
        assert_close(&format!("sheet-assembly/{}", vector["id"]), &json(&sheet_of(quiz, seed)), &vector["sheet"]);
    }
}

#[test]
fn uniform_index_is_what_shuffle_consumes() {
    let mut random = Mt19937::new(1);
    let j = uniform_index(&mut random, 3);
    let mut expected = vec![0, 1, 2];
    expected.swap(2, j);
    let k = uniform_index(&mut random, 2);
    expected.swap(1, k);
    assert_eq!(shuffle(&mut Mt19937::new(1), &[0, 1, 2]), expected);
}
