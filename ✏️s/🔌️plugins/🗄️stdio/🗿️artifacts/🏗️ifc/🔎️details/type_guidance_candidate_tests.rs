//! 🧪️ Actual guided private preparation agrees with the independent exact-word candidate laws.
use super::*;
#[test]
fn original_guided_candidate_reuses_actual_roster_witness_rules_and_request_handback(){
    use super::super::{source_loan::SourceLoan,type_guidance_candidate::GuidedCandidate};
    use semio_framework_plugin::action_staging::{StagedFormRequest,StagedFormToken};
    if DetailsTestProfile::catalog().schema!="IFC4X3_ADD2"{return;}
    let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/type-number-binary-input.json")).unwrap();
    for row in law["cases"].as_array().unwrap(){
        let snapshot=super::type_number_binary_input_tests::snapshot(&law,row["source"].as_str().unwrap());let mut caller=OriginalWorkFixture::new();let mut loan=Some(SourceLoan::default());
        for _ in 0..8{if caller.turn(grant(),|cx|loan.as_mut().unwrap().advance(&snapshot,snapshot.as_ref(),[0;32],cx)).unwrap(){break;}}
        let mut candidate=Some(GuidedCandidate::<DetailsTestProfile>::default());
        let mut donor=Some(StagedFormRequest{token:StagedFormToken{action:editing::SET_SNAPSHOT_VALUE_ACTION_ID.into(),window:"native-type-source".into(),generation:"1".into(),revision:"0".repeat(64)},args:super::type_input_tests::raw(&serde_json::json!({"path":"/entities/0","value":{"$ifcType":row["target"],"$ifcInputs":[{"index":row["index"].as_u64().unwrap().to_string(),"value":row["value"]}]}})),locale:semio_framework_ui_locale::Locale::En,page:None});
        let original=donor.as_ref().unwrap().token.action.as_ptr();let expected=donor.as_ref().unwrap().args.clone();let mut complete=false;
        for _ in 0..1000000{
            let source=loan.as_ref().unwrap().source().unwrap();let demand=candidate.as_ref().unwrap().demands(source,&snapshot,4096).unwrap();
            for axis in 0..6{let mut denied=grant();let mut fuel=1;match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes=demand.copy_bytes-1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes=demand.capacity_bytes-1,3 if demand.release_bytes>0=>denied.maximum_release_bytes=demand.release_bytes-1,4=>denied.maximum_depth=0,5=>fuel=0,_=>continue};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.work_turn(denied,fuel,|cx|candidate.as_mut().unwrap().advance(&mut donor,source,&snapshot,[0;32],cx)));assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
            if caller.work_turn(grant(),1,|cx|candidate.as_mut().unwrap().advance(&mut donor,source,&snapshot,[0;32],cx)).unwrap(){complete=true;break;}
        }
        assert!(complete);assert_eq!(candidate.as_ref().unwrap().valid(),row["admitted"].as_bool().unwrap(),"{}",row["id"]);assert_eq!(donor.as_ref().unwrap().token.action.as_ptr(),original);assert_eq!(donor.as_ref().unwrap().args,expected);
        caller.cancel().cancel_now();super::type_guidance_request_tests::retire_owner(&mut caller,&mut candidate);super::type_guidance_request_tests::retire_owner(&mut caller,&mut donor);super::type_guidance_request_tests::retire_owner(&mut caller,&mut loan);assert_eq!(Arc::strong_count(&snapshot),1);caller.finish();
    }
}
