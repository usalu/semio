//! 💡️ Scope `hub.inference` — closed client intent and immutable server-selected GIS inference identity.
//!
//! Schema authority: [`🔣️.json`](./🔣️.json) (`https://semio.tech/schema/hub/inference/schema.json`),
//! TypeScript mirror: [`🟦️.ts`](./🟦️.ts). This module is the decoder authority; the `$defs` export ids
//! of the JSON Schema are the Rust type names below and are held field-for-field by `tests`.

use serde::{Deserialize, Serialize};

#[path = "✅️approval/🦀️.rs"]
pub mod approval;

/// ✅️ The closed approval intent; its decoder is the sibling [`approval`] leaf of this module.
pub type InferenceApprovalRequestV1 = approval::InferenceApprovalRequestV1;

/// ↩️ The GIS approval undo contract hub decodes; its shared struct is mounted in the os kernel.
pub type GisMapDocumentFrontierV1 = directory::os_directory::CheckpointPublicationFrontierV1;
pub type GisMapApprovalUndoHandleV1 = directory::os_directory::GisMapApprovalUndoHandleV1;
pub type GisMapApprovalUndoReceiptV1 = directory::os_directory::GisMapApprovalUndoReceiptV1;
pub type GisMapApprovalUndoRequestV1 = directory::os_directory::GisMapApprovalUndoRequestV1;

/// 🧬️ The scope id and `$id` every `hub.inference` export resolves under.
pub const SCHEMA_SCOPE: &str = "hub.inference";
pub const SCHEMA_ID: &str = "https://semio.tech/schema/hub/inference/schema.json";

//#region 🔖️ScopeSchemaExports
use semio_framework_schema_registry::{register_scope_schema_exports, FacetLeaves, SchemaExport, ScopeSchemaExports};

/// 🍃 The three leaves this module carries. The registry's spelling of "not provided" is an empty
/// body: no `🔗️.graphql` or `🛰️.proto` file exists here and no export claims one.
const ALL_LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };

/// 🏷️ The leaves of an export that is only ever validated, never transported by a Rust decoder —
/// exactly the set its `"x-semio-formats"` annotation declares (contract §B).
const VALIDATED_ONLY: FacetLeaves = FacetLeaves { rust: "", typescript: include_str!("🟦️.ts"), graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };

/// 📚️ The module document the annotation is read from, so the law never restates it.
#[cfg(test)]
const MODULE_JSON: &str = include_str!("🔣️.json");

/// 🏷️ `$defs` of `🔣️.json`, in declaration order.
const EXPORTS: [SchemaExport; 45] = [
    SchemaExport { id: "InferenceServerIdV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceDocumentScopeV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceRequestV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceParentDialectV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceBindingIdentityV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceIdentityV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceJobStateV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceProposalStateV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceLifecycleKindV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceLimitsV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceApprovalRequestV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceApprovalReceiptV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceApprovalOutboxV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "GisMapDocumentFrontierV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapApprovalUndoHandleV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapApprovalUndoRequestV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapApprovalUndoReceiptV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapApprovalUndoTargetV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "GisInferenceCheckpointControlFrameV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisInferenceCheckpointControlDirectionV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "GisMapInferencePreviewV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceMapBoundsV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceMapSummaryV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceProgressV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceEventV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceJobReceiptV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceEventPageV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceHybridLogicalTimestampV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceCommandPayloadV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceCommandV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceCommandLimitsV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceWalChainPolicyV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceWalTargetV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceCatalogOwnerV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceCatalogFrontierV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceCatalogDescriptorV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceCatalogPackageV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "InferenceCatalogServiceV1", leaves: ALL_LEAVES },
    SchemaExport { id: "InferenceCatalogSelectionV1", leaves: VALIDATED_ONLY },
    SchemaExport { id: "GisMapFrozenExecutionProtocolV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapFrozenPackageV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapFrozenArtifactV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapFrozenSurfaceV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapFrozenGrantV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GisMapFrozenBindingV1", leaves: ALL_LEAVES },
];

/// 📌️ Registers `hub.inference`'s named exports into the process-wide export catalog.
/// See `📋️execution-contract.md` §C and `semio_framework_schema_registry::resolve_schema_export`.
// 🚫️async: pure registration helper (no I/O)
pub fn register_scope_exports() {
    register_scope_schema_exports(ScopeSchemaExports { scope: SCHEMA_SCOPE, exports: &EXPORTS }).expect("hub.inference scope schema exports");
}
/// 🔬 Proves the registration at runtime rather than by inspection: it registers, resolves every
/// export in exactly the formats its `"x-semio-formats"` annotation names and in no other, and
/// asserts the scope is visible in the process-wide catalog.
#[cfg(test)]
#[path = "🧪️tests/🔬️scope-schema-export-law-standalone/🦀️.rs"]
mod scope_schema_export_law;
//#endregion 🔖️ScopeSchemaExports

