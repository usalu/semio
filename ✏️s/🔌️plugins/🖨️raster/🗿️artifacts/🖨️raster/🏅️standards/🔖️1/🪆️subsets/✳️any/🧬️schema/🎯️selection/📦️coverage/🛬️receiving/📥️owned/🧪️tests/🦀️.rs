//! 🧪️ Streamed flat DSL rows use original decoder authority through success and cancelled partial pages.
use super::*;
use semio_framework_dsl_record::{DslField,receiving::owned::OwnedAdvanceSelection};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueRefusalKind};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,OriginalTurnPrimary,StepContext};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};

fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096}}
fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128}}
fn retire<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let mut value=ControlledRetirement::retain(value);for _ in 0..100_000{if value.terminal_is_empty(){return;}caller.turn(loan(),|cx|{let demand=value.selected_demand(32768)?;semio_framework_job::close_original_effect_step(cx,demand,|effect|value.step(32768,effect))?;Ok(())}).unwrap();}panic!("packed DSL fixture retirement did not settle");}
/// 🪙️ Runs one local effect-shaped cursor action in the turn's own original effect and settles its progress and performed work once.
fn local_turn<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneProgress,E>)->Result<(),OriginalTurnFailure>{
    let grant=cx.retained_grant();
    let Some((poll,performed))=cx.with_original_effect(grant,|_,effect|run(effect))? else{return Ok(())};
    let turn=match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(Default::default()),work}};
    semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work.max(performed)})?;
    Ok(())
}
/// 🫴️ Advances the nested record child inside the turn's original native receiving scope.
fn nested_turn(cx:&mut StepContext<'_>,cursor:&mut RasterCoverageFieldReceiving,input:&mut Option<FieldValue>)->Result<(),OriginalTurnFailure>{
    let issued=cx.retained_grant();let units=cx.fuel_remaining();let mut scope=None;let mut stepped=None;
    if !cx.with_native_receiving(&mut scope,|control,grant|{stepped=Some(match control.source_witness(){Ok(witness)=>cursor.advance_nested(input,control,witness,32768,units,grant),Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})});Ok(())})?{return Ok(())}
    let refused=|message:&'static str|ValueError::literal(ValueRefusalKind::InvalidValue,message);
    let scope=scope.ok_or_else(||refused("coverage fixture receiving scope vanished"))?;
    if scope.source_refusal.is_some()||scope.call.is_err(){return Err(refused("coverage fixture receiving scope refused").with_retained_progress(scope.retained_progress()).into())}
    let(result,work)=match stepped.ok_or_else(||refused("coverage fixture receiving step vanished"))?{RetainedWorkPoll::Pending{work}=>(Ok(Default::default()),work),RetainedWorkPoll::Complete(turn)=>(turn.result,turn.work)};
    let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
    let receipt=cx.consume_receiving_retained(issued,progress);cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));
    match result{Ok(_)=>{receipt?;Ok(())},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}
}
fn step(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cursor:&mut RasterCoverageFieldReceiving,input:&mut Option<FieldValue>)->Result<(),OriginalTurnFailure>{
    caller.turn_with_decoder(loan(),decoder,|cx|{
        match cursor.select_advance(input,32768,cx.fuel_remaining())?{
            OwnedAdvanceSelection::Local{..}=>local_turn(cx,|effect|cursor.advance_local(input,32768,effect).map_err(OriginalTurnPrimary::Receiving)),
            OwnedAdvanceSelection::Nested=>nested_turn(cx,cursor,input),
        }
    })
}

#[test]
fn packed_owned_flat_wire_preserves_rows_and_cancelled_frontiers(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../fixture.json")).unwrap();
    for count in [0,1,255,256,257]{
        assert_eq!(fixture["pageRows"],RASTER_SELECTION_PAGE_ROWS);
        for cut in [None,Some(0),Some(1),Some(255),Some(1024)]{
            let mut caller=OriginalJobFixtureCaller::admit(OperationId(697),Generation(17),stock(),||Some(1),u64::MAX,1<<24);
            let mut decoder=OriginalJobDecodeOwner::empty();
            let rows=(0..count).map(|index|RasterSelectionSpan{start:index,length:1,coverage:128}).collect::<Vec<_>>();
            let expected=serde_json::to_value(rows.iter().map(|row|(row.start,row.length,row.coverage)).collect::<Vec<_>>()).unwrap();
            let mut input=Some(FieldValue::List(rows.iter().map(DslField::to_value).collect()));
            let mut cursor=RasterSelectionSpans::receiving_cursor();
            let mut turns=0;
            while !cursor.is_ready()&&cut.is_none_or(|cut|turns<cut){step(&mut caller,&mut decoder,&mut cursor,&mut input).unwrap();turns+=1;assert!(turns<100_000);}
            if cut.is_none(){
                let mut output=None;caller.turn(loan(),|cx|local_turn(cx,|effect|cursor.take_into(&mut output,effect))).unwrap();
                let output=output.unwrap();assert_eq!(output.len(),count as usize);let actual=serde_json::to_value(output.iter().map(|row|(row.start,row.length,row.coverage)).collect::<Vec<_>>()).unwrap();assert_eq!(actual,expected);retire(&mut caller,output);
            }else{caller.cancel().cancel_now();}
            for _ in 0..100_000{if cursor.terminal_is_empty(){break;}caller.turn(loan(),|cx|{let demand=cursor.close_demands(32768)?;semio_framework_job::close_original_effect_step(cx,demand,|effect|cursor.close_step(effect).map(RetainedCloneStep::Progress))?;Ok(())}).unwrap();}
            assert!(cursor.terminal_is_empty());
            for _ in 0..100_000{if decoder.terminal_is_empty(){break;}caller.turn(loan(),|cx|{let demand=decoder.retirement_demands(32768)?;decoder.close_on_source(cx)?;Ok(())}).unwrap();}
            assert!(decoder.terminal_is_empty());if let Some(input)=input{retire(&mut caller,input);}caller.retire(stock());
        }
    }
}
