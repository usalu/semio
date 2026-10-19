use super::*;
use crate::actors::fixtures::{id, ADA, BOB, TENANT};
use teaching_proctor_base::course::fixtures::fixture;
use crate::projections::LEADERBOARD;
use teaching_proctor_base::storage::{Database, SqliteProjectionStore};
use course::{Identity, Leaderboard, LeaderboardPeriod, Transcript, TranscriptRun, CHALLENGES, LEADERBOARD_PERIODS};
use server::contract::{Principal, QueryConsistency, QueryId, Scope};

const DAY: Timestamp = 86_400_000;
/// 🕰️ The instant every handler of these tests asks at: Tuesday 2026-09-29 12:00 UTC.
const NOW: Timestamp = 20_725 * DAY + DAY / 2;

fn query(kind: &str, arguments: &Query, scope: &str) -> QueryEnvelope {
    QueryEnvelope { query_id: QueryId("q1".into()), kind: kind.into(), version: WIRE_VERSION, scope: Scope(scope.into()), principal: Principal::Anonymous, arguments: serde_json::to_vec(arguments).unwrap(), consistency: QueryConsistency::Authority, cursor: None }
}

fn handler(kind: QueryKind) -> CourseQuery {
    CourseQuery { kind, tenant: TENANT.into(), course: Arc::new(b"{\"course\":true}".to_vec()), board: Arc::new(Board::new(Arc::new(fixture()))), now: || NOW }
}

fn board(period: LeaderboardPeriod, quiz: Option<&str>, learner: Option<&str>) -> Query {
    Query::Leaderboard { period, quiz: quiz.map(str::to_string), learner: learner.map(str::to_string) }
}

fn snapshot(result: Result<QueryResult, ServerError>) -> Vec<u8> {
    match result {
        Ok(QueryResult::Snapshot { value, frontier: None }) => value,
        other => panic!("expected a snapshot, got {other:?}"),
    }
}

fn refusal(result: Result<QueryResult, ServerError>) -> String {
    match result {
        Err(ServerError::BadRequest(detail)) => detail,
        other => panic!("expected a bad request, got {other:?}"),
    }
}

/// 📜️ The transcript of learner `seed`: one run of `power` on one of the nine days up to today, and
/// for every third learner one of `homes` the day before, each at a challenge of its own.
fn record(seed: u8) -> Transcript {
    let learner = id(seed);
    let at = NOW - u64::from(seed % 9) * DAY - u64::from(seed);
    let run = |quiz: &str, challenge: usize, score: f64, at: Timestamp| TranscriptRun { quiz: quiz.into(), challenge: CHALLENGES[challenge % 4], score, points: course::points(score, CHALLENGES[challenge % 4]), at };
    let mut runs = vec![run("power", usize::from(seed), f64::from(seed % 11) / 16.0, at)];
    if seed.is_multiple_of(3) {
        runs.insert(0, run("homes", usize::from(seed / 3), f64::from(seed % 5) / 4.0, at - DAY));
    }
    Transcript { tag: course::learner_tag(&learner), learner, identity: Identity::Anonymous, runs, badges: Vec::new() }
}

#[tokio::test]
async fn each_read_answers_a_snapshot_of_its_view() {
    let mut projections = SqliteProjectionStore::new(Database::memory().unwrap());
    projections.put(LEARNERS, ADA, b"{\"learner\":1}".to_vec()).await.unwrap();
    projections.put(RUNS, &id(7), b"{\"run\":1}".to_vec()).await.unwrap();
    assert_eq!(QueryKind::ALL.map(QueryKind::wire), ["course.course", "course.learner", "course.run", "course.leaderboard", "course.crowd", "course.handle"]);
    assert_eq!(handler(QueryKind::Course).kind().await, "course.course");
    assert_eq!(snapshot(handler(QueryKind::Course).handle(&query("course.course", &Query::Course, TENANT), &projections).await), b"{\"course\":true}");
    assert_eq!(snapshot(handler(QueryKind::Learner).handle(&query("course.learner", &Query::Learner { learner: ADA.into() }, TENANT), &projections).await), b"{\"learner\":1}");
    assert_eq!(snapshot(handler(QueryKind::Run).handle(&query("course.run", &Query::Run { run: id(7) }, TENANT), &projections).await), b"{\"run\":1}");
    assert_eq!(snapshot(handler(QueryKind::Leaderboard).handle(&query("course.leaderboard", &board(LeaderboardPeriod::AllTime, None, None), TENANT), &projections).await), b"{\"period\":\"all-time\",\"rows\":[],\"learners\":0,\"submissions\":0}");
    let today = format!("{{\"period\":\"daily\",\"quiz\":\"power\",\"window\":{{\"from\":{},\"until\":{}}},\"rows\":[],\"learners\":0,\"submissions\":0}}", 20_725 * DAY, 20_726 * DAY);
    assert_eq!(String::from_utf8(snapshot(handler(QueryKind::Leaderboard).handle(&query("course.leaderboard", &board(LeaderboardPeriod::Daily, Some("power"), None), TENANT), &projections).await)).unwrap(), today);
    assert!(matches!(handler(QueryKind::Leaderboard).handle(&query("course.leaderboard", &board(LeaderboardPeriod::Weekly, Some("cooling"), None), TENANT), &projections).await, Err(ServerError::NotFound(detail)) if detail == "unknown-quiz cooling"));
    projections.put(CROWDS, "power", b"{\"quiz\":\"power\"}".to_vec()).await.unwrap();
    assert_eq!(handler(QueryKind::Crowd).kind().await, "course.crowd");
    assert_eq!(snapshot(handler(QueryKind::Crowd).handle(&query("course.crowd", &Query::Crowd { quiz: "power".into() }, TENANT), &projections).await), b"{\"quiz\":\"power\"}");
    assert!(matches!(handler(QueryKind::Crowd).handle(&query("course.crowd", &Query::Crowd { quiz: "cooling".into() }, TENANT), &projections).await, Err(ServerError::NotFound(detail)) if detail == "unknown-quiz cooling"));
    assert!(matches!(handler(QueryKind::Crowd).handle(&query("course.crowd", &board(LeaderboardPeriod::AllTime, None, None), TENANT), &projections).await, Err(ServerError::BadRequest(_))));
}

