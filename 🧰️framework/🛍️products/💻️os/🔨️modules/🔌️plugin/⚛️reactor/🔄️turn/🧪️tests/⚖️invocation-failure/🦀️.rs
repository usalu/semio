//! 📨️ The real request receiver preserves original ownership across pending and complete failure returns.
use semio_plugin_reactor_turn::{receive_retained_extension_invocation, retained_command_ingress_occupancy, retained_extension_invocation_occupancy, CommandIngressOwner, RetainedExtensionInvocation, COMMAND_INGRESS_LIVE_SLOT, COMMAND_INGRESS_RETIRING_SLOT};
use semio_plugin_reactor_turn::invocation_failure_fixture::{take_extension_invocation, with_command_ingress};
use semio_framework::kernel::MessageEndpoint;
use semio_plugin_reactor_lifecycle::instance_lifetime;
use semio_framework_job::{
    admit_original_domain_failure, close_original_effect_step, OriginalClosureFailure,
    OriginalDomainProjection, OriginalDomainReservation, OriginalDomainView, OriginalTurnFailure,
};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework::Effect;
use semio_plugin_testing_caller::OriginalFixtureCaller;

fn retirement_grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth.max(1),
    }
}

fn closure_messages(projection: &dyn OriginalDomainProjection) -> [&str; 4] {
    assert_eq!(projection.projection_key(&[], 0).unwrap(), "initial");
    assert_eq!(projection.projection_key(&[], 1).unwrap(), "closure");
    [&[0, 0, 1, 1][..], &[0, 1, 1], &[1, 0, 1, 1], &[1, 1, 1]]
        .map(|path| {
            let OriginalDomainView::IntrinsicText(text) = projection.projection_view(path).unwrap() else {
                panic!("expected an original closure message");
            };
            text
        })
}

