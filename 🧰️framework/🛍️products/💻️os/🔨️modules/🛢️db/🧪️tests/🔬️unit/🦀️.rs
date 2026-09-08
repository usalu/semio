
use super::*;
use crate::{DbError, DurabilityClass, Profile};

async fn decode_query_json(mut stream: QueryStream) -> serde_json::Value {
    let mut bytes = Vec::new();
    for fragment in stream.get(0).and_then(|entry| entry.value()).expect("retained query value").fragments() {
        bytes.extend_from_slice(fragment);
    }
    let value = document::decode_pathmap_json(&bytes).await.unwrap();
    while stream.close_step().unwrap() {}
    assert!(stream.terminal_is_empty());
    value
}

fn test_pool() -> std::sync::Arc<semio_framework_async::WorkerPool> {
    std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)))
}

//#region 🧸️Fixtures
async fn tempdir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!("db-facade-test-{name}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

async fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::ArtifactId, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let mut payload = serde_json::Map::new();
    for (path, value) in entries {
        payload.insert((*path).to_string(), value.clone());
    }
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(document::DB_PATHMAP_SCHEMA.to_string()), payload: document::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(document::DB_PATHMAP_SCHEMA.to_string()), payload: document::encode_pathmap_json(&serde_json::Value::Object(serde_json::Map::new())).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}
//#endregion 🧸️Fixtures

//#region 🔖️Facade round trip
/// @emoji 🧪️ The facade's own core law: everything needed for a real submit -> durable ->
/// query -> frontier -> history round trip is reachable through THIS crate's re-exported
/// names alone (`db::Database`, `db::ArtifactSpec`, `db::Consistency`, `db::Query`,
/// `db::document::CommandBatch`/`SubmitOptions`) — never by reaching past the facade into
/// `db_engine`/`db_artifact` directly. A rename or a dropped re-export in `//#region
/// 🔖️Database`/`//#region 🔖️Family` would fail this test to compile.
#[semio_framework_async_macros::async_test]
async fn full_round_trip_reachable_purely_through_facade_reexports() {
    let root = tempdir("round-trip").await;
    let mut database = Database::open_at(test_pool(), &root, Profile::Dev).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

    let batch = document::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
    let receipt = actor::block_on(handle.submit(batch, document::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
    assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.frontier.head_seq, 1);
    assert!(receipt.conflicts.is_empty());

    let queried = handle.query(Query::Get { path: "name".to_string() }, Consistency::Canonical).await.unwrap();
    let value = decode_query_json(queried).await;
    assert_eq!(value, serde_json::json!("hello"));

    let frontier = handle.frontier().await.unwrap();
    assert!(frontier.dominates(&receipt.frontier).unwrap());

    let mut history = handle.history().await.unwrap();
    assert_eq!(history.entries().len(), 1);
    while history.close_step() {}

    assert_eq!(database.catalog().await.artifacts.len(), 1);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(1))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn database_error_type_is_reachable_at_the_facade_root() {
    let root = tempdir("db-error").await;
    let database = Database::open_at(test_pool(), &root, Profile::Test).await.unwrap();
    let never_created = protocol::ArtifactId("never-created".to_string());
    let result = database.document(&never_created);
    let mut rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("unknown document was admitted"),
    };
    let error = loop {
        match rejected.retry_close().await {
            Ok(error) => break error,
            Err(retained) => rejected = retained,
        }
    };
    assert!(matches!(error, DbError::NotFound(_)));
}
//#endregion 🔖️Facade round trip

//#region 🔖️Family submodule smoke
/// @emoji 🧪️ One representative construction per `db_*` family submodule, proving the facade's
/// glob re-exports actually surface each crate's headline public items at the path this
/// crate's module doc promises (`db::core::…`, `db::state::…`, …) — a wiring/rename regression
/// in `//#region 🔖️Family` breaks this test to compile, independent of any single crate's own
/// internal test suite.
#[semio_framework_async_macros::async_test]
async fn every_family_submodule_reexports_its_headline_public_surface() {
    let limits = ids::DbLimits::default();
    assert!(limits.max_command_bytes > 0);
    ids::check_len(1, 2, "smoke").unwrap();

    let (_address, _receiver) = actor::mailbox::<u8>(policy::MailboxCapacities::default());

    let mut map: state::PMap<String, i32> = state::PMap::new();
    map = map.insert("k".to_string(), 1);
    assert_eq!(map.len(), 1);

    let memory_storage = storage::MemoryStorage::new(crate::db_storage::db_io_test_pool());
    let _: storage::DbBackend = storage::DbBackend::Memory(memory_storage.await.unwrap());

    assert_eq!(wal::WAL_SEGMENT_HEADER, 0x40);
    assert!(wal::is_wal_record_kind(wal::WAL_COMMAND));

    assert_eq!(snapshot::SnapshotOrigin::FullBaseline, snapshot::SnapshotOrigin::FullBaseline);

    assert_eq!(index::IndexKind::ALL.len(), 10);

    let touch = conflict::CommandKind::from("write");
    assert_eq!(touch.0, "write");

    let value: query::Value = 42i64.into();
    assert_eq!(value, query::Value::Int(42));

    let preview_id = preview::PreviewId("preview-1".to_string());
    assert_eq!(preview_id.to_string(), "preview-1");

    let tenant = security::TenantId::from("tenant-1");
    assert_eq!(tenant.to_string(), "tenant-1");

    let budget = compact::CompactionBudget::default();
    assert!(budget.max_wal_segments > 0);

    let node = cluster::NodeId::from("node-1");
    assert_eq!(node.to_string(), "node-1");

    let sink = observe::MemorySink::new();
    assert!(sink.lines().await.is_empty());

    let from = durability::Frontier::genesis(ids::ArtifactId("doc-1".to_string()));
    let to = durability::Frontier { head_seq: 3, commit_seq: 3, ..durability::Frontier::genesis(ids::ArtifactId("doc-1".to_string())) };
    let delta = sync::frontier_delta(&from, &to).await.unwrap();
    assert_eq!(delta.commands, 3);
}
//#endregion 🔖️Family submodule smoke
