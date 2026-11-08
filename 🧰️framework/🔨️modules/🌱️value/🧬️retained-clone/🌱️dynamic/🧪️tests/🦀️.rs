const MAXIMUM_CLONE_WORK:u64=4096;
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::Number;
use super::protocol::*;
use crate::observe_retirement_allocations as observe;

fn encode(value:&DslValue)->serde_json::Value {
    use serde_json::json;
    match value {
        DslValue::Null=>json!({"kind":"null"}),
        DslValue::Bool(value)=>json!({"kind":"bool","value":value}),
        DslValue::Number(Number::UInt(value))=>json!({"kind":"uint","value":value.to_string()}),
        DslValue::Number(Number::Int(value))=>json!({"kind":"int","value":value.to_string()}),
        DslValue::Number(Number::Float(value))=>json!({"kind":"float","bits":format!("{:016x}",value.to_bits())}),
        DslValue::String(value)=>json!({"kind":"text","value":value}),
        DslValue::Bytes(value)=>json!({"kind":"bytes","value":value}),
        DslValue::Array(values)=>json!({"kind":"array","value":values.iter().map(encode).collect::<Vec<_>>()}),
        DslValue::Object(values)=>json!({"kind":"object","value":values.iter().map(|(key,value)|json!([key,encode(value)])).collect::<Vec<_>>()}),
    }
}

fn pointers(value:&DslValue,output:&mut Vec<(usize,usize,usize)>){
    match value {
        DslValue::String(value)=>output.push((value.as_ptr()as usize,value.len(),value.capacity())),
        DslValue::Bytes(value)=>output.push((value.as_ptr()as usize,value.len(),value.capacity())),
        DslValue::Array(values)=>{output.push((values.as_ptr()as usize,values.len(),values.capacity()));for value in values{pointers(value,output);}},
        DslValue::Object(values)=>{output.push((values.as_ptr()as usize,values.len(),values.capacity()));for(key,value)in values{output.push((key.as_ptr()as usize,key.len(),key.capacity()));pointers(value,output);}},
        _=>{},
    }
}

#[test]
fn original_dynamic_value_clone_fullgrant_and_original_custody(){
    for(name,encoded)in cases(){for copy in [1,3,64]{for cancelled in [None,Some(0),Some(1),Some(3),Some(8),Some(17)]{
        let(owner,heap)=observe(||decode(&encoded));let(mut born,mut freed)=heap;let mut original=Vec::new();pointers(&owner,&mut original);
        let(mut source,heap)=admit_source(owner);born+=heap.0;freed+=heap.1;let root=source.borrow().get()as*const DslValue as usize;let mut observed=Vec::new();pointers(source.borrow().get(),&mut observed);assert_eq!(observed,original);
        let(mut cursor,heap)=observe(||Harness::new(&source));assert_eq!(heap,(0,0));let actual=advance_original(&mut source,&mut cursor,cancelled,|_,_|{});born+=actual.born;freed+=actual.freed;let turns=actual.turns;let mut output=false;
        if actual.complete{let maximum=source.wallet().remaining_work();let(d,_)=cursor.quote(&source,Op::Take,0,maximum);let take=grant(d,0);let(step,heap)=drive(&mut source,&mut cursor,Op::Take,0,take);assert_eq!(heap,(0,0));assert!(matches!(step,RetainedCloneStep::Complete(_)));cursor.inspect_output(|value|assert_eq!(encode(value.unwrap()),encoded));output=true;}
        assert!(output||cancelled==Some(turns),"Original dynamic normal clone stalled {name} cancel={cancelled:?}");assert_eq!(encode(source.borrow().get()),encoded);observed.clear();pointers(source.borrow().get(),&mut observed);assert_eq!(observed,original);assert_eq!(source.borrow().get()as*const DslValue as usize,root);
        close_cursor(&mut source,&mut cursor,copy,&mut born,&mut freed);close_output(&mut source,&mut cursor,copy,&mut born,&mut freed);cursor.release(&mut source);assert_eq!(observe(||drop(cursor)).1,(0,0));close_source(&mut source,copy,&mut born,&mut freed);assert_eq!(observe(||drop(source)).1,(0,0));assert_eq!(born,freed,"Original dynamic physical owner conservation {name} copy={copy} cancel={cancelled:?}");
    }}}
}