#[tokio::test]
async fn the_leaderboard_answers_the_top_the_count_and_the_callers_own_row_from_the_board() {
    let mut projections = SqliteProjectionStore::new(Database::memory().unwrap());
    let course = fixture();
    let transcripts: Vec<Transcript> = (1..=130u8).map(record).collect();
    for entry in &transcripts {
        projections.put(LEADERBOARD, &entry.learner, serde_json::to_vec(entry).unwrap()).await.unwrap();
    }
    let leaderboard = handler(QueryKind::Leaderboard);
    let asked = |period: LeaderboardPeriod, quiz: Option<&str>, learner: Option<&str>| query("course.leaderboard", &board(period, quiz, learner), TENANT);
    let core = |period: LeaderboardPeriod, quiz: Option<&str>, learner: Option<&str>| course::leaderboard(&transcripts, course.view(), period, quiz, NOW, learner);
    assert_eq!(core(LeaderboardPeriod::AllTime, None, None).rows.len(), course::LEADERBOARD_TOP);
    for caller in [None, Some(id(1)), Some(id(11)), Some(id(200))] {
        let answered: Leaderboard = serde_json::from_slice(&snapshot(leaderboard.handle(&asked(LeaderboardPeriod::AllTime, None, caller.as_deref()), &projections).await)).unwrap();
        assert_eq!(answered, core(LeaderboardPeriod::AllTime, None, caller.as_deref()), "{caller:?}");
        assert_eq!((answered.rows.len(), answered.learners, answered.submissions), (course::LEADERBOARD_TOP, 130, 173));
    }
    let mut ranked = Vec::new();
    for period in LEADERBOARD_PERIODS {
        for quiz in [None, Some("power"), Some("homes")] {
            for caller in [None, Some(id(9)), Some(id(11))] {
                let answered: Leaderboard = serde_json::from_slice(&snapshot(leaderboard.handle(&asked(period, quiz, caller.as_deref()), &projections).await)).unwrap();
                assert_eq!(answered, core(period, quiz, caller.as_deref()), "{period:?} of {quiz:?} for {caller:?}");
                assert_eq!((answered.period, answered.quiz.as_deref(), answered.window, answered.submissions), (period, quiz, course::period_window(period, NOW), 173));
            }
            ranked.push(core(period, quiz, None).learners);
        }
    }
    assert_eq!(ranked, [14, 14, 0, 29, 29, 14, 130, 130, 43, 130, 130, 43], "every period and quiz ranks its own learners");
    let own: Leaderboard = serde_json::from_slice(&snapshot(leaderboard.handle(&asked(LeaderboardPeriod::AllTime, None, Some(&id(11))), &projections).await)).unwrap();
    assert!(own.own.as_ref().is_some_and(|row| row.rank > course::LEADERBOARD_TOP && row.tag == course::learner_tag(&id(11))), "a caller below the top still gets its own row: {:?}", own.own);
    let text = String::from_utf8(snapshot(leaderboard.handle(&asked(LeaderboardPeriod::AllTime, None, Some(&id(11))), &projections).await)).unwrap();
    assert!(!transcripts.iter().any(|entry| text.contains(&entry.learner)), "no learner id is in the public board");
    projections.clear(LEADERBOARD).await.unwrap();
    assert_eq!(serde_json::from_slice::<Leaderboard>(&snapshot(leaderboard.handle(&asked(LeaderboardPeriod::Monthly, None, None), &projections).await)).unwrap().learners, 130, "a loaded board answers without reading a transcript");
    projections.put(LEADERBOARD, ADA, b"not a transcript".to_vec()).await.unwrap();
    assert!(matches!(handler(QueryKind::Leaderboard).handle(&asked(LeaderboardPeriod::AllTime, None, None), &projections).await, Err(ServerError::Internal(detail)) if detail.contains("course.leaderboard/") && detail.contains("does not decode")));
}

