use super::*;
use crate::registry::ContributionRegistry;

const ALPHA: &str = include_str!("../../🧫️fixtures/📇️contributions/alpha.json");
const BRAVO: &str = include_str!("../../🧫️fixtures/📇️contributions/bravo.json");
const CHARLIE: &str = include_str!("../../🧫️fixtures/📇️contributions/charlie.json");

fn contribution(identity: &'static str) -> ArtifactContribution {
    fn definition(schema: &'static str) -> Result<ArtifactDefinition, PluginAssemblyError> {
        crate::definition_from_schema(schema)
    }
    fn assembly(schema: &'static str) -> Result<ArtifactAssembly, PluginAssemblyError> {
        definition(schema).map(ArtifactAssembly::Definition)
    }
    let (schema, definition, assembly, formats) = match identity {
        "alpha" => (ALPHA, (|| definition(ALPHA)) as fn() -> _, (|| assembly(ALPHA)) as fn() -> _, (|| crate::format_descriptors(ALPHA)) as fn() -> _),
        "bravo" => (BRAVO, (|| definition(BRAVO)) as fn() -> _, (|| assembly(BRAVO)) as fn() -> _, (|| crate::format_descriptors(BRAVO)) as fn() -> _),
        "charlie" => (CHARLIE, (|| definition(CHARLIE)) as fn() -> _, (|| assembly(CHARLIE)) as fn() -> _, (|| crate::format_descriptors(CHARLIE)) as fn() -> _),
        _ => panic!("unknown neutral contribution"),
    };
    ArtifactContribution { definition_constraint: None, identity, schema, definition, assembly, formats, native_codecs: Vec::new }
}

#[test]
fn authored_removal_vectors_match_independent_serde_dependency_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📇️contributions/🔣️.json")).unwrap();
    for row in vectors["cases"].as_array().unwrap() {
        let selected: Vec<_> = row["selected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| {
                contribution(match value.as_str().unwrap() {
                    "alpha" => "alpha",
                    "bravo" => "bravo",
                    "charlie" => "charlie",
                    _ => unreachable!(),
                })
            })
            .collect();
        let mut registry = ContributionRegistry::new(selected.clone()).unwrap();
        let mut oracle = selected;
        let mut accepted = true;
        for value in row["remove"].as_array().unwrap() {
            let identity = value.as_str().unwrap();
            let candidate: Vec<_> = oracle.iter().copied().filter(|item| item.identity != identity).collect();
            let present: BTreeSet<_> = candidate.iter().map(|item| format!("s.stdio.{}", item.identity)).collect();
            let valid = candidate.iter().all(|item| serde_json::from_str::<serde_json::Value>(item.schema).unwrap()["dependencies"].as_array().unwrap().iter().all(|dependency| present.contains(dependency.as_str().unwrap())));
            assert_eq!(registry.remove(identity).is_ok(), valid, "{}", row["id"]);
            accepted &= valid;
            if valid {
                oracle = candidate;
            }
        }
        assert_eq!(accepted, row["accepted"].as_bool().unwrap());
        let actual: Vec<_> = registry.contributions().iter().map(|item| item.identity).collect();
        assert_eq!(serde_json::to_value(&actual).unwrap(), row["remaining"]);
        assert_eq!(actual, oracle.iter().map(|item| item.identity).collect::<Vec<_>>());
        assert_eq!(registry.artifact_assemblies().unwrap().len(), actual.len());
        assert!(registry.native_codec_factory_receipts("neutral", "semio:neutral", "1").unwrap().is_empty());
    }
}

#[test]
fn contribution_registration_and_outputs_fail_closed() {
    let mut registry = ContributionRegistry::new(vec![contribution("alpha")]).unwrap();
    assert!(registry.register(contribution("alpha")).is_err());
    assert_eq!(registry.contributions().len(), 1);
    registry.register(contribution("bravo")).unwrap();
    assert_eq!(registry.format_descriptors().unwrap().len(), 2);
    assert!(registry.format_descriptors_for("missing").is_err());
    let mut foreign = contribution("alpha");
    foreign.definition = contribution("bravo").definition;
    foreign.assembly = contribution("bravo").assembly;
    foreign.formats = contribution("bravo").formats;
    let registry = ContributionRegistry::new(vec![foreign]).unwrap();
    assert!(registry.artifact_definitions().is_err());
    assert!(registry.artifact_assemblies().is_err());
    assert!(registry.format_descriptors().is_err());
}

