//! 💡️ Lift file: 28 item(s), about 299 lines, of `💡️inference/🦀️.rs` that belong to crate `plugin-program` (WP2).
//!
//! WP3 owns the source file and cuts the items here (inference_child_dependency_parts, ARTIFACT_INFERENCE_SERVICE_REGISTRY, artifact_inference_service_registry, ARTIFACT_INFERENCE_WIRE_VERSION, WireArtifactInferenceMetadata, WireInferencePayloadContract, ...); WP2 wires this file with one `mod` line in the crate root. Until `plugin-program` exists, the source module re-exports the items with `pub use`.
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_plugin_identity::ArtifactIdentity;
use semio_framework_plugin_decl_views::child_view::CHILD_CONTENT_ID_BYTES;
use semio_framework_plugin_decl_registry::artifact_inference::ArtifactInferenceExecution;
use semio_framework_plugin_decl_registry::artifact_inference::ArtifactInferenceExecutionError;
use semio_framework_plugin_decl_registry::artifact_inference::ArtifactInferencePayloadContract;
use semio_framework_plugin_decl_registry::artifact_inference::ArtifactInferenceServiceKey;
use semio_framework_plugin_decl_registry::artifact_inference::ArtifactInferenceServiceMetadata;
use semio_framework_plugin_decl_registry::artifact_inference::ArtifactInferenceServiceRegistry;
use semio_framework_plugin_decl_registry::artifact_inference::WireArtifactInferenceBudget;
use semio_framework_plugin_decl_registry::artifact_inference::WireArtifactInferenceCacheMode;
use semio_framework_plugin_decl_registry::artifact_inference::WireArtifactInferenceDiagnostic;
/// 🪆️ The slot and child id an owned-child dependency key names (`child:<slot>/<childId>`, each part non-empty, at most
/// [`CHILD_CONTENT_ID_BYTES`] bytes, without whitespace or control characters, the slot without `/`), else `None` — the grammar
/// [`validate_wire_request_resources`] admits beside artifact identities.
pub fn inference_child_dependency_parts(key: &str) -> Option<(&str, &str)> {
    let (slot, child_id) = key.strip_prefix("child:")?.split_once('/')?;
    let part = |text: &str| !text.is_empty() && text.len() <= CHILD_CONTENT_ID_BYTES && !text.chars().any(|character| character.is_whitespace() || character.is_control());
    (part(slot) && part(child_id)).then_some((slot, child_id))
}

static ARTIFACT_INFERENCE_SERVICE_REGISTRY: std::sync::OnceLock<std::sync::RwLock<ArtifactInferenceServiceRegistry>> = std::sync::OnceLock::new();

pub fn artifact_inference_service_registry() -> &'static std::sync::RwLock<ArtifactInferenceServiceRegistry> {
    ARTIFACT_INFERENCE_SERVICE_REGISTRY.get_or_init(|| std::sync::RwLock::new(ArtifactInferenceServiceRegistry::new()))
}

pub const ARTIFACT_INFERENCE_WIRE_VERSION: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceMetadata {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    /// 📜️ The published payload contract, carried from the guest's own declaration to
    /// `describe`, which commits it into `contributions.inferenceServices[].payload`. Not part
    /// of this row's IDENTITY — [`validate_wire_request_metadata`] compares the identity
    /// projection, because a REQUEST names an inference and never re-states its contract.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<WireInferencePayloadContract>,
}

/// 📜️ Wire twin of [`ArtifactInferencePayloadContract`] — the shape that crosses the guest
/// boundary on the inference roster.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireInferencePayloadContract {
    pub payload_schema_id: String,
    pub input_schema: String,
    pub output_schema: String,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub progress_unit: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_binding: Option<WireInferenceArtifactBinding>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<WireInferenceCommitBinding>,
}

/// 📌️ Wire twin of [`ArtifactInferenceCommitBinding`].
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireInferenceCommitBinding {
    pub action: String,
}

/// 🔗️ Wire twin of [`ArtifactInferenceDocumentBinding`].
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireInferenceArtifactBinding {
    pub field: String,
    pub encoding: String,
    pub required: bool,
}

