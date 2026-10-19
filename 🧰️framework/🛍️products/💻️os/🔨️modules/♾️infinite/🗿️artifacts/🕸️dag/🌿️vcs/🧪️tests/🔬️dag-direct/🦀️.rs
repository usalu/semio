
//! 🧬️ Direct DAG mutation leaves: roster, wire witnesses, codecs, inverses and wire index exactness through the artifact facade.
use super::*;
use crate::os_spr::Mutation;
use crate::os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use crate::os_store::store_schema::ArtifactCommand;
use protocol::{MutationLeaf, OpBinary, OpText, SemanticMutation};
use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
use semio_framework_pack_json::Value;
use semio_framework_test_fixtures::fixture_path;

fn fixture() -> Value {
    semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🔣️mutations.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral Dag mutation fixture")
}

/// 🧾️ The leaf folder of each committed direct-leaf wire witness, in roster order.
const WITNESS_LEAVES: [&str; 14] = [
    "➕️create-node",
    "🗑️delete-node",
    "✏️rename-node",
    "🔤️change-node-name",
    "↔️move-node",
    "📐️resize-node",
    "🖼️change-node-icon",
    "🔡️change-node-abbreviation",
    "🧮️change-node-operator-kind",
    "🔁️replace-node-kind",
    "🗃️replace-node-properties",
    "🔀️reorder-nodes",
    "🔗️connect-nodes",
    "✂️disconnect-nodes",
];

/// 📖️ Reads one committed fixture below `🌿️vcs` at run time, so the test closure counts none of them as source inputs.
pub(crate) fn read_vcs_fixture(relative: &str) -> String {
    let path = fixture_path!("../../🌿️vcs").join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("committed fixture {}: {error}", path.display()))
}

fn witness_text(index: usize) -> String {
    read_vcs_fixture(&format!("🧪️tests/🧫️fixtures/🧬️mutations/{}/🧾️wire-witness/🦠️mutation/🔣️.json", WITNESS_LEAVES[index]))
}

fn witness(index: usize) -> Value {
    semio_framework_pack_json::parse(&witness_text(index), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed Dag wire witness")
}

fn witness_payload(index: usize) -> Value {
    Value::Object(witness(index).as_object().expect("wire object").iter().filter(|(key, _)| *key != "operation").map(|(key, value)| (key.to_string(), value.clone())).collect())
}

/// 🌉️ `T: FromValue` decode of a pack JSON [`Value`] — the in-house `serde_json::from_value` analog.
fn from_pack_value<T: semio_framework_value::FromValue>(value: &Value) -> Result<T, semio_framework_value::ValueError> {
    <T as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value))
}

/// 🌉️ `T: ToValue` encode into a pack JSON [`Value`] — the in-house `serde_json::to_value` analog.
fn to_pack_value<T: semio_framework_value::ToValue>(value: &T) -> Value {
    semio_framework_pack_json::from_dsl_value(&<T as semio_framework_value::ToValue>::to_value(value))
}

pub(crate) fn node(id: &str) -> DagNodeSpec {
    DagNodeSpec { id: id.into(), name: id.into(), ..Default::default() }
}

