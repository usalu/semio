//! 📺️ Unit tests of the views: catalog, learner and run views, leaderboard ordering and tags, shared views.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::lifecycle::tests::{catalog, fold, id, play_perfect, quizzes, registered, step, ALICE, BOB, REVISION, RUN};
use crate::lifecycle::{decide_learner, empty_learner_state, LearnerContext};
use crate::randomness::run_seed;
use crate::schema::{Answer, Command, Event, Identity, RunResult, SortingAnswer, TaskKind, DEFAULT_LIMITS, LEADERBOARD_PERIODS};
use crate::sheet::tests::{quiz, text};

fn submitted(state: &mut LearnerState, run: &str, quiz: &str, score: f64, at: Timestamp) {
    for event in [
        Event::RunStarted { learner: state.learner.clone(), run: run.to_string(), quiz: quiz.to_string(), revision: REVISION.to_string(), seed: run_seed(run), at },
        Event::RunSubmitted { learner: state.learner.clone(), run: run.to_string(), result: RunResult { quiz: quiz.to_string(), score, tasks: Vec::new() }, at },
    ] {
        crate::lifecycle::evolve_learner(state, &event);
    }
}

fn transcripts(states: &[&LearnerState]) -> Vec<Transcript> {
    states.iter().filter_map(|state| transcript(state)).collect()
}

fn ranked(states: &[&LearnerState], catalog: &CatalogView, caller: Option<&str>) -> Leaderboard {
    leaderboard(&transcripts(states), catalog, LeaderboardPeriod::AllTime, None, 0, caller)
}

const DAYS: Timestamp = 86_400_000;

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
    assert_eq!(view.quizzes[0].emoji, "⚡");
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
    let context = LearnerContext { now: 50, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let mut state = registered(ALICE, 1);
    step(&mut state, &Command::StartRun { id: id(1), learner: ALICE.to_string(), run: RUN.to_string(), quiz: "energy".to_string() }, &context);
    let sheet = sheet_of(&quiz(), run_seed(RUN));
    let power: Vec<String> = sheet.tasks.iter().find(|task| task.id() == "power").map(|task| match task {
        crate::schema::SheetTask::Sorting(task) => task.items.iter().map(|item| item.id.clone()).collect(),
        _ => Vec::new(),
    }).unwrap_or_default();
    let answer = Answer::Sorting(SortingAnswer { order: power, guesses: BTreeMap::new() });
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
    let board = ranked(&[&first, &badge_holder, &late, &tie, &early_tie, &idle, &anonymous], &catalog, None);
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
    assert_eq!((board.rows[3].last_activity, board.learners, board.own.is_none()), (80, 5, true));
    assert_eq!((board.period, board.quiz.as_deref(), board.window, board.submissions), (LeaderboardPeriod::AllTime, None, None, 7));
    assert!(transcript(&idle).is_none() && transcript(&anonymous).is_none());
}

#[test]
fn a_period_spans_the_day_the_iso_week_or_the_month_around_an_instant() {
    let window = |from: Timestamp, until: Timestamp| Some(LeaderboardWindow { from: from * DAYS, until: until * DAYS });
    let friday = 20_728 * DAYS + 37_000_000;
    assert_eq!(period_window(LeaderboardPeriod::Daily, friday), window(20_728, 20_729));
    assert_eq!(period_window(LeaderboardPeriod::Weekly, friday), window(20_724, 20_731));
    assert_eq!(period_window(LeaderboardPeriod::Monthly, friday), window(20_727, 20_758));
    assert_eq!(period_window(LeaderboardPeriod::AllTime, friday), None);
    assert_eq!(period_window(LeaderboardPeriod::Weekly, 0), window(0, 4));
    assert_eq!(period_window(LeaderboardPeriod::Monthly, 0), window(0, 31));
    assert_eq!(period_window(LeaderboardPeriod::Monthly, 11_016 * DAYS), window(10_988, 11_017));
    assert_eq!(period_window(LeaderboardPeriod::Monthly, 11_017 * DAYS), window(11_017, 11_048));
    for at in [0, 1, DAYS - 1, friday, 253_399_622_399_999, u64::MAX / 2, u64::MAX] {
        for period in [LeaderboardPeriod::Daily, LeaderboardPeriod::Weekly, LeaderboardPeriod::Monthly] {
            let spans = period_window(period, at).unwrap_or_else(|| unreachable!());
            assert!(spans.from <= at && (at < spans.until || spans.until == u64::MAX), "{period:?} at {at}: {spans:?}");
        }
    }
}

