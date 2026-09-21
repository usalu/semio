use super::*;
use crate::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance;
use crate::JsonSnapshot;
use dsl::Severity;

#[semio_framework_async_macros::async_test]
async fn demo_asset_is_the_printer_s_own_canonical_text() {
    let snapshot = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("well-formed json dsl");
    assert_eq!(<JsonSnapshot as store::ArtifactDsl>::print_dsl(&snapshot), PRIMARY_TEXT);
    let _ = source();
}

#[semio_framework_async_macros::async_test]
async fn demo_asset_conforms_to_i_json() {
    let snapshot = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("well-formed json dsl");
    let diagnostics = check_i_json_conformance(&snapshot);
    assert!(diagnostics.iter().all(|entry| !matches!(entry.severity, Severity::Error | Severity::Fatal)), "got {diagnostics:?}");
}
