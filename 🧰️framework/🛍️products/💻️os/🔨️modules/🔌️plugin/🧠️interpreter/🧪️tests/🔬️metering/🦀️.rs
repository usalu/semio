use semio_plugin_interpreter::{CoreInstanceOpening,CoreModule,CoreOriginalInvocationStatus,Value};
use semio_framework_actor::{OriginalGuestExecutionBudget,RetainedTurnInput};
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
use semio_framework_test_fixtures::fixture_text;
use std::sync::Arc;

fn clock()->Option<u64>{Some(1000)}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:10_000,maximum_copy_bytes:2_000_000,maximum_capacity_bytes:2_000_000,maximum_release_bytes:2_000_000,maximum_depth:64}}

struct Measured{ledger:String,result:i32,instructions:u64,work:u128,turns:u64,receipt:RetainedCloneProgress}

fn measure(bytes:&[u8],turn_fuel:Option<u64>)->Measured{
    let module=Arc::new(CoreModule::parse(bytes).unwrap());
    let stock=stock();
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(197),Generation(1),stock,clock,1_000_000,1_000_000);
    let mut epoch=1;
    let mut owner=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;CoreInstanceOpening::begin(&module,cx)}).unwrap().unwrap();
    while !owner.resources_ready(){
        epoch+=1;
        caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.advance(&module,cx)}).unwrap();
    }
    epoch+=1;
    assert!(caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.begin_function_step(&module,0,&[],cx)}).unwrap());
    if let Some(fuel)=turn_fuel{caller.renew_each_turn(fuel,1_000_000);}
    let mut measured=Measured{ledger:String::new(),result:0,instructions:0,work:0,turns:0,receipt:RetainedCloneProgress::default()};
    for _ in 0..10_000{
        epoch+=1;
        let (call,work)=caller.turn(stock,|cx|{
            cx.bind_original_source_epoch(epoch)?;
            let input=RetainedTurnInput{grant:cx.retained_grant(),..RetainedTurnInput::borrow_original(cx)?};
            let before=cx.original_work_units_spent();
            let call=owner.advance_scalar(&module,OriginalGuestExecutionBudget{input,fuel:10_000,deadline_us:cx.deadline_us()},cx)?;
            Ok((call,cx.original_work_units_spent()-before))
        }).unwrap();
        let spent=call.execution.spent_fuel.unwrap();
        if let Some(fuel)=turn_fuel{assert!(spent<=fuel);}
        measured.instructions+=spent;
        measured.work+=work;
        measured.turns+=1;
        measured.receipt=measured.receipt.checked_add(caller.receipt()).unwrap();
        if call.result.unwrap()==CoreOriginalInvocationStatus::Complete{break}
    }
    let Some([Value::I32(result)])=owner.invocation_values()else{panic!("the invocation must complete with one i32")};
    measured.result=*result;
    measured.ledger=owner.retained_ledger_text();
    caller.cancel().cancel_now();
    for _ in 0..20_000{
        if owner.terminal_is_empty(){break}
        epoch+=1;
        caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.close_step(&module,cx)}).unwrap();
    }
    assert!(owner.terminal_is_empty());
    caller.retire(stock);
    measured
}

fn hex(text:&str)->Vec<u8>{text.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect()}

/// 📏️ Every executed instruction pays one fuel and one work unit; only real copies, allocations and releases pay retained items and copy bytes, and Wasmtime agrees on every result.
#[test]
fn executed_instruction_kinds_pay_the_declared_items_copy_bytes_and_fuel(){
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🏗️instance/▶️invocation/📏️metering/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let name=row["name"].as_str().unwrap();
        let bytes=hex(row["moduleHex"].as_str().unwrap());
        let engine=wasmtime::Engine::default();
        let oracle=wasmtime::Module::new(&engine,&bytes).unwrap();
        let mut store=wasmtime::Store::new(&engine,());
        let instance=wasmtime::Instance::new(&mut store,&oracle,&[]).unwrap();
        let expected=instance.get_typed_func::<(),i32>(&mut store,"run").unwrap().call(&mut store,()).unwrap();
        assert_eq!(i64::from(expected),row["expected"].as_i64().unwrap(),"{name}");
        let retained=row["retained"].as_array().unwrap();
        assert_eq!(retained.len() as u64,row["items"].as_u64().unwrap(),"{name}");
        assert_eq!(retained.iter().map(|effect|effect["bytes"].as_u64().unwrap()).sum::<u64>(),row["copyBytes"].as_u64().unwrap(),"{name}");
        assert_eq!(row["kinds"].as_array().unwrap().len() as u64,row["instructions"].as_u64().unwrap(),"{name}");
        let measured=measure(&bytes,None);
        assert_eq!(measured.result,expected,"{name}");
        assert_eq!(measured.instructions,row["instructions"].as_u64().unwrap(),"{name}: execution fuel");
        let peak=row["stackPeak"].as_u64().unwrap();
        let growths=u64::from(64-(peak-1).leading_zeros())+1;
        let relocated=(1u64<<(growths-1))-1;
        let labels=row["labelVectors"].as_u64().unwrap();
        assert_eq!(measured.receipt.copied_items as u64,row["items"].as_u64().unwrap()+growths+labels,"{name}: retained items");
        assert_eq!(measured.receipt.copied_bytes as u64,row["copyBytes"].as_u64().unwrap()+relocated*std::mem::size_of::<Value>() as u64,"{name}: copy bytes");
        assert!(measured.receipt.retained_capacity_bytes>0,"{name}: operand stack birth");
        let released=row["retained"].as_array().unwrap().iter().any(|effect|effect["kind"]=="data.drop");
        assert_eq!(measured.receipt.released_bytes as u64,relocated*std::mem::size_of::<Value>() as u64+if released{4}else{0},"{name}: release");
        assert!(measured.work>=u128::from(measured.instructions)+u128::from(row["workUnits"].as_u64().unwrap()),"{name}: work covers instructions and moved bytes");
    }
}

