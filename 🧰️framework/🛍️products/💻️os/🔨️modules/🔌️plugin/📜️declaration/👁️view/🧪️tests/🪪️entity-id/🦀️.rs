use super::*;

#[test]
fn operation_entity_ids_obey_the_portable_authoring_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪪️entity-id/🔣️.json")).unwrap();
    let mut ids = std::collections::HashSet::new();
    for row in fixture["rows"].as_array().unwrap() {
        let context = AppOperationContext {
            app_instance_id: 1,
            parent_document_id: row["documentId"].as_str().unwrap().into(),
            operation_id: 1,
            generation: 1,
            canonical_base_revision: [0; 32],
            retained: RetainedCloneGrant::default(),
            authoring_seed: row["authoringSeed"].as_str().unwrap().into(),
        };
        let id = context.entity_id(row["prefix"].as_str().unwrap(), row["localKey"].as_str().unwrap()).unwrap();
        assert_eq!(id, row["expected"].as_str().unwrap(), "{}", row["name"]);
        assert!(ids.insert(id.clone()));
        assert_eq!(context.entity_id(row["prefix"].as_str().unwrap(), row["localKey"].as_str().unwrap()).unwrap(), id);
        assert!(AppOperationContext { authoring_seed: String::new(), ..context }.entity_id("n", "node").is_err());
    }
}
