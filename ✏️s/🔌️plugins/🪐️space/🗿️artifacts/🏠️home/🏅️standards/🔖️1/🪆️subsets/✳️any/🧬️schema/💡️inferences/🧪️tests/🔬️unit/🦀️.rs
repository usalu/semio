
use super::*;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 7 };
    assert_eq!(SHomeInference::infer(&snapshot), SHomeInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SHomeInference::infer(&SHomeSnapshot::default()), SHomeInference::default());
}

#[semio_framework_async_macros::async_test]
async fn different_generations_yield_different_digests() {
    let a = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 1 };
    let b = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 2 };
    assert_ne!(SHomeInference::infer(&a).content_digest, SHomeInference::infer(&b).content_digest);
}
//#endregion 🧪️InferenceLaws
