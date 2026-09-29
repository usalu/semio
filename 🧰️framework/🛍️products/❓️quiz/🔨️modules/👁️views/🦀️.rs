//! 👁️ The four read models a proctor answers (`CatalogView`, `LearnerView`, `RunView`,
//! `Leaderboard`), derived from the catalog and the learner states.
//!
//! Best scores are the maximum submitted score per quiz id; totals sum `best × 100` over the catalog
//! quizzes in catalog order; `reachedAt` is the submission time of the last submission (in
//! submission-time order, ties in start order) that set or raised a best score.
//!
//! @see ../../🧬️schema/🔣️.json — the view contracts
//! @see ../👁️views/🟦️.ts — the TypeScript twin

use crate::lifecycle::{LearnerState, LoadedQuiz, RunState};
use crate::randomness::fnv1a32;
use crate::schema::{Catalog, CatalogBadgeView, CatalogQuizView, CatalogTaskView, CatalogView, Leaderboard, LeaderboardRow, LearnerView, Quiz, RunStatus, RunSummary, RunView, Score, Slug, Timestamp};
use crate::sheet::sheet_of;
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
            .map(|quiz| CatalogQuizView { id: quiz.id.clone(), title: quiz.title.clone(), description: quiz.description.clone(), tasks: quiz.tasks.iter().map(|task| CatalogTaskView { id: task.id().clone(), kind: task.kind(), title: task.title().clone() }).collect() })
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
