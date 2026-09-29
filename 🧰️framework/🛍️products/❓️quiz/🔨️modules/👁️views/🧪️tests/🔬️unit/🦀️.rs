//! 📺️ Unit tests of the views: catalog, learner and run views, leaderboard ordering and tags, shared views.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::lifecycle::tests::{catalog, fold, id, play_perfect, quizzes, registered, step, ALICE, BOB, REVISION, RUN};
use crate::lifecycle::{decide_learner, empty_learner_state, LearnerContext};
use crate::randomness::run_seed;
use crate::schema::{Answer, Command, Event, Identity, RunResult, SortingAnswer, TaskKind};
use crate::sheet::tests::{quiz, text};

fn submitted(state: &mut LearnerState, run: &str, quiz: &str, score: f64, at: Timestamp) {
    for event in [
        Event::RunStarted { learner: state.learner.clone(), run: run.to_string(), quiz: quiz.to_string(), revision: REVISION.to_string(), seed: run_seed(run), at },
        Event::RunSubmitted { learner: state.learner.clone(), run: run.to_string(), result: RunResult { quiz: quiz.to_string(), score, tasks: Vec::new() }, at },
    ] {
        crate::lifecycle::evolve_learner(state, &event);
    }
}

fn view(quiz_ids: &[&str]) -> CatalogView {
    let mut quizzes: Vec<Quiz> = Vec::new();
    for quiz_id in quiz_ids {
        let mut next = quiz();
        next.id = (*quiz_id).to_string();
        quizzes.push(next);
    }
    catalog_view(&catalog(), &quizzes)
}

#[test]
fn catalog_view_is_solution_free() {
    let view = catalog_view(&catalog(), &[quiz()]);
    assert_eq!(view.quizzes[0].tasks.iter().map(|task| (task.id.as_str(), task.kind)).collect::<Vec<_>>(), [("standards", TaskKind::Classification), ("power", TaskKind::Sorting), ("buildings", TaskKind::Matching)]);
    assert_eq!(view.badges.iter().map(|badge| badge.id.as_str()).collect::<Vec<_>>(), ["perfect-energy", "sorter", "done"]);
    assert_eq!(view.quizzes[0].title, text("Energy"));
    let json = serde_json::to_string(&view).unwrap_or_default();
    assert!(!json.contains("\"rule\"") && !json.contains("\"items\"") && !json.contains("\"category\""));
}

#[test]
fn learner_view_lists_runs_newest_first_with_best_and_total() {
    assert!(learner_view(&empty_learner_state(ALICE), &view(&["energy"])).is_none());
    let mut state = registered(ALICE, 1);
    submitted(&mut state, &id(1), "energy", 0.5, 10);
    submitted(&mut state, &id(2), "energy", 0.75, 20);
    submitted(&mut state, &id(3), "heating", 0.25, 30);
    submitted(&mut state, &id(4), "retired", 1.0, 40);
    let learner = learner_view(&state, &view(&["energy", "heating"])).unwrap_or_else(|| unreachable!());
    assert_eq!(learner.runs.iter().map(|run| run.run.clone()).collect::<Vec<_>>(), [id(4), id(3), id(2), id(1)]);
    assert_eq!(learner.best, BTreeMap::from([("energy".to_string(), 0.75), ("heating".to_string(), 0.25), ("retired".to_string(), 1.0)]));
    assert_eq!(learner.total, 100.0);
    assert_eq!(learner.identity, Identity::Pseudonym { handle: "aaaa".to_string() });
}

