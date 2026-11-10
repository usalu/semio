use super::*;
#[path="⏱️effect/🦀️.rs"]
mod effect;
use effect::{authority,bank,denials,edit_grant,measured,primitive,retire,take,turn};
use std::marker::PhantomData;

const BODY:usize=64;
fn ids(row:&serde_json::Value,field:&str)->Vec<u64>{row[field].as_array().unwrap().iter().map(|id|id.as_str().unwrap().parse().unwrap()).collect()}
fn kind(row:&serde_json::Value)->ToolRunEntityEditKind{if row["kind"]=="append"{ToolRunEntityEditKind::Append}else{ToolRunEntityEditKind::Retract}}
fn stops(corpus:&serde_json::Value)->Vec<Option<usize>>{std::iter::once(None).chain(corpus["interruptAfter"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize))).collect()}
fn birth_grant(capacity:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:capacity,maximum_release_bytes:1<<20,maximum_depth:8}}
fn capture_grant(copy:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy+4096,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:8}}

#[test]
fn original_tool_run_entity_edit_preserves_every_workspace_until_funded_completion_or_cancel(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 assert_eq!(corpus["deniedAxes"],serde_json::json!(["items","copy","capacity","release","depth"]));
 for row in corpus["cases"].as_array().unwrap(){
  let(input,targets,expected,kind)=(ids(row,"initial"),ids(row,"targets"),ids(row,"expected"),kind(row));
  for stop in stops(&corpus){
   let((map,targets),original)=measured(||(input.iter().map(|id|(*id,())).collect::<EntityMap>(),targets.clone()));let pointer=targets.as_ptr()as usize;
   let mut bank=bank();let mut authority=authority::<ToolRunEntityEdit>();let(mut job,heap)=measured(||ToolRunEntityEdit::new(map,targets,kind));assert_eq!(heap,(0,0));
   let(mut born,mut released)=(0,0);let mut turns=0;
   while !job.complete()&&turns<stop.unwrap_or(50000){
    let demand=job.next_demands(BODY).unwrap();let grant=edit_grant(demand,job.edit_capacity_bound().unwrap(),size_of::<(u64,())>(),8);
    for retirement in denials(grant.retirement,demand){let observed=turn(&mut bank,&mut authority,&mut job,BODY,ToolRunEntityEditGrant{retirement,..grant});assert_eq!(observed.progress,Default::default());assert_eq!(observed.heap,(0,0));assert_eq!(observed.work,0);}
    assert_eq!(job.targets_identity(),pointer);
    let observed=turn(&mut bank,&mut authority,&mut job,BODY,grant);assert!(observed.progress.fits(grant.retirement));born+=observed.heap.0;released+=observed.heap.1;turns+=1;
   }
   if stop.is_none(){
    assert!(job.complete());let mut taker=effect::taker();let mut output=None;let g=grant_of(ToolRunEntityEdit::<u64>::take_output_demand());
    let denied=take(&mut bank,&mut taker,&mut job,&mut output,RetainedCloneGrant{maximum_copy_bytes:0,..g});assert_eq!(denied.progress,Default::default());assert!(output.is_none());
    let observed=take(&mut bank,&mut taker,&mut job,&mut output,g);assert!(observed.progress.fits(g));assert_eq!(output.as_ref().unwrap().iter().map(|(key,_)|*key).collect::<Vec<_>>(),expected);
    let heap=retire(&mut bank,output.take().unwrap());born+=heap.0;released+=heap.1;
   }else{job.cancel();}
   let heap=retire(&mut bank,job);born+=heap.0;released+=heap.1;assert_eq!(released,original.0-original.1+born);
  }
 }
}
fn grant_of(d:RetirementDemand)->RetainedCloneGrant{effect::grant(d)}

