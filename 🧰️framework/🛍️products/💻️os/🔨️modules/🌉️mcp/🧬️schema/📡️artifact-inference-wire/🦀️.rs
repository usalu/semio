//! 📡️ Artifact inference wire of the os.mcp registry: the wire version, the budget, cache mode, request and result records and the hub inference approval schema. See [`super`].

use super::*;

//#region 🔖️ToolSchemas
//#region 🔖️ArtifactInferenceWire
/// 💡️ `wireVersion` of the guest `artifact-inference-request`/`-result` pair this scope mirrors —
/// the same constant the guest SDK publishes as `ARTIFACT_INFERENCE_WIRE_VERSION`. The router
/// rejects a result whose `wireVersion` does not echo the request's, so a bump on either side is a
/// loud failure rather than a silent reinterpretation.
pub const ARTIFACT_INFERENCE_WIRE_VERSION: u32 = 2;

/// ⏱️ Finite host-enforced inference work limits — field-for-field the guest's own
/// `artifact-inference-budget`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactInferenceBudgetV1 {
    pub allocation_bytes: u64,
    pub work_units: u64,
    pub recursion_depth: u32,
}

fn retained_clone_grant_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    serde_json::from_str(semio_framework_value::retained_clone::RETAINED_CLONE_GRANT_SCHEMA).expect("canonical retained grant schema")
}

/// 🗃️ Requested or actual inference cache behaviour — the guest's own closed vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ArtifactInferenceCacheModeV1 {
    Cold,
    Incremental,
    Bypass,
}

/// 💡️ The request `semio_framework_plugin_host::ArtifactInferenceRouter::infer` decodes — os is a
/// CLIENT of that router here, so this export is an explicit MIRROR of the router's own private
/// `InferenceRouteRequest` (which that crate does not export), never a second authority. The router
/// re-checks every identity field against its registered route and against the guest's echo before
/// any result is publishable, so a drift between this mirror and the router is a decode failure at
/// the first call, not a silently wrong answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactInferenceRequestV1 {
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
    pub budgets: ArtifactInferenceBudgetV1,
    #[schemars(schema_with = "retained_clone_grant_schema")]
    pub retained: semio_framework_value::retained_clone::RetainedCloneGrant,
    pub cancellation_id: String,
    pub previous_state: Option<Vec<u8>>,
    pub requested_cache_mode: ArtifactInferenceCacheModeV1,
    pub canonical_payload: Vec<u8>,
    pub dependencies: Vec<(String, Vec<u8>)>,
}

/// 💡️ The subset of the guest's `artifact-inference-result` this gateway reads back. The guest wire
/// carries more (diagnostics, provenance, validity, quality); those belong to the plugin's own
/// domain contract and the router has ALREADY asserted that every identity field echoes the request
/// exactly before this decode runs, so mirroring them here would duplicate an authority this scope
/// does not own. Unknown fields are therefore accepted and ignored — deliberately, not by oversight.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactInferenceResultV1 {
    pub wire_version: u32,
    pub owner: String,
    pub artifact_kind: String,
    pub inference_schema: String,
    pub revision: u64,
    pub generation: u64,
    pub complete: bool,
    pub actual_cache_mode: ArtifactInferenceCacheModeV1,
    pub canonical_payload: Option<Vec<u8>>,
}
//#endregion 🔖️ArtifactInferenceWire

/// 🔢️ Canonical lower-hex patterns for bounded inference identifiers.
pub(super) fn hex_pattern(length: usize) -> String {
    format!("^[0-9a-f]{{{length}}}$")
}

//#region 🔖️InferenceApproval
/// ✅️ The framework-owned closed approval intent for an inference job and proposal digest.
/// See `✅️approval-request/🔣️.json`; concrete inference hosts consume this contract.
pub fn hub_inference_approval_request_schema() -> serde_json::Value {
    serde_json::from_str(include_str!("../✅️approval-request/🔣️.json")).expect("the authored approval contract parses")
}
//#endregion 🔖️InferenceApproval
//#endregion 🔖️ToolSchemas
