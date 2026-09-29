//! 🏅️ Badge rules (design §7): evaluated after every submission, in catalog badge order, over all
//! submitted results of the learner including the new one; badges already held are skipped.
//!
//! A rule over nothing never awards: a `perfect-tasks` selector matching no catalog task and a
//! `completed-quizzes` rule over no quizzes stay unearned.
//!
//! @see ../🏅️badges/🟦️.ts — the TypeScript twin

use crate::schema::{Badge, BadgeRule, Quiz, RunResult, Slug};
use std::borrow::Borrow;
use std::collections::BTreeSet;

/// 🎁️ The ids of the badges newly earned by `results`, in catalog badge order, skipping `held` ones.
pub fn earned_badges<Q: Borrow<Quiz>, R: Borrow<RunResult>>(badges: &[Badge], quizzes: &[Q], results: &[R], held: &BTreeSet<Slug>) -> Vec<Slug> {
    badges.iter().filter(|badge| !held.contains(&badge.id) && earned(&badge.rule, quizzes, results)).map(|badge| badge.id.clone()).collect()
}

fn earned<Q: Borrow<Quiz>, R: Borrow<RunResult>>(rule: &BadgeRule, quizzes: &[Q], results: &[R]) -> bool {
    match rule {
        BadgeRule::PerfectQuiz { quiz } => results.iter().map(Borrow::borrow).any(|result| &result.quiz == quiz && result.score == 1.0),
        BadgeRule::PerfectTasks { task_kind, quiz } => {
            let mut selected = quizzes
                .iter()
                .map(Borrow::borrow)
                .filter(|candidate| quiz.as_ref().is_none_or(|quiz| &candidate.id == quiz))
                .flat_map(|candidate| candidate.tasks.iter().filter(|task| task_kind.is_none_or(|kind| task.kind() == kind)).map(move |task| (&candidate.id, task.id())))
                .peekable();
            selected.peek().is_some() && selected.all(|(quiz, task)| results.iter().map(Borrow::borrow).any(|result| &result.quiz == quiz && result.tasks.iter().any(|scored| scored.task() == task && scored.score() == 1.0)))
        }
        BadgeRule::CompletedQuizzes => !quizzes.is_empty() && quizzes.iter().map(Borrow::borrow).all(|quiz| results.iter().map(Borrow::borrow).any(|result| result.quiz == quiz.id)),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
