//! 👪️ What the learners answered and scored, kept incrementally (design §17 and §20): one
//! [`CrowdTally`] per quiz, folded from each `run-submitted` result as it is committed and
//! materialized into the quiz core's `CrowdView` against the quiz's current definition.
//!
//! A tally keeps the bins of the run scores and, per task (and matching dimension), the bins of the
//! scores that counted for it and per item the answer counts per category or value; for sortings,
//! how many normalized positions were given, their running sum and how often each position of each
//! order length was given. Runs of every challenge are mixed. An item left unanswered counts
//! nowhere, a sorting nobody guessed in adds its score only (`quiz::crowd_orders`), and a guessed
//! matching value counts under the authored value nearest to it (`quiz::crowd_value`) — the one
//! thing a tally takes from the definition it is folded with, which is the catalog's current one:
//! a changed catalog refolds every tally. The view takes the places a sheet presents from the
//! current quiz and bins the positions against them, so a changed `draw` re-bins every earlier
//! answer. Folding a result costs O(its size) however many runs came before, and the materialized
//! view equals `quiz::crowd_view(quiz, results)` over every result in commit order bit for bit: a
//! result counts its first task per id and kind, and its first item (and dimension) per id, exactly
//! like the core; the sums are accumulated in the same order and stored as their exact bits, never
//! as decimal text.
//!
//! @see ../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/👁️views/🦀️.rs — `crowd_view`, the rules this folds by
//! @see ../🔭️projections/🦀️.rs — where tallies and views are kept

use std::collections::{BTreeMap, HashSet};

use quiz::{crowd_orders, crowd_value, json_number_text, place_bin, presented, score_bin, CrowdCount, CrowdItem, CrowdScores, CrowdTask, CrowdView, Quiz, RunResult, Task, TaskKind, TaskResult};
use serde::{Deserialize, Serialize};

/// 🔢️ How often each category or value was answered, by key.
pub type Counts = BTreeMap<String, usize>;

/// 📍️ The positions one sorting item was given: how many, the exact bits of the sum of their
/// normalized positions, and how often each position, by the length of the order and the position
/// in it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Positions {
    pub answers: usize,
    pub sum: u64,
    pub orders: BTreeMap<usize, BTreeMap<usize, usize>>,
}

impl Positions {
    fn add(&mut self, position: usize, length: usize) {
        self.sum = (f64::from_bits(self.sum) + if length < 2 { 0.0 } else { position as f64 / (length - 1) as f64 }).to_bits();
        self.answers += 1;
        *self.orders.entry(length).or_default().entry(position).or_default() += 1;
    }

    fn mean(&self) -> f64 {
        f64::from_bits(self.sum) / self.answers as f64
    }

    fn places(&self, count: usize) -> Vec<usize> {
        let mut places = vec![0; count];
        for (&length, positions) in &self.orders {
            for (&position, given) in positions {
                if let Some(place) = places.get_mut(place_bin(position, length, count)) {
                    *place += given;
                }
            }
        }
        places
    }
}

/// 🗳️ What counted for one crowd task: the bins of its scores and what was answered per item id.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tallied<T> {
    pub scores: CrowdScores,
    pub items: BTreeMap<String, T>,
}

/// 🧮️ Everything submitted for one quiz: the bins of the run scores and, by task id, the task scores
/// with the classification counts per item and category, the task scores with the sorting positions
/// per item, and per matching dimension its scores with the counts per item and value.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrowdTally {
    pub runs: usize,
    pub scores: CrowdScores,
    pub classification: BTreeMap<String, Tallied<Counts>>,
    pub sorting: BTreeMap<String, Tallied<Positions>>,
    pub matching: BTreeMap<String, BTreeMap<String, Tallied<Counts>>>,
}

