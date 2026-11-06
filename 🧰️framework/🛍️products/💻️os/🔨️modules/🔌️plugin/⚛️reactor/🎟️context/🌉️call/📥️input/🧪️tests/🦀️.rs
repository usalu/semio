//! 🧪️ Actual caller Source owns borrowed tagged input through capture, cancellation and paid close.
use super::*;
use semio_framework_job::{Generation,OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::{RetainedCloneGrant,ValueRefusalKind};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;

fn clock()->Option<u64>{Some(1)}
fn funded<T>(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
    let(result,heap)=observe(||caller.turn(grant,run));let receipt=caller.receipt();
    assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));result.unwrap()
}

#[test]
fn borrowed_actor_input_keeps_identity_and_returns_wire_only_after_source_close(){
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🔁️persistent-local/📊️actor-input.json")).unwrap();
    let stock=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:128};
    let grant=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:131_072,maximum_capacity_bytes:131_072,maximum_release_bytes:131_072,maximum_depth:64};
    for row in rows.as_array().unwrap().iter().filter(|row|row["valid"]==true){
        let wire=serde_json::to_vec(&row["wire"]).unwrap();
        for cancel_at in [None,Some(0),Some(5)]{
            let mut caller=OriginalJobFixtureCaller::admit(OperationId(945),Generation(2),stock,clock,100_000_000,10_000_000);
            let mut receiver=OriginalActorInputReceiving::new(OriginalNativeDecodePolicy{maximum_owned_bytes:stock.maximum_capacity_bytes},JsonReadLimits{maximum_bytes:wire.len() as u64,maximum_allocation_bytes:stock.maximum_capacity_bytes,maximum_depth:64,maximum_items:100_000});
            let mut ready=false;
            for step in 0..100_000{
                if cancel_at==Some(step){break}
                let state=funded(&mut caller,grant,|cx|receiver.advance(Some(&wire),cx));
                assert!(!matches!(state,InputAdmission::Failed));
                if receiver.input_returned(){assert_eq!(state,InputAdmission::Ready);ready=true;break}
                if step==1{let foreign=wire.clone();let refusal=caller.turn(grant,|cx|receiver.advance(Some(&foreign),cx));assert!(matches!(refusal,Err(error) if error.primary_value().is_some_and(|error|error.kind==ValueRefusalKind::InvariantViolated)));}
                assert!(step<99_999);
            }
            if cancel_at.is_none(){assert!(ready);assert!(receiver.input().is_some())}
            if ready&&row["wire"].get("Poll").is_some(){
                assert!(funded(&mut caller,grant,|cx|receiver.prepare_poll_event(cx)));
                assert!(!receiver.poll_event_pending());
                funded(&mut caller,grant,|cx|receiver.with_poll_step(cx,|poll,event,_|{assert!(poll.events.is_empty());assert!(event.is_none());Ok(())}));
            }
            if ready&&row["wire"].get("Job").is_some(){
                let denied=RetainedCloneGrant{maximum_copy_bytes:0,..grant};
                assert!(!funded(&mut caller,denied,|cx|receiver.prepare_job(7,cx)));assert!(receiver.input().is_some());
                assert!(funded(&mut caller,grant,|cx|receiver.prepare_job(7,cx)));
                let pointer=receiver.job_slot(7).unwrap().as_ref().unwrap().kind.as_ptr();
                assert!(funded(&mut caller,grant,|cx|receiver.prepare_job(7,cx)));
                assert_eq!(receiver.job_slot(7).unwrap().as_ref().unwrap().kind.as_ptr(),pointer);
            }
            if cancel_at.is_some(){caller.cancel().cancel_now();}
            let held=if receiver.input_returned(){None}else{Some(wire.as_slice())};
            while !funded(&mut caller,grant,|cx|cx.with_original_retirement(|cx|receiver.begin_close(held,cx))){}
            for step in 0..100_000{
                if receiver.terminal_is_empty(){break}
                let held=if receiver.input_returned(){None}else{Some(wire.as_slice())};
                funded(&mut caller,grant,|cx|cx.with_original_retirement(|cx|receiver.advance(held,cx)));
                assert!(step<99_999);
            }
            assert!(receiver.input_returned());assert!(receiver.terminal_is_empty());drop(receiver);caller.retire(grant);
        }
    }
}

