use semio_s_artifact_stdio_contract::{ArtifactAssembly, ArtifactContribution};
use semio_s_artifact_stdio_contract::registry::ContributionRegistry;
use semio_framework_plugin::{ArtifactDefinition, PluginAssemblyError};
use semio_s_plugin_stdio::{AssemblyOwner, AssemblyPlan};
const ALPHA: &str = include_str!("../../📇️registry/🧬️contract/🧫️fixtures/📇️contributions/alpha.json");
const BRAVO: &str = include_str!("../../📇️registry/🧬️contract/🧫️fixtures/📇️contributions/bravo.json");
const CHARLIE: &str = include_str!("../../📇️registry/🧬️contract/🧫️fixtures/📇️contributions/charlie.json");

fn contribution(identity: &'static str) -> ArtifactContribution {
    fn definition(schema: &'static str) -> Result<ArtifactDefinition, PluginAssemblyError> {
        semio_s_artifact_stdio_contract::definition_from_schema(schema)
    }
    fn assembly(schema: &'static str) -> Result<ArtifactAssembly, PluginAssemblyError> {
        definition(schema).map(ArtifactAssembly::Definition)
    }
    let (schema, definition, assembly, formats) = match identity {
        "alpha" => (ALPHA, (|| definition(ALPHA)) as fn() -> _, (|| assembly(ALPHA)) as fn() -> _, (|| semio_s_artifact_stdio_contract::format_descriptors(ALPHA)) as fn() -> _),
        "bravo" => (BRAVO, (|| definition(BRAVO)) as fn() -> _, (|| assembly(BRAVO)) as fn() -> _, (|| semio_s_artifact_stdio_contract::format_descriptors(BRAVO)) as fn() -> _),
        "charlie" => (CHARLIE, (|| definition(CHARLIE)) as fn() -> _, (|| assembly(CHARLIE)) as fn() -> _, (|| semio_s_artifact_stdio_contract::format_descriptors(CHARLIE)) as fn() -> _),
        _ => panic!("unknown neutral contribution"),
    };
    ArtifactContribution { definition_constraint: None, identity, schema, definition, assembly, formats, native_codecs: Vec::new }
}

#[test]
fn selected_assembly_remains_exact_after_owner_removal() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️assembly/🔣️.json")).unwrap();
    for row in vectors["cases"].as_array().unwrap() {
        let selected = row["selected"].as_array().unwrap().iter().map(|id| contribution(match id.as_str().unwrap() { "alpha" => "alpha", "bravo" => "bravo", "charlie" => "charlie", _ => unreachable!() })).collect::<Vec<_>>();
        let mut registry = ContributionRegistry::new(selected).unwrap();
        let mut admitted = true;
        for id in row["remove"].as_array().unwrap() {
            admitted &= registry.remove(id.as_str().unwrap()).is_ok();
        }
        assert_eq!(admitted, row["accepted"].as_bool().unwrap());
        let plan = AssemblyPlan::new(&registry, AssemblyOwner { plugin_id: "stdio", package_id: "semio:stdio", package_version: "0.1.0" }).unwrap();
        let actual = plan.assemblies().iter().map(|assembly| match assembly { ArtifactAssembly::Definition(definition) => definition.identity().as_str().strip_prefix("s.stdio.").unwrap(), ArtifactAssembly::Runtime(_) => unreachable!() }).collect::<Vec<_>>();
        let oracle = row["remaining"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(actual, oracle);
        assert!(plan.receipts().is_empty());
        let builder = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Authored Assembly").version("0.1.0").package_id("semio:stdio");
        let plugin = plan.apply(builder).try_library().unwrap();
        assert_eq!(plugin.artifact_definitions().len(), oracle.len());
    }
}
