
use super::*;
use protocol::{MutationLeaf, OpBinary, OpText, SemanticMutation};

fn fixture() -> Value {
    dsl::os_pack::json::parse(include_str!("../../🧪️fixtures/🔣️mutations.json")).expect("neutral Dag mutation fixture")
}

/// 🌉️ `T: FromValue` decode of a pack JSON [`Value`] — the in-house `serde_json::from_value` analog.
fn from_pack_value<T: dsl::FromValue>(value: Value) -> Result<T, dsl::ValueError> {
    <T as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&value))
}

/// 🌉️ `T: ToValue` encode into a pack JSON [`Value`] — the in-house `serde_json::to_value` analog.
fn to_pack_value<T: dsl::ToValue>(value: &T) -> Value {
    dsl::os_pack::json::from_dsl_value(&<T as dsl::ToValue>::to_value(value))
}

fn node(id: &str) -> DagNodeSpec {
    DagNodeSpec { id: id.into(), name: id.into(), ..Default::default() }
}

fn base() -> DagSnapshot {
    DagSnapshot {
        schema: DAG_DOCUMENT_SCHEMA.into(),
        nodes: vec![node("a"), node("b"), node("c")],
        edges: vec![
            DagFixtureEdge { id: "e".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() },
            DagFixtureEdge { id: "keep".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() },
            DagFixtureEdge { id: "last".into(), source: "c@out".into(), target: "a@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::from([("weight".into(), PropertyValue::Number(2.0))]) },
        ],
    }
}

fn apply(base: &DagSnapshot, mutation: &DagMutation) -> DagSnapshot {
    mutation.diff(base).diff().apply(base).expect("valid direct Dag mutation")
}

fn assert_codecs(mutation: &DagMutation) {
    let json = dsl::os_pack::json::to_json_string(mutation);
    assert_eq!(dsl::os_pack::json::from_json_str::<DagMutation>(&json).expect("deserialize direct mutation"), *mutation);
    let text = mutation.print_op();
    assert!(text.starts_with(mutation.descriptor().text_opcode.expect("text opcode")));
    assert_eq!(DagMutation::parse_op(&text).expect("direct text decode"), *mutation);
    let bytes = mutation.encode_op().expect("direct binary encode");
    assert_eq!(bytes[0], dsl::variants_binary::OP_BINARY_FORMAT);
    assert_eq!(u32::from(bytes[1]), mutation.descriptor().binary_tag.expect("binary tag"));
    assert_eq!(DagMutation::decode_op(&bytes).expect("direct binary decode"), *mutation);
}

pub(crate) fn assert_leaf_contract<T>(index: usize, wrap: fn(T) -> DagMutation, descriptor: &str)
where
    T: MutationLeaf + dsl::ToValue + dsl::FromValue,
{
    let fixture = fixture();
    let row = &fixture["valid"][index];
    let payload = from_pack_value::<T>(row["payload"].clone()).expect("neutral direct payload");
    let mutation = wrap(payload);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&T::DESCRIPTOR)).expect("descriptor JSON"), serde_json::from_str::<serde_json::Value>(descriptor).expect("owned descriptor"));
    assert_eq!(mutation.descriptor(), &T::DESCRIPTOR);
    assert_eq!(mutation.descriptor().binary_tag, Some(u32::try_from(index).expect("small roster index")));
    assert_eq!(to_pack_value(&mutation)["operation"], row["operation"]);
    let mut unknown_payload = row["payload"].clone();
    if let Some(object) = unknown_payload.as_object_mut() {
        object.insert("unknown".to_string(), Value::from(true));
    }
    assert!(from_pack_value::<T>(unknown_payload).is_err());
    let mut unknown_operation = to_pack_value(&mutation);
    if let Some(object) = unknown_operation.as_object_mut() {
        object.insert("unknown".to_string(), Value::from(true));
    }
    assert!(from_pack_value::<DagMutation>(unknown_operation).is_err());
    let payload_object = row["payload"].as_object().expect("payload object");
    let payload_keys: Vec<String> = payload_object.iter().map(|(key, _)| key.to_string()).filter(|key| key != "newOperatorKind").collect();
    for key in payload_keys {
        let missing: Value = Value::Object(payload_object.iter().filter(|(k, _)| *k != key).map(|(k, v)| (k.to_string(), v.clone())).collect());
        assert!(from_pack_value::<T>(missing.clone()).is_err(), "missing {key}");
        let mut missing_aggregate = missing;
        if let Some(object) = missing_aggregate.as_object_mut() {
            object.insert("operation".to_string(), row["operation"].clone());
        }
        assert!(from_pack_value::<DagMutation>(missing_aggregate).is_err(), "missing aggregate {key}");
    }
    assert_codecs(&mutation);
    let before = base();
    let mut restored = apply(&before, &mutation);
    let inverse = mutation.inverse(&before);
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
    for (index, row) in fixture["valid"].as_array().expect("valid vectors").iter().enumerate() {
        let mut json = row["payload"].clone();
        if let Some(object) = json.as_object_mut() {
            object.insert("operation".to_string(), row["operation"].clone());
        }
        let mutation = from_pack_value::<DagMutation>(json).expect("neutral aggregate");
        assert_eq!(mutation.descriptor().binary_tag, Some(u32::try_from(index).expect("small index")));
        assert_eq!(mutation.descriptor().diff_participation, protocol::MutationDiffParticipation::ApplyOnly);
        assert_codecs(&mutation);
    }
    for row in fixture["invalid"].as_array().expect("invalid vectors") {
        let mut json = row["payload"].clone();
        if let Some(object) = json.as_object_mut() {
            object.insert("operation".to_string(), row["operation"].clone());
        }
        assert!(from_pack_value::<DagMutation>(json).is_err(), "{}", row["name"]);
    }
    for row in fixture["additionalValid"].as_array().expect("additional input vectors") {
        let mut json = row["payload"].clone();
        if let Some(object) = json.as_object_mut() {
            object.insert("operation".to_string(), row["operation"].clone());
        }
        assert_codecs(&from_pack_value::<DagMutation>(json).expect("additional pack-value input"));
    }
}

