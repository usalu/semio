//#region 🧪️JobTestMutationLaws
use super::*;
use protocol::{Mutation,MutationDiff,MutationLeaf};
use store::OpBinary;

#[test]
fn actual_descriptor_provenance(){let provenance=AddValue::PROVENANCE;assert_eq!(provenance.owner,AddValue::DESCRIPTOR.owner);assert_eq!(provenance.source_path,format!("{}/🦀️.rs",provenance.owner));assert_eq!(provenance.descriptor_path,format!("{}/🔣️.json",provenance.owner));let scope=protocol::MutationLeafSourceScope{workspace_token:provenance.workspace_token,mutation_root:provenance.mutation_root,taxonomy_path:provenance.taxonomy_path,source_filename:"🦀️.rs",descriptor_filename:"🔣️.json"};assert!(protocol::validate_mutation_leaf_source(&AddValue::DESCRIPTOR,&provenance,&scope).is_ok());}

#[test]
fn direct_plan_and_inverse_preserve_job_semantics(){let base=JobTestSnapshot{value:10};let leaf=AddValue{delta:5};let direct=JobTestOp::AddValue(leaf.clone()).diff(&base).diff().apply(&base).expect("direct add");let plan=protocol::plan_of::<JobTestSnapshot,JobTestOp,AddValue>(&leaf,&base).expect("plan");assert_eq!(plan.len(),1);assert_eq!(direct,JobTestSnapshot{value:15});let inverse=JobTestOp::AddValue(leaf).inverse(&base);assert_eq!(inverse,vec![JobTestOp::AddValue(AddValue{delta:-5})]);assert_eq!(inverse[0].diff(&direct).diff().apply(&direct).expect("undo"),base);}

#[test]
fn binary_codec_round_trip(){for delta in [-5,0,5]{let operation=JobTestOp::AddValue(AddValue{delta});assert_eq!(JobTestOp::decode_op(&operation.encode_op().expect("encode")).expect("decode"),operation);}}
#[test]
fn neutral_snapshot_and_diff_schema_vectors_match_serde() {
    let vectors:serde_json::Value=serde_json::from_str(include_str!("🧫️vectors.json")).expect("neutral vectors");
    for row in vectors["schemaCases"].as_array().expect("schema cases") {
        let accepted=match row["target"].as_str().expect("schema target") {
            "snapshot"=>serde_json::from_value::<JobTestSnapshot>(row["value"].clone()).is_ok(),
            "diff"=>serde_json::from_value::<JobTestDiff>(row["value"].clone()).is_ok(),
            _=>continue,
        };
        assert_eq!(accepted,row["accept"].as_bool().expect("acceptance"),"{}",row["id"]);
    }
}

#[test]
fn neutral_checked_diff_boundaries_have_typed_rejections() {
    let vectors:serde_json::Value=serde_json::from_str(include_str!("🧫️vectors.json")).expect("neutral vectors");
    for row in vectors["apply"].as_array().expect("apply cases") {
        let base=JobTestSnapshot{value:serde_json::from_value(row["base"].clone()).expect("base")};
        let diff=JobTestDiff{deltas:serde_json::from_value(row["deltas"].clone()).expect("deltas")};
        match diff.apply(&base) {
            Ok(actual)=>assert_eq!(actual.value,serde_json::from_value::<i32>(row["result"].clone()).expect("expected value"),"{}",row["id"]),
            Err(error)=>{
                assert_eq!(error.code,row["error"].as_str().expect("expected rejection"),"{}",row["id"]);
                assert_eq!(error.target,vec!["value".to_owned()]);
            }
        }
        assert_eq!(base.value,serde_json::from_value::<i32>(row["base"].clone()).expect("unchanged base"));
    }
}

#[test]
fn absorb_preserves_order_and_intermediate_rejection() {
    let vectors:serde_json::Value=serde_json::from_str(include_str!("🧫️vectors.json")).expect("neutral vectors");
    for row in vectors["composition"].as_array().expect("composition cases") {
        let base=JobTestSnapshot{value:serde_json::from_value(row["base"].clone()).expect("base")};
        let first=JobTestDiff{deltas:serde_json::from_value(row["left"].clone()).expect("left")};
        let second=JobTestDiff{deltas:serde_json::from_value(row["right"].clone()).expect("right")};
        let sequential=first.apply(&base).and_then(|mid|second.apply(&mid));
        let mut expected=first.deltas.clone();
        expected.extend(second.deltas.clone());
        let mut combined=first;
        combined.absorb(second);
        assert_eq!(combined.deltas,expected,"{}",row["id"]);
        assert_eq!(combined.apply(&base),sequential,"{}",row["id"]);
        if row.get("error").is_some() { assert!(sequential.is_err(),"{}",row["id"]); }
    }
}

#[test]
fn ordered_diff_absorb_is_associative_at_boundaries() {
    let choices=[vec![],vec![0],vec![1],vec![-1],vec![i32::MIN],vec![i32::MAX]];
    for a in &choices { for b in &choices { for c in &choices {
        let first=JobTestDiff{deltas:a.clone()};
        let second=JobTestDiff{deltas:b.clone()};
        let third=JobTestDiff{deltas:c.clone()};
        let mut left=first.clone();
        left.absorb(second.clone());
        left.absorb(third.clone());
        let mut grouped=second.clone();
        grouped.absorb(third.clone());
        let mut right=first.clone();
        right.absorb(grouped);
        assert_eq!(left,right);
        for value in [i32::MIN,i32::MIN+1,-1,0,1,i32::MAX-1,i32::MAX] {
            let base=JobTestSnapshot{value};
            let sequential=first.apply(&base).and_then(|mid|second.apply(&mid)).and_then(|mid|third.apply(&mid));
            assert_eq!(left.apply(&base),sequential);
            assert_eq!(JobTestDiff::default().apply(&base),Ok(base));
        }
    } } }
}
//#endregion 🧪️JobTestMutationLaws
