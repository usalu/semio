//! 🧪️ Scoped test decoding retains the real recipient through independently funded cleanup.
use semio_framework_value::{ErasedSnapshotRetirement as _,RetainedCloneGrant,retained_clone::RetainedCloneProgress};
use semio_framework_value::{RetainedWorkPoll,ValueError};
use semio_framework_value_clone_source_testing::drive::drive;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeProgress,NativeDecodeRetirementRecipient};
use semio_framework_value_testing_admission::finite_work;

/// 🧾️ Original identity and actual cleanup effects of a completed test scope.
#[derive(Clone,Copy,Debug)]
pub struct DecodeScopeReceipt{pub ledger:semio_framework_value::retained_clone::NativeAllocationIdentity,pub recipient:u64,pub semantic_owned_bytes:usize,pub retirement_steps:usize,pub retirement:RetainedCloneProgress}

struct Scope<'a>{control:Option<NativeDecodeControl<'a>>,grant:RetainedCloneGrant,receipt:DecodeScopeReceipt}
impl Scope<'_>{
 fn close(&mut self){
  let Some(control)=self.control.take()else{return};
  assert_eq!(control.ledger_identity(),self.receipt.ledger,"test operation replaced its original decoder");
  assert_eq!(control.retirement_recipient_identity(),Some(self.receipt.recipient),"test operation replaced its original recipient");
  self.receipt.semantic_owned_bytes=control.owned_bytes();
  let mut original=control.pause_with_retirement().expect("original scoped decoder continuation");
  let mut cleanup=|_|true;
  let body=self.grant.maximum_copy_bytes;
  loop{
   let mut quote=finite_work(0,None);
   let Some((birth,units))=quote.with_loan(&mut (),|work|original.retire::<_,ValueError>(self.grant,&mut cleanup,work,|control|{if !control.has_retirement_owner(){return Ok(None)}let birth=control.requires_original_birth();Ok(Some((birth,if birth{u64::MAX}else{control.retirement_work_demand(body)?})))})).expect("original decoder closure quote").expect("original decoder closure quote")else{break};
   let mut authority=finite_work(units,None);
   let step=authority.with_loan(&mut (),|work|original.retire::<_,ValueError>(self.grant,&mut cleanup,work,|control|{
    assert_eq!(control.ledger_identity(),self.receipt.ledger);
    assert_eq!(control.retirement_recipient_identity(),Some(self.receipt.recipient));
    let grant=self.grant;
    let poll=if birth{control.close_original_birth_step(grant)}else{let turn=drive(grant,units,|effect|control.close_retirement_recipient(body,effect));RetainedWorkPoll::Complete(turn)};
    let RetainedWorkPoll::Complete(turn)=poll else{panic!("original decoder closure stayed pending")};
    turn.result.map_err(|failure|failure.into_value_error())
   })).expect("original decoder closure work loan").expect("funded original decoder closure");
   assert!(!authority.has_live_actions());
   let complete=matches!(step,semio_framework_value::RetainedCloneStep::Complete(_));
   if complete&&step.progress()==Default::default(){break}
   assert!(step.progress().fits(self.grant),"original closure receipt exceeds grant");
   assert!(step.progress()!=Default::default()||complete,"decoder closure grant made no progress: {step:?} under {:?} for {units} units",self.grant);
   self.receipt.retirement=self.receipt.retirement.checked_add(step.progress()).expect("original closure receipt overflow");
   self.receipt.retirement_steps+=1;
   if complete{break}
  }
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
