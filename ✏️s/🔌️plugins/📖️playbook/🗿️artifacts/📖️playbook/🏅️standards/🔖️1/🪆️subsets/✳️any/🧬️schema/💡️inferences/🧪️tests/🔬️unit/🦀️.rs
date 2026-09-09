use super::*;
use protocol::Inference;

//#region 🧸️Fixtures
/// 🧸️ Built via `PlaybookStep`/`PlaybookBlock` construction (not raw JSON with a `"steps"` key —
/// `PlaybookSnapshot` no longer has that field; it composes `document`/`flow` children instead)
/// and minted through `playbook_snapshot_with_steps` so the working-scene cache is seeded.
fn step_with_conditional_block() -> PlaybookSnapshot {
    use crate::PlaybookBlock;

    fn block(id: &str, kind: &str, condition: Option<crate::PlaybookExpr>) -> PlaybookBlock {
        PlaybookBlock {
            id: id.into(),
            label: id.into(),
            kind: kind.into(),
            description: None,
            required: None,
            placeholder: None,
            default: None,
            min: None,
            max: None,
            step: None,
            unit: None,
            text: None,
            options: None,
            fields: None,
            schema: None,
            src: None,
            accept: None,
            fixture_slug: None,
            params: None,
            condition,
        }
    }

    let steps = vec![crate::PlaybookStep {
        id: "s1".into(),
        title: "Step 1".into(),
        description: None,
        blocks: vec![block("material", "single", None), block("finish", "text", Some(crate::PlaybookExpr::Truthy { expr: Box::new(crate::PlaybookExpr::Var { name: "material".into() }) }))],
    }];
    crate::playbook_snapshot_with_steps("playbook.playbook", "playbook", "1", None, steps)
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = step_with_conditional_block();
    assert_eq!(PlaybookInference::infer(&snapshot), PlaybookInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(PlaybookInference::infer(&PlaybookSnapshot::default()), PlaybookInference::default());
}

#[semio_framework_async_macros::async_test]
async fn topology_orders_the_conditioned_block_after_its_dependency() {
    let snapshot = step_with_conditional_block();
    let inferred = PlaybookInference::infer(&snapshot);
    let material_index = inferred.topology.topo_order.iter().position(|id| id == "material").unwrap();
    let finish_index = inferred.topology.topo_order.iter().position(|id| id == "finish").unwrap();
    assert!(material_index < finish_index, "finish's condition reads material, so material must precede it");
    assert!(inferred.topology.cycle_free);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_one_empty_step() {
    let inferred = PlaybookInference::infer(&PlaybookSnapshot::default());
    assert_eq!(inferred.topology.node_count, 1);
    assert!(inferred.topology.cycle_free);
}
//#endregion 🧪️InferenceLaws
