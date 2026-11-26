use super::*;

#[test]
fn catalog_projection_budget_matches_serde_and_refuses_before_overdraw() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️catalog/🧪️tests/🧫️fixtures/📇️native-catalog-surface/🧪️budget.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let value = row["text"].as_str().unwrap();
        let mut budget = NativeCatalogProjectionBudget::new();
        budget.string(&[value], 16384).unwrap();
        let expected = row["encodedBytes"].as_u64().unwrap() as usize;
        assert_eq!(serde_json::to_vec(value).unwrap().len(), expected, "{}", row["id"]);
        assert_eq!(budget.projection, expected, "{}", row["id"]);
    }
    for row in fixture["aggregateCases"].as_array().unwrap() {
        let mut budget = NativeCatalogProjectionBudget::new();
        let projection = row["projectionCharges"].as_array().unwrap();
        let descriptors = row["descriptorCharges"].as_array().unwrap();
        let independent =
            projection.iter().map(|v| v.as_u64().unwrap()).sum::<u64>() <= fixture["projectionLimitBytes"].as_u64().unwrap() && descriptors.iter().map(|v| v.as_u64().unwrap()).sum::<u64>() <= fixture["descriptorLimitBytes"].as_u64().unwrap();
        let result = projection.iter().try_for_each(|value| budget.charge(value.as_u64().unwrap() as usize)).and_then(|()| descriptors.iter().try_for_each(|value| budget.descriptor(value.as_u64().unwrap() as usize)));
        assert_eq!(independent, row["accepted"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(result.is_ok(), independent, "{}", row["id"]);
        assert!(budget.projection <= 2 * 1024 * 1024 && budget.descriptors <= 16 * 1024 * 1024);
    }
    let mut budget = NativeCatalogProjectionBudget::new();
    assert!(budget.charge(usize::MAX).is_err());
    assert_eq!(budget.projection, 0);
    assert!(budget.descriptor(usize::MAX).is_err());
    assert_eq!(budget.descriptors, 0);
    assert!(budget.string(&[""], 16).is_err());
    assert!(budget.string(&["éé"], 3).is_err());
}

#[test]
fn catalog_projection_preflight_matches_actual_serde_payload_bytes() {
    let schema = include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📜️artifact-definition.json");
    let authored = catalog_definition_from_schema(schema).unwrap();
    let definitions = [ProjectionDefinition { definition: &authored.definition, authored_executables: Some(&authored.executable_ids) }];
    let identity = CatalogIdentity { plugin_id: "stdio", package_id: component_package_id().unwrap() };
    let expected = preflight_native_catalog_projection(&identity, &definitions, &[]).unwrap();
    let topic = catalog_contribution(NATIVE_ARTIFACT_CATALOG_TOPIC, native_artifact_catalog(&identity, &definitions, Vec::new()).unwrap()).unwrap();
    let json = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&topic.payload));
    let payload: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(serde_json::to_vec(&payload).unwrap().len(), expected.projection);
    assert!(expected.descriptors > 0);
}

#[test]
fn hosted_catalog_topic_is_published_per_host_with_the_host_identity() {
    let host = hosted_artifact_catalog_contribution_for("stdio-demo", "semio:stdio-demo", &[]);
    assert!(host.is_err());
    assert_eq!(hosted_artifact_catalog_topic("stdio-demo"), "stdio.artifact-catalog.stdio-demo");
}
