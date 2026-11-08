//! 🧪️ Actual source and authority bank shared by the paged owner laws.
use semio_framework_value_paged_text::{self as value,ErasedSnapshotRetirement};
use value::{OriginalConstructionRecipient,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,ValueError};
use value::paged_text::PagedText;
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_retirement::retirement::RetireOwned;
pub(crate) struct Work;
impl OriginalWorkRecipient for Work{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<value::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,check:value::OriginalWorkPartialCheck<'reply>)->value::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,check:value::OriginalWorkPartialCheck<'reply>)->value::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(self.validate_context(context))}
 fn admits<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'a>,_:u64,_:u64)->OriginalWorkQuoteComplete<'a>where 'source:'turn{quote.complete(Ok(true))}
}
pub(crate) struct Physical{pub(crate) interrupt:bool,pub(crate) submitted:RetainedCloneProgress}
impl OriginalConstructionRecipient for Physical{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<value::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{let d=quote.demand();let reply=quote.complete(Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}));if self.interrupt{self.interrupt=false;std::hint::black_box(&reply);panic!("original construction grant interrupted after recording its reply")}reply}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let result=self.submitted.checked_add(receipt.progress()).map(|next|self.submitted=next);receipt.complete(result)}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
}
pub(crate) fn grant(d:value::RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}

