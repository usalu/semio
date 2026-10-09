//! 📑️ Unit tests of the sheet: RNG consumption order, draws, rotation, what every challenge shows and the shared sheets; also the shared quiz builders.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::randomness::uniform_index;
use crate::schema::{Axis, Category, ClassificationItem, ClassificationTask, Dimension, MatchingItem, Quantity, Scale, SortingTask, CHALLENGES};
use std::collections::BTreeMap;

pub(crate) fn text(value: &str) -> Text {
    Text { en: value.to_string(), de: format!("{value} (de)") }
}

pub(crate) fn quantity(unit: &str, scale: Scale) -> Quantity {
    Quantity { label: text(unit), short: None, unit: unit.to_string(), scale, prefixed: false, additive: false }
}

pub(crate) fn classification(id: &str, draw: Option<usize>) -> ClassificationTask {
    let axis = |id: &str| Axis { id: id.to_string(), label: text(id), short: None, unit: "u".to_string(), min: 0.0, max: 10.0 };
    let category = |id: &str, profile: [f64; 3]| Category { id: id.to_string(), label: text(id), short: None, icon: None, description: None, profile: Some(BTreeMap::from([("heat".to_string(), profile[0]), ("cool".to_string(), profile[1]), ("cost".to_string(), profile[2])])) };
    let item = |id: &str, category: &str| ClassificationItem { id: id.to_string(), label: text(id), short: None, icon: None, category: category.to_string(), explanation: Some(text(&format!("{id} is {category}"))) };
    ClassificationTask {
        id: id.to_string(),
        title: text(id),
        prompt: text("classify"),
        icon: None,
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
        icon: None,
        quantity: quantity("W", scale),
        items: values.iter().enumerate().map(|(index, &value)| SortingItem { id: format!("s{index}"), label: text(&format!("s{index}")), short: None, icon: None, value, familiar: None, explanation: None }).collect(),
        draw,
    }
}

