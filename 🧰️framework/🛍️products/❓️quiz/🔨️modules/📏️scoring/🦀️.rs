//! 📏️ Partial-credit scoring of a run: magnitude-weighted pair concordance for sorting and matching,
//! profile similarity for classification; the run score is the mean of the task scores in sheet order
//! and means accuracy at every challenge, the points are that score times the par of the challenge.
//! Sums run left to right from `0` in the specified order so every core agrees within 1e-12 (only
//! `log10` may differ by one ulp); a perfect answer scores exactly `1`.
//!
//! Where a sheet task hides the keys the learner's guesses are the answer: an item misses without a
//! guess or with a guess beyond the reach of the presented true values, and a miss costs every pair it
//! touches. A timed sheet task (one that carries `seconds`) is also scored without an answer or with
//! an incomplete one: what is missing scores as a miss.
//!
//! @see <https://en.wikipedia.org/wiki/Kendall_rank_correlation_coefficient> — pair concordance
//! @see ../⛰️challenge/🦀️.rs — `reach`, `misses` and `points`
//! @see ../📏️scoring/🟦️.ts — the TypeScript twin

use crate::challenge::{misses, points, reach};
use crate::schema::{
    Answer, Category, ClassificationAnswer, ClassificationItemResult, ClassificationTask, DimensionResult, MatchingAnswer, MatchingItem, MatchingItemResult, MatchingTask, Quiz, RunResult, Scale, Sheet, SheetClassificationTask, SheetMatchingTask, SheetSortingTask, SheetTask,
    Slug, SortingAnswer, SortingItem, SortingItemResult, SortingTask, Task, TaskResult,
};
use crate::validation::{answer_complete, answer_rejection};
use std::cmp::Ordering;
use std::collections::BTreeMap;

/// 📉️ `s(v)`: the value itself on a linear scale, `log10(v)` on a logarithmic one.
pub fn scaled(scale: Scale, value: f64) -> f64 {
    match scale {
        Scale::Linear => value,
        Scale::Logarithmic => value.log10(),
    }
}

/// 🥢️ Score one task. `None` unless the sheet task presents `task` and the answer is valid and
/// complete. A timed sheet task (one that carries `seconds`) is also scored without an answer or with
/// an incomplete one: what is missing scores as a miss.
pub fn score_task(task: &Task, sheet_task: &SheetTask, answer: Option<&Answer>) -> Option<TaskResult> {
    let timed = sheet_task.seconds().is_some();
    if task.id() != sheet_task.id() || answer.map_or(!timed, |answer| answer_rejection(sheet_task, answer).is_some()) || (!timed && !answer_complete(sheet_task, answer)) {
        return None;
    }
    match (task, sheet_task, answer) {
        (Task::Classification(task), SheetTask::Classification(sheet), None) => classification(task, sheet, None),
        (Task::Classification(task), SheetTask::Classification(sheet), Some(Answer::Classification(answer))) => classification(task, sheet, Some(answer)),
        (Task::Sorting(task), SheetTask::Sorting(sheet), None) => sorting(task, sheet, None),
        (Task::Sorting(task), SheetTask::Sorting(sheet), Some(Answer::Sorting(answer))) => sorting(task, sheet, Some(answer)),
        (Task::Matching(task), SheetTask::Matching(sheet), None) => matching(task, sheet, None),
        (Task::Matching(task), SheetTask::Matching(sheet), Some(Answer::Matching(answer))) => matching(task, sheet, Some(answer)),
        _ => None,
    }
}

/// 🎊️ Score a whole run at the challenge of its sheet: every sheet task in sheet order, the run score
/// being their mean and the points what it earns at the challenge. `None` unless every sheet task has
/// a valid, complete answer; on a timed sheet a task may have no answer or an incomplete one.
pub fn score_run(quiz: &Quiz, sheet: &Sheet, answers: &BTreeMap<Slug, Answer>) -> Option<RunResult> {
    let tasks: Vec<TaskResult> = sheet
        .tasks
        .iter()
        .map(|sheet_task| {
            let task = quiz.tasks.iter().find(|task| task.id() == sheet_task.id())?;
            score_task(task, sheet_task, answers.get(sheet_task.id()))
        })
        .collect::<Option<_>>()?;
    let score = mean(tasks.iter().map(TaskResult::score));
    Some(RunResult { quiz: sheet.quiz.clone(), challenge: sheet.challenge, score, points: points(score, sheet.challenge), tasks })
}

