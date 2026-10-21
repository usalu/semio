//! 🧰️ A command bus over a database with the proctor's deciders, and the perfect runs it plays, shared with the tests of the crates above (`testkit`).

use super::*;
use crate::actors::fixtures::{admission, envelope, perfect};
use crate::actors::{deciders, enrollment, ProctorDeciders};
use teaching_proctor_base::storage::Database;
use course::{Challenge, Command};
use server::authority::{AuthorityDirectory, CommandBus};
use server::contract::{CommandOutcome, HybridLogicalClock, PolicyDecision};

/// 🎬️ A bus over `database` with the proctor's deciders, allowing every command.
pub async fn bus(database: &Database, course: &Arc<LoadedCourse>) -> CommandBus<SqliteAuthorityStore, ProctorDeciders> {
    let mut bus = CommandBus::new(AuthorityDirectory::new(), SqliteAuthorityStore::new(database.clone()), Box::new(|_| PolicyDecision::Allow));
    for decider in deciders(course, &admission(0)) {
        bus.register(decider).await;
    }
    bus
}

/// 📨️ Submit one quiz command and relay what a handle decided, like the enrollment saga does.
pub async fn submit(bus: &mut CommandBus<SqliteAuthorityStore, ProctorDeciders>, command: &Command, millis: u64) -> CommandOutcome {
    let outcome = bus.submit(envelope(command), HybridLogicalClock { millis, counter: 0 }).await;
    if let CommandOutcome::Accepted { events, .. } = &outcome {
        for relay in events.iter().filter_map(enrollment) {
            bus.submit(relay, HybridLogicalClock { millis, counter: 1 }).await;
        }
    }
    outcome
}

/// 🆔️ The id of the numbered command of `run`.
pub fn command_id(number: usize, run: &str) -> String {
    format!("c{number:07x}{}", &run[8..])
}

/// ▶️ Start a run of `quiz` at `challenge` as `learner` and answer every sheet task perfectly — on a
/// timed run each right after opening it —, without submitting.
pub async fn answer(bus: &mut CommandBus<SqliteAuthorityStore, ProctorDeciders>, course: &LoadedCourse, learner: &str, run: &str, quiz: &str, challenge: Challenge, millis: u64) {
    submit(bus, &Command::StartRun { origin: None, id: command_id(1, run), learner: learner.into(), run: run.into(), quiz: quiz.into(), challenge, at: millis }, millis).await;
    let document = &course.current()[quiz].quiz;
    for (index, task) in course::sheet_of(document, course::run_seed(run), challenge).tasks.iter().enumerate() {
        if task.seconds().is_some() {
            let opened = submit(bus, &Command::OpenTask { id: command_id(50 + index, run), learner: learner.into(), run: run.into(), task: task.id().clone(), at: millis }, millis).await;
            assert!(matches!(opened, CommandOutcome::Accepted { .. }), "{opened:?}");
        }
        let command = Command::RecordAnswer { id: command_id(2 + index, run), learner: learner.into(), run: run.into(), task: task.id().clone(), answer: perfect(document, task), at: millis + 1 };
        let outcome = submit(bus, &command, millis + 1).await;
        assert!(matches!(outcome, CommandOutcome::Accepted { .. }), "{outcome:?}");
    }
}

/// 🏁️ Play one perfect run of `quiz` at `challenge` as `learner`.
pub async fn play(bus: &mut CommandBus<SqliteAuthorityStore, ProctorDeciders>, course: &LoadedCourse, learner: &str, run: &str, quiz: &str, challenge: Challenge, millis: u64) {
    answer(bus, course, learner, run, quiz, challenge, millis).await;
    assert!(matches!(submit(bus, &Command::SubmitRun { id: command_id(99, run), learner: learner.into(), run: run.into() }, millis + 2).await, CommandOutcome::Accepted { .. }));
}
