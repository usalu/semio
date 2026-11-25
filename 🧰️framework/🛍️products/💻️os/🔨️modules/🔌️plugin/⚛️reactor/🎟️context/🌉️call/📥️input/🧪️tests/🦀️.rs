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
fn bounded_actor_input_admission_uses_multiple_paid_steps_before_returning() {
    with_input_decode_stack(check_bounded_actor_input);
}

fn with_input_decode_stack(check: fn()) {
    let stack_mib = if cfg!(debug_assertions) { 16 } else { 8 };
    let checked = std::thread::Builder::new()
        .stack_size(stack_mib * 1024 * 1024)
        .spawn(check)
        .unwrap()
        .join();
    if let Err(failure) = checked {
        std::panic::resume_unwind(failure);
    }
}

fn check_bounded_actor_input() {
    let rows: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🔁️persistent-local/📊️actor-input.json")).unwrap();
    let poll = rows.as_array().unwrap().iter().find(|row| {
        row["valid"] == true && row["wire"]["Poll"]["events"][0].get("instanceOpen").is_some()
    }).unwrap();
    let wire = serde_json::to_vec(&poll["wire"]).unwrap();
    let expected_events = poll["wire"]["Poll"]["events"].as_array().unwrap().len();
    let expected_open = &poll["wire"]["Poll"]["events"][0]["instanceOpen"];
    let stock = RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 32_000_000, maximum_capacity_bytes: 32_000_000, maximum_release_bytes: 32_000_000, maximum_depth: 128 };
    let grant = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 };
    let calls = [true, false].map(|batch| {
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(945), Generation(2), stock, clock, 100_000_000, 10_000_000);
        caller.retain_dispatch().unwrap();
        let table = OriginalJobFixtureCaller::table_birth();
        let short = RetainedCloneGrant { maximum_items: table.maximum_items - 1, ..table };
        for _ in 0..4 {
            assert!(!funded(&mut caller, short, |cx| Ok(cx.admit_original_return_table()?)));
        }
        assert_eq!(caller.receipt(), RetainedCloneProgress::default());
        assert_eq!(caller.admit_return_table(), table.maximum_capacity_bytes);
        let mut receiver = OriginalActorInputReceiving::new(
            OriginalNativeDecodePolicy { maximum_owned_bytes: stock.maximum_capacity_bytes },
            JsonReadLimits { maximum_bytes: wire.len() as u64, maximum_allocation_bytes: stock.maximum_capacity_bytes, maximum_depth: 64, maximum_items: 100_000 },
        );
        let mut checked_capacity = false;
        let first_ready = (1..1000u32).find(|step| {
            let admission = funded(&mut caller, grant, |cx| {
                if batch && checked_capacity { receiver.advance_in_turn(&wire, cx) }
                else { receiver.advance((!receiver.input_returned()).then_some(wire.as_slice()), cx) }
            });
            assert!(matches!(admission, InputAdmission::Pending | InputAdmission::Ready), "step {step}, batch {batch}, phase {:?}, value {:?}, receiving {:?}, settlement {:?}", receiver.decoder.receiver().map(|decoder| decoder.phase()), receiver.outcome().and_then(|raw| raw.value.as_ref().err()).map(|error| match error {
                OriginalJsonOutputFailure::Refused(error) => error.message.as_ref(),
                OriginalJsonOutputFailure::Stored(_) => receiver.decoder.failure().and_then(|failure| failure.primary().primary.as_ref().or(failure.primary().quote_refusal.as_ref()).or(failure.primary().allocation_refusal.as_ref())).map_or("stored", |error| error.message.as_ref()),
            }), receiver.outcome().and_then(|raw| receiver.refusal(raw)).map(|error| error.primary.message()), receiver.settlement.as_ref().and_then(ControlledRetirement::original).and_then(|raw| raw.value.as_ref().err()).map(|error| error.message.as_ref()));
            if !checked_capacity {
                if let Some(demand) = receiver.decoder.receiver().and_then(|decoder| decoder.demands(&wire, grant.maximum_copy_bytes, u64::MAX).ok()).filter(|demand| demand.capacity_bytes > 0) {
                    let exact = RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes, ..grant };
                    let short = RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..exact };
                    assert_eq!(funded(&mut caller, short, |cx| receiver.advance_in_turn(&wire, cx)), InputAdmission::Pending);
                    let denied = caller.receipt();
                    assert_eq!((denied.retained_capacity_bytes, denied.released_bytes), (0, 0));
                    assert!(denied.copied_items < 32, "a denied decoder quote repeated capture bookkeeping");
                    assert_eq!(receiver.decoder.receiver().unwrap().demands(&wire, grant.maximum_copy_bytes, u64::MAX).unwrap(), demand);
                    assert_eq!(funded(&mut caller, exact, |cx| receiver.advance_in_turn(&wire, cx)), InputAdmission::Pending);
                    assert_eq!(caller.receipt().retained_capacity_bytes, demand.capacity_bytes);
                    checked_capacity = true;
                }
            }
            admission == InputAdmission::Ready
        }).expect("actual decoder should return its wire within the finite Source");
        assert!(checked_capacity);
        assert!(receiver.input_returned());
        funded(&mut caller, grant, |cx| receiver.with_poll_step(cx, |poll, _, _| {
            assert_eq!(poll.events.len(), expected_events);
            let semio_framework::kernel::Event::InstanceOpen { app_id, actor, config, .. } = &poll.events[0] else { panic!("expected the instance-open event") };
            assert_eq!(Some(app_id.0.as_str()), expected_open["appId"].as_str());
            assert_eq!(Some(actor.as_str()), expected_open["actor"].as_str());
            assert_eq!(config.len(), expected_open["config"].as_array().unwrap().len());
            Ok(())
        }));
        assert!(funded(&mut caller, grant, |cx| cx.with_original_retirement(|cx| receiver.begin_close(None, cx))));
        for _ in 0..1000 {
            if receiver.terminal_is_empty() { break; }
            funded(&mut caller, grant, |cx| cx.with_original_retirement(|cx| receiver.advance(None, cx)));
        }
        assert!(receiver.terminal_is_empty());
        drop(receiver);
        caller.retire(grant);
        first_ready
    });
    assert!(calls[0] < calls[1], "batched {} turns, single-step {} turns", calls[0], calls[1]);
}

