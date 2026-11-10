use super::*;
use semio_framework_value::OriginalRetirementFailure;
use semio_plugin::artifact_app_laws::OriginalWorkFixture;

fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:128}}
fn effect_turn_with<R>(caller:&mut OriginalWorkFixture,turn_grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Option<R>{let mut run=Some(run);let mut out=None;caller.turn(turn_grant,|cx|{let g=cx.retained_grant();let settled=cx.with_original_effect(g,|_,effect|(run.take().expect("one effect per turn"))(effect))?;out=settled.map(|(value,_)|value);Ok(())}).unwrap();out}
fn effect_turn<R>(caller:&mut OriginalWorkFixture,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{effect_turn_with(caller,grant(),run).expect("effect admitted")}
fn advance(caller:&mut OriginalWorkFixture,builder:&mut RasterSelectionSpansBuilder){effect_turn(caller,|effect|builder.advance(effect)).unwrap();}
fn close(caller:&mut OriginalWorkFixture,builder:&mut RasterSelectionSpansBuilder){
    for _ in 0..1_048_576{
        if builder.terminal_is_empty(){return;}
        effect_turn(caller,|effect|{
            if !builder.is_closing(){return builder.begin_close_paid(effect).map(|_|()).map_err(OriginalRetirementFailure::from);}
            let d=builder.close_demands(effect.grant().maximum_copy_bytes).map_err(OriginalRetirementFailure::from)?;assert!(d.release_bytes<=4096);
            let mut slot=None;let reply=OriginalEffectReply::borrow(&mut slot).map_err(OriginalRetirementFailure::from)?;
            builder.close_into(effect.grant().maximum_copy_bytes,effect,reply).map_err(OriginalRetirementFailure::from)?;
            slot.take().map_or(Ok(()),|settled|settled.map(|_|()))
        }).unwrap();
    }
    panic!("packed selection builder failed its original close");
}

#[test]
fn packed_coverage_pages_preserve_flat_rows_and_close_with_four_kib_releases(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../fixture.json")).unwrap();assert_eq!(RASTER_SELECTION_PAGE_ROWS,fixture["pageRows"].as_u64().unwrap()as usize);assert_eq!(size_of::<RasterSelectionSpan>(),fixture["currentSpanBytes"].as_u64().unwrap()as usize);assert!(SealedShared::<RasterSpanPage>::birth_bytes()<=4096);
    for count in fixture["counts"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let mut caller=OriginalWorkFixture::new();let mut builder=RasterSelectionSpansBuilder::new(count).unwrap();let mut accepted=0;let mut row=None;
        for _ in 0..count*8+1024{
            if builder.output_ready(){break;}
            if builder.accepts_row(){if row.is_none(){row=Some(RasterSelectionSpan{start:accepted as u32,length:1,coverage:if accepted%2==0{128}else{255}});}effect_turn(&mut caller,|effect|builder.push(&mut row,effect)).unwrap();if row.is_none(){accepted+=1;}}
            else if accepted==count&&!builder.finishing(){effect_turn(&mut caller,|effect|builder.finish(effect)).unwrap();}else{advance(&mut caller,&mut builder);}
        }
        assert!(builder.output_ready());let output=effect_turn(&mut caller,|effect|builder.take(effect)).unwrap().unwrap();assert_eq!(output.len(),count);assert_eq!(output.pages().len(),RasterSelectionSpans::page_count(count));for(index,row)in output.iter().enumerate(){assert_eq!((row.start,row.length,row.coverage),(index as u32,1,if index%2==0{128}else{255}));}close(&mut caller,&mut builder);caller.retire_owned(output,grant());caller.finish();
    }
}

#[test]
fn denied_and_cancelled_packed_frontiers_preserve_the_input_row(){
    for cut in [0,1,255,256,257]{let mut caller=OriginalWorkFixture::new();let mut builder=RasterSelectionSpansBuilder::new(512).unwrap();let mut accepted=0;let mut row=None;for _ in 0..10000{if accepted==cut{break;}if builder.accepts_row(){if row.is_none(){row=Some(RasterSelectionSpan{start:accepted,length:1,coverage:255});}let before=row;if let Some(denied)=effect_turn_with(&mut caller,RetainedCloneGrant::default(),|effect|builder.push(&mut row,effect)){assert_eq!(denied.unwrap(),RetainedCloneStep::Progress(Default::default()));}assert_eq!(row,before);effect_turn(&mut caller,|effect|builder.push(&mut row,effect)).unwrap();if row.is_none(){accepted+=1;}}else{advance(&mut caller,&mut builder);}}assert_eq!(accepted,cut);assert!(!builder.output_ready());close(&mut caller,&mut builder);caller.finish();}
}
