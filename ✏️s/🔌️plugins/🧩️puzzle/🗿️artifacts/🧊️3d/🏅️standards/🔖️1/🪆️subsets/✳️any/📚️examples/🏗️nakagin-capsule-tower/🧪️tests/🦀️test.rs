//! ️tests for example `🏗️nakagin-capsule-tower`.

#[semio_framework_async_macros::async_test]
async fn dsl_asset_parses_and_round_trips() {
    let text = include_str!("../🖼️assets/🗣️tower.dsl.semio");
    assert!(text.len() > 64, "dsl fixture must carry real payload");
    let projection = crate::artifacts::puzzle3d::dsl::parse_dsl(text).expect("example dsl parses");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
}

#[semio_framework_async_macros::async_test]
async fn op_pack_and_spr_assets_are_nonempty() {
    assert!(include_str!("../🖼️assets/🔧️tower.op.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/🎒️tower.pack.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/📡️tower.spr.semio").len() > 64);
}
