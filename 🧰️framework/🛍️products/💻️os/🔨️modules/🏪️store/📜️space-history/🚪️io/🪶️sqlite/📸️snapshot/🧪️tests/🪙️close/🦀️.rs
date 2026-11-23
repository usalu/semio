//! 🪙️ Original retirement close turns of the encoder and decoder owners, each funded by one explicit authoring wallet.
use semio_framework_value::retained_clone::OriginalEffectReply;
use semio_framework_value::{AuthoringWallet,NativeEncodeControl,OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

pub(crate) type Close=RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>;

fn idle()->Close{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
fn paid(grant:RetainedCloneGrant,close:impl FnOnce(usize,&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>)->Close{
 if grant.maximum_items==0||grant.maximum_depth==0{return idle()}
 let body=grant.maximum_copy_bytes;
 match AuthoringWallet::new(grant,1<<20).run_invoked(grant,1<<10,|effect|close(body,effect)).expect("authoring close effect").expect("close work quote admitted").output{
  RetainedWorkPoll::Complete(turn)=>turn,
  RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},
 }
}

/// ♻️ Closes one turn of the encoder's selected retirement recipient into a local reply slot.
pub(crate) fn close_encode(native:&mut NativeEncodeControl<'_>,grant:RetainedCloneGrant)->Close{
 paid(grant,|body,effect|{let mut slot=None;{let reply=OriginalEffectReply::borrow(&mut slot).unwrap();drop(native.close_retirement_recipient_into(body,effect,reply).unwrap());}RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.expect("encode close recorded its reply"),work:effect.performed_work()})})
}

/// ♻️ Closes one turn of the decoder's selected retirement recipient.
pub(crate) fn close_decode(native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Close{
 if grant.maximum_items!=0&&grant.maximum_depth!=0&&native.with_retirement::<_,semio_framework_value::ValueError>(grant,|original|Ok(original.requires_original_birth())).unwrap_or(false){
  return match native.with_retirement::<_,semio_framework_value::ValueError>(grant,|original|Ok(original.close_original_birth_step(grant))).expect("original birth close"){
   RetainedWorkPoll::Complete(turn)=>turn,
   RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},
  }
 }
 paid(grant,|body,effect|native.close_retirement_recipient(body,effect))
}

/// 🧹️ Retires one controlled owner through funded effects until its terminal is empty.
pub(crate) fn retire<T:semio_framework_value::retirement::RetireOwned>(value:T,grant:RetainedCloneGrant){
 let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).map_err(|(error,_)|error).expect("fixture owner supports typed retirement");
 let mut wallet=AuthoringWallet::new(grant,1<<24);
 for _ in 0..100_000{
  if owner.terminal_is_empty(){return}
  let(turn,_)=wallet.run_settled(grant,1<<10,|effect|owner.step(grant.maximum_copy_bytes,effect)).expect("authoring retire effect").expect("retire work quote admitted");
  turn.result.expect("fixture retire step refused");
 }
 panic!("fixture retire exhausted its turns")
}

/// ♻️ One funded original close turn on either owner direction, so shared laws close without naming the direction.
pub(crate) trait CloseOriginal{fn close_original(&mut self,grant:RetainedCloneGrant)->Close;}
impl CloseOriginal for NativeEncodeControl<'_>{fn close_original(&mut self,grant:RetainedCloneGrant)->Close{close_encode(self,grant)}}
impl CloseOriginal for NativeDecodeControl<'_>{fn close_original(&mut self,grant:RetainedCloneGrant)->Close{close_decode(self,grant)}}

