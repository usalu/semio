use super::*;

#[semio_framework_async_macros::async_test]
async fn minimal_schema_valid_json_defaults_collection_slots() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../../🔣️.json")).expect("committed JSON Schema");
    assert_eq!(schema["required"], serde_json::json!(["schema"]));
    let minimal = serde_json::json!({ "schema": STDIO_SEMIOKIT_DOCUMENT_SCHEMA });
    let minimal_text = minimal.to_string();
    let snapshot = decode_kit_snapshot_json(&minimal_text).expect("schema-valid minimal Kit snapshot");
    assert!(snapshot.types.is_empty());
    assert!(snapshot.designs.is_empty());
    assert!(snapshot.objects.is_empty());
    assert!(snapshot.models.is_empty());
    assert!(snapshot.properties.is_none());
    assert!(snapshot.representations.is_empty());
    let encoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&snapshot)).expect("snapshot reencodes");
    assert_eq!(encoded["schema"], minimal["schema"]);
    assert!(encoded.get("properties").is_none());
    let artifact: crate::standards::v1::subsets::kit::schema::SemioKitArtifact = dsl::json::from_json_str(&minimal_text).expect("schema-valid minimal Kit artifact");
    assert!(artifact.types.is_empty() && artifact.designs.is_empty() && artifact.objects.is_empty() && artifact.models.is_empty());
    assert!(artifact.properties.is_none() && artifact.representations.is_empty());
    let encoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&artifact)).expect("artifact reencodes");
    assert!(encoded.get("properties").is_none());
    assert!(decode_kit_snapshot_json(r#"{"schema":"stdio.semio.kit","properties":null}"#).is_err());
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioKitSnapshot::default();
    let bytes = <SemioKitSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioKitSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioKitSnapshot::default();
    let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = demo_kit_snapshot();
    let bytes = <SemioKitSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioKitSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}

/// 🧪️ A parent snapshot NEVER embeds owned-child content — only handles. Links are references
/// by design (never owned), so this proves both composition primitives stay handle-only.
#[semio_framework_async_macros::async_test]
async fn parent_snapshot_stores_only_handles_never_child_content() {
    let snap = demo_kit_snapshot();
    let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    assert!(text.contains(&enc_str("obj-01")));
    assert!(!text.to_lowercase().contains("primitives") && !text.to_lowercase().contains("elements"), "must never embed object/model field names — only the handle");
}
