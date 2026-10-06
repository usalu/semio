use super::*;

#[semio_framework_async_macros::async_test]
async fn artifact_kind_uses_the_playbook_media_kind_as_both_id_and_schema() {
    assert_eq!(artifact_kind().id, "text.playbook");
    assert_eq!(artifact_kind().schema, PLAYBOOK_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn block_fields_roundtrip() {
    let json = r#"{
            "id":"b1",
            "label":"Panel Count",
            "kind":"number",
            "required":true,
            "min":4,
            "max":64,
            "step":1,
            "unit":"panels"
        }"#;
    let block: PlaybookBlock = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("block json");
    assert_eq!(block.min, Some(4.0));
    assert_eq!(block.unit.as_deref(), Some("panels"));
    assert!(block.required.unwrap_or(false));
}

#[semio_framework_async_macros::async_test]
async fn playbook_child_restore_projection_accepts_the_one_owned_flow_child() {
    let snapshot = PlaybookSnapshot::default();
    let projection = playbook_child_restore_projection(&snapshot).expect("canonical Playbook flow child");
    assert_eq!(projection.len(), 1);
    assert!(projection.admits_member("flow", &snapshot.flow.target));
    assert_eq!(snapshot.flow.child_id, PLAYBOOK_GENESIS_FLOW_ID);
    assert_eq!(snapshot.flow.child_id, snapshot.flow.target.artifact_id);
}

//#region 🌉️ContentBridgeLaws
fn sample_steps() -> Vec<PlaybookStep> {
    vec![
        PlaybookStep {
            id: "intro".into(),
            title: "Introduction".into(),
            description: Some("What this playbook does.".into()),
            blocks: vec![PlaybookBlock {
                id: "name".into(),
                label: "Name".into(),
                kind: "text".into(),
                description: None,
                required: Some(true),
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
                example_id: None,
                params: None,
                condition: Some(PlaybookExpr::Truthy { expr: Box::new(PlaybookExpr::Var { name: "enabled".into() }) }),
            }],
        },
        PlaybookStep { id: "review".into(), title: "Review".into(), description: None, blocks: Vec::new() },
    ]
}

/// ⚖️ LAW: `flow` is the LOSSLESS source of truth — every step field (including nested `condition` trees) round-trips through
/// `flow_content_snapshot_from_steps`/`steps_from_flow_content` exactly.
#[semio_framework_async_macros::async_test]
async fn flow_content_round_trips_every_step_field_losslessly() {
    let steps = sample_steps();
    let content = flow_content_snapshot_from_steps(&steps);
    assert_eq!(content.nodes.len(), steps.len());
    assert_eq!(content.edges.len(), steps.len() - 1, "sequential steps chain via one edge per adjacent pair");
    assert_eq!(steps_from_flow_content(&content).expect("decodable steps"), steps);
}

/// ⚖️ LAW: the chain of `sequence` edges, not the node vector, is the step order; nodes off the chain follow in node order.
#[test]
fn the_chain_not_the_node_vector_is_the_step_order() {
    let mut content = flow_content_snapshot_from_steps(&sample_steps());
    content.nodes.reverse();
    assert_eq!(playbook_step_order(&content), vec!["intro", "review"]);
    content.edges.clear();
    assert_eq!(playbook_step_order(&content), vec!["review", "intro"], "with no chain the node order decides");
}

#[test]
fn an_undecodable_blocks_param_is_a_named_fault() {
    let mut content = flow_content_snapshot_from_steps(&sample_steps());
    content.nodes[0].params[0].value = "not json".into();
    assert!(steps_from_flow_content(&content).expect_err("undecodable").contains("intro"));
    assert_eq!(playbook_step_of(&content, "intro").expect_err("undecodable").code.0, "playbook.flow.content");
}
//#endregion 🌉️ContentBridgeLaws

//#region 🧬️ChildLaneLaws
/// 🧫️ The language-neutral child-leaf vectors (`🧫️fixtures/🧫️child-leaves/🔣️.json`): per verb, the exact flow leaves and the
/// steps they leave. Each vector is applied leaf by leaf through the stdio flow fold and undone through each leaf's own inverse
/// in reverse order, which must restore the base exactly (step order included).
#[test]
fn child_leaf_vectors_hold_and_every_edit_undoes_exactly() {
    use protocol::{Mutation, MutationDiff};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-leaves/🔣️.json")).expect("child-leaf vectors");
    let cases = fixture["cases"].as_array().expect("cases");
    assert_eq!(cases.len(), 9);
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let base: Vec<PlaybookStep> = serde_json::from_value(case["base"].clone()).expect("base steps");
        let content = flow_content_snapshot_from_steps(&base);
        let verb = &case["verb"];
        let text = |key: &str| verb[key].as_str().unwrap_or_default().to_string();
        let index = verb["index"].as_u64().map(|value| value as usize);
        let outcome = match verb["kind"].as_str().expect("verb kind") {
            "add-step" => playbook_add_step_leaves(&content, &serde_json::from_value(verb["step"].clone()).expect("step")),
            "remove-step" => playbook_remove_step_leaves(&content, &text("stepId")),
            "move-step" => playbook_move_step_leaves(&content, &text("stepId"), index.unwrap_or_default()),
            "add-block" => playbook_add_block_leaves(&content, &text("stepId"), serde_json::from_value(verb["block"].clone()).expect("block"), index),
            "remove-block" => playbook_remove_block_leaves(&content, &text("stepId"), &text("blockId")),
            "move-block" => playbook_move_block_leaves(&content, &text("blockId"), &text("fromStepId"), &text("toStepId"), index.unwrap_or_default()),
            other => panic!("{name}: unknown verb {other}"),
        };
        if let Some(code) = case["refusal"].as_str() {
            assert_eq!(outcome.expect_err(name).code.0, code, "{name}");
            continue;
        }
        let leaves = outcome.unwrap_or_else(|fault| panic!("{name}: {fault:?}"));
        let kinds: Vec<&str> = leaves.iter().map(|leaf| protocol::SemanticMutation::semantics(leaf).kind).collect();
        let expected: Vec<&str> = case["leaves"].as_array().expect("leaves").iter().map(|kind| kind.as_str().expect("leaf kind")).collect();
        assert_eq!(kinds, expected, "{name}: exact leaves in applied order");
        let mut applied = content.clone();
        let mut inverses: Vec<Vec<SemioFlowMutation>> = Vec::new();
        for leaf in &leaves {
            inverses.push(leaf.inverse(&applied).expect("leaf inverse"));
            applied = MutationDiff::apply(leaf.diff(&applied).diff(), &applied).unwrap_or_else(|error| panic!("{name}: {error}"));
        }
        let after: Vec<PlaybookStep> = serde_json::from_value(case["after"].clone()).expect("after steps");
        assert_eq!(steps_from_flow_content(&applied).expect("decodable after"), after, "{name}: steps after the edit");
        for inverse in inverses.into_iter().rev().flatten() {
            applied = MutationDiff::apply(inverse.diff(&applied).diff(), &applied).unwrap_or_else(|error| panic!("{name} undo: {error}"));
        }
        assert_eq!(steps_from_flow_content(&applied).expect("decodable undo"), base, "{name}: undo restores the base, order included");
    }
}
//#endregion 🧬️ChildLaneLaws
