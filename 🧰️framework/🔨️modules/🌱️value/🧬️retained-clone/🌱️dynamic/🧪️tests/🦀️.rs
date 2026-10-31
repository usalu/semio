use semio_framework_test_fixtures::fixture_text;
use super::*;
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

fn close_output(value:DslValue,copy:usize,born:&mut usize,freed:&mut usize){
    let admitted=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.max(size_of::<DslValue>()),maximum_capacity_bytes:crate::owned_retirement_birth_bytes::<DslValue>(),maximum_release_bytes:0,maximum_depth:1};
    let(result,heap)=observe(||crate::admit_owned_retirement(value,admitted));let work=result.work;let(ticket,progress)=result.result.map_err(|(error,_)|error).unwrap_or_else(|error|panic!("Original output birth work={work}: {error}"));assert!(work>0);assert!(progress.fits(admitted));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));*born+=heap.0;*freed+=heap.1;let mut ticket=Some(ticket);let mut stalled=0;
    for _ in 0..100000 {
        let Some(owner)=ticket.as_ref()else{assert_eq!(observe(||drop(ticket)).1,(0,0));return;};let demand=crate::factory_ticket_demands(owner,copy).unwrap();let admitted=grant(demand,copy);
        if demand.release_bytes>8 {let denied=RetainedCloneGrant {maximum_release_bytes:8,..admitted};let(result,heap)=observe(||crate::close_factory_ticket(&mut ticket,denied));check_denied(result,heap,denied,demand);}
        let(result,heap)=observe(||crate::close_factory_ticket(&mut ticket,admitted));let progress=check(result,heap,admitted);*born+=heap.0;*freed+=heap.1;stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"Original dynamic output closure stalled copy={copy} demand={demand:?}");
    }
    panic!("Original dynamic output graph remained live");
}

#[test]
fn original_dynamic_value_clone_fullgrant_and_original_custody(){
    for(name,encoded)in cases(){for copy in [1,3,64]{for cancelled in [None,Some(0),Some(1),Some(3),Some(8),Some(17)]{
        let(owner,heap)=observe(||decode(&encoded));let(mut born,mut freed)=heap;let mut original=Vec::new();pointers(&owner,&mut original);
        let(mut source,heap)=admit_source(owner);born+=heap.0;freed+=heap.1;let root=source.borrow().get()as*const DslValue as usize;let mut observed=Vec::new();pointers(source.borrow().get(),&mut observed);assert_eq!(observed,original);
        let(mut cursor,heap)=observe(DslValue::retained_clone_cursor);assert_eq!(heap,(0,0));let actual=advance_original(&source,&mut cursor,cancelled,|_,_|{});born+=actual.born;freed+=actual.freed;let turns=actual.turns;let mut output=None;
        if actual.complete{let(value,heap)=observe(||cursor.take());assert_eq!(heap,(0,0));let value=value.unwrap();assert_eq!(encode(&value),encoded);output=Some(value);}
        assert!(output.is_some()||cancelled==Some(turns),"Original dynamic normal clone stalled {name} cancel={cancelled:?}");assert_eq!(encode(source.borrow().get()),encoded);observed.clear();pointers(source.borrow().get(),&mut observed);assert_eq!(observed,original);assert_eq!(source.borrow().get()as*const DslValue as usize,root);
        close_cursor(&mut cursor,copy,&mut born,&mut freed);assert_eq!(observe(||drop(cursor)).1,(0,0));if let Some(output)=output{close_output(output,copy,&mut born,&mut freed);}close_source(&mut source,copy,&mut born,&mut freed);assert_eq!(observe(||drop(source)).1,(0,0));assert_eq!(born,freed,"Original dynamic physical owner conservation {name} copy={copy} cancel={cancelled:?}");
    }}}
}

#[test]
fn selected_dynamic_close_preserves_body_before_narrow_grant(){
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🌱️dynamic/🧪️tests/🧫️fixtures/selected.json")).unwrap();let body=law["body"].as_u64().unwrap()as usize;let bound=law["maximumTurns"].as_u64().unwrap()as usize;
    let(owner,heap)=observe(||decode(&law["value"]));let(mut born,mut freed)=heap;
    let demand=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes(),maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};
    let(outcome,heap)=observe(||RetainedCloneSource::admit_owned(owner,(),g));let work=outcome.work;let(mut source,p)=outcome.result.map_err(|(error,_,_)|error).unwrap();assert!(work>0);assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;
    let identity=source.borrow().get()as*const DslValue;let mut cursor=DslValue::retained_clone_cursor();
    for _ in 0..bound{if cursor.output_ready(){break}let d=cursor.advance_demands(source.borrow(),body).unwrap();let g=grant(d,body);let(outcome,heap)=observe(||cursor.advance(source.borrow(),body,g));check(outcome,heap,g);born+=heap.0;freed+=heap.1;}
    assert!(cursor.output_ready());cursor.begin_close();let mut reached=false;
    for _ in 0..bound{let d=cursor.close_demands(body).unwrap();if d.copy_bytes==body&&d.capacity_bytes==0&&d.release_bytes==0{reached=true;break}let g=grant(d,body);let(outcome,heap)=observe(||cursor.close_selected(body,g));check(outcome,heap,g);born+=heap.0;freed+=heap.1;}
    assert!(reached);let d=cursor.close_demands(body).unwrap();let denied=RetainedCloneGrant{maximum_copy_bytes:law["deniedCopy"].as_u64().unwrap()as usize,..grant(d,body)};
    let(outcome,heap)=observe(||cursor.close_selected(body,denied));let work=outcome.work;let p=outcome.result.unwrap().progress();assert_eq!(heap,(0,0));assert_eq!(serde_json::json!({"work":work,"copiedBytes":p.copied_bytes}),law["selected"]);assert_eq!(p,Default::default());assert_eq!(cursor.close_demands(body).unwrap(),d);
    let(outcome,heap)=observe(||cursor.close_step(denied));let work=outcome.work;let p=check(outcome,heap,denied);assert_eq!(heap,(0,0));let remaining=cursor.close_demands(body).unwrap().copy_bytes;let mut oracle=law["value"]["value"].as_array().unwrap().clone();oracle.truncate(oracle.len()-denied.maximum_copy_bytes);assert_eq!(remaining,oracle.len());assert_eq!(serde_json::json!({"work":work,"copiedBytes":p.copied_bytes,"remaining":remaining}),law["raw"]);
    assert_eq!(source.borrow().get()as*const DslValue,identity);assert_eq!(encode(source.borrow().get()),law["value"]);close_cursor(&mut cursor,body,&mut born,&mut freed);let(cached,heap)=observe(||cursor.close_selected(body,Default::default()));assert_eq!(cached.work,0);assert_eq!(heap,(0,0));assert!(matches!(cached.result,Ok(RetainedCloneStep::Complete(p))if p==Default::default()));assert_eq!(observe(||drop(cursor)).1,(0,0));close_source(&mut source,body,&mut born,&mut freed);assert_eq!(observe(||drop(source)).1,(0,0));assert_eq!(born,freed);
}