#[test]
fn direct_delete_inverse_declares_descending_edges_before_node() {
    let mut before = base();
    before.edges.push(DagFixtureEdge { id: "loop".into(), source: "a@out".into(), target: "a@in".into(), ..Default::default() });
    let mutation = DagMutation::DeleteNode(DeleteNode { id: "a".into() });
    let inverse = mutation.inverse(&before);
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
    let mut store = create_dag_store("dag", before.clone()).await.expect("store");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DagMutation::DeleteNode(DeleteNode { id: "a".into() })], description: None }).await.expect("delete");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("restored projection"), before);
    store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    assert_eq!(store.snapshot().expect("deleted projection"), apply(&before, &DagMutation::DeleteNode(DeleteNode { id: "a".into() })));
    close_dag_test_store(store);
}

#[test]
fn direct_disconnect_inverse_preserves_nonfinal_position() {
    let before = base();
    let mutation = DagMutation::DisconnectNodes(DisconnectNodes { id: "keep".into() });
    let inverse = mutation.inverse(&before);
    assert!(matches!(&inverse[0], DagMutation::ConnectNodes(value) if value.id == "keep" && value.index == 1));
    assert_eq!(apply(&apply(&before, &mutation), &inverse[0]), before);
}

#[test]
fn direct_rename_preserves_exact_endpoint_suffix() {
    let fixture = fixture();
    for row in fixture["endpointRenames"].as_array().expect("endpoint vectors") {
        let id = row["id"].as_str().expect("id");
        let new_id = row["newId"].as_str().expect("new ID");
        let source = row["source"].as_str().expect("source");
        let expected = row["expected"].as_str().expect("expected endpoint");
        let before = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: vec![node(id), node("b")], edges: vec![DagFixtureEdge { id: "edge".into(), source: source.into(), target: "b@in".into(), ..Default::default() }] };
        let mutation = DagMutation::RenameNode(RenameNode { id: id.into(), new_id: new_id.into() });
        let after = apply(&before, &mutation);
        assert_eq!(after.edges[0].source, expected);
        assert_eq!(apply(&after, &mutation.inverse(&before)[0]), before);
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
            let mutation = from_pack_value::<DagMutation>(value.clone()).expect("sequence mutation");
            let (diff, _) = mutation.diff(&after).into_parts();
            after = diff.apply(&after).expect("sequential diff");
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
        assert_eq!(left.apply(&before).expect("absorbed diff"), after, "{}", row["name"]);
        assert_eq!(to_pack_value(&after.nodes.iter().map(|node| node.id.clone()).collect::<Vec<_>>()), row["nodeOrder"]);
        assert_eq!(to_pack_value(&after.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>()), row["edgeOrder"]);
        for (id, x) in row["x"].as_object().expect("expected positions") {
            assert_eq!(after.nodes.iter().find(|node| node.id == id).expect("position target").x, x.as_f64().expect("x"));
        }
        assert_eq!(dsl::os_pack::json::from_json_str::<DagDiff>(&dsl::os_pack::json::to_json_string(&left)).expect("diff decode"), left);
    }
    let before = base();
    let mut rejected = DagDiff::from(DagDelta { created_node: Some(node("x")), created_node_at: Some(u64::MAX), ..Default::default() });
    rejected.absorb(DagMutation::MoveNode(MoveNode { id: "a".into(), x: 3.0, y: 4.0 }).diff(&before).into_parts().0);
    assert_eq!(rejected.apply(&before).expect_err("rejection survives composition").code, "mutation.apply.invalid-index");
    assert_eq!(before, base());
    assert_eq!(DagDiff::from(DagDelta { connected_edge: Some(before.edges[0].clone()), ..Default::default() }).apply(&before).expect_err("unpaired edge index").code, "mutation.apply.incomplete-diff");
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
        assert!(dsl::os_pack::json::to_json_string(&mutation).contains("18446744073709551615"));
        assert!(mutation.print_op().contains("18446744073709551615"));
        assert_eq!(mutation.diff(&before).diff().apply(&before).expect_err("out of range").code, "mutation.apply.invalid-index");
        let json = dsl::os_pack::json::to_json_string(&mutation);
        for invalid in ["18446744073709551616", "-1", "0.5", "1e21", "null", "\"1\""] {
            assert!(dsl::os_pack::json::from_json_str::<DagMutation>(&json.replace("18446744073709551615", invalid)).is_err(), "{invalid}");
        }
    }
}

#[test]
fn direct_intrinsic_serde_and_selection_are_lossless() {
    let fixture = fixture();
    for row in fixture["nodeKinds"].as_array().expect("node kind vectors") {
        let value = row["value"].clone();
        assert_eq!(from_pack_value::<DagNodeKind>(value.clone()).is_ok(), row["valid"].as_bool().expect("expected validity"), "{}", row["name"]);
        if row["valid"] == true {
            let kind = from_pack_value::<DagNodeKind>(value).expect("kind");
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
    let encoded = dsl::os_pack::json::to_json_string(&app);
    assert_eq!(encoded.matches("\"icon\":").count(), 1);
    assert_eq!(encoded.matches("\"appIcon\":").count(), 1);
    assert_eq!(dsl::os_pack::json::from_json_str::<DagNodeSpec>(&encoded).expect("app round trip"), app);
    assert_codecs(&DagMutation::CreateNode(CreateNode { node: app, index: 0 }));
}
