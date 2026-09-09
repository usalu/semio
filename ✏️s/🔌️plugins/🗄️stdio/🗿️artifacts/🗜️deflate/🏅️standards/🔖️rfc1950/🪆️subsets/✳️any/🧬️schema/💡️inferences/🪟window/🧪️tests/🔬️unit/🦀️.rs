use super::*;
use crate::standards::v_rfc1950::subsets::any::schema::snapshot::DeflateLevelHint;

#[semio_framework_async_macros::async_test]
async fn window_bits_7_yields_the_real_32kb_rfc1950_ceiling() {
    let snapshot = DeflateSnapshot { window_bits: 7, ..DeflateSnapshot::default() };
    assert_eq!(compute_deflate_window(&snapshot).window_size, 32_768);
}

#[semio_framework_async_macros::async_test]
async fn a_reserved_cinfo_value_is_honestly_reported_as_zero_not_fabricated() {
    let snapshot = DeflateSnapshot { window_bits: 15, ..DeflateSnapshot::default() };
    assert_eq!(compute_deflate_window(&snapshot).window_size, 0);
}

#[semio_framework_async_macros::async_test]
async fn preset_dictionary_and_level_hint_are_read_verbatim() {
    let snapshot = DeflateSnapshot { compression_level_hint: DeflateLevelHint::Maximum, dict_id: Some(0xdead_beef), ..DeflateSnapshot::default() };
    let window = compute_deflate_window(&snapshot);
    assert!(window.has_preset_dictionary);
    assert_eq!(window.compression_level_hint, "Maximum");
}

#[semio_framework_async_macros::async_test]
async fn different_payloads_yield_different_digests() {
    let a = DeflateSnapshot { payload: vec![1, 2, 3], ..DeflateSnapshot::default() };
    let b = DeflateSnapshot { payload: vec![4, 5, 6], ..DeflateSnapshot::default() };
    assert_ne!(compute_deflate_window(&a).content_digest, compute_deflate_window(&b).content_digest);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DeflateSnapshot { payload: vec![9, 9, 9], ..DeflateSnapshot::default() };
    assert_eq!(compute_deflate_window(&snapshot), compute_deflate_window(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_deflate_window(&DeflateSnapshot::default()), DeflateWindow::default());
}