#[test]
fn a_standing_is_made_of_the_runs_in_scope_and_the_badges_they_earned() {
    let catalog = view(&["energy", "heating"]);
    let (monday, tuesday) = (20_724 * DAYS, 20_725 * DAYS);
    let mut state = registered(ALICE, 1);
    submitted(&mut state, &id(1), "energy", 0.75, monday + 5);
    crate::lifecycle::evolve_learner(&mut state, &Event::BadgeAwarded { learner: ALICE.to_string(), badge: "sorter".to_string(), run: id(1), at: monday + 5 });
    submitted(&mut state, &id(2), "heating", 0.5, tuesday);
    crate::lifecycle::evolve_learner(&mut state, &Event::BadgeAwarded { learner: ALICE.to_string(), badge: "done".to_string(), run: id(2), at: tuesday });
    submitted(&mut state, &id(3), "energy", 0.25, tuesday + 9);
    crate::lifecycle::evolve_learner(&mut state, &Event::BadgeAwarded { learner: ALICE.to_string(), badge: "lost".to_string(), run: id(9), at: tuesday + 9 });
    let record = transcript(&state).unwrap_or_else(|| unreachable!());
    assert_eq!(record.badges.iter().map(|award| (award.badge.as_str(), award.quiz.as_str(), award.at)).collect::<Vec<_>>(), [("sorter", "energy", monday + 5), ("done", "heating", tuesday)]);
    let summary = |period: LeaderboardPeriod, quiz: Option<&str>, at: Timestamp| standing(&record, &catalog, &BoardScope::of(period, quiz, at)).map(|standing| (standing.total, standing.best.into_iter().collect::<Vec<_>>(), standing.badges, standing.runs, standing.reached_at, standing.last_activity));
    let scored = |quiz: &str, score: f64| (quiz.to_string(), score);
    assert_eq!(summary(LeaderboardPeriod::AllTime, None, 0), Some((125.0, vec![scored("energy", 0.75), scored("heating", 0.5)], vec!["sorter".to_string(), "done".to_string()], 3, tuesday, tuesday + 9)));
    assert_eq!(summary(LeaderboardPeriod::Weekly, None, tuesday), summary(LeaderboardPeriod::AllTime, None, 0));
    assert_eq!(summary(LeaderboardPeriod::Daily, None, monday), Some((75.0, vec![scored("energy", 0.75)], vec!["sorter".to_string()], 1, monday + 5, monday + 5)));
    assert_eq!(summary(LeaderboardPeriod::Daily, None, tuesday + 1), Some((75.0, vec![scored("energy", 0.25), scored("heating", 0.5)], vec!["done".to_string()], 2, tuesday + 9, tuesday + 9)));
    assert_eq!(summary(LeaderboardPeriod::Daily, Some("energy"), tuesday), Some((25.0, vec![scored("energy", 0.25)], Vec::new(), 1, tuesday + 9, tuesday + 9)));
    assert_eq!(summary(LeaderboardPeriod::AllTime, Some("heating"), 0), Some((50.0, vec![scored("heating", 0.5)], vec!["done".to_string()], 1, tuesday, tuesday)));
    assert_eq!(summary(LeaderboardPeriod::Daily, None, monday - 1), None);
    assert_eq!(summary(LeaderboardPeriod::Daily, None, tuesday + DAYS), None);
    assert_eq!(summary(LeaderboardPeriod::AllTime, Some("cooling"), 0), None);
}

