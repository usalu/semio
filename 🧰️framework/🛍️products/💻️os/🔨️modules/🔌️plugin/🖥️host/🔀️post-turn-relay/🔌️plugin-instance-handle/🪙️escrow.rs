//! 🪙️ Retains the Cold relay's actual issuer History through guest and native handback.
use super::*;
use crate::{attest_bootstrap_header_return,attest_header_return,with_bootstrap_history_parts,with_frozen_history_parts};
use semio_framework_job::{OriginalNativeRecipientIdentity,OriginalNativeRetiredWitness};
use semio_plugin_host_wake::{GuestIssuedHistoryHome,GuestRelayAdmissionCall};

#[derive(Default,PartialEq,Eq)]
enum Capture{#[default] Select,BootstrapAccept,Journal,JournalAccept}

pub(super) struct ColdEscrow{
 producer:OriginalNativeRecipientIdentity,
 history:Option<OriginalNativeLease<GuestIssuedHistoryHome>>,
 history_id:Option<OriginalNativeRecipientIdentity>,
 witness:Option<OriginalNativeRetiredWitness>,
 capture:Capture,
 journal:GuestFrozenHistoryCapture,
 reserved:bool,
 issued:bool,
 return_witnessed:bool,
 producer_closed:bool,
 reconciled:bool,
}

impl ColdEscrow{
 pub(super) fn new(producer:OriginalNativeRecipientIdentity)->Self{
  Self{producer,history:None,history_id:None,witness:None,capture:Capture::default(),journal:GuestFrozenHistoryCapture::default(),reserved:false,issued:false,return_witnessed:false,producer_closed:false,reconciled:false}
 }
 fn history(&mut self)->Result<&mut OriginalNativeLease<GuestIssuedHistoryHome>,ValueError>{self.history.as_mut().ok_or_else(||refused("cold job lost its admitted issuer history"))}
 pub(super) fn loan_step(&mut self,driver:&GuestRelayWakeDriver,producer:&mut OriginalNativeLease<GuestRelayNativeHome>,purpose:GuestRelayGuestPurpose,header:usize,cleanup:bool)->Result<Option<GuestRelayGuestSubloan>,GuestRelayAdmissionFailure>{
  if self.history.is_none(){
   if cleanup{return Err(refused("cleanup cannot create its missing cold job issuer history").into())}
   driver.dispatch_admission_step(false,|_,cx|{
    let result=cx.admit_original_native(&mut Some(GuestIssuedHistoryHome::empty()),&mut self.history).map(|_|());
    GuestRelayAdmissionCall{result,execution:None}
   })?;
   self.history_id=self.history.as_ref().map(OriginalNativeLease::recipient_identity);
   return Ok(None)
  }
  if !self.reserved{
   let history=self.history()?;
   let mut ready=false;
   driver.dispatch_operation_step(false,cleanup,|cx|{history.with_native_borrow(cx,|history,cx|{ready=GuestIssuedHistoryHome::reserve_step(history,cx)?;Ok(())})?;Ok(())})?;
   self.reserved=ready;
   return Ok(None)
  }
  let loan=match driver.pending_guest_subloan(producer)?{
   Some(loan)=>Some(loan),
   None if cleanup=>driver.admit_cleanup_guest_subloan(producer,header)?,
   None=>driver.admit_guest_subloan(producer,purpose,header)?,
  };
  let Some(loan)=loan else{return Ok(None)};
  if loan.purpose()!=purpose{return Err(refused("cold job changed the purpose of its reserved issuer loan").into())}
  if !self.issued{
   let mut ready=false;
   let history=self.history()?;
   driver.dispatch_operation_step(false,cleanup,|cx|{history.with_native_pair_borrow(producer,cx,|history,producer,cx|{ready=GuestIssuedHistoryHome::commit_issued_step(history,&producer,loan,cx)?;Ok(())})?;Ok(())})?;
   self.issued=ready;
   return Ok(None)
  }
  Ok(Some(loan))
 }
 pub(super) fn began_loan(&mut self){self.reserved=false;self.issued=false;}

 pub(super) fn capture_step(&mut self,driver:&GuestRelayWakeDriver,producer:&mut OriginalNativeLease<GuestRelayNativeHome>,frontier:FrozenReceiptFrontier,loan:GuestRelayGuestSubloan,cleanup:bool)->Result<(),GuestRelayAdmissionFailure>{
  let mut ready=false;
  match self.capture{
   Capture::Select if matches!(frontier,FrozenReceiptFrontier::Scalar)=>{
    let mut selected=None;
    let history=self.history()?;
    driver.dispatch_escrow_reconciliation_step(cleanup,|cx,mut issuer|{
     selected=with_bootstrap_history_parts(history,producer,cx,|history,frozen,_,cx|{ready=issuer.apply_bootstrap(history,&frozen,loan,cx)?;Ok(())})?;
     Ok(())
    })?;
    if selected==Some(false){self.capture=Capture::Journal}else if ready{self.capture=Capture::BootstrapAccept}
   },
   Capture::Select=>self.capture=Capture::Journal,
   Capture::BootstrapAccept=>{
    let history=self.history()?;
    driver.dispatch_operation_step(false,cleanup,|cx|{
     with_bootstrap_history_parts(history,producer,cx,|history,frozen,state,cx|{
      let controller=state.controller.as_mut().ok_or_else(||refused("bootstrap lost its cold job controller"))?;
      GuestIssuedHistoryHome::with_bootstrap_call_step(history,&frozen,cx,|call,identity,cx|{ready=controller.accept_bootstrap_step(call,identity,cx)?;Ok(())})?;
      Ok(())
     })?;
     Ok(())
    })?;
    if ready{self.capture=Capture::Select}
   },
   Capture::Journal=>{
    let history=self.history.as_mut().ok_or_else(||refused("cold job lost its admitted issuer history"))?;
    if self.journal.advance(driver,history,producer,frontier,loan,cleanup)?{self.capture=Capture::JournalAccept}
   },
   Capture::JournalAccept=>{
    let history=self.history()?;
    driver.dispatch_operation_step(false,cleanup,|cx|{
     with_frozen_history_parts(history,producer,frontier,cx,|history,frozen,state,cx|{
      let controller=state.controller.as_mut().ok_or_else(||refused("journal lost its cold job controller"))?;
      GuestIssuedHistoryHome::with_journaled_receipt_step(history,&frozen,cx,|view,cx|{
       ready=controller.accept_journaled_step(&view,frozen.header_len(),cx)?;
       Ok(())
      })?;
      Ok(())
     })?;
     Ok(())
    })?;
    if ready{self.capture=Capture::Select;self.journal=GuestFrozenHistoryCapture::default();}
   },
  }
  Ok(())
 }

