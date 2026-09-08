
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(block5d_labels("en-US").summary.as_str(), "Part kind");
    assert_eq!(block5d_labels("de-DE").summary.as_str(), "Teilart");
}