#[test]
fn original_failed_open_keeps_the_paid_domain_through_ingress_retirement() {
    let input: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let policy: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
    let stock: RetainedCloneGrant = serde_json::from_value(policy["grant"].clone()).unwrap();
    let maximum = policy["maximumTurns"].as_u64().unwrap();
    let loan = RetainedCloneGrant {
        maximum_items: 1,
        ..semio_plugin_mounted_owner::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close
    };
    let denied = RetainedCloneGrant { maximum_items: 0, ..loan };
    let cases = input["cases"].as_array().unwrap();
    let [pending, refused, failed] = cases.as_slice() else {
        panic!("expected pending, refused and failed cases");
    };
    assert_eq!([pending["step"].as_str(), refused["step"].as_str(), failed["step"].as_str()],
        [Some("pending"), Some("refused"), Some("failed")]);
    let closure = &input["closure"];
    let expected = [
        failed["cause"].as_str().unwrap(),
        closure["receipt"].as_str().unwrap(),
        closure["primary"].as_str().unwrap(),
        closure["context"].as_str().unwrap(),
    ];
    let open = &input["open"];
    let seq = open["seq"].as_u64().unwrap();

    for slot in [COMMAND_INGRESS_LIVE_SLOT, COMMAND_INGRESS_RETIRING_SLOT] {
        let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(
            semio_framework_job::OperationId(policy["operation"].as_u64().unwrap()),
            semio_framework_job::Generation(policy["generation"].as_u64().unwrap()),
            stock, || Some(1000), policy["deadlineUs"].as_u64().unwrap(), policy["fuel"].as_u64().unwrap(),
        );
        caller.retain_dispatch().unwrap();
        caller.admit_return_table();
        caller.refreshed_turn(stock, |cx| {
            cx.acknowledge_return_rows_locally();
            assert_eq!(retained_command_ingress_occupancy(), 0);
            let mut reservation = OriginalDomainReservation::<OriginalClosureFailure>::new();
            assert!(reservation.prepare(cx)?);
            let ((initial, closure, command), birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                let mut initial = OriginalTurnFailure::from(ValueError::new(ValueRefusalKind::InvalidValue, expected[0]));
                initial.receipt_refusal = Some(ValueError::new(ValueRefusalKind::OwnershipLimit, expected[1]));
                let mut closure = OriginalTurnFailure::from(ValueError::new(ValueRefusalKind::AllocationFailed, expected[2]));
                closure.context_refusal = Some(ValueError::new(ValueRefusalKind::DepthLimit, expected[3]));
                let command = semio_os_kernel::DecodedAppCommandOwner::new(semio_os_kernel::AppCommand::OpenArtifact {
                    seq,
                    artifact_ref: open["artifact"].as_str().unwrap().into(),
                    role: u8::try_from(open["role"].as_u64().unwrap()).unwrap(),
                    plugin_id: open["plugin"].as_str().unwrap().into(),
                    app_id: open["app"].as_str().unwrap().into(),
                });
                (initial, closure, command)
            });
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: birth.requested_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            assert!(reservation.capture_original(cx, || OriginalClosureFailure { initial, closure })?);
            let projection = reservation.pending_projection().unwrap();
            let identity = projection.projection_identity();
            let messages = closure_messages(projection);
            assert_eq!(messages, expected);
            let pointers = messages.map(str::as_ptr);
            let failure = admit_original_domain_failure(&mut reservation, cx)?.unwrap();
            assert!(reservation.terminal_is_empty());
            let close_failure = ValueError::literal(ValueRefusalKind::InvariantViolated, "retirement refused");
            let cursor = semio_framework::kernel::CommandPageCursor {
                owner: 7, generation: 11, command_index: 0, command_count: 1, instance: 13,
                seq, kind: 22, page_index: 0, page_count: 1, item_count: 0, metadata: 0,
            };
            let state = CommandIngressOwner::Generic {
                cursor,
                command: Some((seq, semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded {
                    owner: semio_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand::from_wire(command),
                    failure: Some(failure),
                    identity: (cx.operation(), cx.generation()),
                    close_failure: Some(close_failure.into()),
                    compound: None,
                    raw_media: None,
                })),
                exchange: None, exchange_close: None, output: None, output_close: None,
            };
            let mut returned = None;
            with_command_ingress(slot, instance_lifetime::NativeCloseKey::fixture(13, 1), state, maximum, cx, |retained, cx| {
                let before = (cx.retained_progress(), cx.fuel_remaining());
                let deadline = cx.deadline_us();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                    cx.with_retained_loan(denied, |same| {
                        same.with_original_work_loan(0, deadline, |same| retained.retire_step(same))
                    })
                });
                result?;
                assert!(pending["cause"].is_null());
                assert_eq!(Some(retained.has_owner()), pending["kept"].as_bool());
                assert_eq!((cx.retained_progress(), cx.fuel_remaining()), before);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let refusal = cx.with_retained_loan(loan, |same| retained.retire_step(same)).unwrap_err();
                assert_eq!(Some(refusal.primary.message()), refused["cause"].as_str());
                assert_eq!(Some(retained.has_owner()), refused["kept"].as_bool());
                assert_eq!(retained.failed_original().unwrap().primary_domain().unwrap().projection().projection_identity(), identity);
                for _ in 0..maximum {
                    cx.with_retained_loan(loan, |same| {
                        returned = retained.retire_step(same).err();
                        Ok(())
                    })?;
                    if returned.is_some() {
                        break;
                    }
                }
                assert_eq!(Some(retained.has_owner()), failed["kept"].as_bool());
                Ok(())
            })?;
            let failure = returned.expect("the paid ingress lane returns its original domain");
            let domain = failure.primary_domain().unwrap();
            assert_eq!(domain.code(), "original.source-closure");
            let projection = domain.projection();
            assert_eq!(projection.projection_identity(), identity);
            let messages = closure_messages(projection);
            assert_eq!(messages, expected);
            assert_eq!(messages.map(str::as_ptr), pointers);
            let mut closing = ControlledRetirement::new(failure).unwrap();
            for _ in 0..maximum {
                if closing.terminal_is_empty() {
                    break;
                }
                let demand = closing.selected_demand(64)?;
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                    cx.with_original_retirement(|cx| cx.with_retained_loan(retirement_grant(demand), |same| {
                        close_original_effect_step(same, demand, |effect| closing.step(64, effect)).map(|step| step.progress())
                    }))
                });
                let progress = result?;
                assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            }
            assert!(closing.terminal_is_empty());
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(closing));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(retained_command_ingress_occupancy(), 0);
            Ok(())
        }).unwrap();
        caller.retire(stock);
    }
}

