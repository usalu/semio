//! 🧪️ Saved configuration restores packed coverage through the genuine original Pack receiver.
use super::*;
use crate::{config::{RasterConfig,SetPixelSelectionEdit},selection::{RasterPixelSelection,RasterSelectionSpans}};
use semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan;
use semio_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneStep,RetainedWorkOutcome,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
use store::{OriginalArtifactPackReceiving,OriginalSnapshotRecordReceiving};
use store::snapshot_clone_preparation::config_apply::ConfigApplyMutation;

fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<22,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096}}
fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128}}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../fixture.json")).unwrap()}
fn receiving(cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_framework_value::NativeDecodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>)->Result<(),OriginalTurnFailure>{
    let grant=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|native|{let turn=run(native,grant);work=turn.work;turn.result});let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let receipt=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(work);match result{Ok(_)=>{receipt?;Ok(())},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}
}
fn retire<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let mut owner=ControlledRetirement::retain(value);for _ in 0..500_000{if owner.terminal_is_empty(){return;}caller.turn(loan(),|cx|{let demand=owner.selected_demand(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=owner.step(grant);RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;Ok(())}).unwrap();}panic!("Raster Pack fixture retirement did not settle");}

#[test]
fn original_config_pack_restores_flat_coverage_and_retires_cancelled_prefixes(){
    let row=fixture();let mut fixture=OriginalWorkFixture::with_source(OperationId(3697),Generation(17),stock(),||Some(1),u64::MAX,1<<26);
    let spans=fixture.receive_native::<_,RasterSelectionSpans>((0..row["spans"].as_u64().unwrap()).map(|index|RasterSelectionSpan{start:index as u32,length:1,coverage:128}).collect::<Vec<_>>(),loan());fixture.finish();
    let state=RasterConfig{pixel_selection:Some(RasterPixelSelection{layer_id:"pixels".into(),target:"pixels".into(),width:32,height:32,spans}),..Default::default()};
    let source=<RasterConfig as pack::snapshot::ArtifactPack>::encode_pack_with(&state,&Default::default()).unwrap();
    let pointer=source.as_ptr();
    for cut in std::iter::once(None).chain(row["cancelAfterTurns"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap() as usize))){
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(2697),Generation(17),stock(),||Some(1),u64::MAX,1<<26);let mut decoder=OriginalJobDecodeOwner::empty();let mut owner:Option<OriginalSnapshotRecordReceiving<RasterConfig>>=None;
        caller.turn_with_decoder(loan(),&mut decoder,|cx|receiving(cx,|native,grant|{let result=<RasterConfig as OriginalArtifactPackReceiving>::admit_original_pack(&source,protocol::PackLimits::default(),native,grant).map(|candidate|{if let Some((candidate,p))=candidate{owner=Some(candidate);RetainedCloneStep::Progress(p)}else{RetainedCloneStep::Progress(Default::default())}});native.receive_retained_work(grant,RetainedWorkOutcome{result,work:1})})).unwrap();
        let mut owner=owner.unwrap();let mut turns=0;
        while !owner.is_ready()&&cut.is_none_or(|cut|turns<cut){caller.turn_with_decoder(loan(),&mut decoder,|cx|receiving(cx,|native,grant|{let turn=owner.step(&source,native,32768,grant);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})).unwrap();turns+=1;assert_eq!(source.as_ptr(),pointer);assert!(turns<500_000);}
        if cut.is_none(){
            let mut output=None;caller.turn_with_decoder(loan(),&mut decoder,|cx|receiving(cx,|native,grant|{let result=owner.take_into(&mut output,grant).map(RetainedCloneStep::Progress);native.receive_retained_work(grant,RetainedWorkOutcome{result,work:1})})).unwrap();
            let output=output.unwrap();assert_eq!(output,state);let mutation=Mutation::SetPixelSelection(SetPixelSelectionEdit{selection:output.pixel_selection.clone()});assert!(mutation.payload_bytes()>0);retire(&mut caller,mutation);retire(&mut caller,output);
        }else{caller.cancel().cancel_now();}
        retire(&mut caller,owner);
        for _ in 0..500_000{if decoder.terminal_is_empty(){break;}caller.turn(loan(),|cx|{let demand=decoder.retirement_demands(32768)?;decoder.close_on_source(cx)?;Ok(())}).unwrap();}assert!(decoder.terminal_is_empty());caller.retire(stock());
    }
    let mut fixture=OriginalWorkFixture::new();fixture.retire_owned(state,loan());fixture.finish();
}
