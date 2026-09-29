use super::*;
use crate::actors::tests::{envelope, id, perfect, ADA, BOB, TENANT};
use crate::actors::{enrollment, LearnerDecider, ProctorDeciders, RosterDecider};
use crate::catalog::tests::fixture;
use crate::storage::Database;
use quiz::{Command, Identity, Leaderboard, LearnerView, RunStatus, RunView};
use server::authority::{AuthorityDirectory, CommandBus};
use server::contract::{CommandOutcome, HybridLogicalClock, PolicyDecision};

/// 🎬️ A bus over `database` with the proctor's deciders, allowing every command.
pub(crate) async fn bus(database: &Database, catalog: &Arc<LoadedCatalog>) -> CommandBus<SqliteAuthorityStore, ProctorDeciders> {
    let mut bus = CommandBus::new(AuthorityDirectory::new(), SqliteAuthorityStore::new(database.clone()), Box::new(|_| PolicyDecision::Allow));
    bus.register(ProctorDeciders::Roster(RosterDecider { tenant: TENANT.into() })).await;
    bus.register(ProctorDeciders::Learner(LearnerDecider { catalog: Arc::clone(catalog) })).await;
    bus
}

/// 📨️ Submit one quiz command and relay what the roster decided, like the enrollment saga does.
pub(crate) async fn submit(bus: &mut CommandBus<SqliteAuthorityStore, ProctorDeciders>, command: &Command, millis: u64) -> CommandOutcome {
    let outcome = bus.submit(envelope(command), HybridLogicalClock { millis, counter: 0 }).await;
    if let CommandOutcome::Accepted { events, .. } = &outcome {
        for relay in events.iter().filter_map(enrollment) {
            bus.submit(relay, HybridLogicalClock { millis, counter: 1 }).await;
        }
    }
    outcome
}

/// 🏁️ Play one perfect run of `quiz` as `learner`.
pub(crate) async fn play(bus: &mut CommandBus<SqliteAuthorityStore, ProctorDeciders>, catalog: &LoadedCatalog, learner: &str, run: &str, quiz: &str, millis: u64) {
    submit(bus, &Command::StartRun { id: format!("c{:07x}{}", 1, &run[8..]), learner: learner.into(), run: run.into(), quiz: quiz.into() }, millis).await;
    let document = &catalog.current()[quiz].quiz;
    for (index, task) in quiz::sheet_of(document, quiz::run_seed(run)).tasks.iter().enumerate() {
        let command = Command::RecordAnswer { id: format!("c{:07x}{}", 2 + index, &run[8..]), learner: learner.into(), run: run.into(), task: task.id().clone(), answer: perfect(document, task) };
        let outcome = submit(bus, &command, millis + 1).await;
        assert!(matches!(outcome, CommandOutcome::Accepted { .. }), "{outcome:?}");
    }
    assert!(matches!(submit(bus, &Command::SubmitRun { id: format!("c{:07x}{}", 99, &run[8..]), learner: learner.into(), run: run.into() }, millis + 2).await, CommandOutcome::Accepted { .. }));
}

async fn catch_up(projector: &Projector, database: &Database) -> CatchUp {
    let mut projections = SqliteProjectionStore::new(database.clone());
    projector.catch_up(&SqliteAuthorityStore::new(database.clone()), &mut projections, &CancelToken::root_now(), |_| {}).await.unwrap()
}

fn view<T: serde::de::DeserializeOwned>(bytes: Option<Vec<u8>>) -> T {
    serde_json::from_slice(&bytes.expect("a projected view")).unwrap()
}

