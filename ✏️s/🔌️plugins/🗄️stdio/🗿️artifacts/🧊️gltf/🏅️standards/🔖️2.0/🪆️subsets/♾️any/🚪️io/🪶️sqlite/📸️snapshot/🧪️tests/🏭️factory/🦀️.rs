//! 🏭️ The GLTF native codec factory exposes the model's owned SQLite provider and its structural pack hash.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;

#[test]
fn sqlite_snapshot_gltf_actual_factory_exposes_owned_provider_and_structural_hash() {
    let codec = (crate::native_codecs()[0].codec)();
    assert!(codec.snapshot_sqlite.is_some());
    let spec = <GltfSnapshot as store::os_store::serialization::ArtifactPack>::record_spec().expect("owned GLTF record spec");
    assert_eq!(codec.pack_schema_hash, store::schema_hash(&spec));
}
