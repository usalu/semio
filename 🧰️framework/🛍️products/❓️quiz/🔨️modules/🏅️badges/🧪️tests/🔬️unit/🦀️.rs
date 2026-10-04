//! 🎀️ Unit tests of the badges: every rule, least challenges, held badges and the shared badge vectors.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::challenge::points;
use crate::schema::{Task, TaskKind, TaskResult, CHALLENGES};
use crate::sheet::tests::{quiz, text};

fn badge(id: &str, rule: BadgeRule) -> Badge {
    Badge { id: id.to_string(), emoji: "⭐".to_string(), label: text(id), description: text(id), rule }
}

fn badges() -> Vec<Badge> {
    vec![
        badge("all-done", BadgeRule::CompletedQuizzes {}),
        badge("perfect-energy", BadgeRule::PerfectQuiz { quiz: "energy".to_string(), challenge: None }),
        badge("sorter", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None, challenge: None }),
        badge("energy-classifier", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Classification), quiz: Some("energy".to_string()), challenge: None }),
        badge("heating-master", BadgeRule::PerfectTasks { task_kind: None, quiz: Some("heating".to_string()), challenge: None }),
        badge("nobody", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Matching), quiz: Some("heating".to_string()), challenge: None }),
    ]
}

fn quizzes() -> Vec<Quiz> {
    let mut heating = quiz();
    heating.id = "heating".to_string();
    heating.tasks.retain(|task| matches!(task, Task::Sorting(_)));
    heating.tasks.push(heating.tasks[0].clone());
    if let Task::Sorting(task) = &mut heating.tasks[1] {
        task.id = "power-again".to_string();
    }
    vec![quiz(), heating]
}

fn result_at(quiz: &str, challenge: Challenge, tasks: &[(&str, TaskKind, f64)]) -> RunResult {
    let tasks: Vec<TaskResult> = tasks
        .iter()
        .map(|&(task, kind, score)| match kind {
            TaskKind::Classification => TaskResult::Classification { task: task.to_string(), score, items: Vec::new() },
            TaskKind::Sorting => TaskResult::Sorting { task: task.to_string(), score, items: Vec::new() },
            TaskKind::Matching => TaskResult::Matching { task: task.to_string(), score, dimensions: Vec::new() },
        })
        .collect();
    let score = tasks.iter().map(TaskResult::score).fold(0.0, |sum, score| sum + score) / tasks.len() as f64;
    RunResult { quiz: quiz.to_string(), challenge, score, points: points(score, challenge), tasks }
}

fn result(quiz: &str, tasks: &[(&str, TaskKind, f64)]) -> RunResult {
    result_at(quiz, Challenge::Medium, tasks)
}

fn energy_at(challenge: Challenge, classification: f64, sorting: f64, matching: f64) -> RunResult {
    result_at("energy", challenge, &[("standards", TaskKind::Classification, classification), ("power", TaskKind::Sorting, sorting), ("buildings", TaskKind::Matching, matching)])
}

fn energy(classification: f64, sorting: f64, matching: f64) -> RunResult {
    energy_at(Challenge::Medium, classification, sorting, matching)
}

#[test]
fn nothing_is_earned_without_results() {
    assert!(earned_badges(&badges(), &quizzes(), &[] as &[RunResult], &BTreeSet::new()).is_empty());
}

#[test]
fn perfect_quiz_needs_one_run_scoring_exactly_one() {
    assert_eq!(earned_badges(&badges(), &quizzes(), &[energy(1.0, 0.5, 1.0)], &BTreeSet::new()), ["energy-classifier"]);
    assert_eq!(earned_badges(&badges(), &quizzes(), &[energy(1.0, 1.0, 1.0)], &BTreeSet::new()), ["perfect-energy", "energy-classifier"]);
}

#[test]
fn perfect_tasks_may_collect_perfect_scores_across_runs_and_quizzes() {
    let results = [energy(0.2, 1.0, 0.3), result("heating", &[("power", TaskKind::Sorting, 1.0), ("power-again", TaskKind::Sorting, 0.9)])];
    assert_eq!(earned_badges(&badges(), &quizzes(), &results, &BTreeSet::new()), ["all-done"]);
    let more = [&results[0], &results[1], &result("heating", &[("power", TaskKind::Sorting, 0.1), ("power-again", TaskKind::Sorting, 1.0)])];
    assert_eq!(earned_badges(&badges(), &quizzes(), &more, &BTreeSet::new()), ["all-done", "sorter", "heating-master"]);
}

