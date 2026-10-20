//! 🧪️ Immutable projections preserve exact original roots independently of payload size and cancellation.
use semio_framework_test_fixtures::fixture_text;
use super::{SnapshotProjection,SnapshotProjectionCursor};
#[path="📦️owned/🦀️.rs"]
mod owned;
#[path="../🧳️record/🧪️tests/🦀️.rs"]
mod record;
use crate::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneSource},retirement::controlled::ControlledRetirement,shared_issuance::{SealedShared,SharedIssuer}};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
#[derive(crate::RetireOwned)]
#[value(crate="crate")]
struct Payload{rows:Vec<u8>}
impl Clone for Payload{fn clone(&self)->Self{panic!("immutable projection must never clone payloads")}}
fn funded(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn drain<T:crate::retirement::RetireOwned>(value:T){
 let mut owner=ControlledRetirement::new(value).map_err(|(e,_)|e).unwrap();
 for _ in 0..100_000{if owner.terminal_is_empty(){return;}let copy=4096usize.max(owner.next_copy_byte_demand().unwrap());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
 panic!("immutable projection original closure did not terminate");
}
#[test]
fn quick_immutable_snapshot_projection_original_root_and_every_prefix(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){
  let count=row["items"].as_u64().unwrap()as usize;let cut=cut.as_u64().unwrap()as usize;
  let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<Payload>::birth_bytes(),maximum_depth:1,..Default::default()};
  let(root,_)=SealedShared::admit(Payload{rows:vec![7;count]},SharedIssuer::owned(),grant).map_err(|(e,_)|e).unwrap();let identity=root.identity();
  let birth=RetainedCloneSource::<SealedShared<Payload>>::owned_constructor_demand::<()>();let grant=RetainedCloneGrant{maximum_copy_bytes:RetainedCloneSource::<SealedShared<Payload>>::constructor_copy_bytes(),..funded(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()})};
  let(source,_)=RetainedCloneSource::admit_owned(root,(),grant).map_err(|(e,..)|e).unwrap();
  type Snapshot=SealedShared<Payload>;let birth=Snapshot::projection_birth_demand();let grant=funded(birth);
  for deny in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(birth.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:birth.copy_bytes.saturating_sub(1),..grant}),(birth.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:birth.capacity_bytes.saturating_sub(1),..grant}),(birth.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:birth.release_bytes.saturating_sub(1),..grant}),(birth.depth>0).then_some(RetainedCloneGrant{maximum_depth:birth.depth.saturating_sub(1),..grant})].into_iter().flatten(){let(result,heap)=observe_retirement_allocations(||Snapshot::begin_projection(deny));assert!(result.is_err());assert_eq!(heap,(0,0));}
  let((mut cursor,p),heap)=observe_retirement_allocations(||Snapshot::begin_projection(grant).unwrap());assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));let mut turns=0;
  while turns<cut&& !cursor.ready(){let d=cursor.demands(source.borrow(),4096).unwrap();let grant=funded(d);
   for deny in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..grant}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..grant})].into_iter().flatten(){let(result,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,deny).unwrap());assert_eq!(result.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.demands(source.borrow(),4096).unwrap(),d);}
   let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(0,0));assert!(step.progress().copied_items>0);turns+=1;
  }
  if cursor.ready(){assert_eq!(turns,law["expected"]["normalTurns"].as_u64().unwrap()as usize);let d=cursor.take_demand().unwrap();let grant=funded(d);
   for deny in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..grant}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..grant})].into_iter().flatten(){let(output,heap)=observe_retirement_allocations(||cursor.take(deny).unwrap());assert!(output.is_none());assert!(cursor.ready());assert_eq!(heap,(0,0));}
   if cut>turns{let(Some((output,p)),heap)=observe_retirement_allocations(||cursor.take(grant).unwrap())else{panic!("funded immutable output omitted")};assert!(p.fits(grant));assert_eq!(heap,(0,0));assert_eq!(output.identity(),identity);assert_eq!(output.get().rows.len(),count);assert_eq!(output.get().rows.as_ptr(),source.borrow().get().get().rows.as_ptr());drain(output);}
  }
  cursor.begin_close();for _ in 0..100_000{if cursor.terminal_is_empty(){break;}let d=cursor.close_demands(4096).unwrap();let grant=RetainedCloneGrant{maximum_copy_bytes:4096usize.max(d.copy_bytes),..funded(d)};for deny in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..grant}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..grant})].into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||cursor.close_step(deny).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.close_demands(4096).unwrap(),d);}let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress()!=Default::default()||cursor.terminal_is_empty());}
  assert!(cursor.terminal_is_empty());assert_eq!(source.borrow().get().identity(),identity);assert_eq!(source.borrow().get().get().rows.len(),count);assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));drain(source);
  eprintln!("[DEBUG] Immutable snapshot projection name={} rows={count} cut={cut} turns={turns} root identity retained; zero payload clones, zero normal heap births, paid denied output preserved, original cursor/source owners terminal",row["name"]);
 }}
}