#[test]
fn original_tool_run_entity_edit_versions_preserve_original_aliases_and_publish_once(){
 use super::version::{ToolRunEntityVersion,ToolRunEntityVersionEdit};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let law:serde_json::Value=serde_json::from_str(include_str!("../📸️version/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let(input,targets,expected,kind)=(ids(row,"initial"),ids(row,"targets"),ids(row,"expected"),kind(row));
  for order in law["captureCloseOrders"].as_array().unwrap(){for stop in stops(&corpus){
   let((map,targets),original)=measured(||(input.iter().map(|key|(*key,())).collect::<EntityMap>(),targets.clone()));let mut bank=bank();
   let(admitted,heap)=primitive(&mut bank,birth_grant(ToolRunEntityVersion::birth_bytes()),|effect|ToolRunEntityVersion::admit(map,law["originalRevision"].as_u64().unwrap(),effect).map_err(|(error,_)|error));let(primary,receipt)=admitted.unwrap();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));let(mut born,mut released)=(heap.0,heap.1);let identity=primary.identity();
   let copy=capture_grant(ToolRunEntityVersion::capture_copy_bytes());
   let(captured,heap)=primitive(&mut bank,copy,|effect|primary.capture(effect));let(captured,receipt)=captured.unwrap();assert_eq!(heap,(0,0));assert!(receipt.fits(copy));
   let(candidate,heap)=primitive(&mut bank,copy,|effect|primary.capture(effect));let(candidate,_)=candidate.unwrap();assert_eq!(heap,(0,0));
   let mut job=ToolRunEntityVersionEdit::new(candidate,targets,kind);let mut authority=authority::<ToolRunEntityVersionEdit>();let mut turns=0;
   while !job.complete()&&turns<stop.unwrap_or(100000){
    let demand=job.next_demands(BODY).unwrap();let grant=edit_grant(demand,job.edit_capacity_bound().unwrap(),size_of::<(u64,())>(),8);
    for retirement in denials(grant.retirement,demand){let observed=turn(&mut bank,&mut authority,&mut job,BODY,ToolRunEntityEditGrant{retirement,..grant});assert_eq!(observed.progress,Default::default());assert_eq!(observed.heap,(0,0));assert_eq!(job.next_demands(BODY).unwrap(),demand);}
    let observed=turn(&mut bank,&mut authority,&mut job,BODY,grant);assert!(observed.progress.fits(grant.retirement));born+=observed.heap.0;released+=observed.heap.1;turns+=1;
   }
   assert_eq!(primary.identity(),identity);assert_eq!(captured.identity(),identity);assert_eq!(captured.entities().iter().map(|(key,_)|*key).collect::<Vec<_>>(),input);
   if stop.is_none(){
    assert!(job.complete());let demand=job.publication_demands();let publication=grant_of(demand);
    let(result,heap)=primitive(&mut bank,RetainedCloneGrant{maximum_capacity_bytes:0,..publication},|effect|job.take_output(effect));assert!(result.unwrap().is_none());assert_eq!(heap,(0,0));
    let(result,heap)=primitive(&mut bank,publication,|effect|job.take_output(effect));let(published,receipt)=result.unwrap().unwrap();assert!(receipt.fits(publication));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;released+=heap.1;
    assert_eq!(published.revision(),law["publishedRevision"].as_u64().unwrap());assert_eq!(published.entities().iter().map(|(key,_)|*key).collect::<Vec<_>>(),expected);
    let(again,_)=primitive(&mut bank,publication,|effect|job.take_output(effect));assert!(again.unwrap().is_none());let heap=retire(&mut bank,published);born+=heap.0;released+=heap.1;
   }else{job.cancel();let(result,heap)=primitive(&mut bank,grant_of(job.publication_demands()),|effect|job.take_output(effect));assert!(result.unwrap().is_none());assert_eq!(heap,(0,0));}
   let mut primary=Some(primary);let mut captured=Some(captured);let first=if order=="sourceFirst"{primary.take().unwrap()}else{captured.take().unwrap()};
   let heap=retire(&mut bank,first);born+=heap.0;released+=heap.1;let heap=retire(&mut bank,job);born+=heap.0;released+=heap.1;
   let last=primary.take().or_else(||captured.take()).unwrap();assert_eq!(last.identity(),identity);assert_eq!(last.entities().iter().map(|(key,_)|*key).collect::<Vec<_>>(),input);let heap=retire(&mut bank,last);born+=heap.0;released+=heap.1;
   assert_eq!(original.0-original.1+born,released);
  }}
 }
}

