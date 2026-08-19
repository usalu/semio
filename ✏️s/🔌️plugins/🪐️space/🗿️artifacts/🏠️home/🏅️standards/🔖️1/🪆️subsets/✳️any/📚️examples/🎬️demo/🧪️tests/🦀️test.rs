#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use crate::artifacts::home::SHomeSnapshot;
use crate::artifacts::home::standards::v1::subsets::any::schema::inferences::SHomeInference;
use protocol::Inference;

#[test]
async fn inference_determinism_law() {
    let snapshot = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 42 };
    assert_eq!(SHomeInference::infer(&snapshot), SHomeInference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    assert_eq!(SHomeInference::infer(&SHomeSnapshot::default()), SHomeInference::default());
}
//#endregion 🧪️InferenceLaws
