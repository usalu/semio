//! 🧪️ Original immutable policy, System-backed output births and every cancelled foreign prefix agree.
use super::*;
use crate::value::{ToValue,FromValue};
fn observe<T>(work:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,heap.requested_bytes,heap.released_bytes)}
fn policy(law:&serde_json::Value)->RetainedCloneGrant{let p=&law["grant"];RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:p["maximumDepth"].as_u64().unwrap()as usize}}
fn original(row:&serde_json::Value)->ForeignStep{ForeignStep{target:ForeignTarget{artifact_id:row["target"]["artifactId"].as_str().unwrap().into(),artifact_kind:row["target"]["artifactKind"].as_str().unwrap().into(),dialect:row["target"]["dialect"].as_str().map(String::from)},mutation_id:crate::ids::SchemaId(row["mutationId"].as_str().unwrap().into()),payload:row["payload"].as_array().unwrap().iter().map(|b|b.as_u64().unwrap()as u8).collect(),label:row["label"].as_str().unwrap().into()}}
fn close(owner:&mut dyn ErasedSnapshotRetirement,grant:RetainedCloneGrant)->usize{
 let(step,born,freed)=observe(||owner.close_step(RetainedCloneGrant::default()).unwrap());assert_eq!((born,freed),(0,0));assert_eq!(step.progress(),Default::default());
 let mut released=0;
 for _ in 0..16{if owner.terminal_is_empty(){return released;}
  let(demand,born,freed)=observe(||owner.next_release_byte_demand().unwrap());assert_eq!((born,freed),(0,0));assert!(demand<=grant.maximum_release_bytes);
  if demand>0{let short=RetainedCloneGrant{maximum_release_bytes:demand-1,..grant};let(step,born,freed)=observe(||owner.close_step(short).unwrap());assert_eq!((born,freed),(0,0));assert_eq!(step.progress(),Default::default());}
  let(step,born,freed)=observe(||owner.close_step(grant).unwrap());assert_eq!(born,0);assert!(step.progress().fits(grant));assert_eq!(freed,step.progress().released_bytes);released+=freed;
 }
 panic!("fixed original foreign closure policy did not reach terminal");
}
#[test]
fn original_foreign_output_preserves_immutable_policy_and_every_cancelled_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let grant=policy(&law);
 for row in law["cases"].as_array().unwrap(){let original=original(row);let source=ForeignStepSource::borrowed(&original);let identity=source.identity();
  let(result,born,freed)=observe(||ForeignStepCopy::admit(source,RetainedCloneGrant::default()));assert_eq!((born,freed),(0,0));let(_,returned)=result.err().unwrap();assert_eq!(returned.identity(),identity);
  for cut in 0..48{let((mut cursor,ingress),born,freed)=observe(||ForeignStepCopy::admit(source,grant).ok().unwrap());assert_eq!((born,freed),(0,0));assert!(ingress.fits(grant));let mut allocated=0;let mut copied=0;
   for _ in 0..cut{let(demand,born,freed)=observe(||cursor.next_capacity_byte_demand(source).unwrap());assert_eq!((born,freed),(0,0));
    if demand>0{let(step,born,freed)=observe(||cursor.advance(source,RetainedCloneGrant{maximum_capacity_bytes:demand-1,..grant}).unwrap());assert_eq!((born,freed),(0,0));assert_eq!(step.progress(),Default::default());}
    let copy=cursor.next_copy_byte_demand(source).unwrap();if copy>0{let(step,born,freed)=observe(||cursor.advance(source,RetainedCloneGrant{maximum_copy_bytes:copy-1,..grant}).unwrap());assert_eq!((born,freed),(0,0));assert_eq!(step.progress(),Default::default());}
    let(step,born,freed)=observe(||cursor.advance(source,grant).unwrap());assert_eq!(freed,0);assert!(step.progress().fits(grant));assert_eq!(born,step.progress().retained_capacity_bytes);allocated+=born;copied+=step.progress().copied_bytes;
    if matches!(step,RetainedCloneStep::Complete(_)){break;}
   }
   assert_eq!(source.identity(),identity);cursor.cancel();assert!(cursor.take(grant).is_none());assert_eq!(cursor.advance(source,grant).unwrap().progress(),Default::default());assert_eq!(close(&mut cursor,grant),allocated);let(_,born,freed)=observe(||drop(cursor));assert_eq!((born,freed),(0,0));assert!(copied<=original.target.artifact_id.len()+original.target.artifact_kind.len()+original.target.dialect.as_ref().map_or(0,String::len)+original.mutation_id.0.len()+original.payload.len()+original.label.len());
  }
  let(mut cursor,_)=ForeignStepCopy::admit(source,grant).ok().unwrap();let mut allocated=0;let mut copied=0;
  for _ in 0..128{let(step,born,freed)=observe(||cursor.advance(source,grant).unwrap());assert_eq!(freed,0);assert_eq!(born,step.progress().retained_capacity_bytes);allocated+=born;copied+=step.progress().copied_bytes;if matches!(step,RetainedCloneStep::Complete(_)){break;}}
  let(transferred,born,freed)=observe(||cursor.take(grant).unwrap());assert_eq!((born,freed),(0,0));let(output,progress)=transferred;assert!(progress.fits(grant));assert_eq!(output,original);assert_eq!(copied,original.target.artifact_id.len()+original.target.artifact_kind.len()+original.target.dialect.as_ref().map_or(0,String::len)+original.mutation_id.0.len()+original.payload.len()+original.label.len());
  assert_eq!(ForeignStep::from_value(output.to_value()).unwrap(),output);
  let mut wire=serde_json::json!({"target":{"artifactId":output.target.artifact_id,"artifactKind":output.target.artifact_kind},"mutationId":output.mutation_id.0,"payload":output.payload,"label":output.label});if let Some(dialect)=&output.target.dialect{wire["target"]["dialect"]=serde_json::json!(dialect);}let mut expected=row.clone();expected.as_object_mut().unwrap().remove("id");assert_eq!(wire,expected);
  cursor.begin_close();assert_eq!(close(&mut cursor,grant),0);let mut retirement=ForeignStepRetirement::admit(output,grant).ok().unwrap().0;assert_eq!(close(&mut retirement,grant),allocated);
 }
 eprintln!("[DEBUG] original foreign outputs2 fixed five-axis policy, System/Serde identity, every48 cancel cuts, exact backing birth/release and allocation-free handoff");
}

