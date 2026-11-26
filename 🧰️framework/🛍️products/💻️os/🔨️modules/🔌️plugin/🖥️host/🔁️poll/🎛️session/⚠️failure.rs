//! ⚠️ The failure service receives the actual failed Poll before releasing its producer for replay.
use super::*;
use super::super::actor_failure_flow::failure_fields;
use semio_plugin_host_actor_failure::{ActorFailureFetch, ActorFailureFetchStatus, ActorFailureRecipient};
use std::mem::{size_of,size_of_val};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FailureStep {
 Bind, Plan, Reserve, Issue, Commit, Budget, Execute, Reconcile, ReadJournal,
 TakeHeader, Deallocate, Attest, ApplyHeader, ReturnHeader, DrainDeallocate, Prepare, CopyPage,
 Seal, ClearCall, FinishSeal, Deliver, Accept, Confirm, CloseFetch, CloseHome, Resume,
}

impl<T:NativeReceive<DslValue>+RetireOwned+Send+Sync+'static,R:GuestPollFailureReceiving> GuestPollSession<T,R>{
 fn failure_native(&mut self,execution:bool,mut run:impl FnMut(&GuestRuntimes,&mut GuestInstance,&mut GuestPoll<T,R>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<(),GuestRelayAdmissionFailure>{
  let producer=self.producer.as_mut().ok_or_else(||invalid("failure service lost its actual native producer"))?;
  self.driver.dispatch_operation_step(execution,false,|cx|cx.with_original_retirement(|cx|{
   producer.with_native(cx,|home,cx|{home.with_native(cx,|runtime,instance,_,poll,cx|run(runtime,instance,poll,cx))?;Ok(())})?;
   Ok(())
  }))?;
  Ok(())
 }

 pub(super) fn failure_service_step(&mut self)->Result<bool,GuestRelayAdmissionFailure>{
  use FailureStep::*;
  let current=self.failure_step.ok_or_else(||invalid("failure service lacks its current frontier"))?;
  let mut next=current;
  match current{
   Bind=>{
    let fault=&mut *self.fault;
    let seed=&mut *self.fault_input;
    if fault.is_none(){
     self.driver.dispatch_operation_step(false,false,|cx|{
      if seed.is_none(){
       if !failure_fields(cx,&[size_of::<Option<semio_plugin_host_actor_failure::ActorFailureHome>>()])?{return Ok(())}
       *seed=Some(semio_plugin_host_actor_failure::ActorFailureHome::empty());
      }
      cx.admit_original_native(seed,fault)?;
      Ok(())
     })?;
     if let Some(fault)=fault.as_ref(){self.fault_id=Some(fault.recipient_identity());}
    }else{
     let producer=self.producer.as_mut().unwrap();
     self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
      producer.with_native_borrow(cx,|borrow,cx|{
       if ActorFailureFetch::bind_step(borrow,|home|{let(_,poll)=home.frozen_parts()?;Ok((&mut poll.failure_fetch,&mut poll.actor_failure))},fault,cx)?{next=Plan;}
       Ok(())
      })?;
      Ok(())
     }))?;
    }
   },
   Plan=>self.failure_native(false,|_,_,poll,cx|{
    match poll.failure_fetch.status(cx)?{
     ActorFailureFetchStatus::Action|ActorFailureFetchStatus::Page=>{
      if poll.select_failure_plan_step(cx)?{
       next=if poll.failure_plan.as_ref().ok_or_else(||invalid("failure plan was not retained"))?.scoped{Reserve}else{Execute};
      }
     },
     ActorFailureFetchStatus::PrepareFault=>next=Prepare,
     ActorFailureFetchStatus::SealFault=>next=Seal,
     ActorFailureFetchStatus::FinishSeal=>next=FinishSeal,
     ActorFailureFetchStatus::Deliver=>next=Deliver,
     ActorFailureFetchStatus::Recipient=>next=Accept,
     ActorFailureFetchStatus::Complete=>next=Confirm,
     _=>return Err(invalid("failure service has an unjournaled or closed plan")),
    }
    Ok(())
   })?,
   Reserve=>{
    let history=self.history.as_mut().unwrap();
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     history.with_native_borrow(cx,|borrow,cx|{if GuestIssuedHistoryHome::reserve_step(borrow,cx)?{next=Issue;}Ok(())})?;
     Ok(())
    }))?;
   },
   Issue=>{
    if let Some(loan)=self.driver.admit_guest_subloan(self.producer.as_mut().unwrap(),GuestRelayGuestPurpose::Infrastructure,184)?{
     self.guest=Some(loan);
     next=Commit;
    }
   },
   Commit=>{
    let history=self.history.as_mut().unwrap();
    let producer=self.producer.as_mut().unwrap();
    let loan=self.guest.unwrap();
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     history.with_native_pair_borrow(producer,cx,|history,producer,cx|{
      if GuestIssuedHistoryHome::commit_issued_step(history,&producer,loan,cx)?{next=Budget;}
      Ok(())
     })?;
     Ok(())
    }))?;
   },
   Budget=>{
    let loan=self.guest.unwrap();
    self.failure_native(false,|_,_,poll,cx|{
     if poll.receive_failure_budget_step(loan.budget(),cx)?{next=Execute;}
     Ok(())
    })?;
   },
   Execute|Deallocate=>self.failure_native(true,|runtime,instance,poll,cx|{
    if !poll.acknowledge_scalar_progress_step(cx)?||!poll.prepare_failure_step(runtime,instance,cx)?{return Ok(())}
    if current==Execute&&!poll.scalar_started{
     poll.begin_failure_scalar_step(runtime,instance,cx)?;
     return Ok(())
    }
    if current==Deallocate&&!poll.scalar_started{
     let pointer=OriginalCoreValue::I32(poll.failure_pointer as i32);
     let length=OriginalCoreValue::I32(poll.failure_length as i32);
     let address=(&mut poll.failure_born as *mut _)as usize;
     let size=size_of_val(&poll.failure_born);
     if !poll.failure_begin_place.admit_sized(cx,address,size)?||!poll.failure_begin_place.preissue_capture(cx,address,size)?{return Ok(())}
     poll.failure_born=Some(runtime.begin_original_scalar_step(instance,OwnedSemioExport::Deallocate,&[pointer,length],cx)?);
     return Ok(())
    }
    if current==Deallocate&&runtime.original_scalar_export(instance,cx)?==Some(OwnedSemioExport::Deallocate){
     next=Attest;
     return Ok(())
    }
    if poll.frozen{next=Reconcile;return Ok(())}
    if poll.failure_received.is_some(){
     if Self::receive_failure_bytes(poll,false,cx)?{next=CopyPage;}
     return Ok(())
    }
    if runtime.original_scalar_requires_execution(instance)?{
     let mut result=None;
     GuestRelayWakeDriver::execute_current_original(cx,|execution,cx|{
      result=Some(poll.drive_failure_scalar_step(runtime,instance,Some(execution),cx));
      poll.execution_receipt()
     })?;
     result.ok_or_else(||invalid("failure VM callback did not execute"))??;
    }else{poll.drive_failure_scalar_step(runtime,instance,None,cx)?;}
    Ok(())
   })?,
   Reconcile=>{
    let loan=self.guest.unwrap();
    if self.capture.advance(&self.driver,self.history.as_mut().unwrap(),self.producer.as_mut().unwrap(),FrozenReceiptFrontier::Scalar,loan,false)?{
     self.capture=GuestFrozenHistoryCapture::default();
     self.guest=None;
     next=ReadJournal;
    }
   },
   ReadJournal=>{
    let history=self.history.as_mut().unwrap();
    let producer=self.producer.as_mut().unwrap();
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     with_frozen_history_parts(history,producer,FrozenReceiptFrontier::Scalar,cx,|history,frozen,poll,cx|{
      if !poll.failure_fetch.capture_frozen_step(&frozen,cx)?{return Ok(())}
      GuestIssuedHistoryHome::with_journaled_receipt_step(history,&frozen,cx,|view,cx|{
       if !poll.failure_fetch.settle_journaled_step(&view,cx)?||!failure_fields(cx,&[size_of_val(&poll.journal_identity)])?{return Ok(())}
       poll.journal_identity=Some(view.identity());
       next=TakeHeader;
       Ok(())
      })?;
      Ok(())
     })?;
     Ok(())
    }))?;
   },
   TakeHeader=>self.failure_native(false,|runtime,instance,poll,cx|{
    if poll.failure_fetch.terminal_ack_ready()&&poll.failure_ack_frame.is_none()&&!runtime.capture_original_actor_failure_ack(instance,&mut poll.failure_ack_frame,cx)?{return Ok(())}
    if poll.failure_received.is_none(){runtime.take_original_scalar_into_step(instance,&mut poll.failure_received,cx)?;return Ok(())}
    if Self::receive_failure_bytes(poll,true,cx)?{next=Deallocate;}
    Ok(())
   })?,
   Attest=>{
    let producer=self.producer.as_mut().unwrap();
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     producer.with_native_borrow(cx,|mut producer,cx|{
      let (_,poll)=producer.value_mut().frozen_parts()?;
      let identity=poll.journal_identity.ok_or_else(||invalid("failure acknowledgement lost the captured identity"))?;
      if attest_header_return(producer,true,identity,cx,|poll|&mut poll.header_returned)?{next=ApplyHeader;}
      Ok(())
     })?;
     Ok(())
    }))?;
   },
   ApplyHeader=>{
    let producer=self.producer.as_mut().unwrap();
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     producer.with_native_borrow(cx,|producer,cx|{
      NativeInstanceHome::with_frozen_origin(producer,cx,|runtime,mut producer,cx|{
       let(instance,poll)=producer.value_mut().frozen_parts()?;
       if poll.failure_fetch.terminal_ack_ready()&&poll.failure_ack_host.is_none(){
        if poll.failure_ack_lower.is_none(){
         runtime.finish_original_actor_failure_ack(instance,&mut poll.failure_ack_frame,&mut poll.failure_ack_lower,cx)?;
        }else{
         runtime.seal_original_actor_failure_ack(producer,|home|{
          let(instance,poll)=home.frozen_parts()?;
          let reference=poll.failure_fetch.original_reference().ok_or_else(||invalid("failure ACK lost its saved reference"))?;
          let returned=poll.header_returned.as_ref().ok_or_else(||invalid("failure ACK lost its actual header return"))?;
          Ok((instance,reference,returned,&mut poll.failure_ack_lower,&mut poll.failure_ack_host))
         },cx)?;
        }
        return Ok(())
       }
       let returned=poll.header_returned.as_ref().ok_or_else(||invalid("failure response lost its physical header return"))?;
       if poll.failure_fetch.header_returned_step(returned,&mut poll.failure_ack_host,cx)?{next=ReturnHeader;}
       Ok(())
      })?;
      Ok(())
     })?;
     Ok(())
    }))?;
   },
   ReturnHeader=>{
    let history=self.history.as_mut().unwrap();
    let producer=self.producer.as_mut().unwrap();
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     history.with_native_pair_borrow(producer,cx,|history,mut producer,cx|{
      let(_,poll)=producer.value_mut().frozen_parts()?;
      if GuestIssuedHistoryHome::receive_header_return_step(history,&mut poll.header_returned,cx)?{next=DrainDeallocate;}
      Ok(())
     })?;
     Ok(())
    }))?;
   },
   DrainDeallocate=>self.failure_native(false,|runtime,instance,poll,cx|{
    if let Some(result)=poll.failure_received.as_ref(){
     if result.value.is_some(){return Err(invalid("failure deallocator returned a scalar value"))}
     if !failure_fields(cx,&[size_of_val(&poll.failure_received)])?{return Ok(())}
     poll.failure_received=None;
     next=ClearCall;
    }else{
     runtime.take_original_scalar_into_step(instance,&mut poll.failure_received,cx)?;
    }
    Ok(())
   })?,
   ClearCall=>self.failure_native(false,|_,_,poll,cx|{
    if !poll.finish_header_return_step(cx)?{return Ok(())}
    if Self::clear_failure_call(poll,cx)?{next=Plan;}
    Ok(())
   })?,
   other=>next=self.advance_failure_result_step(other)?,
  }
  if current==Resume{
   self.failure_step=None;
   self.fault_id=None;
   return Ok(true)
  }
  self.failure_step=Some(next);
  Ok(false)
 }

 fn receive_failure_bytes(poll:&mut GuestPoll<T,R>,releasing:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let result=poll.failure_received.as_ref().ok_or_else(||invalid("failure service lost the actual scalar result"))?;
  let Some(OriginalCoreValue::I64(word))=result.value else{return Err(invalid("failure service returned another scalar type"))};
  let(pointer,length)=match owned_result::decode(word as u64)?{
   OwnedReturn::Bytes{pointer,length}=>(pointer,length),
   OwnedReturn::Refusal(kind)=>return Err(ValueError::literal(kind,"failure service returned an unframed refusal").into()),
  };
  if !failure_fields(cx,&[size_of_val(&poll.failure_received),3*size_of::<bool>(),2*size_of::<u32>()])?{return Ok(false)}
  poll.failure_received=None;
  poll.failure_pointer=pointer;
  poll.failure_length=length;
  poll.scalar_started=false;
  poll.frozen=false;
  poll.failure_releasing=releasing;
  Ok(true)
 }

 fn clear_failure_call(poll:&mut GuestPoll<T,R>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !failure_fields(cx,&[size_of_val(&poll.failure_plan),size_of_val(&poll.failure_budget),2*size_of::<bool>(),2*size_of::<u32>()])?{return Ok(false)}
  poll.failure_plan=None;
  poll.failure_budget=None;
  poll.failure_releasing=false;
  poll.scalar_started=false;
  poll.failure_pointer=0;
  poll.failure_length=0;
  Ok(true)
 }

 fn advance_failure_result_step(&mut self,current:FailureStep)->Result<FailureStep,GuestRelayAdmissionFailure>{
  use FailureStep::*;
  let mut next=current;
  match current{
   Prepare|CopyPage|Seal=>{
    let producer=self.producer.as_mut().unwrap();
    let fault=&mut *self.fault;
    let bytes=&mut *self.actor_fault_bytes;
    let place=&mut self.actor_fault_place;
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     producer.with_native_pair_slot(fault,cx,|producer,mut home,fault,cx|{
      NativeInstanceHome::with_frozen_origin(producer,cx,|runtime,mut producer,cx|{
       let(instance,poll)=producer.value_mut().frozen_parts()?;
       match current{
        Prepare=>if poll.failure_fetch.prepare_fault_step(home,cx)?{next=Plan;},
        CopyPage=>if poll.failure_fetch.copy_page_step(runtime,instance,poll.failure_pointer,poll.failure_length as usize,home,cx)?{next=ClearCall;},
        Seal=>{
         let permit=poll.failure_fetch.recipient_permit().ok_or_else(||invalid("fault capsule lost its permit"))?;
         if !home.value_mut().seal(permit,cx)?{return Ok(())}
         let address=(bytes as *mut Option<Vec<u8>>)as usize;
         let extent=size_of_val(bytes);
         if !place.capture_is_preissued(){
          if !place.admit_sized(cx,address,extent)?||!place.preissue_capture(cx,address,extent)?{return Ok(())}
         }
         place.qualify_sized(cx,address,extent)?;
         if !Self::copy_fault_bytes(home.value_mut().bytes()?,bytes,cx)?{return Ok(())}
         if poll.failure_fetch.seal_fault_step(home,fault,cx)?{next=Plan;}
         else if fault.is_none(){next=FinishSeal;}
        },
        _=>unreachable!(),
       }
       Ok(())
      })?;
      Ok(())
     })?;
     Ok(())
    }))?;
   },
   FinishSeal=>self.failure_native(false,|_,_,poll,cx|{
    if poll.failure_fetch.finish_seal_step(cx)?{next=Plan;}
    Ok(())
   })?,
   Deliver=>self.failure_native(false,|_,_,poll,cx|{
    if poll.failure_fetch.deliver_step(poll.receiver.actor_failure_recipient(),cx)?{next=Accept;}
    Ok(())
   })?,
   Accept=>{
    let producer=self.producer.as_mut().unwrap();
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     producer.with_native_borrow(cx,|mut producer,cx|{
      let(_,poll)=producer.value_mut().frozen_parts()?;
      if poll.failure_recipient_accepted.is_some(){
       if poll.failure_fetch.accept_recipient_step(&mut poll.failure_recipient_accepted,cx)?{next=Plan;}
      }else{
       ActorFailureRecipient::accept_into_step(producer,|home|{
        let(_,poll)=home.frozen_parts()?;
        Ok((poll.receiver.actor_failure_recipient(),poll.failure_fetch.recipient_permit().ok_or_else(||invalid("fault capsule lost its exact recipient permit"))?,&mut poll.failure_recipient_accepted))
       },cx)?;
      }
      Ok(())
     })?;
     Ok(())
    }))?;
   },
   Confirm=>self.failure_native(false,|runtime,instance,poll,cx|{
    if poll.failure_acknowledged.is_none(){poll.failure_fetch.take_acknowledged_step(&mut poll.failure_acknowledged,cx)?;return Ok(())}
    let proof=poll.failure_acknowledged.as_ref().unwrap().host_proof();
    if proof.runtime_proof().reference().call_kind==semio_framework_actor::ActorFailureCallKind::AdmitActorInput&&!runtime.authorize_original_input_cleanup(instance,proof,cx)?{return Ok(())}
    if !failure_fields(cx,&[size_of_val(&poll.failure_acknowledged)])?{return Ok(())}
    poll.failure_acknowledged=None;
    next=CloseFetch;
    Ok(())
   })?,
   CloseFetch=>self.failure_native(false,|_,_,poll,cx|{
    if !poll.failure_fetch.close_on_source(cx)?||!poll.receiver.actor_failure_recipient().close_on_source(cx)?{return Ok(())}
    if !failure_fields(cx,&[size_of_val(&poll.failure_fetch),size_of::<ActorFailureRecipient>(),size_of::<bool>()])?{return Ok(())}
    poll.failure_fetch=ActorFailureFetch::empty();
    *poll.receiver.actor_failure_recipient()=ActorFailureRecipient::empty();
    poll.failure_active=false;
    next=CloseHome;
    Ok(())
   })?,
   CloseHome=>{
    let id=self.fault_id.ok_or_else(||invalid("fault capsule lost its native recipient identity"))?;
    let place=&mut self.actor_fault_place;
    let mut closed=false;
    self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{
     if place.is_admitted(){place.settle_return(cx)?;return Ok(())}
     closed=cx.close_original_native_recipient_step(id)?;
     Ok(())
    }))?;
    if closed{next=Resume;}
   },
   Resume=>{},
   _=>return Err(invalid("failure service entered an invalid terminal phase").into()),
  }
  Ok(next)
 }

 fn copy_fault_bytes(source:&[u8],out:&mut Option<Vec<u8>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let grant=cx.retained_grant();
  if let Some(bytes)=out.as_mut(){
   if bytes.len()==source.len(){return Ok(true)}
   let remaining=source.len().checked_sub(bytes.len()).ok_or_else(||invalid("fault payload changed its sealed extent"))?;
   let count=remaining.min(grant.maximum_copy_bytes.saturating_sub(size_of::<usize>())).min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));
   if count==0||cx.retirement_should_yield()||grant.maximum_depth==0||grant.maximum_items==0{return Ok(false)}
   let start=bytes.len();
   cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count+size_of::<usize>(),..Default::default()})?;
   cx.consume_fuel(count as u64+1);
   bytes.extend_from_slice(&source[start..start+count]);
   return Ok(bytes.len()==source.len())
  }
  if grant.maximum_capacity_bytes<source.len()||!failure_fields(cx,&[size_of_val(out)])?{return Ok(false)}
  let mut bytes=Vec::new();
  bytes.try_reserve_exact(source.len()).map_err(|_|invalid("fault payload allocation failed"))?;
  cx.consume_retained(RetainedCloneProgress{retained_capacity_bytes:bytes.capacity(),..Default::default()})?;
  *out=Some(bytes);
  Ok(false)
 }
}
