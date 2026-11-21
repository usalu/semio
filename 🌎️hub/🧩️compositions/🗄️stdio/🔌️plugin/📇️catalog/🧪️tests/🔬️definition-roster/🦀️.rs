use super::*;

fn payload_json(contribution: &semio_framework::TopicContribution) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&contribution.payload))
}

#[test]
fn every_stdio_artifact_executable_mapping_matches_its_schema_registrations() {
    let contributions = selected_contributions();
    let divergent = contributions.iter().filter_map(|contribution| (contribution.definition)().err().map(|error| format!("{}: {}", contribution.identity, error.message))).collect::<Vec<_>>();
    assert!(divergent.is_empty(), "{} of {} artifacts diverge:\n{}", divergent.len(), contributions.len(), divergent.join("\n"));
}

#[test]
fn definition_roster_commits_to_the_byte_identical_catalog_payload() {
    let full = native_artifact_catalog_contribution().unwrap();
    let definition = semio_hub_stdio_catalog_definition::native_artifact_catalog_contribution().unwrap();
    validate_native_artifact_catalog_contributions(std::slice::from_ref(&definition)).unwrap();
    assert_eq!(full.topic, definition.topic);
    assert!(catalog_value_matches(&full.payload, &definition.payload));
    let (full_text, definition_text) = (payload_json(&full), payload_json(&definition));
    assert_eq!(full_text, definition_text);
    let oracle: (serde_json::Value, serde_json::Value) = (serde_json::from_str(&full_text).unwrap(), serde_json::from_str(&definition_text).unwrap());
    assert_eq!(oracle.0, oracle.1);
    assert_eq!(semio_hub_stdio_catalog_definition::native_artifact_catalog_dependency().unwrap(), native_artifact_catalog_dependency().unwrap());
}