/// 🏁️ Scalar work keeps its yield cadence: a per-turn work issue of three units executes three instructions per turn and still posts no retained item.
#[test]
fn scalar_work_yields_at_the_issued_cadence_without_retained_items(){
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🏗️instance/▶️invocation/📏️metering/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let row=&fixture["cases"][0];
    let bytes=hex(row["moduleHex"].as_str().unwrap());
    let unbounded=measure(&bytes,None);
    let paced=measure(&bytes,Some(3));
    assert_eq!((paced.result,paced.instructions,paced.work),(unbounded.result,unbounded.instructions,unbounded.work));
    assert!(paced.turns>=paced.instructions.div_ceil(3));
    assert!(paced.turns>unbounded.turns);
    assert_eq!(paced.receipt,unbounded.receipt);
}

/// 🏊️ Sequential calls birth and release their frame storage once: fifty calls post exactly the capacity, release and copy-free births of one call, because every returned frame recycles into the instance pool.
#[test]
fn sequential_calls_recycle_frame_storage_inside_the_instance(){
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🏗️instance/▶️invocation/📏️metering/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let calls=fixture["calls"].as_array().unwrap();
    let measured=calls.iter().map(|row|{
        let bytes=hex(row["moduleHex"].as_str().unwrap());
        let engine=wasmtime::Engine::default();
        let oracle=wasmtime::Module::new(&engine,&bytes).unwrap();
        let mut store=wasmtime::Store::new(&engine,());
        let instance=wasmtime::Instance::new(&mut store,&oracle,&[]).unwrap();
        let expected=instance.get_typed_func::<(),i32>(&mut store,"run").unwrap().call(&mut store,()).unwrap();
        assert_eq!(i64::from(expected),row["expected"].as_i64().unwrap());
        let measured=measure(&bytes,None);
        assert_eq!(measured.result,expected);
        measured
    }).collect::<Vec<_>>();
    let (one,many)=(&measured[0],&measured[measured.len()-1]);
    assert!(one.receipt.retained_capacity_bytes>0);
    assert_eq!(many.receipt.retained_capacity_bytes,one.receipt.retained_capacity_bytes);
    assert_eq!(many.receipt.released_bytes,one.receipt.released_bytes);
    assert!(many.instructions>one.instructions);
    assert!(one.ledger.contains("frame births=")&&one.ledger.contains("operand-stack growth=")&&one.ledger.contains("total="),"{}",one.ledger);
    assert_eq!(one.ledger,many.ledger);
}

fn returned(bytes:&[u8],filled:bool)->(RetirementDemand,([usize;3],bool),usize){
    let module=Arc::new(CoreModule::parse(bytes).unwrap());
    let stock=RetainedCloneGrant{maximum_items:200_000,maximum_copy_bytes:100_000_000,maximum_release_bytes:1_000_000_000,..stock()};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(198),Generation(1),stock,clock,1_000_000,1_000_000);
    let mut epoch=1;
    let mut owner=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;CoreInstanceOpening::begin(&module,cx)}).unwrap().unwrap();
    while !owner.resources_ready(){epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.advance(&module,cx)}).unwrap();}
    epoch+=1;
    assert!(caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.begin_function_step(&module,0,&[],cx)}).unwrap());
    caller.renew_each_turn(1,1_000_000);
    let mut demand=None;
    let mut released=0;
    for _ in 0..1000{
        epoch+=1;
        let (call,quote)=caller.turn(stock,|cx|{
            cx.bind_original_source_epoch(epoch)?;
            let input=RetainedTurnInput{grant:cx.retained_grant(),..RetainedTurnInput::borrow_original(cx)?};
            let call=owner.advance_scalar(&module,OriginalGuestExecutionBudget{input,fuel:1,deadline_us:cx.deadline_us()},cx)?;
            let quote=if demand.is_none(){owner.returned_frame_demand(&module,cx)?}else{None};
            Ok((call,quote))
        }).unwrap();
        released+=caller.receipt().released_bytes;
        if let Some(quote)=quote{
            if filled{owner.testkit_fill_pool();}
            epoch+=1;
            demand=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.returned_frame_demand(&module,cx)}).unwrap();
            assert_eq!(quote.copy_bytes,0);
        }
        if call.result.unwrap()==CoreOriginalInvocationStatus::Complete{break}
    }
    let shape=owner.resources().unwrap().testkit_pool_shape();
    caller.cancel().cancel_now();
    for _ in 0..100_000{if owner.terminal_is_empty(){break}epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.close_step(&module,cx)}).unwrap();}
    assert!(owner.terminal_is_empty());
    caller.retire(stock);
    (demand.expect("a returned callee quotes its next retirement"),shape,released)
}

