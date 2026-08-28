//! 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `semio.mutation-plan` — the cold job
//! kind design-abi.md §2 names as the replacement for the deleted `contributor.artifact-mutation-plan`
//! export. `input`/the result are the SAME DSL wire-pack bytes the deleted export used
//! (`crate::app::WireArtifactMutationPlanRequest`/`WireArtifactMutationPlanResult`, encoded via
//! `store::pack_rt::encode_wire_value(&dsl::to_dsl_value(...))` — NOT plain JSON, matching
//! `🖥️host/🦀️component.rs`'s own `HostArtifactMutationPlanRequest`/`Result` mirror and its
//! `encode_wire_dsl`/`decode_wire_dsl` helpers field-for-field). Dispatch goes through the bare
//! `crate::plugin_runtime::wire_artifact_mutation_plan`, which reads the SAME process-global
//! contributed-mutation registry (`crate::app::commit_contributed_mutation_services`/
//! `contributed_mutation_plan`) `job_io_run`/`job_io_sniff` and `💡️infer` already use for THEIR
//! own process-global registries — not the per-`PLUGIN`-instance `plugin_wire_artifact_mutation_plan`,
//! for the same reason `💡️infer` picked the bare `crate::app::wire_artifact_infer` over
//! `plugin_wire_artifact_infer` (see that module's own doc comment).
//!
//! Sliced across `super::run_two_phase`'s two ticks exactly like `💡️infer`: slice 1 decodes+validates
//! `input` (reporting `(artifact_kind, mutation_id)` as progress) and checkpoints; slice 2 runs the
//! real `wire_artifact_mutation_plan` dispatch, whose own `Result<Vec<u8>, semio_framework::Fault>`
//! return type already matches `JobFn`'s exactly — no fault-code translation needed here, unlike
//! `💡️infer`'s `ArtifactInferenceExecutionError` boundary.

use super::{run_two_phase, JobCtx};
use std::future::Future;
use std::pin::Pin;

// 🚫️async: E4 fn-pointer slot — registered into `JobFn = fn(...) -> Pin<Box<dyn Future<...>>>`
// (see `⚛️reactor/💼️jobs/🦀️component.rs`'s `builtin_registry`); an `async fn` item's pointer type
// is unnameable, so the registry entry itself must stay a plain `fn` returning the already-boxed
// future (the real async work happens inside the `Box::pin(async move {...})` body below).
pub(super) fn job_mutation_plan(ctx: JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move {
        let decode_input = input.clone();
        let execute_input = input;
        run_two_phase(ctx, restored, move || async move { decode(&decode_input).await }, move || async move { crate::plugin_runtime::wire_artifact_mutation_plan(&execute_input).await }).await
    })
}

/// 🔎️ Validates `input` decodes as a `WireArtifactMutationPlanRequest` and reports its
/// `(artifact_kind, mutation_id)` identity as the first slice's progress bytes.
async fn decode(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let value = store::pack_rt::decode_wire_value(input).map_err(|error| super::fault("job.mutation-plan.decode", format!("invalid {} input: {error}", super::JOB_KIND_MUTATION_PLAN)))?;
    let request: crate::app::WireArtifactMutationPlanRequest = dsl::from_dsl_value(value).map_err(|error| super::fault("job.mutation-plan.decode", error))?;
    serde_json::to_vec(&(request.artifact_kind, request.mutation_id)).map_err(|error| super::fault("job.mutation-plan.decode", error.to_string()))
}

