#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::curate::CurateSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::curate::standards::v1::subsets::any::schema::inferences::CurateInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::curate::standards::v1::subsets::any::schema::inferences::CurateInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::curate::standards::v1::subsets::any::schema::inferences::CurateInference::infer(&crate::artifacts::curate::CurateSnapshot::default()),
        crate::artifacts::curate::standards::v1::subsets::any::schema::inferences::CurateInference::default(),
    );
}

#[test]
fn entries_census_the_demo_fixtures_stock_catalog() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::curate::CurateSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::curate::standards::v1::subsets::any::schema::inferences::CurateInference::infer(&snapshot);
    assert_eq!(inference.entries.stock_count, snapshot.stock_extra.len() as u32);
    assert_eq!(inference.entries.entry_count, snapshot.curated.len() as u32);
}
//#endregion 🧪️InferenceLaws
