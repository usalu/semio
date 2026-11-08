//! 🧪️ Native original field equality preserves exact intrinsic semantics and all funded cursor backing.
use super::*;
use crate::observe_retirement_allocations as observe;
fn value(row:&serde_json::Value)->DslValue{let v=&row["value"];match row["type"].as_str().unwrap(){
 "null"=>DslValue::Null,"bool"=>DslValue::Bool(v.as_bool().unwrap()),"uint"=>DslValue::uint(v.as_str().unwrap().parse().unwrap()),"int"=>DslValue::int(v.as_str().unwrap().parse().unwrap()),
 "float"=>DslValue::float(f64::from_bits(u64::from_str_radix(v.as_str().unwrap(),16).unwrap())),"text"=>DslValue::String(v.as_str().unwrap().into()),
 "bytes"=>DslValue::Bytes(v.as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect()),"array"=>DslValue::Array(v.as_array().unwrap().iter().map(value).collect()),
 "object"=>DslValue::Object(v.as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().into(),value(&pair[1]))).collect()),_=>panic!("unknown closed intrinsic fixture")
}}
fn exact(d:RetirementDemand,items:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn measured<R,E:std::fmt::Debug>(stock:RetainedCloneGrant,total:&mut RetainedCloneProgress,g:RetainedCloneGrant,quoted:u64,run:impl FnOnce()->RetainedWorkOutcome<(R,RetainedCloneProgress),E>)->R{
 let(turn,heap)=observe(run);let work=turn.work;let(value,p)=turn.result.unwrap();assert!(work<=u128::from(quoted));assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));*total=total.checked_add(p).unwrap();assert!(total.fits(stock));value
}
#[test]
fn original_value_equality_matches_neutral_intrinsic_partial_eq_and_physical_cancellation(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let stock:RetainedCloneGrant=serde_json::from_value(corpus["stock"].clone()).unwrap();let body=corpus["bodyCopyBytes"].as_u64().unwrap()as usize;let maximum_work=corpus["maximumWorkUnits"].as_u64().unwrap();
 for row in corpus["cases"].as_array().unwrap(){for profile in [1usize,512]{for cut in [None,Some(0usize),Some(1),Some(2),Some(7)]{
  let left=value(&row["left"]);let right=value(&row["right"]);assert_eq!(left==right,row["equal"].as_bool().unwrap());let(mut cursor,heap)=observe(OriginalValueEquality::new);assert_eq!(heap,(0,0));let mut total=RetainedCloneProgress::default();let mut answer=None;
  for turn in 0..16384{if cut==Some(turn){break}let d=cursor.next_demands(&left,&right,body,maximum_work).unwrap();let work=cursor.work_demand(&left,&right,body,maximum_work).unwrap();let g=exact(d,profile);let(denied,heap)=observe(||crate::eff::adv_value(&mut cursor,&left,&right,body,0,g));assert_eq!(denied.work,0);assert!(denied.result.unwrap().equal.is_none());assert_eq!(heap,(0,0));
   for axis in 0..5{let mut denied=g;let number=match axis{0=>&mut denied.maximum_items,1=>&mut denied.maximum_copy_bytes,2=>&mut denied.maximum_capacity_bytes,3=>&mut denied.maximum_release_bytes,_=>&mut denied.maximum_depth};if *number==0{continue}*number-=1;if axis==0&&denied.maximum_items>0{continue}
    let(turn,heap)=observe(||crate::eff::adv_value(&mut cursor,&left,&right,body,maximum_work,denied));assert_eq!(turn.work,0);let step=turn.result.unwrap();assert_eq!(step.progress,Default::default());assert!(step.equal.is_none());assert_eq!(heap,(0,0));}
   answer=measured(stock,&mut total,g,work,||{let turn=crate::eff::adv_value(&mut cursor,&left,&right,body,maximum_work,g);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|step|(step.equal,step.progress))}});if answer.is_some(){break}assert!(turn+1<16384,"{} profile={profile} cut={cut:?} d={d:?} work={work} total={total:?}",row["name"]);
  }
  cursor.begin_close();for turn in 0..16384{if cursor.terminal_is_empty(){break}let d=cursor.next_close_demands(body).unwrap();let work=cursor.next_close_work_demand(body).unwrap();let g=exact(d,profile);measured(stock,&mut total,g,work,||{let turn=crate::eff::close_value(&mut cursor,g);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|step|((),step.progress()))}});assert!(turn+1<16384)}
  assert!(cursor.terminal_is_empty());let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));assert_eq!(total.retained_capacity_bytes,total.released_bytes);if cut.is_none(){assert_eq!(answer,Some(row["equal"].as_bool().unwrap()),"{}",row["name"])}
 }}}
}
#[test]
fn original_value_equality_rejects_equal_but_substituted_source_identity(){
 let left=DslValue::String("original".into());let right=DslValue::String("original".into());let changed=left.clone();let mut cursor=OriginalValueEquality::new();let d=cursor.next_demands(&left,&right,512,64).unwrap();assert!(crate::eff::adv_value(&mut cursor,&left,&right,512,64,exact(d,1)).result.unwrap().equal.is_none());
 let(result,heap)=observe(||cursor.next_demands(&changed,&right,512,64));assert!(result.is_err());assert_eq!(heap,(0,0));cursor.begin_close();while !cursor.terminal_is_empty(){let d=cursor.next_close_demands(512).unwrap();crate::eff::close_value(&mut cursor,exact(d,1)).result.unwrap();}assert!(cursor.terminal_is_empty());
}
