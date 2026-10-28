//! 🧪️ Neutral staging counters, exact text and typed choice payloads have the same native meaning.
use super::*;
#[test]
fn native_page_destinations_match_neutral_closed_values_and_keep_invalid_sources_unchanged(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("page-cases.json")).unwrap();
    let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:8192,maximum_release_bytes:8192,maximum_depth:64};
    for(valid,rows)in [(true,"valid"),(false,"invalid")]{for page in fixture[rows].as_array().unwrap(){
        let row=serde_json::json!({"token":fixture["token"],"args":{},"locale":"de","page":page});let mut source=DslValue::from(&row);let before=source.clone();
        let mut cursor=StagedFormRequestCursor::new(StagedFormLimits::FRAMEWORK).unwrap();let mut output=None;let mut refused=false;
        for turn in 0..16384{match cursor.step(&mut source,&mut output,grant){Err(_)=>{refused=true;break},Ok(semio_framework_value::RetainedCloneStep::Complete(_))=>break,_=>assert!(turn+1<16384)}}
        assert_eq!(!refused,valid);assert_eq!(output.is_some(),valid);if let Some(request)=output{assert_eq!(serde_json::to_value(request).unwrap(),row);}else{assert_eq!(source,before);}
    }}
}

#[test]
fn native_generation_boundary_matches_neutral_u64_tokens_and_consuming_receiver(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("token-cases.json")).unwrap();
    let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
    for(valid,rows)in [(true,"valid"),(false,"invalid")]{for generation in fixture[rows].as_array().unwrap(){
        let row=serde_json::json!({"token":{"action":"ApplyTemplate","generation":generation,"revision":"r1","window":"w1"},"args":{},"locale":"en"});
        let token:StagedFormToken=serde_json::from_value(row["token"].clone()).unwrap();assert_eq!(token.admitted(),valid);
        let mut source=DslValue::from(&row);let mut output=None;let mut cursor=StagedFormRequestCursor::new(StagedFormLimits::FRAMEWORK).unwrap();let mut refused=false;
        for _ in 0..8192{match cursor.step(&mut source,&mut output,grant){Err(_)=>{refused=true;break},Ok(semio_framework_value::RetainedCloneStep::Complete(_))=>break,_=>{}}}
        assert_eq!(!refused,valid);assert_eq!(output.is_some(),valid);
    }}
}
#[test]
fn owned_request_receiving_moves_exact_private_payload_after_neutral_admission() {
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("request-cases.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
    for row in fixture["valid"].as_array().unwrap(){
        let mut source=DslValue::from(row);let mut output=None;
        let args_pointer=match &source{DslValue::Object(fields)=>match &fields.iter().find(|(name,_)|name=="args").unwrap().1{DslValue::Object(fields)=>fields.as_ptr(),_=>unreachable!()},_=>unreachable!()};
        let mut cursor=StagedFormRequestCursor::new(StagedFormLimits::FRAMEWORK).unwrap();
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
            assert_eq!(cursor.step(&mut source,&mut output,denied).unwrap(),RetainedCloneStep::Progress(Default::default()));assert!(output.is_none());
        }
        for _ in 0..8192{let step=cursor.step(&mut source,&mut output,grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){break}}
        let request=output.as_ref().unwrap();assert_eq!(serde_json::to_value(request).unwrap(),*row);
        assert_eq!(match &request.args{DslValue::Object(fields)=>fields.as_ptr(),_=>unreachable!()},args_pointer);
    }
    for row in fixture["invalid"].as_array().unwrap(){
        let mut source=DslValue::from(row);let before=source.clone();let mut output=None;let mut cursor=StagedFormRequestCursor::new(StagedFormLimits::FRAMEWORK).unwrap();let mut refused=false;
        for _ in 0..8192{if cursor.step(&mut source,&mut output,grant).is_err(){refused=true;break}}
        assert!(refused);assert!(output.is_none());assert_eq!(source,before);assert!(cursor.step(&mut source,&mut output,grant).is_err());
    }
}
#[test]
fn standalone_wire_reservation_matches_language_neutral_outer_capacity() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("wire-cases.json")).unwrap();
    assert_eq!(fixture["framingBytesPerNode"],32);assert_eq!(fixture["recordFramingBytes"],32);
    assert_eq!(StagedFormLimits::FRAMEWORK.wire_bytes().unwrap(),fixture["frameworkMaximumBytes"].as_u64().unwrap()as usize);
    assert!(StagedFormLimits{nodes:0,..StagedFormLimits::FRAMEWORK}.wire_bytes().is_err());
}
#[test]
fn native_wire_census_refuses_duplicate_and_reversed_keys_before_movement(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("wire-map-cases.json")).unwrap();
    let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
    for row in fixture["cases"].as_array().unwrap(){
        let pairs=row["keys"].as_array().unwrap().iter().map(|key|(key.as_str().unwrap().into(),DslValue::Null)).collect();
        let source=DslValue::Object(vec![("nested".into(),DslValue::Object(pairs))]);let mut cursor=StagedFormCensusCursor::new_wire(StagedFormLimits::FRAMEWORK).unwrap();let mut refused=false;
        for _ in 0..4096{match cursor.step(&source,grant){Err(_)=>{refused=true;break},Ok(semio_framework_value::RetainedCloneStep::Complete(_))=>break,_=>{}}}
        assert_eq!(!refused,row["valid"].as_bool().unwrap());assert_eq!(cursor.result().is_some(),!refused);
    }
}
#[test]
fn exact_staged_text_preserves_signed_and_unsigned_boundaries() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    assert!(StagedFormTextFormat::DecimalU64.admits(contract["args"]["target"].as_str().unwrap()));
    assert!(StagedFormTextFormat::Binary64.admits(contract["args"]["entries"][0]["value"]["value"]["bits"].as_str().unwrap()));
    assert!(StagedFormTextFormat::DecimalI64.admits(contract["cases"][1]["value"].as_str().unwrap()));
    for value in ["01", "+1", "-0", "9223372036854775808"] {
        assert!(!StagedFormTextFormat::DecimalI64.admits(value));
    }
    for value in ["-1", "18446744073709551616"] {
        assert!(!StagedFormTextFormat::DecimalU64.admits(value));
    }
}
#[test]
fn request_tokens_distinguish_windows_generations_actions_and_source_revisions() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    let original: StagedFormToken = serde_json::from_value(contract["token"].clone()).unwrap();
    for stale in contract["stale"].as_array().unwrap() {
        let token: StagedFormToken = serde_json::from_value(stale.clone()).unwrap();
        assert_ne!(token, original);
    }
    let wire = semio_framework_value::ToValue::to_value(&original);
    assert_eq!(<StagedFormToken as semio_framework_value::FromValue>::from_value(wire).unwrap(), original);
}