pub(crate) fn matching(id: &str, rows: &[(f64, f64)], draw: Option<usize>) -> MatchingTask {
    MatchingTask {
        id: id.to_string(),
        title: text(id),
        prompt: text("match"),
        icon: None,
        dimensions: vec![Dimension { id: "load".to_string(), quantity: quantity("W/m²", Scale::Linear), icon: None }, Dimension { id: "demand".to_string(), quantity: quantity("kWh/(m²a)", Scale::Logarithmic), icon: None }],
        items: rows.iter().enumerate().map(|(index, &(load, demand))| MatchingItem { id: format!("m{index}"), label: text(&format!("m{index}")), short: None, icon: None, values: BTreeMap::from([("load".to_string(), load), ("demand".to_string(), demand)]), familiar: None, explanation: None }).collect(),
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
        short: None,
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
fn sheet_is_a_pure_function_of_quiz_seed_and_challenge() {
    for challenge in CHALLENGES {
        assert_eq!(sheet_of(&quiz(), 17, challenge), sheet_of(&quiz(), 17, challenge));
        let sheets: Vec<Sheet> = (0..32).map(|seed| sheet_of(&quiz(), seed, challenge)).collect();
        assert!(sheets.iter().any(|sheet| sheet != &sheets[0]));
    }
    let sheets = CHALLENGES.map(|challenge| sheet_of(&quiz(), 17, challenge));
    assert!(sheets.iter().enumerate().all(|(index, sheet)| sheets.iter().skip(index + 1).all(|other| other != sheet)));
}

#[test]
fn sheet_copies_the_quiz_header_and_keeps_the_first_task_first() {
    let sheet = sheet_of(&quiz(), 3, Challenge::Hard);
    assert_eq!((sheet.quiz.as_str(), sheet.seed, sheet.challenge, &sheet.title, &sheet.description), ("energy", 3, Challenge::Hard, &text("Energy"), &text("About energy")));
    let presented: Vec<&str> = sheet.tasks.iter().map(|task| task.id().as_str()).collect();
    assert_eq!(presented[0], "standards");
    let mut remaining = presented[1..].to_vec();
    remaining.sort_unstable();
    assert_eq!(remaining, ["buildings", "power"]);
}

#[test]
fn sheet_follows_the_rng_consumption_order() {
    let quiz = quiz();
    let seed = 12_345;
    let mut random = Mt19937::new(seed);
    let shuffled_order = shuffle(&mut random, &[0usize, 1, 2]);
    let order: Vec<usize> = std::iter::once(0).chain(shuffled_order.into_iter().filter(|&index| index != 0)).collect();
    let classification_items = shuffle(&mut random, &[0usize, 1, 2, 3]);
    let categories = shuffle(&mut random, &["passive", "low", "old"]);
    let sorting_items = shuffle(&mut random, &[0usize, 1, 2, 3, 4]);
    let matching_items = shuffle(&mut random, &[0usize, 1, 2]);
    let loads = [10.0, 40.0, 120.0];
    let demands = [15.0, 90.0, 250.0];
    let load_cards = shuffle(&mut random, &matching_items.iter().map(|&i| loads[i]).collect::<Vec<_>>());
    let demand_cards = shuffle(&mut random, &matching_items.iter().map(|&i| demands[i]).collect::<Vec<_>>());
    let expected_order: Vec<&str> = order.iter().map(|&index| quiz.tasks[index].id().as_str()).collect();
    for challenge in CHALLENGES {
        let sheet = sheet_of(&quiz, seed, challenge);
        let shown = challenge <= Challenge::Medium;
        assert_eq!(sheet.tasks.iter().map(|task| task.id().as_str()).collect::<Vec<_>>(), expected_order, "{challenge:?}");
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
                    assert_eq!(task.dimensions[0].cards, shown.then(|| load_cards.clone()), "{challenge:?}");
                    assert_eq!(task.dimensions[1].cards, shown.then(|| demand_cards.clone()), "{challenge:?}");
                }
            }
        }
    }
}

fn dealt(sheet: &Sheet) -> Vec<(String, Vec<String>, Vec<String>)> {
    sheet
        .tasks
        .iter()
        .map(|task| {
            let categories = match task {
                SheetTask::Classification(task) => task.categories.iter().map(|category| category.id.clone()).collect(),
                _ => Vec::new(),
            };
            (task.id().clone(), task.items().iter().map(|item| item.id.clone()).collect(), categories)
        })
        .collect()
}

#[test]
fn one_seed_deals_the_same_items_in_the_same_order_at_every_challenge() {
    for seed in [0, 1, 17, 5489, 12_345, u32::MAX] {
        let medium = sheet_of(&quiz(), seed, Challenge::Medium);
        for challenge in CHALLENGES {
            let sheet = sheet_of(&quiz(), seed, challenge);
            assert_eq!((sheet.challenge, sheet.seed, dealt(&sheet)), (challenge, seed, dealt(&medium)), "seed {seed} at {challenge:?}");
        }
        let easy = sheet_of(&quiz(), seed, Challenge::Easy);
        assert_eq!(Sheet { challenge: Challenge::Medium, ..easy }, medium, "easy and medium sheets differ in their challenge only");
    }
}

#[test]
fn a_sorting_carries_its_ascending_keys_exactly_where_the_challenge_shows_them() {
    let values = [1.0, 60.0, 2000.0, 1.0e6, 1.0e9];
    for seed in 0..50 {
        for challenge in CHALLENGES {
            let sheet = sheet_of(&quiz(), seed, challenge);
            let Some(SheetTask::Sorting(task)) = sheet.tasks.iter().find(|task| task.id() == "power") else { unreachable!() };
            let mut presented: Vec<f64> = task.items.iter().map(|item| values[item.id[1..].parse::<usize>().unwrap_or(0)]).collect();
            presented.sort_by(f64::total_cmp);
            assert_eq!(task.keys, (challenge <= Challenge::Medium).then_some(presented), "seed {seed} at {challenge:?}");
        }
    }
    let tied = Quiz { tasks: vec![Task::Sorting(sorting("tied", &[5.0, 1.0, 5.0], Scale::Linear, None))], ..quiz() };
    let SheetTask::Sorting(task) = &sheet_of(&tied, 4, Challenge::Easy).tasks[0] else { unreachable!() };
    assert_eq!(task.keys, Some(vec![1.0, 5.0, 5.0]));
}

