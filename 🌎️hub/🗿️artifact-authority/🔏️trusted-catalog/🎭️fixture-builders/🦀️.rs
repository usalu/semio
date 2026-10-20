//! 🎭️ Fixture builders: never-executed plugin module files, component receipts and the headless stdio fixture package.

use super::*;

/// 🧫️ The files of a never-executed fixture plugin module: its entry, both descriptor forms (the packed one
/// the package's own descriptor), the package's canonical receipt and one vendored import shared by every fixture module.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn fixture_plugin_module_files(plugin_id: &str, descriptor_bytes: &[u8], receipt: &ComponentReceiptV1) -> Vec<(String, Vec<u8>)> {
    vec![
        (format!("{plugin_id}/🌉️bridge.js"), b"export async function createActorApi() {}\n".to_vec()),
        (format!("{plugin_id}/🔣️.json"), serde_json::json!({ "manifest": { "pluginId": plugin_id } }).to_string().into_bytes()),
        (format!("{plugin_id}/🛂️.descriptor.semio"), descriptor_bytes.to_vec()),
        (format!("{plugin_id}/{}", semio_hub_trusted_catalog_contract::plugin_module::TRUSTED_PLUGIN_MODULE_RECEIPT_FILE), receipt.encode_canonical()),
        ("🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js".to_owned(), b"export const streams = {};\n".to_vec()),
    ]
}

/// 🧫️ The shipped receipt a never-executed fixture component carries: it names exactly the component bytes with this digest
/// and length and this semantic descriptor.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn fixture_component_receipt(plugin_id: &str, package_id: &str, wasm_sha256: &str, wasm_bytes: u64, descriptor_bytes: &[u8]) -> ComponentReceiptV1 {
    ComponentReceiptV1 {
        schema: directory::os_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
        plugin_id: plugin_id.into(),
        package_id: package_id.into(),
        profile: directory::os_component_receipt::ComponentProfileV1::Release,
        target: directory::os_component_receipt::COMPONENT_TARGET.into(),
        wasm_sha256: wasm_sha256.into(),
        wasm_bytes,
        semantic_descriptor_sha256: hex_lower(&Sha256::digest(descriptor_bytes)),
        toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)".into(),
        source_tree_digest: "3".repeat(64),
        provenance_sha256: "4".repeat(64),
    }
}

/// 🧫️ Writes one package's fixture plugin module beneath `root` and answers the `pluginModule` record naming it.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn write_fixture_plugin_module(root: &Path, plugin_id: &str, package_id: &str, version: &str, receipt: &ComponentReceiptV1, descriptor_bytes: &[u8]) -> Result<serde_json::Value, AuthorityError> {
    let descriptor_byte_sha256 = hex_lower(&Sha256::digest(descriptor_bytes));
    let receipt_sha256 = hex_lower(&Sha256::digest(&receipt.encode_canonical()));
    let source = TrustedPluginModuleSourceV1 { plugin_id, package_id, version, component_sha256: &receipt.wasm_sha256, descriptor_byte_sha256: &descriptor_byte_sha256, receipt_sha256: &receipt_sha256 };
    let record = semio_hub_trusted_catalog_contract::plugin_module::write_plugin_module_bundle(root, &format!("plugin-module-{plugin_id}.json"), source, plugin_id, &fixture_plugin_module_files(plugin_id, descriptor_bytes, receipt))?;
    serde_json::to_value(record).map_err(catalog_error)
}

/// 🧫️ Shares headless Stdio metadata between native GIS fixtures; synthetic bytes are never executed.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn headless_stdio_fixture_package(root: &Path) -> Result<(serde_json::Value, serde_json::Value), AuthorityError> {
    let dependency = semio_hub_stdio_catalog::native_artifact_catalog_dependency().map_err(catalog_error)?;
    let version = dependency.version.0.to_string();
    let receipts = semio_hub_stdio_catalog::native_codec_factory_receipts().map_err(catalog_error)?;
    let mut builder = semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("stdio").label("Stdio Fixture").version(version.clone()).package_id("semio:stdio");
    for kind in semio_hub_stdio_catalog::native_codec_artifact_kinds() {
        builder = builder.artifact_kind(kind);
    }
    let plugin = builder.contributes_topic(semio_hub_stdio_catalog::native_artifact_catalog_contribution().map_err(catalog_error)?).try_library().map_err(catalog_error)?;
    let component = b"synthetic-stdio-component-for-linked-catalog-test";
    let component_sha256 = hex_lower(&Sha256::digest(component));
    let mut component_blake3 = Hasher::new();
    component_blake3.update(component);
    let descriptor = semio_framework::PackageDescriptor {
        descriptor_version: 1, package_id: "semio:stdio".into(), role: semio_framework::PackageRole::Plugin,
        manifest: plugin.manifest, activation_events: Vec::new(), capability_requests: Vec::new(), extension_points: Vec::new(),
        execution: semio_framework::ExecutionMode::Isolated,
        execution_protocol: semio_framework::ExecutionProtocol { app_channel_version: directory::os_spr::CHANNEL_VERSION },
        quotas: semio_framework::kernel::QuotaSchema::default(), contributions: semio_framework::ContributionSet::default(), assets: Vec::new(),
    };
    let bytes = directory::os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor));
    decode_package_descriptor(&bytes)?;
    std::fs::write(root.join("stdio-component.wasm"), component).map_err(catalog_error)?;
    std::fs::write(root.join("stdio-descriptor.semio"), &bytes).map_err(catalog_error)?;
    let identity = serde_json::json!({ "pluginId": "stdio", "packageId": "semio:stdio", "version": version });
    let codecs: Vec<_> = receipts.into_iter().map(|receipt| serde_json::json!({ "artifactKind": receipt.artifact_kind, "artifactSchema": receipt.schema, "packSchemaHash": hex_lower(&receipt.pack_schema_hash) })).collect();
    let receipt = fixture_component_receipt("stdio", "semio:stdio", &component_sha256, component.len() as u64, &bytes);
    let plugin_module = write_fixture_plugin_module(root, "stdio", "semio:stdio", &version, &receipt, &bytes)?;
    let record = serde_json::json!({
        "pluginId": "stdio", "packageId": "semio:stdio", "version": version, "role": "plugin", "dependencies": [],
        "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
        "component": { "path": "stdio-component.wasm", "byteLength": component.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
        "descriptor": { "path": "stdio-descriptor.semio", "byteLength": bytes.len(), "sha256": hex_lower(&Sha256::digest(&bytes)) },
        "receipt": receipt,
        "browserActor": { "kind": "none" }, "pluginModule": plugin_module, "nativeCodecs": codecs, "openTargets": [],
    });
    serde_json::from_value::<TrustedBundlePackageV1>(record.clone()).map_err(catalog_error)?;
    Ok((identity, record))
}
