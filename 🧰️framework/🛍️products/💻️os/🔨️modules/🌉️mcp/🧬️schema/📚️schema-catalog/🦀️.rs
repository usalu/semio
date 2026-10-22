//! 📚️ Schema catalog of the os.mcp registry: the schemas registry, the committed mirror document and the tool output schema. See [`super`].

use super::*;

//#region 🔖️SchemaCatalog
/// 📋️ `(ExportId, JSON Schema document)` for every schema the `os.mcp` scope owns — the ONE
/// registry every facet reads from and the normative source `🧬️schema/🔣️.json` (draft-07) and
/// `🧬️schema/🟦️.ts` are generated from (`repo run @semio-tech/framework-os-mcp-rs:schema-mirror`).
/// Three families, one catalogue:
///
/// 1. **gateway wire types** — `schemars`-derived from the structs above (`RevisionStamp` …
///    `GatewayError`);
/// 2. **MCP protocol types** — `schemars`-derived from `🧭️protocol/🦀️.rs`. These MIRROR the external
///    Model Context Protocol specification rather than defining it; they are catalogued so the
///    surface this crate speaks is visible, never invisible;
/// 3. **tool schemas** — the hand-authored `inputSchema`/`outputSchema` shapes every facet stamps a
///    `$id` onto (`🔖️ToolSchemas` above), plus the [`hub_inference_approval_request_schema`]
///    mirror of hub's own approval contract.
pub fn schemas() -> Vec<(&'static str, serde_json::Value)> {
    let mut entries = vec![
        ("RevisionStamp", serde_json::to_value(schema_for!(RevisionStamp)).expect("RevisionStamp schema")),
        ("InvocationStatus", serde_json::to_value(schema_for!(InvocationStatus)).expect("InvocationStatus schema")),
        ("InvocationReport", serde_json::to_value(schema_for!(InvocationReport)).expect("InvocationReport schema")),
        ("JobState", serde_json::to_value(schema_for!(JobState)).expect("JobState schema")),
        ("GatewayErrorCode", serde_json::to_value(schema_for!(GatewayErrorCode)).expect("GatewayErrorCode schema")),
        ("PreparedActionReport", serde_json::to_value(schema_for!(PreparedActionReport)).expect("PreparedActionReport schema")),
        ("SearchHit", serde_json::to_value(schema_for!(SearchHit)).expect("SearchHit schema")),
        ("JobStatus", serde_json::to_value(schema_for!(JobStatus)).expect("JobStatus schema")),
        ("ContextSummary", serde_json::to_value(schema_for!(ContextSummary)).expect("ContextSummary schema")),
        ("GatewayError", serde_json::to_value(schema_for!(GatewayError)).expect("GatewayError schema")),
        ("ContentBlock", serde_json::to_value(schema_for!(ContentBlock)).expect("ContentBlock schema")),
        ("Tool", serde_json::to_value(schema_for!(Tool)).expect("Tool schema")),
        ("CallToolResult", serde_json::to_value(schema_for!(CallToolResult)).expect("CallToolResult schema")),
        ("Resource", serde_json::to_value(schema_for!(Resource)).expect("Resource schema")),
        ("ResourceTemplate", serde_json::to_value(schema_for!(ResourceTemplate)).expect("ResourceTemplate schema")),
        ("ResourceContent", serde_json::to_value(schema_for!(ResourceContent)).expect("ResourceContent schema")),
        ("PromptArgument", serde_json::to_value(schema_for!(PromptArgument)).expect("PromptArgument schema")),
        ("Prompt", serde_json::to_value(schema_for!(Prompt)).expect("Prompt schema")),
        ("PromptMessage", serde_json::to_value(schema_for!(PromptMessage)).expect("PromptMessage schema")),
        ("PromptGetResult", serde_json::to_value(schema_for!(PromptGetResult)).expect("PromptGetResult schema")),
        ("NullableRevisionStamp", nullable_revision_stamp_shape()),
        ("CapabilitiesSearchInput", capabilities_search_input_shape()),
        ("CapabilitiesSearchOutput", capabilities_search_output_shape()),
        ("CapabilitiesDescribeInput", capabilities_describe_input_shape()),
        ("CapabilitiesDescribeOutput", capabilities_describe_output_shape()),
        ("ContextResolveInput", context_resolve_input_shape()),
        ("ContextResolveOutput", context_resolve_output_shape()),
        ("ActionPrepareInput", action_prepare_input_shape()),
        ("ActionInvokeInput", action_invoke_input_shape()),
        ("HandleInput", handle_input_shape()),
        ("TransactionBeginInput", transaction_begin_input_shape()),
        ("ArtifactOpenInput", artifact_open_input_shape()),
        ("ArtifactOpenOutput", artifact_open_output_shape()),
        ("ArtifactCreateInput", artifact_create_input_shape()),
        ("ArtifactCreateOutput", artifact_create_output_shape()),
        ("ArtifactValidateInput", artifact_validate_input_shape()),
        ("ArtifactValidateOutput", artifact_validate_output_shape()),
        ("ArtifactSnapshotInput", artifact_snapshot_input_shape()),
        ("ArtifactSnapshotOutput", artifact_snapshot_output_shape()),
        ("ArtifactExportInput", artifact_export_input_shape()),
        ("ArtifactExportOutput", artifact_export_output_shape()),
        ("UntrustedContentV1", untrusted_content_shape()),
        ("UntrustedProvenanceV1", untrusted_provenance_shape()),
        ("UiFocusInput", ui_focus_input_shape()),
        ("UiFocusOutput", ui_focus_output_shape()),
        ("UiRevealInput", ui_reveal_input_shape()),
        ("UiRevealOutput", ui_reveal_output_shape()),
        ("ConversationReplyInput", conversation_reply_input_shape()),
        ("ConversationReplyOutput", conversation_reply_output_shape()),
        ("JobGetInput", job_get_input_shape()),
        ("JobCancelInput", job_cancel_input_shape()),
        ("JobSnapshotOutput", job_snapshot_output_shape()),
        ("CapabilityActionInput", capability_action_input_shape()),
        ("CapabilityGenericInput", capability_generic_input_shape()),
        ("CapabilityGenericOutput", capability_generic_output_shape()),
        ("UiDialogOpenInput", ui_dialog_open_input_shape()),
        ("ArtifactCreateTemplateInput", artifact_create_template_input_shape()),
        ("InferenceListInput", inference_list_input_shape()),
        ("InferenceListOutput", inference_list_output_shape()),
        ("InferenceGetInput", inference_get_input_shape()),
        ("InferenceGetOutput", inference_get_output_shape()),
        ("InferenceSubmitInput", inference_submit_input_shape()),
        ("InferenceJobHandleInput", inference_job_handle_input_shape()),
        ("InferenceApproveInput", inference_approve_input_shape()),
        ("InferenceJobOutput", inference_job_output_shape()),
        ("InferenceRetainedAuthority", inference_original_retained_shape()),
        ("InferenceOriginalWork", inference_original_work_shape()),
        ("InferenceRetainedTurnInput", inference_original_turn_shape()),
        ("InferenceRunInput", inference_run_input_shape()),
        ("InferenceRunOutput", inference_run_output_shape()),
        ("ArtifactInferenceBudgetV1", serde_json::to_value(schema_for!(ArtifactInferenceBudgetV1)).expect("ArtifactInferenceBudgetV1 schema")),
        ("ArtifactInferenceCacheModeV1", serde_json::to_value(schema_for!(ArtifactInferenceCacheModeV1)).expect("ArtifactInferenceCacheModeV1 schema")),
        ("ArtifactInferenceRequestV1", serde_json::to_value(schema_for!(ArtifactInferenceRequestV1)).expect("ArtifactInferenceRequestV1 schema")),
        ("ArtifactInferenceResultV1", serde_json::to_value(schema_for!(ArtifactInferenceResultV1)).expect("ArtifactInferenceResultV1 schema")),
        ("HubInferenceApprovalRequestV1", hub_inference_approval_request_schema()),
    ];
    for (_, schema) in entries.iter_mut() {
        normalize_boolean_subschemas(schema);
    }
    entries
}