#[test]
fn mcp_instance_open_admits_the_descriptor_grant() {
    with_input_decode_stack(check_mcp_instance_open);
}

fn check_mcp_instance_open() {
    let rows: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🔁️persistent-local/📊️actor-input.json")).unwrap();
    let infrastructure = RetainedCloneGrant {
        maximum_items: 65_536, maximum_copy_bytes: 16 << 20, maximum_capacity_bytes: 64 << 20,
        maximum_release_bytes: 64 << 20, maximum_depth: 64,
    };
    let bounded = RetainedCloneGrant {
        maximum_items: 256, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20, maximum_depth: 64,
    };
    let stock = RetainedCloneGrant {
        maximum_items: 1_000_000, maximum_copy_bytes: 128 << 20, maximum_capacity_bytes: 128 << 20,
        maximum_release_bytes: 128 << 20, maximum_depth: 64,
    };
    for app in ["s.note.note@1/*#editor", "s.wfc.bitmap@1/*#editor"] {
        let vector = rows.as_array().unwrap().iter().map(|row| &row["wire"]).find(|wire| {
            let event = &wire["Poll"]["events"][0]["instanceOpen"];
            event["appId"] == app && event["capabilities"].get(0).is_some()
        }).unwrap();
        assert_eq!(vector["Poll"]["limits"].as_array().unwrap().len(), semio_plugin_abi::owned_abi::OwnedCorePollLimits::ARITY);
        let typed: ActorInput = serde_json::from_value(vector.clone()).unwrap();
        assert_eq!(serde_json::to_value(&typed).unwrap(), *vector);
        let wire = serde_json::to_vec(&typed).unwrap();
        let wire_identity = (wire.as_ptr(), wire.len());
        let profiles = [("infrastructure", infrastructure), ("bounded", bounded)].map(|(scope, grant)| {
            let mut caller = OriginalJobFixtureCaller::admit(OperationId(948), Generation(2), stock, clock, 10_000_000, 100_000);
            caller.retain_dispatch().unwrap();
            caller.admit_return_table();
            let mut receiver = OriginalActorInputReceiving::new(
                OriginalNativeDecodePolicy { maximum_owned_bytes: infrastructure.maximum_capacity_bytes },
                JsonReadLimits {
                    maximum_bytes: infrastructure.maximum_copy_bytes as u64,
                    maximum_allocation_bytes: infrastructure.maximum_capacity_bytes,
                    maximum_depth: infrastructure.maximum_depth,
                    maximum_items: infrastructure.maximum_items as u64,
                },
            );
            let mut receipt = RetainedCloneProgress::default();
            let mut position = 0;
            let start_fuel = caller.fuel_remaining();
            let ready_turn = (1..=1024).find(|turn| {
                let admission = funded(&mut caller, grant, |cx| receiver.advance_in_turn(&wire, cx));
                receipt = receipt.checked_add(caller.receipt()).unwrap();
                assert_eq!((wire.as_ptr(), wire.len()), wire_identity);
                if receiver.decoder.receiver().is_some() && !receiver.input_returned() {
                    assert!(receiver.owns_wire(&wire));
                }
                if let Some(decoder) = receiver.decoder.receiver() {
                    assert!(decoder.bytes_read() >= position);
                    position = decoder.bytes_read();
                }
                assert!(matches!(admission, InputAdmission::Pending | InputAdmission::Ready),
                    "{app} {scope} turn {turn} at {position}/{}: {:?}, {:?}, {:?}, {receipt:?}",
                    wire.len(), receiver.failure_kind(), receiver.decoder.receiver().map(|decoder| decoder.phase()),
                    receiver.decoder.failure().map(NativeReceivingFailure::primary));
                assert_eq!(receiver.input_returned(), admission == InputAdmission::Ready);
                admission == InputAdmission::Ready
            }).unwrap_or_else(|| panic!("{app} {scope} did not admit at {position}/{}: receipt {receipt:?}", wire.len()));
            let work = start_fuel - caller.fuel_remaining();
            assert_eq!(serde_json::to_value(receiver.input().unwrap()).unwrap(), *vector);
            assert_eq!(position, wire.len());
            assert!(work > 0 && receipt.copied_bytes > wire.len() && receipt.retained_capacity_bytes > 0);
            if scope == "infrastructure" {
                assert_eq!(ready_turn, 1);
                assert!(receipt.fits(grant), "{app} spent {receipt:?}");
            }
            assert!(funded(&mut caller, grant, |cx| cx.with_original_retirement(|cx| receiver.begin_close(None, cx))));
            for _ in 0..1024 {
                if receiver.terminal_is_empty() { break; }
                funded(&mut caller, grant, |cx| cx.with_original_retirement(|cx| receiver.advance(None, cx)));
            }
            assert!(receiver.terminal_is_empty());
            caller.retire(grant);
            (ready_turn, receipt.retained_capacity_bytes, receipt.released_bytes)
        });
        assert!(profiles[1].0 > profiles[0].0);
        assert_eq!((profiles[0].1, profiles[0].2), (profiles[1].1, profiles[1].2));
    }
}

