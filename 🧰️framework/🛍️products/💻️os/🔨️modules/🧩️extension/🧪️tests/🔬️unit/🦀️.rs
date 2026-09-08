
use super::*;

#[test]
fn authored_installation_directory_survives_the_wire_codec() {
    let bytes = include_bytes!("../../🧪️installation.json");
    let owned = crate::os_pack::json::parse_bytes(bytes).unwrap();
    let oracle: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    let manifest = ExtensionPackageManifest::from_json(owned.get("manifest").unwrap()).unwrap();
    let serialized = manifest.to_json();
    assert_eq!(serialized.get("extensionId").and_then(crate::os_pack::json::Value::as_str), oracle["manifest"]["extensionId"].as_str());
    assert_eq!(serialized.get("directoryName").and_then(crate::os_pack::json::Value::as_str), oracle["manifest"]["directoryName"].as_str());
    assert_ne!(serialized.get("extensionId"), serialized.get("directoryName"));
}

async fn sample_manifest() -> ExtensionPackageManifest {
    use crate::os_pack::json::{Value, object};
    ExtensionPackageManifest {
        extension_id: "flow.math".into(),
        directory_name: "🧮️flow-math".into(),
        label: "Flow Math".into(),
        version: "0.1.0".into(),
        extends: "flow".into(),
        capabilities: vec!["flow.operator".into()],
        topic_contributions: Value::Array(vec![object([("kind".to_string(), Value::from("flowExtension")), ("id".to_string(), Value::from("math.add"))])]),
        dependencies: vec![PackagePluginDependency { plugin_id: "flow".into(), version: "^1.0.0".into() }],
        contributions: Value::Array(Vec::new()),
        package_format: EXTENSION_PACKAGE_FORMAT,
    }
}

//#region 🔖️DependencyAndContributionTests
#[semio_framework_async_macros::async_test]
async fn extends_matches_primary_dependency_holds_for_the_sample_and_the_vacuous_case() {
    assert!(sample_manifest().await.extends_matches_primary_dependency().await);

    let vacuous = ExtensionPackageManifest { extends: String::new(), dependencies: Vec::new(), ..sample_manifest().await };
    assert!(vacuous.extends_matches_primary_dependency().await);
}

#[semio_framework_async_macros::async_test]
async fn extends_matches_primary_dependency_rejects_mismatch_and_missing_dependency() {
    let mismatched = ExtensionPackageManifest { extends: "cad".into(), ..sample_manifest().await };
    assert!(!mismatched.extends_matches_primary_dependency().await);

    let no_dependencies = ExtensionPackageManifest { dependencies: Vec::new(), ..sample_manifest().await };
    assert!(!no_dependencies.extends_matches_primary_dependency().await, "non-empty extends with no dependencies is inconsistent");
}

#[semio_framework_async_macros::async_test]
async fn dependencies_default_absent_on_the_wire() {
    use crate::os_pack::json::{Value, object};
    let bare = object([
        ("extensionId".to_string(), Value::from("flow.math")),
        ("directoryName".to_string(), Value::from("🧮️flow-math")),
        ("label".to_string(), Value::from("Flow Math")),
        ("version".to_string(), Value::from("0.1.0")),
        ("extends".to_string(), Value::from("")),
        ("capabilities".to_string(), Value::Array(Vec::new())),
        ("topicContributions".to_string(), Value::Array(Vec::new())),
        ("packageFormat".to_string(), Value::from(EXTENSION_PACKAGE_FORMAT as u64)),
    ]);
    let parsed = ExtensionPackageManifest::from_json(&bare).unwrap();
    assert!(parsed.dependencies.is_empty());
    assert_eq!(parsed.contributions, Value::Null);
}

#[semio_framework_async_macros::async_test]
async fn package_plugin_dependency_round_trips_as_a_plain_string_pair() {
    use crate::os_pack::json::{Value, object};
    let dependency = PackagePluginDependency { plugin_id: "cad".into(), version: "^1.0.0".into() };
    let json = dependency.to_json();
    assert_eq!(json, object([("pluginId".to_string(), Value::from("cad")), ("version".to_string(), Value::from("^1.0.0"))]));
    let round_tripped = PackagePluginDependency::from_json(&json).unwrap();
    assert_eq!(round_tripped, dependency);
}
//#endregion 🔖️DependencyAndContributionTests

#[semio_framework_async_macros::async_test]
async fn pack_unpack_verify_round_trip() {
    let manifest = sample_manifest().await;
    let component = b"\0asm\x01\x00\x00\x00fake-component".to_vec();
    let assets = vec![("readme.txt".into(), b"hello".to_vec()), ("nested/icon.svg".into(), b"<svg/>".to_vec())];

    let packed = pack(&manifest, &component, &assets).await.expect("pack");
    assert!(packed.starts_with(&crate::os_semio::BINARY_MAGIC));

    let verified = verify(&packed).await.expect("verify");
    assert_eq!(verified, manifest);
    let unpacked = unpack(&packed).await.expect("unpack");
    assert_eq!(unpacked.manifest, manifest);
    assert_eq!(unpacked.component_wasm, component);
    assert_eq!(unpacked.assets.get("readme.txt").map(Vec::as_slice), Some(b"hello".as_slice()));
    assert_eq!(unpacked.assets.get("nested/icon.svg").map(Vec::as_slice), Some(b"<svg/>".as_slice()));

    let again = pack(&unpacked.manifest, &unpacked.component_wasm, &unpacked.assets.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()).await.expect("repack");
    assert_eq!(packed, again);
    assert_eq!(content_hash(&packed), content_hash(&again));
}

#[semio_framework_async_macros::async_test]
async fn content_hash_is_stable_blake3() {
    let packed = pack(&sample_manifest().await, b"component-bytes", &[]).await.expect("pack");
    assert_eq!(content_hash(&packed), semio_framework_hash::hash_bytes(&packed));
    assert_ne!(content_hash(&packed), content_hash(b"other"));
}

#[semio_framework_async_macros::async_test]
async fn verify_rejects_wrong_envelope() {
    let foreign = wrap_binary(&SemioEnvelope { plugin: "os".into(), artifact: "collection".into(), component: Component::Pack, version: 1 }, b"not-an-sxt");
    assert!(matches!(verify(&foreign).await, Err(ExtensionPackageError::UnexpectedEnvelope(_))));
}

#[semio_framework_async_macros::async_test]
async fn pack_rejects_empty_component() {
    assert!(matches!(pack(&sample_manifest().await, b"", &[]).await, Err(ExtensionPackageError::EmptyComponent)));
}