pub(crate) fn base() -> DagSnapshot {
    DagSnapshot {
        schema: DAG_DOCUMENT_SCHEMA.into(),
        nodes: vec![node("a"), node("b"), node("c")],
        edges: vec![
            DagHostSnapshotEdge { id: "e".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() },
            DagHostSnapshotEdge { id: "keep".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() },
            DagHostSnapshotEdge { id: "last".into(), source: "c@out".into(), target: "a@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::from([("weight".into(), PropertyValue::Number(2.0))]) },
        ],
    }
}

fn apply(base: &DagSnapshot, mutation: &DagMutation) -> DagSnapshot {
    protocol::apply_diff_cold(mutation.diff(base).diff(), base).expect("valid direct Dag mutation")
}

fn assert_codecs(mutation: &DagMutation) {
    let json = semio_framework_pack_json::to_json_string(mutation);
    assert_eq!(semio_framework_pack_json::from_json_str::<DagMutation>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("deserialize direct mutation"), *mutation);
    let text = mutation.print_op();
    assert!(text.starts_with(mutation.descriptor().text_opcode.expect("text opcode")));
    assert_eq!(DagMutation::parse_op(&text).expect("direct text decode"), *mutation);
    let bytes = mutation.encode_op().expect("direct binary encode");
    assert_eq!(bytes[0], dsl::variants_binary::OP_BINARY_FORMAT);
    assert_eq!(u32::from(bytes[1]), mutation.descriptor().binary_tag.expect("binary tag"));
    assert_eq!(DagMutation::decode_op(&bytes).expect("direct binary decode"), *mutation);
}

/// 🧾️ The mutation the committed wire witness of roster row `index` decodes to.
pub(crate) fn witness_mutation(index: usize) -> DagMutation {
    from_pack_value::<DagMutation>(&witness(index)).expect("committed Dag wire witness")
}

pub(crate) fn assert_leaf_contract<T>(index: usize, wrap: fn(T) -> DagMutation, descriptor: &str)
where
    T: MutationLeaf + semio_framework_value::ToValue + semio_framework_value::FromValue,
{
    let wire = witness(index);
    let payload_value = witness_payload(index);
    let payload = from_pack_value::<T>(&payload_value).expect("neutral direct payload");
    let mutation = wrap(payload);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&T::DESCRIPTOR)).expect("descriptor JSON"), serde_json::from_str::<serde_json::Value>(descriptor).expect("owned descriptor"));
    assert_eq!(mutation.descriptor(), &T::DESCRIPTOR);
    assert_eq!(mutation.descriptor().binary_tag, Some(u32::try_from(index).expect("small roster index")));
    assert_eq!(to_pack_value(&mutation)["operation"], wire["operation"]);
    let mut unknown_payload = payload_value.clone();
    if let Some(object) = unknown_payload.as_object_mut() {
        object.insert("unknown".to_string(), Value::from(true));
    }
    assert!(from_pack_value::<T>(&unknown_payload).is_err());
    let mut unknown_operation = to_pack_value(&mutation);
    if let Some(object) = unknown_operation.as_object_mut() {
        object.insert("unknown".to_string(), Value::from(true));
    }
    assert!(from_pack_value::<DagMutation>(&unknown_operation).is_err());
    let payload_object = payload_value.as_object().expect("payload object");
    let payload_keys: Vec<String> = payload_object.iter().map(|(key, _)| key.to_string()).filter(|key| key != "newOperatorKind").collect();
    for key in payload_keys {
        let missing: Value = Value::Object(payload_object.iter().filter(|(k, _)| *k != key).map(|(k, v)| (k.to_string(), v.clone())).collect());
        assert!(from_pack_value::<T>(&missing).is_err(), "missing {key}");
        let mut missing_aggregate = missing;
        if let Some(object) = missing_aggregate.as_object_mut() {
            object.insert("operation".to_string(), wire["operation"].clone());
        }
        assert!(from_pack_value::<DagMutation>(&missing_aggregate).is_err(), "missing aggregate {key}");
    }
    assert_codecs(&mutation);
    let before = base();
    let mut restored = apply(&before, &mutation);
    let inverse = mutation.inverse(&before).expect("valid retained mutation inverse fixture");
    assert!(!inverse.is_empty());
    for inverse in inverse.into_iter().rev() {
        restored = apply(&restored, &inverse);
    }
    assert_eq!(restored, before);
}

#[test]
fn direct_leaf_roster_and_codec_contracts() {
    let fixture = fixture();
    assert_eq!(DagMutation::kinds().len(), 14);
    assert_eq!(<DagMutation as Mutation<DagSnapshot>>::DESCRIPTORS.len(), 14);
    for index in 0..WITNESS_LEAVES.len() {
        let mutation = from_pack_value::<DagMutation>(&witness(index)).expect("committed aggregate wire witness");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation)).expect("wire JSON"), serde_json::from_str::<serde_json::Value>(&witness_text(index)).expect("witness JSON"), "the committed wire witness is the canonical Rust wire");
        assert_eq!(mutation.descriptor().binary_tag, Some(u32::try_from(index).expect("small index")));
        assert_eq!(mutation.descriptor().diff_participation, protocol::MutationDiffParticipation::ApplyOnly);
        assert_codecs(&mutation);
    }
    for row in fixture["invalid"].as_array().expect("invalid vectors") {
        let mut json = row["payload"].clone();
        if let Some(object) = json.as_object_mut() {
            object.insert("operation".to_string(), row["operation"].clone());
        }
        assert!(from_pack_value::<DagMutation>(&json).is_err(), "{}", row["name"]);
    }
    for row in fixture["additionalValid"].as_array().expect("additional input vectors") {
        let mut json = row["payload"].clone();
        if let Some(object) = json.as_object_mut() {
            object.insert("operation".to_string(), row["operation"].clone());
        }
        assert_codecs(&from_pack_value::<DagMutation>(&json).expect("additional pack-value input"));
    }
}