/// 🏊️ A returned frame quotes no release while the bounded pool has room, recycles its empty vectors into it, and quotes the exact release once the pool is full.
#[test]
fn returned_frame_quotes_recycling_while_the_pool_has_room_and_release_when_full(){
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🏗️instance/▶️invocation/📏️metering/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let bytes=hex(fixture["calls"][0]["moduleHex"].as_str().unwrap());
    let (room,pooled,room_released)=returned(&bytes,false);
    let (full,saturated,full_released)=returned(&bytes,true);
    assert_eq!(room.release_bytes,0);
    assert!(full.release_bytes>0);
    assert_eq!(room.copy_bytes,0);
    assert!(full.copy_bytes>0);
    assert_eq!(pooled,([1,1,4],true));
    assert!(saturated.1);
    assert!(full_released>room_released);
}

/// 📏️ A large guest-internal fill posts no copy ledger, costs work proportional to its bytes, and a small per-turn work issue splits it across turns without changing its result.
#[test]
fn guest_internal_moves_cost_work_in_proportion_and_stay_interruptible(){
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🏗️instance/▶️invocation/📏️metering/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let divisor=fixture["workBytesPerUnit"].as_u64().unwrap();
    let row=fixture["cases"].as_array().unwrap().iter().find(|row|row["moved"].as_array().is_some_and(|moved|moved.iter().any(|move_|move_["bytes"]==60000))).unwrap();
    let bytes=hex(row["moduleHex"].as_str().unwrap());
    let whole=measure(&bytes,None);
    let paced=measure(&bytes,Some(8));
    assert_eq!(whole.result,7);
    assert_eq!((paced.result,paced.receipt),(whole.result,whole.receipt));
    assert!(whole.work>=u128::from(60000/divisor));
    assert!(paced.turns>=whole.turns+60000/(8*divisor));
}

const NESTED_BRANCH_VECTOR:&str="0061736d01000000011b056000017f60017f017f60027e7d017e60017c017c60027f7f017f030908000101020301040005030100010707010372756e00000a940308410041031001410510026a4207430000a03f1003a76a4400000000000010401004aa6a410610056a4102410210066a410010016a412810056a410910026a10076a0b4503017e017c017f41e400027f41014102027f4103410420000d011a1a41050b6a6a0b6a410a027f027f027f4107410820000e03000102020b1a410b0c010b410c6a0b6a6a0b3303017e017d017f4100210302400340200320006a2103200041016b22000d000b0b20034102027f41284129412a0c000b6a6a0b2502017c017e200042037e21032001bb440000000000000440a0210220032002aab7aaac7c0b1e02027f017e4103210142052103200044000000000000f83fa22001b7a00b1500200045047f4100052000200041016b10056a0b0b1c01037e20012000027f4109410820002001460d001a1a41010b6a6a0b5e01027f024003402000200041076c3a0000200041016a220041c002480d000b0b41e400410041c801fc0a00004100413241fa01fc0a000041002100024003402001411f6c20002d00006a2101200041016a220041c002480d000b0b20010b";

/// 🧬️ Nested blocks, loops, `br_if` and `br_table` moving results over extra operands, recursion and sequential calls whose locals change type, and overlapping `memory.copy` in both directions agree with Wasmtime whatever the turn pacing.
#[test]
fn nested_branches_calls_and_overlapping_copies_match_wasmtime_at_every_pacing(){
    let bytes=hex(NESTED_BRANCH_VECTOR);
    let engine=wasmtime::Engine::default();
    let oracle=wasmtime::Module::new(&engine,&bytes).unwrap();
    let mut store=wasmtime::Store::new(&engine,());
    let instance=wasmtime::Instance::new(&mut store,&oracle,&[]).unwrap();
    let expected=instance.get_typed_func::<(),i32>(&mut store,"run").unwrap().call(&mut store,()).unwrap();
    let whole=measure(&bytes,None);
    assert_eq!(whole.result,expected);
    for fuel in [2,3,7,64]{
        let paced=measure(&bytes,Some(fuel));
        assert_eq!((paced.result,paced.instructions,paced.receipt),(expected,whole.instructions,whole.receipt),"fuel {fuel}");
        assert_eq!(paced.ledger,whole.ledger,"fuel {fuel}");
    }
}
