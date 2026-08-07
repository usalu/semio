//! ️tests for example `🏗️️nakagin-capsule-tower`.

#[test]
fn dsl_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️tower.dsl.semio");
    assert!(text.len() > 16, "dsl fixture must be non-empty");
}
