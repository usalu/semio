use super::*;

async fn document_id() -> protocol::ArtifactId {
    protocol::ArtifactId("doc-1".to_string())
}

async fn envelope(id: &str, deps: &[&str], actor: &str, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let object: Vec<(String, DslValue)> = entries.iter().map(|(path, value)| (path.to_string(), DslValue::from(value))).collect();
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document_id().await,
        actor: protocol::ActorId(actor.to_string().into()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(object)).await },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(vec![])).await },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}
//#region 🔖️Command
#[semio_framework_async_macros::async_test]
async fn command_batch_rejects_empty_and_mixed_documents() {
    assert!(CommandBatch::new(Vec::new()).await.is_err());
    let mut mismatched = envelope("op-2", &[], "alice", &[("x", serde_json::json!(1))]).await;
    mismatched.document_id = protocol::ArtifactId("other-doc".to_string());
    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await, mismatched]).await;
    assert!(batch.is_err());
}
//#endregion 🔖️Command
//#region 🔖️Bridge
mod bridge {
    use super::*;

    #[derive(Clone, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
    struct Counter {
        value: i64,
    }

    #[derive(Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
    struct AddDiff {
        amount: i64,
    }

    impl protocol::DiffAlgebra<Counter> for AddDiff {
        fn is_empty(&self) -> bool {
            self.amount == 0
        }
    }

    impl protocol::ColdDiffAlgebra<Counter> for AddDiff {
        fn inverse(&self, _base: &Counter) -> Self {
            Self { amount: -self.amount }
        }
        fn absorb(&mut self, other: Self) {
            self.amount += other.amount;
        }
    }

    impl protocol::ColdMutationDiff<Counter> for AddDiff {
        fn apply(&self, base: &mut Counter, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
            base.value = base.value + self.amount;
            Ok(())
        }
    }

    impl protocol::MutationDiff<Counter> for AddDiff {

    }

    #[derive(Clone, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
    struct Add {
        amount: i64,
    }

    const ADD_DESCRIPTOR: protocol::MutationLeafDescriptor = protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "framework/os/db/artifact/test-add",
        semantic_kind: "add",
        display_name: "Add",
        emoji: "➕️",
        aggregate_variant: "Add",
        payload_schema: "db.artifact.test-add/v1",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    };

    impl protocol::Mutation<Counter> for Add {
        type Diff = AddDiff;
        const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[ADD_DESCRIPTOR];
        fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
            &ADD_DESCRIPTOR
        }
    }

    impl protocol::ColdMutation<Counter> for Add {
        type ReferenceDiff = AddDiff;
        fn diff(&self, _base: &Counter) -> protocol::MutationOutcome<AddDiff> {
            protocol::MutationOutcome::new(AddDiff { amount: self.amount })
        }
        fn inverse(&self, _base: &Counter) -> Result<Vec<Self>, semio_framework_value::ValueError> {
            Ok(vec![Add { amount: -self.amount }])
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn envelope_from_operation_cold_uses_operation_and_diff_traits() {
        let base = Counter { value: 10 };
        let op = Add { amount: 5 };
        let envelope = envelope_from_operation_cold(document_id().await, "counter", &op, &base, protocol::ActorId("alice".to_string().into()), protocol::MutationId("op-add-1".to_string()), protocol::HybridLogicalTimestamp::new(1, 0)).await.unwrap();
        let entries = diff_entries(&envelope.diff).await.unwrap();
        assert_eq!(entries.len(), 1, "only the changed field is persisted");
        let (path, value) = &entries[0];
        assert_eq!(path, "counter/value");
        let new_value: i64 = semio_framework_value::FromValue::from_value(value.clone().unwrap()).unwrap();
        assert_eq!(new_value, 15);
        let inverse = inverse_entries(&envelope.inverse).await.unwrap();
        assert_eq!(inverse.len(), 1, "the inverse carries the base value of the changed field only");
        assert_eq!(inverse[0].0, "counter/value");
        assert_eq!(inverse[0].1.clone().map(|value| <i64 as semio_framework_value::FromValue>::from_value(value).unwrap()), Some(10));
    }
}
//#endregion 🔖️Bridge

//#region 🔖️Source laws
const CONTRACT_SOURCE: &str = concat!(include_str!("../../🏷️ids-command-and-messages/🦀️.rs"), include_str!("../../🗺️pathmap-diff-and-bridge/🦀️.rs"), include_str!("../../⚔️conflict-records/🦀️.rs"), include_str!("../../🧾️receipts-and-checkpoints/🦀️.rs"), include_str!("../../🛡️durable-group-recovery/🦀️.rs"), include_str!("../../🦀️.rs"));

#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_artifact_contract", CONTRACT_SOURCE)] {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start().trim_start_matches("} else ");
            let Some((_, scrutinee)) = trimmed.strip_prefix("if let ").or_else(|| trimmed.strip_prefix("while let ")).and_then(|rest| rest.split_once(" = ")) else { continue };
            if !(scrutinee.contains("retry_job.lock()") && !scrutinee.starts_with("{ ") && scrutinee.trim_end().ends_with('{')) {
                continue;
            }
            let mut depth = 1i64;
            for body in &lines[index + 1..] {
                assert!(!body.contains("retry_job.lock()") && !body.contains("submit_exact("), "{name}:{} holds the retry-job guard across a body that relocks or resubmits it: {line}", index + 1);
                depth += body.matches('{').count() as i64 - body.matches('}').count() as i64;
                if depth <= 0 {
                    break;
                }
            }
        }
    }
}

#[test]
fn durable_group_witness_api_exposes_store_owned_recovery_without_record_access() {
    let source = CONTRACT_SOURCE;
    let witness_api = &source[source.find("pub struct ArtifactCommittedDurableGroupDecisionV1").unwrap()..source.find("pub async fn committed_durable_group_decision_from_transaction").unwrap()];
    assert!(witness_api.contains("into_store_owned_recovery") && witness_api.contains("take_rejected_terminal"));
    assert!(!witness_api.contains("fn record(&self)") && !witness_api.contains("fn into_record("));
    assert!(!witness_api.contains("fn cancel("));
}
//#endregion 🔖️Source laws
