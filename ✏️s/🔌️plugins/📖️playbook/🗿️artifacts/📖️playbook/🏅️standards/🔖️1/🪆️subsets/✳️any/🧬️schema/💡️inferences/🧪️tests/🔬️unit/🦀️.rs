use super::*;
use protocol::Inference;

//#region 🧸️Fixtures
fn block(id: &str, kind: &str, condition: Option<crate::PlaybookExpr>) -> crate::PlaybookBlock {
    let mut block = crate::schema::default_block(id.into(), kind);
    block.condition = condition;
    block
}

fn step_with_conditional_block() -> Vec<crate::PlaybookStep> {
    vec![crate::PlaybookStep {
        id: "s1".into(),
        title: "Step 1".into(),
        description: None,
        blocks: vec![block("material", "single", None), block("finish", "text", Some(crate::PlaybookExpr::Truthy { expr: Box::new(crate::PlaybookExpr::Var { name: "material".into() }) }))],
    }]
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = PlaybookSnapshot::default();
    assert_eq!(PlaybookInference::infer(&snapshot).expect("pure inference"), PlaybookInference::infer(&snapshot).expect("pure inference"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(PlaybookInference::infer(&PlaybookSnapshot::default()).expect("pure inference"), PlaybookInference::default());
}

/// 🧭️ The parent alone carries no steps (they live on the `flow` child, design §20.15), so it infers the empty topology.
#[semio_framework_async_macros::async_test]
async fn the_parent_alone_infers_the_topology_of_no_steps() {
    let inferred = PlaybookInference::infer(&PlaybookSnapshot::default()).expect("pure inference");
    assert_eq!(inferred.topology.node_count, 0);
    assert!(inferred.topology.cycle_free);
}

#[semio_framework_async_macros::async_test]
async fn topology_orders_the_conditioned_block_after_its_dependency() {
    let topology = compute_playbook_topology(&step_with_conditional_block());
    let material_index = topology.topo_order.iter().position(|id| id == "material").unwrap();
    let finish_index = topology.topo_order.iter().position(|id| id == "finish").unwrap();
    assert!(material_index < finish_index, "finish's condition reads material, so material must precede it");
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 3);
}
//#endregion 🧪️InferenceLaws
