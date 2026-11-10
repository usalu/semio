//! 🧊️ Field receiving reads sets, maps, literals and blocks natively from the moved field: every action is exactly quoted, denied turns charge nothing, Pending keeps state and every cancelled frontier closes through paid turns.
use semio_framework_dsl_record::receiving::owned::{DslOwnedReceivingField,NativeFieldReceiving,OriginalReceivingPrimary,OwnedAdvanceSelection,OwnedFieldReceiving,OwnedPoll};
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_value::{DslValue,Number,OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkPoll,RetirementDemand,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_ordered::ordered::{OrderedMap,OrderedSet};
use semio_framework_value_testing_admission::{EffectBank,Wallet,denials,driven,session_driven,session_nested,session_open};
use std::collections::BTreeSet;
const BODY:usize=4096;
const MAXIMUM:u64=1<<20;
const STALL:u32=1024;
const SETTLE:usize=1_000_000;
#[derive(Debug,PartialEq,semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
struct Node{id:String,weight:u64}
fn law()->serde_json::Value{serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧊️field-receiving/🔣️.json")).unwrap()}
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn post<E:std::fmt::Debug>(bank:&mut EffectBank,g:RetainedCloneGrant,heap:(usize,usize),poll:RetainedWorkPoll<RetainedCloneProgress,E>)->u128{match poll{RetainedWorkPoll::Pending{work}=>work,RetainedWorkPoll::Complete(turn)=>{bank.post(turn.result.map_err(|error|format!("{error:?}")).unwrap(),g,heap);turn.work}}}
#[track_caller]fn refused<E:std::fmt::Debug>(poll:RetainedWorkPoll<RetainedCloneProgress,E>,heap:(usize,usize)){assert_eq!(heap,(0,0),"denied turn allocated");match poll{RetainedWorkPoll::Complete(turn)=>{assert_eq!(turn.work,0,"denied turn worked");assert_eq!(turn.result.map_err(|error|format!("{error:?}")).unwrap(),Default::default(),"denied turn progressed")},RetainedWorkPoll::Pending{..}=>panic!("denied turn parked")}}
fn driven_turn<F:Send+'static>(bank:&mut EffectBank,g:RetainedCloneGrant,work:u64,mut run:impl FnMut(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneProgress,F>)->RetainedWorkPoll<RetainedCloneProgress,F>{
    session_driven::<F>(bank,g,work.max(1),|effect,reply,_|Ok(match run(effect){RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result.map(RetainedCloneStep::Progress)),RetainedWorkPoll::Pending{..}=>reply.pending()})).0.map(|step|step.progress())
}
fn advance<T:RetireOwned,C:OwnedFieldReceiving<T>>(bank:&mut EffectBank,selection:OwnedAdvanceSelection,g:RetainedCloneGrant,cursor:&mut C,source:&mut Option<FieldValue>)->OwnedPoll{match selection{OwnedAdvanceSelection::Nested=>session_nested(bank,g,|control,witness|cursor.advance_nested(source,control,witness,BODY,MAXIMUM,g)),OwnedAdvanceSelection::Local{work}=>driven_turn::<OriginalReceivingPrimary>(bank,g,work,|effect|cursor.advance_local(source,BODY,effect))}}
fn retire<V:RetireOwned>(value:V,bank:&mut EffectBank){
    let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();
    for _ in 0..SETTLE{if owner.terminal_is_empty(){return}
        let d=owner.next_demand(BODY).unwrap();let w=owner.next_computational_work(BODY).unwrap();let g=RetainedCloneGrant{maximum_copy_bytes:BODY.max(d.copy_bytes),..grant(d)};
        let((poll,_),heap)=observe(||driven::<OriginalRetirementFailure>(bank,g,w.max(1),|effect,reply,recover|if recover{owner.recover_step_into(BODY,effect,reply)}else{owner.step_into(BODY,effect,reply)}));
        match poll{RetainedWorkPoll::Pending{..}=>{},RetainedWorkPoll::Complete(turn)=>bank.post(turn.result.map_err(|error|format!("{error:?}")).unwrap().progress(),g,heap)}
    }
    panic!("owned value failed its funded retirement");
}
fn close<T:RetireOwned,C:OwnedFieldReceiving<T>>(cursor:&mut C,bank:&mut EffectBank){
    for _ in 0..SETTLE{if cursor.terminal_is_empty(){return}
        let d=cursor.close_demands(BODY).unwrap();let w=cursor.close_work_demand(BODY).unwrap();let g=grant(d);
        for denied in denials(g,d){let(poll,heap)=observe(||driven_turn::<OriginalRetirementFailure>(bank,denied,w,|effect|cursor.close_step(effect)));refused(poll,heap);assert_eq!(cursor.close_demands(BODY).unwrap(),d,"denied close changed its quote");}
        let(poll,heap)=observe(||driven_turn::<OriginalRetirementFailure>(bank,g,w,|effect|cursor.close_step(effect)));post(bank,g,heap,poll);
    }
    panic!("cursor failed its funded close");
}
fn receive<T:RetireOwned,C:OwnedFieldReceiving<T>>(mut cursor:C,source:FieldValue,bank:&mut EffectBank,cut:Option<usize>,inspect:impl FnOnce(&T))->bool{
    let mut source=Some(source);let(mut turns,mut idle,mut parked)=(0,0u32,false);session_open(bank);
    while !cursor.is_ready()&&(parked||cut.is_none_or(|cut|turns<cut)){
        let d=cursor.demands(&source,BODY).unwrap();let selection=cursor.select_advance(&source,BODY,MAXIMUM).unwrap();let g=grant(d);
        if !parked{for denied in denials(g,d){let(poll,heap)=observe(||advance(bank,selection,denied,&mut cursor,&mut source));refused(poll,heap);assert_eq!(cursor.demands(&source,BODY).unwrap(),d,"denied turn changed its quote");}}
        let(poll,heap)=observe(||advance(bank,selection,g,&mut cursor,&mut source));parked=matches!(poll,RetainedWorkPoll::Pending{..});
        idle=if post(bank,g,heap,poll)==0{idle+1}else{0};assert!(idle<STALL,"cursor stalled: {STALL} funded turns performed no work");turns+=1;assert!(turns<SETTLE);
    }
    let received=cursor.is_ready();
    if received{
        let w=cursor.take_work_demand().unwrap();let g=grant(RetirementDemand{depth:1,..Default::default()});let mut output=None;
        for denied in denials(g,RetirementDemand{depth:1,..Default::default()}){let(poll,heap)=observe(||driven_turn::<semio_framework_value::ValueError>(bank,denied,w,|effect|cursor.take_into(&mut output,effect)));refused(poll,heap);assert!(output.is_none()&&cursor.is_ready(),"denied take lost the output");}
        let(poll,heap)=observe(||driven_turn::<semio_framework_value::ValueError>(bank,g,w,|effect|cursor.take_into(&mut output,effect)));post(bank,g,heap,poll);
        let output=output.expect("take publishes the output");inspect(&output);retire(output,bank);
    }
    close(&mut cursor,bank);assert!(cursor.terminal_is_empty());drop(cursor);if let Some(source)=source{retire(source,bank)}received
}
fn bank(law:&serde_json::Value)->EffectBank{EffectBank::new(Wallet::from_profile(&law["stock"]))}
fn text(value:&serde_json::Value)->String{value.as_str().unwrap().to_owned()}
fn dsl(value:&serde_json::Value)->DslValue{match value{serde_json::Value::Null=>DslValue::Null,serde_json::Value::Bool(flag)=>DslValue::Bool(*flag),serde_json::Value::Number(number)=>DslValue::Number(number.as_u64().map(Number::UInt).or_else(||number.as_i64().map(Number::Int)).unwrap_or_else(||Number::Float(number.as_f64().unwrap()))),serde_json::Value::String(text)=>DslValue::String(text.clone()),serde_json::Value::Array(items)=>DslValue::Array(items.iter().map(dsl).collect()),serde_json::Value::Object(entries)=>DslValue::Object(entries.iter().map(|(key,value)|(key.clone(),dsl(value))).collect())}}
fn json(value:&DslValue)->serde_json::Value{match value{DslValue::Null=>serde_json::Value::Null,DslValue::Bool(flag)=>serde_json::Value::Bool(*flag),DslValue::Number(Number::UInt(number))=>serde_json::json!(*number),DslValue::Number(Number::Int(number))=>serde_json::json!(*number),DslValue::Number(Number::Float(number))=>serde_json::json!(*number),DslValue::String(text)=>serde_json::Value::String(text.clone()),DslValue::Array(items)=>serde_json::Value::Array(items.iter().map(json).collect()),DslValue::Object(entries)=>serde_json::Value::Object(entries.iter().map(|(key,value)|(key.clone(),json(value))).collect()),_=>panic!("corpus literal has no JSON form")}}
fn cuts(law:&serde_json::Value)->impl Iterator<Item=Option<usize>>+'_{std::iter::once(None).chain(law["cancelAt"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()as usize)))}
fn cases<'a>(law:&'a serde_json::Value,kind:&'a str)->impl Iterator<Item=&'a serde_json::Value>+'a{law["cases"].as_array().unwrap().iter().filter(move|case|case["kind"]==kind)}
#[test]
fn set_receiving_equals_btreeset_and_sqlite_oracle_at_every_cancel_prefix(){
    let law=law();let mut completed=0;
    for case in cases(&law,"set"){
        let members:Vec<String>=case["source"].as_array().unwrap().iter().map(text).collect();let expected:Vec<String>=case["expected"].as_array().unwrap().iter().map(text).collect();
        assert_eq!(members.iter().cloned().collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>(),expected,"independent std oracle for {}",case["id"]);
        for cut in cuts(&law){let source=FieldValue::List(members.iter().cloned().map(FieldValue::Text).collect());let mut bank=bank(&law);
            completed+=usize::from(receive(<OrderedSet as DslOwnedReceivingField>::receiving_cursor(),source,&mut bank,cut,|set:&OrderedSet|assert_eq!(set.iter().cloned().collect::<Vec<_>>(),expected)));}
    }
    assert!(completed>0);
}
#[test]
fn map_receiving_equals_btreemap_oracle_at_every_cancel_prefix(){
    let law=law();let mut completed=0;
    for case in cases(&law,"map"){
        let pairs:Vec<(String,String)>=case["source"].as_array().unwrap().iter().map(|pair|(text(&pair[0]),text(&pair[1]))).collect();let expected:Vec<(String,String)>=case["expected"].as_array().unwrap().iter().map(|pair|(text(&pair[0]),text(&pair[1]))).collect();
        assert_eq!(pairs.iter().cloned().collect::<std::collections::BTreeMap<_,_>>().into_iter().collect::<Vec<_>>(),expected,"independent std oracle for {}",case["id"]);
        for cut in cuts(&law){let source=FieldValue::Map(pairs.iter().map(|(key,value)|(key.clone(),FieldValue::Text(value.clone()))).collect());let mut bank=bank(&law);
            completed+=usize::from(receive(<OrderedMap<String> as DslOwnedReceivingField>::receiving_cursor(),source,&mut bank,cut,|map:&OrderedMap<String>|assert_eq!(map.iter().map(|(key,value)|(key.clone(),value.clone())).collect::<Vec<_>>(),expected)));}
    }
    assert!(completed>0);
}
#[test]
fn value_receiving_equals_serde_json_at_every_cancel_prefix(){
    let law=law();let mut completed=0;
    for case in cases(&law,"value"){
        for cut in cuts(&law){let mut bank=bank(&law);
            completed+=usize::from(receive(NativeFieldReceiving::<DslValue>::default(),FieldValue::Value(dsl(&case["source"])),&mut bank,cut,|value:&DslValue|assert_eq!(json(value),case["expected"])));}
    }
    assert!(completed>0);
}
#[test]
fn block_receiving_equals_serde_json_record_at_every_cancel_prefix(){
    let law=law();let mut completed=0;
    for case in cases(&law,"block"){
        let id=text(&case["source"]["id"]);let weight=case["source"]["weight"].as_u64().unwrap();
        for cut in cuts(&law){let source=FieldValue::Record(RecordValue{fields:[(0,FieldValue::Text(id.clone())),(1,FieldValue::UInt(weight))].into_iter().collect()});let mut bank=bank(&law);
            completed+=usize::from(receive(NativeFieldReceiving::<Node>::default(),source,&mut bank,cut,|node:&Node|assert_eq!(serde_json::json!({"id":node.id,"weight":node.weight}),case["expected"])));}
    }
    assert!(completed>0);
}
