//! 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `semio.mutation-plan` — the cold job
//! kind design-abi.md §2 names as the replacement for the deleted `contributor.artifact-mutation-plan`
//! export. `input`/the result are the SAME DSL wire-pack bytes the deleted export used
//! (`crate::app::WireArtifactMutationPlanRequest`/`WireArtifactMutationPlanResult`, encoded via
//! `store::pack_rt::encode_wire_value(&dsl::to_dsl_value(...))` — NOT plain JSON, matching
//! `🖥️host/🦀️.rs`'s own `HostArtifactMutationPlanRequest`/`Result` mirror and its
//! `encode_wire_dsl`/`decode_wire_dsl` helpers field-for-field). Dispatch goes through the bare
//! `crate::plugin_runtime::wire_artifact_mutation_plan`, which reads the SAME process-global
//! contributed-mutation registry (`crate::app::commit_contributed_mutation_services`/
//! `contributed_mutation_plan`) `job_io_run`/`job_io_sniff` and `💡️infer` already use for THEIR
//! own process-global registries — not the per-`PLUGIN`-instance `plugin_wire_artifact_mutation_plan`,
//! for the same reason `💡️infer` picked the bare `crate::app::wire_artifact_infer` over
//! `plugin_wire_artifact_infer` (see that module's own doc comment).
//!
//! Sliced across `super::TwoPhaseBoundedJob`'s two explicit states exactly like `💡️infer`'s own
//! registry route: `Decode` validates `input` (reporting `(artifact_kind, mutation_id)` as
//! progress) and makes `PHASE_DECODED` its checkpoint; `Execute` runs the real
//! `wire_artifact_mutation_plan` dispatch, whose own `Result<Vec<u8>, semio_framework::Fault>`
//! return type already matches `BuiltinPhaseFn`'s exactly — no fault-code translation needed here,
//! unlike `💡️infer`'s `ArtifactInferenceExecutionError` boundary.

use super::{BoundedJob, TwoPhaseBoundedJob};

// 🚫️async: E4 fn-pointer slot — registered into `BoundedJobFactory` (see
// `⚛️reactor/💼️jobs/🦀️.rs`'s `builtin_registry`); the admission body is a pure constructor call.
pub(super) fn job_mutation_plan(_job: u64, input: &[u8], restored: Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    Ok(Box::new(TwoPhaseBoundedJob::admit("job.mutation-plan", input, restored, decode_phase, execute_phase)))
}

// 🚫️async: E4 phase slot — `BuiltinPhaseFn` is synchronous by contract; `decode` itself has no
// suspension point, so `settle_in_step` resolves it inside this state action.
fn decode_phase(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    super::settle_in_step("job.mutation-plan", decode(input))
}

// 🚫️async: E4 phase slot — see `decode_phase`; `wire_artifact_mutation_plan` is the unchunked
// native call this state action declares `WORK_UNITS_EXECUTE` for.
fn execute_phase(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    super::settle_in_step("job.mutation-plan", crate::plugin_runtime::wire_artifact_mutation_plan(input))
}

/// 🔎️ Validates `input` decodes as a `WireArtifactMutationPlanRequest` and reports its
/// `(artifact_kind, mutation_id)` identity as the first slice's progress bytes.
async fn decode(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let value = store::pack_rt::decode_wire_value(input).map_err(|error| super::fault("job.mutation-plan.decode", format!("invalid {} input: {error}", super::JOB_KIND_MUTATION_PLAN)))?;
    let request: crate::app::WireArtifactMutationPlanRequest = dsl::from_dsl_value(value).map_err(|error| super::fault("job.mutation-plan.decode", error))?;
    Ok(dsl::os_pack::json::to_json_string(&(request.artifact_kind, request.mutation_id)).into_bytes())
}

//#region 🧬️JobTestMutationFixtureMount
#[cfg(test)]
#[path = "🧪️tests/🧬️job-test-mutations/🦀️.rs"]
mod job_test_mutation_fixture;
//#endregion 🧬️JobTestMutationFixtureMount

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
