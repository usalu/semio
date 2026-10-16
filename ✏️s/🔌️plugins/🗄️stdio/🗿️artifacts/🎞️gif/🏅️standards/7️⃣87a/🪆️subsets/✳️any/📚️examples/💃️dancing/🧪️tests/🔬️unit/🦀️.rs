use crate::examples::dancing::*;

#[semio_framework_async_macros::async_test]
async fn dancing_source_nonempty_and_decodes() {
    let src = source();
    assert!(!src.document_json().is_empty());
    let _ = decoded_snapshot();
}

/// 📦️ The declared package asset is the fixture the host would deliver: exact length, a repository-relative name, a well-formed digest.
/// The digest itself is recomputed independently by the `📦️package-asset-constants` law.
#[semio_framework_async_macros::async_test]
async fn the_declared_package_asset_matches_the_fixture_bytes() {
    let bytes = dancing_gif_bytes();
    assert_eq!(bytes.len() as u64, DANCING_GIF_ASSET_SIZE_BYTES);
    assert!(bytes.starts_with(b"GIF89a"), "the fixture is a GIF89a stream");
    assert!(!DANCING_GIF_ASSET_NAME.starts_with('/') && !DANCING_GIF_ASSET_NAME.contains(".."), "{DANCING_GIF_ASSET_NAME}");
    assert_eq!(DANCING_GIF_ASSET_SHA256.len(), 64);
    assert!(DANCING_GIF_ASSET_SHA256.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
}
