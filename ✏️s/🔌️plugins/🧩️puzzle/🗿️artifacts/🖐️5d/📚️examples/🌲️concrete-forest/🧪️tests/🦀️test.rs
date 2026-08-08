//! ️tests for example `🌲️concrete-forest`.

#[test]
fn dsl_asset_parses_and_round_trips() {
    let text = include_str!("../🖼️assets/🗣️forest.dsl.semio");
    assert!(text.len() > 64, "dsl fixture must carry real payload");
    let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text).expect("example dsl parses");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
}

#[test]
fn op_pack_and_spr_assets_are_nonempty() {
    assert!(include_str!("../🖼️assets/🔧️forest.op.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/🎒️forest.pack.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/📡️forest.spr.semio").len() > 64);
}
