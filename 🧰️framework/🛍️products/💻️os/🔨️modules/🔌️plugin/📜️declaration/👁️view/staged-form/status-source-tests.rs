//! 🧪️ Ready status traverses the resident nested commit source with the same exact neutral edit carriers.
use crate::{StagedStatusPayload,StagedPayloadCursor};
use semio_framework::action_staging::{StagedFormStatus,StagedFormToken};
use semio_ui_contract::{UiMapBuilder,UiValue,UiValueRetirement};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneStep};
fn retire(value:UiValue){let mut owner=UiValueRetirement::new(value);for _ in 0..16384{if owner.terminal_is_empty(){return}owner.close_step(1,32768).unwrap();}panic!("status payload did not retire")}
#[test]
fn ready_status_nested_commit_payload_matches_neutral_source_without_cloning_a_wrapper(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/binding-cases.json")).unwrap();
    let warm=UiMapBuilder::try_new().unwrap();retire(UiValue::Map(warm.finish()));
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:32768,maximum_depth:64};
    for row in fixture["cases"].as_array().unwrap(){
        let args:DslValue=serde_json::from_value(row["edit"].clone()).unwrap();
        let status=StagedFormStatus{token:StagedFormToken{action:"insertItem".into(),generation:"1".into(),revision:"0".repeat(64),window:"norm-form".into()},ready:true,commit_args:Some(args),diagnostics:Vec::new(),diagnostic_offset:"0".into(),diagnostic_total:"0".into()};
        let source=StagedStatusPayload{status:&status};let mut cursor=StagedPayloadCursor::default();let mut output=None;
        for _ in 0..2048{
            let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut output,grant));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            if matches!(step.unwrap(),RetainedCloneStep::Complete(_)){break}
        }
        let value=output.expect("nested status completed");assert_eq!(serde_json::to_value(&value).unwrap(),serde_json::to_value(&status).unwrap());assert!(cursor.terminal_is_empty());retire(value);
    }
}

#[test]
fn staged_native_heap_witness_observes_actual_allocation_and_release(){
    let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let bytes=std::hint::black_box(vec![7u8;73]);std::hint::black_box(&bytes);drop(bytes);});
    assert_eq!((heap.requested_bytes,heap.released_bytes),(73,73));
}
