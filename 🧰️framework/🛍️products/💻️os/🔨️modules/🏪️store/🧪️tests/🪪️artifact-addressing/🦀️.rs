//! 🧪️ Shared child addressing preserves wire identity and excludes local materialization.
use super::*;

#[test]
fn shared_artifact_addressing_matches_neutral_identities_and_rejects_foreign_fields() {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _,DialectCoordinateText as _};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️artifact-addressing/🔣️.json")).unwrap();
    for row in fixture["valid"].as_array().unwrap() {
        let value = DslValue::from(row["child"].clone());
        let mut child = ArtifactChild::<()>::from_value(value).unwrap();
        child.set_local_owner(Arc::new(String::from("local-only")));
        let encoded = semio_framework_pack_json::to_json_string(&child);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap(), row["child"]);
        assert_eq!(child.target.dialect.to_coordinate(), row["coordinate"].as_str().unwrap());
        assert_eq!(child.target.to_uri(), row["uri"].as_str().unwrap());
        assert_eq!(semio_framework_artifact_reference::ArtifactRef::parse_uri(row["uri"].as_str().unwrap()).unwrap(), child.target);
        let decoded = ArtifactChild::<()>::from_value(child.to_value()).unwrap();
        assert!(decoded.local_owner::<String>().is_none());
        assert_eq!(decoded, child);
    }
    for value in fixture["invalidChildren"].as_array().unwrap() {
        assert!(ArtifactChild::<()>::from_value(DslValue::from(value.clone())).is_err(), "accepted foreign child shape {value}");
    }
    for value in fixture["invalidCoordinates"].as_array().unwrap() {
        assert!(semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(value.as_str().unwrap()).is_err());
    }
    for value in fixture["invalidUris"].as_array().unwrap() {
        assert!(semio_framework_artifact_reference::ArtifactRef::parse_uri(value.as_str().unwrap()).is_err());
    }
}

/// 🔗️ Link identity admits exactly its target, pin variant and role across native JSON and independent vectors.
#[test]
fn shared_artifact_addressing_links_match_neutral_pin_variants_and_reject_foreign_fields() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️artifact-addressing/🔣️.json")).unwrap();
    for row in fixture["validLinks"].as_array().unwrap() {
        let link = ArtifactLink::from_value(DslValue::from(row.clone())).unwrap();
        let encoded = semio_framework_pack_json::to_json_string(&link);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap(), *row);
        assert_eq!(ArtifactLink::from_value(link.to_value()).unwrap(), link);
    }
    for row in fixture["invalidLinks"].as_array().unwrap() {
        assert!(ArtifactLink::from_value(DslValue::from(row.clone())).is_err(), "accepted foreign link shape {row}");
    }
}

/// 🆔️ `content_id` is `<prefix>-` + the first 16 hex digits of SHA-256: equal to Python `hashlib`'s answer for every
/// committed vector (`contentIds`), admitted by the child schema's `ContentId`, and independent of the process that
/// computes it.
#[test]
fn content_id_is_the_specified_sha256_prefix() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️artifact-addressing/🔣️.json")).unwrap();
    for row in fixture["contentIds"].as_array().unwrap() {
        let (prefix, text) = (row["prefix"].as_str().unwrap(), row["text"].as_str().unwrap());
        let id = content_id(prefix, text.as_bytes());
        assert_eq!(id, row["id"].as_str().unwrap());
        assert_eq!(id, content_id(prefix, text.as_bytes()));
    }
}
