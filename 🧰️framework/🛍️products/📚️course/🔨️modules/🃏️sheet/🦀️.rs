//! 🃏️ The sheet of a run: the randomized presentation of a quiz at a challenge as a pure function of
//! `(quiz, seed, challenge)`. The first task stays first and the remaining tasks are shuffled. The
//! RNG consumption order is part of the contract: the shuffled task order first, then per task in
//! definition order its items, then its categories (classification) or its cards per dimension in
//! definition order (matching).
//!
//! The generator is consumed in the same order whatever the challenge, so one seed deals the same
//! items in the same order at every challenge; the challenge only decides what a task shows
//! afterwards. Where it shows the keys a sorting carries the ascending true values of its presented
//! items (a ladder, never which item holds which), a matching its cards, a classification its category
//! descriptions and whole axes. Where it hides them the sheet carries none of these numbers: no
//! sorting keys, no cards, no descriptions, no axis numbers, and profiles only as shares of their axis
//! range. Where it is timed every task carries its seconds.
//!
//! @see ../🎲️randomness/🦀️.rs — the generator and shuffle
//! @see ../⛰️challenge/🦀️.rs — the rules of a challenge and the seconds of a task
//! @see ../🃏️sheet/🟦️.ts — the TypeScript twin

use crate::challenge::{challenge_rules, task_seconds, ChallengeRules};
use crate::randomness::{fnv1a32, shuffle, weighted_sample, Mt19937};
use crate::schema::{Axis, Category, Challenge, Distribution, Icon, Mockup, MatchingTask, Quiz, Sheet, SheetAxis, SheetClassificationTask, SheetDimension, SheetItem, SheetMatchingTask, SheetSortingTask, SheetTask, SortingItem, Task, TaskKind, Text};
use std::cmp::Ordering;

/// 🎰️ Present `quiz` for `seed` at `challenge`. Expects a quiz free of
/// [`crate::validation::quiz_issues`]; a matching item missing a dimension value contributes a `NaN`
/// card.
pub fn sheet_of(quiz: &Quiz, seed: u32, challenge: Challenge) -> Sheet {
    let mut random = Mt19937::new(seed);
    let rules = challenge_rules(challenge);
    let shuffled_order = shuffle(&mut random, &(0..quiz.tasks.len()).collect::<Vec<_>>());
    let order = if shuffled_order.len() > 1 {
        std::iter::once(0).chain(shuffled_order.into_iter().filter(|&index| index != 0)).collect()
    } else {
        shuffled_order
    };
    let mut presented: Vec<Option<SheetTask>> = quiz.tasks.iter().map(|task| Some(present(&mut random, task, rules, seed))).collect();
    Sheet {
        quiz: quiz.id.clone(),
        seed,
        challenge,
        title: quiz.title.clone(),
        description: quiz.description.clone(),
        tasks: order.iter().filter_map(|&index| presented[index].take()).collect(),
    }
}

fn present(random: &mut Mt19937, task: &Task, rules: ChallengeRules, seed: u32) -> SheetTask {
    let seconds = |kind: TaskKind, items: usize, dimensions: usize| rules.timed.then(|| task.pool().map_or_else(|| task_seconds(kind, items, dimensions), |pool| u64::from(pool.seconds)));
    match task {
        Task::Classification(task) => {
            let items = pooled(random, &task.id, task.items.iter().map(|item| item.id.as_str()).collect(), task.draw, task.pool.as_ref(), &magnitudes(task.items.len(), |_| None), seed);
            let axes = task.axes.as_deref().unwrap_or_default();
            SheetTask::Classification(SheetClassificationTask {
                id: task.id.clone(),
                title: task.title.clone(),
                prompt: task.prompt.clone(),
                icon: task.icon.clone(),
                axes: task.axes.as_ref().map(|axes| axes.iter().map(|axis| sheet_axis(axis, rules.keys)).collect()),
                categories: shuffle(random, &task.categories).into_iter().map(|category| sheet_category(category, axes, rules.keys)).collect(),
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label, task.items[index].short.as_ref(), task.items[index].icon.as_ref(), task.items[index].mockup.as_ref())).collect(),
                seconds: seconds(TaskKind::Classification, items.len(), 1),
            })
        }
        Task::Sorting(task) => {
            let mut items = pooled(random, &task.id, task.items.iter().map(|item| item.id.as_str()).collect(), task.draw, task.pool.as_ref(), &magnitudes(task.items.len(), |index| Some(task.items[index].value)), seed);
            let keys = rules.keys.then(|| ladder(&task.items, &items));
            if items.len() >= 2 && ascending(&task.items, &items) {
                items.rotate_left(1);
            }
            SheetTask::Sorting(SheetSortingTask {
                id: task.id.clone(),
                title: task.title.clone(),
                prompt: task.prompt.clone(),
                icon: task.icon.clone(),
                quantity: task.quantity.clone(),
                keys,
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label, task.items[index].short.as_ref(), task.items[index].icon.as_ref(), task.items[index].mockup.as_ref())).collect(),
                seconds: seconds(TaskKind::Sorting, items.len(), 1),
            })
        }
        Task::Matching(task) => {
            let items = pooled(random, &task.id, task.items.iter().map(|item| item.id.as_str()).collect(), task.draw, task.pool.as_ref(), &magnitudes(task.items.len(), |index| task.items[index].values.get(&task.dimensions[0].id).copied()), seed);
            SheetTask::Matching(SheetMatchingTask {
                id: task.id.clone(),
                title: task.title.clone(),
                prompt: task.prompt.clone(),
                icon: task.icon.clone(),
                dimensions: task
                    .dimensions
                    .iter()
                    .map(|dimension| {
                        let cards = shuffle(random, &values(task, &items, &dimension.id));
                        SheetDimension { id: dimension.id.clone(), quantity: dimension.quantity.clone(), icon: dimension.icon.clone(), cards: rules.keys.then_some(cards) }
                    })
                    .collect(),
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label, task.items[index].short.as_ref(), task.items[index].icon.as_ref(), task.items[index].mockup.as_ref())).collect(),
                seconds: seconds(TaskKind::Matching, items.len(), task.dimensions.len()),
            })
        }
    }
}

