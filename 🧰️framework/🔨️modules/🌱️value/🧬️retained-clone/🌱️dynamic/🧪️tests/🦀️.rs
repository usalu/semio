use super::*;
use crate::{Number,value::observe_retirement_allocations as observe};

fn decode(value:&serde_json::Value)->DslValue {
    match value["kind"].as_str().unwrap(){
        "null"=>DslValue::Null,
        "bool"=>DslValue::Bool(value["value"].as_bool().unwrap()),
        "uint"=>DslValue::Number(Number::UInt(value["value"].as_str().unwrap().parse().unwrap())),
        "int"=>DslValue::Number(Number::Int(value["value"].as_str().unwrap().parse().unwrap())),
        "float"=>DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(value["bits"].as_str().unwrap(),16).unwrap()))),
        "text"=>DslValue::String(value["value"].as_str().unwrap().into()),
        "bytes"=>DslValue::Bytes(value["value"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect()),
        "array"=>DslValue::Array(value["value"].as_array().unwrap().iter().map(decode).collect()),
        "object"=>DslValue::Object(value["value"].as_array().unwrap().iter().map(|entry|(entry[0].as_str().unwrap().into(),decode(&entry[1]))).collect()),
        _=>unreachable!(),
    }
}

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

fn grant(demand:RetirementDemand,copy:usize)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

fn denied(grant:RetainedCloneGrant,demand:RetirementDemand)->Vec<RetainedCloneGrant>{
    [Some(RetainedCloneGrant {maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant {maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant {maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth>0).then_some(RetainedCloneGrant {maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten().collect()
}

fn check(result:Result<RetainedCloneStep,ValueError>,heap:(usize,usize),grant:RetainedCloneGrant)->RetainedCloneProgress {
    let progress=result.unwrap_or_else(|error|panic!("[DEBUG] Original dynamic granted step refused {:?}: {}",error.kind,error)).progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));progress
}

fn check_denied(result:Result<RetainedCloneStep,ValueError>,heap:(usize,usize),grant:RetainedCloneGrant,demand:RetirementDemand){
    assert_eq!(heap,(0,0));match result {Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>{assert!(grant.maximum_depth<demand.depth);assert_eq!(error.kind,ValueRefusalKind::DepthLimit);assert_eq!(error.retained_progress(),Default::default());}}
}

fn close_cursor(cursor:&mut DslValueRetainedCloneCursor,copy:usize,born:&mut usize,freed:&mut usize){
    assert_eq!(observe(||cursor.begin_close()).1,(0,0));let mut stalled=0;
    for _ in 0..100000 {
        if cursor.terminal_is_empty(){return;}
        let(demand,heap)=observe(||cursor.close_demands(copy).unwrap());assert_eq!(heap,(0,0));let admitted=grant(demand,copy);
        for denied in denied(admitted,demand){let(result,heap)=observe(||cursor.close_step(denied));check_denied(result,heap,denied,demand);assert_eq!(cursor.close_demands(copy).unwrap(),demand);}
        if demand.release_bytes>8 {let denied=RetainedCloneGrant {maximum_release_bytes:8,..admitted};let(result,heap)=observe(||cursor.close_step(denied));check_denied(result,heap,denied,demand);}
        let(result,heap)=observe(||cursor.close_step(admitted));let progress=check(result,heap,admitted);*born+=heap.0;*freed+=heap.1;stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"[DEBUG] Original dynamic cursor closure stalled copy={copy} demand={demand:?}");
    }
    panic!("[DEBUG] Original dynamic cursor did not close its retained original graph");
}

fn close_source(source:&mut RetainedCloneSource<DslValue>,copy:usize,born:&mut usize,freed:&mut usize){
    let mut stalled=0;
    for _ in 0..100000 {
        if source.terminal_is_empty(){return;}
        let demand=RetirementDemand {copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()};let admitted=grant(demand,copy);
        if demand.release_bytes>8 {let denied=RetainedCloneGrant {maximum_release_bytes:8,..admitted};let(result,heap)=observe(||source.close_step(denied));check_denied(result,heap,denied,demand);}
        let(result,heap)=observe(||source.close_step(admitted));let progress=check(result,heap,admitted);*born+=heap.0;*freed+=heap.1;stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"[DEBUG] Original dynamic source closure stalled copy={copy} demand={demand:?}");
    }
    panic!("[DEBUG] Original dynamic source graph remained live");
}

fn close_output(value:DslValue,copy:usize,born:&mut usize,freed:&mut usize){
    let admitted=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:crate::owned_retirement_birth_bytes::<DslValue>(),maximum_release_bytes:0,maximum_depth:1};
    let(result,heap)=observe(||crate::admit_owned_retirement(value,admitted));let(ticket,progress)=result.map_err(|(error,_)|error).unwrap();assert!(progress.fits(admitted));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));*born+=heap.0;*freed+=heap.1;let mut ticket=Some(ticket);let mut stalled=0;
    for _ in 0..100000 {
        let Some(owner)=ticket.as_ref()else{assert_eq!(observe(||drop(ticket)).1,(0,0));return;};let demand=crate::factory_ticket_demands(owner,copy).unwrap();let admitted=grant(demand,copy);
        if demand.release_bytes>8 {let denied=RetainedCloneGrant {maximum_release_bytes:8,..admitted};let(result,heap)=observe(||crate::close_factory_ticket(&mut ticket,denied));check_denied(result,heap,denied,demand);}
        let(result,heap)=observe(||crate::close_factory_ticket(&mut ticket,admitted));let progress=check(result,heap,admitted);*born+=heap.0;*freed+=heap.1;stalled=if progress==Default::default(){stalled+1}else{0};assert!(stalled<=143,"[DEBUG] Original dynamic output closure stalled copy={copy} demand={demand:?}");
    }
    panic!("[DEBUG] Original dynamic output graph remained live");
}