#[test]
fn a_leaderboard_of_a_period_ranks_the_learners_of_its_window_and_counts_every_submission() {
    let catalog = view(&["energy", "heating"]);
    let (monday, tuesday) = (20_724 * DAYS, 20_725 * DAYS);
    let mut early = registered(&id(0xa1), 1);
    submitted(&mut early, &id(1), "energy", 1.0, monday);
    submitted(&mut early, &id(2), "heating", 0.25, tuesday + 7);
    let mut late = registered(&id(0xb2), 1);
    submitted(&mut late, &id(3), "energy", 0.5, tuesday - 1);
    submitted(&mut late, &id(4), "energy", 0.75, tuesday + 3);
    let records = transcripts(&[&early, &late]);
    let outline = |period: LeaderboardPeriod, quiz: Option<&str>, at: Timestamp| {
        let board = leaderboard(&records, &catalog, period, quiz, at, Some(&id(0xa1)));
        (board.rows.iter().map(|row| (row.tag.clone(), row.total, row.runs)).collect::<Vec<_>>(), board.learners, board.submissions, board.own.map(|row| row.rank), board.window, board.quiz)
    };
    let (first, second) = (learner_tag(&id(0xa1)), learner_tag(&id(0xb2)));
    assert_eq!(outline(LeaderboardPeriod::AllTime, None, tuesday), (vec![(first.clone(), 125.0, 2), (second.clone(), 75.0, 2)], 2, 4, Some(1), None, None));
    assert_eq!(outline(LeaderboardPeriod::Daily, None, tuesday), (vec![(second.clone(), 75.0, 1), (first.clone(), 25.0, 1)], 2, 4, Some(2), period_window(LeaderboardPeriod::Daily, tuesday), None));
    assert_eq!(outline(LeaderboardPeriod::Daily, None, monday), (vec![(first.clone(), 100.0, 1), (second.clone(), 50.0, 1)], 2, 4, Some(1), period_window(LeaderboardPeriod::Daily, monday), None));
    assert_eq!(outline(LeaderboardPeriod::Weekly, Some("heating"), tuesday), (vec![(first, 25.0, 1)], 1, 4, Some(1), period_window(LeaderboardPeriod::Weekly, monday), Some("heating".to_string())));
    assert_eq!(outline(LeaderboardPeriod::Daily, Some("energy"), tuesday + 5), (vec![(second, 75.0, 1)], 1, 4, None, period_window(LeaderboardPeriod::Daily, tuesday), Some("energy".to_string())));
    assert_eq!(outline(LeaderboardPeriod::Monthly, None, tuesday + 40 * DAYS).0, Vec::new());
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
    let board = ranked(&[&states[0], &states[1]], &view(&["energy"]), Some(&second));
    assert_eq!(board.own.as_ref().map(|row| (row.rank, row.tag.clone())), Some((2, learner_tag(&second))));
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
        let records = transcripts(&states.iter().collect::<Vec<_>>());
        for board in entries(&vector["boards"]) {
            for caller in entries(&vector["callers"]) {
                let label = format!("{}/{}", board["id"].as_str().unwrap_or_default(), caller["id"].as_str().unwrap_or_default());
                let answer = leaderboard(&records, &catalog, typed(&board["period"]), board["quiz"].as_str(), board["at"].as_u64().unwrap_or_default(), caller["learner"].as_str());
                assert_close(&format!("{}/leaderboards/{label}", vector["id"]), &json(&answer), &vector["expected"]["leaderboards"][board["id"].as_str().unwrap_or_default()][caller["id"].as_str().unwrap_or_default()]);
            }
        }
    }
}

#[test]
fn shared_windows_of_the_python_reference_hold() {
    use crate::schema::tests::{entries, fixture, json};
    for vector in entries(&fixture("leaderboard")["windows"]) {
        for period in LEADERBOARD_PERIODS {
            let name = json(&period).as_str().unwrap_or_default().to_string();
            assert_eq!(json(&period_window(period, vector["at"].as_u64().unwrap_or_default())), vector["expected"][&name], "{}/{name}", vector["id"]);
        }
    }
}

