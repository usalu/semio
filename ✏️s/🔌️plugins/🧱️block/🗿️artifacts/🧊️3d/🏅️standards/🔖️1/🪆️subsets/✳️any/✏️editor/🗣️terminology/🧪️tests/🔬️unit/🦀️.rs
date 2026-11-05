use crate::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_shared_view_state() {
    assert_eq!(block3d_labels(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).summary.as_str(), "Object kind");
    assert_eq!(block3d_labels(&semio_plugin::ViewModel { locale: semio_ui_locale::Locale::De, ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::De, semio_ui_locale::Terminology::Native) }).summary.as_str(), "Objektart");
}
