use super::*;

#[semio_framework_async_macros::async_test]
async fn package_descriptor_advertises_metadata_only_cold_inference_routes() {
    let metadata = crate::app::ArtifactInferenceServiceMetadata {
        owner: "describe-routed-inference",
        artifact_kind: "s.describe.route",
        artifact_schema: "s.describe.route",
        artifact_schema_version: 1,
        document_schema: "s.describe.route",
        document_schema_version: 1,
        inference_schema: "s.describe.route.solve",
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
    };
    let plugin = crate::app::Plugin::<crate::app::NoPluginApp>::builder(metadata.owner).label("Describe Routed Inference").version("0.1.0").routed_inference(metadata).try_build().expect("routed plugin assembles");
    let runtime = crate::plugin_runtime::PluginRuntime::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
    let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("descriptor wire decodes");
    let descriptor: PackageDescriptor = serde_json::from_value(value.into()).expect("descriptor shape decodes");
    assert_eq!(descriptor.contributions.inference_services.len(), 1);
    let route = &descriptor.contributions.inference_services[0];
    assert_eq!((route.owner.as_str(), route.artifact_kind.as_str(), route.inference_schema.as_str()), (metadata.owner, metadata.artifact_kind, metadata.inference_schema));
    assert!(crate::app::artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service lookup").is_none());
}
