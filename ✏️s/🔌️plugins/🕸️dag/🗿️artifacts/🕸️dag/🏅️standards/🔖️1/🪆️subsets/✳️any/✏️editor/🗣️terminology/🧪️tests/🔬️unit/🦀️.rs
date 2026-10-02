use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_shared_view_state() {
    assert_eq!(dag_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).nodes.as_str(), "Nodes");
    assert_eq!(dag_play_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::De, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) }).nodes.as_str(), "Knoten");
}

#[semio_framework_async_macros::async_test]
async fn is_de_locale_matches_the_shared_view_state_locale() {
    assert!(is_de_locale(&semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::De, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) }));
    assert!(!is_de_locale(&semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::En, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }));
}