#[tokio::test]
async fn a_handle_is_recalled_by_a_read_that_writes_nothing() {
    let database = Database::memory().unwrap();
    let mut projections = SqliteProjectionStore::new(database.clone());
    let holder = HandleHolder { learner: ADA.into(), identity: Identity::Pseudonym { handle: "Ada Lovelace".into() } };
    projections.put(HANDLES, "ada lovelace", serde_json::to_vec(&holder).unwrap()).await.unwrap();
    let handle = handler(QueryKind::Handle);
    let asked = |text: &str| query("course.handle", &Query::Handle { handle: text.into() }, TENANT);
    let recalled: HandleView = serde_json::from_slice(&snapshot(handle.handle(&asked("  ADA   lovelace "), &projections).await)).unwrap();
    assert_eq!(recalled, HandleView { display: "ADA lovelace".into(), holder: Some(holder) });
    assert_eq!(snapshot(handle.handle(&asked("Grace"), &projections).await), b"{\"display\":\"Grace\"}", "a free handle has no holder");
    for refused in ["   ", "A\u{200b}da", "\u{202e}adA", "Ade\u{301}"] {
        assert!(refusal(handle.handle(&asked(refused), &projections).await).starts_with("handle-invalid"), "{refused:?}");
    }
    projections.put(HANDLES, "grace", b"{}".to_vec()).await.unwrap();
    assert!(matches!(handle.handle(&asked("Grace"), &projections).await, Err(ServerError::Internal(_))), "a holder that does not decode is not a free handle");
    assert_eq!(teaching_proctor_base::storage::SqliteAuthorityStore::new(database).log_head().unwrap(), 0, "recalling appends no event");
}

#[tokio::test]
async fn unknown_ids_scopes_and_mismatched_arguments_are_refused() {
    let projections = SqliteProjectionStore::new(Database::memory().unwrap());
    let learner = handler(QueryKind::Learner);
    assert!(matches!(learner.handle(&query("course.learner", &Query::Learner { learner: BOB.into() }, TENANT), &projections).await, Err(ServerError::NotFound(detail)) if detail.contains("unknown-learner")));
    assert!(matches!(learner.handle(&query("course.learner", &board(LeaderboardPeriod::AllTime, None, None), TENANT), &projections).await, Err(ServerError::BadRequest(_))));
    assert!(matches!(learner.handle(&query("course.learner", &Query::Learner { learner: ADA.into() }, "elsewhere"), &projections).await, Err(ServerError::NotFound(_))));
    let mut old = query("course.learner", &Query::Learner { learner: ADA.into() }, TENANT);
    old.version = WIRE_VERSION - 1;
    assert!(matches!(learner.handle(&old, &projections).await, Err(ServerError::BadRequest(_))));
    let mut broken = old.clone();
    broken.version = WIRE_VERSION;
    broken.arguments = b"{\"type\":\"learner\"}".to_vec();
    assert!(refusal(learner.handle(&broken, &projections).await).starts_with("query-malformed"));
}

#[tokio::test]
async fn no_malformed_id_is_looked_up() {
    let mut projections = SqliteProjectionStore::new(Database::memory().unwrap());
    let megabyte = "a".repeat(1 << 20);
    for malformed in ["l1", "ADA", "../run", megabyte.as_str(), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n", ""] {
        projections.put(LEARNERS, malformed, b"{}".to_vec()).await.unwrap();
        projections.put(RUNS, malformed, b"{}".to_vec()).await.unwrap();
        assert!(refusal(handler(QueryKind::Learner).handle(&query("course.learner", &Query::Learner { learner: malformed.into() }, TENANT), &projections).await).starts_with("id-invalid"));
        assert!(refusal(handler(QueryKind::Run).handle(&query("course.run", &Query::Run { run: malformed.into() }, TENANT), &projections).await).starts_with("id-invalid"));
        assert!(refusal(handler(QueryKind::Leaderboard).handle(&query("course.leaderboard", &board(LeaderboardPeriod::AllTime, None, Some(malformed)), TENANT), &projections).await).starts_with("id-invalid"));
    }
    for malformed in ["Power", "../power", "power-", megabyte.as_str(), "power\n", ""] {
        projections.put(CROWDS, malformed, b"{}".to_vec()).await.unwrap();
        assert!(refusal(handler(QueryKind::Crowd).handle(&query("course.crowd", &Query::Crowd { quiz: malformed.into() }, TENANT), &projections).await).starts_with("id-invalid"));
        assert!(refusal(handler(QueryKind::Leaderboard).handle(&query("course.leaderboard", &board(LeaderboardPeriod::Daily, Some(malformed), None), TENANT), &projections).await).starts_with("id-invalid"));
    }
    let overlong = "a".repeat(course::HANDLE_INPUT_MAX + 1);
    assert!(refusal(handler(QueryKind::Handle).handle(&query("course.handle", &Query::Handle { handle: overlong }, TENANT), &projections).await).starts_with("handle-invalid"));
}
