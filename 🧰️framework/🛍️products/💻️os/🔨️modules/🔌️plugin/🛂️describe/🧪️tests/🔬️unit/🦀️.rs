use super::*;

/// 🛣️ LAW: a package with one `io_mechanism` serializer and one deserializer of its own artifact describes exactly those two io rows in
/// registry order — the deserializer as an import into its native dialect, the serializer as an export out of it.
#[semio_framework_async_macros::async_test]
async fn package_descriptor_lists_its_io_mechanism_rows_with_their_native_side() {
    use semio_framework::io::io_mechanism::{io_register, IoEntry, IoEntryDirection as Side};
    use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
    const NATIVE: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.describe-io.native", standard: semio_framework::StandardId("1"), subset: semio_framework::SubsetId("*") };
    const FOREIGN: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.describe-io-foreign.format", standard: semio_framework::StandardId("1"), subset: semio_framework::SubsetId("*") };
    #[allow(clippy::unnecessary_wraps, reason = "IoEntry test doubles implement its fallible function-pointer contract")]
    fn passthrough(payload: &IoPayload) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(payload.clone()))
    }
    static ENTRIES: [IoEntry; 2] = [
        IoEntry { from: NATIVE, into: FOREIGN, fidelity: IoFidelity::Lossy, direction: Side::Export, sniff: None, run: passthrough },
        IoEntry { from: FOREIGN, into: NATIVE, fidelity: IoFidelity::Lossy, direction: Side::Import, sniff: None, run: passthrough },
    ];
    io_register(&ENTRIES).expect("the entries register");
    let plugin = crate::app::Plugin::<crate::app::NoPluginApp>::builder("describe-io").label("Describe Io").version("0.1.0").package_id("semio:describe-io").try_build().expect("the plugin assembles");
    let runtime = crate::plugin_runtime::PluginRuntime::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
    let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("descriptor wire decodes");
    let descriptor: PackageDescriptor = serde_json::from_value(value.into()).expect("descriptor shape decodes");
    let (native, foreign) = (semio_framework::ArtifactDialect::from(NATIVE), semio_framework::ArtifactDialect::from(FOREIGN));
    assert_eq!(
        descriptor.contributions.io_entries,
        vec![
            IoEntryDescriptor { owner: native.clone(), counterpart: foreign.clone(), direction: IoEntryDirection::Import },
            IoEntryDescriptor { owner: native, counterpart: foreign, direction: IoEntryDirection::Export },
        ]
    );
}

#[semio_framework_async_macros::async_test]
async fn package_descriptor_advertises_metadata_only_cold_inference_routes() {
    let metadata = crate::app::ArtifactInferenceServiceMetadata {
        owner: "describe-routed-inference",
        artifact_kind: "s.describe.route",
        artifact_schema: "s.describe.route",
        artifact_schema_version: 1,
        inference_schema: "s.describe.route.solve",
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        payload: None,
    };
    let plugin =
        crate::app::Plugin::<crate::app::NoPluginApp>::builder(metadata.owner).label("Describe Routed Inference").version("0.1.0").package_id("semio:describe-routed-inference").routed_inference(metadata).try_build().expect("routed plugin assembles");
    let runtime = crate::plugin_runtime::PluginRuntime::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
    let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("descriptor wire decodes");
    let descriptor: PackageDescriptor = serde_json::from_value(value.into()).expect("descriptor shape decodes");
    assert_eq!(descriptor.contributions.inference_services.len(), 1);
    let route = &descriptor.contributions.inference_services[0];
    assert_eq!((route.owner.as_str(), route.artifact_kind.as_str(), route.inference_schema.as_str()), (metadata.owner, metadata.artifact_kind, metadata.inference_schema));
    assert!(crate::app::artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service lookup").is_none());
}