struct EventPort<F>(F);
impl<F:FnMut(semio_framework_value_native_owned::native_decoding::NativeAllocationEvent)->Result<(),semio_framework_value_native_owned::native_decoding::NativeAllocationRefusal>> semio_framework_value_native_owned::native_decoding::NativeAllocationAuthorityPort for EventPort<F>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),semio_framework_value::ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:semio_framework_value_native_owned::native_decoding::NativeAllocationEvent,reply:semio_framework_value_native_owned::native_decoding::NativeAllocationReply<'reply>)->Result<semio_framework_value_native_owned::native_decoding::NativeAllocationReplyComplete<'reply>,semio_framework_value::ValueError> where 'source:'turn{Ok(reply.complete((self.0)(event)))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:semio_framework_value_native_owned::native_decoding::NativeAllocationEvent,reply:semio_framework_value_native_owned::native_decoding::NativeAllocationReply<'reply>)->Result<semio_framework_value_native_owned::native_decoding::NativeAllocationReplyComplete<'reply>,semio_framework_value::ValueError> where 'source:'turn{Ok(reply.pending())}
}
/// 🛂️ Lends one Send allocation loan of an original port that answers every event through the recording closure.
pub(crate) fn with_port<R>(port:impl FnMut(semio_framework_value_native_owned::native_decoding::NativeAllocationEvent)->Result<(),semio_framework_value_native_owned::native_decoding::NativeAllocationRefusal>+Send,operation:impl FnOnce(&mut dyn semio_framework_value_native_owned::native_decoding::NativeSendAllocationLoan)->R)->R{
 let mut authority=semio_framework_value_native_owned::native_decoding::NativeAllocationAuthority::new(EventPort(port));
 let result=authority.with_send_loan(&mut (),operation).expect("fixture allocation port validates");
 assert!(authority.into_port().is_ok(),"fixture allocation port settles every transaction");
 result
}

/// 🪙️ Work authority of one fixture stock that admits every quote whose cumulative total fits.
pub(crate) struct WorkStock(pub u64);
impl semio_framework_value_native_owned::native_decoding::OriginalWorkRecipient for WorkStock{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),semio_framework_value::ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:semio_framework_value_native_owned::native_decoding::OriginalWorkQuote<'reply>,observed:u64,additional:u64)->semio_framework_value_native_owned::native_decoding::OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
}

/// 🪙️ Runs one closure inside one original effect funded by a fresh authoring wallet and reports the work its receipt recorded.
pub(crate) fn funded_work<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,u128){
 let done=AuthoringWallet::new(grant,1<<24).run(grant,1<<12,operation).expect("authoring effect").expect("work quote admitted");
 (done.output,done.performed)
}
/// 🪙️ [`funded_work`] without the recorded work.
pub(crate) fn funded<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{funded_work(grant,operation).0}
/// 🧵️ [`funded`] for retirement turns whose nested closes are reply-shaped and need the original invocation.
pub(crate) fn funded_invoked<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 AuthoringWallet::new(grant,1<<24).run_invoked(grant,1<<12,operation).expect("authoring effect").expect("work quote admitted").output
}

/// 🏗️ Construction authority of one fixture stock that grants its whole grant to every quote and accepts every receipt.
pub(crate) struct ConstructionStock{pub grant:RetainedCloneGrant,pub submitted:semio_framework_value::RetainedCloneProgress}
impl ConstructionStock{pub(crate) fn new(grant:RetainedCloneGrant)->Self{Self{grant,submitted:Default::default()}}}
impl semio_framework_value_native_owned::native_decoding::OriginalConstructionRecipient for ConstructionStock{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),semio_framework_value::ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:semio_framework_value_native_owned::native_decoding::OriginalConstructionGrant<'a>)->semio_framework_value_native_owned::native_decoding::OriginalConstructionGrantComplete<'a> where 'source:'turn{quote.complete(Ok(self.grant))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:semio_framework_value_native_owned::native_decoding::OriginalConstructionAccept<'a>)->semio_framework_value_native_owned::native_decoding::OriginalConstructionAcceptComplete<'a> where 'source:'turn{let progress=receipt.progress();let posted=self.submitted.checked_add(progress).map(|total|self.submitted=total);receipt.complete(posted)}
 fn submitted<'turn,'source>(&self,_:&())->semio_framework_value::RetainedCloneProgress where 'source:'turn{self.submitted}
}

/// 🧊️ A copyable cell that original Send authorities may share with the caller's assertions.
#[derive(Default)]
pub(crate) struct Cell<T>(std::sync::Mutex<T>);
impl<T:Copy> Cell<T>{
 pub(crate) fn new(value:T)->Self{Self(std::sync::Mutex::new(value))}
 pub(crate) fn get(&self)->T{*self.0.lock().unwrap()}
 pub(crate) fn set(&self,value:T){*self.0.lock().unwrap()=value}
 pub(crate) fn replace(&self,value:T)->T{std::mem::replace(&mut *self.0.lock().unwrap(),value)}
}