#[test]
fn shared_cuts_of_the_python_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, typed};
    let vectors = fixture("leaderboard");
    let quizzes: Vec<Quiz> = entries(&vectors["quizzes"]).iter().map(typed).collect();
    let catalog = catalog_view(&typed(&vectors["catalog"]), &quizzes);
    let crowd: Vec<Transcript> = typed(&vectors["crowd"]["transcripts"]);
    let asked = &vectors["crowd"]["board"];
    let place = |row: &LeaderboardRow| format!("{}:{}", row.rank, row.tag);
    for vector in entries(&vectors["crowd"]["cuts"]) {
        let records = &crowd[..vector["learners"].as_u64().unwrap_or_default() as usize];
        for caller in entries(&vector["callers"]) {
            let label = caller["id"].as_str().unwrap_or_default();
            let board = leaderboard(records, &catalog, typed(&asked["period"]), asked["quiz"].as_str(), asked["at"].as_u64().unwrap_or_default(), caller["learner"].as_str());
            let outline = serde_json::json!({ "rows": board.rows.iter().map(place).collect::<Vec<_>>(), "learners": board.learners, "own": board.own.as_ref().map(place) });
            assert_close(&format!("cuts/{}/{label}", vector["id"]), &outline, &vector["expected"][label]);
        }
    }
}

#[test]
fn a_leaderboard_carries_the_top_rows_counts_everyone_and_finds_the_caller_beyond_the_top() {
    let catalog = view(&["energy"]);
    let learners: Vec<LearnerState> = (0..LEADERBOARD_TOP as u32 + 30)
        .map(|seed| {
            let mut state = registered(&format!("{seed:032x}"), 1);
            submitted(&mut state, &format!("{:032x}", seed + 4096), "energy", f64::from(seed + 1) / 256.0, 10 + u64::from(seed));
            state
        })
        .collect();
    let last = learners[0].learner.clone();
    let board = ranked(&learners.iter().collect::<Vec<_>>(), &catalog, Some(&last));
    assert_eq!((board.rows.len(), board.learners), (LEADERBOARD_TOP, LEADERBOARD_TOP + 30));
    assert_eq!(board.rows.iter().map(|row| row.rank).collect::<Vec<_>>(), (1..=LEADERBOARD_TOP).collect::<Vec<_>>());
    assert_eq!(board.own.as_ref().map(|row| (row.rank, row.tag.clone())), Some((LEADERBOARD_TOP + 30, learner_tag(&last))));
    assert!(board.rows.iter().all(|row| row.tag != learner_tag(&last)));
    let first = learners[learners.len() - 1].learner.clone();
    let top = ranked(&learners.iter().collect::<Vec<_>>(), &catalog, Some(&first));
    assert_eq!(top.own.as_ref(), top.rows.first());
    assert_eq!(ranked(&learners.iter().collect::<Vec<_>>(), &catalog, Some("nobody")).own, None);
    let empty: [Transcript; 0] = [];
    assert_eq!(leaderboard(&empty, &catalog, LeaderboardPeriod::AllTime, None, 0, Some(&last)), Leaderboard { period: LeaderboardPeriod::AllTime, quiz: None, window: None, rows: Vec::new(), learners: 0, submissions: 0, own: None });
}

#[test]
fn a_transcript_ignores_answers_and_open_runs() {
    let mut state = registered(ALICE, 1);
    submitted(&mut state, &id(1), "energy", 0.5, 10);
    let before = transcript(&state);
    crate::lifecycle::evolve_learner(&mut state, &Event::RunStarted { learner: ALICE.to_string(), run: id(2), quiz: "heating".to_string(), revision: REVISION.to_string(), seed: run_seed(&id(2)), at: 50 });
    crate::lifecycle::evolve_learner(&mut state, &Event::AnswerRecorded { learner: ALICE.to_string(), run: id(2), task: "power".to_string(), answer: Answer::Sorting(SortingAnswer { order: Vec::new(), guesses: BTreeMap::new() }), at: 60 });
    assert_eq!(before.as_ref().map(|record| record.runs.clone()), Some(vec![TranscriptRun { quiz: "energy".to_string(), score: 0.5, at: 10 }]));
    assert_eq!(transcript(&state), before);
}

