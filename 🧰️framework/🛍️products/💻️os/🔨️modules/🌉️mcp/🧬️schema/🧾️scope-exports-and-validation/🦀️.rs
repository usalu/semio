//! 🧾️ Scope exports and validation of the os.mcp registry: the format leaves, the scope export declaration and the validator compiler. See [`super`].

use super::*;

//#region 🔖️ScopeSchemaExports
/// 🍃 The three format leaves `os.mcp` publishes. One `🧬️schema/` module carries every named
/// export, so all of them resolve to the same documents; this scope ships no GraphQL or proto leaf.
const LEAVES: FacetLeaves = FacetLeaves { rust: concat!(include_str!("../🦀️.rs"), include_str!("../🧭️core-and-mutation-tool-schemas/🦀️.rs"), include_str!("../🛡️untrusted-content-schemas/🦀️.rs"), include_str!("../📄️artifact-tool-schemas/🦀️.rs"), include_str!("../🖥️ui-and-catalog-tool-schemas/🦀️.rs"), include_str!("../💡️inference-tool-schemas/🦀️.rs"), include_str!("../📡️artifact-inference-wire/🦀️.rs"), include_str!("../📚️schema-catalog/🦀️.rs"), include_str!("🦀️.rs")), typescript: include_str!("../🟦️.ts"), graphql: "", json_schema: SCHEMA_MIRROR_JSON, proto: "" };

/// 🏷️ `$defs` of `🔣️.json`, which `🧪️Tests::the_json_mirror_publishes_exactly_the_registry_exports`
/// pins to [`schemas`]; `🧪️Tests::the_scope_export_declaration_matches_the_registry` pins this list to
/// the same set, so a new registry entry cannot be published without being resolvable.
const EXPORTS: [SchemaExport; 75] = [
    SchemaExport { id: "ActionInvokeInput", leaves: LEAVES },
    SchemaExport { id: "ArtifactInferenceBudgetV1", leaves: LEAVES },
    SchemaExport { id: "ArtifactInferenceCacheModeV1", leaves: LEAVES },
    SchemaExport { id: "ArtifactInferenceRequestV1", leaves: LEAVES },
    SchemaExport { id: "ArtifactInferenceResultV1", leaves: LEAVES },
    SchemaExport { id: "ActionPrepareInput", leaves: LEAVES },
    SchemaExport { id: "ArtifactCreateInput", leaves: LEAVES },
    SchemaExport { id: "ArtifactCreateOutput", leaves: LEAVES },
    SchemaExport { id: "ArtifactCreateTemplateInput", leaves: LEAVES },
    SchemaExport { id: "ArtifactExportInput", leaves: LEAVES },
    SchemaExport { id: "ArtifactExportOutput", leaves: LEAVES },
    SchemaExport { id: "ArtifactOpenInput", leaves: LEAVES },
    SchemaExport { id: "ArtifactOpenOutput", leaves: LEAVES },
    SchemaExport { id: "ArtifactSnapshotInput", leaves: LEAVES },
    SchemaExport { id: "ArtifactSnapshotOutput", leaves: LEAVES },
    SchemaExport { id: "ArtifactValidateInput", leaves: LEAVES },
    SchemaExport { id: "ArtifactValidateOutput", leaves: LEAVES },
    SchemaExport { id: "CallToolResult", leaves: LEAVES },
    SchemaExport { id: "CapabilitiesDescribeInput", leaves: LEAVES },
    SchemaExport { id: "CapabilitiesDescribeOutput", leaves: LEAVES },
    SchemaExport { id: "CapabilitiesSearchInput", leaves: LEAVES },
    SchemaExport { id: "CapabilitiesSearchOutput", leaves: LEAVES },
    SchemaExport { id: "CapabilityActionInput", leaves: LEAVES },
    SchemaExport { id: "CapabilityGenericInput", leaves: LEAVES },
    SchemaExport { id: "CapabilityGenericOutput", leaves: LEAVES },
    SchemaExport { id: "ContentBlock", leaves: LEAVES },
    SchemaExport { id: "ContextResolveInput", leaves: LEAVES },
    SchemaExport { id: "ContextResolveOutput", leaves: LEAVES },
    SchemaExport { id: "ContextSummary", leaves: LEAVES },
    SchemaExport { id: "ConversationReplyInput", leaves: LEAVES },
    SchemaExport { id: "ConversationReplyOutput", leaves: LEAVES },
    SchemaExport { id: "GatewayError", leaves: LEAVES },
    SchemaExport { id: "GatewayErrorCode", leaves: LEAVES },
    SchemaExport { id: "HubInferenceApprovalRequestV1", leaves: LEAVES },
    SchemaExport { id: "HandleInput", leaves: LEAVES },
    SchemaExport { id: "InferenceApproveInput", leaves: LEAVES },
    SchemaExport { id: "InferenceGetInput", leaves: LEAVES },
    SchemaExport { id: "InferenceGetOutput", leaves: LEAVES },
    SchemaExport { id: "InferenceJobHandleInput", leaves: LEAVES },
    SchemaExport { id: "InferenceJobOutput", leaves: LEAVES },
    SchemaExport { id: "InferenceListInput", leaves: LEAVES },
    SchemaExport { id: "InferenceListOutput", leaves: LEAVES },
    SchemaExport { id: "InferenceRetainedAuthority", leaves: LEAVES },
    SchemaExport { id: "InferenceRetainedTurnInput", leaves: LEAVES },
    SchemaExport { id: "InferenceOriginalWork", leaves: LEAVES },
    SchemaExport { id: "InferenceRunInput", leaves: LEAVES },
    SchemaExport { id: "InferenceRunOutput", leaves: LEAVES },
    SchemaExport { id: "InferenceSubmitInput", leaves: LEAVES },
    SchemaExport { id: "InvocationReport", leaves: LEAVES },
    SchemaExport { id: "InvocationStatus", leaves: LEAVES },
    SchemaExport { id: "JobCancelInput", leaves: LEAVES },
    SchemaExport { id: "JobGetInput", leaves: LEAVES },
    SchemaExport { id: "JobSnapshotOutput", leaves: LEAVES },
    SchemaExport { id: "JobState", leaves: LEAVES },
    SchemaExport { id: "JobStatus", leaves: LEAVES },
    SchemaExport { id: "NullableRevisionStamp", leaves: LEAVES },
    SchemaExport { id: "PreparedActionReport", leaves: LEAVES },
    SchemaExport { id: "Prompt", leaves: LEAVES },
    SchemaExport { id: "PromptArgument", leaves: LEAVES },
    SchemaExport { id: "PromptGetResult", leaves: LEAVES },
    SchemaExport { id: "PromptMessage", leaves: LEAVES },
    SchemaExport { id: "Resource", leaves: LEAVES },
    SchemaExport { id: "ResourceContent", leaves: LEAVES },
    SchemaExport { id: "ResourceTemplate", leaves: LEAVES },
    SchemaExport { id: "RevisionStamp", leaves: LEAVES },
    SchemaExport { id: "SearchHit", leaves: LEAVES },
    SchemaExport { id: "Tool", leaves: LEAVES },
    SchemaExport { id: "TransactionBeginInput", leaves: LEAVES },
    SchemaExport { id: "UiDialogOpenInput", leaves: LEAVES },
    SchemaExport { id: "UiFocusInput", leaves: LEAVES },
    SchemaExport { id: "UiFocusOutput", leaves: LEAVES },
    SchemaExport { id: "UiRevealInput", leaves: LEAVES },
    SchemaExport { id: "UiRevealOutput", leaves: LEAVES },
    SchemaExport { id: "UntrustedContentV1", leaves: LEAVES },
    SchemaExport { id: "UntrustedProvenanceV1", leaves: LEAVES },
];