#[test]
fn original_tool_run_entity_edit_provenance_preserves_earlier_introductions_and_original_custody(){
 use super::provenance::{ToolRunEntityProvenance,ToolRunEntityProvenanceEdit,ProvenanceMap};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧾️provenance/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let key=|mark:&serde_json::Value|((mark["end"].as_u64().unwrap()as u128)<<64)|mark["entity"].as_str().unwrap().parse::<u64>().unwrap()as u128;
 for row in corpus["cases"].as_array().unwrap(){
  let keys:Vec<u128>=row["original"].as_array().unwrap().iter().map(key).collect();let expected:Vec<u128>=row["expectedMarks"].as_array().unwrap().iter().map(key).collect();
  let expected_entities=ids(row,"expectedEntities");let append=ids(row,"append");let retract=row["retractTo"].as_u64().map(|v|v as u32);let end=row["end"].as_u64().unwrap()as u32;
  for stop in stops(&corpus){
   let((marks,entities,append),original)=measured(||(keys.iter().map(|key|(*key,())).collect::<ProvenanceMap>(),keys.iter().map(|key|(*key as u64,())).collect::<EntityMap>(),append.clone()));let mut bank=bank();
   let(admitted,heap)=primitive(&mut bank,birth_grant(ToolRunEntityProvenance::birth_bytes()),|effect|ToolRunEntityProvenance::admit(marks,entities,1,effect).map_err(|(error,_,_)|error));let(root,receipt)=admitted.unwrap();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));let(mut born,mut released)=(heap.0,heap.1);let pointer=root.identity();
   let(captured,heap)=primitive(&mut bank,capture_grant(ToolRunEntityProvenance::capture_copy_bytes()),|effect|root.capture(effect));let(captured,_)=captured.unwrap();assert_eq!(heap,(0,0));
   let mut job=ToolRunEntityProvenanceEdit::new(captured,retract,end,append);let mut authority=authority::<ToolRunEntityProvenanceEdit>();let mut turns=0;
   while !job.complete()&&turns<stop.unwrap_or(50000){
    let quote=job.next_demands(BODY).unwrap();let grant=edit_grant(quote,job.edit_capacity_bound().unwrap(),size_of::<(u128,())>(),16);
    for retirement in denials(grant.retirement,quote){let observed=turn(&mut bank,&mut authority,&mut job,BODY,ToolRunEntityEditGrant{retirement,..grant});assert_eq!(observed.progress,Default::default());assert_eq!(observed.heap,(0,0));assert_eq!(quote,job.next_demands(BODY).unwrap());}
    let observed=turn(&mut bank,&mut authority,&mut job,BODY,grant);assert!(observed.progress.fits(grant.retirement));born+=observed.heap.0;released+=observed.heap.1;turns+=1;assert_eq!(root.identity(),pointer);
   }
   let published=if stop.is_none(){
    assert!(job.complete());let demand=job.publication_demands();let publication=grant_of(demand);
    let(output,heap)=primitive(&mut bank,RetainedCloneGrant{maximum_capacity_bytes:0,..publication},|effect|job.take_output(effect));assert!(output.unwrap().is_none());assert_eq!(heap,(0,0));
    let(output,heap)=primitive(&mut bank,publication,|effect|job.take_output(effect));let(output,progress)=output.unwrap().unwrap();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;released+=heap.1;
    assert_eq!(output.revision(),2);assert_eq!(output.marks().iter().map(|(key,_)|*key).collect::<Vec<_>>(),expected);assert_eq!(output.entities().iter().map(|(key,_)|*key).collect::<Vec<_>>(),expected_entities);
    let(again,_)=primitive(&mut bank,publication,|effect|job.take_output(effect));assert!(again.unwrap().is_none());Some(output)
   }else{job.cancel();None};
   assert_eq!(root.marks().iter().map(|(key,_)|*key).collect::<Vec<_>>(),{let mut sorted=keys.clone();sorted.sort();sorted.dedup();sorted});
   let heap=retire(&mut bank,job);born+=heap.0;released+=heap.1;assert_eq!(root.identity(),pointer);let heap=retire(&mut bank,root);born+=heap.0;released+=heap.1;
   if let Some(output)=published{let heap=retire(&mut bank,output);born+=heap.0;released+=heap.1;}
   assert_eq!(released,original.0-original.1+born);
  }
 }
}

#[test]
fn original_tool_run_entity_version_cancel_after_every_turn_completes_cleanup(){
 use super::version::{ToolRunEntityVersion,ToolRunEntityVersionEdit};
 for turns in 0..24{
  let mut bank=bank();let map:EntityMap=[1u64,2,3].iter().map(|id|(*id,())).collect();
  let(primary,_)=primitive(&mut bank,birth_grant(ToolRunEntityVersion::birth_bytes()),|effect|ToolRunEntityVersion::admit(map,1,effect).map_err(|(error,_)|error)).0.unwrap();
  let(captured,_)=primitive(&mut bank,capture_grant(ToolRunEntityVersion::capture_copy_bytes()),|effect|primary.capture(effect)).0.unwrap();
  let mut job=ToolRunEntityVersionEdit::new(captured,vec![4],ToolRunEntityEditKind::Append);let mut authority=authority::<ToolRunEntityVersionEdit>();
  for _ in 0..turns{if job.complete(){break;}let demand=job.next_demands(BODY).unwrap();{let bound=job.edit_capacity_bound().unwrap();turn(&mut bank,&mut authority,&mut job,BODY,edit_grant(demand,bound,size_of::<(u64,())>(),16))};}
  job.cancel();retire(&mut bank,job);retire(&mut bank,primary);
 }
}

