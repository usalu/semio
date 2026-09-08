//! 💡️ Scope `hub.inference` — closed client intent and immutable server-selected GIS inference identity.
//!
//! Schema authority: [`🔣️.json`](./🔣️.json) (`https://semio.tech/schema/hub/inference/schema.json`),
//! TypeScript mirror: [`🟦️.ts`](./🟦️.ts). This module is the decoder authority; the `$defs` export ids
//! of the JSON Schema are the Rust type names below and are held field-for-field by `tests`.

use serde::{Deserialize, Serialize};

#[path = "✅️approval/🦀️.rs"]
mod approval;
pub use approval::InferenceApprovalRequestV1;

/// ↩️ The GIS approval undo contract hub decodes; its shared struct is mounted in the os kernel.
pub use directory::os_directory::{CheckpointPublicationFrontierV1 as GisMapDocumentFrontierV1, GisMapApprovalUndoHandleV1, GisMapApprovalUndoReceiptV1, GisMapApprovalUndoRequestV1};

/// 🧬️ The scope id and `$id` every `hub.inference` export resolves under.
pub const SCHEMA_SCOPE: &str = "hub.inference";
pub const SCHEMA_ID: &str = "https://semio.tech/schema/hub/inference/schema.json";

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

pub fn hex(value: &str, length: usize) -> bool {
    value.len() == length && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(super) fn server_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= SERVER_ID_MAX_BYTES && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPORTS: [&str; 45] = [
        "InferenceServerIdV1",
        "InferenceDocumentScopeV1",
        "InferenceRequestV1",
        "InferenceParentDialectV1",
        "InferenceBindingIdentityV1",
        "InferenceIdentityV1",
        "InferenceJobStateV1",
        "InferenceProposalStateV1",
        "InferenceLifecycleKindV1",
        "InferenceLimitsV1",
        "InferenceApprovalRequestV1",
        "InferenceApprovalReceiptV1",
        "InferenceApprovalOutboxV1",
        "GisMapDocumentFrontierV1",
        "GisMapApprovalUndoHandleV1",
        "GisMapApprovalUndoRequestV1",
        "GisMapApprovalUndoReceiptV1",
        "GisMapApprovalUndoTargetV1",
        "GisInferenceCheckpointControlFrameV1",
        "GisInferenceCheckpointControlDirectionV1",
        "GisMapInferencePreviewV1",
        "InferenceMapBoundsV1",
        "InferenceMapSummaryV1",
        "InferenceProgressV1",
        "InferenceEventV1",
        "InferenceJobReceiptV1",
        "InferenceEventPageV1",
        "InferenceHybridLogicalTimestampV1",
        "InferenceCommandPayloadV1",
        "InferenceCommandV1",
        "InferenceCommandLimitsV1",
        "InferenceWalChainPolicyV1",
        "InferenceWalTargetV1",
        "InferenceCatalogOwnerV1",
        "InferenceCatalogFrontierV1",
        "InferenceCatalogDescriptorV1",
        "InferenceCatalogPackageV1",
        "InferenceCatalogServiceV1",
        "InferenceCatalogSelectionV1",
        "GisMapFrozenExecutionProtocolV1",
        "GisMapFrozenPackageV1",
        "GisMapFrozenArtifactV1",
        "GisMapFrozenSurfaceV1",
        "GisMapFrozenGrantV1",
        "GisMapFrozenBindingV1",
    ];

    /// 🫧️ The only properties an export may declare without requiring them.
    const OPTIONAL: [(&str, &str); 1] = [("InferenceEventPageV1", "preview")];

    fn module() -> serde_json::Value {
        serde_json::from_str(include_str!("🔣️.json")).expect("hub.inference module is valid JSON")
    }

    fn declared(module: &serde_json::Value, export: &str, keyword: &str) -> Vec<String> {
        let node = &module["$defs"][export];
        assert!(!node.is_null(), "hub.inference exports no {export}");
        let mut keys: Vec<String> = match keyword {
            "required" => node["required"].as_array().unwrap_or(&Vec::new()).iter().filter_map(|value| value.as_str().map(str::to_owned)).collect(),
            _ => node["properties"].as_object().map(|properties| properties.keys().cloned().collect()).unwrap_or_default(),
        };
        keys.sort();
        keys
    }

    fn encoded<T: Serialize>(value: &T) -> Vec<String> {
        let mut keys: Vec<String> = serde_json::to_value(value).expect("serializable").as_object().expect("object").keys().cloned().collect();
        keys.sort();
        keys
    }

    /// 🧬️ Every export is declared exactly once, closed, and reachable under the scope `$id`.
    #[test]
    fn hub_inference_module_declares_every_export_as_a_closed_draft_07_contract() {
        let module = module();
        assert_eq!(module["$schema"], "http://json-schema.org/draft-07/schema#");
        assert_eq!(module["$id"], SCHEMA_ID);
        let defs = module["$defs"].as_object().expect("$defs");
        let exported: Vec<&String> = defs.keys().filter(|key| key.starts_with(|first: char| first.is_ascii_uppercase())).collect();
        assert_eq!(exported.len(), EXPORTS.len(), "exported $defs are {exported:?}");
        for export in EXPORTS {
            let node = &defs[export];
            assert!(!node.is_null(), "hub.inference exports no {export}");
            if node["type"] == "object" {
                assert_eq!(node["additionalProperties"], serde_json::Value::Bool(false), "{export} is not closed");
                let required = declared(&module, export, "required");
                let optional: Vec<String> = declared(&module, export, "properties").into_iter().filter(|key| !required.contains(key)).collect();
                assert!(required.iter().all(|key| declared(&module, export, "properties").contains(key)), "{export} requires an undeclared property");
                assert_eq!(optional, OPTIONAL.iter().filter(|(owner, _)| *owner == export).map(|(_, key)| (*key).to_owned()).collect::<Vec<String>>(), "{export} optional properties are undeclared");
            }
        }
    }

    /// 📥 Compiles one export through the owned draft-07 validator by pinning the document root at it.
    fn structural(module: &serde_json::Value, export: &str) -> semio_framework_schema::OwnedJsonSchemaValidator {
        let mut document = module.clone();
        document["$ref"] = serde_json::Value::String(format!("#/$defs/{export}"));
        semio_framework_schema::OwnedJsonSchemaValidator::compile(&document.to_string()).unwrap_or_else(|error| panic!("{export} does not compile: {error:?}"))
    }

    /// ✅️ Every fixture member the Rust decoders accept is also structurally valid against its export.
    #[test]
    fn hub_inference_fixtures_validate_through_the_owned_draft_07_validator() {
        let module = module();
        let ledger: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).expect("ledger fixture");
        let approval: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/✅️inference-approval-v1/🔣️.json")).expect("approval fixture");
        let undo: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/↩️gis-map-approval-undo-v1/🔣️.json")).expect("undo fixture");
        let frozen: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json")).expect("frozen binding fixture");
        let proposal: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture");
        let proof: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json")).expect("wal proof fixture");
        let accepted: [(&str, &serde_json::Value); 12] = [
            ("InferenceIdentityV1", &ledger["identity"]),
            ("InferenceRequestV1", &ledger["identity"]["request"]),
            ("InferenceBindingIdentityV1", &ledger["identity"]["binding"]),
            ("InferenceApprovalOutboxV1", &ledger["outbox"]),
            ("InferenceMapSummaryV1", &ledger["expectedInference"]),
            ("InferenceApprovalRequestV1", &approval["request"]),
            ("GisMapApprovalUndoTargetV1", &undo["target"]),
            ("GisMapApprovalUndoRequestV1", &undo["request"]),
            ("GisMapFrozenBindingV1", &frozen["binding"]),
            ("GisMapInferencePreviewV1", &proposal["preview"]),
            ("InferenceLimitsV1", &proposal["limits"]),
            ("InferenceCommandV1", &proof["command"]),
        ];
        for (export, value) in accepted {
            let validator = structural(&module, export);
            let _ = validator.validate_json(&value.to_string()).unwrap_or_else(|error| panic!("{export} rejected its own fixture: {error:?}"));
        }
        let approval_validator = structural(&module, "InferenceApprovalRequestV1");
        for hostile in approval["hostile"].as_array().expect("hostile rows") {
            let mut candidate = approval["request"].clone();
            candidate[hostile["field"].as_str().expect("field")] = hostile["value"].clone();
            assert!(approval_validator.validate_json(&candidate.to_string()).is_err(), "structural validator admitted {}", hostile["code"]);
        }
    }

    /// 🤝️ The JSON Schema exports and the Rust decoders carry exactly the same fields.
    #[test]
    fn hub_inference_exports_agree_with_the_rust_decoders_field_for_field() {
        let module = module();
        let ledger: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).expect("ledger fixture");
        let identity: InferenceIdentityV1 = serde_json::from_value(ledger["identity"].clone()).expect("identity decodes");
        identity.validate().expect("identity validates");
        assert_eq!(encoded(&identity), declared(&module, "InferenceIdentityV1", "required"));
        assert_eq!(encoded(&identity.request), declared(&module, "InferenceRequestV1", "required"));
        assert_eq!(encoded(&identity.binding), declared(&module, "InferenceBindingIdentityV1", "required"));
        assert_eq!(encoded(&identity.binding.parent_dialect), declared(&module, "InferenceParentDialectV1", "required"));

        let approval: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/✅️inference-approval-v1/🔣️.json")).expect("approval fixture");
        let request = InferenceApprovalRequestV1::decode(&serde_json::to_vec(&approval["request"]).expect("bytes")).expect("approval decodes");
        assert_eq!(encoded(&request), declared(&module, "InferenceApprovalRequestV1", "required"));

        let undo: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/↩️gis-map-approval-undo-v1/🔣️.json")).expect("undo fixture");
        let undo_request: GisMapApprovalUndoRequestV1 = serde_json::from_value(undo["request"].clone()).expect("undo request decodes");
        assert!(undo_request.validate(), "undo request validates");
        assert_eq!(encoded(&undo_request), declared(&module, "GisMapApprovalUndoRequestV1", "required"));
        assert_eq!(encoded(&undo_request.expected_current), declared(&module, "GisMapDocumentFrontierV1", "required"));
        let target = &undo["target"];
        let mut target_keys: Vec<String> = target.as_object().expect("target object").keys().cloned().collect();
        target_keys.sort();
        assert_eq!(target_keys, declared(&module, "GisMapApprovalUndoTargetV1", "required"));

        let handle = GisMapApprovalUndoHandleV1 { target_id: undo_request.target_id.clone(), expected_current: undo_request.expected_current.clone() };
        assert_eq!(encoded(&handle), declared(&module, "GisMapApprovalUndoHandleV1", "required"));
    }

    /// 🚧️ Every hostile fixture row the module rejects is declared as a contract-stage expectation.
    #[test]
    fn hub_inference_approval_hostiles_declare_their_rejection_stage() {
        let approval: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/✅️inference-approval-v1/🔣️.json")).expect("approval fixture");
        let hostiles = approval["hostile"].as_array().expect("hostile rows");
        assert_eq!(hostiles.len(), 14);
        for hostile in hostiles {
            assert_eq!(hostile["stage"], "contract", "{}", hostile["field"]);
            assert_eq!(hostile["result"], "rejected", "{}", hostile["field"]);
            assert!(hostile["code"].as_str().is_some_and(|code| !code.is_empty()), "{}", hostile["field"]);
            let mut candidate = approval["request"].clone();
            candidate[hostile["field"].as_str().expect("field")] = hostile["value"].clone();
            assert_eq!(InferenceApprovalRequestV1::decode(&serde_json::to_vec(&candidate).expect("bytes")), Err(super::super::InferenceErrorV1::Invalid), "{}", hostile["code"]);
        }
    }
}