#[test]
fn direct_delete_inverse_declares_descending_edges_before_node() {
    let mut before = base();
    before.edges.push(DagHostSnapshotEdge { id: "loop".into(), source: "a@out".into(), target: "a@in".into(), ..Default::default() });
    let mutation = DagMutation::DeleteNode(DeleteNode { id: "a".into() });
    let inverse = mutation.inverse(&before).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 4);
    assert!(matches!(&inverse[0], DagMutation::ConnectNodes(value) if value.id == "loop" && value.index == 3));
    assert!(matches!(&inverse[1], DagMutation::ConnectNodes(value) if value.id == "last" && value.index == 2));
    assert!(matches!(&inverse[2], DagMutation::ConnectNodes(value) if value.id == "e" && value.index == 0));
    assert!(matches!(&inverse[3], DagMutation::CreateNode(value) if value.node.id == "a" && value.index == 0));
    let mut restored = apply(&before, &mutation);
    for inverse in inverse.into_iter().rev() {
        restored = apply(&restored, &inverse);
    }
    assert_eq!(restored, before);
}

#[semio_framework_async_macros::async_test]
async fn direct_store_undo_restores_incident_edge_order() {
    let before = base();
    let mut store = create_dag_store("dag", before.clone(), opened_dag_test_actor()).await.expect("store");
    crate::os_store::test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DagMutation::DeleteNode(DeleteNode { id: "a".into() })], transaction: None }).await.expect("delete");
    crate::os_store::test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("restored projection"), before);
    crate::os_store::test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo");
    assert_eq!(store.snapshot().expect("deleted projection"), apply(&before, &DagMutation::DeleteNode(DeleteNode { id: "a".into() })));
    close_dag_test_store(store);
}

#[test]
fn direct_disconnect_inverse_preserves_nonfinal_position() {
    let before = base();
    let mutation = DagMutation::DisconnectNodes(DisconnectNodes { id: "keep".into() });
    let inverse = mutation.inverse(&before).expect("valid retained mutation inverse fixture");
    assert!(matches!(&inverse[0], DagMutation::ConnectNodes(value) if value.id == "keep" && value.index == 1));
    assert_eq!(apply(&apply(&before, &mutation), &inverse[0]), before);
}

/// 🔀️ L1/L2: a reorder is positional `{id, from, to}` rows read from the base order — never an order list — and inverts to the reversed moves.
#[test]
fn direct_reorder_diff_is_positional_rows_and_inverts_to_the_reversed_moves() {
    let before = base();
    let mutation = DagMutation::ReorderNodes(ReorderNodes { moves: vec![NodeMove { id: "c".into(), to: 0 }, NodeMove { id: "a".into(), to: 2 }] });
    let (diff, _) = mutation.diff(&before).into_parts();
    assert_eq!(diff.steps[0].reordered_nodes, Some(vec![ReorderedNode { id: "c".into(), from: 2, to: 0 }, ReorderedNode { id: "a".into(), from: 1, to: 2 }]));
    let json = semio_framework_pack_json::to_json_string(&diff);
    assert!(json.contains("\"reorderedNodes\"") && json.contains("\"from\"") && !json.contains("\"order\""), "{json}");
    assert_eq!(mutation.inverse(&before).expect("valid retained mutation inverse fixture"), vec![DagMutation::ReorderNodes(ReorderNodes { moves: vec![NodeMove { id: "a".into(), to: 1 }, NodeMove { id: "c".into(), to: 2 }] })]);
    let after = protocol::apply_diff_cold(&diff, &before).expect("reorder applies");
    assert_eq!(after.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), ["c", "b", "a"]);
    let negative = DiffAlgebra::<DagSnapshot>::inverse(&diff, &before);
    assert_eq!(protocol::apply_diff_cold(&negative, &after).expect("negative applies"), before);
    for (rows, code) in [
        (vec![ReorderedNode { id: "b".into(), from: 0, to: 1 }], "mutation.apply.order-mismatch"),
        (vec![ReorderedNode { id: "a".into(), from: 7, to: 0 }], "mutation.apply.missing-target"),
        (vec![ReorderedNode { id: "a".into(), from: 0, to: 3 }], "mutation.apply.invalid-index"),
    ] {
        assert_eq!(protocol::apply_diff_cold(&DagDiff::from(DagDelta { reordered_nodes: Some(rows), ..Default::default() }), &before).expect_err("invalid reorder row").code, code);
    }
    let missing = DagMutation::ReorderNodes(ReorderNodes { moves: vec![NodeMove { id: "absent".into(), to: 0 }] });
    assert!(protocol::apply_diff_cold(missing.diff(&before).diff(), &before).is_err());
    assert!(missing.inverse(&before).expect("valid retained mutation inverse fixture").is_empty());
}

