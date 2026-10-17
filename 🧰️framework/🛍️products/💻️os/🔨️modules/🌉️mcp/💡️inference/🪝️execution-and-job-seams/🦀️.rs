//! 🪝️ Execution and job seams of inference: the lookup outcome, the not-wired and no-such-service errors and the inference job payload. See [`super`].

use super::*;

//#region 🔖️ExecutionSeam
/// 💡️ Outcome of matching one `inferenceSchema` against a `declared_inferences_for_artifact` roster
/// — the fully-bound tier's seam. `Execute`'s only caller-visible outcome today is
/// `execution_not_wired_error` (no `artifact-infer` route exists on `crate::actions::ArtifactChannel`
/// anywhere in this crate — see this file's own module doc); wiring a real channel command later
/// only needs to replace THAT one call site, not this lookup.
pub enum InferenceLookup {
    NoSuchService,
    Execute(DeclaredInference),
}

/// 💡️ Pure lookup — exercised directly by this file's own tests independently of whether any live
/// tool call can reach the `Execute` arm today.
pub fn lookup_inference(declared: &[DeclaredInference], inference_schema: &str) -> InferenceLookup {
    match declared.iter().find(|item| item.inference_schema == inference_schema) {
        Some(item) => InferenceLookup::Execute(item.clone()),
        None => InferenceLookup::NoSuchService,
    }
}

/// 💡️ Why a bare READ of a declared inference still cannot answer a value: `inference_get` and the
/// `semio://artifact/{id}/inference/{field}` resource carry no canonical request body, and an
/// inference is a computation over one — the guest has nothing to run against. Execution itself is
/// no longer missing: `inference_run` (`AppCommand::Infer` →
/// `semio_framework_plugin_host::ArtifactInferenceRouter`) runs any declared service for real. This
/// stays retryable because a later packet that caches a document-derived canonical payload per
/// artifact turns this exact read into a hit with no change to the discovery path above.
///
/// 🔗️ An artifact-bound inference HAS a request body for this artifact — the gateway can
/// build it — but building it means running the guest, which a READ must not do on its
/// own budget. So the refusal is now an instruction with everything in it.
///
/// 📜️ The published contract travels WITH the refusal, so a client that asked for a value
/// and got a gap learns in the same round trip what to call and what to send — instead of
/// having to find `inference_list` and correlate the row itself.
pub(crate) fn execution_not_wired_error(item: &DeclaredInference) -> GatewayError {
    GatewayError::new(
        GatewayErrorCode::PluginUnavailable,
        match item.payload.as_ref().and_then(|contract| contract.artifact_binding.as_ref()) {
            Some(binding) => format!(
                "`{}/{}` is declared, executable and artifact-bound: run it with `inference_run {{ artifactKind, inferenceSchema, artifactId }}` — this gateway binds the artifact's document into `payload.{}` for you. A READ does not spend guest time on its own",
                item.artifact_kind, item.inference_schema, binding.field
            ),
            None => format!("`{}/{}` is declared and executable, but a bare read carries no canonical request payload to run it against — call `inference_run` with `payload` instead", item.artifact_kind, item.inference_schema),
        },
    )
    .with_details(serde_json::json!({
        "artifactKind": item.artifact_kind,
        "inferenceSchema": item.inference_schema,
        "owner": item.owner,
        "runWith": "inference_run",
        "pluginId": item.route_plugin_id(),
        "payload": item.payload,
    }))
    .retryable()
}

pub(super) fn no_such_service_error(schema: &str, inference_schema: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::NotFound, format!("no inference service `{inference_schema}` is declared for artifact kind `{schema}`")).with_details(serde_json::json!({ "artifactKind": schema, "inferenceSchema": inference_schema }))
}
//#endregion 🔖️ExecutionSeam

//#region 🔖️JobSeam
/// 💡️ The `job_` handle payload an expensive inference execution would mint via
/// `crate::handles::HandleTable::mint(HandleKind::Job, ..)` once a real `artifact-infer` route
/// exists — `job_get`/`job_cancel` (`🦀️.rs`'s own `DECLARED_STUB_TOOL_NAMES`, still
/// unimplemented) resolve/`mark_terminal` that SAME shared `HandleTable` by the id `mint` returns.
/// This facet never mints one itself: no execution route exists to mint FOR yet, and an inert job
/// handle nobody could ever progress would be its own kind of fabrication. Pinning this shape now
/// means wiring the two sides together later is additive, not a redesign.
#[derive(Clone, Debug, PartialEq, serde::Serialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferenceJobPayload {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub inference_schema: String,
    pub cancellation_id: String,
}

pub fn inference_job_payload(artifact_id: &str, item: &DeclaredInference, cancellation_id: &str) -> InferenceJobPayload {
    InferenceJobPayload { artifact_id: artifact_id.to_string(), artifact_kind: item.artifact_kind.clone(), inference_schema: item.inference_schema.clone(), cancellation_id: cancellation_id.to_string() }
}
//#endregion 🔖️JobSeam
