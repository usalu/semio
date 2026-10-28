#[path="../../../../🌱️value/📽️projection/👁️post/🧪️testing/🦀️.rs"]
mod post_fixture;
use post_fixture::PostFixture;
use semio_framework_test_fixtures::fixture_text;
use super::*;
use super::diff::tests::{Projection,Diff};
use crate::{MutationOutcome,MutationLeafDescriptor};
use semio_framework_value::{retirement::RetireOwned,retained_clone::RetainedCloneSource};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use std::sync::atomic::{AtomicUsize,AtomicBool,Ordering};
#[derive(Clone,Debug,PartialEq,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue,semio_framework_value::RetireOwned)]
struct Operation{set:i64,refused:bool}
impl Mutation<Projection> for Operation{
 type Diff=Diff;const DESCRIPTORS:&'static[MutationLeafDescriptor]=&[];
 fn descriptor(&self)->&'static MutationLeafDescriptor{panic!("retained fixture has no cold descriptor")}
}
struct Preparation{decision:ManuallyDrop<Option<Diff>>,inverse:ManuallyDrop<Option<Vec<Operation>>>,retirement:RetainedCloneClose,inverse_close:Option<FactoryVectorRetirement<Operation>>,refused:bool,phase:u8,closing:bool}
impl RetainedMutation<Projection> for Operation{type Preparation=Preparation;}
impl RetainedMutationPreparation<Projection,Operation> for Preparation{
 fn new()->Self{Self{decision:ManuallyDrop::new(None),inverse:ManuallyDrop::new(None),retirement:Default::default(),inverse_close:None,refused:false,phase:0,closing:false}}
 fn decision_demand(&self,_:RetainedCloneRef<'_,Operation>,_:RetainedCloneRef<'_,Projection>,_:usize)->Result<RetirementDemand,ValueError>{assert_eq!(self.phase,0);Ok(scalar(size_of::<Diff>()+size_of::<u8>()+size_of::<bool>()))}
 fn decide(&mut self,m:RetainedCloneRef<'_,Operation>,p:RetainedCloneRef<'_,Projection>,body:usize,g:RetainedCloneGrant)->Result<RetainedMutationAdvance,ValueError>{let d=self.decision_demand(m,p,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedMutationAdvance::Progress(Default::default()));}self.refused=m.get().refused;if !self.refused{*self.decision=Some(Diff{set:m.get().set,refused:false});}self.phase=1;Ok(RetainedMutationAdvance::Ready(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 fn take_decision(&mut self)->RetainedMutationDecision<Diff>{assert_eq!(self.phase,1);self.phase=2;if self.refused{RetainedMutationDecision::Refused(Vec::new())}else{RetainedMutationDecision::Admitted(self.decision.take().unwrap(),Vec::new())}}
 fn inverse_demand(&self,_:RetainedCloneRef<'_,Operation>,_:RetainedCloneRef<'_,Projection>,_:RetainedCloneRef<'_,Diff>,_:usize)->Result<RetirementDemand,ValueError>{assert_eq!(self.phase,2);Ok(RetirementDemand{copy_bytes:size_of::<Operation>()+size_of::<Vec<Operation>>()+size_of::<u8>(),capacity_bytes:size_of::<Operation>(),depth:1,..Default::default()})}
 fn inverse(&mut self,m:RetainedCloneRef<'_,Operation>,p:RetainedCloneRef<'_,Projection>,d:RetainedCloneRef<'_,Diff>,body:usize,g:RetainedCloneGrant)->Result<RetainedMutationAdvance,ValueError>{let demand=self.inverse_demand(m,p,d,body)?;if !retained_mutation_demand_fits(demand,g){return Ok(RetainedMutationAdvance::Progress(Default::default()));}assert_eq!(d.get().set,m.get().set);*self.inverse=Some(Vec::with_capacity(1));self.inverse.as_mut().unwrap().push(Operation{set:p.get().value,refused:false});self.phase=3;Ok(RetainedMutationAdvance::Ready(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:self.inverse.as_ref().unwrap().capacity()*size_of::<Operation>(),..Default::default()}))}
 fn take_inverse(&mut self)->Vec<Operation>{assert_eq!(self.phase,3);self.phase=4;self.inverse.take().unwrap()}
 fn begin_close(&mut self){self.closing=true;}
 fn close_demand(&self,c:RetainedMutationCloseContext<'_,Operation>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?});}
  if self.decision.is_some(){return Ok(self.retirement.next_owner_demands::<Diff>(true,&None,body)?);}
  if let Some(i)=self.inverse_close.as_ref(){return i.demands(c.mutations,body);}
  Ok(if self.inverse.is_some(){FactoryVectorRetirement::<Operation>::constructor_demand()}else{Default::default()})
 }
 fn close_step(&mut self,c:RetainedMutationCloseContext<'_,Operation>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  assert!(self.closing);if !retained_mutation_demand_fits(self.close_demand(c,g.maximum_copy_bytes)?,g){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(g).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(s)=self.retirement.begin_granted(&mut self.decision,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(i)=self.inverse_close.as_mut(){let s=i.close_step(c.mutations,g)?;if i.terminal_is_empty(){self.inverse_close=None;}return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(rows)=self.inverse.take(){return match FactoryVectorRetirement::admit(rows,g){Ok((i,p))=>{self.inverse_close=Some(i);Ok(RetainedCloneStep::Progress(p))},Err((e,rows))=>{*self.inverse=Some(rows);Err(e)}};}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn terminal_is_empty(&self)->bool{self.decision.is_none()&&self.inverse.is_none()&&self.inverse_close.is_none()&&self.retirement.is_empty()}
}
impl Drop for Preparation{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking());if empty{unsafe{ManuallyDrop::drop(&mut self.decision);ManuallyDrop::drop(&mut self.inverse);}}}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(f:impl FnOnce()->T)->(T,(usize,usize)){let(v,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);(v,(h.requested_bytes,h.released_bytes))}
fn source<T:RetireOwned+Sync+'static>(v:T)->RetainedCloneSource<T>{let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096};match RetainedCloneSource::admit_owned(v,(),g){Ok((s,_))=>s,Err((e,_,_))=>panic!("{e}")}}
fn close_source<T:RetireOwned+Sync+'static>(s:&mut RetainedCloneSource<T>){let mut turns=0;while !s.terminal_is_empty(){let g=grant(RetirementDemand{copy_bytes:s.next_close_copy_byte_demand().unwrap(),capacity_bytes:s.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:s.next_close_release_byte_demand().unwrap(),depth:s.next_close_depth_demand().unwrap()});s.close_step(g).unwrap();turns+=1;assert!(turns<10000);}}
fn close_owner(o:&mut RetainedMutationOwner<Projection,Operation>,c:RetainedMutationCloseContext<'_,Operation>){o.begin_close();let mut turns=0;while !o.terminal_is_empty(){let g=grant(o.close_demand(c,4096).unwrap());let(_,h)=observe(||o.close_step(c,Default::default()).unwrap());assert_eq!(h,(0,0));o.close_step(c,g).unwrap();turns+=1;assert!(turns<10000);}}
fn close_diff(o:&mut RetainedDiffOwner<Projection,Diff>){o.begin_close();for turn in 0..1000{if o.terminal_is_empty(){return;}let g=grant(o.close_demand(4096).unwrap());let step=o.close_step(g).unwrap();assert!(turn<999,"diff closure stalled: demand={g:?}, receipt={:?}",step.progress());}}
fn decision(o:&mut RetainedMutationOwner<Projection,Operation>,m:&RetainedCloneSource<Operation>,p:&RetainedCloneSource<Projection>)->Option<Diff>{let mut turns=0;loop{let g=grant(o.decision_demand(m.borrow(),p.borrow(),4096).unwrap());let(s,h)=observe(||decide_retained_mutation(o,m.borrow(),p.borrow(),4096,Default::default()).unwrap());assert_eq!(s.progress(),Default::default());assert_eq!(h,(0,0));let(s,h)=observe(||decide_retained_mutation(o,m.borrow(),p.borrow(),4096,g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));match s{RetainedMutationDecisionStep::Progress(_)=>{},RetainedMutationDecisionStep::Admitted(_,d,msg)=>{assert!(msg.is_empty());return Some(d)},RetainedMutationDecisionStep::Refused(_,msg)=>{assert!(msg.is_empty());return None}}turns+=1;assert!(turns<1000);}}
fn preflight(o:&mut RetainedDiffOwner<Projection,Diff>,d:&RetainedCloneSource<Diff>,p:&RetainedCloneSource<Projection>,post:&PostFixture<Projection>){let mut receiving_callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1<<24,&mut receiving_callback);assert!(o.inverse_admission(d.borrow(),p.borrow()).is_err());let mut turns=0;loop{turns+=1;assert!(turns<1000,"central preflight stalled");let g=grant(o.preflight_demand(d.borrow(),p.borrow(),post.borrow(),4096).unwrap());match preflight_retained_diff(o,d.borrow(),p.borrow(),post.borrow(),4096,g,&mut receiving).unwrap(){RetainedDiffPreflightStep::Progress(_)=>{},RetainedDiffPreflightStep::Admitted(_,rows)=>{assert!(rows.is_empty());return},RetainedDiffPreflightStep::Refused(..)=>panic!("forward fixture unexpectedly refused")}}}

/// 🧫️ Portable decision → original typed D → preflight → inverse → central commit agrees with the SQLite oracle.
#[test]
fn retained_mutation_yields_typed_diff_before_preflight_inverse_and_commit(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let base=case["base"].as_i64().unwrap();let mut m=source(Operation{set:case["set"].as_i64().unwrap(),refused:case["refused"].as_bool().unwrap()});let mut p=source(Projection{value:base,untouched:vec![1;256]});let mut post=Projection{value:base,untouched:vec![2;256]};let pointer=post.untouched.as_ptr();
  let(mut o,_)=begin_retained_mutation::<Projection,Operation>(grant(retained_mutation_birth_demand::<Projection,Operation>())).unwrap();let factory=semio_framework_value::factory::OwnedValueRetirementFactory::<Operation>::default();let c=RetainedMutationCloseContext{mutations:&factory};
  if let Some(diff)=decision(&mut o,&m,&p){
   assert_eq!(post.value,base);let mut d=source(diff);let(mut apply,_)=begin_retained_diff::<Projection,Diff>(grant(retained_diff_birth_demand::<Projection,Diff>())).unwrap();preflight(&mut apply,&d,&p,&post);assert_eq!(post.value,base);
   let a=apply.inverse_admission(d.borrow(),p.borrow()).unwrap();let rows=loop{let g=grant(o.inverse_demand(m.borrow(),&a,4096).unwrap());let(s,h)=observe(||inverse_retained_mutation(&mut o,m.borrow(),&a,4096,g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));assert_eq!(post.value,base);match s{RetainedMutationInverseStep::Progress(_)=>{},RetainedMutationInverseStep::Ready(_,rows)=>break rows}};
   assert_eq!(rows,vec![Operation{set:base,refused:false}]);let mut rows=source(rows);close_source(&mut rows);
   loop{let g=grant(apply.commit_demand(d.borrow(),p.borrow(),post.mutable_read(),4096).unwrap());if matches!(commit_retained_diff(&mut apply,d.borrow(),p.borrow(),&mut post,g).unwrap(),RetainedCloneStep::Complete(_)){break;}}
   assert_eq!(post.value,case["expected"].as_i64().unwrap());assert_eq!(post.untouched.as_ptr(),pointer);close_owner(&mut o,c);close_diff(&mut apply);close_source(&mut d);
  }else{assert!(case["refused"].as_bool().unwrap());assert_eq!(post.value,base);close_owner(&mut o,c);}
  close_source(&mut p);close_source(&mut m);
 }
}
#[derive(Default,semio_framework_value::FactoryPayloadRetirement)]
struct InverseIssuer{
 #[factory_owned] attempts:AtomicUsize,
 #[factory_owned] issued:AtomicUsize,
 #[factory_owned] last_set:AtomicUsize,
 #[factory_owned] refuse:AtomicBool,
}
impl semio_framework_value::ArtifactOwnedValueRetirementFactory<Operation> for InverseIssuer{
 fn retirement_birth_bytes(&self,_:&Operation)->usize{semio_framework_value::owned_retirement_birth_bytes::<Operation>()}
 fn retire_owned(&self,row:Operation,g:RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Operation)>{self.attempts.fetch_add(1,Ordering::SeqCst);self.last_set.store(usize::try_from(row.set).unwrap(),Ordering::SeqCst);if self.refuse.swap(false,Ordering::SeqCst){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"exact original issuer refuses once"),row));}self.issued.fetch_add(1,Ordering::SeqCst);semio_framework_value::admit_owned_retirement(row,g)}
}
/// 🏭️ Cancellation after prepared inverse restores its exact row and allocation on original-provider refusal.
#[test]
fn retained_mutation_inverse_cancellation_preserves_exact_issuer_and_restores_refused_row(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let law=&corpus["issuer"];let mut m=source(Operation{set:7,refused:false});let mut p=source(Projection{value:law["inverseSet"].as_i64().unwrap(),untouched:vec![]});let post=PostFixture::new(Projection{value:law["inverseSet"].as_i64().unwrap(),untouched:vec![]});let(mut o,_)=begin_retained_mutation::<Projection,Operation>(grant(retained_mutation_birth_demand::<Projection,Operation>())).unwrap();let mut d=source(decision(&mut o,&m,&p).unwrap());let(mut apply,_)=begin_retained_diff::<Projection,Diff>(grant(retained_diff_birth_demand::<Projection,Diff>())).unwrap();preflight(&mut apply,&d,&p,&post);let a=apply.inverse_admission(d.borrow(),p.borrow()).unwrap();
 let mut inverse_turns=0;while o.phase!=4{inverse_turns+=1;assert!(inverse_turns<1000,"semantic inverse stalled phase={}",o.phase);let g=grant(o.inverse_demand(m.borrow(),&a,4096).unwrap());inverse_retained_mutation(&mut o,m.borrow(),&a,4096,g).unwrap();}
 let pointer=o.preparation.as_ref().unwrap().inverse.as_ref().unwrap().as_ptr();let row=o.preparation.as_ref().unwrap().inverse.as_ref().unwrap()[0].clone();let factory=InverseIssuer::default();factory.refuse.store(true,Ordering::SeqCst);let c=RetainedMutationCloseContext{mutations:&factory};o.begin_close();let mut refused=false;
 let mut close_turns=0;while !o.terminal_is_empty(){close_turns+=1;assert!(close_turns<1000,"semantic exact issuer close stalled phase={}",o.phase);let g=grant(o.close_demand(c,4096).unwrap());match o.close_step(c,g){Ok(_)=>{},Err(_)=>{assert!(!refused);refused=true;let retained=o.preparation.as_ref().unwrap().inverse_close.as_ref().unwrap();assert_eq!(retained.pending(),Some(&row));assert_eq!(retained.allocation_address(),pointer as usize);}}}
 assert!(refused);assert_eq!(factory.attempts.load(Ordering::SeqCst),law["attempts"].as_u64().unwrap()as usize);assert_eq!(factory.issued.load(Ordering::SeqCst),law["issued"].as_u64().unwrap()as usize);assert_eq!(post.value,law["inverseSet"].as_i64().unwrap());close_diff(&mut apply);close_source(&mut d);close_source(&mut p);close_source(&mut m);
}
/// 🎟️ The neutral grant corpus and every fixed constructor undergrant are inert.
#[test]
fn retained_mutation_all_grant_axes_refuse_before_constructor(){
 let c:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for v in c["grants"].as_array().unwrap(){let d=RetirementDemand{copy_bytes:v["demand"]["copyBytes"].as_u64().unwrap()as usize,capacity_bytes:v["demand"]["capacityBytes"].as_u64().unwrap()as usize,release_bytes:v["demand"]["releaseBytes"].as_u64().unwrap()as usize,depth:v["demand"]["depth"].as_u64().unwrap()as usize};let g:RetainedCloneGrant=serde_json::from_value(v["grant"].clone()).unwrap();assert_eq!(retained_mutation_demand_fits(d,g),v["admitted"].as_bool().unwrap());}
 let d=retained_mutation_birth_demand::<Projection,Operation>();let good=grant(d);for axis in 0..4{let mut g=good;match axis{0=>g.maximum_items=0,1=>g.maximum_copy_bytes=d.copy_bytes-1,2=>g.maximum_capacity_bytes=d.capacity_bytes-1,_=>g.maximum_depth=0}let(result,h)=observe(||begin_retained_mutation::<Projection,Operation>(g));assert!(result.is_none());assert_eq!(h,(0,0));}
}

