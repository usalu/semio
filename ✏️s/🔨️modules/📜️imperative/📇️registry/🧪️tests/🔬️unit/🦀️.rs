
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

#[cfg(feature = "linked-modules")]
#[semio_framework_async_macros::async_test]
async fn linked_modules_bootstrap_registers_text_operators() {
    super::linked_modules::bootstrap_linked_modules().await;
    let registry = imperative_module_registry();
    assert!(registry.operator_info("text.uppercase").is_some());
    assert!(registry.operator_info("math.add").is_some());
}
