//! ⚔️ LAW: the database grades a write only against the other authors' writes committed after the operation its
//! author observed, and only when both touch the same part (`🧬️schema/⚔️concurrent-write`, fixture
//! `🧫️fixtures/⚔️concurrent-write`, TS twin `💻️os/🧪️tests/⚔️concurrent-write`). Every write is submitted WITHOUT
//! dependencies: `observed` is advisory authoring context and never an ordering constraint (ticket 26/09/23 LD
//! item 2, C10's vigilant hub that accepted a same-field write authored during a 12 s cut).

use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fixture {
    schema: String,
    vectors: Vec<Vector>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Vector {
    id: String,
    policy: String,
    commits: Vec<Commit>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Commit {
    write: Write,
    outcome: String,
    codes: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Write {
    id: String,
    actor: String,
    observed: Option<String>,
    writes: Writes,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Writes {
    target: Option<Vec<String>>,
    paths: Option<Vec<String>>,
    transition: Option<bool>,
}

/// ✉️ One fixture write as the envelope a replica would submit: no dependencies, its observation and target.
async fn fixture_envelope(write: &Write) -> protocol::MutationEnvelope {
    let (diff, target) = match (&write.writes.target, &write.writes.paths, write.writes.transition) {
        (Some(target), None, None) => (protocol::ArtifactDiff { schema: protocol::SchemaId("fixture.opaque.v1".into()), payload: write.id.as_bytes().to_vec() }, target.clone()),
        (None, Some(paths), None) => {
            let object = paths.iter().map(|path| (path.clone(), DslValue::from(&serde_json::json!(write.id)))).collect();
            (protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(object)).await }, Vec::new())
        }
        (None, None, Some(true)) => (protocol::ArtifactDiff { schema: protocol::SchemaId(protocol::HISTORY_TRANSITION_SCHEMA.to_string()), payload: write.id.as_bytes().to_vec() }, Vec::new()),
        _ => panic!("write {} declares exactly one of target, paths or transition", write.id),
    };
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(write.id.clone()),
        document_id: protocol::ArtifactId("doc-1".to_string()),
        actor: protocol::ActorId(write.actor.clone()),
        dependencies: Vec::new(),
        observed: write.observed.clone().map(protocol::MutationId),
        target,
        inverse: protocol::InverseMutation { schema: diff.schema.clone(), payload: Vec::new() },
        diff,
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}

#[semio_framework_async_macros::async_test]
async fn a_write_is_graded_only_against_unseen_foreign_writes_to_the_same_part() {
    let fixture: Fixture = serde_json::from_str(include_str!("../../🧫️fixtures/⚔️concurrent-write/🔣️.json")).expect("concurrent write fixture");
    assert_eq!(fixture.schema, "semio.db.concurrent-write/v1");
    let mut commits = 0usize;
    for vector in &fixture.vectors {
        let policy = match vector.policy.as_str() {
            "vigilant" => protocol::MergePolicy::Vigilant,
            "normal" => protocol::MergePolicy::Normal,
            other => panic!("{}: undeclared policy {other}", vector.id),
        };
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.expect("memory storage")));
        let mut engine = ArtifactEngine::create(protocol::ArtifactId("doc-1".to_string()), storage, ArtifactEngineConfig::default(), 0).expect("engine");
        for (index, commit) in vector.commits.iter().enumerate() {
            let batch = CommandBatch::new(vec![fixture_envelope(&commit.write).await]).await.expect("batch");
            let (outcome, codes) = match engine.submit(batch, SubmitOptions { policy, ..Default::default() }, index as u64 + 1).await {
                Ok(receipt) => ("accepted", receipt.messages.iter().map(|message| message.code.0.clone()).collect::<Vec<_>>()),
                Err(DbError::Rejected { messages, .. }) => ("refused", messages.iter().map(|message| message.code.0.clone()).collect::<Vec<_>>()),
                Err(other) => panic!("{} commit {index}: {other:?}", vector.id),
            };
            assert_eq!((outcome, &codes), (commit.outcome.as_str(), &commit.codes), "{} commit {index} ({})", vector.id, commit.write.id);
            commits += 1;
        }
    }
    assert!(commits >= 27, "the fixture walks every declared commit");
}
