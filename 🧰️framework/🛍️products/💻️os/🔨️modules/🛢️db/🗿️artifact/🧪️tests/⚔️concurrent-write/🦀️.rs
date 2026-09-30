//! ⚔️ LAW: the database grades a write only against the other authors' writes committed after the operation its
//! author observed, and only when both touch the same part (`🧬️schema/⚔️concurrent-write`, fixture
//! `🧫️fixtures/⚔️concurrent-write`, TS twin `💻️os/🧪️tests/⚔️concurrent-write`). Every write is submitted WITHOUT
//! dependencies: `observed` is advisory authoring context and never an ordering constraint (ticket 26/09/23 LD
//! item 2, C10's vigilant hub that accepted a same-field write authored during a 12 s cut). A supersession (ticket
//! 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING, `📋️design.md` §2, §9.1) depends on exactly its targets, may name any
//! author's operation the log holds, and is graded like a write by its declared target.

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
    supersede: Option<Supersede>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Supersede {
    inputs: Vec<SupersededInput>,
    target: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SupersededInput {
    target: String,
    replacement: String,
}

/// 📜️ The fixture every law here walks.
fn fixture() -> Fixture {
    let fixture: Fixture = serde_json::from_str(include_str!("../../🧫️fixtures/⚔️concurrent-write/🔣️.json")).expect("concurrent write fixture");
    assert_eq!(fixture.schema, "semio.db.concurrent-write/v1");
    fixture
}

/// ✉️ One fixture write as the envelope a replica would submit, every operation id passed through `scoped`: a domain
/// write or an undo carries no dependencies; a supersession depends on exactly its targets. Each carries its
/// observation and declared target.
async fn fixture_envelope(write: &Write, scoped: &impl Fn(&str) -> String) -> protocol::MutationEnvelope {
    let document = protocol::ArtifactId("doc-1".to_string());
    let actor = protocol::ActorId(write.actor.clone());
    let transition = |transition: &protocol::HistoryTransition, dependencies: Vec<protocol::MutationId>| protocol::history_transition_envelope(transition, &document, &actor, dependencies, protocol::HybridLogicalTimestamp::new(0, 0));
    let (mut envelope, target) = match (&write.writes.target, &write.writes.paths, write.writes.transition, &write.writes.supersede) {
        (Some(target), None, None, None) => {
            let diff = protocol::ArtifactDiff { schema: protocol::SchemaId("fixture.opaque.v1".into()), payload: write.id.as_bytes().to_vec() };
            (opaque_envelope(&document, &actor, diff), target.clone())
        }
        (None, Some(paths), None, None) => {
            let object = paths.iter().map(|path| (path.clone(), DslValue::from(&serde_json::json!(write.id)))).collect();
            let diff = protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(object)).await };
            (opaque_envelope(&document, &actor, diff), Vec::new())
        }
        (None, None, Some(true), None) => (transition(&protocol::HistoryTransition::Revert { mutation_ids: Vec::new() }, Vec::new()), Vec::new()),
        (None, None, None, Some(supersede)) => {
            let inputs: Vec<protocol::SupersededInput> = supersede
                .inputs
                .iter()
                .map(|input| protocol::SupersededInput {
                    target: protocol::MutationId(scoped(&input.target)),
                    replacement: match input.replacement.as_str() {
                        "input" => protocol::InputReplacement::Input { schema: "fixture.opaque.v1".into(), payload: write.id.as_bytes().to_vec() },
                        "withdrawn" => protocol::InputReplacement::Withdrawn,
                        other => panic!("write {} declares replacement {other}", write.id),
                    },
                })
                .collect();
            let supersede = protocol::TransitionSupersede { scope: None, inputs };
            let dependencies = supersede.targets();
            (transition(&protocol::HistoryTransition::Supersede(supersede), dependencies), supersede_target(write))
        }
        _ => panic!("write {} declares exactly one of target, paths, transition or supersede", write.id),
    };
    envelope.mutation_id = protocol::MutationId(scoped(&write.id));
    envelope.observed = write.observed.as_deref().map(|observed| protocol::MutationId(scoped(observed)));
    envelope.target = target;
    envelope
}

