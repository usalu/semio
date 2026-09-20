use super::*;

/// 📇️ The bound a descriptor is actually read against, taken from its own declaration site rather
/// than restated here — `trustedBootstrapReadRegular` and the directory's execution-target read
/// both refuse a descriptor above it.
const DIRECTORY_SCHEMA_SOURCE: &str = include_str!("../../../../📇️directory/🧬️schema/🦀️.rs");

const DESCRIPTOR_MAX_BYTES: usize = 4 * 1024 * 1024;

/// 🧪️ One manifest carrying exactly one example of `body`, on a dialect no app claims.
fn manifest_with_example(body: String) -> PluginManifest {
    PluginManifest {
        plugin_id: "example-bound".to_string(),
        label: "Example Bound".to_string(),
        version: "0.1.0".to_string(),
        apps: Vec::new(),
        examples: vec![semio_framework::ExampleDefinition {
            id: "capsule-dream".to_string(),
            label: crate::LocalizedLabel::data("Capsule Dream"),
            icon_id: semio_framework::IconName::from("file"),
            artifact_json: body,
            dialect: semio_framework::ArtifactDialect { artifact_kind: "s.puzzle.5d".to_string(), standard: "1".to_string(), subset: "*".to_string() },
        }],
        capabilities: Vec::new(),
        topic_contributions: Vec::new(),
        commands: Vec::new(),
        artifact_kinds: Vec::new(),
        dependencies: Vec::new(),
        contributions: Vec::new(),
    }
}

fn encoded_bytes(manifest: &PluginManifest) -> usize {
    store::pack_rt::encode_wire_value(&dsl::to_dsl_value(manifest).expect("manifest encodes structurally")).len()
}

#[semio_framework_async_macros::async_test]
async fn the_descriptor_bound_this_law_measures_against_is_the_declared_one() {
    assert!(
        DIRECTORY_SCHEMA_SOURCE.contains("pub const DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES: u64 = 4 * 1024 * 1024;"),
        "DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES moved off 4 MiB — this law measures against a stale bound"
    );
}

/// 🛡️ A descriptor holding a multi-megabyte example stays under the 4 MiB bound, because the body
/// leaves as a referenced asset instead of riding inside the manifest.
///
/// 🐛️ `🧩️puzzle` is the measured case: its `capsule-dream` example is 3 577 295 B of inlined
/// `artifact_json` — 84.9 % of a 4 803 294 B descriptor that no consumer can read and whose
/// `describe` cannot finish (`🎫️…/🗑️generated/ex1-example-body-census.txt`). The fixture is that
/// exact body: inline it eats three quarters of the entire descriptor budget on its own, leaving
/// the package's real declarations to fit in the remainder — which is precisely how `🧩️puzzle`
/// went over. Externalized, the same manifest is an eighth of the budget.
#[semio_framework_async_macros::async_test]
async fn a_multi_megabyte_example_body_leaves_the_descriptor_as_a_referenced_asset() {
    let body = "x".repeat(3_577_295);
    let inline = encoded_bytes(&manifest_with_example(body.clone()));
    assert!(inline > DESCRIPTOR_MAX_BYTES * 3 / 4, "one inlined example body must dominate the descriptor budget or this law proves nothing (measured {inline} B)");

    let mut manifest = manifest_with_example(body.clone());
    let mut assets = Vec::new();
    externalize_oversized_example_bodies(&mut manifest, &mut assets);

    let split = encoded_bytes(&manifest);
    assert!(split <= DESCRIPTOR_MAX_BYTES / 8, "a manifest whose oversized example body was externalized must be a small fraction of the 4 MiB bound (measured {split} B)");
    assert_eq!(manifest.examples.len(), 1, "the example row itself must survive so every picker still resolves it");
    assert_eq!(manifest.examples[0].id, "capsule-dream");
    assert!(manifest.examples[0].artifact_json.is_empty(), "an externalized example must not also carry its body");
    assert_eq!(assets.len(), 1);
    assert_eq!(assets[0].name, "📚️examples/s.puzzle.5d.1.-/capsule-dream.json");
    assert_eq!(assets[0].size_bytes, body.len() as u64);
    assert_eq!(assets[0].sha256, semio_framework_hash::sha256_hex(body.as_bytes()), "the declaration must hash the exact body it replaced");
}

/// 🪶️ Every example body measured in the tree today is inline-sized and stays untouched, so this
/// split changes exactly the descriptors that are actually unbounded.
#[semio_framework_async_macros::async_test]
async fn a_body_at_the_inline_ceiling_is_left_alone() {
    let mut manifest = manifest_with_example("x".repeat(DESCRIPTOR_INLINE_EXAMPLE_MAX_BYTES));
    let mut assets = Vec::new();
    externalize_oversized_example_bodies(&mut manifest, &mut assets);
    assert!(assets.is_empty(), "a body at the ceiling must not be externalized");
    assert_eq!(manifest.examples[0].artifact_json.len(), DESCRIPTOR_INLINE_EXAMPLE_MAX_BYTES);
}
