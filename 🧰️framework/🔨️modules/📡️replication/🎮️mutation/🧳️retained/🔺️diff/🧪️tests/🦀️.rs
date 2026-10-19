use semio_framework_test_fixtures::fixture_text;
use super::*;
#[path="../🫴️receiving/🧪️tests/🦀️.rs"]mod receiving;
use crate::DiffAlgebra;
use crate::retained::{RetainedFieldPreparation,RetainedDiagnosticPreparation,RetainedDiagnosticSpec};
use semio_framework_value::{retirement::RetireOwned,retained_clone::RetainedCloneSource};
#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct Projection{pub(crate) value:i64,pub(crate) untouched:Vec<u64>}
#[derive(Clone,Default,Debug,PartialEq,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue,semio_framework_value::RetireOwned)]
pub(crate) struct Diff{pub(crate) set:i64,pub(crate) refused:bool}
impl DiffAlgebra<Projection> for Diff{fn is_empty(&self)->bool{false}}
impl MutationDiff<Projection> for Diff{}
pub(crate) struct Apply{field:RetainedFieldPreparation<i64>,diagnostic:RetainedDiagnosticPreparation<0>,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,phase:usize,prepared:bool,committed:bool,closing:bool}
impl RetainedDiff<Projection> for Diff{type Apply=Apply;}
impl RetainedDiffApply<Projection,Diff> for Apply{
 fn new()->Self{Self{field:Default::default(),diagnostic:Default::default(),messages:ManuallyDrop::new(None),ledger:ManuallyDrop::new(None),phase:0,prepared:false,committed:false,closing:false}}
 fn preflight_demand(&self,diff:RetainedCloneRef<'_,Diff>,_:RetainedCloneRef<'_,Projection>,_:&Projection,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.prepared{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture diff preflight already settled"));}
  Ok(match self.phase{0=>RetirementDemand{copy_bytes:size_of::<usize>()+size_of::<bool>(),depth:1,..Default::default()},1=>{if self.field.value().is_some(){RetirementDemand{copy_bytes:size_of::<usize>()+size_of::<bool>(),depth:1,..Default::default()}}else{self.field.demand(diff.project(0,|diff|&diff.set),body)?}},2=>RetainedDiagnosticPreparation::<0>::start_demand(),3=>self.diagnostic.demand([],body)?,4=>RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>()*2+size_of::<usize>(),depth:1,..Default::default()},_=>unreachable!("fixture preflight phase")})
 }
 fn preflight(&mut self,diff:RetainedCloneRef<'_,Diff>,original:RetainedCloneRef<'_,Projection>,post:&Projection,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<RetainedDiffPreflightAdvance,ValueError>{
  let demand=self.preflight_demand(diff,original,post,body)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}
  let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  match self.phase{
   0=>{self.phase=if diff.get().refused{2}else{1};Ok(RetainedDiffPreflightAdvance::Progress(receipt))},
   1=>{if self.field.value().is_some(){self.prepared=true;Ok(RetainedDiffPreflightAdvance::Ready(receipt,true))}else{let step=self.field.advance(diff.project(0,|diff|&diff.set),body,grant)?;Ok(RetainedDiffPreflightAdvance::Progress(step.progress()))}},
   2=>{let receipt=self.diagnostic.start(RetainedDiagnosticSpec{level:semio_framework_diagnostic::Severity::Fatal,code:"mutation.invariant",message:"The authored diff is inadmissible."},grant)?;self.phase=3;Ok(RetainedDiffPreflightAdvance::Progress(receipt))},
   3=>{let step=self.diagnostic.advance([],body,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=4;}Ok(RetainedDiffPreflightAdvance::Progress(step.progress()))},
   4=>{let(rows,mut receipt)=self.diagnostic.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Vec<MutationMessage>>()-size_of::<usize>(),..grant})?.expect("admitted original fixture diagnostic");*self.messages=Some(rows);receipt.copied_bytes+=size_of::<Vec<MutationMessage>>()+size_of::<usize>();self.prepared=true;Ok(RetainedDiffPreflightAdvance::Ready(receipt,true))},
   _=>unreachable!("fixture admitted preflight"),
  }
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{self.messages.take().unwrap_or_default()}
 fn commit_demand(&self,_:RetainedCloneRef<'_,Diff>,_:RetainedCloneRef<'_,Projection>,_:&Projection,_:usize)->Result<RetirementDemand,ValueError>{if !self.prepared||self.committed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture diff not ready for owned commit"));}let mut demand=self.field.exchange_demand()?;demand.copy_bytes+=size_of::<bool>();Ok(demand)}
 fn commit(&mut self,diff:RetainedCloneRef<'_,Diff>,original:RetainedCloneRef<'_,Projection>,post:&mut Projection,grant:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let demand=self.commit_demand(diff,original,post,grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut receipt=self.field.install(&mut post.value,capability,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>(),..grant})?;receipt.copied_bytes+=size_of::<bool>();self.committed=true;Ok(RetainedCloneStep::Complete(receipt))}
 fn begin_close(&mut self){self.closing=true;self.field.begin_close();self.diagnostic.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.field.terminal_is_empty(){return self.field.close_demand(body);}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body);}
  if self.messages.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>()+size_of::<MutationMessageLedgerRetirement>(),depth:1,..Default::default()});}
  if let Some(ledger)=self.ledger.as_ref(){return Ok(RetirementDemand{copy_bytes:ledger.next_copy_byte_demand(),capacity_bytes:ledger.next_capacity_byte_demand(body)?,release_bytes:ledger.next_release_byte_demand()?,depth:ledger.next_depth_demand()?});}
  Ok(Default::default())
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture diff closure not started"));}let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.field.terminal_is_empty(){return self.field.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(messages)=self.messages.take(){*self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),messages));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  if let Some(ledger)=self.ledger.as_mut(){let step=ledger.close_step(grant)?;if ledger.terminal_is_empty(){*self.ledger=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn terminal_is_empty(&self)->bool{self.field.terminal_is_empty()&&self.diagnostic.terminal_is_empty()&&self.messages.is_none()&&self.ledger.is_none()}
}
impl Drop for Apply{fn drop(&mut self){let empty=<Self as RetainedDiffApply<Projection,Diff>>::terminal_is_empty(self);assert!(std::thread::panicking()||empty,"fixture diff keeps original diagnostics and typed fragments until closure");if empty{unsafe{ManuallyDrop::drop(&mut self.messages);ManuallyDrop::drop(&mut self.ledger);}}}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(op:impl FnOnce()->T)->(T,(usize,usize)){let(result,born,released)=crate::test_allocation::observe_backing(op);(result,(born,released))}
fn source<T:RetireOwned+Sync>(value:T)->RetainedCloneSource<T>{match RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}){Ok((source,_))=>source,Err((error,_,()))=>panic!("original diff source:{error}")}}
fn close_source<T:RetireOwned+Sync>(source:&mut RetainedCloneSource<T>){let mut turns=0;while !source.terminal_is_empty(){turns+=1;assert!(turns<1000,"diff source close stalled");let quote=grant(RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});source.close_step(quote).unwrap();}}
fn close(owner:&mut RetainedDiffOwner<Projection,Diff>){owner.begin_close();let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000);let quote=grant(owner.close_demand(4096).unwrap());let(step,heap)=observe(||owner.close_step(quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}}
fn messages(rows:Vec<MutationMessage>){let mut owner=MutationMessageLedgerRetirement::new(String::new(),rows);let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000,"original preflight message close stalled");let quote=grant(RetirementDemand{copy_bytes:owner.next_copy_byte_demand(),capacity_bytes:owner.next_capacity_byte_demand(4096).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()});owner.close_step(quote).unwrap();}}

