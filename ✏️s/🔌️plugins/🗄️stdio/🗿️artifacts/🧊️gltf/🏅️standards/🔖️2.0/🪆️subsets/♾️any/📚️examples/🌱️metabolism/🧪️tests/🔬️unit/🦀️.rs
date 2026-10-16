use super::*;

#[semio_framework_async_macros::async_test]
async fn base_glb_decodes_to_a_nonempty_real_document() {
    let snapshot = decoded_snapshot();
    assert_eq!(snapshot.document.asset.version, "2.0");
    assert!(!snapshot.buffers.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn demo_source_nonempty() {
    let source = source();
    assert_eq!(source.id(), ID);
    assert!(!source.document_json().is_empty());
}

/// 🔏️ The digest sidecar declares the fixture the host would deliver: a repository-relative name, the exact length of the bytes, a
/// well-formed SHA-256 (recomputed independently by the `📦️package-asset-constants` law), and the example is declared by it.
#[semio_framework_async_macros::async_test]
async fn the_digest_sidecar_declares_the_fixture_it_stands_for() {
    let (name, size_bytes, sha256) = base_glb_bytes_digest();
    assert!(!name.starts_with('/') && !name.contains(".."), "{name}");
    assert_eq!(base_glb_bytes().len() as u64, size_bytes);
    assert_eq!(sha256.len(), 64);
    assert!(sha256.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    let source = source();
    assert!(matches!(source.body(), semio_framework_plugin::ExampleSourceBody::Deferred { source_suffix, source, .. } if *source_suffix == ".glb.digest" && *source == BASE_GLB_DIGEST));
}
