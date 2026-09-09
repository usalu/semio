use super::*;

#[semio_framework_async_macros::async_test]
async fn real_bytes_yield_a_real_nonzero_extent() {
    let snapshot = BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..BinarySnapshot::default() };
    let extent = compute_binary_extent(&snapshot);
    assert_eq!(extent.byte_length, 5);
    assert!(!extent.is_empty);
}

#[semio_framework_async_macros::async_test]
async fn empty_bytes_yield_an_honest_empty_extent() {
    let extent = compute_binary_extent(&BinarySnapshot::default());
    assert_eq!(extent.byte_length, 0);
    assert!(extent.is_empty);
}

#[semio_framework_async_macros::async_test]
async fn different_bytes_yield_different_digests() {
    let a = BinarySnapshot { bytes: vec![1, 2, 3], ..BinarySnapshot::default() };
    let b = BinarySnapshot { bytes: vec![9, 9, 9], ..BinarySnapshot::default() };
    assert_ne!(compute_binary_extent(&a).content_digest, compute_binary_extent(&b).content_digest);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = BinarySnapshot { bytes: vec![7, 7, 7], ..BinarySnapshot::default() };
    assert_eq!(compute_binary_extent(&snapshot), compute_binary_extent(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_binary_extent(&BinarySnapshot::default()), BinaryExtent::default());
}
