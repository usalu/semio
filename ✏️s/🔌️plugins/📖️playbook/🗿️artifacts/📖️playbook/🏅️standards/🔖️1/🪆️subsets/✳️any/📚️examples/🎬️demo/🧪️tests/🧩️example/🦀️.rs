#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🌊️ The demo parent names its `flow` child by the stable id the catalogue answers, and that child's own asset holds the three
/// steps in chain order (the `sequence` edges, not the node vector, are the order).
#[semio_framework_async_macros::async_test]
async fn the_demo_flow_child_holds_its_steps_in_chain_order() {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::parse_playbook_dsl(crate::examples::demo::PRIMARY_TEXT).expect("demo parent parses");
    assert_eq!(snapshot.flow.child_id, crate::examples::demo::FLOW_ID);
    let content = crate::playbook_genesis_flow(&snapshot.flow.child_id).expect("the catalogue answers the demo flow child");
    let steps = crate::steps_from_flow_content(&content).expect("decodable demo steps");
    assert_eq!(steps.iter().map(|step| step.id.as_str()).collect::<Vec<_>>(), vec!["basics", "schedule", "pattern"]);
    assert!(steps.iter().all(|step| !step.blocks.is_empty()));
}

//#region 💡️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::PlaybookInference;
    use protocol::Inference;

    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::parse_playbook_dsl(crate::examples::demo::PRIMARY_TEXT).expect("demo parent parses");
    assert_eq!(PlaybookInference::infer(&snapshot).expect("pure inference"), PlaybookInference::infer(&snapshot).expect("pure inference"));
}

#[semio_framework_async_macros::async_test]
async fn the_demo_steps_have_an_acyclic_topology() {
    use crate::standards::v1::subsets::any::schema::inferences::topology::compute_playbook_topology;
    let content = crate::playbook_genesis_flow(crate::examples::demo::FLOW_ID).expect("demo flow");
    let steps = crate::steps_from_flow_content(&content).expect("decodable demo steps");
    let topology = compute_playbook_topology(&steps);
    assert_eq!(topology.node_count, steps.iter().map(|step| 1 + step.blocks.len() as u32).sum::<u32>());
    assert!(topology.cycle_free, "the demo document has no cyclic block conditions");
}
//#endregion 💡️InferenceLaws
