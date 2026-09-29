//! 🃏️ The sheet of a run: the randomized, solution-free presentation of a quiz as a pure function of
//! `(quiz, seed)` (design §4). The RNG consumption order is part of the contract: the task order
//! first, then per task in definition order its items, then its categories (classification) or its
//! cards per dimension in definition order (matching).
//!
//! @see ../🎲️randomness/🦀️.rs — the generator and shuffle
//! @see ../🃏️sheet/🟦️.ts — the TypeScript twin

use crate::randomness::{shuffle, Mt19937};
use crate::schema::{MatchingTask, Quiz, Sheet, SheetClassificationTask, SheetDimension, SheetItem, SheetMatchingTask, SheetSortingTask, SheetTask, SortingItem, Task, Text};

/// 🎰️ Present `quiz` for `seed`. Expects a quiz free of [`crate::validation::quiz_issues`]; a matching
/// item missing a dimension value contributes a `NaN` card.
pub fn sheet_of(quiz: &Quiz, seed: u32) -> Sheet {
    let mut random = Mt19937::new(seed);
    let order = shuffle(&mut random, &(0..quiz.tasks.len()).collect::<Vec<_>>());
    let mut presented: Vec<Option<SheetTask>> = quiz.tasks.iter().map(|task| Some(present(&mut random, task))).collect();
    Sheet {
        quiz: quiz.id.clone(),
        seed,
        title: quiz.title.clone(),
        description: quiz.description.clone(),
        tasks: order.iter().filter_map(|&index| presented[index].take()).collect(),
    }
}

fn present(random: &mut Mt19937, task: &Task) -> SheetTask {
    match task {
        Task::Classification(task) => {
            let items = drawn(random, task.items.len(), task.draw);
            SheetTask::Classification(SheetClassificationTask {
                id: task.id.clone(),
                title: task.title.clone(),
                prompt: task.prompt.clone(),
                axes: task.axes.clone(),
                categories: shuffle(random, &task.categories),
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label)).collect(),
            })
        }
        Task::Sorting(task) => {
            let mut items = drawn(random, task.items.len(), task.draw);
            if items.len() >= 2 && ascending(&task.items, &items) {
                items.rotate_left(1);
            }
            SheetTask::Sorting(SheetSortingTask {
                id: task.id.clone(),
                title: task.title.clone(),
                prompt: task.prompt.clone(),
                quantity: task.quantity.clone(),
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label)).collect(),
            })
        }
        Task::Matching(task) => {
            let items = drawn(random, task.items.len(), task.draw);
            SheetTask::Matching(SheetMatchingTask {
                id: task.id.clone(),
                title: task.title.clone(),
                prompt: task.prompt.clone(),
                dimensions: task.dimensions.iter().map(|dimension| SheetDimension { id: dimension.id.clone(), quantity: dimension.quantity.clone(), cards: shuffle(random, &values(task, &items, &dimension.id)) }).collect(),
                items: items.iter().map(|&index| sheet_item(&task.items[index].id, &task.items[index].label)).collect(),
            })
        }
    }
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

fn sheet_item(id: &str, label: &Text) -> SheetItem {
    SheetItem { id: id.to_string(), label: label.clone() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
