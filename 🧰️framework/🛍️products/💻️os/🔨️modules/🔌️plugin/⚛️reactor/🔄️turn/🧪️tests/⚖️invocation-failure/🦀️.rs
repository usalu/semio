//! 📨️ The real request receiver preserves original ownership across pending and complete failure returns.
use semio_framework::kernel::MessageEndpoint;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
use semio_framework_plugin_reactor_lifecycle::instance_lifetime;
use semio_framework_plugin_reactor_lifecycle as pending;
use semio_framework_plugin_reactor_turn as turn;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework_plugin_reactor_wit_bridge::wit_bridge::poll;
use semio_framework_job::{OriginalTurnFailure, close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework::Effect;
use semio_framework_plugin_reactor_turn::CommandIngressOwner;
use semio_framework_plugin_reactor_turn::RetainedExtensionInvocation;

#[test]
fn original_guest_failure_debit_failed_ingress_retiring_lane_returns_same_three_causes_after_paid_close(){
    let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
    let input:serde_json::Value=serde_json::from_str(include_str!("../../../🏃️runtime/🎞️command/📥️custody/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();let maximum=policy["maximumTurns"].as_u64().unwrap();
    let observe=|body:&mut dyn FnMut()|semio_framework_trace::observe_heap_allocations_on_this_thread(body).1;
    for slot in [COMMAND_INGRESS_LIVE_SLOT,COMMAND_INGRESS_RETIRING_SLOT]{for copy in [1,3,64]{
        let(mut caller,parent_birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(policy["operation"].as_u64().unwrap()),semio_framework_job::Generation(policy["generation"].as_u64().unwrap()),stock,||Some(1000),policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap()));
        caller.turn(stock,|cx|{
            assert_eq!(retained_command_ingress_occupancy(),0);let key=instance_lifetime::NativeCloseKey::fixture(13,1);
            let((state,pointers),birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
                let failure={let original=OriginalTurnFailure::from(ValueError::new(ValueRefusalKind::InvalidValue,input["failure"]["primary"].as_str().unwrap()));OriginalTurnFailure{receipt_refusal:Some(ValueError::new(ValueRefusalKind::OwnershipLimit,input["failure"]["receipt"].as_str().unwrap())),context_refusal:Some(ValueError::new(ValueRefusalKind::DepthLimit,input["failure"]["context"].as_str().unwrap())),..original}};
                let pointers=(failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr());
                let command=semio_framework_os_kernel::DecodedAppCommandOwner::new(semio_framework_os_kernel::AppCommand::SubmitMediaExport{seq:input["command"]["seq"].as_str().unwrap().parse().unwrap(),port:input["command"]["port"].as_str().unwrap().into(),expected_parent_document_id:input["command"]["parent"].as_str().unwrap().into(),expected_base_revision:input["command"]["revision"].as_str().unwrap().parse().unwrap()});
                let cursor=semio_framework::kernel::CommandPageCursor{owner:7,generation:11,command_index:0,command_count:1,instance:13,seq:17,kind:19,page_index:0,page_count:1,item_count:0,metadata:0};
                (CommandIngressOwner::Generic{cursor,command:semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded{owner:command,failure:Some(failure),identity:(cx.operation(),cx.generation()),close_failure:None,compound:None,raw_media:None}},pointers)
            });
            cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth.requested_bytes,..Default::default()})?;cx.consume_fuel(1);let(mut born,mut freed)=(birth.requested_bytes,birth.released_bytes);
            COMMAND_INGRESS.with(|owners|owners.borrow_mut()[slot]=Some(RetainedCommandIngress{key,state}));
            let identity=(cx.operation(),cx.generation(),cx.deadline_us(),cx.clock_source()as usize,cx.original_recipient_identity());let loan=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,..stock};let mut returned=None;
            for turn in 0..maximum{
                let before=cx.retained_progress();let fuel=cx.fuel_remaining();let denied=RetainedCloneGrant{maximum_items:0,..loan};let mut denied_result=None;
                let heap=observe(&mut||denied_result=Some(cx.with_retained_loan(denied,|same|if slot==COMMAND_INGRESS_RETIRING_SLOT{step_retiring_command_ingress(same).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"denied retiring lane cannot return an original cause"))}else{close_command_ingress_step(key,same).map(|_|()).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"denied live lane cannot return an original cause"))})));denied_result.unwrap()?;
                assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(retained_command_ingress_occupancy(),1);
                let before=cx.retained_progress();let mut result=None;let heap=observe(&mut||result=Some(cx.with_retained_loan(loan,|same|{let answer=if slot==COMMAND_INGRESS_RETIRING_SLOT{step_retiring_command_ingress(same)}else{close_command_ingress_step(key,same).map(|_|())};if let Err(failure)=answer{returned=Some(failure)}Ok(())})));result.unwrap()?;
                let after=cx.retained_progress();let progress=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};assert!(progress.fits(loan));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;
                assert_eq!((cx.operation(),cx.generation(),cx.deadline_us(),cx.clock_source()as usize,cx.original_recipient_identity()),identity);if returned.is_some(){break}assert!(turn+1<maximum);
            }
            assert_eq!(retained_command_ingress_occupancy(),0);let failure=returned.unwrap();assert_eq!((failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr()),pointers);
            let mut closing=ControlledRetirement::new(failure).unwrap_or_else(|_|panic!("complete original ingress failure supports native retirement"));
            for turn in 0..maximum{if closing.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|close_original_retained_step(same,closing.selected_demand(copy)?,|grant|closing.step(grant)).map(|step|step.progress())));let progress=result?;assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn+1<maximum)}
            assert!(closing.terminal_is_empty());assert_eq!(born,freed);let(_,drop_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(closing));assert_eq!((drop_heap.requested_bytes,drop_heap.released_bytes),(0,0));assert_eq!(retained_command_ingress_occupancy(),0);
            Ok(())
        }).unwrap();let(_,close)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.retire(stock));assert_eq!(parent_birth.requested_bytes+close.requested_bytes,parent_birth.released_bytes+close.released_bytes);
    }}
}