#[derive(crate::RetireOwned)]
#[value(crate="crate")]
struct InvalidReceiptRoot{rows:SealedShared<Payload>}
impl super::ImmutableRootProjection for InvalidReceiptRoot{
 fn root_projection_demand(_original:crate::retained_clone::RetainedCloneRef<'_,Self>)->Result<RetirementDemand,crate::ValueError>{Ok(RetirementDemand{copy_bytes:3*std::mem::size_of::<Self>(),depth:1,..Default::default()})}
 fn project_root(original:crate::retained_clone::RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->Result<Option<(Self,crate::retained_clone::RetainedCloneProgress)>,crate::ValueError>{let(rows,_)=original.get().rows.try_duplicate(grant)?;Ok(Some((Self{rows},crate::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:grant.maximum_copy_bytes+1,..Default::default()})))}
}
#[test]
fn quick_immutable_snapshot_projection_bad_receipt_keeps_unadmitted_original_owner(){
 let(root,_)=SealedShared::admit(Payload{rows:vec![3;4096]},SharedIssuer::owned(),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<Payload>::birth_bytes(),maximum_depth:1,..Default::default()}).map_err(|(e,_)|e).unwrap();let identity=root.identity();
 let original=InvalidReceiptRoot{rows:root};let d=RetainedCloneSource::<InvalidReceiptRoot>::owned_constructor_demand::<()>();let grant=RetainedCloneGrant{maximum_copy_bytes:RetainedCloneSource::<InvalidReceiptRoot>::constructor_copy_bytes(),..funded(RetirementDemand{capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})};let(source,_)=RetainedCloneSource::admit_owned(original,(),grant).map_err(|(e,..)|e).unwrap();
 let(mut cursor,_)=InvalidReceiptRoot::begin_projection(funded(InvalidReceiptRoot::projection_birth_demand())).unwrap();let d=cursor.demands(source.borrow(),4096).unwrap();cursor.advance(source.borrow(),4096,funded(d)).unwrap();let d=cursor.demands(source.borrow(),4096).unwrap();
 assert!(source.borrow().get().rows.original_aliases_are_returned());let(error,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,funded(d)).unwrap_err());assert_eq!(error.kind,crate::ValueRefusalKind::InvariantViolated);assert_eq!(heap,(0,0));assert!(!cursor.ready());assert!(cursor.take(funded(d)).is_err());assert!(!source.borrow().get().rows.original_aliases_are_returned());
 cursor.begin_close();for _ in 0..10_000{if cursor.terminal_is_empty(){break;}let d=cursor.close_demands(4096).unwrap();let grant=RetainedCloneGrant{maximum_copy_bytes:4096usize.max(d.copy_bytes),..funded(d)};let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
 assert!(cursor.terminal_is_empty());assert_eq!(source.borrow().get().rows.identity(),identity);assert!(source.borrow().get().rows.original_aliases_are_returned());drop(cursor);drain(source);eprintln!("[DEBUG] Immutable snapshot bad receipt retains exact candidate before validation, rejects output handoff, closes original shared issuer, original source alias count restored");
}

