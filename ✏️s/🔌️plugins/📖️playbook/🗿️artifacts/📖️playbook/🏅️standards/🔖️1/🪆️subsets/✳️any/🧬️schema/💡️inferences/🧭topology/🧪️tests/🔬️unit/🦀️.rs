
use super::*;
use crate::PlaybookBlock;

fn block(id: &str, condition: Option<PlaybookExpr>) -> PlaybookBlock {
    PlaybookBlock {
        id: id.into(),
        label: id.into(),
        kind: "text".into(),
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

fn step(id: &str, blocks: Vec<PlaybookBlock>) -> PlaybookStep {
    PlaybookStep { id: id.into(), title: id.into(), description: None, blocks }
}

//#region 🧪️TopologyLaws
#[semio_framework_async_macros::async_test]
async fn a_direct_cycle_between_two_blocks_is_reported() {
    let a = block("a", Some(PlaybookExpr::Var { name: "b".into() }));
    let b = block("b", Some(PlaybookExpr::Var { name: "a".into() }));
    let topology = compute_playbook_topology(&[step("s1", vec![a, b])]);
    assert!(!topology.cycle_free, "a reads b and b reads a — this is a genuine cycle");
}

#[semio_framework_async_macros::async_test]
async fn sequential_steps_without_conditions_stay_cycle_free_with_increasing_depth() {
    let topology = compute_playbook_topology(&[step("s1", vec![block("q1", None)]), step("s2", vec![block("q2", None)])]);
    assert!(topology.cycle_free);
    assert_eq!(topology.topo_order, vec!["s1", "q1", "s2", "q2"]);
    assert!(topology.depth["s2"] > topology.depth["s1"]);
}
//#endregion 🧪️TopologyLaws
