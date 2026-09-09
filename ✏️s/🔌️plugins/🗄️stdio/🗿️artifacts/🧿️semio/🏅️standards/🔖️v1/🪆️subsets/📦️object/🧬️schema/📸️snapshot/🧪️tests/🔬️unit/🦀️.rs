use super::*;

#[semio_framework_async_macros::async_test]
async fn minimal_schema_valid_json_defaults_optional_children() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../../🔣️.json")).expect("committed JSON Schema");
    assert_eq!(schema["required"], serde_json::json!(["schema", "transform"]));
    let minimal = serde_json::json!({
        "schema": STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA,
        "transform": {
            "translation": { "x": 0.0, "y": 0.0, "z": 0.0 },
            "rotation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0 },
            "scale": { "x": 1.0, "y": 1.0, "z": 1.0 }
        }
    });
    let minimal_text = minimal.to_string();
    let snapshot = decode_semio_object_snapshot_json(&minimal_text).expect("schema-valid minimal Object snapshot");
    assert!(snapshot.brep.is_none() && snapshot.mesh.is_none() && snapshot.properties.is_none());
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&snapshot)).expect("snapshot reencodes"), minimal);
    let artifact: crate::standards::v1::subsets::object::schema::SemioObjectArtifact = dsl::json::from_json_str(&minimal_text).expect("schema-valid minimal Object artifact");
    assert!(artifact.brep.is_none() && artifact.mesh.is_none() && artifact.properties.is_none());
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&artifact)).expect("artifact reencodes"), minimal);
    let mut malformed = minimal;
    malformed.as_object_mut().expect("minimal Object is an object").insert("brep".into(), serde_json::Value::Null);
    assert!(decode_semio_object_snapshot_json(&malformed.to_string()).is_err());
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioObjectSnapshot::default();
    let bytes = <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioObjectSnapshot::default();
    let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law on a fully-populated snapshot (all 3 child handles present, non-
/// identity transform), not just the default.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = demo_object_snapshot();
    let bytes = <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}

/// 🧪️ A parent snapshot NEVER embeds child content — only the handle's two strings. Proven by
/// asserting the printed DSL contains the child's `child_id`/target URI but never a byte
/// sequence that could only come from parsing the CHILD's own snapshot type.
#[semio_framework_async_macros::async_test]
async fn parent_snapshot_stores_only_child_handles_never_content() {
    let snap = demo_object_snapshot();
    let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    assert!(text.contains(&enc_str("crate-brep")), "hex-encoded child_id must be present");
    assert!(!text.to_lowercase().contains("vertices") && !text.to_lowercase().contains("faces"), "must never embed brep/mesh field names — only the handle");
}