#[test]
fn run_view_recomputes_the_sheet_and_carries_answers() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let context = LearnerContext { now: 50, catalog: &catalog, quizzes: &quizzes };
    let mut state = registered(ALICE, 1);
    step(&mut state, &Command::StartRun { id: id(1), learner: ALICE.to_string(), run: RUN.to_string(), quiz: "energy".to_string() }, &context);
    let sheet = sheet_of(&quiz(), run_seed(RUN));
    let power: Vec<String> = sheet.tasks.iter().find(|task| task.id() == "power").map(|task| match task {
        crate::schema::SheetTask::Sorting(task) => task.items.iter().map(|item| item.id.clone()).collect(),
        _ => Vec::new(),
    }).unwrap_or_default();
    let answer = Answer::Sorting(SortingAnswer { order: power });
    let decision = decide_learner(&state, &Command::RecordAnswer { id: id(2), learner: ALICE.to_string(), run: RUN.to_string(), task: "power".to_string(), answer: answer.clone() }, &context);
    fold(&mut state, &decision);
    let view = run_view(&state, RUN, &quizzes).unwrap_or_else(|| unreachable!());
    assert_eq!((view.status, view.sheet, view.answers.get("power"), view.started_at, view.result), (RunStatus::Open, sheet, Some(&answer), 50, None));
    assert!(run_view(&state, &id(9), &quizzes).is_none());
    assert!(run_view(&state, RUN, &BTreeMap::new()).is_none());
}

#[test]
fn leaderboard_orders_by_total_badges_reached_at_and_id() {
    let catalog = view(&["energy", "heating"]);
    let mut first = registered(&id(0xa1), 1);
    submitted(&mut first, &id(1), "energy", 1.0, 50);
    let mut badge_holder = registered(&id(0xb2), 1);
    submitted(&mut badge_holder, &id(2), "energy", 1.0, 60);
    crate::lifecycle::evolve_learner(&mut badge_holder, &Event::BadgeAwarded { learner: id(0xb2), badge: "perfect-energy".to_string(), run: id(2), at: 60 });
    let mut late = registered(&id(0xc3), 1);
    submitted(&mut late, &id(3), "energy", 0.5, 10);
    submitted(&mut late, &id(4), "heating", 0.5, 70);
    submitted(&mut late, &id(5), "energy", 0.25, 80);
    let mut tie = registered(&id(0x0d), 1);
    submitted(&mut tie, &id(6), "energy", 1.0, 70);
    let mut early_tie = registered(&id(0xe5), 1);
    submitted(&mut early_tie, &id(7), "heating", 1.0, 70);
    let idle = registered(&id(0xf6), 1);
    let anonymous = empty_learner_state(&id(0x77));
    let board = leaderboard([&first, &badge_holder, &late, &tie, &early_tie, &idle, &anonymous], &catalog);
    let summary: Vec<(usize, String, f64, usize, Timestamp, usize)> = board.rows.iter().map(|row| (row.rank, row.tag.clone(), row.total, row.badges.len(), row.reached_at, row.runs)).collect();
    assert_eq!(
        summary,
        [
            (1, learner_tag(&id(0xb2)), 100.0, 1, 60, 1),
            (2, learner_tag(&id(0xa1)), 100.0, 0, 50, 1),
            (3, learner_tag(&id(0x0d)), 100.0, 0, 70, 1),
            (4, learner_tag(&id(0xc3)), 100.0, 0, 70, 3),
            (5, learner_tag(&id(0xe5)), 100.0, 0, 70, 1),
        ]
    );
    assert_eq!(board.rows[3].best, BTreeMap::from([("energy".to_string(), 0.5), ("heating".to_string(), 0.5)]));
    assert_eq!(board.rows[3].last_activity, 80);
}

#[test]
fn learner_tags_are_zero_padded_fnv1a_hex() {
    assert_eq!(learner_tag(""), "811c9dc5");
    assert_eq!(learner_tag("a"), "e40c292c");
    let padded = (0..4096u32).map(|seed| format!("{seed:032x}")).find(|learner| fnv1a32(learner) < 0x1000_0000).unwrap_or_default();
    let tag = learner_tag(&padded);
    assert_eq!((tag.len(), tag.starts_with('0'), u32::from_str_radix(&tag, 16).ok()), (8, true, Some(fnv1a32(&padded))));
}