impl CrowdTally {
    /// 🧺️ Fold one submitted result of `quiz`, this tally's quiz as the catalog defines it now.
    pub fn fold(&mut self, quiz: &Quiz, result: &RunResult) {
        self.runs += 1;
        self.scores[score_bin(result.score)] += 1;
        let mut tasks = HashSet::new();
        for scored in result.tasks.iter().filter(|scored| tasks.insert((std::mem::discriminant(*scored), scored.task()))) {
            match scored {
                TaskResult::Classification { task, score, items } => {
                    let tally = self.classification.entry(task.clone()).or_default();
                    tally.scores[score_bin(*score)] += 1;
                    for (item, assigned) in first(items, |item| &item.item).filter_map(|item| Some((&item.item, item.assigned.as_ref()?))) {
                        *tally.items.entry(item.clone()).or_default().entry(assigned.clone()).or_default() += 1;
                    }
                }
                TaskResult::Sorting { task, score, items } => {
                    let tally = self.sorting.entry(task.clone()).or_default();
                    tally.scores[score_bin(*score)] += 1;
                    for item in first(items, |item| &item.item).filter(|_| crowd_orders(items)) {
                        tally.items.entry(item.item.clone()).or_default().add(item.position, items.len());
                    }
                }
                TaskResult::Matching { task, dimensions, .. } => {
                    let defined = quiz.tasks.iter().find_map(|defined| match defined {
                        Task::Matching(defined) if defined.id == *task => Some(defined),
                        _ => None,
                    });
                    let tally = self.matching.entry(task.clone()).or_default();
                    for dimension in first(dimensions, |dimension| &dimension.dimension) {
                        let tally = tally.entry(dimension.dimension.clone()).or_default();
                        tally.scores[score_bin(dimension.score)] += 1;
                        let Some((defined, quantity)) = defined.and_then(|defined| Some((defined, defined.dimensions.iter().find(|quantity| quantity.id == dimension.dimension)?))) else { continue };
                        for (item, value) in first(&dimension.items, |item| &item.item).filter_map(|item| Some((&item.item, crowd_value(defined, quantity, item)?))) {
                            *tally.items.entry(item.clone()).or_default().entry(json_number_text(value)).or_default() += 1;
                        }
                    }
                }
            }
        }
    }

    /// 🪟️ The crowd view of `quiz`: the run scores, then its tasks in definition order (matching once
    /// per dimension), per task its scores and the items in definition order somebody answered, a
    /// sorting item's positions binned against the places a sheet of the task presents now.
    pub fn view(&self, quiz: &Quiz) -> CrowdView {
        let mut tasks = Vec::new();
        for task in &quiz.tasks {
            match task {
                Task::Classification(definition) => {
                    let tally = self.classification.get(&definition.id);
                    let items = definition.items.iter().filter_map(|item| counted(&item.id, tally.and_then(|tally| tally.items.get(&item.id)))).collect();
                    tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Classification, dimension: None, scores: tally.map(|tally| tally.scores).unwrap_or_default(), items });
                }
                Task::Sorting(definition) => {
                    let tally = self.sorting.get(&definition.id);
                    let count = presented(task);
                    let items = definition
                        .items
                        .iter()
                        .filter_map(|item| tally.and_then(|tally| tally.items.get(&item.id)).filter(|positions| positions.answers > 0).map(|positions| CrowdItem { item: item.id.clone(), answers: positions.answers, counts: None, mean_position: Some(positions.mean()), places: Some(positions.places(count)) }))
                        .collect();
                    tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Sorting, dimension: None, scores: tally.map(|tally| tally.scores).unwrap_or_default(), items });
                }
                Task::Matching(definition) => {
                    for dimension in &definition.dimensions {
                        let tally = self.matching.get(&definition.id).and_then(|tally| tally.get(&dimension.id));
                        let items = definition.items.iter().filter_map(|item| counted(&item.id, tally.and_then(|tally| tally.items.get(&item.id)))).collect();
                        tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Matching, dimension: Some(dimension.id.clone()), scores: tally.map(|tally| tally.scores).unwrap_or_default(), items });
                    }
                }
            }
        }
        CrowdView { quiz: quiz.id.clone(), runs: self.runs, scores: self.scores, tasks }
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
    (answers > 0).then(|| CrowdItem { item: item.to_string(), answers, counts: Some(counts.iter().map(|(key, count)| CrowdCount { key: key.clone(), count: *count }).collect()), mean_position: None, places: None })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
