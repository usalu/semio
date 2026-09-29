//! 👁️ The read models a proctor answers (`CatalogView`, `LearnerView`, `RunView`, `Leaderboard`,
//! `CrowdView`), derived from the catalog, the learner states and the submitted results.
//!
//! Best scores are the maximum submitted score per quiz id; totals sum `best × 100` over the catalog
//! quizzes in catalog order; `reachedAt` is the submission time of the last submission (in
//! submission-time order, ties in start order) that set or raised a best score.
//!
//! @see ../../🧬️schema/🔣️.json — the view contracts
//! @see ../👁️views/🟦️.ts — the TypeScript twin

use crate::lifecycle::{LearnerState, LoadedQuiz, RunState};
use crate::randomness::fnv1a32;
use crate::schema::{
    Catalog, CatalogBadgeView, CatalogQuizView, CatalogTaskView, CatalogView, CrowdCount, CrowdItem, CrowdTask, CrowdView, Leaderboard, LeaderboardRow, LearnerView, Quiz, RunResult, RunStatus, RunSummary, RunView, Score, Slug, Task, TaskKind, TaskResult, Timestamp,
};
use crate::sheet::sheet_of;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap;

/// 🪟️ The solution-free catalog: quizzes (in the order given, i.e. catalog order) with their task
/// titles and kinds, and badges without rules.
pub fn catalog_view(catalog: &Catalog, quizzes: &[Quiz]) -> CatalogView {
    CatalogView {
        id: catalog.id.clone(),
        title: catalog.title.clone(),
        introduction: catalog.introduction.clone(),
        quizzes: quizzes
            .iter()
            .map(|quiz| CatalogQuizView { id: quiz.id.clone(), emoji: quiz.emoji.clone(), title: quiz.title.clone(), description: quiz.description.clone(), tasks: quiz.tasks.iter().map(|task| CatalogTaskView { id: task.id().clone(), kind: task.kind(), title: task.title().clone() }).collect() })
            .collect(),
        badges: catalog.badges.iter().map(|badge| CatalogBadgeView { id: badge.id.clone(), emoji: badge.emoji.clone(), label: badge.label.clone(), description: badge.description.clone() }).collect(),
    }
}

/// 🧑‍🏫️ A registered learner's runs (newest first), badges, best scores and total; `None` before
/// registration.
pub fn learner_view(state: &LearnerState, catalog: &CatalogView) -> Option<LearnerView> {
    let Standing { best, .. } = standing(state);
    Some(LearnerView {
        learner: state.learner.clone(),
        identity: state.identity.clone()?,
        runs: state.runs.iter().rev().map(|run| RunSummary { run: run.run.clone(), quiz: run.quiz.clone(), status: run.status, score: run.result.as_ref().map(|result| result.score), started_at: run.started_at, submitted_at: run.submitted_at }).collect(),
        badges: state.badges.clone(),
        total: total(&best, catalog),
        best,
    })
}

/// 🔬️ One run with its sheet (recomputed from the current quiz and the run seed), answers and result;
/// `None` when the run or its quiz is unknown.
pub fn run_view(state: &LearnerState, run: &str, quizzes: &BTreeMap<Slug, LoadedQuiz>) -> Option<RunView> {
    let found = state.run(run)?;
    let current = quizzes.get(&found.quiz)?;
    Some(RunView {
        run: found.run.clone(),
        learner: state.learner.clone(),
        quiz: found.quiz.clone(),
        status: found.status,
        sheet: sheet_of(&current.quiz, found.seed),
        answers: found.answers.clone(),
        result: found.result.clone(),
        started_at: found.started_at,
        submitted_at: found.submitted_at,
    })
}

/// 🎫️ The public tag of a learner: FNV-1a of the learner id as 8 lowercase hex digits. A client
/// recognises its own leaderboard row by computing it from its own id.
pub fn learner_tag(learner: &str) -> String {
    format!("{:08x}", fnv1a32(learner))
}

/// 🏟️ Every registered learner with a submitted run, ordered by total descending, badge count
/// descending, `reachedAt` ascending and learner id ascending; ranks are 1-based positions. Rows carry
/// the [`learner_tag`], never the learner id.
pub fn leaderboard<'a>(states: impl IntoIterator<Item = &'a LearnerState>, catalog: &CatalogView) -> Leaderboard {
    let mut rows: Vec<(&str, LeaderboardRow)> = states
        .into_iter()
        .filter_map(|state| {
            let identity = state.identity.clone()?;
            let Standing { best, reached_at, submitted } = standing(state);
            let reached_at = reached_at?;
            Some((
                state.learner.as_str(),
                LeaderboardRow {
                    rank: 0,
                    tag: learner_tag(&state.learner),
                    identity,
                    total: total(&best, catalog),
                    reached_at,
                    best,
                    badges: state.badges.iter().map(|award| award.badge.clone()).collect(),
                    runs: submitted,
                    last_activity: state.last_activity.unwrap_or(reached_at),
                },
            ))
        })
        .collect();
    rows.sort_by(|(left, a), (right, b)| b.total.partial_cmp(&a.total).unwrap_or(Ordering::Equal).then(b.badges.len().cmp(&a.badges.len())).then(a.reached_at.cmp(&b.reached_at)).then_with(|| left.cmp(right)));
    Leaderboard { rows: rows.into_iter().enumerate().map(|(index, (_, row))| LeaderboardRow { rank: index + 1, ..row }).collect() }
}

