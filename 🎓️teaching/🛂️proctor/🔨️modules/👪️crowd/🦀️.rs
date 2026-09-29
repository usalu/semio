//! 👪️ What the learners answered, kept incrementally (design §17): one [`CrowdTally`] per quiz,
//! folded from each `run-submitted` result as it is committed and materialized into the quiz core's
//! `CrowdView` against the quiz's current definition.
//!
//! A tally keeps per task (and matching dimension) and item the answer counts per category or value
//! and, for sortings, how many normalized positions were given and their running sum. Folding a
//! result costs O(its size) however many runs came before, and the materialized view equals
//! `quiz::crowd_view(quiz, results)` over every result in commit order bit for bit: a result counts
//! its first task per id and kind, and its first item (and dimension) per id, exactly like the core;
//! the sums are accumulated in the same order and stored as their exact bits, never as decimal text.
//!
//! @see ../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/👁️views/🦀️.rs — `crowd_view`, the rules this folds by
//! @see ../🔭️projections/🦀️.rs — where tallies and views are kept

use std::collections::{BTreeMap, HashSet};

use quiz::{json_number_text, CrowdCount, CrowdItem, CrowdTask, CrowdView, Quiz, RunResult, Task, TaskKind, TaskResult};
use serde::{Deserialize, Serialize};

/// 🔢️ How often each category or value was answered, by key.
pub type Counts = BTreeMap<String, usize>;

/// 📍️ The normalized positions one sorting item was given: how many, and the exact bits of their sum.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Positions {
    pub answers: usize,
    pub sum: u64,
}

impl Positions {
    fn add(&mut self, position: f64) {
        self.sum = (f64::from_bits(self.sum) + position).to_bits();
        self.answers += 1;
    }

    fn mean(self) -> f64 {
        f64::from_bits(self.sum) / self.answers as f64
    }
}

/// 🧮️ Everything submitted for one quiz, by task id: classification counts per item and category,
/// sorting positions per item, matching counts per dimension, item and value.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrowdTally {
    pub runs: usize,
    pub classification: BTreeMap<String, BTreeMap<String, Counts>>,
    pub sorting: BTreeMap<String, BTreeMap<String, Positions>>,
    pub matching: BTreeMap<String, BTreeMap<String, BTreeMap<String, Counts>>>,
}

impl CrowdTally {
    /// 🧺️ Fold one submitted result of this tally's quiz.
    pub fn fold(&mut self, result: &RunResult) {
        self.runs += 1;
        let mut tasks = HashSet::new();
        for scored in result.tasks.iter().filter(|scored| tasks.insert((std::mem::discriminant(*scored), scored.task()))) {
            match scored {
                TaskResult::Classification { task, items, .. } => {
                    let tally = self.classification.entry(task.clone()).or_default();
                    for item in first(items, |item| &item.item) {
                        *tally.entry(item.item.clone()).or_default().entry(item.assigned.clone()).or_default() += 1;
                    }
                }
                TaskResult::Sorting { task, items, .. } => {
                    let tally = self.sorting.entry(task.clone()).or_default();
                    for item in first(items, |item| &item.item) {
                        tally.entry(item.item.clone()).or_default().add(if items.len() < 2 { 0.0 } else { item.position as f64 / (items.len() - 1) as f64 });
                    }
                }
                TaskResult::Matching { task, dimensions, .. } => {
                    let tally = self.matching.entry(task.clone()).or_default();
                    for dimension in first(dimensions, |dimension| &dimension.dimension) {
                        let tally = tally.entry(dimension.dimension.clone()).or_default();
                        for item in first(&dimension.items, |item| &item.item) {
                            *tally.entry(item.item.clone()).or_default().entry(json_number_text(item.assigned)).or_default() += 1;
                        }
                    }
                }
            }
        }
    }

    /// 🪟️ The crowd view of `quiz`: its tasks in definition order (matching once per dimension), per
    /// task the items in definition order somebody answered.
    pub fn view(&self, quiz: &Quiz) -> CrowdView {
        let mut tasks = Vec::new();
        for task in &quiz.tasks {
            match task {
                Task::Classification(definition) => {
                    let tally = self.classification.get(&definition.id);
                    let items = definition.items.iter().filter_map(|item| counted(&item.id, tally.and_then(|tally| tally.get(&item.id)))).collect();
                    tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Classification, dimension: None, items });
                }
                Task::Sorting(definition) => {
                    let tally = self.sorting.get(&definition.id);
                    let items = definition
                        .items
                        .iter()
                        .filter_map(|item| tally.and_then(|tally| tally.get(&item.id)).filter(|positions| positions.answers > 0).map(|positions| CrowdItem { item: item.id.clone(), answers: positions.answers, counts: None, mean_position: Some(positions.mean()) }))
                        .collect();
                    tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Sorting, dimension: None, items });
                }
                Task::Matching(definition) => {
                    for dimension in &definition.dimensions {
                        let tally = self.matching.get(&definition.id).and_then(|tally| tally.get(&dimension.id));
                        let items = definition.items.iter().filter_map(|item| counted(&item.id, tally.and_then(|tally| tally.get(&item.id)))).collect();
                        tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Matching, dimension: Some(dimension.id.clone()), items });
                    }
                }
            }
        }
        CrowdView { quiz: quiz.id.clone(), runs: self.runs, tasks }
    }
}

/// 🥇️ Each element of `all` whose id no earlier element had.
fn first<T>(all: &[T], id: impl Fn(&T) -> &String) -> impl Iterator<Item = &T> {
    let mut seen = HashSet::new();
    all.iter().filter(move |element| seen.insert(id(element).clone()))
}

fn counted(item: &str, counts: Option<&Counts>) -> Option<CrowdItem> {
    let counts = counts?;
    let answers = counts.values().sum::<usize>();
    (answers > 0).then(|| CrowdItem { item: item.to_string(), answers, counts: Some(counts.iter().map(|(key, count)| CrowdCount { key: key.clone(), count: *count }).collect()), mean_position: None })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