fn described() -> Quiz {
    let mut quiz = quiz();
    let Task::Classification(task) = &mut quiz.tasks[0] else { unreachable!() };
    for category in &mut task.categories {
        category.description = Some(text(&format!("about {}", category.id)));
    }
    quiz
}

#[test]
fn a_classification_keeps_descriptions_and_whole_axes_where_the_keys_show() {
    let quiz = described();
    let Task::Classification(definition) = &quiz.tasks[0] else { unreachable!() };
    for challenge in [Challenge::Easy, Challenge::Medium] {
        let sheet = sheet_of(&quiz, 21, challenge);
        let SheetTask::Classification(task) = &sheet.tasks[0] else { unreachable!() };
        assert_eq!(task.axes.as_ref().map(|axes| axes.iter().map(|axis| (axis.id.as_str(), axis.unit.as_deref(), axis.min, axis.max)).collect::<Vec<_>>()), Some(vec![("heat", Some("u"), Some(0.0), Some(10.0)), ("cool", Some("u"), Some(0.0), Some(10.0)), ("cost", Some("u"), Some(0.0), Some(10.0))]));
        for category in &task.categories {
            assert_eq!(Some(category), definition.categories.iter().find(|candidate| candidate.id == category.id));
        }
    }
}

#[test]
fn a_classification_loses_descriptions_and_axis_numbers_where_the_keys_are_hidden() {
    let mut quiz = described();
    let Task::Classification(definition) = &mut quiz.tasks[0] else { unreachable!() };
    if let Some(axes) = definition.axes.as_mut() {
        axes[0].min = -10.0;
        axes[2].max = 18.0;
    }
    if let Some(profile) = definition.categories[1].profile.as_mut() {
        profile.insert("ghost".to_string(), 4.0);
    }
    definition.categories[2].profile = None;
    for challenge in [Challenge::Hard, Challenge::Expert] {
        let sheet = sheet_of(&quiz, 21, challenge);
        let SheetTask::Classification(task) = &sheet.tasks[0] else { unreachable!() };
        assert_eq!(task.axes, Some(["heat", "cool", "cost"].map(|id| SheetAxis { id: id.to_string(), label: text(id), short: None, unit: None, min: None, max: None }).to_vec()));
        let category = |id: &str| task.categories.iter().find(|category| category.id == id).unwrap_or_else(|| unreachable!());
        assert!(task.categories.iter().all(|category| category.description.is_none() && category.label == text(&category.id)));
        assert_eq!(category("passive").profile, Some(BTreeMap::from([("heat".to_string(), (1.0 - -10.0) / (10.0 - -10.0)), ("cool".to_string(), (2.0 - 0.0) / (10.0 - 0.0)), ("cost".to_string(), (9.0 - 0.0) / (18.0 - 0.0))])));
        assert_eq!(category("low").profile, Some(BTreeMap::from([("heat".to_string(), 0.65), ("cool".to_string(), 0.3), ("cost".to_string(), 6.0 / 18.0)])), "a value of no axis is left out");
        assert_eq!(category("old").profile, None);
    }
    let Task::Classification(definition) = &mut quiz.tasks[0] else { unreachable!() };
    definition.axes = None;
    let sheet = sheet_of(&quiz, 21, Challenge::Hard);
    let SheetTask::Classification(task) = &sheet.tasks[0] else { unreachable!() };
    assert_eq!((task.axes.as_ref(), task.categories.iter().filter_map(|category| category.profile.as_ref()).map(BTreeMap::len).collect::<Vec<_>>()), (None, vec![0, 0]));
}