impl From<ArtifactInferencePayloadContract> for WireInferencePayloadContract {
    fn from(contract: ArtifactInferencePayloadContract) -> Self {
        Self {
            payload_schema_id: contract.payload_schema_id.to_owned(),
            input_schema: contract.input_schema.to_owned(),
            output_schema: contract.output_schema.to_owned(),
            progress_unit: contract.progress_unit.to_owned(),
            artifact_binding: contract.artifact_binding.map(|binding| WireInferenceArtifactBinding { field: binding.field.to_owned(), encoding: binding.encoding.to_owned(), required: binding.required }),
            commit: contract.commit.map(|commit| WireInferenceCommitBinding { action: commit.action.to_owned() }),
        }
    }
}

impl From<WireInferencePayloadContract> for semio_framework::InferencePayloadContract {
    fn from(contract: WireInferencePayloadContract) -> Self {
        Self {
            payload_schema_id: contract.payload_schema_id,
            input_schema: contract.input_schema,
            output_schema: contract.output_schema,
            progress_unit: contract.progress_unit,
            artifact_binding: contract.artifact_binding.map(|binding| semio_framework::InferenceArtifactBinding { field: binding.field, encoding: binding.encoding, required: binding.required }),
            commit: contract.commit.map(|commit| semio_framework::InferenceCommitBinding { action: commit.action }),
        }
    }
}