#[test]
fn original_dynamic_value_clone_fullgrant_and_original_custody(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut cases=law["cases"].as_array().unwrap().iter().map(|row|(row["name"].as_str().unwrap().to_owned(),row["value"].clone())).collect::<Vec<_>>();
    for depth in law["depths"].as_array().unwrap(){let depth=depth.as_u64().unwrap()as usize;let mut value=serde_json::json!({"kind":"text","value":"ä日🌱"});for _ in 0..depth{value=serde_json::json!({"kind":"array","value":[value]});}cases.push((format!("depth-{depth}"),value));}
    for(name,encoded)in cases {for copy in [1,3,64] {for cancelled in [None,Some(0),Some(1),Some(3),Some(8),Some(17)]{
        let(owner,heap)=observe(||decode(&encoded));let(mut born,mut freed)=heap;let mut original=Vec::new();pointers(&owner,&mut original);
        let demand=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();let admitted=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes(),maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};
        let(result,heap)=observe(||RetainedCloneSource::admit_owned(owner,(),admitted));let(mut source,progress)=result.map_err(|(error,_,_)|error).unwrap();assert!(progress.fits(admitted));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;freed+=heap.1;
        let root=source.borrow().get()as *const DslValue as usize;let mut observed=Vec::new();pointers(source.borrow().get(),&mut observed);assert_eq!(observed,original);
        let(mut cursor,heap)=observe(DslValue::retained_clone_cursor);assert_eq!(heap,(0,0));let mut output=None;let mut turns=0;
        for _ in 0..100000 {
            if cancelled==Some(turns){break;}
            let(demand,heap)=observe(||cursor.normal_demands(source.borrow(),64).unwrap());assert_eq!(heap,(0,0));let admitted=grant(demand,64);
            for denied in denied(admitted,demand){let bound=cursor.state.source.is_some();let(result,heap)=observe(||cursor.advance(source.borrow(),denied));check_denied(result,heap,denied,demand);assert_eq!(cursor.state.source.is_some(),bound);assert_eq!(cursor.normal_demands(source.borrow(),64).unwrap(),demand);assert_eq!(source.borrow().get()as *const DslValue as usize,root);}
            let(result,heap)=observe(||cursor.advance(source.borrow(),admitted));let done=matches!(result,Ok(RetainedCloneStep::Complete(_)));let progress=check(result,heap,admitted);assert_eq!(progress.copied_items,1);born+=heap.0;freed+=heap.1;turns+=1;
            if done {let(value,heap)=observe(||cursor.take());assert_eq!(heap,(0,0));let value=value.unwrap();assert_eq!(encode(&value),encoded);output=Some(value);break;}
        }
        assert!(output.is_some()||cancelled==Some(turns),"[DEBUG] Original dynamic normal clone stalled {name} cancel={cancelled:?}");assert_eq!(encode(source.borrow().get()),encoded);observed.clear();pointers(source.borrow().get(),&mut observed);assert_eq!(observed,original);
        close_cursor(&mut cursor,copy,&mut born,&mut freed);assert_eq!(observe(||drop(cursor)).1,(0,0));
        if let Some(output)=output {close_output(output,copy,&mut born,&mut freed);}
        close_source(&mut source,copy,&mut born,&mut freed);assert_eq!(observe(||drop(source)).1,(0,0));assert_eq!(born,freed,"[DEBUG] Original dynamic physical owner conservation {name} copy={copy} cancel={cancelled:?}");
        println!("[DEBUG] Original dynamic value {name} closeCopy={copy} cancel={cancelled:?} sourceIdentity=true fiveAxes=true physical={born} terminalDrop=0");
    }}}
}
