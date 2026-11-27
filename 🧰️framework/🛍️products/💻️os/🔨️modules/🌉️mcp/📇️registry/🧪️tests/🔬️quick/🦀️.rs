//! 🧱️ Registry_quick of the capabilities crate: Gateway capability declarations and the core catalog.

use semio_framework::manifest;
use semio_os_kernel::native_catalog::NativeCatalogV1;
use std::path::Path;
use crate::core_capabilities::core_tool_capabilities;
use semio_os_mcp_catalog::catalog::{CatalogSource, compile, compute_catalog_entries_hash};
use semio_os_mcp_catalog::registry::RegistryDiscovery;
#[cfg(any(test, feature = "testkit"))]
use crate::source_builders::{cad_descriptor, note_descriptor};

/// 📇️ Authors a closed native document and admits it through the production publication reader.
fn write_registry(root:&Path,rows:&[(&str,&str)])->NativeCatalogV1{
    let plugins:Vec<serde_json::Value>=rows.iter().map(|(plugin_id,owner_rel)|serde_json::json!({"pluginId":plugin_id,"cratePath":format!("{owner_rel}/📦️packages/🦀️rust"),"wasmOut":format!("{plugin_id}.wasm"),"role":"plugin","extends":null,"host":null,"capabilities":[],"activationEvents":[]})).collect();
    let publication=root.join("catalog");std::fs::create_dir_all(&publication).unwrap();std::fs::write(publication.join("🦀️catalog.json"),serde_json::to_vec(&serde_json::json!({"version":1,"plugins":plugins,"playgrounds":[]})).unwrap()).unwrap();semio_os_kernel::native_catalog::load_native_catalog_v1(Some(&publication)).unwrap()
}

fn write_descriptor(root: &Path, owner_rel: &str, descriptor: &manifest::PackageDescriptor) {
    let owner_root = root.join(owner_rel);
    std::fs::create_dir_all(&owner_root).expect("create owner root");
    std::fs::write(owner_root.join("🔣️.json"), serde_json::to_vec(descriptor).expect("descriptor serializes")).expect("write descriptor");
}

#[test]
fn discovers_exactly_the_descriptors_a_synthetic_registry_names() {
    let dir = semio_os_store_test_support::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    let catalog=write_registry(&root, &[("note", "plugins/note"), ("cad", "plugins/cad")]);
    write_descriptor(&root, "plugins/note", &note_descriptor());
    write_descriptor(&root, "plugins/cad", &cad_descriptor());

    let descriptors = RegistryDiscovery::scan(root.clone(),&catalog).unwrap().descriptors;
    let ids: Vec<&str> = descriptors.iter().map(|descriptor| descriptor.manifest.plugin_id.as_str()).collect();
    assert_eq!(ids, vec!["cad", "note"], "sorted by plugin id, exactly the two synthetic plugins, nothing else");
}

#[test]
fn a_malformed_descriptor_is_skipped_with_a_diagnostic_not_a_panic_or_err() {
    let dir = semio_os_store_test_support::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    let catalog=write_registry(&root, &[("broken", "plugins/broken")]);
    let owner_root = root.join("plugins/broken");
    std::fs::create_dir_all(&owner_root).expect("create owner root");
    std::fs::write(owner_root.join("🔣️.json"), b"not valid json").expect("write malformed descriptor");

    let discovery = RegistryDiscovery::scan(root.clone(),&catalog).unwrap();
    assert!(discovery.descriptors.is_empty(), "the malformed descriptor must not decode into a fabricated value");
    assert_eq!(discovery.diagnostics().len(), 1);
    assert!(discovery.diagnostics()[0].contains("broken"), "the diagnostic names the skipped plugin: {:?}", discovery.diagnostics());

    let descriptors = RegistryDiscovery::scan(root.clone(),&catalog).unwrap().descriptors;
    assert!(descriptors.is_empty());
}