impl From<ArtifactInferenceServiceMetadata> for WireArtifactInferenceMetadata {
    fn from(metadata: ArtifactInferenceServiceMetadata) -> Self {
        Self {
            owner: metadata.owner.into(),
            artifact_kind: metadata.artifact_kind.into(),
            artifact_schema: metadata.artifact_schema.into(),
            artifact_schema_version: metadata.artifact_schema_version,
            inference_schema: metadata.inference_schema.into(),
            inference_schema_version: metadata.inference_schema_version,
            algorithm_version: metadata.algorithm_version,
            policy_version: metadata.policy_version,
            payload: metadata.payload.map(Into::into),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceRequest {
    pub wire_version: u32,
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub revision: u64,
    pub generation: u64,
    pub source_dialect: String,
    pub policy: Vec<u8>,
    pub budgets: WireArtifactInferenceBudget,
    pub retained: RetainedCloneGrant,
    pub cancellation_id: String,
    pub previous_state: Option<Vec<u8>>,
    pub requested_cache_mode: WireArtifactInferenceCacheMode,
    pub canonical_payload: Vec<u8>,
    pub dependencies: Vec<(String, Vec<u8>)>,
}

/// 🧾️ Immutable provenance for one inference result.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceProvenance {
    pub owner: String,
    pub inference_schema: String,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub source_dialect: String,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceResult {
    pub retirement_progress: RetainedCloneProgress,
    pub wire_version: u32,
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub revision: u64,
    pub generation: u64,
    pub source_dialect: String,
    pub policy: Vec<u8>,
    pub budgets: WireArtifactInferenceBudget,
    pub retained: RetainedCloneGrant,
    pub previous_state: Option<Vec<u8>>,
    pub requested_cache_mode: WireArtifactInferenceCacheMode,
    pub canonical_payload: Option<Vec<u8>>,
    pub dependencies: Vec<(String, Vec<u8>)>,
    pub diagnostics: Vec<WireArtifactInferenceDiagnostic>,
    pub provenance: WireArtifactInferenceProvenance,
    pub validity: String,
    pub quality: String,
    pub complete: bool,
    pub actual_cache_mode: WireArtifactInferenceCacheMode,
    pub cancellation_id: String,
}

static ACTIVE_ARTIFACT_INFERENCES: std::sync::OnceLock<std::sync::Mutex<BTreeSet<String>>> = std::sync::OnceLock::new();

static CANCELLED_ARTIFACT_INFERENCES: std::sync::OnceLock<std::sync::Mutex<BTreeSet<String>>> = std::sync::OnceLock::new();

// 🚫️async: E1 pure accessor consumed by `impl Drop for ArtifactInferenceCancellationGuard`
// (Drop::drop is an externally-declared trait method and can never be async) — see R9.
pub fn active_artifact_inferences() -> &'static std::sync::Mutex<BTreeSet<String>> {
    ACTIVE_ARTIFACT_INFERENCES.get_or_init(|| std::sync::Mutex::new(BTreeSet::new()))
}

// 🚫️async: E1 pure accessor consumed by `impl Drop for ArtifactInferenceCancellationGuard`
// (Drop::drop is an externally-declared trait method and can never be async) — see R9.
pub fn cancelled_artifact_inferences() -> &'static std::sync::Mutex<BTreeSet<String>> {
    CANCELLED_ARTIFACT_INFERENCES.get_or_init(|| std::sync::Mutex::new(BTreeSet::new()))
}

pub struct ArtifactInferenceCancellationGuard(String);

impl Drop for ArtifactInferenceCancellationGuard {
    fn drop(&mut self) {
        if let Ok(mut active) = active_artifact_inferences().lock() {
            active.remove(&self.0);
        }
        if let Ok(mut cancelled) = cancelled_artifact_inferences().lock() {
            cancelled.remove(&self.0);
        }
    }
}

pub fn begin_artifact_inference(cancellation_id: &str) -> Result<ArtifactInferenceCancellationGuard, ArtifactInferenceExecutionError> {
    let mut active = active_artifact_inferences().lock().map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.unavailable", "active inference registry is poisoned"))?;
    if !active.insert(cancellation_id.to_owned()) {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.in-flight", format!("inference {cancellation_id:?} is already active")));
    }
    Ok(ArtifactInferenceCancellationGuard(cancellation_id.to_owned()))
}

pub fn inference_cancelled(cancellation_id: &str) -> Result<bool, ArtifactInferenceExecutionError> {
    cancelled_artifact_inferences().lock().map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.unavailable", "cancelled inference registry is poisoned")).map(|cancelled| cancelled.contains(cancellation_id))
}

fn bounded_len(value: u64, field: &str) -> Result<usize, ArtifactInferenceExecutionError> {
    usize::try_from(value).map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.budget", format!("{field} exceeds this runtime's address space")))
}

pub fn validate_wire_request_resources(request: &WireArtifactInferenceRequest) -> Result<(), ArtifactInferenceExecutionError> {
    if request.source_dialect.trim().is_empty() {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.source-dialect", "source dialect is required"));
    }
    ArtifactIdentity::parse(&request.source_dialect).map_err(|error| ArtifactInferenceExecutionError::new("artifact-inference.source-dialect", error.message()))?;
    if request.cancellation_id.trim().is_empty() {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.cancellation", "cancellation identity is required"));
    }
    if request.budgets.allocation_bytes == 0 || request.budgets.work_units == 0 || request.budgets.recursion_depth == 0 {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.budget", "allocation, work, and recursion budgets must be finite non-zero values"));
    }
    match request.requested_cache_mode {
        WireArtifactInferenceCacheMode::Incremental if request.previous_state.is_none() => {
            return Err(ArtifactInferenceExecutionError::new("artifact-inference.previous-state", "incremental inference requires previous state"));
        }
        WireArtifactInferenceCacheMode::Cold | WireArtifactInferenceCacheMode::Bypass if request.previous_state.is_some() => {
            return Err(ArtifactInferenceExecutionError::new("artifact-inference.previous-state", "cold or bypass inference must not receive previous state"));
        }
        _ => {}
    }
    let mut identifiers = BTreeSet::new();
    let mut bytes = request.policy.len().saturating_add(request.canonical_payload.len());
    if let Some(previous_state) = &request.previous_state {
        bytes = bytes.saturating_add(previous_state.len());
    }
    for (owner, payload) in &request.dependencies {
        if owner.trim().is_empty() || !identifiers.insert(owner) {
            return Err(ArtifactInferenceExecutionError::new("artifact-inference.dependencies", "dependency owners must be non-empty and unique"));
        }
        if inference_child_dependency_parts(owner).is_none() {
            ArtifactIdentity::parse(owner).map_err(|error| ArtifactInferenceExecutionError::new("artifact-inference.dependencies", error.message()))?;
        }
        bytes = bytes.saturating_add(owner.len()).saturating_add(payload.len());
    }
    let allocation = bounded_len(request.budgets.allocation_bytes, "allocation-bytes")?;
    if bytes > allocation {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.budget", format!("request consumes {bytes} bytes, above allocation limit {allocation}")));
    }
    let work = 1u64.saturating_add(request.dependencies.len() as u64);
    if work > request.budgets.work_units {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.budget", format!("request requires {work} work units, above limit {}", request.budgets.work_units)));
    }
    Ok(())
}

pub fn validate_inference_execution(request: &WireArtifactInferenceRequest, execution: &ArtifactInferenceExecution) -> Result<(), ArtifactInferenceExecutionError> {
    if execution.validity.trim().is_empty() || execution.quality.trim().is_empty() {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.result", "inference execution must report validity and quality"));
    }
    if execution.actual_cache_mode != request.requested_cache_mode {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.cache-mode", "inference execution did not honor the requested cache mode"));
    }
    let diagnostics = execution.diagnostics.iter().fold(0usize, |total, diagnostic| {
        total
            .saturating_add(diagnostic.code.len())
            .saturating_add(diagnostic.message.len())
            .saturating_add(diagnostic.severity.len())
            .saturating_add(diagnostic.parameters.iter().fold(0usize, |total, (key, value)| total.saturating_add(key.len()).saturating_add(value.len())))
    });
    if execution.complete&&execution.canonical_payload.is_none(){return Err(ArtifactInferenceExecutionError::new("artifact-inference.result-pending","terminal inference requires its original physical payload"));}
    let total = execution.canonical_payload.as_ref().map_or(0,Vec::len).saturating_add(diagnostics);
    let allocation = bounded_len(request.budgets.allocation_bytes, "allocation-bytes")?;
    if total > allocation {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.budget", format!("result consumes {total} bytes, above allocation limit {allocation}")));
    }
    Ok(())
}

pub async fn wire_list_artifact_inference_services() -> Result<Vec<u8>, ArtifactInferenceExecutionError> {
    let registry = artifact_inference_service_registry().read().map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.unavailable", "artifact inference registry is poisoned"))?;
    Ok(wire_list_artifact_inference_services_with_routes(&registry, &[]))
}

pub fn wire_list_artifact_inference_services_with_routes(registry: &ArtifactInferenceServiceRegistry, routes: &[ArtifactInferenceServiceMetadata]) -> Vec<u8> {
    let mut catalog: BTreeMap<ArtifactInferenceServiceKey, ArtifactInferenceServiceMetadata> = registry.metadata().into_iter().map(|metadata| (metadata.into(), metadata)).collect();
    for metadata in routes {
        catalog.insert((*metadata).into(), *metadata);
    }
    let metadata: Vec<WireArtifactInferenceMetadata> = catalog.into_values().map(Into::into).collect();
    semio_framework_pack_json::to_json_string(&metadata).into_bytes()
}

/// 📜️ The IDENTITY projection, not the whole row: a request names an inference and never
/// re-states the contract the service publishes, so comparing `payload` here would refuse
/// every real call the moment an owner published one.
pub fn validate_wire_request_metadata(request: &WireArtifactInferenceRequest, expected: &WireArtifactInferenceMetadata) -> Result<(), ArtifactInferenceExecutionError> {
    let actual = WireArtifactInferenceMetadata {
        owner: request.owner.clone(),
        artifact_kind: request.artifact_kind.clone(),
        artifact_schema: request.artifact_schema.clone(),
        artifact_schema_version: request.artifact_schema_version,
        inference_schema: request.inference_schema.clone(),
        inference_schema_version: request.inference_schema_version,
        algorithm_version: request.algorithm_version,
        policy_version: request.policy_version,
        payload: None,
    };
    let expected_identity = WireArtifactInferenceMetadata { payload: None, ..expected.clone() };
    if actual != expected_identity {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.metadata-mismatch", format!("request metadata {actual:?} does not match registered service {expected_identity:?}")));
    }
    Ok(())
}
