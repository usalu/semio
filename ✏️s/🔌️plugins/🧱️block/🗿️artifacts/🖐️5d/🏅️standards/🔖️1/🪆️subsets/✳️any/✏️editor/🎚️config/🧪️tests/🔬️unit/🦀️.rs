
use super::*;

#[semio_framework_async_macros::async_test]
async fn block5d_config_default_has_locale() {
    let config = Block5dConfig::default();
    assert_eq!(config.locale, "en-US");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection moved off this config
/// onto the framework's `grip` interaction domain — this now exercises `SetLocale` for the
/// backwards-restores-snapshot contract.
#[semio_framework_async_macros::async_test]
async fn config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = Block5dConfig::default();
    let operation = Block5dConfigMutation::SetLocale { value: "de-DE".into() };
    let next = operation.diff(&base).into_parts().0;
    assert_eq!(next.locale, "de-DE");
    let inverse = operation.inverse(&base);
    assert_eq!(inverse, vec![Block5dConfigMutation::Snapshot { config: base.clone() }]);
    assert_eq!(inverse[0].diff(&next).into_parts().0, base);
}
