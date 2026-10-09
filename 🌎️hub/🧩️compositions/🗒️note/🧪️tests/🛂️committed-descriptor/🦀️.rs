//! 🗒️ Committed note descriptor smoke law owned by the note composition.
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use semio_framework_os_run::{InMemoryBlobStore, WasmtimeNodeHost};

/// 🧭️ Walks up from `CARGO_MANIFEST_DIR` looking for `nx.json` — the SAME strategy
/// `🏗️bootstrap/🦀️.rs`'s own `find_repo_root` uses, duplicated here (not `include!`d — the bin crate's
/// own doc explains a `[[bin]]` target does not share the lib's module tree).
fn test_repo_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("nx.json").is_file() {
            return dir;
        }
        assert!(dir.pop(), "walked past the filesystem root looking for nx.json");
    }
}

/// 🧪️ Loads the committed note descriptor using the canonical runtime profile order.
/// Absence skips this optional integration probe and is never publication identity evidence.
#[semio_framework_async_macros::async_test]
async fn note_plugin_manifest_loads_from_its_committed_descriptor() {
    let repo_root = test_repo_root();
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛂️committed-descriptor/🔣️.json")).expect("note smoke contract");
    let descriptor_path = repo_root.join(contract["descriptorPath"].as_str().expect("descriptor path"));
    assert!(descriptor_path.is_file(), "committed note descriptor missing at {}", descriptor_path.display());

    let candidate_wasm_paths = contract["profiles"].as_array().expect("component profiles").iter().map(|profile| repo_root.join(contract["componentDirectory"].as_str().expect("component directory")).join(profile.as_str().expect("component profile")).join(contract["componentName"].as_str().expect("component name")));
    let Some(wasm_path) = candidate_wasm_paths.into_iter().find(|path| path.is_file()) else {
        return;
    };

    let mut plugin_paths = HashMap::new();
    plugin_paths.insert("note".to_string(), wasm_path);
    let mut descriptor_paths = HashMap::new();
    descriptor_paths.insert("note".to_string(), descriptor_path);
    let mut host = WasmtimeNodeHost::new(plugin_paths, descriptor_paths, Arc::new(InMemoryBlobStore::default()),original_test_shard_issuer()).await;

    host.hot_reload_plugin("note").await.expect("note must load natively from its committed descriptor, zero live describe() calls");
    let manifest = host.plugin_graph().manifest("note").await.expect("registered note manifest query").expect("loaded note manifest");
    assert_eq!(manifest.plugin_id, "note");
    assert!(!manifest.apps.is_empty(), "note's real manifest declares at least one app");
    assert!(manifest.dependencies.is_empty(), "note's committed descriptor declares zero PluginManifest.dependencies");

    let (routed_plugins, _routes) = host.io_router_stats().await;
    assert_eq!(routed_plugins, 1, "note must be the one plugin registered with the io router after this load");
    assert!(host.plugin_graph().is_registered("note").await.unwrap_or(false), "note must be registered in the plugin graph");
    assert!(host.app_router().owned_surface_gaps().await.is_empty(), "note's own panels leave no viewer/editor surface gap");
}

fn original_test_shard_issuer()->semio_framework_plugin_host::shard::OriginalShardIdentityIssuer{
 let policy=serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/⚙️configuration/🔣️.json")).expect("original independent fixture identity policy");
 semio_framework_plugin_host::shard::native_identity_issuer(policy,|_|Box::new(|_|true))
}
