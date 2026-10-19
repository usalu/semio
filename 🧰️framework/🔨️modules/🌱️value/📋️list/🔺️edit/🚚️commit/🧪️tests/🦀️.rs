use super::*;
use crate::{IndexedEditShape,IndexedEditStep,retained_clone::RetainedCloneSource,retirement::{RetireOwned,controlled::ControlledRetirement}};
#[derive(crate::RetainedClone,crate::RetireOwned)]
struct Row{id:u64,text:String}
impl Clone for Row{fn clone(&self)->Self{panic!("unchanged owned row must never be cloned")}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn source<T:RetireOwned+Sync+'static>(value:T)->RetainedCloneSource<T>{let d=RetainedCloneSource::<T>::owned_constructor_demand::<()>();RetainedCloneSource::admit_owned(value,(),grant(RetirementDemand{copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})).map_err(|(error,..)|error).unwrap().0}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let d=RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(4096).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()};owner.step(grant(d)).unwrap();}}
fn close_plan(plan:&mut IndexedEditPlan){plan.begin_close();while !plan.terminal_is_empty(){let d=plan.close_demand(4096).unwrap();let(step,heap)=crate::observe_retirement_allocations(||plan.close_step(grant(d)).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}}
fn close_source<T:RetireOwned+Sync+'static>(source:&mut RetainedCloneSource<T>){while !source.terminal_is_empty(){let d=RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()};source.close_step(grant(d)).unwrap();}}
fn check(base:Vec<String>,removed:Vec<usize>,added:Vec<(usize,String)>,expected:Vec<String>,cut:Option<usize>){
 let base_bytes:usize=base.iter().map(String::len).sum();let added_bytes:usize=added.iter().map(|(_,s)|s.len()).sum();let limit=1024+200*(base_bytes+added_bytes+base.len()+added.len());
 let mut plan=IndexedEditPlan::new(IndexedEditShape{base_len:base.len(),removed:removed.len(),modified:0,added:added.len()});
 while !plan.settled(){let d=plan.demand().unwrap();let step=plan.advance(grant(d),|kind,index|match kind{IndexedEditKind::Removed=>removed[index],IndexedEditKind::Added=>added[index].0,_=>unreachable!()}).unwrap();assert!(!matches!(step,IndexedEditStep::Refused(..)));}assert!(plan.ready());
 let mut post:Vec<Row>=base.into_iter().enumerate().map(|(id,text)|Row{id:id as u64,text}).collect();let original_backing=post.as_ptr();let original_payloads:Vec<*const u8>=post.iter().map(|row|row.text.as_ptr()).collect();
 let original_count=post.len();let mut expected_ids:Vec<u64>=(0..original_count).filter(|index|!removed.contains(index)).map(|index|index as u64).collect();let mut added_order:Vec<usize>=(0..added.len()).collect();added_order.sort_by_key(|index|added[*index].0);for ordinal in added_order{expected_ids.insert(added[ordinal].0,(original_count+ordinal)as u64);}
 let mut authored=source(added.into_iter().enumerate().map(|(id,(_,text))|Row{id:(original_count+id)as u64,text}).collect::<Vec<_>>());let authored_pointers:Vec<*const u8>=authored.borrow().get().iter().map(|row|row.text.as_ptr()).collect();
 let(mut owner,heap)=crate::observe_retirement_allocations(IndexedCollectionCommit::<Row>::new);assert_eq!(heap,(0,0));let mut turns=0;
 while !owner.complete()&&cut.is_none_or(|cut|turns<cut){
  let ordinal=owner.next_added_ordinal(&plan).unwrap();let field=ordinal.map(|index|authored.borrow().project(index,|rows|&rows[index]));let d=owner.demand(&plan,&post,field,4096).unwrap();let issued=grant(d);
  let(step,heap)=crate::observe_retirement_allocations(||owner.advance(&plan,&mut post,field,RetainedCloneGrant{maximum_items:0,..issued}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));
  if d.capacity_bytes>0{let(step,heap)=crate::observe_retirement_allocations(||owner.advance(&plan,&mut post,field,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..issued}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));}
  let(step,heap)=crate::observe_retirement_allocations(||owner.advance(&plan,&mut post,field,issued).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(issued));turns+=1;assert!(turns<limit);
  if let Some(original)=owner.original.as_ref(){if let Some(first)=original.remaining().first(){assert_eq!(first.text.as_ptr(),original_payloads[first.id as usize]);assert_eq!(original.remaining().as_ptr(),original_backing.wrapping_add(first.id as usize));}}
 }
 if cut.is_none(){assert!(owner.complete());assert_eq!(post.iter().map(|row|row.text.as_str()).collect::<Vec<_>>(),expected.iter().map(String::as_str).collect::<Vec<_>>());assert_eq!(post.iter().map(|row|row.id).collect::<Vec<_>>(),expected_ids);for row in &post{if (row.id as usize)<original_count{assert_eq!(row.text.as_ptr(),original_payloads[row.id as usize]);}else if !row.text.is_empty(){assert_ne!(row.text.as_ptr(),authored_pointers[row.id as usize-original_count]);}}if plan.shape().removed==0&&plan.shape().added==0{assert_eq!(post.as_ptr(),original_backing);}}
 for(index,row)in authored.borrow().get().iter().enumerate(){assert_eq!(row.text.as_ptr(),authored_pointers[index]);}
 owner.begin_close();let mut close_turns=0;while !owner.terminal_is_empty(){let d=owner.close_demand(4096).unwrap();let issued=grant(d);let(step,heap)=crate::observe_retirement_allocations(||owner.close_step(RetainedCloneGrant{maximum_items:0,..issued}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));let(step,heap)=crate::observe_retirement_allocations(||owner.close_step(issued).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));close_turns+=1;assert!(close_turns<limit);}
 assert_eq!(crate::observe_retirement_allocations(||drop(owner)).1,(0,0));close_plan(&mut plan);close_source(&mut authored);drain(post);
 eprintln!("[DEBUG] original indexed collection cut={cut:?} turns={turns} close={close_turns} exact physical receipts, no unchanged row Clone, original/authored payload custody terminal");
}
/// 🧪️ Actual native object headers preserve survivor allocations and close exact originals at every neutral frontier.
#[test]
fn indexed_collection_commit_preserves_original_payloads_and_exact_cancellation(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){for cut in std::iter::once(None).chain(fixture["cancelCuts"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()as usize))){
  let strings=|key:&str|row[key].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_string()).collect::<Vec<_>>();let removed=row["removed"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize).collect();let added=row["added"].as_array().unwrap().iter().map(|value|(value["index"].as_u64().unwrap()as usize,value["item"].as_str().unwrap().to_string())).collect();check(strings("base"),removed,added,strings("expected"),cut);
 }}
}
/// 📦️ Long authored fields remain on real native projection cursors instead of a whole-row Clone shortcut.
#[test]
fn indexed_collection_commit_copies_large_authored_fields_without_cloning_survivors(){
 for cut in[Some(0),Some(2),Some(9),Some(17),Some(1025),None]{check(vec!["original-left".into(),"original-right".into()],vec![],vec![(1,"x".repeat(65537))],vec!["original-left".into(),"x".repeat(65537),"original-right".into()],cut);}
}