#[test]
fn poll_input_retains_an_unconsumed_event_until_paid_retirement(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🔁️persistent-local/📊️actor-input.json")).unwrap();let mut wire=rows.as_array().unwrap().iter().find_map(|row|row["wire"].get("Poll").map(|_|row["wire"].clone())).unwrap();
 wire["Poll"]["events"]=serde_json::json!([{"instanceClose":{"lifetime":{"activationGeneration":"1","instanceId":0,"guestLifetime":"2"},"requestSequence":2}}]);let wire=serde_json::to_vec(&wire).unwrap();
 let stock=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:128};let grant=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:131_072,maximum_capacity_bytes:131_072,maximum_release_bytes:131_072,maximum_depth:64};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(946),Generation(2),stock,clock,100_000_000,10_000_000);let mut receiver=OriginalActorInputReceiving::new(OriginalNativeDecodePolicy{maximum_owned_bytes:stock.maximum_capacity_bytes},JsonReadLimits{maximum_bytes:wire.len() as u64,maximum_allocation_bytes:stock.maximum_capacity_bytes,maximum_depth:64,maximum_items:100_000});
 for step in 0..100_000{if receiver.input_returned(){break}assert_ne!(funded(&mut caller,grant,|cx|receiver.advance(Some(&wire),cx)),InputAdmission::Failed);assert!(step<99_999)}assert!(receiver.input_returned());
 let low=RetainedCloneGrant{maximum_items:1,..grant};assert!(!funded(&mut caller,low,|cx|receiver.prepare_poll_event(cx)));assert!(receiver.poll_event_pending());assert!(matches!(receiver.input(),Some(ActorInput::Poll(poll)) if poll.events.len()==1));
 assert!(funded(&mut caller,grant,|cx|receiver.prepare_poll_event(cx)));assert!(receiver.poll_event_pending());funded(&mut caller,grant,|cx|receiver.with_poll_step(cx,|poll,event,_|{assert!(poll.events.is_empty());assert!(event.is_some());Ok(())}));assert!(receiver.poll_event_pending());
 while !funded(&mut caller,grant,|cx|cx.with_original_retirement(|cx|receiver.begin_close(None,cx))){}for step in 0..100_000{if receiver.terminal_is_empty(){break}funded(&mut caller,grant,|cx|cx.with_original_retirement(|cx|receiver.advance(None,cx)));assert!(step<99_999)}assert!(receiver.terminal_is_empty());drop(receiver);caller.retire(grant);
}

#[test]
fn input_failure_projection_borrows_all_original_branches_in_fixture_order(){
 use semio_framework_job::{OriginalDomainProjection,OriginalDomainView,OriginalReceivingPrimary};
 let rows:serde_json::Value=serde_json::from_str(include_str!("failure-view.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:1000,maximum_copy_bytes:100000,maximum_capacity_bytes:100000,maximum_release_bytes:100000,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(947),Generation(2),grant,clock,1000000,1000000);
 for row in rows["cases"].as_array().unwrap(){caller.turn(grant,|cx|{let present=row["present"].as_array().unwrap();let error=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original input law cause");
  let outcome=OriginalJsonOutputOutcome{value:if present[0]==true{Err(OriginalReceivingPrimary{primary:Some(error()),retirement_failure:None,work_refusal:None,quote_refusal:None,allocation_refusal:None,allocation_failure:None,native_allocation:None,receipt_refusal:None,progress:Default::default()})}else{Ok(RetainedCloneStep::Complete(Default::default()))},progress:Default::default(),work:0,reported_work:0,observed_native_work:0,receiving_refusal:if present[1]==true{Some(error().into())}else{None}};
  let settlement=OriginalNativeDecodeOutcome{value:if present[2]==true{Err(error())}else{Ok(RetainedCloneStep::Complete(Default::default()))},progress:Default::default(),work:0,reported_work:0,observed_native_work:0,receiving_refusal:if present[3]==true{Some(error().into())}else{None}};
  let owner=&outcome as*const _ as usize;let view=OriginalActorInputFailureView{identity:OriginalActorInputFailureIdentity{owner,outcome:owner,settlement:&settlement as*const _ as usize,binding:OriginalSourceBinding::borrow(cx)},outcome:Some(&outcome),settlement:Some(&settlement)};
  assert_eq!(view.projection_identity(),owner);let expected=row["keys"].as_array().unwrap();assert!(matches!(view.projection_view(&[]).unwrap(),OriginalDomainView::IntrinsicObject(n)if n==expected.len()));for(index,key)in expected.iter().enumerate(){assert_eq!(view.projection_key(&[],index).unwrap(),key.as_str().unwrap());assert!(view.projection_view(&[index]).is_ok())}assert!(view.projection_key(&[],expected.len()).is_err());Ok(())}).unwrap();}
 caller.retire(grant);
}