/// 🛑️ Interrupting output retirement with a bounded grant hands the whole generic owner to the retirement ladder: the exchange guard refuses to run or replace it until every controlled retirement is terminal empty, which is the state whose replacement trapped the mounted Open in `Drop`.
#[test]
fn original_guest_failure_debit_interrupted_output_retirement_holds_the_exchange_until_terminal_empty(){
    let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
    let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();let maximum=policy["maximumTurns"].as_u64().unwrap();
    let loan=RetainedCloneGrant{maximum_items:1,..semio_plugin_mounted_owner::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close};
    for slot in [COMMAND_INGRESS_LIVE_SLOT,COMMAND_INGRESS_RETIRING_SLOT]{
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(policy["operation"].as_u64().unwrap()),semio_framework_job::Generation(policy["generation"].as_u64().unwrap()),stock,||Some(1000),policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());
        caller.retain_dispatch().unwrap();caller.admit_return_table();
        caller.refreshed_turn(stock,|cx|{
            cx.acknowledge_return_rows_locally();
            assert_eq!(retained_command_ingress_occupancy(),0);
            let(state,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
                let cursor=semio_framework::kernel::CommandPageCursor{owner:7,generation:11,command_index:0,command_count:1,instance:13,seq:17,kind:19,page_index:0,page_count:1,item_count:0,metadata:0};
                CommandIngressOwner::Generic{cursor,command:None,exchange:Some(semio_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState::empty()),exchange_close:None,output:Some(semio_plugin_runtime_ops::ops_ingress::PluginExchangeOutput{frames:vec![vec![5;300];4],..Default::default()}),output_close:None}
            });
            cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth.requested_bytes,..Default::default()})?;cx.consume_fuel(1);
            with_command_ingress(slot,instance_lifetime::NativeCloseKey::fixture(13,1),state,maximum,cx,|retained,cx|{
                assert!(retained.exchange_may_run(&[]));assert!(retained.exchange_may_run(&[14]));assert!(!retained.exchange_may_run(&[13]));
                let mut interrupted=0;
                for _ in 0..maximum{
                    cx.with_retained_loan(loan,|same|retained.retire_step(same))?;
                    if !retained.has_owner(){break}
                    if retained.output_retiring(){interrupted+=1;assert!(!retained.exchange_may_run(&[]));}
                }
                assert!(!retained.has_owner(),"the paid close completes the interrupted owner");assert!(interrupted>0,"a bounded grant interrupts output retirement");
                Ok(())
            })?;
            assert_eq!(retained_command_ingress_occupancy(),0);Ok(())
        }).unwrap();
        caller.retire(stock);
    }
}