#[test]
fn leaderboard_follows_decided_runs() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let mut alice = registered(ALICE, 1);
    let mut bob = registered(BOB, 1);
    play_perfect(&mut alice, RUN, &quizzes, &catalog, 500);
    play_perfect(&mut bob, &id(5), &quizzes, &catalog, 400);
    let board = ranked(&[&alice, &bob], &catalog_view(&catalog, &[quiz()]), None);
    assert_eq!(board.rows.iter().map(|row| (row.rank, row.tag.clone(), row.total, row.reached_at)).collect::<Vec<_>>(), [(1, learner_tag(BOB), 100.0, 400), (2, learner_tag(ALICE), 100.0, 500)]);
}

#[test]
fn json_number_texts_render_like_ecmascript_number_to_string() {
    let expected = [
        (42.6, "42.6"),
        (50.0, "50"),
        (0.027, "0.027"),
        (0.2, "0.2"),
        (0.1 + 0.2, "0.30000000000000004"),
        (1e21, "1e+21"),
        (1e20, "100000000000000000000"),
        (123_456_789_012_345_680_000.0, "123456789012345680000"),
        (1.5e-7, "1.5e-7"),
        (1e-6, "0.000001"),
        (1e-7, "1e-7"),
        (0.000_001_234, "0.000001234"),
        (-3.5, "-3.5"),
        (-0.0, "0"),
        (0.0, "0"),
        (5e-324, "5e-324"),
        (f64::MAX, "1.7976931348623157e+308"),
        (18.0, "18"),
        (120.0, "120"),
        (0.34, "0.34"),
        (1.0 / 3.0, "0.3333333333333333"),
        (9_007_199_254_740_992.0, "9007199254740992"),
        (123.456e10, "1234560000000"),
        (9.999_999_999_999_999e20, "999999999999999900000"),
        (1e-5, "0.00001"),
    ];
    for (value, key) in expected {
        assert_eq!(json_number_text(value), key, "{value:e}");
    }
}

fn classified(items: &[(&str, &str)]) -> TaskResult {
    TaskResult::Classification {
        task: "standards".to_string(),
        score: 0.0,
        items: items.iter().map(|(item, assigned)| crate::schema::ClassificationItemResult { item: (*item).to_string(), assigned: (*assigned).to_string(), correct: "old".to_string(), credit: 0.0, explanation: None }).collect(),
    }
}

fn sorted(task: &str, order: &[&str]) -> TaskResult {
    TaskResult::Sorting { task: task.to_string(), score: 0.0, items: order.iter().enumerate().map(|(position, item)| crate::schema::SortingItemResult { item: (*item).to_string(), value: 0.0, position, rank: 0, explanation: None }).collect() }
}

fn matched(load: [f64; 3], demand: [f64; 3]) -> TaskResult {
    let dimension = |id: &str, values: [f64; 3]| crate::schema::DimensionResult {
        dimension: id.to_string(),
        score: 0.0,
        items: values.iter().enumerate().map(|(index, &assigned)| crate::schema::MatchingItemResult { item: format!("m{index}"), assigned, correct: assigned, explanation: None }).collect(),
    };
    TaskResult::Matching { task: "buildings".to_string(), score: 0.0, dimensions: vec![dimension("load", load), dimension("demand", demand)] }
}

type Tally = Vec<(String, usize, Vec<(String, usize)>)>;
type Rows<'a> = [(&'a str, usize, &'a [(&'a str, usize)])];

fn outcome(quiz: &str, tasks: Vec<TaskResult>) -> RunResult {
    RunResult { quiz: quiz.to_string(), score: 0.0, tasks }
}

