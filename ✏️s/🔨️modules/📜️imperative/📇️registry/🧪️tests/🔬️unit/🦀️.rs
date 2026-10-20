
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_contributions_yield_empty_registry() {
    sync_imperative_module_contributions("[]");
    let registry = imperative_module_registry();
    assert!(registry.operator_catalogue().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn sync_is_idempotent_for_same_json() {
    sync_imperative_module_contributions("[]");
    sync_imperative_module_contributions("[]");
    assert!(imperative_module_registry().operator_catalogue().is_empty());
}
