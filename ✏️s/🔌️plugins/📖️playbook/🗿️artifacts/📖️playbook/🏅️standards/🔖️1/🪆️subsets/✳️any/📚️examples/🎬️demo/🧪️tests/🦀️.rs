#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::playbook::standards::v1::subsets::any::schema::inferences::PlaybookInference;
    use crate::artifacts::playbook::PlaybookSnapshot;
    use protocol::Inference;

    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    let snapshot = <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo asset parses as a playbook snapshot");
    let inference = PlaybookInference::infer(&snapshot);
    assert_eq!(inference, PlaybookInference::infer(&snapshot));

    let expected_nodes: u32 = snapshot.steps().iter().map(|step| 1 + step.blocks.len() as u32).sum();
    assert_eq!(inference.topology.node_count, expected_nodes);
    assert!(inference.topology.cycle_free, "the demo document has no cyclic block conditions");
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::playbook::standards::v1::subsets::any::schema::inferences::PlaybookInference;
    use crate::artifacts::playbook::PlaybookSnapshot;
    use protocol::Inference;

    assert_eq!(PlaybookInference::infer(&PlaybookSnapshot::default()), PlaybookInference::default());
}
//#endregion 💡️InferenceLaws
