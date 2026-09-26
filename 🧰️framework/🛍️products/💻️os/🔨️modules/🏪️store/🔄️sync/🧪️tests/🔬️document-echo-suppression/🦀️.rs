//! 🔁️ Language-agnostic echo-suppression fixture — Rust runner. The same
//! `🏪️store/🧫️fixtures/document-echo-suppression-v1/🔣️.json` drives the React twin (`admitRemoteEnvelopes` in
//! `🛍️products/💻️os/🟦️.ts`), so the native actor, the browser actor and the React worker suppress echoes by operation identity and
//! never discard a frame by its origin (ticket 26/09/23 session 12, run s12i: late joiners saw no history).

use super::{admit_remote_envelopes, note_authored_envelopes};
use crate::os_spr::{ActorId, ArtifactId, MutationEnvelope, MutationId};

const FIXTURE: &str = include_str!("../../../🧫️fixtures/document-echo-suppression-v1/🔣️.json");

fn envelope(value: &serde_json::Value) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: MutationId(value["mutationId"].as_str().expect("fixture mutationId").into()),
        document_id: ArtifactId("artifact-echo".into()),
        actor: ActorId(value["actor"].as_str().expect("fixture actor").into()),
        dependencies: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".into()), payload: vec![1] },
        inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".into()), payload: vec![2] },
        timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 },
    }
}

#[test]
fn every_vector_applies_by_operation_identity_and_never_discards_a_frame_by_origin() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("echo suppression fixture json");
    let mut frames = 0;
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let id = vector["id"].as_str().expect("vector id");
        let mut applied = std::collections::HashSet::new();
        for (index, step) in vector["steps"].as_array().expect("steps").iter().enumerate() {
            if let Some(authored) = step.get("authored") {
                let authored: Vec<MutationEnvelope> = authored.as_array().expect("authored").iter().map(|id| envelope(&serde_json::json!({ "mutationId": id, "actor": vector["socketActor"] }))).collect();
                note_authored_envelopes(&mut applied, &authored);
                continue;
            }
            let envelopes: Vec<MutationEnvelope> = step["frame"]["envelopes"].as_array().expect("frame envelopes").iter().map(envelope).collect();
            let admitted: Vec<String> = admit_remote_envelopes(&mut applied, envelopes).into_iter().map(|envelope| envelope.mutation_id.0).collect();
            let expected: Vec<String> = step["expectApplied"].as_array().expect("expectApplied").iter().map(|id| id.as_str().expect("id").to_string()).collect();
            assert_eq!(admitted, expected, "{id} step {index}");
            frames += 1;
        }
    }
    assert!(frames >= 7, "the fixture walks relay and tail frames ({frames})");
}

#[test]
fn neither_actor_reads_a_frame_origin_to_suppress_an_echo() {
    let source = include_str!("../../🦀️.rs");
    assert!(!source.contains("Some(origin.0.as_str())"), "no Commands frame is discarded by its origin");
    assert_eq!(source.matches("admit_remote_envelopes(&mut self.applied_op_ids").count(), 2, "both actors admit by operation identity");
    assert_eq!(source.matches("note_authored_envelopes(&mut self.applied_op_ids, envelopes);").count(), 2, "both actors record what they author");
}