#[test]
fn the_crowd_counts_categories_and_values_and_averages_positions_in_definition_order() {
    let results = [
        outcome("energy", vec![classified(&[("a", "low"), ("b", "low"), ("c", "old")]), sorted("power", &["s1", "s0", "s2", "s3"]), matched([10.0, 40.0, 120.0], [15.0, 250.0, 90.0])]),
        outcome("energy", vec![classified(&[("a", "passive"), ("b", "low"), ("d", "low")]), sorted("power", &["s0", "s2", "s4", "s1"]), matched([40.0, 10.0, 120.0], [15.0, 90.0, 250.0])]),
        outcome("heating", vec![classified(&[("a", "old")])]),
        outcome("energy", vec![sorted("standards", &["a", "b"])]),
    ];
    let crowd = crowd_view(&quiz(), &results);
    let counts = |task: &CrowdTask| -> Tally { task.items.iter().map(|item| (item.item.clone(), item.answers, item.counts.iter().flatten().map(|count| (count.key.clone(), count.count)).collect())).collect() };
    let owned = |rows: &Rows<'_>| -> Tally { rows.iter().map(|(item, answers, counts)| ((*item).to_string(), *answers, counts.iter().map(|(key, count)| ((*key).to_string(), *count)).collect())).collect() };
    assert_eq!((crowd.quiz.as_str(), crowd.runs), ("energy", 3));
    assert_eq!(crowd.tasks.iter().map(|task| (task.task.as_str(), task.kind, task.dimension.as_deref())).collect::<Vec<_>>(), [("standards", TaskKind::Classification, None), ("power", TaskKind::Sorting, None), ("buildings", TaskKind::Matching, Some("load")), ("buildings", TaskKind::Matching, Some("demand"))]);
    assert_eq!(counts(&crowd.tasks[0]), owned(&[("a", 2, &[("low", 1), ("passive", 1)]), ("b", 2, &[("low", 2)]), ("c", 1, &[("old", 1)]), ("d", 1, &[("low", 1)])]));
    let positions: Vec<(&str, usize, Option<f64>)> = crowd.tasks[1].items.iter().map(|item| (item.item.as_str(), item.answers, item.mean_position)).collect();
    assert_eq!(positions, [("s0", 2, Some((1.0 / 3.0 + 0.0) / 2.0)), ("s1", 2, Some(0.5)), ("s2", 2, Some((2.0 / 3.0 + 1.0 / 3.0) / 2.0)), ("s3", 1, Some(1.0)), ("s4", 1, Some(2.0 / 3.0))]);
    assert!(crowd.tasks[1].items.iter().all(|item| item.counts.is_none()));
    assert_eq!(crowd.tasks[1].items.iter().map(|item| item.places.clone().unwrap_or_default()).collect::<Vec<_>>(), [[1, 1, 0, 0], [1, 0, 0, 1], [0, 1, 1, 0], [0, 0, 0, 1], [0, 0, 1, 0]]);
    assert!(crowd.tasks.iter().filter(|task| task.kind != TaskKind::Sorting).flat_map(|task| &task.items).all(|item| item.places.is_none() && item.mean_position.is_none()));
    assert_eq!(counts(&crowd.tasks[2]), owned(&[("m0", 2, &[("10", 1), ("40", 1)]), ("m1", 2, &[("10", 1), ("40", 1)]), ("m2", 2, &[("120", 2)])]));
    assert_eq!(counts(&crowd.tasks[3]), owned(&[("m0", 2, &[("15", 2)]), ("m1", 2, &[("250", 1), ("90", 1)]), ("m2", 2, &[("250", 1), ("90", 1)])]));
    assert_eq!((crowd.scores, crowd.tasks.iter().map(|task| task.scores).collect::<Vec<_>>()), (lowest(3), vec![lowest(2); 4]));
    let json = serde_json::to_value(&crowd.tasks[1].items[3]).unwrap_or_default();
    assert_eq!(json, serde_json::json!({"item": "s3", "answers": 1, "meanPosition": 1.0, "places": [0, 0, 0, 1]}));
}

fn lowest(count: usize) -> CrowdScores {
    let mut scores = [0; CROWD_SCORE_BINS];
    scores[0] = count;
    scores
}

