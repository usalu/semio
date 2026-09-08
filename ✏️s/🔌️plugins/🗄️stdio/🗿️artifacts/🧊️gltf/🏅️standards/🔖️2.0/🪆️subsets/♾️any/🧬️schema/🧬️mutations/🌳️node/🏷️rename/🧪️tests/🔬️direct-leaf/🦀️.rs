
use super::*;
use protocol::{Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary, OpText};

fn value(entries: impl IntoIterator<Item = (&'static str, DslValue)>) -> DslValue {
    DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), value)))
}

fn snapshot(name: Option<&str>) -> GltfSnapshot {
    let name = name.map(|value| format!(r#""name":{value:?}"#)).unwrap_or_default();
    serde_json::from_str(&format!(r#"{{"schema":"stdio.gltf","document":{{"asset":{{"version":"2.0"}},"nodes":[{{{}}}]}},"buffers":[],"sourceForm":"json"}}"#, name)).expect("minimal glTF snapshot decodes")
}

fn apply_mutation(base: &GltfSnapshot, mutation: &super::super::GltfMutation) -> GltfSnapshot {
    let outcome = <super::super::GltfMutation as Mutation<GltfSnapshot>>::diff(mutation, base);
    assert!(outcome.messages().is_empty(), "mutation must apply: {:?}", outcome.messages());
    <crate::schema::diff::GltfDiff as MutationDiff<GltfSnapshot>>::apply(outcome.diff(), base).expect("aggregate diff applies")
}

#[test]
fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("valid canonical node-name descriptor");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&(<ChangeNodeNameMutation as MutationLeaf>::DESCRIPTOR))).expect("serializable descriptor"), expected);
    let provenance = <ChangeNodeNameMutation as MutationLeaf>::PROVENANCE;
    assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations");
    assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename");
    assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/🦀️.rs");
    assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/🔣️.json");
    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
    assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
}

#[test]
fn concrete_inverse_round_trips_names_and_actual_aggregate_codecs() {
    let base = snapshot(Some("Root"));
    let mutation = super::super::GltfMutation::ChangeNodeName(ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 0, value: Some("Pivot".into()) }));
    let inverse = <super::super::GltfMutation as Mutation<GltfSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse, vec![super::super::GltfMutation::ChangeNodeName(ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node: 0, before: Some("Root".into()), after: Some("Pivot".into()) }))]);
    let mut current = apply_mutation(&base, &mutation);
    assert_eq!(current.document.nodes[0].name.as_deref(), Some("Pivot"));
    assert_eq!(super::super::GltfMutation::parse_op(&mutation.print_op()).expect("text codec decodes"), mutation);
    assert_eq!(super::super::GltfMutation::decode_op(&mutation.encode_op().expect("binary codec encodes")).expect("binary codec decodes"), mutation);
    assert_eq!(super::super::GltfMutation::parse_op(&inverse[0].print_op()).expect("inverse text codec decodes"), inverse[0]);
    assert_eq!(super::super::GltfMutation::decode_op(&inverse[0].encode_op().expect("inverse binary codec encodes")).expect("inverse binary codec decodes"), inverse[0]);
    let redo = <super::super::GltfMutation as Mutation<GltfSnapshot>>::inverse(&inverse[0], &current);
    assert_eq!(redo, vec![super::super::GltfMutation::ChangeNodeName(ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node: 0, before: Some("Pivot".into()), after: Some("Root".into()) }))]);
    assert_eq!(super::super::GltfMutation::parse_op(&redo[0].print_op()).expect("redo text codec decodes"), redo[0]);
    assert_eq!(super::super::GltfMutation::decode_op(&redo[0].encode_op().expect("redo binary codec encodes")).expect("redo binary codec decodes"), redo[0]);
    current = apply_mutation(&current, &inverse[0]);
    assert_eq!(current, base);
    assert_eq!(apply_mutation(&current, &redo[0]), snapshot(Some("Pivot")));
    let stale_redo = <ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::diff(
        match &redo[0] {
            super::super::GltfMutation::ChangeNodeName(value) => value,
            _ => unreachable!(),
        },
        &snapshot(Some("Other")),
    );
    assert_eq!(stale_redo.messages()[0].code, protocol::FaultCode::new("mutation.invariant"));
}

