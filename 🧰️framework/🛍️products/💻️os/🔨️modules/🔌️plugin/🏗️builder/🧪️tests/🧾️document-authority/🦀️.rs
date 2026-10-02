use super::*;

#[semio_framework_async_macros::async_test]
async fn schema_document_authority_follows_portable_owner_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️document-authority/🔣️.json")).expect("portable owner corpus");
    for row in corpus["cases"].as_array().expect("cases") {
        let input = &row["input"];
        let plugin_id = input["pluginId"].as_str().expect("plugin identity");
        let owner = input["ownerPluginId"].as_str().expect("document owner");
        let scope: &'static str = Box::leak(input["scope"].as_str().expect("scope").to_owned().into_boxed_str());
        let mut builder = Plugin::<crate::app::NoPluginApp>::builder(plugin_id).label("Schema Authority Fixture").version("0.1.0").package_id(format!("semio:{plugin_id}"));
        for dependency in input["dependencies"].as_array().expect("dependencies") {
            builder = builder.depends_on(dependency.as_str().expect("dependency identity"), semio_framework::tree_pin!());
        }
        let result = builder.schema_documents(owner, semio_framework_schema_registry::ScopeSchemaExports { scope, exports: &[] }).try_build();
        assert_eq!(result.is_ok(), row["accepted"].as_bool().expect("expected authority"), "{}", row["id"]);
        if let Err(error) = result {
            assert_eq!(error.code, "plugin-assembly.schema-documents-owner");
        }
    }
}