/// 🧫️ Real typed diffs remain unchanged and serializable while only the central owner can commit admitted fragments.
#[test]
fn retained_central_diff_preserves_original_serialization_and_refuses_fatal_preflight(){let mut receiving_callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1<<24,&mut receiving_callback);
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🔺️diff/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus.as_array().unwrap(){
  let original_diff=Diff{set:case["diff"]["set"].as_i64().unwrap(),refused:case["diff"]["refused"].as_bool().unwrap()};let mut diff=source(original_diff.clone());let mut before=source(Projection{value:case["before"].as_i64().unwrap(),untouched:vec![42;256]});let mut post=Projection{value:case["before"].as_i64().unwrap(),untouched:vec![42;256]};let pointer=post.untouched.as_ptr();
  let((mut owner,receipt),heap)=observe(||begin_retained_diff::<Projection,Diff>(grant(retained_diff_birth_demand::<Projection,Diff>())).unwrap());assert_eq!(heap,(receipt.retained_capacity_bytes,0));assert!(owner.commit_demand(diff.borrow(),before.borrow(),&post,128).is_err());
  let mut turns=0;let admitted=loop{turns+=1;assert!(turns<1000,"typed diff preflight stalled case={:?} phase={}",case["id"],owner.apply.as_ref().unwrap().phase);let quote=grant(owner.preflight_demand(diff.borrow(),before.borrow(),&post,128).unwrap());let(step,heap)=observe(||preflight_retained_diff(&mut owner,diff.borrow(),before.borrow(),&post,128,RetainedCloneGrant::default(),&mut receiving).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());let(step,heap)=observe(||preflight_retained_diff(&mut owner,diff.borrow(),before.borrow(),&post,128,quote,&mut receiving).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_eq!(post.value,case["before"].as_i64().unwrap());match step{RetainedDiffPreflightStep::Progress(_)=>{},RetainedDiffPreflightStep::Admitted(_,rows)=>{messages(rows);break true;},RetainedDiffPreflightStep::Refused(_,rows)=>{assert_eq!(rows.len(),1);assert_eq!(rows[0].level,semio_framework_diagnostic::Severity::Fatal);messages(rows);break false;}}};
  assert_eq!(admitted,case["admitted"].as_bool().unwrap());
  if admitted{let quote=grant(owner.commit_demand(diff.borrow(),before.borrow(),&post,128).unwrap());let(step,heap)=observe(||commit_retained_diff(&mut owner,diff.borrow(),before.borrow(),&mut post,RetainedCloneGrant::default()).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());assert_eq!(post.value,case["before"].as_i64().unwrap());let(step,heap)=observe(||commit_retained_diff(&mut owner,diff.borrow(),before.borrow(),&mut post,quote).unwrap());assert!(step.progress().fits(quote));assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(heap,(0,0));}else{assert!(commit_retained_diff(&mut owner,diff.borrow(),before.borrow(),&mut post,RetainedCloneGrant::default()).is_err());}
  assert_eq!(post.value,case["expected"].as_i64().unwrap());assert_eq!(post.untouched.as_ptr(),pointer);assert_eq!(diff.borrow().get(),&original_diff);assert_eq!(semio_framework_value::ToValue::to_value(diff.borrow().get()),semio_framework_value::ToValue::to_value(&original_diff));
  close(&mut owner);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));close_source(&mut diff);close_source(&mut before);
  eprintln!("[DEBUG] central typed diff {:?}: readonly preflight, fatal override, original D serialization stable, staged scalar owned commit, terminalDrop0",case["id"]);
 }
}