#[test]
fn original_foreign_collection_keeps_each_refused_row_and_real_backing_receipt(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let grant=policy(&serde_json::json!({"grant":law["collectionGrant"]}));
 for count in 0..=law["cases"].as_array().unwrap().len(){
  let(mut owner,born,freed)=observe(ForeignStepsOwner::empty);assert_eq!((born,freed),(0,0));let mut live=0;
  for row in law["cases"].as_array().unwrap().iter().take(count){
   let(value,born,freed)=observe(||original(row));assert_eq!(freed,0);live+=born;let identity=ForeignStepSource::borrowed(&value).identity();
   let(demand,born,freed)=observe(||owner.append_demands().unwrap());assert_eq!((born,freed),(0,0));
   let denied=RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant};let(result,born,freed)=observe(||owner.try_append(value,denied));assert_eq!((born,freed),(0,0));let(_,value)=result.err().unwrap();assert_eq!(ForeignStepSource::borrowed(&value).identity(),identity);
   let index=owner.len();let(result,born,freed)=observe(||owner.try_append(value,grant));let p=result.unwrap();assert!(p.fits(grant));assert_eq!(born,p.retained_capacity_bytes);assert_eq!(freed,p.released_bytes);live+=born;live-=freed;assert_eq!(ForeignStepSource::borrowed(owner.get(index).unwrap()).identity(),identity);
  }
  owner.begin_close();let mut released=0;
  for _ in 0..64{if owner.terminal_is_empty(){break;}let(demand,born,freed)=observe(||owner.next_release_byte_demand().unwrap());assert_eq!((born,freed),(0,0));
   if demand>0{let(step,born,freed)=observe(||owner.close_step(RetainedCloneGrant{maximum_release_bytes:demand-1,..grant}).unwrap());assert_eq!((born,freed),(0,0));assert_eq!(step.progress(),Default::default());}
   let(step,born,freed)=observe(||owner.close_step(grant).unwrap());assert_eq!(born,0);assert!(step.progress().fits(grant));assert_eq!(freed,step.progress().released_bytes);released+=freed;
  }
  assert!(owner.terminal_is_empty());assert_eq!(released,live);let(_,born,freed)=observe(||drop(owner));assert_eq!((born,freed),(0,0));
 }
 eprintln!("[DEBUG] original foreign sequence2 fixed five-axis policy, denied original row identity, paid real contiguous transfers and every prefix System release conserved");
}