#[test]
fn original_tool_run_entity_provenance_cancel_after_every_turn_completes_cleanup(){
 use super::provenance::{ToolRunEntityProvenance,ToolRunEntityProvenanceEdit,ProvenanceMap};
 for turns in 0..24{
  let mut bank=bank();let marks:ProvenanceMap=[(1u128,())].into_iter().collect();let entities:EntityMap=[(1u64,())].into_iter().collect();
  let(root,_)=primitive(&mut bank,birth_grant(ToolRunEntityProvenance::birth_bytes()),|effect|ToolRunEntityProvenance::admit(marks,entities,1,effect).map_err(|(error,_,_)|error)).0.unwrap();
  let(captured,_)=primitive(&mut bank,capture_grant(ToolRunEntityProvenance::capture_copy_bytes()),|effect|root.capture(effect)).0.unwrap();
  let mut job=ToolRunEntityProvenanceEdit::new(captured,None,1,vec![2]);let mut authority=authority::<ToolRunEntityProvenanceEdit>();
  for _ in 0..turns{if job.complete(){break;}let demand=job.next_demands(BODY).unwrap();{let bound=job.edit_capacity_bound().unwrap();turn(&mut bank,&mut authority,&mut job,BODY,edit_grant(demand,bound,size_of::<(u128,())>(),16))};}
  job.cancel();retire(&mut bank,job);retire(&mut bank,root);
 }
}

#[test]
fn sealed_birth_is_funded_by_its_exact_quote(){
 fn exact<T:RetireOwned+Send+Sync+'static>(value:T){
  let mut bank=bank();let demand=SealedShared::<T>::source_admission_demand().unwrap();let units=SealedShared::<T>::source_admission_work_demand();
  let(admitted,_)=primitive(&mut bank,effect::grant(demand),|effect|{let boundary=effect.performed_boundary();let(sealed,progress)=admit_sealed(value,effect).map_err(|(error,_)|error)?;assert_eq!(effect.performed_since(boundary).unwrap(),u128::from(units));Ok((sealed,progress))});
  let(sealed,_)=admitted.unwrap();drop(retire(&mut bank,sealed));
 }
 exact(String::from("draw.command"));exact(vec![1u8,2,3]);exact(7u64);let _:PhantomData<u8>=PhantomData;
}

#[test]
fn original_tool_run_entity_version_edit_cost_is_independent_of_the_cloned_size(){
 use super::version::{ToolRunEntityVersion,ToolRunEntityVersionEdit};
 use effect::Edited;
 let mut costs=Vec::new();
 for size in [16u64,2048]{
  let mut bank=bank();let map:EntityMap=(0..size).map(|id|(id,())).collect();
  let(primary,_)=primitive(&mut bank,birth_grant(ToolRunEntityVersion::birth_bytes()),|effect|ToolRunEntityVersion::admit(map,1,effect).map_err(|(error,_)|error)).0.unwrap();
  let(captured,_)=primitive(&mut bank,capture_grant(ToolRunEntityVersion::capture_copy_bytes()),|effect|primary.capture(effect)).0.unwrap();
  let mut job=ToolRunEntityVersionEdit::new(captured,vec![size+1,size+2],ToolRunEntityEditKind::Append);let mut authority=authority::<ToolRunEntityVersionEdit>();let(mut clone,mut edit)=(0u128,0u128);
  while !job.complete(){let editing=job.editing();let demand=job.next_demands(BODY).unwrap();let observed={let bound=job.edit_capacity_bound().unwrap();turn(&mut bank,&mut authority,&mut job,BODY,edit_grant(demand,bound,size_of::<(u64,())>(),16))};if editing{edit+=observed.work}else{clone+=observed.work}}
  costs.push((clone,edit));retire(&mut bank,job);retire(&mut bank,primary);
 }
 let[(small_clone,small_edit),(large_clone,large_edit)]=costs[..]else{unreachable!()};
 assert!(large_clone>=8*small_clone,"the clone alone scales with the original");
 assert!(large_edit<=4*small_edit,"the target edit must not scale with the original: {small_edit} -> {large_edit}");
}

#[test]
fn original_controlled_retirement_closes_as_nested_child_for_every_owner_width() {
    use semio_framework_value::retirement::{RetireOwned, controlled::ControlledRetirement};
    fn nested<T: RetireOwned>(value: T) {
        let child = ControlledRetirement::new(value).map_err(|(error, _)| error).unwrap();
        let mut bank = bank();
        retire(&mut bank, Some(child));
    }
    nested(vec![1u64, 2, 3]);
    nested(vec![1u128 << 70, 2, 3]);
    nested(vec![("a".to_owned(), 1u128), ("b".to_owned(), 2)]);
}
