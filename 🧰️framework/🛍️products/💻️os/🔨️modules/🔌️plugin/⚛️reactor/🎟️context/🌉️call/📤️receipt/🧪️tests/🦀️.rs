//! 🧪️ Actual inline scalar receipt bytes preserve failed effects and maximum-width original identities.
use super::*;

#[test]
fn close_output_retires_only_accepted_poll_causes_before_reentry() {
    use semio_framework_actor::ActorFailureCallKind::{AdmitActorInput, CloseActorOutput, Poll};
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/poll-failure-retirement.json")).unwrap();
    let stock = RetainedCloneGrant {
        maximum_items: 65_536,
        maximum_copy_bytes: 16 * 1024 * 1024,
        maximum_capacity_bytes: 16 * 1024 * 1024,
        maximum_release_bytes: 16 * 1024 * 1024,
        maximum_depth: 64,
    };
    for (index, row) in cases.as_array().unwrap().iter().enumerate() {
        let kind = match row["kind"].as_str().unwrap() {
            "Poll" => Poll,
            "AdmitActorInput" => AdmitActorInput,
            _ => unreachable!(),
        };
        let accepted = row["accepted"].as_bool().unwrap();
        let failure_frame_released = row["frameReleased"].as_bool().unwrap();
        let operation = 9230 + index as u64;
        let mut input = RetainedTurnInput {
            operation, generation: 1, epoch: 1, grant: stock,
            work: semio_framework_actor::OriginalActorTurnWork {
                clock_binding: operation, absolute_deadline_us: u64::MAX, fuel: 1_000_000,
            },
        };
        let clock = || Some(1);
        let budget = |input: RetainedTurnInput| super::super::StepBudget::new(input.work.fuel, input.work.absolute_deadline_us, input.grant);
        let settle = |input: &mut RetainedTurnInput, spent, work| {
            let returned = input.return_original(spent, work).unwrap();
            input.epoch += 1;
            input.grant = returned.remaining;
            input.work.fuel = returned.remaining_fuel;
        };
        let mut entry = super::super::OriginalActorGuestEntry::new();
        for _ in 0..64 {
            if entry.context.is_ready() { break; }
            let call = entry.admit_step(input, budget(input), clock);
            assert!(call.result.is_ok());
            settle(&mut input, call.receipt_spent, call.work_units_spent);
        }
        assert!(entry.context.is_ready());
        entry.context.begin_pending_context(input).unwrap();
        let address = &entry as *const _ as usize;
        let cause = &*entry.value_fault as *const _ as usize;
        let mut debit = RetainedCloneProgress::default();
        let mut origin = None;
        let call = entry.context.dispatch_core_wire_step(input, budget(input), &mut debit, clock, true, |cx| {
            cx.admit_original_escrow_publisher()?;
            origin = Some(OriginalActorFailureOrigin::borrow(input, kind, address, cause, cx)?);
            Ok::<(), OriginalTurnFailure>(())
        });
        call.result.unwrap();
        settle(&mut input, call.receipt_spent, call.work_units_spent);
        *entry.value_fault = Some(ValueError::literal(ValueRefusalKind::InvalidValue, "retained test cause").into());
        entry.failure_origin = origin;
        if accepted {
            let reference = entry.failure_origin.unwrap().reference();
            for _ in 0..2 {
                let mut debit = RetainedCloneProgress::default();
                let call = entry.context.dispatch_core_wire_step(input, budget(input), &mut debit, clock, false, |cx| {
                    let mut body = OriginalHeaderFailureBody {
                        value_fault: entry.value_fault.as_ref(), cleanup_fault: None, origin: &mut entry.failure_origin,
                        cleanup_origin: &mut entry.cleanup_origin, input_identity: None, entry: address,
                        value_address: cause, cleanup_address: 0, value_publication: &mut entry.value_publication,
                    };
                    cx.with_original_retirement(|cx| body.acknowledge_publication_step(reference, cx))
                });
                call.result.unwrap();
                settle(&mut input, call.receipt_spent, call.work_units_spent);
            }
        }
        let requires_retirement = retires_poll_failure(CloseActorOutput, entry.failure_origin.map(|origin| origin.kind));
        let mut advance_close = !requires_retirement;
        let mode = HeaderMode::CloseOutput { failure_frame_released };
        if requires_retirement && mode.poll_frame_released() {
            for _ in 0..64 {
                let mut debit = RetainedCloneProgress::default();
                let call = entry.context.dispatch_core_wire_step(input, budget(input), &mut debit, clock, false, |cx| {
                    let before = cx.retained_progress();
                    let result = cx.with_original_retirement(|cx| close_accepted_failure_causes_step(address, &mut entry.value_fault, &mut entry.value_retirement, &mut entry.cleanup_fault, &mut entry.cleanup_fault_retirement, &mut entry.value_publication, &mut entry.failure_origin, &mut entry.cleanup_origin, None, cx));
                    if !accepted { assert_eq!(cx.retained_progress(), before); }
                    result
                });
                advance_close = call.result.unwrap();
                settle(&mut input, call.receipt_spent, call.work_units_spent);
                if !accepted || advance_close { break; }
            }
        }
        assert_eq!(Some(advance_close), row["advanceClose"].as_bool());
        assert_eq!(entry.failure_origin.is_none(), requires_retirement && accepted && failure_frame_released);
        assert_eq!(entry.value_fault.is_none(), requires_retirement && accepted && failure_frame_released);
        if let Some(origin) = entry.failure_origin.as_mut() { origin.accepted = true; }
        for _ in 0..1024 {
            if entry.terminal_is_empty() { break; }
            let call = entry.close_plugin_step(semio_framework_actor::OriginalJobBudget {
                input, retained_policy: input.grant, fuel: input.work.fuel,
                deadline_us: input.work.absolute_deadline_us, cancelled: true,
            });
            assert!(call.result.is_ok());
            settle(&mut input, call.receipt_spent, call.work_units_spent);
        }
        assert!(entry.terminal_is_empty());
    }
}

