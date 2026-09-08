
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(dag_play_labels(&DagConfig::default()).nodes.as_str(), "Nodes");
    assert_eq!(dag_play_labels(&DagConfig { locale: "de-DE".into(), ..DagConfig::default() }).nodes.as_str(), "Knoten");
}

#[semio_framework_async_macros::async_test]
async fn is_de_locale_matches_any_de_prefixed_tag() {
    assert!(is_de_locale(&DagConfig { locale: "de".into(), ..DagConfig::default() }));
    assert!(is_de_locale(&DagConfig { locale: "de-DE".into(), ..DagConfig::default() }));
    assert!(!is_de_locale(&DagConfig { locale: "en-US".into(), ..DagConfig::default() }));
}