#[test]
fn a_score_falls_into_the_tenth_of_its_whole_percent_and_the_last_bin_is_closed() {
    assert_eq!(CROWD_SCORE_BINS, 10);
    for (score, bin) in [(0.0, 0), (0.004, 0), (0.0949, 0), (0.095, 1), (0.1, 1), (0.1949, 1), (0.195, 2), (0.5, 5), (0.8949, 8), (0.895, 9), (0.9, 9), (0.995, 9), (1.0, 9), (-0.25, 0), (7.0, 9)] {
        assert_eq!(score_bin(score), bin, "{score}");
    }
    let bins = binned((0..=1000).map(|thousandth| f64::from(thousandth) / 1000.0));
    assert_eq!(bins, [95, 100, 100, 100, 100, 100, 100, 100, 100, 106]);
    for percent in 0..=100u32 {
        assert_eq!(score_bin(f64::from(percent) / 100.0), (percent as usize / 10).min(9), "{percent} %");
    }
}

#[test]
fn a_position_counts_for_the_place_its_exact_share_of_the_presented_places_rounds_to() {
    for places in [1, 3, 6] {
        assert_eq!((place_bin(0, 0, places), place_bin(0, 1, places)), (0, 0));
    }
    let bins = |length: usize, places: usize| (0..length).map(|position| place_bin(position, length, places)).collect::<Vec<_>>();
    assert_eq!((bins(3, 6), bins(2, 6), bins(7, 6), bins(8, 3), bins(4, 1)), (vec![0, 3, 5], vec![0, 5], vec![0, 1, 2, 3, 3, 4, 5], vec![0, 0, 1, 1, 1, 1, 2, 2], vec![0; 4]));
    for places in 1..=12usize {
        for length in 2..=16usize {
            let bins = bins(length, places);
            assert_eq!((bins[0], bins[length - 1]), (0, places - 1));
            assert!(bins.windows(2).all(|pair| pair[0] <= pair[1]));
            if length == places {
                assert_eq!(bins, (0..length).collect::<Vec<_>>());
            }
            for (position, &place) in bins.iter().enumerate() {
                let (share, half) = (2 * position * (places - 1), length - 1);
                assert!((2 * place + 1) * half > share && share + half >= 2 * place * half, "{position} of {length} in {places}: {place}");
            }
        }
    }
    assert_eq!((place_bin(9, 3, 6), place_bin(usize::MAX, 4, 3), place_bin(2, 3, 0)), (5, 2, 0));
}

#[test]
fn a_sheet_presents_the_draw_of_a_task_or_every_item() {
    let mut quiz = quiz();
    assert_eq!(quiz.tasks.iter().map(presented).collect::<Vec<_>>(), [4, 4, 3]);
    for (draw, expected) in [(Some(2), 2), (Some(5), 5), (Some(9), 5), (None, 5)] {
        if let Task::Sorting(task) = &mut quiz.tasks[1] {
            task.draw = draw;
        }
        assert_eq!(presented(&quiz.tasks[1]), expected);
    }
}

