#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🏛️bestest-650/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🧪️ The committed asset must decode to exactly the case this example claims — the example and
/// the engine's own case catalogue cannot drift apart silently.
#[semio_framework_async_macros::async_test]
async fn asset_carries_the_registered_case_model() {
    let text = include_str!("../../../../🖼️assets/🏛️bestest-650/🗣️.dsl.semio");
    let snapshot = <crate::EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("bestest 650 example parses");
    assert_eq!(snapshot.model, super::model());
}
