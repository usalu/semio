//! 🏙️ Committed bim descriptor smoke law owned by the bim composition.
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use semio_framework_async::{CancelToken, OperationContext, TraceId};
use semio_os_run::{AppChannelHost, AppCommand, AppFrame, InMemoryBlobStore, WasmtimeNodeHost};

/// 🧭️ Declared package workspace above `CARGO_MANIFEST_DIR`, the SAME rule `🏗️bootstrap/🦀️.rs` uses.
fn test_repo_root() -> PathBuf {
    semio_framework_test_fixtures::repo_root!()
}

/// 🧪️ Loads the committed BIM descriptor with the staged component, or the component selected by `SEMIO_BIM_GUEST_COMPONENT`.
#[semio_framework_async_macros::async_test]
async fn bim_plugin_manifest_loads_from_its_committed_descriptor() {
    let repo_root = test_repo_root();
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛂️committed-descriptor/🔣️.json")).expect("bim smoke contract");
    let descriptor_path = repo_root.join(contract["descriptorPath"].as_str().expect("descriptor path"));
    assert!(descriptor_path.is_file(), "committed bim descriptor missing at {}", descriptor_path.display());

    let plugin_id = contract["pluginId"].as_str().expect("plugin identity");
    let mode = std::env::var("SEMIO_BUILD_MODE").unwrap_or_else(|_| "prototype".to_owned());
    let staged = || repo_root.join(contract["componentDirectory"].as_str().expect("component directory"))
        .join(contract["deliverables"][mode.as_str()].as_str().expect("component build mode"))
        .join(contract["componentName"].as_str().expect("component name"));
    let override_path = std::env::var_os("SEMIO_BIM_GUEST_COMPONENT").map(PathBuf::from);
    assert!(override_path.as_ref().is_none_or(|path| path.is_absolute()), "SEMIO_BIM_GUEST_COMPONENT must be absolute");
    let wasm_path = override_path.unwrap_or_else(staged);
    assert!(wasm_path.is_file(), "BIM guest component missing at {}", wasm_path.display());

    let plugin_paths = HashMap::from([(plugin_id.to_owned(), wasm_path)]);
    let descriptor_paths = HashMap::from([(plugin_id.to_owned(), descriptor_path)]);
    let policy = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🧫️fixtures/host-driver-policy.json")).expect("native guest relay fixture policy");
    let mut host = WasmtimeNodeHost::new(plugin_paths, descriptor_paths, Arc::new(InMemoryBlobStore::default()), original_test_shard_issuer(), policy, semio_framework_job::default_now_us, CancelToken::root_now()).await.expect("BIM host starts");

    host.hot_reload_plugin(plugin_id).await.expect("BIM loads from its committed descriptor");
    let manifest = host.plugin_graph().manifest(plugin_id).await.expect("registered BIM manifest query").expect("loaded BIM manifest");
    assert_eq!(manifest.plugin_id, plugin_id);
    let expected_apps: Vec<String> = serde_json::from_value(contract["applicationIds"].clone()).expect("application identities");
    let mut registered_apps: Vec<&str> = manifest.apps.iter().map(|app| app.id.as_str()).collect();
    registered_apps.sort_unstable();
    assert_eq!(registered_apps, expected_apps);
    assert!(manifest.dependencies.is_empty(), "bim's committed descriptor declares zero PluginManifest.dependencies");

    let (routed_plugins, _routes) = host.io_router_stats().await;
    assert_eq!(routed_plugins, 1, "bim must be the one plugin registered with the io router after this load");
    assert!(host.plugin_graph().is_registered(plugin_id).await.unwrap_or(false), "BIM must be registered in the plugin graph");
    assert!(host.app_router().owned_surface_gaps().await.is_empty(), "bim's own panels leave no viewer/editor surface gap");

    let sequence = contract["guestReadConfigSequence"].as_u64().expect("live request sequence");
    for app in expected_apps {
        let node = host.open(plugin_id, &app, "bim-smoke").await.expect("BIM Wasm application opens in the native host");
        let node_id = u64::from(node);
        let context = OperationContext { actor: node_id, generation: 0, trace: TraceId(node_id), lane: 0, deadline_ms: None, cancel: CancelToken::root_now(), capability: None };
        let frames = host.exchange(&context, node, vec![AppCommand::ReadConfig { seq: sequence }]).await.expect("BIM Wasm application executes a real channel request");
        assert!(frames.iter().any(|frame| matches!(frame, AppFrame::Config { in_reply_to, .. } if *in_reply_to == sequence)), "{app} did not return its guest configuration: {frames:?}");
    }
}

/// 🪪️ Issues independent shard identity for the native host fixture.
fn original_test_shard_issuer() -> semio_plugin_host::shard::OriginalShardIdentityIssuer {
    let policy = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/⚙️configuration/🔣️.json")).expect("original independent fixture identity policy");
    semio_plugin_host::shard::native_identity_issuer(policy, |_| Box::new(|_| true))
}
