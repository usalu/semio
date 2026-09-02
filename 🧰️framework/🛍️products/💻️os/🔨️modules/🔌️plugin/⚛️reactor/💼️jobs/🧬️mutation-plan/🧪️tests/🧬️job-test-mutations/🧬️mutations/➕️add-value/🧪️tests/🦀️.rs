//#region 🧪️AddJobTestValueLaws
use super::*;
use protocol::{Mutation,MutationDiff,MutationLeaf};
use store::OpBinary;

fn vectors()->serde_json::Value {
    serde_json::from_str(include_str!("../../../🧪️tests/🔣️.json")).expect("neutral job vectors")
}

fn apply_stored_inverse(stored:&[JobTestOp],after:&JobTestSnapshot)->protocol::MutationApplyResult<JobTestSnapshot> {
    stored.iter().rev().try_fold(after.clone(),|state,operation|operation.diff(&state).diff().apply(&state))
}

#[test]
fn leaf_descriptor_matches_actual_authored_file() {
    let authored:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).expect("descriptor");
    assert_eq!(authored["owner"],AddValue::DESCRIPTOR.owner);
    assert_eq!(authored["semanticKind"],AddValue::DESCRIPTOR.semantic_kind);
    assert_eq!(authored["aggregateVariant"],"AddValue");
    assert_eq!(AddValue::PROVENANCE.source_path,format!("{}/🦀️.rs",AddValue::DESCRIPTOR.owner));
    assert_eq!(AddValue::PROVENANCE.descriptor_path,format!("{}/🔣️.json",AddValue::DESCRIPTOR.owner));
}

#[test]
fn neutral_payload_and_binary_schema_vectors_match_actual_codecs() {
    for row in vectors()["schemaCases"].as_array().expect("schema cases") {
        let bytes=serde_json::to_vec(&row["value"]).expect("fixture bytes");
        let accepted=match row["target"].as_str().expect("schema target") {
            "payload"=>serde_json::from_slice::<AddValue>(&bytes).is_ok(),
            "operation"=>JobTestOp::decode_op(&bytes).is_ok(),
            _=>continue,
        };
        assert_eq!(accepted,row["accept"].as_bool().expect("acceptance"),"{}",row["id"]);
        if accepted&&row["target"]=="operation" {
            let operation=JobTestOp::decode_op(&bytes).expect("accepted operation");
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&operation.encode_op().expect("encoded operation")).expect("JSON bytes"),row["value"]);
        }
    }
}

#[test]
fn minimum_inverse_is_stored_as_one_then_maximum() {
    let operation=JobTestOp::AddValue(AddValue{delta:i32::MIN});
    for value in [0,1,i32::MAX] {
        let base=JobTestSnapshot{value};
        let stored=operation.inverse(&base);
        assert_eq!(stored,vec![JobTestOp::AddValue(AddValue{delta:1}),JobTestOp::AddValue(AddValue{delta:i32::MAX})]);
        let after=operation.diff(&base).diff().apply(&base).expect("minimum delta");
        assert_eq!(apply_stored_inverse(&stored,&after),Ok(base));
    }
}

#[test]
fn neutral_inverse_vectors_restore_in_store_order() {
    for row in vectors()["inverse"].as_array().expect("inverse cases") {
        let base=JobTestSnapshot{value:serde_json::from_value(row["base"].clone()).expect("base")};
        let operation=JobTestOp::AddValue(AddValue{delta:serde_json::from_value(row["delta"].clone()).expect("delta")});
        let after=operation.diff(&base).diff().apply(&base).expect("valid direct operation");
        assert_eq!(after.value,serde_json::from_value::<i32>(row["result"].clone()).expect("expected result"));
        let expected:Vec<i32>=serde_json::from_value(row["stored"].clone()).expect("stored inverse deltas");
        let stored=operation.inverse(&base);
        assert_eq!(stored,expected.into_iter().map(|delta|JobTestOp::AddValue(AddValue{delta})).collect::<Vec<_>>());
        assert_eq!(apply_stored_inverse(&stored,&after),Ok(base),"{}",row["id"]);
    }
}

#[test]
fn mixed_inverse_groups_stay_forward_before_store_reversal() {
    for row in vectors()["sequences"].as_array().expect("sequence cases") {
        let base=JobTestSnapshot{value:serde_json::from_value(row["base"].clone()).expect("base")};
        let deltas:Vec<i32>=serde_json::from_value(row["deltas"].clone()).expect("deltas");
        let mut state=base.clone();
        let mut stored=Vec::new();
        for delta in deltas {
            let operation=JobTestOp::AddValue(AddValue{delta});
            stored.extend(operation.inverse(&state));
            state=operation.diff(&state).diff().apply(&state).expect("valid operation sequence");
        }
        assert_eq!(state.value,serde_json::from_value::<i32>(row["result"].clone()).expect("expected result"));
        let expected:Vec<i32>=serde_json::from_value(row["stored"].clone()).expect("expected stored inverse");
        assert_eq!(stored,expected.into_iter().map(|delta|JobTestOp::AddValue(AddValue{delta})).collect::<Vec<_>>());
        assert_eq!(apply_stored_inverse(&stored,&state),Ok(base),"{}",row["id"]);
        if let Some(value)=row.get("wrongReversedGroups") {
            let deltas:Vec<i32>=serde_json::from_value(value.clone()).expect("wrong group order");
            let wrong=deltas.into_iter().map(|delta|JobTestOp::AddValue(AddValue{delta})).collect::<Vec<_>>();
            assert_eq!(apply_stored_inverse(&wrong,&state).expect_err("wrong group order overflows").code,"job-test.value-overflow");
        }
    }
}

#[test]
fn ordinary_contributed_plan_keeps_direct_leaf_and_label() {
    for delta in [-5,0,5] {
        let base=JobTestSnapshot{value:10};
        let leaf=AddValue{delta};
        let plan=protocol::plan_of::<JobTestSnapshot,JobTestOp,AddValue>(&leaf,&base).expect("contribution plan");
        assert_eq!(plan.len(),1);
        assert_eq!(<AddValue as protocol::CompositeMutationKind<JobTestSnapshot,JobTestOp>>::label(&leaf),format!("Add {delta} to value"));
        assert_eq!(JobTestOp::AddValue(leaf).diff(&base).diff(),&JobTestDiff{deltas:vec![delta]});
    }
}
//#endregion 🧪️AddJobTestValueLaws
