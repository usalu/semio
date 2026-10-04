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
use crate::randomness::{shuffle, Mt19937};
use crate::schema::{Axis, Category, Challenge, Icon, MatchingTask, Quiz, Sheet, SheetAxis, SheetClassificationTask, SheetDimension, SheetItem, SheetMatchingTask, SheetSortingTask, SheetTask, SortingItem, Task, TaskKind, Text};
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
    let mut presented: Vec<Option<SheetTask>> = quiz.tasks.iter().map(|task| Some(present(&mut random, task, rules))).collect();
    Sheet {
        quiz: quiz.id.clone(),
        seed,
        challenge,
        title: quiz.title.clone(),
        description: quiz.description.clone(),
        tasks: order.iter().filter_map(|&index| presented[index].take()).collect(),
    }
}

fn present(random: &mut Mt19937, task: &Task, rules: ChallengeRules) -> SheetTask {
    let seconds = |kind: TaskKind, items: usize, dimensions: usize| rules.timed.then(|| task_seconds(kind, items, dimensions));
    match task {
        Task::Classification(task) => {
            let items = drawn(random, task.items.len(), task.draw);
            let axes = task.axes.as_deref().unwrap_or_default();
            SheetTask::Classification(SheetClassificationTask {
                id: task.id.clone(),
                title: task.title.clone(),
                prompt: task.prompt.clone(),
                icon: task.icon.clone(),
                axes: task.axes.as_ref().map(|axes| axes.iter().map(|axis| sheet_axis(axis, rules.keys)).collect()),
                categories: shuffle(random, &task.categories).into_iter().map(|category| sheet_category(category, axes, rules.keys)).collect(),
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label, task.items[index].short.as_ref(), task.items[index].icon.as_ref())).collect(),
                seconds: seconds(TaskKind::Classification, items.len(), 1),
            })
        }
        Task::Sorting(task) => {
            let mut items = drawn(random, task.items.len(), task.draw);
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
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label, task.items[index].short.as_ref(), task.items[index].icon.as_ref())).collect(),
                seconds: seconds(TaskKind::Sorting, items.len(), 1),
            })
        }
        Task::Matching(task) => {
            let items = drawn(random, task.items.len(), task.draw);
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
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label, task.items[index].short.as_ref(), task.items[index].icon.as_ref())).collect(),
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

fn sheet_item(id: &str, label: &Text, short: Option<&Text>, icon: Option<&Icon>) -> SheetItem {
    SheetItem { id: id.to_string(), label: label.clone(), short: short.cloned(), icon: icon.cloned() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