#[test]
fn original_resident_binding_plan_borrows_exact_neutral_values() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("binding-cases.json")).unwrap();
    let contract:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();
    let token:StagedFormToken=serde_json::from_value(contract["token"].clone()).unwrap();
    let status=StagedFormStatus{token:token.clone(),ready:false,diagnostics:Vec::new(),diagnostic_offset:"0".into(),diagnostic_total:"0".into()};
    for row in cases["invalid"].as_array().unwrap(){
        let binding=StagedFormBinding{token:token.clone(),layout:StagedFormLayout::Replace,edits:vec![serde_json::from_value(row["edit"].clone()).unwrap()]};
        let input=DslValue::from(&row["input"]);
        assert!(binding.plan(&status,Some(&input)).is_err());
    }
    for row in cases["cases"].as_array().unwrap(){
        let binding=StagedFormBinding{token:token.clone(),layout:StagedFormLayout::Replace,edits:vec![serde_json::from_value(row["edit"].clone()).unwrap()]};
        let input=row.get("input").map(DslValue::from);
        let mut plan=binding.plan(&status,input.as_ref()).unwrap();
        let (kind,path,value)=match plan.next().unwrap(){
            StagedFormBorrowedEdit::Set{path,value}=>("set",path,Some(value)),
            StagedFormBorrowedEdit::Append{path,value}=>("append",path,Some(value)),
            StagedFormBorrowedEdit::Remove{path}=>("remove",path,None),
        };
        assert_eq!(kind,row["kind"].as_str().unwrap());
        assert_eq!(serde_json::to_value(path).unwrap(),row["path"]);
        assert_eq!(value.map(serde_json::Value::from),row.get("value").cloned());
        assert!(plan.next().is_none());
        let mut stale=status.clone();stale.token.revision.push('x');
        assert!(binding.plan(&stale,input.as_ref()).is_err());
    }
}

