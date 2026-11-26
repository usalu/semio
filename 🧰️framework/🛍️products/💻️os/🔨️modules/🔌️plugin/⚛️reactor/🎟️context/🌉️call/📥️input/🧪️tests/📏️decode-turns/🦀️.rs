//! 📏️ The 681-byte first Poll Input decodes within a pinned ceiling of outer control turns, caller turns and clock reads, and spends the same parse and source work units.
use super::*;
use semio_framework_job::{Generation,OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::RetainedCloneGrant;
use std::collections::BTreeMap;

thread_local! { static CLOCK_READS:std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
fn clock()->Option<u64>{CLOCK_READS.set(CLOCK_READS.get()+1);Some(1)}

/// 🧾️ Rebuilds the exact length of the live Note first Input: the checked-in InstanceOpen fixture under the host's effective semantic limits.
fn note_wire()->Vec<u8>{
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧬️schema/🔁️persistent-local/📊️actor-input.json")).unwrap();
    let mut wire=rows.as_array().unwrap().iter().map(|row|row["wire"].clone()).find(|wire|{let event=&wire["Poll"]["events"][0]["instanceOpen"];event["appId"]=="s.note.note@1/*#editor"&&event["capabilities"].get(0).is_some()}).unwrap();
    wire["Poll"]["limits"]=serde_json::json!([47836160u64,2949668352u64,1946157056u64,1476395008u64,64,979000000u64,1000000u64,86415461134u64,256,1048576,256]);
    wire["Poll"]["events"][0]["instanceOpen"]["actor"]=serde_json::json!("agent:0");
    let base=serde_json::to_vec(&wire).unwrap().len();
    wire["Poll"]["events"][0]["instanceOpen"]["actor"]=serde_json::json!(format!("agent:{}",".".repeat(681-base+1)));
    let bytes=serde_json::to_vec(&wire).unwrap();assert_eq!(bytes.len(),681);bytes
}

#[derive(Default,Debug,PartialEq,Eq,Clone)]
struct Tally{steps:u64,work:u64,exits:u64,clock_reads:u64,receipt:RetainedCloneProgress,fuel:u64,front:u64,phases:BTreeMap<String,(u64,u64)>}

fn label(receiver:&OriginalActorInputReceiving)->String{
    match receiver.decoder.receiver(){None=>"admit".into(),Some(decoder)=>match decoder.grammar_phase(){""=>decoder.phase().into(),grammar=>format!("{}:{}",decoder.phase(),grammar)}}
}

fn drive(receiver:&mut OriginalActorInputReceiving,wire:&[u8],cx:&mut StepContext<'_>,tally:&mut Tally)->Result<InputAdmission,OriginalTurnFailure>{
    tally.exits+=1;
    loop{
        if cx.should_yield()||cx.is_cancelled(){return Ok(InputAdmission::Pending)}
        let before=cx.original_work_units_spent();let from=label(receiver);let mut progress=true;
        let admission=receiver.advance_step((!receiver.input_returned()).then_some(wire),cx,&mut progress)?;
        let spent=u64::try_from(cx.original_work_units_spent()-before).unwrap();let name=format!("{from} > {}",label(receiver));
        tally.steps+=1;tally.work+=spent;if from.starts_with("admit")||from.starts_with("parse"){tally.front+=spent}let row=tally.phases.entry(name).or_default();row.0+=1;row.1+=spent;
        if admission!=InputAdmission::Pending||!progress||spent==0{return Ok(admission)}
    }
}

fn measure(wire:&[u8],grant:RetainedCloneGrant)->(Tally,u64,serde_json::Value,RetainedCloneProgress){
    let stock=RetainedCloneGrant{maximum_items:100_000_000,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:128};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(951),Generation(2),stock,clock,100_000_000,100_000_000);
    caller.retain_dispatch().unwrap();caller.admit_return_table();
    let mut receiver=OriginalActorInputReceiving::new(OriginalNativeDecodePolicy{maximum_owned_bytes:stock.maximum_capacity_bytes},JsonReadLimits{maximum_bytes:1<<24,maximum_allocation_bytes:stock.maximum_capacity_bytes,maximum_depth:64,maximum_items:100_000});
    let mut tally=Tally::default();let mut turns=0u64;let reads=CLOCK_READS.get();
    loop{
        turns+=1;assert!(turns<100_000);let fuel=caller.fuel_remaining();
        let admission=caller.turn(grant,|cx|drive(&mut receiver,wire,cx,&mut tally)).unwrap();
        tally.fuel+=fuel-caller.fuel_remaining();tally.receipt=tally.receipt.checked_add(caller.receipt()).unwrap();
        if admission==InputAdmission::Ready{break}assert_eq!(admission,InputAdmission::Pending);
    }
    let value=serde_json::to_value(receiver.input().unwrap()).unwrap();
    assert!(caller.turn(grant,|cx|cx.with_original_retirement(|cx|receiver.begin_close(None,cx))).unwrap());
    let mut closed=caller.receipt();
    for _ in 0..100_000{
        if receiver.terminal_is_empty(){break}
        caller.turn(grant,|cx|cx.with_original_retirement(|cx|receiver.advance(None,cx))).unwrap();
        closed=closed.checked_add(caller.receipt()).unwrap();
    }
    assert!(receiver.terminal_is_empty());caller.retire(grant);
    tally.clock_reads=CLOCK_READS.get()-reads;
    (tally,turns,value,closed)
}

fn grant_of(row:&serde_json::Value)->RetainedCloneGrant{let field=|key:&str|row[key].as_u64().unwrap()as usize;RetainedCloneGrant{maximum_items:field("items"),maximum_copy_bytes:field("copyBytes"),maximum_capacity_bytes:field("capacityBytes"),maximum_release_bytes:field("releaseBytes"),maximum_depth:field("depth")}}

#[test]
fn canonical_empty_poll_keeps_its_typed_owner_and_paid_close() {
    std::thread::Builder::new().stack_size(64*1024*1024).spawn(|| {
        let rows: serde_json::Value = serde_json::from_str(include_str!("../🧫️empty-poll.json")).unwrap();
        let profile: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let grant = grant_of(&profile["grants"][0]);
        for row in rows.as_array().unwrap().iter().filter(|row| row["canonical"] == true) {
            let wire = row["wire"].as_str().unwrap();
            let padded = format!(" {wire}");
            let expected: serde_json::Value = serde_json::from_str(wire).unwrap();
            let (fast, fast_turns, actual, fast_close) = measure(wire.as_bytes(), grant);
            let (generic, generic_turns, reference, generic_close) = measure(padded.as_bytes(), grant);
            assert_eq!(actual, expected);
            assert_eq!(actual, reference);
            assert_eq!(fast.fuel, fast.work);
            assert!(fast.work >= wire.len() as u64);
            assert!(fast.work < generic.work);
            assert!(fast.receipt.copied_bytes < generic.receipt.copied_bytes);
            assert!(fast.receipt.retained_capacity_bytes < generic.receipt.retained_capacity_bytes);
            assert!(fast_turns <= generic_turns);
            for progress in [fast.receipt.checked_add(fast_close).unwrap(), generic.receipt.checked_add(generic_close).unwrap()] {
                assert_eq!(progress.retained_capacity_bytes, progress.released_bytes);
            }
            eprintln!("[DEBUG] {}: work {} vs {}, capacity {} vs {}, steps {} vs {}", row["name"], fast.work, generic.work, fast.receipt.retained_capacity_bytes, generic.receipt.retained_capacity_bytes, fast.steps, generic.steps);
        }
    }).unwrap().join().unwrap();
}

#[test]
fn ordinary_open_owns_exact_event_and_retires_its_capacity() {
    std::thread::Builder::new().stack_size(64*1024*1024).spawn(|| {
        let cases: serde_json::Value = serde_json::from_str(include_str!("../ordinary-open.json")).unwrap();
        let profile: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let grant = grant_of(&profile["grants"][0]);
        for row in cases.as_array().unwrap().iter().filter(|row| row["canonical"]==true) {
            let wire = row["wire"].as_str().unwrap();
            let (direct, _, event, closed) = measure(wire.as_bytes(), grant);
            let (generic, _, expected, _) = measure(format!(" {wire}").as_bytes(), grant);
            assert_eq!(event, expected);
            assert_eq!(direct.work, direct.fuel);
            assert!(direct.work < generic.work);
            let total = direct.receipt.checked_add(closed).unwrap();
            assert_eq!(total.retained_capacity_bytes, total.released_bytes);
            eprintln!("[DEBUG] {} opening used {} vs {} work units, {} vs {} capacity bytes", row["name"], direct.work, generic.work, direct.receipt.retained_capacity_bytes, generic.receipt.retained_capacity_bytes);
        }
    }).unwrap().join().unwrap();
}

#[test]
fn canonical_lifecycle_ack_keeps_its_owned_event_and_paid_close() {
    std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn(|| {
        let rows: serde_json::Value = serde_json::from_str(include_str!("../🧫️lifecycle-ack.json")).unwrap();
        let profile: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let grant = grant_of(&profile["grants"][0]);
        for row in rows.as_array().unwrap().iter().filter(|row| row["canonical"] == true) {
            let wire = row["wire"].as_str().unwrap();
            let generic_wire = format!(" {wire}");
            let (fast, _, typed, close) = measure(wire.as_bytes(), grant);
            let (generic, _, reference, generic_close) = measure(generic_wire.as_bytes(), grant);
            assert_eq!(typed, reference);
            assert_eq!(fast.work, fast.fuel);
            assert!(fast.work >= wire.len() as u64);
            assert!(fast.work < generic.work && fast.steps < generic.steps);
            assert!(fast.receipt.retained_capacity_bytes < generic.receipt.retained_capacity_bytes);
            for cumulative in [fast.receipt.checked_add(close).unwrap(), generic.receipt.checked_add(generic_close).unwrap()] {
                assert_eq!(cumulative.retained_capacity_bytes, cumulative.released_bytes);
            }
            eprintln!("[DEBUG] {}: work {} vs {}, capacity {} vs {}, steps {} vs {}",
                row["name"], fast.work, generic.work, fast.receipt.retained_capacity_bytes, generic.receipt.retained_capacity_bytes, fast.steps, generic.steps);
        }
    }).unwrap().join().unwrap();
}

#[test]
fn first_note_input_keeps_its_work_and_receipts_while_control_turns_stay_under_their_ceiling(){
    let checked=std::thread::Builder::new().stack_size(64*1024*1024).spawn(||{
        let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let wire=note_wire();assert_eq!(wire.len(),corpus["wireBytes"].as_u64().unwrap()as usize);
        if let Some(repeat)=std::env::var("SEMIO_DECODE_REPEAT").ok().and_then(|value|value.parse::<u32>().ok()){let row=&corpus["grants"][0];for _ in 0..repeat{measure(&wire,grant_of(row));}return}
        for row in corpus["grants"].as_array().unwrap(){
            let name=row["name"].as_str().unwrap();let expected=&row["expected"];
            let(tally,turns,value,_)=measure(&wire,grant_of(row));
            assert_eq!(serde_json::to_vec(&value).unwrap().len(),wire.len(),"{name}: decoded value");
            assert_eq!(tally.fuel,tally.work,"{name}: fuel equals work");
            if let Some(front)=expected["frontWork"].as_u64(){assert_eq!(tally.front,front,"{name}: parse and source work units")}
            assert!(tally.steps<=expected["maxOuterSteps"].as_u64().unwrap(),"{name}: {} outer control turns",tally.steps);
            assert!(turns<=expected["maxCallerTurns"].as_u64().unwrap(),"{name}: {turns} caller turns");
            assert!(tally.clock_reads<=expected["maxClockReads"].as_u64().unwrap(),"{name}: {} clock reads",tally.clock_reads);
        }
    }).unwrap().join();
    if let Err(failure)=checked{std::panic::resume_unwind(failure)}
}
