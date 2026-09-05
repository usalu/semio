//! 🧪️ Shared declaration-channel laws run by each genuine leaf owner.
use protocol::{FromValue, Mutation, MutationDiff, MutationLeaf, OpBinary, OpText, SemanticMutation, ToValue};
use std::fmt::Debug;

fn cases() -> serde_json::Value { serde_json::from_str(include_str!("../🔣️.json")).expect("declaration fixture cases") }
fn i32_value(value: &serde_json::Value) -> i32 { i32::try_from(value.as_i64().expect("integer")).expect("i32") }

pub(super) fn assert_metadata<S, M, L>(descriptor: &str, operation: fn(i32) -> M)
where M: Mutation<S> + SemanticMutation<S>, L: MutationLeaf {
    let descriptor_json: serde_json::Value = serde_json::from_str(descriptor).expect("owned descriptor JSON");
    assert_eq!(serde_json::Value::from(L::DESCRIPTOR.to_value()), descriptor_json);
    assert!(L::DESCRIPTOR.validate().is_ok());
    assert_eq!(M::DESCRIPTORS, &[L::DESCRIPTOR]);
    assert_eq!(operation(42).descriptor(), &L::DESCRIPTOR);
    assert_eq!(L::DESCRIPTOR.text_opcode, None);
    assert_eq!(L::DESCRIPTOR.binary_tag, None);
    assert_eq!(operation(-1).semantics().kind, "set-value");
    assert_eq!(operation(-1).label(), "Set value to -1");
    assert_eq!(operation(-1).target(), ["value"]);
    assert_eq!(operation(0).timestamp(), None);
    let provenance = L::PROVENANCE;
    assert_eq!(provenance.owner, L::DESCRIPTOR.owner);
    assert_eq!(provenance.source_path, format!("{}/🦀️.rs", provenance.owner));
    assert_eq!(provenance.descriptor_path, format!("{}/🔣️.json", provenance.owner));
    assert!(provenance.owner.ends_with("/🧬️mutations/📝️set-value"));
    let scope = protocol::MutationLeafSourceScope { workspace_token: provenance.workspace_token, mutation_root: provenance.mutation_root, owner_layout: protocol::MutationOwnerLayout::Flat, taxonomy_path: provenance.taxonomy_path, mutation_payload_facet: "🦠️mutation", source_filename: "🦀️.rs", descriptor_filename: "🔣️.json" };
    assert!(protocol::validate_mutation_leaf_source(&L::DESCRIPTOR, &provenance, &scope).is_ok());
    let mut wrong = provenance;
    wrong.source_path = "macro-template/🦀️.rs";
    assert!(protocol::validate_mutation_leaf_source(&L::DESCRIPTOR, &wrong, &scope).is_err());
}

pub(super) fn assert_laws<S, M>(snapshot: fn(i32) -> S, operation: fn(i32) -> M)
where S: Clone + Debug + PartialEq, M: Mutation<S> + Debug + PartialEq, M::Diff: ToValue + FromValue {
    for row in cases()["cases"].as_array().expect("assignment cases") {
        let base = snapshot(i32_value(&row["base"]));
        let mut current = base.clone();
        let mut stored = Vec::new();
        for value in row["values"].as_array().expect("values") {
            let mutation = operation(i32_value(value));
            stored.extend(mutation.inverse(&current));
            current = mutation.diff(&current).diff().apply(&current).expect("assignment");
        }
        assert_eq!(current, snapshot(i32_value(&row["result"])), "{row}");
        let expected: Vec<M> = row["inverse"].as_array().expect("inverse").iter().map(|value| operation(i32_value(value))).collect();
        assert_eq!(stored, expected, "{row}");
        for mutation in stored.iter().rev() { current = mutation.diff(&current).diff().apply(&current).expect("Store reverse undo"); }
        assert_eq!(current, base, "{row}");
    }
    for row in cases()["composition"].as_array().expect("composition cases") {
        let base = snapshot(i32_value(&row["base"]));
        let mut current = base.clone();
        let mut combined = M::Diff::default();
        for value in row["steps"].as_array().expect("steps") {
            let diff = M::Diff::from_value(serde_json::json!({"value": value}).into()).expect("replacement diff");
            current = diff.apply(&current).expect("sequential diff");
            combined.absorb(diff);
        }
        assert_eq!(current, snapshot(i32_value(&row["result"])), "{row}");
        assert_eq!(combined.apply(&base).expect("composed diff"), current, "{row}");
        assert_eq!(serde_json::Value::from(combined.to_value()), serde_json::json!({"value":row["combined"]}));
    }
    for a in [None, Some(i32::MIN), Some(0), Some(i32::MAX)] {
        for b in [None, Some(i32::MIN), Some(0), Some(i32::MAX)] {
            for c in [None, Some(i32::MIN), Some(0), Some(i32::MAX)] {
                let diff = |value: Option<i32>| M::Diff::from_value(serde_json::json!({"value":value}).into()).expect("diff");
                let mut left = diff(a);
                left.absorb(diff(b));
                left.absorb(diff(c));
                let mut middle = diff(b);
                middle.absorb(diff(c));
                let mut right = diff(a);
                right.absorb(middle);
                assert_eq!(serde_json::Value::from(left.to_value()), serde_json::Value::from(right.to_value()));
            }
        }
    }
}

