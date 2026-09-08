
use super::*;
#[semio_framework_async_macros::async_test]
async fn demo_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}

/// 🧪️ Ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING's
/// inference laws, exercised against this example's own real fixture (`PRIMARY_TEXT`,
/// parsed through the real `ArtifactDsl` codec — not a hand-built stub).
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::JpgSnapshot;
    use crate::standards::v_jfif_1_01::subsets::document::schema::inferences::JpgInference;
    use protocol::Inference;
    let snapshot = <JpgSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("demo fixture must parse");
    assert_eq!(JpgInference::infer(&snapshot), JpgInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::JpgSnapshot;
    use crate::standards::v_jfif_1_01::subsets::document::schema::inferences::JpgInference;
    use protocol::Inference;
    assert_eq!(JpgInference::infer(&JpgSnapshot::default()), JpgInference::default());
}
