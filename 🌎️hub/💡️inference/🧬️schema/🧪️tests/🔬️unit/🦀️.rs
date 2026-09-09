use super::*;

const EXPORTS: [&str; 51] = [
    "InferenceServerIdV1",
    "InferenceDocumentScopeV1",
    "InferenceRequestV1",
    "InferenceJobReconcileRequestV1",
    "InferenceJobReconcileApprovalStateV1",
    "InferenceJobReconcileApprovalV1",
    "InferenceJobReconcilePageV1",
    "InferenceJobReconcileJobV1",
    "InferenceJobReconcileResultV1",
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
    serde_json::from_str(include_str!("../../🔣️.json")).expect("hub.inference module is valid JSON")
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
    let ledger: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).expect("ledger fixture");
    let approval: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/✅️inference-approval-v1/🔣️.json")).expect("approval fixture");
    let undo: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/↩️gis-map-approval-undo-v1/🔣️.json")).expect("undo fixture");
    let frozen: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json")).expect("frozen binding fixture");
    let proposal: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture");
    let proof: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json")).expect("wal proof fixture");
    let reconcile: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧭️inference-job-reconcile-v1/🔣️.json")).expect("reconcile fixture");
    let checkpoint: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/⏸️gis-inference-checkpoint-control-v1/🔣️.json")).expect("checkpoint fixture");
    let accepted: [(&str, &serde_json::Value); 15] = [
        ("InferenceIdentityV1", &ledger["identity"]),
        ("InferenceRequestV1", &ledger["identity"]["request"]),
        ("InferenceJobReconcileRequestV1", &reconcile["request"]),
        ("InferenceJobReconcileResultV1", &reconcile["results"][0]["value"]),
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
        ("GisInferenceCheckpointControlFrameV1", &checkpoint["frames"][0]["frame"]),
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
    let ledger: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).expect("ledger fixture");
    let identity: InferenceIdentityV1 = serde_json::from_value(ledger["identity"].clone()).expect("identity decodes");
    identity.validate().expect("identity validates");
    assert_eq!(encoded(&identity), declared(&module, "InferenceIdentityV1", "required"));
    assert_eq!(encoded(&identity.request), declared(&module, "InferenceRequestV1", "required"));
    assert_eq!(encoded(&identity.binding), declared(&module, "InferenceBindingIdentityV1", "required"));
    assert_eq!(encoded(&identity.binding.parent_dialect), declared(&module, "InferenceParentDialectV1", "required"));

    let approval: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/✅️inference-approval-v1/🔣️.json")).expect("approval fixture");
    let request = InferenceApprovalRequestV1::decode(&serde_json::to_vec(&approval["request"]).expect("bytes")).expect("approval decodes");
    assert_eq!(encoded(&request), declared(&module, "InferenceApprovalRequestV1", "required"));

    let reconcile: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧭️inference-job-reconcile-v1/🔣️.json")).expect("reconcile fixture");
    let request = InferenceJobReconcileRequestV1::decode(&serde_json::to_vec(&reconcile["request"]).expect("bytes")).expect("reconcile request decodes");
    assert_eq!(encoded(&request), declared(&module, "InferenceJobReconcileRequestV1", "required"));

    #[cfg(feature = "test-support")]
    {
        let checkpoint: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/⏸️gis-inference-checkpoint-control-v1/🔣️.json")).expect("checkpoint fixture");
        let frame: GisInferenceCheckpointControlFrameV1 = serde_json::from_value(checkpoint["frames"][0]["frame"].clone()).expect("checkpoint frame decodes");
        assert_eq!(encoded(&frame), declared(&module, "GisInferenceCheckpointControlFrameV1", "required"));
    }

    let undo: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/↩️gis-map-approval-undo-v1/🔣️.json")).expect("undo fixture");
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
    let approval: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/✅️inference-approval-v1/🔣️.json")).expect("approval fixture");
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
