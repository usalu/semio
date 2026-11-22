//! 🧰️ LAW: the native extension store door answers the browser door's requests with the browser door's answers over the TypeScript store's own on-disk layout — list, `.sxt` install (verified, materialized as `component.wasm` beside `📥️install.json`), cancelled pick, uninstall, and refusals — so the shell's marketplace flow cannot tell the backends apart.

use super::*;
use std::future::Future;
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..1_000_000 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
    }
    panic!("the store door never settled");
}

struct Root(PathBuf);

impl Root {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("semio-extension-store-law-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn answer(root: &Path, request: serde_json::Value) -> Result<serde_json::Value, String> {
    block_on(native_extension_store_answer(&request, root))
}

fn sample_package(component: &[u8]) -> Vec<u8> {
    use semio_framework_pack_json::{object, Value};
    let manifest = semio_os_extension::ExtensionPackageManifest {
        extension_id: "flow.math".into(),
        directory_name: "🧮️flow-math".into(),
        label: "Flow Math".into(),
        version: "0.1.0".into(),
        extends: "flow".into(),
        capabilities: vec!["flow.operator".into()],
        topic_contributions: Value::Array(vec![object([("kind".to_string(), Value::from("flowExtension")), ("id".to_string(), Value::from("math.add"))])]),
        dependencies: vec![semio_os_extension::PackagePluginDependency { plugin_id: "flow".into(), version: "=1.0.0".into() }],
        contributions: Value::Array(Vec::new()),
        package_format: semio_os_extension::EXTENSION_PACKAGE_FORMAT,
    };
    block_on(semio_os_extension::pack(&manifest, component, &[])).expect("the sample package packs")
}

#[test]
fn an_sxt_install_materializes_the_store_layout_and_lists_uninstalls_like_the_browser_door() {
    let root = Root::new("install");
    let store = root.0.join("store");
    assert_eq!(answer(&store, serde_json::json!({ "op": "extension-store-list" })).unwrap(), serde_json::json!({ "extensions": [] }), "an absent store lists nothing");
    assert_eq!(answer(&store, serde_json::json!({ "op": "extension-store-install-file" })).unwrap(), serde_json::json!({ "cancelled": true }), "no picked file is a cancelled pick");

    let package = root.0.join("flow-math.sxt");
    let component = b"\0asm-law-component";
    std::fs::write(&package, sample_package(component)).unwrap();
    let installed = answer(&store, serde_json::json!({ "op": "extension-store-install-file", "path": package })).unwrap();
    let record = &installed["extension"];
    assert_eq!((record["extensionId"].as_str(), record["directoryName"].as_str(), record["version"].as_str(), record["extends"].as_str()), (Some("flow.math"), Some("🧮️flow-math"), Some("0.1.0"), Some("flow")));
    assert_eq!(record["moduleUrl"], "/🧩️extension-modules/🧮️flow-math/component.wasm");
    assert!(record["packageHash"].as_str().is_some_and(|hash| !hash.is_empty()) && record["installedAt"].is_u64());
    assert_eq!(std::fs::read(store.join("🧮️flow-math").join(NATIVE_EXTENSION_COMPONENT_FILE)).unwrap(), component, "the raw component is what the native host loads");
    assert!(store.join(NATIVE_EXTENSION_WATCH_MARKER).is_file(), "the watch marker the store's stream reads");
    assert_eq!(serde_json::from_slice::<Vec<String>>(&std::fs::read(store.join("🧮️flow-math").join(NATIVE_EXTENSION_CAPABILITIES_FILE)).unwrap()).unwrap(), ["flow.operator"], "the package's requested capabilities travel beside the record, to be scoped to the host's grants at activation");

    assert_eq!(answer(&store, serde_json::json!({ "op": "extension-store-list" })).unwrap(), serde_json::json!({ "extensions": [record] }), "the listed record is the installed record");
    assert_eq!(answer(&store, serde_json::json!({ "op": "extension-store-install-url", "url": format!("file://{}", package.display()) })).unwrap()["extension"], *record, "a file:// URL is the same install, idempotent");

    assert_eq!(answer(&store, serde_json::json!({ "op": "extension-store-uninstall", "extensionId": "flow.math" })).unwrap(), serde_json::json!({ "extensionId": "flow.math" }));
    assert!(!store.join("🧮️flow-math").exists());
    assert_eq!(answer(&store, serde_json::json!({ "op": "extension-store-list" })).unwrap(), serde_json::json!({ "extensions": [] }));
}

#[test]
fn the_door_refuses_what_the_browser_door_refuses() {
    let root = Root::new("refuse");
    let store = root.0.join("store");
    assert!(answer(&store, serde_json::json!({ "op": "extension-store-uninstall" })).unwrap_err().contains("extensionId"));
    assert!(answer(&store, serde_json::json!({ "op": "extension-store-install-url", "url": "https://example.test/a.sxt" })).is_err(), "a native host moves no package bytes over the network here");
    assert!(answer(&store, serde_json::json!({ "op": "nope" })).is_err());
    let garbage = root.0.join("garbage.sxt");
    std::fs::write(&garbage, b"not a package").unwrap();
    assert!(answer(&store, serde_json::json!({ "op": "extension-store-install-file", "path": garbage })).unwrap_err().contains("extension package"));
    let mismatched = store.join("📥️wrong");
    std::fs::create_dir_all(&mismatched).unwrap();
    std::fs::write(mismatched.join(NATIVE_EXTENSION_INSTALL_META), r#"{"extensionId":"x","directoryName":"other","version":"1","label":"X","extends":"flow","moduleUrl":"/m","packageHash":"h","installedAt":1}"#).unwrap();
    assert!(answer(&store, serde_json::json!({ "op": "extension-store-list" })).unwrap_err().contains("identity mismatch"), "a record must name its own directory");
}

#[test]
fn the_install_root_sits_beside_the_plugin_modules_root() {
    assert_eq!(native_extension_install_root(Path::new("/app/🔌️plugin-modules")), PathBuf::from("/app/🧩️extension-modules"));
}
