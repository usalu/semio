//! ️tests for example `🌲️concrete-forest`.

#[test]
async fn dsl_asset_parses_and_round_trips() {
    let text = include_str!("../🖼️assets/🗣️forest.dsl.semio");
    assert!(text.len() > 64, "dsl fixture must carry real payload");
    let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text).expect("example dsl parses");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
}

#[test]
async fn op_pack_and_spr_assets_are_nonempty() {
    assert!(include_str!("../🖼️assets/🔧️forest.op.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/🎒️forest.pack.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/📡️forest.spr.semio").len() > 64);
}

#[test]
async fn inference_default_law() {
    use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::inferences::Puzzle5dInference;
    use protocol::Inference;
    assert_eq!(
        Puzzle5dInference::infer(&crate::artifacts::puzzle5d::Puzzle5dSnapshot::default()),
        Puzzle5dInference::default()
    );
}

#[test]
async fn inference_determinism_law() {
    use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::inferences::Puzzle5dInference;
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️forest.dsl.semio");
    let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(Puzzle5dInference::infer(&projection), Puzzle5dInference::infer(&projection));
}
