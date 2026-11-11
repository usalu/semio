//! 🧪️ Scoped test decoding retains the real recipient through independently funded cleanup.
use semio_framework_value::{ErasedSnapshotRetirement as _,RetainedCloneGrant,retained_clone::RetainedCloneProgress};
use semio_framework_value::{RetainedWorkPoll,ValueError};
use semio_framework_value_control_contract::{OriginalWork,OriginalWorkSink,RetainedAllocationEffect};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeProgress,NativeDecodeRetirementRecipient,OriginalSourceTurnIdentity,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient};

/// 🧾️ Original identity and actual cleanup effects of a completed test scope.
#[derive(Clone,Copy,Debug)]
pub struct DecodeScopeReceipt{pub ledger:semio_framework_value::retained_clone::NativeAllocationIdentity,pub recipient:u64,pub semantic_owned_bytes:usize,pub retirement_steps:usize,pub retirement:RetainedCloneProgress}

const CLOSE_WORK:u64=1<<20;
struct CloseWork;
impl OriginalWorkRecipient for CloseWork{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=CLOSE_WORK)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original Work effect settled against the scope's own authority.
fn with_effect<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 let mut authority=OriginalWorkAuthority::new(CloseWork);
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(CLOSE_WORK,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let mut actual=None;let output={let mut effect=RetainedAllocationEffect::new(grant,CLOSE_WORK,&mut actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().unwrap();assert_eq!(partial.total(),performed);assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}output}).unwrap();
 assert!(!authority.has_live_actions());result
}
struct Scope<'a>{control:Option<NativeDecodeControl<'a>>,grant:RetainedCloneGrant,receipt:DecodeScopeReceipt}
impl Scope<'_>{
 fn close(&mut self){
  let Some(control)=self.control.take()else{return};
  assert_eq!(control.ledger_identity(),self.receipt.ledger,"test operation replaced its original decoder");
  assert_eq!(control.retirement_recipient_identity(),Some(self.receipt.recipient),"test operation replaced its original recipient");
  self.receipt.semantic_owned_bytes=control.owned_bytes();
  let mut original=control.pause_with_retirement().expect("original scoped decoder continuation");
  let mut cleanup=|_|true;
  let mut authority=OriginalWorkAuthority::new(CloseWork);
  authority.with_loan(&mut (),|work|loop{
   let step=original.retire::<_,ValueError>(self.grant,&mut cleanup,work,|control|{
    assert_eq!(control.ledger_identity(),self.receipt.ledger);
    assert_eq!(control.retirement_recipient_identity(),Some(self.receipt.recipient));
    let grant=self.grant;
    let poll=if control.requires_original_birth(){control.close_original_birth_step(grant)}else{with_effect(grant,|effect|control.close_retirement_recipient(grant.maximum_copy_bytes,effect))};
    let RetainedWorkPoll::Complete(turn)=poll else{panic!("original decoder closure stayed pending")};
    turn.result.map_err(|failure|failure.into_value_error())
   }).expect("funded original decoder closure");
   let complete=matches!(step,semio_framework_value::RetainedCloneStep::Complete(_));
   if complete&&step.progress()==Default::default(){break}
   assert!(self.receipt.retirement_steps<1_000_000,"funded decoder closure stalled");
   assert!(step.progress().fits(self.grant),"original closure receipt exceeds grant");
   assert!(step.progress()!=Default::default()||matches!(step,semio_framework_value::RetainedCloneStep::Complete(_)),"decoder closure grant made no progress");
   self.receipt.retirement=self.receipt.retirement.checked_add(step.progress()).expect("original closure receipt overflow");
   self.receipt.retirement_steps+=1;
   if complete{break}
  }).expect("original decoder closure work loan");
  let(detached,recipient)=original.detach();
  assert_eq!(detached.identity(),(self.receipt.ledger,self.receipt.recipient));
  assert!(recipient.terminal_is_empty(),"original recipient kept ownership or a reserved slot");
 }
}
impl Drop for Scope<'_>{fn drop(&mut self){self.close();}}

/// 🛬️ Runs one unchanged decoder hop and preserves unwinding while its real recipient completes paid cleanup.
pub fn with_decoding<T>(maximum_bytes:usize,grant:RetainedCloneGrant,observer:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->T)->(T,DecodeScopeReceipt){
 let mut recipient=NativeDecodeRetirementRecipient::new();
 let mut control=NativeDecodeControl::new_retained(observer);
 control.admit_turn_capacity(grant.maximum_capacity_bytes).expect("original parent allocation grant");
 control.install_retirement_recipient(&mut recipient).expect("actual empty decoder recipient");
 let receipt=DecodeScopeReceipt{ledger:control.ledger_identity(),recipient:control.retirement_recipient_identity().expect("original recipient identity"),semantic_owned_bytes:0,retirement_steps:0,retirement:Default::default()};
 let mut scope=Scope{control:Some(control),grant,receipt};
 let output=scope.control.as_mut().unwrap().scoped_maximum(maximum_bytes,|control|Ok::<T,semio_framework_value::ValueError>(operation(control))).expect("original semantic allocation scope");
 scope.close();
 let receipt=scope.receipt;
 drop(scope);
 (output,receipt)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
