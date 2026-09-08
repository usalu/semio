use super::*;
use crate::os_dsl::schema::{FieldSpec, RecordLayout, RecordSpec, Shape};

/// @emoji 🧬️ A real (not mocked) minimal `RecordSpec` — one `Int` field under `Inline` layout.
// 🚫️async: E4 fn-pointer slot — passed to `register_schema_spec`/`FullResolver::from_map`,
// both `fn() -> RecordSpec` (sync, unnameable if async) — see R9/E4.
fn sample_spec() -> RecordSpec {
    RecordSpec::new(Some("p2m3-sample"), RecordLayout::Inline, vec![FieldSpec::new(0, "value", Shape::Int)])
}

#[semio_framework_async_macros::async_test]
async fn full_resolver_resolves_a_registered_schema_and_none_for_an_unregistered_one() {
    register_schema_spec("p2m3.registry-test.schema", sample_spec).await;
    let resolver = full_resolver().await;
    assert!(resolver.names().await.contains(&"p2m3.registry-test.schema".to_string()));
    let resolved = resolver.resolve("p2m3.registry-test.schema").await.expect("registered schema must resolve");
    assert_eq!(resolved.keyword.as_deref(), Some("p2m3-sample"));
    assert_eq!(resolved.fields.len(), 1);
    assert!(resolver.resolve("p2m3.registry-test.never-registered").await.is_none());
}

#[semio_framework_async_macros::async_test]
async fn full_resolver_resolves_the_diff_schema_id_separately_from_its_document_id() {
    register_schema_spec("p2m3.diff-test.doc", sample_spec).await;
    register_schema_spec("p2m3.diff-test.doc#diff", sample_spec).await;
    let resolver = full_resolver().await;
    assert!(resolver.resolve("p2m3.diff-test.doc").await.is_some());
    assert!(resolver.resolve("p2m3.diff-test.doc#diff").await.is_some());
    assert!(resolver.resolve("p2m3.diff-test.doc#nonsense").await.is_none());
}

#[semio_framework_async_macros::async_test]
async fn full_resolver_from_map_bypasses_the_global_registry() {
    let mut schemas: HashMap<&'static str, fn() -> RecordSpec> = HashMap::new();
    schemas.insert("p2m3.isolated.schema", sample_spec as fn() -> RecordSpec);
    let resolver = FullResolver::from_map(schemas).await;
    assert_eq!(resolver.names().await, vec!["p2m3.isolated.schema".to_string()]);
    assert!(resolver.resolve("p2m3.isolated.schema").await.is_some());
    // Not in THIS table (even though another test registers it globally) — proves from_map
    // is a genuinely isolated table, not a view into the process-global registry.
    assert!(resolver.resolve("p2m3.registry-test.schema").await.is_none());
}