#[test]
fn short_labels_reach_every_sheet_and_familiarity_stays_behind() {
    let mut quiz = quiz();
    let short = |id: &str| Some(text(&format!("{id}·")));
    for task in &mut quiz.tasks {
        match task {
            Task::Classification(task) => {
                task.axes.iter_mut().flatten().for_each(|axis| axis.short = short(&axis.id));
                task.categories.iter_mut().for_each(|category| category.short = short(&category.id));
                task.items.iter_mut().for_each(|item| item.short = short(&item.id));
            }
            Task::Sorting(task) => {
                task.quantity.short = short("power");
                task.items.iter_mut().for_each(|item| (item.short, item.familiar) = (short(&item.id), Some(true)));
            }
            Task::Matching(task) => {
                task.dimensions.iter_mut().for_each(|dimension| dimension.quantity.short = short(&dimension.id));
                task.items.iter_mut().for_each(|item| (item.short, item.familiar) = (short(&item.id), Some(true)));
            }
        }
    }
    for challenge in CHALLENGES {
        let sheet = sheet_of(&quiz, 5, challenge);
        assert!(sheet.tasks.iter().all(|task| task.items().iter().all(|item| item.short == short(&item.id))), "{challenge:?}");
        for task in &sheet.tasks {
            match task {
                SheetTask::Classification(task) => assert!(task.axes.iter().flatten().all(|axis| axis.short == short(&axis.id)) && task.categories.iter().all(|category| category.short == short(&category.id))),
                SheetTask::Sorting(task) => assert_eq!(task.quantity.short, short("power")),
                SheetTask::Matching(task) => assert!(task.dimensions.iter().all(|dimension| dimension.quantity.short == short(&dimension.id))),
            }
        }
        assert!(!serde_json::to_string(&sheet).unwrap_or_default().contains("familiar"), "{challenge:?}");
    }
}

#[test]
fn every_task_carries_its_seconds_exactly_on_a_timed_sheet() {
    for challenge in CHALLENGES {
        let sheet = sheet_of(&quiz(), 9, challenge);
        let seconds: BTreeMap<&str, Option<u64>> = sheet.tasks.iter().map(|task| (task.id().as_str(), task.seconds())).collect();
        let expected = if challenge == Challenge::Expert { [Some(102), Some(78), Some(62)] } else { [None; 3] };
        assert_eq!(seconds, BTreeMap::from([("buildings", expected[0]), ("power", expected[1]), ("standards", expected[2])]), "{challenge:?}");
    }
    let drawn = Quiz { tasks: vec![Task::Matching(matching("m", &[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0), (7.0, 8.0)], Some(2)))], ..quiz() };
    assert_eq!(sheet_of(&drawn, 9, Challenge::Expert).tasks[0].seconds(), Some(30 + 12 * 2 * 2), "the presented items count, not the authored ones");
}

#[test]
fn a_sheet_that_hides_the_keys_carries_no_value_at_all() {
    for (challenge, hidden) in CHALLENGES.into_iter().zip([false, false, true, true]) {
        let json = serde_json::to_string(&sheet_of(&described(), 5, challenge)).unwrap_or_default();
        for member in ["\"keys\"", "\"cards\"", "\"min\"", "\"max\"", "\"description\":{\"en\":\"about"] {
            assert_eq!(!json.contains(member), hidden, "{member} at {challenge:?}");
        }
        assert_eq!(json.contains("\"seconds\""), challenge == Challenge::Expert);
        assert!(json.contains(&format!("\"challenge\":{}", serde_json::to_string(&challenge).unwrap_or_default())) && !json.contains("\"value\"") && !json.contains("\"values\"") && !json.contains("\"category\""));
    }
}

