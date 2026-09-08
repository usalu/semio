
use super::*;

fn has_bare_boolean(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Bool(_) => true,
        serde_json::Value::Object(map) => map.iter().any(|(key, entry)| !SCHEMA_BOOLEAN_KEYWORDS.contains(&key.as_str()) && has_bare_boolean(entry)),
        serde_json::Value::Array(items) => items.iter().any(has_bare_boolean),
        _ => false,
    }
}

#[test]
fn no_published_schema_contains_a_bare_boolean_subschema() {
    for (name, schema) in schemas() {
        assert!(!has_bare_boolean(&schema), "{name} still publishes a boolean sub-schema — the MCP SDK's Zod model rejects the whole tools/list response when it sees one");
    }
}

#[test]
fn normalize_rewrites_booleans_but_leaves_boolean_keywords_alone() {
    let mut value = serde_json::json!({ "properties": { "free": true, "never": false }, "additionalProperties": false, "uniqueItems": true });
    normalize_boolean_subschemas(&mut value);
    assert_eq!(value["properties"]["free"], serde_json::json!({}));
    assert_eq!(value["properties"]["never"], serde_json::json!({ "not": {} }));
    assert_eq!(value["additionalProperties"], serde_json::json!(false));
    assert_eq!(value["uniqueItems"], serde_json::json!(true));
}

fn revision_stamp_example() -> serde_json::Value {
    serde_json::to_value(RevisionStamp { artifact_id: "cad-1".into(), head_edit_id: "edit-7".into(), cursor: "c0".into() }).unwrap()
}

fn invocation_report_example() -> serde_json::Value {
    serde_json::to_value(InvocationReport {
        invocation_id: "inv-1".into(),
        capability_id: "cad.viewport.translateSelection".into(),
        status: InvocationStatus::Succeeded,
        affected_resources: vec!["semio://artifact/cad-1".into()],
        revision_before: Some(RevisionStamp { artifact_id: "cad-1".into(), head_edit_id: "edit-6".into(), cursor: "c0".into() }),
        revision_after: Some(RevisionStamp { artifact_id: "cad-1".into(), head_edit_id: "edit-7".into(), cursor: "c1".into() }),
        diff_uri: None,
        warnings: Vec::new(),
        undo_token: Some("undo_abc".into()),
        postconditions: vec!["selection.moved".into()],
        replayed: false,
    })
    .unwrap()
}

fn prepared_action_report_example() -> serde_json::Value {
    serde_json::to_value(PreparedActionReport {
        prepared_handle: "prep_abc".into(),
        capability_id: "cad.viewport.translateSelection".into(),
        expected_revision: Some(RevisionStamp { artifact_id: "cad-1".into(), head_edit_id: "edit-6".into(), cursor: "c0".into() }),
        preview: serde_json::json!({ "dx": 1.0, "dy": 0.0, "dz": 0.0 }),
        expires_at_ms: 1_000,
    })
    .unwrap()
}

fn search_hit_example() -> serde_json::Value {
    serde_json::to_value(SearchHit {
        capability_id: "cad.viewport.translateSelection".into(),
        title: "Translate selection".into(),
        description: "Moves the current selection by (dx, dy, dz)".into(),
        score: 0.92,
        plugin_id: "cad".into(),
        app_id: "viewport".into(),
    })
    .unwrap()
}

fn job_status_example() -> serde_json::Value {
    serde_json::to_value(JobStatus { job_id: "job_1".into(), state: JobState::Running, progress: Some(0.5), result: None, error: None }).unwrap()
}

fn context_summary_example() -> serde_json::Value {
    serde_json::to_value(ContextSummary {
        session_id: "sess_1".into(),
        principal: "agent:local".into(),
        scopes: vec!["cad.viewport.translateSelection".into()],
        active_artifact_id: Some("cad-1".into()),
        catalog_hash: "blake3:abc".into(),
        locale: "en".into(),
    })
    .unwrap()
}

fn gateway_error_example() -> serde_json::Value {
    serde_json::to_value(GatewayError::new(GatewayErrorCode::NotFound, "no such capability")).unwrap()
}

#[test]
fn every_schema_compiles_and_validates_its_own_example() {
    let examples: Vec<(&str, serde_json::Value)> = vec![
        ("RevisionStamp", revision_stamp_example()),
        ("InvocationReport", invocation_report_example()),
        ("PreparedActionReport", prepared_action_report_example()),
        ("SearchHit", search_hit_example()),
        ("JobStatus", job_status_example()),
        ("ContextSummary", context_summary_example()),
        ("GatewayError", gateway_error_example()),
    ];
    let catalog = schemas();
    for (name, schema) in &catalog {
        let owned = compile_validator(schema).unwrap_or_else(|error| panic!("{name}: owned schema did not compile: {error}"));
        let Some((_, example)) = examples.iter().find(|(example_name, _)| example_name == name) else { continue };
        validate(&owned, example).unwrap_or_else(|error| panic!("{name}: own example failed validation: {error}"));
        assert!(validate(&owned, &serde_json::Value::Null).is_err(), "{name}: null must fail the object-root schema");
    }
    for (name, _) in &examples {
        assert!(catalog.iter().any(|(export_id, _)| export_id == name), "{name} lost its registry export");
    }
}