#[test]
fn borrowed_actor_input_keeps_identity_and_returns_wire_only_after_source_close(){
    with_input_decode_stack(check_borrowed_actor_input);
}

fn check_borrowed_actor_input(){
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🔁️persistent-local/📊️actor-input.json")).unwrap();
    let stock=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:128};
    let grant=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:1 << 20,maximum_capacity_bytes:1 << 20,maximum_release_bytes:1 << 20,maximum_depth:64};
    for row in rows.as_array().unwrap().iter().filter(|row|row["valid"]==true){
        let wire=serde_json::to_vec(&row["wire"]).unwrap();
        for cancel_at in [None,Some(0),Some(5)]{
            let mut caller=OriginalJobFixtureCaller::admit(OperationId(945),Generation(2),stock,clock,100_000_000,10_000_000);
            caller.retain_dispatch().unwrap();
            caller.admit_return_table();
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
                let has_event = !row["wire"]["Poll"]["events"].as_array().unwrap().is_empty();
                assert!(funded(&mut caller,grant,|cx|receiver.prepare_poll_event(cx)));
                assert_eq!(receiver.poll_event_pending(), has_event);
                funded(&mut caller,grant,|cx|receiver.with_poll_step(cx,|poll,event,_|{assert!(poll.events.is_empty());assert_eq!(event.is_some(),has_event);Ok(())}));
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
    with_input_decode_stack(check_poll_event_retirement);
}

fn check_poll_event_retirement(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🔁️persistent-local/📊️actor-input.json")).unwrap();let mut wire=rows.as_array().unwrap().iter().find_map(|row|row["wire"].get("Poll").map(|_|row["wire"].clone())).unwrap();
 wire["Poll"]["events"]=serde_json::json!([{"instanceClose":{"lifetime":{"activationGeneration":"1","instanceId":0,"guestLifetime":"2"},"requestSequence":2}}]);let wire=serde_json::to_vec(&wire).unwrap();
 let stock=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:128};let grant=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(946),Generation(2),stock,clock,100_000_000,10_000_000);let mut receiver=OriginalActorInputReceiving::new(OriginalNativeDecodePolicy{maximum_owned_bytes:stock.maximum_capacity_bytes},JsonReadLimits{maximum_bytes:wire.len() as u64,maximum_allocation_bytes:stock.maximum_capacity_bytes,maximum_depth:64,maximum_items:100_000});
 caller.retain_dispatch().unwrap();
 caller.admit_return_table();
 for step in 0..100_000{if receiver.input_returned(){break}assert_ne!(funded(&mut caller,grant,|cx|receiver.advance(Some(&wire),cx)),InputAdmission::Failed);assert!(step<99_999)}assert!(receiver.input_returned());
 let low=RetainedCloneGrant{maximum_items:1,..grant};assert!(!funded(&mut caller,low,|cx|receiver.prepare_poll_event(cx)));assert!(receiver.poll_event_pending());assert!(matches!(receiver.input(),Some(ActorInput::Poll(poll)) if poll.events.len()==1));
 assert!(funded(&mut caller,grant,|cx|receiver.prepare_poll_event(cx)));assert!(receiver.poll_event_pending());funded(&mut caller,grant,|cx|receiver.with_poll_step(cx,|poll,event,_|{assert!(poll.events.is_empty());assert!(event.is_some());Ok(())}));assert!(receiver.poll_event_pending());
 while !funded(&mut caller,grant,|cx|cx.with_original_retirement(|cx|receiver.begin_close(None,cx))){}for step in 0..100_000{if receiver.terminal_is_empty(){break}funded(&mut caller,grant,|cx|cx.with_original_retirement(|cx|receiver.advance(None,cx)));assert!(step<99_999)}assert!(receiver.terminal_is_empty());drop(receiver);caller.retire(grant);
}

