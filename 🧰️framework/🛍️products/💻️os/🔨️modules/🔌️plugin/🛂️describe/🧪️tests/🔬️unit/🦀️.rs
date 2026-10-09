use super::*;

/// 🛣️ LAW: a package with one `io_mechanism` serializer and one deserializer of its own artifact describes exactly those two io rows in
/// registry order — the deserializer as an import into its native dialect, the serializer as an export out of it.
#[semio_framework_async_macros::async_test]
async fn package_descriptor_lists_its_io_mechanism_rows_with_their_native_side() {
    use semio_framework_os_kernel::io::io_mechanism::{io_register, IoEntry, IoEntryDirection as Side};
    use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
    const NATIVE: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.describe-io.native", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") };
    const FOREIGN: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.describe-io-foreign.format", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") };
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
    let runtime = crate::plugin_runtime::PluginRuntime::new({ let grant = crate::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; crate::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
    let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("descriptor wire decodes");
    let descriptor: PackageDescriptor = serde_json::from_value(value.into()).expect("descriptor shape decodes");
    let (native, foreign) = (semio_framework_artifact_reference::ArtifactDialect::from(NATIVE), semio_framework_artifact_reference::ArtifactDialect::from(FOREIGN));
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
    let runtime = crate::plugin_runtime::PluginRuntime::new({ let grant = crate::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; crate::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
    let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("descriptor wire decodes");
    let descriptor: PackageDescriptor = serde_json::from_value(value.into()).expect("descriptor shape decodes");
    assert_eq!(descriptor.contributions.inference_services.len(), 1);
    let route = &descriptor.contributions.inference_services[0];
    assert_eq!((route.owner.as_str(), route.artifact_kind.as_str(), route.inference_schema.as_str()), (metadata.owner, metadata.artifact_kind, metadata.inference_schema));
    assert!(crate::app::artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service lookup").is_none());
}

/// 🔤️ Guest descriptors preserve the emitter's neutral canonical bytes and serde member ordering.
#[semio_framework_async_macros::async_test]
async fn native_descriptor_topic_payload_matches_canonical_emission() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../../🖨️describe/🧫️fixtures/🧫️canonical-descriptor-pack/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("canonical descriptor vectors");
    let plugin = crate::app::Plugin::<crate::app::NoPluginApp>::builder("canonical-native").label("Canonical Native").version("1.0.0").package_id("semio:canonical-native").try_build().expect("native plugin");
    let runtime = crate::plugin_runtime::PluginRuntime::new({ let grant = crate::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; crate::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
    let initial = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect("native descriptor");
    let mut descriptor: PackageDescriptor = semio_framework_value::FromValue::from_value(initial).expect("descriptor shape");
    for case in fixture.get("cases").and_then(|value| value.as_array()).expect("cases") {
        let payload = semio_framework_pack_json::to_dsl_value(case.get("authored").expect("authored payload"));
        descriptor.manifest.topic_contributions = vec![semio_framework::TopicContribution::new("canonical.payload", payload)];
        descriptor.contributions.topic_contributions = descriptor.manifest.topic_contributions.clone();
        let oracle = serde_json::to_value(&descriptor).expect("independent ordered object oracle");
        let expected: semio_framework_value::DslValue = serde_json::from_value(oracle).expect("ordered oracle projection");
        let bytes = encode_package_descriptor(&descriptor);
        assert_eq!(bytes, store::pack_rt::encode_wire_value(&expected));
        assert_eq!(crate::plugin_runtime::descriptor_bytes_with_blank_hashes(&bytes).expect("native freshness bytes"), bytes);
        eprintln!("[DEBUG] Native canonical descriptor matched neutral payload and serde oracle: {}", case.get("id").and_then(|value| value.as_str()).expect("case id"));
    }
}
