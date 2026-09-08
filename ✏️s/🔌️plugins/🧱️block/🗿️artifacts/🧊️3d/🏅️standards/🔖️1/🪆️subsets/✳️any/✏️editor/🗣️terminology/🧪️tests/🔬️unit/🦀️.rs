
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(block3d_labels(&Block3dConfig::default()).summary.as_str(), "Object kind");
    assert_eq!(block3d_labels(&Block3dConfig { locale: "de-DE".into(), ..Block3dConfig::default() }).summary.as_str(), "Objektart");
}
