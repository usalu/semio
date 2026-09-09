use super::*;

/// 🌱 Reuses `demo_mesh_snapshot()` (single source of truth, also feeds the shipped fixtures
/// and `🎹️composer/🦀️.rs`'s conformance-law tests) rather than an independent copy.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioMeshSnapshot {
    demo_mesh_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = populated();
    let bytes = <SemioMeshSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = populated();
    let text = <SemioMeshSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_no_meshes_materials_or_textures() {
    let snap = SemioMeshSnapshot::default();
    assert!(snap.meshes.is_empty() && snap.materials.is_empty() && snap.textures.is_empty());
}
