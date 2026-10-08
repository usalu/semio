//! 🧪️ Exact original edge ordering and optional flag restoration.

use crate::{Puzzle2dNode, Puzzle2dSnapshot, Puzzle2dEdge};
use crate::standards::v1::subsets::any::schema::{empty_puzzle2d_snapshot,mutations::{Puzzle2dMutation,delete_node,disconnect_handles,remove_node_handle,apply_puzzle2d_mutation,inverse_puzzle2d_mutation}};
use serde_json::{Value,json};

#[test]
fn history_edit_puzzle2d_edge_restoration_preserves_original_ordinals_and_optional_flags() {
    let corpus: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("committed edge order corpus");
    for row in corpus["cases"].as_array().expect("cases") {
        let mut base = empty_puzzle2d_snapshot();
        let node: Puzzle2dNode = serde_json::from_value(json!({"id":"node","anchor":"fixed","x":3.0,"y":-4.0,"handles":[{"id":"retained-handle","angle":1.0},{"id":"handle","angle":2.0}],"text":"😀\u{0}"})).expect("native node");
        base.nodes.push(node);
        for edge in row["edges"].as_array().expect("edges") {
            let mut edge: Puzzle2dEdge = serde_json::from_value(edge.clone()).expect("native edge");
            edge.edge_kind = Some("😀\u{0}kind".into());
            edge.source_tip = Some("".into());
            edge.target_tip = Some("é".into());
            edge.gap = -3.0; edge.shift = 2.0; edge.rise = 1.0; edge.rotation = 4.0;
            edge.turn = 5.0; edge.tilt = -6.0; edge.x = 7.0; edge.y = -8.0;
            base.edges.push(edge);
        }
        let original = serde_json::to_value(&base).expect("original native state");
        let mut mutations = vec![delete_node("node".into()),remove_node_handle("node".into(),"handle".into())];
        for index in row["removed"].as_array().expect("removed indices") {
            mutations.push(disconnect_handles(base.edges.get(index.as_u64().expect("ordinal") as usize).expect("native edge").id.clone()));
        }
        for mutation in mutations {
            let inverse = inverse_puzzle2d_mutation(&base,&mutation).expect("native captured inverse");
            let mut after = base.clone();
            apply_puzzle2d_mutation(&mut after,&mutation).expect("native forward applies");
            for step in &inverse {
                let binary = protocol::OpBinary::encode_op(step).expect("native inverse binary");
                let decoded = <Puzzle2dMutation as protocol::OpBinary>::decode_op(&binary).expect("native inverse binary decodes");
                assert_eq!(&decoded,step,"literal native inverse binary roundtrip");
                apply_puzzle2d_mutation(&mut after,&decoded).expect("native inverse applies");
            }
            assert_eq!(after,base,"exact native original ordering and optional flags: {}",row["name"]);
            assert_eq!(serde_json::to_value(&base).expect("retained native state"),original,"original is unchanged");
        }
        println!("[DEBUG] Puzzle2d edge restoration native cascade={} restores complete original order and optional flags",row["name"]);
    }
    for row in corpus["insertions"].as_array().expect("insertions") {
        let mut base = empty_puzzle2d_snapshot();
        for id in row["before"].as_array().expect("before identifiers") {
            base.edges.push(serde_json::from_value(json!({"id":id,"source":"outside","target":"outside"})).expect("native retained edge"));
        }
        let mutation: Puzzle2dMutation = serde_json::from_value(json!({"mutation":"connectHandles","id":"new","source":"handle","target":"outside","edgeKind":null,"gap":0.0,"shift":0.0,"rise":0.0,"rotation":0.0,"turn":0.0,"tilt":0.0,"x":0.0,"y":0.0,"sourceTip":null,"targetTip":null,"index":row["index"]})).expect("native insertion payload");
        apply_puzzle2d_mutation(&mut base,&mutation).expect("native ordered insertion");
        assert_eq!(base.edges.iter().map(|edge|edge.id.to_string_owner()).collect::<Vec<_>>(),row["after"].as_array().expect("after identifiers").iter().map(|id|id.as_str().expect("identifier").to_owned()).collect::<Vec<_>>());
    }
    println!("[DEBUG] Puzzle2d edge restoration six insertion cases preserve append, exact ordinal, and clamped insertion semantics");
}
