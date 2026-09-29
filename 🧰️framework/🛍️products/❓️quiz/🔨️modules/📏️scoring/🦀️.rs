//! 📏️ Partial-credit scoring of a run (design §6): magnitude-weighted pair concordance for sorting and
//! matching, profile similarity for classification; the run score is the mean of the task scores in
//! sheet order. Sums run left to right from `0` in the specified order so every core agrees within
//! 1e-12 (only `log10` may differ by one ulp); a perfect answer scores exactly `1`.
//!
//! @see <https://en.wikipedia.org/wiki/Kendall_rank_correlation_coefficient> — pair concordance
//! @see ../📏️scoring/🟦️.ts — the TypeScript twin

use crate::schema::{
    Answer, Category, ClassificationAnswer, ClassificationItemResult, ClassificationTask, DimensionResult, MatchingAnswer, MatchingItem, MatchingItemResult, MatchingTask, Quiz, RunResult, Scale, Sheet, SheetClassificationTask, SheetMatchingTask, SheetTask, Slug,
    SortingAnswer, SortingItemResult, SortingTask, Task, TaskResult,
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

/// 🥢️ Score one task. `None` unless the sheet task presents `task` and the answer is valid and complete.
pub fn score_task(task: &Task, sheet_task: &SheetTask, answer: &Answer) -> Option<TaskResult> {
    if task.id() != sheet_task.id() || answer_rejection(sheet_task, answer).is_some() || !answer_complete(sheet_task, Some(answer)) {
        return None;
    }
    match (task, sheet_task, answer) {
        (Task::Classification(task), SheetTask::Classification(sheet), Answer::Classification(answer)) => classification(task, sheet, answer),
        (Task::Sorting(task), SheetTask::Sorting(_), Answer::Sorting(answer)) => sorting(task, answer),
        (Task::Matching(task), SheetTask::Matching(sheet), Answer::Matching(answer)) => matching(task, sheet, answer),
        _ => None,
    }
}

/// 🎊️ Score a whole run: every sheet task in sheet order, the run score being their mean. `None`
/// unless every sheet task has a valid, complete answer.
pub fn score_run(quiz: &Quiz, sheet: &Sheet, answers: &BTreeMap<Slug, Answer>) -> Option<RunResult> {
    let tasks: Vec<TaskResult> = sheet
        .tasks
        .iter()
        .map(|sheet_task| {
            let task = quiz.tasks.iter().find(|task| task.id() == sheet_task.id())?;
            score_task(task, sheet_task, answers.get(sheet_task.id())?)
        })
        .collect::<Option<_>>()?;
    Some(RunResult { quiz: sheet.quiz.clone(), score: mean(tasks.iter().map(TaskResult::score)), tasks })
}

fn mean(values: impl ExactSizeIterator<Item = f64>) -> f64 {
    let count = values.len();
    if count == 0 {
        return 0.0;
    }
    values.fold(0.0, |sum, value| sum + value) / count as f64
}

fn concordance(total: f64, discordant: f64) -> f64 {
    if total > 0.0 {
        1.0 - discordant / total
    } else {
        1.0
    }
}

fn sorting(task: &SortingTask, answer: &SortingAnswer) -> Option<TaskResult> {
    let items: Vec<(usize, &crate::schema::SortingItem)> = answer.order.iter().map(|id| task.items.iter().enumerate().find(|(_, item)| &item.id == id)).collect::<Option<_>>()?;
    let values: Vec<f64> = items.iter().map(|(_, item)| item.value).collect();
    let scaled: Vec<f64> = values.iter().map(|&value| scaled(task.quantity.scale, value)).collect();
    let (mut total, mut discordant) = (0.0, 0.0);
    for i in 0..items.len() {
        for j in i + 1..items.len() {
            let weight = (scaled[i] - scaled[j]).abs();
            total += weight;
            if values[i] > values[j] {
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
        score: concordance(total, discordant),
        items: items.iter().enumerate().map(|(position, (_, item))| SortingItemResult { item: item.id.clone(), value: item.value, position, rank: ranks[position], explanation: item.explanation.clone() }).collect(),
    })
}

fn matching(task: &MatchingTask, sheet: &SheetMatchingTask, answer: &MatchingAnswer) -> Option<TaskResult> {
    let items: Vec<&MatchingItem> = sheet.items.iter().map(|presented| task.items.iter().find(|item| item.id == presented.id)).collect::<Option<_>>()?;
    let dimensions: Vec<DimensionResult> = sheet
        .dimensions
        .iter()
        .map(|presented| {
            let scale = task.dimensions.iter().find(|dimension| dimension.id == presented.id)?.quantity.scale;
            let assignment = answer.assignments.get(&presented.id)?;
            let correct: Vec<f64> = items.iter().map(|item| item.values.get(&presented.id).copied()).collect::<Option<_>>()?;
            let assigned: Vec<f64> = items.iter().map(|item| assignment.get(&item.id).and_then(|&card| presented.cards.get(card).copied())).collect::<Option<_>>()?;
            let (mut total, mut discordant) = (0.0, 0.0);
            for i in 0..items.len() {
                for j in i + 1..items.len() {
                    let weight = (scaled(scale, correct[i]) - scaled(scale, correct[j])).abs();
                    total += weight;
                    if (correct[i] > correct[j] && assigned[i] < assigned[j]) || (correct[i] < correct[j] && assigned[i] > assigned[j]) {
                        discordant += weight;
                    } else if assigned[i] == assigned[j] && correct[i] != correct[j] {
                        discordant += weight / 2.0;
                    }
                }
            }
            Some(DimensionResult {
                dimension: presented.id.clone(),
                score: concordance(total, discordant),
                items: items.iter().enumerate().map(|(index, item)| MatchingItemResult { item: item.id.clone(), assigned: assigned[index], correct: correct[index], explanation: item.explanation.clone() }).collect(),
            })
        })
        .collect::<Option<_>>()?;
    Some(TaskResult::Matching { task: task.id.clone(), score: mean(dimensions.iter().map(|dimension| dimension.score)), dimensions })
}

fn classification(task: &ClassificationTask, sheet: &SheetClassificationTask, answer: &ClassificationAnswer) -> Option<TaskResult> {
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
            let assigned = answer.assignments.get(&item.id)?;
            let credit = if *assigned == item.category {
                1.0
            } else {
                match (profile(assigned), profile(&item.category)) {
                    (Some(left), Some(right)) if largest > 0.0 => (1.0 - distance(left, right) / largest).max(0.0),
                    _ => 0.0,
                }
            };
            Some(ClassificationItemResult { item: item.id.clone(), assigned: assigned.clone(), correct: item.category.clone(), credit, explanation: item.explanation.clone() })
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
