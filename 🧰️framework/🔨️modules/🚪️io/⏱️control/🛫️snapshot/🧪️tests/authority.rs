//! 🧭️ Original Work and construction producers lent to a decoder under finite fixture stock.
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value_control_contract::*;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
pub struct Work(pub bool);
impl OriginalWorkRecipient for Work{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(self.0&&observed.checked_add(additional).is_some_and(|total|total<=1_000_000)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
pub struct Physical{pub grant:RetainedCloneGrant,pub submitted:RetainedCloneProgress,pub deny_quote:bool,pub deny_post:bool}
impl Physical{pub fn new(grant:RetainedCloneGrant)->Self{Self{grant,submitted:Default::default(),deny_quote:false,deny_post:false}}}
impl OriginalConstructionRecipient for Physical{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{let mut grant=self.grant;if self.deny_quote{grant.maximum_release_bytes=0}quote.complete(Ok(grant))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let progress=receipt.progress();assert_eq!(receipt.grant(),self.grant);let result=self.submitted.checked_add(progress).map(|total|self.submitted=total).and_then(|_|if self.deny_post{Err(ValueError::literal(ValueRefusalKind::Canceled,"release receipt refused"))}else{Ok(())});receipt.complete(result)}
}
/// 🧭️ Runs one decoder operation with both original authorities lent and returns them with their final stock.
pub fn with_decode<R>(native:&mut NativeDecodeControl<'_>,work:Work,physical:Physical,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->R)->(R,Physical){
 warm();
 let mut work=OriginalWorkAuthority::new(work);let mut physical=OriginalConstructionAuthority::new(physical);
 let output=physical.with_loan(&mut (),|physical_loan|work.with_loan(&mut (),|work_loan|native.with_work_recipient(work_loan,|native|native.with_construction_recipient(physical_loan,|native|Ok::<_,ValueError>(operation(native))))))
  .unwrap().unwrap().unwrap();
 (output,physical.into_recipient().ok().unwrap())
}
use semio_framework_value::{RetainedWorkOutcome,RetainedCloneStep,ErasedSnapshotRetirement,OriginalRetirementFailure,RetainedAllocationEffect,RetainedWorkPoll,NativeEncodeControl,retirement::{RetireOwned,controlled::ControlledRetirement}};
#[path="../../../../🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]
pub mod driver;
pub type Step=Result<RetainedCloneStep,OriginalRetirementFailure>;
/// 🧭️ Runs one operation under the original invocation of its funded effect.
pub fn invoked<R>(effect:&mut RetainedAllocationEffect<'_>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{let mut operation=Some(operation);let mut output=None;with_original_invocation(None,effect,&mut |effect|output=Some((operation.take().expect("fixture invocation ran once"))(effect))).unwrap();output.expect("fixture invocation ran")}
pub fn settle<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("fixture turn stayed pending")}}
/// 🏦️ Funds one synchronous turn and reports its physical allocations.
pub fn funded<T,E>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<T,E>)->(RetainedWorkOutcome<T,E>,usize,usize){warm();let(turn,_,heap)=driver::funded(&mut driver::bank(),grant,1<<20,|effect|invoked(effect,operation));(turn,heap.0,heap.1)}
pub fn erased_close(owner:&mut dyn ErasedSnapshotRetirement,grant:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,usize,usize){funded(grant,|effect|{let body=effect.grant().maximum_copy_bytes;settle(owner.close_step(body,effect))})}
pub fn controlled_step<T:RetireOwned>(owner:&mut ControlledRetirement<T>,grant:RetainedCloneGrant)->(Step,usize,usize){let(turn,born,released)=funded(grant,|effect|{let body=effect.grant().maximum_copy_bytes;settle(owner.step(body,effect))});(turn.result,born,released)}
pub fn decode_close(native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->(Step,usize,usize){with_decode(native,Work(true),Physical::new(grant),|native|decode_close_installed(native,grant)).0}
/// ♻️ Closes the decoder recipient while its original authorities are already installed.
pub fn decode_close_installed(native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->(Step,usize,usize){
 let(turn,born,released)=semio_framework_trace_memory_testing::observe_backing(||{
  let birth=native.with_retirement::<_,ValueError>(grant,|loan|Ok(if loan.requires_original_birth(){Some(loan.close_original_birth_step(grant))}else{None})).ok().flatten();
  match birth{
   Some(poll)=>Ok(Some(poll)),
   None=>native.with_effect(grant,1<<16,|native,effect|invoked(effect,|effect|->RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>{let body=effect.grant().maximum_copy_bytes;native.close_retirement_recipient(body,effect)})),
  }
 });
 match turn{Ok(Some(RetainedWorkPoll::Complete(turn)))=>(turn.result,born,released),_=>(Ok(RetainedCloneStep::Progress(Default::default())),born,released)}
}
pub fn encode_close(native:&mut NativeEncodeControl<'_>,grant:RetainedCloneGrant)->(Step,usize,usize){let(turn,born,released)=funded(grant,|effect|{let body=effect.grant().maximum_copy_bytes;let mut slot=None;{let reply=OriginalEffectReply::borrow(&mut slot).unwrap();drop(native.close_retirement_recipient_into(body,effect,reply).unwrap());}RetainedWorkOutcome{result:slot.expect("fixture close recorded its reply"),work:effect.performed_work()}});(turn.result,born,released)}
/// 🧭️ Runs one encoder operation with both original authorities lent and returns the physical stock.
pub fn with_encode<R>(native:&mut NativeEncodeControl<'_>,work:Work,physical:Physical,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->R)->(R,Physical){
 warm();
 let mut work=OriginalWorkAuthority::new(work);let mut physical=OriginalConstructionAuthority::new(physical);
 let output=physical.with_loan(&mut (),|physical_loan|work.with_loan(&mut (),|work_loan|native.with_work_recipient(work_loan,|native|native.with_construction_recipient(physical_loan,|native|Ok::<_,ValueError>(operation(native))))))
  .unwrap().unwrap().unwrap();
 (output,physical.into_recipient().ok().unwrap())
}
/// 🧭️ Runs one operation over both natives with every original authority and retirement recipient installed.
pub fn with_natives<R>(decode_observer:&mut dyn FnMut(semio_framework_value_native_owned::native_decoding::NativeDecodeProgress)->bool,encode_observer:&mut dyn FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool,grant:RetainedCloneGrant,operation:impl for<'b> FnOnce(&mut NativeDecodeControl<'b>,&mut NativeEncodeControl<'b>)->R)->R{
 warm();
 let mut decode_work=OriginalWorkAuthority::new(Work(true));let mut decode_physical=OriginalConstructionAuthority::new(Physical::new(grant));let mut encode_work=OriginalWorkAuthority::new(Work(true));let mut encode_physical=OriginalConstructionAuthority::new(Physical::new(grant));
 let mut decode_recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut encode_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
 decode_work.with_loan(&mut (),|dw|decode_physical.with_loan(&mut (),|dp|encode_work.with_loan(&mut (),|ew|encode_physical.with_loan(&mut (),|ep|{
  let mut dw=dw.reborrow();let mut dp=dp.reborrow();let mut ew=ew.reborrow();let mut ep=ep.reborrow();
  let mut decode=NativeDecodeControl::new(1048576,&mut *decode_observer);let mut encode=NativeEncodeControl::new(1048576,&mut *encode_observer);
  decode.install_retirement_recipient(&mut decode_recipient).unwrap();encode.install_retirement_recipient(&mut encode_recipient).unwrap();
  decode.install_work_recipient(&mut dw).unwrap();decode.install_construction_recipient(&mut dp).unwrap();encode.install_work_recipient(&mut ew).unwrap();encode.install_construction_recipient(&mut ep).unwrap();
  operation(&mut decode,&mut encode)
 })))).unwrap().unwrap().unwrap().unwrap()
}
thread_local!{static WARM:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};}
/// 🔥️ Initializes this thread's one-time invocation bookkeeping outside every heap observation.
pub fn warm(){if WARM.replace(true){return}let _=driver::funded(&mut driver::bank(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:64,maximum_release_bytes:64,maximum_depth:4},4,|effect|invoked(effect,|_|RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work:0}));}
