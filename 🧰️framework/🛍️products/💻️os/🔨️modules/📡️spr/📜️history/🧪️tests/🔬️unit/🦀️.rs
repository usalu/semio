use super::*;

fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn transition_record(doc_id: &str, actor: &str, transition: protocol::HistoryTransition, timestamp: (u64, u64, u64)) -> HistoryTransitionRecord {
    let envelope = protocol::history_transition_envelope(
        &transition,
        &protocol::ArtifactId(doc_id.to_string()),
        &protocol::ActorId(actor.into()),
        Vec::new(),
        protocol::HybridLogicalTimestamp { actor: timestamp.0, physical_ms: timestamp.1, logical: timestamp.2 },
    );
    HistoryTransitionRecord::from_envelope(&envelope)
}

fn commit(checkpoint_id: &str, parent_id: Option<&str>, change_id: &str, mutation_ids: &[&str]) -> protocol::HistoryTransition {
    protocol::HistoryTransition::Commit(protocol::TransitionCheckpoint {
        checkpoint_id: checkpoint_id.to_string(),
        parent_id: parent_id.map(str::to_string),
        change_id: change_id.to_string(),
        mutation_ids: mutation_ids.iter().map(|id| protocol::MutationId(id.to_string())).collect(),
        description: None,
        saved_at: "2024-01-15T10:31:00Z".to_string(),
        authors: vec![protocol::Author { id: "u1".to_string(), name: "Ueli Saluz".to_string(), avatar: None }],
        message: Some("first checkpoint".to_string()),
        timestamp: "2024-01-15T10:32:00Z".to_string(),
        line_id: None,
    })
}

async fn sample_log() -> HistoryLog {
    HistoryLog {
        doc_id: "doc-1".to_string(),
        schema: "org.semio.demo.v1".to_string(),
        edits: [
            HistoryEdit { line: None,
                id: "edit-1".to_string(),
                actor: Some("alice".into()),
                started_at: "2024-01-15T10:30:00Z".to_string(),
                finished_at: Some("2024-01-15T10:30:05Z".to_string()), verb: Some("typeText".to_string()),
                ops: vec![OpPayload { text: Some("set foo=1".to_string()), binary: None }, OpPayload { text: Some("set bar=2".to_string()), binary: None }],
                inverse: Vec::new(),
                meta: None, lane: None,
            },
            HistoryEdit { line: None,
                id: "edit-2".to_string(),
                actor: None,
                started_at: "not-a-canonical-timestamp".to_string(),
                finished_at: None, verb: None,
                ops: vec![OpPayload { text: Some("set baz=3".to_string()), binary: None }],
                inverse: Vec::new(),
                meta: Some(vec![HistoryOpMeta {
                    op_id: Some("op-1".to_string()),
                    dependencies: vec!["edit-1".to_string()],
                    base_version: 7,
                    author_id: Some("alice".into()),
                    hlt: Some((1, 1_700_000_000_000, 3)),
                    undo_policy: 2,
                    payload_hash: Some([9u8; 32]),
                    group_id: Some("group-composite-1".to_string()),
                    origin: protocol::mutation::MutationOrigin::Contributed { plugin_id: "flow".to_string(), mutation_id: protocol::ids::SchemaId("widget.doc#recolor".to_string()), payload_hash: protocol::ids::PayloadHash([3u8; 32]) },
                    messages: Vec::new(),
                    transaction: Some(protocol::mutation::TransactionRef { id: "tx-0123456789abcdef".to_string(), tool: "s.puzzle.2d#select".to_string() }),
                }]), lane: None,
            },
        ].into_iter().collect(),
        transitions: vec![
            transition_record("doc-1", "alice", commit("ck-1", None, "change-1", &["edit-1#0", "edit-1#1", "op-1"]), (1, 1_700_000_001_000, 0)),
            transition_record("doc-1", "alice", protocol::HistoryTransition::Branch { alternative_id: "alt-1".to_string(), name: "main".to_string(), checkpoint_id: "ck-1".to_string() }, (1, 1_700_000_001_000, 1)),
        ],
        composition: None,
        conflicts: Vec::new(),
        viewer_line: None,
        viewer_checkpoint: None,
    }
}

async fn sample_conflicts() -> Vec<HistoryConflict> {
    vec![
        HistoryConflict {
            id: "conflict-quarantine-1".to_string(),
            kind: 0,
            status: 0,
            actors: vec!["alice".into(), "bob".into()],
            hlt: (1, 1_700_000_000_000, 4),
            edit_ids: Vec::new(),
            envelopes: vec![vec![1, 2, 3], vec![4, 5, 6, 7]],
            messages: vec![HistoryMessage { level: 2, code: "mutation.duplicate-id".to_string(), message: "id collided".to_string(), target: vec!["node-1".to_string()], op_index: Some(0) }],
        },
        HistoryConflict {
            id: "conflict-degraded-1".to_string(),
            kind: 1,
            status: 1,
            actors: vec!["carol".into()],
            hlt: (2, 1_700_000_001_000, 0),
            edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()],
            envelopes: Vec::new(),
            messages: vec![
                HistoryMessage { level: 0, code: "mutation.cascade".to_string(), message: "cascaded".to_string(), target: Vec::new(), op_index: None },
                HistoryMessage { level: 1, code: "mutation.partial".to_string(), message: "partial apply".to_string(), target: vec!["a".to_string(), "b".to_string()], op_index: Some(1) },
            ],
        },
    ]
}

include!("🧩️composition-and-retained-decode/🦀️.rs");

include!("⚔️conflicts-and-messages/🦀️.rs");

include!("📜️text-grammar-and-payloads/🦀️.rs");

include!("🔐️codec-and-append/🦀️.rs");

include!("🧮️fold-scan-frontier-and-index/🦀️.rs");

include!("⏳️native-fold-law/🦀️.rs");