/// 🛑️ Every original typed diff plan frontier closes before its genuine D/P sources retire.
#[test]
fn retained_central_diff_cancellation_settles_staged_fields_and_original_sources(){let mut receiving_callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1<<24,&mut receiving_callback);
 for refused in [false,true]{for cancel in 0..28{
  let mut diff=source(Diff{set:2,refused});let mut before=source(Projection{value:1,untouched:vec![]});let mut post=Projection{value:1,untouched:vec![]};let(mut owner,_)=begin_retained_diff::<Projection,Diff>(grant(retained_diff_birth_demand::<Projection,Diff>())).unwrap();let mut admitted=false;
  for _ in 0..cancel{if admitted{let quote=grant(owner.commit_demand(diff.borrow(),before.borrow(),&post,128).unwrap());commit_retained_diff(&mut owner,diff.borrow(),before.borrow(),&mut post,quote).unwrap();break;}let quote=grant(owner.preflight_demand(diff.borrow(),before.borrow(),&post,128).unwrap());match preflight_retained_diff(&mut owner,diff.borrow(),before.borrow(),&post,128,quote,&mut receiving).unwrap(){RetainedDiffPreflightStep::Progress(_)=>{},RetainedDiffPreflightStep::Admitted(_,rows)=>{messages(rows);admitted=true;},RetainedDiffPreflightStep::Refused(_,rows)=>{messages(rows);break;}}}
  close(&mut owner);close_source(&mut diff);close_source(&mut before);
 }}
}