#[test]
fn input_failure_projection_borrows_all_original_branches_in_fixture_order(){
 use semio_framework_job::{OriginalDomainProjection,OriginalDomainView};
 use semio_framework_pack_json_output_receiving::OriginalJsonOutputFailure;
 let rows:serde_json::Value=serde_json::from_str(include_str!("failure-view.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:1000,maximum_copy_bytes:100000,maximum_capacity_bytes:100000,maximum_release_bytes:100000,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(947),Generation(2),grant,clock,1000000,1000000);
 for row in rows["cases"].as_array().unwrap(){caller.turn(grant,|cx|{let present=row["present"].as_array().unwrap();let error=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original input law cause");
  let outcome=OriginalJsonOutputOutcome{value:if present[0]==true{Err(OriginalJsonOutputFailure::Refused(error()))}else{Ok(RetainedCloneStep::Complete(Default::default()))},progress:Default::default(),work:0,reported_work:0,observed_native_work:0,receiving_refusal:if present[1]==true{Some(Default::default())}else{None}};let outcome_refusal=if present[1]==true{Some(semio_framework_job::OriginalTurnFailure::from(error()))}else{None};
  let settlement=OriginalNativeDecodeOutcome{value:if present[2]==true{Err(error())}else{Ok(RetainedCloneStep::Complete(Default::default()))},progress:Default::default(),work:0,reported_work:0,observed_native_work:0,receiving_refusal:if present[3]==true{Some(Default::default())}else{None}};let settlement_refusal=if present[3]==true{Some(semio_framework_job::OriginalTurnFailure::from(error()))}else{None};
  let owner=&outcome as*const _ as usize;let view=OriginalActorInputFailureView{identity:OriginalActorInputFailureIdentity{owner,outcome:owner,settlement:&settlement as*const _ as usize,binding:OriginalSourceBinding::borrow(cx)},outcome:Some(&outcome),receiving:None,settlement:Some(&settlement),outcome_refusal:outcome_refusal.as_ref(),settlement_refusal:settlement_refusal.as_ref()};
  assert_eq!(view.projection_identity(),owner);let expected=row["keys"].as_array().unwrap();assert!(matches!(view.projection_view(&[]).unwrap(),OriginalDomainView::IntrinsicObject(n)if n==expected.len()));for(index,key)in expected.iter().enumerate(){assert_eq!(view.projection_key(&[],index).unwrap(),key.as_str().unwrap());assert!(view.projection_view(&[index]).is_ok())}assert!(view.projection_key(&[],expected.len()).is_err());Ok(())}).unwrap();}
 caller.retire(grant);
}

#[test]
fn compact_job_source_matches_serde_and_retires_its_partial_and_complete_owners() {
    with_input_decode_stack(check_compact_job_source);
}

fn check_compact_job_source() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("cold-job.json")).unwrap();
    let cases=fixture["cases"].as_array().unwrap();
    for row in cases {
        let wire=compact_hex(row["hex"].as_str().unwrap());
        let mut source=row["source"].clone();
        source["job"]=source["job"].as_str().unwrap().parse::<u64>().unwrap().into();
        check_compact_job_case(&wire,Some(source),false);
    }
    check_compact_job_case(b"SJS1",None,false);
    check_compact_job_case(br#"{"Job":{"job":19,"kind":"snapshot","input":null,"checkpoint":null}}"#,None,false);
    for row in fixture["malformed"].as_array().unwrap() {
        let mut wire=compact_hex(cases[row["case"].as_u64().unwrap() as usize]["hex"].as_str().unwrap());
        wire[row["index"].as_u64().unwrap() as usize]=row["value"].as_u64().unwrap() as u8;
        check_compact_job_case(&wire,None,false);
    }

    let large=&fixture["largeInput"];
    let pattern:Vec<u8>=serde_json::from_value(large["pattern"].clone()).unwrap();
    let payload=pattern.repeat(large["repeat"].as_u64().unwrap() as usize);
    let job=large["job"].as_u64().unwrap();
    let kind=large["kind"].as_str().unwrap();
    let mut wire=b"SJS1\x01\x00\x00\x00".to_vec();
    for word in [job,kind.len() as u64,payload.len() as u64,0] { wire.extend(word.to_le_bytes()); }
    wire.extend(kind.as_bytes());
    wire.extend(&payload);
    let input=serde_json::json!({"job":job,"kind":kind,"input":payload,"checkpoint":null});
    check_compact_job_case(&wire,Some(input.clone()),false);
    check_compact_job_case(&wire,Some(input),true);
}

fn compact_hex(hex:&str)->Vec<u8> {
    (0..hex.len()).step_by(2).map(|index|u8::from_str_radix(&hex[index..index+2],16).unwrap()).collect()
}

fn check_compact_job_case(wire:&[u8], expected:Option<serde_json::Value>, cancel_partial:bool) {
    let receiver_bytes=size_of::<OriginalActorInputReceiving>();
    let json=wire.starts_with(b"{");
    let stock=if json{
        RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:128}
    }else{
        RetainedCloneGrant{maximum_items:20_000,maximum_copy_bytes:(1<<20)+receiver_bytes,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}
    };
    let admission=RetainedCloneGrant{maximum_copy_bytes:receiver_bytes+16_384,..stock};
    let slice=if json{
        RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}
    }else{
        RetainedCloneGrant{maximum_items:100,maximum_copy_bytes:16_384,..stock}
    };
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(952),Generation(2),stock,clock,5_000_000,100_000);
    caller.retain_dispatch().unwrap();
    caller.admit_return_table();
    let mut receiver=funded(&mut caller,admission,|cx|{
        assert!(charge(cx,false,receiver_bytes)?);
        Ok(OriginalActorInputReceiving::new_bound(
            OriginalNativeDecodePolicy{maximum_owned_bytes:stock.maximum_capacity_bytes},
            JsonReadLimits{maximum_bytes:stock.maximum_copy_bytes as u64,maximum_allocation_bytes:stock.maximum_capacity_bytes,maximum_depth:64,maximum_items:stock.maximum_items as u64},
            wire,cx,
        ))
    });
    let substitute=wire.to_vec();
    assert!(caller.turn(slice,|cx|receiver.advance_in_turn(&substitute,cx)).is_err());
    let mut opened=0;
    let mut closed=0;
    let mut items=0;
    let wanted=usize::from(expected.is_some()&&!cancel_partial);
    let mut reads=0;
    let mut final_state=InputAdmission::Pending;
    for _ in 0..100 {
        final_state=funded(&mut caller,slice,|cx|receiver.advance_in_turn(wire,cx));
        let receipt=caller.receipt();
        opened+=receipt.retained_capacity_bytes;
        closed+=receipt.released_bytes;
        items+=receipt.copied_items;
        if final_state!=InputAdmission::Pending { break; }
        if let Some(ActorInput::Job(job))=receiver.input.as_ref() {
            reads=job.input.as_ref().map_or(0,Vec::len);
            if cancel_partial&&reads>0 { break; }
        }
    }
    assert_eq!(usize::from(final_state==InputAdmission::Ready),wanted);
    if cancel_partial {
        assert!(matches!(final_state,InputAdmission::Pending)&&reads>0&&reads<wire.len()-64);
    }else if let Some(expected)=expected {
        let job=expected["job"].as_u64().unwrap();
        let expected:semio_framework_actor::OriginalJobSource=serde_json::from_value(expected).unwrap();
        let Some(ActorInput::Job(actual))=receiver.input()else{panic!("expected compact Job")};
        assert_eq!(serde_json::to_value(actual).unwrap(),serde_json::to_value(expected).unwrap());
        if wire.len()>16_384 { assert!(items<1_000,"retained item count grew with the payload octets: {items}"); }
        let denied=RetainedCloneGrant{maximum_copy_bytes:0,..stock};
        assert!(!funded(&mut caller,denied,|cx|receiver.prepare_job(job,cx)));
        assert!(funded(&mut caller,stock,|cx|receiver.prepare_job(job,cx)));
        let original=receiver.job_slot(job).unwrap().as_ref().unwrap().kind.as_ptr();
        assert!(funded(&mut caller,stock,|cx|receiver.prepare_job(job,cx)));
        assert_eq!(receiver.job_slot(job).unwrap().as_ref().unwrap().kind.as_ptr(),original);
    }else {
        assert_eq!(final_state,InputAdmission::Failed);
        assert_eq!(receiver.failure_kind(),Some(ValueRefusalKind::InvalidValue));
        assert!(!receiver.input_returned());
    }
    caller.cancel().cancel_now();
    for _ in 0..(if json{1024}else{100}) {
        if receiver.terminal_is_empty(){break;}
        let held=(!receiver.input_returned()).then_some(wire);
        funded(&mut caller,stock,|cx|cx.with_original_retirement(|cx|{
            if receiver.begin_close(held,cx)?{receiver.advance(held,cx)?;}
            Ok(())
        }));
        closed+=caller.receipt().released_bytes;
        opened+=caller.receipt().retained_capacity_bytes;
    }
    assert!(receiver.input_returned()&&receiver.terminal_is_empty());
    assert_eq!(opened,closed);
    caller.retire(stock);
}