#[test]
fn a_missing_registry_yields_an_empty_descriptor_list_plus_gateway_capabilities() {
    let dir = semio_os_store_test_support::tempdir().expect("tempdir");
    let source=CatalogSource{descriptors:RegistryDiscovery::scan(dir.path().to_path_buf(),&NativeCatalogV1::default()).unwrap().descriptors,gateway:core_tool_capabilities(),shell:Vec::new(),os_commands:Vec::new()};
    assert!(source.descriptors.is_empty(), "no registry was ever written under this tempdir");
    assert_eq!(source.gateway, core_tool_capabilities(), "gateway capabilities are always present, fixture-independent");
    assert!(source.shell.is_empty());
    assert!(source.os_commands.is_empty());
}

#[test]
fn the_same_input_compiles_to_a_byte_identical_catalog_hash_twice() {
    let dir = semio_os_store_test_support::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    let catalog=write_registry(&root, &[("note", "plugins/note"), ("cad", "plugins/cad")]);
    write_descriptor(&root, "plugins/note", &note_descriptor());
    write_descriptor(&root, "plugins/cad", &cad_descriptor());

    let source_a=CatalogSource{descriptors:RegistryDiscovery::scan(root.clone(),&catalog).unwrap().descriptors,gateway:core_tool_capabilities(),shell:Vec::new(),os_commands:Vec::new()};
    let source_b=CatalogSource{descriptors:RegistryDiscovery::scan(root.clone(),&catalog).unwrap().descriptors,gateway:core_tool_capabilities(),shell:Vec::new(),os_commands:Vec::new()};
    let catalog_a = compile(&source_a, semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).expect("compiles");
    let catalog_b = compile(&source_b, semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).expect("compiles");
    assert_eq!(catalog_a.hash, catalog_b.hash, "discovering + compiling the same install twice must be byte-identical");
}

#[test]
fn the_catalog_hash_changes_exactly_when_the_catalog_content_changes() {
    let source = CatalogSource { descriptors: vec![note_descriptor(), cad_descriptor()], gateway: core_tool_capabilities(), shell: Vec::new(), os_commands: Vec::new() };
    let compiled = |source: &CatalogSource| compile(source, semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).expect("compiles");
    let baseline = compiled(&source);
    assert_eq!(compiled(&source).hash, baseline.hash, "the same content hashes identically however the work is spread over workers");
    let (rehashed, entries) = compute_catalog_entries_hash(baseline.entries.clone());
    assert_eq!((rehashed, entries), (baseline.hash.clone(), baseline.entries.clone()), "the published hash is a pure function of the sorted entries");
    let mut changed = source.clone();
    changed.gateway[0].description.push('!');
    assert_ne!(compiled(&changed).hash, baseline.hash, "one changed character in one entry changes the hash");
    let mut fewer = source.clone();
    fewer.descriptors.pop();
    assert_ne!(compiled(&fewer).hash, baseline.hash, "a removed entry changes the hash");
}

#[test]
fn parallel_descriptor_loading_keeps_registry_order_for_paths_and_diagnostics() {
    let dir = semio_os_store_test_support::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    let rows = [("note", "plugins/note"), ("gone-a", "plugins/gone-a"), ("cad", "plugins/cad"), ("gone-b", "plugins/gone-b")];
    let catalog = write_registry(&root, &rows);
    write_descriptor(&root, "plugins/note", &note_descriptor());
    write_descriptor(&root, "plugins/cad", &cad_descriptor());
    let discovery = RegistryDiscovery::scan(root.clone(), &catalog).unwrap();
    let expected_paths: Vec<_> = rows.iter().map(|(_, owner)| root.join(owner).join("🔣️.json")).collect();
    assert_eq!(discovery.descriptor_paths, expected_paths);
    assert_eq!(discovery.descriptors.len(), 2);
    assert_eq!(discovery.diagnostics.len(), 2);
    assert!(discovery.diagnostics[0].contains("gone-a") && discovery.diagnostics[1].contains("gone-b"), "{:?}", discovery.diagnostics);
}