pub(super) fn assert_codecs<S, M, L>(operation: fn(i32) -> M)
where S: Clone + Debug + PartialEq + ToValue + FromValue + store::ArtifactDsl + store::ArtifactPack,
      M: Mutation<S> + Debug + PartialEq + OpText + OpBinary,
      M::Diff: ToValue + FromValue,
      L: ToValue + FromValue {
    let vectors = cases();
    for value in vectors["values"].as_array().expect("values") {
        let mutation = operation(i32_value(value));
        let payload = serde_json::json!({"value":value});
        let leaf = L::from_value(payload.clone().into()).expect("leaf JSON");
        assert_eq!(serde_json::Value::from(leaf.to_value()), payload);
        let envelope = serde_json::json!({"SetValue":payload});
        assert_eq!(serde_json::Value::from(mutation.to_value()), envelope);
        assert_eq!(M::from_value(envelope.clone().into()).expect("JSON decode"), mutation);
        let text = mutation.print_op();
        assert_eq!(text, serde_json::to_string(&envelope).expect("external envelope"));
        assert_eq!(M::parse_op(&text).expect("JSON text decode"), mutation);
        let bytes = mutation.encode_op().expect("JSON binary encode");
        assert_eq!(bytes, text.as_bytes());
        assert_eq!(M::decode_op(&bytes).expect("JSON binary decode"), mutation);
    }
    for value in vectors["invalidPayloads"].as_array().expect("invalid payloads") {
        assert!(L::from_value(value.clone().into()).is_err(), "{value}");
        let text = serde_json::to_string(&serde_json::json!({"SetValue":value})).expect("bad envelope");
        assert!(M::parse_op(&text).is_err(), "{text}");
        assert!(M::decode_op(text.as_bytes()).is_err(), "{text}");
    }
    for value in vectors["invalidEnvelopes"].as_array().expect("invalid envelopes") {
        assert!(M::from_value(value.clone().into()).is_err(), "{value}");
        let text = serde_json::to_string(value).expect("bad JSON");
        assert!(M::parse_op(&text).is_err(), "{text}");
        assert!(M::decode_op(text.as_bytes()).is_err(), "{text}");
    }
    assert!(M::parse_op("").is_err());
    assert!(M::decode_op(&[]).is_err());
    assert!(M::decode_op(&[0xff]).is_err());
    assert!(M::decode_op(b"{}{}").is_err());
    for row in vectors["wireCases"].as_array().expect("raw wire cases") {
        let text = row["text"].as_str().expect("raw JSON text");
        let accept = row["accept"].as_bool().expect("raw JSON expectation");
        let parsed = M::parse_op(text);
        let decoded = M::decode_op(text.as_bytes());
        assert_eq!(parsed.is_ok(), accept, "{}", row["name"]);
        assert_eq!(decoded.is_ok(), accept, "{}", row["name"]);
        if let Some(value) = row.get("decodedValue") {
            let expected = operation(i32_value(value));
            assert_eq!(parsed.expect("accepted raw text"), expected, "{}", row["name"]);
            assert_eq!(decoded.expect("accepted raw bytes"), expected, "{}", row["name"]);
        }
    }
    for value in vectors["snapshots"].as_array().expect("snapshots") {
        let snapshot = S::from_value(value.clone().into()).expect("snapshot");
        let text = snapshot.print_dsl();
        assert_eq!(S::parse_dsl(&text).expect("snapshot text"), snapshot);
        assert_eq!(S::decode_pack(&snapshot.encode_pack()).expect("snapshot pack"), snapshot);
    }
    for value in vectors["invalidSnapshots"].as_array().expect("invalid snapshots") { assert!(S::from_value(value.clone().into()).is_err(), "{value}"); }
    assert_eq!(serde_json::Value::from(S::parse_dsl(" ").expect("empty snapshot text").to_value()), serde_json::json!({"value":0}));
    assert_eq!(serde_json::Value::from(S::decode_pack(&[]).expect("empty snapshot pack").to_value()), serde_json::json!({"value":0}));
    for value in vectors["diffs"].as_array().expect("diffs") { assert!(M::Diff::from_value(value.clone().into()).is_ok(), "{value}"); }
    for value in vectors["invalidDiffs"].as_array().expect("invalid diffs") { assert!(M::Diff::from_value(value.clone().into()).is_err(), "{value}"); }
}

#[test]
fn strict_profile_is_an_io_rule_not_a_mutation_constraint() {
    use crate::app::declarations::fixture::{Std1StrictSnapshot, StrictFromAny};
    let check = <StrictFromAny as semio_framework::io::io_mechanism::Deserializer<Std1StrictSnapshot>>::CONFORMANCE.expect("strict profile check");
    for row in cases()["conformance"].as_array().expect("conformance cases") {
        let snapshot = Std1StrictSnapshot { value: i32_value(&row["value"]) };
        assert_eq!(check(&snapshot).is_empty(), row["accept"].as_bool().expect("accept"));
        let mutation = super::std1_strict::Std1StrictMutation::SetValue(super::std1_strict::SetValue { value: snapshot.value });
        assert_eq!(mutation.diff(&snapshot).diff().apply(&snapshot).expect("negative mutation stays valid"), snapshot);
    }
}
