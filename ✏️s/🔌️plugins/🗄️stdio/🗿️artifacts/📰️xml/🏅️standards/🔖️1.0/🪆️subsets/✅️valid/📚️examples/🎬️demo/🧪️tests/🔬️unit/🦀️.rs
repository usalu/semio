use super::*;
use crate::standards::v1_0::subsets::base::schema::snapshot::XmlSnapshot;
use crate::standards::v1_0::subsets::valid::schema::check_valid_conformance;
use dsl::Severity;

#[semio_framework_async_macros::async_test]
async fn positive_asset_is_well_formed_and_valid() {
    assert!(!PRIMARY_TEXT.is_empty());
    let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("well-formed");
    let diagnostics = check_valid_conformance(&snapshot);
    assert!(diagnostics.iter().all(|entry| !matches!(entry.severity, Severity::Error | Severity::Fatal)), "got {diagnostics:?}");
    let _ = source();
}