pub const REQUEST_MAX_BYTES: usize = 1024;
pub const SERVER_ID_MAX_BYTES: usize = 96;
pub const INPUT_MAX_BYTES: usize = 65_536;
pub const RESULT_MAX_BYTES: usize = 16_384;
pub const PROPOSAL_MAX_BYTES: usize = 4096;
pub const IDENTITY_JSON_MAX_BYTES: usize = 8192;
pub const JOB_CAPACITY: usize = 128;
pub const PROGRESS_MAX_CURSOR: u64 = 16;
pub const EVENT_PAGE_MAX_ITEMS: usize = 8;
pub const CLAIM_LEASE_MAX_MS: u64 = 30_000;
pub const JOB_MAX_LIFETIME_MS: u64 = 120_000;
pub const SAFE_INTEGER_MAX: u64 = 9_007_199_254_740_991;
pub const GIS_SERVICE_ID: &str = "s.gis.gismap.inference";
pub const GIS_PACKAGE_ID: &str = "semio:gis";
pub const GIS_ARTIFACT_KIND: &str = "s.gis.gismap";
pub const GIS_DOCUMENT_SCHEMA: &str = "gis.map";
pub const GIS_EDITOR_SURFACE_ID: &str = "s.gis.gismap@1/*#editor";
pub const GIS_GRANTED_MODE: &str = "read-write-observe";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceRequestV1 {
    pub schema: String,
    pub version: u32,
    pub request_id: String,
    pub service_id: String,
    pub policy_version: u32,
    pub lifetime_ms: u64,
}

impl InferenceRequestV1 {
    pub fn decode(bytes: &[u8]) -> Result<Self, super::InferenceErrorV1> {
        if bytes.len() > REQUEST_MAX_BYTES {
            return Err(super::InferenceErrorV1::Bounds);
        }
        let request: Self = serde_json::from_slice(bytes).map_err(|_| super::InferenceErrorV1::Invalid)?;
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<(), super::InferenceErrorV1> {
        if self.schema != "semio.hub.inference-request/v1" || self.version != 1 || !hex(&self.request_id, 32) || self.service_id != GIS_SERVICE_ID || self.policy_version != 1 || self.lifetime_ms == 0 || self.lifetime_ms > JOB_MAX_LIFETIME_MS {
            return Err(super::InferenceErrorV1::Invalid);
        }
        Ok(())
    }
}

/// 🧬️ The exact retained parent dialect the frozen Map binding admitted; never a client label.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceParentDialectV1 {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}

/// 🧊️ Every frozen executable fact of the selected GIS Map binding, carried inside job identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceBindingIdentityV1 {
    pub digest: String,
    pub catalog_generation_id: String,
    pub package_id: String,
    pub package_version: String,
    pub component_sha256: String,
    pub component_blake3: String,
    pub artifact_kind: String,
    pub document_schema: String,
    pub parent_dialect: InferenceParentDialectV1,
    pub surface_id: String,
    pub granted_mode: String,
    pub service_id: String,
    pub service_version: u32,
    pub algorithm_version: u32,
}