#[test]
fn the_crowd_bins_run_task_and_dimension_scores_of_the_first_task_result_and_places_orders_of_any_length() {
    let scored = |run: f64, standards: f64, power: f64, load: f64, demand: Option<f64>| {
        let dimension = |id: &str, score: f64| crate::schema::DimensionResult { dimension: id.to_string(), score, items: Vec::new() };
        let dimensions = [Some(dimension("load", load)), demand.map(|score| dimension("demand", score)), Some(dimension("load", 1.0))].into_iter().flatten().collect();
        RunResult {
            quiz: "energy".to_string(),
            score: run,
            tasks: vec![
                TaskResult::Classification { task: "standards".to_string(), score: standards, items: Vec::new() },
                TaskResult::Sorting { task: "power".to_string(), score: power, items: Vec::new() },
                TaskResult::Sorting { task: "power".to_string(), score: 1.0, items: Vec::new() },
                TaskResult::Matching { task: "buildings".to_string(), score: 0.5, dimensions },
            ],
        }
    };
    let ordered = |order: &[&str]| outcome("energy", vec![sorted("power", order)]);
    let results = [
        scored(0.0, 0.095, 0.1, 0.895, Some(0.9)),
        scored(0.995, 1.0, 0.0949, 0.8949, None),
        scored(0.5, 0.55, 0.59, 0.6, Some(0.25)),
        RunResult { quiz: "energy".to_string(), score: 0.42, tasks: Vec::new() },
        RunResult { quiz: "heating".to_string(), score: 1.0, tasks: Vec::new() },
        ordered(&["s0"]),
        ordered(&["s0", "s1"]),
        ordered(&["s1", "s0"]),
        ordered(&["s1", "s0", "s2"]),
        ordered(&["s1", "s2", "s3", "s4", "x", "y", "s0"]),
    ];
    let crowd = crowd_view(&quiz(), &results);
    assert_eq!((crowd.runs, crowd.scores, crowd.scores.iter().sum::<usize>()), (9, [6, 0, 0, 0, 1, 1, 0, 0, 0, 1], 9));
    assert_eq!(crowd.tasks.iter().map(|task| task.scores).collect::<Vec<_>>(), [[0, 1, 0, 0, 0, 1, 0, 0, 0, 1], [6, 1, 0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 1, 0, 1, 1], [0, 0, 1, 0, 0, 0, 0, 0, 0, 1]]);
    let first = &crowd.tasks[1].items[0];
    assert_eq!((first.item.as_str(), first.answers, first.places.clone()), ("s0", 5, Some(vec![2, 0, 1, 2])));
    assert!(crowd.tasks[1].items.iter().all(|item| item.places.as_ref().is_some_and(|places| places.len() == 4 && places.iter().sum::<usize>() == item.answers)));
    let mut wider = quiz();
    if let Task::Sorting(task) = &mut wider.tasks[1] {
        task.draw = None;
    }
    assert_eq!(crowd_view(&wider, &results).tasks[1].items[0].places, Some(vec![2, 0, 1, 0, 2]));
}

#[test]
fn shared_crowd_views_of_the_python_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    let vectors = fixture("crowd-view");
    let quizzes: Vec<Quiz> = entries(&vectors["quizzes"]).iter().map(typed).collect();
    for vector in entries(&vectors["vectors"]) {
        let quiz = quizzes.iter().find(|quiz| Some(quiz.id.as_str()) == vector["quiz"].as_str()).unwrap_or_else(|| panic!("{}", vector["quiz"]));
        let results: Vec<RunResult> = entries(&vector["results"]).iter().map(typed).collect();
        assert_close(&format!("crowd-view/{}", vector["id"]), &json(&crowd_view(quiz, &results)), &vector["expected"]);
    }
}

#[test]
fn an_unanswered_quiz_has_every_task_ten_empty_score_bins_and_no_items() {
    let crowd = crowd_view::<RunResult>(&quiz(), &[]);
    assert_eq!((crowd.runs, crowd.scores, crowd.tasks.len()), (0, [0; CROWD_SCORE_BINS], 4));
    assert!(crowd.tasks.iter().all(|task| task.items.is_empty() && task.scores == [0; CROWD_SCORE_BINS]));
    assert_eq!(serde_json::to_value(&crowd.tasks[0]).unwrap_or_default(), serde_json::json!({"task": "standards", "kind": "classification", "scores": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "items": []}));
    for scores in [serde_json::json!([0, 0, 0, 0, 0, 0, 0, 0, 0]), serde_json::json!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), serde_json::json!([0.5, 0, 0, 0, 0, 0, 0, 0, 0, 0]), serde_json::json!([-1, 0, 0, 0, 0, 0, 0, 0, 0, 0])] {
        assert!(serde_json::from_value::<CrowdView>(serde_json::json!({"quiz": "energy", "runs": 0, "scores": scores, "tasks": []})).is_err(), "{scores}");
    }
    assert!(serde_json::from_value::<CrowdView>(serde_json::json!({"quiz": "energy", "runs": 0, "tasks": []})).is_err());
}
