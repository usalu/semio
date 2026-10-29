use super::*;

fn payload_json(contribution: &semio_framework::TopicContribution) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&contribution.payload))
}

#[test]
fn definition_roster_lists_the_full_rosters_identities_in_authored_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️catalog/🧪️tests/🧫️fixtures/📜️definition-roster/🔣️.json")).unwrap();
    let expected: Vec<&str> = fixture["identities"].as_array().unwrap().iter().map(|identity| identity.as_str().unwrap()).collect();
    let full = selected_registry().unwrap();
    let definition = semio_hub_stdio_catalog_definition::selected_registry().unwrap();
    assert_eq!(full.contributions().iter().map(|item| item.identity).collect::<Vec<_>>(), expected);
    assert_eq!(definition.contributions().iter().map(|item| item.identity).collect::<Vec<_>>(), expected);
}

#[test]
fn definition_only_artifacts_publish_no_native_codec_and_keep_their_definition() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📇️catalog/🧪️tests/🧫️fixtures/📜️definition-roster/🔣️.json")).unwrap();
    let only: BTreeSet<&str> = fixture["definitionOnly"].as_array().unwrap().iter().map(|identity| identity.as_str().unwrap()).collect();
    let full = selected_registry().unwrap().artifact_assemblies().unwrap();
    let definition = semio_hub_stdio_catalog_definition::selected_registry().unwrap().artifact_assemblies().unwrap();
    assert_eq!(full.len(), definition.len());
    for (runtime, light) in full.iter().zip(&definition) {
        let identity = runtime.definition().identity().as_str().strip_prefix("s.stdio.").unwrap();
        assert_eq!(runtime.definition().identity().as_str(), light.definition().identity().as_str());
        assert_eq!(matches!(light, ArtifactAssembly::Definition(_)), only.contains(identity), "{identity}");
        if only.contains(identity) {
            assert!(matches!(runtime, ArtifactAssembly::Runtime(_)), "{identity}");
            assert!(selected_contributions().iter().find(|item| item.identity == identity).map(|item| (item.native_codecs)().is_empty()).unwrap(), "{identity}");
        }
    }
}

#[test]
fn definition_roster_commits_to_the_byte_identical_catalog_payload() {
    let full = native_artifact_catalog_contribution().unwrap();
    let definition = semio_hub_stdio_catalog_definition::native_artifact_catalog_contribution().unwrap();
    assert_eq!(full.topic, definition.topic);
    assert!(catalog_value_matches(&full.payload, &definition.payload));
    let (full_text, definition_text) = (payload_json(&full), payload_json(&definition));
    assert_eq!(full_text, definition_text);
    let oracle: (serde_json::Value, serde_json::Value) = (serde_json::from_str(&full_text).unwrap(), serde_json::from_str(&definition_text).unwrap());
    assert_eq!(oracle.0, oracle.1);
    assert_eq!(semio_hub_stdio_catalog_definition::native_artifact_catalog_dependency().unwrap(), native_artifact_catalog_dependency().unwrap());
}
