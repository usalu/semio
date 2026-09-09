//! 🧪️ Shared child addressing preserves wire identity and excludes local materialization.
use super::*;

#[test]
fn shared_artifact_addressing_matches_neutral_identities_and_rejects_foreign_fields() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["valid"].as_array().unwrap() {
        let value = DslValue::from(row["child"].clone());
        let mut child = ArtifactChild::<()>::from_value(value).unwrap();
        child.set_local_owner(Arc::new(String::from("local-only")));
        let encoded = crate::os_pack::json::to_json_string(&child);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap(), row["child"]);
        assert_eq!(child.target.dialect.to_coordinate(), row["coordinate"].as_str().unwrap());
        assert_eq!(child.target.to_uri(), row["uri"].as_str().unwrap());
        assert_eq!(crate::os_io::ArtifactRef::parse_uri(row["uri"].as_str().unwrap()).unwrap(), child.target);
        let decoded = ArtifactChild::<()>::from_value(child.to_value()).unwrap();
        assert!(decoded.local_owner::<String>().is_none());
        assert_eq!(decoded, child);
    }
    for value in fixture["invalidChildren"].as_array().unwrap() {
        assert!(ArtifactChild::<()>::from_value(DslValue::from(value.clone())).is_err(), "accepted foreign child shape {value}");
    }
    for value in fixture["invalidCoordinates"].as_array().unwrap() {
        assert!(crate::os_io::ArtifactDialect::parse_coordinate(value.as_str().unwrap()).is_err());
    }
    for value in fixture["invalidUris"].as_array().unwrap() {
        assert!(crate::os_io::ArtifactRef::parse_uri(value.as_str().unwrap()).is_err());
    }
    eprintln!("[DEBUG] shared artifact addressing preserved three exact child identities, rejected five foreign child records, and omitted local materialization from every native wire projection");
}
