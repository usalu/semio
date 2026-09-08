
use super::*;

/// 📝️ Writes a synthetic `🔣️plugins.json` under `root`, matching `load_plugin_registry`'s real
/// schema (`pluginId`/`cratePath`/`wasmOut`) — `cratePath` is `<owner_rel>/📦️packages/🦀️rust` so
/// `load_plugin_registry`'s own owner-root derivation (two components back) resolves to
/// `root.join(owner_rel)`, exactly like the real generated registry.
fn write_registry(root: &Path, rows: &[(&str, &str)]) {
    let entries: Vec<serde_json::Value> = rows.iter().map(|(plugin_id, owner_rel)| serde_json::json!({ "pluginId": plugin_id, "cratePath": format!("{owner_rel}/📦️packages/🦀️rust"), "wasmOut": format!("{plugin_id}.wasm") })).collect();
    let registry_path = root.join("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json");
    std::fs::create_dir_all(registry_path.parent().expect("registry path has a parent")).expect("create registry dir");
    std::fs::write(&registry_path, serde_json::to_vec(&entries).expect("registry rows serialize")).expect("write registry");
}

fn write_descriptor(root: &Path, owner_rel: &str, descriptor: &manifest::PackageDescriptor) {
    let owner_root = root.join(owner_rel);
    std::fs::create_dir_all(&owner_root).expect("create owner root");
    std::fs::write(owner_root.join("🔣️.json"), serde_json::to_vec(descriptor).expect("descriptor serializes")).expect("write descriptor");
}

#[test]
fn discovers_exactly_the_descriptors_a_synthetic_registry_names() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    write_registry(&root, &[("note", "plugins/note"), ("cad", "plugins/cad")]);
    write_descriptor(&root, "plugins/note", &crate::note_descriptor());
    write_descriptor(&root, "plugins/cad", &crate::cad_descriptor());

    let descriptors = discover_descriptors(&root).expect("discovery over a valid directory never hard-fails");
    let ids: Vec<&str> = descriptors.iter().map(|descriptor| descriptor.manifest.plugin_id.as_str()).collect();
    assert_eq!(ids, vec!["cad", "note"], "sorted by plugin id, exactly the two synthetic plugins, nothing else");
}

#[test]
fn a_malformed_descriptor_is_skipped_with_a_diagnostic_not_a_panic_or_err() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    write_registry(&root, &[("broken", "plugins/broken")]);
    let owner_root = root.join("plugins/broken");
    std::fs::create_dir_all(&owner_root).expect("create owner root");
    std::fs::write(owner_root.join("🔣️.json"), b"not valid json").expect("write malformed descriptor");

    let discovery = RegistryDiscovery::scan(root.clone());
    assert!(discovery.descriptors.is_empty(), "the malformed descriptor must not decode into a fabricated value");
    assert_eq!(discovery.diagnostics().len(), 1);
    assert!(discovery.diagnostics()[0].contains("broken"), "the diagnostic names the skipped plugin: {:?}", discovery.diagnostics());

    let descriptors = discover_descriptors(&root).expect("a malformed descriptor is a diagnostic, not a hard Err");
    assert!(descriptors.is_empty());
}

#[test]
fn a_missing_registry_yields_an_empty_descriptor_list_plus_gateway_capabilities() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let source = discover_catalog_source(Some(dir.path()));
    assert!(source.descriptors.is_empty(), "no registry was ever written under this tempdir");
    assert_eq!(source.gateway, crate::core_tool_capabilities(), "gateway capabilities are always present, fixture-independent");
    assert!(source.shell.is_empty());
    assert!(source.os_commands.is_empty());
}

#[test]
fn the_same_input_compiles_to_a_byte_identical_catalog_hash_twice() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    write_registry(&root, &[("note", "plugins/note"), ("cad", "plugins/cad")]);
    write_descriptor(&root, "plugins/note", &crate::note_descriptor());
    write_descriptor(&root, "plugins/cad", &crate::cad_descriptor());

    let source_a = discover_catalog_source(Some(&root));
    let source_b = discover_catalog_source(Some(&root));
    let catalog_a = crate::compile(&source_a, semio_framework::Locale::En, semio_framework::Terminology::Native).expect("compiles");
    let catalog_b = crate::compile(&source_b, semio_framework::Locale::En, semio_framework::Terminology::Native).expect("compiles");
    assert_eq!(catalog_a.hash, catalog_b.hash, "discovering + compiling the same install twice must be byte-identical");
}