#[tokio::test]
async fn played_runs_fold_into_learner_run_and_leaderboard_views() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Pseudonym { handle: "Ada".into() } }, 100).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(2), learner: BOB.into(), identity: Identity::Anonymous }, 101).await;
    play(&mut bus, &catalog, ADA, &id(10), "power", 200).await;
    let projector = Projector::new(Arc::clone(&catalog));
    let CatchUp::Current(at) = catch_up(&projector, &database).await else { panic!("not cancelled") };
    assert_eq!(at.position, SqliteAuthorityStore::new(database.clone()).log_head().unwrap());
    let projections = SqliteProjectionStore::new(database.clone());
    let ada: LearnerView = view(projections.get(LEARNERS, ADA).await);
    assert_eq!(ada.identity, Identity::Pseudonym { handle: "Ada".into() });
    assert_eq!(ada.best.get("power"), Some(&1.0));
    assert_eq!(ada.total, 100.0);
    assert_eq!(ada.badges.iter().map(|award| award.badge.as_str()).collect::<Vec<_>>(), ["perfect-power", "sorter"]);
    let run: RunView = view(projections.get(RUNS, &id(10)).await);
    assert_eq!((run.status, run.result.map(|result| result.score)), (RunStatus::Submitted, Some(1.0)));
    let bob: LearnerView = view(projections.get(LEARNERS, BOB).await);
    assert!(bob.runs.is_empty());
    let board: Leaderboard = view(projections.get(LEADERBOARD, LEADERBOARD_KEY).await);
    assert_eq!(board.rows.iter().map(|row| (row.rank, row.tag.clone(), row.total)).collect::<Vec<_>>(), [(1, quiz::learner_tag(ADA), 100.0)]);
}

#[tokio::test]
async fn a_refold_and_a_restarted_projector_reach_the_same_views() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Anonymous }, 100).await;
    play(&mut bus, &catalog, ADA, &id(10), "power", 200).await;
    let first = Projector::new(Arc::clone(&catalog));
    catch_up(&first, &database).await;
    let projections = SqliteProjectionStore::new(database.clone());
    let before = (projections.get(LEARNERS, ADA).await, projections.get(RUNS, &id(10)).await, projections.get(LEADERBOARD, LEADERBOARD_KEY).await);
    let mut rewound = SqliteProjectionStore::new(database.clone());
    rewound.set_checkpoint(CHECKPOINT, 0).await.unwrap();
    let restarted = Projector::new(Arc::clone(&catalog));
    catch_up(&restarted, &database).await;
    assert_eq!((projections.get(LEARNERS, ADA).await, projections.get(RUNS, &id(10)).await, projections.get(LEADERBOARD, LEADERBOARD_KEY).await), before);
    let mut cleared = SqliteProjectionStore::new(database.clone());
    restarted.reset(&mut cleared).await.unwrap();
    assert_eq!(cleared.get(LEARNERS, ADA).await, None);
    catch_up(&restarted, &database).await;
    assert_eq!((projections.get(LEARNERS, ADA).await, projections.get(RUNS, &id(10)).await, projections.get(LEADERBOARD, LEADERBOARD_KEY).await), before);
}

#[tokio::test]
async fn a_cancelled_fold_stops_between_batches_and_resumes() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Anonymous }, 100).await;
    let projector = Projector::new(Arc::clone(&catalog));
    let cancel = CancelToken::root_now();
    cancel.cancel_now();
    let mut projections = SqliteProjectionStore::new(database.clone());
    let log = SqliteAuthorityStore::new(database.clone());
    assert!(matches!(projector.catch_up(&log, &mut projections, &cancel, |_| {}).await.unwrap(), CatchUp::Cancelled(Progress { position: 0, folded: 0, .. })));
    let mut reported = Vec::new();
    let CatchUp::Current(at) = projector.catch_up(&log, &mut projections, &CancelToken::root_now(), |progress| reported.push(progress)).await.unwrap() else { panic!("not cancelled") };
    assert_eq!((at.position, at.folded, reported.last().copied()), (log.log_head().unwrap(), 2, Some(at)));
}

#[tokio::test]
async fn projections_of_another_catalog_are_reset() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut projections = SqliteProjectionStore::new(database.clone());
    let projector = Projector::new(Arc::clone(&catalog));
    assert!(projector.prepare(&mut projections).await.unwrap());
    assert!(!projector.prepare(&mut projections).await.unwrap());
    projections.put(META, FINGERPRINT_KEY, b"another catalog".to_vec()).await.unwrap();
    projections.put(LEARNERS, ADA, b"stale".to_vec()).await.unwrap();
    assert!(projector.prepare(&mut projections).await.unwrap());
    assert_eq!(projections.get(LEARNERS, ADA).await, None);
    assert_eq!(projections.get(META, FINGERPRINT_KEY).await, Some(catalog.fingerprint.as_bytes().to_vec()));
}
