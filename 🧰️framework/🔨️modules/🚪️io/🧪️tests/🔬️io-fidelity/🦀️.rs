
use super::*;

#[semio_framework_async_macros::async_test]
async fn io_fidelity_class_parse_and_rank() {
    assert_eq!(IoFidelityClass::parse("exact").await.unwrap(), IoFidelityClass::Exact);
    assert_eq!(IoFidelityClass::parse("lossy").await.unwrap(), IoFidelityClass::Lossy);
    assert!(IoFidelityClass::parse("bogus").await.is_err());
    assert!(IoFidelityClass::Exact.rank().await > IoFidelityClass::Canonical.rank().await);
    assert!(IoFidelityClass::Canonical.rank().await > IoFidelityClass::Semantic.rank().await);
    assert!(IoFidelityClass::Semantic.rank().await > IoFidelityClass::Lossy.rank().await);
    assert_eq!(IoFidelityClass::Exact.as_str().await, "exact");
}

#[semio_framework_async_macros::async_test]
async fn io_fidelity_declaration_validate() {
    IoFidelityDeclaration { class: IoFidelityClass::Exact, drops: vec![] }.validate().await.unwrap();
    assert!(IoFidelityDeclaration { class: IoFidelityClass::Exact, drops: vec!["x".into()] }.validate().await.is_err());
    IoFidelityDeclaration { class: IoFidelityClass::Lossy, drops: vec!["meta.author".into()] }.validate().await.unwrap();
    assert!(IoFidelityDeclaration { class: IoFidelityClass::Lossy, drops: vec![] }.validate().await.is_err());
}
