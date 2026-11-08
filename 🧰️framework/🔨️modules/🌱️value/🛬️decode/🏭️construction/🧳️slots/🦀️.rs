//! 🔐️ Shared receiving keeps its source, output and callback frame in one movable owner.
use semio_framework_value_native_decoding::{NativeDecodeControl,NativeSharedAdmission,OriginalReceivingPrimary};
use semio_framework_value_control_contract::{OriginalRetirementCloseLoan,RetainedAllocationEffect,RetainedWorkPoll,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,retained_clone_grant_admits};
use semio_framework_value_retained_clone::retained_clone::RetainedCloneClose;
use semio_framework_value_retirement::retirement::RetireOwned;
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
use std::mem::{ManuallyDrop,size_of};

/// 🧳️ One movable owner carries the actual input, output and original pending admission together.
pub struct NativeSharedSlots<T:RetireOwned+Send+Sync+'static,R:RetireOwned+Send+Sync+'static>{input:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<R>>,admission:NativeSharedAdmission<T>,retirement:RetainedCloneClose}
impl<T:RetireOwned+Send+Sync+'static,R:RetireOwned+Send+Sync+'static> Default for NativeSharedSlots<T,R>{fn default()->Self{Self{input:ManuallyDrop::new(None),output:ManuallyDrop::new(None),admission:Default::default(),retirement:Default::default()}}}
impl<T:RetireOwned+Send+Sync+'static,R:RetireOwned+Send+Sync+'static> NativeSharedSlots<T,R>{
 /// 📏️ Quotes the funded transfer of the original input into this owner.
 pub fn capture_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<T>>(),depth:1,..Default::default()}}
 /// ⏱️ Quotes the actual original input-slot transfer.
 pub fn capture_work_demand(&self,source:&Option<T>)->Result<u64,ValueError>{if self.input.is_some()||self.output.is_some()||!self.admission.terminal_is_empty()||!self.retirement.is_empty()||source.is_none(){return Err(ValueError::invariant("shared slots require their empty original capture"))}Ok(1)}
 /// 📥️ Captures the input before any shared birth can be quoted.
 pub fn capture(&mut self,source:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let grant=effect.grant();let demand=Self::capture_demand();if self.input.is_some()||self.output.is_some()||!self.admission.terminal_is_empty()||!self.retirement.is_empty()||source.is_none(){return RetainedWorkOutcome{result:Err(ValueError::invariant("shared slots require their empty original capture").into()),work:0}}if !retained_clone_grant_admits(grant,demand)||effect.maximum_work_units()==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original shared-slot work exceeded its authority")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:1}}*self.input=source.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}}
 /// 📤️ Quotes publication of the retained result to its caller.
 pub fn take_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<R>>(),depth:1,..Default::default()}}
 /// ⏱️ Quotes the actual original output-slot transfer.
 pub fn take_work_demand(&self,target:&Option<R>)->Result<u64,ValueError>{if target.is_some()||self.output.is_none()||!self.admission.terminal_is_empty(){return Err(ValueError::invariant("shared slots require their original completed output"))}Ok(1)}
 /// 🪣️ Transfers the completed output only on its own funded turn.
 pub fn take(&mut self,target:&mut Option<R>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let grant=effect.grant();let demand=Self::take_demand();if target.is_some()||self.output.is_none()||!self.admission.terminal_is_empty(){return RetainedWorkOutcome{result:Err(ValueError::invariant("shared slots require their original completed output").into()),work:0}}if !retained_clone_grant_admits(grant,demand)||effect.maximum_work_units()==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original shared-slot work exceeded its authority")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:1}}*target=self.output.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}}
 /// 👁️ Observes the original source without transferring its custody.
 pub fn input(&self)->Option<&T>{self.input.as_ref()}
 /// 🪪️ Observes the retained original admission without transferring it.
 pub fn admission_ref(&self)->&NativeSharedAdmission<T>{&self.admission}
 /// 👁️ Observes whether the original output is ready for funded transfer.
 pub fn output_ready(&self)->bool{self.output.is_some()&&self.admission.terminal_is_empty()}
 /// 🪪️ Borrows the retained callback frame for original recovery.
 pub fn has_pending_recovery(&self)->bool{self.admission.has_pending_recovery()}
 pub fn resume_recovery(&mut self,control:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{if self.has_pending_recovery(){self.admission.resume(control)}else{semio_framework_value_native_decoding::no_pending_receiving_recovery()}}
 pub fn admission(&mut self)->&mut NativeSharedAdmission<T>{&mut self.admission}
 /// 📏️ Quotes the next retained owned close turn.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.admission.terminal_is_empty(){return self.admission.close_demand()}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.input.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}self.retirement.next_owner_demands::<R>(self.output.is_some(),&None,body)}
 /// ⛽️ Quotes the defining work of the next retained owned close turn.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.admission.terminal_is_empty(){self.admission.close_demand()?;return Ok(u64::from(self.admission.has_birth()))}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.input.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}self.retirement.next_owner_work::<R>(self.output.is_some(),&None,body)}
 /// ♻️ Retires the captured original values and any unpublished birth with funded work.
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{use RetainedWorkPoll::Complete;let grant=effect.grant();if !self.admission.terminal_is_empty(){return self.admission.close_step(&mut effect.nested(grant))}if !self.retirement.is_empty(){return self.retirement.step_granted(effect)}if self.input.is_some(){let turn=self.retirement.begin_granted(&mut self.input,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
 /// 🪹️ Observes whether every original slot and owned close is empty.
 pub fn requires_original_birth(&self)->bool{self.admission.terminal_is_empty()&&self.retirement.requires_original_birth()}
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{self.retirement.step_original_birth(original,grant)}
 pub fn requires_original_control(&self)->bool{self.admission.terminal_is_empty()&&self.retirement.requires_original_control()}
 pub fn close_original_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{if self.requires_original_birth(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth first").into()),work:0})}if self.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
 pub fn terminal_is_empty(&self)->bool{self.input.is_none()&&self.output.is_none()&&self.admission.terminal_is_empty()&&self.retirement.is_empty()}
}
impl<T:RetireOwned+Send+Sync+'static> NativeSharedSlots<T,SealedShared<T>>{
 /// ⛽️ Quotes work only for the actual original sealed input and empty destination.
 pub fn advance_work_demand(&self)->Result<u64,ValueError>{if self.input.is_none()||self.output.is_some(){return Err(ValueError::invariant("sealed slots require original input and empty output"))}self.admission.shared_work_demand()}
 /// 🧷️ Advances the original sealed owner without lending its slots for replacement.
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,issuer:SharedIssuer<T>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{control.receive_shared(&mut self.input,&mut self.output,issuer,&mut self.admission)}
 /// 📏️ Quotes the next physical sealed birth or publication.
 pub fn demand(&self)->RetirementDemand{self.admission.shared_demand()}
}
impl<T:RetireOwned+Send+Sync+'static> NativeSharedSlots<T,std::sync::Arc<T>>{
 /// ⛽️ Quotes work only for the actual original Arc input and empty destination.
 pub fn advance_work_demand(&self)->Result<u64,ValueError>{if self.input.is_none()||self.output.is_some(){return Err(ValueError::invariant("Arc slots require original input and empty output"))}self.admission.arc_work_demand()}
 /// 🖼️ Advances the original Arc owner without lending its slots for replacement.
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{control.receive_arc(&mut self.input,&mut self.output,&mut self.admission)}
 /// 📏️ Quotes the next physical Arc birth or publication.
 pub fn demand(&self)->RetirementDemand{self.admission.arc_demand()}
}
impl<T:RetireOwned+Send+Sync+'static,R:RetireOwned+Send+Sync+'static> Drop for NativeSharedSlots<T,R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"shared slots require original funded closure")}}

#[cfg(test)]
mod tests{
 use super::*;use semio_framework_value_testing_admission::{BankWallet,EffectBank,ReceiveFrame,WorkLoan,close_owned,Wallet};
 fn stock()->serde_json::Value{serde_json::json!({"items":1u64<<20,"copyBytes":1u64<<20,"capacityBytes":1u64<<20,"releaseBytes":1u64<<20,"depth":64u64,"fuel":1u64<<20})}
 fn settled<T>(poll:RetainedWorkPoll<T,ValueError>)->RetainedWorkOutcome<T,ValueError>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous slot action stayed pending")}}
 /// 🧪️ The shared four-driver effect corpus: the decode control settles every case exactly as the job provider, the authoring wallet and the encode control do.
 #[test]fn the_shared_corpus_settles_identically_through_the_decode_control(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧵️job/⏱️context/🧪️tests/🪙️effect-provider/🧫️fixtures/🔣️.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:8};
  for case in corpus["cases"].as_array().unwrap(){let maximum=case["maximumWork"].as_u64().unwrap();let steps:Vec<u64>=case["performs"].as_array().unwrap().iter().map(|step|step.as_u64().unwrap()).collect();let mut callback=|_|true;let mut control=NativeDecodeControl::new(4096,&mut callback);let remaining=1u128<<20;let mut frame=ReceiveFrame::new(grant,WorkLoan::new(&remaining,1<<20));let mut seen=None;
   let turn=frame.turn(&mut control,|control|{let outcome=control.with_effect(grant,maximum,|_,effect|{let all=steps.iter().all(|step|effect.performed(*step).unwrap());(all,effect.performed_work())});RetainedWorkOutcome{result:outcome.map(|done|{seen=done}),work:0}});
   assert!(turn.result.is_ok(),"{}",case["name"]);let (all,performed)=seen.expect("an admitted quote runs");assert!(all,"{}",case["name"]);assert_eq!(performed,u128::from(case["performed"].as_u64().unwrap()),"{}",case["name"]);assert!(performed<=u128::from(maximum));
  }}
 /// 🔐️ Relocation retains the captured original payload and rejects replacement.
 #[test]fn native_shared_slots_keep_original_across_relocation_and_reject_replacement(){let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🏭️construction/🧪️tests/🧫️fixtures/effect-frame.json")).unwrap();let mut original=Some(fixture["slotCustody"][0]["owner"].as_str().unwrap().to_owned());let pointer=original.as_ref().unwrap().as_ptr();let mut replacement=Some(fixture["slotCustody"][1]["candidate"].as_str().unwrap().to_owned());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:8};let mut slots=NativeSharedSlots::<String,SealedShared<String>>::default();let mut bank=EffectBank::new(BankWallet::from_profile(&stock()));
  let quote=slots.capture_work_demand(&original).unwrap();let(capture,_)=bank.run(grant,quote,|effect|RetainedWorkPoll::Complete(slots.capture(&mut original,effect)));assert!(settled(capture).result.is_ok());assert_ne!(bank.take_admitted(),Some(false));
  assert!(slots.capture_work_demand(&replacement).is_err());let(reject,_)=bank.run(grant,1,|effect|RetainedWorkPoll::Complete(slots.capture(&mut replacement,effect)));assert!(settled(reject).result.is_err());assert!(replacement.is_some());
  let mut slots=Box::new(slots);assert_eq!(slots.input().unwrap().as_ptr(),pointer);let mut callback=|_|true;let mut control=NativeDecodeControl::new(4096,&mut callback);let remaining=1u128<<20;let mut frame=ReceiveFrame::new(grant,WorkLoan::new(&remaining,1<<20));for _ in 0..2{let turn=frame.turn(&mut control,|control|slots.advance(control,SharedIssuer::owned()));assert!(turn.result.is_ok())}assert!(slots.output_ready());
  let mut output=None;let quote=slots.take_work_demand(&output).unwrap();let(take,_)=bank.run(grant,quote,|effect|RetainedWorkPoll::Complete(slots.take(&mut output,effect)));assert!(settled(take).result.is_ok());assert!(slots.terminal_is_empty());assert_eq!(output.as_ref().unwrap().get().as_ptr(),pointer);
  let mut wallet=Wallet::from_profile(&stock());close_owned(output.take().unwrap(),&mut wallet,4096,10000);close_owned(replacement.take().unwrap(),&mut wallet,4096,10000);}
}