#[test]
fn selected_dynamic_close_preserves_body_before_narrow_grant(){
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🌱️dynamic/🧪️tests/🧫️fixtures/selected.json")).unwrap();let header=std::mem::size_of::<usize>();assert_eq!(header,law["lengthHeaderBytes"].as_u64().unwrap()as usize);let body=(law["body"].as_u64().unwrap()as usize).checked_add(header).unwrap();let bound=law["maximumTurns"].as_u64().unwrap()as usize;
    let(owner,heap)=observe(||decode(&law["value"]));let(mut born,mut freed)=heap;
    let(mut source,heap)=admit_source(owner);born+=heap.0;freed+=heap.1;let identity=source.borrow().get()as*const DslValue;let mut cursor=Harness::new(&source);
    for _ in 0..bound{if cursor.ready(){break}let maximum=source.wallet().remaining_work();let(d,_)=cursor.quote(&source,Op::Advance,body,maximum);let g=grant(d,body);let(step,heap)=drive(&mut source,&mut cursor,Op::Advance,body,g);check(step,heap,g);born+=heap.0;freed+=heap.1;}
    assert!(cursor.ready());let maximum=source.wallet().remaining_work();let(d,_)=cursor.quote(&source,Op::Start,0,maximum);let(step,heap)=drive(&mut source,&mut cursor,Op::Start,0,grant(d,0));check(step,heap,grant(d,0));born+=heap.0;freed+=heap.1;
let mut reached=false;
    for _ in 0..bound{let d=cursor.quote(&source,Op::Close,body,source.wallet().remaining_work()).0;if d.copy_bytes==body&&d.capacity_bytes==0&&d.release_bytes==0{reached=true;break}let g=grant(d,body);let(step,heap)=drive(&mut source,&mut cursor,Op::Close,body,g);check(step,heap,g);born+=heap.0;freed+=heap.1;}
    assert!(reached);let d=cursor.quote(&source,Op::Close,body,source.wallet().remaining_work()).0;let denied_payload=law["deniedCopy"].as_u64().unwrap()as usize;let denied=RetainedCloneGrant{maximum_items:law["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:denied_payload.checked_add(header).unwrap(),..grant(d,body)};
    let before=source.wallet().remaining_work();let(step,heap)=drive(&mut source,&mut cursor,Op::Close,body,denied);let work=before-source.wallet().remaining_work();let p=step.progress();assert_eq!(heap,(0,0));assert_eq!(serde_json::json!({"work":work,"copiedBytes":p.copied_bytes}),law["selected"]);assert_eq!(p,Default::default());assert_eq!(cursor.quote(&source,Op::Close,body,source.wallet().remaining_work()).0,d);
    let before=source.wallet().remaining_work();let(step,heap)=drive(&mut source,&mut cursor,Op::Close,denied.maximum_copy_bytes,denied);let work=before-source.wallet().remaining_work();let p=check(step,heap,denied);assert_eq!(heap,(0,0));assert_eq!(p.copied_bytes,denied.maximum_copy_bytes);let remaining=cursor.quote(&source,Op::Close,body,source.wallet().remaining_work()).0.copy_bytes.checked_sub(header).unwrap();let mut oracle=law["value"]["value"].as_array().unwrap().clone();oracle.truncate(oracle.len()-denied_payload);assert_eq!(remaining,oracle.len());assert_eq!(serde_json::json!({"work":work,"copiedBytes":p.copied_bytes.checked_sub(header).unwrap(),"remaining":remaining}),law["raw"]);
    assert_eq!(source.borrow().get()as*const DslValue,identity);assert_eq!(encode(source.borrow().get()),law["value"]);close_cursor(&mut source,&mut cursor,body,&mut born,&mut freed);let(cached,heap)=drive(&mut source,&mut cursor,Op::Close,body,Default::default());assert_eq!(cached.progress(),Default::default());assert_eq!(heap,(0,0));assert!(matches!(cached,RetainedCloneStep::Complete(p)if p==Default::default()));cursor.release(&mut source);assert_eq!(observe(||drop(cursor)).1,(0,0));close_source(&mut source,body,&mut born,&mut freed);assert_eq!(observe(||drop(source)).1,(0,0));assert_eq!(born,freed);
}
