use super::*;
use crate::storage::{Database, SqliteProjectionStore};
use quiz::Query;
use server::contract::{Principal, QueryConsistency, QueryId, Scope};

const TENANT: &str = "proctor-fixture";

fn query(kind: &str, arguments: &Query, scope: &str) -> QueryEnvelope {
    QueryEnvelope { query_id: QueryId("q1".into()), kind: kind.into(), version: WIRE_VERSION, scope: Scope(scope.into()), principal: Principal::Anonymous, arguments: serde_json::to_vec(arguments).unwrap(), consistency: QueryConsistency::Authority, cursor: None }
}

fn handler(kind: QueryKind) -> QuizQuery {
    QuizQuery { kind, tenant: TENANT.into(), catalog: Arc::new(b"{\"catalog\":true}".to_vec()) }
}

fn snapshot(result: Result<QueryResult, ServerError>) -> Vec<u8> {
    match result {
        Ok(QueryResult::Snapshot { value, frontier: None }) => value,
        other => panic!("expected a snapshot, got {other:?}"),
    }
}

#[tokio::test]
async fn each_read_answers_a_snapshot_of_its_view() {
    let mut projections = SqliteProjectionStore::new(Database::memory().unwrap());
    projections.put(LEARNERS, "l1", b"{\"learner\":1}".to_vec()).await.unwrap();
    projections.put(RUNS, "r1", b"{\"run\":1}".to_vec()).await.unwrap();
    assert_eq!(handler(QueryKind::Catalog).kind().await, "quiz.catalog");
    assert_eq!(snapshot(handler(QueryKind::Catalog).handle(&query("quiz.catalog", &Query::Catalog, TENANT), &projections).await), b"{\"catalog\":true}");
    assert_eq!(snapshot(handler(QueryKind::Learner).handle(&query("quiz.learner", &Query::Learner { learner: "l1".into() }, TENANT), &projections).await), b"{\"learner\":1}");
    assert_eq!(snapshot(handler(QueryKind::Run).handle(&query("quiz.run", &Query::Run { run: "r1".into() }, TENANT), &projections).await), b"{\"run\":1}");
    assert_eq!(snapshot(handler(QueryKind::Leaderboard).handle(&query("quiz.leaderboard", &Query::Leaderboard, TENANT), &projections).await), b"{\"rows\":[]}");
}

#[tokio::test]
async fn unknown_ids_scopes_and_mismatched_arguments_are_refused() {
    let projections = SqliteProjectionStore::new(Database::memory().unwrap());
    let learner = handler(QueryKind::Learner);
    assert!(matches!(learner.handle(&query("quiz.learner", &Query::Learner { learner: "nobody".into() }, TENANT), &projections).await, Err(ServerError::NotFound(detail)) if detail.contains("unknown-learner")));
    assert!(matches!(learner.handle(&query("quiz.learner", &Query::Leaderboard, TENANT), &projections).await, Err(ServerError::BadRequest(_))));
    assert!(matches!(learner.handle(&query("quiz.learner", &Query::Learner { learner: "l1".into() }, "elsewhere"), &projections).await, Err(ServerError::NotFound(_))));
    let mut old = query("quiz.learner", &Query::Learner { learner: "l1".into() }, TENANT);
    old.version = 2;
    assert!(matches!(learner.handle(&old, &projections).await, Err(ServerError::BadRequest(_))));
    let mut broken = old.clone();
    broken.version = WIRE_VERSION;
    broken.arguments = b"{\"type\":\"learner\"}".to_vec();
    assert!(matches!(learner.handle(&broken, &projections).await, Err(ServerError::BadRequest(detail)) if detail.starts_with("query-malformed")));
}
