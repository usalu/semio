//! 🔐️ Original borrowed authority and projected clone bindings obey exact physical custody.
use crate::{RetainedCloneSource,RetainedCloneBinding,RetainedCloneGrant,observe_retirement_allocations};

use crate::source_law_bank::{Bank,observed,settled,retain,drive_take,CPU};
fn exact_grant(d:crate::RetirementDemand,bank:&Bank)->RetainedCloneGrant{bank.grant(d)}

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="crate")]
struct Captured{owner:String,metadata:String}
impl crate::OriginalSourceProjection<String> for Captured{
 fn projection_work_demand(&self)->Result<u64,crate::ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalProjectionReply<'reply,'source,String>)->Result<crate::OriginalEffectReplyComplete<'reply,std::ptr::NonNull<String>,crate::ValueError>,crate::ValueError>{assert!(effect.performed(1)?);Ok(reply.complete(Ok(&self.owner)))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalProjectionReply<'reply,'source,String>)->Result<crate::OriginalEffectReplyComplete<'reply,std::ptr::NonNull<String>,crate::ValueError>,crate::ValueError>{Ok(reply.pending())}
}

#[test]
fn sealed_clone_source_original_projected_authority_survives_every_alias_until_funded_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔐️custody.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){let mut bank=Bank::new();
  let(mut original,heap)=observe_retirement_allocations(||Captured{owner:row["owner"].as_str().unwrap().to_string(),metadata:row["metadata"].as_str().unwrap().to_string()});
  let original_bytes=heap.0;let owner_pointer=original.owner.as_ptr();let metadata_pointer=original.metadata.as_ptr();
  let mut admission=RetainedCloneSource::<String,Captured>::admission_borrowed(original);for axis in ["items","capacity","depth"]{bank.denied(&mut admission,axis);let original=admission.original_authority().unwrap();assert_eq!(original.owner.as_ptr(),owner_pointer);assert_eq!(original.metadata.as_ptr(),metadata_pointer);}
  let(mut source,receipt)=bank.source(admission);let heap=(receipt.retained_capacity_bytes,receipt.released_bytes);assert_eq!(heap,(receipt.retained_capacity_bytes,0));assert_eq!(source.borrow().get().as_ptr(),owner_pointer);
  let mut born=heap.0;let mut released=0;let mut binding=None;let copy=source.borrow().binding_copy_bytes();assert_eq!(copy,crate::shared_issuance::SealedShared::<crate::RetainedCloneLeaseOwner>::duplicate_copy_bytes()+size_of::<Option<crate::shared_issuance::SealedShared<crate::RetainedCloneLeaseOwner>>>()+size_of::<bool>());
  for denied in [RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,..Default::default()}]{
   let(result,heap)=observed(denied,CPU,&mut None,|effect|source.borrow().bind(&mut binding,effect));assert_eq!(heap,(0,0));assert_eq!(result.result.unwrap().unwrap(),Default::default());assert!(binding.is_none());
  }
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()};
  let(result,heap)=bank.observe_eff(grant,|effect|source.borrow().bind(&mut binding,effect),|p|p.unwrap_or_default());assert_eq!(heap,(0,0));let receipt=result.result.unwrap().unwrap();assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,copy);
  let(mut projection,_)=retain(&mut bank,source.borrow().project(1,|owner|owner));
  for _ in 0..10000{
   if source.terminal_is_empty(){break;}
   let d=source.next_close_demand(4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);
   let(step,heap)=bank.success_eff(grant,|effect|settled(source.close_step(effect.grant().maximum_copy_bytes,effect)),|s|s.progress());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }
  assert!(source.terminal_is_empty());assert!(binding.is_some());assert!(released<=born);assert_eq!(projection.borrow().unwrap().get().as_ptr(),owner_pointer);assert_eq!(projection.borrow().unwrap().get().as_str(),row["owner"].as_str().unwrap());
  for _ in 0..10000{
   if binding.is_none(){break;}
   let d=RetainedCloneBinding::close_demand(&binding,4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(RetainedCloneBinding::close_demand(&binding,grant.maximum_copy_bytes).unwrap(),d);
   let(step,heap)=bank.success_eff(grant,|effect|settled(RetainedCloneBinding::close_one(&mut binding,effect.grant().maximum_copy_bytes,effect)),|s|s.progress());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }
  assert!(binding.is_none());assert_eq!(projection.borrow().unwrap().get().as_ptr(),owner_pointer);
  for _ in 0..10000{if projection.terminal_is_empty(){break;}let d=projection.next_close_demand(4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(projection.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=bank.success_eff(grant,|effect|settled(projection.close_step(effect.grant().maximum_copy_bytes,effect)),|s|s.progress());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
  assert!(projection.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(projection)).1,(0,0));assert_eq!(released,original_bytes+born);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
 }
}
   
#[test]
fn sealed_clone_source_returns_same_original_workspace_after_all_projection_aliases_return(){
 use crate::retirement::controlled::ControlledRetirement;
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔐️custody.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){let mut bank=Bank::new();
  let(original,heap)=observe_retirement_allocations(||Captured{owner:row["owner"].as_str().unwrap().into(),metadata:row["metadata"].as_str().unwrap().into()});let bytes=heap.0;let pointer=original.owner.as_ptr();let metadata=original.metadata.as_ptr();
  let(mut source,p)=bank.source(RetainedCloneSource::admission_owned(original,()));let heap=(p.retained_capacity_bytes,p.released_bytes);assert_eq!(heap,(p.retained_capacity_bytes,0));let(mut born,mut freed)=(heap.0,0);
  let(mut child,_)=retain(&mut bank,source.borrow().project(1,|root|&root.owner));let mut taken:Option<Captured>=None;
  let mut retained=false;let mut last=String::new();for _ in 0..10000{let d=source.next_take_demand(4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(source.next_take_demand(grant.maximum_copy_bytes).unwrap(),d);let(turn,heap)=bank.take(&mut source,&mut taken,grant);last=format!("{last}\n{:?} demand {d:?} work {}",turn.result.as_ref().map(|step|step.progress()),turn.work);match turn.result{Ok(step)=>{assert!(taken.is_none(),"live genuine projection must retain original workspace");let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));if p==Default::default(){retained=true;break;}born+=heap.0;freed+=heap.1;},Err(error)=>{assert_eq!(heap,(0,0));assert_eq!(error.primary().unwrap().kind,crate::ValueRefusalKind::WorkLimit);retained=true;break;}}}
  assert!(retained,"live original projection did not reach its retained handback frontier: {last}");assert_eq!(child.borrow().unwrap().get().as_ptr(),pointer);
  for _ in 0..10000{if child.terminal_is_empty(){break;}let d=child.next_close_demand(4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(child.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=bank.success_eff(grant,|effect|settled(child.close_step(effect.grant().maximum_copy_bytes,effect)),|s|s.progress());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
  assert!(child.terminal_is_empty());let d=source.next_take_demand(4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(source.next_take_demand(grant.maximum_copy_bytes).unwrap(),d);assert_eq!(d.copy_bytes,size_of::<Option<crate::shared_issuance::SealedShared<Captured>>>()+size_of::<Option<crate::retirement::arc_custody::original::SharedOriginalStorage<Captured>>>()+size_of::<Option<crate::retirement::arc_custody::original::SharedOriginal<Captured>>>()+size_of::<std::sync::atomic::AtomicUsize>(),"{last}");assert!(grant.maximum_copy_bytes>0);
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=drive_take(&mut source,&mut taken,denied,CPU);assert!(result.result.map_or(true,|step|step.progress()==Default::default()));assert!(taken.is_none());assert_eq!(heap,(0,0));assert!(!source.terminal_is_empty());}
  let mut turns=0;let mut grant=grant;while taken.is_none(){let(turn,heap)=bank.take(&mut source,&mut taken,grant);let p=turn.result.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<10000,"original handback did not deliver the original workspace");if taken.is_none(){let d=source.next_take_demand(4096).unwrap();grant=exact_grant(d,&bank);}}let original=taken.take().expect("all genuine aliases returned");let mut cleared=0;while !source.terminal_is_empty(){let d=source.next_take_demand(4096).unwrap();let g=exact_grant(d,&bank);let(turn,heap)=bank.take(&mut source,&mut taken,g);let p=turn.result.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;cleared+=1;assert!(cleared<100&&taken.is_none(),"original handback did not clear its election owner");}assert_eq!(original.owner.as_ptr(),pointer);assert_eq!(original.metadata.as_ptr(),metadata);
  let mut close=ControlledRetirement::new(original).unwrap_or_else(|_|panic!("typed original workspace close"));for _ in 0..10000{if close.terminal_is_empty(){break;}let d=close.next_demand(4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(close.next_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=bank.success_eff(grant,|effect|settled(close.step(effect.grant().maximum_copy_bytes,effect)),|s|s.progress());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
  assert!(close.terminal_is_empty());assert_eq!(bytes+born,freed);assert_eq!(observe_retirement_allocations(||{drop(source);drop(child);drop(close);}).1,(0,0));
 }
}     
#[test]
fn immutable_binding_query_matches_exact_original_lease_and_projected_path(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔐️custody.json")).unwrap();let law=&fixture["bindingIdentity"];let mut bank=Bank::new();let(mut first,_)=bank.source(RetainedCloneSource::admission_owned(1u64,()));let(mut second,_)=bank.source(RetainedCloneSource::admission_owned(1u64,()));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:first.borrow().binding_copy_bytes(),maximum_depth:1,..Default::default()};let mut binding=None;bank.success_eff(grant,|effect|first.borrow().bind(&mut binding,effect),|p|p.unwrap_or_default());let bound=binding.as_ref().unwrap();let query=first.borrow();assert!(std::ptr::eq(query.get(),first.borrow().get()));assert!(!std::ptr::eq(query.get(),second.borrow().get()));assert_eq!(query.matches_binding(bound),law["sameOrigin"].as_bool().unwrap());assert_eq!(query.project(1,|value|value).matches_binding(bound),law["differentProjection"].as_bool().unwrap());assert_eq!(second.borrow().matches_binding(bound),law["differentOrigin"].as_bool().unwrap());
 for _ in 0..10000{if binding.is_none(){break;}let d=RetainedCloneBinding::close_demand(&binding,4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(RetainedCloneBinding::close_demand(&binding,grant.maximum_copy_bytes).unwrap(),d);bank.success_eff(grant,|effect|settled(RetainedCloneBinding::close_one(&mut binding,effect.grant().maximum_copy_bytes,effect)),|s|s.progress());}assert!(binding.is_none());for owner in [&mut first,&mut second]{for _ in 0..10000{if owner.terminal_is_empty(){break;}let d=owner.next_close_demand(4096).unwrap();let grant=exact_grant(d,&bank);assert_eq!(owner.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);bank.success_eff(grant,|effect|settled(owner.close_step(effect.grant().maximum_copy_bytes,effect)),|s|s.progress());}assert!(owner.terminal_is_empty());}
}

fn closing_rows<A:crate::retirement::RetireOwned+Sync+'static>(make:A,alias_live:bool,strict:bool)->(Vec<[usize;8]>,(usize,usize)){
 let mut bank=Bank::new();
 let(mut source,_)=bank.source(RetainedCloneSource::admission_owned(make,()));
 let copy=source.borrow().binding_copy_bytes();let mut binding=None;
 bank.success_eff(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()},|effect|source.borrow().bind(&mut binding,effect),|p|p.unwrap_or_default());
 let close_source=|source:&mut RetainedCloneSource<A>,bank:&mut Bank|for _ in 0..10000{if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let grant=exact_grant(d,bank);bank.success_eff(grant,|effect|settled(source.close_step(effect.grant().maximum_copy_bytes,effect)),|s|s.progress());};
 if !alias_live{close_source(&mut source,&mut bank);}
 let mut rows=Vec::new();let mut total=(0,0);
 for _ in 0..10000{
  if binding.is_none(){break;}
  let d=RetainedCloneBinding::close_demand(&binding,4096).unwrap();let w=RetainedCloneBinding::close_work_demand(&binding,4096).unwrap();let grant=exact_grant(d,&bank);
  for denied in strict.then(||[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_depth:0,..grant}]).into_iter().flatten(){
   if denied==grant{continue}
   let(turn,heap)=bank.observe_eff(denied,|effect|settled(RetainedCloneBinding::close_one(&mut binding,effect.grant().maximum_copy_bytes,effect)),|s|s.progress());
   assert_eq!(heap,(0,0));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(RetainedCloneBinding::close_demand(&binding,4096).unwrap(),d);
  }
  let(turn,heap)=bank.observe_eff(grant,|effect|settled(RetainedCloneBinding::close_one(&mut binding,effect.grant().maximum_copy_bytes,effect)),|s|s.progress());
  let p=turn.result.unwrap().progress();assert_eq!(turn.work,u128::from(w));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!((p.copied_bytes,p.retained_capacity_bytes,p.released_bytes),(d.copy_bytes,d.capacity_bytes,d.release_bytes));
  total.0+=heap.0;total.1+=heap.1;
  rows.push([d.copy_bytes,d.capacity_bytes,d.release_bytes,d.depth,p.copied_bytes,p.retained_capacity_bytes,p.released_bytes,usize::try_from(turn.work).unwrap()]);
 }
 assert!(binding.is_none());close_source(&mut source,&mut bank);assert!(source.terminal_is_empty());
 (rows,total)
}
#[test]
fn binding_close_ledger_keeps_exact_quotes_short_grant_refusals_and_net_release(){
 let(exclusive,total)=closing_rows(Captured{owner:"original-source".into(),metadata:"original-authority".into()},false,true);
 assert_eq!(total.1-total.0,2*size_of::<crate::retirement::controlled::ControlledRetirement<u8>>()+2*size_of::<crate::shared_issuance::SealedShared<Captured>>()+EXCLUSIVE_NET_RELEASE,"exclusive lease close net released bytes");
 assert!(exclusive.iter().map(|row|row[7]).sum::<usize>()>=exclusive.len());
 let(alias,total)=closing_rows(Captured{owner:"original-source".into(),metadata:"original-authority".into()},true,true);
 assert_eq!(alias.len(),1);assert_eq!(total,(0,0));
}
const EXCLUSIVE_NET_RELEASE:usize=153;

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="crate")]
struct PlainBytes{bytes:Vec<u8>}
#[test]
fn cold_plain_source_closes_in_one_unit_with_the_same_net_release_as_the_ladder(){
 let page=||(0..4096u32).map(|index|index as u8).collect::<Vec<u8>>();
 let(ladder,ladder_total)=closing_rows(PlainBytes{bytes:page()},false,false);
 let(cold,cold_total)=closing_rows(page(),false,true);
 assert_eq!(cold_total.0,0,"cold close births no frame");
 assert_eq!(cold_total.1,ladder_total.1-ladder_total.0,"cold close releases exactly the ladder's net bytes");
 assert!(cold.len()<ladder.len()&&cold.len()<=6,"cold close has {} steps against {}",cold.len(),ladder.len());
 assert!(cold.iter().all(|row|row[1]==0),"no cold step quotes a birth");
}