impl InferenceBindingIdentityV1 {
    pub fn validate(&self) -> Result<(), super::InferenceErrorV1> {
        if [&self.digest, &self.catalog_generation_id, &self.component_sha256, &self.component_blake3].iter().any(|digest| !hex(digest, 64))
            || self.package_id != GIS_PACKAGE_ID
            || !server_id(&self.package_version)
            || self.artifact_kind != GIS_ARTIFACT_KIND
            || self.document_schema != GIS_DOCUMENT_SCHEMA
            || self.parent_dialect.artifact_kind != GIS_ARTIFACT_KIND
            || self.parent_dialect.standard != "1"
            || self.parent_dialect.subset != "*"
            || self.surface_id != GIS_EDITOR_SURFACE_ID
            || self.granted_mode != GIS_GRANTED_MODE
            || self.service_id != GIS_SERVICE_ID
            || self.service_version != 1
            || self.algorithm_version != 1
        {
            return Err(super::InferenceErrorV1::Invalid);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceIdentityV1 {
    pub request: InferenceRequestV1,
    pub user_id: String,
    pub session_id: String,
    pub authorization_generation: u64,
    pub space_id: String,
    pub document_id: String,
    pub descriptor_digest: String,
    pub binding: InferenceBindingIdentityV1,
    pub head_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_hash: String,
    pub input_hash: String,
}

impl InferenceIdentityV1 {
    pub fn validate(&self) -> Result<(), super::InferenceErrorV1> {
        self.request.validate()?;
        self.binding.validate()?;
        if [&self.user_id, &self.session_id, &self.space_id, &self.document_id].iter().any(|id| !server_id(id))
            || [&self.descriptor_digest, &self.chain_hash, &self.input_hash].iter().any(|digest| !hex(digest, 64))
            || self.authorization_generation == 0
            || [self.authorization_generation, self.head_ordinal, self.last_commit_seq].iter().any(|value| *value > SAFE_INTEGER_MAX)
            || !(server_id(&self.head_edit_id) || self.head_ordinal == 0 && self.head_edit_id.is_empty())
        {
            return Err(super::InferenceErrorV1::Invalid);
        }
        Ok(())
    }

    pub fn digest(&self) -> Result<String, super::InferenceErrorV1> {
        self.validate()?;
        let mut bytes = b"semio.hub.inference-identity/v1\0".to_vec();
        bytes.extend(serde_json::to_vec(self).map_err(|_| super::InferenceErrorV1::Invalid)?);
        Ok(super::sha256(&bytes))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceJobStateV1 {
    Accepted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceProposalStateV1 {
    None,
    Offered,
    Approved,
    Stale,
    Cancelled,
}

/// ⚙️ The frozen app-channel protocol version the selected GIS Map package was admitted under.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapFrozenExecutionProtocolV1 {
    pub app_channel_version: u32,
}

/// 📦️ The exact catalog package bytes the frozen binding pins, by both digests and descriptor hash.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapFrozenPackageV1 {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub component_sha256: String,
    pub component_blake3: String,
    pub descriptor_byte_sha256: String,
    pub execution_protocol: GisMapFrozenExecutionProtocolV1,
}

/// 🗿️ The artifact kind, document schema and pack schema hash the frozen binding is closed over.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapFrozenArtifactV1 {
    pub kind: String,
    pub schema: String,
    pub pack_schema_hash: String,
}

/// 🖼️ The single editor surface the frozen binding admits; no other role or renderer may be used.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapFrozenSurfaceV1 {
    pub surface_id: String,
    pub app_id: String,
    pub window_kind_id: String,
    pub role: String,
    pub renderer_target: String,
}

/// 🔑️ The exact grant the frozen surface carries; inference requires all three.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapFrozenGrantV1 {
    pub read: bool,
    pub write: bool,
    pub observe: bool,
}

/// 💡️ One declared inference service of the catalog package, exactly as the descriptor contributed it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceCatalogServiceV1 {
    pub owner: String,
    pub contributor: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub document_schema: String,
    pub document_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub depends_on: Vec<String>,
}

/// 🧊️ Every frozen executable fact of one GIS Map editor selection; its canonical bytes are the binding digest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapFrozenBindingV1 {
    pub catalog_generation_id: String,
    pub package: GisMapFrozenPackageV1,
    pub artifact: GisMapFrozenArtifactV1,
    pub parent_dialect: InferenceParentDialectV1,
    pub surface: GisMapFrozenSurfaceV1,
    pub grant: GisMapFrozenGrantV1,
    pub service: InferenceCatalogServiceV1,
    pub native_executable: String,
}

/// 🧾️ The closed receipt a submitted job returns; it never carries private result or base bytes.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceJobReceiptV1 {
    pub schema: &'static str,
    pub job_id: String,
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub proposal_hash: Option<String>,
    pub cursor: u64,
    pub expires_at_ms: u64,
}

/// 📈️ One owner-private progress row rendered on the wire.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceProgressV1 {
    pub cursor: u64,
    pub run_epoch: u64,
    pub completed: u64,
    pub total: u64,
    pub at_ms: u64,
}

/// 🗓️ One owner-private lifecycle event rendered on the wire.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceEventV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
}

/// 🗺️ The bounded, host-only geometry an owner may inspect before approving a proposal.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapInferencePreviewV1 {
    pub schema: &'static str,
    pub job_id: String,
    pub proposal_hash: String,
    pub region_id: String,
    pub ring: [[f64; 2]; 5],
}

/// 📃️ The owner-private bounded page a single `events` read returns.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceEventPageV1 {
    pub schema: &'static str,
    pub job_id: String,
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub cancel_requested: bool,
    pub stale: bool,
    pub proposal_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<GisMapInferencePreviewV1>,
    pub events: Vec<InferenceEventV1>,
    pub progress: Vec<InferenceProgressV1>,
    pub next_cursor: u64,
}

/// ✅️ The closed approval outcome; `applied` is true only after a real committed-WAL witness.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceApprovalReceiptV1 {
    pub schema: &'static str,
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub applied: bool,
    pub undo: GisMapApprovalUndoHandleV1,
}

/// ⏸️ The exact length-prefixed frame the checkpoint-control pipe carries in both directions.
#[cfg(feature = "test-support")]
#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GisInferenceCheckpointControlFrameV1 {
    pub schema: String,
    pub version: u8,
    pub sequence: u8,
    pub kind: String,
    pub job_id: String,
}

pub fn hex(value: &str, length: usize) -> bool {
    value.len() == length && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(super) fn server_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= SERVER_ID_MAX_BYTES && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