#[derive(Clone,Copy,semio_framework_value::FactoryPayloadRetirement)]
struct ZstInverseOriginal;
#[derive(Clone,Copy,semio_framework_value::FactoryPayloadRetirement)]
struct ZstInverseOther;
union InverseIssuerSlot{original:ZstInverseOriginal,other:ZstInverseOther}
thread_local!{static OTHER_INVERSE_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
impl semio_framework_value::ArtifactOwnedValueRetirementFactory<Operation> for ZstInverseOriginal{
 fn retirement_birth_bytes(&self,_:&Operation)->usize{semio_framework_value::owned_retirement_birth_bytes::<Operation>()}
 fn retire_owned(&self,row:Operation,g:RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Operation)>{semio_framework_value::admit_owned_retirement(row,g)}
}
impl semio_framework_value::ArtifactOwnedValueRetirementFactory<Operation> for ZstInverseOther{
 fn retirement_birth_bytes(&self,_:&Operation)->usize{semio_framework_value::owned_retirement_birth_bytes::<Operation>()}
 fn retire_owned(&self,row:Operation,_:RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Operation)>{OTHER_INVERSE_CALLS.set(OTHER_INVERSE_CALLS.get()+1);Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"foreign zero-sized inverse issuer must not receive original rows"),row))}
}
/// 🔑️ Shared semantic cancellation refuses substituted method-table authority at an exact pending inverse row.
#[test]
fn retained_mutation_rejects_same_data_different_method_table_inverse_issuer(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../🌱️value/♻️retirement/🏭️factory/📋️vector/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let law=&corpus["issuerAlias"];let slot=InverseIssuerSlot{original:ZstInverseOriginal};let original=unsafe{&slot.original};let foreign=unsafe{&slot.other};assert_eq!(std::ptr::from_ref(original).cast::<()>(),std::ptr::from_ref(foreign).cast::<()>());OTHER_INVERSE_CALLS.set(0);
 let mut m=source(Operation{set:7,refused:false});let mut p=source(Projection{value:law["pendingId"].as_i64().unwrap(),untouched:vec![]});let post=PostFixture::new(Projection{value:law["pendingId"].as_i64().unwrap(),untouched:vec![]});let(mut o,_)=begin_retained_mutation::<Projection,Operation>(grant(retained_mutation_birth_demand::<Projection,Operation>())).unwrap();let mut d=source(decision(&mut o,&m,&p).unwrap());let(mut apply,_)=begin_retained_diff::<Projection,Diff>(grant(retained_diff_birth_demand::<Projection,Diff>())).unwrap();preflight(&mut apply,&d,&p,&post);let a=apply.inverse_admission(d.borrow(),p.borrow()).unwrap();
 for turn in 0..1000{if o.phase==4{break;}let g=grant(o.inverse_demand(m.borrow(),&a,4096).unwrap());inverse_retained_mutation(&mut o,m.borrow(),&a,4096,g).unwrap();assert!(turn<999);}
 let pointer=o.preparation.as_ref().unwrap().inverse.as_ref().unwrap().as_ptr()as usize;let c=RetainedMutationCloseContext{mutations:original};o.begin_close();
 for turn in 0..1000{if o.preparation.as_ref().unwrap().inverse_close.as_ref().is_some_and(|v|v.pending().is_some()){break;}let g=grant(o.close_demand(c,4096).unwrap());o.close_step(c,g).unwrap();assert!(turn<999);}
 let pending=o.preparation.as_ref().unwrap().inverse_close.as_ref().unwrap();assert_eq!(pending.pending().unwrap().set,law["pendingId"].as_i64().unwrap());assert_eq!(pending.allocation_address(),pointer);
 let g=grant(o.close_demand(c,4096).unwrap());let(result,heap)=observe(||o.close_step(RetainedMutationCloseContext{mutations:foreign},g));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(OTHER_INVERSE_CALLS.get(),law["providerCalls"].as_u64().unwrap()as usize);let pending=o.preparation.as_ref().unwrap().inverse_close.as_ref().unwrap();assert_eq!(pending.pending().unwrap().set,law["pendingId"].as_i64().unwrap());assert_eq!(pending.allocation_address(),pointer);assert_eq!(post.value,law["pendingId"].as_i64().unwrap());
 close_owner(&mut o,c);close_diff(&mut apply);close_source(&mut d);close_source(&mut p);close_source(&mut m);
}