fn mean(values: impl ExactSizeIterator<Item = f64>) -> f64 {
    let count = values.len();
    if count == 0 {
        return 0.0;
    }
    values.fold(0.0, |sum, value| sum + value) / count as f64
}

fn concordance(total: f64, discordant: f64, missed: &[bool]) -> f64 {
    if total > 0.0 {
        1.0 - discordant / total
    } else if missed.contains(&true) {
        0.0
    } else {
        1.0
    }
}

fn sorting(task: &SortingTask, sheet: &SheetSortingTask, answer: Option<&SortingAnswer>) -> Option<TaskResult> {
    let found = |id: &Slug| task.items.iter().enumerate().find(|(_, item)| &item.id == id);
    let items: Vec<(usize, &SortingItem)> = match answer {
        Some(answer) => answer.order.iter().map(found).collect::<Option<_>>()?,
        None => sheet.items.iter().map(|presented| found(&presented.id)).collect::<Option<_>>()?,
    };
    let scale = task.quantity.scale;
    let hidden = sheet.keys.is_none();
    let values: Vec<f64> = items.iter().map(|(_, item)| item.value).collect();
    let within = reach(&values, scale);
    let guesses: Vec<Option<f64>> = items.iter().map(|(_, item)| answer.filter(|_| hidden).and_then(|answer| answer.guesses.as_ref()).and_then(|guesses| guesses.get(&item.id).copied())).collect();
    let missed: Vec<bool> = guesses.iter().zip(&values).map(|(guess, &value)| if hidden { guess.is_none_or(|guess| misses(guess, value, scale, within)) } else { answer.is_none() }).collect();
    let scaled: Vec<f64> = values.iter().map(|&value| scaled(scale, value)).collect();
    let (mut total, mut discordant) = (0.0, 0.0);
    for i in 0..items.len() {
        for j in i + 1..items.len() {
            let weight = (scaled[i] - scaled[j]).abs();
            total += weight;
            if values[i] > values[j] || missed[i] || missed[j] {
                discordant += weight;
            }
        }
    }
    let mut ascending: Vec<usize> = (0..items.len()).collect();
    ascending.sort_by(|&a, &b| values[a].partial_cmp(&values[b]).unwrap_or(Ordering::Equal).then(items[a].0.cmp(&items[b].0)));
    let mut ranks = vec![0; items.len()];
    for (rank, &position) in ascending.iter().enumerate() {
        ranks[position] = rank;
    }
    Some(TaskResult::Sorting {
        task: task.id.clone(),
        score: concordance(total, discordant, &missed),
        items: items
            .iter()
            .enumerate()
            .map(|(position, (_, item))| SortingItemResult { item: item.id.clone(), value: item.value, position, rank: ranks[position], guess: guesses[position], miss: hidden.then_some(missed[position]), explanation: item.explanation.clone() })
            .collect(),
    })
}

