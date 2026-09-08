
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(sourcing_curation_labels(&SourcingCurationConfig::default()).window_pool.as_str(), "Pool");
    assert_eq!(sourcing_curation_labels(&SourcingCurationConfig { locale: "de-DE".into(), ..SourcingCurationConfig::default() }).col_curated.as_str(), "Kuratiert");
}
