mod contributed_mutation_wire_tests {
    use super::*;
    use crate::contributed_mutation_wire::{AddValue, WireTestMutation, WireTestSnapshot};
    use store::{ArtifactPack, OpBinary};

    async fn commit_test_contribution(artifact_kind: &str, target_document_schema: &str, contributor: &str, delta: i32) -> String {
        let contribution = crate::app::ArtifactContribution::builder(artifact_kind).await.mutation::<WireTestSnapshot, WireTestMutation, AddValue>(target_document_schema, 1, 1).await.build();
        let (descriptor, _inferences, mutation_runtime) = contribution.resolve(contributor);
        let mutation_id = descriptor.mutations[0].mutation_id.clone();
        crate::app::commit_contributed_mutation_services(mutation_runtime).await.expect("commit contributed mutation services");
        let _ = delta;
        mutation_id
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_mutation_plan_rejects_a_mismatched_artifact_kind() {
        let mutation_id = commit_test_contribution("s.wiretest.kind-mismatch", "wiretest.kind-mismatch.document", "wiretest-contributor-a", 5).await;
        let payload = crate::app::encode_contributed_wire(&AddValue { delta: 5 }).await;
        let request = crate::app::WireArtifactMutationPlanRequest { artifact_kind: "s.wiretest.wrong-kind".into(), mutation_id, revision: 7, generation: 3, snapshot_pack: WireTestSnapshot { value: 10 }.encode_pack(), payload };
        let error = wire_artifact_mutation_plan(&encode_wire_serialized(&request)).await.expect_err("mismatched artifact_kind must be rejected");
        assert!(error.message.contains("artifact-mutation.artifact-kind-mismatch"), "unexpected message: {}", error.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_mutation_plan_rejects_an_unregistered_mutation_id() {
        let request = crate::app::WireArtifactMutationPlanRequest {
            artifact_kind: "s.wiretest.unregistered".into(),
            mutation_id: "wiretest.unregistered.document#nobody:add-value".into(),
            revision: 1,
            generation: 1,
            snapshot_pack: WireTestSnapshot { value: 0 }.encode_pack(),
            payload: Vec::new(),
        };
        let error = wire_artifact_mutation_plan(&encode_wire_serialized(&request)).await.expect_err("unregistered mutation_id must be rejected");
        assert!(error.message.contains("artifact-mutation.not-registered"), "unexpected message: {}", error.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_mutation_plan_echoes_identity_and_runs_the_registered_plan() {
        let mutation_id = commit_test_contribution("s.wiretest.echo", "wiretest.echo.document", "wiretest-contributor-b", 5).await;
        let payload = crate::app::encode_contributed_wire(&AddValue { delta: 5 }).await;
        let request = crate::app::WireArtifactMutationPlanRequest { artifact_kind: "s.wiretest.echo".into(), mutation_id: mutation_id.clone(), revision: 42, generation: 9, snapshot_pack: WireTestSnapshot { value: 10 }.encode_pack(), payload };
        let encoded = wire_artifact_mutation_plan(&encode_wire_serialized(&request)).await.expect("matching artifact_kind must be accepted");
        let result: crate::app::WireArtifactMutationPlanResult = decode_wire_serialized(&encoded).await.expect("result decodes");
        assert_eq!(result.artifact_kind, "s.wiretest.echo");
        assert_eq!(result.mutation_id, mutation_id);
        assert_eq!(result.revision, 42);
        assert_eq!(result.generation, 9);
        assert_eq!(result.label, "Add 5 to value");
        assert!(result.foreign.is_empty());
        assert_eq!(result.owner_ops.len(), 1);
        let op = WireTestMutation::decode_op(&result.owner_ops[0]).expect("owner op decodes");
        assert_eq!(op, WireTestMutation::AddValue(AddValue { delta: 5 }));
    }
}
