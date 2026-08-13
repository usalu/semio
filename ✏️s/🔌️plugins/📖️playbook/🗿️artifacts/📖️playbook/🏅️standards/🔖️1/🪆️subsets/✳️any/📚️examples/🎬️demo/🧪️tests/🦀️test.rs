#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 💡️InferenceLaws
#[test]
fn inference_determinism_law() {
    use crate::artifacts::playbook::PlaybookSnapshot;
    use crate::artifacts::playbook::standards::v1::subsets::any::schema::inferences::PlaybookInference;
    use protocol::Inference;

    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo asset parses as a playbook snapshot");
    let inference = PlaybookInference::infer(&snapshot);
    assert_eq!(inference, PlaybookInference::infer(&snapshot));

    let expected_nodes: u32 = snapshot.steps().iter().map(|step| 1 + step.blocks.len() as u32).sum();
    assert_eq!(inference.topology.node_count, expected_nodes);
    assert!(inference.topology.cycle_free, "the demo document has no cyclic block conditions");
}

#[test]
fn inference_default_law() {
    use crate::artifacts::playbook::PlaybookSnapshot;
    use crate::artifacts::playbook::standards::v1::subsets::any::schema::inferences::PlaybookInference;
    use protocol::Inference;

    assert_eq!(PlaybookInference::infer(&PlaybookSnapshot::default()), PlaybookInference::default());
}
//#endregion 💡️InferenceLaws