 pub(super) fn handback_step(&mut self,driver:&GuestRelayWakeDriver,producer:&mut OriginalNativeLease<GuestRelayNativeHome>,header:ColdHeader,scalar:bool,cleanup:bool)->Result<bool,GuestRelayAdmissionFailure>{
  let mut witnessed=self.return_witnessed;
  if !witnessed{driver.dispatch_operation_step(false,cleanup,|cx|{
   producer.with_native_borrow(cx,|mut producer,cx|{
    producer.value_mut().with_native(cx,|_,_,_,state,_|{
     let controller=state.controller.as_ref().ok_or_else(||refused("handback lost its cold job controller"))?;
     witnessed=controller.header_returned.is_some()||controller.bootstrap_returned.is_some();
     Ok(())
    })?;
    if !witnessed{
     witnessed=match header{
      ColdHeader::Bootstrap(identity)=>attest_bootstrap_header_return(producer,identity,cx,|state|&mut state.controller.as_mut().unwrap().bootstrap_returned)?,
      ColdHeader::Journal(identity)=>attest_header_return(producer,scalar,identity,cx,|state|&mut state.controller.as_mut().unwrap().header_returned)?,
     };
    }
    Ok(())
   })?;
   Ok(())
  })?;}
  if !witnessed{return Ok(false)}
  self.return_witnessed=true;
  let history=self.history()?;
  let mut finished=false;
  driver.dispatch_operation_step(false,cleanup,|cx|{
   history.with_native_pair_borrow(producer,cx,|history,mut producer,cx|{
    producer.value_mut().with_native(cx,|_,_,_,state,cx|{
     let controller=state.controller.as_mut().ok_or_else(||refused("handback lost its cold job controller"))?;
     let ready=match header{
      ColdHeader::Bootstrap(_)=>GuestIssuedHistoryHome::receive_bootstrap_header_return_step(history,&mut controller.bootstrap_returned,cx)?,
      ColdHeader::Journal(_)=>GuestIssuedHistoryHome::receive_header_return_step(history,&mut controller.header_returned,cx)?,
     };
     if ready{finished=controller.finish_header_return_step(cx)?;}
     Ok(())
    })?;
    Ok(())
   })?;
   Ok(())
  })?;
  if finished{self.return_witnessed=false;}
  Ok(true)
 }

 pub(super) fn retire_step(&mut self,driver:&GuestRelayWakeDriver)->Result<bool,GuestRelayAdmissionFailure>{
  let Some(history)=self.history_id else{return Ok(true)};
  if self.history.is_some(){
   driver.dispatch_operation_step(false,true,|cx|{OriginalNativeLease::return_slot_step(&mut self.history,cx)?;Ok(())})?;
   return Ok(false)
  }
  if !self.producer_closed{
   let producer=self.producer;
   let mut closed=false;
   driver.dispatch_operation_step(true,true,|cx|{closed=cx.close_original_native_recipient_step(producer)?;Ok(())})?;
   self.producer_closed=closed;
   return Ok(false)
  }
  if self.witness.is_none(){
   let producer=self.producer;
   driver.dispatch_return_birth_close_step(|cx|{
    if let Some(witness)=cx.original_native_retired_witness(producer)?{
     if pay(cx,size_of::<Option<OriginalNativeRetiredWitness>>(),true)?{self.witness=Some(witness)}
    }
    Ok(())
   })?;
   return Ok(false)
  }
  if !self.reconciled{
   let mut reconciled=false;
   driver.dispatch_escrow_reconciliation_step(true,|cx,mut issuer|{
   cx.with_original_native_root_step::<GuestIssuedHistoryHome>(history,|home,cx|{
    reconciled=issuer.retire_closed_history_step(home,self.producer,self.witness.as_ref().unwrap(),cx)?;
    Ok(())
   })?;
   Ok(())
   })?;
   self.reconciled=reconciled;
   return Ok(false)
  }
  let mut closed=false;
  driver.dispatch_operation_step(false,true,|cx|{closed=cx.close_original_native_recipient_step(history)?;Ok(())})?;
  if closed{self.history_id=None;self.witness=None;}
  Ok(closed)
 }
}
