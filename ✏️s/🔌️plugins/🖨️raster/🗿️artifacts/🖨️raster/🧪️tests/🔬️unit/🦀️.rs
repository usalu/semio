
use super::*;

trait RasterChildOwnerOracle {
    fn expected() -> dsl::json::Value;
}

struct CommittedRasterChildOwnerOracle;

impl RasterChildOwnerOracle for CommittedRasterChildOwnerOracle {
    fn expected() -> dsl::json::Value {
        dsl::json::parse(include_str!("../../🧪️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral Raster child-owner fixture")
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_matching_the_store_schema() {
    assert_eq!(artifact_kind().schema, RASTER_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn raster_materialization_is_owned_by_the_exact_snapshot_child() {
    let content = SemioImageSnapshot::default();
    let owned = image_content_child_handle("isolated", &content).with_local_owner(std::sync::Arc::new(content));
    let wire = dsl::json::to_string(&dsl::json::from_dsl_value(&dsl::ToValue::to_value(&owned)));
    let reconstructed: RasterAssetChild = dsl::json::from_json_str(&wire).expect("Raster child wire roundtrip");
    let observed = dsl::json::object([
        ("ownedHasMaterialization".to_string(), dsl::json::Value::Bool(owned.local_owner::<SemioImageSnapshot>().is_some())),
        ("wireIdentityMatches".to_string(), dsl::json::Value::Bool(owned == reconstructed)),
        ("wireHasMaterialization".to_string(), dsl::json::Value::Bool(reconstructed.local_owner::<SemioImageSnapshot>().is_some())),
    ]);

    assert!(dsl::json::value_eq_ignoring_object_order(&observed, &CommittedRasterChildOwnerOracle::expected()));
}