#[test]
fn original_guest_failure_debit_failed_ingress_retiring_lane_returns_same_three_causes_after_paid_close(){
    let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
    let input:serde_json::Value=serde_json::from_str(include_str!("../../../🏃️runtime/🎞️command/📥️custody/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();let maximum=policy["maximumTurns"].as_u64().unwrap();
    let observe=|body:&mut dyn FnMut()|semio_framework_trace::observe_heap_allocations_on_this_thread(body).1;
    for slot in [COMMAND_INGRESS_LIVE_SLOT,COMMAND_INGRESS_RETIRING_SLOT]{for copy in [1,3,64]{
        let (mut caller, parent_birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
            let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(
                semio_framework_job::OperationId(policy["operation"].as_u64().unwrap()),
                semio_framework_job::Generation(policy["generation"].as_u64().unwrap()),
                stock, || Some(1000), policy["deadlineUs"].as_u64().unwrap(), policy["fuel"].as_u64().unwrap(),
            );
            caller.retain_dispatch().unwrap();
            caller.admit_return_table();
            caller
        });
        caller.refreshed_turn(stock,|cx|{
            cx.acknowledge_return_rows_locally();
            assert_eq!(retained_command_ingress_occupancy(),0);let key=instance_lifetime::NativeCloseKey::fixture(13,1);
            let((state,pointers),birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
                let failure={let original=OriginalTurnFailure::from(ValueError::new(ValueRefusalKind::InvalidValue,input["failure"]["primary"].as_str().unwrap()));OriginalTurnFailure{receipt_refusal:Some(ValueError::new(ValueRefusalKind::OwnershipLimit,input["failure"]["receipt"].as_str().unwrap())),context_refusal:Some(ValueError::new(ValueRefusalKind::DepthLimit,input["failure"]["context"].as_str().unwrap())),..original}};
                let pointers=(failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr());
                let command=semio_os_kernel::DecodedAppCommandOwner::new(semio_os_kernel::AppCommand::SubmitMediaExport{seq:input["command"]["seq"].as_str().unwrap().parse().unwrap(),port:input["command"]["port"].as_str().unwrap().into(),expected_parent_document_id:input["command"]["parent"].as_str().unwrap().into(),expected_base_revision:input["command"]["revision"].as_str().unwrap().parse().unwrap(),checkpoint_id:None});
                let cursor=semio_framework::kernel::CommandPageCursor{owner:7,generation:11,command_index:0,command_count:1,instance:13,seq:17,kind:19,page_index:0,page_count:1,item_count:0,metadata:0};
                (CommandIngressOwner::Generic{cursor,command:Some((17,semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded{owner:semio_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand::from_wire(command),failure:Some(failure),identity:(cx.operation(),cx.generation()),close_failure:None,compound:None,raw_media:None})),exchange:None,exchange_close:None,output:None,output_close:None},pointers)
            });
            cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth.requested_bytes,..Default::default()})?;cx.consume_fuel(1);let(mut born,mut freed)=(birth.requested_bytes,birth.released_bytes);
            let mut returned=None;
            let loan = RetainedCloneGrant { maximum_items: 1, ..semio_plugin_mounted_owner::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close };
            let narrow_copy = RetainedCloneGrant { maximum_copy_bytes: copy, ..loan };
            with_command_ingress(slot,key,state,maximum,cx,|retained,cx|{
                let identity=(cx.operation(),cx.generation(),cx.deadline_us(),cx.clock_source()as usize,cx.original_recipient_identity());
                let before = cx.retained_progress();
                let fuel = cx.fuel_remaining();
                let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                    cx.with_retained_loan(narrow_copy, |same| retained.retire_step(same))
                });
                denied?;
                assert!(retained.has_owner());
                assert_eq!(cx.retained_progress(), before);
                assert_eq!(cx.fuel_remaining(), fuel);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                for turn in 0..maximum{
                    let before=cx.retained_progress();let fuel=cx.fuel_remaining();let denied=RetainedCloneGrant{maximum_items:0,..loan};let mut denied_result=None;
                    let heap=observe(&mut||denied_result=Some(cx.with_retained_loan(denied,|same|retained.retire_step(same).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"denied lane cannot return an original cause").into()))));denied_result.unwrap()?;
                    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert!(retained.has_owner());
                    let before=cx.retained_progress();let mut result=None;let heap=observe(&mut||result=Some(cx.with_retained_loan(loan,|same|{let answer=retained.retire_step(same);if let Err(failure)=answer{returned=Some(failure)}Ok(())})));result.unwrap()?;
                    let after=cx.retained_progress();let progress=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};assert!(progress.fits(loan));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;
                    assert_eq!((cx.operation(),cx.generation(),cx.deadline_us(),cx.clock_source()as usize,cx.original_recipient_identity()),identity);if returned.is_some(){break}assert!(turn+1<maximum);
                }
                Ok(())
            })?;
            assert_eq!(retained_command_ingress_occupancy(),0);let failure=returned.unwrap();assert_eq!((failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr()),pointers);
            let mut closing=ControlledRetirement::new(failure).unwrap_or_else(|_|panic!("complete original ingress failure supports native retirement"));
            for turn in 0..maximum{if closing.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_retirement(|cx| { let demand = closing.selected_demand(copy)?; cx.with_retained_loan(retirement_grant(demand), |same| close_original_effect_step(same, demand, |effect| closing.step(copy, effect)).map(|step| step.progress())) }));let progress=result?;assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn+1<maximum)}
            assert!(closing.terminal_is_empty());assert_eq!(born,freed);let(_,drop_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(closing));assert_eq!((drop_heap.requested_bytes,drop_heap.released_bytes),(0,0));assert_eq!(retained_command_ingress_occupancy(),0);
            Ok(())
        }).unwrap();let(_,close)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.retire(stock));assert_eq!(parent_birth.requested_bytes+close.requested_bytes,parent_birth.released_bytes+close.released_bytes);
    }}
}

