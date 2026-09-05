//#region 🧪️ContributedMutationWireLaws
//! 🧪️ Direct contributed-wire mutation laws and codec provenance.

use super::*;
use protocol::{CompositeMutationKind, Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary};

fn cases() -> serde_json::Value {
    serde_json::from_str(include_str!("../🔣️.json")).expect("contributed wire neutral cases")
}

fn operation(delta: i32) -> WireTestMutation {
    WireTestMutation::AddValue(AddValue { delta })
}

#[test]
fn descriptor_and_provenance_are_direct() {
    assert_eq!(serde_json::Value::from(protocol::ToValue::to_value(&AddValue::DESCRIPTOR)), serde_json::from_str::<serde_json::Value>(include_str!("../🧬️mutations/➕️add-value/🔣️.json")).expect("owned descriptor JSON"));
    assert!(AddValue::DESCRIPTOR.validate().is_ok());
    assert_eq!(<WireTestMutation as Mutation<WireTestSnapshot>>::DESCRIPTORS, &[AddValue::DESCRIPTOR]);
    assert_eq!(operation(5).descriptor(), &AddValue::DESCRIPTOR);
    let provenance = AddValue::PROVENANCE;
    assert_eq!(provenance.owner, AddValue::DESCRIPTOR.owner);
    assert_eq!(provenance.source_path, format!("{}/🦀️.rs", provenance.owner));
    assert_eq!(provenance.descriptor_path, format!("{}/🔣️.json", provenance.owner));
    let scope = protocol::MutationLeafSourceScope { workspace_token: provenance.workspace_token, mutation_root: provenance.mutation_root, owner_layout: protocol::MutationOwnerLayout::Flat, taxonomy_path: provenance.taxonomy_path, mutation_payload_facet: "🦠️mutation", source_filename: "🦀️.rs", descriptor_filename: "🔣️.json" };
    assert!(protocol::validate_mutation_leaf_source(&AddValue::DESCRIPTOR, &provenance, &scope).is_ok());
}

#[test]
fn ordered_checked_diff_and_minimum_inverse_are_lawful() {
    for row in cases()["diffs"].as_array().expect("diff cases") {
        let base = WireTestSnapshot { value: row["base"].as_i64().expect("base").try_into().expect("i32 base") };
        let mut current = base.clone();
        let mut stored_inverse = Vec::new();
        let mut rejected = false;
        for delta in row["deltas"].as_array().expect("deltas") {
            let mutation = operation(delta.as_i64().expect("delta").try_into().expect("i32 delta"));
            stored_inverse.extend(mutation.inverse(&current));
            match mutation.diff(&current).diff().apply(&current) {
                Ok(next) => current = next,
                Err(error) => { assert_eq!(error.code, "mutation.apply.overflow"); rejected = true; break; }
            }
        }
        if row.get("error").is_some() {
            assert!(rejected, "{row}");
            continue;
        }
        assert!(!rejected, "{row}");
        assert_eq!(i64::from(current.value), row["result"].as_i64().expect("result"));
        let inverse: Vec<i32> = stored_inverse.iter().map(|mutation| match mutation { WireTestMutation::AddValue(value) => value.delta }).collect();
        assert_eq!(serde_json::to_value(inverse).expect("inverse JSON"), row["inverse"]);
        for mutation in stored_inverse.iter().rev() {
            current = mutation.diff(&current).diff().apply(&current).expect("Store reverse inverse");
        }
        assert_eq!(current, base);
    }
    let mut diff = WireTestDiff { deltas: vec![i32::MAX] };
    diff.absorb(WireTestDiff { deltas: vec![-i32::MAX] });
    assert!(diff.apply(&WireTestSnapshot { value: 1 }).is_err());
    assert_eq!(diff.apply(&WireTestSnapshot { value: 0 }).expect("ordered cancellation").value, 0);
}

#[test]
fn serde_binary_and_composite_plan_match_the_leaf() {
    let mutation = operation(5);
    assert_eq!(serde_json::to_value(&mutation).expect("operation JSON"), serde_json::json!({ "operation": "addValue", "delta": 5 }));
    assert_eq!(WireTestMutation::decode_op(&mutation.encode_op().expect("serde binary")).expect("serde binary decode"), mutation);
    assert!(WireTestMutation::decode_op(br#"{"operation":"addValue","delta":5,"unknown":true}"#).is_err());
    let base = WireTestSnapshot { value: 7 };
    let plan = protocol::plan_of::<WireTestSnapshot, WireTestMutation, AddValue>(&AddValue { delta: 5 }, &base).expect("plan");
    assert_eq!(plan.len(), 1);
    assert!(matches!(&plan[0], protocol::PlanStep::Local(WireTestMutation::AddValue(AddValue { delta: 5 }))));
    assert_eq!(protocol::fold_plan_diff(&AddValue { delta: 5 }, &base).diff().apply(&base).expect("planned diff"), mutation.diff(&base).diff().apply(&base).expect("direct diff"));
    assert_eq!(<AddValue as CompositeMutationKind<WireTestSnapshot, WireTestMutation>>::SEMANTICS.kind, "add-value");
    assert_eq!(<AddValue as CompositeMutationKind<WireTestSnapshot, WireTestMutation>>::label(&AddValue { delta: 5 }), "Add 5 to value");
}
//#endregion 🧪️ContributedMutationWireLaws