/// 🪞️ The whole `os.mcp` scope as ONE draft-07 document — the exact bytes
/// `🧬️schema/🔣️.json` carries, emitted by `semio-os-mcp schemas` and written by
/// `repo run @semio-tech/framework-os-mcp-rs:schema-mirror`.
///
/// Three real conversions turn [`schemas`]'s wire documents into `$defs` entries, none of them a
/// relabelling: the per-entry dialect/`$id` header is dropped (a `$defs` member may declare
/// neither), `schemars`' own `definitions` bucket is hoisted to the document's single `$defs` (every
/// hoisted name must already BE a registry export, or this panics rather than publishing a schema
/// nothing catalogues), and every `#/definitions/X` reference is repointed at `#/$defs/X`.
pub fn schema_mirror_document() -> serde_json::Value {
    let catalog = schemas();
    let export_ids: std::collections::BTreeSet<&str> = catalog.iter().map(|(name, _)| *name).collect();
    let mut defs = serde_json::Map::new();
    for (name, schema) in catalog.iter() {
        let mut entry = schema.clone();
        let hoisted = strip_wire_header(&mut entry);
        for (definition_name, definition) in hoisted {
            assert!(export_ids.contains(definition_name.as_str()), "`{definition_name}` is reachable from `{name}` but is not an os.mcp export — register it in `schemas()`");
            let mut definition = definition;
            strip_wire_header(&mut definition);
            defs.entry(definition_name).or_insert(definition);
        }
        defs.insert((*name).to_string(), entry);
    }
    for entry in defs.values_mut() {
        resolve_schemars_number_formats(entry);
    }
    serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": SCHEMA_MIRROR_ID,
        "title": "Semio OS MCP Gateway",
        "$defs": defs,
    })
}