/// 🎯️ The target a fixture supersession declares.
fn supersede_target(write: &Write) -> Vec<String> {
    write.writes.supersede.as_ref().map(|supersede| supersede.target.clone()).unwrap_or_default()
}

/// ✉️ A domain operation's envelope with `diff`, before its identity, observation and target are stamped.
fn opaque_envelope(document: &protocol::ArtifactId, actor: &protocol::ActorId, diff: protocol::ArtifactDiff) -> protocol::MutationEnvelope {
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(String::new()),
        document_id: document.clone(),
        actor: actor.clone(),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        inverse: protocol::InverseMutation { schema: diff.schema.clone(), payload: Vec::new() },
        diff,
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn a_write_is_graded_only_against_unseen_foreign_writes_to_the_same_part() {
    let fixture = fixture();
    let mut commits = 0usize;
    for vector in &fixture.vectors {
        let policy = match vector.policy.as_str() {
            "vigilant" => protocol::MergePolicy::Vigilant,
            "normal" => protocol::MergePolicy::Normal,
            other => panic!("{}: undeclared policy {other}", vector.id),
        };
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.expect("memory storage")));
        let mut engine = ArtifactEngine::create(protocol::ArtifactId("doc-1".to_string()), storage, ArtifactEngineConfig::default(), 0).expect("engine");
        for (index, commit) in vector.commits.iter().enumerate() {
            let batch = CommandBatch::new(vec![fixture_envelope(&commit.write, &str::to_string).await]).await.expect("batch");
            let (outcome, codes) = match engine.submit(batch, SubmitOptions { policy, ..Default::default() }, index as u64 + 1).await {
                Ok(receipt) => ("accepted", receipt.messages.iter().map(|message| message.code.0.clone()).collect::<Vec<_>>()),
                Err(DbError::Rejected { messages, .. }) => ("refused", messages.iter().map(|message| message.code.0.clone()).collect::<Vec<_>>()),
                Err(other) => panic!("{} commit {index}: {other:?}", vector.id),
            };
            assert_eq!((outcome, &codes), (commit.outcome.as_str(), &commit.codes), "{} commit {index} ({})", vector.id, commit.write.id);
            commits += 1;
        }
    }
    assert!(commits >= 57, "the fixture walks every declared commit");
}

/// 📜️ Every committed write of the fixture — opaque ones with an observation and a declared target, readable
/// path-map ones, history transitions, supersessions — is durable and replays from the document's history in commit
/// order, written and read back through the same WAL a hub reopens (ticket 26/09/23 LD item 2: the history walker did
/// not know the envelope's `observed`/`target` fields and refused every new-wire record as trailing bytes). Under
/// `normal` a grading refusal commits; an admission refusal never does, and is skipped.
#[semio_framework_async_macros::async_test]
async fn the_history_replays_every_committed_write_with_its_observation_and_target() {
    if !db_storage::process_isolated_law("db_artifact::concurrent_write_tests::the_history_replays_every_committed_write_with_its_observation_and_target") {
        return;
    }
    let _history_capacity = history_capacity_test_lock();
    let fixture = fixture();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.expect("memory storage")));
    let mut engine = ArtifactEngine::create_retained(protocol::ArtifactId("doc-1".to_string()), storage, ArtifactEngineConfig::default(), 0).await.expect("retained engine");
    let mut committed: Vec<String> = Vec::new();
    for vector in &fixture.vectors {
        let scoped = |id: &str| format!("{}.{id}", vector.id);
        for commit in &vector.commits {
            if commit.codes.iter().any(|code| code != "mutation.clamped") {
                continue;
            }
            let envelope = fixture_envelope(&commit.write, &scoped).await;
            let options = SubmitOptions { durability: DurabilityClass::Fsync, policy: protocol::MergePolicy::Normal };
            engine.submit(CommandBatch::new(vec![envelope]).await.expect("batch"), options, committed.len() as u64 + 1).await.expect("normal commits every graded write");
            committed.push(scoped(&commit.write.id));
        }
    }
    let mut replay = engine.history_replay(1, Arc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().expect("history reservation"));
    let mut view = (&mut replay).await.expect("history replay");
    assert!(replay.terminal_is_empty());
    assert_eq!(view.entries.len(), committed.len(), "one history entry per committed write");
    for (index, id) in committed.iter().enumerate() {
        assert!(view.operation_id_eq(index, 0, id), "history entry {index} replays {id}");
    }
    while view.close_step() {}
    assert!(view.terminal_is_empty());
    assert!(committed.iter().any(|id| id.ends_with(".b-edit")) && committed.len() >= 55, "the fixture's observed, targeted and superseding writes are all in the replay");
}

