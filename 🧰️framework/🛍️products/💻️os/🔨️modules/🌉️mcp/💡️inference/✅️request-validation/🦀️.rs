//! ✅️ Request validation of inference runs: the run payload bytes and result value, the request validation, the contract fields and the artifact document resolution. See [`super`].

use super::*;

//#region 💡️InferenceJobTools
/// 📦️ The canonical request body one `inference_run` call carries into the guest. The gateway never
/// interprets it: whatever JSON the caller supplies is encoded as UTF-8 JSON and handed to the
/// plugin's own inference schema verbatim, because only that plugin's guest understands its own
/// canonical payload (the same host-opaque rule `🏠️workspace`'s module doc states for documents).
pub(super) fn inference_run_payload_bytes(arguments: &serde_json::Value) -> Vec<u8> {
    match arguments.get("payload") {
        None | Some(serde_json::Value::Null) => b"{}".to_vec(),
        Some(value) => serde_json::to_vec(value).unwrap_or_else(|_| b"{}".to_vec()),
    }
}

/// 🔎️ The guest's result bytes, projected for the tool reply: a UTF-8 JSON body becomes structured
/// `payload`, anything else is reported by length alone rather than guessed at.
pub(super) fn inference_run_result_value(payload: &[u8]) -> Option<serde_json::Value> {
    std::str::from_utf8(payload).ok().and_then(|text| serde_json::from_str(text).ok())
}

/// 📜️ The published contract check, BEFORE a single guest is touched. Three named refusals, each
/// carrying the exact field a client has to fix:
///
/// 1. an artifact-bound inference with neither `artifactId` nor a caller-authored payload field —
///    the case that used to spend a whole component compile and 240 s of guest time before the
///    guest answered `missing field 'snapshot'` (`📓️pz2-…md` §5.2, `📓️ce3-…md` §3.3);
/// 2. a caller payload missing a field the contract's own `input_schema` declares `required`;
/// 3. an inference that publishes NO contract at all AND was called with no payload — nothing in
///    the tree can tell the caller what to send, so saying so beats dispatching `{}`.
///
/// 🧾️ `input_schema` is the plugin's own JSON Schema TEXT. Only its top-level `required` list is
/// enforced here: that is what makes the difference between "answers in 200 ms naming the field"
/// and "hangs" — a full JSON Schema evaluator belongs to the guest that owns the schema.
pub(crate) fn validate_inference_request(item: &DeclaredInference, payload: Option<&serde_json::Value>, artifact_id: Option<&str>) -> Result<(), GatewayError> {
    let Some(contract) = item.payload.as_ref() else {
        if payload.is_none() {
            return Err(inference_field_invalid(
                "payload",
                format!("`{}/{}` publishes no payload contract, so this gateway cannot build its request body — send `payload` yourself, or have `{}` declare one on its inference metadata and be re-described", item.artifact_kind, item.inference_schema, item.route_plugin_id()),
            ));
        }
        return Ok(());
    };
    let binding = contract.artifact_binding.as_ref();
    if let Some(body) = payload.filter(|value| value.as_object().is_none_or(|object| !object.is_empty())) {
        let Some(object) = body.as_object() else {
            return Err(inference_field_invalid("payload", format!("`{}` takes a JSON object body, per `{}`", item.inference_schema, contract.payload_schema_id)));
        };
        for field in contract_required_fields(&contract.input_schema) {
            if object.contains_key(&field) {
                continue;
            }
            if binding.is_some_and(|binding| binding.field == field) && artifact_id.is_some() {
                continue;
            }
            return Err(inference_field_invalid(&field, format!("`{}` requires `payload.{}` (contract `{}`)", item.inference_schema, field, contract.payload_schema_id)));
        }
        return Ok(());
    }
    if let Some(binding) = binding {
        if binding.required && artifact_id.is_none() {
            return Err(inference_field_invalid(
                "artifactId",
                format!("`{}` runs ON an artifact: name it with `artifactId` and this gateway binds its document into `payload.{}` for you (contract `{}`)", item.inference_schema, binding.field, contract.payload_schema_id),
            ));
        }
        return Ok(());
    }
    for field in contract_required_fields(&contract.input_schema) {
        return Err(inference_field_invalid(&field, format!("`{}` requires `payload.{}` (contract `{}`)", item.inference_schema, field, contract.payload_schema_id)));
    }
    Ok(())
}

/// 🧾️ The top-level `required` names of a JSON Schema document carried as TEXT. A schema this
/// gateway cannot parse declares nothing here rather than refusing every call — the plugin's own
/// guest is still the authority on its body.
fn contract_required_fields(input_schema: &str) -> Vec<String> {
    serde_json::from_str::<serde_json::Value>(input_schema)
        .ok()
        .and_then(|schema| schema.get("required").and_then(serde_json::Value::as_array).cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| value.as_str().map(str::to_string))
        .collect()
}

/// 🏷️ The field a refusal already named, so folding the job identity onto its details never loses
/// it. `details` is a plain value here, not a map the two writers share, so the name is read back
/// rather than threaded through every construction site.
pub(super) fn inference_error_field(error: &GatewayError) -> String {
    error.details.get("field").and_then(serde_json::Value::as_str).unwrap_or("artifactId").to_string()
}

/// 🚧️ `INPUT_INVALID` that NAMES the field — the difference between a client that can retry and one
/// that can only guess. `details.field` is machine-readable on purpose.
pub(super) fn inference_field_invalid(field: &str, message: impl Into<String>) -> GatewayError {
    GatewayError::new(GatewayErrorCode::InputInvalid, message).with_details(serde_json::json!({ "field": field }))
}

/// 🔗️ The named artifact's canonical pair, for an inference whose contract binds one. `None` when
/// no artifact was named or the contract declares no binding; a NAMED artifact this workspace
/// cannot read is `NOT_FOUND`, never a silent empty document.
///
/// ⏳️ Binding is the job's own first expensive step — on the first call of a session it activates
/// the plugin guest the artifact lives in — so it runs as a job: each phase it actually runs is
/// pushed to the client as progress (`crate::actions::ActivationPhase`), and a cancel is answered as
/// soon as it lands. Laws: `🧫️fixtures/⏱️binding-cancellation-law.json` and
/// `🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json`.
pub(super) fn resolve_inference_artifact_document(workspace: &Arc<HeadlessWorkspace>, item: &DeclaredInference, artifact_id: Option<&str>, job_id: &str, cancel: &semio_framework_async::CancelToken) -> Result<Option<ArtifactDocumentBinding>, GatewayError> {
    let Some(artifact_id) = artifact_id else { return Ok(None) };
    if item.payload.as_ref().and_then(|contract| contract.artifact_binding.as_ref()).is_none() {
        return Err(inference_field_invalid("artifactId", format!("`{}` publishes no artifact binding, so naming `artifactId` cannot change what it computes — send `payload` instead", item.inference_schema)));
    }
    let job_id = job_id.to_string();
    let scope = ActivationScope::new(cancel.clone(), move |phase, fraction| {
        job_registry().report_progress(&job_id, INFERENCE_BINDING_PROGRESS_START + INFERENCE_BINDING_PROGRESS_SPAN * phase.at(fraction), Some(format!("binding: {}", phase.id())));
    });
    match workspace.bind_artifact_document_cancellably(artifact_id, scope)? {
        Some((pack, spr)) => Ok(Some(ArtifactDocumentBinding { pack, spr })),
        None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no readable document for artifact `{artifact_id}` — open or create it before running an inference on it"))),
    }
}
//#endregion 💡️InferenceJobTools