fn matching(task: &MatchingTask, sheet: &SheetMatchingTask, answer: Option<&MatchingAnswer>) -> Option<TaskResult> {
    let items: Vec<&MatchingItem> = sheet.items.iter().map(|presented| task.items.iter().find(|item| item.id == presented.id)).collect::<Option<_>>()?;
    let dimensions: Vec<DimensionResult> = sheet
        .dimensions
        .iter()
        .map(|presented| {
            let scale = task.dimensions.iter().find(|dimension| dimension.id == presented.id)?.quantity.scale;
            let correct: Vec<f64> = items.iter().map(|item| item.values.get(&presented.id).copied()).collect::<Option<_>>()?;
            let assigned: Vec<Option<f64>> = match &presented.cards {
                Some(cards) => {
                    let given = answer.and_then(|answer| answer.assignments.as_ref()).and_then(|assignments| assignments.get(&presented.id));
                    items.iter().map(|item| given.and_then(|given| given.get(&item.id)).and_then(|&card| cards.get(card).copied())).collect()
                }
                None => {
                    let given = answer.and_then(|answer| answer.guesses.as_ref()).and_then(|guesses| guesses.get(&presented.id));
                    items.iter().map(|item| given.and_then(|given| given.get(&item.id).copied())).collect()
                }
            };
            let hidden = presented.cards.is_none();
            let within = reach(&correct, scale);
            let missed: Vec<bool> = assigned.iter().zip(&correct).map(|(assigned, &truth)| assigned.is_none_or(|assigned| hidden && misses(assigned, truth, scale, within))).collect();
            let (mut total, mut discordant) = (0.0, 0.0);
            for i in 0..items.len() {
                for j in i + 1..items.len() {
                    let weight = (scaled(scale, correct[i]) - scaled(scale, correct[j])).abs();
                    total += weight;
                    match (assigned[i], assigned[j]) {
                        (Some(left), Some(right)) if !missed[i] && !missed[j] => {
                            if (correct[i] > correct[j] && left < right) || (correct[i] < correct[j] && left > right) {
                                discordant += weight;
                            } else if left == right && correct[i] != correct[j] {
                                discordant += weight / 2.0;
                            }
                        }
                        _ => discordant += weight,
                    }
                }
            }
            Some(DimensionResult {
                dimension: presented.id.clone(),
                score: concordance(total, discordant, &missed),
                items: items.iter().enumerate().map(|(index, item)| MatchingItemResult { item: item.id.clone(), assigned: assigned[index], correct: correct[index], miss: hidden.then_some(missed[index]), explanation: item.explanation.clone() }).collect(),
            })
        })
        .collect::<Option<_>>()?;
    Some(TaskResult::Matching { task: task.id.clone(), score: mean(dimensions.iter().map(|dimension| dimension.score)), dimensions })
}

fn classification(task: &ClassificationTask, sheet: &SheetClassificationTask, answer: Option<&ClassificationAnswer>) -> Option<TaskResult> {
    let axes = task.axes.as_deref().unwrap_or_default();
    let normalized = |category: &Category| -> Option<Vec<f64>> {
        let profile = category.profile.as_ref()?;
        axes.iter().map(|axis| profile.get(&axis.id).map(|&value| (value - axis.min) / (axis.max - axis.min))).collect()
    };
    let profiles: Vec<Option<Vec<f64>>> = task.categories.iter().map(normalized).collect();
    let mut largest = 0.0f64;
    for i in 0..profiles.len() {
        for j in i + 1..profiles.len() {
            if let (Some(left), Some(right)) = (&profiles[i], &profiles[j]) {
                largest = largest.max(distance(left, right));
            }
        }
    }
    let profile = |id: &str| task.categories.iter().position(|category| category.id == id).and_then(|index| profiles[index].as_ref());
    let items: Vec<ClassificationItemResult> = sheet
        .items
        .iter()
        .map(|presented| {
            let item = task.items.iter().find(|item| item.id == presented.id)?;
            let assigned = answer.and_then(|answer| answer.assignments.get(&presented.id));
            let credit = match assigned {
                None => 0.0,
                Some(assigned) if *assigned == item.category => 1.0,
                Some(assigned) => match (profile(assigned), profile(&item.category)) {
                    (Some(left), Some(right)) if largest > 0.0 => (1.0 - distance(left, right) / largest).max(0.0),
                    _ => 0.0,
                },
            };
            Some(ClassificationItemResult { item: item.id.clone(), assigned: assigned.cloned(), correct: item.category.clone(), credit, explanation: item.explanation.clone() })
        })
        .collect::<Option<_>>()?;
    Some(TaskResult::Classification { task: task.id.clone(), score: mean(items.iter().map(|item| item.credit)), items })
}

fn distance(left: &[f64], right: &[f64]) -> f64 {
    left.iter().zip(right).map(|(a, b)| (a - b) * (a - b)).fold(0.0, |sum, square| sum + square).sqrt()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