#[test]
fn original_extension_invocation_receiver_retains_failed_source_and_completes_paid_close(){
    assert_eq!(retained_extension_invocation_occupancy(), 0);
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🧫️fixtures/📨️invocation.json")).unwrap();
    let(mut caller,admission)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OriginalFixtureCaller::admit().unwrap());
    let mut cx=caller.context();let loan=RetainedCloneGrant{maximum_items:1,..semio_plugin_mounted_owner::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close};let maximum=OriginalFixtureCaller::maximum_turns();
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
        let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receive_retained_extension_invocation(0,owner,returned));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(result.is_err(),row["failureReturned"].as_bool().unwrap());let pending=take_extension_invocation(0);assert_eq!(pending.is_some(),row["ownerRetained"].as_bool().unwrap());
        if let Err(failure)=result{assert_eq!(Some((failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr())),fault_pointers);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).unwrap();cx.consume_fuel(1);let mut failure=ControlledRetirement::new(failure).unwrap();for turn in 0..maximum{if failure.terminal_is_empty(){break}let demand=failure.selected_demand(64).unwrap();let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_retirement(|cx|cx.with_retained_loan(retirement_grant(demand),|same|{close_original_effect_step(same,demand,|effect|failure.step(64,effect))?;Ok(())})));result.unwrap();let progress=delta(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn+1<maximum)}assert!(failure.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(failure));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
        else if let Ok(Some(Effect::Respond{req,result:semio_framework::kernel::RequestOutcome::Ok(bytes)}))=result{assert_eq!(req.0,721);assert_eq!(bytes,vec![9;79]);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).unwrap();cx.consume_fuel(1);let mut bytes=ControlledRetirement::new(bytes).unwrap();for turn in 0..maximum{if bytes.terminal_is_empty(){break}let demand=bytes.selected_demand(64).unwrap();let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_retirement(|cx|cx.with_retained_loan(retirement_grant(demand),|same|{close_original_effect_step(same,demand,|effect|bytes.step(64,effect))?;Ok(())})));result.unwrap();let progress=delta(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn+1<maximum)}assert!(bytes.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(bytes));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break}
        owner=pending.unwrap();let held=owner.source.as_ref().unwrap();assert_eq!((held.0.as_ptr(),held.1.as_ptr(),held.2.as_ptr()),pointers);
    }
    assert_eq!(retained_extension_invocation_occupancy(),0);let fuel=cx.fuel_remaining();drop(cx);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{caller.retire(fuel).unwrap();drop(caller);});born+=heap.requested_bytes;freed+=heap.released_bytes;assert_eq!(born,freed);
}
