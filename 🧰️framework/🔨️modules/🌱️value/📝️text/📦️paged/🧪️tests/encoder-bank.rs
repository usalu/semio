//! 🏦️ Paged encoder laws retain one genuine Source and finite wallet through every phase.
use semio_framework_value_paged_text as value;
use value::*;
use semio_framework_value_native_encoding::{self as native,*};
use native::native_receiving::span::*;
use std::sync::{Arc,Mutex};
#[path="encoder-rig.rs"]
mod rig;
use rig::*;
#[test]
fn original_encoder_bank_debits_only_its_same_source_receipts(){
 let law:serde_json::Value=serde_json::from_str(include_str!("encoder-bank.json")).unwrap();
 let stock=Stock{grant:RetainedCloneGrant{maximum_items:law["items"].as_u64().unwrap()as usize,maximum_copy_bytes:law["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:law["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:law["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:law["depth"].as_u64().unwrap()as usize},work:u128::from(law["work"].as_u64().unwrap()),submitted:Default::default(),partials:0,posts:0};
 let mut bank=EncodeBank::new(stock,false);
 bank.run(4096,8,|turn,control|{let mut payment=Payment::default();let demand=RetirementDemand{copy_bytes:law["performedCopy"].as_u64().unwrap()as usize,depth:1,..Default::default()};payment.begin(control,demand,1)?;payment.stage(control,|_,_,effect|{assert_eq!(effect.work_source_turn(),Some(turn.identity()));assert!(effect.performed(1)?);Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})?;assert!(payment.post(control)?);payment.finish(control)?;Ok(())}).unwrap();
 let left=bank.finish();assert_eq!(left.work,u128::from(law["remainingWork"].as_u64().unwrap()));assert_eq!(left.grant.maximum_copy_bytes,law["remainingCopy"].as_u64().unwrap()as usize);assert_eq!(left.grant.maximum_items,law["remainingItems"].as_u64().unwrap()as usize);assert_eq!(left.partials,1);assert_eq!(left.posts,1);
}

struct OutputClose<'a,const N:usize>{source:OriginalSourceTurnIdentity,text:&'a mut value::paged_text::PagedText<N>}
impl<const N:usize> OriginalEffectProducer for OutputClose<'_,N>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=OriginalSourceTurnWitness<'a> where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if self.source==c.identity(){Ok(())}else{Err(ValueError::invariant("paged output close Source changed"))}}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.identity())}
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{value::ErasedSnapshotRetirement::close_into(self.text,4096,e,r)}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{value::ErasedSnapshotRetirement::recover_close_into(self.text,4096,e,r)}
}
#[test]
fn original_encoder_frame_returns_its_real_loan_before_producer_cleanup(){
 use semio_framework_value_clone_source_testing::FixtureSourceBirth;
 use value::paged_text::{PagedTextEncodeProducer,PagedTextEncodeFrame,PagedTextEncodeOperation,PagedTextLoanReturn};
 let law:serde_json::Value=serde_json::from_str(include_str!("encoder-bank.json")).unwrap();for sample in law["samples"].as_array().unwrap(){assert_eq!(law["prefix"],"Set label to ");assert_eq!(law["suffix"],"");let original=semio_framework_value_clone_source::RetainedCloneSource::from_owner(sample["body"].as_str().unwrap().to_owned());
 let stock=Stock{grant:RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64},work:10000,submitted:Default::default(),partials:0,posts:0};let mut bank=EncodeBank::new(stock,false);let mut output=None;
 let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(65536,64,|turn,control|{
  output=Some(encode_success::<_,1024>(&original,control,turn,1024,"Set label to ","")?);
  assert!(output.as_ref().unwrap().borrow().unwrap().bytes().eq(sample["expected"].as_str().unwrap().bytes()));let mut closer=Some(OriginalEffectAuthority::new(OutputClose{source:turn.identity(),text:output.as_mut().unwrap()}));
  for _ in 0..1000{if closer.as_ref().unwrap().inspect(|p|p.text.terminal_is_empty()){break}let(d,maximum)=closer.as_ref().unwrap().inspect(|p|Ok::<_,ValueError>((p.text.next_demand(4096)?,p.text.next_computational_work(4096)?)))?;let mut pay=Payment::default();pay.begin(control,d,maximum)?;let mut action=None;let mut reply=None;pay.stage(control,|_,_,effect|{assert!(closer.as_mut().unwrap().drive(&mut turn.witness(),effect,&mut action)?);assert!(action.as_mut().unwrap().transfer_reply(&mut reply)?);closer.as_mut().unwrap().finish(&mut action)?;Ok(Some(reply.take().unwrap().unwrap().progress()))})?;assert!(pay.post(control)?);pay.finish(control)?}
  assert!(closer.as_ref().unwrap().inspect(|p|p.text.terminal_is_empty()));let mut holder=None;let d=OriginalEffectAuthority::<OutputClose<'_,1024>>::producer_transfer_demand()?;let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|OriginalEffectAuthority::transfer_producer(&mut closer,&turn.witness(),&mut holder,effect).result)?;assert!(pay.post(control)?);pay.finish(control)?;
  let d=RetirementDemand{copy_bytes:std::mem::size_of_val(&holder),depth:1,..Default::default()};let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);drop(holder.take());Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;
  drop(holder);drop(closer);let d=RetirementDemand{copy_bytes:std::mem::size_of_val(&output),depth:1,..Default::default()};let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);drop(output.take());Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;Ok(())
 }));result.unwrap();let remaining=bank.finish();assert_eq!(heap,(remaining.submitted.retained_capacity_bytes,remaining.submitted.released_bytes));assert_eq!(heap.0,heap.1);assert!(remaining.work<stock.work);assert!(output.is_none());
 let mut original=original;for _ in 0..10000{if original.terminal_is_empty(){break}original.close_once().unwrap();}assert!(original.terminal_is_empty());
}
}

#[test]
fn original_encoder_bank_reuses_only_its_remaining_work(){
 let stock=Stock{grant:RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:8},work:3,submitted:Default::default(),partials:0,posts:0};let mut bank=EncodeBank::new(stock,false);
 bank.run(4096,8,|_,control|{for maximum in [1,2]{let mut pay=Payment::default();let d=RetirementDemand{copy_bytes:1,depth:1,..Default::default()};pay.begin(control,d,maximum)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;}Ok(())}).unwrap();let left=bank.finish();assert_eq!(left.work,1);assert_eq!(left.submitted.copied_bytes,2);assert_eq!(left.partials,2);assert_eq!(left.posts,2);
}

#[test]
fn original_encoder_plan_uses_real_native_reserve_bounds(){
 let law:serde_json::Value=serde_json::from_str(include_str!("encoder-bank.json")).unwrap();for row in law["budgets"].as_array().unwrap(){if row["bits"].as_u64().unwrap()!=u64::from(usize::BITS){continue}let bytes=row["bytes"].as_u64().unwrap()as usize;let reserve=value::list::PagedList::<u8,{isize::MAX as usize}>::empty().reserve_capacity_one_work_demand(bytes).unwrap();assert_eq!(reserve,row["reserve"].as_u64().unwrap());let stock=Stock::for_outputs::<{isize::MAX as usize}>(&[(bytes,row["bind"].as_u64().unwrap())],65536,64).unwrap();assert_eq!(stock.work,u128::from(row["work"].as_u64().unwrap()));}
}
