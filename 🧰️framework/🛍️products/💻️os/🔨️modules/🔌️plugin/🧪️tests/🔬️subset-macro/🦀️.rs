use super::*;
use dsl::{Diagnostic, FaultCode, Severity, TextSpan};

struct MacroDerivedValidator;

impl SubsetValidator for MacroDerivedValidator {
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.subset-macro", standard: StandardId("1"), subset: SubsetId("derived") };

    async fn validate(_payload: &IoPayload) -> Vec<Diagnostic> {
        vec![Diagnostic { code: FaultCode::new("test.subset-macro.ok"), severity: Severity::Info, span: TextSpan::at(1, 1), message: "subset! macro derived validator smoke".into(), expected: None, scope: FaultScope::default() }]
    }
}

subset! {
    pub derived dialect "s.test.subset-macro" / "1" / "derived" {
        validator: MacroDerivedValidator,
    }
}

#[semio_framework_async_macros::async_test]
async fn subset_macro_derived_register_is_idempotent() {
    let completion: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️completion/🧪️fixture/🔣️.json"))).unwrap();
    register_subset().await;
    register_subset().await;
    let registered = io::list_registered_subset_validator_dialects().await.expect("registry observation");
    assert_eq!(registered.iter().filter(|dialect| **dialect == SUBSET_DIALECT).count() as u64, completion["registration"]["registeredEntries"].as_u64().unwrap(), "two completed registrations must install exactly one actual registry entry");
    let diagnostics = MacroDerivedValidator::validate(&IoPayload::Text(String::new())).await;
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code.0, "test.subset-macro.ok");
}

#[semio_framework_async_macros::async_test]
async fn subset_macro_derived_kind_and_dialect() {
    assert_eq!(KIND, SubsetKind::Derived);
    assert_eq!(SUBSET_DIALECT.artifact_kind, "s.test.subset-macro");
    assert_eq!(SUBSET_DIALECT.standard.0, "1");
    assert_eq!(SUBSET_DIALECT.subset.0, "derived");
}
