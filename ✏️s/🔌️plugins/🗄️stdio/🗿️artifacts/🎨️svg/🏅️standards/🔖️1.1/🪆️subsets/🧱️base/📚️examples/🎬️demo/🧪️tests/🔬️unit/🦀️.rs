#[allow(unused_imports)]
use semio_framework_plugin::ExampleSource;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
use crate::examples::demo::*;
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
    use crate::standards::v1_1::subsets::base::schema::inferences::SvgInference;
    use crate::SvgSnapshot;
    use protocol::Inference;
    let snapshot = <SvgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("demo fixture must parse");
    assert_eq!(SvgInference::infer(&snapshot).expect("valid materialized inference fixture"), SvgInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::standards::v1_1::subsets::base::schema::inferences::SvgInference;
    use crate::SvgSnapshot;
    use protocol::Inference;
    assert_eq!(SvgInference::infer(&SvgSnapshot::default()).expect("valid materialized inference fixture"), SvgInference::default());
}