#[test]
fn fixed_receipt_prices_this_dispatch_against_its_resident_baseline() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/dispatch-receipt.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let read = |field: &str| case[field].as_u64().unwrap() as usize;
        let axis = read("axis");
        let progress = |amount| {
            let mut fields = [0; 4];
            fields[axis] = amount;
            RetainedCloneProgress {
                copied_items: fields[0],
                copied_bytes: fields[1],
                retained_capacity_bytes: fields[2],
                released_bytes: fields[3],
            }
        };
        let aggregate = dispatch_retained_progress(progress(read("baseline")), progress(read("observed")))
            .and_then(|receipt| receipt.checked_add(progress(read("completion"))))
            .and_then(|receipt| receipt.checked_add(progress(read("carry"))))
            .ok().map(|receipt| receipt.wire_fields()[axis].1 as u64);
        assert_eq!(aggregate, case["aggregate"].as_u64(), "{}", case["name"]);
        assert_eq!(aggregate.and_then(|amount| amount.checked_sub(read("redeemed") as u64)), case["fresh"].as_u64(), "{}", case["name"]);
    }
}

#[test]
fn first_response_quote_refuses_an_insufficient_scope_before_source_or_producer_effects() {
    let law: serde_json::Value = serde_json::from_str(include_str!("first-response.json")).unwrap();
    let required_items = law["required"]["items"].as_u64().unwrap() as usize;
    let required_work = law["required"]["work"].as_u64().unwrap();
    let stock = RetainedCloneGrant {
        maximum_items: 128,
        maximum_copy_bytes: 16 * 1024 * 1024,
        maximum_capacity_bytes: 16 * 1024 * 1024,
        maximum_release_bytes: 16 * 1024 * 1024,
        maximum_depth: 64,
    };
    for length in [144, 152, 184] {
        let (items, demand, work) = super::super::original_core_first_response_reserve(length).unwrap();
        assert_eq!((items, work), (required_items, required_work));
        assert!(demand.copy_bytes < stock.maximum_copy_bytes);
        assert!(demand.capacity_bytes < stock.maximum_capacity_bytes);
        assert_eq!(demand.depth, 1);
    }
    let input = RetainedTurnInput {
        operation: 1,
        generation: 1,
        epoch: 1,
        work: semio_framework_actor::OriginalActorTurnWork {
            clock_binding: 1,
            absolute_deadline_us: u64::MAX,
            fuel: 10000,
        },
        grant: stock,
    };
    for row in law["cases"].as_array().unwrap() {
        let items = row["items"].as_u64().unwrap() as usize;
        let work = row["work"].as_u64().unwrap();
        let accepted = items >= required_items && work >= required_work;
        assert_eq!(Some(accepted), row["accepted"].as_bool());
        if accepted { continue; }
        let mut entry = super::super::OriginalActorGuestEntry::new();
        let budget = super::super::StepBudget::new(work, u64::MAX, RetainedCloneGrant { maximum_items: items, ..stock });
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
            entry.dispatch_poll_header_step(input, budget, || Some(1), |_| panic!("underfunded first response cannot enter its producer"))
        });
        let kind = if items < required_items { ValueRefusalKind::OwnershipLimit } else { ValueRefusalKind::WorkLimit };
        assert_eq!(result.unwrap_err().kind, kind);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(entry.terminal_is_empty());
    }
}
fn escrow()->OriginalEscrowExtent{OriginalEscrowExtent{pointer:3000,length:48}}
#[test]
fn typed_actor_and_turn_codecs_emit_exact_sequential_work(){struct Sink{bytes:Vec<u8>,writes:usize}impl OriginalEscrowHeaderWrite for Sink{fn write(&mut self,bytes:&[u8])->Result<(),ValueError>{self.writes+=1;self.bytes.extend_from_slice(bytes);Ok(())}}let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let bytes:Vec<u8>=fixture["failureHex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect();let(base,_)=OriginalActorFixedReceipt::read(&bytes).unwrap();let mut actor=Sink{bytes:Vec::new(),writes:0};OriginalActorFixedReceipt::write_to(&mut actor,base,escrow()).unwrap();assert_eq!(actor.bytes,bytes);assert_eq!(actor.writes,20);let call=OriginalActorFixedCall{kind:OriginalActorFixedKind::TurnOutput{ready:true,captured:true,input_pending:true,pointer:42,length:7},..base};let mut turn=Sink{bytes:Vec::new(),writes:0};OriginalActorTurnOutputReceipt::write_to(&mut turn,call,escrow()).unwrap();assert_eq!(turn.writes,22);assert_eq!(OriginalActorFixedReceipt::read(&turn.bytes).unwrap(),(call,escrow()));}
#[test]
fn escrow_snapshot_preserves_real_origin_and_rejects_unsettled_tombstones(){
 let fixture:serde_json::Value=serde_json::from_str::<serde_json::Value>(include_str!("shared.json")).unwrap()["escrow"].clone();let rows=fixture["rows"].as_array().unwrap();let mut bytes=vec![0u8;48+rows.len()*184];bytes[..4].copy_from_slice(b"SRE1");bytes[4..8].copy_from_slice(&(rows.len()as u32).to_le_bytes());for(i,value)in [101u64,1,202,fixture["revision"].as_u64().unwrap(),6].into_iter().enumerate(){bytes[8+i*8..16+i*8].copy_from_slice(&value.to_le_bytes())}for(i,row)in rows.iter().enumerate(){for(j,value)in row.as_array().unwrap().iter().enumerate(){bytes[48+i*184+j*8..56+i*184+j*8].copy_from_slice(&value.as_u64().unwrap().to_le_bytes())}}
 let snapshot=OriginalEscrowSnapshot::read(&bytes).unwrap();assert_eq!(snapshot.len(),2);assert_eq!(snapshot.source_address,101);assert_eq!(snapshot.acknowledged_revision,6);assert_eq!(snapshot.row_at(0).unwrap().origin_epoch,5);assert_eq!(snapshot.row_at(1).unwrap().serial,2);assert!(snapshot.row_at(2).is_none());let mut invalid=bytes.clone();invalid[32..40].fill(0);assert!(OriginalEscrowSnapshot::read(&invalid).is_err());let mut invalid=bytes.clone();invalid[40..48].copy_from_slice(&7u64.to_le_bytes());assert!(OriginalEscrowSnapshot::read(&invalid).is_err());let mut invalid=bytes.clone();invalid[8..16].fill(0);assert!(OriginalEscrowSnapshot::read(&invalid).is_err());let mut invalid=bytes.clone();invalid[48..56].fill(0);assert!(OriginalEscrowSnapshot::read(&invalid).is_err());let mut invalid=bytes.clone();invalid[48+6*8..56+6*8].fill(0);assert!(OriginalEscrowSnapshot::read(&invalid).is_err());let mut invalid=bytes.clone();invalid[48+184+13*8..56+184+13*8].fill(0);assert!(OriginalEscrowSnapshot::read(&invalid).is_err());
}
#[test]
fn turn_output_receipt_round_trips_and_requires_exact_handback(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let bytes:Vec<u8>=fixture["failureHex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect();let (base,_)=OriginalActorFixedReceipt::read(&bytes).unwrap();let mut receipt=OriginalActorTurnOutputReceipt::empty();
 for input_pending in [false,true]{for kind in [OriginalActorFixedKind::TurnOutput{ready:false,captured:false,input_pending,pointer:0,length:0},OriginalActorFixedKind::TurnOutput{ready:false,captured:true,input_pending,pointer:0,length:0},OriginalActorFixedKind::TurnOutput{ready:true,captured:true,input_pending,pointer:42,length:7}]{let call=OriginalActorFixedCall{kind,..base};let wire=receipt.produce(call,escrow()).unwrap();let pointer=wire.as_ptr()as usize;assert_eq!(OriginalActorFixedReceipt::read(wire).unwrap(),(call,escrow()));assert!(receipt.is_borrowed());assert!(receipt.produce(call,escrow()).is_err());assert!(!receipt.hand_back(pointer+1,152));assert!(!receipt.hand_back(pointer,136));assert!(receipt.hand_back(pointer,152));}}
 let call=OriginalActorFixedCall{kind:OriginalActorFixedKind::TurnOutput{ready:true,captured:true,input_pending:false,pointer:42,length:7},..base};let wire=receipt.produce(call,escrow()).unwrap();let pointer=wire.as_ptr()as usize;let mut invalid=*wire;invalid[5]=1;assert!(OriginalActorFixedReceipt::read(&invalid).is_err());assert!(receipt.hand_back(pointer,152));
 assert!(receipt.produce(call,OriginalEscrowExtent{pointer:3000,length:16}).is_err());assert!(receipt.produce(call,OriginalEscrowExtent{pointer:3000,length:49}).is_err());assert!(receipt.produce(call,OriginalEscrowExtent{pointer:3000,length:4536}).is_err());assert!(receipt.produce(call,OriginalEscrowExtent{pointer:0,length:48}).is_err());
 for kind in [OriginalActorFixedKind::TurnOutput{ready:true,captured:false,input_pending:false,pointer:42,length:7},OriginalActorFixedKind::TurnOutput{ready:false,captured:false,input_pending:false,pointer:1,length:0},OriginalActorFixedKind::TurnOutput{ready:true,captured:true,input_pending:false,pointer:0,length:7},OriginalActorFixedKind::TurnOutput{ready:true,captured:true,input_pending:false,pointer:1,length:0}]{assert!(receipt.produce(OriginalActorFixedCall{kind,..base},escrow()).is_err());}
}
#[test]
fn original_input_admission_requires_actual_return_proof_without_new_frame_storage(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let expected=fixture["failureHex"].as_str().unwrap();let mut bytes=[0u8;144];for(index,pair)in expected.as_bytes().chunks_exact(2).enumerate(){bytes[index]=u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()}bytes[4]=4;bytes[6]=0;for flags in 0..8{bytes[5]=flags;let(result,heap)=observe(||OriginalActorFixedReceipt::read(&bytes));assert_eq!(heap,(0,0));assert_eq!(result.is_ok(),fixture["inputAdmissionFlags"][flags as usize].as_bool().unwrap());if let Ok((call,escrow))=result{assert_eq!(call.kind,OriginalActorFixedKind::InputAdmission{ready:flags&1!=0,input_returned:flags&2!=0,refusal:None});let mut receipt=OriginalActorFixedReceipt::empty();let(_,heap)=observe(||receipt.write(call,escrow));assert_eq!(heap,(0,0));assert_eq!(receipt.bytes(),&bytes);}}}
#[test]
fn original_actor_receipt_loan_refuses_body_before_exact_resident_handback(){
 let fixture:serde_json::Value=serde_json::from_str::<serde_json::Value>(include_str!("shared.json")).unwrap()["lifecycle"].clone();let bytes:Vec<u8>=serde_json::from_str::<serde_json::Value>(include_str!("🧫️fixtures/🔣️.json")).unwrap()["failureHex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect();let (original,_)=OriginalActorFixedReceipt::read(&bytes).unwrap();let mut receipt=OriginalActorFixedReceipt::empty();let pointer=receipt.bytes().as_ptr()as usize;let mut bodies=0;
 for event in fixture["events"].as_array().unwrap(){let offset=event["offset"].as_i64().unwrap();let length=event["length"].as_u64().unwrap()as usize;let((accepted,known),heap)=observe(||{let accepted=if event["operation"]=="produce"{receipt.produce(escrow(),||{bodies+=1;original}).is_ok()}else{receipt.hand_back(pointer.wrapping_add_signed(offset as isize),length)};(accepted,receipt.owns_pointer(pointer.wrapping_add_signed(offset as isize)))});assert_eq!(heap,(0,0));assert_eq!(accepted,event["accepted"].as_bool().unwrap());assert_eq!(known,event["known"].as_bool().unwrap());assert_eq!(bodies,event["bodyCalls"].as_u64().unwrap());assert_eq!(receipt.bytes().as_ptr()as usize,pointer)}assert!(!receipt.is_borrowed());assert_eq!(observe(||drop(receipt)).1,(0,0));
}
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
#[test]
fn original_actor_fixed_native_receipt_preserves_raw_spend_and_all_u64_axes_without_heap(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let expected=fixture["failureHex"].as_str().unwrap();let expected:Vec<u8>=(0..expected.len()).step_by(2).map(|index|u8::from_str_radix(&expected[index..index+2],16).unwrap()).collect();
 let original=OriginalActorFixedCall{input:RetainedTurnInput{operation:71,generation:3,epoch:24,work:semio_framework_actor::OriginalActorTurnWork{clock_binding:711,absolute_deadline_us:1500,fuel:65536},grant:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:512,maximum_release_bytes:512,maximum_depth:8}},spent:RetainedCloneProgress{copied_items:1,copied_bytes:7,..Default::default()},work_units_spent:4,kind:OriginalActorFixedKind::Failure{authority:None}};
 let(mut wire,heap)=observe(OriginalActorFixedReceipt::empty);assert_eq!(heap,(0,0));let backing=wire.bytes().as_ptr();let(_,heap)=observe(||wire.write(original,escrow()).unwrap());assert_eq!(heap,(0,0));assert_eq!(wire.bytes().as_slice(),expected);let(decoded,heap)=observe(||OriginalActorFixedReceipt::read(wire.bytes()));assert_eq!(heap,(0,0));assert_eq!(decoded.unwrap(),(original,escrow()));assert_eq!(wire.bytes().as_ptr(),backing);
 for kind in [None,Some(ValueRefusalKind::InvalidValue),Some(ValueRefusalKind::Canceled),Some(ValueRefusalKind::OwnershipLimit),Some(ValueRefusalKind::AllocationFailed),Some(ValueRefusalKind::WorkLimit),Some(ValueRefusalKind::DepthLimit),Some(ValueRefusalKind::UnsupportedOwner),Some(ValueRefusalKind::InvariantViolated)]{for variant in [OriginalActorFixedKind::Failure{authority:kind},OriginalActorFixedKind::Admission{ready:false,refusal:kind},OriginalActorFixedKind::Close{complete:false,blocked:false,refusal:kind}]{let actual=OriginalActorFixedCall{kind:variant,..original};let(decoded,heap)=observe(||{wire.write(actual,escrow()).unwrap();OriginalActorFixedReceipt::read(wire.bytes())});assert_eq!(heap,(0,0));assert_eq!(decoded.unwrap(),(actual,escrow()));}}
 let mut excessive=original;excessive.spent.copied_items=9;excessive.spent.copied_bytes=8192;let(decoded,heap)=observe(||{wire.write(excessive,escrow()).unwrap();OriginalActorFixedReceipt::read(wire.bytes())});assert_eq!(heap,(0,0));assert_eq!(decoded.unwrap(),(excessive,escrow()));assert!(excessive.input.return_original(excessive.spent,excessive.work_units_spent).is_err());
 let maximum=OriginalActorFixedCall{input:RetainedTurnInput{operation:u64::MAX,generation:u64::MAX,epoch:u64::MAX,work:semio_framework_actor::OriginalActorTurnWork{clock_binding:u64::MAX,absolute_deadline_us:u64::MAX,fuel:u64::MAX},grant:RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX}},spent:RetainedCloneProgress{copied_items:usize::MAX,copied_bytes:usize::MAX,retained_capacity_bytes:usize::MAX,released_bytes:usize::MAX},work_units_spent:u64::MAX,..original};let(decoded,heap)=observe(||{wire.write(maximum,escrow()).unwrap();OriginalActorFixedReceipt::read(wire.bytes())});assert_eq!(heap,(0,0));assert_eq!(decoded.unwrap(),(maximum,escrow()));assert_eq!(&wire.bytes()[8..32],&[255;24]);
 for(index,value)in[(0,0),(4,5),(5,3),(6,9),(7,1)]{let mut invalid=*wire.bytes();invalid[index]=value;let(result,heap)=observe(||OriginalActorFixedReceipt::read(&invalid));assert_eq!(heap,(0,0));assert!(result.is_err());}let(result,heap)=observe(||OriginalActorFixedReceipt::read(&wire.bytes()[..103]));assert_eq!(heap,(0,0));assert!(result.is_err());assert_eq!(observe(||drop(wire)).1,(0,0));
}