#[test]
fn held_badges_and_empty_selectors_never_award() {
    let results = [energy(1.0, 1.0, 1.0), result("heating", &[("power", TaskKind::Sorting, 1.0), ("power-again", TaskKind::Sorting, 1.0)])];
    let held = BTreeSet::from(["sorter".to_string(), "all-done".to_string()]);
    assert_eq!(earned_badges(&badges(), &quizzes(), &results, &held), ["perfect-energy", "energy-classifier", "heating-master"]);
    assert!(earned_badges(&badges()[..1], &[] as &[Quiz], &results, &BTreeSet::new()).is_empty());
}

#[test]
fn a_perfect_quiz_with_a_least_challenge_counts_only_runs_that_meet_it() {
    for least in CHALLENGES {
        let rule = [badge("hard-energy", BadgeRule::PerfectQuiz { quiz: "energy".to_string(), challenge: Some(least) })];
        for challenge in CHALLENGES {
            let earned = earned_badges(&rule, &quizzes(), &[energy_at(challenge, 1.0, 1.0, 1.0)], &BTreeSet::new());
            assert_eq!(earned.len(), usize::from(challenge >= least), "{challenge:?} against {least:?}");
        }
    }
    let rule = [badge("hard-energy", BadgeRule::PerfectQuiz { quiz: "energy".to_string(), challenge: Some(Challenge::Hard) })];
    assert!(earned_badges(&rule, &quizzes(), &[energy_at(Challenge::Easy, 1.0, 1.0, 1.0), energy_at(Challenge::Expert, 1.0, 1.0, 0.9)], &BTreeSet::new()).is_empty(), "a perfect easy run and an imperfect expert run do not add up");
    assert_eq!(earned_badges(&rule, &quizzes(), &[energy_at(Challenge::Easy, 0.0, 0.0, 0.0), energy_at(Challenge::Hard, 1.0, 1.0, 1.0)], &BTreeSet::new()), ["hard-energy"]);
}

#[test]
fn perfect_tasks_with_a_least_challenge_collect_only_runs_that_meet_it() {
    let rule = [badge("sorter", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None, challenge: Some(Challenge::Medium) })];
    let heating = |challenge: Challenge, power: f64, again: f64| result_at("heating", challenge, &[("power", TaskKind::Sorting, power), ("power-again", TaskKind::Sorting, again)]);
    assert!(earned_badges(&rule, &quizzes(), &[energy_at(Challenge::Easy, 0.0, 1.0, 0.0), heating(Challenge::Easy, 1.0, 1.0)], &BTreeSet::new()).is_empty());
    assert!(earned_badges(&rule, &quizzes(), &[energy_at(Challenge::Easy, 0.0, 1.0, 0.0), heating(Challenge::Hard, 1.0, 1.0)], &BTreeSet::new()).is_empty(), "the easy perfect sorting does not count");
    assert_eq!(earned_badges(&rule, &quizzes(), &[energy_at(Challenge::Medium, 0.0, 1.0, 0.0), heating(Challenge::Expert, 1.0, 0.0), heating(Challenge::Hard, 0.5, 1.0)], &BTreeSet::new()), ["sorter"], "perfect tasks collect across runs of different challenges that all meet the least one");
    let done = [badge("done", BadgeRule::CompletedQuizzes {})];
    assert_eq!(earned_badges(&done, &quizzes(), &[energy_at(Challenge::Easy, 0.0, 0.0, 0.0), heating(Challenge::Expert, 0.0, 0.0)], &BTreeSet::new()), ["done"], "completing counts at every challenge");
}

#[test]
fn shared_badge_vectors_of_the_python_reference_hold() {
    use crate::schema::tests::{entries, fixture, typed};
    let vectors = fixture("badge-rules");
    let quizzes: Vec<Quiz> = entries(&vectors["quizzes"]).iter().map(typed).collect();
    let badges: Vec<Badge> = entries(&vectors["badges"]).iter().map(typed).collect();
    for vector in entries(&vectors["vectors"]) {
        let results: Vec<RunResult> = entries(&vector["results"]).iter().map(typed).collect();
        let held: BTreeSet<Slug> = entries(&vector["held"]).iter().map(typed).collect();
        let expected: Vec<Slug> = entries(&vector["expected"]).iter().map(typed).collect();
        assert_eq!(earned_badges(&badges, &quizzes, &results, &held), expected, "{}", vector["id"]);
    }
}

#[test]
fn completed_quizzes_counts_any_submitted_result_per_quiz() {
    let only_energy = [energy(0.0, 0.0, 0.0)];
    assert!(!earned_badges(&badges()[..1], &quizzes(), &only_energy, &BTreeSet::new()).contains(&"all-done".to_string()));
    let both = [energy(0.0, 0.0, 0.0), result("heating", &[("power", TaskKind::Sorting, 0.0), ("power-again", TaskKind::Sorting, 0.0)])];
    assert_eq!(earned_badges(&badges()[..1], &quizzes(), &both, &BTreeSet::new()), ["all-done"]);
}