/// 🗑️ L2/L3: the negative of a delete restores the node first and then each severed edge at its base position, ascending.
#[test]
fn direct_delete_negative_diff_restores_the_node_before_its_edges() {
    let before = base();
    let mutation = DagMutation::DeleteNode(DeleteNode { id: "a".into() });
    let (diff, _) = mutation.diff(&before).into_parts();
    let after = protocol::apply_diff_cold(&diff, &before).expect("delete applies");
    assert_eq!(after.edges.iter().map(|edge| edge.id.as_str()).collect::<Vec<_>>(), ["keep"]);
    let negative = DiffAlgebra::<DagSnapshot>::inverse(&diff, &before);
    assert_eq!(negative.steps.len(), 3);
    assert_eq!(negative.steps[0].created_node_at, Some(0));
    assert_eq!(negative.steps[1].connected_edge_at, Some(0));
    assert_eq!(negative.steps[2].connected_edge_at, Some(2));
    assert_eq!(protocol::apply_diff_cold(&negative, &after).expect("negative applies"), before);
}

/// ✏️ L2/L3: the negative of a rename is ONE delta — the rename back together with the endpoint cascade — because the cascade names the id the rename introduces.
#[test]
fn direct_rename_negative_diff_is_one_delta_with_the_cascade() {
    let before = base();
    let mutation = DagMutation::RenameNode(RenameNode { id: "a".into(), new_id: "z".into() });
    let (diff, _) = mutation.diff(&before).into_parts();
    let after = protocol::apply_diff_cold(&diff, &before).expect("rename applies");
    let negative = DiffAlgebra::<DagSnapshot>::inverse(&diff, &before);
    assert_eq!(negative.steps.len(), 1);
    assert_eq!(protocol::apply_diff_cold(&negative, &after).expect("negative applies"), before);
}

#[test]
fn direct_rename_preserves_exact_endpoint_suffix() {
    let fixture = fixture();
    for row in fixture["endpointRenames"].as_array().expect("endpoint vectors") {
        let id = row["id"].as_str().expect("id");
        let new_id = row["newId"].as_str().expect("new ID");
        let source = row["source"].as_str().expect("source");
        let expected = row["expected"].as_str().expect("expected endpoint");
        let before = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: vec![node(id), node("b")], edges: vec![DagHostSnapshotEdge { id: "edge".into(), source: source.into(), target: "b@in".into(), ..Default::default() }] };
        let mutation = DagMutation::RenameNode(RenameNode { id: id.into(), new_id: new_id.into() });
        let after = apply(&before, &mutation);
        assert_eq!(after.edges[0].source, expected);
        assert_eq!(apply(&after, &mutation.inverse(&before).expect("valid retained mutation inverse fixture")[0]), before);
    }
}