/// 📌️ Registers `os.mcp`'s named exports into the OS-wide export catalog.
/// See `📋️execution-contract.md` §C and `semio_framework_schema_registry::resolve_schema_export`.
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_scope_exports() {
    register_exports(ScopeSchemaExports { scope: "os.mcp", exports: &EXPORTS }).expect("os.mcp scope schema exports");
}

/// 🏷️ The export ids this scope declares, for tests and the derived catalog cross-check.
pub fn scope_export_ids() -> Vec<&'static str> {
    EXPORTS.iter().map(|export| export.id).collect()
}
//#endregion 🔖️ScopeSchemaExports

//#region ✅️Validation
/// 🧬️ Compiles MCP's serde boundary through the repo-owned string contract without leaking either
/// JSON implementation across the framework schema API.
pub fn compile_validator(schema: &serde_json::Value) -> Result<semio_framework_schema::OwnedJsonSchemaValidator, String> {
    let schema = serde_json::to_string(schema).map_err(|error| error.to_string())?;
    semio_framework_schema::OwnedJsonSchemaValidator::compile(&schema).map_err(|error| error.to_string())
}

/// ✅️ Validates one MCP value and preserves the owned validator's deterministic first diagnostic.
pub fn validate(validator: &semio_framework_schema::OwnedJsonSchemaValidator, value: &serde_json::Value) -> Result<semio_framework_schema::ValidationProgress, String> {
    let value = serde_json::to_string(value).map_err(|error| error.to_string())?;
    validator.validate_json(&value).map_err(|error| error.to_string())
}
//#endregion ✅️Validation
