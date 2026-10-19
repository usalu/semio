//! 🧪️tests for example `🌲️concrete-forest`.

use semio_framework_test_fixtures::{fixture_bytes, fixture_text};

#[test]
fn dsl_asset_parses_and_round_trips() {
    let text = fixture_text!("../../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio");
    assert!(text.len() > 64, "dsl snapshot must carry real payload");
    let projection = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).expect("example dsl parses");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
}

#[test]
fn op_pack_and_spr_assets_are_nonempty() {
    assert!(fixture_text!("../../../../📚️examples/🌲️concrete-forest/🖼️assets/🔧️forest.op.semio").len() > 64);
    assert!(fixture_bytes!("../../../../📚️examples/🌲️concrete-forest/🖼️assets/🎒️.pack.semio").len() > 64);
    assert!(fixture_bytes!("../../../../📚️examples/🌲️concrete-forest/🖼️assets/📡️forest.spr.semio").len() > 64);
}