#[test]
fn direct_structural_absorb_is_associative_and_preserves_rejection() {
    let fixture = fixture();
    for row in fixture["algebra"].as_array().expect("algebra vectors") {
        let before = base();
        let mut after = before.clone();
        let mut diffs = Vec::new();
        for value in row["mutations"].as_array().expect("mutation sequence") {
            let mutation = from_pack_value::<DagMutation>(value).expect("sequence mutation");
            let (diff, _) = mutation.diff(&after).into_parts();
            after = protocol::apply_diff_cold(&diff, &after).expect("sequential diff");
            diffs.push(diff);
        }
        let mut left = DagDiff::default();
        for diff in &diffs {
            left.absorb(diff.clone());
        }
        let mut right = DagDiff::default();
        for mut diff in diffs.into_iter().rev() {
            diff.absorb(right);
            right = diff;
        }
        assert_eq!(left, right, "{}", row["name"]);
        assert_eq!(protocol::apply_diff_cold(&left, &before).expect("absorbed diff"), after, "{}", row["name"]);
        assert_eq!(to_pack_value(&after.nodes.iter().map(|node| node.id.clone()).collect::<Vec<_>>()), row["nodeOrder"]);
        assert_eq!(to_pack_value(&after.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>()), row["edgeOrder"]);
        for (id, x) in row["x"].as_object().expect("expected positions") {
            assert_eq!(after.nodes.iter().find(|node| node.id == id).expect("position target").x, x.as_f64().expect("x"));
        }
        assert_eq!(semio_framework_pack_json::from_json_str::<DagDiff>(&semio_framework_pack_json::to_json_string(&left), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("diff decode"), left);
    }
    let before = base();
    let mut rejected = DagDiff::from(DagDelta { created_node: Some(node("x")), created_node_at: Some(u64::MAX), ..Default::default() });
    rejected.absorb(DagMutation::MoveNode(MoveNode { id: "a".into(), x: 3.0, y: 4.0 }).diff(&before).into_parts().0);
    assert_eq!(protocol::apply_diff_cold(&rejected, &before).expect_err("rejection survives composition").code, "mutation.apply.invalid-index");
    assert_eq!(before, base());
    assert_eq!(protocol::apply_diff_cold(&DagDiff::from(DagDelta { connected_edge: Some(before.edges[0].clone()), ..Default::default() }), &before).expect_err("unpaired edge index").code, "mutation.apply.incomplete-diff");
}

#[test]
fn direct_wire_indices_are_exact_and_apply_rejects_out_of_range() {
    let before = base();
    assert_eq!(dag_index_to_wire(usize::MAX), u64::try_from(usize::MAX).expect("guarded native width"));
    if usize::BITS < u64::BITS {
        assert_eq!(dag_index_from_wire(u64::MAX).expect_err("narrow native width").code, "mutation.apply.invalid-index");
    }
    for mutation in [
        DagMutation::CreateNode(CreateNode { node: node("x"), index: u64::MAX }),
        DagMutation::ConnectNodes(ConnectNodes { id: "x".into(), source: "a@out".into(), target: "b@in".into(), route_style: EdgeRouteStyle::Bezier, properties: PropertyBag::new(), index: u64::MAX }),
    ] {
        assert_codecs(&mutation);
        assert!(semio_framework_pack_json::to_json_string(&mutation).contains("18446744073709551615"));
        assert!(mutation.print_op().contains("18446744073709551615"));
        assert_eq!(protocol::apply_diff_cold(mutation.diff(&before).diff(), &before).expect_err("out of range").code, "mutation.apply.invalid-index");
        let json = semio_framework_pack_json::to_json_string(&mutation);
        for invalid in ["18446744073709551616", "-1", "0.5", "1e21", "null", "\"1\""] {
            assert!(semio_framework_pack_json::from_json_str::<DagMutation>(&json.replace("18446744073709551615", invalid), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{invalid}");
        }
    }
}

#[test]
fn direct_intrinsic_serde_and_selection_are_lossless() {
    let fixture = fixture();
    for row in fixture["nodeKinds"].as_array().expect("node kind vectors") {
        let value = row["value"].clone();
        assert_eq!(from_pack_value::<DagNodeKind>(&value).is_ok(), row["valid"].as_bool().expect("expected validity"), "{}", row["name"]);
        if row["valid"] == true {
            let kind = from_pack_value::<DagNodeKind>(&value).expect("kind");
            assert_codecs(&DagMutation::ReplaceNodeKind(ReplaceNodeKind { id: "a".into(), new_kind: kind }));
        }
    }
    let mut selected = node("select");
    selected.kind = DagNodeKind::Select { options: vec!["a".into(), "b".into()], selected: u64::MAX, output: IoPortSpec::simple("out", "Output") };
    assert_codecs(&DagMutation::CreateNode(CreateNode { node: selected.clone(), index: 0 }));
    assert_eq!(advance_select_option(&mut selected).as_deref(), Some("a"));
    selected.kind = DagNodeKind::Select { options: vec![], selected: u64::MAX, output: IoPortSpec::simple("out", "Output") };
    assert_eq!(advance_select_option(&mut selected), None);
    let mut app = node("app");
    app.icon = "node-icon".into();
    app.kind = DagNodeKind::AppInstance { instance_id: "instance".into(), plugin_id: "plugin".into(), app_id: "app".into(), icon: "app-icon".into(), inputs: vec![], outputs: vec![] };
    let encoded = semio_framework_pack_json::to_json_string(&app);
    assert_eq!(encoded.matches("\"icon\":").count(), 1);
    assert_eq!(encoded.matches("\"appIcon\":").count(), 1);
    assert_eq!(semio_framework_pack_json::from_json_str::<DagNodeSpec>(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("app round trip"), app);
    assert_codecs(&DagMutation::CreateNode(CreateNode { node: app, index: 0 }));
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn move_node_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(4)];
    cases.extend([
        DagMutation::MoveNode(MoveNode { id: "b".into(), x: 5.0, y: -6.5 }),
        DagMutation::MoveNode(MoveNode { id: "c".into(), x: 1.0, y: 2.0 }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn disconnect_nodes_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(13)];
    cases.extend([
        DagMutation::DisconnectNodes(DisconnectNodes { id: "keep".into() }),
        DagMutation::DisconnectNodes(DisconnectNodes { id: "last".into() }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn rename_node_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(2)];
    cases.extend([
        DagMutation::RenameNode(RenameNode { id: "b".into(), new_id: "x".into() }),
        DagMutation::RenameNode(RenameNode { id: "c".into(), new_id: "x".into() }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn create_node_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(0)];
    cases.extend([
        DagMutation::CreateNode(CreateNode { node: self::node("x"), index: 0 }),
        DagMutation::CreateNode(CreateNode { node: self::node("x"), index: 1 }),
        DagMutation::CreateNode(CreateNode { node: self::node("x"), index: 3 }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn resize_node_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(5)];
    cases.extend([
        DagMutation::ResizeNode(ResizeNode { id: "b".into(), width: 90.0, height: 33.0 }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn reorder_nodes_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(11)];
    cases.extend([
        DagMutation::ReorderNodes(ReorderNodes { moves: vec![NodeMove { id: "b".into(), to: 0 }] }),
        DagMutation::ReorderNodes(ReorderNodes { moves: vec![NodeMove { id: "b".into(), to: 2 }] }),
        DagMutation::ReorderNodes(ReorderNodes { moves: vec![NodeMove { id: "a".into(), to: 1 }, NodeMove { id: "a".into(), to: 2 }, NodeMove { id: "c".into(), to: 1 }] }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn replace_node_kind_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(9)];
    cases.extend([
        DagMutation::ReplaceNodeKind(ReplaceNodeKind { id: "b".into(), new_kind: DagNodeKind::Note { text: "middle".into(), output: crate::snapshot::IoPortSpec::default() } }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn connect_nodes_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(12)];
    cases.extend([
        DagMutation::ConnectNodes(ConnectNodes { id: "x".into(), source: "a@out".into(), target: "c@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::new(), index: 0 }),
        DagMutation::ConnectNodes(ConnectNodes { id: "x".into(), source: "a@out".into(), target: "c@in".into(), route_style: EdgeRouteStyle::Bezier, properties: PropertyBag::new(), index: 2 }),
        DagMutation::ConnectNodes(ConnectNodes { id: "x".into(), source: "a@out".into(), target: "c@in".into(), route_style: EdgeRouteStyle::Bezier, properties: PropertyBag::new(), index: 3 }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn change_node_abbreviation_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(7)];
    cases.extend([
        DagMutation::ChangeNodeAbbreviation(ChangeNodeAbbreviation { id: "b".into(), new_abbreviation: "B".into() }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn change_node_name_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(3)];
    cases.extend([
        DagMutation::ChangeNodeName(ChangeNodeName { id: "b".into(), new_name: "Middle".into() }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn change_node_icon_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(6)];
    cases.extend([
        DagMutation::ChangeNodeIcon(ChangeNodeIcon { id: "b".into(), new_icon: "square".into() }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn replace_node_properties_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(10)];
    cases.extend([
        DagMutation::ReplaceNodeProperties(ReplaceNodeProperties { id: "b".into(), new_properties: PropertyBag::from([("weight".into(), semio_framework_graph_manifest::PropertyValue::Number(3.0))]) }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn delete_node_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(1)];
    cases.extend([
        DagMutation::DeleteNode(DeleteNode { id: "b".into() }),
        DagMutation::DeleteNode(DeleteNode { id: "c".into() }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn change_node_operator_kind_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![self::witness_mutation(8)];
    cases.extend([
        DagMutation::ChangeNodeOperatorKind(ChangeNodeOperatorKind { id: "b".into(), new_operator_kind: Some("math.add".into()) }),
    ]);
    for mutation in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &self::base()).await;
    }
}