/// 🪪️ Supersession admission beyond the fixture's one-envelope batches: a supersession names an operation earlier in
/// its own batch, may name another author's operation, is refused as malformed when its dependencies are not exactly
/// its targets or its payload does not decode, and every refusal leaves the log untouched whatever the policy.
#[semio_framework_async_macros::async_test]
async fn a_supersession_is_admitted_by_its_targets_never_by_its_author() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.expect("memory storage")));
    let mut engine = ArtifactEngine::create(protocol::ArtifactId("doc-1".to_string()), storage, ArtifactEngineConfig::default(), 0).expect("engine");
    let write = |id: &str, actor: &str, target: &str| Write { id: id.into(), actor: actor.into(), observed: None, writes: Writes { target: Some(vec![target.into()]), paths: None, transition: None, supersede: None } };
    let supersede = |id: &str, actor: &str, targets: &[&str]| Write {
        id: id.into(),
        actor: actor.into(),
        observed: None,
        writes: Writes { target: None, paths: None, transition: None, supersede: Some(Supersede { inputs: targets.iter().map(|target| SupersededInput { target: (*target).into(), replacement: "input".into() }).collect(), target: vec!["text".into()] }) },
    };
    let identity = str::to_string;
    let mut undeclared = fixture_envelope(&supersede("b-undeclared", "bob", &["a1"]), &identity).await;
    undeclared.dependencies.clear();
    let mut undecodable = fixture_envelope(&supersede("b-undecodable", "bob", &["a1"]), &identity).await;
    undecodable.diff.payload.push(0);
    let cases: Vec<(&str, Vec<protocol::MutationEnvelope>, Result<(), &str>)> = vec![
        ("alice writes", vec![fixture_envelope(&write("a1", "alice", "text"), &identity).await], Ok(())),
        ("bob supersedes alice's operation", vec![fixture_envelope(&supersede("b-edit", "bob", &["a1"]), &identity).await], Ok(())),
        ("carol writes and supersedes it in one batch", vec![fixture_envelope(&write("c1", "carol", "name"), &identity).await, fixture_envelope(&supersede("c-edit", "carol", &["c1"]), &identity).await], Ok(())),
        ("a supersession before its target in one batch", vec![fixture_envelope(&supersede("d-edit", "dave", &["d1"]), &identity).await, fixture_envelope(&write("d1", "dave", "name"), &identity).await], Err(UNKNOWN_SUPERSEDE_TARGET_CODE)),
        ("a supersession whose dependencies are not its targets", vec![undeclared], Err(MALFORMED_HISTORY_TRANSITION_CODE)),
        ("a history transition that does not decode", vec![undecodable], Err(MALFORMED_HISTORY_TRANSITION_CODE)),
    ];
    for (index, (label, envelopes, expected)) in cases.into_iter().enumerate() {
        for policy in [protocol::MergePolicy::LaissezFaire, protocol::MergePolicy::Vigilant] {
            let before = engine.frontier().await;
            let batch = CommandBatch::new(envelopes.clone()).await.expect("batch");
            match (engine.submit(batch, SubmitOptions { policy, ..Default::default() }, index as u64 + 1).await, expected) {
                (Ok(_), Ok(())) => break,
                (Err(DbError::Rejected { messages, .. }), Err(code)) => {
                    assert_eq!(messages.iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), vec![code], "{label} ({policy:?})");
                    assert_eq!(engine.frontier().await.head_seq, before.head_seq, "{label}: a refused supersession leaves the log untouched");
                }
                (outcome, expected) => panic!("{label} ({policy:?}): {outcome:?}, expected {expected:?}"),
            }
        }
    }
}
