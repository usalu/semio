
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(imperative_labels(&ImperativeConfig::default()).action_control_while.as_str(), "While");
    assert_eq!(imperative_labels(&ImperativeConfig { locale: "de-DE".into(), ..ImperativeConfig::default() }).action_control_while.as_str(), "Solange");
}
