
use super::*;

fn descriptor() -> PackageDescriptor {
    PackageDescriptor {
        descriptor_version: 1,
        package_id: "semio:value-codec-fixture".into(),
        role: PackageRole::Plugin,
        manifest: PluginManifest {
            plugin_id: "value-codec-fixture".into(),
            label: "Value Codec Fixture".into(),
            version: "1.0.0".into(),
            apps: Vec::new(),
            examples: Vec::new(),
            capabilities: Vec::new(),
            topic_contributions: Vec::new(),
            commands: Vec::new(),
            artifact_kinds: Vec::new(),
            dependencies: Vec::new(),
            contributions: Vec::new(),
        },
        activation_events: Vec::new(),
        capability_requests: Vec::new(),
        extension_points: Vec::new(),
        execution: ExecutionMode::Isolated,
        execution_protocol: ExecutionProtocol { app_channel_version: 14 },
        quotas: kernel::QuotaSchema::default(),
        contributions: ContributionSet::default(),
        assets: Vec::new(),
        hashes: PackageHashes { wasm_sha256: String::new(), core_wasm_sha256: String::new(), descriptor_sha256: String::new() },
    }
}

#[test]
fn package_descriptor_first_party_codec_preserves_serde_wire_and_required_fields() {
    let descriptor = descriptor();
    let value = descriptor.to_value();
    assert_eq!(serde_json::Value::from(&value), serde_json::to_value(&descriptor).expect("serde oracle encodes descriptor"));
    assert_eq!(PackageDescriptor::from_value(value.clone()).expect("first-party codec decodes descriptor"), descriptor);
    let DslValue::Object(fields) = value else { panic!("descriptor must encode as an object") };
    for omitted in ["activationEvents", "capabilityRequests", "extensionPoints", "assets"] {
        assert!(!fields.iter().any(|(name, _)| name == omitted), "{omitted} must remain omitted when empty");
    }
    for required in ["descriptorVersion", "packageId", "executionProtocol"] {
        let mut missing = fields.clone();
        missing.retain(|(name, _)| name != required);
        let error = PackageDescriptor::from_value(DslValue::Object(missing)).expect_err("missing required descriptor field must fail");
        assert!(error.to_string().contains(required), "{error}");
    }
}