#[test]
fn portable_registration_and_receipt_vectors_preserve_owned_claims() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📇️contributions/🔣️.json")).unwrap();
    for row in vectors["registrationCases"].as_array().unwrap() {
        let mut second = contribution(if row["second"] == "alpha" { "alpha" } else { "bravo" });
        second.schema = match row["second"].as_str().unwrap() {
            "bravo-directory" => include_str!("../../🧫️fixtures/📇️contributions/bravo-directory.json"),
            "bravo-mime" => include_str!("../../🧫️fixtures/📇️contributions/bravo-mime.json"),
            "bravo-extension" => include_str!("../../🧫️fixtures/📇️contributions/bravo-extension.json"),
            _ => second.schema,
        };
        let first = contribution("alpha");
        let mut registry = ContributionRegistry::new(vec![first]).unwrap();
        let left: serde_json::Value = serde_json::from_str(first.schema).unwrap();
        let right: serde_json::Value = serde_json::from_str(second.schema).unwrap();
        let disjoint = |field: &str| left["representations"][0][field].as_array().unwrap().iter().all(|claim| !right["representations"][0][field].as_array().unwrap().contains(claim));
        let oracle = left["id"] != right["id"] && left["directory"] != right["directory"] && disjoint("mimes") && disjoint("extensions");
        assert_eq!(oracle, row["accepted"].as_bool().unwrap());
        assert_eq!(registry.register(second).is_ok(), oracle, "{}", row["id"]);
        assert_eq!(registry.contributions().len(), if oracle { 2 } else { 1 });
    }
    for row in vectors["receiptCases"].as_array().unwrap() {
        let mut selected = contribution("alpha");
        if row["authoredFactory"].as_bool().unwrap() {
            selected.native_codecs = || vec![NativeCodecFactory { id: "unauthorized", artifact: "alpha", kind: || unreachable!("unauthorized factory must not instantiate"), codec: || unreachable!("unauthorized codec must not instantiate") }];
        }
        let registry = ContributionRegistry::new(vec![selected]).unwrap();
        let schema: serde_json::Value = serde_json::from_str(selected.schema).unwrap();
        let expected = schema["codecs"].as_array().unwrap().iter().filter(|codec| codec["executable_registration"] == true).count();
        let oracle = (selected.native_codecs)().len() == expected;
        assert_eq!(oracle, row["accepted"].as_bool().unwrap());
        assert_eq!(registry.native_codec_factory_receipts("neutral", "semio:neutral", "1").is_ok(), oracle);
    }
}

#[test]
fn authored_definition_constraints_match_independent_serde_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📇️contributions/🔣️.json")).unwrap();
    for row in vectors["validationCases"].as_array().unwrap() {
        let mut source: serde_json::Value = serde_json::from_str(ALPHA).unwrap();
        source["representations"][0]["mimes"] = row["mimes"].clone();
        let claims: Vec<_> = row["mimes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|mime| serde_json::json!({"namespace": "mime", "value": mime}))
            .chain(source["representations"][0]["extensions"].as_array().unwrap().iter().map(|extension| serde_json::json!({"namespace": "extension", "value": extension})))
            .collect();
        let descriptor = format!("runtime-capability:representation:{}", claims.iter().map(|claim| format!("{}:{}", claim["namespace"].as_str().unwrap(), claim["value"].as_str().unwrap())).collect::<Vec<_>>().join("|"));
        let representation = source["runtime_capabilities"].as_array_mut().unwrap().iter_mut().find(|item| item["category"] == "representation").unwrap();
        representation["claims"] = serde_json::to_value(claims).unwrap();
        representation["descriptor"] = descriptor.into();
        let mut selected = contribution("alpha");
        selected.schema = Box::leak(serde_json::to_string(&source).unwrap().into_boxed_str());
        selected.definition_constraint = row["constraint"].as_bool().unwrap().then_some(include_str!("../../🧫️fixtures/📇️contributions/📜️constraint.json"));
        let expected = !row["constraint"].as_bool().unwrap() || source["representations"].as_array().unwrap().iter().all(|item| item["mimes"].as_array().unwrap().is_empty());
        assert_eq!(expected, row["accepted"].as_bool().unwrap());
        let result = ContributionRegistry::new(vec![selected]);
        let admitted = result.is_ok();
        assert_eq!(admitted, expected, "{}", row["id"]);
    }
}
