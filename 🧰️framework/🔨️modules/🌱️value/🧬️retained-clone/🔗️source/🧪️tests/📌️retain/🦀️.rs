//! 📌️ A genuine issued field remains the same original after its root caller closes.
use crate::{RetainedCloneSource,RetainedCloneGrant,RetainedCloneProgress,RetainedOwnedProjection,RetirementDemand,observe_retirement_allocations};
use crate::source_law_bank::{Bank,observed,settled,CPU};
fn grant(d:RetirementDemand,bank:&Bank)->RetainedCloneGrant{assert!(d.copy_bytes<=4096);bank.grant(d)}
fn source(value:Vec<String>,bank:&mut Bank)->RetainedCloneSource<Vec<String>>{bank.source(RetainedCloneSource::admission_owned(value,())).0}
fn close_source(owner:&mut RetainedCloneSource<Vec<String>>,bank:&mut Bank){for _ in 0..10000{if owner.terminal_is_empty(){return;}let g=grant(owner.next_close_demand(4096).unwrap(),bank);let(s,h)=bank.run_eff(g,|effect|settled(owner.close_step(effect)),|step|step.progress());let s=s.unwrap();assert!(s.progress().fits(g));assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));}panic!("original root did not close");}
#[test]
fn retained_issued_row_reference_preserves_original_pointer_path_and_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let rank=law["selected"].as_u64().unwrap()as usize;
 for root_first in law["rootFirst"].as_array().unwrap(){let mut bank=Bank::new();let rows=law["rows"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_owned()).collect();let mut root=source(rows,&mut bank);let mut foreign=source(vec!["foreign".into()],&mut bank);let row=root.borrow().project(rank,|rows|&rows[rank]);let pointer=row.get().as_ptr();let identity=row.source_identity();let d=RetainedOwnedProjection::<String>::constructor_demand();let g=grant(d,&bank);
  for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{let(result,h)=observed(denied,CPU,&mut None,|effect|row.retain_owned(effect));assert!(result.result.is_err());assert_eq!(h,(0,0));}
  let(result,h)=bank.run_eff(g,|effect|row.retain_owned(effect),|(_,p)|*p);let(mut retained,p)=result.unwrap();assert!(p.fits(g));assert_eq!(h,(0,0));assert!(retained.matches_source(row));assert!(!retained.matches_source(foreign.borrow().project(0,|v|&v[0])));assert_eq!(retained.borrow().unwrap().source_identity(),identity);
  if root_first.as_bool().unwrap(){close_source(&mut root,&mut bank);}assert_eq!(retained.borrow().unwrap().get().as_ptr(),pointer);assert_eq!(retained.borrow().unwrap().get().as_str(),law["retained"].as_str().unwrap());
  for _ in 0..10000{if retained.terminal_is_empty(){break;}let d=retained.close_demand(4096).unwrap();let g=grant(d,&bank);for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g},RetainedCloneGrant{maximum_capacity_bytes:0,..g},RetainedCloneGrant{maximum_release_bytes:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{if crate::retained_clone_grant_admits(denied,d){continue;}let(result,h)=observed(denied,CPU,&mut None,|effect|settled(retained.close_step(effect)));let p=match result.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert_eq!(p,RetainedCloneProgress::default());assert_eq!(h,(0,0));assert_eq!(retained.close_demand(4096).unwrap(),d);}let(s,h)=bank.run_eff(g,|effect|settled(retained.close_step(effect)),|step|step.progress());let s=s.unwrap();assert!(s.progress().fits(g));assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));}assert!(retained.terminal_is_empty());assert!(retained.borrow().is_err());assert_eq!(observe_retirement_allocations(||drop(retained)).1,(0,0));close_source(&mut root,&mut bank);close_source(&mut foreign,&mut bank);
 }
}

#[test]
fn fallible_child_retention_is_funded_before_accessor_and_preserves_original_alias(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in law["childCases"].as_array().unwrap(){
  let rows:Vec<String>=serde_json::from_value(law["rows"].clone()).unwrap();let rank=row["selected"].as_u64().unwrap()as usize;
  let mut bank=Bank::new();let mut root=source(rows,&mut bank);let g=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_depth:row["depth"].as_u64().unwrap()as usize,..grant(RetainedOwnedProjection::<String>::constructor_demand(),&bank)};let calls=std::cell::Cell::new(0);
  if row["retained"].as_bool().unwrap(){
   let(result,heap)=bank.run_eff(g,|effect|root.borrow().try_project_owned(rank,|rows|{calls.set(calls.get()+1);rows.get(rank).ok_or_else(||crate::ValueError::invariant("absent original child"))},effect),|(_,p)|*p);
   let(mut retained,p)=result.unwrap();assert_eq!(heap,(0,0));assert!(p.fits(g));assert_eq!(retained.borrow().unwrap().get().as_str(),law["rows"][rank].as_str().unwrap());assert_eq!(retained.borrow().unwrap().get().as_ptr(),root.borrow().get()[rank].as_ptr());assert_eq!(retained.borrow().unwrap().source_identity(),root.borrow().source_identity());
   for _ in 0..10000{if retained.terminal_is_empty(){break}let g=grant(retained.close_demand(4096).unwrap(),&bank);let(result,_)=bank.run_eff(g,|effect|settled(retained.close_step(effect)),|s|s.progress());result.unwrap();}
   assert!(retained.terminal_is_empty());
  }else{
   let(result,heap)=observed(g,CPU,&mut None,|effect|root.borrow().try_project_owned(rank,|rows|{calls.set(calls.get()+1);rows.get(rank).ok_or_else(||crate::ValueError::invariant("absent original child"))},effect));assert!(result.result.is_err());assert_eq!(result.work,0);assert_eq!(heap,(0,0));
  }
  assert_eq!(calls.get(),row["accessorCalls"].as_u64().unwrap()as usize);close_source(&mut root,&mut bank);
 }
}
