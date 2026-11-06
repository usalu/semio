//! 🧪️ Streamed flat DSL rows use original decoder authority through success and cancelled partial pages.
use super::*;
use semio_framework_dsl_record::DslField;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};

fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096}}
fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128}}
fn retire<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let mut value=ControlledRetirement::retain(value);for _ in 0..100_000{if value.terminal_is_empty(){return;}caller.turn(loan(),|cx|{let demand=value.selected_demand(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=value.step(grant);RetainedWorkOutcome{result,work:value.last_turn_work()}})?;Ok(())}).unwrap();}panic!("packed DSL fixture retirement did not settle");}
fn step(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cursor:&mut RasterCoverageFieldReceiving,input:&mut Option<FieldValue>)->Result<(),OriginalTurnFailure>{
    caller.turn_with_decoder(loan(),decoder,|cx|{let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|native|{let turn=cursor.step(input,native,32768,issued);work=turn.work;turn.result});let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};let receipt=cx.consume_receiving_retained(issued,progress);cx.consume_fuel(work);match result{Ok(_)=>{receipt?;Ok(())},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}})
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
                let mut output=None;caller.turn(loan(),|cx|{let turn=cursor.take_into(&mut output,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})?;Ok(())}).unwrap();
                let output=output.unwrap();assert_eq!(output.len(),count as usize);let actual=serde_json::to_value(output.iter().map(|row|(row.start,row.length,row.coverage)).collect::<Vec<_>>()).unwrap();assert_eq!(actual,expected);retire(&mut caller,output);
            }else{caller.cancel().cancel_now();}
            for _ in 0..100_000{if cursor.terminal_is_empty(){break;}caller.turn(loan(),|cx|{let demand=cursor.close_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let turn=cursor.close_step(grant);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())}).unwrap();}
            assert!(cursor.terminal_is_empty());
            for _ in 0..100_000{if decoder.terminal_is_empty(){break;}caller.turn(loan(),|cx|{let demand=decoder.retirement_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())}).unwrap();}
            assert!(decoder.terminal_is_empty());if let Some(input)=input{retire(&mut caller,input);}caller.retire(stock());
        }
    }
}
