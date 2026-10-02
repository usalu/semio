use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_shared_view_state() {
    assert_eq!(block3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).summary.as_str(), "Object kind");
    assert_eq!(block3d_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::De, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) }).summary.as_str(), "Objektart");
}