//#region 🧬️JobTestMutationFixtureMount
#[cfg(test)]
#[path = "🧪️tests/🧬️job-test-mutations/🦀️.rs"]
mod job_test_mutation_fixture;
//#endregion 🧬️JobTestMutationFixtureMount

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::job_test_mutation_fixture::{AddValue,JobTestOp,JobTestSnapshot};
    use store::{ArtifactPack, OpBinary};

    /// 🪪️ Commits one contributed mutation kind into the SAME process-global registry
    /// `job_mutation_plan`'s `execute` phase reads from, mirroring `🔌️plugin/🦀️component.rs`'s own
    /// `contributed_mutation_wire_tests::commit_test_contribution` fixture recipe (that helper is
    /// private to its own test module, so this is a from-scratch copy, not a shared import).
    async fn commit_job_test_contribution(artifact_kind: &str, target_document_schema: &str, contributor: &str) -> String {
        let contribution = crate::app::ArtifactContribution::builder(artifact_kind).await.mutation::<JobTestSnapshot, JobTestOp, AddValue>(target_document_schema, 1, 1).await.build();
        let (descriptor, _inferences, mutation_runtime) = contribution.resolve(contributor);
        let mutation_id = descriptor.mutations[0].mutation_id.clone();
        crate::app::commit_contributed_mutation_services(mutation_runtime).await.expect("commit contributed mutation services");
        mutation_id
    }

    async fn request_wire_bytes(artifact_kind: &str, mutation_id: &str, payload: Vec<u8>) -> Vec<u8> {
        let request = crate::app::WireArtifactMutationPlanRequest { artifact_kind: artifact_kind.to_string(), mutation_id: mutation_id.to_string(), revision: 42, generation: 9, snapshot_pack: JobTestSnapshot { value: 10 }.encode_pack(), payload };
        store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&request).expect("test request serializes to DslValue"))
    }

    /// 🧬️ Registers a real contributed mutation kind (not mocked away) and drives
    /// `semio.mutation-plan` through two real `step_job` slices to `Done`, proving `job_mutation_plan`
    /// really reaches `crate::plugin_runtime::wire_artifact_mutation_plan` and runs the registered
    /// `Planner`, not just `job.unknown-kind`.
    #[semio_framework_async_macros::async_test]
    async fn a_two_slice_mutation_plan_job_decodes_then_dispatches_to_the_registered_kind() {
        let mutation_id = commit_job_test_contribution("s.jobtest.mutation-echo", "jobtest.mutation-echo.document", "jobtest-contributor-a").await;
        let payload = crate::app::encode_contributed_wire(&AddValue { delta: 5 }).await;
        let input = request_wire_bytes("s.jobtest.mutation-echo", &mutation_id, payload).await;
        start_job(300, JOB_KIND_MUTATION_PLAN, &input).await;

        match step_job(300, JobBudget { fuel: 1, deadline_ms: 1 }).await {
            JobStep::Running(Some(progress)) => {
                let (artifact_kind, decoded_mutation_id): (String, String) = serde_json::from_slice(&progress).expect("slice 1 progress decodes");
                assert_eq!(artifact_kind, "s.jobtest.mutation-echo");
                assert_eq!(decoded_mutation_id, mutation_id);
            }
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                panic!("slice 1 must be Running(Some(identity)), not fail: {} {}", fault.code.0, fault.message);
            }
            JobStep::Done(_) => panic!("slice 1 must not finish in one tick"),
            JobStep::Running(None) => panic!("slice 1 must be Running(Some(identity)), not a bare Running(None)"),
        }
        match step_job(300, JobBudget { fuel: 1, deadline_ms: 1 }).await {
            JobStep::Done(bytes) => {
                let value = store::pack_rt::decode_wire_value(&bytes).expect("wire value decodes");
                let result: crate::app::WireArtifactMutationPlanResult = dsl::from_dsl_value(value).expect("result decodes");
                assert_eq!(result.mutation_id, mutation_id);
                assert_eq!(result.label, "Add 5 to value");
                assert_eq!(result.owner_ops.len(), 1);
                let op = JobTestOp::decode_op(&result.owner_ops[0]).expect("owner op decodes");
                assert_eq!(op, JobTestOp::AddValue(AddValue { delta: 5 }));
            }
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                panic!("slice 2 must dispatch to the registered kind, not fail: {} {}", fault.code.0, fault.message);
            }
            JobStep::Running(_) => panic!("slice 2 must finish Done, the native mutation-plan call is atomic"),
        }
    }

    /// 📸️ Interrupts after slice 1 (decode only), checkpoints, cancels, restores, and confirms the
    /// resumed run reaches the SAME `Done` output as an uninterrupted run.
    #[semio_framework_async_macros::async_test]
    async fn mutation_plan_job_checkpoint_restore_matches_an_uninterrupted_run() {
        let mutation_id = commit_job_test_contribution("s.jobtest.mutation-checkpoint", "jobtest.mutation-checkpoint.document", "jobtest-contributor-b").await;
        let payload = crate::app::encode_contributed_wire(&AddValue { delta: 3 }).await;
        let input = request_wire_bytes("s.jobtest.mutation-checkpoint", &mutation_id, payload).await;

        start_job(301, JOB_KIND_MUTATION_PLAN, &input).await;
        step_job(301, JobBudget::default()).await;
        let baseline = match step_job(301, JobBudget::default()).await {
            JobStep::Done(bytes) => bytes,
            _ => panic!("uninterrupted run must finish Done within 2 slices"),
        };

        start_job(302, JOB_KIND_MUTATION_PLAN, &input).await;
        step_job(302, JobBudget::default()).await;
        let entries = checkpoint_jobs().await;
        let entry = entries.iter().find(|entry| entry.job == 302).expect("job 302 must appear in checkpoint_jobs()");
        assert_eq!(entry.checkpoint.as_deref(), Some(PHASE_DECODED));
        let checkpoint = entry.checkpoint.clone();
        cancel_job(302).await;

        restore_job(302, JOB_KIND_MUTATION_PLAN, &input, checkpoint).await;
        let restored_final = match step_job(302, JobBudget::default()).await {
            JobStep::Done(bytes) => bytes,
            _ => panic!("a restore from PHASE_DECODED must finish Done on its FIRST step_job call"),
        };
        assert_eq!(restored_final, baseline, "checkpoint/restore must produce the identical final output");
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_plan_job_reports_a_named_decode_fault_on_garbage_input() {
        start_job(303, JOB_KIND_MUTATION_PLAN, b"not a wire value").await;
        match step_job(303, JobBudget::default()).await {
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                assert_eq!(fault.code.0, "job.mutation-plan.decode");
            }
            _ => panic!("garbage mutation-plan input must fail on slice 1"),
        }
    }
}