#[test]
fn every_export_id_is_unique_and_pascal_case() {
    let names: Vec<&str> = schemas().into_iter().map(|(name, _)| name).collect();
    let mut sorted = names.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), names.len(), "an ExportId is registered twice");
    for name in &names {
        let first = name.chars().next().expect("non-empty ExportId");
        assert!(first.is_ascii_uppercase(), "{name} is not PascalCase");
        assert!(name.chars().all(|character| character.is_ascii_alphanumeric()), "{name} is not PascalCase");
    }
}

#[test]
fn the_json_mirror_publishes_exactly_the_registry_exports() {
    let mirror: serde_json::Value = serde_json::from_str(SCHEMA_MIRROR_JSON).expect("🔣️.json parses");
    assert_eq!(mirror["$schema"], "http://json-schema.org/draft-07/schema#", "the module mirror must be draft-07 at its root");
    assert_eq!(mirror["$id"], SCHEMA_MIRROR_ID);
    let mut published: Vec<&str> = mirror["$defs"].as_object().expect("$defs object").keys().map(String::as_str).collect();
    published.sort_unstable();
    let mut registered: Vec<&str> = schemas().into_iter().map(|(name, _)| name).collect();
    registered.sort_unstable();
    assert_eq!(published, registered, "🔣️.json is stale — regenerate it with `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`");
    assert_eq!(mirror, schema_mirror_document(), "🔣️.json's CONTENT drifted from the registry — regenerate it with `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`");
    for value in mirror["$defs"].as_object().expect("$defs object").values() {
        assert!(value.get("$schema").is_none(), "a $defs entry must not redeclare the dialect");
        assert!(value.get("$id").is_none(), "a $defs entry must not carry a wire $id");
    }
}

/// 🪞️ os is a CLIENT of hub for the GIS Map approval intent — this proves the `os.mcp` mirror is
/// byte-identical in value space to hub's own authority, so a hub-side change breaks here loudly
/// instead of drifting. It only READS hub's file.
/// 📌️ The scope registration must publish exactly the registry's export ids, so
/// `resolve_schema_export("os.mcp", …)` can answer for every `$defs` key the mirror publishes.
#[test]
fn the_scope_export_declaration_matches_the_registry() {
    let mut declared = scope_export_ids();
    declared.sort_unstable();
    let mut registered: Vec<&str> = schemas().into_iter().map(|(name, _)| name).collect();
    registered.sort_unstable();
    assert_eq!(declared, registered, "the ScopeSchemaExports declaration drifted from schemas()");
    register_scope_exports();
    for id in scope_export_ids() {
        assert!(semio_framework_schema::resolve_schema_export("os.mcp", id, semio_framework_schema::SchemaFormat::JsonSchema).is_ok(), "{id} does not resolve");
    }
}

#[test]
fn os_mirror_of_the_hub_approval_request_is_structurally_identical() {
    let hub: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🌎️hub/💡️inference/🧬️schema/🔣️.json")).expect("hub module schema parses");
    let authority = hub["$defs"].get("InferenceApprovalRequestV1").expect("hub publishes InferenceApprovalRequestV1");
    let authority = inline_local_refs(authority, &hub);
    let mirror = gis_map_inference_approval_request_schema();
    for key in ["type", "additionalProperties", "required", "properties"] {
        assert_eq!(&mirror[key], &authority[key], "the os.mcp approval mirror drifted from hub on `{key}`");
    }
    let approval = crate::inference::GisMapInferenceApprovalRequestV1::new("00112233445566778899aabbccddeeff", &"ab".repeat(32));
    let owned = compile_validator(&mirror).expect("the mirror compiles");
    validate(&owned, &serde_json::to_value(&approval).expect("approval serializes")).expect("the Rust type's own encoding satisfies hub's contract");
}

/// 🔗️ Replaces every `{"$ref": "#/$defs/X"}` with the document's own `X`, so a mirror that inlines
/// a pattern can be compared with an authority that names it.
fn inline_local_refs(value: &serde_json::Value, document: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => match map.get("$ref").and_then(serde_json::Value::as_str).and_then(|reference| reference.strip_prefix("#/$defs/")) {
            Some(name) => inline_local_refs(&document["$defs"][name], document),
            None => serde_json::Value::Object(map.iter().map(|(key, entry)| (key.clone(), inline_local_refs(entry, document))).collect()),
        },
        serde_json::Value::Array(items) => serde_json::Value::Array(items.iter().map(|item| inline_local_refs(item, document)).collect()),
        other => other.clone(),
    }
}
