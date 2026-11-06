//! 🛬️ Original Arc construction refuses before birth and retains every born root through cancellation or receipt refusal.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use super::*;
use semio_framework_job::{OriginalTurnFailure,OriginalTurnPrimary};
use semio_framework_value_native_owned::{OriginalReceivingPrimary};

fn advance(caller:&mut OriginalWorkFixture,owner:&mut NativeReceivingOriginal<semio_framework_value::DslValue,RasterImagePayload>)->Result<(),OriginalTurnFailure>{caller.turn(loan(),|cx|{let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control|{let turn=owner.advance(control,issued.maximum_copy_bytes,issued);work=turn.work;turn.result});let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let receipt=cx.consume_receiving_retained(issued,progress);cx.consume_fuel(work);match result{Ok(_)=>{receipt?;Ok(())},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}})}
fn close(caller:&mut OriginalWorkFixture,owner:&mut NativeReceivingOriginal<semio_framework_value::DslValue,RasterImagePayload>){owner.begin_close();for turn in 0..100_000{if owner.terminal_is_empty(){return}caller.turn(loan(),|cx|{let demand=owner.close_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.close_step(grant))?;Ok(())}).unwrap();assert!(turn<99_999);}panic!("native payload fixture close did not settle")}

#[test]
fn original_arc_quote_denial_keeps_input_before_allocating(){
    let mut caller=caller();let mut input=Some(image(1,1,vec![1,2,3,4]));let pointer=input.as_ref().unwrap().frames[0].rgba8.as_ptr();let mut output=None;
    let mut refusal=None;let denied=RetainedCloneGrant{maximum_capacity_bytes:0,..loan()};caller.turn(denied,|cx|{let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control|{let turn=control.receive_arc(&mut input,&mut output);work=turn.work;turn.result});assert_eq!(work,0);let error=result.expect_err("actual caller must refuse an unfunded Arc quote");assert!(error.quote_refusal.is_some());let progress=error.retained_progress();refusal=Some(error);assert_eq!(progress,Default::default());cx.consume_receiving_retained(issued,progress)?;cx.consume_fuel(work);Ok(())}).unwrap();assert!(output.is_none());assert_eq!(input.as_ref().unwrap().frames[0].rgba8.as_ptr(),pointer);caller.retire_owned(refusal.unwrap(),loan());
    caller.turn(loan(),|cx|{let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control|{let turn=control.receive_arc(&mut input,&mut output);work=turn.work;turn.result});let progress=result.unwrap();assert_eq!(progress.retained_capacity_bytes,RasterImagePayload::owned_birth_demands().capacity_bytes);cx.consume_receiving_retained(issued,progress)?;cx.consume_fuel(work);Ok(())}).unwrap();assert!(input.is_none());let output=output.unwrap();assert_eq!(output.frames[0].rgba8.as_ptr(),pointer);close_alias(&mut caller,output);caller.finish();
}

#[test]
fn born_original_arc_survives_enclosing_receipt_refusal_and_funded_close(){
    let mut caller=caller();let mut input=Some(image(1,1,vec![1,2,3,4]));let pointer=input.as_ref().unwrap().frames[0].rgba8.as_ptr();let mut output=None;let mut born=Default::default();
    let result=caller.turn(loan(),|cx|{let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control|{let turn=control.receive_arc(&mut input,&mut output);work=turn.work;turn.result});born=result.unwrap();cx.consume_fuel(work);cx.consume_receiving_retained(issued,Default::default())?;Ok(())});let error=result.expect_err("omitted actual allocation receipt must refuse");assert!(born.retained_capacity_bytes>0);assert!(input.is_none());let output=output.unwrap();assert_eq!(output.frames[0].rgba8.as_ptr(),pointer);caller.retire_owned(error,loan());let (_,released)=close_alias(&mut caller,output);assert!(released>=born.retained_capacity_bytes+4);caller.finish();
}

#[test]
fn original_native_payload_cancels_at_every_actual_receiving_turn(){
    let input=image(1,1,vec![255,128,64,32]).to_value();let maximum=fixture()["maximumNativeTurns"].as_u64().unwrap() as usize;
    let mut baseline=caller();let original=source(&mut baseline,input.clone());let mut owner=NativeReceivingOriginal::<_,RasterImagePayload>::new(original);let mut turns=0;
    while !owner.output_ready(){advance(&mut baseline,&mut owner).unwrap();turns+=1;assert!(turns<=maximum);}close(&mut baseline,&mut owner);baseline.finish();
    for cut in 0..=turns{let mut caller=caller();let original=source(&mut caller,input.clone());let mut owner=NativeReceivingOriginal::<_,RasterImagePayload>::new(original);for _ in 0..cut{advance(&mut caller,&mut owner).unwrap();}caller.cancel().cancel_now();if !owner.output_ready(){let error=advance(&mut caller,&mut owner).expect_err("original cancellation must refuse the next construction turn");assert!(error.primary.message().contains("cancel"));caller.retire_owned(error,loan());}close(&mut caller,&mut owner);caller.finish();}
}