fn compact_turn<A:crate::retirement::RetireOwned+Sync+'static>(make:A,compact:bool)->(usize,(usize,usize),bool){
 let mut bank=Bank::new();
 let(mut source,_)=bank.source(RetainedCloneSource::admission_owned(make,()));
 let copy=source.borrow().binding_copy_bytes();let mut binding=None;
 bank.success_eff(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()},|effect|source.borrow().bind(&mut binding,effect),|p|p.unwrap_or_default());
 for _ in 0..10000{if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let grant=exact_grant(d,&bank);bank.success_eff(grant,|effect|settled(source.close_step(effect.grant().maximum_copy_bytes,effect)),|s|s.progress());}
 let mut turns=0;let mut total=(0,0);let mut first_complete=false;
 if compact{
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<16,maximum_release_bytes:1<<16,maximum_depth:8,..Default::default()};
  let(turn,heap)=bank.observe_eff(grant,|effect|match RetainedCloneBinding::close_compact(&mut binding,effect){Some(poll)=>settled(poll),None=>panic!("exclusive lease close must run compactly")},|s|s.progress());
  let step=turn.result.unwrap();first_complete=matches!(step,crate::RetainedCloneStep::Complete(_));assert_eq!(step.progress().copied_items,1);assert!(turn.work>=1);total=(heap.0,heap.1);turns=1;
 }
 for _ in 0..10000{
  if binding.is_none(){break;}
  let d=RetainedCloneBinding::close_demand(&binding,4096).unwrap();let grant=exact_grant(d,&bank);
  let(_,heap)=bank.observe_eff(grant,|effect|settled(RetainedCloneBinding::close_one(&mut binding,effect.grant().maximum_copy_bytes,effect)),|s|s.progress());
  total.0+=heap.0;total.1+=heap.1;turns+=1;
 }
 assert!(binding.is_none());
 for _ in 0..10000{if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let grant=exact_grant(d,&bank);bank.success_eff(grant,|effect|settled(source.close_step(effect.grant().maximum_copy_bytes,effect)),|s|s.progress());}
 (turns,total,first_complete)
}
#[test]
fn compact_close_runs_a_cold_exclusive_source_in_one_turn_and_a_frame_source_resumably(){
 let page=||(0..4096u32).map(|index|index as u8).collect::<Vec<u8>>();
 let(stepped_turns,stepped,_)=compact_turn(page(),false);
 let(compact_turns,compacted,complete)=compact_turn(page(),true);
 assert!(complete&&compact_turns==1,"cold exclusive close completes in one turn, not {compact_turns}");
 assert_eq!(compacted,stepped,"one turn releases exactly the stepwise bytes with no birth");assert!(stepped_turns>1);
 let record=||Captured{owner:"original-source".into(),metadata:"original-authority".into()};
 let(stepped_turns,stepped,_)=compact_turn(record(),false);
 let(resumed_turns,resumed,complete)=compact_turn(record(),true);
 assert!(!complete&&resumed_turns<stepped_turns,"a frame source stops before its first frame birth and resumes stepwise");
 assert_eq!(resumed.1-resumed.0,stepped.1-stepped.0,"resumed close keeps the net release");
}
