//! 🧪️ Source-owned numeric choices retain full raw carriers and typed nested text paths.
use super::*;
use semio_framework_plugin::action_staging::*;use semio_framework_plugin_decl_staged_form::*;
type Guidance=super::super::super::type_guidance::IfcTypeInputSourceWork<DetailsTestProfile>;
type Prepared=super::super::super::type_guidance::IfcPreparedTypeInputSource;
fn request(row:&serde_json::Value)->StagedFormRequest{StagedFormRequest{token:StagedFormToken{action:editing::SET_SNAPSHOT_VALUE_ACTION_ID.into(),window:"numeric-source".into(),generation:"1".into(),revision:"0".repeat(64)},locale:semio_framework_ui_locale::Locale::De,args:super::type_input_tests::raw(&serde_json::json!({"path":"/entities/0","value":{"$ifcType":"IFCQUANTITYNUMBER","$ifcInputs":[{"index":"0","value":row["input"]}]}})),page:None}}
#[test]
fn original_number_mode_choices_preserve_selected_utf8_carrier_and_blank_other_mode(){
    if DetailsTestProfile::catalog().schema!="IFC4X3_ADD2"{return;}
    let law:serde_json::Value=serde_json::from_str(include_str!("../🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-number-binary-cases.json")).unwrap();
    let models:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/type-number-binary-input.json")).unwrap();
    for row in law["cases"].as_array().unwrap().iter().filter(|row|row["id"].as_str().unwrap().ends_with("invalid-utf8")){
        let snapshot=super::type_number_binary_input_tests::snapshot(&models,"number");let mut caller=OriginalWorkFixture::new();let mut guide=Some(Guidance::default());let mut donor=Some(request(row));let mut output:Option<Prepared>=None;
        assert!(caller.turn(grant(),|cx|guide.as_mut().unwrap().receive(&mut donor,&snapshot,[0;32],cx)).unwrap());
        for turn in 0..100000{
            let demand=guide.as_ref().unwrap().demands().unwrap();
            for axis in 0..6{let mut denied=grant();let mut fuel=1;match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes=demand.copy_bytes-1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes=demand.capacity_bytes-1,3 if demand.release_bytes>0=>denied.maximum_release_bytes=demand.release_bytes-1,4=>denied.maximum_depth=0,5=>fuel=0,_=>continue};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.work_turn(denied,fuel,|cx|guide.as_mut().unwrap().advance(&snapshot,[0;32],cx)));assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
            if caller.work_turn(grant(),1,|cx|guide.as_mut().unwrap().advance(&snapshot,[0;32],cx)).unwrap(){break;}assert!(turn<99999);
        }
        assert!(caller.turn(grant(),|cx|guide.as_mut().unwrap().take(&mut output,cx)).unwrap());let prepared=output.as_ref().unwrap();assert!(!prepared.status().ready);assert_eq!(prepared.controls(),2);
        let mode=prepared.control(0).unwrap();let StagedScalarKind::Choice{value,choices}=mode.scalar.kind else{panic!("explicit number representation")};assert_eq!(value,row["input"]["kind"].as_str().unwrap());assert_eq!(choices.len(),2);
        let StagedBodyBinding::Action(binding)=mode.binding.unwrap()else{panic!("original action binding")};let options=binding.edits.iter().find_map(|edit|if let StagedFormBindingEdit::Choice{options,..}=edit{Some(options)}else{None}).unwrap();assert_eq!(options.len(),2);
        for option in options{let expected=if option.key==row["input"]["kind"].as_str().unwrap(){row["input"].clone()}else{serde_json::json!({"kind":option.key.as_str(),"value":if option.key=="real"{serde_json::json!({"bits":""})}else{serde_json::json!("")}})};assert_eq!(option.value,super::type_input_tests::raw(&expected));}
        let scalar=prepared.control(1).unwrap();let StagedBodyBinding::Action(binding)=scalar.binding.unwrap()else{panic!("original scalar binding")};let path=binding.edits.iter().find_map(|edit|if let StagedFormBindingEdit::Text{path}=edit{Some(path)}else{None}).unwrap();assert_eq!(path.last().unwrap(),if row["input"]["kind"]=="real"{"bits"}else{"value"});
        caller.cancel().cancel_now();super::type_guidance_request_tests::retire_owner(&mut caller,&mut output);super::type_guidance_request_tests::retire_owner(&mut caller,&mut guide);assert_eq!(Arc::strong_count(&snapshot),1);caller.finish();
    }
}

#[test]
fn original_nested_number_choice_closes_every_partial_utf8_and_bit_object_prefix(){
    if DetailsTestProfile::catalog().schema!="IFC4X3_ADD2"{return;}
    let law:serde_json::Value=serde_json::from_str(include_str!("../🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-number-binary-cases.json")).unwrap();let row=law["cases"].as_array().unwrap().iter().find(|row|row["id"]=="number-real-invalid-utf8").unwrap();let models:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/type-number-binary-input.json")).unwrap();
    let mut total=None;
    for prefix in std::iter::once(None).chain((0..100000).map(Some)){
        if prefix.is_some_and(|prefix|total.is_some_and(|total|prefix>=total)){break;}
        let snapshot=super::type_number_binary_input_tests::snapshot(&models,"number");let mut caller=OriginalWorkFixture::new();let mut guide=Some(Guidance::default());let mut donor=Some(request(row));assert!(caller.turn(grant(),|cx|guide.as_mut().unwrap().receive(&mut donor,&snapshot,[0;32],cx)).unwrap());
        for turn in 0..100000{
            if prefix==Some(turn){caller.cancel().cancel_now();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.work_turn(grant(),1,|cx|{let before=cx.retained_progress();let result=guide.as_mut().unwrap().advance(&snapshot,[0;32],cx);assert_eq!(before,cx.retained_progress());result}));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break;}
            if caller.work_turn(grant(),1,|cx|guide.as_mut().unwrap().advance(&snapshot,[0;32],cx)).unwrap(){assert!(prefix.is_none());total=Some(turn+1);break;}assert!(turn<99999);
        }
        super::type_guidance_request_tests::retire_owner(&mut caller,&mut guide);assert_eq!(Arc::strong_count(&snapshot),1);caller.finish();
    }
    assert!(total.is_some());
}
