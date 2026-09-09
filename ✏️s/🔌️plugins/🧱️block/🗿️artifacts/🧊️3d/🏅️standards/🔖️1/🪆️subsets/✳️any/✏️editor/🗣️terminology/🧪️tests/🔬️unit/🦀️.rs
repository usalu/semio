use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_shared_view_state() {
    assert_eq!(block3d_labels(&semio_framework_plugin::ViewModel::default()).summary.as_str(), "Object kind");
    assert_eq!(block3d_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).summary.as_str(), "Objektart");
}
