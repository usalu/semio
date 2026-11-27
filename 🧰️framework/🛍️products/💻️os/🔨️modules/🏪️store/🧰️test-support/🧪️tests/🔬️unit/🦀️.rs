//! 🔬️ Unit tests of os-store-test-support; the work package moves the tests of its sources here.

use std::collections::{BTreeMap, VecDeque};
use std::path::{Path, PathBuf};

/// 🧭️ Locates the cargo workspace root that owns `Cargo.lock`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|dir| dir.join("Cargo.lock").is_file()).expect("a workspace root with Cargo.lock").to_path_buf()
}

/// 📦️ Reads every workspace member manifest keyed by its package name.
fn workspace_manifests() -> BTreeMap<String, toml::Table> {
    let root = workspace_root();
    let workspace: toml::Table = std::fs::read_to_string(root.join("Cargo.toml")).expect("root manifest").parse().expect("root manifest is toml");
    workspace["workspace"]["members"].as_array().expect("workspace members").iter().filter_map(|member| {
        let manifest: toml::Table = std::fs::read_to_string(root.join(member.as_str()?).join("Cargo.toml")).ok()?.parse().ok()?;
        Some((manifest.get("package")?.get("name")?.as_str()?.to_owned(), manifest))
    }).collect()
}

/// 🔗️ Names every package a manifest links into its production build: normal and build dependencies, plain and per target.
fn production_dependencies(manifest: &toml::Table) -> Vec<String> {
    let direct = ["dependencies", "build-dependencies"].into_iter().filter_map(|section| manifest.get(section));
    let targeted = manifest.get("target").and_then(toml::Value::as_table).into_iter().flat_map(|targets| targets.values()).flat_map(|target| ["dependencies", "build-dependencies"].into_iter().filter_map(|section| target.get(section)));
    direct.chain(targeted).filter_map(toml::Value::as_table).flat_map(|table| table.iter()).map(|(name, spec)| spec.get("package").and_then(toml::Value::as_str).unwrap_or(name).to_owned()).collect()
}

#[test]
fn kernel_production_closure_never_links_test_support() {
    let manifests = workspace_manifests();
    let mut parents: BTreeMap<String, String> = BTreeMap::new();
    let mut queue = VecDeque::from(["semio-os-kernel".to_owned()]);
    parents.insert("semio-os-kernel".into(), String::new());
    while let Some(package) = queue.pop_front() {
        let Some(manifest) = manifests.get(&package) else { continue };
        for dependency in production_dependencies(manifest) {
            if parents.contains_key(&dependency) { continue }
            parents.insert(dependency.clone(), package.clone());
            queue.push_back(dependency);
        }
    }
    let mut chain = vec!["semio-os-store-test-support".to_owned()];
    while let Some(parent) = parents.get(chain.last().expect("a chain")).filter(|parent| !parent.is_empty()) { chain.push(parent.clone()) }
    assert!(!parents.contains_key("semio-os-store-test-support"), "test support reaches production through {}", chain.iter().rev().cloned().collect::<Vec<_>>().join(" -> "));
}

#[test]
fn guard_walk_sees_the_kernel_and_its_schema_exports() {
    let manifests = workspace_manifests();
    assert!(manifests.contains_key("semio-os-kernel"));
    assert!(production_dependencies(&manifests["semio-os-kernel"]).contains(&"semio-os-store-schema-exports".to_owned()));
    assert!(!production_dependencies(&manifests["semio-os-kernel"]).contains(&"semio-os-store-test-support".to_owned()));
}