#[test]
fn first_defined_task_stays_first_while_remaining_tasks_are_shuffled() {
    let mut orders = std::collections::BTreeSet::new();
    for seed in 0..100 {
        let sheet = sheet_of(&quiz(), seed, Challenge::Medium);
        assert_eq!(sheet.tasks[0].id(), "standards");
        orders.insert(sheet.tasks[1..].iter().map(|task| task.id().to_string()).collect::<Vec<_>>());
    }
    assert!(orders.len() > 1);
}

#[test]
fn sheet_draws_and_never_presents_a_sorting_ascending() {
    for seed in 0..500 {
        let sheet = sheet_of(&quiz(), seed, CHALLENGES[seed as usize % 4]);
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
        let sheet = sheet_of(&tied, seed, CHALLENGES[seed as usize % 4]);
        let SheetTask::Sorting(task) = &sheet.tasks[0] else { unreachable!() };
        assert_eq!(ids(&task.items), ["s1", "s0"]);
    }
}

#[test]
fn matching_cards_are_the_drawn_values() {
    let drawn = Quiz { tasks: vec![Task::Matching(matching("m", &[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0), (7.0, 8.0)], Some(2)))], ..quiz() };
    let sheet = sheet_of(&drawn, 99, Challenge::Easy);
    let SheetTask::Matching(task) = &sheet.tasks[0] else { unreachable!() };
    assert_eq!(task.items.len(), 2);
    let expect: Vec<f64> = task.items.iter().map(|item| [1.0, 3.0, 5.0, 7.0][item.id[1..].parse::<usize>().unwrap_or(0)]).collect();
    let mut cards = task.dimensions[0].cards.clone().unwrap_or_default();
    let mut expect_sorted = expect;
    cards.sort_by(f64::total_cmp);
    expect_sorted.sort_by(f64::total_cmp);
    assert_eq!(cards, expect_sorted);
}

#[test]
fn sheet_items_never_say_which_number_is_theirs() {
    for challenge in CHALLENGES {
        let json = serde_json::to_value(sheet_of(&quiz(), 5, challenge)).unwrap_or_default();
        for task in json["tasks"].as_array().into_iter().flatten() {
            assert!(!task["items"].as_array().is_none_or(Vec::is_empty));
            for item in task["items"].as_array().into_iter().flatten() {
                let mut keys: Vec<&str> = item.as_object().into_iter().flat_map(|object| object.keys().map(String::as_str)).collect();
                keys.sort_unstable();
                assert_eq!(keys, ["id", "label"]);
            }
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
        assert_close(&format!("sheet-assembly/{}", vector["id"]), &json(&sheet_of(quiz, seed, typed(&vector["sheet"]["challenge"]))), &vector["sheet"]);
    }
}

#[test]
fn a_value_read_from_a_document_is_the_key_the_device_deals() {
    let sun: SortingItem = serde_json::from_str(r#"{"id":"sun","label":{"en":"Sun","de":"Sonne"},"value":3.828e+26}"#).unwrap_or_else(|error| panic!("{error}"));
    let tea: SortingItem = serde_json::from_str(r#"{"id":"tea","label":{"en":"Tea light","de":"Teelicht"},"value":35}"#).unwrap_or_else(|error| panic!("{error}"));
    let task = SortingTask { items: vec![tea, sun], ..sorting("powers", &[], Scale::Logarithmic, None) };
    let sheet = sheet_of(&Quiz { tasks: vec![Task::Sorting(task)], ..quiz() }, 7, Challenge::Easy);
    let SheetTask::Sorting(dealt) = &sheet.tasks[0] else { unreachable!() };
    assert_eq!(dealt.keys.as_deref().map(|keys| keys.iter().map(|key| key.to_bits()).collect::<Vec<_>>()), Some(vec![35_f64.to_bits(), 3.828e26_f64.to_bits()]), "read one double low, the proctor's ladder is not the device's and its deputy takes the run for revised");
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