/// 🖨️ [`schema_mirror_document`] rendered exactly as `🧬️schema/🔣️.json` stores it: two-space
/// indentation, one trailing newline — so `semio-os-mcp schemas > 🔣️.json` is byte-idempotent.
pub fn schema_mirror_json() -> String {
    let mut buffer = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut buffer, serde_json::ser::PrettyFormatter::with_indent(b"  "));
    Serialize::serialize(&schema_mirror_document(), &mut serializer).expect("the mirror serializes");
    let mut rendered = String::from_utf8(buffer).expect("the mirror is utf-8");
    rendered.push('\n');
    rendered
}

/// ✂️ Removes the per-entry dialect/`$id` header and the `schemars` `definitions` bucket, repointing
/// every `#/definitions/X` reference at `#/$defs/X`; returns whatever was hoisted out.
fn strip_wire_header(value: &mut serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    let mut hoisted = serde_json::Map::new();
    if let Some(map) = value.as_object_mut() {
        map.remove("$schema");
        map.remove("$id");
        map.remove("title");
        if let Some(serde_json::Value::Object(definitions)) = map.remove("definitions") {
            hoisted = definitions;
        }
    }
    repoint_definition_refs(value);
    hoisted
}

/// 🔢️ Replaces `schemars`' Rust-typed number `format` annotations with the draft-07 semantics they
/// stand for, because they are the ONE thing in these documents a strict validator cannot read:
/// `new Ajv({ strict: true })` throws `unknown format "double"` and refuses to compile the whole
/// mirror (observed live running `repo ./📜️script.ts schema-mirror`). An unsigned format becomes a
/// real `minimum: 0`; a signed/floating one carries no constraint JSON can express and is dropped.
/// Only the generated mirror is normalized — `schemas()` keeps the annotation for the MCP wire,
/// whose SDK validators accept it.
fn resolve_schemars_number_formats(value: &mut serde_json::Value) {
    const UNSIGNED: &[&str] = &["uint", "uint8", "uint16", "uint32", "uint64", "uint128", "usize"];
    const UNCONSTRAINED: &[&str] = &["int", "int8", "int16", "int32", "int64", "int128", "isize", "float", "double"];
    match value {
        serde_json::Value::Object(map) => {
            if let Some(format) = map.get("format").and_then(serde_json::Value::as_str).map(str::to_string) {
                if UNSIGNED.contains(&format.as_str()) {
                    map.remove("format");
                    map.entry("minimum").or_insert(serde_json::json!(0));
                } else if UNCONSTRAINED.contains(&format.as_str()) {
                    map.remove("format");
                }
            }
            for entry in map.values_mut() {
                resolve_schemars_number_formats(entry);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                resolve_schemars_number_formats(item);
            }
        }
        _ => {}
    }
}

/// 🔗️ `#/definitions/X` → `#/$defs/X`, everywhere in one document.
fn repoint_definition_refs(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(reference)) = map.get_mut("$ref") {
                if let Some(name) = reference.strip_prefix("#/definitions/") {
                    *reference = format!("#/$defs/{name}");
                }
            }
            for entry in map.values_mut() {
                repoint_definition_refs(entry);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                repoint_definition_refs(item);
            }
        }
        _ => {}
    }
}

/// 🔧️ The catalogued document one `Tool.outputSchema` publishes, looked up by its `ExportId` —
/// the ONE `schemars` derivation per type, normalized once by [`normalize_boolean_subschemas`]
/// (`PreparedActionReport.preview`'s free-form `serde_json::Value` field otherwise emits a bare
/// boolean sub-schema, which the MCP SDK's Zod model rejects outright: `$ZodError` at
/// `tools[2].outputSchema.properties.preview`, caught live by
/// `repo run @semio-tech/framework-os-mcp:test -- quick`).
pub fn tool_output_schema(export_id: &str) -> serde_json::Value {
    schemas().into_iter().find(|(name, _)| *name == export_id).map(|(_, schema)| schema).unwrap_or_else(|| panic!("{export_id} is not an os.mcp schema export"))
}

/// 🪞️ The generated draft-07 mirror `🧬️schema/🟦️.ts` and every TypeScript consumer read — its
/// `$defs` key set is asserted equal to [`schemas`] by
/// `🧪️Tests::the_json_mirror_publishes_exactly_the_registry_exports`.
pub const SCHEMA_MIRROR_JSON: &str = include_str!("../🔣️.json");

/// 🆔️ The `$id` of [`SCHEMA_MIRROR_JSON`] — the `os.mcp` scope's one component document.
pub const SCHEMA_MIRROR_ID: &str = "https://json.schemas.assets.semio-tech.com/os/mcp/component.json";
//#endregion 🔖️SchemaCatalog