#[derive(crate::RetireOwned,crate::SnapshotProjection)]
#[value(crate="crate")]
struct ImmutableRecord{epoch:u64,primary:SealedShared<Payload>,secondary:SealedShared<Payload>}
fn execute_record(count:usize,cut:Option<usize>,policy:&serde_json::Value)->usize{
 let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<Payload>::birth_bytes(),maximum_depth:1,..Default::default()};
 let(primary,_)=SealedShared::admit(Payload{rows:vec![1;count]},SharedIssuer::owned(),grant).map_err(|(e,_)|e).unwrap();let(secondary,_)=SealedShared::admit(Payload{rows:vec![2;count]},SharedIssuer::owned(),grant).map_err(|(e,_)|e).unwrap();let roots=(primary.identity(),secondary.identity());
 let original=ImmutableRecord{epoch:policy["epoch"].as_u64().unwrap(),primary,secondary};let d=RetainedCloneSource::<ImmutableRecord>::owned_constructor_demand::<()>();let grant=RetainedCloneGrant{maximum_copy_bytes:RetainedCloneSource::<ImmutableRecord>::constructor_copy_bytes(),..funded(RetirementDemand{capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})};let(source,_)=RetainedCloneSource::admit_owned(original,(),grant).map_err(|(e,..)|e).unwrap();
 let d=ImmutableRecord::projection_birth_demand();let(mut cursor,_)=ImmutableRecord::begin_projection(funded(d)).unwrap();let mut turns=0;
 while !cursor.ready()&&cut!=Some(turns){let d=cursor.demands(source.borrow(),4096).unwrap();let grant=RetainedCloneGrant{maximum_copy_bytes:4096usize.max(d.copy_bytes),..funded(d)};
  for deny in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..grant}),Some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..grant})].into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,deny).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.demands(source.borrow(),4096).unwrap(),d);}
  let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_ne!(step.progress(),Default::default());turns+=1;assert!(turns<policy["maximumTurns"].as_u64().unwrap()as usize);
 }
 if cursor.ready()&&cut.is_none_or(|cut|cut>turns){let d=cursor.take_demand().unwrap();let grant=funded(d);for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(out,heap)=observe_retirement_allocations(||cursor.take(denied).unwrap());assert!(out.is_none());assert_eq!(heap,(0,0));assert!(cursor.ready());}let(output,p)=cursor.take(grant).unwrap().unwrap();assert!(p.fits(grant));assert_eq!(output.epoch,policy["epoch"].as_u64().unwrap());assert_eq!((output.primary.identity(),output.secondary.identity()),roots);assert_eq!(output.primary.get().rows.as_ptr(),source.borrow().get().primary.get().rows.as_ptr());assert_eq!(output.secondary.get().rows.as_ptr(),source.borrow().get().secondary.get().rows.as_ptr());assert_eq!(output.primary.get().rows.len(),count);assert_eq!(output.secondary.get().rows.len(),count);drain(output);}
 cursor.begin_close();for _ in 0..100_000{if cursor.terminal_is_empty(){break;}let d=cursor.close_demands(4096).unwrap();let grant=RetainedCloneGrant{maximum_copy_bytes:4096usize.max(d.copy_bytes),..funded(d)};let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress()!=Default::default()||cursor.terminal_is_empty());}
 assert!(cursor.terminal_is_empty());assert_eq!((source.borrow().get().primary.identity(),source.borrow().get().secondary.identity()),roots);assert_eq!(source.borrow().get().epoch,policy["epoch"].as_u64().unwrap());assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));drain(source);turns
}
#[test]
fn quick_immutable_snapshot_projection_composed_record_every_prefix(){let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut fixed_turns=None;for row in law["cases"].as_array().unwrap(){let count=row["items"].as_u64().unwrap()as usize;let turns=execute_record(count,None,&law["composition"]);if let Some(fixed)=fixed_turns{assert_eq!(turns,fixed,"immutable record projection work must not depend on payload cardinality");}else{fixed_turns=Some(turns);}for cut in 0..=turns+1{execute_record(count,Some(cut),&law["composition"]);}eprintln!("[DEBUG] Immutable composed record rows={count} normalTurns={turns} all normal cancellation prefixes terminal; genuine field projections, no payloadClone, original roots stable, exact heap ownership receipts and paid output");}}

#[test]
fn stable_projection_output_preserves_ownership_on_every_denied_placement(){
 use super::ProjectionOutput;use std::mem::MaybeUninit;
 let mut output=Box::new(42u64);let pointer=output.as_ref()as*const u64;let mut target=MaybeUninit::new(7u64);let d=<Box<u64>as ProjectionOutput<u64>>::placement_demand();let g=funded(d);
 for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g},RetainedCloneGrant{maximum_depth:0,..g}]{let(result,heap)=observe_retirement_allocations(||<Box<u64>as ProjectionOutput<u64>>::place(output,&mut target,denied));let(_,returned)=result.err().unwrap();output=returned;assert_eq!(heap,(0,0));assert_eq!(output.as_ref()as*const u64,pointer);assert_eq!(*output,42);assert_eq!(unsafe{*target.assume_init_ref()},7);}
 let(result,heap)=observe_retirement_allocations(||<Box<u64>as ProjectionOutput<u64>>::place(output,&mut target,g));let p=result.map_err(|(e,_)|e).unwrap();assert!(p.fits(g));assert_eq!(heap,(0,std::mem::size_of::<u64>()));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(unsafe{target.assume_init()},42);
}