#[test]
fn original_extension_invocation_receiver_retains_failed_source_and_completes_paid_close(){
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🧫️fixtures/📨️invocation.json")).unwrap();
    let(mut caller,admission)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap());
    let mut cx=caller.context();let loan=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,..cx.retained_grant()};let maximum=semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns();
    let((mut owner,mut refusal,mut output),birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(RetainedExtensionInvocation::new(semio_framework::kernel::RequestId(721),MessageEndpoint::Shell{instance:semio_framework::kernel::PluginInstanceId("original source".into())},"original capability".into(),vec![7;211]),Some({let original=OriginalTurnFailure::from(ValueError::new(ValueRefusalKind::AllocationFailed,"original failed child source"));OriginalTurnFailure{receipt_refusal:Some(ValueError::new(ValueRefusalKind::OwnershipLimit,"original receipt source")),context_refusal:Some(ValueError::new(ValueRefusalKind::InvariantViolated,"original context source")),..original}}),Some(vec![9;79])));
    let pointers={let held=owner.source.as_ref().unwrap();(held.0.as_ptr(),held.1.as_ptr(),held.2.as_ptr())};let mut born=admission.requested_bytes+birth.requested_bytes;let mut freed=admission.released_bytes+birth.released_bytes;
    let delta=|a:RetainedCloneProgress,b:RetainedCloneProgress|RetainedCloneProgress{copied_items:b.copied_items.checked_sub(a.copied_items).unwrap(),copied_bytes:b.copied_bytes.checked_sub(a.copied_bytes).unwrap(),retained_capacity_bytes:b.retained_capacity_bytes.checked_sub(a.retained_capacity_bytes).unwrap(),released_bytes:b.released_bytes.checked_sub(a.released_bytes).unwrap()};
    for row in rows.as_array().unwrap(){
        let outcome=row["outcome"].as_str().unwrap();let returned=if outcome=="completed"{
            owner.result=Some(semio_framework::kernel::RequestOutcome::Ok(output.take().unwrap()));let mut answer=None;
            for turn in 0..maximum{let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{answer=semio_framework_async::poll::resolve_ready(owner.step(same))?;Ok(())}));let progress=delta(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;result.unwrap();if answer.is_some(){break}assert!(turn+1<maximum)}
            Ok(answer)
        }else if outcome=="failed"{Err(refusal.take().unwrap())}else{Ok(None)};
        let fault_pointers=returned.as_ref().err().map(|failure|(failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr()));
        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receive_retained_extension_invocation(0,owner,returned));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(result.is_err(),row["failureReturned"].as_bool().unwrap());let pending=RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow_mut()[0].take());assert_eq!(pending.is_some(),row["ownerRetained"].as_bool().unwrap());
        if let Err(failure)=result{assert_eq!(Some((failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr())),fault_pointers);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).unwrap();cx.consume_fuel(1);let mut failure=ControlledRetirement::new(failure).unwrap();for turn in 0..maximum{if failure.terminal_is_empty(){break}let demand=failure.selected_demand(64).unwrap();let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{close_original_retained_step(same,demand,|grant|failure.step(grant))?;Ok(())}));result.unwrap();let progress=delta(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn+1<maximum)}assert!(failure.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(failure));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
        else if let Ok(Some(Effect::Respond{req,result:semio_framework::kernel::RequestOutcome::Ok(bytes)}))=result{assert_eq!(req.0,721);assert_eq!(bytes,vec![9;79]);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).unwrap();cx.consume_fuel(1);let mut bytes=ControlledRetirement::new(bytes).unwrap();for turn in 0..maximum{if bytes.terminal_is_empty(){break}let demand=bytes.selected_demand(64).unwrap();let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{close_original_retained_step(same,demand,|grant|bytes.step(grant))?;Ok(())}));result.unwrap();let progress=delta(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn+1<maximum)}assert!(bytes.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(bytes));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break}
        owner=pending.unwrap();let held=owner.source.as_ref().unwrap();assert_eq!((held.0.as_ptr(),held.1.as_ptr(),held.2.as_ptr()),pointers);
    }
    assert_eq!(retained_extension_invocation_occupancy(),0);let fuel=cx.fuel_remaining();drop(cx);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.retire(fuel).unwrap());born+=heap.requested_bytes;freed+=heap.released_bytes;assert_eq!(born,freed);
}