#[test]
fn leaderboard_rows_never_carry_the_learner_id_and_ties_break_by_the_hidden_id() {
    let (first, second) = (0..4096u32)
        .map(|seed| format!("{seed:032x}"))
        .collect::<Vec<_>>()
        .windows(2)
        .find(|pair| learner_tag(&pair[0]) > learner_tag(&pair[1]))
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .unwrap_or_default();
    let mut states = [registered(&second, 1), registered(&first, 1)];
    for (index, state) in states.iter_mut().enumerate() {
        submitted(state, &id(index as u8 + 1), "energy", 0.5, 20);
    }
    let board = leaderboard(&states, &view(&["energy"]));
    assert_eq!(board.rows.iter().map(|row| row.tag.clone()).collect::<Vec<_>>(), [learner_tag(&first), learner_tag(&second)]);
    let json = serde_json::to_string(&board).unwrap_or_default();
    assert!(!json.contains(&first) && !json.contains(&second) && !json.contains("\"learner\""), "{json}");
}

#[test]
fn shared_learner_and_run_views_of_the_python_reference_hold() {
    use crate::lifecycle::tests::replay;
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    let vectors = fixture("learner-lifecycle");
    let quizzes: Vec<Quiz> = entries(&vectors["quizzes"]).iter().map(typed).collect();
    let catalog = catalog_view(&typed(&vectors["catalog"]), &quizzes);
    for sequence in entries(&vectors["learners"]).iter().filter(|sequence| sequence.get("views").is_some()) {
        let (_, state, loaded) = replay(&vectors, sequence);
        let expected = &sequence["views"]["expected"];
        assert_close(&format!("{}/learner", sequence["id"]), &json(&learner_view(&state, &catalog)), &expected["learner"]);
        for run in entries(&sequence["views"]["runs"]).iter().filter_map(serde_json::Value::as_str) {
            assert_close(&format!("{}/runs/{run}", sequence["id"]), &json(&run_view(&state, run, &loaded)), &expected["runs"][run]);
        }
    }
}

#[test]
fn shared_leaderboards_of_the_python_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    let vectors = fixture("leaderboard");
    let quizzes: Vec<Quiz> = entries(&vectors["quizzes"]).iter().map(typed).collect();
    let catalog = catalog_view(&typed(&vectors["catalog"]), &quizzes);
    for vector in entries(&vectors["vectors"]) {
        let states: Vec<LearnerState> = entries(&vector["learners"])
            .iter()
            .map(|learner| {
                let mut state = empty_learner_state(learner["learner"].as_str().unwrap_or_default());
                for event in entries(&learner["events"]) {
                    crate::lifecycle::evolve_learner(&mut state, &typed(event));
                }
                state
            })
            .collect();
        for state in &states {
            assert_close(&format!("{}/learnerViews/{}", vector["id"], state.learner), &json(&learner_view(state, &catalog)), &vector["expected"]["learnerViews"][&state.learner]);
        }
        assert_close(&format!("{}/leaderboard", vector["id"]), &json(&leaderboard(&states, &catalog)), &vector["expected"]["leaderboard"]);
    }
}

#[test]
fn leaderboard_follows_decided_runs() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let mut alice = registered(ALICE, 1);
    let mut bob = registered(BOB, 1);
    play_perfect(&mut alice, RUN, &quizzes, &catalog, 500);
    play_perfect(&mut bob, &id(5), &quizzes, &catalog, 400);
    let board = leaderboard([&alice, &bob], &catalog_view(&catalog, &[quiz()]));
    assert_eq!(board.rows.iter().map(|row| (row.rank, row.tag.clone(), row.total, row.reached_at)).collect::<Vec<_>>(), [(1, learner_tag(BOB), 100.0, 400), (2, learner_tag(ALICE), 100.0, 500)]);
}