fn sheet_axis(axis: &Axis, keys: bool) -> SheetAxis {
    SheetAxis { id: axis.id.clone(), label: axis.label.clone(), short: axis.short.clone(), unit:keys.then(|| axis.unit.clone()), min: keys.then_some(axis.min), max: keys.then_some(axis.max) }
}

fn sheet_category(category: Category, axes: &[Axis], keys: bool) -> Category {
    if keys {
        return category;
    }
    let share = |(id, value): (&String, &f64)| axes.iter().find(|axis| axis.id == *id).map(|axis| (id.clone(), (value - axis.min) / (axis.max - axis.min)));
    Category { id: category.id, label: category.label, short: category.short, icon: category.icon, description: None,profile: category.profile.map(|profile| profile.iter().filter_map(share).collect()) }
}

fn ladder(items: &[SortingItem], order: &[usize]) -> Vec<f64> {
    let mut sorted = order.to_vec();
    sorted.sort_by(|&left, &right| items[left].value.partial_cmp(&items[right].value).unwrap_or(Ordering::Equal).then(left.cmp(&right)));
    sorted.into_iter().map(|index| items[index].value).collect()
}

fn magnitudes(count: usize, value: impl Fn(usize) -> Option<f64>) -> Vec<f64> {
    (0..count).map(|index| value(index).map_or(index as f64, f64::abs)).collect()
}

fn drawn_slots(id: &str, items: &[&str], draw: Option<usize>, pool: &crate::schema::QuestionPool, magnitudes: &[f64], seed: u32) -> Vec<usize> {
    let Some(Distribution::Zipf { exponent }) = pool.distribution else { return (0..pool.slots.len()).collect() };
    let smallest: Vec<f64> = pool.slots.iter().map(|slot| slot.iter().filter_map(|entry| items.iter().position(|item| *item == entry.as_str())).map(|position| magnitudes[position]).fold(f64::INFINITY, f64::min)).collect();
    let mut ranked: Vec<usize> = (0..pool.slots.len()).collect();
    ranked.sort_by(|&left, &right| smallest[left].partial_cmp(&smallest[right]).unwrap_or(Ordering::Equal).then(left.cmp(&right)));
    let weights: Vec<f64> = (0..ranked.len()).map(|rank| ((rank + 1) as f64).powf(-exponent)).collect();
    let mut stream = Mt19937::new(seed ^ fnv1a32(&format!("{id}:zipf")));
    weighted_sample(&mut stream, &weights, draw.unwrap_or(0)).into_iter().map(|rank| ranked[rank]).collect()
}

fn pooled(random: &mut Mt19937, id: &str, items: Vec<&str>, draw: Option<usize>, pool: Option<&crate::schema::QuestionPool>, magnitudes: &[f64], seed: u32) -> Vec<usize> {
    let Some(pool) = pool else { return drawn(random, items.len(), draw) };
    let selected: std::collections::BTreeSet<&str> = drawn_slots(id, &items, draw, pool, magnitudes, seed).into_iter().map(|index| {
        let slot = &pool.slots[index];
        let mut deck_random = Mt19937::new((seed >> 16) ^ fnv1a32(&format!("{id}:{index}")));
        let deck = shuffle(&mut deck_random, slot);
        let chosen = &deck[(seed as usize & 65535) % deck.len()];
        slot.iter().find(|item| *item == chosen).unwrap().as_str()
    }).collect();
    shuffle(random, &(0..items.len()).collect::<Vec<_>>()).into_iter().filter(|&index| selected.contains(items[index])).collect()
}

fn drawn(random: &mut Mt19937, count: usize, draw: Option<usize>) -> Vec<usize> {
    let mut indices = shuffle(random, &(0..count).collect::<Vec<_>>());
    if let Some(draw) = draw {
        indices.truncate(draw);
    }
    indices
}

fn ascending(items: &[SortingItem], order: &[usize]) -> bool {
    order.windows(2).all(|pair| {
        let (left, right) = (items[pair[0]].value, items[pair[1]].value);
        left < right || (left == right && pair[0] < pair[1])
    })
}

fn values(task: &MatchingTask, items: &[usize], dimension: &str) -> Vec<f64> {
    items.iter().map(|&index| task.items[index].values.get(dimension).copied().unwrap_or(f64::NAN)).collect()
}

fn sheet_item(id: &str, label: &Text, short: Option<&Text>, icon: Option<&Icon>, mockup: Option<&Mockup>) -> SheetItem {
    SheetItem { mockup: mockup.cloned(), id: id.to_string(), label: label.clone(), short: short.cloned(), icon: icon.cloned() }
}

