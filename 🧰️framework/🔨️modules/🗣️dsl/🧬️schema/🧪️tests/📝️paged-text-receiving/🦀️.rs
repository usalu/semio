//! 📝️ Paged text receives one chunk per funded turn, equals std UTF-8, and closes every cancelled frontier.
use semio_framework_dsl_record::receiving::owned::{DslOwnedReceivingField,OwnedFieldReceiving,TextReceiving,OriginalReceivingPrimary};
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,paged::PagedUtf8,retirement::RetireOwned};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure};
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn close_value<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let mut value=ControlledRetirement::retain(value);for _ in 0..100_000{if value.terminal_is_empty(){return}caller.turn(loan(),|cx|{let demand=value.selected_demand(loan().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=value.step(grant);RetainedWorkOutcome{result,work:value.last_turn_work()}})?;Ok(())}).unwrap();}panic!("paged text fixture retirement did not settle")}
fn close<const N:usize>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cursor:&mut TextReceiving<N>){for _ in 0..100_000{if cursor.terminal_is_empty(){break}caller.turn(loan(),|cx|{let demand=cursor.close_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let turn=cursor.close_step(grant);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())}).unwrap();}assert!(cursor.terminal_is_empty());for _ in 0..100_000{if decoder.terminal_is_empty(){return}caller.turn(loan(),|cx|{let demand=decoder.retirement_demands(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())}).unwrap();}panic!("paged text fixture decoder did not settle")}
fn step<const N:usize>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,cursor:&mut TextReceiving<N>,input:&mut Option<FieldValue>)->Result<(),OriginalTurnFailure>{caller.turn_with_decoder(loan(),decoder,|cx|{let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|native|{let turn=cursor.step(input,native,32768,issued);work=turn.work;turn.result});let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};let receipt=cx.consume_receiving_retained(issued,progress);cx.consume_fuel(work);match result{Ok(_)=>{receipt?;Ok(())},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None})}})}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096}}
fn samples()->Vec<String>{vec![String::new(),"a".into(),"plain ascii text ".repeat(300),"\u{e4}\u{1f600}\u{4e2d}".repeat(900),"x".repeat(1024),"\u{e4}".repeat(513)]}
#[test]
fn paged_text_receives_chunk_per_turn_matches_std_utf8_and_cancels(){
    for text in samples(){for cut in [None,Some(0),Some(1),Some(2),Some(5)]{
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(998),Generation(18),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();
        let mut input=Some(FieldValue::Text(text.clone()));let mut cursor=PagedUtf8::<{1<<20}>::receiving_cursor();let mut turns=0;
        while !cursor.is_ready()&&cut.is_none_or(|cut|turns<cut){step(&mut caller,&mut decoder,&mut cursor,&mut input).unwrap();turns+=1;assert!(turns<100_000);}
        if cut.is_none(){
            let chunks=text.len().div_ceil(1024);assert!(turns>=chunks+2,"one funded turn per chunk plus admit and finish: {turns} for {} bytes",text.len());assert!(turns<=chunks+64,"bounded extra turns: {turns} for {chunks} chunks");
            let mut output=None;caller.turn(loan(),|cx|{let turn=cursor.take_into(&mut output,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})?;Ok(())}).unwrap();
            let output=output.unwrap();let bytes:Vec<u8>=output.bytes().collect();assert_eq!(std::str::from_utf8(&bytes).unwrap(),text);assert_eq!(output.len(),text.len());close_value(&mut caller,output);
        }else{caller.cancel().cancel_now();}
        close(&mut caller,&mut decoder,&mut cursor);if let Some(input)=input{close_value(&mut caller,input);}caller.retire(stock());
    }}
}
#[test]
fn paged_text_refuses_foreign_shapes_and_over_capacity_without_consuming_the_source(){
    for foreign in [FieldValue::Bool(true),FieldValue::UInt(1),FieldValue::Absent,FieldValue::Text("abcd".into())]{
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(999),Generation(19),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();
        let mut input=Some(foreign.clone());let mut cursor=PagedUtf8::<3>::receiving_cursor();
        assert!(step(&mut caller,&mut decoder,&mut cursor,&mut input).is_err(),"refused: {foreign:?}");assert!(input.is_some());
        close(&mut caller,&mut decoder,&mut cursor);if let Some(input)=input{close_value(&mut caller,input);}caller.retire(stock());
    }
}