use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
pub(crate) fn source<T:RetireOwned+Sync>(owner:T)->FixtureSource<T>{RetainedCloneSource::from_owner(owner)}
pub(crate) fn close_source<T:RetireOwned+Sync>(mut source:FixtureSource<T>){for _ in 0..100000{if source.terminal_is_empty(){break}let step=source.close_once().unwrap();assert!(!matches!(step,semio_framework_value_clone_source_testing::FixtureCloseStep::Denied|semio_framework_value_clone_source_testing::FixtureCloseStep::Blocked));}assert!(source.terminal_is_empty())}
enum CloseTarget<'a,'b,const N:usize>{Text(&'a mut PagedText<N>),Decode(&'a mut value::PagedTextDecodeFrame<'b,N>)}
impl<const N:usize> CloseTarget<'_,'_,N>{
 fn empty(&self)->bool{match self{Self::Text(v)=>v.terminal_is_empty(),Self::Decode(v)=>v.terminal_is_empty()}}
 fn demand(&self)->value::RetirementDemand{match self{Self::Text(v)=>v.next_demand(4096).unwrap(),Self::Decode(v)=>v.close_demand(4096).unwrap()}}
 fn work(&self)->u64{match self{Self::Text(v)=>v.next_computational_work(4096).unwrap(),Self::Decode(v)=>v.close_work_demand(4096).unwrap()}}
}
struct CloseOriginal<const N:usize>;
impl<const N:usize> value::OriginalEffectProducer for CloseOriginal<N>{
 type Output=value::RetainedCloneStep;type Failure=value::OriginalRetirementFailure;type Context<'a,'b>=CloseTarget<'a,'b,N> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<value::OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:value::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<value::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{match c{CloseTarget::Text(v)=>ErasedSnapshotRetirement::close_into(*v,4096,e,r),CloseTarget::Decode(v)=>v.close_into(4096,e,r)}}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:value::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<value::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{match c{CloseTarget::Text(v)=>ErasedSnapshotRetirement::recover_close_into(*v,4096,e,r),CloseTarget::Decode(v)=>v.recover_close_into(4096,e,r)}}
}
fn close_bank()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1000000,maximum_copy_bytes:16*1024*1024,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:64},1000000))}
fn close_target<const N:usize>(target:CloseTarget<'_,'_,N>)->usize{close_target_observed(target,&mut |_,_|{})}
fn close_target_observed<const N:usize>(mut target:CloseTarget<'_,'_,N>,observe:&mut impl FnMut(RetainedCloneProgress,u128))->usize{
 let mut bank=close_bank();let mut authority=Some(value::OriginalEffectAuthority::new(CloseOriginal::<N>));let mut released=0;let mut born=0;for _ in 0..100000{if target.empty(){break}let d=target.demand();let g=grant(d);let units=target.work();let(mut action,mut reply)=(None,None);let mut observed=(0,0);let mut spent=0;
 for _ in 0..10000{let((poll,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(g,units,|effect|{let boundary=effect.performed_boundary();let ready=authority.as_mut().unwrap().drive(&mut target,effect,&mut action).unwrap();let work=effect.performed_since(boundary).unwrap();if ready{value::RetainedWorkPoll::Complete(value::RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{value::RetainedWorkPoll::Pending{work}}}));observed.0+=heap.0;observed.1+=heap.1;spent+=poll.work();if let Some(actual)=actual{assert_eq!(actual,observed.0)}if matches!(poll,value::RetainedWorkPoll::Complete(_)){assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.as_mut().unwrap().finish(&mut action).unwrap();break}}
 let progress=reply.expect("original text close reply").unwrap().progress();assert_eq!(bank.take_admitted(),if spent==0{None}else{Some(true)});bank.post(progress,g,observed);observe(progress,spent);released+=progress.released_bytes;born+=progress.retained_capacity_bytes;}assert!(target.empty());let mut producer=None;let d=value::OriginalEffectAuthority::<CloseOriginal<N>>::producer_transfer_demand().unwrap();let g=grant(d);let((poll,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(g,1,|effect|value::RetainedWorkPoll::Complete(value::OriginalEffectAuthority::transfer_producer(&mut authority,&target,&mut producer,effect))));let value::RetainedWorkPoll::Complete(turn)=poll else{panic!("text close producer transfer pending")};let progress=turn.result.unwrap().unwrap();if let Some(actual)=actual{assert_eq!(actual,heap.0)}assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,g,heap);let d=value::RetirementDemand{copy_bytes:std::mem::size_of::<Option<CloseOriginal<N>>>(),depth:1,..Default::default()};let g=grant(d);let((poll,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(g,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());drop(producer.take());value::RetainedWorkPoll::Complete(value::RetainedWorkOutcome{result:Ok::<_,ValueError>(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:effect.performed_work()-before})}));let value::RetainedWorkPoll::Complete(turn)=poll else{panic!("text close producer clear pending")};if let Some(actual)=actual{assert_eq!(actual,heap.0)}assert_eq!(bank.take_admitted(),Some(true));bank.post(turn.result.unwrap(),g,heap);assert!(authority.is_none()&&producer.is_none());released.checked_sub(born).expect("original close retained unpaid backing")
}
pub(crate) fn close_text<const N:usize>(mut text:PagedText<N>)->usize{close_target(CloseTarget::Text(&mut text))}
/// 🧵️ Drives the actual decoder frame and original authorities to paid publication.
pub(crate) fn decode_success<T:RetireOwned+Sync+value::paged_text::TextReadSource,const N:usize>(original:&RetainedCloneSource<T>,control:&mut value::NativeDecodeControl<'_>,physical:&mut value::OriginalConstructionAuthority<Physical>,work:&mut value::OriginalWorkAuthority<Work>)->PagedText<N>{
 let mut output=None;let mut frame=value::paged_text::PagedTextDecodeFrame::<N>::new(&mut output).unwrap();let mut completed=false;
 for _ in 0..100000{let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();match turn.result{Ok(value::paged_text::PagedTextReadStep::Complete(_))=>{completed=true;break},Ok(value::paged_text::PagedTextReadStep::Pending|value::paged_text::PagedTextReadStep::Progress(_))=>{},_=>panic!("fully funded original decoder refused")}}assert!(completed);
 frame.begin_close();close_target(CloseTarget::Decode(&mut frame));assert!(frame.terminal_is_empty());drop(frame);output.expect("original decoder paid publication")
}

struct FailureClose<T:RetireOwned>{owner:std::mem::ManuallyDrop<Option<T>>,close:std::mem::ManuallyDrop<semio_framework_value_clone_source::RetainedCloneClose>}
impl<T:RetireOwned> FailureClose<T>{fn empty(&self)->bool{self.owner.is_none()&&self.close.is_empty()}}
impl<T:RetireOwned> Drop for FailureClose<T>{fn drop(&mut self){let empty=self.empty();assert!(empty||std::thread::panicking(),"failure cleanup retains its original owner");if empty{unsafe{std::mem::ManuallyDrop::drop(&mut self.owner);std::mem::ManuallyDrop::drop(&mut self.close)}}}}
impl<T:RetireOwned> value::OriginalEffectProducer for FailureClose<T>{
 type Output=value::RetainedCloneStep;type Failure=value::OriginalRetirementFailure;type Context<'a,'b>=() where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&())->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&())->Option<value::OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,_:&mut (),e:&mut RetainedAllocationEffect<'_>,r:value::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<value::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{if self.owner.is_some(){let turn=self.close.begin_granted(&mut *self.owner,e);return Ok(r.complete(turn.result.map(|step|step.unwrap_or(value::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}self.close.step_into(4096,e,r)}
 fn recover<'r,'a,'b>(&mut self,_:&mut (),e:&mut RetainedAllocationEffect<'_>,r:value::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<value::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{self.close.recover_step_into(4096,e,r)}
}
pub(crate) fn close_owned_failure<T:RetireOwned>(failure:T){
 let mut bank=close_bank();let mut authority=value::OriginalEffectAuthority::new(FailureClose{owner:std::mem::ManuallyDrop::new(Some(failure)),close:std::mem::ManuallyDrop::new(Default::default())});
 for _ in 0..100000{if authority.inspect(FailureClose::empty){break}let(d,units)=authority.inspect(|p|(p.close.next_owner_demands::<T>(p.owner.is_some(),&None,4096).unwrap(),p.close.next_owner_work::<T>(p.owner.is_some(),&None,4096).unwrap()));let g=grant(d);let(mut action,mut reply)=(None,None);let mut observed=(0,0);let mut spent=0;
 for _ in 0..10000{let((poll,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(g,units,|effect|{let boundary=effect.performed_boundary();let ready=authority.drive(&mut (),effect,&mut action).unwrap();let work=effect.performed_since(boundary).unwrap();if ready{value::RetainedWorkPoll::Complete(value::RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{value::RetainedWorkPoll::Pending{work}}}));observed.0+=heap.0;observed.1+=heap.1;spent+=poll.work();if let Some(actual)=actual{assert_eq!(actual,observed.0)}if matches!(poll,value::RetainedWorkPoll::Complete(_)){assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.finish(&mut action).unwrap();break}}
 let progress=reply.expect("original failure close reply").unwrap().progress();assert_eq!(bank.take_admitted(),if spent==0{None}else{Some(true)});bank.post(progress,g,observed);}assert!(authority.inspect(FailureClose::empty));assert!(authority.into_producer().is_ok());
}

pub(crate) fn close_encode_failure(failure:value::paged_text::PagedTextReservationFailure){close_owned_failure(failure)}

/// 🎟️ Advances the actual bound decoder once with the same original authority loans.
pub(crate) fn decode_turn<T:RetireOwned+Sync+value::paged_text::TextReadSource,const N:usize>(frame:&mut value::paged_text::PagedTextDecodeFrame<'_,N>,original:&RetainedCloneSource<T>,control:&mut value::NativeDecodeControl<'_>,physical:&mut value::OriginalConstructionAuthority<Physical>,work:&mut value::OriginalWorkAuthority<Work>)->value::RetainedWorkOutcome<value::paged_text::PagedTextReadStep,value::native_decoding::NativeReceivingEffectFailure<ValueError>>{
 let g=grant(frame.demand(original.borrow()).unwrap());physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap()
}
/// 🧹️ Retires the same bound decoder frame and reconciles its actual cleanup births.
pub(crate) fn close_decode_frame<const N:usize>(frame:&mut value::paged_text::PagedTextDecodeFrame<'_,N>)->usize{frame.begin_close();close_target(CloseTarget::Decode(frame))}


#[path="encoder-rig.rs"]
pub(crate) mod encoder;
pub(crate) use encoder::{EncodeBank,Stock,encode_success,encode_inline_success};

/// 🔬️ Observes only settled native close receipts after their original bank posts.
pub(crate) fn close_decode_frame_observed<const N:usize>(frame:&mut value::paged_text::PagedTextDecodeFrame<'_,N>,mut observe:impl FnMut(RetainedCloneProgress,u128))->usize{frame.begin_close();close_target_observed(CloseTarget::Decode(frame),&mut observe)}