#[test]
fn finite_native_staging_census_matches_neutral_reservations_and_refuses_resumption() {
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("limits-cases.json")).unwrap();
    let caps:StagedFormLimits=serde_json::from_str(include_str!("limits.json")).unwrap();
    assert_eq!(caps,StagedFormLimits::FRAMEWORK);
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
    for row in fixture["cases"].as_array().unwrap(){
        let source=DslValue::from(&row["value"]);let mut cursor=StagedFormCensusCursor::new(caps).unwrap();
        assert_eq!(cursor.step(&source,RetainedCloneGrant{maximum_items:0,..grant}).unwrap(),RetainedCloneStep::Progress(Default::default()));
        assert_eq!(cursor.step(&source,RetainedCloneGrant{maximum_copy_bytes:0,..grant}).unwrap(),RetainedCloneStep::Progress(Default::default()));
        assert_eq!(cursor.step(&source,RetainedCloneGrant{maximum_depth:0,..grant}).unwrap(),RetainedCloneStep::Progress(Default::default()));
        for _ in 0..4096 {let step=cursor.step(&source,grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){break}}
        let actual=cursor.result().unwrap();assert_eq!(actual.utf8_bytes,row["reservation"].as_u64().unwrap()as usize);assert_eq!(actual.nodes,row["nodes"].as_u64().unwrap()as usize);assert_eq!(actual.depth,row["depth"].as_u64().unwrap()as usize);
        assert!(actual.utf8_bytes>=serde_json::to_vec(&row["value"]).unwrap().len());
    }
    for row in fixture["refusals"].as_array().unwrap(){
        let source=DslValue::from(&row["value"]);let mut limits=caps;let value=row["limit"].as_u64().unwrap()as usize;
        match row["axis"].as_str().unwrap(){"utf8Bytes"=>limits.utf8_bytes=value,"nodes"=>limits.nodes=value,"depth"=>limits.depth=value,_=>unreachable!()}
        let mut cursor=StagedFormCensusCursor::new(limits).unwrap();let mut refused=false;
        for _ in 0..4096 {if cursor.step(&source,grant).is_err(){refused=true;break}}
        assert!(refused);assert!(cursor.result().is_none());assert!(cursor.step(&source,grant).is_err());
    }
}
#[test]
fn staged_original_donor_context_matches_shared_vectors(){
    let law:serde_json::Value=serde_json::from_str(include_str!("handoff-cases.json")).unwrap();let context=&law["context"];
    for row in law["cases"].as_array().unwrap(){let request:super::StagedFormRequest=serde_json::from_value(serde_json::json!({"token":{"action":"applyIfcTemplate","revision":"0".repeat(64),"generation":row["generation"],"window":row["window"]},"locale":row["locale"],"args":{}})).unwrap();let locale=serde_json::from_value(context["locale"].clone()).unwrap();assert_eq!(request.context_matches(context["window"].as_str(),locale),row["expected"].as_bool().unwrap());assert!(!request.context_matches(None,locale));}
}