/// 👪️ What the learners answered in the submitted runs of `quiz` (results of other quizzes are ignored): every
/// task in definition order, matching once per dimension in definition order; per task the items in definition
/// order that at least one result answered. Classification counts the assigned category ids, matching the
/// assigned values as [`json_number_text`]s, both in ascending key order (code point order, so `"120"` precedes
/// `"50"`); sorting gives the mean
/// over the results, in result order, of the normalized position `position / (n − 1)` in the learner's order of
/// `n` items (`0` when `n < 2`). A task result counts only when its kind matches the quiz task.
pub fn crowd_view<R: Borrow<RunResult>>(quiz: &Quiz, results: &[R]) -> CrowdView {
    let results: Vec<&RunResult> = results.iter().map(Borrow::borrow).filter(|result| result.quiz == quiz.id).collect();
    let scored = |task: &Task| -> Vec<&TaskResult> { results.iter().filter_map(|result| result.tasks.iter().find(|scored| scored.task() == task.id() && kind_of(scored) == task.kind())).collect() };
    let mut tasks = Vec::new();
    for task in &quiz.tasks {
        let scored = scored(task);
        match task {
            Task::Classification(definition) => {
                let items = definition.items.iter().filter_map(|item| counted(&item.id, scored.iter().filter_map(|scored| match scored {
                    TaskResult::Classification { items, .. } => items.iter().find(|result| result.item == item.id).map(|result| result.assigned.clone()),
                    _ => None,
                })));
                tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Classification, dimension: None, items: items.collect() });
            }
            Task::Sorting(definition) => {
                let items = definition.items.iter().filter_map(|item| {
                    let positions: Vec<f64> = scored
                        .iter()
                        .filter_map(|scored| match scored {
                            TaskResult::Sorting { items, .. } => items.iter().find(|result| result.item == item.id).map(|result| if items.len() < 2 { 0.0 } else { result.position as f64 / (items.len() - 1) as f64 }),
                            _ => None,
                        })
                        .collect();
                    (!positions.is_empty()).then(|| CrowdItem { item: item.id.clone(), answers: positions.len(), counts: None, mean_position: Some(positions.iter().fold(0.0, |sum, position| sum + position) / positions.len() as f64) })
                });
                tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Sorting, dimension: None, items: items.collect() });
            }
            Task::Matching(definition) => {
                for dimension in &definition.dimensions {
                    let items = definition.items.iter().filter_map(|item| counted(&item.id, scored.iter().filter_map(|scored| match scored {
                        TaskResult::Matching { dimensions, .. } => dimensions.iter().find(|result| result.dimension == dimension.id)?.items.iter().find(|result| result.item == item.id).map(|result| json_number_text(result.assigned)),
                        _ => None,
                    })));
                    tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Matching, dimension: Some(dimension.id.clone()), items: items.collect() });
                }
            }
        }
    }
    CrowdView { quiz: quiz.id.clone(), runs: results.len(), tasks }
}

/// 💱️ A value in JSON number syntax as ECMAScript's `Number.prototype.toString` renders it: the shortest
/// round-trip digits, plain notation for decimal exponents −7 < e < 21, otherwise `d.ddde±x`; `-0` → `"0"`.
pub fn json_number_text(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if !value.is_finite() {
        return if value.is_nan() { "NaN" } else if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let scientific = format!("{:e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let digits = mantissa.replace('.', "");
    let (count, point) = (digits.len() as i32, exponent.parse::<i32>().unwrap_or(0) + 1);
    let body = if count <= point && point <= 21 {
        format!("{digits}{}", "0".repeat((point - count) as usize))
    } else if 0 < point && point <= 21 {
        format!("{}.{}", &digits[..point as usize], &digits[point as usize..])
    } else if -6 < point && point <= 0 {
        format!("0.{}{digits}", "0".repeat((-point) as usize))
    } else {
        let exponent = if point > 0 { format!("+{}", point - 1) } else { (point - 1).to_string() };
        if count == 1 { format!("{digits}e{exponent}") } else { format!("{}.{}e{exponent}", &digits[..1], &digits[1..]) }
    };
    if value < 0.0 { format!("-{body}") } else { body }
}

fn kind_of(scored: &TaskResult) -> TaskKind {
    match scored {
        TaskResult::Classification { .. } => TaskKind::Classification,
        TaskResult::Sorting { .. } => TaskKind::Sorting,
        TaskResult::Matching { .. } => TaskKind::Matching,
    }
}

fn counted(item: &str, keys: impl Iterator<Item = String>) -> Option<CrowdItem> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for key in keys {
        *counts.entry(key).or_insert(0) += 1;
    }
    let answers = counts.values().sum::<usize>();
    (answers > 0).then(|| CrowdItem { item: item.to_string(), answers, counts: Some(counts.into_iter().map(|(key, count)| CrowdCount { key, count }).collect()), mean_position: None })
}

struct Standing {
    best: BTreeMap<Slug, Score>,
    reached_at: Option<Timestamp>,
    submitted: usize,
}

fn standing(state: &LearnerState) -> Standing {
    let mut runs: Vec<(&RunState, Score, Timestamp)> = state.runs.iter().filter(|run| run.status == RunStatus::Submitted).filter_map(|run| Some((run, run.result.as_ref()?.score, run.submitted_at?))).collect();
    runs.sort_by_key(|&(_, _, at)| at);
    let mut best: BTreeMap<Slug, Score> = BTreeMap::new();
    let mut reached_at = None;
    for &(run, score, at) in &runs {
        if best.get(&run.quiz).is_none_or(|&previous| score > previous) {
            best.insert(run.quiz.clone(), score);
            reached_at = Some(at);
        }
    }
    Standing { best, reached_at, submitted: runs.len() }
}

fn total(best: &BTreeMap<Slug, Score>, catalog: &CatalogView) -> f64 {
    catalog.quizzes.iter().filter_map(|quiz| best.get(&quiz.id)).fold(0.0, |sum, score| sum + score * 100.0)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