#[test]
fn none_normalization_stale_guards_and_noops_emit_no_inverse() {
    let absent = snapshot(None);
    assert_eq!(absent.document.nodes[0].name, None);
    let restore = ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node: 0, before: Some("Root".into()), after: None });
    let restored = apply_restore(
        match &restore {
            ChangeNodeNameMutation::Restore(value) => value,
            _ => unreachable!(),
        },
        &absent,
    )
    .expect("absent node matches null after witness");
    assert_eq!(restored.document.nodes[0].name.as_deref(), Some("Root"));
    assert!(<ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::inverse(&ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 0, value: None }), &absent).is_empty());
    assert!(<ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::inverse(&restore, &snapshot(Some("Other"))).is_empty());
    let stale = <ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::diff(&restore, &snapshot(Some("Pivot")));
    assert_eq!(stale.messages()[0].code, protocol::FaultCode::new("mutation.invariant"));
    let missing = <ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::diff(&ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 1, value: Some("Pivot".into()) }), &snapshot(Some("Root")));
    assert_eq!(missing.messages()[0].code, protocol::FaultCode::new("mutation.target-missing"));
}

#[test]
fn restore_wire_requires_nullable_witnesses_and_excludes_document_diffs() {
    assert!(pack::from_json_str::<ChangeNodeNameMutation>(r#"{"phase":"restore","value":{"node":0,"after":"Pivot"}}"#).is_err());
    assert!(pack::from_json_str::<ChangeNodeNameMutation>(r#"{"phase":"apply","value":{"node":0}}"#).is_err());
    assert!(pack::from_json_str::<ChangeNodeNameMutation>(r#"{"phase":"restore","value":{"node":0,"before":null,"after":"Pivot","sourceForm":"glb"}}"#).is_err());
    let wire = pack::to_json_string(&ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node: 0, before: None, after: Some("Pivot".into()) }));
    assert_eq!(pack::parse_json(&wire).expect("restore encodes valid json"), pack::json!({ "phase": "restore", "value": { "node": 0, "before": null, "after": "Pivot" } }));
}

#[test]
fn facade_decoders_construct_canonical_mutations_and_reject_malformed_boundaries() {
    let apply = ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 0, value: Some("Pivot".into()) });
    let present = value([("present", DslValue::String("Pivot".into()))]);
    let graphql = value([("apply", value([("node", DslValue::uint(0)), ("value", present.clone())]))]);
    assert_eq!(decode_gltf_change_node_name_graphql(&graphql).expect("graphql apply decodes"), apply);
    assert_eq!(decode_gltf_change_node_name_proto(&graphql).expect("proto apply decodes"), apply);
    assert_eq!(decode_gltf_change_node_name_protobuf(&[0x0a, 0x0b, 0x08, 0x00, 0x12, 0x07, 0x0a, 0x05, b'P', b'i', b'v', b'o', b't']).expect("protobuf apply decodes"), apply);
    let bom = ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 0, value: Some("\u{feff}Pivot".into()) });
    assert_eq!(decode_gltf_change_node_name_protobuf(&[0x0a, 0x0e, 0x08, 0x00, 0x12, 0x0a, 0x0a, 0x08, 0xef, 0xbb, 0xbf, b'P', b'i', b'v', b'o', b't']).expect("protobuf preserves bom"), bom);
    let false_absent = value([("apply", value([("node", DslValue::uint(0)), ("value", value([("absent", DslValue::Bool(false))]))]))]);
    assert_eq!(decode_gltf_change_node_name_graphql(&false_absent).expect_err("graphql false absent rejects").code, "nullable");
    let nonempty_absent = value([("apply", value([("node", DslValue::uint(0)), ("value", value([("absent", value([("extra", DslValue::Bool(true))]))]))]))]);
    assert_eq!(decode_gltf_change_node_name_proto(&nonempty_absent).expect_err("proto nonempty absent rejects").code, "unknown");
    let duplicate = DslValue::Object(vec![("apply".into(), value([("node", DslValue::uint(0)), ("value", present.clone())])), ("apply".into(), value([("node", DslValue::uint(0)), ("value", present)]))]);
    assert_eq!(decode_gltf_change_node_name_graphql(&duplicate).expect_err("duplicate graphql key rejects").code, "duplicate");
    assert_eq!(decode_gltf_change_node_name_protobuf(&[]).expect_err("protobuf missing phase rejects").code, "truncated");
    assert_eq!(decode_gltf_change_node_name_protobuf(&[0x18, 0x00]).expect_err("protobuf unknown phase rejects").code, "phase");
    assert_eq!(decode_gltf_change_node_name_protobuf(&[0x0a, 0x04, 0x08, 0x00, 0x08, 0x00]).expect_err("protobuf duplicate node rejects").code, "duplicate");
}

#[test]
fn semantic_identity_matches_the_language_neutral_descriptor() {
    assert_eq!(<ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-node-name");
    let mutation = ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 7, value: Some("Pivot".into()) });
    assert_eq!(mutation.target(), vec!["document/nodes/7/name"]);
}
