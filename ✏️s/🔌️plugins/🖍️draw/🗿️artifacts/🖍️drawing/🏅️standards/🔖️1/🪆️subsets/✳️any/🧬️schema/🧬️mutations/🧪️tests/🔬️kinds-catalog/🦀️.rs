use super::*;
use protocol::SemanticMutation;

/// 🏷️ `KINDS` must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed
/// catalog — the framework reads the manifest and never parses Rust, so this test is the only
/// thing that keeps the two in step. A plain `#[test]`: it suspends on nothing.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = DrawingMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared DrawingMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifests: Vec<serde_json::Value> =
        [include_str!("../../../../../🎨️style/🔮️oracles/🔣️.json"), include_str!("../../../../../🏷️metadata/🔮️oracles/🔣️.json"), include_str!("../../../../../🔀️transform/🔮️oracles/🔣️.json"), include_str!("../../../../../🧱️structure/🔮️oracles/🔣️.json")]
            .into_iter()
            .map(|text| serde_json::from_str(text).expect("language-neutral subset oracle"))
            .collect();
    let entries: Vec<_> = manifests.iter().flat_map(|manifest| manifest["mutationManifests"].as_array().unwrap()).flat_map(|manifest| manifest["mutations"].as_array().unwrap()).collect();
    let leaf_descriptors = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::DESCRIPTORS;
    assert_eq!(entries.len(), leaf_descriptors.len());
    for (index, descriptor) in leaf_descriptors.iter().enumerate() {
        let entry = entries.iter().find(|entry| entry["id"] == descriptor.semantic_kind).expect("every leaf has a declared catalog entry");
        assert_eq!(entry["productionDispatch"]["variant"], descriptor.aggregate_variant);
        assert_eq!(entry["payloadSchema"], descriptor.payload_schema, "the catalog must publish the leaf descriptor's exact owner-relative payload authority");
        assert_eq!(descriptor.payload_schema, "🧬️schema/🔣️.json", "Drawing mutation payloads use the canonical owner-local JSON Schema surface");
        assert_eq!(descriptor.text_opcode, Some(KINDS[index]));
        assert_eq!(descriptor.binary_tag, Some(index as u32));
    }
}

#[test]
fn canonical_tagged_mutations_match_the_owned_schema_validator() {
    fn directories(path: &std::path::Path) -> Vec<std::path::PathBuf> {
        std::fs::read_dir(path).unwrap().map(|entry| entry.unwrap().path()).filter(|path| path.is_dir()).collect()
    }
    let subsets = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets");
    let mut documents = vec![include_str!("../../../🔣️.json").to_owned()];
    let mut fixtures = Vec::new();
    for subset in ["🎨️style", "🏷️metadata", "🔀️transform", "🧱️structure"] {
        let owner = subsets.join(subset);
        for mutation in directories(&owner.join("🧬️schema/🧬️mutations")) {
            documents.push(std::fs::read_to_string(mutation.join("🧬️schema/🔣️.json")).unwrap());
        }
        for mutation in directories(&owner.join("🧫️fixtures/🧬️mutations")) {
            for scenario in directories(&mutation) {
                fixtures.push(std::fs::read_to_string(scenario.join("🦠️mutation/🔣️.json")).unwrap());
            }
        }
    }
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(include_str!("../../🔣️.json"), &documents.iter().map(String::as_str).collect::<Vec<_>>()).unwrap();
    let mut kinds = std::collections::BTreeSet::new();
    for fixture in &fixtures {
        let mut value: serde_json::Value = serde_json::from_str(fixture).unwrap();
        let mutation: DrawingMutation = serde_json::from_str(fixture).unwrap();
        validator.validate_json(&serde_json::to_string(&mutation).unwrap()).unwrap();
        kinds.insert(value["mutation"].as_str().unwrap().to_owned());
        validator.validate_json(fixture).unwrap();
        value["unexpected"] = serde_json::Value::Bool(true);
        assert!(validator.validate_json(&value.to_string()).is_err());
        value.as_object_mut().unwrap().remove("unexpected");
        value["mutation"] = serde_json::Value::String("unknown".into());
        assert!(validator.validate_json(&value.to_string()).is_err());
    }
    assert_eq!(kinds.len(), DrawingMutation::kinds().len());
    eprintln!("[DEBUG] {} canonical tagged Drawing mutations match the owned schema validator and reject unknown tags and fields", kinds.len());
}
