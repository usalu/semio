
use super::*;


/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection moved off this config
/// backwards-restores-snapshot contract.
#[semio_framework_async_macros::async_test]
async fn config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = Block2dConfig::default();
    let next = operation.diff(&base).into_parts().0;
    assert_eq!(next.locale, "de-DE");
    let inverse = operation.inverse(&base);
    assert_eq!(inverse, vec![Block2dConfigMutation::Snapshot { config: base.clone() }]);
    assert_eq!(inverse[0].diff(&next).into_parts().0, base);
}
